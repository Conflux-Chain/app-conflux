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
use crate::consts::ADDRRESS_BYTES_LEN;
use crate::AppSW;

use include_gif::include_gif;
use ledger_device_sdk::nbgl::{NbglAddressReview, NbglGlyph};

pub fn ui_display_pk(addr: &[u8], chain_id: u32) -> Result<bool, AppSW> {
    let addr = &addr[addr.len() - ADDRRESS_BYTES_LEN..]; // last 20 bytes
    let network = Network::from_network_id(chain_id as u64);
    let cfx_addr = cfx_addr_encode(addr, network).map_err(|_e| AppSW::AddrDisplayFail)?;

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    const CFX: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/cfx_64.gif", NBGL));
    #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
    const CFX: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/cfx_14.gif", NBGL));
    // Display the address confirmation screen.
    Ok(NbglAddressReview::new()
        .glyph(&CFX)
        .review_title("Verify CFX address")
        .show(&cfx_addr))
}
