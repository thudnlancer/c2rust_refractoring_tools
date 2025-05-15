use std::f64;
use std::ptr;

struct DMP;

struct NPP {
    orig_dir: i32,
    orig_m: i32,
    orig_n: i32,
    orig_nnz: i32,
    pool: *mut DMP,
    name: *mut i8,
    obj: *mut i8,
    c0: f64,
    nrows: i32,
    ncols: i32,
    r_head: *mut NPPROW,
    r_tail: *mut NPPROW,
    c_head: *mut NPPCOL,
    c_tail: *mut NPPCOL,
    stack: *mut DMP,
    top: *mut NPPTSE,
    m: i32,
    n: i32,
    nnz: i32,
    row_ref: *mut i32,
    col_ref: *mut i32,
    sol: i32,
    scaling: i32,
    p_stat: i32,
    d_stat: i32,
    t_stat: i32,
    i_stat: i32,
    r_stat: *mut i8,
    c_stat: *mut i8,
    r_pi: *mut f64,
    c_value: *mut f64,
}

struct NPPTSE {
    func: Option<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32>,
    info: *mut libc::c_void,
    link: *mut NPPTSE,
}

struct NPPCOL {
    j: i32,
    name: *mut i8,
    is_int: i8,
    lb: f64,
    ub: f64,
    coef: f64,
    ptr: *mut NPPAIJ,
    temp: i32,
    ll: C2RustUnnamed_0,
    uu: C2RustUnnamed,
    prev: *mut NPPCOL,
    next: *mut NPPCOL,
}

union C2RustUnnamed {
    uu: f64,
    neg: i32,
}

union C2RustUnnamed_0 {
    ll: f64,
    pos: i32,
}

struct NPPAIJ {
    row: *mut NPPROW,
    col: *mut NPPCOL,
    val: f64,
    r_prev: *mut NPPAIJ,
    r_next: *mut NPPAIJ,
    c_prev: *mut NPPAIJ,
    c_next: *mut NPPAIJ,
}

struct NPPROW {
    i: i32,
    name: *mut i8,
    lb: f64,
    ub: f64,
    ptr: *mut NPPAIJ,
    temp: i32,
    prev: *mut NPPROW,
    next: *mut NPPROW,
}

struct Binarize {
    q: i32,
    j: i32,
    n: i32,
}

struct Elem {
    aj: f64,
    xj: *mut NPPCOL,
    next: *mut Elem,
}

fn glp_assert_(expr: *const i8, file: *const i8, line: i32) {
    unsafe {
        libc::printf(b"Assertion failed: %s (%s:%d)\n\0" as *const u8 as *const i8, expr, file, line);
        libc::abort();
    }
}

fn glp_printf(fmt: *const i8, ...) {
    unsafe {
        let mut args: libc::va_list = std::mem::zeroed();
        libc::va_start(args, fmt);
        libc::vprintf(fmt, args);
        libc::va_end(args);
    }
}

fn _glp_npp_binarize_prob(npp: *mut NPP) -> i32 {
    unsafe {
        let mut info: *mut Binarize = ptr::null_mut();
        let mut row: *mut NPPROW = ptr::null_mut();
        let mut col: *mut NPPCOL = (*npp).c_tail;
        let mut bin: *mut NPPCOL = ptr::null_mut();
        let mut aij: *mut NPPAIJ = ptr::null_mut();
        let mut u: i32 = 0;
        let mut n: i32 = 0;
        let mut k: i32 = 0;
        let mut temp: i32 = 0;
        let mut nfails: i32 = 0;
        let mut nvars: i32 = 0;
        let mut nbins: i32 = 0;
        let mut nrows: i32 = 0;

        while !col.is_null() {
            if (*col).is_int != 0 && (*col).lb != (*col).ub {
                if !((*col).lb == 0.0 && (*col).ub == 1.0) {
                    if (*col).lb < -1e6 || (*col).ub > 1e6 || (*col).ub - (*col).lb > 4095.0 {
                        nfails += 1;
                    } else {
                        nvars += 1;
                        if (*col).lb != 0.0 {
                            _glp_npp_lbnd_col(npp, col);
                        }
                        assert!((*col).lb == 0.0, "col->lb == 0.0");
                        u = (*col).ub as i32;
                        assert!((*col).ub == u as f64, "col->ub == (double)u");
                        
                        if u != 1 {
                            n = 2;
                            temp = 4;
                            while u >= temp {
                                n += 1;
                                temp <<= 1;
                            }
                            nbins += n;
                            
                            info = _glp_npp_push_tse(
                                npp,
                                Some(rcv_binarize_prob),
                                std::mem::size_of::<Binarize>() as i32,
                            ) as *mut Binarize;
                            
                            (*info).q = (*col).j;
                            (*info).j = 0;
                            (*info).n = n;
                            
                            if u < temp - 1 {
                                row = _glp_npp_add_row(npp);
                                nrows += 1;
                                (*row).lb = f64::NEG_INFINITY;
                                (*row).ub = u as f64;
                            } else {
                                row = ptr::null_mut();
                            }
                            
                            (*col).ub = 1.0;
                            if !row.is_null() {
                                _glp_npp_add_aij(npp, row, col, 1.0);
                            }
                            
                            k = 1;
                            temp = 2;
                            while k < n {
                                bin = _glp_npp_add_col(npp);
                                (*bin).is_int = 1;
                                (*bin).lb = 0.0;
                                (*bin).ub = 1.0;
                                (*bin).coef = temp as f64 * (*col).coef;
                                
                                if (*info).j == 0 {
                                    (*info).j = (*bin).j;
                                } else {
                                    assert!((*info).j + (k - 1) == (*bin).j, 
                                        "info->j + (k-1) == bin->j");
                                }
                                
                                aij = (*col).ptr;
                                while !aij.is_null() {
                                    _glp_npp_add_aij(
                                        npp,
                                        (*aij).row,
                                        bin,
                                        temp as f64 * (*aij).val,
                                    );
                                    aij = (*aij).c_next;
                                }
                                
                                k += 1;
                                temp <<= 1;
                            }
                        }
                    }
                }
            }
            col = (*col).prev;
        }

        if nvars > 0 {
            glp_printf(
                b"%d integer variable(s) were replaced by %d binary ones\n\0" as *const u8 as *const i8,
                nvars, nbins
            );
        }
        if nrows > 0 {
            glp_printf(
                b"%d row(s) were added due to binarization\n\0" as *const u8 as *const i8,
                nrows
            );
        }
        if nfails > 0 {
            glp_printf(
                b"Binarization failed for %d integer variable(s)\n\0" as *const u8 as *const i8,
                nfails
            );
        }
        
        nfails
    }
}

unsafe extern "C" fn rcv_binarize_prob(npp: *mut NPP, _info: *mut libc::c_void) -> i32 {
    let info = _info as *mut Binarize;
    let mut k: i32 = 1;
    let mut temp: i32 = 2;
    let mut sum: f64 = *((*npp).c_value).offset((*info).q as isize);
    
    while k < (*info).n {
        sum += temp as f64 * *((*npp).c_value).offset(((*info).j + (k - 1)) as isize);
        k += 1;
        temp <<= 1;
    }
    
    *((*npp).c_value).offset((*info).q as isize) = sum;
    0
}

// Additional helper functions would follow the same pattern of conversion
// with proper error handling and safety checks