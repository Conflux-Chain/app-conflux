use k256::ecdsa::Signature;

pub fn decode_der_sig(
    der_sig: &[u8],
    r_bytes: &mut [u8; 32],
    s_bytes: &mut [u8; 32],
) -> Result<(), &'static str> {
    let sig = Signature::from_der(der_sig).map_err(|_e| "der_sig decode failed")?;
    r_bytes.copy_from_slice(&sig.r().to_bytes());
    s_bytes.copy_from_slice(&sig.s().to_bytes());
    Ok(())
}
