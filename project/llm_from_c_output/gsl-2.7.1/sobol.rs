/* Author: G. Jungman
 */
/* Implementation for Sobol generator.
 * See
 *   [Bratley+Fox, TOMS 14, 88 (1988)]
 *   [Antonov+Saleev, USSR Comput. Maths. Math. Phys. 19, 252 (1980)]
 */

/* maximum allowed space dimension */
const SOBOL_MAX_DIMENSION: usize = 40;

/* bit count; assumes sizeof(int) >= 32-bit */
const SOBOL_BIT_COUNT: usize = 30;

#[derive(Debug)]
pub enum SobolError {
    InvalidDimension,
    Exhausted,
}

/* primitive polynomials in binary encoding */
const PRIMITIVE_POLYNOMIALS: [i32; SOBOL_MAX_DIMENSION] = [
    1, 3, 7, 11, 13, 19, 25, 37, 59, 47,
    61, 55, 41, 67, 97, 91, 109, 103, 115, 131,
    193, 137, 145, 143, 241, 157, 185, 167, 229, 171,
    213, 191, 253, 203, 211, 239, 247, 285, 369, 299,
];

/* degrees of the primitive polynomials */
const DEGREE_TABLE: [usize; SOBOL_MAX_DIMENSION] = [
    0, 1, 2, 3, 3, 4, 4, 5, 5, 5,
    5, 5, 5, 6, 6, 6, 6, 6, 6, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 8, 8, 8,
];

/* initial values for direction tables */
const V_INIT: [[i32; SOBOL_MAX_DIMENSION]; 8] = [
    [
        0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        0, 0, 1, 3, 1, 3, 1, 3, 3, 1,
        3, 1, 3, 1, 3, 1, 1, 3, 1, 3,
        1, 3, 1, 3, 3, 1, 3, 1, 3, 1,
        3, 1, 1, 3, 1, 3, 1, 3, 1, 3,
    ],
    [
        0, 0, 0, 7, 5, 1, 3, 3, 7, 5,
        5, 7, 7, 1, 3, 3, 7, 5, 1, 1,
        5, 3, 3, 1, 7, 5, 1, 3, 3, 7,
        5, 1, 1, 5, 7, 7, 5, 1, 3, 3,
    ],
    [
        0, 0, 0, 0, 0, 1, 7, 9, 13, 11,
        1, 3, 7, 9, 5, 13, 13, 11, 3, 15,
        5, 3, 15, 7, 9, 13, 9, 1, 11, 7,
        5, 15, 1, 15, 11, 5, 3, 1, 7, 9,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 9, 3, 27,
        15, 29, 21, 23, 19, 11, 25, 7, 13, 17,
        1, 25, 29, 3, 31, 11, 5, 23, 27, 19,
        21, 5, 1, 17, 13, 7, 15, 9, 31, 9,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 37, 33, 7, 5, 11, 39, 63,
        27, 17, 15, 23, 29, 3, 21, 13, 31, 25,
        9, 49, 33, 19, 29, 11, 19, 27, 15, 25,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 13,
        33, 115, 41, 79, 17, 29, 119, 75, 73, 105,
        7, 59, 65, 21, 3, 113, 61, 89, 45, 107,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 7, 23, 39,
    ],
];

#[derive(Debug)]
pub struct SobolState {
    sequence_count: u32,
    last_denominator_inv: f64,
    last_numerator_vec: [i32; SOBOL_MAX_DIMENSION],
    v_direction: [[i32; SOBOL_MAX_DIMENSION]; SOBOL_BIT_COUNT],
}

impl SobolState {
    pub fn new(dimension: usize) -> Result<Self, SobolError> {
        if dimension < 1 || dimension > SOBOL_MAX_DIMENSION {
            return Err(SobolError::InvalidDimension);
        }

        let mut state = SobolState {
            sequence_count: 0,
            last_denominator_inv: 0.0,
            last_numerator_vec: [0; SOBOL_MAX_DIMENSION],
            v_direction: [[0; SOBOL_MAX_DIMENSION]; SOBOL_BIT_COUNT],
        };

        // Initialize direction table in dimension 0
        for k in 0..SOBOL_BIT_COUNT {
            state.v_direction[k][0] = 1;
        }

        // Initialize in remaining dimensions
        for i_dim in 1..dimension {
            let poly_index = i_dim;
            let degree_i = DEGREE_TABLE[poly_index];
            let mut includ = [false; 8];

            // Expand the polynomial bit pattern
            let mut p_i = PRIMITIVE_POLYNOMIALS[poly_index];
            for k in (0..degree_i).rev() {
                includ[k] = (p_i % 2) == 1;
                p_i /= 2;
            }

            // Leading elements from v_init
            for j in 0..degree_i {
                state.v_direction[j][i_dim] = V_INIT[j][i_dim];
            }

            // Calculate remaining elements
            for j in degree_i..SOBOL_BIT_COUNT {
                let mut newv = state.v_direction[j - degree_i][i_dim];
                let mut ell = 1;
                for k in 0..degree_i {
                    ell *= 2;
                    if includ[k] {
                        newv ^= ell * state.v_direction[j - k - 1][i_dim];
                    }
                }
                state.v_direction[j][i_dim] = newv;
            }
        }

        // Multiply columns by appropriate power of 2
        let mut ell = 1;
        for j in (0..SOBOL_BIT_COUNT - 1).rev() {
            ell *= 2;
            for i_dim in 0..dimension {
                state.v_direction[j][i_dim] *= ell;
            }
        }

        state.last_denominator_inv = 1.0 / (2.0 * ell as f64);
        state.last_numerator_vec = [0; SOBOL_MAX_DIMENSION];

        Ok(state)
    }

    pub fn next(&mut self, dimension: usize) -> Result<Vec<f64>, SobolError> {
        let mut v = vec![0.0; dimension];

        // Find position of least-significant zero in count
        let mut ell = 0;
        let mut c = self.sequence_count;
        loop {
            ell += 1;
            if (c % 2) == 1 {
                c /= 2;
            } else {
                break;
            }
        }

        if ell > SOBOL_BIT_COUNT as u32 {
            return Err(SobolError::Exhausted);
        }

        for i_dimension in 0..dimension {
            let direction_i = self.v_direction[(ell - 1) as usize][i_dimension];
            let old_numerator_i = self.last_numerator_vec[i_dimension];
            let new_numerator_i = old_numerator_i ^ direction_i;
            self.last_numerator_vec[i_dimension] = new_numerator_i;
            v[i_dimension] = new_numerator_i as f64 * self.last_denominator_inv;
        }

        self.sequence_count += 1;
        Ok(v)
    }
}