//! The layout tree handle: a viewport, a cached solution and its invalidation.

use crate::sv_layout::SVViewport;
use crate::sv_layout::node::SVLayoutNode;
use crate::sv_layout::resolved::SVResolvedLayout;
use crate::sv_layout::solver::sv_solve;

/// A layout tree solved for one viewport and cached until it is invalidated.
///
/// The tree is the object a backend holds. Calling
/// [`SVLayoutTree::set_viewport`] with a different viewport invalidates the
/// cached solution and the next [`SVLayoutTree::resolved_layout`] re-solves it,
/// bumping the revision so a backend can tell that the geometry changed.
#[derive(Clone, Debug)]
pub struct SVLayoutTree {
    root: SVLayoutNode,
    viewport: SVViewport,
    resolved: Option<SVResolvedLayout>,
    revision: u64,
}

impl SVLayoutTree {
    /// Wraps a root node with the default viewport and no cached solution.
    pub fn new(root: SVLayoutNode) -> Self {
        Self {
            root,
            viewport: SVViewport::default(),
            resolved: None,
            revision: 0,
        }
    }

    /// Sets the initial viewport and leaves the tree invalidated.
    pub fn with_viewport(mut self, viewport: SVViewport) -> Self {
        self.viewport = viewport;
        self.resolved = None;
        self
    }

    /// Returns the root node.
    pub fn root(&self) -> &SVLayoutNode {
        &self.root
    }

    /// Returns the current viewport.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the number of solves performed through this handle.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns `true` when the cached solution is up to date.
    pub fn is_resolved(&self) -> bool {
        self.resolved.is_some()
    }

    /// Reports a new viewport.
    ///
    /// Returns `true` when the viewport actually changed and the cached layout
    /// was invalidated; reporting the same viewport twice is a no-op, so a
    /// resize storm that repeats a size does not re-solve the tree.
    pub fn set_viewport(&mut self, viewport: SVViewport) -> bool {
        if self.viewport == viewport {
            return false;
        }
        self.viewport = viewport;
        self.resolved = None;
        true
    }

    /// Drops the cached solution without changing the viewport.
    pub fn invalidate(&mut self) {
        self.resolved = None;
    }

    /// Solves one viewport without touching the cached state.
    ///
    /// Useful for measuring a prospective viewport; the returned revision is
    /// the one the cache will use next.
    pub fn solve(&self, viewport: &SVViewport) -> SVResolvedLayout {
        sv_solve(&self.root, viewport, self.revision.saturating_add(1))
    }

    /// Returns the solved layout, re-solving it if it is invalidated.
    pub fn resolved_layout(&mut self) -> &SVResolvedLayout {
        let Self {
            root,
            viewport,
            resolved,
            revision,
        } = self;
        let next_revision = revision.saturating_add(1);
        resolved.get_or_insert_with(|| {
            *revision = next_revision;
            sv_solve(root, viewport, next_revision)
        })
    }
}
