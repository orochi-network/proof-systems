use std::collections::HashMap;

use kimchi::circuits::expr::{CacheId, FormattedOutput};

/// Describe a generic indexed variable X_{i}.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Column {
    X(usize),
    // Columns related to the lookup protocol
    /// Partial sums. This corresponds to the `h_i`.
    /// It is first indexed by the table ID, and after that internal index.
    LookupPartialSum((u32, usize)),
    /// Multiplicities, indexed. This corresponds to the `m_i`
    LookupMultiplicity(u32),
    /// The lookup aggregation, i.e. `phi`
    LookupAggregation,
    /// The fixed tables. The parameter is considered to the indexed table.
    LookupFixedTable(u32),
}

impl FormattedOutput for Column {
    fn latex(&self, _cache: &mut HashMap<CacheId, Self>) -> String {
        match self {
            Column::X(i) => format!("x_{{{i}}}"),
            Column::LookupPartialSum((table_id, i)) => format!("h_{{{table_id}, {i}}}"),
            Column::LookupMultiplicity(i) => format!("m_{{{i}}}"),
            Column::LookupFixedTable(i) => format!("t_{{{i}}}"),
            Column::LookupAggregation => String::from("φ"),
        }
    }

    fn text(&self, _cache: &mut HashMap<CacheId, Self>) -> String {
        match self {
            Column::X(i) => format!("x[{i}]"),
            Column::LookupPartialSum((table_id, i)) => format!("h[{table_id}, {i}]"),
            Column::LookupMultiplicity(i) => format!("m[{i}]"),
            Column::LookupFixedTable(i) => format!("t[{i}]"),
            Column::LookupAggregation => String::from("φ"),
        }
    }

    fn ocaml(&self, _cache: &mut HashMap<CacheId, Self>) -> String {
        // FIXME
        unimplemented!("Not used at the moment")
    }

    fn is_alpha(&self) -> bool {
        // FIXME
        unimplemented!("Not used at the moment")
    }
}

/// A datatype expressing a generalized column, but with potentially
/// more convenient interface than a bare column.
pub trait ColumnIndexer {
    fn to_column(self) -> Column;
}
