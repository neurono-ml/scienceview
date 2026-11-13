//! The value, category-placement and nominal-axis scales one render needs.

use super::configuration::SVStripConfiguration;
use super::data_contract::{SVStripCategory, SVStripObservation};
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::SVBarOrientation;
use crate::sv_scale::{
    SVDomain, SVScale, SVScaleBuilder, SVScaleKind, SVScaleTransform, sv_nice_domain,
};

/// The three scales one strip composition reads.
pub(super) struct SVStripScales<'labels> {
    pub(super) value: SVScale<'labels>,
    pub(super) category: SVScale<'labels>,
    pub(super) nominal: SVScale<'labels>,
}

/// Builds the value scale, the category placement scale and the nominal axis.
pub(super) fn sv_build_scales<'labels>(
    configuration: &SVStripConfiguration<'_>,
    observations: &[SVStripObservation],
    categories: &[SVStripCategory],
    labels: &'labels [&'labels str],
    plot: SVResolvedRectangle,
) -> Result<SVStripScales<'labels>, SVError> {
    let domain = sv_value_domain(observations)?;
    let (value_start, value_end) = match configuration.orientation {
        SVBarOrientation::Vertical => (plot.bottom(), plot.y),
        SVBarOrientation::Horizontal => (plot.x, plot.right()),
    };
    let value = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(domain.lower(), domain.upper())
        .range(value_start, value_end)
        .build()?;

    // One category slot is exactly one unit wide and the slot of category `i`
    // is centred on `i`. Half-unit bounds keep that true for a single
    // category, where a `0..0` domain would collapse every dot to one column.
    let category_count = categories.len().max(1) as f64;
    let (category_start, category_end) = match configuration.orientation {
        SVBarOrientation::Vertical => (plot.x, plot.right()),
        SVBarOrientation::Horizontal => (plot.bottom(), plot.y),
    };
    let category = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(-0.5, category_count - 0.5)
        .range(category_start, category_end)
        .build()?;

    let slots = categories.len().max(1) as f64;
    let (nominal_start, nominal_end) = match configuration.orientation {
        SVBarOrientation::Vertical => {
            let inset = plot.width / (2.0 * slots);
            (inset, plot.width - inset)
        }
        SVBarOrientation::Horizontal => {
            let inset = plot.height / (2.0 * slots);
            (plot.height - inset, inset)
        }
    };
    let nominal = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(labels)
        .range(nominal_start, nominal_end)
        .build()?;

    Ok(SVStripScales {
        value,
        category,
        nominal,
    })
}

/// Returns the nice domain that covers every drawn value.
///
/// A degenerate sample (all values equal) is padded by half a unit so the dot
/// lands in the middle of the axis instead of on a zero-height band.
fn sv_value_domain(observations: &[SVStripObservation]) -> Result<SVDomain, SVError> {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for value in observations.iter().map(|observation| observation.value) {
        lower = lower.min(value);
        upper = upper.max(value);
    }
    if !lower.is_finite() || !upper.is_finite() {
        return SVDomain::new(0.0, 1.0);
    }
    let domain = if lower < upper {
        SVDomain::new(lower, upper)?
    } else {
        SVDomain::new(lower - 0.5, upper + 0.5)?
    };
    Ok(sv_nice_domain(domain, SVScaleTransform::Linear))
}

/// Returns the category labels in slot order.
pub(super) fn sv_category_labels(categories: &[SVStripCategory]) -> Vec<String> {
    categories
        .iter()
        .map(|category| category.label.clone())
        .collect()
}
