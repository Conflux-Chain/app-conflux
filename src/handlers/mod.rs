use crate::{bip32_path::Bip32Path, crypto::decode_der_sig, AppSW};
use ledger_device_sdk::{
    ecc::{Secp256k1, SeedDerive},
    hash::{sha3::Keccak256, HashInit},
    io::Comm,
};

pub mod common;
pub mod get_public_key;
pub mod get_version;
pub mod sign_191;
pub mod sign_712;
pub mod sign_tx;

// compute hash, sign, and write sig to common
pub fn hash_sign_and_send(comm: &mut Comm, path: &Bip32Path, raw_data: &[u8]) -> Result<(), AppSW> {
    let mut keccak256 = Keccak256::new();
    let mut message_hash: [u8; 32] = [0u8; 32];
    let _ = keccak256.hash(raw_data, &mut message_hash);

    sign_and_send(comm, path, &message_hash)
}

pub fn sign_and_send(comm: &mut Comm, path: &Bip32Path, message_hash: &[u8]) -> Result<(), AppSW> {
    let (sig, siglen, parity) = Secp256k1::derive_from_path(path.as_ref())
        .deterministic_sign(message_hash)
        .map_err(|_| AppSW::TxSignFail)?;

    let mut r: [u8; 32] = [0u8; 32];
    let mut s: [u8; 32] = [0u8; 32];

    decode_der_sig(&sig[..siglen as usize], &mut r, &mut s).map_err(|_| AppSW::TxSignFail)?;

    comm.append(&[parity as u8]);
    comm.append(&r);
    comm.append(&s);

    Ok(())
}
