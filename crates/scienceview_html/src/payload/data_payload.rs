//! The embedded data payload: what the runtime applies to a live document.
//!
//! The payload is produced by the Rust model and consumed by the inlined
//! runtime. It carries the representation the data arrived in, the current
//! snapshot, the model clock and the already-rendered SVG markup, so the
//! runtime never re-derives geometry and never owns figure state (design D2).

use super::json::{
    sv_format_json_number, sv_push_json_indices, sv_push_json_numbers, sv_push_json_string,
};
use scienceview_core::sv_data::{SVColumnView, SVRepresentation};
use scienceview_theme::sv_default_theme;

/// One embedded snapshot, kept in the representation it arrived in.
#[derive(Clone, Debug, PartialEq)]
enum SVHtmlPayloadData {
    /// A contiguous column.
    Dense(Vec<f64>),
    /// Borrowed sparse indices and values, preserved as stored.
    Sparse {
        indices: Vec<usize>,
        values: Vec<f64>,
        logical_length: usize,
    },
    /// The bounded window a live source already carries.
    Streaming {
        values: Vec<f64>,
        sequence: u64,
        dropped_samples: u64,
    },
}

/// Everything the inlined runtime needs to apply one figure state.
///
/// The value owns its markup and its numbers, so a backend can render it on
/// one thread and hand the bytes to another without borrowing the figure.
#[derive(Clone, Debug, PartialEq)]
pub struct SVHtmlDataPayload {
    representation: SVRepresentation,
    data: SVHtmlPayloadData,
    points: usize,
    revision: u64,
    appended_points: u64,
    tick_milliseconds: u32,
    reduced_motion: bool,
    markup: String,
}

impl SVHtmlDataPayload {
    /// Captures a figure view and its rendered markup.
    ///
    /// The representation is dispatched exactly once: a sparse column stays
    /// sparse and is never densified, and a streaming column contributes the
    /// snapshot it already carries, so rendering never blocks the producer.
    pub fn new(view: SVColumnView<'_>, markup: impl Into<String>) -> Self {
        let data = view.dispatch(
            |values| SVHtmlPayloadData::Dense(values.to_vec()),
            |sparse| SVHtmlPayloadData::Sparse {
                indices: sparse.indices().to_vec(),
                values: sparse.values().to_vec(),
                logical_length: sparse.length(),
            },
            |streaming| SVHtmlPayloadData::Streaming {
                values: streaming.values().to_vec(),
                sequence: streaming.sequence(),
                dropped_samples: streaming.dropped_samples(),
            },
        );

        Self {
            representation: view.representation(),
            data,
            points: view.length(),
            revision: 0,
            appended_points: 0,
            tick_milliseconds: 16,
            reduced_motion: sv_default_theme().reduced_motion(),
            markup: markup.into(),
        }
    }

    /// Records how many points the model clock appended in this revision.
    #[must_use]
    pub fn with_appended_points(mut self, appended_points: usize) -> Self {
        self.appended_points = appended_points as u64;
        self.revision = appended_points as u64;
        self
    }

    /// The representation the embedded data arrived in.
    pub fn representation(&self) -> SVRepresentation {
        self.representation
    }

    /// The logical number of points in the column.
    pub fn points(&self) -> usize {
        self.points
    }

    /// The model clock position this payload carries.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// The rendered SVG markup the runtime applies.
    pub fn markup(&self) -> &str {
        &self.markup
    }

    /// Serializes the payload as deterministic JSON.
    ///
    /// The key order is fixed, numbers use one representation and every string
    /// is escaped, so the same figure always produces the same bytes.
    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\"version\":1,\"representation\":");
        sv_push_json_string(&mut json, self.representation.as_str());
        json.push_str(",\"data\":");
        self.push_data(&mut json);
        json.push_str(",\"points\":");
        json.push_str(&self.points.to_string());
        json.push_str(",\"revision\":");
        json.push_str(&self.revision.to_string());
        json.push_str(",\"appended_points\":");
        json.push_str(&self.appended_points.to_string());
        json.push_str(",\"clock\":{\"position\":");
        json.push_str(&self.revision.to_string());
        json.push_str(",\"tick_milliseconds\":");
        json.push_str(&self.tick_milliseconds.to_string());
        json.push_str(",\"reduced_motion\":");
        json.push_str(if self.reduced_motion { "true" } else { "false" });
        json.push_str("},\"markup\":");
        sv_push_json_string(&mut json, &self.markup);
        json.push('}');
        json
    }

    fn push_data(&self, json: &mut String) {
        match &self.data {
            SVHtmlPayloadData::Dense(values) => {
                json.push_str("{\"values\":");
                sv_push_json_numbers(json, values);
                json.push('}');
            }
            SVHtmlPayloadData::Sparse {
                indices,
                values,
                logical_length,
            } => {
                json.push_str("{\"indices\":");
                sv_push_json_indices(json, indices);
                json.push_str(",\"values\":");
                sv_push_json_numbers(json, values);
                json.push_str(",\"logical_length\":");
                json.push_str(&logical_length.to_string());
                json.push('}');
            }
            SVHtmlPayloadData::Streaming {
                values,
                sequence,
                dropped_samples,
            } => {
                json.push_str("{\"values\":");
                sv_push_json_numbers(json, values);
                json.push_str(",\"sequence\":");
                json.push_str(&sequence.to_string());
                json.push_str(",\"dropped_samples\":");
                json.push_str(&dropped_samples.to_string());
                json.push('}');
            }
        }
    }
}

/// Formats one value for the payload, exposed for tests and diagnostics.
pub fn sv_payload_number(value: f64) -> String {
    sv_format_json_number(value)
}
