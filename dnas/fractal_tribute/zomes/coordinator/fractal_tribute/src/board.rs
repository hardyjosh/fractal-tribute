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
    let board_bytes = board.to_bytes();
    Ok(board_bytes)
}

#[hdk_extern]
pub fn get_board_at_move(move_action_hash: ActionHash) -> ExternResult<Vec<u8>> {
    let mut all_game_moves = get_all_game_moves(())?;

    if let Some(pos) = all_game_moves.iter().position(|record| record.action_hashed().as_hash().eq(&move_action_hash)) {
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
    
    Ok(board_bytes)
}

#[hdk_extern]
pub fn get_board_from_link(base: ExternalHash) -> ExternResult<Vec<u8>> {
    let links = get_links(base, LinkTypes::TokenIdToGameMove, None)?;
    let action_hash = links[0].target.clone();
    let board_bytes = get_board_at_move(action_hash.into())?;
    Ok(board_bytes)
}