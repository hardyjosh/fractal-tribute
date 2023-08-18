use hdk::prelude::*;
use fractal_tribute_integrity::*;

#[hdk_extern]
pub fn get_all_game_moves(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("game_moves");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllGameMoves, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}


#[hdk_extern]
pub fn get_all_my_game_moves(_: ()) -> ExternResult<Vec<Record>> {
    let query_filter = ChainQueryFilter::new()
        .include_entries(true)
        .entry_type(EntryType::App(AppEntryDef::new(
            0.into(),
            zome_info()?.id,
            EntryVisibility::Public,
        )));

    let records = query(query_filter).map_err(|_| wasm_error!(
        WasmErrorInner::Guest(String::from("Could not query for all my game moves"))
    ))?;
    let records: Vec<Record> = records.into_iter().collect();
    Ok(records)
}