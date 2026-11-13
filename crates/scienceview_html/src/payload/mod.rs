//! The embedded payload and its deterministic JSON writer.

mod data_payload;
mod json;

pub use data_payload::SVHtmlDataPayload;
pub use data_payload::sv_payload_number;

#[cfg(test)]
#[path = "data_payload_tests.rs"]
mod data_payload_tests;
#[cfg(test)]
#[path = "json_tests.rs"]
mod json_tests;
