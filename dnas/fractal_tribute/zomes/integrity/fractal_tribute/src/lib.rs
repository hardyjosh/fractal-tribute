pub mod evm_key_binding;
pub use evm_key_binding::*;
pub mod game_move;
pub use game_move::*;
pub mod board;
pub use board::*;
pub mod participation_proof;
pub use participation_proof::*;

use hdi::prelude::*;
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    GameMove(GameMove),
    EvmKeyBinding(EvmKeyBinding),
    ParticipationProof(ParticipationProof)
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    TokenIdToGameMove,
    AllGameMoves,
    AgentToEvmKeyBinding
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkBaseInput {
    pub evm_key: Vec<u8>,
    pub content_bytes: Vec<u8>,
}

#[hdk_extern]
pub fn genesis_self_check(
    _data: GenesisSelfCheckData,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.to_type::<EntryTypes, LinkTypes>()? {
        OpType::StoreEntry(store_entry) => {
            match store_entry {
                OpEntry::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::GameMove(game_move) => {
                            validate_create_game_move(
                                EntryCreationAction::Create(action),
                                game_move,
                            )
                        }
                        EntryTypes::EvmKeyBinding(evm_key_binding) => {
                            validate_create_evm_key_binding(
                                EntryCreationAction::Create(action),
                                evm_key_binding,
                            )
                        }
                        EntryTypes::ParticipationProof(participation_proof) => {
                            validate_create_participation_proof(
                                EntryCreationAction::Create(action),
                                participation_proof,
                            )
                        }
                    }
                }
                OpEntry::UpdateEntry { app_entry, action, .. } => {
                    match app_entry {
                        EntryTypes::GameMove(game_move) => {
                            validate_create_game_move(
                                EntryCreationAction::Update(action),
                                game_move,
                            )
                        }
                        EntryTypes::EvmKeyBinding(evm_key_binding) => {
                            validate_create_evm_key_binding(
                                EntryCreationAction::Update(action),
                                evm_key_binding,
                            )
                        }
                        EntryTypes::ParticipationProof(participation_proof) => {
                            validate_create_participation_proof(
                                EntryCreationAction::Update(action),
                                participation_proof,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        OpType::RegisterUpdate(update_entry) => {
            match update_entry {
                OpUpdate::Entry {
                    original_action,
                    original_app_entry,
                    app_entry,
                    action,
                } => {
                    match (app_entry, original_app_entry) {
                        (
                            EntryTypes::EvmKeyBinding(evm_key_binding),
                            EntryTypes::EvmKeyBinding(original_evm_key_binding),
                        ) => {
                            validate_update_evm_key_binding(
                                action,
                                evm_key_binding,
                                original_action,
                                original_evm_key_binding,
                            )
                        }
                        (
                            EntryTypes::GameMove(game_move),
                            EntryTypes::GameMove(original_game_move),
                        ) => {
                            validate_update_game_move(
                                action,
                                game_move,
                                original_action,
                                original_game_move,
                            )
                        }
                        (
                            EntryTypes::ParticipationProof(participation_proof),
                            EntryTypes::ParticipationProof(original_participation_proof),
                        ) => {
                            validate_update_participation_proof(
                                action,
                                participation_proof,
                                original_action,
                                original_participation_proof,
                            )
                        }
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original and updated entry types must be the same"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        OpType::RegisterDelete(delete_entry) => {
            match delete_entry {
                OpDelete::Entry { original_action, original_app_entry, action } => {
                    match original_app_entry {
                        EntryTypes::GameMove(game_move) => {
                            validate_delete_game_move(action, original_action, game_move)
                        }
                        EntryTypes::EvmKeyBinding(evm_key_binding) => {
                            validate_delete_evm_key_binding(
                                action,
                                original_action,
                                evm_key_binding,
                            )
                        }
                        EntryTypes::ParticipationProof(participation_proof) => {
                            validate_delete_participation_proof(
                                action,
                                original_action,
                                participation_proof,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        OpType::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        } => {
            match link_type {
                LinkTypes::TokenIdToGameMove => {
                    validate_create_link_tokenid_to_game_move(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllGameMoves => {
                    validate_create_link_all_game_moves(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AgentToEvmKeyBinding => {
                    validate_create_link_agent_to_evm_key_binding(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        OpType::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => {
            match link_type {
                LinkTypes::TokenIdToGameMove => {
                    validate_delete_link_tokenid_to_game_move(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllGameMoves => {
                    validate_delete_link_all_game_moves(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AgentToEvmKeyBinding => {
                    validate_delete_link_agent_to_evm_key_binding(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        OpType::StoreRecord(store_record) => {
            match store_record {
                OpRecord::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::GameMove(game_move) => {
                            validate_create_game_move(
                                EntryCreationAction::Create(action),
                                game_move,
                            )
                        }
                        EntryTypes::EvmKeyBinding(evm_key_binding) => {
                            validate_create_evm_key_binding(
                                EntryCreationAction::Create(action),
                                evm_key_binding,
                            )
                        }
                        EntryTypes::ParticipationProof(participation_proof) => {
                            validate_create_participation_proof(
                                EntryCreationAction::Create(action),
                                participation_proof,
                            )
                        }
                    }
                }
                OpRecord::UpdateEntry {
                    original_action_hash,
                    app_entry,
                    action,
                    ..
                } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for an update must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match app_entry {
                        EntryTypes::GameMove(game_move) => {
                            let result = validate_create_game_move(
                                EntryCreationAction::Update(action.clone()),
                                game_move.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_game_move: Option<GameMove> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_game_move = match original_game_move {
                                    Some(game_move) => game_move,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_game_move(
                                    action,
                                    game_move,
                                    original_action,
                                    original_game_move,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::EvmKeyBinding(evm_key_binding) => {
                            let result = validate_create_evm_key_binding(
                                EntryCreationAction::Update(action.clone()),
                                evm_key_binding.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_evm_key_binding: Option<EvmKeyBinding> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_evm_key_binding = match original_evm_key_binding {
                                    Some(evm_key_binding) => evm_key_binding,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_evm_key_binding(
                                    action,
                                    evm_key_binding,
                                    original_action,
                                    original_evm_key_binding,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ParticipationProof(participation_proof) => {
                            let result = validate_create_participation_proof(
                                EntryCreationAction::Update(action.clone()),
                                participation_proof.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_participation_proof: Option<ParticipationProof> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_participation_proof = match original_participation_proof {
                                    Some(participation_proof) => participation_proof,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_participation_proof(
                                    action,
                                    participation_proof,
                                    original_action,
                                    original_participation_proof,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                    }
                }
                OpRecord::DeleteEntry { original_action_hash, action, .. } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for a delete must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let app_entry_type = match original_action.entry_type() {
                        EntryType::App(app_entry_type) => app_entry_type,
                        _ => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    let entry = match original_record.entry().as_option() {
                        Some(entry) => entry,
                        None => {
                            if original_action.entry_type().visibility().is_public() {
                                return Ok(
                                    ValidateCallbackResult::Invalid(
                                        "Original record for a delete of a public entry must contain an entry"
                                            .to_string(),
                                    ),
                                );
                            } else {
                                return Ok(ValidateCallbackResult::Valid);
                            }
                        }
                    };
                    let original_app_entry = match EntryTypes::deserialize_from_type(
                        app_entry_type.zome_index.clone(),
                        app_entry_type.entry_index.clone(),
                        &entry,
                    )? {
                        Some(app_entry) => app_entry,
                        None => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match original_app_entry {
                        EntryTypes::GameMove(original_game_move) => {
                            validate_delete_game_move(
                                action,
                                original_action,
                                original_game_move,
                            )
                        }
                        EntryTypes::EvmKeyBinding(original_evm_key_binding) => {
                            validate_delete_evm_key_binding(
                                action,
                                original_action,
                                original_evm_key_binding,
                            )
                        }
                        EntryTypes::ParticipationProof(original_participation_proof) => {
                            validate_delete_participation_proof(
                                action,
                                original_action,
                                original_participation_proof,
                            )
                        }
                    }
                }
                OpRecord::CreateLink {
                    base_address,
                    target_address,
                    tag,
                    link_type,
                    action,
                } => {
                    match link_type {
                        LinkTypes::TokenIdToGameMove => {
                            validate_create_link_tokenid_to_game_move(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllGameMoves => {
                            validate_create_link_all_game_moves(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AgentToEvmKeyBinding => {
                            validate_create_link_agent_to_evm_key_binding(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                    }
                }
                OpRecord::DeleteLink { original_action_hash, base_address, action } => {
                    let record = must_get_valid_record(original_action_hash)?;
                    let create_link = match record.action() {
                        Action::CreateLink(create_link) => create_link.clone(),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "The action that a DeleteLink deletes must be a CreateLink"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let link_type = match LinkTypes::from_type(
                        create_link.zome_index.clone(),
                        create_link.link_type.clone(),
                    )? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    match link_type {
                        LinkTypes::TokenIdToGameMove => {
                            validate_delete_link_tokenid_to_game_move(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllGameMoves => {
                            validate_delete_link_all_game_moves(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AgentToEvmKeyBinding => {
                            validate_delete_link_agent_to_evm_key_binding(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                    }
                }
                OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        OpType::RegisterAgentActivity(agent_activity) => {
            match agent_activity {
                OpActivity::CreateAgent { agent, action } => {
                    let previous_action = must_get_action(action.prev_action)?;
                    match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
    }
}
