use std::cmp::Ordering;
use std::mem;

// Assuming these are defined in the internals module
use crate::internals::{
    z_t, zbits, zlsh, zrsh, zcmpmag, zsub_unsigned, zbset, zzero, zsignum, zseti, zabs, zswap,
    ZERROR_DIV_0, ZERROR_0_DIV_0, libzahl_failure, BITS_PER_CHAR,
};

thread_local! {
    static TA: z_t = z_t::default();
    static TB: z_t = z_t::default();
    static TD: z_t = z_t::default();
    static TDS_PROPER: [z_t; BITS_PER_CHAR] = [z_t::default(); BITS_PER_CHAR];
}

fn zdivmod_impl(a: &mut z_t, b: &mut z_t, c: &z_t, d: &z_t) {
    let c_bits = zbits(c);
    let d_bits = zbits(d);
    let mut bit = c_bits - d_bits;

    TA.with(|ta| {
        TB.with(|tb| {
            TD.with(|td| {
                TDS_PROPER.with(|tds_proper| {
                    zlsh(td, d, bit);
                    td.sign = 1;

                    if zcmpmag(td, c) > 0 {
                        zrsh(td, td, 1);
                        bit -= 1;
                    }

                    ta.sign = 0;
                    zabs(tb, c);

                    if bit <= BITS_PER_CHAR {
                        loop {
                            if zcmpmag(td, tb) <= 0 {
                                zsub_unsigned(tb, tb, td);
                                zbset(ta, ta, bit, 1);
                            }
                            if bit == 0 || zzero(tb) {
                                break;
                            }
                            bit -= 1;
                            zrsh(td, td, 1);
                        }
                    } else {
                        let mut tds = [z_t::default(); BITS_PER_CHAR];
                        for i in 0..BITS_PER_CHAR {
                            zrsh(&mut tds[i], td, i);
                            tds[i].used = tds_proper[i].used;
                            tds[i].sign = tds_proper[i].sign;
                            tds[i].chars = tds_proper[i].chars.clone();
                        }

                        loop {
                            for i in 0..BITS_PER_CHAR {
                                if zcmpmag(&tds[i], tb) <= 0 {
                                    zsub_unsigned(tb, tb, &tds[i]);
                                    zbset(ta, ta, bit, 1);
                                }
                                if bit == 0 || zzero(tb) {
                                    break 'done;
                                }
                                bit -= 1;
                            }
                            for i in (0..=bit.min(BITS_PER_CHAR - 1)).rev() {
                                zrsh_taint(&mut tds[i], BITS_PER_CHAR);
                            }
                        }
                    }

                    'done: {
                        zswap(a, ta);
                        zswap(b, tb);
                    }
                });
            });
        });
    });
}

pub fn zdivmod(a: &mut z_t, b: &mut z_t, c: &z_t, d: &z_t) -> Result<(), i32> {
    let c_sign = zsignum(c);
    let sign = c_sign * zsignum(d);

    if sign == 0 {
        if !zzero(c) {
            return Err(-ZERROR_DIV_0);
        } else if zzero(d) {
            return Err(-ZERROR_0_DIV_0);
        } else {
            a.sign = 0;
            b.sign = 0;
            return Ok(());
        }
    }

    let cmpmag = zcmpmag(c, d);
    if cmpmag <= 0 {
        if cmpmag == 0 {
            zseti(a, sign);
            b.sign = 0;
        } else {
            *b = c.clone();
            a.sign = 0;
        }
        return Ok(());
    }

    zdivmod_impl(a, b, c, d);
    a.sign = sign;
    if zsignum(b) > 0 {
        b.sign = c_sign;
    }

    Ok(())
}