from eth_keys import keys

def signature(data: bytes) -> tuple[int, int, int]:
    assert len(data) == (1 + 32 + 32)

    v = int.from_bytes(data[0:1], "big")
    data = data[1:]
    r = int.from_bytes(data[0:32], "big")
    data = data[32:]
    s = int.from_bytes(data[0:32], "big")

    return v, r, s

def pk_addr(data: bytes, has_chaincode: bool = False):
    idx = 0

    if len(data) < (idx + 1):
        return None
    pk_len = data[idx]
    idx += 1

    if len(data) < (idx + pk_len):
        return None
    pk = data[idx:idx + pk_len]
    idx += pk_len


    # normalize public key bytes (handle uncompressed 0x04 prefix)
    if len(pk) == 65 and pk[0] == 0x04:
        pub_key_bytes = pk[1:]
    else:
        pub_key_bytes = pk

    # expect 64-byte raw public key
    if len(pub_key_bytes) != 64:
        return None

    pub_key = keys.PublicKey(pub_key_bytes)
    # to_checksum_address returns "0x..." string; strip "0x" and store as bytes
    addr = pub_key.to_checksum_address()[2:].encode() # pylint: disable=unsubscriptable-object

    if has_chaincode:
        idx += 1
        if len(data) < (idx + 32):
            return None
        chaincode = data[idx:idx + 32]
        idx += 32
    else:
        chaincode = None

    if idx != len(data):
        return None

    return pk, bytes.fromhex(addr.decode()), chaincode
