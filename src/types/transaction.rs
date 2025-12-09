use super::{Address, H256, U256};
use crate::consts::STORAGE_OF_ONE_CFX;
use alloc::vec::Vec;
use rlp_decoder::{Decodable, DecoderError, Rlp};

pub const TX_RLP_PREFIX_2930: [u8; 4] = [0x63, 0x66, 0x78, 0x01]; // "cfx" + 1
pub const TX_RLP_PREFIX_1559: [u8; 4] = [0x63, 0x66, 0x78, 0x02]; // "cfx" + 2
pub const ONE_CFX_IN_DRIP: u64 = 1_000_000_000_000_000_000;

#[allow(dead_code)]
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
        if self.gas_price.is_some() {
            self.gas_price.unwrap() * U256::from(self.gas)
        } else {
            self.max_fee_per_gas.unwrap() * U256::from(self.gas)
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
        self.data.is_empty() || self.to.is_user_address()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AccessListItem {
    pub address: Address,
    pub storage_keys: Vec<H256>,
}

impl Decodable for AccessListItem {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(Self {
            address: rlp.val_at(0)?,
            storage_keys: rlp.list_at(1)?,
        })
    }
}

pub type AccessList = Vec<AccessListItem>;

pub const TX_LEGACY_RLP_LEN: usize = 9;
pub const TX_EIP2930_RLP_LEN: usize = 10;
pub const TX_EIP1559_RLP_LEN: usize = 11;

impl Decodable for Transaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if rlp.as_raw().is_empty() {
            return Err(DecoderError::RlpInvalidLength);
        };
        if rlp.is_list() {
            if rlp.item_count()? != TX_LEGACY_RLP_LEN {
                return Err(DecoderError::RlpInvalidLength);
            }
            Ok(Transaction {
                nonce: rlp.val_at(0)?,
                gas_price: Some(rlp.val_at(1)?),
                gas: rlp.val_at(2)?,
                to: rlp.val_at(3)?,
                value: rlp.val_at(4)?,
                storage_limit: rlp.val_at(5)?,
                epoch_height: rlp.val_at(6)?,
                chain_id: rlp.val_at(7)?,
                data: rlp.val_at(8)?,
                access_list: None,
                max_priority_fee_per_gas: None,
                max_fee_per_gas: None,
            })
        } else {
            let data = rlp.as_raw();
            let first4_bytes: [u8; 4] = match data.get(0..4) {
                Some(bytes) => bytes.try_into().unwrap(),
                None => [0; 4],
            };
            let rlp = Rlp::new(&data[4..]);
            match first4_bytes {
                TX_RLP_PREFIX_2930 => {
                    if rlp.item_count()? != TX_EIP2930_RLP_LEN {
                        return Err(DecoderError::RlpInvalidLength);
                    }
                    Ok(Transaction {
                        nonce: rlp.val_at(0)?,
                        gas_price: Some(rlp.val_at(1)?),
                        gas: rlp.val_at(2)?,
                        to: rlp.val_at(3)?,
                        value: rlp.val_at(4)?,
                        storage_limit: rlp.val_at(5)?,
                        epoch_height: rlp.val_at(6)?,
                        chain_id: rlp.val_at(7)?,
                        data: rlp.val_at(8)?,
                        access_list: Some(rlp.list_at(9)?),
                        max_priority_fee_per_gas: None,
                        max_fee_per_gas: None,
                    })
                }
                TX_RLP_PREFIX_1559 => {
                    if rlp.item_count()? != TX_EIP1559_RLP_LEN {
                        return Err(DecoderError::RlpInvalidLength);
                    }
                    Ok(Transaction {
                        nonce: rlp.val_at(0)?,
                        gas_price: None,
                        max_priority_fee_per_gas: Some(rlp.val_at(1)?),
                        max_fee_per_gas: Some(rlp.val_at(2)?),
                        gas: rlp.val_at(3)?,
                        to: rlp.val_at(4)?,
                        value: rlp.val_at(5)?,
                        storage_limit: rlp.val_at(6)?,
                        epoch_height: rlp.val_at(7)?,
                        chain_id: rlp.val_at(8)?,
                        data: rlp.val_at(9)?,
                        access_list: Some(rlp.list_at(10)?),
                    })
                }
                _ => Err(DecoderError::Custom("invalid raw transaction")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex::{FromHex, ToHex};
    use rlp::{decode, encode};

    #[test]
    fn decode_basic() {
        let to = Address::from_slice(
            hex::decode("0123456789012345678901234567890123456789")
                .unwrap()
                .as_slice(),
        );
        let mut tx = Transaction {
            to,
            value: U256::from(1),
            nonce: 1,
            data: Bytes::from(""),
            gas: 1,
            gas_price: Some(U256::from(1)),
            storage_limit: 1,
            epoch_height: 1,
            chain_id: 1,
            access_list: None,
            max_priority_fee_per_gas: None,
            max_fee_per_gas: None,
        };
        // let out = encode(&tx);
        let out = "dd0101019401234567890123456789012345678901234567890101010180"
            .from_hex::<Vec<u8>>()
            .unwrap();
        let decode_tx: Transaction = decode(&out).expect("decode should success");
        assert_eq!(tx.value, decode_tx.value);
        assert_eq!(tx.nonce, decode_tx.nonce);
        assert_eq!(tx.gas, decode_tx.gas);
        assert_eq!(tx.gas_price, decode_tx.gas_price);
        assert_eq!(tx.storage_limit, decode_tx.storage_limit);
        assert_eq!(tx.epoch_height, decode_tx.epoch_height);
        assert_eq!(tx.chain_id, decode_tx.chain_id);
        assert_eq!(tx.data, decode_tx.data);
        assert_eq!(tx.to, decode_tx.to);
        assert_eq!(tx.access_list.is_none(), decode_tx.access_list.is_none());
        assert_eq!(
            tx.max_priority_fee_per_gas,
            decode_tx.max_priority_fee_per_gas
        );
        assert_eq!(tx.max_fee_per_gas, decode_tx.max_fee_per_gas);

        tx.data = Bytes::from("hello");
        let out = "e2010101940123456789012345678901234567890123456789010101018568656c6c6f"
            .from_hex::<Vec<u8>>()
            .unwrap();
        let decode_tx: Transaction = decode(&out).expect("decode should success");
        assert_eq!(tx.data, decode_tx.data);
    }

    #[test]
    fn decode_2930() {
        let to = Address::from_slice(
            hex::decode("0123456789012345678901234567890123456789")
                .unwrap()
                .as_slice(),
        );
        let mut tx = Transaction {
            to,
            value: U256::from(1),
            nonce: 1,
            data: Bytes::from("hello"),
            gas: 1,
            gas_price: Some(U256::from(1)),
            storage_limit: 1,
            epoch_height: 1,
            chain_id: 1,
            access_list: Some(vec![]),
            max_priority_fee_per_gas: None,
            max_fee_per_gas: None,
        };
        // let out = encode(&tx);
        let out =
            "63667801e3010101940123456789012345678901234567890123456789010101018568656c6c6fc0"
                .from_hex::<Vec<u8>>()
                .unwrap();
        let decode_tx: Transaction = decode(&out).expect("decode should success");
        assert_eq!(tx.value, decode_tx.value);
        assert_eq!(tx.nonce, decode_tx.nonce);
        assert_eq!(tx.gas, decode_tx.gas);
        assert_eq!(tx.gas_price, decode_tx.gas_price);
        assert_eq!(tx.storage_limit, decode_tx.storage_limit);
        assert_eq!(tx.epoch_height, decode_tx.epoch_height);
        assert_eq!(tx.chain_id, decode_tx.chain_id);
        assert_eq!(tx.data, decode_tx.data);
        assert_eq!(tx.to, decode_tx.to);
        assert_eq!(tx.access_list.is_none(), decode_tx.access_list.is_none());
        assert_eq!(
            tx.max_priority_fee_per_gas,
            decode_tx.max_priority_fee_per_gas
        );
        assert_eq!(tx.max_fee_per_gas, decode_tx.max_fee_per_gas);

        tx.access_list = Some(vec![AccessListItem {
            address: to,
            storage_keys: vec![H256::from_slice(
                hex::decode("3d709d64e3b668ddc615a5b05d6f109275096d27571d99ba02d28e84feac6b00")
                    .unwrap()
                    .as_slice(),
            )],
        }]);
        let out = "63667801f85c010101940123456789012345678901234567890123456789010101018568656c6c6ff838f7940123456789012345678901234567890123456789e1a03d709d64e3b668ddc615a5b05d6f109275096d27571d99ba02d28e84feac6b00".from_hex::<Vec<u8>>().unwrap();
        let decode_tx: Transaction = decode(&out).expect("decode should success");
        assert_eq!(
            tx.access_list.as_ref().expect("")[0].address,
            decode_tx.access_list.as_ref().expect("")[0].address
        );
        assert_eq!(
            tx.access_list.expect("")[0].storage_keys,
            decode_tx.access_list.expect("")[0].storage_keys
        );
    }

    #[test]
    fn decode_1559() {
        let to = Address::from_slice(
            hex::decode("0123456789012345678901234567890123456789")
                .unwrap()
                .as_slice(),
        );
        let mut tx = Transaction {
            to,
            value: U256::from(1),
            nonce: 1,
            data: Bytes::from("hello"),
            gas: 1,
            gas_price: None,
            storage_limit: 1,
            epoch_height: 1,
            chain_id: 1,
            access_list: None,
            max_priority_fee_per_gas: Some(U256::from(1)),
            max_fee_per_gas: Some(U256::from(1)),
        };
        // let out = encode(&tx);
        let out =
            "63667802e401010101940123456789012345678901234567890123456789010101018568656c6c6fc0"
                .from_hex::<Vec<u8>>()
                .unwrap();
        let decode_tx: Transaction = decode(&out).expect("decode should success");
        assert_eq!(tx.value, decode_tx.value);
        assert_eq!(tx.nonce, decode_tx.nonce);
        assert_eq!(tx.gas, decode_tx.gas);
        assert_eq!(tx.gas_price, decode_tx.gas_price);
        assert_eq!(tx.storage_limit, decode_tx.storage_limit);
        assert_eq!(tx.epoch_height, decode_tx.epoch_height);
        assert_eq!(tx.chain_id, decode_tx.chain_id);
        assert_eq!(tx.data, decode_tx.data);
        assert_eq!(tx.to, decode_tx.to);
        // assert_eq!(tx.access_list.is_none(), decode_tx.access_list.is_none());
        assert_eq!(decode_tx.access_list.unwrap().len(), 0);
        assert_eq!(
            tx.max_priority_fee_per_gas,
            decode_tx.max_priority_fee_per_gas
        );
        assert_eq!(tx.max_fee_per_gas, decode_tx.max_fee_per_gas);

        tx.access_list = Some(vec![AccessListItem {
            address: to,
            storage_keys: vec![H256::from_slice(
                hex::decode("3d709d64e3b668ddc615a5b05d6f109275096d27571d99ba02d28e84feac6b00")
                    .unwrap()
                    .as_slice(),
            )],
        }]);
        let out = "63667802f85d01010101940123456789012345678901234567890123456789010101018568656c6c6ff838f7940123456789012345678901234567890123456789e1a03d709d64e3b668ddc615a5b05d6f109275096d27571d99ba02d28e84feac6b00".from_hex::<Vec<u8>>().unwrap();
        let decode_tx: Transaction = decode(&out).expect("decode should success");
        assert_eq!(tx.data, decode_tx.data);
        assert_eq!(
            tx.access_list.as_ref().expect("")[0].address,
            decode_tx.access_list.as_ref().expect("")[0].address
        );
        assert_eq!(
            tx.access_list.expect("")[0].storage_keys,
            decode_tx.access_list.expect("")[0].storage_keys
        );
    }
}
