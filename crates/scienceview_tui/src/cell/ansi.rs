//! ANSI colour mapping.
//!
//! A cell frame is plain data; this module turns a colour into the escape
//! sequence a terminal understands. Three depths are offered because a
//! terminal may support 24-bit colour, the 256-colour cube or only the sixteen
//! system colours, and a caller must be able to pick the widest one its
//! terminal accepts.

use scienceview_core::sv_scale::SVColour;
use std::fmt::Write;

/// How many colours the target terminal can display.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVAnsiColourDepth {
    /// Twenty-four bit colour (`38;2;r;g;b`).
    TrueColor,
    /// The xterm 256-colour palette (`38;5;n`).
    #[default]
    Ansi256,
    /// The sixteen system colours (`30`–`37` and `90`–`97`).
    Ansi16,
}

/// The xterm 256-colour palette index closest to `colour`.
///
/// The first sixteen entries are the system colours, the next 216 are a
/// 6 by 6 by 6 cube and the last twenty-four are a grey ramp.
pub fn sv_ansi_256_index(colour: SVColour) -> u8 {
    if colour.alpha() < 128 {
        return 0;
    }

    let cube = |channel: u8| -> u8 {
        let level = f64::from(channel) / 255.0 * 5.0;
        level.round().clamp(0.0, 5.0) as u8
    };
    let red = cube(colour.red());
    let green = cube(colour.green());
    let blue = cube(colour.blue());
    let cube_index = 16 + 36 * red + 6 * green + blue;

    let grey_level =
        (f64::from(colour.red()) + f64::from(colour.green()) + f64::from(colour.blue())) / 3.0;
    let grey_step = ((grey_level - 8.0) / 10.0).round().clamp(0.0, 23.0) as u8;
    let grey_index = 232 + grey_step;

    let cube_distance = sv_colour_distance(
        colour,
        sv_cube_colour(red),
        sv_cube_colour(green),
        sv_cube_colour(blue),
    );
    let grey_value = 8 + 10 * grey_step;
    let grey_distance = sv_colour_distance(colour, grey_value, grey_value, grey_value);

    if grey_distance < cube_distance {
        grey_index
    } else {
        cube_index
    }
}

/// The sixteen-colour index (0–15) closest to `colour`.
pub fn sv_ansi_16_index(colour: SVColour) -> u8 {
    const PALETTE: [(u8, u8, u8); 16] = [
        (0, 0, 0),
        (128, 0, 0),
        (0, 128, 0),
        (128, 128, 0),
        (0, 0, 128),
        (128, 0, 128),
        (0, 128, 128),
        (192, 192, 192),
        (128, 128, 128),
        (255, 0, 0),
        (0, 255, 0),
        (255, 255, 0),
        (0, 0, 255),
        (255, 0, 255),
        (0, 255, 255),
        (255, 255, 255),
    ];

    PALETTE
        .iter()
        .enumerate()
        .min_by_key(|(_, (red, green, blue))| {
            let distance = sv_colour_distance(colour, *red, *green, *blue);
            (distance * 1000.0) as u64
        })
        .map(|(index, _)| index as u8)
        .unwrap_or(7)
}

/// The SGR fragment for a foreground colour at `depth`, or `None` when the
/// colour is fully transparent.
pub fn sv_ansi_colour_code(colour: SVColour, depth: SVAnsiColourDepth) -> Option<String> {
    if colour.alpha() < 16 {
        return None;
    }

    let mut code = String::new();
    match depth {
        SVAnsiColourDepth::TrueColor => {
            let _ = write!(
                code,
                "38;2;{};{};{}",
                colour.red(),
                colour.green(),
                colour.blue()
            );
        }
        SVAnsiColourDepth::Ansi256 => {
            let _ = write!(code, "38;5;{}", sv_ansi_256_index(colour));
        }
        SVAnsiColourDepth::Ansi16 => {
            let index = sv_ansi_16_index(colour);
            let base = if index < 8 { 30 } else { 90 };
            let _ = write!(code, "{}", base + u16::from(index % 8));
        }
    }
    Some(code)
}

fn sv_cube_colour(level: u8) -> u8 {
    if level == 0 { 0 } else { 55 + 40 * level }
}

fn sv_colour_distance(colour: SVColour, red: u8, green: u8, blue: u8) -> f64 {
    let red = f64::from(colour.red()) - f64::from(red);
    let green = f64::from(colour.green()) - f64::from(green);
    let blue = f64::from(colour.blue()) - f64::from(blue);
    red * red + green * green + blue * blue
}
