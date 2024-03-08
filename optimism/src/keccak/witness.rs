//! This file contains the witness for the Keccak hash function for the zkVM project.
//! It assigns the witness values to the corresponding columns of KeccakWitness in the environment.
//!
//! The actual witness generation code makes use of the code which is already present in Kimchi,
//! to avoid code duplication and reduce error-proneness.
//!
//! For a pseudo code implementation of Keccap-f, see
//! https://keccak.team/keccak_specs_summary.html
use crate::keccak::column::KeccakWitness;
use ark_ff::Field;

/// This struct contains all that needs to be kept track of during the execution of the Keccak step interpreter
#[derive(Clone, Debug)]
pub struct Env<Fp> {
    /// The full state of the Keccak gate (witness)
    pub witness: KeccakWitness<Fp>,
    // The multiplicities of each lookup table
    // TODO
    /// A counter of constraints to help with debugging, starts with 1
    pub(crate) check_idx: usize,
}

impl<F: Field> Default for Env<F> {
    fn default() -> Self {
        Self {
            witness: KeccakWitness::default(),
            check_idx: 0,
        }
    }
}
