# Tasks

## Workspace skeleton

- [x] Write the failing test: a workspace-level smoke test that asserts the core crate is reachable and that a backend feature is off by default.
- [x] Create the workspace `Cargo.toml` with the members and the feature map already declared by the PRD.
- [x] Create the core crate and the ten backend crate skeletons, each with a minimal `lib.rs` carrying the crate documentation.

## Toolchain and lints

- [x] Pin the toolchain in `rust-toolchain.toml` and set `rust-version` in every crate.
- [x] Configure the lint set (warnings as errors) and the formatter.
- [x] Verify `cargo fmt --all --check`, `cargo clippy --workspace --all-targets` and `cargo test --workspace` are green.

## CI gates

- [x] Add the CI workflow with the four gates.
- [x] Add the `figures` job that runs the examples and fails on a diff in `docs/figures/`.
- [x] Verify the workflow fails when a gate is deliberately broken, then restore it.

## Documentation of the skeleton

- [x] Document the crate map and the feature map in the core crate's documentation.
- [x] Add the CI status, crates.io and docs.rs badges to `README.md`.
- [x] Add the `CHANGELOG.md` entry for the bootstrap.

