# Design

## Context

The kit is the same code seen from two sides: the built-in controls and a
third-party widget are both adapters over the action vocabulary.

## Decisions

### D1 — Controls are models

They live in the observable graph, dispatch actions and subscribe to events.

### D2 — One vocabulary

A third-party widget dispatches the same actions as a built-in control.

### D3 — Playback binds to the timeline

Play, pause, step and scrub read and write the animation playback state.

### D4 — Themed, not hardcoded

Controls consume the same tokens as the marks.

## Risks / Trade-offs

- The action vocabulary can grow ad hoc; a new core action must be justified by reuse across the kit and the dashboard.

