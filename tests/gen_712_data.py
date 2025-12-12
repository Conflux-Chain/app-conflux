from application_client.eip712.InputData import process_data
from application_client.mock_client import EthAppClient
import os
import fnmatch
import json

app_client = EthAppClient()

data = json.load(open("./tests/eip712_input_files/cip23-simple-mail-data.json"))

""" data = {
    "domain": {
        "chainId": 1,
        "name": "Simple Mail",
        "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC",
        "version": "1"
    },
    "message": {
        "from": {
            "name": "Cow",
            "wallets": [
                "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826",
                "0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF"
            ]
        },
        "to": {
            "name": "Bob",
            "wallets": [
                "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB",
                "0xB0BdaBea57B0BDABeA57b0bdABEA57b0BDabEa57",
                "0xB0B0b0b0b0b0B000000000000000000000000000"
            ]
        },
        "contents": "Hello, Bob!",
        "timestamp": 1633072800,
        "amount": 1000000,
        "payback": "0x1000000000000000000"
    },
    "primaryType": "Mail",
    "types": {
        "EIP712Domain": [
            { "name": "name", "type": "string" },
            { "name": "version", "type": "string" },
            { "name": "chainId", "type": "uint256" },
            { "name": "verifyingContract", "type": "address" }
        ],
        "Mail": [
            { "name": "from", "type": "Person" },
            { "name": "to", "type": "Person" },
            { "name": "contents", "type": "string" },
            { "name": "timestamp", "type": "uint64" },
            { "name": "amount", "type": "uint256" },
            { "name": "payback", "type": "uint256" }
        ],
        "Person": [
            { "name": "name", "type": "string" },
            { "name": "wallets", "type": "address[]" }
        ]
    }
} """


process_data(app_client, data)

