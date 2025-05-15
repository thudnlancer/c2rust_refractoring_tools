use std::f64;

const FLOAT_RADIX: f64 = 2.0;
const FLOAT_RADIX_SQ: f64 = FLOAT_RADIX * FLOAT_RADIX;

/// Balance a general matrix by scaling the rows and columns, so the
/// new row and column norms are the same order of magnitude.
///
/// B =  D^-1 A D
///
/// where D is a diagonal matrix
///
/// This is necessary for the unsymmetric eigenvalue problem since the
/// calculation can become numerically unstable for unbalanced
/// matrices.
///
/// See Golub & Van Loan, "Matrix Computations" (3rd ed), Section 7.5.7
/// and Wilkinson & Reinsch, "Handbook for Automatic Computation", II/11 p320.
pub fn gsl_linalg_balance_matrix(
    a: &mut [Vec<f64>],
    d: &mut [f64],
) -> Result<(), &'static str> {
    let n = a.len();

    if n != d.len() {
        return Err("vector must match matrix size");
    }

    // initialize D to the identity matrix
    for val in d.iter_mut() {
        *val = 1.0;
    }

    let mut not_converged = true;

    while not_converged {
        not_converged = false;

        for i in 0..n {
            let mut row_norm = 0.0;
            let mut col_norm = 0.0;

            for j in 0..n {
                if j != i {
                    col_norm += a[j][i].abs();
                    row_norm += a[i][j].abs();
                }
            }

            if col_norm == 0.0 || row_norm == 0.0 {
                continue;
            }

            let mut g = row_norm / FLOAT_RADIX;
            let mut f = 1.0;
            let s = col_norm + row_norm;

            /*
             * find the integer power of the machine radix which
             * comes closest to balancing the matrix
             */
            while col_norm < g {
                f *= FLOAT_RADIX;
                col_norm *= FLOAT_RADIX_SQ;
            }

            g = row_norm * FLOAT_RADIX;

            while col_norm > g {
                f /= FLOAT_RADIX;
                col_norm /= FLOAT_RADIX_SQ;
            }

            if (row_norm + col_norm) < 0.95 * s * f {
                not_converged = true;

                g = 1.0 / f;

                /*
                 * apply similarity transformation D, where
                 * D_{ij} = f_i * delta_{ij}
                 */

                // multiply by D^{-1} on the left (scale row)
                for val in a[i].iter_mut() {
                    *val *= g;
                }

                // multiply by D on the right (scale column)
                for row in a.iter_mut() {
                    row[i] *= f;
                }

                // keep track of transformation
                d[i] *= f;
            }
        }
    }

    Ok(())
}

/// Accumulate a balancing transformation into a matrix.
/// This is used during the computation of Schur vectors since the
/// Schur vectors computed are the vectors for the balanced matrix.
/// We must at some point accumulate the balancing transformation into
/// the Schur vector matrix to get the vectors for the original matrix.
///
/// A -> D A
///
/// where D is the diagonal matrix
///
/// Inputs:
/// - a: matrix to transform
/// - d: vector containing diagonal elements of D
pub fn gsl_linalg_balance_accum(
    a: &mut [Vec<f64>],
    d: &[f64],
) -> Result<(), &'static str> {
    let n = a.len();

    if n != d.len() {
        return Err("vector must match matrix size");
    }

    for i in 0..n {
        let s = d[i];
        for val in a[i].iter_mut() {
            *val *= s;
        }
    }

    Ok(())
}