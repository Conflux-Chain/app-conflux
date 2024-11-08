from pathlib import Path
from hashlib import sha256
from sha3 import keccak_256

from ecdsa.curves import SECP256k1
from ecdsa.keys import VerifyingKey
from ecdsa.util import sigdecode_der, sigdecode_string


ROOT_SCREENSHOT_PATH = Path(__file__).parent.resolve()

PREFIX = b"\x19Conflux Signed Message:\n"

# Check if a des signature of a given message is valid
def check_signature_validity(public_key: bytes, signature: bytes, message: bytes) -> bool:
    pk: VerifyingKey = VerifyingKey.from_string(
        public_key,
        curve=SECP256k1,
        hashfunc=sha256
    )
    return pk.verify(signature=signature,
                     data=message,
                     hashfunc=keccak_256,
                     sigdecode=sigdecode_der)

def check_rs_signature_validity(public_key: bytes, signature: bytes, message: bytes) -> bool:
    pk: VerifyingKey = VerifyingKey.from_string(
        public_key,
        curve=SECP256k1,
        hashfunc=sha256
    )
    return pk.verify(signature=signature,
                     data=message,
                     hashfunc=keccak_256,
                     sigdecode=sigdecode_string)


# Check if a signature of a given message is valid
# signature is r+s (not der encoded)
def check_rs_prefix_msg_signature_validity(public_key: bytes, signature: bytes, message: bytes) -> bool:
    pk: VerifyingKey = VerifyingKey.from_string(
        public_key,
        curve=SECP256k1,
        hashfunc=sha256
    )
    message = PREFIX + str(len(message)).encode() + message
    return pk.verify(signature=signature,
                     data=message,
                     hashfunc=keccak_256,
                     sigdecode=sigdecode_string)
