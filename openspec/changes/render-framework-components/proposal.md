# Proposal

## Why

Teams embed charts in React, Vue and Angular applications. Shipping a
component per framework is what makes the library usable without asking the
application to adopt a rendering paradigm.

## What Changes

- Implement component output for React, Vue and Angular on top of the HTML backend.
- Expose the figure as a component with properties for data, theme and backend kind, and events for the action vocabulary.
- Keep one behaviour across the three frameworks.
- Declare the capability matrix for a component backend.

## Capabilities

### New Capabilities

- `render-framework-components`: render framework components.

### Modified Capabilities

None.

## Impact

- New crates `scienceview_react`, `scienceview_vue`, `scienceview_angular`.
- Depends on the HTML backend.
- One capability, three crates: the frameworks differ in packaging, not behaviour.
