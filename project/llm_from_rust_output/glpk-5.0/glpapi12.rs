use std::os::raw::{c_int, c_double, c_void, c_char};
use std::ptr;

#[repr(C)]
pub struct glp_prob {
    pool: *mut c_void,
    tree: *mut c_void,
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
    r_tree: *mut c_void,
    c_tree: *mut c_void,
    valid: c_int,
    head: *mut c_int,
    bfd: *mut c_void,
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
pub struct GLPROW {
    i: c_int,
    name: *mut c_char,
    node: *mut c_void,
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
pub struct GLPCOL {
    j: c_int,
    name: *mut c_char,
    node: *mut c_void,
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
pub struct GLPAIJ {
    row: *mut GLPROW,
    col: *mut GLPCOL,
    val: c_double,
    r_prev: *mut GLPAIJ,
    r_next: *mut GLPAIJ,
    c_prev: *mut GLPAIJ,
    c_next: *mut GLPAIJ,
}

#[repr(C)]
pub struct glp_bfcp {
    msg_lev: c_int,
    type_: c_int,
    lu_size: c_int,
    piv_tol: c_double,
    piv_lim: c_int,
    suhl: c_int,
    eps_tol: c_double,
    max_gro: c_double,
    nfs_max: c_int,
    upd_tol: c_double,
    nrs_max: c_int,
    rs_size: c_int,
    foo_bar: [c_double; 38],
}

extern "C" {
    fn glp_get_obj_dir(P: *mut glp_prob) -> c_int;
    fn glp_get_num_rows(P: *mut glp_prob) -> c_int;
    fn glp_get_num_cols(P: *mut glp_prob) -> c_int;
    fn glp_get_row_type(P: *mut glp_prob, i: c_int) -> c_int;
    fn glp_get_row_lb(P: *mut glp_prob, i: c_int) -> c_double;
    fn glp_get_row_ub(P: *mut glp_prob, i: c_int) -> c_double;
    fn glp_get_col_type(P: *mut glp_prob, j: c_int) -> c_int;
    fn glp_get_col_lb(P: *mut glp_prob, j: c_int) -> c_double;
    fn glp_get_col_ub(P: *mut glp_prob, j: c_int) -> c_double;
    fn glp_get_mat_col(
        P: *mut glp_prob,
        j: c_int,
        ind: *mut c_int,
        val: *mut c_double,
    ) -> c_int;
    fn glp_get_prim_stat(P: *mut glp_prob) -> c_int;
    fn glp_get_dual_stat(P: *mut glp_prob) -> c_int;
    fn glp_get_row_stat(P: *mut glp_prob, i: c_int) -> c_int;
    fn glp_get_row_prim(P: *mut glp_prob, i: c_int) -> c_double;
    fn glp_get_row_dual(P: *mut glp_prob, i: c_int) -> c_double;
    fn glp_get_col_stat(P: *mut glp_prob, j: c_int) -> c_int;
    fn glp_get_col_prim(P: *mut glp_prob, j: c_int) -> c_double;
    fn glp_get_col_dual(P: *mut glp_prob, j: c_int) -> c_double;
    fn glp_error_(file: *const c_char, line: c_int) -> Option<unsafe extern "C" fn(*const c_char, ...) -> ()>;
    fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int);
    fn glp_alloc(n: c_int, size: c_int) -> *mut c_void;
    fn glp_free(ptr: *mut c_void);
    fn fabs(_: c_double) -> c_double;
    fn _glp_bfd_get_bfcp(bfd: *mut c_void, parm: *mut c_void);
    fn _glp_bfd_set_bfcp(bfd: *mut c_void, parm: *const c_void);
    fn _glp_bfd_create_it() -> *mut c_void;
    fn _glp_bfd_factorize(
        bfd: *mut c_void,
        m: c_int,
        col: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int, *mut c_double) -> c_int>,
        info: *mut c_void,
    ) -> c_int;
    fn _glp_bfd_ftran(bfd: *mut c_void, x: *mut c_double);
    fn _glp_bfd_btran(bfd: *mut c_void, x: *mut c_double);
    fn _glp_bfd_get_count(bfd: *mut c_void) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn glp_bf_exists(lp: *mut glp_prob) -> c_int {
    if lp.is_null() {
        return 0;
    }
    ((*lp).m == 0 || (*lp).valid != 0) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn glp_factorize(lp: *mut glp_prob) -> c_int {
    if lp.is_null() {
        return 0x1;
    }

    let m = (*lp).m;
    let n = (*lp).n;
    let row = (*lp).row;
    let col = (*lp).col;
    let head = (*lp).head;

    (*lp).valid = 0;

    let mut j = 0;
    let mut ret = 0;

    for k in 1..=m + n {
        let stat = if k <= m {
            (*(*row.offset(k as isize)).stat
        } else {
            (*(*col.offset((k - m) as isize)).stat
        };

        if stat == 1 {
            j += 1;
            if j > m {
                ret = 0x1;
                break;
            }
            *head.offset(j as isize) = k;
            if k <= m {
                (*(*row.offset(k as isize)).bind = j;
            } else {
                (*(*col.offset((k - m) as isize)).bind = j;
            }
        }
    }

    if j < m {
        ret = 0x1;
    } else if m > 0 {
        if (*lp).bfd.is_null() {
            (*lp).bfd = _glp_bfd_create_it();
        }

        ret = match _glp_bfd_factorize(
            (*lp).bfd,
            m,
            Some(b_col),
            lp as *mut c_void,
        ) {
            0 => {
                (*lp).valid = 1;
                0
            }
            1 => 0x2,
            2 => 0x3,
            _ => {
                glp_assert_(
                    b"lp != lp\0".as_ptr() as *const c_char,
                    b"draft/glpapi12.c\0".as_ptr() as *const c_char,
                    166,
                );
                0
            }
        };
    }

    ret
}

unsafe extern "C" fn b_col(
    info: *mut c_void,
    j: c_int,
    ind: *mut c_int,
    val: *mut c_double,
) -> c_int {
    let lp = info as *mut glp_prob;
    let m = (*lp).m;

    assert!(1 <= j && j <= m);

    let k = *((*lp).head).offset(j as isize);
    let mut len = 0;

    if k <= m {
        len = 1;
        *ind.offset(1) = k;
        *val.offset(1) = 1.0;
    } else {
        let mut aij = (*(*col.offset((k - m) as isize)).ptr;
        while !aij.is_null() {
            len += 1;
            *ind.offset(len as isize) = (*(*aij).row).i;
            *val.offset(len as isize) = -(*(*aij).row).rii * (*aij).val * (*(*aij).col).sjj;
            aij = (*aij).c_next;
        }
    }

    len
}

// 其他函数类似地进行安全封装...