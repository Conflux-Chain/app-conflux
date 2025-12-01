#![allow(unused_assignments)]
use super::CFX_ICON;
use crate::{
    eip712::{types::EIP712_DOMAIN_TYPE_NAME, Eip712Context},
    AppSW,
};
use alloc::{format, vec, vec::Vec};
use ledger_device_sdk::nbgl::{Field, NbglReview};

pub fn ui_display_712_message(ctx: &Eip712Context) -> Result<bool, AppSW> {
    let mut my_fields = vec![Field {
        name: "Review struct",
        value: EIP712_DOMAIN_TYPE_NAME,
    }];

    if ctx.eip712_domain.name.is_some() {
        my_fields.push(Field {
            name: "Name",
            value: ctx
                .eip712_domain
                .name
                .as_ref()
                .map(|c| c.as_ref())
                .expect("success"),
        });
    }

    if ctx.eip712_domain.version.is_some() {
        my_fields.push(Field {
            name: "Version",
            value: ctx
                .eip712_domain
                .version
                .as_ref()
                .map(|c| c.as_ref())
                .expect("success"),
        });
    }

    let mut chain_id_str = Default::default();
    if ctx.eip712_domain.chain_id.is_some() {
        chain_id_str = ctx
            .eip712_domain
            .chain_id
            .as_ref()
            .map(|c| format!("{}", c))
            .expect("succss");
        my_fields.push(Field {
            name: "ChainId",
            value: chain_id_str.as_str(),
        });
    }

    let mut contract_str = Default::default();
    if ctx.eip712_domain.verifying_contract.is_some() {
        contract_str = ctx
            .eip712_domain
            .verifying_contract
            .as_ref()
            .map(|c| format!("{}", c))
            .expect("success");
        my_fields.push(Field {
            name: "Verifying Contract",
            value: contract_str.as_str(),
        });
    }

    let mut salt_str = Default::default();
    if ctx.eip712_domain.salt.is_some() {
        salt_str = ctx
            .eip712_domain
            .salt
            .as_ref()
            .map(|c| format!("{}", c))
            .expect("success");
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

    let fields = ctx.message_fields().map_err(|_| AppSW::TxDisplayFail)?;
    let fields: Vec<Field> = fields
        .iter()
        .map(|f| Field {
            name: f.name.as_str(),
            value: f.value.as_str(),
        })
        .collect();

    my_fields.extend(fields);

    let review: NbglReview = NbglReview::new()
        .titles(
            "Review Typed Message",
            "blid Signing required",
            "Access risk and sign typed message",
        )
        .glyph(&CFX_ICON);

    Ok(review.show(&my_fields))
}
