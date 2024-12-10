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

use crate::app_ui::address::ui_display_pk;
use crate::consts::{ADDRRESS_BYTES_LEN, HASH_BYTES_LEN};
use crate::utils::Bip32Path;
use crate::AppSW;
use ledger_device_sdk::ecc::{Secp256k1, SeedDerive};
use ledger_device_sdk::hash::{sha3::Keccak256, HashInit};
use ledger_device_sdk::io::Comm;

pub fn handler_get_public_key(
    comm: &mut Comm,
    display: bool,
    return_chain_code: bool,
) -> Result<(), AppSW> {
    let mut chain_id: u32 = 0;
    let mut data = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;

    // Extract chain id if display is requested (last 4 bytes are chain id)
    if display {
        chain_id = u32::from_be_bytes(data[data.len() - 4..].try_into().unwrap());
        data = &data[..data.len() - 4];
    }

    let path: Bip32Path = data.try_into()?;

    let (k, cc) = Secp256k1::derive_from(path.as_ref());
    let pk = k.public_key().map_err(|_| AppSW::KeyDeriveFail)?;
    drop(k);

    // Display address on device if requested
    if display {
        let mut keccak256 = Keccak256::new();
        let mut address: [u8; 32] = [0u8; HASH_BYTES_LEN];

        let _ = keccak256.hash(&pk.pubkey[1..], &mut address);

        // Conflux user addresses start with b0001
        address[HASH_BYTES_LEN - ADDRRESS_BYTES_LEN] &= 0x0f;
        address[HASH_BYTES_LEN - ADDRRESS_BYTES_LEN] |= 0x10;

        if !ui_display_pk(&address, chain_id)? {
            return Err(AppSW::Deny);
        }
    }

    comm.append(&[pk.pubkey.len() as u8]);
    comm.append(&pk.pubkey);

    if !return_chain_code {
        return Ok(());
    }
    const CHAINCODE_LEN: u8 = 32;
    let code = cc.unwrap();
    comm.append(&[CHAINCODE_LEN]);
    comm.append(&code.value);

    Ok(())
}
