use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use alloy_primitives::B256;
use ledger_rust_eip712::{eip712::eip712_signing_hash, Eip712Domain};
pub use ledger_rust_eip712::{types, utils, CIP23_DOMAIN_TYPE_NAME, EIP712_DOMAIN_TYPE_NAME};
use types::{
    Eip712FieldDefinition, Eip712FieldValue, Eip712StructDefinitions, Eip712StructImplementation,
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
    pub rest_len_to_read: usize,
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
            rest_len_to_read: 0,
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
        self.rest_len_to_read = 0;
    }

    pub fn complete_one_struct_def(&mut self) {
        if self.current_struct_name.is_none() {
            return;
        }
        let name = self.current_struct_name.take().unwrap();
        let fields = self.current_struct_fields.drain(..).collect();
        self.struct_definitions.insert(name, fields);
    }

    pub fn parse_eip712_domain(&mut self) -> Result<(), String> {
        if self.current_root_struct != Some(EIP712_DOMAIN_TYPE_NAME.to_string())
            && self.current_root_struct != Some(CIP23_DOMAIN_TYPE_NAME.to_string())
        {
            return Ok(());
        }
        let field_defs = self
            .struct_definitions
            .get(EIP712_DOMAIN_TYPE_NAME)
            .or(self.struct_definitions.get(CIP23_DOMAIN_TYPE_NAME))
            .ok_or("field defs not found")?;

        // If we already have a struct name and fields, we should finalize the previous struct
        let name = self.current_root_struct.take().unwrap();
        let field_values: Vec<_> = self.current_struct_field_values.drain(..).collect();

        if field_defs.len() != field_values.len() {
            return Err(format!(
                "field defs len {} values len {}",
                field_defs.len(),
                field_values.len()
            ));
        }

        let eip712_impls = Eip712StructImplementation {
            name,
            values: field_values,
        };

        eip712_impls
            .parse_eip712_domain(field_defs, &mut self.eip712_domain)
            .map_err(|e| e.to_string())
    }

    pub fn is_cip23_domain(&self) -> bool {
        self.struct_definitions.contains_key(CIP23_DOMAIN_TYPE_NAME)
    }

    pub fn is_eip712_domain_set_up(&self) -> bool {
        self.eip712_domain.name.is_some()
            || self.eip712_domain.version.is_some()
            || self.eip712_domain.chain_id.is_some()
            || self.eip712_domain.verifying_contract.is_some()
            || self.eip712_domain.salt.is_some()
    }

    pub fn custom_eip712_signing_hash(&mut self) -> Result<B256, &str> {
        if !self.is_eip712_domain_set_up() {
            return Err("no domain data");
        }
        if self.current_root_struct.is_none() {
            return Err("no primary type");
        }
        let primary_type = self.current_root_struct.take().unwrap();
        let field_values: Vec<_> = self.current_struct_field_values.drain(..).collect();
        let mut data_iter = field_values.into_iter().map(|v| v.value);

        eip712_signing_hash(
            &self.struct_definitions,
            &mut data_iter,
            &primary_type,
            &self.eip712_domain,
        )
        .map_err(|_| "compute eip712 hash failed")
    }
}
