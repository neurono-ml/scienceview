//! The optional colour column and its stable category assignment.

use super::SVResolvedColumns;
use crate::sv_error::SVError;
use crate::sv_scale::{SVColour, SVScaleBuilder, SVScaleKind, sv_default_scale_palette};

/// The observed values of the optional colour column.
pub(super) struct SVColourGroups {
    pub(super) entries: Vec<(String, SVColour)>,
    values: Vec<f64>,
}

impl SVColourGroups {
    pub(super) fn resolve(&self, value: f64) -> (Option<String>, SVColour) {
        if !value.is_finite() {
            return (None, sv_default_scale_palette().colour_at_index(0));
        }
        let index = self
            .values
            .iter()
            .position(|candidate| candidate.total_cmp(&value).is_eq());
        match index.and_then(|index| self.entries.get(index)) {
            Some((label, colour)) => (Some(label.clone()), *colour),
            None => (None, sv_default_scale_palette().colour_at_index(0)),
        }
    }
}

pub(super) fn sv_colour_groups(
    columns: &mut SVResolvedColumns<'_>,
) -> Result<SVColourGroups, SVError> {
    let Some(snapshot) = columns.colour.as_mut() else {
        return Ok(SVColourGroups {
            entries: Vec::new(),
            values: Vec::new(),
        });
    };

    let mut values: Vec<f64> = Vec::new();
    let mut labels: Vec<String> = Vec::new();
    for index in 0..columns.length {
        let value = {
            let view = snapshot.column_view();
            view.value_at(index).unwrap_or(0.0)
        };
        if value.is_finite()
            && !values
                .iter()
                .any(|candidate| candidate.total_cmp(&value).is_eq())
        {
            values.push(value);
            labels.push(sv_group_label(value));
        }
    }

    if values.is_empty() {
        return Ok(SVColourGroups {
            entries: Vec::new(),
            values,
        });
    }

    let references: Vec<&str> = labels.iter().map(String::as_str).collect();
    let scale = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&references)
        .range(0.0, 1.0)
        .build()?;
    let Some(nominal) = scale.as_nominal() else {
        return Err(SVError::InvalidConfiguration {
            message: "the colour scale must be nominal".to_string(),
        });
    };
    let entries = labels
        .iter()
        .map(|label| Ok((label.clone(), nominal.colour(label)?)))
        .collect::<Result<Vec<_>, SVError>>()?;
    Ok(SVColourGroups { entries, values })
}

fn sv_group_label(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else {
        value.to_string()
    }
}
