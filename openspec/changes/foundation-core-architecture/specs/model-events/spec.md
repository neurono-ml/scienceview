# Spec Delta

## Purpose

Defines how user interaction and state changes are represented, dispatched and
delivered, so that callbacks, the UI kit and third-party integrations all use
one coherent mechanism.

## ADDED Requirements

### Requirement: Interaction is expressed as a semantic action

Every user interaction that the library interprets SHALL be expressed as a
semantic action from a defined vocabulary, independent of the backend that
captured the input.

#### Scenario: Same action from different backends
- **WHEN** a user zooms in from one backend and zooms in from another backend
- **THEN** both interactions produce the same semantic zoom action

#### Scenario: Custom action payload
- **WHEN** a user dispatches an action that carries a custom payload
- **THEN** the payload is preserved and delivered to subscribers unchanged

### Requirement: Actions are applied as transactional mutations

An action SHALL be applied to the model as a batch of mutations that is
committed atomically, so that no observer sees a partially applied action.

#### Scenario: Failing mutation
- **WHEN** one mutation of an action fails
- **THEN** none of the mutations of that action remain applied

#### Scenario: Single commit
- **WHEN** an action is applied
- **THEN** subscribers are notified once, after the whole batch is applied

### Requirement: Mutations are serializable deltas

Every committed batch of mutations SHALL be expressible as a serializable
delta describing the changed properties and their new values.

#### Scenario: Delta round trip
- **WHEN** a batch of mutations is serialized and deserialized
- **THEN** the deserialized delta describes the same changes

### Requirement: Callbacks receive a handle, not direct ownership

A callback SHALL receive a figure handle through which it requests mutations,
and SHALL NOT receive mutable ownership of the figure's internal state.

#### Scenario: Mutation from a callback
- **WHEN** a callback requests a mutation through the handle
- **THEN** the mutation is queued and applied when the callback returns

### Requirement: Changes emit events

Every committed batch of mutations SHALL emit an event describing what changed,
delivered to the subscribers of the affected model objects.

#### Scenario: Subscriber notification
- **WHEN** a mutation changes an axis range
- **THEN** subscribers receive an event describing the new range

### Requirement: Cascading changes are bounded

A mutation caused by a callback SHALL NOT re-trigger the same callback, and
the depth of cascading changes SHALL be bounded.

#### Scenario: Self-triggering callback
- **WHEN** a callback mutates a property that it itself observes
- **THEN** the callback is not invoked again as a result of its own mutation

#### Scenario: Excessive cascade
- **WHEN** cascading changes exceed the configured depth budget
- **THEN** the cascade is stopped and an error describing the cycle is reported

### Requirement: Callbacks may be written in Rust or in a script

A figure SHALL accept callbacks written in Rust and callbacks written in an
external script language, and SHALL deliver the same event payloads to both.

#### Scenario: Script callback receives the same payload
- **WHEN** a figure has both a Rust callback and a script callback registered for the same event
- **THEN** both receive the same event payload

### Requirement: Scripts can be edited and reloaded at runtime

A registered script SHALL be replaceable at runtime without rebuilding the
figure, after which the new version receives subsequent events.

#### Scenario: Reload after edit
- **WHEN** a user replaces the source of a registered script and reloads it
- **THEN** subsequent events are delivered to the new version of the script

### Requirement: Subscribers can be removed

A registered callback or subscriber SHALL be removable, after which it receives
no further events.

#### Scenario: Unsubscription
- **WHEN** a user removes a registered callback
- **THEN** that callback receives no further events
