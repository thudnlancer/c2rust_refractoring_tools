/* Author: G. Jungman
 */
/* Implement Niederreiter base 2 generator.
 * See:
 *   Bratley, Fox, Niederreiter, ACM Trans. Model. Comp. Sim. 2, 195 (1992)
 */

const NIED2_CHARACTERISTIC: usize = 2;
const NIED2_BASE: usize = 2;

const NIED2_MAX_DIMENSION: usize = 12;
const NIED2_MAX_PRIM_DEGREE: usize = 5;
const NIED2_MAX_DEGREE: usize = 50;

const NIED2_BIT_COUNT: usize = 30;
const NIED2_NBITS: usize = NIED2_BIT_COUNT + 1;

const MAXV: usize = NIED2_NBITS + NIED2_MAX_DEGREE;

/* Z_2 field operations */
fn nied2_add(x: usize, y: usize) -> usize { (x + y) % 2 }
fn nied2_mul(x: usize, y: usize) -> usize { (x * y) % 2 }
fn nied2_sub(x: usize, y: usize) -> usize { nied2_add(x, y) }

#[derive(Debug)]
pub struct Niederreiter2 {
    sequence_count: u32,
    cj: [[i32; NIED2_MAX_DIMENSION]; NIED2_NBITS],
    nextq: [i32; NIED2_MAX_DIMENSION],
}

/* primitive polynomials in binary encoding */
const PRIMITIVE_POLY: [[i32; NIED2_MAX_PRIM_DEGREE + 1]; NIED2_MAX_DIMENSION + 1] = [
    [1, 0, 0, 0, 0, 0],  /*  1               */
    [0, 1, 0, 0, 0, 0],  /*  x               */
    [1, 1, 0, 0, 0, 0],  /*  1 + x           */
    [1, 1, 1, 0, 0, 0],  /*  1 + x + x^2     */
    [1, 1, 0, 1, 0, 0],  /*  1 + x + x^3     */
    [1, 0, 1, 1, 0, 0],  /*  1 + x^2 + x^3   */
    [1, 1, 0, 0, 1, 0],  /*  1 + x + x^4     */
    [1, 0, 0, 1, 1, 0],  /*  1 + x^3 + x^4   */
    [1, 1, 1, 1, 1, 0],  /*  1 + x + x^2 + x^3 + x^4   */
    [1, 0, 1, 0, 0, 1],  /*  1 + x^2 + x^5             */
    [1, 0, 0, 1, 0, 1],  /*  1 + x^3 + x^5             */
    [1, 1, 1, 1, 0, 1],  /*  1 + x + x^2 + x^3 + x^5   */
    [1, 1, 1, 0, 1, 1],  /*  1 + x + x^2 + x^4 + x^5   */
];

/* degrees of primitive polynomials */
const POLY_DEGREE: [usize; NIED2_MAX_DIMENSION + 1] = [
    0, 1, 1, 2, 3, 3, 4, 4, 4, 5, 5, 5, 5,
];

/* Multiply polynomials over Z_2.
 * Notice use of a temporary vector,
 * side-stepping aliasing issues when
 * one of inputs is the same as the output
 */
fn poly_multiply(
    pa: &[i32],
    pa_degree: usize,
    pb: &[i32],
    pb_degree: usize,
    pc: &mut [i32],
    pc_degree: &mut usize,
) {
    let mut pt = [0; NIED2_MAX_DEGREE + 1];
    let pt_degree = pa_degree + pb_degree;

    for k in 0..=pt_degree {
        let mut term = 0;
        for j in 0..=k {
            let conv_term = nied2_mul(pa[k - j] as usize, pb[j] as usize);
            term = nied2_add(term, conv_term);
        }
        pt[k] = term as i32;
    }

    for k in 0..=pt_degree {
        pc[k] = pt[k];
    }
    for k in pt_degree + 1..=NIED2_MAX_DEGREE {
        pc[k] = 0;
    }

    *pc_degree = pt_degree;
}

/* Calculate the values of the constants V(J,R) */
fn calculate_v(
    px: &[i32],
    px_degree: usize,
    pb: &mut [i32],
    pb_degree: &mut usize,
    v: &mut [i32],
    maxv: usize,
) {
    const NONZERO_ELEMENT: usize = 1;    /* nonzero element of Z_2  */
    const ARBITRARY_ELEMENT: usize = 1;  /* arbitray element of Z_2 */

    /* The polynomial ph is px**(J-1), which is the value of B on arrival. */
    let mut ph = [0; NIED2_MAX_DEGREE + 1];
    let bigm = *pb_degree;      /* m from section 3.3 */
    let mut m;                  /* m from section 2.3 */

    for k in 0..=NIED2_MAX_DEGREE {
        ph[k] = pb[k];
    }

    /* Now multiply B by PX so B becomes PX**J. */
    poly_multiply(px, px_degree, pb, *pb_degree, pb, pb_degree);
    m = *pb_degree;

    /* Choose Kj as defined in section 3.3 */
    let kj = bigm;

    /* Choose values of V in accordance with the conditions in section 3.3 */
    for r in 0..kj {
        v[r] = 0;
    }
    v[kj] = 1;

    if kj >= bigm {
        for r in kj + 1..m {
            v[r] = ARBITRARY_ELEMENT as i32;
        }
    } else {
        /* This block is never reached. */
        let mut term = nied2_sub(0, ph[kj] as usize) as i32;

        for r in kj + 1..bigm {
            v[r] = ARBITRARY_ELEMENT as i32;
            term = nied2_sub(term as usize, nied2_mul(ph[r] as usize, v[r] as usize)) as i32;
        }

        v[bigm] = nied2_add(NONZERO_ELEMENT, term as usize) as i32;
        for r in bigm + 1..m {
            v[r] = ARBITRARY_ELEMENT as i32;
        }
    }

    /* Calculate the remaining V's using the recursion of section 2.3 */
    for r in 0..=maxv - m {
        let mut term = 0;
        for k in 0..m {
            term = nied2_sub(term as usize, nied2_mul(pb[k] as usize, v[r + k] as usize));
        }
        v[r + m] = term as i32;
    }
}

fn calculate_cj(ns: &mut Niederreiter2, dimension: usize) {
    let mut ci = [[0; NIED2_NBITS]; NIED2_NBITS];
    let mut v = [0; MAXV + 1];

    for i_dim in 0..dimension {
        let poly_index = i_dim + 1;
        let mut u = 0;

        /* Initialize PX and PB */
        let mut px = [0; NIED2_MAX_DEGREE + 1];
        let mut pb = [0; NIED2_MAX_DEGREE + 1];
        let px_degree = POLY_DEGREE[poly_index];
        let mut pb_degree = 0;

        for k in 0..=px_degree {
            px[k] = PRIMITIVE_POLY[poly_index][k];
            pb[k] = 0;
        }
        for k in px_degree + 1..=NIED2_MAX_DEGREE {
            px[k] = 0;
            pb[k] = 0;
        }
        pb[0] = 1;

        for j in 0..NIED2_NBITS {
            /* If U = 0, recalculate V */
            if u == 0 {
                calculate_v(&px, px_degree, &mut pb, &mut pb_degree, &mut v, MAXV);
            }

            /* Now C is obtained from V */
            for r in 0..NIED2_NBITS {
                ci[r][j] = v[r + u];
            }

            /* Advance state variables */
            u += 1;
            if u == px_degree {
                u = 0;
            }
        }

        /* Pack CI into CJ */
        for r in 0..NIED2_NBITS {
            let mut term = 0;
            for j in 0..NIED2_NBITS {
                term = 2 * term + ci[r][j];
            }
            ns.cj[r][i_dim] = term;
        }
    }
}

impl Niederreiter2 {
    pub fn new(dimension: usize) -> Result<Self, &'static str> {
        if dimension < 1 || dimension > NIED2_MAX_DIMENSION {
            return Err("Invalid dimension");
        }

        let mut state = Niederreiter2 {
            sequence_count: 0,
            cj: [[0; NIED2_MAX_DIMENSION]; NIED2_NBITS],
            nextq: [0; NIED2_MAX_DIMENSION],
        };

        calculate_cj(&mut state, dimension);

        Ok(state)
    }

    pub fn get(&mut self, dimension: usize) -> Result<Vec<f64>, &'static str> {
        const RECIP: f64 = 1.0 / ((1u64 << NIED2_NBITS) as f64); /* 2^(-nbits) */

        if dimension < 1 || dimension > NIED2_MAX_DIMENSION {
            return Err("Invalid dimension");
        }

        /* Load the result from the saved state */
        let mut v = vec![0.0; dimension];
        for i_dim in 0..dimension {
            v[i_dim] = self.nextq[i_dim] as f64 * RECIP;
        }

        /* Find the position of the least-significant zero in sequence_count */
        let mut r = 0;
        let mut c = self.sequence_count;
        loop {
            if (c % 2) == 1 {
                r += 1;
                c /= 2;
            } else {
                break;
            }
        }

        if r >= NIED2_NBITS {
            return Err("Sequence count overflow");
        }

        /* Calculate the next state */
        for i_dim in 0..dimension {
            self.nextq[i_dim] ^= self.cj[r][i_dim];
        }

        self.sequence_count += 1;

        Ok(v)
    }
}