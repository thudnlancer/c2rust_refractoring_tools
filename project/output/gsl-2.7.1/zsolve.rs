#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
}
pub type size_t = u64;
pub type C2RustUnnamed = i32;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
pub type gsl_complex_packed_ptr = *mut libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_poly_complex_workspace {
    pub nc: size_t,
    pub matrix: *mut libc::c_double,
}
unsafe extern "C" fn set_companion_matrix(
    mut a: *const libc::c_double,
    mut nc: size_t,
    mut m: *mut libc::c_double,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < nc {
        j = 0 as i32 as size_t;
        while j < nc {
            *m.offset(i.wrapping_mul(nc).wrapping_add(j) as isize) = 0.0f64;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 1 as i32 as size_t;
    while i < nc {
        *m
            .offset(
                i.wrapping_mul(nc).wrapping_add(i.wrapping_sub(1 as i32 as u64)) as isize,
            ) = 1.0f64;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as i32 as size_t;
    while i < nc {
        *m
            .offset(
                i.wrapping_mul(nc).wrapping_add(nc.wrapping_sub(1 as i32 as u64))
                    as isize,
            ) = -*a.offset(i as isize) / *a.offset(nc as isize);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn balance_companion_matrix(
    mut m: *mut libc::c_double,
    mut nc: size_t,
) {
    let mut not_converged: i32 = 1 as i32;
    let mut row_norm: libc::c_double = 0 as i32 as libc::c_double;
    let mut col_norm: libc::c_double = 0 as i32 as libc::c_double;
    while not_converged != 0 {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut g: libc::c_double = 0.;
        let mut f: libc::c_double = 0.;
        let mut s: libc::c_double = 0.;
        not_converged = 0 as i32;
        i = 0 as i32 as size_t;
        while i < nc {
            if i != nc.wrapping_sub(1 as i32 as u64) {
                col_norm = fabs(
                    *m
                        .offset(
                            i
                                .wrapping_add(1 as i32 as u64)
                                .wrapping_mul(nc)
                                .wrapping_add(i) as isize,
                        ),
                );
            } else {
                col_norm = 0 as i32 as libc::c_double;
                j = 0 as i32 as size_t;
                while j < nc.wrapping_sub(1 as i32 as u64) {
                    col_norm
                        += fabs(
                            *m
                                .offset(
                                    j
                                        .wrapping_mul(nc)
                                        .wrapping_add(nc.wrapping_sub(1 as i32 as u64)) as isize,
                                ),
                        );
                    j = j.wrapping_add(1);
                    j;
                }
            }
            if i == 0 as i32 as u64 {
                row_norm = fabs(
                    *m
                        .offset(
                            (0 as i32 as u64)
                                .wrapping_mul(nc)
                                .wrapping_add(nc.wrapping_sub(1 as i32 as u64)) as isize,
                        ),
                );
            } else if i == nc.wrapping_sub(1 as i32 as u64) {
                row_norm = fabs(
                    *m
                        .offset(
                            i
                                .wrapping_mul(nc)
                                .wrapping_add(i.wrapping_sub(1 as i32 as u64)) as isize,
                        ),
                );
            } else {
                row_norm = fabs(
                    *m
                        .offset(
                            i
                                .wrapping_mul(nc)
                                .wrapping_add(i.wrapping_sub(1 as i32 as u64)) as isize,
                        ),
                )
                    + fabs(
                        *m
                            .offset(
                                i
                                    .wrapping_mul(nc)
                                    .wrapping_add(nc.wrapping_sub(1 as i32 as u64)) as isize,
                            ),
                    );
            }
            if !(col_norm == 0 as i32 as libc::c_double
                || row_norm == 0 as i32 as libc::c_double)
            {
                g = row_norm / 2 as i32 as libc::c_double;
                f = 1 as i32 as libc::c_double;
                s = col_norm + row_norm;
                while col_norm < g {
                    f *= 2 as i32 as libc::c_double;
                    col_norm *= (2 as i32 * 2 as i32) as libc::c_double;
                }
                g = row_norm * 2 as i32 as libc::c_double;
                while col_norm > g {
                    f /= 2 as i32 as libc::c_double;
                    col_norm /= (2 as i32 * 2 as i32) as libc::c_double;
                }
                if row_norm + col_norm < 0.95f64 * s * f {
                    not_converged = 1 as i32;
                    g = 1 as i32 as libc::c_double / f;
                    if i == 0 as i32 as u64 {
                        *m
                            .offset(
                                (0 as i32 as u64)
                                    .wrapping_mul(nc)
                                    .wrapping_add(nc.wrapping_sub(1 as i32 as u64)) as isize,
                            ) *= g;
                    } else {
                        *m
                            .offset(
                                i
                                    .wrapping_mul(nc)
                                    .wrapping_add(i.wrapping_sub(1 as i32 as u64)) as isize,
                            ) *= g;
                        *m
                            .offset(
                                i
                                    .wrapping_mul(nc)
                                    .wrapping_add(nc.wrapping_sub(1 as i32 as u64)) as isize,
                            ) *= g;
                    }
                    if i == nc.wrapping_sub(1 as i32 as u64) {
                        j = 0 as i32 as size_t;
                        while j < nc {
                            *m.offset(j.wrapping_mul(nc).wrapping_add(i) as isize) *= f;
                            j = j.wrapping_add(1);
                            j;
                        }
                    } else {
                        *m
                            .offset(
                                i
                                    .wrapping_add(1 as i32 as u64)
                                    .wrapping_mul(nc)
                                    .wrapping_add(i) as isize,
                            ) *= f;
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
}
unsafe extern "C" fn qr_companion(
    mut h: *mut libc::c_double,
    mut nc: size_t,
    mut zroot: gsl_complex_packed_ptr,
) -> i32 {
    let mut t: libc::c_double = 0.0f64;
    let mut iterations: size_t = 0;
    let mut e: size_t = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut m: size_t = 0;
    let mut w: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut p: libc::c_double = 0 as i32 as libc::c_double;
    let mut q: libc::c_double = 0 as i32 as libc::c_double;
    let mut r: libc::c_double = 0 as i32 as libc::c_double;
    let mut notlast: i32 = 0;
    let mut n: size_t = nc;
    loop {
        if n == 0 as i32 as u64 {
            return GSL_SUCCESS as i32;
        }
        iterations = 0 as i32 as size_t;
        loop {
            e = n;
            while e >= 2 as i32 as u64 {
                let mut a1: libc::c_double = fabs(
                    *h
                        .offset(
                            e
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(nc)
                                .wrapping_add(
                                    e
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_sub(1 as i32 as u64),
                                ) as isize,
                        ),
                );
                let mut a2: libc::c_double = fabs(
                    *h
                        .offset(
                            e
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(nc)
                                .wrapping_add(
                                    e
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_sub(1 as i32 as u64),
                                ) as isize,
                        ),
                );
                let mut a3: libc::c_double = fabs(
                    *h
                        .offset(
                            e
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(nc)
                                .wrapping_add(e.wrapping_sub(1 as i32 as u64)) as isize,
                        ),
                );
                if a1 <= 2.2204460492503131e-16f64 * (a2 + a3) {
                    break;
                }
                e = e.wrapping_sub(1);
                e;
            }
            x = *h
                .offset(
                    n
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul(nc)
                        .wrapping_add(n.wrapping_sub(1 as i32 as u64)) as isize,
                );
            if e == n {
                *zroot
                    .offset(
                        (2 as i32 as u64).wrapping_mul(n.wrapping_sub(1 as i32 as u64))
                            as isize,
                    ) = x + t;
                *zroot
                    .offset(
                        (2 as i32 as u64)
                            .wrapping_mul(n.wrapping_sub(1 as i32 as u64))
                            .wrapping_add(1 as i32 as u64) as isize,
                    ) = 0 as i32 as libc::c_double;
                n = n.wrapping_sub(1);
                n;
                break;
            } else {
                y = *h
                    .offset(
                        n
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(nc)
                            .wrapping_add(
                                n
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64),
                            ) as isize,
                    );
                w = *h
                    .offset(
                        n
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(nc)
                            .wrapping_add(n.wrapping_sub(1 as i32 as u64)) as isize,
                    )
                    * *h
                        .offset(
                            n
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(nc)
                                .wrapping_add(
                                    n
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_sub(1 as i32 as u64),
                                ) as isize,
                        );
                if e == n.wrapping_sub(1 as i32 as u64) {
                    p = (y - x) / 2 as i32 as libc::c_double;
                    q = p * p + w;
                    y = sqrt(fabs(q));
                    x += t;
                    if q > 0 as i32 as libc::c_double {
                        if p < 0 as i32 as libc::c_double {
                            y = -y;
                        }
                        y += p;
                        *zroot
                            .offset(
                                (2 as i32 as u64)
                                    .wrapping_mul(n.wrapping_sub(1 as i32 as u64)) as isize,
                            ) = x - w / y;
                        *zroot
                            .offset(
                                (2 as i32 as u64)
                                    .wrapping_mul(n.wrapping_sub(1 as i32 as u64))
                                    .wrapping_add(1 as i32 as u64) as isize,
                            ) = 0 as i32 as libc::c_double;
                        *zroot
                            .offset(
                                (2 as i32 as u64)
                                    .wrapping_mul(n.wrapping_sub(2 as i32 as u64)) as isize,
                            ) = x + y;
                        *zroot
                            .offset(
                                (2 as i32 as u64)
                                    .wrapping_mul(n.wrapping_sub(2 as i32 as u64))
                                    .wrapping_add(1 as i32 as u64) as isize,
                            ) = 0 as i32 as libc::c_double;
                    } else {
                        *zroot
                            .offset(
                                (2 as i32 as u64)
                                    .wrapping_mul(n.wrapping_sub(1 as i32 as u64)) as isize,
                            ) = x + p;
                        *zroot
                            .offset(
                                (2 as i32 as u64)
                                    .wrapping_mul(n.wrapping_sub(1 as i32 as u64))
                                    .wrapping_add(1 as i32 as u64) as isize,
                            ) = -y;
                        *zroot
                            .offset(
                                (2 as i32 as u64)
                                    .wrapping_mul(n.wrapping_sub(2 as i32 as u64)) as isize,
                            ) = x + p;
                        *zroot
                            .offset(
                                (2 as i32 as u64)
                                    .wrapping_mul(n.wrapping_sub(2 as i32 as u64))
                                    .wrapping_add(1 as i32 as u64) as isize,
                            ) = y;
                    }
                    n = (n as u64).wrapping_sub(2 as i32 as u64) as size_t as size_t;
                    break;
                } else {
                    if iterations == 120 as i32 as u64 {
                        return GSL_FAILURE as i32;
                    }
                    if iterations.wrapping_rem(10 as i32 as u64) == 0 as i32 as u64
                        && iterations > 0 as i32 as u64
                    {
                        t += x;
                        i = 1 as i32 as size_t;
                        while i <= n {
                            *h
                                .offset(
                                    i
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_mul(nc)
                                        .wrapping_add(i.wrapping_sub(1 as i32 as u64)) as isize,
                                ) -= x;
                            i = i.wrapping_add(1);
                            i;
                        }
                        s = fabs(
                            *h
                                .offset(
                                    n
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_mul(nc)
                                        .wrapping_add(
                                            n
                                                .wrapping_sub(1 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64),
                                        ) as isize,
                                ),
                        )
                            + fabs(
                                *h
                                    .offset(
                                        n
                                            .wrapping_sub(1 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                            .wrapping_mul(nc)
                                            .wrapping_add(
                                                n
                                                    .wrapping_sub(2 as i32 as u64)
                                                    .wrapping_sub(1 as i32 as u64),
                                            ) as isize,
                                    ),
                            );
                        y = 0.75f64 * s;
                        x = y;
                        w = -0.4375f64 * s * s;
                    }
                    iterations = iterations.wrapping_add(1);
                    iterations;
                    m = n.wrapping_sub(2 as i32 as u64);
                    while m >= e {
                        let mut a1_0: libc::c_double = 0.;
                        let mut a2_0: libc::c_double = 0.;
                        let mut a3_0: libc::c_double = 0.;
                        z = *h
                            .offset(
                                m
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_mul(nc)
                                    .wrapping_add(m.wrapping_sub(1 as i32 as u64)) as isize,
                            );
                        r = x - z;
                        s = y - z;
                        p = *h
                            .offset(
                                m
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_mul(nc)
                                    .wrapping_add(
                                        m
                                            .wrapping_add(1 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64),
                                    ) as isize,
                            )
                            + (r * s - w)
                                / *h
                                    .offset(
                                        m
                                            .wrapping_add(1 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                            .wrapping_mul(nc)
                                            .wrapping_add(m.wrapping_sub(1 as i32 as u64)) as isize,
                                    );
                        q = *h
                            .offset(
                                m
                                    .wrapping_add(1 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_mul(nc)
                                    .wrapping_add(
                                        m
                                            .wrapping_add(1 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64),
                                    ) as isize,
                            ) - z - r - s;
                        r = *h
                            .offset(
                                m
                                    .wrapping_add(2 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_mul(nc)
                                    .wrapping_add(
                                        m
                                            .wrapping_add(1 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64),
                                    ) as isize,
                            );
                        s = fabs(p) + fabs(q) + fabs(r);
                        p /= s;
                        q /= s;
                        r /= s;
                        if m == e {
                            break;
                        }
                        a1_0 = fabs(
                            *h
                                .offset(
                                    m
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_mul(nc)
                                        .wrapping_add(
                                            m
                                                .wrapping_sub(1 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64),
                                        ) as isize,
                                ),
                        );
                        a2_0 = fabs(
                            *h
                                .offset(
                                    m
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_mul(nc)
                                        .wrapping_add(
                                            m
                                                .wrapping_sub(1 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64),
                                        ) as isize,
                                ),
                        );
                        a3_0 = fabs(
                            *h
                                .offset(
                                    m
                                        .wrapping_add(1 as i32 as u64)
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_mul(nc)
                                        .wrapping_add(
                                            m
                                                .wrapping_add(1 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64),
                                        ) as isize,
                                ),
                        );
                        if a1_0 * (fabs(q) + fabs(r))
                            <= 2.2204460492503131e-16f64 * fabs(p) * (a2_0 + a3_0)
                        {
                            break;
                        }
                        m = m.wrapping_sub(1);
                        m;
                    }
                    i = m.wrapping_add(2 as i32 as u64);
                    while i <= n {
                        *h
                            .offset(
                                i
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_mul(nc)
                                    .wrapping_add(
                                        i
                                            .wrapping_sub(2 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64),
                                    ) as isize,
                            ) = 0 as i32 as libc::c_double;
                        i = i.wrapping_add(1);
                        i;
                    }
                    i = m.wrapping_add(3 as i32 as u64);
                    while i <= n {
                        *h
                            .offset(
                                i
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_mul(nc)
                                    .wrapping_add(
                                        i
                                            .wrapping_sub(3 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64),
                                    ) as isize,
                            ) = 0 as i32 as libc::c_double;
                        i = i.wrapping_add(1);
                        i;
                    }
                    let mut current_block_122: u64;
                    k = m;
                    while k <= n.wrapping_sub(1 as i32 as u64) {
                        notlast = (k != n.wrapping_sub(1 as i32 as u64)) as i32;
                        if k != m {
                            p = *h
                                .offset(
                                    k
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_mul(nc)
                                        .wrapping_add(
                                            k
                                                .wrapping_sub(1 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64),
                                        ) as isize,
                                );
                            q = *h
                                .offset(
                                    k
                                        .wrapping_add(1 as i32 as u64)
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_mul(nc)
                                        .wrapping_add(
                                            k
                                                .wrapping_sub(1 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64),
                                        ) as isize,
                                );
                            r = if notlast != 0 {
                                *h
                                    .offset(
                                        k
                                            .wrapping_add(2 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                            .wrapping_mul(nc)
                                            .wrapping_add(
                                                k
                                                    .wrapping_sub(1 as i32 as u64)
                                                    .wrapping_sub(1 as i32 as u64),
                                            ) as isize,
                                    )
                            } else {
                                0.0f64
                            };
                            x = fabs(p) + fabs(q) + fabs(r);
                            if x == 0 as i32 as libc::c_double {
                                current_block_122 = 15462640364611497761;
                            } else {
                                p /= x;
                                q /= x;
                                r /= x;
                                current_block_122 = 851619935621435220;
                            }
                        } else {
                            current_block_122 = 851619935621435220;
                        }
                        match current_block_122 {
                            851619935621435220 => {
                                s = sqrt(p * p + q * q + r * r);
                                if p < 0 as i32 as libc::c_double {
                                    s = -s;
                                }
                                if k != m {
                                    *h
                                        .offset(
                                            k
                                                .wrapping_sub(1 as i32 as u64)
                                                .wrapping_mul(nc)
                                                .wrapping_add(
                                                    k
                                                        .wrapping_sub(1 as i32 as u64)
                                                        .wrapping_sub(1 as i32 as u64),
                                                ) as isize,
                                        ) = -s * x;
                                } else if e != m {
                                    *h
                                        .offset(
                                            k
                                                .wrapping_sub(1 as i32 as u64)
                                                .wrapping_mul(nc)
                                                .wrapping_add(
                                                    k
                                                        .wrapping_sub(1 as i32 as u64)
                                                        .wrapping_sub(1 as i32 as u64),
                                                ) as isize,
                                        ) *= -(1 as i32) as libc::c_double;
                                }
                                p += s;
                                x = p / s;
                                y = q / s;
                                z = r / s;
                                q /= p;
                                r /= p;
                                j = k;
                                while j <= n {
                                    p = *h
                                        .offset(
                                            k
                                                .wrapping_sub(1 as i32 as u64)
                                                .wrapping_mul(nc)
                                                .wrapping_add(j.wrapping_sub(1 as i32 as u64)) as isize,
                                        )
                                        + q
                                            * *h
                                                .offset(
                                                    k
                                                        .wrapping_add(1 as i32 as u64)
                                                        .wrapping_sub(1 as i32 as u64)
                                                        .wrapping_mul(nc)
                                                        .wrapping_add(j.wrapping_sub(1 as i32 as u64)) as isize,
                                                );
                                    if notlast != 0 {
                                        p
                                            += r
                                                * *h
                                                    .offset(
                                                        k
                                                            .wrapping_add(2 as i32 as u64)
                                                            .wrapping_sub(1 as i32 as u64)
                                                            .wrapping_mul(nc)
                                                            .wrapping_add(j.wrapping_sub(1 as i32 as u64)) as isize,
                                                    );
                                        *h
                                            .offset(
                                                k
                                                    .wrapping_add(2 as i32 as u64)
                                                    .wrapping_sub(1 as i32 as u64)
                                                    .wrapping_mul(nc)
                                                    .wrapping_add(j.wrapping_sub(1 as i32 as u64)) as isize,
                                            ) -= p * z;
                                    }
                                    *h
                                        .offset(
                                            k
                                                .wrapping_add(1 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64)
                                                .wrapping_mul(nc)
                                                .wrapping_add(j.wrapping_sub(1 as i32 as u64)) as isize,
                                        ) -= p * y;
                                    *h
                                        .offset(
                                            k
                                                .wrapping_sub(1 as i32 as u64)
                                                .wrapping_mul(nc)
                                                .wrapping_add(j.wrapping_sub(1 as i32 as u64)) as isize,
                                        ) -= p * x;
                                    j = j.wrapping_add(1);
                                    j;
                                }
                                j = if k.wrapping_add(3 as i32 as u64) < n {
                                    k.wrapping_add(3 as i32 as u64)
                                } else {
                                    n
                                };
                                i = e;
                                while i <= j {
                                    p = x
                                        * *h
                                            .offset(
                                                i
                                                    .wrapping_sub(1 as i32 as u64)
                                                    .wrapping_mul(nc)
                                                    .wrapping_add(k.wrapping_sub(1 as i32 as u64)) as isize,
                                            )
                                        + y
                                            * *h
                                                .offset(
                                                    i
                                                        .wrapping_sub(1 as i32 as u64)
                                                        .wrapping_mul(nc)
                                                        .wrapping_add(
                                                            k
                                                                .wrapping_add(1 as i32 as u64)
                                                                .wrapping_sub(1 as i32 as u64),
                                                        ) as isize,
                                                );
                                    if notlast != 0 {
                                        p
                                            += z
                                                * *h
                                                    .offset(
                                                        i
                                                            .wrapping_sub(1 as i32 as u64)
                                                            .wrapping_mul(nc)
                                                            .wrapping_add(
                                                                k
                                                                    .wrapping_add(2 as i32 as u64)
                                                                    .wrapping_sub(1 as i32 as u64),
                                                            ) as isize,
                                                    );
                                        *h
                                            .offset(
                                                i
                                                    .wrapping_sub(1 as i32 as u64)
                                                    .wrapping_mul(nc)
                                                    .wrapping_add(
                                                        k
                                                            .wrapping_add(2 as i32 as u64)
                                                            .wrapping_sub(1 as i32 as u64),
                                                    ) as isize,
                                            ) -= p * r;
                                    }
                                    *h
                                        .offset(
                                            i
                                                .wrapping_sub(1 as i32 as u64)
                                                .wrapping_mul(nc)
                                                .wrapping_add(
                                                    k
                                                        .wrapping_add(1 as i32 as u64)
                                                        .wrapping_sub(1 as i32 as u64),
                                                ) as isize,
                                        ) -= p * q;
                                    *h
                                        .offset(
                                            i
                                                .wrapping_sub(1 as i32 as u64)
                                                .wrapping_mul(nc)
                                                .wrapping_add(k.wrapping_sub(1 as i32 as u64)) as isize,
                                        ) -= p;
                                    i = i.wrapping_add(1);
                                    i;
                                }
                            }
                            _ => {}
                        }
                        k = k.wrapping_add(1);
                        k;
                    }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_poly_complex_solve(
    mut a: *const libc::c_double,
    mut n: size_t,
    mut w: *mut gsl_poly_complex_workspace,
    mut z: gsl_complex_packed_ptr,
) -> i32 {
    let mut status: i32 = 0;
    let mut m: *mut libc::c_double = 0 as *mut libc::c_double;
    if n == 0 as i32 as u64 {
        gsl_error(
            b"number of terms must be a positive integer\0" as *const u8 as *const i8,
            b"zsolve.c\0" as *const u8 as *const i8,
            50 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if n == 1 as i32 as u64 {
        gsl_error(
            b"cannot solve for only one term\0" as *const u8 as *const i8,
            b"zsolve.c\0" as *const u8 as *const i8,
            55 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if *a.offset(n.wrapping_sub(1 as i32 as u64) as isize) == 0 as i32 as libc::c_double
    {
        gsl_error(
            b"leading term of polynomial must be non-zero\0" as *const u8 as *const i8,
            b"zsolve.c\0" as *const u8 as *const i8,
            60 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*w).nc != n.wrapping_sub(1 as i32 as u64) {
        gsl_error(
            b"size of workspace does not match polynomial\0" as *const u8 as *const i8,
            b"zsolve.c\0" as *const u8 as *const i8,
            65 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    m = (*w).matrix;
    set_companion_matrix(a, n.wrapping_sub(1 as i32 as u64), m);
    balance_companion_matrix(m, n.wrapping_sub(1 as i32 as u64));
    status = qr_companion(m, n.wrapping_sub(1 as i32 as u64), z);
    if status != 0 {
        gsl_error(
            b"root solving qr method failed to converge\0" as *const u8 as *const i8,
            b"zsolve.c\0" as *const u8 as *const i8,
            78 as i32,
            GSL_EFAILED as i32,
        );
        return GSL_EFAILED as i32;
    }
    return GSL_SUCCESS as i32;
}