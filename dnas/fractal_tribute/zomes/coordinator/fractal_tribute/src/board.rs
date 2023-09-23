use hdk::prelude::*;
use fractal_tribute_integrity::*;
use serde::de;
use crate::all_game_moves::*;
use resvg::*;
use resvg::usvg::TreeParsing;
use resvg::usvg::TreeWriting;
use usvg::XmlOptions;
use tiny_skia::*;
use base64::{Engine as _, engine::general_purpose};

pub fn svg_to_png(svg_data: String) -> ExternResult<String> {
    // Parse the SVG string
    let opts = resvg::usvg::Options::default();
    let utree = usvg::Tree::from_str(&svg_data, &opts)
        .map_err(|e| wasm_error!(format!("Could not parse SVG: {:?} {}", e, svg_data)))?;
    
    let rtree = resvg::Tree::from_usvg(&utree);
    wasm_error!("{:?}", utree.to_string(&XmlOptions::default()));

    // Create a pixmap for rendering
    let mut binding = Pixmap::new(rtree.size.width() as u32, rtree.size.height() as u32)
        .ok_or_else(|| wasm_error!("Could not create Pixmap"))?;
    let mut pixmap = binding.as_mut();
    
    rtree.render(Transform::from_scale(1.0, 1.0), &mut pixmap);

    let pixmap = pixmap.to_owned();

    // Encode the pixmap as a PNG image
    let png_bytes = pixmap.encode_png()
        .map_err(|e| wasm_error!(format!("Could not encode PNG: {:?}", e)))?;
    let base64_encoded = general_purpose::URL_SAFE_NO_PAD.encode(png_bytes);
    let data_uri = format!("data:image/png;base64,{}", base64_encoded);
    Ok(data_uri)
}

#[hdk_extern]
pub fn get_latest_board(_: ()) -> ExternResult<BoardWithMetadata> {
    let game_moves = get_all_game_moves_from_link_tags(())?;
    println!("{:?}", game_moves);
    // let creator = if all_game_moves.is_empty() {
    //     HoloHash::from_raw_36([0; 36].to_vec())
    // } else {
    //     all_game_moves[all_game_moves.len() - 1].action().author().clone()
    // };

    // let creation_hash = if all_game_moves.is_empty() {
    //     HoloHash::from_raw_36([0; 36].to_vec())
    // } else {
    //     all_game_moves[all_game_moves.len() - 1].action_hashed().as_hash().clone()
    // };

    let board = Board::reconstruct_from_game_moves(&game_moves);
    let bytes = board.to_bytes();
    let svg = board.generate_svg_data_uri();
    // let png = svg_to_png(board.generate_svg())?;

    let creator = HoloHash::from_raw_36([0; 36].to_vec());
    let creation_hash = HoloHash::from_raw_36([0; 36].to_vec());

    Ok(
        BoardWithMetadata {
            // png,
            svg,
            bytes,
            creator,
            creation_hash,
        }
    )
}

// #[hdk_extern]
// pub fn get_latest_board(_: ()) -> ExternResult<BoardWithMetadata> {
//     let all_game_moves = get_all_game_moves(())?;

//     let creator = if all_game_moves.is_empty() {
//         HoloHash::from_raw_36([0; 36].to_vec())
//     } else {
//         all_game_moves[all_game_moves.len() - 1].action().author().clone()
//     };

//     let creation_hash = if all_game_moves.is_empty() {
//         HoloHash::from_raw_36([0; 36].to_vec())
//     } else {
//         all_game_moves[all_game_moves.len() - 1].action_hashed().as_hash().clone()
//     };

//     // convert the records into a vec of game moves
//     let game_moves = all_game_moves.clone().into_iter().map(|record| {
//             let entry = record.entry.to_app_option::<GameMove>().map_err(|e| {
//                 wasm_error!(format!("Could not convert record to GameMove: {:?}", e))
//             }).unwrap().unwrap();
//             return entry;
//         }
//     ).into_iter().collect::<Vec<GameMove>>();

//     let board = Board::reconstruct_from_game_moves(&game_moves);
//     let bytes = board.to_bytes();
//     let svg = board.generate_svg_data_uri();

//     Ok(
//         BoardWithMetadata {
//             svg,
//             bytes,
//             creator,
//             creation_hash,
//         }
//     )
// }

#[hdk_extern]
pub fn get_board_at_move(move_action_hash: ActionHash) -> ExternResult<BoardWithMetadata> {
    let time = sys_time()?;
    // debug!("{:?}: starting", move_action_hash);
    // let mut game_moves = get_all_game_moves_from_link_tags(())?;

    let path = hdk::hash_path::path::Path::from("all_game_moves");
    let links: Vec<Link> = get_links(path.path_entry_hash()?, LinkTypes::AllGameMoves, None)?;

    // truncate the links
    let mut links = links.clone();
    if let Some(pos) = links.clone().iter().position(|link| link.target.clone().into_action_hash().unwrap().eq(&move_action_hash)) {
        links.truncate(pos + 1)
    } else {
        return Err(wasm_error!("Could not find a game move for that action hash"));
    }

    // get the bytes from each link tag and make a vector of game moves
    let game_moves: Vec<GameMove> = links.clone()
        .into_iter()
        .map(|link| {
            let bytes = link.tag.0;
            let game_move = GameMove::from_bytes(&bytes).ok().unwrap();
            Ok(game_move)
        })
        .collect::<ExternResult<Vec<GameMove>>>()?;

    // get the author of the last game move link
    let creator = if links.is_empty() {
        HoloHash::from_raw_36([0; 36].to_vec())
    } else {
        links[links.len() - 1].author.clone()
    };

    // let creator = all_game_moves[all_game_moves.len() - 1].action().author().clone();

    // if let Some(pos) = all_game_moves.clone().iter().position(|record| record.action_hashed().as_hash().eq(&move_action_hash)) {
    //     all_game_moves.truncate(pos + 1)
    // } else {
    //     return Err(wasm_error!("Could not find a game move for that action hash"));
    // }
    // // convert the records into a vec of game moves
    // let game_moves = all_game_moves.into_iter().map(|record| {
    //         let entry = record.entry.to_app_option::<GameMove>().map_err(|e| {
    //             wasm_error!(format!("Could not convert record to GameMove: {:?}", e))
    //         }).unwrap().unwrap();
    //         return entry;
    //     }
    // ).into_iter().collect::<Vec<GameMove>>();

    // let game_moves: &[GameMove] = &game_moves;
    let mut new_time = sys_time()?;
    debug!("{:?}: got game moves in {:?}", move_action_hash, new_time - time);
    let board = Board::reconstruct_from_game_moves(&game_moves);
    let bytes = board.to_bytes();
    new_time = sys_time()?;
    debug!("{:?}: got bytes in {:?}", move_action_hash, new_time - time);
    let svg = board.generate_svg_data_uri();
    // let png = svg_to_png(board.generate_svg())?;
    new_time = sys_time()?;
    debug!("{:?}: got svg in {:?}", move_action_hash, new_time - time);
    Ok(
        BoardWithMetadata {
            // png,
            svg,
            bytes,
            creator,
            creation_hash: move_action_hash,
        }
    )
}

// #[hdk_extern]
// pub fn get_board_at_move(move_action_hash: ActionHash) -> ExternResult<BoardWithMetadata> {
//     let time = sys_time()?;
//     debug!("{:?}: starting", move_action_hash);
//     let mut all_game_moves = get_all_game_moves(())?;
//     let creator = all_game_moves[all_game_moves.len() - 1].action().author().clone();

//     if let Some(pos) = all_game_moves.clone().iter().position(|record| record.action_hashed().as_hash().eq(&move_action_hash)) {
//         all_game_moves.truncate(pos + 1)
//     } else {
//         return Err(wasm_error!("Could not find a game move for that action hash"));
//     }
//     // convert the records into a vec of game moves
//     let game_moves = all_game_moves.into_iter().map(|record| {
//             let entry = record.entry.to_app_option::<GameMove>().map_err(|e| {
//                 wasm_error!(format!("Could not convert record to GameMove: {:?}", e))
//             }).unwrap().unwrap();
//             return entry;
//         }
//     ).into_iter().collect::<Vec<GameMove>>();

//     let game_moves: &[GameMove] = &game_moves;
//     let mut new_time = sys_time()?;
//     debug!("{:?}: got game moves in {:?}", move_action_hash, new_time - time);
//     let board = Board::reconstruct_from_game_moves(game_moves);
//     let bytes = board.to_bytes();
//     new_time = sys_time()?;
//     debug!("{:?}: got bytes in {:?}", move_action_hash, new_time - time);
//     let svg = board.generate_svg_data_uri();
//     new_time = sys_time()?;
//     debug!("{:?}: got svg in {:?}", move_action_hash, new_time - time);
//     Ok(
//         BoardWithMetadata {
//             svg,
//             bytes,
//             creator,
//             creation_hash: move_action_hash,
//         }
//     )
// }

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