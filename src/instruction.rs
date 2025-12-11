use crate::{
    ins_consts::{
        ins, p1_eip712_struct_impl, p1_personal_msg, p1_sign_tx, p2_eip712_struct_def,
        p2_eip712_struct_impl, p2_sign_eip712, p2_sign_tx,
    },
    AppSW,
};
use ledger_device_sdk::io::ApduHeader;

/// Possible input commands received through APDUs.
pub enum Instruction {
    GetVersion,
    GetAppName,
    GetPubkey {
        display: bool,
        return_chain_code: bool,
    },
    SignTx {
        chunk: u8,
        more: bool,
    },
    SignCip23 {
        chunk: u8,
        more: bool,
    },
    SignEip191 {
        chunk: u8,
        more: bool,
    },
    Eip712StructDefinition {
        is_struct_name: bool, // whether this command is sending struct name or fields
    },
    Eip712StructImplementation {
        more: bool,
        data_type: u8,
    },
    // Eip712Filter,
    Sign712,
}

impl TryFrom<ApduHeader> for Instruction {
    type Error = AppSW;

    /// APDU parsing logic.
    ///
    /// Parses INS, P1 and P2 bytes to build an [`Instruction`]. P1 and P2 are translated to
    /// strongly typed variables depending on the APDU instruction code. Invalid INS, P1 or P2
    /// values result in errors with a status word, which are automatically sent to the host by the
    /// SDK.
    ///
    /// This design allows a clear separation of the APDU parsing logic and commands handling.
    ///
    /// Note that CLA is not checked here. Instead the method [`Comm::set_expected_cla`] is used in
    /// [`sample_main`] to have this verification automatically performed by the SDK.
    fn try_from(value: ApduHeader) -> Result<Self, Self::Error> {
        match (value.ins, value.p1, value.p2) {
            (ins::GET_VERSION, 0, 0) => Ok(Instruction::GetVersion),
            (ins::GET_PUBLIC_ADDRESS, 0 | 1, 0 | 1) => Ok(Instruction::GetPubkey {
                display: value.p1 != 0,
                return_chain_code: value.p2 != 0,
            }),
            (ins::SIGN_TRANSACTION, p1_sign_tx::P1_SIGN_TX_START, p2_sign_tx::P2_SIGN_TX_MORE)
            | (
                ins::SIGN_TRANSACTION,
                1..=p1_sign_tx::P1_SIGN_TX_MAX,
                p2_sign_tx::P2_SIGN_TX_LAST | p2_sign_tx::P2_SIGN_TX_MORE,
            ) => Ok(Instruction::SignTx {
                chunk: value.p1,
                more: value.p2 == p2_sign_tx::P2_SIGN_TX_MORE,
            }),
            // CIP23 personal message
            (
                ins::SIGN_PERSONAL_MESSAGE,
                p1_personal_msg::P1_SIGN_MSG_START,
                p2_sign_tx::P2_SIGN_TX_MORE,
            )
            | (
                ins::SIGN_PERSONAL_MESSAGE,
                1..=p1_personal_msg::P1_SIGN_MSG_MAX,
                p2_sign_tx::P2_SIGN_TX_LAST | p2_sign_tx::P2_SIGN_TX_MORE,
            ) => Ok(Instruction::SignCip23 {
                chunk: value.p1,
                more: value.p2 == p2_sign_tx::P2_SIGN_TX_MORE,
            }),
            // EIP191 personal message
            (
                ins::SIGN_ETH_PERSONAL_MESSAGE,
                p1_personal_msg::P1_SIGN_MSG_START,
                p2_sign_tx::P2_SIGN_TX_MORE,
            )
            | (
                ins::SIGN_ETH_PERSONAL_MESSAGE,
                1..=p1_personal_msg::P1_SIGN_MSG_MAX,
                p2_sign_tx::P2_SIGN_TX_LAST | p2_sign_tx::P2_SIGN_TX_MORE,
            ) => Ok(Instruction::SignEip191 {
                chunk: value.p1,
                more: value.p2 == p2_sign_tx::P2_SIGN_TX_MORE,
            }),
            (ins::GET_APP_NAME, 0, 0) => Ok(Instruction::GetAppName),

            (
                ins::EIP712_SEND_STRUCT_DEFINITION,
                0,
                p2_eip712_struct_def::STRUCT_NAME | p2_eip712_struct_def::STRUCT_FIELD,
            ) => Ok(Instruction::Eip712StructDefinition {
                is_struct_name: value.p2 == p2_eip712_struct_def::STRUCT_NAME,
            }),

            (
                ins::EIP712_SEND_STRUCT_IMPLEMENTATION,
                p1_eip712_struct_impl::PARTIAL_SEND | p1_eip712_struct_impl::COMPLETE_SEND,
                p2_eip712_struct_impl::ROOT_STRUCT
                | p2_eip712_struct_impl::ARRAY
                | p2_eip712_struct_impl::STRUCT_FIELD,
            ) => Ok(Instruction::Eip712StructImplementation {
                more: value.p1 == p1_eip712_struct_impl::PARTIAL_SEND,
                data_type: value.p2,
            }),

            (ins::SIGN_EIP712, 0, p2_sign_eip712::FULL_IMPLEMENTATION) => Ok(Instruction::Sign712),

            // wrong p1 p2
            (ins::GET_VERSION..=ins::GET_APP_NAME, _, _) => Err(AppSW::WrongP1P2),
            (
                ins::SIGN_EIP712
                | ins::EIP712_SEND_STRUCT_DEFINITION
                | ins::EIP712_SEND_STRUCT_IMPLEMENTATION,
                _,
                _,
            ) => Err(AppSW::WrongP1P2),

            (_, _, _) => Err(AppSW::InsNotSupported),
        }
    }
}
