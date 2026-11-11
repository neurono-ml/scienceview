# Tasks

## 1. Code side: move the book off `main`

- [x] 1.1 Move the generated figures to `docs/figures/` on the code branch and verify that `git ls-files docs` lists only `PRD.md` and the figures, with no book file
- [x] 1.2 Change the two examples to write to `docs/figures/` and verify that `cargo run --example line_plot_svg` and `cargo run --example bar_chart_svg` regenerate identical figures at the new path
- [x] 1.3 Update the `README.md` to embed the figures from the new path and verify that every referenced image file exists in the repository
- [x] 1.4 Update the `figures` job of the code pipeline to check `docs/figures` and verify that the job's command detects a deliberately altered figure
- [x] 1.5 Remove `docs/book.toml`, `docs/scripts/` and the book chapters from the code branch and verify that `mdbook build docs` is no longer expected on `main`

## 2. Code side: describe the split in the rule files

- [x] 2.1 Rewrite the documentation section of `AGENTS.md` so it states that the book lives only on `docs/documentations`, that the generated figures live on `main` under `docs/figures/` and are mirrored into the book, and that a figure change requires both edits
- [x] 2.2 Update `openspec/config.yaml` so the documentation rules name the new paths and the mirror obligation, and verify that `openspec validate --strict` still passes

## 3. Book side: mirror the figures and enforce the equality

- [x] 3.1 Mirror the figures into `docs/src/assets/` on the book branch and verify that each file is byte-identical to its counterpart on the code branch
- [x] 3.2 Add a pipeline job to the book branch that regenerates the figures from the code on `main` and fails when the book's copy differs, and verify that the job fails when a mirrored figure is altered
- [x] 3.3 Synchronise the book branch's copies of `AGENTS.md` and `README.md` with `main` and verify that they match
- [x] 3.4 Remove the workspace, the sources and the examples from the book branch, and verify that the book still builds there and that the pipeline tolerates a branch without a Cargo workspace

## Workflow follow-up

- Merge the code and the planning pull requests, then the documentation pull
  request, so that the book branch receives the mirrored figures and the new
  pipeline job; the deployment workflow then republishes the site.
- Verify the published site after the deployment, confirming that the figures
  render in the chapters.
