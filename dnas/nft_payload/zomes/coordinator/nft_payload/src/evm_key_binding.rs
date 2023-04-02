use hdk::prelude::*;
use nft_payload_integrity::*;

#[hdk_extern]
pub fn create_evm_key_binding(evm_key_binding: EvmKeyBinding) -> ExternResult<Record> {
    let evm_key_binding_hash = create_entry(
        &EntryTypes::EvmKeyBinding(evm_key_binding.clone()),
    )?;

    // create_link(
    //     evm_key_binding.creator.clone(),
    //     evm_key_binding_hash.clone(),
    //     LinkTypes::CreatorToEvmKeyBindings,
    //     (),
    // )?;
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
    // let evm_key_entry = get_evm_binding_entry().ok().unwrap().entry.as_option().unwrap().as_app_entry().unwrap().clone().into_sb();
    // let try_into_evm_key_binding: Result<EvmKeyBinding, SerializedBytesError> = evm_key_entry.try_into();
    // let res = try_into_evm_key_binding.ok().unwrap();
    // let key = res.evm_key;
    return Ok(_get_evm_address());
    // return get_evm_binding_entry();
}

pub fn _get_evm_address() -> ByteArray {
    let query_filter = ChainQueryFilter::new().include_entries(true);
    let records = query(query_filter).ok().unwrap();
    let record = records.get(4).cloned();
    let result = record.ok_or( wasm_error!(
        WasmErrorInner::Guest(String::from("Record not found"))
    ),);
    let sb = result.ok().unwrap().entry.as_option().unwrap().as_app_entry().unwrap().clone().into_sb();
    let evm_key_binding: EvmKeyBinding = sb.try_into().ok().unwrap();
    let key = evm_key_binding.evm_key;
    return key;
}

// #[hdk_extern]
// pub fn get_evm_key_binding(
//     evm_key_binding_hash: ActionHash,
// ) -> ExternResult<Option<Record>> {
//     get(evm_key_binding_hash, GetOptions::default())
// }
// #[hdk_extern]
// pub fn get_evm_key_bindings_for_creator(
//     creator: AgentPubKey,
// ) -> ExternResult<Vec<Record>> {
    // let links = get_links(creator, LinkTypes::CreatorToEvmKeyBindings, None)?;
//     let get_input: Vec<GetInput> = links
//         .into_iter()
//         .map(|link| GetInput::new(
//             ActionHash::from(link.target).into(),
//             GetOptions::default(),
//         ))
//         .collect();
//     let records: Vec<Record> = HDK
//         .with(|hdk| hdk.borrow().get(get_input))?
//         .into_iter()
//         .filter_map(|r| r)
//         .collect();
//     Ok(records)
// }
