//! The hexagonal lattice: the axial coordinates of every cell.
//!
//! A pointy-top hexagon of radius `r` tiles the plane with a horizontal
//! spacing of `sqrt(3) * r` and a vertical spacing of `1.5 * r`. Every
//! observation is rounded to the nearest lattice cell in cube coordinates,
//! which is the standard exact inverse of that tiling.

use crate::sv_error::SVError;
use std::collections::HashMap;

pub(super) const SV_HEXBIN_SQRT_THREE: f64 = 1.732_050_807_568_877_2;

/// The resolved domain and cell radius of one composition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct SVHexbinDomain {
    pub(super) x_lower: f64,
    pub(super) x_upper: f64,
    pub(super) y_lower: f64,
    pub(super) y_upper: f64,
    pub(super) radius: f64,
}

impl SVHexbinDomain {
    pub(super) fn new(
        x_lower: f64,
        x_upper: f64,
        y_lower: f64,
        y_upper: f64,
        grid_size: usize,
    ) -> Result<Self, SVError> {
        if grid_size == 0 {
            return Err(SVError::InvalidConfiguration {
                message: "the hexbin grid size must be at least one".to_string(),
            });
        }
        let (x_lower, x_upper) = sv_expand(x_lower, x_upper);
        let (y_lower, y_upper) = sv_expand(y_lower, y_upper);
        let radius = (x_upper - x_lower) / (SV_HEXBIN_SQRT_THREE * grid_size as f64);
        Ok(Self {
            x_lower,
            x_upper,
            y_lower,
            y_upper,
            radius,
        })
    }

    pub(super) fn centre(self) -> (f64, f64) {
        (
            (self.x_lower + self.x_upper) / 2.0,
            (self.y_lower + self.y_upper) / 2.0,
        )
    }

    pub(super) fn cell_centre(self, column: i64, row: i64) -> (f64, f64) {
        let (centre_x, centre_y) = self.centre();
        let horizontal = self.radius * SV_HEXBIN_SQRT_THREE * (column as f64 + row as f64 / 2.0);
        let vertical = self.radius * 1.5 * row as f64;
        (centre_x + horizontal, centre_y + vertical)
    }

    pub(super) fn cell_of(self, x: f64, y: f64) -> (i64, i64) {
        let (centre_x, centre_y) = self.centre();
        let horizontal = (x - centre_x) / self.radius;
        let vertical = (y - centre_y) / self.radius;
        let axial_column = (SV_HEXBIN_SQRT_THREE / 3.0) * horizontal - vertical / 3.0;
        let axial_row = (2.0 / 3.0) * vertical;
        sv_round_axial(axial_column, axial_row)
    }
}

fn sv_expand(lower: f64, upper: f64) -> (f64, f64) {
    if upper > lower {
        (lower, upper)
    } else {
        (lower - 0.5, lower + 0.5)
    }
}

/// Rounds fractional axial coordinates to the nearest hexagon in cube space.
fn sv_round_axial(column: f64, row: f64) -> (i64, i64) {
    let cube_x = column;
    let cube_z = row;
    let cube_y = -cube_x - cube_z;
    let mut rounded_x = cube_x.round();
    let rounded_y = cube_y.round();
    let mut rounded_z = cube_z.round();
    let difference_x = (rounded_x - cube_x).abs();
    let difference_y = (rounded_y - cube_y).abs();
    let difference_z = (rounded_z - cube_z).abs();
    if difference_x > difference_y && difference_x > difference_z {
        rounded_x = -rounded_y - rounded_z;
    } else if difference_z > difference_y && difference_z > difference_x {
        rounded_z = -rounded_x - rounded_y;
    }
    (rounded_x as i64, rounded_z as i64)
}

/// The key that identifies one cell inside one series.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct SVHexbinKey {
    pub(super) column: i64,
    pub(super) row: i64,
    pub(super) series: Option<u64>,
}

impl SVHexbinKey {
    pub(super) fn new(column: i64, row: i64, series: Option<f64>) -> Self {
        Self {
            column,
            row,
            series: series.map(f64::to_bits),
        }
    }

    pub(super) fn series_value(self) -> Option<f64> {
        self.series.map(f64::from_bits)
    }
}

/// The accumulated cells of one composition, in first-appearance order.
#[derive(Clone, Debug)]
pub(super) struct SVHexbinLattice {
    domain: SVHexbinDomain,
    keys: HashMap<SVHexbinKey, usize>,
    entries: Vec<SVHexbinAccumulated>,
}

#[derive(Clone, Debug)]
pub(super) struct SVHexbinAccumulated {
    pub(super) key: SVHexbinKey,
    pub(super) centre_x: f64,
    pub(super) centre_y: f64,
    pub(super) count: usize,
    pub(super) sum: f64,
}

impl SVHexbinLattice {
    pub(super) fn new(domain: SVHexbinDomain) -> Self {
        Self {
            domain,
            keys: HashMap::new(),
            entries: Vec::new(),
        }
    }

    pub(super) fn domain(&self) -> SVHexbinDomain {
        self.domain
    }

    /// Returns the cell index, creating the cell on first appearance.
    pub(super) fn deposit(&mut self, x: f64, y: f64, series: Option<f64>, value: f64) -> usize {
        let (column, row) = self.domain.cell_of(x, y);
        let key = SVHexbinKey::new(column, row, series);
        let index = match self.keys.get(&key).copied() {
            Some(index) => index,
            None => {
                let (centre_x, centre_y) = self.domain.cell_centre(column, row);
                let index = self.entries.len();
                self.keys.insert(key, index);
                self.entries.push(SVHexbinAccumulated {
                    key,
                    centre_x,
                    centre_y,
                    count: 0,
                    sum: 0.0,
                });
                index
            }
        };
        if let Some(entry) = self.entries.get_mut(index) {
            entry.count += 1;
            entry.sum += value;
        }
        index
    }

    pub(super) fn entries(&self) -> &[SVHexbinAccumulated] {
        &self.entries
    }

    pub(super) fn entries_mut(&mut self) -> &mut [SVHexbinAccumulated] {
        &mut self.entries
    }
}
