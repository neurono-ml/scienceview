# Proposal

## Why

A figure on the web must live in a page, not only in a file: it must
update in place, dispatch events and host the controls. The HTML backend is the
first document target with a clock, and it is the host for the framework
components that follow.

## What Changes

- Implement `scienceview_html`, a self-contained HTML document backend with no external dependencies.
- Emit the compiled theme stylesheet alongside the figure so a user can override individual rules.
- Support in-place update and events through a small embedded runtime.
- Declare the capability matrix for an interactive document backend.

## Capabilities

### New Capabilities

- `render-html-backend`: render html backend.

### Modified Capabilities

None.

## Impact

- New crate `crates/scienceview_html`.
- Host for the React, Vue and Angular component outputs.
- Depends on the SVG rendering contract and on the animation clock.
