/*****************************************************************************
 *   Ledger App Conflux Rust.
 *   (c) 2023 Conflux Foundation.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *****************************************************************************/
use crate::cfx_addr::{cfx_addr_encode, Network};
use crate::handlers::sign_tx::TxContext;
use crate::settings::Settings;
use crate::types::{Transaction, U256};
use crate::AppSW;

#[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14, WARNING},
    gadgets::{clear_screen, Field, MultiFieldReview, Page},
};

#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use include_gif::include_gif;
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::{Field, NbglChoice, NbglGlyph, NbglReview, PageIndex};

use alloc::{format, vec};

/// Displays a transaction and returns true if user approved it.
///
/// This method can return [`AppSW::TxDisplayFail`] error
///
/// # Arguments
///
/// * `tx` - Transaction to be displayed for validation
#[allow(unused_variables)]
pub fn ui_display_tx(tx: &Transaction, ctx: &mut TxContext) -> Result<bool, AppSW> {
    let fully_decoded = tx.fully_decoded();

    let value_str = tx.value.cfx_str().ok_or(AppSW::TxDisplayFail)?;
    let value_with_unit = format!("{} CFX", value_str);

    let network = Network::from_network_id(tx.chain_id);
    let to_str = cfx_addr_encode(&*tx.to, network).map_err(|_e| AppSW::AddrDisplayFail)?;

    let fee_str = tx.max_gas_fee().cfx_str().ok_or(AppSW::TxDisplayFail)?;
    let fee_with_unit = format!("{} CFX", fee_str);

    // Define transaction review fields
    let mut my_fields = vec![
        Field {
            name: "Amount",
            value: value_with_unit.as_str(),
        },
        Field {
            name: "To",
            value: to_str.as_str(),
        },
        Field {
            name: "Max Gas Fees",
            value: fee_with_unit.as_str(),
        },
    ];

    // If max storage fee is not zero, add it to the review fields
    let storage_fee_str = tx.max_storage_fee().cfx_str().ok_or(AppSW::TxDisplayFail)?;
    let storage_fee_with_unit = format!("{} CFX", storage_fee_str);
    if tx.max_storage_fee() > U256::zero() {
        my_fields.push(Field {
            name: "Max Storage Fees",
            value: storage_fee_with_unit.as_str(),
        });
    }

    // If data is not empty, add it to the review fields
    let data_str = format!("0x{}", hex::encode(tx.data.clone()).to_uppercase());
    if !tx.data.is_empty() {
        my_fields.push(Field {
            name: "Data",
            value: data_str.as_str(),
        });
    }

    let settings: Settings = Default::default();

    // Create transaction review
    #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
    {
        if !fully_decoded && settings.get_element(0)? == 0 {
            // show warning and return
            let warning =
                Page::from((["Blind signing must", "be enabled in Settings"], &CROSSMARK));
            clear_screen();
            warning.place_and_wait();
            return Ok(false);
        }

        if !fully_decoded {
            // show warning
            let warning = Page::from((["Blind", "Signing"], &WARNING));
            clear_screen();
            warning.place_and_wait();
        }

        let my_review = MultiFieldReview::new(
            &my_fields,
            &["Review ", "Transaction"],
            Some(&EYE),
            "Approve",
            Some(&VALIDATE_14),
            "Reject",
            Some(&CROSSMARK),
        );
        Ok(my_review.show())
    }

    #[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
    {
        if !fully_decoded && settings.get_element(0)? == 0 {
            let confirmed = NbglChoice::new().show(
                "This transaction cannot be clear-signed",
                "Enable blind signing in the settings to sign this transaction.",
                "Go to settings",
                "Reject transaction",
            );

            if confirmed {
                ctx.home.set_start_page(PageIndex::Settings(0));
            }

            return Ok(false);
        } else {
            ctx.home.set_start_page(PageIndex::Home);
        }
        // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
        const CFX: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/cfx_64.gif", NBGL));
        // Create NBGL review. Maximum number of fields and string buffer length can be customised
        // with constant generic parameters of NbglReview. Default values are 32 and 1024 respectively.
        let mut review: NbglReview = NbglReview::new()
            .titles(
                "Review transaction\nto send CFX",
                "",
                "Sign transaction\nto send CFX",
            )
            .glyph(&CFX);

        if !fully_decoded {
            review = review.blind();
        }

        // If second setting switch is disabled do not display the transaction data
        if settings.get_element(1)? == 0 && !tx.data.is_empty() {
            let field_len = my_fields.len() - 1;
            Ok(review.show(&my_fields[0..field_len]))
        } else {
            Ok(review.show(&my_fields))
        }
    }
}
