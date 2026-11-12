//! Path commands serialized as one `d` attribute value.

use crate::serialize::sv_format_number;
use scienceview_core::sv_mark::{SVArcTo, SVGeometryPath, SVMarkPoint, SVPathCommand};

pub(crate) fn sv_serialize_path_data(path: &SVGeometryPath) -> String {
    let mut data = String::new();
    let mut current: Option<SVMarkPoint> = None;

    for command in path.commands() {
        match command {
            SVPathCommand::MoveTo(point) => {
                sv_push_command(&mut data, "M", *point);
                current = Some(*point);
            }
            SVPathCommand::LineTo(point) => {
                sv_push_command(&mut data, "L", *point);
                current = Some(*point);
            }
            SVPathCommand::ArcTo(arc) => {
                let start = sv_arc_start(arc);
                if current.is_none() {
                    sv_push_command(&mut data, "M", start);
                }
                sv_push_arc(&mut data, arc);
                current = Some(sv_arc_end(arc));
            }
            SVPathCommand::Close => data.push_str(" Z"),
        }
    }

    data.trim_start().to_string()
}

fn sv_push_command(data: &mut String, command: &str, point: SVMarkPoint) {
    data.push(' ');
    data.push_str(command);
    data.push(' ');
    data.push_str(&sv_format_number(point.x()));
    data.push(' ');
    data.push_str(&sv_format_number(point.y()));
}

fn sv_push_arc(data: &mut String, arc: &SVArcTo) {
    let end = sv_arc_end(arc);
    let sweep = arc.end_angle() >= arc.start_angle();
    let large_arc = (arc.end_angle() - arc.start_angle()).abs() > std::f64::consts::PI;
    let radius = sv_format_number(arc.radius());

    data.push_str(" A ");
    data.push_str(&radius);
    data.push(' ');
    data.push_str(&radius);
    data.push_str(" 0 ");
    data.push_str(if large_arc { "1" } else { "0" });
    data.push(' ');
    data.push_str(if sweep { "1" } else { "0" });
    data.push(' ');
    data.push_str(&sv_format_number(end.x()));
    data.push(' ');
    data.push_str(&sv_format_number(end.y()));
}

fn sv_arc_start(arc: &SVArcTo) -> SVMarkPoint {
    sv_point_at_angle(arc.centre(), arc.radius(), arc.start_angle())
}

fn sv_arc_end(arc: &SVArcTo) -> SVMarkPoint {
    sv_point_at_angle(arc.centre(), arc.radius(), arc.end_angle())
}

fn sv_point_at_angle(centre: SVMarkPoint, radius: f64, angle: f64) -> SVMarkPoint {
    SVMarkPoint::new(
        centre.x() + radius * angle.cos(),
        centre.y() + radius * angle.sin(),
    )
}
