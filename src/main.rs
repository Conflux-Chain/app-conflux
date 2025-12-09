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

#![no_std]
#![no_main]

// Required for using String, Vec, format!...
extern crate alloc;

mod app_sw;
mod app_ui;
mod bip32_path;
mod cfx_addr;
mod consts;
mod crypto;
mod eip712;
mod handlers;
mod ins_consts;
mod instruction;
mod types;

mod settings;

use app_sw::AppSW;
use app_ui::menu::ui_menu_main;
use eip712::Eip712Context;
use handlers::{
    common::Context,
    get_public_key::handler_get_public_key,
    get_version::handler_get_version,
    sign_191::handler_sign_191,
    sign_712::{
        handler_sign_712, handler_sign_712_struct_definition, handler_sign_712_struct_impl,
    },
    sign_tx::handler_sign_tx,
};
use ins_consts::CLA;
use instruction::Instruction;
use ledger_device_sdk::{
    io::Comm,
    nbgl::{init_comm, NbglReviewStatus, StatusType},
};

ledger_device_sdk::set_panic!(ledger_device_sdk::exiting_panic);

fn show_status_and_home_if_needed(ins: &Instruction, tx_ctx: &mut Context, status: &AppSW) {
    let (show_status, status_type) = match (ins, status) {
        (
            Instruction::GetPubkey {
                display: true,
                return_chain_code: _,
            },
            AppSW::Deny | AppSW::Ok,
        ) => (true, StatusType::Address),
        (Instruction::SignTx { .. }, AppSW::Deny | AppSW::Ok) if tx_ctx.finished() => {
            (true, StatusType::Transaction)
        }
        (Instruction::Sign191 { .. }, AppSW::Deny | AppSW::Ok) if tx_ctx.finished() => {
            (true, StatusType::Message)
        }
        (Instruction::Sign712 { .. }, AppSW::Deny | AppSW::Ok) => (true, StatusType::Message),
        (_, _) => (false, StatusType::Address),
    };

    if show_status {
        let success = *status == AppSW::Ok;
        NbglReviewStatus::new()
            .status_type(status_type)
            .show(success);

        // call home.show_and_return() to show home and setting screen
        tx_ctx.home.show_and_return();
    }
}

#[no_mangle]
extern "C" fn sample_main() {
    // Create the communication manager, and configure it to accept only APDU from the 0xe0 class.
    // If any APDU with a wrong class value is received, comm will respond automatically with
    // BadCla status word.
    let mut comm = Comm::new().set_expected_cla(CLA);
    init_comm(&mut comm);

    let mut tx_ctx = Context::new();
    tx_ctx.home = ui_menu_main(&mut comm);
    tx_ctx.home.show_and_return();

    let mut eip712_ctx = Eip712Context::new();

    loop {
        let ins: Instruction = comm.next_command();

        let status = match handle_apdu(&mut comm, &ins, &mut tx_ctx, &mut eip712_ctx) {
            Ok(()) => {
                comm.reply_ok();
                AppSW::Ok
            }
            Err(sw) => {
                comm.reply(sw);
                sw
            }
        };
        show_status_and_home_if_needed(&ins, &mut tx_ctx, &status);
    }
}

fn handle_apdu(
    comm: &mut Comm,
    ins: &Instruction,
    ctx: &mut Context,
    eip712_ctx: &mut Eip712Context,
) -> Result<(), AppSW> {
    match ins {
        Instruction::GetAppName => {
            comm.append(env!("CARGO_PKG_NAME").as_bytes());
            Ok(())
        }
        Instruction::GetVersion => handler_get_version(comm),
        Instruction::GetPubkey {
            display,
            return_chain_code,
        } => handler_get_public_key(comm, *display, *return_chain_code),
        Instruction::SignTx { chunk, more } => handler_sign_tx(comm, *chunk, *more, ctx),
        Instruction::Sign191 { chunk, more } => handler_sign_191(comm, *chunk, *more, ctx),
        Instruction::Eip712StructDefinition { is_struct_name } => {
            handler_sign_712_struct_definition(comm, *is_struct_name, eip712_ctx)
        }
        Instruction::Eip712StructImplementation { more, data_type } => {
            handler_sign_712_struct_impl(comm, *more, *data_type, eip712_ctx)
        }
        Instruction::Sign712 => handler_sign_712(comm, eip712_ctx),
    }
}
