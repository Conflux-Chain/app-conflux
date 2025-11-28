#![allow(unused)]

use crate::bip32_path::Bip32Path;
use crate::AppSW;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use alloy_primitives::{Address, B256, U256};
pub use ledger_rust_eip712::types;
use ledger_rust_eip712::{parser, Eip712Domain, Eip712Types, Resolver, TypedData};
use types::{
    build_resolver_from_struct_defs, Eip712FieldDefinition, Eip712FieldValue,
    Eip712StructDefinitions, EIP712_DOMAIN_TYPE_NAME,
};

pub struct Eip712Context {
    pub current_struct_name: Option<String>,
    pub current_struct_fields: Vec<Eip712FieldDefinition>,
    pub struct_definitions: Eip712StructDefinitions,

    pub current_root_struct: Option<String>,
    pub current_struct_field_values: Vec<Eip712FieldValue>,

    pub eip712_domain: Eip712Domain,
    // used as tmp store of large data which need to send in chunks, normally are string or bytes
    pub field_data: Vec<u8>,
    pub path: Bip32Path,
    pub domain_reviewed: bool,
    pub message_reviewed: bool,
}

impl Eip712Context {
    pub fn new() -> Self {
        Eip712Context {
            current_struct_name: None,
            current_struct_fields: Vec::new(),
            struct_definitions: Default::default(),
            current_root_struct: None,
            current_struct_field_values: Vec::new(),
            eip712_domain: Default::default(),
            field_data: Default::default(),
            path: Default::default(),
            domain_reviewed: false,
            message_reviewed: false,
        }
    }

    pub fn reset(&mut self) {
        self.current_struct_name = None;
        self.current_struct_fields.clear();
        self.struct_definitions.clear();
        self.current_root_struct = None;
        self.current_struct_field_values.clear();
        self.eip712_domain = Default::default();
        self.field_data.clear();
        self.path = Default::default();
        self.domain_reviewed = false;
        self.message_reviewed = false;
    }

    pub fn complete_one_struct_def(&mut self) {
        if self.current_struct_name.is_none() {
            return;
        }
        let name = self.current_struct_name.take().unwrap();
        let fields = self.current_struct_fields.drain(..).collect();
        self.struct_definitions.insert(name, fields);
    }

    pub fn parse_eip712_domain(&mut self) -> Result<(), &str> {
        if self.current_root_struct != Some(EIP712_DOMAIN_TYPE_NAME.to_string()) {
            return Ok(());
        }
        let field_defs = self
            .struct_definitions
            .get(&EIP712_DOMAIN_TYPE_NAME.to_string())
            .ok_or("field defs not found")?;
        if field_defs.len() != self.current_struct_field_values.len() {
            return Err("invalid data len");
        }
        // If we already have a struct name and fields, we should finalize the previous struct
        let _name = self.current_root_struct.take().unwrap();
        let field_values: Vec<_> = self.current_struct_field_values.drain(..).collect();

        for (i, def) in field_defs.iter().enumerate() {
            let value = field_values[i].clone();
            match def.name.as_str() {
                "name" => {
                    let name_value = value.to_string()?;
                    self.eip712_domain.name = Some(name_value.into());
                }
                "version" => {
                    let version_value = value.to_string()?;
                    self.eip712_domain.version = Some(version_value.into());
                }
                "chainId" => {
                    let chain_id = value.to_u64()?;
                    self.eip712_domain.chain_id = Some(U256::from(chain_id));
                }
                "verifyingContract" => {
                    let raw_addr_value = value.value;
                    if raw_addr_value.len() != 20 {
                        return Err("invalid address len");
                    }
                    let mut buf = [0u8; 20];
                    buf.copy_from_slice(&raw_addr_value);
                    self.eip712_domain.verifying_contract = Some(Address::from(buf));
                }
                "salt" => {
                    let raw_hash_value = value.value;
                    if raw_hash_value.len() != 32 {
                        return Err("invalid hash len");
                    }
                    let mut buf = [0u8; 32];
                    buf.copy_from_slice(&raw_hash_value);
                    self.eip712_domain.salt = Some(B256::from(buf));
                }
                _ => {
                    // should not happen
                    unreachable!();
                }
            }
        }

        Ok(())
    }

    pub fn is_eip712_domain_set_up(&self) -> bool {
        self.eip712_domain.name.is_some()
            || self.eip712_domain.version.is_some()
            || self.eip712_domain.chain_id.is_some()
            || self.eip712_domain.verifying_contract.is_some()
            || self.eip712_domain.salt.is_some()
    }

    pub fn eip712_signing_hash(&self) -> Result<B256, &str> {
        if !self.is_eip712_domain_set_up() {
            return Err("no domain data");
        }
        if self.current_root_struct.is_none() {
            return Err("no primary type");
        }
        let primary_type = self
            .current_root_struct
            .as_ref()
            .expect("should exist")
            .clone();

        let resolver = build_resolver_from_struct_defs(&self.struct_definitions)?;

        let type_schema = parser::build_schema(&self.struct_definitions, &primary_type)
            .map_err(|_| "build schema failed")?;

        let mut data_iter = self
            .current_struct_field_values
            .iter()
            .map(|v| v.value.clone());
        let value =
            parser::build_value(&type_schema, &mut data_iter).map_err(|_| "invalid data")?;

        let typed_data = TypedData {
            domain: self.eip712_domain.clone(),
            resolver,
            primary_type,
            message: value,
        };

        Ok(typed_data
            .eip712_signing_hash()
            .map_err(|_| "signing hash compute failed")?)
    }

    pub fn message_fields(&self) -> Result<Vec<parser::UIField>, String> {
        let primary_type = self
            .current_root_struct
            .as_ref()
            .expect("should exist")
            .clone();

        let type_schema = parser::build_schema(&self.struct_definitions, &primary_type)
            .map_err(|_| "build schema failed")?;

        let mut data_iter = self
            .current_struct_field_values
            .iter()
            .map(|v| v.value.clone());

        parser::build_ui_fields(&type_schema, &mut data_iter, "")
    }
}
