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
use include_gif::include_gif;
use ledger_device_sdk::io::Comm;

#[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
use ledger_device_sdk::ui::{
    bitmaps::{Glyph, BACK, CERTIFICATE, COGGLE, DASHBOARD_X},
    gadgets::{EventOrPageIndex, MultiPageMenu, Page},
};

use crate::settings::Settings;
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::{NbglGlyph, NbglHomeAndSettings};

#[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
use crate::Instruction;
#[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
use ledger_device_sdk::io::Event;

#[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
fn ui_about_menu(comm: &mut Comm) -> Event<Instruction> {
    let pages = [
        &Page::from((["Conflux Core App", "(c) 2024 Conflux"], true)),
        &Page::from(("Back", &BACK)),
    ];
    loop {
        match MultiPageMenu::new(comm, &pages).show() {
            EventOrPageIndex::Event(e) => return e,
            EventOrPageIndex::Index(1) => return ui_menu_main(comm),
            EventOrPageIndex::Index(_) => (),
        }
    }
}

#[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
fn ui_setting_menu(comm: &mut Comm) -> Event<Instruction> {
    let blind_signing_index = 0;

    let settings: Settings = Default::default();
    let mut bs_enabled: bool = settings.get_element(blind_signing_index).unwrap() != 0;
    let mut bs_status = if bs_enabled { "Enabled" } else { "Disabled" };

    loop {
        let pages = [
            &Page::from((["Blind Signing", bs_status], true)),
            &Page::from(("Back", &BACK)),
        ];
        match MultiPageMenu::new(comm, &pages).show() {
            EventOrPageIndex::Event(e) => return e,
            EventOrPageIndex::Index(0) => {
                bs_enabled = !bs_enabled;
                match bs_enabled {
                    true => {
                        settings
                            .set_element(blind_signing_index, 1)
                            .expect("should success");
                        bs_status = "Enabled";
                    }
                    false => {
                        settings
                            .set_element(blind_signing_index, 0)
                            .expect("should success");
                        bs_status = "Disabled";
                    }
                }
            }
            EventOrPageIndex::Index(1) => return ui_menu_main(comm),
            EventOrPageIndex::Index(_) => (),
        }
    }
}

#[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
pub fn ui_menu_main(comm: &mut Comm) -> Event<Instruction> {
    const APP_ICON: Glyph = Glyph::from_include(include_gif!("icons/cfx_16.gif"));
    let pages = [
        // The from trait allows to create different styles of pages
        // without having to use the new() function.
        &Page::from((["Conflux", "is ready"], &APP_ICON)),
        &Page::from((["Version", env!("CARGO_PKG_VERSION")], true)),
        &Page::from(("Settings", &COGGLE)),
        &Page::from(("About", &CERTIFICATE)),
        &Page::from(("Quit", &DASHBOARD_X)),
    ];
    loop {
        match MultiPageMenu::new(comm, &pages).show() {
            EventOrPageIndex::Event(e) => return e,
            EventOrPageIndex::Index(2) => return ui_setting_menu(comm),
            EventOrPageIndex::Index(3) => return ui_about_menu(comm),
            EventOrPageIndex::Index(4) => ledger_device_sdk::exit_app(0),
            EventOrPageIndex::Index(_) => (),
        }
    }
}

#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
pub fn ui_menu_main(_: &mut Comm) -> NbglHomeAndSettings {
    // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    const CFX: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/cfx_64.gif", NBGL));
    #[cfg(target_os = "apex_p")]
    const CFX: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/cfx_48.png", NBGL));

    let settings_strings = [
        ["Blind Signing", "Enable transaction blind signing."],
        ["Display Data", "Allow display of transaction data."],
    ];
    let mut settings: Settings = Default::default();

    // Display the home screen.
    NbglHomeAndSettings::new()
        .glyph(&CFX)
        .infos(
            "Conflux",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
        )
        .settings(settings.get_mut(), &settings_strings)
}
