#[derive(Copy, Clone)]
pub struct GslComplex {
    pub dat: [f64; 2],
}

pub fn gsl_poly_complex_solve_quadratic(
    a: f64,
    b: f64,
    c: f64,
) -> (i32, Option<GslComplex>, Option<GslComplex>) {
    let disc = b * b - 4.0 * a * c;

    if a == 0.0 {
        if b == 0.0 {
            return (0, None, None);
        } else {
            let z0 = GslComplex {
                dat: [-c / b, 0.0],
            };
            return (1, Some(z0), None);
        }
    }

    if disc > 0.0 {
        if b == 0.0 {
            let s = (0.5 * disc.sqrt() / a).abs();
            let z0 = GslComplex { dat: [-s, 0.0] };
            let z1 = GslComplex { dat: [s, 0.0] };
            return (2, Some(z0), Some(z1));
        } else {
            let sgnb = if b > 0.0 { 1.0 } else { -1.0 };
            let temp = -0.5 * (b + sgnb * disc.sqrt());
            let r1 = temp / a;
            let r2 = c / temp;

            let (z0, z1) = if r1 < r2 {
                (
                    GslComplex { dat: [r1, 0.0] },
                    GslComplex { dat: [r2, 0.0] },
                )
            } else {
                (
                    GslComplex { dat: [r2, 0.0] },
                    GslComplex { dat: [r1, 0.0] },
                )
            };
            return (2, Some(z0), Some(z1));
        }
    } else if disc == 0.0 {
        let root = GslComplex {
            dat: [-0.5 * b / a, 0.0],
        };
        return (2, Some(root), Some(root));
    } else {
        let s = (0.5 * (-disc).sqrt() / a).abs();
        let real_part = -0.5 * b / a;
        let z0 = GslComplex {
            dat: [real_part, -s],
        };
        let z1 = GslComplex {
            dat: [real_part, s],
        };
        return (2, Some(z0), Some(z1));
    }
}