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
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_set_row_stat(P: *mut glp_prob, i: i32, stat: i32);
    fn glp_set_col_stat(P: *mut glp_prob, j: i32, stat: i32);
    fn glp_std_basis(P: *mut glp_prob);
    fn _glp_triang(
        m: i32,
        n: i32,
        mat_0: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                i32,
                *mut i32,
                *mut libc::c_double,
            ) -> i32,
        >,
        info: *mut libc::c_void,
        tol: libc::c_double,
        rn: *mut i32,
        cn: *mut i32,
    ) -> i32;
}
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
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
unsafe extern "C" fn mat(
    mut info: *mut libc::c_void,
    mut k: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut P: *mut glp_prob = info as *mut glp_prob;
    let mut m: i32 = (*P).m;
    let mut n: i32 = (*P).n;
    let mut row: *mut *mut GLPROW = (*P).row;
    let mut col: *mut *mut GLPCOL = (*P).col;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut len: i32 = 0;
    if k > 0 as i32 {
        i = k;
        (1 as i32 <= i && i <= m
            || {
                glp_assert_(
                    b"1 <= i && i <= m\0" as *const u8 as *const i8,
                    b"api/advbas.c\0" as *const u8 as *const i8,
                    59 as i32,
                );
                1 as i32 != 0
            }) as i32;
        len = 0 as i32;
        if (**row.offset(i as isize)).type_0 == 5 as i32 {
            aij = (**row.offset(i as isize)).ptr;
            while !aij.is_null() {
                j = (*(*aij).col).j;
                if (**col.offset(j as isize)).type_0 != 5 as i32 {
                    len += 1;
                    len;
                    *ind.offset(len as isize) = j;
                    *val.offset(len as isize) = (*(*aij).row).rii * (*aij).val
                        * (*(*aij).col).sjj;
                }
                aij = (*aij).r_next;
            }
        }
    } else {
        j = -k;
        (1 as i32 <= j && j <= n
            || {
                glp_assert_(
                    b"1 <= j && j <= n\0" as *const u8 as *const i8,
                    b"api/advbas.c\0" as *const u8 as *const i8,
                    75 as i32,
                );
                1 as i32 != 0
            }) as i32;
        len = 0 as i32;
        if (**col.offset(j as isize)).type_0 != 5 as i32 {
            aij = (**col.offset(j as isize)).ptr;
            while !aij.is_null() {
                i = (*(*aij).row).i;
                if (**row.offset(i as isize)).type_0 == 5 as i32 {
                    len += 1;
                    len;
                    *ind.offset(len as isize) = i;
                    *val.offset(len as isize) = (*(*aij).row).rii * (*aij).val
                        * (*(*aij).col).sjj;
                }
                aij = (*aij).c_next;
            }
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn glp_adv_basis(mut P: *mut glp_prob, mut flags: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut min_mn: i32 = 0;
    let mut size: i32 = 0;
    let mut rn: *mut i32 = 0 as *mut i32;
    let mut cn: *mut i32 = 0 as *mut i32;
    let mut flag: *mut i8 = 0 as *mut i8;
    if flags != 0 as i32 {
        (glp_error_(b"api/advbas.c\0" as *const u8 as *const i8, 95 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_adv_basis: flags = %d; invalid flags\n\0" as *const u8 as *const i8,
            flags,
        );
    }
    m = (*P).m;
    n = (*P).n;
    if m == 0 as i32 || n == 0 as i32 {
        glp_std_basis(P);
    } else {
        glp_printf(b"Constructing initial basis...\n\0" as *const u8 as *const i8);
        min_mn = if m < n { m } else { n };
        rn = glp_alloc(1 as i32 + min_mn, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        cn = glp_alloc(1 as i32 + min_mn, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        flag = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i8>() as u64 as i32)
            as *mut i8;
        i = 1 as i32;
        while i <= m {
            *flag.offset(i as isize) = 0 as i32 as i8;
            glp_set_row_stat(P, i, 5 as i32);
            i += 1;
            i;
        }
        j = 1 as i32;
        while j <= n {
            glp_set_col_stat(P, j, 5 as i32);
            j += 1;
            j;
        }
        size = _glp_triang(
            m,
            n,
            Some(
                mat
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        i32,
                        *mut i32,
                        *mut libc::c_double,
                    ) -> i32,
            ),
            P as *mut libc::c_void,
            0.001f64,
            rn,
            cn,
        );
        (0 as i32 <= size && size <= min_mn
            || {
                glp_assert_(
                    b"0 <= size && size <= min_mn\0" as *const u8 as *const i8,
                    b"api/advbas.c\0" as *const u8 as *const i8,
                    123 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k = 1 as i32;
        while k <= size {
            i = *rn.offset(k as isize);
            (1 as i32 <= i && i <= m
                || {
                    glp_assert_(
                        b"1 <= i && i <= m\0" as *const u8 as *const i8,
                        b"api/advbas.c\0" as *const u8 as *const i8,
                        128 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *flag.offset(i as isize) = 1 as i32 as i8;
            j = *cn.offset(k as isize);
            (1 as i32 <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const i8,
                        b"api/advbas.c\0" as *const u8 as *const i8,
                        131 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            glp_set_col_stat(P, j, 1 as i32);
            k += 1;
            k;
        }
        i = 1 as i32;
        while i <= m {
            if *flag.offset(i as isize) as i32 == 0 as i32 {
                glp_set_row_stat(P, i, 1 as i32);
                if (**((*P).row).offset(i as isize)).type_0 != 5 as i32 {
                    size += 1;
                    size;
                }
            }
            i += 1;
            i;
        }
        glp_printf(b"Size of triangular part is %d\n\0" as *const u8 as *const i8, size);
        glp_free(rn as *mut libc::c_void);
        glp_free(cn as *mut libc::c_void);
        glp_free(flag as *mut libc::c_void);
    };
}