# Application Protocol Data Unit (APDU)

## Protocol Explanation

The communication protocol used by [BOLOS](https://ledger.readthedocs.io/en/latest/bolos/overview.html) to exchange [APDU](https://en.wikipedia.org/wiki/Smart_card_application_protocol_data_unit) is very close to [ISO 7816-4](https://www.iso.org/standard/77180.html) with a few differences:

- `Lc` length is always exactly 1 byte
- No `Le` field in APDU command
- Maximum size of APDU command is 260 bytes: 5 bytes of header + 255 bytes of data
- Maximum size of APDU response is 260 bytes: 258 bytes of response data + 2 bytes of status word

Status words tend to be similar to common [APDU responses](https://www.eftlab.com/knowledge-base/complete-list-of-apdu-responses/) in the industry.

### Command APDU

| Field name | Length (bytes) | Description |
| --- | --- | --- |
| CLA | 1 | Instruction class - indicates the type of command |
| INS | 1 | Instruction code - indicates the specific command |
| P1 | 1 | Instruction parameter 1 for the command |
| P2 | 1 | Instruction parameter 2 for the command |
| Lc | 1 | The number of bytes of command data to follow (a value from 0 to 255) |
| CData | var | Command data with `Lc` bytes |

### Response APDU

| Field name | Length (bytes) | Description |
| --- | --- | --- |
| RData | var | Response data (can be empty) |
| SW | 2 | Status word containing command processing status (e.g. `0x9000` for success) |

## Status Words

| SW | SW Name | Description |
| --- | --- | --- |
| 0x9000 | Ok | Success code |
| 0x6985 | Deny | User denied |
| 0x6A86 | WrongP1P2 |  |
| 0x6D00 | InsNotSupported | Instruction not supported |
| 0x6E00 | ClaNotSupported | Cla not supported |
| 0xB001 | TxDisplayFail |  |
| 0xB002 | AddrDisplayFail |  |
| 0xB003 | AmountDisplayFail |  |
| 0xB004 | TxWrongLength |  |
| 0xB005 | TxParsingFail |  |
| 0xB006 | TxHashFail |  |
| 0xB007 | BadState |  |
| 0xB008 | TxSignFail |  |
| 0xB009 | KeyDeriveFail |  |
| 0xB00A | VersionParsingFail |  |
| 0x6e03 | WrongApduLength |  |
| 0x6A80 | InvalidData |  |
| 0x6A87 | WrongDataLength |  |
| 0xB000 | WrongResponseLength |  |

## Commands

The currently supported commands are as follows::

| INS    | Name   |    |
| ------ | ------ |----|
| `0x01` | GET_APP_INFO |    |
| `0x02` | GET_PUBLIC_ADDRESS |    |
| `0x03` | SIGN_TRANSACTION |    |
| `0x04` | SIGN_PERSONAL_MESSAGE |    |
| `0x0A` | SIGN_EIP712 |    |
| `0x0B` | EIP712_SEND_STRUCT_DEFINITION |    |
| `0x0C` | EIP712_SEND_STRUCT_IMPLEMENTATION |    |

### GET_APP_INFO

#### Request format

| CLA    | INS    | P1     | P2     | Lc     |
| ------ | ------ | ------ | ------ | ------ |
| `0xe0` | `0x01` | `0x00` | `0x00` | `0x00` |

##### Request payload

None.

#### Response format

| Description   | Length |
| ------------- | ------ |
| Flags         | 1      |
| Major version | 1      |
| Minor version | 1      |
| Patch version | 1      |

Flags: `b00000NYX`

- `N`: indicate this is 2 generate app
- `X`: blind signing enabled
- `Y`: detailed display enabled

#### Examples

**Command**: `e001000000`

| CLA    | INS    | P1     | P2     | Lc     |
| ------ | ------ | ------ | ------ | ------ |
| `0xe0` | `0x01` | `0x00` | `0x00` | `0x00` |

**Response**: `03000002 9000`, blind signing enabled, detailed display enabled, application version: `0.0.2`.

### GET_PUBLIC_KEY

#### Request format

| CLA  | INS  | P1                    | P2                      | Lc       | Le       |
| ---- | ---- | --------------------- | ----------------------- | -------- | -------- |
| `E0` | `02` | `00`: no display      | `00`: no chain code     | variable | variable |
|      |      | `01`: display address | `01`: return chain code |          |          |

##### Request payload

| Description                                      | Length |
| ------------------------------------------------ | ------ |
| Number of BIP 32 derivations to perform (max 10) | 1      |
| First derivation index (big endian)              | 4      |
| ...                                              | 4      |
| Last derivation index (big endian)               | 4      |
| Chain ID (required when `P1 == 01`)              | 4      |

#### Response format

| Description             | Length |
| ----------------------- | ------ |
| Public key length       | 1      |
| Uncompressed public key | var    |
| Chain code length       | 1      |
| Chain code              | var    |

#### Examples

**Command**: `e002000015058000002c800001f7800000000000000000000000`

| CLA    | INS    | P1     | P2     | Lc     | Le                                                            |
| ------ | ------ | ------ | ------ | ------ | ------------------------------------------------------------- |
| `0xe0` | `0x02` | `0x00` | `0x00` | `0x15` | `0x05 0x8000002c 0x800001f7 0x80000000 0x00000000 0x00000000` |

`44'/503'/0'/0/0` is encoded as `0x05 0x8000002c 0x800001f7 0x80000000 0x00000000 0x00000000`.

**Response**: `41 047b88d05ba40b8e6ed961b526ab68c7051d2a8602862c788f84416cc37e9c0a5c4213b20660a6591cd53ad81d5b68499acb835ac7a08c88e18bf8f4998061eb4a 9000`

---

**Command**: `e002000115058000002c800001f7800000000000000000000000`

| CLA    | INS    | P1     | P2     | Lc     | Le                                                            |
| ------ | ------ | ------ | ------ | ------ | ------------------------------------------------------------- |
| `0xe0` | `0x02` | `0x00` | `0x01` | `0x15` | `0x05 0x8000002c 0x800001f7 0x80000000 0x00000000 0x00000000` |

**Response**: `41 047b88d05ba40b8e6ed961b526ab68c7051d2a8602862c788f84416cc37e9c0a5c4213b20660a6591cd53ad81d5b68499acb835ac7a08c88e18bf8f4998061eb4a 20 20b19d018f0bf5264aa6a0953a22d2cc432205fc022adfeb0160b1cad0b4ab8b 9000`

---

**Command**: `e002010019058000002c800001f780000000000000000000000000000405`

| CLA    | INS    | P1     | P2     | Lc     | Le                                                                       |
| ------ | ------ | ------ | ------ | ------ | ------------------------------------------------------------------------ |
| `0xe0` | `0x02` | `0x01` | `0x00` | `0x19` | `0x05 0x8000002c 0x800001f7 0x80000000 0x00000000 0x00000000 0x00000405` |

Notice the chain ID (`0x00000405 ~ 1029`, aka mainnet) at the end.

**Response**: `41 047b88d05ba40b8e6ed961b526ab68c7051d2a8602862c788f84416cc37e9c0a5c4213b20660a6591cd53ad81d5b68499acb835ac7a08c88e18bf8f4998061eb4a 9000`

---

**Command**: `e002010119058000002c800001f780000000000000000000000000000405`

| CLA    | INS    | P1     | P2     | Lc     | Le                                                                       |
| ------ | ------ | ------ | ------ | ------ | ------------------------------------------------------------------------ |
| `0xe0` | `0x02` | `0x01` | `0x01` | `0x19` | `0x05 0x8000002c 0x800001f7 0x80000000 0x00000000 0x00000000 0x00000405` |

**Response**: `41 047b88d05ba40b8e6ed961b526ab68c7051d2a8602862c788f84416cc37e9c0a5c4213b20660a6591cd53ad81d5b68499acb835ac7a08c88e18bf8f4998061eb4a 20 20b19d018f0bf5264aa6a0953a22d2cc432205fc022adfeb0160b1cad0b4ab8b 9000`

### SIGN_TX

#### Request format

| CLA  | INS  | P1                                      | P2   | Lc       | Le       |
| ---- | ---- | --------------------------------------- | ---- | -------- | -------- |
| `e0` | `03` | `00`: first data block      | `80` more | variable | variable |
|      |      | `01`-`03`: subsequent data block index |  `00` last    |          |          |

##### Request payload

First data block:

| Description                                      | Length |
| ------------------------------------------------ | ------ |
| Number of BIP 32 derivations to perform (max 10) | 1      |
| First derivation index (big endian)              | 4      |
| ...                                              | 4      |
| Last derivation index (big endian)               | 4      |

Subsequent data blocks:

| Description    | Length |
| -------------- | ------ |
| RLP data chunk | var    |

#### **Response** format

| Description | Length |
| ----------- | ------ |
| v           | 1      |
| r           | 32     |
| s           | 32      |

#### Examples

**Command**: `e003008015058000002c800001f7800000000000000000000000`

| CLA    | INS    | P1     | P2     | Lc     | Le                                                                                                                                                       |
| ------ | ------ | ------ | ------ | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `0xe0` | `0x03` | `0x00` | `0x80` | `0x15` | `0x05 0x8000002c 0x800001f7 0x80000000 0x00000000 0x00000000` |

`44'/503'/0'/0/0` is encoded as `0x05 0x8000002c 0x800001f7 0x80000000 0x00000000 0x00000000`.

**Command**: `e00301002ceb1284561f61b9831e84809410109fc8df283027b6285cc889f5aa624eac1f55843b9aca0081800182040580`

| CLA    | INS    | P1     | P2     | Lc     | Le                                                                                                                                                       |
| ------ | ------ | ------ | ------ | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `0xe0` | `0x03` | `0x01` | `0x00` | `0x2c` | `0xeb1284561f61b9831e84809410109fc8df283027b6285cc889f5aa624eac1f55843b9aca0081800182040580` |

`0xeb1284561f61b9831e84809410109fc8df283027b6285cc889f5aa624eac1f55843b9aca0081800182040580` is the RLP encoded list `["0x12", "0x561f61b9", "0x1e8480", "0x10109fC8DF283027b6285cc889F5aA624EaC1F55", "0x3b9aca00", "0x80", "0x1", "0x405", "0x"]` which represents the following transaction:

```js
{
    nonce: 18,
    gasPrice: 1444897209,
    gasLimit: 2000000,
    to: '0x10109fC8DF283027b6285cc889F5aA624EaC1F55',
    value: 1000000000,
    storageLimit: 128,
    epochHeight: 1,
    chainId: 1029,
    data: '0x
}
```

**Response**: `00 f9071161c2dbc19dabf54d14d42944cecacf61943a9898f4f64c8aa6d23a58b6 64ea364f092d23d7a94388f2f43cf54a86fe644d221e822210fde413d406ebb6 9000`

### PERSONAL_SIGN

This command has been supported since app version 2.3.0

#### Request format

| CLA  | INS  | P1                                      | P2   | Lc       | Le       |
| ---- | ---- | --------------------------------------- | ---- | -------- | -------- |
| `e0` | `04` | `00`: first data block      | `80` more | variable | variable |
|      |      | `01`-`03`: subsequent data block index |  `00` last    |          |          |

##### Request payload

First data block:

| Description                                      | Length |
| ------------------------------------------------ | ------ |
| Number of BIP 32 derivations to perform (max 10) | 1      |
| First derivation index (big endian)              | 4      |
| ...                                              | 4      |
| Last derivation index (big endian)               | 4      |

Subsequent data blocks:

| Description    | Length |
| -------------- | ------ |
| Message data chunk | var    |

#### **Response** format

| Description | Length |
| ----------- | ------ |
| v           | 1      |
| r           | 32     |
| s           | 32     |

#### Examples

**Command**: `e004008015058000002c800001f7800000000000000000000000`

| CLA    | INS    | P1     | P2     | Lc     | Le                                                                                                                                                       |
| ------ | ------ | ------ | ------ | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `0xe0` | `0x04` | `0x00` | `0x80` | `0x15` | `0x05 0x8000002c 0x800001f7 0x80000000 0x00000000 0x00000000` |

`44'/503'/0'/0/0` is encoded as `0x05 0x8000002c 0x800001f7 0x80000000 0x00000000 0x00000000`.

**Command**: `e0040100c48656c6c6f2c20576f726c64`

| CLA    | INS    | P1     | P2     | Lc     | Le                                                                          |
| ------ | ------ | ------ | ------ | ------ | ----------------------------------------------------------------------------|
| `0xe0` | `0x04` | `0x01` | `0x00` | `0xc` | `0x48656c6c6f2c20576f726c64` |

`0x48656c6c6f2c20576f726c64` is the message "Hello, World" in hex.

### SIGN ETH EIP 712

#### Description

This command signs an Ethereum message following the EIP 712 specification (https://github.com/ethereum/EIPs/blob/master/EIPS/eip-712.md)

This command has been supported since app version 2.3.0

The full implementation uses all the JSON data and does all the hashing on the
device. This command should come last, after all the EIP712 SEND STRUCT DEFINITION & SEND STRUCT IMPLEMENTATION.

#### Coding

_Command_



| *CLA* | *INS*  | *P1*               | *P2*       | *Lc*     | *Le* |
| ------ | ------ | ------ | ------ | ------ | ---------------------|
|   E0  |   0A   |  00 | 01: full implementation | variable | variable |


_Input data_

| Description                                      | Length |
| ------------------------------------------------ | ------ |
| Number of BIP 32 derivations to perform (max 10) | 1      |
| First derivation index (big endian)              | 4      |
| ...                                              | 4      |
| Last derivation index (big endian)               | 4      |


**Response** format

| Description | Length |
| ----------- | ------ |
| v           | 1      |
| r           | 32     |
| s           | 32     |

### EIP712 SEND STRUCT DEFINITION

#### Description

This command sends the message definition with all its types. +
These commands should come before the EIP712 SEND STRUCT IMPLEMENTATION ones.

#### Coding

_Command_


| *CLA* | *INS*  | *P1*               | *P2*       | *LC*     | *Le* |
| ------ | ------ | ------ | ------ | ------ | ---------------------|
|   E0  |   0B   |  00 |   00 : struct name <br> FF : struct field| variable | variable |

_Input data_

##### If P2 == struct name


| *Description*         | *Length (byte)* |
| ---- | ----|
| Name                  | LC |


##### If P2 == struct field

| *Description*                     | *Length (byte)*   | *Mandatory*|
| ------ | ------ | ------ |
| TypeDesc (type description)       | 1                 | ✅|
| TypeNameLength                    | 1                 | ❌|
| TypeName                          | variable          | ❌|
| TypeSize                          | 1                 | ❌|
| ArrayLevelCount                   | 1                 | ❌|
| ArrayLevels                       | variable          | ❌|
| KeyNameLength                     | 1                 | ✅|
| KeyName                           | variable          | ✅|


###### TypeDesc

From MSB to LSB:


| *Description*                             | *Length (bit)* |
| ----- | ------ |
| TypeArray (is it an array?)               | 1 |
| TypeSize (is a type size specified?)      | 1 |
| Unused                                    | 2 |
| Type                                      | 4 |


How to interpret Type from its value :


| *Value*           | *Type* |
| ----- | ------ |
| 0                 | custom (struct type) |
| 1                 | int|
| 2                 | uint|
| 3                 | address|
| 4                 | bool|
| 5                 | string|
| 6                 | fixed-sized bytes|
| 7                 | dynamic-sized bytes|


###### TypeName

_Only present if the Type is set to custom._

Indicates the name of the struct that will be the type of the field.


###### TypeSize

_Only present if the TypeSize bit is set in TypeDesc._

Indicates the byte size of the field. (Ex: 8 for an int64)


###### ArrayLevelCount

_Only present if the TypeArray bit is set in TypeDesc._

Indicates how many array levels that field has (Ex: 3 for int16[2][][4]).

###### ArrayLevels

_Only present if the TypeArray bit is set in TypeDesc._

Types of array level:

| *Byte value*  | *Type* |
| ---- | --- |
| 0             | Dynamic sized (type[]) |
| 1             | Fixed size (type[N]) |

Each fixed-sized array level is followed by a byte indicating its size (number of elements).

_Output data_

None

### EIP712 SEND STRUCT IMPLEMENTATION

#### Description

This command sends the message implementation with all its values.
These commands should come after the EIP712 SEND STRUCT DEFINITION ones.

#### Coding

_Command_


| *CLA* | *INS*  | *P1*               | *P2*       | *LC*     | *Le* |
| ------ | ------ | ------ | ------ | ------ | ---------------------|
|   E0  |   0C   |  00 : complete send <br> 01 : partial send, more to come |   00 : root struct <br> 0F : array<br> FF : struct field | variable | variable |


_Input data_

##### If P2 == root struct


| *Description*         | *Length (byte)* |
| ---- | ---- |
| Name                  | LC |

Sets the name of the upcoming root structure all the following fields will be apart
of until we set another root structure.

##### If P2 == array

| *Description*         | *Length (byte)* |
| ---- | ---- |
| Array size            | 1 |


Sets the size of the upcoming array the following N fields will be apart of.

##### If P2 == struct field


| *Description*         | *Length (byte)* |
| ---- | ---- |
| Value length          | 2 (BE) |
| Value                 | variable |


Sets the raw value of the next field in order in the current root structure.
Raw as in, an integer in the JSON file represented as "128" would only be 1 byte long (0x80)
instead of 3 as an array of ASCII characters, same for addresses and so on.


_Output data_

None