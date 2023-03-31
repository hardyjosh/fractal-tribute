use hdk::prelude::*;
use nft_payload_integrity::*;
#[hdk_extern]
pub fn create_payload(payload: Payload) -> ExternResult<Record> {
    let payload_hash = create_entry(&EntryTypes::Payload(payload.clone()))?;
    let record = get(payload_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Payload"))
            ),
        )?;

    let my_agent_pub_key = agent_info()?.agent_latest_pubkey; // this should be hash of evm pubkey + payload_hash.clone()

    create_link(
        my_agent_pub_key,
        payload_hash.clone(),
        LinkTypes::TokenIdToPayload,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_payload(payload_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(payload_hash, GetOptions::default())
}