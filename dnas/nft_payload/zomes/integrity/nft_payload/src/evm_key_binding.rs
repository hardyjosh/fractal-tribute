use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct EvmKeyBinding {
    pub evm_key: u32,
    pub creator: AgentPubKey,
}
pub fn validate_create_evm_key_binding(
    _action: EntryCreationAction,
    _evm_key_binding: EvmKeyBinding,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_evm_key_binding(
    _action: Update,
    _evm_key_binding: EvmKeyBinding,
    _original_action: EntryCreationAction,
    _original_evm_key_binding: EvmKeyBinding,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Evm Key Bindings cannot be updated"),
        ),
    )
}
pub fn validate_delete_evm_key_binding(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_evm_key_binding: EvmKeyBinding,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Evm Key Bindings cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_creator_to_evm_key_bindings(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _evm_key_binding: crate::EvmKeyBinding = record
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
pub fn validate_delete_link_creator_to_evm_key_bindings(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("CreatorToEvmKeyBindings links cannot be deleted"),
        ),
    )
}
