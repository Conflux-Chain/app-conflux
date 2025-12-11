use crate::{
    app_ui::sign::ui_display_personal_message,
    handlers::{common::Context, hash_sign_and_send},
    AppSW,
};
use alloc::{format, vec::Vec};
use alloy_primitives::utils::eip191_message;
use ledger_device_sdk::io::Comm;

pub fn handler_personal_sign(
    comm: &mut Comm,
    chunk: u8,
    more: bool,
    ctx: &mut Context,
    eip191: bool,
) -> Result<(), AppSW> {
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
            ctx.review_finished = true;
            // Display message. If user approves
            // the message, sign it. Otherwise,
            // return a "deny" status word.
            if ui_display_personal_message(ctx)? {
                let raw_data = if eip191 {
                    eip191_message(&ctx.raw_tx)
                } else {
                    cip23_message(&ctx.raw_tx)
                };
                hash_sign_and_send(comm, &ctx.path, &raw_data)
            } else {
                Err(AppSW::Deny)
            }
        }
    }
}

fn cip23_message(message: &[u8]) -> Vec<u8> {
    let prefix = format!("\x19Conflux Signed Message:\n{}", message.len());
    let mut out = Vec::with_capacity(prefix.len() + message.len());
    out.extend_from_slice(prefix.as_bytes());
    out.extend_from_slice(message);
    out
}
