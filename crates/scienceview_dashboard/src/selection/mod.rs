//! Linked selections: a filter over a shared source and its shared state.

mod filter;
mod state;

pub use filter::{SVSelectionConstraint, SVSelectionFilter};
pub use state::{SVLinkedSelectionChange, SVLinkedSelectionState, SVSelectionCascade};

#[cfg(test)]
#[path = "selection_tests.rs"]
mod selection_tests;
#[cfg(test)]
#[path = "state_tests.rs"]
mod state_tests;
