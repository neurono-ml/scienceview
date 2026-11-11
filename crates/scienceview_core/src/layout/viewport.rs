//! The environment description a backend supplies to the layout engine.
//!
//! The viewport is the only environment input: logical size, device pixel
//! ratio and, for a coarse grid such as a terminal, the size of one cell.
//! Nothing else about the environment reaches the solver.

/// The environment in which a figure is laid out.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVViewport {
    width: f64,
    height: f64,
    device_pixel_ratio: f64,
    cell_width: f64,
    cell_height: f64,
}

impl SVViewport {
    /// Describes a viewport of `width` by `height` logical pixels.
    ///
    /// Non-finite or negative sizes become zero; the device pixel ratio starts
    /// at one and the raster grid starts at one logical pixel per cell.
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width: sv_sanitize_size(width),
            height: sv_sanitize_size(height),
            device_pixel_ratio: 1.0,
            cell_width: 1.0,
            cell_height: 1.0,
        }
    }

    /// Sets the device pixel ratio; a non-positive or non-finite ratio keeps one.
    pub fn with_device_pixel_ratio(mut self, device_pixel_ratio: f64) -> Self {
        self.device_pixel_ratio = sv_sanitize_ratio(device_pixel_ratio);
        self
    }

    /// Sets the size of one raster cell; non-positive sizes keep one pixel.
    ///
    /// A terminal reports its character cell here, so the solved tree stays
    /// continuous and only rasterization snaps to the coarse grid.
    pub fn with_cell_grid(mut self, cell_width: f64, cell_height: f64) -> Self {
        self.cell_width = sv_sanitize_cell(cell_width);
        self.cell_height = sv_sanitize_cell(cell_height);
        self
    }

    /// Returns the logical width.
    pub const fn width(&self) -> f64 {
        self.width
    }

    /// Returns the logical height.
    pub const fn height(&self) -> f64 {
        self.height
    }

    /// Returns the device pixel ratio.
    pub const fn device_pixel_ratio(&self) -> f64 {
        self.device_pixel_ratio
    }

    /// Returns the width of one raster cell in logical pixels.
    pub const fn cell_width(&self) -> f64 {
        self.cell_width
    }

    /// Returns the height of one raster cell in logical pixels.
    pub const fn cell_height(&self) -> f64 {
        self.cell_height
    }

    /// Returns the width in device pixels.
    pub fn physical_width(&self) -> f64 {
        self.width * self.device_pixel_ratio
    }

    /// Returns the height in device pixels.
    pub fn physical_height(&self) -> f64 {
        self.height * self.device_pixel_ratio
    }

    /// Returns how many raster cells fit across the viewport.
    pub fn cells_wide(&self) -> usize {
        sv_cell_count(self.width, self.cell_width)
    }

    /// Returns how many raster cells fit down the viewport.
    pub fn cells_high(&self) -> usize {
        sv_cell_count(self.height, self.cell_height)
    }

    /// Snaps a horizontal coordinate to the nearest raster cell boundary.
    pub fn snap_horizontal(&self, value: f64) -> f64 {
        sv_snap(value, self.cell_width)
    }

    /// Snaps a vertical coordinate to the nearest raster cell boundary.
    pub fn snap_vertical(&self, value: f64) -> f64 {
        sv_snap(value, self.cell_height)
    }
}

impl Default for SVViewport {
    /// A 640 by 480 logical pixel viewport, the shape the documentation uses.
    fn default() -> Self {
        Self::new(640.0, 480.0)
    }
}

fn sv_sanitize_size(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        0.0
    }
}

fn sv_sanitize_cell(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        1.0
    }
}

fn sv_sanitize_ratio(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        1.0
    }
}

fn sv_cell_count(size: f64, cell: f64) -> usize {
    let count = (size / cell).floor();
    if count.is_finite() && count > 0.0 {
        count as usize
    } else {
        0
    }
}

fn sv_snap(value: f64, cell: f64) -> f64 {
    if value.is_finite() {
        (value / cell).round() * cell
    } else {
        0.0
    }
}
