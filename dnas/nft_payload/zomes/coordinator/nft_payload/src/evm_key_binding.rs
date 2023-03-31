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
pub fn get_evm_address(_:()) -> ExternResult<Record> {
    return get_evm_binding_entry();
}

pub fn get_evm_binding_entry() -> Result<Record, WasmError> {
    let query_filter = ChainQueryFilter::new().include_entries(true);
    let records = query(query_filter)?;
    let record = records.get(4).cloned();
    let result = record.ok_or( wasm_error!(
        WasmErrorInner::Guest(String::from("Record not found"))
    ),);
    return result;
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
//     let links = get_links(creator, LinkTypes::CreatorToEvmKeyBindings, None)?;
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
