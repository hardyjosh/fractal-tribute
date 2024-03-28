use hdi::prelude::*;
use ethers_core::types::*;

use crate::dna_properties::{DnaProperties, _get_dna_properties};

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct AgentParticipation {
    pub agent: AgentPubKey,
    pub evm_key: Vec<u8>,
    pub pixels_changed: u32,
    pub percentage_of_total_pixels_changed: f32,
    pub rank: u16,
    pub message_bytes: Vec<u8>,
    pub signature_bytes: Vec<u8>
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ParticipationProof {
    pub total_pixels_changed: u32,
    pub agent_participations: Vec<AgentParticipation>,
}

pub fn validate_create_participation_proof(
    _action: EntryCreationAction,
    _participation_proof: ParticipationProof,
) -> ExternResult<ValidateCallbackResult> {

    let DnaProperties { game_master_evm_key, ..} = _get_dna_properties(())?;
    let game_master_evm_address: H160 = game_master_evm_key.parse().unwrap();

    for agent_participation in &_participation_proof.agent_participations {
        let signature: ethers_core::types::Signature = agent_participation.signature_bytes.as_slice().try_into().unwrap();
        let message: RecoveryMessage = agent_participation.message_bytes.as_slice().try_into().ok().unwrap();
        let verified = signature.verify(message, game_master_evm_address);

        if !verified.is_ok() {
            return Ok(
                ValidateCallbackResult::Invalid(
                    String::from("Participation proof signature is invalid"),
                ),
            );
        }
    }

    // @TODO things to validate:
    // - someone must be able to produce the same proof

    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_participation_proof(
    _action: Update,
    _participation_proof: ParticipationProof,
    _original_action: EntryCreationAction,
    _original_participation_proof: ParticipationProof,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Participation proofs cannot be updated"),
        ),
    )
}
pub fn validate_delete_participation_proof(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_participation_proof: ParticipationProof,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Participation proofs cannot be deleted"),
        ),
    )
}

pub fn validate_create_link_signed_participation_proof(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_signed_participation_proof(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Participation proof links cannot be deleted")))
}
