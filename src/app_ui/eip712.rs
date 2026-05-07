use crate::{
    app_ui::CFX_ICON,
    eip712::{Eip712Context, CIP23_DOMAIN_TYPE_NAME, EIP712_DOMAIN_TYPE_NAME},
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

    let domain_type_name = if ctx.is_cip23_domain() {
        CIP23_DOMAIN_TYPE_NAME
    } else {
        EIP712_DOMAIN_TYPE_NAME
    };

    my_fields.push(Field {
        name: "Review struct",
        value: domain_type_name,
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

    let chain_id_str = ctx
        .eip712_domain
        .chain_id
        .as_ref()
        .map(|c| format!("{}", c));
    if let Some(s) = chain_id_str.as_ref() {
        my_fields.push(Field {
            name: "ChainId",
            value: s.as_str(),
        });
    }

    let contract_addr_str = ctx
        .eip712_domain
        .verifying_contract
        .as_ref()
        .map(|c| format!("{}", c));
    if let Some(s) = contract_addr_str.as_ref() {
        my_fields.push(Field {
            name: "Verifying Contract",
            value: s.as_str(),
        });
    }

    let salt_str = ctx.eip712_domain.salt.as_ref().map(|s| format!("{}", s));
    if let Some(s) = salt_str.as_ref() {
        my_fields.push(Field {
            name: "Salt",
            value: s.as_str(),
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
            "Accept risk and sign message",
        )
        .blind()
        .glyph(&CFX_ICON);

    Ok(review.show(&my_fields))
}
