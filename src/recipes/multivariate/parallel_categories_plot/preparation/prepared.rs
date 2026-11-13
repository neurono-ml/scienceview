//! The owned, data-space result of one parallel categories preparation.

use crate::sv_scale::SVColour;

/// One categorical axis after counting its categories.
pub(crate) struct SVPreparedAxis {
    pub(crate) name: String,
    pub(crate) label: String,
    pub(crate) categories: Vec<f64>,
    pub(crate) category_labels: Vec<String>,
}

/// One aggregated ribbon between two adjacent axes.
pub(crate) struct SVPreparedRibbon {
    pub(crate) left_axis: usize,
    pub(crate) right_axis: usize,
    pub(crate) left_category: usize,
    pub(crate) right_category: usize,
    pub(crate) colour_index: usize,
    pub(crate) count: u64,
    pub(crate) colour: SVColour,
}

/// The prepared parallel categories plot: every axis and every ribbon.
pub(crate) struct SVPreparedParallelCategories {
    pub(crate) axes: Vec<SVPreparedAxis>,
    pub(crate) ribbons: Vec<SVPreparedRibbon>,
    pub(crate) number_of_observations: usize,
}
