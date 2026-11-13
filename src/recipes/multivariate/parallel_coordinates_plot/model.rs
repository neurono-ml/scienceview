//! The public axis and polyline records of one composition.

use crate::sv_mark::SVMarkPoint;
use crate::sv_scale::{SVColour, SVScale};

/// One drawn vertical axis with its own continuous scale.
#[derive(Clone, Debug, PartialEq)]
pub struct SVParallelCoordinatesAxis {
    label: String,
    position: f64,
    scale: SVScale<'static>,
    coloured: bool,
}

impl SVParallelCoordinatesAxis {
    pub(super) fn new(
        label: String,
        position: f64,
        scale: SVScale<'static>,
        coloured: bool,
    ) -> Self {
        Self {
            label,
            position,
            scale,
            coloured,
        }
    }

    /// Returns the variable name the axis displays.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the horizontal position of the axis line in logical pixels.
    pub fn position(&self) -> f64 {
        self.position
    }

    /// Returns the continuous scale that maps values onto the axis height.
    pub fn scale(&self) -> &SVScale<'static> {
        &self.scale
    }

    /// Returns whether the axis line is tinted with the categorical palette.
    pub fn is_coloured(&self) -> bool {
        self.coloured
    }
}

/// One drawn polyline through the visible axes.
#[derive(Clone, Debug, PartialEq)]
pub struct SVParallelCoordinatesLine {
    observation_index: usize,
    group: Option<String>,
    colour: SVColour,
    vertices: Vec<SVMarkPoint>,
}

impl SVParallelCoordinatesLine {
    pub(super) fn new(
        observation_index: usize,
        group: Option<String>,
        colour: SVColour,
        vertices: Vec<SVMarkPoint>,
    ) -> Self {
        Self {
            observation_index,
            group,
            colour,
            vertices,
        }
    }

    /// Returns the logical index of the observation the line came from.
    pub fn observation_index(&self) -> usize {
        self.observation_index
    }

    /// Returns the colour group of the observation, when a colour column is bound.
    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }

    /// Returns the stroke colour of the polyline.
    pub fn colour(&self) -> SVColour {
        self.colour
    }

    /// Returns every vertex in axis order.
    pub fn vertices(&self) -> &[SVMarkPoint] {
        &self.vertices
    }
}
