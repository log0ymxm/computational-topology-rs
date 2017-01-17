type Point = (f64, f64);

fn determinant(m: ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64))) -> f64 {
    ((m.0).0 * (m.1).1 * (m.2).2) - ((m.0).0 * (m.1).2 * (m.2).1) -
    ((m.0).1 * (m.1).0 * (m.2).2) - ((m.0).1 * (m.1).2 * (m.2).0) -
    ((m.0).2 * (m.1).0 * (m.2).1) - ((m.0).2 * (m.1).1 * (m.2).0)
}

fn does_cross(x: Point, a: Point, b: Point) -> bool {
    if !(a.1 < x.1 && x.1 < b.1) {
        return false;
    }

    let delta = ((1., x.0, x.1), (1., a.0, a.1), (1., b.0, b.1));

    determinant(delta) > 0.
}

#[test]
fn does_cross_works() {
    assert!(!does_cross((0., 0.), (1., 0.), (0., 1.)));
    assert!(does_cross((0., 0.5), (1., 0.), (0., 1.)));
}
