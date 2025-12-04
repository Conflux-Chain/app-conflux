#![allow(unused_assignments)]
use crate::{
    app_ui::CFX_ICON,
    eip712::{types::EIP712_DOMAIN_TYPE_NAME, Eip712Context},
    AppSW,
};
use alloc::{format, vec::Vec};
use ledger_device_sdk::nbgl::{Field, NbglReview};
use ledger_rust_eip712::parser;

pub fn ui_display_712_message(ctx: &Eip712Context) -> Result<bool, AppSW> {
    let base_count = 2; // two "Review struct"
    let domain_count = ctx.eip712_domain.name.is_some() as usize
        + ctx.eip712_domain.version.is_some() as usize
        + ctx.eip712_domain.chain_id.is_some() as usize
        + ctx.eip712_domain.verifying_contract.is_some() as usize
        + ctx.eip712_domain.salt.is_some() as usize;
    let field_count = ctx.current_struct_field_values.len();
    let mut my_fields: Vec<Field> = Vec::with_capacity(base_count + domain_count + field_count);

    my_fields.push(Field {
        name: "Review struct",
        value: EIP712_DOMAIN_TYPE_NAME,
    });

    if let Some(name) = ctx.eip712_domain.name.as_ref() {
        my_fields.push(Field {
            name: "Name",
            value: name.as_ref(),
        });
    }

    if let Some(version) = ctx.eip712_domain.version.as_ref() {
        my_fields.push(Field {
            name: "Version",
            value: version.as_ref(),
        });
    }

    let mut chain_id_str = Default::default();
    if let Some(chain_id) = ctx.eip712_domain.chain_id.as_ref() {
        chain_id_str = format!("{}", chain_id);
        my_fields.push(Field {
            name: "ChainId",
            value: chain_id_str.as_str(),
        });
    }

    let mut contract_addr_str = Default::default();
    if let Some(contract) = ctx.eip712_domain.verifying_contract.as_ref() {
        contract_addr_str = format!("{}", contract);
        my_fields.push(Field {
            name: "Verifying Contract",
            value: contract_addr_str.as_str(),
        });
    }

    let mut salt_str = Default::default();
    if let Some(salt) = ctx.eip712_domain.salt.as_ref() {
        salt_str = format!("{}", salt);
        my_fields.push(Field {
            name: "Salt",
            value: salt_str.as_str(),
        });
    }

    let root_struct_name = ctx
        .current_root_struct
        .as_ref()
        .map(|c| c.as_ref())
        .unwrap_or("N/A");
    my_fields.push(Field {
        name: "Review struct",
        value: root_struct_name,
    });

    let primary_type = ctx.current_root_struct.as_ref().expect("should exist");

    let type_schema = parser::build_schema(&ctx.struct_definitions, primary_type)
        .map_err(|_| AppSW::TxDisplayFail)?;

    let mut data_iter = ctx
        .current_struct_field_values
        .iter()
        .map(|v| v.value.as_slice());

    let fields = parser::build_ui_fields(&type_schema, &mut data_iter, "")
        .map_err(|_| AppSW::TxDisplayFail)?;

    my_fields.extend(
        fields
            .iter()
            .map(|f| Field {
                name: f.name,
                value: f.value.as_ref(),
            })
            .collect::<Vec<_>>(),
    );

    let review: NbglReview = NbglReview::new()
        .titles(
            "Review Typed Message",
            "Blind signing required",
            "Accept risk and sign typed message?",
        )
        .blind()
        .glyph(&CFX_ICON);

    Ok(review.show(&my_fields))
}
