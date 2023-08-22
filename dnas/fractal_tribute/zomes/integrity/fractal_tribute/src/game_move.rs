use hdi::prelude::*;
use crate::*;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct GameMove {
    pub changes: Vec<PixelChange>,  // Contains up to 10 PixelChanges
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct PixelChange {
    pub x: usize,
    pub y: usize,
    pub color: Color,
    pub graphic_option: u8,  // Assuming range 0-15 for 16 graphic options
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Tile {
    color: Option<Color>,
    graphic_option: Option<u8>,
}

impl GameMove {
    // Method to count the number of changes
    pub fn count_changes(&self) -> usize {
        self.changes.len()
    }

    // Parsing a game_move from a dynamic-length byte array.
    // If the byte array's length is not a multiple of 4, we'll return an error.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() % 4 != 0 {
            return Err("Invalid length");
        }

        let num_changes = bytes.len() / 4;

        if (num_changes > 10) {
            return Err("Maximum of 10 pixel changes per move")
        }

        let mut changes = Vec::with_capacity(num_changes);

        for i in 0..num_changes {
            let start = i * 4;
            
            // Extracting x and y
            let x = (bytes[start] & 0b11111) as usize;
            let y = (bytes[start + 1] >> 3) as usize;

            // Extracting color values based on our encoding scheme
            let r = ((bytes[start + 1] & 0b00000111) << 5) | (bytes[start + 2] >> 3);
            let g = ((bytes[start + 2] & 0b00000111) << 5) | (bytes[start + 3] >> 3);
            let b = bytes[start + 3] & 0b00000111;
            let color = Color { r, g, b };
            
            // Extracting graphic option
            let graphic_option = bytes[start] >> 5; // Assuming the first 3 bits of the first byte

            changes.push(PixelChange { x, y, color, graphic_option });
        }

        Ok(GameMove { changes })
    }
    
    // Returns a dynamic-length byte array.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.changes.len() * 4);

        for change in &self.changes {
            let start = bytes.len();

            // Encoding x and graphic option
            bytes.push((change.x as u8 & 0b11111) | (change.graphic_option << 5));

            // Encoding y and a part of red
            bytes.push(((change.y as u8 & 0b11111) << 3) | (change.color.r >> 5));

            // Continuing with red and then encoding green
            bytes.push(((change.color.r & 0b00000111) << 3) | (change.color.g >> 5));

            // Continuing with green and then encoding blue
            bytes.push(((change.color.g & 0b00000111) << 3) | (change.color.b >> 5));
        }

        bytes
    }
}

pub fn validate_create_game_move(
    _action: EntryCreationAction,
    _game_move: GameMove,
) -> ExternResult<ValidateCallbackResult> {
    if *_action.action_seq() < 5u32 {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("EVM pubkey binding must be the first action after genesis"),
            ),
        )
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_game_move(
    _action: Update,
    _game_move: GameMove,
    _original_action: EntryCreationAction,
    _original_game_move: GameMove,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("GameMoves cannot be updated")))
}
pub fn validate_delete_game_move(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_game_move: GameMove,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("GameMoves cannot be deleted")))
}
pub fn validate_create_link_tokenid_to_game_move(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash.clone())?;
    let _game_move: crate::GameMove = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;

    // we need to validate that the link base really is a hash of the author's evm key + game_move hash
    let game_move_bytes = action_hash.get_raw_39().to_vec();

    // get the link author's evm key
    let filter = ChainFilter::new(_action.prev_action).include_cached_entries();
    let mut activity = must_get_agent_activity(_action.author, filter)?;
    activity.reverse();
    let record_action = activity.get(4).ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("No entry #4 in the source chain"))
    ))?.action.as_hash().clone();
    let record = must_get_valid_record(record_action)?;
    let evm_key_binding: EvmKeyBinding = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Couldn't convert entry #4 to EVM key binding"))
            ),
        )?;
    let evm_key_bytes = evm_key_binding.evm_key;

    let valid_link_base = create_link_base(evm_key_bytes, game_move_bytes).unwrap();
    
    let base = _base_address.as_hash().clone().into_external_hash().unwrap();
    if valid_link_base != base {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("Link base address does not match a hash derived from the agent's EVM key and the content game_move bytes"),
            ),
        )
    }

    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_tokenid_to_game_move(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("TokenIdToGameMove links cannot be deleted"),
        ),
    )
}

pub fn create_link_base(evm_key: Vec<u8>, content_bytes: Vec<u8>) -> Result<ExternalHash, WasmError> {
    if evm_key.len() != 20 {
        return Err(wasm_error!(WasmErrorInner::Guest("EVM key must be 20 bytes".to_string())));
    }
    debug!("actionhash bytes: {:?}", content_bytes);
    
    // EVM key as 32 bytes
    let mut hash_input:Vec<u8> = vec![0; 12];
    hash_input.extend_from_slice(&evm_key);

    // hash the content bytes and append to the key bytes
    let content_hash = hash_keccak256(content_bytes).unwrap();
    debug!("hashed actionhash {:?}", content_hash);
    hash_input.extend_from_slice(&content_hash);

    // hash the key + content hash
    let mut hash: Vec<u8> = hash_keccak256(hash_input).unwrap();

    // resize the 32 byte keccak hash to 36 bytes so we can use it as a Holohash
    // @todo the extra 4 bytes should be a derived location for sharding
    hash.resize(36, 0);
    let link_base = ExternalHash::from_raw_36(hash);
    return Ok(link_base);
}

pub fn validate_create_link_all_game_moves(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _game_move: crate::GameMove = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_game_moves(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("All game moves links cannot be deleted")))
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use holochain::sweettest::*;
//     use futures::future;

//     const DNA_FILEPATH: &str = "../../../workdir/nft_payload.dna";

//     pub mod create_link_base_tests {
//         use crate::LinkBaseInput;

//         async fn _create_link_base(
//             conductor: &SweetConductor,
//             alice: &SweetCell,
//             evm_key: Vec<u8>, 
//             content_bytes: Vec<u8>
//         ) -> ExternalHash {
//             let link_base_input = LinkBaseInput {
//                 evm_key: evm_key,
//                 content_bytes: content_bytes,
//             };
//             let link_base: ExternalHash = conductor
//                 .call(
//                     &alice.zome("nft_payload"), 
//                     "extern_create_link_base", 
//                     link_base_input
//                 ).await;
//             return link_base;
//         }

//         async fn _hash(
//             conductor: &SweetConductor,
//             alice: &SweetCell,
//             input: Vec<u8>,
//         ) -> Vec<u8> {
//             let hash: Vec<u8> = conductor
//                 .call(
//                     &alice.zome("nft_payload"), 
//                     "hash", 
//                     input
//                 ).await;
//             return hash;
//         }

//         use super::*;
//         // test that create_link_base errors when the EVM key is not 20 bytes
//         #[tokio::test(flavor = "multi_thread")]
//         #[should_panic(expected = "EVM key must be 20 bytes")]
//         async fn test_create_link_base_invalid_evm_key() {
//             let (conductors, _agents, apps) = setup_conductors(1).await;
//             let conductor: &SweetConductor = &conductors[0];
//             let cells = apps.cells_flattened();
//             let alice = &cells[0];

//             let evm_key = vec![0; 21];
//             let content_bytes = vec![1; 100];
//             let _link_base = _create_link_base(conductor, alice, evm_key, content_bytes).await;
//         }

//         // test that the content bytes can be of any length and the function won't panic
//         #[tokio::test(flavor = "multi_thread")]
//         async fn test_create_link_base_content_bytes_length() {
//             let (conductors, _agents, apps) = setup_conductors(1).await;
//             let conductor: &SweetConductor = &conductors[0];
//             let cells = apps.cells_flattened();
//             let alice = &cells[0];

//             let evm_key = vec![0; 20];

//             let content_bytes = vec![1; 100];
//             let _link_base = _create_link_base(conductor, alice, evm_key.clone(), content_bytes).await;

//             let content_bytes = vec![1; 1000000];
//             let _link_base = _create_link_base(conductor, alice, evm_key.clone(), content_bytes).await;

//             let content_bytes = vec![1; 0];
//             let _link_base = _create_link_base(conductor, alice, evm_key.clone(), content_bytes).await;
//         }

//         #[tokio::test(flavor = "multi_thread")]
//         async fn test_create_link_base() {
//             let (conductors, _agents, apps) = setup_conductors(1).await;
//             let conductor: &SweetConductor = &conductors[0];
//             let cells = apps.cells_flattened();
//             let alice = &cells[0];
        
//             // 20 random bytes for the evm key
//             let rand_evm_key = vec![5; 20];
//             // 200 random bytes
//             let content_bytes = vec![9; 200];
//             // hashed content bytes
//             let content_hash = _hash(conductor, alice, content_bytes.clone()).await;
//             // set up the bytes for hashing
//             let mut final_for_hashing:Vec<u8> = Vec::new();
//             final_for_hashing.extend_from_slice(&vec![0; 12]);
//             final_for_hashing.extend_from_slice(&rand_evm_key);
//             final_for_hashing.extend_from_slice(&content_hash);
//             // hashed final bytes
//             let mut final_hash:Vec<u8> = _hash(conductor, alice, final_for_hashing.clone()).await;
//             final_hash.extend_from_slice(& vec![0; 4]);
//             let expected_link_base = ExternalHash::from_raw_36(final_hash);

//             let link_base = _create_link_base(conductor, alice, rand_evm_key.clone(), content_bytes.clone()).await;
  
//             assert_eq!(link_base, expected_link_base);
//         }
//     }

//     pub mod validate_create_link_tokenid_to_payloads_tests {
//         use super::*;

//         // test that validate_create_link_tokenid_to_payloads returns Invalid when the base address does not match the expected hash
//         #[tokio::test(flavor = "multi_thread")]
//         #[ignore]

//         async fn test_validate_create_link_tokenid_to_payloads_invalid_base_address() {
//             let (_conductors, _agents, _apps) = setup_conductors(2).await;
//         }

//     }

//     async fn setup_conductors(n: usize) -> (SweetConductorBatch, Vec<AgentPubKey>, SweetAppBatch) {
//         let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
//             .await
//             .unwrap();

//         let mut conductors = SweetConductorBatch::from_standard_config(n).await;

//         let all_agents: Vec<AgentPubKey> =
//             future::join_all(conductors.iter().map(|c| SweetAgents::one(c.keystore()))).await;
//         let apps = conductors
//             .setup_app_for_zipped_agents("app", &all_agents, &[dna])
//             .await
//             .unwrap();

//         conductors.exchange_peer_info().await;
//         (conductors, all_agents, apps)
//     }
// }