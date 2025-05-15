const RADIX: f64 = 2.0;
const RADIX2: f64 = RADIX * RADIX;

fn balance_companion_matrix(m: &mut [f64], nc: usize) {
    let mut not_converged = true;

    while not_converged {
        not_converged = false;

        for i in 0..nc {
            // column norm, excluding the diagonal
            let col_norm = if i != nc - 1 {
                m[(i + 1) * nc + i].abs()
            } else {
                (0..nc - 1).map(|j| m[j * nc + (nc - 1)].abs()).sum()
            };

            // row norm, excluding the diagonal
            let row_norm = if i == 0 {
                m[0 * nc + (nc - 1)].abs()
            } else if i == nc - 1 {
                m[i * nc + (i - 1)].abs()
            } else {
                m[i * nc + (i - 1)].abs() + m[i * nc + (nc - 1)].abs()
            };

            if col_norm == 0.0 || row_norm == 0.0 {
                continue;
            }

            let mut g = row_norm / RADIX;
            let mut f = 1.0;
            let s = col_norm + row_norm;

            while col_norm < g {
                f *= RADIX;
                g *= RADIX2;
            }

            g = row_norm * RADIX;

            while col_norm > g {
                f /= RADIX;
                g /= RADIX2;
            }

            if (row_norm + col_norm) < 0.95 * s * f {
                not_converged = true;
                g = 1.0 / f;

                if i == 0 {
                    m[0 * nc + (nc - 1)] *= g;
                } else {
                    m[i * nc + (i - 1)] *= g;
                    m[i * nc + (nc - 1)] *= g;
                }

                if i == nc - 1 {
                    for j in 0..nc {
                        m[j * nc + i] *= f;
                    }
                } else {
                    m[(i + 1) * nc + i] *= f;
                }
            }
        }
    }
}