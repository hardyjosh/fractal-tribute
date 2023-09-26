use hdk::prelude::*;
use fractal_tribute_integrity::*;

#[hdk_extern]
pub fn create_profile(profile: Profile) -> ExternResult<Record> {
    let profile_hash: HoloHash<holo_hash::hash_type::Action> = create_entry(
        &EntryTypes::Profile(profile.clone()),
    )?;
    let record = get(profile_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Profile"))
            ),
        )?;

    // create a link from the agent to the profile
    let agent_address = agent_info()?.agent_latest_pubkey;
    let agent_profile_link = create_link(
        agent_address,
        profile_hash.clone(),
        LinkTypes::AgentToProfile,
        ()
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_profile(_agent_pub_key: AgentPubKey) -> ExternResult<Profile> {
    let links = get_links(_agent_pub_key, LinkTypes::AgentToProfile, None)?;
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
        return Err(wasm_error!("No profile found for this agent"));
    }
    let profile: Profile = records[0].entry().to_app_option().map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Profile not found at link target"))
            ),
        )?;
    Ok(profile)
}