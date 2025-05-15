use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct mpz_seg {
    d: [c_ushort; 6],
    next: *mut mpz_seg,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct mpz {
    val: c_int,
    ptr: *mut mpz_seg,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct mpq {
    p: mpz,
    q: mpz,
}

type mpq_t = *mut mpq;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct GLPAIJ {
    row: *mut GLPROW,
    col: *mut GLPCOL,
    val: c_double,
    r_prev: *mut GLPAIJ,
    r_next: *mut GLPAIJ,
    c_prev: *mut GLPAIJ,
    c_next: *mut GLPAIJ,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct GLPROW {
    i: c_int,
    name: *mut c_char,
    node: *mut AVLNODE,
    level: c_int,
    origin: u8,
    klass: u8,
    type_: c_int,
    lb: c_double,
    ub: c_double,
    ptr: *mut GLPAIJ,
    rii: c_double,
    stat: c_int,
    bind: c_int,
    prim: c_double,
    dual: c_double,
    pval: c_double,
    dval: c_double,
    mipx: c_double,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct GLPCOL {
    j: c_int,
    name: *mut c_char,
    node: *mut AVLNODE,
    kind: c_int,
    type_: c_int,
    lb: c_double,
    ub: c_double,
    coef: c_double,
    ptr: *mut GLPAIJ,
    sjj: c_double,
    stat: c_int,
    bind: c_int,
    prim: c_double,
    dual: c_double,
    pval: c_double,
    dval: c_double,
    mipx: c_double,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct glp_prob {
    pool: *mut DMP,
    tree: *mut glp_tree,
    name: *mut c_char,
    obj: *mut c_char,
    dir: c_int,
    c0: c_double,
    m_max: c_int,
    n_max: c_int,
    m: c_int,
    n: c_int,
    nnz: c_int,
    row: *mut *mut GLPROW,
    col: *mut *mut GLPCOL,
    r_tree: *mut AVL,
    c_tree: *mut AVL,
    valid: c_int,
    head: *mut c_int,
    bfd: *mut BFD,
    pbs_stat: c_int,
    dbs_stat: c_int,
    obj_val: c_double,
    it_cnt: c_int,
    some: c_int,
    ipt_stat: c_int,
    ipt_obj: c_double,
    mip_stat: c_int,
    mip_obj: c_double,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct glp_smcp {
    msg_lev: c_int,
    meth: c_int,
    pricing: c_int,
    r_test: c_int,
    tol_bnd: c_double,
    tol_dj: c_double,
    tol_piv: c_double,
    obj_ll: c_double,
    obj_ul: c_double,
    it_lim: c_int,
    tm_lim: c_int,
    out_frq: c_int,
    out_dly: c_int,
    presolve: c_int,
    excl: c_int,
    shift: c_int,
    aorn: c_int,
    foo_bar: [c_double; 33],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct SSX {
    m: c_int,
    n: c_int,
    type_: *mut c_int,
    lb: *mut mpq_t,
    ub: *mut mpq_t,
    dir: c_int,
    coef: *mut mpq_t,
    A_ptr: *mut c_int,
    A_ind: *mut c_int,
    A_val: *mut mpq_t,
    stat: *mut c_int,
    Q_row: *mut c_int,
    Q_col: *mut c_int,
    binv: *mut BFX,
    bbar: *mut mpq_t,
    pi: *mut mpq_t,
    cbar: *mut mpq_t,
    p: c_int,
    rho: *mut mpq_t,
    ap: *mut mpq_t,
    q: c_int,
    aq: *mut mpq_t,
    q_dir: c_int,
    p_stat: c_int,
    delta: mpq_t,
    msg_lev: c_int,
    it_lim: c_int,
    it_cnt: c_int,
    tm_lim: c_double,
    out_frq: c_double,
    tm_beg: c_double,
    tm_lag: c_double,
}

type BFD = c_void;
type AVL = c_void;
type AVLNODE = c_void;
type glp_tree = c_void;
type DMP = c_void;
type BFX = c_void;

extern "C" {
    fn glp_get_mat_col(
        P: *mut glp_prob,
        j: c_int,
        ind: *mut c_int,
        val: *mut c_double,
    ) -> c_int;
    fn glp_set_row_stat(P: *mut glp_prob, i: c_int, stat: c_int);
    fn glp_set_col_stat(P: *mut glp_prob, j: c_int, stat: c_int);
    fn glp_init_smcp(parm: *mut glp_smcp);
    fn glp_printf(fmt: *const c_char, ...);
    fn glp_error_(file: *const c_char, line: c_int) -> Option<unsafe extern "C" fn(*const c_char, ...)>;
    fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int);
    fn glp_alloc(n: c_int, size: c_int) -> *mut c_void;
    fn glp_free(ptr: *mut c_void);
    fn glp_time() -> c_double;
    fn frexp(_: c_double, _: *mut c_int) -> c_double;
    fn fabs(_: c_double) -> c_double;
    fn floor(_: c_double) -> c_double;
    fn _glp_ssx_create(m: c_int, n: c_int, nnz: c_int) -> *mut SSX;
    fn _glp_mpq_init() -> mpq_t;
    fn _glp_mpq_set_d(x: mpq_t, val: c_double);
    fn _glp_mpq_set_si(x: mpq_t, p: c_int, q: c_uint);
    fn abs(_: c_int) -> c_int;
    fn _glp_mpq_add(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_mul(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_div(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_clear(x: mpq_t);
    fn _glp_mpq_neg(z: mpq_t, x: mpq_t);
    fn _glp_ssx_driver(ssx: *mut SSX) -> c_int;
    fn _glp_mpq_get_d(x: mpq_t) -> c_double;
    fn _glp_ssx_delete(ssx: *mut SSX);
    fn _glp_gmp_pool_count() -> c_int;
    fn _glp_gmp_free_mem();
    fn _glp_fp2rat(
        x: c_double,
        eps: c_double,
        p: *mut c_double,
        q: *mut c_double,
    ) -> c_int;
}

unsafe fn set_d_eps(x: mpq_t, val: c_double) {
    let mut s: c_int = 0;
    let mut n: c_int = 0;
    let mut j: c_int = 0;
    let mut f: c_double = 0.;
    let mut p: c_double = 0.;
    let mut q: c_double = 0.;
    let eps: c_double = 1e-9;
    let mut temp: mpq_t = ptr::null_mut();

    assert!(
        -1.7976931348623157e+308 <= val && val <= 1.7976931348623157e+308,
        "-DBL_MAX <= val && val <= +DBL_MAX"
    );

    if val == floor(val) {
        _glp_mpq_set_d(x, val);
    } else {
        if val > 0.0 {
            s = 1;
        } else if val < 0.0 {
            s = -1;
        } else {
            _glp_mpq_set_si(x, 0, 1);
            return;
        }

        f = frexp(fabs(val), &mut n);
        _glp_fp2rat(f, 0.1 * eps, &mut p, &mut q);
        temp = _glp_mpq_init();
        _glp_mpq_set_d(x, p);
        _glp_mpq_set_d(temp, q);
        _glp_mpq_div(x, x, temp);
        _glp_mpq_set_si(temp, 1, 1);
        j = 1;
        while j <= abs(n) {
            _glp_mpq_add(temp, temp, temp);
            j += 1;
        }
        if n > 0 {
            _glp_mpq_mul(x, x, temp);
        } else if n < 0 {
            _glp_mpq_div(x, x, temp);
        }
        _glp_mpq_clear(temp);
        if s < 0 {
            _glp_mpq_neg(x, x);
        }
        assert!(
            fabs(val - _glp_mpq_get_d(x)) <= eps * (1.0 + fabs(val)),
            "fabs(val - mpq_get_d(x)) <= eps * (1.0 + fabs(val))"
        );
    }
}

unsafe fn load_data(ssx: *mut SSX, lp: *mut glp_prob) {
    let m: c_int = (*ssx).m;
    let n: c_int = (*ssx).n;
    let nnz: c_int = *(*ssx).A_ptr.offset((n + 1) as isize) - 1;
    let mut j: c_int = 0;
    let mut k: c_int = 0;
    let mut type_: c_int = 0;
    let mut loc: c_int = 0;
    let mut len: c_int = 0;
    let mut ind: *mut c_int = ptr::null_mut();
    let mut lb: c_double = 0.;
    let mut ub: c_double = 0.;
    let mut coef: c_double = 0.;
    let mut val: *mut c_double = ptr::null_mut();

    assert!((*lp).m == m, "lp->m == m");
    assert!((*lp).n == n, "lp->n == n");
    assert!((*lp).nnz == nnz, "lp->nnz == nnz");

    k = 1;
    while k <= m + n {
        if k <= m {
            type_ = (**(*lp).row.offset(k as isize)).type_;
            lb = (**(*lp).row.offset(k as isize)).lb;
            ub = (**(*lp).row.offset(k as isize)).ub;
        } else {
            type_ = (**(*lp).col.offset((k - m) as isize)).type_;
            lb = (**(*lp).col.offset((k - m) as isize)).lb;
            ub = (**(*lp).col.offset((k - m) as isize)).ub;
        }

        *(*ssx).type_.offset(k as isize) = match type_ {
            1 => 0,
            2 => 1,
            3 => 2,
            4 => 3,
            5 => 4,
            _ => panic!("type != type"),
        };

        set_d_eps(*(*ssx).lb.offset(k as isize), lb);
        set_d_eps(*(*ssx).ub.offset(k as isize), ub);
        k += 1;
    }

    (*ssx).dir = match (*lp).dir {
        1 => 0,
        2 => 1,
        _ => panic!("lp != lp"),
    };

    k = 0;
    while k <= m + n {
        coef = if k == 0 {
            (*lp).c0
        } else if k <= m {
            0.0
        } else {
            (**(*lp).col.offset((k - m) as isize)).coef
        };
        set_d_eps(*(*ssx).coef.offset(k as isize), coef);
        k += 1;
    }

    ind = glp_alloc(1 + m, std::mem::size_of::<c_int>() as c_int) as *mut c_int;
    val = glp_alloc(1 + m, std::mem::size_of::<c_double>() as c_int) as *mut c_double;
    loc = 0;
    j = 1;
    while j <= n {
        *(*ssx).A_ptr.offset(j as isize) = loc + 1;
        len = glp_get_mat_col(lp, j, ind, val);
        k = 1;
        while k <= len {
            loc += 1;
            *(*ssx).A_ind.offset(loc as isize) = *ind.offset(k as isize);
            set_d_eps(*(*ssx).A_val.offset(loc as isize), *val.offset(k as isize));
            k += 1;
        }
        j += 1;
    }
    assert!(loc == nnz, "loc == nnz");
    glp_free(ind as *mut c_void);
    glp_free(val as *mut c_void);
}

unsafe fn load_basis(ssx: *mut SSX, lp: *mut glp_prob) -> c_int {
    let m: c_int = (*ssx).m;
    let n: c_int = (*ssx).n;
    let type_: *mut c_int = (*ssx).type_;
    let stat: *mut c_int = (*ssx).stat;
    let Q_row: *mut c_int = (*ssx).Q_row;
    let Q_col: *mut c_int = (*ssx).Q_col;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut k: c_int = 0;

    assert!((*lp).m == m, "lp->m == m");
    assert!((*lp).n == n, "lp->n == n");

    k = 1;
    while k <= m + n {
        *stat.offset(k as isize) = if k <= m {
            (**(*lp).row.offset(k as isize)).stat
        } else {
            (**(*lp).col.offset((k - m) as isize)).stat
        };

        *stat.offset(k as isize) = match *stat.offset(k as isize) {
            1 => 0,
            2 => {
                assert!(
                    *type_.offset(k as isize) == 1 || *type_.offset(k as isize) == 3,
                    "type[k] == SSX_LO || type[k] == SSX_DB"
                );
                1
            }
            3 => {
                assert!(
                    *type_.offset(k as isize) == 2 || *type_.offset(k as isize) == 3,
                    "type[k] == SSX_UP || type[k] == SSX_DB"
                );
                2
            }
            4 => {
                assert!(*type_.offset(k as isize) == 0, "type[k] == SSX_FR");
                3
            }
            5 => {
                assert!(*type_.offset(k as isize) == 4, "type[k] == SSX_FX");
                4
            }
            _ => panic!("stat != stat"),
        };
        k += 1;
    }

    j = 0;
    i = j;
    k = 1;
    while k <= m + n {
        if *stat.offset(k as isize) == 0 {
            i += 1;
            if i > m {
                return 1;
            }
            *Q_row.offset(k as isize) = i;
            *Q_col.offset(i as isize) = k;
        } else {
            j += 1;
            if j > n {
                return 1;
            }
            *Q_row.offset(k as isize) = m + j;
            *Q_col.offset((m + j) as isize) = k;
        }
        k += 1;
    }
    assert!(i == m && j == n, "i == m && j == n");
    0
}

#[no_mangle]
pub unsafe extern "C" fn glp_exact(lp: *mut glp_prob, parm: *const glp_smcp) -> c_int {
