//! The frame one transition draws at one moment.

use super::mark_properties::SVMarkProperties;
use super::match_key::SVMatchKeyValue;

/// Which set of the transition a mark belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVTransitionPhase {
    /// The mark is only in the next state.
    Enter,
    /// The mark is in both states and moves between them.
    Update,
    /// The mark is only in the previous state.
    Exit,
}

impl SVTransitionPhase {
    /// Returns the phase name as written in observability spans.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enter => "enter",
            Self::Update => "update",
            Self::Exit => "exit",
        }
    }
}

/// One mark of a frame, with the properties it draws at this moment.
#[derive(Clone, Debug, PartialEq)]
pub struct SVResolvedMark {
    key: SVMatchKeyValue,
    phase: SVTransitionPhase,
    properties: SVMarkProperties,
}

impl SVResolvedMark {
    /// Builds a resolved mark.
    pub(crate) fn new(
        key: SVMatchKeyValue,
        phase: SVTransitionPhase,
        properties: SVMarkProperties,
    ) -> Self {
        Self {
            key,
            phase,
            properties,
        }
    }

    /// Returns the match key of the mark.
    pub fn key(&self) -> SVMatchKeyValue {
        self.key
    }

    /// Returns the set the mark belongs to.
    pub fn phase(&self) -> SVTransitionPhase {
        self.phase
    }

    /// Returns the properties the mark draws.
    pub fn properties(&self) -> &SVMarkProperties {
        &self.properties
    }
}

/// One frame of a transition: what a backend draws at one moment.
///
/// The order is deterministic: enters and updates follow the next state's order,
/// then exits follow the previous state's order. The same plan and the same
/// progress always produce the same frame.
#[derive(Clone, Debug, PartialEq)]
pub struct SVTransitionFrame {
    entries: Vec<SVResolvedMark>,
}

impl SVTransitionFrame {
    /// Builds a frame from its resolved marks.
    pub(crate) fn new(entries: Vec<SVResolvedMark>) -> Self {
        Self { entries }
    }

    /// Returns the resolved marks in frame order.
    pub fn entries(&self) -> &[SVResolvedMark] {
        &self.entries
    }

    /// Returns the number of marks the frame draws.
    pub fn number_of_marks(&self) -> usize {
        self.entries.len()
    }

    /// Returns one mark by its key, when the frame draws it.
    pub fn mark(&self, key: SVMatchKeyValue) -> Option<&SVResolvedMark> {
        self.entries.iter().find(|mark| mark.key == key)
    }
}
