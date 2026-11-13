# Spec Delta

## Purpose

`SVClusteredHeatmapPlot` renders a heatmap whose rows and columns follow a
supplied hierarchical clustering, with the two supplied dendrograms drawn
alongside. The recipe validates and draws the clustering; it never computes
one.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVClusteredHeatmapPlot` SHALL be produced by exactly the composition
`data -> [SVStat::Aggregate (matrix)] -> matrix + supplied orders and trees ->
[validate the supplied clustering] -> [SVScale::Continuous over a colour map]
-> SVMark::Image (cells) + supplied dendrogram paths`, and SHALL expose the
configuration surface recorded in its FDR.

#### Scenario: Supplied-clustering composition
- **WHEN** the chart is built from a matrix, a row order, a column order and
  the two merge trees
- **THEN** it draws the reordered cells and the two dendrograms without
  running a clustering algorithm

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart renders the supplied order

`SVClusteredHeatmapPlot` SHALL draw slot `i` of each axis from the supplied
original index `order[i]`, and SHALL preserve each cell's original row and
column identity while it moves to its slot.

#### Scenario: Supplied order is honoured
- **WHEN** the row order is `[2, 0, 1]`
- **THEN** the first drawn row is the original row 2 and the row labels follow
  the same order

#### Scenario: Cell identity survives the reorder
- **WHEN** a cell moves to a new slot
- **THEN** it still reports its original row and column and its original value

### Requirement: The chart validates the supplied clustering trees

`SVClusteredHeatmapPlot` SHALL validate each supplied tree against its axis
dimension and SHALL report an inconsistent tree as a typed error naming the
axis and the merge position.

#### Scenario: Duplicate leaf
- **WHEN** a leaf appears in more than one merge
- **THEN** the build fails with `InvalidClusterTree` naming the axis and
  position

#### Scenario: Out-of-range child
- **WHEN** a merge refers to an index that is not an earlier leaf or merge
- **THEN** the build fails with `InvalidClusterTree` naming the axis and
  position

#### Scenario: Decreasing merge distance
- **WHEN** a merge distance is shorter than one of its children
- **THEN** the build fails with `InvalidClusterTree` naming the axis and
  position

#### Scenario: Wrong leaf count
- **WHEN** a merge declares a leaf count inconsistent with its children
- **THEN** the build fails with `InvalidClusterTree` naming the axis and
  position

#### Scenario: Order is not a permutation
- **WHEN** an order repeats or omits an axis index
- **THEN** the build fails with `InvalidClusterTree` naming the axis and
  position

#### Scenario: Order length mismatch
- **WHEN** the order length differs from the matrix dimension
- **THEN** the build fails with `MismatchedLengths`

#### Scenario: Dendrogram without both trees
- **WHEN** the dendrogram is requested but one tree is absent
- **THEN** the build fails with an explicit configuration error naming the
  missing input

### Requirement: The chart reproduces the supplied merge distances

`SVClusteredHeatmapPlot` SHALL draw each dendrogram at the supplied merge
distance, with the zero distance at the cell edge and the largest supplied
distance at the outer edge of the dendrogram region.

#### Scenario: Row distances are reproduced
- **WHEN** the row tree supplies distances `[1.0, 2.0]`
- **THEN** the resolved row dendrogram reports the same distances in order

#### Scenario: Column distances are reproduced
- **WHEN** the column tree supplies distances `[0.5, 1.5]`
- **THEN** the resolved column dendrogram reports the same distances in order

### Requirement: The chart consumes its documented data contract

`SVClusteredHeatmapPlot` SHALL consume a dense `SVHeatmapMatrix` or a long
table with a `row`, `column` and `value` column pivoted through
`SVStat::Aggregate`, and SHALL reject a configuration whose columns are missing
or of the wrong kind.

#### Scenario: Dense matrix and long table agree
- **WHEN** the same data is supplied as a dense matrix and as a long table
- **THEN** both draw the same cells and serialize the same figure

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration or column error

### Requirement: Sparse input is read in place

`SVClusteredHeatmapPlot` SHALL accept a sparse value column, SHALL dispatch on
the representation once per operation, and SHALL never densify a sparse input.

#### Scenario: Sparse stored entries only
- **WHEN** a sparse value column stores three of nine cells
- **THEN** the scene reports the Sparse representation and draws exactly the
  three stored cells

### Requirement: Streaming input is a static-only exception

`SVClusteredHeatmapPlot` SHALL reject a streaming source up front with
`UnsupportedRepresentation` and SHALL name the documented alternative: take a
snapshot of the window, compute the clustering outside `scienceview`, then
render that snapshot.

#### Scenario: Streaming rejected with the alternative
- **WHEN** a streaming source is supplied
- **THEN** the build fails with `UnsupportedRepresentation` naming
  `SVClusteredHeatmapPlot` and the snapshot alternative

### Requirement: The chart renders on every declared backend

`SVClusteredHeatmapPlot` SHALL render the scene as SVG and as a native frame,
and SHALL return a typed error for a backend kind it does not serialize yet.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering path
- **WHEN** the chart is rendered natively
- **THEN** the configured native path rasterizes the same scene

#### Scenario: Unsupported backend
- **WHEN** the configured backend has no recipe serializer
- **THEN** the operation fails explicitly rather than drawing nothing

### Requirement: The chart records its operation span

Every public operation SHALL record one span carrying the configured backend
kind, the resolved representation and the number of drawn cells.

#### Scenario: Compose span fields
- **WHEN** the chart composes successfully
- **THEN** the span records the backend kind, the representation and the drawn
  cell count
