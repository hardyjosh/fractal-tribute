use hdi::prelude::*;

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

    // EVM key as 32 bytes
    let mut hash_input:Vec<u8> = vec![0; 12];
    hash_input.extend_from_slice(&evm_key);

    // hash the content bytes and append to the key bytes
    let content_hash = hash_keccak256(content_bytes).unwrap();
    hash_input.extend_from_slice(&content_hash);

    // hash the key + content hash
    let mut hash: Vec<u8> = hash_keccak256(hash_input).unwrap();

    // resize the 32 byte keccak hash to 36 bytes so we can use it as a Holohash
    // @todo the extra 4 bytes should be a derived location for sharding
    hash.resize(36, 0);
    let link_base = ExternalHash::from_raw_36(hash);
    return Ok(link_base);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use holochain::sweettest::*;
    use futures::future;

    const DNA_FILEPATH: &str = "../../../workdir/nft_payload.dna";

    pub mod create_link_base_tests {
        use crate::LinkBaseInput;

        async fn _create_link_base(
            conductor: &SweetConductor,
            alice: &SweetCell,
            evm_key: Vec<u8>, 
            content_bytes: Vec<u8>
        ) -> Result<ExternalHash, WasmError> {
            let link_base_input = LinkBaseInput {
                evm_key: evm_key,
                content_bytes: content_bytes,
            };
            let link_base: Result<ExternalHash, WasmError> = conductor
                .call(
                    &alice.zome("nft_payload"), 
                    "extern_create_link_base", 
                    link_base_input
                ).await;
            return link_base;
        }

        async fn _hash(
            conductor: &SweetConductor,
            alice: &SweetCell,
            input: Vec<u8>,
        ) -> Vec<u8> {
            let hash: Vec<u8> = conductor
                .call(
                    &alice.zome("nft_payload"), 
                    "hash", 
                    input
                ).await;
            return hash;
        }

        use super::*;
        // test that create_link_base errors when the EVM key is not 20 bytes
        #[tokio::test(flavor = "multi_thread")]
        #[should_panic(expected = "EVM key must be 20 bytes")]
        async fn test_create_link_base_invalid_evm_key() {
            let (conductors, _agents, apps) = setup_conductors(1).await;
            let conductor: &SweetConductor = &conductors[0];
            let cells = apps.cells_flattened();
            let alice = &cells[0];

            let evm_key = vec![0; 21];
            let content_bytes = vec![1; 100];
            let link_base = _create_link_base(conductor, alice, evm_key, content_bytes).await;
        }

        // test that the content bytes can be of any length and the function won't panic
        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn test_create_link_base_content_bytes_length() {
            let (conductors, _agents, apps) = setup_conductors(1).await;
            let conductor: &SweetConductor = &conductors[0];
            let cells = apps.cells_flattened();
            let alice = &cells[0];

            let evm_key = vec![0; 20];

            let content_bytes = vec![1; 100];
            let _link_base = _create_link_base(conductor, alice, evm_key.clone(), content_bytes).await;

            let content_bytes = vec![1; 1000000];
            let _link_base = _create_link_base(conductor, alice, evm_key.clone(), content_bytes).await;

            let content_bytes = vec![1; 0];
            let _link_base = _create_link_base(conductor, alice, evm_key.clone(), content_bytes).await;
        }

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn test_create_link_base() {
            let (conductors, _agents, apps) = setup_conductors(1).await;
            let conductor: &SweetConductor = &conductors[0];
            let cells = apps.cells_flattened();
            let alice = &cells[0];
        
            // 20 random bytes for the evm key
            let rand_evm_key = vec![5; 20];
            // 200 random bytes
            let content_bytes = vec![9; 200];
            // hashed content bytes
            let content_hash = _hash(conductor, alice, content_bytes.clone()).await;
            // set up the bytes for hashing
            let mut final_for_hashing:Vec<u8> = Vec::new();
            final_for_hashing.extend_from_slice(&vec![0; 12]);
            final_for_hashing.extend_from_slice(&rand_evm_key);
            final_for_hashing.extend_from_slice(&content_hash);
            // hashed final bytes
            let mut final_hash:Vec<u8> = _hash(conductor, alice, final_for_hashing.clone()).await;
            final_hash.extend_from_slice(& vec![0; 4]);
            let expected_link_base = ExternalHash::from_raw_36(final_hash);

            let link_base = _create_link_base(conductor, alice, rand_evm_key.clone(), content_bytes.clone()).await;

            // assert_eq!(link_base, expected_link_base);
        }
    }

    pub mod validate_create_link_tokenid_to_payloads_tests {
        use super::*;

        // test that validate_create_link_tokenid_to_payloads returns Invalid when the base address does not match the expected hash
        #[tokio::test(flavor = "multi_thread")]
        #[ignore]

        async fn test_validate_create_link_tokenid_to_payloads_invalid_base_address() {
            let (_conductors, _agents, _apps) = setup_conductors(2).await;
        }

    }

    async fn setup_conductors(n: usize) -> (SweetConductorBatch, Vec<AgentPubKey>, SweetAppBatch) {
        let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
            .await
            .unwrap();

        let mut conductors = SweetConductorBatch::from_standard_config(n).await;

        let all_agents: Vec<AgentPubKey> =
            future::join_all(conductors.iter().map(|c| SweetAgents::one(c.keystore()))).await;
        let apps = conductors
            .setup_app_for_zipped_agents("app", &all_agents, &[dna])
            .await
            .unwrap();

        conductors.exchange_peer_info().await;
        (conductors, all_agents, apps)
    }
}