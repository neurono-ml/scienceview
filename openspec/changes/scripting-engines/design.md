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


## Implementation notes

### Dependency-light engines

The shipped Lua and TypeScript/JavaScript engines are reference front-ends over a
documented, deterministic statement subset (`dispatch`, `increment`, `log`,
`read_file`, `write_file`, `http_get`, `run_process`, `read_environment`) in
each language's surface syntax. They do not embed a Lua virtual machine or a
JavaScript engine, so the default build stays dependency-free. Full-language
scripts plug in through `SVScriptEngine`, or through the narrower
`SVHostScriptExecutor` plus `sv_host_executor_engine`; the runtime, sandbox
policy, reload machinery and observability are unchanged either way.

### Scripts are producers of the existing vocabulary

A script resolves its actions at preparation time into `SVControlAction` values,
so it cannot invent a mutation. `sv_dispatch_outcome` applies them to an
`SVControlSession` under `SVControlOrigin::Script(name)`.

### The sandbox never performs I/O

The built-in interpreter grants or refuses a capability and reports a granted
request in the outcome for the host to service; it never touches the filesystem,
the network, processes or the environment itself.
