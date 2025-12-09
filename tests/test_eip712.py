import pytest
from pathlib import Path
import json
from typing import Optional
import fnmatch
import os

from ragger.navigator import NavIns, NavInsID
from ragger.navigator import NavigateWithScenario
from ragger.backend import BackendInterface

import application_client.response_parser as ResponseParser
from application_client.command_sender import ConfluxCommandSender
from application_client.eip712 import InputData
from utils import recover_message


BIP32_PATH = "m/44'/503'/0'/0/0"
DEVICE_ADDR: Optional[bytes] = None

def set_wallet_addr(backend: BackendInterface) -> bytes:
    global DEVICE_ADDR

    # don't ask again if we already have it
    if DEVICE_ADDR is None:
        client = ConfluxCommandSender(backend)
        res = client.get_public_key(BIP32_PATH)
        _, DEVICE_ADDR, _ = ResponseParser.pk_addr(res.data, True)


@pytest.fixture(autouse=True)
def init_wallet_addr(backend: BackendInterface):
    """Initialize wallet address before each test"""
    set_wallet_addr(backend)

def eip712_json_path() -> str:
    return f"{os.path.dirname(__file__)}/eip712_input_files"


def input_files() -> list[str]:
    files = []
    for file in os.scandir(eip712_json_path()):
        if fnmatch.fnmatch(file, "*-data.json"):
            files.append(file.path)
    return sorted(files)


@pytest.fixture(name="input_file", params=input_files())
def input_file_fixture(request) -> Path:
    return Path(request.param)
    
def eip712_new_common(scenario_navigator: NavigateWithScenario,
                      data: dict,
                      filters: Optional[dict] = None,
                      snapshots_dirname: Optional[str] = None,
                      with_warning: bool = True) -> bytes:
    app_client = ConfluxCommandSender(scenario_navigator.backend)

    InputData.process_data(app_client, data, filters)
    do_compare = snapshots_dirname is not None
    with app_client.eip712_sign_new(BIP32_PATH):
        if with_warning:
            # Warning screen
            scenario_navigator.review_approve_with_warning(test_name=snapshots_dirname, do_comparison=do_compare)
        else:
            scenario_navigator.review_approve(test_name=snapshots_dirname, do_comparison=do_compare)

    vrs = ResponseParser.signature(app_client.response().data)
    # verify signature
    assert DEVICE_ADDR == recover_message(data, vrs)
    
def test_eip712_new(scenario_navigator: NavigateWithScenario, input_file: Path):
    filters = None

    with open(input_file, encoding="utf-8") as file:
        data = json.load(file)
        eip712_new_common(scenario_navigator, data, filters)