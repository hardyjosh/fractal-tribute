use hdk::prelude::*;
use nft_payload_integrity::*;
use crate::evm_key_binding::{_get_evm_address};

#[hdk_extern]
pub fn create_payload(payload: Payload) -> ExternResult<Vec<u8>> {
    let payload_hash = create_entry(&EntryTypes::Payload(payload.clone()))?;
    let record = get(payload_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Payload"))
            ),
        )?;

    // let my_agent_pub_key = agent_info()?.agent_latest_pubkey; // this should be hash of evm pubkey + payload_hash.clone()

    // add the extra 12 empty bytes so it matches the Solidty uint256
    let key = _get_evm_address();
    let mut key_bytes = vec![0; 12];
    key_bytes.append(&mut key.into_vec());

    // hash the content bytes
    let content_bytes = payload.payload_bytes.into_vec();
    let content_hash = hash_keccak256(content_bytes)?;
    // let payload_bytes = record.entry.as_option().unwrap().as_app_entry().unwrap().bytes();
    // let payload_bytes = record.entry.to_app_option::<EvmKeyBinding>().ok().unwrap().unwrap();

    let mut hash_input = key_bytes.to_vec();
    hash_input.extend_from_slice(&content_hash);
    let mut hash = hash_keccak256(hash_input).ok().unwrap();

    // resize the 32 byte keccak hash to 36 bytes so we can use it as a Holohash
    // @todo the extra 4 bytes should be a derived location for sharding
    hash.resize(36, 0);
    let link_base = ExternalHash::from_raw_36(hash);
    let return_link = link_base.clone();

    create_link(
        link_base,
        payload_hash.clone(),
        LinkTypes::TokenIdToPayload,
        (),
    )?;
    Ok(return_link.get_raw_39().to_vec())
}

#[hdk_extern]
pub fn get_payload(payload_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(payload_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_payload_from_link(base: ExternalHash) -> ExternResult<Vec<Record>> {
    let links = get_links(base, LinkTypes::TokenIdToPayload, None)?;
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
    Ok(records)
}