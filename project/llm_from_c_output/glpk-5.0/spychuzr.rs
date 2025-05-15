/* spychuzr.rs */

use std::f64;
use std::ptr;
use std::mem;
use std::cmp;
use std::ops::{Index, IndexMut};

use crate::spxlp::SPXLP;
use crate::env::talloc;
use crate::bfd::bfd_ftran;

#[repr(C)]
pub struct SPYSE {
    valid: i32,
    refsp: *mut u8,
    gamma: *mut f64,
    work: *mut f64,
    u: FVS,
}

#[repr(C)]
pub struct FVS {
    n: i32,
    nnz: i32,
    ind: *mut i32,
    vec: *mut f64,
}

pub fn spy_chuzr_sel(
    lp: &SPXLP,
    beta: &[f64],
    tol: f64,
    tol1: f64,
    list: Option<&mut [i32]>,
) -> i32 {
    let m = lp.m;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;
    let mut num = 0;

    for i in 1..=m {
        let k = head[i as usize] as usize;
        let lk = l[k];
        let uk = u[k];

        if beta[i as usize] < lk {
            let eps = tol + tol1 * if lk >= 0.0 { lk } else { -lk };
            if beta[i as usize] < lk - eps {
                num += 1;
                if let Some(list) = list {
                    list[num as usize] = i;
                }
            }
        } else if beta[i as usize] > uk {
            let eps = tol + tol1 * if uk >= 0.0 { uk } else { -uk };
            if beta[i as usize] > uk + eps {
                num += 1;
                if let Some(list) = list {
                    list[num as usize] = i;
                }
            }
        }
    }

    num
}

pub fn spy_chuzr_std(lp: &SPXLP, beta: &[f64], num: i32, list: &[i32]) -> i32 {
    let m = lp.m;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;
    let mut p = 0;
    let mut abs_rp = -1.0;

    assert!(0 < num && num <= m);

    for t in 1..=num {
        let i = list[t as usize] as usize;
        let k = head[i] as usize;
        let abs_ri = if beta[i] < l[k] {
            l[k] - beta[i]
        } else if beta[i] > u[k] {
            beta[i] - u[k]
        } else {
            unreachable!()
        };

        if abs_rp < abs_ri {
            p = i as i32;
            abs_rp = abs_ri;
        }
    }

    assert!(p != 0);
    p
}

pub fn spy_alloc_se(lp: &SPXLP, se: &mut SPYSE) {
    let m = lp.m;
    let n = lp.n;

    se.valid = 0;
    se.refsp = talloc(1 + n, mem::size_of::<u8>()) as *mut u8;
    se.gamma = talloc(1 + m, mem::size_of::<f64>()) as *mut f64;
    se.work = talloc(1 + m, mem::size_of::<f64>()) as *mut f64;

    se.u.n = m;
    se.u.nnz = 0;
    se.u.ind = talloc(1 + m, mem::size_of::<i32>()) as *mut i32;
    se.u.vec = talloc(1 + m, mem::size_of::<f64>()) as *mut f64;

    unsafe {
        for i in 1..=m {
            *se.u.vec.add(i as usize) = 0.0;
        }
    }
}

pub fn spy_reset_refsp(lp: &SPXLP, se: &mut SPYSE) {
    let m = lp.m;
    let n = lp.n;
    let head = &lp.head;

    se.valid = 1;

    unsafe {
        ptr::write_bytes(se.refsp.add(1), 0, n as usize);
        for i in 1..=m {
            let k = head[i as usize] as usize;
            *se.refsp.add(k) = 1;
            *se.gamma.add(i as usize) = 1.0;
        }
    }
}

pub fn spy_eval_gamma_i(lp: &SPXLP, se: &SPYSE, i: i32) -> f64 {
    let m = lp.m;
    let n = lp.n;
    let head = &lp.head;

    assert!(se.valid != 0);
    assert!(1 <= i && i <= m);

    let k = head[i as usize] as usize;
    let mut gamma_i = if unsafe { *se.refsp.add(k) != 0 } {
        1.0
    } else {
        0.0
    };

    let rho = unsafe { &mut *se.work };
    spx_eval_rho(lp, i, rho);

    for j in 1..=(n - m) {
        let k = head[(m + j) as usize] as usize;
        if unsafe { *se.refsp.add(k) != 0 } {
            let t_ij = spx_eval_tij(lp, rho, j);
            gamma_i += t_ij * t_ij;
        }
    }

    gamma_i
}

pub fn spy_chuzr_pse(
    lp: &SPXLP,
    se: &SPYSE,
    beta: &[f64],
    num: i32,
    list: &[i32],
) -> i32 {
    let m = lp.m;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;
    let gamma = unsafe { &*se.gamma };

    assert!(0 < num && num <= m);

    let mut p = 0;
    let mut best = -1.0;

    for t in 1..=num {
        let i = list[t as usize] as usize;
        let k = head[i] as usize;
        let ri = if beta[i] < l[k] {
            l[k] - beta[i]
        } else if beta[i] > u[k] {
            u[k] - beta[i]
        } else {
            unreachable!()
        };

        let temp = if gamma[i] < f64::EPSILON {
            0.0
        } else {
            (ri * ri) / gamma[i]
        };

        if best < temp {
            p = i as i32;
            best = temp;
        }
    }

    assert!(p != 0);
    p
}

pub fn spy_update_gamma(
    lp: &SPXLP,
    se: &mut SPYSE,
    p: i32,
    q: i32,
    trow: &[f64],
    tcol: &[f64],
) -> f64 {
    let m = lp.m;
    let n = lp.n;
    let head = &lp.head;

    assert!(se.valid != 0);
    assert!(1 <= p && p <= m);
    assert!(1 <= q && q <= (n - m));

    let k = head[p as usize] as usize;
    let mut gamma_p = if unsafe { *se.refsp.add(k) != 0 } {
        1.0
    } else {
        0.0
    };
    let delta_p = gamma_p;

    let u = unsafe { &mut *se.work };
    for i in 1..=m {
        u[i as usize] = 0.0;
    }

    for j in 1..=(n - m) {
        let k = head[(m + j) as usize] as usize;
        if unsafe { *se.refsp.add(k) != 0 } && trow[j as usize] != 0.0 {
            gamma_p += trow[j as usize] * trow[j as usize];

            let mut ptr = lp.A_ptr[k];
            let end = lp.A_ptr[k + 1];
            while ptr < end {
                let idx = lp.A_ind[ptr as usize] as usize;
                u[idx] += trow[j as usize] * lp.A_val[ptr as usize];
                ptr += 1;
            }
        }
    }

    bfd_ftran(lp.bfd, u);

    let e = (gamma_p - unsafe { *se.gamma.add(p as usize) }).abs() / (1.0 + gamma_p);

    unsafe {
        *se.gamma.add(p as usize) = gamma_p / (tcol[p as usize] * tcol[p as usize]);

        for i in 1..=m {
            if i == p {
                continue;
            }

            let r = tcol[i as usize] / tcol[p as usize];
            let t1 = *se.gamma.add(i as usize) + r * (r * gamma_p + u[i as usize] + u[i as usize]);
            let k = head[i as usize] as usize;
            let t2 = if *se.refsp.add(k) != 0 { 1.0 } else { 0.0 } + delta_p * r * r;
            *se.gamma.add(i as usize) = if t1 >= t2 { t1 } else { t2 };
        }
    }

    e
}

pub fn spy_update_gamma_s(
    lp: &SPXLP,
    se: &mut SPYSE,
    p: i32,
    q: i32,
    trow: &FVS,
    tcol: &FVS,
) -> f64 {
    let m = lp.m;
    let n = lp.n;
    let head = &lp.head;

    assert!(se.valid != 0);
    assert!(1 <= p && p <= m);
    assert!(1 <= q && q <= (n - m));

    let k = head[p as usize] as usize;
    let mut gamma_p = if unsafe { *se.refsp.add(k) != 0 } {
        1.0
    } else {
        0.0
    };
    let delta_p = gamma_p;

    let u = unsafe { &mut *se.work };
    for i in 1..=m {
        u[i as usize] = 0.0;
    }

    for t in 1..=trow.nnz {
        let j = unsafe { *trow.ind.add(t as usize) };
        let k = head[(m + j) as usize] as usize;
        if unsafe { *se.refsp.add(k) != 0 } {
            let trow_j = unsafe { *trow.vec.add(j as usize) };
            gamma_p += trow_j * trow_j;

            let mut ptr = lp.A_ptr[k];
            let end = lp.A_ptr[k + 1];
            while ptr < end {
                let idx = lp.A_ind[ptr as usize] as usize;
                u[idx] += trow_j * lp.A_val[ptr as usize];
                ptr += 1;
            }
        }
    }

    bfd_ftran(lp.bfd, u);

    let e = (gamma_p - unsafe { *se.gamma.add(p as usize) }).abs() / (1.0 + gamma_p);

    unsafe {
        let tcol_p = *tcol.vec.add(p as usize);
        *se.gamma.add(p as usize) = gamma_p / (tcol_p * tcol_p);

        for t in 1..=tcol.nnz {
            let i = *tcol.ind.add(t as usize);
            if i == p {
                continue;
            }

            let r = *tcol.vec.add(i as usize) / tcol_p;
            let t1 = *se.gamma.add(i as usize) + r * (r * gamma_p + u[i as usize] + u[i as usize]);
            let k = head[i as usize] as usize;
            let t2 = if *se.refsp.add(k) != 0 { 1.0 } else { 0.0 } + delta_p * r * r;
            *se.gamma.add(i as usize) = if t1 >= t2 { t1 } else { t2 };
        }
    }

    e
}

pub fn spy_free_se(_lp: &SPXLP, se: &mut SPYSE) {
    unsafe {
        ptr::drop_in_place(se.refsp);
        ptr::drop_in_place(se.gamma);
        ptr::drop_in_place(se.work);
        ptr::drop_in_place(se.u.ind);
        ptr::drop_in_place(se.u.vec);
    }
}