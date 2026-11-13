//! Finite-run splitting and the ownership transfer of the mark geometry.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVMarkGeometry, SVMarkPrimitive, SVMarkShape};

use super::configuration::sv_invalid;
use super::missing_value_policy::SVMissingValuePolicy;

/// A contiguous run of finite samples of one band.
pub(crate) struct SVRun {
    pub(crate) x: Vec<f64>,
    pub(crate) lower: Vec<f64>,
    pub(crate) upper: Vec<f64>,
}
pub(crate) fn sv_finite_runs(
    x: SVColumnView<'_>,
    lower: SVColumnView<'_>,
    upper: SVColumnView<'_>,
    policy: SVMissingValuePolicy,
) -> Result<Vec<SVRun>, SVError> {
    let length = x.length();
    let mut runs = Vec::new();
    let mut current = SVRun {
        x: Vec::new(),
        lower: Vec::new(),
        upper: Vec::new(),
    };
    for index in 0..length {
        let sample = (
            x.value_at(index).unwrap_or(f64::NAN),
            lower.value_at(index).unwrap_or(f64::NAN),
            upper.value_at(index).unwrap_or(f64::NAN),
        );
        let finite = sample.0.is_finite() && sample.1.is_finite() && sample.2.is_finite();
        if !finite {
            policy.report("x/lower/upper", index)?;
            if !current.x.is_empty() {
                runs.push(std::mem::replace(
                    &mut current,
                    SVRun {
                        x: Vec::new(),
                        lower: Vec::new(),
                        upper: Vec::new(),
                    },
                ));
            }
            continue;
        }
        current.x.push(sample.0);
        current.lower.push(sample.1);
        current.upper.push(sample.2);
    }
    if !current.x.is_empty() {
        runs.push(current);
    }
    Ok(runs)
}

pub(crate) fn sv_own_geometry(
    geometry: SVMarkGeometry<'_>,
) -> Result<SVMarkGeometry<'static>, SVError> {
    let primitives = geometry
        .primitives()
        .iter()
        .map(|primitive| {
            Ok(SVMarkPrimitive::new(
                sv_own_shape(primitive.shape())?,
                primitive.paint().clone(),
            ))
        })
        .collect::<Result<Vec<_>, SVError>>()?;
    Ok(SVMarkGeometry::new(primitives))
}

fn sv_own_shape(shape: &SVMarkShape<'_>) -> Result<SVMarkShape<'static>, SVError> {
    match shape {
        SVMarkShape::Path(path) => Ok(SVMarkShape::Path(path.clone())),
        SVMarkShape::Rectangle(rectangle) => Ok(SVMarkShape::Rectangle(*rectangle)),
        SVMarkShape::Circle(circle) => Ok(SVMarkShape::Circle(*circle)),
        SVMarkShape::Text(text) => Ok(SVMarkShape::Text(text.clone())),
        SVMarkShape::Image(_) => Err(sv_invalid(
            "a band plot draws paths only; an image primitive is not part of its composition",
        )),
    }
}
