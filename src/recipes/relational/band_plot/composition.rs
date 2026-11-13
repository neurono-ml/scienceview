//! Composing the band from the existing mark kernel.
//!
//! A band is the `Ribbon` mark; the optional dashed borders and centre line
//! are the `Line` mark. No geometry is re-derived here: every primitive comes
//! from [`SVMark::geometry`], which is the single place a value becomes a
//! path.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::{
    SVDashPattern, SVMark, SVMarkGeometry, SVMarkRequest, SVRibbonBoundary, SVRibbonOptions,
};
use scienceview_core::sv_scale::{SVScale, sv_default_scale_palette};

use super::composition_geometry::{SVRun, sv_finite_runs, sv_own_geometry};
use super::configuration::SVBandPlotConfiguration;
use super::configuration::sv_invalid;
use super::data_contract::SVBandData;

/// Builds every mark geometry of one band plot.
pub(crate) fn sv_compose_band_geometries(
    data: &mut SVBandData<'_>,
    configuration: &SVBandPlotConfiguration<'_>,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    plot: SVResolvedRectangle,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let palette = sv_default_scale_palette();
    let mut geometries = Vec::new();
    for (index, band) in data.bands.iter_mut().enumerate() {
        let colour = palette.colour_at_index(index);
        let x = band.x.view();
        let lower = band.lower.view();
        let upper = band.upper.view();
        sv_require_same_length(x.length(), lower.length())?;
        sv_require_same_length(x.length(), upper.length())?;

        let positions = super::column_access::sv_positions(x);
        for run in sv_finite_runs(
            &positions,
            x,
            lower,
            upper,
            configuration.missing_value_policy,
        )? {
            sv_push_run_geometry(
                &mut geometries,
                &run,
                colour,
                configuration,
                horizontal,
                vertical,
                plot,
            )?;
        }
    }
    Ok(geometries)
}

fn sv_push_run_geometry(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    run: &SVRun,
    colour: scienceview_core::sv_scale::SVColour,
    configuration: &SVBandPlotConfiguration<'_>,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    plot: SVResolvedRectangle,
) -> Result<(), SVError> {
    let x = SVColumnView::Dense(&run.x);
    let lower = SVColumnView::Dense(&run.lower);
    let upper = SVColumnView::Dense(&run.upper);
    let request = SVMarkRequest::new(x, lower, horizontal, vertical, plot)
        .with_lower(lower)
        .with_upper(upper);

    if configuration.fill {
        let options = sv_ribbon_options(configuration, colour).boundary(SVRibbonBoundary::Both);
        let geometry = SVMark::from(options).geometry(&request)?;
        geometries.push(sv_own_geometry(geometry)?);
    }

    if configuration.border {
        if configuration.dashed_borders {
            let dash = SVDashPattern::Dashed;
            sv_push_boundary_lines(
                geometries,
                run,
                colour,
                dash,
                configuration,
                horizontal,
                vertical,
                plot,
            )?;
        } else {
            sv_push_boundaries(geometries, &request, configuration, colour)?;
        }
    } else if !configuration.fill {
        sv_push_boundaries(geometries, &request, configuration, colour)?;
    }

    if configuration.centre_line {
        let centre: Vec<f64> = run
            .lower
            .iter()
            .zip(&run.upper)
            .map(|(lower, upper)| (lower + upper) / 2.0)
            .collect();
        let line_request = SVMarkRequest::new(
            SVColumnView::Dense(&run.x),
            SVColumnView::Dense(&centre),
            horizontal,
            vertical,
            plot,
        );
        let geometry =
            sv_line_mark(colour, SVDashPattern::Solid, configuration).geometry(&line_request)?;
        geometries.push(sv_own_geometry(geometry)?);
    }
    Ok(())
}

fn sv_push_boundaries(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    request: &SVMarkRequest<'_, '_>,
    configuration: &SVBandPlotConfiguration<'_>,
    colour: scienceview_core::sv_scale::SVColour,
) -> Result<(), SVError> {
    for boundary in [SVRibbonBoundary::Upper, SVRibbonBoundary::Lower] {
        let options = sv_ribbon_options(configuration, colour).boundary(boundary);
        let geometry = SVMark::from(options).geometry(request)?;
        geometries.push(sv_own_geometry(geometry)?);
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn sv_push_boundary_lines(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    run: &SVRun,
    colour: scienceview_core::sv_scale::SVColour,
    dash: SVDashPattern,
    configuration: &SVBandPlotConfiguration<'_>,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    plot: SVResolvedRectangle,
) -> Result<(), SVError> {
    for boundary in [&run.lower, &run.upper] {
        let request = SVMarkRequest::new(
            SVColumnView::Dense(&run.x),
            SVColumnView::Dense(boundary),
            horizontal,
            vertical,
            plot,
        );
        let geometry = sv_line_mark(colour, dash.clone(), configuration).geometry(&request)?;
        geometries.push(sv_own_geometry(geometry)?);
    }
    Ok(())
}

fn sv_line_mark(
    colour: scienceview_core::sv_scale::SVColour,
    dash: SVDashPattern,
    configuration: &SVBandPlotConfiguration<'_>,
) -> SVMark {
    let mut options = SVMark::line().colour(colour).dash(dash);
    if let Some(opacity) = configuration.opacity {
        options = options.opacity(opacity);
    }
    SVMark::from(options)
}

fn sv_ribbon_options(
    configuration: &SVBandPlotConfiguration<'_>,
    colour: scienceview_core::sv_scale::SVColour,
) -> SVRibbonOptions {
    let options = SVMark::ribbon().colour(colour);
    match configuration.opacity {
        Some(opacity) => options.opacity(opacity),
        None => options,
    }
}

fn sv_require_same_length(first: usize, second: usize) -> Result<(), SVError> {
    if first == second {
        Ok(())
    } else {
        Err(sv_invalid(
            "the x, lower and upper columns of a band must have the same length",
        ))
    }
}
