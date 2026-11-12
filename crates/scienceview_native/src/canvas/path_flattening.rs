//! Flattening a backend-agnostic path into device-space polylines.
//!
//! The primitive vocabulary is small — move, line, circular arc and close — so
//! flattening is a short, deterministic walk. An arc becomes a fixed number of
//! segments derived from its radius and sweep, never from the clock, so the
//! same path always produces the same pixels.

use scienceview_core::sv_mark::SVPathCommand;

/// A path flattened into device-space polylines.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SVFlattenedPath {
    subpaths: Vec<Vec<(f64, f64)>>,
    closed: Vec<bool>,
}

impl SVFlattenedPath {
    /// Builds a path from its subpaths and their closed flags.
    fn new(subpaths: Vec<Vec<(f64, f64)>>, closed: Vec<bool>) -> Self {
        Self { subpaths, closed }
    }

    /// Returns every subpath, ignoring empty ones.
    pub(crate) fn subpaths(&self) -> &[Vec<(f64, f64)>] {
        &self.subpaths
    }

    /// Returns whether the subpath at `position` was closed.
    pub(crate) fn is_closed(&self, position: usize) -> bool {
        self.closed.get(position).copied().unwrap_or(false)
    }

    /// Returns the non-zero winding number at a device-space point.
    ///
    /// Every subpath is implicitly closed for filling, exactly as a filled path
    /// closes an open contour.
    pub(crate) fn winding_number(&self, point_x: f64, point_y: f64) -> i32 {
        let mut winding = 0;
        for subpath in &self.subpaths {
            if subpath.len() < 2 {
                continue;
            }
            for position in 0..subpath.len() {
                let start = subpath[position];
                let end = subpath[(position + 1) % subpath.len()];
                if (start.1 <= point_y) == (end.1 <= point_y) {
                    continue;
                }
                let fraction = (point_y - start.1) / (end.1 - start.1);
                let crossing_x = start.0 + fraction * (end.0 - start.0);
                if crossing_x > point_x {
                    winding += if end.1 > start.1 { 1 } else { -1 };
                }
            }
        }
        winding
    }
}

/// Flattens a closed polygon into a single device-space subpath.
pub(crate) fn sv_flattened_polygon(points: &[(f64, f64)]) -> SVFlattenedPath {
    SVFlattenedPath::new(vec![points.to_vec()], vec![true])
}

/// Flattens the commands of a path, scaling every coordinate by `scale`.
pub(crate) fn sv_flatten_path(commands: &[SVPathCommand], scale: f64) -> SVFlattenedPath {
    let mut subpaths: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut closed: Vec<bool> = Vec::new();
    let mut current: Vec<(f64, f64)> = Vec::new();

    let flush = |subpaths: &mut Vec<Vec<(f64, f64)>>,
                 closed: &mut Vec<bool>,
                 current: &mut Vec<(f64, f64)>,
                 is_closed: bool| {
        if !current.is_empty() {
            subpaths.push(std::mem::take(current));
            closed.push(is_closed);
        }
    };

    for command in commands {
        match *command {
            SVPathCommand::MoveTo(point) => {
                flush(&mut subpaths, &mut closed, &mut current, false);
                current.push((point.x() * scale, point.y() * scale));
            }
            SVPathCommand::LineTo(point) => {
                current.push((point.x() * scale, point.y() * scale));
            }
            SVPathCommand::ArcTo(arc) => {
                let sweep = arc.end_angle() - arc.start_angle();
                let radius = arc.radius() * scale;
                let steps = sv_arc_step_count(radius, sweep);
                current.push((
                    arc.centre().x() * scale + radius * arc.start_angle().cos(),
                    arc.centre().y() * scale + radius * arc.start_angle().sin(),
                ));
                for step in 1..=steps {
                    let angle = arc.start_angle() + sweep * step as f64 / steps as f64;
                    current.push((
                        arc.centre().x() * scale + radius * angle.cos(),
                        arc.centre().y() * scale + radius * angle.sin(),
                    ));
                }
            }
            SVPathCommand::Close => {
                flush(&mut subpaths, &mut closed, &mut current, true);
            }
        }
    }

    flush(&mut subpaths, &mut closed, &mut current, false);
    SVFlattenedPath::new(subpaths, closed)
}

fn sv_arc_step_count(radius: f64, sweep: f64) -> usize {
    if !radius.is_finite() || !sweep.is_finite() || radius <= 0.0 || sweep == 0.0 {
        return 1;
    }
    let length = radius * sweep.abs();
    (length / 2.0).ceil().clamp(1.0, 512.0) as usize
}

/// Splits a polyline into the spans a dash pattern leaves visible.
///
/// An empty pattern returns the whole polyline as one span. A closed polyline
/// gets its closing edge walked, so a dashed loop is dashed all the way round.
pub(crate) fn sv_dashed_spans(
    points: &[(f64, f64)],
    closed: bool,
    segments: &[f64],
) -> Vec<Vec<(f64, f64)>> {
    if points.is_empty() {
        return Vec::new();
    }
    if segments.len() < 2 || segments.len() % 2 != 0 {
        return vec![points.to_vec()];
    }

    let mut edges: Vec<((f64, f64), (f64, f64))> = Vec::new();
    for position in 0..points.len().saturating_sub(1) {
        edges.push((points[position], points[position + 1]));
    }
    if closed && points.len() > 1 {
        edges.push((points[points.len() - 1], points[0]));
    }

    let mut spans: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut current: Vec<(f64, f64)> = Vec::new();
    let mut pattern_index = 0;
    let mut pattern_remaining = segments[0];
    let mut pattern_on = true;

    for (start, end) in edges {
        let edge_length = ((end.0 - start.0).powi(2) + (end.1 - start.1).powi(2)).sqrt();
        if edge_length <= f64::EPSILON {
            continue;
        }
        let mut consumed = 0.0;
        while consumed < edge_length - f64::EPSILON {
            let available = edge_length - consumed;
            let take = pattern_remaining.min(available);
            let next_consumed = consumed + take;
            let fraction = next_consumed / edge_length;
            let point = (
                start.0 + (end.0 - start.0) * fraction,
                start.1 + (end.1 - start.1) * fraction,
            );
            if pattern_on {
                if current.is_empty() {
                    let start_fraction = consumed / edge_length;
                    current.push((
                        start.0 + (end.0 - start.0) * start_fraction,
                        start.1 + (end.1 - start.1) * start_fraction,
                    ));
                }
                current.push(point);
            }
            consumed = next_consumed;
            pattern_remaining -= take;
            if pattern_remaining <= f64::EPSILON {
                if pattern_on && !current.is_empty() {
                    spans.push(std::mem::take(&mut current));
                }
                pattern_index = (pattern_index + 1) % segments.len();
                pattern_remaining = segments[pattern_index];
                pattern_on = !pattern_on;
            }
        }
    }

    if !current.is_empty() {
        spans.push(current);
    }
    spans
}
