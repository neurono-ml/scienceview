# Tasks

## Stack and dodge

- [x] Write failing tests for the stacking modes and the dodge width.
- [x] Implement stack and dodge, including the window semantics.

## Jitter and shift

- [x] Write failing tests for determinism and the displacement modes.
- [x] Implement jitter and shift.

## Swarm

- [x] Write failing tests for non-overlap and determinism.
- [x] Implement the swarm layout.

## Normalise and composition order

- [x] Write failing tests for normalisation and the fixed order.
- [x] Implement normalize and the composition pipeline.


## Review follow-up

- [x] Replace the dense expansion of sparse views with a representation-aware
      accessor and a sparse working column (no slot per logical position).
- [x] Make the value buffer lazy: dodge, jitter and shift share the borrowed
      column and the test counters prove zero value materialisations.
- [x] Validate the swarm at sequence construction (`width >= 2 * marker_extent`)
      and return a typed error when the bounded search cannot place a point.
- [x] Reject non-finite values with the typed error naming the position.
- [x] Tick the FDR acceptance criteria against their tests and record the
      deferred visual example with its tracking note.
- [x] Exercise `SVMoveStage::ALL` and the per-point output accessors in a test
      and a documentation example.
- [x] Split `move_definition` into a folder module with a pure `mod.rs`.
- [x] Rewrite the sparse claim in the book chapter and document the swarm and
      non-finite behaviour.
- [x] Record the history notes below.

## Implementation notes

These notes are written for a reader of the history; the branch is not
rewritten.

- Two commits in the code history share the subject
  `test(move): assert normalisation, composition order, parity and concurrency`
  (`73dcc37` and `f675807`). `f675807` is a one-line fix-up of the test file
  `73dcc37` introduced; the subject was reused by mistake. They are kept as two
  commits so the branch history stays append-only, and this note records why.
- The first implementation of the move kernel was planned in grouped openspec
  commits (one commit per task group: stack/dodge, jitter/shift, swarm,
  normalise/pipeline) rather than one per task. The groups are intact and this
  note records the granularity; later tasks were committed one per task.
- The review follow-up was implemented as one scoped code commit per item, with
  the matching openspec update in this worktree.
