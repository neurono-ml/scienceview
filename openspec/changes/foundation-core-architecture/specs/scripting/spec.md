# Spec Delta

## Purpose

Defines how scripts extend a figure, which script languages are supported, and
where a script runs, so that behaviour can be customised at runtime without
rebuilding the library or the user's program.

## ADDED Requirements

### Requirement: Script engines are pluggable

Scripting SHALL be provided through a public engine contract, so that a new
script language can be added without modifying the core.

#### Scenario: Third-party engine
- **WHEN** a user implements the public engine contract for a language
- **THEN** scripts written in that language can be registered on a figure

### Requirement: Lua and TypeScript are supported

The library SHALL provide a Lua engine and an engine able to run TypeScript and
JavaScript, selectable through the figure's configuration.

#### Scenario: Selecting an engine
- **WHEN** a user configures a figure to use the Lua engine and registers a script
- **THEN** the script is executed by the Lua engine

#### Scenario: TypeScript script
- **WHEN** a user configures a figure to use the TypeScript engine and registers a script
- **THEN** the script is executed after being prepared for the engine

### Requirement: Execution placement is configurable

A user SHALL be able to choose where a script executes: in the host runtime, or
embedded in the WebAssembly artifact produced for web backends.

#### Scenario: Host execution is the default
- **WHEN** a user enables scripting without choosing an execution placement
- **THEN** scripts execute in the host runtime

#### Scenario: Embedded execution in WebAssembly
- **WHEN** a user chooses embedded execution on a web backend that supports it
- **THEN** the script executes inside the WebAssembly artifact

### Requirement: Sandboxing is configurable

A script SHALL run with restricted capabilities by default, and the user SHALL
be able to widen the capabilities explicitly.

#### Scenario: Restricted by default
- **WHEN** a user registers a script without widening its capabilities
- **THEN** the script cannot perform unrestricted system access

#### Scenario: Widened capabilities
- **WHEN** a user explicitly widens the capabilities of a script
- **THEN** the script is granted exactly those capabilities

### Requirement: Scripts are editable at runtime

A registered script SHALL be replaceable at runtime, and the replacement SHALL
take effect for subsequent invocations without rebuilding the figure.

#### Scenario: Edit and reload
- **WHEN** a user replaces the source of a registered script and reloads it
- **THEN** the new source is used for subsequent invocations

### Requirement: Scripts can observe and act on figure state

A script SHALL be able to subscribe to the figure's events and to request
mutations of the figure.

#### Scenario: Script reacting to a range change
- **WHEN** a script subscribes to range-change events and requests a new range
- **THEN** the requested range is applied to the figure

### Requirement: Script failures are reported, not silent

A script that fails to load or fails during an invocation SHALL report the
failure with the script identity and the error, and SHALL NOT terminate the
host program.

#### Scenario: Failing script
- **WHEN** a registered script raises an error during an invocation
- **THEN** the failure is reported with the script identity and the host program continues
