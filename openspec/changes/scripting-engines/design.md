# Design

## Context

Scripting is the capability that benefits most from being optional and
interchangeable, which is why the engine is a trait and execution placement is
explicit.

## Decisions

### D1 — A trait, not a dependency

The core does not depend on any engine; engines are features of a separate crate.

### D2 — Host execution by default

The Rust runtime is the authority; embedded execution is an opt-in exception.

### D3 — Reload is part of the contract

A script is a named resource that can be replaced at runtime.

### D4 — Failures are contained

A script error is reported, never fatal to the host.

## Risks / Trade-offs

- An embedded engine inflates the WebAssembly artifact; it stays opt-in.

