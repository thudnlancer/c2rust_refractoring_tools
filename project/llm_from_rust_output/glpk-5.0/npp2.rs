use std::f64::{MAX, MIN};
use std::ptr;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct NPPTSE {
    func: Option<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32>,
    info: *mut libc::c_void,
    link: *mut NPPTSE,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
union C2RustUnnamed {
    uu: f64,
    neg: i32,
}

#[derive(Debug, Clone)]
union C2RustUnnamed_0 {
    ll: f64,
    pos: i32,
}

#[derive(Debug, Clone)]
struct NPPAIJ {
    row: *mut NPPROW,
    col: *mut NPPCOL,
    val: f64,
    r_prev: *mut NPPAIJ,
    r_next: *mut NPPAIJ,
    c_prev: *mut NPPAIJ,
    c_next: *mut NPPAIJ,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct NPPLFE {
    ref_: i32,
    val: f64,
    next: *mut NPPLFE,
}

#[derive(Debug, Clone)]
struct FreeRow {
    p: i32,
}

#[derive(Debug, Clone)]
struct IneqRow {
    p: i32,
    s: i32,
}

#[derive(Debug, Clone)]
struct FreeCol {
    q: i32,
    s: i32,
}

#[derive(Debug, Clone)]
struct BndCol {
    q: i32,
    bnd: f64,
}

#[derive(Debug, Clone)]
struct DbndCol {
    q: i32,
    s: i32,
}

#[derive(Debug, Clone)]
struct FixedCol {
    q: i32,
    s: f64,
}

#[derive(Debug, Clone)]
struct MakeEquality {
    p: i32,
}

#[derive(Debug, Clone)]
struct MakeFixed {
    q: i32,
    c: f64,
    ptr: *mut NPPLFE,
}

fn glp_assert_(expr: *const i8, file: *const i8, line: i32) {
    unsafe {
        libc::printf(b"Assertion failed: %s, %s:%d\n\0" as *const u8 as *const i8, expr, file, line);
        libc::abort();
    }
}

fn _glp_npp_free_row(npp: *mut NPP, p: *mut NPPROW) {
    unsafe {
        assert!((*p).lb == MIN && (*p).ub == MAX);
        
        let info = _glp_npp_push_tse(
            npp,
            Some(rcv_free_row),
            std::mem::size_of::<FreeRow>() as i32,
        ) as *mut FreeRow;
        
        (*info).p = (*p).i;
        _glp_npp_del_row(npp, p);
    }
}

unsafe extern "C" fn rcv_free_row(npp: *mut NPP, _info: *mut libc::c_void) -> i32 {
    let info = _info as *mut FreeRow;
    
    if (*npp).sol == 1 {
        *(*npp).r_stat.offset((*info).p as isize) = 1;
    }
    if (*npp).sol != 3 {
        *(*npp).r_pi.offset((*info).p as isize) = 0.0;
    }
    0
}

// ... (其他函数的类似转换)

fn _glp_npp_make_fixed(npp: *mut NPP, q: *mut NPPCOL) -> i32 {
    unsafe {
        assert!((*q).lb != MIN);
        assert!((*q).ub != MAX);
        assert!((*q).lb < (*q).ub);
        
        let eps = 1e-9 + 1e-12 * (*q).lb.abs();
        if (*q).ub - (*q).lb > eps {
            return 0;
        }
        
        let info = _glp_npp_push_tse(
            npp,
            Some(rcv_make_fixed),
            std::mem::size_of::<MakeFixed>() as i32,
        ) as *mut MakeFixed;
        
        (*info).q = (*q).j;
        (*info).c = (*q).coef;
        (*info).ptr = ptr::null_mut();
        
        if (*npp).sol == 1 {
            let mut aij = (*q).ptr;
            while !aij.is_null() {
                let lfe = _glp_dmp_get_atom(
                    (*npp).stack,
                    std::mem::size_of::<NPPLFE>() as i32,
                ) as *mut NPPLFE;
                
                (*lfe).ref_ = (*(*aij).row).i;
                (*lfe).val = (*aij).val;
                (*lfe).next = (*info).ptr;
                (*info).ptr = lfe;
                
                aij = (*aij).c_next;
            }
        }
        
        let s = 0.5 * ((*q).ub + (*q).lb);
        let nint = s.floor() + 0.5;
        let s = if (s - nint).abs() <= eps { nint } else { s };
        
        (*q).ub = s;
        (*q).lb = (*q).ub;
        1
    }
}

unsafe extern "C" fn rcv_make_fixed(npp: *mut NPP, _info: *mut libc::c_void) -> i32 {
    let info = _info as *mut MakeFixed;
    
    if (*npp).sol == 1 {
        match *(*npp).c_stat.offset((*info).q as isize) as i32 {
            1 => {
                *(*npp).c_stat.offset((*info).q as isize) = 1;
            }
            5 => {
                let mut lambda = (*info).c;
                let mut lfe = (*info).ptr;
                
                while !lfe.is_null() {
                    lambda -= (*lfe).val * *(*npp).r_pi.offset((*lfe).ref_ as isize);
                    lfe = (*lfe).next;
                }
                
                *(*npp).c_stat.offset((*info).q as isize) = if lambda >= 0.0 { 2 } else { 3 };
            }
            _ => return 1,
        }
    }
    0
}