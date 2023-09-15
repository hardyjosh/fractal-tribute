use hdk::prelude::*;
use fractal_tribute_integrity::dna_properties::{DnaProperties, _get_dna_properties};

#[hdk_extern]
pub fn get_dna_properties(_:()) -> ExternResult<DnaProperties> {
   _get_dna_properties(())
}
