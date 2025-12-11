#![allow(unused)]
pub const CLA: u8 = 0xe0;

pub mod ins {
    /// GET VERSION
    pub const GET_VERSION: u8 = 0x01;
    /// GET PUBLIC ADDRESS
    pub const GET_PUBLIC_ADDRESS: u8 = 0x02;
    /// SIGN TRANSACTION
    pub const SIGN_TRANSACTION: u8 = 0x03;
    /// SIGN PERSONAL MESSAGE
    pub const SIGN_PERSONAL_MESSAGE: u8 = 0x04; // this is cip23 personal message
    /// GET APP NAME
    pub const GET_APP_NAME: u8 = 0x05;
    // GET APP CONFIGURATION
    pub const GET_APP_CONFIGURATION: u8 = 0x06;
    /// SIGN PERSONAL MESSAGE
    pub const SIGN_ETH_PERSONAL_MESSAGE: u8 = 0x07;
    /// SIGN EIP 712
    pub const SIGN_EIP712: u8 = 0x0A;
    /// EIP712 SEND STRUCT DEFINITION
    pub const EIP712_SEND_STRUCT_DEFINITION: u8 = 0x0B;
    /// EIP712 SEND STRUCT IMPLEMENTATION
    pub const EIP712_SEND_STRUCT_IMPLEMENTATION: u8 = 0x0C;
    /// EIP712 FILTERING
    pub const EIP712_FILTERING: u8 = 0x0D;
}

pub mod p1_sign_tx {
    // P1 for first APDU number.
    pub const P1_SIGN_TX_START: u8 = 0x00;
    // P1 for maximum APDU number.
    pub const P1_SIGN_TX_MAX: u8 = 0x03;
}

pub mod p1_personal_msg {
    // P1 for first APDU number.
    pub const P1_SIGN_MSG_START: u8 = 0x00;
    // P1 for maximum APDU number.
    pub const P1_SIGN_MSG_MAX: u8 = 0x20;
}

pub mod p2_sign_tx {
    // P2 for last APDU to receive.
    pub const P2_SIGN_TX_LAST: u8 = 0x00;
    // P2 for more APDU to receive.
    pub const P2_SIGN_TX_MORE: u8 = 0x80;
}

// P1 parameter constants for SIGN ETH EIP 712
// pub mod p1_sign_eip712 {
//     /// First chunk
//     pub const FIRST_CHUNK: u8 = 0x00;
//     /// Following chunk
//     pub const FOLLOWING_CHUNK: u8 = 0x01;
// }

/// P2 parameter constants for SIGN ETH EIP 712
pub mod p2_sign_eip712 {
    /// v0 implementation (domain hash + message hash)
    // pub const V0_IMPLEMENTATION: u8 = 0x00; // We don't support this version
    /// Full implementation (complete JSON processing)
    pub const FULL_IMPLEMENTATION: u8 = 0x01;
}

/// P2 parameter constants for EIP712 SEND STRUCT DEFINITION
pub mod p2_eip712_struct_def {
    /// Struct name
    pub const STRUCT_NAME: u8 = 0x00;
    /// Struct field
    pub const STRUCT_FIELD: u8 = 0xFF;
}

/// P1 parameter constants for EIP712 SEND STRUCT IMPLEMENTATION
pub mod p1_eip712_struct_impl {
    /// Complete send
    pub const COMPLETE_SEND: u8 = 0x00;
    /// Partial send, more to come
    pub const PARTIAL_SEND: u8 = 0x01;
}

/// P2 parameter constants for EIP712 SEND STRUCT IMPLEMENTATION
pub mod p2_eip712_struct_impl {
    /// Root struct
    pub const ROOT_STRUCT: u8 = 0x00;
    /// Array
    pub const ARRAY: u8 = 0x0F;
    /// Struct field
    pub const STRUCT_FIELD: u8 = 0xFF;
}

/// P1 parameter constants for EIP712 FILTERING
#[allow(unused)]
pub mod p1_eip712_filtering {
    /// Standard
    pub const STANDARD: u8 = 0x00;
    /// Discarded
    pub const DISCARDED: u8 = 0x01;
}

/// P2 parameter constants for EIP712 FILTERING
#[allow(unused)]
pub mod p2_eip712_filtering {
    /// Activation
    pub const ACTIVATION: u8 = 0x00;
    /// Discarded filter path
    pub const DISCARDED_FILTER_PATH: u8 = 0x01;
    /// Message info
    pub const MESSAGE_INFO: u8 = 0x0F;
    /// Trusted name
    pub const TRUSTED_NAME: u8 = 0xFB;
    /// Date/time
    pub const DATE_TIME: u8 = 0xFC;
    /// Amount-join token
    pub const AMOUNT_JOIN_TOKEN: u8 = 0xFD;
    /// Amount-join value
    pub const AMOUNT_JOIN_VALUE: u8 = 0xFE;
    /// Raw field
    pub const RAW_FIELD: u8 = 0xFF;
}
