pub mod evm_key_binding;
pub use evm_key_binding::*;
pub mod payload;
pub use payload::*;
use hdi::prelude::*;
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Payload(Payload),
    EvmKeyBinding(EvmKeyBinding),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    TokenIdToPayload
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ByteArray(#[serde(with = "serde_bytes")] Vec<u8>);
impl ByteArray {
    // Add a public method to create a new ByteArray from a Vec<u8>
    pub fn new(vec: Vec<u8>) -> Self {
        ByteArray(vec)
    }
    
    // Add a public method to convert ByteArray into a Vec<u8>
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
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
                        EntryTypes::Payload(payload) => {
                            validate_create_payload(
                                EntryCreationAction::Create(action),
                                payload,
                            )
                        }
                        EntryTypes::EvmKeyBinding(evm_key_binding) => {
                            validate_create_evm_key_binding(
                                EntryCreationAction::Create(action),
                                evm_key_binding,
                            )
                        }
                    }
                }
                OpEntry::UpdateEntry { app_entry, action, .. } => {
                    match app_entry {
                        EntryTypes::Payload(payload) => {
                            validate_create_payload(
                                EntryCreationAction::Update(action),
                                payload,
                            )
                        }
                        EntryTypes::EvmKeyBinding(evm_key_binding) => {
                            validate_create_evm_key_binding(
                                EntryCreationAction::Update(action),
                                evm_key_binding,
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
                            EntryTypes::Payload(payload),
                            EntryTypes::Payload(original_payload),
                        ) => {
                            validate_update_payload(
                                action,
                                payload,
                                original_action,
                                original_payload,
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
                        EntryTypes::Payload(payload) => {
                            validate_delete_payload(action, original_action, payload)
                        }
                        EntryTypes::EvmKeyBinding(evm_key_binding) => {
                            validate_delete_evm_key_binding(
                                action,
                                original_action,
                                evm_key_binding,
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
                LinkTypes::TokenIdToPayload => {
                    validate_create_link_tokenid_to_payloads(
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
                LinkTypes::TokenIdToPayload => {
                    validate_delete_link_tokenid_to_payloads(
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
                        EntryTypes::Payload(payload) => {
                            validate_create_payload(
                                EntryCreationAction::Create(action),
                                payload,
                            )
                        }
                        EntryTypes::EvmKeyBinding(evm_key_binding) => {
                            validate_create_evm_key_binding(
                                EntryCreationAction::Create(action),
                                evm_key_binding,
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
                        EntryTypes::Payload(payload) => {
                            let result = validate_create_payload(
                                EntryCreationAction::Update(action.clone()),
                                payload.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_payload: Option<Payload> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_payload = match original_payload {
                                    Some(payload) => payload,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_payload(
                                    action,
                                    payload,
                                    original_action,
                                    original_payload,
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
                        EntryTypes::Payload(original_payload) => {
                            validate_delete_payload(
                                action,
                                original_action,
                                original_payload,
                            )
                        }
                        EntryTypes::EvmKeyBinding(original_evm_key_binding) => {
                            validate_delete_evm_key_binding(
                                action,
                                original_action,
                                original_evm_key_binding,
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
                        LinkTypes::TokenIdToPayload => {
                            validate_create_link_tokenid_to_payloads(
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
                        LinkTypes::TokenIdToPayload => {
                            validate_delete_link_tokenid_to_payloads(
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
