/* See LICENSE file for copyright and license details. */
use std::convert::TryFrom;
use libzahl::{
    z_t,
    zabs, zzero, znegative, zsetu, zset, zsqr_ll, zmul_ll, zneg,
    ZERROR_0_POW_0,
    libzahl_failure, libzahl_tmp_pow_b,
};

pub fn zpowu(a: &mut z_t, b: &z_t, mut c: u64) -> Result<(), i32> {
    let neg;

    if c == 0 {
        if zzero(b) {
            libzahl_failure(-ZERROR_0_POW_0)?;
        }
        zsetu(a, 1);
        return Ok(());
    } else if zzero(b) {
        a.sign = 0;
        return Ok(());
    }

    neg = znegative(b) && (c & 1 != 0);
    zabs(&mut libzahl_tmp_pow_b, b);

    if c & 1 != 0 {
        zset(a, &libzahl_tmp_pow_b);
    } else {
        zsetu(a, 1);
    }

    while {
        c >>= 1;
        c != 0
    } {
        zsqr_ll(&mut libzahl_tmp_pow_b, &libzahl_tmp_pow_b);
        if c & 1 != 0 {
            zmul_ll(a, a, &libzahl_tmp_pow_b);
        }
    }

    if neg {
        zneg(a, a);
    }

    Ok(())
}