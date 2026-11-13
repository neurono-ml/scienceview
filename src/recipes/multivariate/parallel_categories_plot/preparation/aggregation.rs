//! Aggregating observations into ribbons between two adjacent axes.
//!
//! The recipe never counts observations itself. It builds one group code per
//! observation from the two axis ranks (and the optional colour rank), then
//! hands that derived column to [`SVStat::Count`](crate::sv_stat::SVStat::Count).
//! The source columns stay in their original representation; only the
//! aggregation key is materialised, because a group is a property of the
//! observation rather than of one stored column.

use std::collections::HashMap;

use super::categories::{sv_category_key, sv_category_rank};
use super::prepared::SVPreparedRibbon;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_stat::{SVCountOptions, SVStat, SVStatInput, SVStatResult};

/// The counted categories of one adjacent axis pair.
pub(super) struct SVPairAggregation {
    pub(super) ribbons: Vec<SVPreparedRibbon>,
}

/// Builds and counts the group codes of one adjacent axis pair.
#[allow(clippy::too_many_arguments)]
pub(super) fn sv_aggregate_pair(
    left_axis: usize,
    right_axis: usize,
    left_view: SVColumnView<'_>,
    right_view: SVColumnView<'_>,
    left_ranks: &HashMap<u64, usize>,
    right_ranks: &HashMap<u64, usize>,
    number_of_right_categories: usize,
    colour_view: Option<SVColumnView<'_>>,
    colour_ranks: Option<&HashMap<u64, usize>>,
    number_of_colour_categories: usize,
) -> Result<SVPairAggregation, SVError> {
    let length = left_view.length().min(right_view.length());
    let mut codes: Vec<f64> = Vec::with_capacity(length);
    let mut decoded: HashMap<u64, (usize, usize, usize)> = HashMap::new();

    for position in 0..length {
        let left_value = left_view.value_at(position).unwrap_or(0.0);
        let right_value = right_view.value_at(position).unwrap_or(0.0);
        let left_rank = sv_category_rank(left_ranks, left_value)?;
        let right_rank = sv_category_rank(right_ranks, right_value)?;
        let colour_rank = match (colour_view, colour_ranks) {
            (Some(view), Some(ranks)) => {
                let value = view.value_at(position).unwrap_or(0.0);
                sv_category_rank(ranks, value)?
            }
            _ => 0,
        };
        let code = sv_group_code(
            left_rank,
            right_rank,
            colour_rank,
            number_of_right_categories,
            number_of_colour_categories,
        );
        decoded.insert(code.to_bits(), (left_rank, right_rank, colour_rank));
        codes.push(code);
    }

    let options = SVCountOptions::new();
    let result = match SVStat::count(options)
        .apply(SVStatInput::new(SVColumnView::Dense(&codes)))?
    {
        SVStatResult::Categories(result) => result,
        other => {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the parallel categories plot expected the aggregation stat to produce categories, but it produced `{}`",
                    other.kind().as_str()
                ),
            });
        }
    };

    let mut ribbons = Vec::with_capacity(result.categories().len());
    for (code, count) in result.categories().iter().zip(result.counts().iter()) {
        let Some((left_category, right_category, colour_index)) =
            decoded.get(&code.to_bits()).copied()
        else {
            return Err(SVError::InvalidConfiguration {
                message: "the counting kernel returned a group code the recipe did not build"
                    .to_string(),
            });
        };
        ribbons.push(SVPreparedRibbon {
            left_axis,
            right_axis,
            left_category,
            right_category,
            colour_index,
            count: *count,
            colour: crate::sv_scale::SVColour::new(0, 0, 0, 255),
        });
    }
    Ok(SVPairAggregation { ribbons })
}

fn sv_group_code(
    left_rank: usize,
    right_rank: usize,
    colour_rank: usize,
    number_of_right_categories: usize,
    number_of_colour_categories: usize,
) -> f64 {
    let pair = left_rank * number_of_right_categories + right_rank;
    let combined = pair * number_of_colour_categories + colour_rank;
    combined as f64
}

/// Returns the value-keyed rank map of one counted axis.
pub(super) fn sv_rank_map(categories: &[f64]) -> HashMap<u64, usize> {
    categories
        .iter()
        .enumerate()
        .map(|(index, value)| (sv_category_key(*value), index))
        .collect()
}
