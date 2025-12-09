pub mod address;
pub mod eip712;
pub mod menu;
pub mod sign;
use ledger_device_sdk::{include_gif, nbgl::NbglGlyph};

#[cfg(any(target_os = "stax", target_os = "flex"))]
pub const CFX_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/cfx_64.gif", NBGL));
#[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
pub const CFX_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/cfx_14.gif", NBGL));
#[cfg(target_os = "apex_p")]
pub const CFX_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/cfx_48.png", NBGL));
