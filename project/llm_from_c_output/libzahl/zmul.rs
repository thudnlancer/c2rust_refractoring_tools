/* See LICENSE file for copyright and license details. */
use std::cmp::max;

struct Z {
    chars: Vec<u64>,
    used: usize,
    signum: i32,
}

impl Z {
    fn new() -> Self {
        Z {
            chars: Vec::new(),
            used: 0,
            signum: 0,
        }
    }

    fn ensure_size(&mut self, size: usize) {
        if self.chars.len() < size {
            self.chars.resize(size, 0);
        }
    }

    fn is_zero(&self) -> bool {
        self.signum == 0
    }
}

const BITS_PER_CHAR: usize = 64;

fn zmul_ll_single_char(a: &mut Z, b: &Z, c: &Z) {
    a.ensure_size(1);
    a.used = 1;
    a.chars[0] = b.chars[0] * c.chars[0];
    a.signum = 1;
}

fn zmul_ll(a: &mut Z, b: &Z, c: &Z) {
    /*
     * Karatsuba algorithm
     * 
     * Basically, this is how you were taught to multiply large numbers
     * by hand in school: 4010⋅3020 = (4000 + 10)(3000 + 20) =
     * = 40⋅30⋅10⁴ + (40⋅20 + 30⋅10)⋅10² + 10⋅20, but the middle is
     * optimised to only one multiplication:
     * 40⋅20 + 30⋅10 = (40 + 10)(30 + 20) − 40⋅30 − 10⋅20.
     * This optimisation is crucial. Without it, the algorithm with
     * run in O(n²).
     */

    if b.is_zero() || c.is_zero() {
        a.signum = 0;
        return;
    }

    let m = zbits(b);
    let mut m2 = if b == c { m } else { zbits(c) };

    if m + m2 <= BITS_PER_CHAR {
        zmul_ll_single_char(a, b, c);
        return;
    }

    let m = max(m, m2);
    m2 = m >> 1;

    let mut b_high = Z::new();
    let mut b_low = Z::new();
    let mut c_high = Z::new();
    let mut c_low = Z::new();

    zsplit_pz(&mut b_high, &mut b_low, b, m2);
    zsplit_pz(&mut c_high, &mut c_low, c, m2);

    let mut z0 = Z::new();
    zmul_ll(&mut z0, &b_low, &c_low);

    let mut z1 = Z::new();
    zadd_unsigned_assign(&mut b_low, &b_high);
    zadd_unsigned_assign(&mut c_low, &c_high);
    zmul_ll(&mut z1, &b_low, &c_low);

    let mut z2 = Z::new();
    zmul_ll(&mut z2, &b_high, &c_high);

    zsub_nonnegative_assign(&mut z1, &z0);
    zsub_nonnegative_assign(&mut z1, &z2);

    zlsh(&mut z1, &z1, m2);
    m2 <<= 1;
    zlsh(&mut z2, &z2, m2);
    zadd_unsigned_assign(a, &z1);
    zadd_unsigned_assign(a, &z2);
}

// Helper functions (stubs - actual implementations would depend on your number representation)
fn zbits(z: &Z) -> usize { 0 }
fn zsplit_pz(high: &mut Z, low: &mut Z, z: &Z, bits: usize) {}
fn zadd_unsigned_assign(a: &mut Z, b: &Z) {}
fn zsub_nonnegative_assign(a: &mut Z, b: &Z) {}
fn zlsh(res: &mut Z, z: &Z, bits: usize) {}