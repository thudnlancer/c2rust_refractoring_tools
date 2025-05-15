use std::ptr;
use std::mem;
use std::cmp;
use std::os::raw::{c_int, c_uint, c_void};

type mpq_t = *mut mpq;
type glp_errfunc = Option<unsafe extern "C" fn(*const libc::c_char, ...)>;

#[repr(C)]
struct mpz {
    val: c_int,
    ptr: *mut mpz_seg,
}

#[repr(C)]
struct mpz_seg {
    d: [libc::c_ushort; 6],
    next: *mut mpz_seg,
}

#[repr(C)]
struct mpq {
    p: mpz,
    q: mpz,
}

#[repr(C)]
struct LUXELM {
    i: c_int,
    j: c_int,
    val: mpq_t,
    r_prev: *mut LUXELM,
    r_next: *mut LUXELM,
    c_prev: *mut LUXELM,
    c_next: *mut LUXELM,
}

#[repr(C)]
struct LUX {
    n: c_int,
    pool: *mut DMP,
    F_row: *mut *mut LUXELM,
    F_col: *mut *mut LUXELM,
    V_piv: *mut mpq_t,
    V_row: *mut *mut LUXELM,
    V_col: *mut *mut LUXELM,
    P_row: *mut c_int,
    P_col: *mut c_int,
    Q_row: *mut c_int,
    Q_col: *mut c_int,
    rank: c_int,
}

#[repr(C)]
struct LUXWKA {
    R_len: *mut c_int,
    R_head: *mut c_int,
    R_prev: *mut c_int,
    R_next: *mut c_int,
    C_len: *mut c_int,
    C_head: *mut c_int,
    C_prev: *mut c_int,
    C_next: *mut c_int,
}

extern "C" {
    fn glp_free(ptr: *mut c_void);
    fn glp_alloc(n: c_int, size: c_int) -> *mut c_void;
    fn glp_assert_(expr: *const libc::c_char, file: *const libc::c_char, line: c_int);
    fn glp_error_(file: *const libc::c_char, line: c_int) -> glp_errfunc;
    fn _glp_mpq_init() -> mpq_t;
    fn _glp_mpq_clear(x: mpq_t);
    fn _glp_mpq_set(z: mpq_t, x: mpq_t);
    fn _glp_mpq_set_si(x: mpq_t, p: c_int, q: c_uint);
    fn _glp_mpq_sub(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_mul(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_div(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_neg(z: mpq_t, x: mpq_t);
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut c_void, size: c_int);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: c_int) -> *mut c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_mpq_sgn(x: mpq_t) -> c_int;
}

pub unsafe fn _glp_lux_create(n: c_int) -> *mut LUX {
    if n < 1 {
        glp_error_(b"draft/lux.c\0".as_ptr(), 51).expect("non-null function pointer")(
            b"lux_create: n = %d; invalid parameter\n\0".as_ptr(),
            n,
        );
    }

    let lux = glp_alloc(1, mem::size_of::<LUX>() as c_int) as *mut LUX;
    (*lux).n = n;
    (*lux).pool = _glp_dmp_create_pool();
    (*lux).F_row = glp_alloc(1 + n, mem::size_of::<*mut LUXELM>() as c_int) as *mut *mut LUXELM;
    (*lux).F_col = glp_alloc(1 + n, mem::size_of::<*mut LUXELM>() as c_int) as *mut *mut LUXELM;
    (*lux).V_piv = glp_alloc(1 + n, mem::size_of::<mpq_t>() as c_int) as *mut mpq_t;
    (*lux).V_row = glp_alloc(1 + n, mem::size_of::<*mut LUXELM>() as c_int) as *mut *mut LUXELM;
    (*lux).V_col = glp_alloc(1 + n, mem::size_of::<*mut LUXELM>() as c_int) as *mut *mut LUXELM;
    (*lux).P_row = glp_alloc(1 + n, mem::size_of::<c_int>() as c_int) as *mut c_int;
    (*lux).P_col = glp_alloc(1 + n, mem::size_of::<c_int>() as c_int) as *mut c_int;
    (*lux).Q_row = glp_alloc(1 + n, mem::size_of::<c_int>() as c_int) as *mut c_int;
    (*lux).Q_col = glp_alloc(1 + n, mem::size_of::<c_int>() as c_int) as *mut c_int;

    for k in 1..=n {
        *(*lux).F_col.add(k as usize) = ptr::null_mut();
        *(*lux).F_row.add(k as usize) = *(*lux).F_col.add(k as usize);
        *(*lux).V_piv.add(k as usize) = _glp_mpq_init();
        _glp_mpq_set_si(*(*lux).V_piv.add(k as usize), 1, 1);
        *(*lux).V_col.add(k as usize) = ptr::null_mut();
        *(*lux).V_row.add(k as usize) = *(*lux).V_col.add(k as usize);
        *(*lux).P_col.add(k as usize) = k;
        *(*lux).P_row.add(k as usize) = *(*lux).P_col.add(k as usize);
        *(*lux).Q_col.add(k as usize) = k;
        *(*lux).Q_row.add(k as usize) = *(*lux).Q_col.add(k as usize);
    }

    (*lux).rank = n;
    lux
}

// Remaining functions would follow similar patterns of conversion, 
// replacing raw pointers with safer alternatives where possible,
// using Rust's ownership system, and minimizing unsafe blocks.
// However, due to the extensive use of FFI and pointer manipulation,
// some unsafe blocks would still be necessary for interoperability.