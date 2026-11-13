//! Mark geometry for the composed ECDF plot.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVMark, SVMarkGeometry, SVMarkRequest, SVMarkTheme};

use super::definition::SVEcdfPlotComposition;

impl SVEcdfPlotComposition {
    /// Produces one line geometry per curve.
    ///
    /// # Errors
    ///
    /// Returns the mark kernel's configuration error when an option or a
    /// scale cannot be honoured.
    pub fn mark_geometries(&self) -> Result<Vec<SVMarkGeometry<'_>>, SVError> {
        let theme = SVMarkTheme::default();
        let mut geometries = Vec::with_capacity(self.series.len());
        for series in &self.series {
            let mut line = SVMark::line().colour(series.colour);
            if let Some(width) = self.line_width {
                line = line.width(width);
            }
            let mark: SVMark = line.into();
            let request = SVMarkRequest::new(
                SVColumnView::Dense(&series.horizontal),
                SVColumnView::Dense(&series.vertical),
                &self.horizontal_scale,
                &self.vertical_scale,
                self.plot_area,
            )
            .with_theme(theme.clone());
            geometries.push(mark.geometry(&request)?);
        }
        Ok(geometries)
    }
}
