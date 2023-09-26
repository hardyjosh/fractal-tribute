use hdk::prelude::*;
use fractal_tribute_integrity::*;
use crate::{all_game_moves::*, evm_key_binding::*};
use std::collections::HashMap;
use ethers_core::types::*;
use ethers_core::abi::*;

#[hdk_extern]
pub fn create_participation_proof(proof: ParticipationProof) -> ExternResult<Record> {
    let participation_proof_hash = create_entry(
        &EntryTypes::ParticipationProof(proof.clone()),
    )?;
    let record = get(participation_proof_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created ParticipationProof"))
            ),
        )?;
    Ok(record)
}

#[hdk_extern]
pub fn build_agent_participation(_: ()) -> ExternResult<ParticipationProof> {
    let all_moves = get_all_game_moves(())?;
    let mut agent_pixels_changed: HashMap<AgentPubKey, u32> = HashMap::new();
    let mut total_pixels_changed: u32 = 0;

    for record in all_moves {
        match &record.entry {
            RecordEntry::Present(Entry::App(app_entry_bytes)) => {
                match std::convert::TryInto::<GameMove>::try_into(app_entry_bytes.clone().into_sb()) {
                    Ok(game_move) => {
                        let agent = record.action().author();
                        let result = get_agent_evm_address(agent.clone());
                        if let Ok(_) = result {
                            let pixels_changed = game_move.count_changes();
                            *agent_pixels_changed.entry(agent.clone()).or_insert(0) += pixels_changed as u32;
                            total_pixels_changed += pixels_changed as u32;
                        }
                    },
                    Err(_) => return Err(wasm_error!("Could not convert record to GameMove")),
                }
            },
            _ => {} // Other cases
        }
    }

    let agent_participations_result: ExternResult<Vec<AgentParticipation>> = agent_pixels_changed
        .into_iter()
        .map(|(agent, pixels_changed)| {

            // producing the packed message for signing is a little annoying because ethers-rs encode_packed doesn't work the same as solidity's
            // see https://github.com/gakonst/ethers-rs/issues/2225
            let evm_key = get_agent_evm_address(agent.clone())?;
            let mut evm_key_slice = [0u8; 32];
            evm_key_slice[12..].copy_from_slice(&evm_key[..]);
            let evm_key_uint: U256 = evm_key_slice.try_into().unwrap();

            let percentage_of_total_pixels_changed = pixels_changed as f32 / total_pixels_changed as f32;

            // convert to an int with 18 decimal places of precision, so we can use it in solidity
            let percentage_of_total_pixels_changed_eighteen_dec = pixels_changed as u128 * 10u128.pow(18) / total_pixels_changed as u128;
            let percentage_uint: U128 = percentage_of_total_pixels_changed_eighteen_dec.into();
            let percentage_uint: U256 = percentage_uint.into();

            // debug!("percentage_of_total_pixels_changed_eighteen_dec: {:?}", percentage_of_total_pixels_changed_eighteen_dec);
            // debug!("percentage_of_total_pixels_changed: {:?}", percentage_of_total_pixels_changed);
            // debug!("percentage_uint: {:?}", percentage_uint);

            let mut percentage_buf = [0; 32];
            percentage_uint.to_big_endian(&mut percentage_buf);

            let token_address: U256 = "0x2Eb1D24aB0eC5FD0058ab5073F1EA2d8A59783E5".parse().unwrap();
            let mut token_address_buf = [0; 32];
            token_address.to_big_endian(&mut token_address_buf);

            let contract_address: U256 = "0x6c10ca75f0eEfdefC7652F21633ae904038F16a9".parse().unwrap();
            let mut contract_address_buf = [0; 32];
            contract_address.to_big_endian(&mut contract_address_buf);
            let start = 32 - 256 / 8;

            let message = encode_packed(&[Token::Bytes(evm_key_slice.to_vec()), Token::Bytes(percentage_buf[start..32].to_vec()), Token::Bytes(token_address_buf[start..32].to_vec()),Token::Bytes(contract_address_buf[start..32].to_vec())])
                .map_err(|_| wasm_error!("Could not encode message"))?;

            // debug!("message: {:?}", message);

            let message_bytes = hash_keccak256(message).map_err(|_| wasm_error!("Could not hash message"))?;

            Ok(AgentParticipation {
                evm_key,
                agent,
                pixels_changed,
                percentage_of_total_pixels_changed,
                rank: 0,
                signature_bytes: vec![], // for replacing before posting back to the happ
                message_bytes
            })
        })
        .collect();

    let mut agent_participations = agent_participations_result?;

    // Sort by the number of pixels changed
    agent_participations.sort_by(|a, b| b.pixels_changed.cmp(&a.pixels_changed));

    // Update rank based on the sorted order
    for (index, participation) in agent_participations.iter_mut().enumerate() {
        participation.rank = (index + 1) as u16;
    }

    Ok(ParticipationProof {
        total_pixels_changed,
        agent_participations,
    })
}
