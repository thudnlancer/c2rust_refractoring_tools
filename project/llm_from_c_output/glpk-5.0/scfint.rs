use std::mem;
use std::ptr;
use std::slice;

pub struct SCF {
    type_: i32,
    n: i32,
    n0: i32,
    nn_max: i32,
    nn: i32,
    rr_ref: i32,
    ss_ref: i32,
    sva: *mut SVA,
    a0: A0,
    ifu: IFU,
    pp_ind: *mut i32,
    pp_inv: *mut i32,
    qq_ind: *mut i32,
    qq_inv: *mut i32,
}

pub union A0 {
    luf: *mut LUF,
    btf: *mut BTF,
}

pub struct IFU {
    n_max: i32,
    n: i32,
    f: *mut f64,
    u: *mut f64,
}

pub struct SCFINT {
    valid: i32,
    scf: SCF,
    u: U,
    w1: *mut f64,
    w2: *mut f64,
    w3: *mut f64,
    w4: *mut f64,
    w5: *mut f64,
    nn_max: i32,
}

pub union U {
    lufi: *mut LUFINT,
    btfi: *mut BTFINT,
}

pub fn scfint_create(type_: i32) -> *mut SCFINT {
    let fi = Box::into_raw(Box::new(SCFINT {
        valid: 0,
        scf: SCF {
            type_: type_,
            n: 0,
            n0: 0,
            nn_max: 0,
            nn: 0,
            rr_ref: 0,
            ss_ref: 0,
            sva: ptr::null_mut(),
            a0: match type_ {
                1 => A0 { luf: ptr::null_mut() },
                2 => A0 { btf: ptr::null_mut() },
                _ => panic!("Invalid type"),
            },
            ifu: IFU {
                n_max: 0,
                n: 0,
                f: ptr::null_mut(),
                u: ptr::null_mut(),
            },
            pp_ind: ptr::null_mut(),
            pp_inv: ptr::null_mut(),
            qq_ind: ptr::null_mut(),
            qq_inv: ptr::null_mut(),
        },
        u: match type_ {
            1 => U {
                lufi: lufint_create(),
            },
            2 => U {
                btfi: btfint_create(),
            },
            _ => panic!("Invalid type"),
        },
        w1: ptr::null_mut(),
        w2: ptr::null_mut(),
        w3: ptr::null_mut(),
        w4: ptr::null_mut(),
        w5: ptr::null_mut(),
        nn_max: 0,
    }));

    unsafe {
        (*fi).scf.type_ = type_;
    }

    fi
}

pub fn scfint_factorize(
    fi: *mut SCFINT,
    n: i32,
    col: Option<fn(*mut std::ffi::c_void, i32, *mut i32, *mut f64) -> i32>,
    info: *mut std::ffi::c_void,
) -> i32 {
    unsafe {
        assert!(n > 0);
        (*fi).valid = 0;

        let mut nn_max = (*fi).nn_max;
        if nn_max == 0 {
            nn_max = 100;
        }
        assert!(nn_max > 0);

        let mut old_n0_max = 0;
        let mut n0_max = 0;
        let mut ret = 0;

        match (*fi).scf.type_ {
            1 => {
                old_n0_max = (*(*fi).u.lufi).n_max;
                (*(*fi).u.lufi).sva_n_max = 4 * n + 2 * nn_max;
                ret = lufint_factorize((*fi).u.lufi, n, col, info);
                n0_max = (*(*fi).u.lufi).n_max;
                (*fi).scf.sva = (*(*fi).u.lufi).sva;
                (*fi).scf.a0.luf = (*(*fi).u.lufi).luf;
            }
            2 => {
                old_n0_max = (*(*fi).u.btfi).n_max;
                (*(*fi).u.btfi).sva_n_max = 6 * n + 2 * nn_max;
                ret = btfint_factorize((*fi).u.btfi, n, col, info);
                n0_max = (*(*fi).u.btfi).n_max;
                (*fi).scf.sva = (*(*fi).u.btfi).sva;
                (*fi).scf.a0.btf = (*(*fi).u.btfi).btf;
            }
            _ => panic!("Invalid type"),
        }

        if old_n0_max < n0_max {
            if !(*fi).w1.is_null() {
                Box::from_raw((*fi).w1);
            }
            if !(*fi).w2.is_null() {
                Box::from_raw((*fi).w2);
            }
            if !(*fi).w3.is_null() {
                Box::from_raw((*fi).w3);
            }
            (*fi).w1 = Box::into_raw(vec![0.0; (1 + n0_max) as usize].into_boxed_slice()) as *mut f64;
            (*fi).w2 = Box::into_raw(vec![0.0; (1 + n0_max) as usize].into_boxed_slice()) as *mut f64;
            (*fi).w3 = Box::into_raw(vec![0.0; (1 + n0_max) as usize].into_boxed_slice()) as *mut f64;
        }

        if (*fi).scf.nn_max != nn_max {
            if !(*fi).scf.ifu.f.is_null() {
                Box::from_raw((*fi).scf.ifu.f);
            }
            if !(*fi).scf.ifu.u.is_null() {
                Box::from_raw((*fi).scf.ifu.u);
            }
            (*fi).scf.ifu.f = Box::into_raw(vec![0.0; (nn_max * nn_max) as usize].into_boxed_slice()) as *mut f64;
            (*fi).scf.ifu.u = Box::into_raw(vec![0.0; (nn_max * nn_max) as usize].into_boxed_slice()) as *mut f64;
        }

        if old_n0_max < n0_max || (*fi).scf.nn_max != nn_max {
            if !(*fi).scf.pp_ind.is_null() {
                Box::from_raw((*fi).scf.pp_ind);
            }
            if !(*fi).scf.pp_inv.is_null() {
                Box::from_raw((*fi).scf.pp_inv);
            }
            if !(*fi).scf.qq_ind.is_null() {
                Box::from_raw((*fi).scf.qq_ind);
            }
            if !(*fi).scf.qq_inv.is_null() {
                Box::from_raw((*fi).scf.qq_inv);
            }
            if !(*fi).w4.is_null() {
                Box::from_raw((*fi).w4);
            }
            if !(*fi).w5.is_null() {
                Box::from_raw((*fi).w5);
            }
            (*fi).scf.pp_ind = Box::into_raw(vec![0; (1 + n0_max + nn_max) as usize].into_boxed_slice()) as *mut i32;
            (*fi).scf.pp_inv = Box::into_raw(vec![0; (1 + n0_max + nn_max) as usize].into_boxed_slice()) as *mut i32;
            (*fi).scf.qq_ind = Box::into_raw(vec![0; (1 + n0_max + nn_max) as usize].into_boxed_slice()) as *mut i32;
            (*fi).scf.qq_inv = Box::into_raw(vec![0; (1 + n0_max + nn_max) as usize].into_boxed_slice()) as *mut i32;
            (*fi).w4 = Box::into_raw(vec![0.0; (1 + n0_max + nn_max) as usize].into_boxed_slice()) as *mut f64;
            (*fi).w5 = Box::into_raw(vec![0.0; (1 + n0_max + nn_max) as usize].into_boxed_slice()) as *mut f64;
        }

        (*fi).scf.n = n;
        (*fi).scf.n0 = n;
        (*fi).scf.nn_max = nn_max;
        (*fi).scf.nn = 0;
        (*fi).scf.rr_ref = sva_alloc_vecs((*fi).scf.sva, nn_max);
        (*fi).scf.ss_ref = sva_alloc_vecs((*fi).scf.sva, nn_max);
        (*fi).scf.ifu.n_max = nn_max;
        (*fi).scf.ifu.n = 0;

        for k in 1..=n {
            *(*fi).scf.pp_ind.offset(k as isize) = k;
            *(*fi).scf.pp_inv.offset(k as isize) = k;
            *(*fi).scf.qq_ind.offset(k as isize) = k;
            *(*fi).scf.qq_inv.offset(k as isize) = k;
        }

        if ret == 0 {
            (*fi).valid = 1;
        }

        ret
    }
}

pub fn scfint_update(
    fi: *mut SCFINT,
    upd: i32,
    j: i32,
    len: i32,
    ind: *const i32,
    val: *const f64,
) -> i32 {
    unsafe {
        assert!((*fi).valid != 0);
        let n = (*fi).scf.n;
        let n0 = (*fi).scf.n0;
        let nn = (*fi).scf.nn;
        let pp_ind = (*fi).scf.pp_ind;
        let qq_ind = (*fi).scf.qq_ind;
        let qq_inv = (*fi).scf.qq_inv;
        let bf = (*fi).w4;
        let dg = (*fi).w5;

        assert!(0 <= n && n <= n0 + nn);

        for k in 1..=n0 + nn {
            *bf.offset(k as isize) = 0.0;
        }

        for t in 1..=len {
            let k = *ind.offset(t as isize - 1);
            assert!(1 <= k && k <= n);
            assert!(*bf.offset(k as isize) == 0.0);
            assert!(*val.offset(t as isize - 1) != 0.0);
            *bf.offset(k as isize) = *val.offset(t as isize - 1);
        }

        for k in 1..=n0 + nn {
            *dg.offset(k as isize) = 0.0;
        }
        assert!(1 <= j && j <= n);
        *dg.offset((*fi).scf.qq_inv.offset(j as isize) as isize) = 1.0;

        let ret = scf_update_aug(
            &mut (*fi).scf,
            bf,
            dg,
            bf.offset(n0 as isize),
            dg.offset(n0 as isize),
            0.0,
            upd,
            (*fi).w1,
            (*fi).w2,
            (*fi).w3,
        );

        if ret == 0 {
            scf_swap_q_cols(j, n0 + nn + 1);
        } else {
            (*fi).valid = 0;
        }

        ret
    }
}

pub fn scfint_ftran(fi: *mut SCFINT, x: *mut f64) {
    unsafe {
        assert!((*fi).valid != 0);
        scf_a_solve(
            &mut (*fi).scf,
            x,
            (*fi).w4,
            (*fi).w5,
            (*fi).w1,
            (*fi).w2,
        );
    }
}

pub fn scfint_btran(fi: *mut SCFINT, x: *mut f64) {
    unsafe {
        assert!((*fi).valid != 0);
        scf_at_solve(
            &mut (*fi).scf,
            x,
            (*fi).w4,
            (*fi).w5,
            (*fi).w1,
            (*fi).w2,
        );
    }
}

pub fn scfint_estimate(fi: *mut SCFINT) -> f64 {
    unsafe {
        assert!((*fi).valid != 0);
        assert!((*fi).scf.n == (*fi).scf.n0);

        match (*fi).scf.type_ {
            1 => luf_estimate_norm(
                (*fi).scf.a0.luf,
                (*fi).w1,
                (*fi).w2,
            ),
            2 => btf_estimate_norm(
                (*fi).scf.a0.btf,
                (*fi).w1,
                (*fi).w2,
                (*fi).w3,
                (*fi).w4,
            ),
            _ => panic!("Invalid type"),
        }
    }
}

pub fn scfint_delete(fi: *mut SCFINT) {
    unsafe {
        match (*fi).scf.type_ {
            1 => lufint_delete((*fi).u.lufi),
            2 => btfint_delete((*fi).u.btfi),
            _ => panic!("Invalid type"),
        }

        if !(*fi).scf.ifu.f.is_null() {
            Box::from_raw((*fi).scf.ifu.f);
        }
        if !(*fi).scf.ifu.u.is_null() {
            Box::from_raw((*fi).scf.ifu.u);
        }
        if !(*fi).scf.pp_ind.is_null() {
            Box::from_raw((*fi).scf.pp_ind);
        }
        if !(*fi).scf.pp_inv.is_null() {
            Box::from_raw((*fi).scf.pp_inv);
        }
        if !(*fi).scf.qq_ind.is_null() {
            Box::from_raw((*fi).scf.qq_ind);
        }
        if !(*fi).scf.qq_inv.is_null() {
            Box::from_raw((*fi).scf.qq_inv);
        }
        if !(*fi).w1.is_null() {
            Box::from_raw((*fi).w1);
        }
        if !(*fi).w2.is_null() {
            Box::from_raw((*fi).w2);
        }
        if !(*fi).w3.is_null() {
            Box::from_raw((*fi).w3);
        }
        if !(*fi).w4.is_null() {
            Box::from_raw((*fi).w4);
        }
        if !(*fi).w5.is_null() {
            Box::from_raw((*fi).w5);
        }

        Box::from_raw(fi);
    }
}