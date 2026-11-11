# Proposal

## Why

The reference libraries are Python. Without bindings, a Python user
cannot use the library at all, and the project's claim to cover those libraries
is only theoretical. This is the last associated change by design: the surface
is bound only once it is validated natively.

## What Changes

- Implement Python bindings over the validated figure surface.
- Expose builders, data sources and themes with Pythonic naming while keeping the explicit backend selection.
- Ship wheels and the packaging metadata.
- Document the binding surface and its differences from the Rust API.

## Capabilities

### New Capabilities

- `python-bindings`: python bindings.

### Modified Capabilities

None.

## Impact

- New crate `crates/scienceview_python` plus the packaging directory.
- Depends on the dashboard and scripting changes: only a validated surface is bound.
- Kept last in the DAG by the AGENTS rule on associated changes.
