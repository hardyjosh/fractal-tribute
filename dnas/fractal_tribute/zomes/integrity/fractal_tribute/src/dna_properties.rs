use hdk::prelude::*;

#[hdk_entry_helper]
pub struct DnaProperties {
    pub nft_contract_address: String,
    pub payment_token_address: String,
    pub game_end_time: u32,
    pub game_master_evm_key: String,
    pub chain_id: u32
}

pub fn _get_dna_properties(_:()) -> ExternResult<DnaProperties> {
    let info = hdk::info::dna_info()?;
    info.properties.try_into()
        .map_err(|_| wasm_error!("Failed to deserialize properties"))
}