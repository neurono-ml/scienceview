//! Titles, subtitles and captions.

mod title_definition;

pub use title_definition::{SVTitle, SVTitleMeasurement, SVTitlePlacement, SVTitleRole};

#[cfg(test)]
#[path = "title_tests.rs"]
mod title_tests;
