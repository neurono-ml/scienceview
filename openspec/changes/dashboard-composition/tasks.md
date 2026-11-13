# Tasks

## Grid composition

- [x] Write the failing tests for grid layout and viewport change.
- [x] Implement the grid and cell layout.

## Shared sources and derived views

- [x] Write the failing parity test: a shared source is read without copying.
- [x] Implement shared sources and derived selection views.

## Linked events

- [x] Write the failing test: a selection filters the linked figures.
- [x] Implement the coordinated events and filters.

## Acceptance

- [x] Add the example, generated figure and book chapter (book branch).
- [x] Run the four gates and open the ADR issue.

## Review fixes

- [x] Route zoom, pan and range actions by linked axis identifier and reject an
      action whose local axis disagrees with the registration.
- [x] Bound the linked-selection cascade and suppress a reappearing filter.
- [x] Render one deterministic accessible SVG document with every panel placed
      by the layout solution, plus the opt-in `render-html` page.
- [x] Cover empty, minimal and many-panel dashboards and concurrent renders.
