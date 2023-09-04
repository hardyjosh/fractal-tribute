use hdk::prelude::*;
use fractal_tribute_integrity::*;
use crate::all_game_moves::*;

#[hdk_extern]
pub fn get_latest_board(_: ()) -> ExternResult<Vec<u8>> {
    let all_game_moves = get_all_game_moves(())?;

    // convert the records into a vec of game moves
    let game_moves = all_game_moves.into_iter().map(|record| {
            let entry = record.entry.to_app_option::<GameMove>().map_err(|e| {
                wasm_error!(format!("Could not convert record to GameMove: {:?}", e))
            }).unwrap().unwrap();
            return entry;
        }
    ).into_iter().collect::<Vec<GameMove>>();
    let game_moves: &[GameMove] = &game_moves;
    let board = Board::reconstruct_from_game_moves(game_moves);
    debug!("board: {:?}", board);
    let board_bytes = board.to_bytes();
    Ok(board_bytes)
}

#[hdk_extern]
pub fn get_board_at_move(move_action_hash: ActionHash) -> ExternResult<BoardWithMetadata> {
    let mut all_game_moves = get_all_game_moves(())?;
    let creator = all_game_moves[all_game_moves.len() - 1].action().author().clone();

    if let Some(pos) = all_game_moves.clone().iter().position(|record| record.action_hashed().as_hash().eq(&move_action_hash)) {
        all_game_moves.truncate(pos + 1)
    } else {
        return Err(wasm_error!("Could not find a game move for that action hash"));
    }

    // convert the records into a vec of game moves
    let game_moves = all_game_moves.into_iter().map(|record| {
            let entry = record.entry.to_app_option::<GameMove>().map_err(|e| {
                wasm_error!(format!("Could not convert record to GameMove: {:?}", e))
            }).unwrap().unwrap();
            return entry;
        }
    ).into_iter().collect::<Vec<GameMove>>();

    let game_moves: &[GameMove] = &game_moves;
    let board = Board::reconstruct_from_game_moves(game_moves);
    let board_bytes = board.to_bytes();
    
    Ok(
        BoardWithMetadata {
            bytes: board_bytes,
            creator,
            creationHash: move_action_hash,
        }
    )
}

#[hdk_extern]
pub fn get_boards_from_moves(move_action_hashes: Vec<ActionHash>) -> ExternResult<Vec<BoardWithMetadata>> {
    let mut boards = Vec::new();
    for move_action_hash in move_action_hashes {
        let board = get_board_at_move(move_action_hash)?;
        boards.push(board);
    }
    Ok(boards)
}

#[hdk_extern]
pub fn get_boards_from_all_my_moves(_:()) -> ExternResult<Vec<BoardWithMetadata>> {
    let moves = get_all_my_game_moves(())?;
    let action_hashes = moves.into_iter().map(|record| {
        record.action_hashed().as_hash().clone()
    }).collect::<Vec<ActionHash>>();
    get_boards_from_moves(action_hashes)
}

#[hdk_extern]
pub fn get_board_from_link(base: ExternalHash) -> ExternResult<BoardWithMetadataAndId> {
    let links = get_links(base.clone(), LinkTypes::TokenIdToGameMove, None)?;
    if let 0 = links.len() {
        return Err(wasm_error!("No game moves found for that token id"));
    }
    let action_hash = links[0].target.clone();
    let board_with_creator = get_board_at_move(action_hash.into())?;
    let id = base.get_raw_32().to_vec();
    Ok(BoardWithMetadataAndId {
        board: board_with_creator,
        id,
    })
}

#[hdk_extern]
pub fn get_boards_from_links(link_bases: Vec<ExternalHash>) -> ExternResult<Vec<BoardWithMetadataAndId>> {
    let mut boards = Vec::new();
    for link_base in link_bases {
        if let Ok(board) = get_board_from_link(link_base) {
            boards.push(board);
        }
    }
    Ok(boards)
}