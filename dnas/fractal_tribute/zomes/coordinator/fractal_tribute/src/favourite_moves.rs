use hdk::prelude::*;
use fractal_tribute_integrity::*;

#[hdk_extern]
pub fn create_favourite_move(game_move_hash: ActionHash) -> ExternResult<()> {
    // to create a favourite move, we're just linking the agent to the game move
    create_link(
        agent_info().unwrap().agent_latest_pubkey,
        game_move_hash.clone(),
        LinkTypes::AgentToGameMove,
        (),
    )?;
    Ok(())
}

#[hdk_extern]
pub fn get_favourite_moves_for_agent(agent_pubkey: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(agent_pubkey, LinkTypes::AgentToGameMove, None)?;
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

#[hdk_extern]
pub fn get_favourite_moves_for_current_agent(_: ()) -> ExternResult<Vec<Record>> {
    get_favourite_moves_for_agent(agent_info()?.agent_latest_pubkey)
}