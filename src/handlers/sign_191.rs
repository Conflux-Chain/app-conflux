use super::common::Context;
use crate::app_ui::sign::ui_display_191_message;
// use crate::consts::EIP191_PREFIX;
use super::hash_sign_and_send;
use crate::AppSW;
// use alloc::vec::Vec;
use alloy_primitives::utils::eip191_message;
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
                let raw_data = eip191_message(&ctx.raw_tx);
                hash_sign_and_send(comm, &ctx.path, &raw_data)
            } else {
                ctx.review_finished = true;
                Err(AppSW::Deny)
            }
        }
    }
}

// fn eip191_personal_message_bytes(message: &[u8]) -> Vec<u8> {
//     let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());
//     let mut out = Vec::with_capacity(prefix.len() + message.len());
//     out.extend_from_slice(prefix.as_bytes());
//     out.extend_from_slice(message);
//     out
// }
