//! Grouping observations into one fit per series.

/// Groups logical positions by group code, preserving first-appearance order.
pub(crate) fn sv_group_positions(
    series: Option<&[f64]>,
    length: usize,
) -> Vec<(String, Vec<usize>)> {
    let Some(codes) = series else {
        return vec![("all".to_string(), (0..length).collect())];
    };

    let mut groups: Vec<(f64, Vec<usize>)> = Vec::new();
    for (position, code) in codes.iter().copied().enumerate() {
        match groups
            .iter_mut()
            .find(|(existing, _)| existing.total_cmp(&code).is_eq())
        {
            Some((_, positions)) => positions.push(position),
            None => groups.push((code, vec![position])),
        }
    }
    groups
        .into_iter()
        .map(|(code, positions)| (sv_group_label(code), positions))
        .collect()
}

/// Formats a numeric group code without an avoidable decimal tail.
fn sv_group_label(code: f64) -> String {
    if code.fract() == 0.0 {
        format!("{code:.0}")
    } else {
        format!("{code}")
    }
}
