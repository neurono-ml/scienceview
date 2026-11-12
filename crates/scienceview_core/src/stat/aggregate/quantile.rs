//! Quantiles of a sorted slice plus an analytic zero multiplicity.

/// Returns the quantile of an ascending slice plus a run of implicit zeros.
///
/// The zeros are a multiplicity between the negative and the positive segment,
/// the same shape `SVSortedObservations` uses, so a billion implicit zeros cost
/// one comparison rather than one rank each.
pub(super) fn sv_quantile_with_zeros(
    sorted: &[f64],
    implicit_zeros: usize,
    quantile: f64,
) -> Option<f64> {
    let length = sorted.len().saturating_add(implicit_zeros);
    if length == 0 {
        return None;
    }

    let position = quantile * (length - 1) as f64;
    let lower_rank = position.floor() as usize;
    let upper_rank = position.ceil() as usize;
    let lower = sv_value_at_rank_with_zeros(sorted, implicit_zeros, lower_rank)?;
    let upper = sv_value_at_rank_with_zeros(sorted, implicit_zeros, upper_rank)?;

    if lower_rank == upper_rank {
        Some(lower)
    } else {
        Some(lower + (position - lower_rank as f64) * (upper - lower))
    }
}

/// Returns the value at a logical rank of a slice plus its implicit zeros.
pub(super) fn sv_value_at_rank_with_zeros(
    sorted: &[f64],
    implicit_zeros: usize,
    rank: usize,
) -> Option<f64> {
    let negative_end = sorted.partition_point(|value| *value < 0.0);
    if rank < negative_end {
        return sorted.get(rank).copied();
    }

    let stored_zero_end = sorted.partition_point(|value| *value <= 0.0);
    let zeros = (stored_zero_end - negative_end).saturating_add(implicit_zeros);
    if rank < negative_end + zeros {
        return Some(0.0);
    }

    let index = stored_zero_end + (rank - negative_end - zeros);
    sorted.get(index).copied()
}
