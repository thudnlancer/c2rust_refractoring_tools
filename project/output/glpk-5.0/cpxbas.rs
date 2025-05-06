#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_printf(fmt: *const i8, _: ...);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn glp_get_obj_dir(P: *mut glp_prob) -> i32;
    fn glp_get_num_rows(P: *mut glp_prob) -> i32;
    fn glp_get_num_cols(P: *mut glp_prob) -> i32;
    fn glp_get_row_type(P: *mut glp_prob, i: i32) -> i32;
    fn glp_get_row_lb(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_row_ub(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_col_type(P: *mut glp_prob, j: i32) -> i32;
    fn glp_get_col_lb(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_ub(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_obj_coef(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_mat_col(
        P: *mut glp_prob,
        j: i32,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
    fn glp_set_row_stat(P: *mut glp_prob, i: i32, stat: i32);
    fn glp_set_col_stat(P: *mut glp_prob, j: i32, stat: i32);
    fn glp_std_basis(P: *mut glp_prob);
}
pub type size_t = u64;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_prob {
    pub pool: *mut DMP,
    pub tree: *mut glp_tree,
    pub name: *mut i8,
    pub obj: *mut i8,
    pub dir: i32,
    pub c0: libc::c_double,
    pub m_max: i32,
    pub n_max: i32,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut AVL,
    pub c_tree: *mut AVL,
    pub valid: i32,
    pub head: *mut i32,
    pub bfd: *mut BFD,
    pub pbs_stat: i32,
    pub dbs_stat: i32,
    pub obj_val: libc::c_double,
    pub it_cnt: i32,
    pub some: i32,
    pub ipt_stat: i32,
    pub ipt_obj: libc::c_double,
    pub mip_stat: i32,
    pub mip_obj: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPCOL {
    pub j: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub kind: i32,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPAIJ {
    pub row: *mut GLPROW,
    pub col: *mut GLPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut GLPAIJ,
    pub r_next: *mut GLPAIJ,
    pub c_prev: *mut GLPAIJ,
    pub c_next: *mut GLPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPROW {
    pub i: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub level: i32,
    pub origin: u8,
    pub klass: u8,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct var {
    pub j: i32,
    pub q: libc::c_double,
}
unsafe extern "C" fn fcmp(
    mut ptr1: *const libc::c_void,
    mut ptr2: *const libc::c_void,
) -> i32 {
    let mut col1: *mut var = ptr1 as *mut libc::c_void as *mut var;
    let mut col2: *mut var = ptr2 as *mut libc::c_void as *mut var;
    if (*col1).q < (*col2).q {
        return -(1 as i32);
    }
    if (*col1).q > (*col2).q {
        return 1 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn get_column(
    mut lp: *mut glp_prob,
    mut j: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    let mut big: libc::c_double = 0.;
    len = glp_get_mat_col(lp, j, ind, val);
    big = 0.0f64;
    k = 1 as i32;
    while k <= len {
        if big < fabs(*val.offset(k as isize)) {
            big = fabs(*val.offset(k as isize));
        }
        k += 1;
        k;
    }
    if big == 0.0f64 {
        big = 1.0f64;
    }
    k = 1 as i32;
    while k <= len {
        *val.offset(k as isize) /= big;
        k += 1;
        k;
    }
    return len;
}
unsafe extern "C" fn cpx_basis(mut lp: *mut glp_prob) {
    let mut C: *mut var = 0 as *mut var;
    let mut C2: *mut var = 0 as *mut var;
    let mut C3: *mut var = 0 as *mut var;
    let mut C4: *mut var = 0 as *mut var;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jk: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut ll: i32 = 0;
    let mut t: i32 = 0;
    let mut n2: i32 = 0;
    let mut n3: i32 = 0;
    let mut n4: i32 = 0;
    let mut type_0: i32 = 0;
    let mut len: i32 = 0;
    let mut I: *mut i32 = 0 as *mut i32;
    let mut r: *mut i32 = 0 as *mut i32;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut alpha: libc::c_double = 0.;
    let mut gamma: libc::c_double = 0.;
    let mut cmax: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut v: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    glp_printf(b"Constructing initial basis...\n\0" as *const u8 as *const i8);
    m = glp_get_num_rows(lp);
    n = glp_get_num_cols(lp);
    C = glp_alloc(1 as i32 + n, ::core::mem::size_of::<var>() as u64 as i32) as *mut var;
    I = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i32>() as u64 as i32) as *mut i32;
    r = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i32>() as u64 as i32) as *mut i32;
    v = glp_alloc(1 as i32 + m, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    ind = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(1 as i32 + m, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    i = 1 as i32;
    while i <= m {
        if glp_get_row_type(lp, i) != 4 as i32 {
            glp_set_row_stat(lp, i, 5 as i32);
        } else if fabs(glp_get_row_lb(lp, i)) <= fabs(glp_get_row_ub(lp, i)) {
            glp_set_row_stat(lp, i, 2 as i32);
        } else {
            glp_set_row_stat(lp, i, 3 as i32);
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        if glp_get_col_type(lp, j) != 4 as i32 {
            glp_set_col_stat(lp, j, 5 as i32);
        } else if fabs(glp_get_col_lb(lp, j)) <= fabs(glp_get_col_ub(lp, j)) {
            glp_set_col_stat(lp, j, 2 as i32);
        } else {
            glp_set_col_stat(lp, j, 3 as i32);
        }
        j += 1;
        j;
    }
    n2 = 0 as i32;
    C2 = C.offset(0 as i32 as isize);
    j = 1 as i32;
    while j <= n {
        type_0 = glp_get_col_type(lp, j);
        if type_0 == 1 as i32 {
            n2 += 1;
            n2;
            (*C2.offset(n2 as isize)).j = j;
            (*C2.offset(n2 as isize)).q = 0.0f64;
        }
        j += 1;
        j;
    }
    n3 = 0 as i32;
    C3 = C2.offset(n2 as isize);
    j = 1 as i32;
    while j <= n {
        type_0 = glp_get_col_type(lp, j);
        if type_0 == 2 as i32 {
            n3 += 1;
            n3;
            (*C3.offset(n3 as isize)).j = j;
            (*C3.offset(n3 as isize)).q = glp_get_col_lb(lp, j);
        } else if type_0 == 3 as i32 {
            n3 += 1;
            n3;
            (*C3.offset(n3 as isize)).j = j;
            (*C3.offset(n3 as isize)).q = -glp_get_col_ub(lp, j);
        }
        j += 1;
        j;
    }
    n4 = 0 as i32;
    C4 = C3.offset(n3 as isize);
    j = 1 as i32;
    while j <= n {
        type_0 = glp_get_col_type(lp, j);
        if type_0 == 4 as i32 {
            n4 += 1;
            n4;
            (*C4.offset(n4 as isize)).j = j;
            (*C4.offset(n4 as isize)).q = glp_get_col_lb(lp, j) - glp_get_col_ub(lp, j);
        }
        j += 1;
        j;
    }
    gamma = 0.0f64;
    j = 1 as i32;
    while j <= n {
        temp = fabs(glp_get_obj_coef(lp, j));
        if gamma < temp {
            gamma = temp;
        }
        j += 1;
        j;
    }
    cmax = if gamma == 0.0f64 { 1.0f64 } else { 1000.0f64 * gamma };
    match glp_get_obj_dir(lp) {
        1 => {
            temp = 1.0f64;
        }
        2 => {
            temp = -1.0f64;
        }
        _ => {
            (lp != lp
                || {
                    glp_assert_(
                        b"lp != lp\0" as *const u8 as *const i8,
                        b"api/cpxbas.c\0" as *const u8 as *const i8,
                        143 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    k = 1 as i32;
    while k <= n2 + n3 + n4 {
        j = (*C.offset(k as isize)).j;
        (*C.offset(k as isize)).q += temp * glp_get_obj_coef(lp, j) / cmax;
        k += 1;
        k;
    }
    qsort(
        C2.offset(1 as i32 as isize) as *mut libc::c_void,
        n2 as size_t,
        ::core::mem::size_of::<var>() as u64,
        Some(
            fcmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    k = 1 as i32;
    while k < n2 {
        ((*C2.offset(k as isize)).q <= (*C2.offset((k + 1 as i32) as isize)).q
            || {
                glp_assert_(
                    b"C2[k].q <= C2[k+1].q\0" as *const u8 as *const i8,
                    b"api/cpxbas.c\0" as *const u8 as *const i8,
                    152 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k += 1;
        k;
    }
    qsort(
        C3.offset(1 as i32 as isize) as *mut libc::c_void,
        n3 as size_t,
        ::core::mem::size_of::<var>() as u64,
        Some(
            fcmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    k = 1 as i32;
    while k < n3 {
        ((*C3.offset(k as isize)).q <= (*C3.offset((k + 1 as i32) as isize)).q
            || {
                glp_assert_(
                    b"C3[k].q <= C3[k+1].q\0" as *const u8 as *const i8,
                    b"api/cpxbas.c\0" as *const u8 as *const i8,
                    154 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k += 1;
        k;
    }
    qsort(
        C4.offset(1 as i32 as isize) as *mut libc::c_void,
        n4 as size_t,
        ::core::mem::size_of::<var>() as u64,
        Some(
            fcmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    k = 1 as i32;
    while k < n4 {
        ((*C4.offset(k as isize)).q <= (*C4.offset((k + 1 as i32) as isize)).q
            || {
                glp_assert_(
                    b"C4[k].q <= C4[k+1].q\0" as *const u8 as *const i8,
                    b"api/cpxbas.c\0" as *const u8 as *const i8,
                    156 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k += 1;
        k;
    }
    i = 1 as i32;
    while i <= m {
        type_0 = glp_get_row_type(lp, i);
        if type_0 != 5 as i32 {
            glp_set_row_stat(lp, i, 1 as i32);
            *I.offset(i as isize) = 1 as i32;
            *r.offset(i as isize) = 1 as i32;
        } else {
            *I.offset(i as isize) = 0 as i32;
            *r.offset(i as isize) = 0 as i32;
        }
        *v.offset(i as isize) = 1.7976931348623157e+308f64;
        i += 1;
        i;
    }
    k = 1 as i32;
    while k <= n2 + n3 + n4 {
        jk = (*C.offset(k as isize)).j;
        len = get_column(lp, jk, ind, val);
        alpha = 0.0f64;
        ll = 0 as i32;
        t = 1 as i32;
        while t <= len {
            l = *ind.offset(t as isize);
            if *r.offset(l as isize) == 0 as i32 && alpha < fabs(*val.offset(t as isize))
            {
                alpha = fabs(*val.offset(t as isize));
                ll = l;
            }
            t += 1;
            t;
        }
        if alpha >= 0.99f64 {
            glp_set_col_stat(lp, jk, 1 as i32);
            *I.offset(ll as isize) = 1 as i32;
            *v.offset(ll as isize) = alpha;
            t = 1 as i32;
            while t <= len {
                l = *ind.offset(t as isize);
                if *val.offset(t as isize) != 0.0f64 {
                    let ref mut fresh0 = *r.offset(l as isize);
                    *fresh0 += 1;
                    *fresh0;
                }
                t += 1;
                t;
            }
        } else {
            t = 1 as i32;
            while t <= len {
                l = *ind.offset(t as isize);
                if fabs(*val.offset(t as isize)) > 0.01f64 * *v.offset(l as isize) {
                    break;
                }
                t += 1;
                t;
            }
            if !(t <= len) {
                alpha = 0.0f64;
                ll = 0 as i32;
                t = 1 as i32;
                while t <= len {
                    l = *ind.offset(t as isize);
                    if *I.offset(l as isize) == 0 as i32
                        && alpha < fabs(*val.offset(t as isize))
                    {
                        alpha = fabs(*val.offset(t as isize));
                        ll = l;
                    }
                    t += 1;
                    t;
                }
                if !(alpha == 0.0f64) {
                    glp_set_col_stat(lp, jk, 1 as i32);
                    *I.offset(ll as isize) = 1 as i32;
                    *v.offset(ll as isize) = alpha;
                    t = 1 as i32;
                    while t <= len {
                        l = *ind.offset(t as isize);
                        if *val.offset(t as isize) != 0.0f64 {
                            let ref mut fresh1 = *r.offset(l as isize);
                            *fresh1 += 1;
                            *fresh1;
                        }
                        t += 1;
                        t;
                    }
                }
            }
        }
        k += 1;
        k;
    }
    i = 1 as i32;
    while i <= m {
        if *I.offset(i as isize) == 0 as i32 {
            glp_set_row_stat(lp, i, 1 as i32);
        }
        i += 1;
        i;
    }
    glp_free(C as *mut libc::c_void);
    glp_free(I as *mut libc::c_void);
    glp_free(r as *mut libc::c_void);
    glp_free(v as *mut libc::c_void);
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn glp_cpx_basis(mut lp: *mut glp_prob) {
    if (*lp).m == 0 as i32 || (*lp).n == 0 as i32 {
        glp_std_basis(lp);
    } else {
        cpx_basis(lp);
    };
}