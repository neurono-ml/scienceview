//! The option surface of every mark variant.

mod area;
mod bar;
mod dot;
mod error_bar;
mod image;
mod line;
mod ribbon;
mod text;
mod validation;
mod wedge;

pub use area::{SVAreaOptions, SVBaselineMode};
pub use bar::{SVBarOptions, SVBarOrientation};
pub use dot::{SVDotOptions, SVDotShape, SVDotSizeMode, SVFillMode};
pub use error_bar::{SVErrorBarDirection, SVErrorBarOptions};
pub use image::{SVImageInterpolation, SVImageOptions};
pub use line::{SVDashPattern, SVDashSequence, SVLineCap, SVLineJoin, SVLineOptions};
pub use ribbon::{SVRibbonBoundary, SVRibbonOptions};
pub use text::{SVFormatSpec, SVTextAlign, SVTextBaseline, SVTextOptions};
pub use wedge::SVWedgeOptions;
