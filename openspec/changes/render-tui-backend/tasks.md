# Tasks

## Cell rasterizer

- [x] Write the failing tests for snapping and partial blocks.
- [x] Implement the rasterizer over the layout tree.

## Marks and decorations

- [x] Write the failing snapshot tests for each mark and decoration.
- [x] Implement the cell approximations.

## Clock

- [x] Write the failing test for elapsed-time advancement.
- [x] Implement the tick loop.

## Acceptance

- [x] Add the headless example (`crates/scienceview_tui/examples/terminal_frame.rs`)
  and the book chapter (book branch). A terminal frame is text, not a vector
  document, so this change generates no SVG figure under `docs/figures/`.
- [x] Run the four gates and open the ADR issue.
