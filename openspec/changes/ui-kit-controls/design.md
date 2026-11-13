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

### D5 — Scripts are payload consumers

A control carries a script source and every subscriber receives the same
`SVControlEvent` payloads. Executing the source belongs to the scripting engine
(`scripting-engines`, wave 8); this change delivers the contract, not an
interpreter. The alternative — embedding a script runtime in the core — would
break the dependency-light core and duplicate the scripting capability.

## Risks / Trade-offs

- The action vocabulary can grow ad hoc; a new core action must be justified by reuse across the kit and the dashboard.

