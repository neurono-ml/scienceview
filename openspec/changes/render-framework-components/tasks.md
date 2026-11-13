# Tasks

## Shared component contract

- [x] Write the failing test for identical properties and events across the three wrappers.
- [x] Define the shared contract.

## React, Vue and Angular wrappers

- [x] Write the failing test for each wrapper rendering and dispatching an action.
- [x] Implement the three wrappers.

## Acceptance

- [x] Add the example, generated component sources and book chapter (book branch). The runnable examples live beside each backend crate because the root `examples/` figure pipeline builds every example with `render-svg` only; the generated components are the checked-in golden sources.
- [x] Run the four gates and open the ADR issue.

## Review fixes

- [x] Tests cover golden source per framework, hostile-label escaping, determinism, the update path, ten-thousand-point output, concurrency, capability errors and the unaffected default build.
- [x] `show` reports a typed `BackendFailure` because the host application mounts the component, while the capability row still declares presentation.
