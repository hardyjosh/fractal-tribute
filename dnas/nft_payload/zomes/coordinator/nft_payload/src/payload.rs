use hdk::prelude::*;
use nft_payload_integrity::*;
#[hdk_extern]
pub fn create_payload(payload: Payload) -> ExternResult<Record> {
    let payload_hash = create_entry(&EntryTypes::Payload(payload.clone()))?;
    create_link(
        payload.creator.clone(),
        payload_hash.clone(),
        LinkTypes::CreatorToPayloads,
        (),
    )?;
    let record = get(payload_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Payload"))
            ),
        )?;
    let path = Path::from("all_payloads");
    create_link(
        path.path_entry_hash()?,
        payload_hash.clone(),
        LinkTypes::AllPayloads,
        (),
    )?;
    let my_agent_pub_key = agent_info()?.agent_latest_pubkey;
    create_link(
        my_agent_pub_key,
        payload_hash.clone(),
        LinkTypes::PayloadsByCreator,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_payload(payload_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(payload_hash, GetOptions::default())
}
#[hdk_extern]
pub fn get_payloads_for_creator(creator: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(creator, LinkTypes::CreatorToPayloads, None)?;
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
