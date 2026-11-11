//! Adapters that expose external storage through the ingestion trait.

mod borrowed_slice;
#[cfg(feature = "data-ndarray")]
mod ndarray_view;
#[cfg(feature = "data-sprs")]
mod sprs_column;
#[cfg(feature = "streaming")]
mod streaming_channel;
#[cfg(feature = "streaming-tokio")]
mod tokio_channel;

pub use borrowed_slice::SVSliceColumn;
#[cfg(feature = "data-ndarray")]
pub use ndarray_view::SVNdarrayColumn;
#[cfg(feature = "data-sprs")]
pub use sprs_column::SVSparseColumn;
#[cfg(feature = "streaming")]
pub use streaming_channel::SVChannelColumn;
#[cfg(feature = "streaming-tokio")]
pub use tokio_channel::SVTokioChannelColumn;
