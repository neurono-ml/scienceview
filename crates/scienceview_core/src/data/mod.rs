//! The data boundary: open ingestion, closed representation, bounded streaming.
//!
//! Every source implements [`SVDataSource`] and resolves to exactly one
//! [`SVColumnView`] — dense, sparse or streaming — which a kernel dispatches on
//! once. Sparse storage keeps its indices and is never densified; streaming
//! sources present a bounded rolling window so a read never blocks the render
//! thread. Representation parity is proven with
//! [`SVRepresentationParityHarness`], whose tolerance policy is documented on
//! [`SVRepresentationTolerance`].
//!
//! The built-in adapters are feature-gated where they need an extra
//! dependency: `data-ndarray`, `data-sprs`, `streaming` and `streaming-tokio`.
//! The borrowed-slice, table, sparse and parity modules are always available.

mod adapters;
mod column_view;
mod parity;
mod representation;
mod source;
mod sparse;
mod table;

pub use adapters::*;
pub use column_view::{SVColumnView, SVStreamingView};
pub use parity::{
    SVRepresentationParityHarness, SVRepresentationTolerance, sv_assert_representation_parity,
};
pub use representation::SVRepresentation;
pub use source::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVDataSource, SVReactivity, SVSourceSchema,
};
pub use sparse::SVSparseView;
pub use table::{SVDataTable, SVDataTableBuilder};
