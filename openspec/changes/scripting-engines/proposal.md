# Proposal

## Why

Callbacks in Rust cover the compiled case; scripts cover the case where
the behaviour is authored and reloaded at runtime. The scripting capability is
specified with pluggable engines, host execution by default and runtime
editing; none of it exists.

## What Changes

- Implement `SVScriptEngine` and the host-side execution path.
- Ship the Lua and TypeScript/JavaScript engines, with TypeScript compiled to JavaScript by a pure-Rust front-end.
- Support named scripts that can be replaced and reloaded at runtime, sandboxed by default.
- Report script failures without taking the host program down.

## Capabilities

### New Capabilities

- `scripting-engines`: scripting engines.

### Modified Capabilities

None.

## Impact

- New crate `crates/scienceview_script`.
- Depends on the figure model and its mutation pipeline.
- Optional features: engine selection, sandboxing policy, embedded execution.
