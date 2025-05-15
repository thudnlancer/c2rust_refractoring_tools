use std::cmp::Ordering;
use libzahl::{zahl_char_t, ZError, ZNumber};

const BITS_PER_CHAR: usize = std::mem::size_of::<zahl_char_t>() * 8;

pub fn zpow(a: &mut ZNumber, b: &ZNumber, c: &ZNumber) -> Result<(), ZError> {
    /*
     * Exponentiation by squaring.
     * 
     * 7↑19 = 7↑10011₂ = 7↑2⁰ ⋅ 7↑2¹ ⋅ 7↑2⁴ where a↑2↑(n + 1) = (a↑2↑n)².
     */

    // TODO use zpowu when possible

    if c.signum() <= 0 {
        if c.is_zero() {
            if b.is_zero() {
                return Err(ZError::ZeroPowZero);
            }
            a.set_u32(1);
        } else if b.is_zero() {
            return Err(ZError::DivisionByZero);
        } else {
            a.set_zero();
        }
        return Ok(());
    } else if b.is_zero() {
        a.set_zero();
        return Ok(());
    }

    let bits = c.bits();
    let n = bits / BITS_PER_CHAR;

    let neg = b.is_negative() && c.is_odd();
    let mut tb = b.abs();
    let mut tc = c.clone();
    a.set_u32(1);

    for i in 0..n {
        let mut x = tc.chars[i];
        for _ in 0..BITS_PER_CHAR {
            if x & 1 != 0 {
                a.mul_assign(&tb)?;
            }
            tb.square_assign()?;
            x >>= 1;
        }
    }

    let mut x = tc.chars[n];
    while x != 0 {
        if x & 1 != 0 {
            a.mul_assign(&tb)?;
        }
        tb.square_assign()?;
        x >>= 1;
    }

    if neg {
        a.negate();
    }

    Ok(())
}