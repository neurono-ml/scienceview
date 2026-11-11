# Design

## Context

The repository contains only a documentation-only `src/lib.rs`, two examples
that hand-assemble SVG strings, and prose rules in `AGENTS.md` /
`openspec/config.yaml`. Nothing in the codebase constrains the API yet, so the
design decisions recorded here are the ones everything else will be built on.

The product brief combines two audiences that historically needed different
tools: people who want an immediate, single, interactive figure, and people who
assemble many linked figures into a dashboard with shared filters and real-time
updates. The reference points for capabilities and style are Plotly (declarative
figure schema, rich named events, modebar-style controls) and Bokeh (model
graph with observable properties, `ColumnDataSource` as the pivot for streaming
and shared filtering, JavaScript callbacks). Neither offers scripting that runs
outside the browser, and neither offers a coherent cross-backend theming model.

Constraints inherited from the repository: Rust 1.85 / edition 2024, dependency-
light core, every non-core capability behind a Cargo feature, zero-copy public
APIs, builder pattern with `SV`-prefixed public items, files over 200 lines
split into standardized folder modules, tests in companion `*_tests.rs` modules,
and the "no automatic backend" rule.

## Goals / Non-Goals

**Goals:**

- Establish one vocabulary (actions, events, deltas, tokens, lengths) reused by
  figures, the UI kit, dashboards and every backend.
- Keep the core free of heavy dependencies and of any async runtime.
- Make third-party extension points real and cheap: new backends, new data
  sources, new script engines and third-party UI frameworks must plug in
  without modifying the core.
- Ensure the architecture survives 3D, streaming, theming and dashboards
  without an API break.

**Non-Goals:**

- Implementing any figure, artist, backend or dashboard in this change.
- Choosing the exact set of Rust and web UI frameworks to support first.
- Binding to a specific version of the Sass toolchain or of any JavaScript
  engine beyond what the scripting capability requires.
- Designing the Python binding surface.

## Decisions

### D1 — Open traits at the boundary, closed enumerations in the middle

Every extension point is a public trait (`SVDataSource`, `SVScriptEngine`,
`SVRenderBackend`), and every trait is funnelled into a small closed
enumeration before computation begins (`SVColumnView` with `Dense`, `Sparse`,
`Streaming`).

*Rationale.* A purely trait-based design gives third parties full extension but
costs the kernels exhaustive dispatch and SIMD-friendly static typing; a purely
enumerated design is fast but forces every new source into the core. Splitting
the world at the representation boundary gives both: the kernel sees exactly
three shapes and can `match` exhaustively once per operation, while any
producer — slices, `ndarray`, Polars, Arrow, DataFusion, a user's own crate —
only has to produce a view.

*Alternatives considered.* Trait-only dispatch (rejected: dynamic dispatch on
the hot path, no exhaustive matching, weak zero-copy guarantees). Enum-only
(rejected: third-party sources would require core changes).

### D2 — Backend selection is explicit and resolved per operation

The builder requires `backend(SVBackendKind)` with no `Automatic` variant.
Enabling a Cargo feature makes a backend available; it never selects one. The
kind is carried by the built figure and recorded in the observability span of
every operation; dispatch to the concrete `SVRenderBackend` happens inside each
operation, never in `build()`.

*Rationale.* Automatic selection hides a real cost difference (SVG vs. native
vs. WASM) and makes rendered output unpredictable. Carrying the kind as a
runtime value keeps the figure non-generic apart from an optional `SVFloat`
scalar, so a figure can be moved between backends at runtime.

*Alternatives considered.* Compile-time generic parameter per backend
(rejected: it would make the figure type explode and prevent runtime switching
that dashboards need).

### D3 — Four-layer event pipeline: gesture → action → mutation → event

User input is captured by a backend, resolved into a semantic *action*
(`SVAction`), applied to the model as a *mutation* (a serialized delta), and
only then published as an *event* (`SVEvent`) to subscribers.

*Rationale.* Separating the intent from the mutation is what makes the UI kit
integration contract possible: any third-party widget (egui, iced, React, Vue,
Angular) only needs to dispatch an action and subscribe to events; it never
needs to know how the figure stores its state. It also breaks the feedback
loop: a callback that mutates the model produces a new event, which may
trigger another callback — the cascade budget and origin-based suppression
bound that.

*Alternatives considered.* Direct mutable access to the figure in callbacks
(rejected: borrow conflicts, no dirty tracking, re-entrancy hazards). Callbacks
returning a full declarative delta (rejected as the only mode: too verbose for
conditional logic; kept as the wire format instead).

### D4 — Transactional handle, batched application, serialized deltas

Callbacks receive an `SVFigureHandle`; mutations are queued commands and
applied atomically when the callback returns, producing a single change batch.
Each batch is a serializable delta, so it can be logged, replayed, tested, or
sent across a WebAssembly boundary.

*Rationale.* This gives imperative ergonomics (Bokeh-like) and declarative
portability (Plotly-like) at once, and it makes dirty tracking exact: the batch
says precisely which properties changed, which is what drives incremental
rendering.

### D5 — "Rust is the brain", with documented exceptions

Events always travel up to the Rust runtime; mutations always travel down as
deltas. The default for scripts is host-side execution in Rust. Two documented
exceptions exist, both opt-in through features and builder configuration: a
script engine embedded in the WebAssembly artifact (for backends where the
browser-side runtime is the only sensible latency path), and JavaScript /
TypeScript engines for web-native integration.

*Rationale.* A single authority for state avoids divergence between a host
model and a UI-side model. The exceptions exist because some deployments
genuinely need the interaction to be resolved entirely in the client.

*Trade-off.* Round-trip latency for UI interactions in web backends. Mitigated
by requiring in-place, incremental update from backends.

### D6 — Script engines are pluggable; Lua and TypeScript are the first two

`SVScriptEngine` loads, invokes, and reloads source at runtime. Engines are
selected in the figure builder together with the execution placement
(`Host` or `Embedded`) and a sandboxing policy.

*Rationale.* Scripting is the one feature that most benefits from being
optional and interchangeable. Runtime editing (a script being replaced and
reloaded without rebuilding the figure) is a first-class requirement, which is
why `reload` is part of the engine trait rather than an implementation detail.
TypeScript is compiled to JavaScript before execution by a pure-Rust
front-end, because no embeddable engine executes TypeScript natively.

### D7 — Theme is authored in Sass, resolved to tokens in Rust

A theme is a Sass source compiled to CSS at build time *and* at runtime. The
compiled result is resolved into a token set in Rust. Web backends emit the
CSS so users can override it; native and terminal backends consume the
resolved tokens directly.

*Rationale.* One authored source for a single visual identity across ten
backends, with per-platform overrides only where the platform demands it
(text rendering differences). Runtime compilation is what makes "the user
restyles the figure from their own program" possible without a rebuild.

### D8 — Layout is resolution-independent and solved in two passes

The backend supplies a viewport (size, device pixel ratio, or cell grid); the
layout engine measures and solves a layout tree in logical pixels; each backend
rasterizes that tree onto its own grid.

*Rationale.* TUI cells, SVG subpixels and CSS pixels are different raster
grids; solving layout per backend would multiply the code and the bugs. The
two-pass split (measure from the environment, solve against constraints) also
gives the responsive story a single trigger: a viewport change invalidates the
layout tree and re-ticks the axes.

### D9 — Sizing is a first-class value with constraints

`SVLength` covers pixels, fractions of the parent, intrinsic size, aspect
ratio and fill, with minimum and maximum constraints on both axes, combined
into an explicit responsiveness mode.

*Rationale.* "Responsive" is meaningless without saying *responsive to what*,
and min/max bounds are what make a figure usable in a dashboard cell or a
mobile viewport.

### D10 — The UI kit is a set of model objects driven by an action vocabulary

Controls (buttons, toggles, sliders, selects, range sliders, brush tools) are
models in the observable graph, not artists. They are configured declaratively,
styled from theme tokens, and dispatch `SVAction` values. The same action
vocabulary is the integration contract for third-party UI frameworks.

*Rationale.* The built-in kit and third-party integration are then the same
code seen from two sides: the built-in kit is simply one more adapter that
renders control models in a given backend. This avoids the classic duplication
where every dashboard widget reimplements zoom or range logic.

*Alternatives considered.* Making controls artists (rejected: controls are
chrome, not data marks, and mixing them pollutes the artist model). Exposing
the figure's internal state to third-party widgets (rejected: it leaks the
representation and breaks the action abstraction).

### D11 — Backends must support in-place update and declare their capabilities

A backend that can only produce a one-shot export is not sufficient for
dashboards, callbacks or streaming. Each backend declares a capability matrix
(events, scripting, streaming, in-place update, export formats) that the
builder consults, so unsupported combinations fail at build time with an
explicit error instead of failing silently at runtime.

*Rationale.* Silent degradation is the worst outcome for an interactive
library: the user discovers at runtime that their callback never fires.

### D12 — Data sources are synchronous at the boundary, asynchronous inside

`SVDataSource` exposes synchronous views. Adapters for asynchronous producers
(channels, `Stream`, DataFusion record batches) own their runtime: they drain
into an internal rolling window and present a `Streaming` view, optionally
waking the render scheduler when new data arrives.

*Rationale.* The core stays runtime-free and the render thread never blocks.
The cost is that the window copy is not zero-copy for asynchronous sources —
an accepted, documented trade-off.

## Risks / Trade-offs

- **Window copy for asynchronous sources breaks strict zero-copy** → Document
  the boundary explicitly in `data-view-boundary`; asynchronous adapters are
  the only sanctioned copy, and the copy is bounded by the rollover window.
- **Two-pass layout can disagree with backend reality** (e.g. TUI rounding,
  font metrics) → Backends report their measurement constraints through the
  viewport, and the layout tree is the single source of truth for geometry;
  backends never re-derive positions.
- **In-place update is significantly harder than one-shot export** → The
  capability is declared in the specs now, so backends are designed for it
  from the start rather than retrofitted.
- **The action vocabulary may grow ad hoc** → Keep it small and generic
  (roughly twenty core actions), allow a serialized custom payload for
  third-party actions, and require that a new core action be justified by
  reuse across the UI kit and the dashboard.
- **Runtime Sass compilation pulls a toolchain into the runtime** → Keep it
  behind a feature, isolate it in its own crate, and fall back to the
  build-time compiled theme when the feature is absent.
- **Scripting in WebAssembly inflates the artifact** → Keep embedded execution
  opt-in per feature, never on by default.
- **The core and the dashboard crate evolve together** → The dashboard is a
  separate crate precisely so it cannot force core APIs; if a dashboard need
  appears to require a core change, the core change is justified on its own
  merits or the capability moves.
- **The specs are broad** → Mitigation: each capability is written with
  testable scenarios, and the implementation changes that follow are split into
  the repository's usual groups (figure/artist, backends, documentation).

## Migration Plan

Not applicable — this change introduces specifications only; no runtime code or
public API changes.

## Open Questions

These are deferrable: answering them later does not change the specs, the
approach or the task breakdown.

- Which Rust UI frameworks are supported first (`egui`, `ratatui` before
  `iced`, `slint`)? The contract does not depend on the order.
- Which export formats are first-class beyond SVG and HTML (PNG, PDF, CSV of
  the underlying data)?
- Whether `dashboard` remains one capability or splits into composition and
  linked-filtering capabilities once the first dashboard is built.
- Whether a `wasm`-embedded Lua engine or a JavaScript engine is the better
  default for the WebAssembly exception.
- Whether the TypeScript engine should ship a small preprocessor of its own or
  rely on an existing pure-Rust transpiler.
