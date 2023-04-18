use hdi::prelude::*;
use crate::ByteArray;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct EvmKeyBinding {
    pub evm_key: ByteArray,
}

pub fn validate_create_evm_key_binding(
    _action: EntryCreationAction,
    _evm_key_binding: EvmKeyBinding,
) -> ExternResult<ValidateCallbackResult> {

    if *_action.action_seq() != 4u32 {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("EVM pubkey binding must be the first action after genesis"),
            ),
        )
    }
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