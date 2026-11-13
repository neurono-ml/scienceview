//! Multivariate recipes: charts whose subject is several variables at once.
//!
//! Every chart in this family appends one `#[path] pub mod` and one
//! `pub use` to this dispatcher; the family batch integrator resolves the
//! ordering, so recipe changes never edit a central registry (patterns.md
//! P4). Recipe module names carry the `sv_` prefix so the family scope
//! never collides.

#[path = "joint_plot/mod.rs"]
pub mod sv_joint_plot;
pub use sv_joint_plot::{
    SVJointFrame, SVJointKind, SVJointMarginal, SVJointMissingValuePolicy, SVJointPlot,
    SVJointPlotBuilder, SVJointRegressionModel, SVJointRenderOutput, SVJointScene,
};
