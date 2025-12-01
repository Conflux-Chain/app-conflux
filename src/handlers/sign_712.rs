use crate::{
    app_ui::eip712::ui_display_712_message,
    bip32_path::Bip32Path,
    eip712::{
        types::{Eip712FieldDefinition, Eip712FieldValue, EIP712_DOMAIN_TYPE_NAME},
        utils::parse_utf8_string,
        Eip712Context,
    },
    handlers::sign_and_send,
    ins_consts::p2_eip712_struct_impl,
    AppSW,
};
use alloc::borrow::ToOwned;
use ledger_device_sdk::io::Comm;

pub fn handler_sign_712_struct_definition(
    comm: &mut Comm,
    is_struct_name: bool,
    ctx: &mut Eip712Context,
) -> Result<(), AppSW> {
    let data = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;

    if is_struct_name {
        if ctx.current_struct_name.is_none() {
            ctx.reset();
        } else {
            ctx.complete_one_struct_def();
        }
        // decode struct name
        let struct_name = parse_utf8_string(data).map_err(|_| AppSW::InvalidData)?;
        ctx.current_struct_name = Some(struct_name);
    } else {
        // decode struct info
        if ctx.current_struct_name.is_none() {
            return Err(AppSW::InvalidData); // We need a struct name before we can add fields
        }

        let field_definition =
            Eip712FieldDefinition::from_bytes(&data).map_err(|_| AppSW::InvalidData)?;
        ctx.current_struct_fields.push(field_definition);
    }

    Ok(())
}

// must first pass EIP712Domain implementation, and then primary type's implementation
pub fn handler_sign_712_struct_impl(
    comm: &mut Comm,
    more: bool,
    data_type: u8,
    ctx: &mut Eip712Context,
) -> Result<(), AppSW> {
    let data = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;
    match data_type {
        p2_eip712_struct_impl::ROOT_STRUCT => {
            ctx.complete_one_struct_def();

            let struct_name = parse_utf8_string(data).map_err(|_| AppSW::InvalidString)?;

            // EIP712_DOMAIN_TYPE_NAME must come first
            if ctx.current_root_struct.is_none() && struct_name.as_str() != EIP712_DOMAIN_TYPE_NAME
            {
                return Err(AppSW::InvalidData);
            }
            ctx.parse_eip712_domain().map_err(|_| AppSW::InvalidData)?;

            ctx.current_root_struct = Some(struct_name);
        }
        p2_eip712_struct_impl::ARRAY => {
            if data.len() != 1 {
                return Err(AppSW::WrongDataLength);
            }
            ctx.current_struct_field_values.push(Eip712FieldValue {
                value: data.to_owned(),
                is_array_size: true,
            });
        }
        p2_eip712_struct_impl::STRUCT_FIELD => {
            if data.len() <= 2 {
                return Err(AppSW::WrongDataLength);
            }
            let bytes = [data[0], data[1]];
            let size = u16::from_be_bytes(bytes) as usize;
            let field_value = &data[2..2 + size];
            ctx.field_data.extend_from_slice(field_value);
            if !more {
                ctx.current_struct_field_values
                    .push(Eip712FieldValue::from_bytes(ctx.field_data.to_owned()));
                ctx.field_data.clear();
            }
        }
        _ => {
            // should not happen
            return Err(AppSW::InternalError);
        }
    }
    Ok(())
}

pub fn handler_sign_712(comm: &mut Comm, ctx: &mut Eip712Context) -> Result<(), AppSW> {
    // retrieve bip path
    let data = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;
    let path: Bip32Path = data.try_into()?;

    let message_reviewed = ui_display_712_message(ctx)?;

    if !message_reviewed {
        ctx.reset();
        return Err(AppSW::Deny);
    }

    // compute 712 message hash
    let message_hash = ctx
        .eip712_signing_hash()
        .map_err(|_| AppSW::InternalError)?;

    let res = sign_and_send(comm, &path, message_hash.as_slice())?;

    // reset the context
    ctx.reset();

    Ok(res)
}
