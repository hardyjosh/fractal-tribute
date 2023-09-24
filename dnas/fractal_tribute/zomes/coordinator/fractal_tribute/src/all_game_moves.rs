use hdk::prelude::*;
use fractal_tribute_integrity::*;

#[hdk_extern]
pub fn get_all_game_moves(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("all_game_moves");
    let mut time = sys_time()?;
    debug!("Starting to get moves at: {:?}", time);
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllGameMoves, None)?;
    debug!("Time to get links: {:?}", sys_time()? - time);
    time = sys_time()?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
            GetOptions::content(),
        ))
        .collect();
    debug!("Time to get inputs: {:?}", sys_time()? - time);
    time = sys_time()?;
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    debug!("Time to get records: {:?}", sys_time()? - time);
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}

#[hdk_extern]
pub fn get_all_game_moves_from_link_tags(_:()) -> ExternResult<Vec<GameMove>> {
    debug!("Starting to get all game moves from link tags");
    let path = Path::from("all_game_moves");
    debug!("Got path {:?}", path);
    let path_entry_hash = path.path_entry_hash()?;
    debug!("Got path entry hash");
    let links = get_links(path_entry_hash, LinkTypes::AllGameMoves, None)?;
    debug!("Got links");
    // get the bytes from each link tag and make a vector of game moves
    let game_moves: Vec<GameMove> = links
        .into_iter()
        .map(|link| {
            let bytes = link.tag.0;
            let game_move = GameMove::from_bytes(&bytes).ok().unwrap();
            Ok(game_move)
        })
        .collect::<ExternResult<Vec<GameMove>>>()?;
    Ok(game_moves)
}

#[hdk_extern]
pub fn get_number_of_moves(_:()) -> ExternResult<u32> {
    let path = Path::from("all_game_moves");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllGameMoves, None)?;
    Ok(links.len() as u32)
}

#[hdk_extern]
pub fn get_all_my_game_moves(_: ()) -> ExternResult<Vec<Record>> {
    let query_filter = ChainQueryFilter::new()
        .include_entries(true)
        .entry_type(EntryType::App(AppEntryDef::new(
            0.into(),
            0.into(),
            EntryVisibility::Public,
        )));

    let records = query(query_filter).map_err(|_| wasm_error!(
        WasmErrorInner::Guest(String::from("Could not query for all my game moves"))
    ))?;
    let records: Vec<Record> = records.into_iter().collect();
    Ok(records)
}