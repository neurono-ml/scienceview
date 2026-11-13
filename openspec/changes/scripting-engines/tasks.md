# Tasks

## Engine abstraction

- [x] Write the failing tests for loading, invoking and reloading.
- [x] Implement the engine trait and the host execution path.

## Lua and TypeScript engines

- [x] Write the failing conformance tests shared by both engines.
- [x] Implement the Lua engine and the TypeScript front-end plus JavaScript engine.

## Sandboxing and failure containment

- [x] Write the failing test: a failing script does not terminate the host.
- [x] Implement the sandbox policy and error reporting.

## Acceptance

- [x] Add the runnable example (`crates/scienceview_script/examples/scripting_engines.rs`; scripting produces actions, not pixels, so there is no generated figure) and the book chapter.
- [x] Run the four gates and open the ADR issue.

