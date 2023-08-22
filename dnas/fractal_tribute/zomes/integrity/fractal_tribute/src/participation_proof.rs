use hdi::prelude::*;
use ethers_core::types::*;

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

    // @TODO things to validate:
    // - must be from the game steward
    // - the percentages must add up to 100%
    // - the same agent cannot be listed twice
    // - someone must be able to produce the same proof
    // - the signature for each proof must verify against the steward's address

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