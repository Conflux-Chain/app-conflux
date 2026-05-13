use crate::consts::EXPONENT_SMALLEST_UNIT;
use alloc::string::{String, ToString};
use bigdecimal::{BigDecimal, FromPrimitive};
use core::str::FromStr;

mod transaction;

pub use alloy_primitives::{Address, U256};
pub use transaction::Transaction;

pub fn address_type(address: &Address) -> u8 {
    address.0[0] & 0xf0
}

pub fn is_user_address(address: &Address) -> bool {
    address_type(address) == 0x10
}

pub fn cfx_str(u256: &U256) -> Option<String> {
    let wei_str = u256.to_string();
    let wei = BigDecimal::from_str(&wei_str).ok()?;
    let eth_conversion = BigDecimal::from_i64(10_i64.pow(EXPONENT_SMALLEST_UNIT as u32))?;
    Some((wei / eth_conversion).to_string())
}
