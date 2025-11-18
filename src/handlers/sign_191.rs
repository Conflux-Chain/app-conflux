use super::common::Context;
use crate::app_ui::sign::ui_display_191_message;
use crate::consts::EIP191_PREFIX;
use crate::crypto::decode_der_sig;
use crate::AppSW;
use alloc::vec::Vec;
use ledger_device_sdk::ecc::{Secp256k1, SeedDerive};
use ledger_device_sdk::hash::{sha3::Keccak256, HashInit};
use ledger_device_sdk::io::Comm;

pub fn handler_sign_191(
    comm: &mut Comm,
    chunk: u8,
    more: bool,
    ctx: &mut Context,
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
        // Append data to raw_tx
        ctx.raw_tx.extend(data);

        // If we expect more chunks, return
        if more {
            ctx.review_finished = false;
            Ok(())
        // Otherwise, try to parse the transaction
        } else {
            // Display 191 message. If user approves
            // the message, sign it. Otherwise,
            // return a "deny" status word.
            if ui_display_191_message(ctx)? {
                ctx.review_finished = true;
                compute_191_signature_and_append(comm, ctx)
            } else {
                ctx.review_finished = true;
                Err(AppSW::Deny)
            }
        }
    }
}

fn eip191_personal_message_bytes(message: &[u8]) -> Vec<u8> {
    // let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());
    // let mut out = Vec::with_capacity(prefix.len() + message.len());
    // out.extend_from_slice(prefix.as_bytes());
    // out.extend_from_slice(message);
    // out

    let len = message.len();
    let mut len_string_buffer = itoa::Buffer::new();
    let len_string = len_string_buffer.format(len);

    let mut eth_message = Vec::with_capacity(EIP191_PREFIX.len() + len_string.len() + len);
    eth_message.extend_from_slice(EIP191_PREFIX.as_bytes());
    eth_message.extend_from_slice(len_string.as_bytes());
    eth_message.extend_from_slice(message);
    eth_message
}

// compute 191 signature and append to comm
fn compute_191_signature_and_append(comm: &mut Comm, ctx: &mut Context) -> Result<(), AppSW> {
    let mut keccak256 = Keccak256::new();
    let mut message_hash: [u8; 32] = [0u8; 32];

    let raw_data = eip191_personal_message_bytes(&ctx.raw_tx);

    let _ = keccak256.hash(&raw_data, &mut message_hash);

    let (sig, siglen, parity) = Secp256k1::derive_from_path(ctx.path.as_ref())
        .deterministic_sign(&message_hash)
        .map_err(|_| AppSW::TxSignFail)?;

    let mut r: [u8; 32] = [0u8; 32];
    let mut s: [u8; 32] = [0u8; 32];

    decode_der_sig(&sig[..siglen as usize], &mut r, &mut s).map_err(|_| AppSW::TxSignFail)?;

    comm.append(&[parity as u8]);
    comm.append(&r);
    comm.append(&s);
    Ok(())
}
