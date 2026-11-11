# Proposal

## Why

`scienceview` is a freshly initialized repository: there is no Cargo workspace, no
durable OpenSpec capability, and no `docs/PRD.md` — the document the repository
guidelines already designate as the product source of truth. Every architectural
rule the project depends on (explicit backend selection, representation parity,
builder pattern, zero-copy public APIs) currently exists only as prose in
`AGENTS.md` and `openspec/config.yaml`, with nothing enforceable behind it.

At the same time the product scope has grown well past "a plotter": the library
must serve both single, immediate figures and full dashboards with linked
filters, real-time updates, theming, scripting and an interactive UI kit that
integrates with third-party UI frameworks. These concerns interact — an event
model drives the UI kit, the UI kit drives the backend capability matrix, the
theme drives the controls, the data source drives the dashboard — so they
cannot be designed in isolation, one figure at a time, without rework.

This change establishes the foundation: the product requirements document and
the normative capability specs that the subsequent figure, backend and
dashboard changes will be implemented against. It is deliberately a
specification-first change: it produces no runtime code beyond what is needed
to make the specs testable.

## What Changes

- Add `docs/PRD.md`, the product source of truth: vision, architectural
  principles, capability catalogue (backends, data sources, script engines, UI
  kit), phased roadmap, and an explicit list of open decisions.
- Formalize the normative capabilities as OpenSpec specs, so the rules already
  written down in prose become enforceable, testable requirements:
  - **Backend selection** — every builder requires an explicit
    `backend(SVBackendKind)`; there is no `Automatic` variant; enabling a
    Cargo feature makes a backend available but never selects it; dispatch to
    the concrete backend happens per operation, never at build time.
  - **Data view boundary** — a public `SVDataSource` trait as the extensible
    ingestion contract, funnelled into a closed `SVColumnView` enumeration
    (`Dense` / `Sparse` / `Streaming`) that kernels dispatch on exactly once
    per operation; never densify sparse input, never block the render thread
    on a stream.
  - **Model events** — an observable model graph, transactional figure handles,
    serialized deltas, a cascade budget, and the distinction between user
    gesture, semantic action, mutation and event.
  - **Event payloads** — 3D-aware payloads carrying data and pixel
    coordinates, ranges, camera state, selection geometry and source identity.
  - **Backend capabilities** — a per-backend capability matrix (events,
    scripting, streaming, in-place update, export formats) surfaced to the
    builder so unsupported combinations fail early and explicitly.
  - **Theming** — a Sass/CSS theme compiled at build time *and* at runtime,
    resolved into design tokens in Rust so that native and terminal backends
    share the same visual identity as web backends.
  - **Layout engine** — a resolution-independent measure/solve pipeline with an
    `SVLength` model (pixels, fractions, intrinsic, aspect ratio, fill),
    constraints, responsiveness modes, and a backend-agnostic layout tree.
  - **UI kit** — a minimal set of configurable, themeable controls modelled as
    first-class model objects, driven by a serialized action vocabulary that is
    the integration contract for third-party UI frameworks in Rust and in
    JavaScript/TypeScript.
  - **Scripting** — an `SVScriptEngine` abstraction with Lua and
    TypeScript/JavaScript engines, host-side execution by default and
    WebAssembly-embedded execution as an opt-in feature, plus runtime editing
    and reloading of scripts.
  - **Dashboard** — grids of linked figures, shared data sources, derived
    selection views and coordinated events, implemented as a separate crate.
- Fix the architectural principle that separates *ingestion* from
  *computation*: open traits at the boundary (data sources, script engines,
  render backends), closed enumerations in the middle (`SVColumnView`), so
  third parties can extend the former while kernels keep exhaustive dispatch.
- Record the "Rust is the brain" posture — events always travel up to the Rust
  runtime, mutations always travel down as serialized deltas — with
  WebAssembly-embedded scripting and JavaScript/TypeScript engines as
  documented opt-in exceptions.
- Require in-place update (`render` + `update`, reattachable output) as a
  first-class backend obligation, because dashboards, callbacks and streaming
  are impossible without it.

## Capabilities

### New Capabilities

- `backend-selection`: explicit, per-operation backend selection with no
  automatic variant.
- `data-view-boundary`: the `SVDataSource` ingestion contract and the
  `SVColumnView` computation boundary, with dense/sparse/streaming parity and
  source adaptors (channels, streams, arrays, Polars, DataFusion).
- `model-events`: the observable model graph, transactional figure handle,
  serialized deltas and the gesture → action → mutation → event pipeline.
- `event-payloads`: the payload vocabulary of emitted events, including
  3D-aware data and pixel coordinates, ranges, camera and selection geometry.
- `backend-capabilities`: the declared capability matrix per backend and the
  early, explicit failure of unsupported combinations.
- `theming`: Sass/CSS themes, build-time and runtime compilation, design
  tokens and cross-backend visual consistency.
- `layout-engine`: the measure/solve layout pipeline, the `SVLength` sizing
  model, constraints and responsiveness modes.
- `ui-kit`: the minimal control set, its configuration surface, its integration
  contract with third-party UI frameworks, and its theming behaviour.
- `scripting`: the script engine abstraction, Lua and TypeScript engines,
  execution placement and runtime editing.
- `dashboard`: composition of linked figures, shared sources, derived
  selection views and coordinated updates.

### Modified Capabilities

None. `openspec/specs/` is empty; this change introduces the first durable
capabilities.

## Impact

- New tracked files: `docs/PRD.md` and the spec files under
  `openspec/changes/foundation-core-architecture/specs/`.
- No production code is changed by this change; it creates the contract that
  the follow-up changes (workspace creation, first figure/artist, first
  backends, dashboard crate) implement.
- The specs constrain all future changes: any new backend, data source, script
  engine or UI adapter must satisfy the capabilities above.
- The rules recorded in `AGENTS.md` and `openspec/config.yaml` that reference
  specs (`backend-selection`, `data-view-boundary`) become verifiable against
  real spec files.
