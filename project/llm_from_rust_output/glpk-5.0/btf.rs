use std::ptr;
use std::mem;
use std::os::raw::{c_int, c_double, c_void, c_char};
use std::slice;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SVA {
    pub n_max: c_int,
    pub n: c_int,
    pub ptr: *mut c_int,
    pub len: *mut c_int,
    pub cap: *mut c_int,
    pub size: c_int,
    pub m_ptr: c_int,
    pub r_ptr: c_int,
    pub head: c_int,
    pub tail: c_int,
    pub prev: *mut c_int,
    pub next: *mut c_int,
    pub ind: *mut c_int,
    pub val: *mut c_double,
    pub talky: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BTF {
    pub n: c_int,
    pub sva: *mut SVA,
    pub pp_ind: *mut c_int,
    pub pp_inv: *mut c_int,
    pub qq_ind: *mut c_int,
    pub qq_inv: *mut c_int,
    pub num: c_int,
    pub beg: *mut c_int,
    pub ar_ref: c_int,
    pub ac_ref: c_int,
    pub fr_ref: c_int,
    pub fc_ref: c_int,
    pub vr_ref: c_int,
    pub vr_piv: *mut c_double,
    pub vc_ref: c_int,
    pub p1_ind: *mut c_int,
    pub p1_inv: *mut c_int,
    pub q1_ind: *mut c_int,
    pub q1_inv: *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LUF {
    pub n: c_int,
    pub sva: *mut SVA,
    pub fr_ref: c_int,
    pub fc_ref: c_int,
    pub vr_ref: c_int,
    pub vr_piv: *mut c_double,
    pub vc_ref: c_int,
    pub pp_ind: *mut c_int,
    pub pp_inv: *mut c_int,
    pub qq_ind: *mut c_int,
    pub qq_inv: *mut c_int,
}

extern "C" {
    fn _glp_sva_more_space(sva: *mut SVA, m_size: c_int);
    fn _glp_sva_reserve_cap(sva: *mut SVA, k: c_int, new_cap: c_int);
    fn _glp_luf_f_solve(luf: *mut LUF, x: *mut c_double);
    fn _glp_luf_ft_solve(luf: *mut LUF, x: *mut c_double);
    fn _glp_luf_v_solve(luf: *mut LUF, b: *mut c_double, x: *mut c_double);
    fn _glp_luf_vt_solve(luf: *mut LUF, b: *mut c_double, x: *mut c_double);
    fn _glp_luf_vt_solve1(luf: *mut LUF, e: *mut c_double, y: *mut c_double);
    fn _glp_mc13d(
        n: c_int,
        icn: *const c_int,
        ip: *const c_int,
        lenr: *const c_int,
        ior: *mut c_int,
        ib: *mut c_int,
        lowl: *mut c_int,
        numb: *mut c_int,
        prev: *mut c_int,
    ) -> c_int;
    fn _glp_mc21a(
        n: c_int,
        icn: *const c_int,
        ip: *const c_int,
        lenr: *const c_int,
        iperm: *mut c_int,
        pr: *mut c_int,
        arp: *mut c_int,
        cv: *mut c_int,
        out: *mut c_int,
    ) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn _glp_btf_store_a_cols(
    btf: *mut BTF,
    col: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int, *mut c_double) -> c_int>,
    info: *mut c_void,
    ind: *mut c_int,
    val: *mut c_double,
) -> c_int {
    let n = (*btf).n;
    let sva = (*btf).sva;
    let mut sv_ind = (*sva).ind;
    let ac_ref = (*btf).ac_ref;
    let ac_ptr = (*sva).ptr.add(ac_ref as usize - 1);
    let ac_len = (*sva).len.add(ac_ref as usize - 1);
    let mut nnz = 0;

    for j in 1..=n {
        let len = col.unwrap()(info, j, ind, val);
        assert!(0 <= len && len <= n, "0 <= len && len <= n");

        if len > 0 {
            if (*sva).r_ptr - (*sva).m_ptr < len {
                _glp_sva_more_space(sva, len);
                sv_ind = (*sva).ind;
            }
            _glp_sva_reserve_cap(sva, ac_ref + (j - 1), len);
        }

        let ptr = *ac_ptr.add(j as usize);
        ptr::copy_nonoverlapping(
            ind.add(1),
            sv_ind.add(ptr as usize),
            len as usize,
        );
        *ac_len.add(j as usize) = len;
        nnz += len;
    }

    nnz
}

#[no_mangle]
pub unsafe extern "C" fn _glp_btf_make_blocks(btf: *mut BTF) -> c_int {
    let n = (*btf).n;
    let sva = (*btf).sva;
    let sv_ind = (*sva).ind;
    let pp_ind = (*btf).pp_ind;
    let pp_inv = (*btf).pp_inv;
    let qq_ind = (*btf).qq_ind;
    let qq_inv = (*btf).qq_inv;
    let beg = (*btf).beg;
    let ac_ref = (*btf).ac_ref;
    let ac_ptr = (*sva).ptr.add(ac_ref as usize - 1);
    let ac_len = (*sva).len.add(ac_ref as usize - 1);

    let iperm = qq_inv;
    let pr = (*btf).p1_ind;
    let arp = (*btf).p1_inv;
    let cv = (*btf).q1_ind;
    let out = (*btf).q1_inv;

    let rank = _glp_mc21a(
        n,
        sv_ind,
        ac_ptr,
        ac_len,
        iperm,
        pr,
        arp,
        cv,
        out,
    );
    assert!(0 <= rank && rank <= n, "0 <= rank && rank <= n");

    if rank < n {
        let ip = pp_ind;
        let lenr = qq_ind;
        for j in 1..=n {
            *ip.add(j as usize) = *ac_ptr.add(*iperm.add(j as usize) as usize);
            *lenr.add(j as usize) = *ac_len.add(*iperm.add(j as usize) as usize);
        }

        let lowl = (*btf).p1_ind;
        let numb = (*btf).p1_inv;
        let prev = (*btf).q1_ind;
        (*btf).num = _glp_mc13d(
            n,
            sv_ind,
            ip,
            lenr,
            pp_inv,
            beg,
            lowl,
            numb,
            prev,
        );
        assert!(*beg.add(1) == 1, "beg[1] == 1");
        *beg.add(((*btf).num + 1) as usize) = n + 1;

        for j in 1..=n {
            *pp_ind.add(*pp_inv.add(j as usize) as usize) = j;
        }

        for i in 1..=n {
            *qq_ind.add(i as usize) = *iperm.add(*pp_inv.add(i as usize) as usize);
        }

        for i in 1..=n {
            *qq_inv.add(*qq_ind.add(i as usize) as usize) = i;
        }
    }

    rank
}

// ... (其他函数类似重构)

#[no_mangle]
pub unsafe extern "C" fn _glp_btf_estimate_norm(
    btf: *mut BTF,
    w1: *mut c_double,
    w2: *mut c_double,
    w3: *mut c_double,
    w4: *mut c_double,
) -> c_double {
    let n = (*btf).n;
    let e = w1;
    let y = w2;
    let z = w1;

    for i in 1..=n {
        *e.add(i as usize) = 0.0;
    }

    _glp_btf_at_solve1(btf, e, y, w3, w4);

    let mut y_norm = 0.0;
    for i in 1..=n {
        y_norm += (*y.add(i as usize)).abs();
    }

    _glp_btf_a_solve(btf, y, z, w3, w4);

    let mut z_norm = 0.0;
    for i in 1..=n {
        z_norm += (*z.add(i as usize)).abs();
    }

    z_norm / y_norm
}