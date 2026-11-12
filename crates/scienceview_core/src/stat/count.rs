//! The counting kernel.

use std::collections::HashMap;

use crate::sv_error::SVError;
use crate::sv_stat::count_options::{SVCountOptions, SVCountOrder};
use crate::sv_stat::count_result::SVCountResult;
use crate::sv_stat::input::SVStatInput;
use crate::sv_stat::observations::SVObservationValues;
use crate::sv_stat::stat_definition::SVStatResult;

/// Applies the counting configuration to the input column.
pub(crate) fn sv_apply_count(
    options: &SVCountOptions,
    input: &SVStatInput<'_>,
) -> Result<SVStatResult, SVError> {
    let observations = SVObservationValues::from_view(input.values());
    if observations.length() == 0 {
        return Err(SVError::EmptyInput { operation: "count" });
    }
    observations.validate_finite("count")?;

    let mut categories: Vec<f64> = Vec::new();
    let mut counts: Vec<u64> = Vec::new();
    let mut positions: HashMap<u64, usize> = HashMap::new();

    observations.for_each_first_appearance(|value, multiplicity| {
        let key = sv_category_key(value);
        match positions.get(&key).copied() {
            Some(position) => counts[position] += multiplicity,
            None => {
                positions.insert(key, categories.len());
                categories.push(value);
                counts.push(multiplicity);
            }
        }
    });

    sv_order_categories(&mut categories, &mut counts, options.requested_order());

    Ok(SVStatResult::Categories(SVCountResult::new(
        categories,
        counts,
        options.requested_order(),
    )))
}

/// Maps a category value onto its exact storage key.
///
/// `-0.0` and `0.0` are the same category, so both map onto zero.
pub(crate) fn sv_category_key(value: f64) -> u64 {
    if value == 0.0 { 0 } else { value.to_bits() }
}

/// Applies the requested order to the category and count pairs.
fn sv_order_categories(categories: &mut [f64], counts: &mut [u64], order: SVCountOrder) {
    let mut pairs: Vec<(f64, u64)> = categories
        .iter()
        .copied()
        .zip(counts.iter().copied())
        .collect();

    match order {
        SVCountOrder::DataOrder => return,
        SVCountOrder::Ascending => pairs.sort_by(|left, right| left.0.total_cmp(&right.0)),
        SVCountOrder::Descending => pairs.sort_by(|left, right| right.0.total_cmp(&left.0)),
    }

    for (position, (category, count)) in pairs.into_iter().enumerate() {
        categories[position] = category;
        counts[position] = count;
    }
}
