use ledger_device_sdk::io::{Reply, StatusWords};

// Application status words.
#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AppSW {
    Deny = 0x6985,
    WrongP1P2 = 0x6A86,
    InsNotSupported = 0x6D00,
    ClaNotSupported = 0x6E00,
    TxDisplayFail = 0xB001,
    AddrDisplayFail = 0xB002,
    AmountDisplayFail = 0xB003,
    TxWrongLength = 0xB004,
    TxParsingFail = 0xB005,
    TxHashFail = 0xB006,
    BadState = 0xB007,
    TxSignFail = 0xB008,
    KeyDeriveFail = 0xB009,
    VersionParsingFail = 0xB00A,
    WrongApduLength = StatusWords::BadLen as u16,
    Ok = 0x9000,
    //
    InvalidData = 0x6A80,
    InvalidString = 0x6A81,
    WrongDataLength = 0x6A87,
    WrongResponseLength = 0xB000,
    InternalError = 0x6F01,
}

impl From<AppSW> for Reply {
    fn from(sw: AppSW) -> Reply {
        Reply(sw as u16)
    }
}

// To keep consistency with c version app-conflux
#[allow(dead_code)]
const APP_SW_CIP37_CONVERSION_FAIL: u16 = 0xB008;
#[allow(dead_code)]
const APP_SW_DISPLAY_BIP32_PATH_FAIL: u16 = 0xB001;
