use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

pub fn solve_cubic(a: f64, b: f64, c: f64) -> (i32, [Complex; 3]) {
    let mut roots = [
        Complex { real: 0.0, imag: 0.0 },
        Complex { real: 0.0, imag: 0.0 },
        Complex { real: 0.0, imag: 0.0 },
    ];

    let q = a * a - 3.0 * b;
    let r = 2.0 * a * a * a - 9.0 * a * b + 27.0 * c;
    let q_div_9 = q / 9.0;
    let r_div_54 = r / 54.0;
    let q3 = q_div_9 * q_div_9 * q_div_9;
    let r2 = r_div_54 * r_div_54;
    let cr2 = 729.0 * r * r;
    let cq3 = 2916.0 * q * q * q;

    if r == 0.0 && q == 0.0 {
        let root = -a / 3.0;
        roots[0] = Complex { real: root, imag: 0.0 };
        roots[1] = Complex { real: root, imag: 0.0 };
        roots[2] = Complex { real: root, imag: 0.0 };
        (3, roots)
    } else if cr2 == cq3 {
        let sqrt_q = q_div_9.sqrt();
        if r_div_54 > 0.0 {
            roots[0] = Complex {
                real: -2.0 * sqrt_q - a / 3.0,
                imag: 0.0,
            };
            roots[1] = Complex {
                real: sqrt_q - a / 3.0,
                imag: 0.0,
            };
            roots[2] = Complex {
                real: sqrt_q - a / 3.0,
                imag: 0.0,
            };
        } else {
            roots[0] = Complex {
                real: -sqrt_q - a / 3.0,
                imag: 0.0,
            };
            roots[1] = Complex {
                real: -sqrt_q - a / 3.0,
                imag: 0.0,
            };
            roots[2] = Complex {
                real: 2.0 * sqrt_q - a / 3.0,
                imag: 0.0,
            };
        }
        (3, roots)
    } else if r2 < q3 {
        let sgn_r = if r_div_54 >= 0.0 { 1.0 } else { -1.0 };
        let ratio = sgn_r * (r2 / q3).sqrt();
        let theta = ratio.acos();
        let norm = -2.0 * q_div_9.sqrt();

        let mut r0 = norm * (theta / 3.0).cos() - a / 3.0;
        let mut r1 = norm * ((theta + 2.0 * PI) / 3.0).cos() - a / 3.0;
        let mut r2 = norm * ((theta - 2.0 * PI) / 3.0).cos() - a / 3.0;

        if r0 > r1 {
            std::mem::swap(&mut r0, &mut r1);
        }
        if r1 > r2 {
            std::mem::swap(&mut r1, &mut r2);
            if r0 > r1 {
                std::mem::swap(&mut r0, &mut r1);
            }
        }

        roots[0] = Complex { real: r0, imag: 0.0 };
        roots[1] = Complex { real: r1, imag: 0.0 };
        roots[2] = Complex { real: r2, imag: 0.0 };
        (3, roots)
    } else {
        let sgn_r = if r_div_54 >= 0.0 { 1.0 } else { -1.0 };
        let a_val = -sgn_r * (r_div_54.abs() + (r2 - q3).sqrt()).powf(1.0 / 3.0);
        let b_val = q_div_9 / a_val;

        if a_val + b_val < 0.0 {
            roots[0] = Complex {
                real: a_val + b_val - a / 3.0,
                imag: 0.0,
            };
            roots[1] = Complex {
                real: -0.5 * (a_val + b_val) - a / 3.0,
                imag: -(3.0f64.sqrt() / 2.0) * (a_val - b_val).abs(),
            };
            roots[2] = Complex {
                real: -0.5 * (a_val + b_val) - a / 3.0,
                imag: (3.0f64.sqrt() / 2.0) * (a_val - b_val).abs(),
            };
        } else {
            roots[0] = Complex {
                real: -0.5 * (a_val + b_val) - a / 3.0,
                imag: -(3.0f64.sqrt() / 2.0) * (a_val - b_val).abs(),
            };
            roots[1] = Complex {
                real: -0.5 * (a_val + b_val) - a / 3.0,
                imag: (3.0f64.sqrt() / 2.0) * (a_val - b_val).abs(),
            };
            roots[2] = Complex {
                real: a_val + b_val - a / 3.0,
                imag: 0.0,
            };
        }
        (3, roots)
    }
}