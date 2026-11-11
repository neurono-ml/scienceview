# Tasks

## 1. Product requirements document

- [x] 1.1 Write `docs/PRD.md` with the vision, the architectural principles, the capability catalogue (backends, data sources, script engines, UI kit), the phased roadmap and the open decisions, and verify that every architectural principle named in `AGENTS.md` and `openspec/config.yaml` appears in it
- [x] 1.2 Verify that `docs/PRD.md` contains no reference to capabilities that are not described in `openspec/changes/foundation-core-architecture/specs/`, by cross-checking every capability name mentioned in the document against the spec directories

## 2. Capability specifications

- [x] 2.1 Verify that each of the ten capability specs has a purpose section, requirements written with normative wording, and at least one scenario per requirement, by running `openspec validate "foundation-core-architecture" --strict`
- [x] 2.2 Verify that the scenarios of `data-view-boundary` cover dense, sparse and streaming input for every artist operation that accepts more than one representation, by reviewing the scenario list against the requirement text
- [x] 2.3 Verify that `backend-selection` and `backend-capabilities` together forbid an automatic backend and require an explicit failure for every unsupported capability combination
- [x] 2.4 Verify that `model-events` and `event-payloads` together describe the full pipeline gesture, action, mutation and event, with a scenario for the cascade budget and for the three-dimensional payload

## 3. Repository wiring

- [x] 3.1 Reference the new product requirements document from `README.md` and from `openspec/config.yaml`, and verify that the links resolve
- [x] 3.2 Add the product requirements document to the documentation plan for the companion documentation change, listing the chapters to be written for the development-explained book, and verify the chapter list covers the ten capabilities

## Workflow follow-up

- The runtime implementation is deliberately not part of this change. It is delivered by the follow-up changes, each with its own proposal, specs, tasks, code worktree and documentation worktree: workspace creation and the core model graph, the first figure and artist with the SVG backend, the remaining backends, the data source adaptors, the script engines, the UI kit adaptors, and the dashboard crate.
- Open the matching architecture decision record issue in English, referencing this change, and close it with the planning pull request.
- After the planning pull request merges, sync and archive this change so that the ten capabilities become durable specs under `openspec/specs/`.
