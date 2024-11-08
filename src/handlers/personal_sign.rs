use crate::app_ui::sign::ui_display_msg;
use crate::consts::{MAX_TRANSACTION_LEN, PERSONAL_SIGN_PREFIX};
use crate::crypto::decode_der_sig;
use crate::handlers::sign_tx::TxContext;
use crate::AppSW;
use alloc::{format, vec::Vec};
use ledger_device_sdk::{
    ecc::{Secp256k1, SeedDerive},
    hash::{sha3::Keccak256, HashInit},
    io::Comm,
};

pub fn handler_personal_sign(
    comm: &mut Comm,
    chunk: u8,
    more: bool,
    ctx: &mut TxContext,
) -> Result<(), AppSW> {
    // Try to get data from comm
    let data = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;
    // First chunk, try to parse the path
    if chunk == 0 {
        // Reset transaction context
        ctx.reset();
        // This will propagate the error if the path is invalid
        ctx.path = data.try_into()?;
        Ok(())
    // Next chunks, append data to raw_tx and return or parse
    // the transaction if it is the last chunk.
    } else {
        if ctx.raw_tx.len() + data.len() > MAX_TRANSACTION_LEN {
            return Err(AppSW::TxWrongLength);
        }

        // Append data to raw_tx
        ctx.raw_tx.extend(data);

        // If we expect more chunks, return
        if more {
            ctx.review_finished = false;
            Ok(())
        // Otherwise, try to parse the transaction
        } else {
            // Display msg. If user approves
            // the msg, sign it. Otherwise,
            // return a "deny" status word.
            if ui_display_msg(&ctx.raw_tx)? {
                ctx.review_finished = true;
                compute_signature_and_append(comm, ctx)
            } else {
                ctx.review_finished = true;
                Err(AppSW::Deny)
            }
        }
    }
}

fn compute_signature_and_append(comm: &mut Comm, ctx: &mut TxContext) -> Result<(), AppSW> {
    let mut keccak256 = Keccak256::new();
    let mut message_hash: [u8; 32] = [0u8; 32];

    let msg_with_prefix = add_prefix_to_message(&ctx.raw_tx);

    let _ = keccak256.hash(&msg_with_prefix, &mut message_hash);

    let (sig, siglen, parity) = Secp256k1::derive_from_path(ctx.path.as_ref())
        .deterministic_sign(&message_hash)
        .map_err(|_| AppSW::TxSignFail)?;

    let mut r: [u8; 32] = [0u8; 32];
    let mut s: [u8; 32] = [0u8; 32];
    let _ =
        decode_der_sig(&sig[..siglen as usize], &mut r, &mut s).map_err(|_| AppSW::TxSignFail)?;

    comm.append(&[parity as u8]);
    comm.append(&r);
    comm.append(&s);

    Ok(())
}

fn add_prefix_to_message(msg: &[u8]) -> Vec<u8> {
    let msg_len_str = format!("{}", msg.len());
    let mut prefixed_msg: Vec<u8> =
        Vec::with_capacity(PERSONAL_SIGN_PREFIX.len() + msg_len_str.len() + msg.len());
    prefixed_msg.extend(PERSONAL_SIGN_PREFIX.as_bytes());
    prefixed_msg.extend(msg_len_str.as_bytes());
    prefixed_msg.extend(msg);
    prefixed_msg
}
