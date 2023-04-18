use hdk::prelude::*;
use nft_payload_integrity::*;

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

#[hdk_extern]
pub fn get_evm_address(_:()) -> ExternResult<ByteArray> {
    return Ok(_get_evm_address());
}

pub fn _get_evm_address() -> ByteArray {
    let query_filter = ChainQueryFilter::new().include_entries(true);
    let records = query(query_filter).ok().unwrap();
    let record = records.get(4).cloned();
    let result = record.ok_or( wasm_error!(
        WasmErrorInner::Guest(String::from("Record not found"))
    ),);
    let empty_byte_array = ByteArray::new(vec![]);
    // let sb = result.ok().unwrap().entry.as_option().unwrap().as_app_entry().unwrap().clone().into_sb();
    // let evm_key_binding: EvmKeyBinding = sb.try_into().ok().unwrap();
    // let key = evm_key_binding.evm_key;
    return empty_byte_array;
}
