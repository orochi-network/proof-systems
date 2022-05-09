use crate::circuits::{
    gate::{CurrOrNext, GateType},
    lookup::lookups::{JointLookupSpec, LocalPosition},
    wires::COLUMNS,
};
use ark_ff::{FftField, One, Zero};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use CurrOrNext::{Curr, Next};

pub mod xor;

//~ spec:startcode
/// The table ID associated with the XOR lookup table.
pub const XOR_TABLE_ID: i32 = 0;
//~ spec:endcode

/// Enumerates the different 'fixed' lookup tables used by individual gates
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GateLookupTable {
    Xor,
}

/// Specifies the relative position of gates and the fixed lookup table (if applicable) that a
/// given lookup configuration should apply to.
pub struct GatesLookupSpec {
    /// The set of positions relative to an active gate where a lookup configuration applies.
    pub gate_positions: HashSet<(GateType, CurrOrNext)>,
    /// The fixed lookup table that should be used for these lookups, if applicable.
    pub gate_lookup_table: Option<GateLookupTable>,
}

/// Specifies mapping from positions defined relative to gates into lookup data.
pub struct GatesLookupMaps {
    /// Enumerates the selector that should be active for a particular gate-relative position.
    pub gate_selector_map: HashMap<(GateType, CurrOrNext), usize>,
    /// Enumerates the fixed tables that should be used for lookups in a particular gate-relative
    /// position.
    pub gate_table_map: HashMap<(GateType, CurrOrNext), GateLookupTable>,
}

/// Evaluates a joint lookup.
pub fn evaluate_joint_lookup<F: FftField>(
    (joint_combiner, table_id_combiner): &(F, F),
    j: &JointLookupSpec<F>,
    witness: &[Vec<F>; COLUMNS],
    row: usize,
) -> F {
    let eval = |pos: LocalPosition| -> F {
        let row = match pos.row {
            Curr => row,
            Next => row + 1,
        };
        witness[pos.column][row]
    };

    j.evaluate(joint_combiner, table_id_combiner, &eval)
}

/// A table of values that can be used for a lookup, along with the ID for the table.
#[derive(Debug)]
pub struct LookupTable<F> {
    pub id: i32,
    pub data: Vec<Vec<F>>,
}

impl<F> LookupTable<F>
where
    F: FftField,
{
    /// Return true if the table has an entry containing all zeros.
    pub fn has_zero_entry(&self) -> bool {
        // reminder: a table is written as a list of columns,
        // not as a list of row entries.
        for row in 0..self.data[0].len() {
            for col in &self.data {
                if !col[row].is_zero() {
                    continue;
                }
                return true;
            }
        }

        false
    }
}

/// Returns the lookup table associated to a [GateLookupTable].
pub fn get_table<F: FftField>(table_name: GateLookupTable) -> LookupTable<F> {
    match table_name {
        GateLookupTable::Xor => xor::xor_table(),
    }
}

/// Let's say we want to do a lookup in a "vector-valued" table `T: Vec<[F; n]>` (here I
/// am using `[F; n]` to model a vector of length `n`).
///
/// For `i < n`, define `T_i := T.map(|t| t[i]).collect()`. In other words, the table
/// obtained by taking the `ith` entry of each element of `T`.
///
/// In the lookup argument, we perform lookups in `T` by sampling a random challenge
/// `joint_combiner`, and computing a "combined" lookup table `sum_{i < n} joint_combiner^i T_i`.
///
/// To check a vector's membership in this lookup table, we combine the values in that vector
/// analogously using `joint_combiner`.
///
/// This function computes that combined value.
pub fn combine_table_entry<'a, F, I>(
    joint_combiner: &F,
    table_id_combiner: &F,
    v: I,
    // TODO: this should be an option?
    table_id: &F,
) -> F
where
    F: 'a, // Any references in `F` must have a lifetime longer than `'a`.
    F: Zero + One + Clone,
    I: DoubleEndedIterator<Item = &'a F>,
{
    v.rev()
        .fold(F::zero(), |acc, x| joint_combiner.clone() * acc + x.clone())
        + table_id_combiner.clone() * table_id.clone()
}
