use hdi::prelude::*;
use ethers_core::utils::keccak256;

use crate::{ByteArray, EvmKeyBinding};
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Payload {
    pub payload_bytes: ByteArray,
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

    // we need to validate that the link base really is a hash of the author's evm key + payload hash
    let payload_bytes = _payload.payload_bytes.into_vec();

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
    let evm_key_bytes = evm_key_binding.evm_key.into_vec();

    let valid_link_base = create_link_base(evm_key_bytes, payload_bytes).unwrap();
    
    let base = _base_address.as_hash().clone().into_external_hash().unwrap();
    if valid_link_base != base {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("Link base address does not match a hash derived from the agent's EVM key and the content payload bytes"),
            ),
        )
    }

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

pub fn create_link_base(evm_key: Vec<u8>, content_bytes: Vec<u8>) -> Result<ExternalHash, WasmError> {
    if evm_key.len() != 20 {
        return Err(wasm_error!(WasmErrorInner::Guest("EVM key must be 20 bytes".to_string())));
    }

    // create the final bytes for hashing - EVM key as 32 bytes + content hash
    let mut key_bytes:Vec<u8> = vec![0; 12];
    key_bytes.append(&mut evm_key.clone().to_vec());

    // hash the content bytes and append to the key bytes
    let content_hash = keccak256(content_bytes);
    key_bytes.extend_from_slice(&content_hash);

    // hash the key + content hash
    let hash_input = key_bytes;
    let mut hash: Vec<u8> = keccak256(hash_input).try_into()?;

    // resize the 32 byte keccak hash to 36 bytes so we can use it as a Holohash
    // @todo the extra 4 bytes should be a derived location for sharding
    hash.resize(36, 0);
    let link_base = ExternalHash::from_raw_36(hash);
    return Ok(link_base);
}

#[cfg(test)]
pub mod create_link_base_tests {
    use super::*;

    // test that create_link_base errors when the EVM key is not 20 bytes
    #[test]
    fn test_create_link_base_invalid_evm_key() {
        let evm_key = vec![0; 21];
        let content_bytes = vec![1; 100];
        let link_base = create_link_base(evm_key, content_bytes);

        let expected_error = wasm_error!(WasmErrorInner::Guest(String::from("EVM key must be 20 bytes")));
        assert_eq!(link_base.map_err(|e| e.error).unwrap_err(), expected_error.error);
    }

    // test that the content bytes can be of any length and the function won't panic
    #[test]
    fn test_create_link_base_content_bytes_length() {
        let evm_key = vec![0; 20];

        let content_bytes = vec![1; 100];
        let link_base = create_link_base(evm_key.clone(), content_bytes);

        assert!(link_base.is_ok());

        let content_bytes = vec![1; 1000000];
        let link_base = create_link_base(evm_key.clone(), content_bytes);

        assert!(link_base.is_ok());

        let content_bytes = vec![1; 0];
        let link_base = create_link_base(evm_key.clone(), content_bytes);

        assert!(link_base.is_ok());
    }

    // test that the link base is created correctly
    #[test]
    fn test_create_link_base() {
        // 20 random bytes for the evm key
        let rand_evm_key = vec![5; 20];
        // 200 random bytes
        let content_bytes = vec![9; 200];
        // hashed content bytes
        let content_hash: Vec<u8> = keccak256(content_bytes.clone()).try_into().unwrap();
        // set up the bytes for hashing
        let mut final_for_hashing:Vec<u8> = Vec::new();
        final_for_hashing.extend_from_slice(&vec![0; 12]);
        final_for_hashing.extend_from_slice(&rand_evm_key);
        final_for_hashing.extend_from_slice(&content_hash);
        // hashed final bytes
        let mut final_hash: Vec<u8> = keccak256(final_for_hashing).try_into().unwrap();
        final_hash.extend_from_slice(& vec![0; 4]);
        let expected_link_base = ExternalHash::from_raw_36(final_hash);

        let link_base = create_link_base(rand_evm_key, content_bytes).unwrap();

        assert_eq!(link_base, expected_link_base);
    }
}

// #[cfg(test)]
// pub mod validate_create_link_tokenid_to_payloads_tests {
//     use super::*;
//     use hdk::prelude::*;
//     use holochain_integrity_types::*;

//     // test that validate_create_link_tokenid_to_payloads returns Invalid when the base address does not match the expected hash
//     #[test]
//     fn test_validate_create_link_tokenid_to_payloads_invalid_base_address() {
//         let mut mock_hdk = MockHdkT::new();

//         // let create_link:CreateLink = fixt!(CreateLink);

//     }
// }