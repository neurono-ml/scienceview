//! One shared data-space interval per variable.

use super::super::configuration::SVPairConfiguration;
use super::value::{sv_report_missing, sv_value_at};
use crate::sv_error::SVError;
use std::collections::BTreeMap;

/// The data-space interval a variable occupies across every cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SVPairDomain {
    pub(crate) lower: f64,
    pub(crate) upper: f64,
}

pub(crate) fn sv_variable_domains(
    configuration: &SVPairConfiguration<'_>,
) -> Result<BTreeMap<String, SVPairDomain>, SVError> {
    let mut domains = BTreeMap::new();
    for name in configuration.variable_names() {
        if domains.contains_key(&name) {
            continue;
        }
        domains.insert(name.clone(), sv_domain_of(configuration, &name)?);
    }
    Ok(domains)
}

fn sv_domain_of(
    configuration: &SVPairConfiguration<'_>,
    name: &str,
) -> Result<SVPairDomain, SVError> {
    let mut snapshot = configuration.data_source.column(name)?;
    let view = snapshot.column_view();
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for index in 0..view.length() {
        let value = sv_value_at(view, index)?;
        if !value.is_finite() {
            sv_report_missing(configuration.missing_value_policy, name, index)?;
            continue;
        }
        lower = lower.min(value);
        upper = upper.max(value);
    }
    if !lower.is_finite() || !upper.is_finite() {
        return Ok(SVPairDomain {
            lower: 0.0,
            upper: 1.0,
        });
    }
    if upper - lower <= f64::EPSILON {
        let padding = if lower.abs() <= f64::EPSILON {
            1.0
        } else {
            lower.abs() * 0.1
        };
        return Ok(SVPairDomain {
            lower: lower - padding,
            upper: upper + padding,
        });
    }
    Ok(SVPairDomain { lower, upper })
}
