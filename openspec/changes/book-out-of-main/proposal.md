# Proposal

## Why

The repository's rule files say the documentation book is hosted by the
`docs/documentations` branch and that documentation pull requests merge into
it rather than into `main`. In practice the book was carried on `main` too:
`docs/book.toml`, `docs/scripts/` and the whole `docs/src/` tree with its
chapters, its `SUMMARY.md` and its figures.

Two copies of a book that is meant to have one authoritative version is a
standing invitation to divergence: the branch that GitHub Pages serves and the
branch that `main` advertises will eventually tell different stories, and
nobody is responsible for reconciling them. The code side also carries a
directory it does not own — the book directory — while the figures inside it
are produced by the code, checked by the code pipeline and embedded by the
`README.md` that lives on `main`.

## What Changes

- Remove the book from `main`: `docs/book.toml`, `docs/scripts/` and the book
  chapters under `docs/src/`. The book continues to live, complete, on the
  `docs/documentations` branch.
- Move the tool-generated figures to a code-owned path, `docs/figures/`, on
  `main`: the examples write there, the `figures` job of the code pipeline
  checks that path, and the `README.md` embeds it.
- Mirror the figures into the book, under `docs/src/assets/`, on the book
  branch, so that the published book stays self-contained and pinned to the
  revision that produced the figures.
- Add a pipeline job on the book branch that regenerates the figures from the
  code on `main` and fails when the book's copy differs, so the mirror cannot
  drift unnoticed.
- Keep the book branch's `AGENTS.md` and `README.md` copies in step with
  `main`.
- Rewrite the documentation rules in `AGENTS.md` and `openspec/config.yaml` to
  describe the split: what lives on `main`, what lives on the book branch, and
  how a figure change reaches both.

**BREAKING** — the book is no longer present on `main`; a contributor reading
the book on `main` must read it on `docs/documentations`, and the published
site is the reference rendering.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. The change alters repository layout and pipeline wiring only; no
capability behaviour changes.

## Impact

- `main`: `docs/book.toml`, `docs/scripts/` and `docs/src/*.md` are removed;
  `docs/src/assets/*.svg` becomes `docs/figures/*.svg`; the two examples, the
  `README.md`, the `figures` job of the code pipeline and the documentation
  rules are updated.
- `docs/documentations`: gains the mirrored figures, the new pipeline job and
  the synchronised copies of `AGENTS.md` and `README.md`.
- The deployment workflow is unaffected: it checks out the book branch, which
  keeps the whole book.
- Depends on the foundation change being merged, because the requirements
  document arrives with it and the documentation rules cite it.
