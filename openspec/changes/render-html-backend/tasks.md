# Tasks

## Document and stylesheet

- [x] Write the failing test: a saved document renders without external resources.
- [x] Implement the document emitter and stylesheet inlining.
- [x] Update the `Html` capability row in the core matrix (export, streaming, events, in-place update; no scripting, no presentation) and its tests.

## Runtime and events

- [x] Write the failing test: a gesture produces an action and a delta is applied.
- [x] Implement the embedded runtime and the event bridge.

## In-place update

- [x] Write the failing test: an update preserves the host page's state.
- [x] Implement in-place update driven by the model clock.
- [x] Embed dense, sparse and streaming snapshots without densifying sparse data.

## Acceptance

- [x] Add the example, generated figure and book chapter (book branch).
- [x] Run the four gates and open the ADR issue.
