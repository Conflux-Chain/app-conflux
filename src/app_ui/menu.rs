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
use super::CFX_ICON;
use crate::settings::Settings;
use ledger_device_sdk::io::Comm;
use ledger_device_sdk::nbgl::NbglHomeAndSettings;

pub fn ui_menu_main(_: &mut Comm) -> NbglHomeAndSettings {
    let settings_strings = [
        ["Blind Signing", "Enable transaction blind signing."],
        ["Display Data", "Allow display of transaction data."],
    ];
    let mut settings: Settings = Default::default();

    // Display the home screen.
    NbglHomeAndSettings::new()
        .glyph(&CFX_ICON)
        .infos(
            "Conflux",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
        )
        .settings(settings.get_mut(), &settings_strings)
}
