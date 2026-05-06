use crate::consts::{ADDRRESS_BYTES_LEN, EXPONENT_SMALLEST_UNIT, HASH_BYTES_LEN};
use alloc::string::{String, ToString};
use bigdecimal::{BigDecimal, FromPrimitive};
use core::{cmp::Ordering, ops::Deref, str::FromStr};
use rlp_decoder::{Decodable, DecoderError, Rlp};
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

impl U256 {
    pub fn cfx_str(&self) -> Option<String> {
        let wei_str = self.to_string();
        let wei = BigDecimal::from_str(&wei_str).ok()?;
        let eth_conversion = BigDecimal::from_i64(10_i64.pow(EXPONENT_SMALLEST_UNIT as u32))?;
        Some((wei / eth_conversion).to_string())
    }
}

impl Decodable for U256 {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder().decode_value(|bytes| {
            if !bytes.is_empty() && bytes[0] == 0 {
                Err(DecoderError::RlpInvalidIndirection)
            } else if bytes.len() <= 32 {
                Ok(U256::from_big_endian(bytes))
            } else {
                Err(DecoderError::RlpIsTooBig)
            }
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct H256(pub [u8; HASH_BYTES_LEN]);

impl Decodable for H256 {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder()
            .decode_value(|bytes| match bytes.len().cmp(&(HASH_BYTES_LEN)) {
                Ordering::Less => Err(DecoderError::RlpIsTooShort),
                Ordering::Greater => Err(DecoderError::RlpIsTooBig),
                Ordering::Equal => {
                    let mut t = [0u8; HASH_BYTES_LEN];
                    t.copy_from_slice(bytes);
                    Ok(H256(t))
                }
            })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Address(pub [u8; ADDRRESS_BYTES_LEN]);

impl Address {
    pub fn address_type(&self) -> u8 {
        self.0[0] & 0xf0
    }

    pub fn is_user_address(&self) -> bool {
        self.address_type() == 0x10
    }

    #[allow(unused)]
    pub fn is_contract_address(&self) -> bool {
        self.address_type() == 0x80
    }

    #[allow(unused)]
    pub fn is_builtin_address(&self) -> bool {
        self.address_type() == 0x00
    }
}

impl Decodable for Address {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder()
            .decode_value(|bytes| match bytes.len().cmp(&(ADDRRESS_BYTES_LEN)) {
                Ordering::Less => Err(DecoderError::RlpIsTooShort),
                Ordering::Greater => Err(DecoderError::RlpIsTooBig),
                Ordering::Equal => {
                    let mut t = [0u8; ADDRRESS_BYTES_LEN];
                    t.copy_from_slice(bytes);
                    Ok(Address(t))
                }
            })
    }
}

impl Deref for Address {
    type Target = [u8; ADDRRESS_BYTES_LEN];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
