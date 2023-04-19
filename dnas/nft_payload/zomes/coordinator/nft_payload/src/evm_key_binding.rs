use hdk::prelude::*;
use nft_payload_integrity::*;
use std::convert::TryInto;
use std::fmt;

#[hdk_extern]
pub fn create_evm_key_binding(evm_key_binding: EvmKeyBinding) -> ExternResult<Record> {
    let evm_key_binding_hash = create_entry(
        &EntryTypes::EvmKeyBinding(evm_key_binding.clone()),
    )?;
    let record = get(evm_key_binding_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created EvmKeyBinding"))
            ),
        )?;
    Ok(record)
}

#[derive(Debug)]
pub enum EvmAddressError {
    QueryError,
    RecordNotFound,
    EntryError,
    AppEntryError,
    EvmKeyBindingError,
}

impl fmt::Display for EvmAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvmAddressError::QueryError => write!(f, "Error in query"),
            EvmAddressError::RecordNotFound => write!(f, "Record not found"),
            EvmAddressError::EntryError => write!(f, "Error in entry"),
            EvmAddressError::AppEntryError => write!(f, "Error in app entry"),
            EvmAddressError::EvmKeyBindingError => write!(f, "Error converting to EvmKeyBinding"),
        }
    }
}

#[hdk_extern]
pub fn get_evm_address(_:()) -> ExternResult<Option<ByteArray>> {
    _get_evm_address().map_err(|e| wasm_error!(e.to_string()))
}

pub fn _get_evm_address() -> Result<Option<ByteArray>, EvmAddressError> {
    let query_filter = ChainQueryFilter::new().include_entries(true);
    let records = query(query_filter).map_err(|_| EvmAddressError::QueryError)?;
    let record = match records.get(4).cloned() {
        Some(record) => record,
        None => return Ok(None),
    };
    let sb = record.entry.as_option().ok_or(EvmAddressError::EntryError)?
                    .as_app_entry().ok_or(EvmAddressError::AppEntryError)?
                    .clone().into_sb();
    let evm_key_binding: EvmKeyBinding = sb.try_into().map_err(|_| EvmAddressError::EvmKeyBindingError)?;
    let key = evm_key_binding.evm_key;
    Ok(Some(key))
}