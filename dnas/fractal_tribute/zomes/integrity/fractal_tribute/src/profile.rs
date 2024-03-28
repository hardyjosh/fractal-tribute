use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Profile {
    pub name: String
}

pub fn validate_create_profile(
    _action: EntryCreationAction,
    _profile: Profile,
) -> ExternResult<ValidateCallbackResult> {

    // check that the name string isn't empty
    if _profile.name.is_empty() {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("Profile name cannot be empty"),
            ),
        )
    }

    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_profile(
    _action: Update,
    _profile: Profile,
    _original_action: EntryCreationAction,
    _original_profile: Profile,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Profiles cannot be updated"),
        ),
    )
}
pub fn validate_delete_profile(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_profile: Profile,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Profiles cannot be deleted"),
        ),
    )
}

pub fn validate_create_link_agent_to_profile(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _profile: crate::Profile = record
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

pub fn validate_delete_link_agent_to_profile(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Profile links cannot be deleted")))
}



// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use holochain::sweettest::*;
//     use futures::future;

//     const DNA_FILEPATH: &str = "../../../workdir/nft_payload.dna";

//     pub mod evm_key_binding_tests {
//         use super::*;
//         use ethers_core::rand::thread_rng;
//         use ethers_signers::{LocalWallet, Signer};

//         #[tokio::test(flavor = "multi_thread")]
//         async fn test_create_evm_binding() {
//             let (conductors, _agents, apps) = setup_conductors(2).await;
//             let conductor: &SweetConductor = &conductors[0];
//             let cells = apps.cells_flattened();
//             let alice = &cells[0];

//             let wallet = LocalWallet::new(&mut thread_rng());

//             // The wallet can be used to sign messages
//             let message = alice.agent_pubkey().get_raw_39();
//             let signature = wallet.sign_message(message).await.unwrap();
//             assert_eq!(signature.recover(&message[..]).unwrap(), wallet.address());

//             let evm_key_binding = Profile {
//                 evm_key: ByteArray(wallet.address().as_bytes().to_vec()),
//                 signature_bytes: ByteArray(signature.to_vec()),
//             };

//             let record: Record = conductor
//                 .call(
//                     &alice.zome("nft_payload"), 
//                     "create_profile", 
//                     evm_key_binding.clone()
//                 ).await;

//             println!("{:#?}", record);
//         }

//         #[tokio::test(flavor = "multi_thread")]
//         #[should_panic(expected = "EVM pubkey binding signature is invalid")]        
//         async fn test_create_evm_binding_bad_sig() {
//             let (conductors, _agents, apps) = setup_conductors(2).await;
//             let conductor: &SweetConductor = &conductors[0];
//             let cells = apps.cells_flattened();
//             let alice = &cells[0];

//             let wallet = LocalWallet::new(&mut thread_rng());
//             let second_wallet = LocalWallet::new(&mut thread_rng());

//             // The wallet can be used to sign messages
//             let message = alice.agent_pubkey().get_raw_39();
//             let signature = wallet.sign_message(message).await.unwrap();
//             assert_eq!(signature.recover(&message[..]).unwrap(), wallet.address());

//             let evm_key_binding = Profile {
//                 evm_key: ByteArray(second_wallet.address().as_bytes().to_vec()),
//                 signature_bytes: ByteArray(signature.to_vec()),
//             };

//             let record: Record = conductor
//                 .call(
//                     &alice.zome("nft_payload"), 
//                     "create_profile", 
//                     evm_key_binding.clone()
//                 ).await;

//             println!("{:#?}", record);
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