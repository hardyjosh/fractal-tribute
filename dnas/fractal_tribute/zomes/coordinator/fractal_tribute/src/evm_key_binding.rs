use hdk::prelude::*;
use fractal_tribute_integrity::*;
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

    // create a link from the agent's pubkey to the evm key binding
    let agentPubKey = agent_info()?.agent_latest_pubkey;
    create_link(agentPubKey, evm_key_binding_hash.clone(), LinkTypes::AgentToEvmKeyBinding, ())?;
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
pub fn get_evm_address(_:()) -> ExternResult<Option<Vec<u8>>> {
    _get_evm_address().map_err(|e| wasm_error!(e.to_string()))
}

pub fn _get_evm_address() -> Result<Option<Vec<u8>>, EvmAddressError> {
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
    if (records.len() == 0) {
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