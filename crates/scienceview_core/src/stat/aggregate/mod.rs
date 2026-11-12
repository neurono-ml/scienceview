//! The aggregation kernel, split by concern.

mod accumulator;
mod apply;
mod buckets;
mod quantile;
mod zeros;

pub(crate) use apply::sv_apply_aggregate;
