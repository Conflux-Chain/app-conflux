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
use crate::{settings::Settings, AppSW};
use core::str::FromStr;
use ledger_device_sdk::io;

pub fn handler_get_version(comm: &mut io::Comm) -> Result<(), AppSW> {
    if let Some((major, minor, patch)) = parse_version_string(env!("CARGO_PKG_VERSION")) {
        let mut flags: u8 = 0b0000_0100;
        let settings: Settings = Default::default();
        if settings.get_element(0)? == 1 {
            // If the first byte is 1, then the user has enabled the "Blind signing" setting
            flags |= crate::consts::APP_FLAG_BLIND_SIGNING_ENABLED;
        }
        if settings.get_element(1)? == 1 {
            // If the first byte is 1, then the user has enabled the "Display data" setting
            flags |= crate::consts::APP_FLAG_DETAILED_DISPLAY_ENABLED;
        }

        comm.append(&[flags, major, minor, patch]);
        Ok(())
    } else {
        Err(AppSW::VersionParsingFail)
    }
}

fn parse_version_string(input: &str) -> Option<(u8, u8, u8)> {
    // Split the input string by '.'.
    // Input should be of the form "major.minor.patch",
    // where "major", "minor", and "patch" are integers.
    let mut parts = input.split('.');
    let major = u8::from_str(parts.next()?).ok()?;
    let minor = u8::from_str(parts.next()?).ok()?;
    let patch = u8::from_str(parts.next()?).ok()?;
    Some((major, minor, patch))
}
