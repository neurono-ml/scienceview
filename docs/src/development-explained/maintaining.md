# Maintaining this book

This part is a didactic, order-of-PRs textbook explaining the *why* behind
every `scienceview` change, for a reader with no Rust or numerical-computing
background.

- A new feature, figure, artist, kernel, or method adds a **new** chapter
  (`chNN-<slug>.md`), registered in `SUMMARY.md` under this part.
- A bug fix or refactor that changes existing behavior **edits** the existing
  chapter covering that behavior; fixes amend, they do not duplicate.

Every chapter is grounded in the real code and specs: the companion
`*_tests.rs` modules are the executable source of truth walked through as
examples.
