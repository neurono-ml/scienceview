# Proposal

## Why

The repository is a documentation shell: `src/lib.rs` is 40 lines of crate
documentation and `Cargo.toml` declares features that point at crates which do
not exist. Every capability specified by `foundation-core-architecture` — and
every chart in `docs/FDRs/` — needs a compilation unit to live in, a pinned
toolchain, and a CI gate that can fail. Without this change nothing else can
be reviewed, because there is nothing to compile.

This is the root of the execution DAG: every other change depends on it.

## What Changes

- Turn the package into a Cargo workspace with `crates/scienceview_*` members, one per backend feature, plus the core crate.
- Pin the toolchain (Rust 1.85, edition 2024) and add the CI gates: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets`, `cargo test --workspace`, and the `figures` drift job.
- Create the empty `docs/figures/` directory that examples write into, and the `examples/` crate surface.
- Add the `figures` CI job that reruns the examples and fails on any diff in `docs/figures/`.
- Add the license headers, `README` badges and the initial `CHANGELOG.md` wiring expected of a published Rust crate.

## Capabilities

### New Capabilities
- `workspace-bootstrap`: workspace bootstrap.

### Modified Capabilities

None.

## Impact

- New tracked paths: `crates/`, `rust-toolchain.toml`, `.github/workflows/`.
- `src/lib.rs` moves into the core crate; the root package becomes a facade that re-exports the enabled features.
- No public behaviour is introduced: the change exists to make every later change compilable and reviewable.
