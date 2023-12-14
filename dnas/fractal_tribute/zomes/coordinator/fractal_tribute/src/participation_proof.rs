use fractal_tribute_integrity::dna_properties::DnaProperties;
use fractal_tribute_integrity::dna_properties::_get_dna_properties;
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
    let path = Path::from("signed participation proof");
    create_link(path.path_entry_hash()?, participation_proof_hash.clone(), LinkTypes::SignedParticipationProof, ())?;
    Ok(record)
}

#[hdk_extern]
pub fn build_agent_participation(_: ()) -> ExternResult<ParticipationProof> {
    let all_moves = get_all_game_moves(())?;
    let mut agent_pixels_changed: HashMap<AgentPubKey, u32> = HashMap::new();
    let mut total_pixels_changed: u32 = 0;
    let mut agents_with_evm_binding: HashSet<AgentPubKey> = HashSet::new();

    for record in all_moves {
        match &record.entry {
            RecordEntry::Present(Entry::App(app_entry_bytes)) => {
                match std::convert::TryInto::<GameMove>::try_into(app_entry_bytes.clone().into_sb()) {
                    Ok(game_move) => {
                        let agent = record.action().author();
                        if !agents_with_evm_binding.contains(&agent) {
                            agents_with_evm_binding.insert(agent.clone());
                        }
                        if agents_with_evm_binding.contains(&agent) {
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

    let DnaProperties { nft_contract_address, payment_token_address, ..} = _get_dna_properties(())?;

    let agent_participations_result: ExternResult<Vec<AgentParticipation>> = agent_pixels_changed
        .into_iter()
        .map(|(agent, pixels_changed)| {

            let percentage_of_total_pixels_changed = pixels_changed as f32 / total_pixels_changed as f32;

            match get_agent_evm_address(agent.clone()) {
                Ok(evm_key) => {
                    // producing the packed message for signing is a little annoying because ethers-rs encode_packed doesn't work the same as solidity's
                    // see https://github.com/gakonst/ethers-rs/issues/2225
                    let mut evm_key_slice = [0u8; 32];
                    evm_key_slice[12..].copy_from_slice(&evm_key[..]);
        
                    // convert to an int with 18 decimal places of precision, so we can use it in solidity
                    let percentage_of_total_pixels_changed_eighteen_dec = pixels_changed as u128 * 10u128.pow(18) / total_pixels_changed as u128;
                    let percentage_uint: U128 = percentage_of_total_pixels_changed_eighteen_dec.into();
                    let percentage_uint: U256 = percentage_uint.into();
                
                    let mut percentage_buf = [0; 32];
                    percentage_uint.to_big_endian(&mut percentage_buf);
        
                    let token_address: U256 = payment_token_address.parse().unwrap();
                    let mut token_address_buf = [0; 32];
                    token_address.to_big_endian(&mut token_address_buf);
        
                    let contract_address: U256 = nft_contract_address.parse().unwrap();
                    let mut contract_address_buf = [0; 32];
                    contract_address.to_big_endian(&mut contract_address_buf);
                    let start = 32 - 256 / 8;
        
                    let message = encode_packed(&[Token::Bytes(evm_key_slice.to_vec()), Token::Bytes(percentage_buf[start..32].to_vec()), Token::Bytes(token_address_buf[start..32].to_vec()),Token::Bytes(contract_address_buf[start..32].to_vec())])
                        .map_err(|_| wasm_error!("Could not encode message"))?;
        
                    debug!("message: {:?}", message);
        
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
                },
                Err(_) => {
                    Ok(AgentParticipation {
                        evm_key: vec![0u8; 20],
                        agent,
                        pixels_changed,
                        percentage_of_total_pixels_changed,
                        rank: 0,
                        signature_bytes: vec![], // for replacing before posting back to the happ
                        message_bytes: vec![]
                    })
                }
            }
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

#[hdk_extern]
pub fn get_signed_participation(evm_key: Vec<u8>) -> ExternResult<AgentParticipation> {
    let base = Path::from("signed participation proof");
    let links = get_links(base.path_entry_hash()?, LinkTypes::SignedParticipationProof, None)?;
    let get_input: Vec<GetInput> = links
    .into_iter()
    .map(|link| GetInput::new(
        ActionHash::from(link.target).into(),
        GetOptions::default(),
    ))
    .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    if (records.len() == 0) {
        return Err(wasm_error!("No signed participation proof found"));
    }
    let record = records[0].clone();
    let participation_proof: ParticipationProof = match &record.entry {
        RecordEntry::Present(Entry::App(app_entry_bytes)) => {
            match std::convert::TryInto::<ParticipationProof>::try_into(app_entry_bytes.clone().into_sb()) {
                Ok(participation_proof) => participation_proof,
                Err(_) => return Err(wasm_error!("Could not convert record to ParticipationProof")),
            }
        },
        _ => return Err(wasm_error!("Could not convert record to ParticipationProof")),
    };
    let agent_participation = participation_proof.agent_participations.into_iter().find(|agent_participation| agent_participation.evm_key == evm_key);
    match agent_participation {
        Some(agent_participation) => Ok(agent_participation),
        None => Err(wasm_error!("Could not find agent participation for evm key")),
    }
}