from ragger.utils import RAPDU
from typing import Optional
import contextlib
from .status_word import StatusWord
from .eip712 import EIP712FieldType
from .command_builder import CommandBuilder

# This is a mock client used to print the encode data
class EthAppClient:
    def __init__(self):
        self._cmd_builder = CommandBuilder()

    def _exchange_async(self, payload: bytes):
        print(f"Exchanging: ins->{payload[1]:02x} p1->{payload[2]:02x} p2->{payload[3]:02x} len->{payload[4]} data -> {payload[5:].hex()}")
        return contextlib.nullcontext(StatusWord.OK)

    def _exchange(self, payload: bytes) -> RAPDU:
        print(f"Exchanging: ins->{payload[1]:02x} p1->{payload[2]:02x} p2->{payload[3]:02x} len->{payload[4]} data -> {payload[5:].hex()}")
        return StatusWord.OK

    def response(self) -> Optional[RAPDU]:
        return RAPDU(StatusWord.OK, b"")
    
    def eip712_send_struct_def_struct_name(self, name: str):
        print("send struct def->name:", name)
        return self._exchange_async(self._cmd_builder.eip712_send_struct_def_struct_name(name))

    def eip712_send_struct_def_struct_field(self,
                                            field_type: EIP712FieldType,
                                            type_name: str,
                                            type_size: int,
                                            array_levels: list,
                                            key_name: str):
        return self._exchange_async(self._cmd_builder.eip712_send_struct_def_struct_field(
                          field_type,
                          type_name,
                          type_size,
                          array_levels,
                          key_name))

    def eip712_send_struct_impl_root_struct(self, name: str):
        print("send struct impl->root struct: ", name)
        return self._exchange_async(self._cmd_builder.eip712_send_struct_impl_root_struct(name))

    def eip712_send_struct_impl_array(self, size: int):
        print("send struct impl->array size", size)
        return self._exchange_async(self._cmd_builder.eip712_send_struct_impl_array(size))

    def eip712_send_struct_impl_struct_field(self, raw_value: bytes):
        chunks = self._cmd_builder.eip712_send_struct_impl_struct_field(bytearray(raw_value))
        for chunk in chunks[:-1]:
            self._exchange(chunk)
        return self._exchange_async(chunks[-1])

    def eip712_sign_new(self, bip32_path: str):
        return self._exchange_async(self._cmd_builder.eip712_sign_new(bip32_path))
