//! Solving the three panel rectangles through the layout kernel.

use super::super::configuration::{
    SV_JOINT_BOTTOM_INSET, SV_JOINT_GAP, SV_JOINT_LEFT_INSET, SV_JOINT_MARGINAL_EXTENT,
    SV_JOINT_TOP_INSET, SV_JOINT_VIEWPORT_HEIGHT, SV_JOINT_VIEWPORT_WIDTH,
};
use crate::sv_error::SVError;
use crate::sv_layout::{
    SVLayoutDirection, SVLayoutNode, SVLayoutTree, SVLength, SVResolvedRectangle,
    SVResponsivenessMode, SVViewport,
};

/// The three rectangles one joint figure solves.
pub(super) struct SVJointPanels {
    pub(super) viewport: SVViewport,
    pub(super) central: SVResolvedRectangle,
    pub(super) horizontal_marginal: SVResolvedRectangle,
    pub(super) vertical_marginal: SVResolvedRectangle,
}

/// Solves the central panel, the top marginal and the right marginal.
///
/// The layout kernel owns every number here: the recipe only names the regions
/// and their relative weights, so a viewport change re-solves the whole figure.
pub(super) fn sv_solve_panels() -> Result<SVJointPanels, SVError> {
    let viewport = SVViewport::new(SV_JOINT_VIEWPORT_WIDTH, SV_JOINT_VIEWPORT_HEIGHT);
    let fixed = |pixels: f64| SVLength::absolute(pixels);

    let root = SVLayoutNode::new("joint")
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .with_direction(SVLayoutDirection::Vertical)
        .with_spacing(SV_JOINT_GAP)
        .with_children([
            SVLayoutNode::new("joint-title-inset").with_height(fixed(SV_JOINT_TOP_INSET)),
            SVLayoutNode::new("joint-body")
                .with_width(SVLength::fill())
                .with_height(SVLength::fill())
                .with_direction(SVLayoutDirection::Horizontal)
                .with_spacing(SV_JOINT_GAP)
                .with_children([
                    SVLayoutNode::new("joint-left-inset").with_width(fixed(SV_JOINT_LEFT_INSET)),
                    SVLayoutNode::new("joint-content")
                        .with_width(SVLength::fill())
                        .with_height(SVLength::fill())
                        .with_direction(SVLayoutDirection::Vertical)
                        .with_spacing(SV_JOINT_GAP)
                        .with_children([
                            SVLayoutNode::new("horizontal-marginal")
                                .with_width(SVLength::fill())
                                .with_height(fixed(SV_JOINT_MARGINAL_EXTENT)),
                            SVLayoutNode::new("joint-middle")
                                .with_width(SVLength::fill())
                                .with_height(SVLength::fill())
                                .with_direction(SVLayoutDirection::Horizontal)
                                .with_spacing(SV_JOINT_GAP)
                                .with_children([
                                    SVLayoutNode::new("central-panel")
                                        .with_width(SVLength::fill())
                                        .with_height(SVLength::fill()),
                                    SVLayoutNode::new("vertical-marginal")
                                        .with_width(fixed(SV_JOINT_MARGINAL_EXTENT))
                                        .with_height(SVLength::fill()),
                                ]),
                            SVLayoutNode::new("joint-bottom-inset")
                                .with_height(fixed(SV_JOINT_BOTTOM_INSET)),
                        ]),
                ]),
        ]);

    let mut tree = SVLayoutTree::new(root)
        .with_viewport(viewport)
        .with_responsiveness(SVResponsivenessMode::fit_to_parent());
    let layout = tree.resolved_layout();

    let panel = |name: &str| {
        layout
            .find(name)
            .map(|node| node.rectangle())
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!("the joint layout did not solve the `{name}` region"),
            })
    };

    Ok(SVJointPanels {
        viewport,
        central: panel("central-panel")?,
        horizontal_marginal: panel("horizontal-marginal")?,
        vertical_marginal: panel("vertical-marginal")?,
    })
}
