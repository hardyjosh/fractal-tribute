use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Payload {
    pub name: String,
    pub description: String,
}
pub fn validate_create_payload(
    _action: EntryCreationAction,
    _payload: Payload,
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
pub fn validate_update_payload(
    _action: Update,
    _payload: Payload,
    _original_action: EntryCreationAction,
    _original_payload: Payload,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Payloads cannot be updated")))
}
pub fn validate_delete_payload(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_payload: Payload,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Payloads cannot be deleted")))
}
pub fn validate_create_link_tokenid_to_payloads(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _payload: crate::Payload = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_tokenid_to_payloads(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("TokenIdToPayloads links cannot be deleted"),
        ),
    )
}

// pub fn validate_create_link_creator_to_payloads(
//     _action: CreateLink,
//     _base_address: AnyLinkableHash,
//     target_address: AnyLinkableHash,
//     _tag: LinkTag,
// ) -> ExternResult<ValidateCallbackResult> {
//     let action_hash = ActionHash::from(target_address);
//     let record = must_get_valid_record(action_hash)?;
//     let _payload: crate::Payload = record
//         .entry()
//         .to_app_option()
//         .map_err(|e| wasm_error!(e))?
//         .ok_or(
//             wasm_error!(
//                 WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
//             ),
//         )?;
//     Ok(ValidateCallbackResult::Valid)
// }
// pub fn validate_delete_link_creator_to_payloads(
//     _action: DeleteLink,
//     _original_action: CreateLink,
//     _base: AnyLinkableHash,
//     _target: AnyLinkableHash,
//     _tag: LinkTag,
// ) -> ExternResult<ValidateCallbackResult> {
//     Ok(
//         ValidateCallbackResult::Invalid(
//             String::from("CreatorToPayloads links cannot be deleted"),
//         ),
//     )
// }
// pub fn validate_create_link_all_payloads(
//     _action: CreateLink,
//     _base_address: AnyLinkableHash,
//     target_address: AnyLinkableHash,
//     _tag: LinkTag,
// ) -> ExternResult<ValidateCallbackResult> {
//     let action_hash = ActionHash::from(target_address);
//     let record = must_get_valid_record(action_hash)?;
//     let _payload: crate::Payload = record
//         .entry()
//         .to_app_option()
//         .map_err(|e| wasm_error!(e))?
//         .ok_or(
//             wasm_error!(
//                 WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
//             ),
//         )?;
//     Ok(ValidateCallbackResult::Valid)
// }
// pub fn validate_delete_link_all_payloads(
//     _action: DeleteLink,
//     _original_action: CreateLink,
//     _base: AnyLinkableHash,
//     _target: AnyLinkableHash,
//     _tag: LinkTag,
// ) -> ExternResult<ValidateCallbackResult> {
//     Ok(
//         ValidateCallbackResult::Invalid(
//             String::from("AllPayloads links cannot be deleted"),
//         ),
//     )
// }
// pub fn validate_create_link_payloads_by_creator(
//     _action: CreateLink,
//     _base_address: AnyLinkableHash,
//     target_address: AnyLinkableHash,
//     _tag: LinkTag,
// ) -> ExternResult<ValidateCallbackResult> {
//     let action_hash = ActionHash::from(target_address);
//     let record = must_get_valid_record(action_hash)?;
//     let _payload: crate::Payload = record
//         .entry()
//         .to_app_option()
//         .map_err(|e| wasm_error!(e))?
//         .ok_or(
//             wasm_error!(
//                 WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
//             ),
//         )?;
//     Ok(ValidateCallbackResult::Valid)
// }
// pub fn validate_delete_link_payloads_by_creator(
//     _action: DeleteLink,
//     _original_action: CreateLink,
//     _base: AnyLinkableHash,
//     _target: AnyLinkableHash,
//     _tag: LinkTag,
// ) -> ExternResult<ValidateCallbackResult> {
//     Ok(
//         ValidateCallbackResult::Invalid(
//             String::from("PayloadsByCreator links cannot be deleted"),
//         ),
//     )
// }
