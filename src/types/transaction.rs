use crate::{
    consts::{ONE_CFX_IN_DRIP, STORAGE_OF_ONE_CFX},
    types::is_user_address,
};
use alloc::vec::Vec;
use alloy_primitives::{Address, B256, U256};
use alloy_rlp::{
    bytes::{Buf, Bytes},
    Decodable, Error, Header, RlpDecodable,
};

pub const TX_RLP_PREFIX_2930: [u8; 4] = [0x63, 0x66, 0x78, 0x01]; // "cfx" + 1
pub const TX_RLP_PREFIX_1559: [u8; 4] = [0x63, 0x66, 0x78, 0x02]; // "cfx" + 2

#[derive(Debug, Clone, Default, PartialEq, Eq, RlpDecodable)]
pub struct AccessListItem {
    pub address: Address,
    pub storage_keys: Vec<B256>,
}

pub type AccessList = Vec<AccessListItem>;

#[allow(unused)]
#[derive(Debug, Default, Clone)]
pub struct Transaction {
    pub to: Address,
    pub value: U256,
    pub nonce: u64,
    pub data: Vec<u8>,
    pub gas: u64,
    pub gas_price: Option<U256>,
    pub storage_limit: u64,
    pub epoch_height: u64,
    pub chain_id: u64,
    pub access_list: Option<AccessList>,
    pub max_priority_fee_per_gas: Option<U256>,
    pub max_fee_per_gas: Option<U256>,
}

impl Transaction {
    pub fn max_gas_fee(&self) -> U256 {
        let gas = U256::from(self.gas);
        if let Some(gas_price) = self.gas_price {
            gas_price * gas
        } else if let Some(max_fee_per_gas) = self.max_fee_per_gas {
            max_fee_per_gas * gas
        } else {
            U256::ZERO
        }
    }

    pub fn max_storage_fee(&self) -> U256 {
        U256::from(self.storage_limit) * U256::from(ONE_CFX_IN_DRIP)
            / U256::from(STORAGE_OF_ONE_CFX)
    }

    // whether the tx is fully decoded
    // when the tx is to a contract address, the data field is not empty
    // we call it not fully decoded
    pub fn fully_decoded(&self) -> bool {
        self.data.is_empty() || is_user_address(&self.to)
    }
}

impl Decodable for Transaction {
    fn decode(data: &mut &[u8]) -> Result<Self, Error> {
        let first4_bytes: [u8; 4] = match data.get(0..4) {
            Some(bytes) => bytes.try_into().unwrap(),
            None => [0; 4],
        };

        let tx = match first4_bytes {
            TX_RLP_PREFIX_2930 => {
                data.advance(4);
                let mut data = Header::decode_bytes(data, true)?;
                Transaction {
                    nonce: u64::decode(&mut data)?,
                    gas_price: Some(U256::decode(&mut data)?),
                    gas: u64::decode(&mut data)?,
                    to: Address::decode(&mut data)?,
                    value: U256::decode(&mut data)?,
                    storage_limit: u64::decode(&mut data)?,
                    epoch_height: u64::decode(&mut data)?,
                    chain_id: u64::decode(&mut data)?,
                    data: Bytes::decode(&mut data)?.to_vec(),
                    access_list: Some(AccessList::decode(&mut data)?),
                    max_priority_fee_per_gas: None,
                    max_fee_per_gas: None,
                }
            }
            TX_RLP_PREFIX_1559 => {
                data.advance(4);
                let mut data = Header::decode_bytes(data, true)?;
                Transaction {
                    nonce: u64::decode(&mut data)?,
                    max_priority_fee_per_gas: Some(U256::decode(&mut data)?),
                    max_fee_per_gas: Some(U256::decode(&mut data)?),
                    gas: u64::decode(&mut data)?,
                    to: Address::decode(&mut data)?,
                    value: U256::decode(&mut data)?,
                    storage_limit: u64::decode(&mut data)?,
                    epoch_height: u64::decode(&mut data)?,
                    chain_id: u64::decode(&mut data)?,
                    data: Bytes::decode(&mut data)?.to_vec(),
                    access_list: Some(AccessList::decode(&mut data)?),
                    gas_price: None,
                }
            }
            _ => {
                let mut data = Header::decode_bytes(data, true)?;
                Transaction {
                    nonce: u64::decode(&mut data)?,
                    gas_price: Some(U256::decode(&mut data)?),
                    gas: u64::decode(&mut data)?,
                    to: Address::decode(&mut data)?,
                    value: U256::decode(&mut data)?,
                    storage_limit: u64::decode(&mut data)?,
                    epoch_height: u64::decode(&mut data)?,
                    chain_id: u64::decode(&mut data)?,
                    data: Bytes::decode(&mut data)?.to_vec(),
                    access_list: None,
                    max_priority_fee_per_gas: None,
                    max_fee_per_gas: None,
                }
            }
        };
        Ok(tx)
    }
}
