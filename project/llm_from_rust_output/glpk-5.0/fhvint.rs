use std::ptr;
use std::mem;
use std::ffi::CString;
use std::os::raw::{c_int, c_double, c_void, c_char};

struct SVA {
    n_max: c_int,
    n: c_int,
    ptr: *mut c_int,
    len: *mut c_int,
    cap: *mut c_int,
    size: c_int,
    m_ptr: c_int,
    r_ptr: c_int,
    head: c_int,
    tail: c_int,
    prev: *mut c_int,
    next: *mut c_int,
    ind: *mut c_int,
    val: *mut c_double,
    talky: c_int,
}

struct LUF {
    n: c_int,
    sva: *mut SVA,
    fr_ref: c_int,
    fc_ref: c_int,
    vr_ref: c_int,
    vr_piv: *mut c_double,
    vc_ref: c_int,
    pp_ind: *mut c_int,
    pp_inv: *mut c_int,
    qq_ind: *mut c_int,
    qq_inv: *mut c_int,
}

struct FHV {
    luf: *mut LUF,
    nfs_max: c_int,
    nfs: c_int,
    hh_ind: *mut c_int,
    hh_ref: c_int,
    p0_ind: *mut c_int,
    p0_inv: *mut c_int,
}

struct SGF {
    luf: *mut LUF,
    rs_head: *mut c_int,
    rs_prev: *mut c_int,
    rs_next: *mut c_int,
    cs_head: *mut c_int,
    cs_prev: *mut c_int,
    cs_next: *mut c_int,
    vr_max: *mut c_double,
    flag: *mut c_char,
    work: *mut c_double,
    updat: c_int,
    piv_tol: c_double,
    piv_lim: c_int,
    suhl: c_int,
    eps_tol: c_double,
}

struct LUFINT {
    n_max: c_int,
    valid: c_int,
    sva: *mut SVA,
    luf: *mut LUF,
    sgf: *mut SGF,
    sva_n_max: c_int,
    sva_size: c_int,
    delta_n0: c_int,
    delta_n: c_int,
    sgf_updat: c_int,
    sgf_piv_tol: c_double,
    sgf_piv_lim: c_int,
    sgf_suhl: c_int,
    sgf_eps_tol: c_double,
}

struct FHVINT {
    valid: c_int,
    fhv: FHV,
    lufi: *mut LUFINT,
    nfs_max: c_int,
}

fn glp_assert(expr: &str, file: &str, line: c_int) {
    assert!(expr.is_empty(), "{}:{}: assertion failed: {}", file, line, expr);
}

fn glp_alloc(n: c_int, size: c_int) -> *mut c_void {
    unsafe { libc::malloc((n * size) as libc::size_t) }
}

fn glp_free(ptr: *mut c_void) {
    unsafe { libc::free(ptr) }
}

fn _glp_fhvint_create() -> *mut FHVINT {
    let fi = unsafe {
        let ptr = glp_alloc(1, mem::size_of::<FHVINT>() as c_int) as *mut FHVINT;
        libc::memset(ptr as *mut c_void, 0, mem::size_of::<FHVINT>());
        (*ptr).lufi = _glp_lufint_create();
        ptr
    };
    fi
}

fn _glp_fhvint_factorize(
    fi: *mut FHVINT,
    n: c_int,
    col: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int, *mut c_double) -> c_int>,
    info: *mut c_void,
) -> c_int {
    unsafe {
        assert!(n > 0, "n must be positive");
        (*fi).valid = 0;

        let mut nfs_max = (*fi).nfs_max;
        if nfs_max == 0 {
            nfs_max = 100;
        }
        assert!(nfs_max > 0, "nfs_max must be positive");

        let old_n_max = (*(*fi).lufi).n_max;
        (*(*fi).lufi).sva_n_max = 4 * n + nfs_max;
        (*(*fi).lufi).sgf_updat = 1;

        let ret = _glp_lufint_factorize((*fi).lufi, n, col, info);
        let n_max = (*(*fi).lufi).n_max;

        if (*fi).fhv.nfs_max != nfs_max {
            if !(*fi).fhv.hh_ind.is_null() {
                glp_free((*fi).fhv.hh_ind as *mut c_void);
            }
            (*fi).fhv.hh_ind = glp_alloc(1 + nfs_max, mem::size_of::<c_int>() as c_int) as *mut c_int;
        }

        if old_n_max < n_max {
            if !(*fi).fhv.p0_ind.is_null() {
                glp_free((*fi).fhv.p0_ind as *mut c_void);
            }
            if !(*fi).fhv.p0_inv.is_null() {
                glp_free((*fi).fhv.p0_inv as *mut c_void);
            }
            (*fi).fhv.p0_ind = glp_alloc(1 + n_max, mem::size_of::<c_int>() as c_int) as *mut c_int;
            (*fi).fhv.p0_inv = glp_alloc(1 + n_max, mem::size_of::<c_int>() as c_int) as *mut c_int;
        }

        (*fi).fhv.luf = (*(*fi).lufi).luf;
        (*fi).fhv.nfs_max = nfs_max;
        (*fi).fhv.nfs = 0;
        (*fi).fhv.hh_ref = _glp_sva_alloc_vecs((*(*fi).lufi).sva, nfs_max);

        for k in 1..=n {
            *(*fi).fhv.p0_ind.offset(k as isize) = *(*(*fi).fhv.luf).pp_ind.offset(k as isize);
            *(*fi).fhv.p0_inv.offset(k as isize) = *(*(*fi).fhv.luf).pp_inv.offset(k as isize);
        }

        if ret == 0 {
            (*fi).valid = 1;
        }
        ret
    }
}

fn _glp_fhvint_delete(fi: *mut FHVINT) {
    unsafe {
        _glp_lufint_delete((*fi).lufi);
        if !(*fi).fhv.hh_ind.is_null() {
            glp_free((*fi).fhv.hh_ind as *mut c_void);
        }
        if !(*fi).fhv.p0_ind.is_null() {
            glp_free((*fi).fhv.p0_ind as *mut c_void);
        }
        if !(*fi).fhv.p0_inv.is_null() {
            glp_free((*fi).fhv.p0_inv as *mut c_void);
        }
        glp_free(fi as *mut c_void);
    }
}