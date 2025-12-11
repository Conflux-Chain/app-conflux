import pytest

from application_client.command_sender import ConfluxCommandSender
from application_client.response_unpacker import unpack_get_public_key_response, unpack_vrs_response
from utils import check_rs_cfx_prefix_msg_signature_validity
from ragger.navigator import NavIns, NavInsID

def test_sign_191_message(backend, scenario_navigator, device, navigator):
    # Use the app interface instead of raw interface
    client = ConfluxCommandSender(backend)
    # The path used for this entire test
    path: str = "m/44'/503'/0'/0/0"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    _, public_key, _, _ = unpack_get_public_key_response(rapdu.data)

    message = b"Hello, World!"
    
    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.personal_sign(path=path, data=message):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve()

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    _, sig, _ = unpack_vrs_response(response)
    assert check_rs_cfx_prefix_msg_signature_validity(public_key, sig, message)