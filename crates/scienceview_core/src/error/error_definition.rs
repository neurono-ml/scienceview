//! The single public error enumeration of the crate (P7).
//!
//! Every fallible public operation — figure configuration, rendering and the
//! data boundary — returns this type. Nothing here panics on a user path: a
//! missing backend, an unsupported capability, a malformed sparse view and a
//! source that lacks a column are all values.
//!
//! The data boundary used to carry its own `SVDataError`; its variants now live
//! here so callers match on one enumeration instead of translating between two.

use crate::sv_animation::SVKeyframeSource;
use crate::sv_backend::SVBackendCapability;
use crate::sv_backend::SVBackendKind;
use crate::sv_control::SVControlAxis;
use crate::sv_data::SVRepresentation;
use crate::sv_mark::{SVMarkKind, SVMarkProperty};
use crate::sv_move::SVMoveStage;
use thiserror::Error;

use super::SVScriptLocation;

/// Everything that can go wrong while configuring, operating or reading a
/// figure.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SVError {
    /// The builder reached `build` without a backend selection.
    #[error("a backend selection is required before a figure can be built")]
    MissingBackendSelection,

    /// The builder reached `build` without a data source.
    #[error("a data source is required before a figure can be built")]
    MissingDataSource,

    /// The selected backend cannot host a capability the figure requested.
    #[error("backend `{backend_kind}` does not support the `{capability}` capability")]
    UnsupportedBackendCapability {
        /// The configured backend.
        backend_kind: SVBackendKind,
        /// The capability it lacks.
        capability: SVBackendCapability,
    },

    /// The selected backend cannot render a mark variant the figure composed.
    #[error("backend `{backend_kind}` cannot render the `{mark_kind}` mark")]
    UnsupportedMark {
        /// The configured backend.
        backend_kind: SVBackendKind,
        /// The mark variant it lacks.
        mark_kind: SVMarkKind,
    },

    /// A kernel cannot accept the representation it was given.
    #[error(
        "`{operation}` does not support the {representation} representation; use {alternative}"
    )]
    UnsupportedRepresentation {
        /// The operation that rejected the view.
        operation: &'static str,
        /// The rejected representation.
        representation: SVRepresentation,
        /// The documented alternative path to use instead.
        alternative: &'static str,
    },

    /// No implementation of the configured backend is registered in the process.
    #[error("backend `{backend_kind}` is unavailable: no render backend is registered for it")]
    BackendUnavailable {
        /// The configured backend.
        backend_kind: SVBackendKind,
    },

    /// A registered backend failed while performing an operation.
    #[error("backend `{backend_kind}` failed during `{operation}`: {message}")]
    BackendFailure {
        /// The configured backend.
        backend_kind: SVBackendKind,
        /// The operation that failed (`render`, `update`, `save` or `show`).
        operation: &'static str,
        /// The backend's own explanation.
        message: String,
    },

    /// The configuration is internally inconsistent.
    #[error("invalid figure configuration: {message}")]
    InvalidConfiguration {
        /// What is wrong and how to fix it.
        message: String,
    },

    /// A supplied clustering tree is not a valid binary merge tree.
    #[error("the {axis} clustering tree is inconsistent at position {position}: {message}")]
    InvalidClusterTree {
        /// The axis the inconsistent tree describes (`row` or `column`).
        axis: &'static str,
        /// The merge position the violation was found at.
        position: usize,
        /// What is wrong and how to fix it.
        message: String,
    },

    /// Two moves claim the same stage of the fixed composition order.
    #[error("two moves claim the `{stage}` stage of the move order; a stage runs at most once")]
    ConflictingMoveStage {
        /// The stage both moves claim.
        stage: SVMoveStage,
    },

    /// A move occupies a stage earlier than its predecessor.
    #[error(
        "the move at position {position} occupies the `{found_stage}` stage after `{preceding_stage}`; the fixed order is displacement, then shift, then normalize"
    )]
    MoveOrderViolation {
        /// The position of the offending move in the sequence.
        position: usize,
        /// The stage of the preceding move.
        preceding_stage: SVMoveStage,
        /// The stage of the offending move.
        found_stage: SVMoveStage,
    },

    /// The requested column is not part of the source.
    #[error("column `{name}` is not part of the data source")]
    ColumnNotFound {
        /// The name that was requested.
        name: String,
    },

    /// Two columns of one table declared the same name.
    #[error("the data source declares the column `{name}` more than once")]
    DuplicateColumnName {
        /// The repeated name.
        name: String,
    },

    /// Two columns of one table have different lengths.
    #[error(
        "the columns of a data source must have the same length; the column `{name}` has {actual} values but {expected} were expected"
    )]
    MismatchedLengths {
        /// The column whose length differs.
        name: String,
        /// The length fixed by the first column.
        expected: usize,
        /// The length of the offending column.
        actual: usize,
    },

    /// A sparse view declared a different number of indices and values.
    #[error(
        "a sparse view needs one value per index; it received {indices} indices and {values} values"
    )]
    MismatchedSparseLengths {
        /// The number of stored indices.
        indices: usize,
        /// The number of stored values.
        values: usize,
    },

    /// A sparse index points past the logical length of the column.
    #[error("the sparse index {index} is out of range for a column of length {length}")]
    SparseIndexOutOfRange {
        /// The offending index.
        index: usize,
        /// The logical length of the column.
        length: usize,
    },

    /// Sparse indices are not strictly increasing.
    #[error(
        "sparse indices must be strictly increasing; the entry at position {position} does not increase"
    )]
    SparseIndicesNotIncreasing {
        /// The position of the offending index.
        position: usize,
    },

    /// An external view is not contiguous, so it cannot be borrowed as a slice.
    #[error(
        "the {origin} view is not contiguous, so it cannot be borrowed as a dense slice; {alternative}"
    )]
    NonContiguousView {
        /// The external library that produced the view.
        origin: &'static str,
        /// The documented alternative path to use instead.
        alternative: &'static str,
    },

    /// A rolling window was created with a capacity of zero.
    #[error("a rolling window needs a capacity of at least one sample, received {capacity}")]
    InvalidWindowCapacity {
        /// The rejected capacity.
        capacity: usize,
    },

    /// A source adapter was asked for a column it does not expose.
    #[error("the source adapter exposes `{expected}` and cannot serve `{requested}`")]
    UnknownColumn {
        /// The column the adapter exposes.
        expected: String,
        /// The column that was requested.
        requested: String,
    },

    /// A statistical kernel received a column with no observations.
    #[error("`{operation}` needs at least one observation but the column is empty")]
    EmptyInput {
        /// The stat operation that rejected the column.
        operation: &'static str,
    },

    /// A statistical kernel received a value that is not finite.
    #[error("`{operation}` received a non-finite value at position {position}")]
    NonFiniteValue {
        /// The stat operation that rejected the value.
        operation: &'static str,
        /// The logical position of the offending value.
        position: usize,
    },

    /// A statistical kernel received a negative observation weight.
    #[error("`{operation}` received a negative observation weight at position {position}")]
    NegativeWeight {
        /// The stat operation that rejected the weight.
        operation: &'static str,
        /// The logical position of the offending weight.
        position: usize,
    },

    /// Two columns given to one statistical kernel have different lengths.
    #[error(
        "`{operation}` needs columns of equal length; the first has {first} values and the second has {second}"
    )]
    MismatchedStatisticLengths {
        /// The stat operation that rejected the columns.
        operation: &'static str,
        /// The length of the first column.
        first: usize,
        /// The length of the second column.
        second: usize,
    },

    /// An animation duration is negative or not finite.
    #[error("an animation duration must be finite and non-negative")]
    InvalidAnimationDuration,

    /// A playback speed is zero, negative or not finite.
    #[error("a playback speed must be finite and greater than zero")]
    InvalidPlaybackSpeed,

    /// A keyframe carries a value that is not finite.
    #[error(
        "the keyframe change at position {position} sets `{property:?}` to a value that is not finite"
    )]
    NonFiniteAnimationValue {
        /// The property the non-finite value belongs to.
        property: SVMarkProperty,
        /// The position of the change inside its delta.
        position: usize,
    },

    /// One keyframe delta sets the same property twice.
    #[error("a keyframe delta sets `{property:?}` more than once")]
    DuplicateAnimationProperty {
        /// The repeated property.
        property: SVMarkProperty,
    },

    /// A fixed keyframe source decreases in time.
    #[error(
        "the keyframe at position {position} is stamped {found_milliseconds} ms but the preceding keyframe is stamped {previous_milliseconds} ms; timestamps must not decrease"
    )]
    OutOfOrderKeyframes {
        /// The position of the offending keyframe.
        position: usize,
        /// The timestamp of the preceding keyframe, in milliseconds.
        previous_milliseconds: u64,
        /// The timestamp of the offending keyframe, in milliseconds.
        found_milliseconds: u64,
    },

    /// A fixed keyframe source was built without any keyframe.
    #[error("a fixed keyframe source needs at least one keyframe")]
    EmptyKeyframeSource,

    /// A keyframe arrived on a source that does not accept arrivals.
    #[error(
        "`{operation}` requires a streaming keyframe source; this timeline is `{keyframe_source}`"
    )]
    AnimationSourceMismatch {
        /// The operation that needed a stream.
        operation: &'static str,
        /// The configured keyframe source.
        keyframe_source: SVKeyframeSource,
    },

    /// A streaming keyframe window is too small to hold one transition.
    #[error("a streaming animation needs a keyframe capacity of at least two, received {capacity}")]
    InvalidAnimationCapacity {
        /// The rejected capacity.
        capacity: usize,
    },

    /// A control was built with an extent that is not finite or not positive.
    #[error("a control needs finite, strictly positive width and height")]
    InvalidControlSize,

    /// A disabled control refused to activate.
    #[error("the control `{name}` is disabled and cannot be activated")]
    ControlDisabled {
        /// The control identity.
        name: String,
    },

    /// An action targeted an axis the reducer never registered.
    #[error("the {axis} axis is not registered with the control state")]
    UnknownControlAxis {
        /// The axis the action targeted.
        axis: SVControlAxis,
    },

    /// A playback action was dispatched without an animation timeline.
    #[error("a playback control needs an animation timeline attached to the session")]
    MissingAnimationTimeline,

    /// A script was invoked by a name the runtime never registered.
    #[error("no script is registered under the name `{name}`")]
    ScriptNotFound {
        /// The name that was requested.
        name: String,
    },

    /// A script was registered under a name that is already taken.
    #[error("a script is already registered under the name `{name}`")]
    ScriptAlreadyRegistered {
        /// The duplicated script identity.
        name: String,
    },

    /// A script could not be prepared by its engine.
    #[error("the script `{script}` failed to load at {location}: {message}")]
    ScriptParseFailure {
        /// The script identity.
        script: String,
        /// Where the failure happened.
        location: SVScriptLocation,
        /// The engine's own explanation.
        message: String,
    },

    /// A script failed while executing one invocation.
    #[error("the script `{script}` failed at {location}: {message}")]
    ScriptRuntimeFailure {
        /// The script identity.
        script: String,
        /// Where the failure happened.
        location: SVScriptLocation,
        /// The engine's own explanation.
        message: String,
    },

    /// A script requested a capability its sandbox does not grant.
    #[error(
        "the script `{script}` requested the `{capability}` capability at {location}, which its sandbox does not grant"
    )]
    ScriptSandboxViolation {
        /// The script identity.
        script: String,
        /// Where the refused statement is.
        location: SVScriptLocation,
        /// The capability that was refused.
        capability: String,
    },

    /// A script dispatched an action name outside the control vocabulary.
    #[error(
        "the script `{script}` dispatched the unknown action `{action}` at {location}; the vocabulary is zoom, pan, reset, series, range, selection, playback and custom"
    )]
    ScriptUnknownAction {
        /// The script identity.
        script: String,
        /// Where the unknown action was dispatched.
        location: SVScriptLocation,
        /// The name the script used.
        action: String,
    },

    /// A script exceeded the number of statements one invocation may run.
    #[error(
        "the script `{script}` exceeded its statement budget of {budget} statements in one invocation"
    )]
    ScriptBudgetExceeded {
        /// The script identity.
        script: String,
        /// The configured limit.
        budget: usize,
    },

    /// The selected execution placement is not compiled into this build.
    #[error(
        "script execution placement `{placement}` is not available: enable the `embedded-execution` feature"
    )]
    UnsupportedScriptPlacement {
        /// The placement the configuration requested.
        placement: String,
    },

    /// An engine instance does not match the engine the runtime was built with.
    #[error(
        "the runtime is configured for the `{configured}` engine but the program was prepared for `{prepared}`"
    )]
    ScriptEngineMismatch {
        /// The engine the runtime selected.
        configured: String,
        /// The engine that prepared the program.
        prepared: String,
    },

    /// A dashboard grid was declared with an unusable shape or spacing.
    #[error("invalid dashboard grid: {message}")]
    InvalidDashboardGrid {
        /// What is wrong and how to fix it.
        message: String,
    },

    /// A panel placement falls outside the declared grid.
    #[error(
        "the panel `{panel}` at row {row}, column {column} with span {row_span}x{column_span} falls outside a {rows}x{columns} grid"
    )]
    DashboardPlacementOutOfBounds {
        /// The offending panel.
        panel: String,
        /// The declared first row.
        row: usize,
        /// The declared first column.
        column: usize,
        /// The declared row span.
        row_span: usize,
        /// The declared column span.
        column_span: usize,
        /// The number of rows the grid declares.
        rows: usize,
        /// The number of columns the grid declares.
        columns: usize,
    },

    /// Two dashboard panels occupy the same grid unit.
    #[error("the dashboard panels `{first}` and `{second}` occupy the same grid unit")]
    OverlappingDashboardPanels {
        /// One of the two panels.
        first: String,
        /// The other panel.
        second: String,
    },

    /// A panel identifier was registered twice.
    #[error("the dashboard declares the panel `{panel}` more than once")]
    DuplicateDashboardPanel {
        /// The repeated identifier.
        panel: String,
    },

    /// A dashboard operation named a panel that was never registered.
    #[error("the dashboard has no panel named `{panel}`")]
    UnknownDashboardPanel {
        /// The requested identifier.
        panel: String,
    },

    /// A panel or a filter named a shared source that was never registered.
    #[error("the dashboard has no shared source named `{source_identifier}`")]
    UnknownDashboardSource {
        /// The requested identifier.
        source_identifier: String,
    },

    /// A shared source identifier was registered twice.
    #[error("the dashboard registers the shared source `{source_identifier}` more than once")]
    DuplicateDashboardSource {
        /// The repeated identifier.
        source_identifier: String,
    },

    /// A panel or an event named a linked axis that was never registered.
    #[error("the dashboard has no linked axis named `{axis}`")]
    UnknownDashboardAxis {
        /// The requested identifier.
        axis: String,
    },

    /// A linked axis identifier was registered twice.
    #[error("the dashboard registers the linked axis `{axis}` more than once")]
    DuplicateDashboardAxis {
        /// The repeated identifier.
        axis: String,
    },

    /// An action targeted a local axis that the linked axis does not serve.
    #[error(
        "the linked axis `{axis}` serves the {expected} axis but the action targeted the {found} axis"
    )]
    DashboardAxisMismatch {
        /// The linked axis identifier.
        axis: String,
        /// The direction the linked axis was registered with.
        expected: SVControlAxis,
        /// The direction the action targeted.
        found: SVControlAxis,
    },

    /// A linked selection cascade exceeded its depth budget and was stopped.
    #[error(
        "the linked selection cascade reached depth {depth}, beyond the configured budget of {budget}"
    )]
    LinkedSelectionCycle {
        /// The depth that exceeded the budget.
        depth: usize,
        /// The configured maximum depth.
        budget: usize,
    },

    /// A panel produced markup the dashboard cannot place in a cell.
    #[error("the panel `{panel}` did not produce a placeable SVG fragment: {message}")]
    InvalidDashboardPanelFragment {
        /// The offending panel.
        panel: String,
        /// Why the fragment was rejected.
        message: String,
    },

    /// A control action was routed to a dashboard path that does not serve it.
    #[error("a dashboard cannot route the `{action}` control action; {alternative}")]
    UnsupportedDashboardAction {
        /// The action family that was rejected.
        action: &'static str,
        /// The documented alternative path to use instead.
        alternative: &'static str,
    },
}
