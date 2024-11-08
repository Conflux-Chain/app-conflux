import pytest

from application_client.transaction import Transaction
from application_client.command_sender import ConfluxCommandSender, Errors
from application_client.response_unpacker import unpack_get_public_key_response, unpack_vrs_response
from ragger.error import ExceptionRAPDU
from ragger.navigator import NavIns, NavInsID
from utils import ROOT_SCREENSHOT_PATH, check_rs_signature_validity, check_rs_prefix_msg_signature_validity
from web3 import Web3
from cfx_address import Base32Address

# cfx:aajwgvnhveawgvnhveawgvnhveawgvnhve8c2ukvxz
# cfxtest:aajwgvnhveawgvnhveawgvnhveawgvnhveyknat519
TARGET_ADDRESS = "0x1123456789012345678901234567890123456789"

# In these tests we check the behavior of the device when asked to sign a transaction

# In this test a transaction is sent to the device to be signed and validated on screen.
# The transaction is short and will be sent in one chunk.
# We will ensure that the displayed information is correct by using screenshots comparison.
def test_sign_tx_short_tx(backend, scenario_navigator, firmware, navigator):
    # Use the app interface instead of raw interface
    client = ConfluxCommandSender(backend)
    # The path used for this entire test
    path: str = "m/503'/1'/0'/0/0"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    _, public_key, _, _ = unpack_get_public_key_response(rapdu.data)

    # Create the transaction that will be sent to the device for signing
    transaction = Transaction(
        to=Base32Address(TARGET_ADDRESS, network_id=1029),
        value=Web3.to_wei(1, 'ether'),
        nonce=1,
        gas=1,
        gasPrice=1,
        storageLimit=1,
        epochHeight=1,
        chainId=1029,
        data="hello cfx".encode("utf-8")
    ).serialize()

    # Enable display of transaction data (NBGL devices only)
    if not firmware.device.startswith("nano"):
        navigator.navigate([NavInsID.USE_CASE_HOME_SETTINGS,
                            NavIns(NavInsID.TOUCH, (200, 113)),
                            NavInsID.USE_CASE_SUB_SETTINGS_EXIT],
                            screen_change_before_first_instruction=False,
                            screen_change_after_last_instruction=False)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_tx(path=path, transaction=transaction):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve()

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    _, sig, _ = unpack_vrs_response(response)
    assert check_rs_signature_validity(public_key, sig, transaction)

def test_sign_tx_1gwei(backend, scenario_navigator, firmware, navigator):
    # Use the app interface instead of raw interface
    client = ConfluxCommandSender(backend)
    # The path used for this entire test
    path: str = "m/503'/1'/0'/0/0"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    _, public_key, _, _ = unpack_get_public_key_response(rapdu.data)

    # Create the transaction that will be sent to the device for signing
    transaction = Transaction(
        to=Base32Address(TARGET_ADDRESS, network_id=1029),
        value=Web3.to_wei(1, 'gwei'),
        nonce=1,
        gas=1,
        gasPrice=1,
        storageLimit=1,
        epochHeight=1,
        chainId=1029,
        data="hello cfx".encode("utf-8")
    ).serialize()

    # Enable display of transaction data (NBGL devices only)
    if not firmware.device.startswith("nano"):
        navigator.navigate([NavInsID.USE_CASE_HOME_SETTINGS,
                            NavIns(NavInsID.TOUCH, (200, 113)),
                            NavInsID.USE_CASE_SUB_SETTINGS_EXIT],
                            screen_change_before_first_instruction=False,
                            screen_change_after_last_instruction=False)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_tx(path=path, transaction=transaction):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve()

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    _, sig, _ = unpack_vrs_response(response)
    assert check_rs_signature_validity(public_key, sig, transaction)

def test_sign_tx_very_big_value(backend, scenario_navigator, firmware, navigator):
    # Use the app interface instead of raw interface
    client = ConfluxCommandSender(backend)
    # The path used for this entire test
    path: str = "m/503'/1'/0'/0/0"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    _, public_key, _, _ = unpack_get_public_key_response(rapdu.data)

    # Create the transaction that will be sent to the device for signing
    transaction = Transaction(
        to=Base32Address(TARGET_ADDRESS, network_id=1029),
        value=Web3.to_wei(123456789000.12345, 'ether'),
        nonce=1,
        gas=1,
        gasPrice=1,
        storageLimit=1,
        epochHeight=1,
        chainId=1029,
        data="hello cfx".encode("utf-8")
    ).serialize()

    # Enable display of transaction data (NBGL devices only)
    if not firmware.device.startswith("nano"):
        navigator.navigate([NavInsID.USE_CASE_HOME_SETTINGS,
                            NavIns(NavInsID.TOUCH, (200, 113)),
                            NavInsID.USE_CASE_SUB_SETTINGS_EXIT],
                            screen_change_before_first_instruction=False,
                            screen_change_after_last_instruction=False)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_tx(path=path, transaction=transaction):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve()

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    _, sig, _ = unpack_vrs_response(response)
    assert check_rs_signature_validity(public_key, sig, transaction)

def test_sign_tx_xgwei(backend, scenario_navigator, firmware, navigator):
    # Use the app interface instead of raw interface
    client = ConfluxCommandSender(backend)
    # The path used for this entire test
    path: str = "m/503'/1'/0'/0/0"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    _, public_key, _, _ = unpack_get_public_key_response(rapdu.data)

    # Create the transaction that will be sent to the device for signing
    transaction = Transaction(
        to=Base32Address(TARGET_ADDRESS, network_id=1029),
        value=Web3.to_wei(123456.789, 'gwei'),
        nonce=1,
        gas=1,
        gasPrice=1,
        storageLimit=1,
        epochHeight=1,
        chainId=1029,
        data="hello cfx".encode("utf-8")
    ).serialize()

    # Enable display of transaction data (NBGL devices only)
    if not firmware.device.startswith("nano"):
        navigator.navigate([NavInsID.USE_CASE_HOME_SETTINGS,
                            NavIns(NavInsID.TOUCH, (200, 113)),
                            NavInsID.USE_CASE_SUB_SETTINGS_EXIT],
                            screen_change_before_first_instruction=False,
                            screen_change_after_last_instruction=False)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_tx(path=path, transaction=transaction):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve()

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    _, sig, _ = unpack_vrs_response(response)
    assert check_rs_signature_validity(public_key, sig, transaction)

# In this test a transaction is sent to the device to be signed and validated on screen.
# The transaction is short and will be sent in one chunk.
# We will ensure that the displayed information is correct by using screenshots comparison.
def test_sign_tx_normal_tx(backend, scenario_navigator, firmware, navigator):
    # Use the app interface instead of raw interface
    client = ConfluxCommandSender(backend)
    # The path used for this entire test
    path: str = "m/503'/1'/0'/0/0"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    _, public_key, _, _ = unpack_get_public_key_response(rapdu.data)

    # Create the transaction that will be sent to the device for signing
    transaction = Transaction(
        to=Base32Address(TARGET_ADDRESS, network_id=1029),
        value=Web3.to_wei(12345.6789, 'ether'),
        nonce=1,
        gas=31000,
        gasPrice=Web3.to_wei(10, 'gwei'),
        storageLimit=1000,
        epochHeight=108829345,
        chainId=1029,
        data="hello cfx".encode("utf-8")
    ).serialize()

    # Enable display of transaction data (NBGL devices only)
    if not firmware.device.startswith("nano"):
        navigator.navigate([NavInsID.USE_CASE_HOME_SETTINGS,
                            NavIns(NavInsID.TOUCH, (200, 113)),
                            NavInsID.USE_CASE_SUB_SETTINGS_EXIT],
                            screen_change_before_first_instruction=False,
                            screen_change_after_last_instruction=False)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_tx(path=path, transaction=transaction):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve()

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    _, sig, _ = unpack_vrs_response(response)
    assert check_rs_signature_validity(public_key, sig, transaction)

# In this test a transaction is sent to the device to be signed and validated on screen.
# The transaction is short and will be sent in one chunk.
# We will ensure that the displayed information is correct by using screenshots comparison.
def test_sign_tx_1559_tx(backend, scenario_navigator, firmware, navigator):
    # Use the app interface instead of raw interface
    client = ConfluxCommandSender(backend)
    # The path used for this entire test
    path: str = "m/503'/1'/0'/0/0"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    _, public_key, _, _ = unpack_get_public_key_response(rapdu.data)

    # Create the transaction that will be sent to the device for signing
    transaction = Transaction(
        to=Base32Address(TARGET_ADDRESS, network_id=1029),
        value=Web3.to_wei(12345.6789, 'ether'),
        nonce=1,
        gas=31000,
        maxPriorityFeePerGas=Web3.to_wei(10, 'gwei'),
        maxFeePerGas=Web3.to_wei(20, 'gwei'),
        storageLimit=1000,
        epochHeight=108829345,
        chainId=1029,
        data="hello cfx".encode("utf-8"),
        accessList=[{
            "address": Base32Address(TARGET_ADDRESS, network_id=1029),
            "storageKeys": ["0x3d709d64e3b668ddc615a5b05d6f109275096d27571d99ba02d28e84feac6b00"]
        }],
    ).serialize()

    # Enable display of transaction data (NBGL devices only)
    if not firmware.device.startswith("nano"):
        navigator.navigate([NavInsID.USE_CASE_HOME_SETTINGS,
                            NavIns(NavInsID.TOUCH, (200, 113)),
                            NavInsID.USE_CASE_SUB_SETTINGS_EXIT],
                            screen_change_before_first_instruction=False,
                            screen_change_after_last_instruction=False)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_tx(path=path, transaction=transaction):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve()

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    _, sig, _ = unpack_vrs_response(response)
    assert check_rs_signature_validity(public_key, sig, transaction)
    
# In this test a transaction is sent to the device to be signed and validated on screen.
# The transaction is short and will be sent in one chunk
# We will ensure that the displayed information is correct by using screenshots comparison
# The transaction data should not be displayed as we have not enabled it in the app settings.
def test_sign_tx_short_tx_no_memo(backend, scenario_navigator, firmware):
    if firmware.device.startswith("nano"):
        pytest.skip("Skipping this test for Nano devices")
    
    # Use the app interface instead of raw interface
    client = ConfluxCommandSender(backend)
    # The path used for this entire test
    path: str = "m/503'/1'/0'/0/0"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    _, public_key, _, _ = unpack_get_public_key_response(rapdu.data)

    # Create the transaction that will be sent to the device for signing
    transaction = Transaction(
        to=Base32Address(TARGET_ADDRESS, network_id=1029),
        value=1,
        nonce=1,
        gas=1,
        gasPrice=1,
        storageLimit=1,
        epochHeight=1,
        chainId=1029,
        data="".encode("utf-8")
    ).serialize()

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_tx(path=path, transaction=transaction):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve()

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    _, sig, _ = unpack_vrs_response(response)
    
    assert check_rs_signature_validity(public_key, sig, transaction)


# In this test a transaction is sent to the device to be signed and validated on screen.
# This test is mostly the same as the previous one but with different values.
# In particular the long data will force the transaction to be sent in multiple chunks
# def test_sign_tx_long_tx(firmware, backend, navigator, test_name):
def test_sign_tx_long_tx(backend, scenario_navigator, firmware, navigator):
    # Use the app interface instead of raw interface
    client = ConfluxCommandSender(backend)
    path: str = "m/503'/1'/0'/0/0"

    rapdu = client.get_public_key(path=path)
    _, public_key, _, _ = unpack_get_public_key_response(rapdu.data)

    data_str = ("This is a very long memo. "
              "It will force the app client to send the serialized transaction to be sent in chunk. "
              "As the maximum chunk size is 255 bytes we will make this memo greater than 255 characters. "
              "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed non risus. Suspendisse lectus tortor, dignissim sit amet, adipiscing nec, ultricies sed, dolor. Cras elementum ultrices diam.")

    transaction = Transaction(
        to=Base32Address(TARGET_ADDRESS, network_id=1029),
        value=1,
        nonce=1,
        gas=1,
        gasPrice=1,
        storageLimit=1,
        epochHeight=1,
        chainId=1029,
        data=data_str.encode("utf-8")
    ).serialize()
    
    # Enable display of transaction data (NBGL devices only)
    if not firmware.device.startswith("nano"):
        navigator.navigate([NavInsID.USE_CASE_HOME_SETTINGS,
                            NavIns(NavInsID.TOUCH, (200, 113)),
                            NavInsID.USE_CASE_SUB_SETTINGS_EXIT],
                            screen_change_before_first_instruction=False,
                            screen_change_after_last_instruction=False)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_tx(path=path, transaction=transaction):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve()

    response = client.get_async_response().data
    _, sig, _ = unpack_vrs_response(response)
    assert check_rs_signature_validity(public_key, sig, transaction)


# Transaction signature refused test
# The test will ask for a transaction signature that will be refused on screen
def test_sign_tx_refused(backend, scenario_navigator):
    # Use the app interface instead of raw interface
    client = ConfluxCommandSender(backend)
    path: str = "m/503'/1'/0'/0/0"

    rapdu = client.get_public_key(path=path)
    _, pub_key, _, _ = unpack_get_public_key_response(rapdu.data)

    transaction = Transaction(
        to=Base32Address(TARGET_ADDRESS, network_id=1029),
        value=1,
        nonce=1,
        gas=1,
        gasPrice=1,
        storageLimit=1,
        epochHeight=1,
        chainId=1029,
        data="This transaction will be refused by the user".encode("utf-8")  # todo this should be a long string
    ).serialize()

    with pytest.raises(ExceptionRAPDU) as e:
        with client.sign_tx(path=path, transaction=transaction):
            scenario_navigator.review_reject()
    
    # Assert that we have received a refusal
    assert e.value.status == Errors.SW_DENY
    assert len(e.value.data) == 0

def test_personal_sign(backend, scenario_navigator, firmware, navigator):
    if not firmware.device.startswith("nano"):
        pytest.skip("Skipping this test for None Nano devices")
    # Use the app interface instead of raw interface
    client = ConfluxCommandSender(backend)
    # The path used for this entire test
    path: str = "m/503'/1'/0'/0/0"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    _, public_key, _, _ = unpack_get_public_key_response(rapdu.data)

    msg = "Hello, world!".encode("utf-8")

    # Enable display of message data (NBGL devices only)
    if not firmware.device.startswith("nano"):
        navigator.navigate([NavInsID.USE_CASE_HOME_SETTINGS,
                            NavIns(NavInsID.TOUCH, (200, 113)),
                            NavInsID.USE_CASE_SUB_SETTINGS_EXIT],
                            screen_change_before_first_instruction=False,
                            screen_change_after_last_instruction=False)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.personal_sign(path=path, data=msg):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve()

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    _, sig, _ = unpack_vrs_response(response)
    assert check_rs_prefix_msg_signature_validity(public_key, sig, msg)
