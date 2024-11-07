import pytest

from application_client.command_sender import ConfluxCommandSender, Errors
from application_client.response_unpacker import unpack_get_public_key_response
from ragger.bip import calculate_public_key_and_chaincode, CurveChoice
from ragger.error import ExceptionRAPDU
from ragger.navigator import NavInsID, NavIns
from utils import ROOT_SCREENSHOT_PATH


# In this test we check that the GET_PUBLIC_KEY works in non-confirmation mode
def test_get_public_key_no_confirm(backend):
    for path in ["m/503'/1'/0'/0/0", "m/503'/1'/0/0/0", "m/503'/1'/911'/0/0", "m/503'/1'/255/255/255", "m/503'/1'/2147483647/0/0/0/0/0/0/0"]:
        client = ConfluxCommandSender(backend)
        response = client.get_public_key(path=path).data
        _, public_key, _, _ = unpack_get_public_key_response(response)

        ref_public_key, _ = calculate_public_key_and_chaincode(CurveChoice.Secp256k1, path=path)
        assert public_key.hex() == ref_public_key


# In this test we check that the GET_PUBLIC_KEY works in confirmation mode
# 0xFD2095A37E72BE2CD575D18FE8F16E78C51EAFA3  eth address
# 0x1D2095A37E72BE2CD575D18FE8F16E78C51EAFA3  cfx core hex address
# cfx:aaswbfrdt33n6ngzs1j294hvr36pmhztypumh8c62t cfx core base32 address
def test_get_public_key_confirm_accepted(backend, scenario_navigator):
    client = ConfluxCommandSender(backend)
    path = "m/503'/1'/0'/0/0"
    
    with client.get_public_key_with_confirmation(path=path, chain_id=1029):
        scenario_navigator.address_review_approve()
        
    response = client.get_async_response().data
    _, public_key, _, _ = unpack_get_public_key_response(response)

    ref_public_key, _ = calculate_public_key_and_chaincode(CurveChoice.Secp256k1, path=path)
    assert public_key.hex() == ref_public_key


# In this test we check that the GET_PUBLIC_KEY in confirmation mode replies an error if the user refuses
def test_get_public_key_confirm_refused(backend, scenario_navigator):
    client = ConfluxCommandSender(backend)
    path = "m/503'/1'/0'/0/0"

    with pytest.raises(ExceptionRAPDU) as e:
        with client.get_public_key_with_confirmation(path=path, chain_id=1029):
            scenario_navigator.address_review_reject()

    # Assert that we have received a refusal
    assert e.value.status == Errors.SW_DENY
    assert len(e.value.data) == 0
