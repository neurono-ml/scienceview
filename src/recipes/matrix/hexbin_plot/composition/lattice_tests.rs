//! The hexagonal lattice: axis inversion and gap-free coverage.

use super::lattice::{SV_HEXBIN_SQRT_THREE, SVHexbinDomain};

fn sv_domain() -> SVHexbinDomain {
    SVHexbinDomain::new(0.0, 10.0, 0.0, 10.0, 10).expect("the domain is well formed")
}

#[test]
fn every_cell_centre_rounds_back_to_its_own_cell() {
    let domain = sv_domain();
    for column in -20..=20 {
        for row in -20..=20 {
            let (x, y) = domain.cell_centre(column, row);
            assert_eq!(
                domain.cell_of(x, y),
                (column, row),
                "the axial rounding inverts the tiling at ({column}, {row})"
            );
        }
    }
}

#[test]
fn every_point_lands_inside_the_hexagon_of_its_cell() {
    let domain = sv_domain();
    let radius = domain.radius;
    let mut x = -2.0;
    while x <= 12.0 {
        let mut y = -2.0;
        while y <= 12.0 {
            let (column, row) = domain.cell_of(x, y);
            let (centre_x, centre_y) = domain.cell_centre(column, row);
            let distance = ((x - centre_x).powi(2) + (y - centre_y).powi(2)).sqrt();
            assert!(
                distance <= radius + 1.0e-9,
                "({x}, {y}) is {distance} from its centre, beyond the radius {radius}"
            );
            y += 0.137;
        }
        x += 0.137;
    }
}

#[test]
fn adjacent_cell_centres_keep_the_hexagonal_spacing() {
    let domain = sv_domain();
    let radius = domain.radius;
    let (centre_x, centre_y) = domain.cell_centre(0, 0);
    let (right_x, right_y) = domain.cell_centre(1, 0);
    let (upper_right_x, upper_right_y) = domain.cell_centre(1, 1);

    assert!((right_x - centre_x - SV_HEXBIN_SQRT_THREE * radius).abs() < 1.0e-9);
    assert!((right_y - centre_y).abs() < 1.0e-9);
    assert!((upper_right_y - centre_y - 1.5 * radius).abs() < 1.0e-9);
    assert!((upper_right_x - centre_x - 1.5 * SV_HEXBIN_SQRT_THREE * radius).abs() < 1.0e-9);
}
