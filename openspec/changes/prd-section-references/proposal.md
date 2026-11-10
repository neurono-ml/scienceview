# Proposal

## Why

`AGENTS.md` and `openspec/config.yaml` cite sections of the product
requirements document as `PRD §3.1`, `§3.4`, `§4.5`, `§8.7`, `§10.3` and
`§12`. Those numbers were taken from a draft of the document that never
existed, and they point at nothing: the actual requirements document numbers
its sections differently.

The consequence is silent. An agent or contributor following `AGENTS.md`
looks for a rule in `PRD §3.4` and finds nothing there, then has to guess
whether the rule is missing, misnumbered or simply absent. The foundation
change added a temporary "section map" note at the top of the requirements
document to compensate, but a map in the target document is a workaround: the
references still point at the wrong places.

The rules that depend on those references are load-bearing — where the
workspace crates live, the naming convention, the representation-parity rule
and the acceptance criteria — so a broken pointer is a correctness problem in
the repository's own source of truth, not a cosmetic one.

## What Changes

- Repoint every `PRD §…` reference in `AGENTS.md` and `openspec/config.yaml`
  at the section that actually carries the rule in the requirements document.
- Remove the temporary "section map" note from the requirements document once
  the references are correct.
- Cite the durable capability specifications, where they exist, as the
  normative source for each rule, so that a pointer resolves to a testable
  requirement and not only to prose.

**BREAKING** — none: the change alters documentation pointers only, no
behaviour, no API and no requirement.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. The capability specifications themselves are unaffected; this change
repairs the pointers that reference them from the repository's rule files.

## Impact

- `AGENTS.md` — five section references corrected.
- `openspec/config.yaml` — two section references corrected.
- `docs/PRD.md` — the temporary section map removed once the references are
  accurate.
- Depends on the foundation change being merged, since the requirements
  document it repairs arrives with it.
