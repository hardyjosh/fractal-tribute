use hdk::prelude::*;
use fractal_tribute_integrity::*;

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

    // create a link from the agent's pubkey to the evm key binding
    let agent_pubkey = agent_info()?.agent_latest_pubkey;
    create_link(agent_pubkey, evm_key_binding_hash.clone(), LinkTypes::AgentToEvmKeyBinding, ())?;
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

#[hdk_extern]
pub fn get_evm_address(_:()) -> ExternResult<Vec<u8>> {
    let agent_pubkey = agent_info()?.agent_latest_pubkey;
    let evm_address = get_agent_evm_address(agent_pubkey)?;
    Ok(evm_address)
}

#[hdk_extern]
pub fn get_agent_evm_address(base: AgentPubKey) -> ExternResult<Vec<u8>> {
    let links = get_links(base, LinkTypes::AgentToEvmKeyBinding, None)?;
        let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    if records.len() == 0 {
        return Err(wasm_error!("No EvmKeyBinding found for this agent"));
    }
    let evm_key_binding: EvmKeyBinding = records[0].entry().to_app_option().map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("EvmKeyBinding not found at link target"))
            ),
        )?;
    Ok(evm_key_binding.evm_key)
}