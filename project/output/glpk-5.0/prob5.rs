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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
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
#[no_mangle]
pub unsafe extern "C" fn glp_set_row_stat(
    mut lp: *mut glp_prob,
    mut i: i32,
    mut stat: i32,
) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"api/prob5.c\0" as *const u8 as *const i8, 53 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_row_stat: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    if !(stat == 1 as i32 || stat == 2 as i32 || stat == 3 as i32 || stat == 4 as i32
        || stat == 5 as i32)
    {
        (glp_error_(b"api/prob5.c\0" as *const u8 as *const i8, 57 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_row_stat: i = %d; stat = %d; invalid status\n\0" as *const u8
                as *const i8,
            i,
            stat,
        );
    }
    row = *((*lp).row).offset(i as isize);
    if stat != 1 as i32 {
        match (*row).type_0 {
            1 => {
                stat = 4 as i32;
            }
            2 => {
                stat = 2 as i32;
            }
            3 => {
                stat = 3 as i32;
            }
            4 => {
                if stat != 3 as i32 {
                    stat = 2 as i32;
                }
            }
            5 => {
                stat = 5 as i32;
            }
            _ => {
                (row != row
                    || {
                        glp_assert_(
                            b"row != row\0" as *const u8 as *const i8,
                            b"api/prob5.c\0" as *const u8 as *const i8,
                            67 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
    }
    if (*row).stat == 1 as i32 && stat != 1 as i32
        || (*row).stat != 1 as i32 && stat == 1 as i32
    {
        (*lp).valid = 0 as i32;
    }
    (*row).stat = stat;
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_col_stat(
    mut lp: *mut glp_prob,
    mut j: i32,
    mut stat: i32,
) {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob5.c\0" as *const u8 as *const i8, 107 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_col_stat: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    if !(stat == 1 as i32 || stat == 2 as i32 || stat == 3 as i32 || stat == 4 as i32
        || stat == 5 as i32)
    {
        (glp_error_(b"api/prob5.c\0" as *const u8 as *const i8, 111 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_col_stat: j = %d; stat = %d; invalid status\n\0" as *const u8
                as *const i8,
            j,
            stat,
        );
    }
    col = *((*lp).col).offset(j as isize);
    if stat != 1 as i32 {
        match (*col).type_0 {
            1 => {
                stat = 4 as i32;
            }
            2 => {
                stat = 2 as i32;
            }
            3 => {
                stat = 3 as i32;
            }
            4 => {
                if stat != 3 as i32 {
                    stat = 2 as i32;
                }
            }
            5 => {
                stat = 5 as i32;
            }
            _ => {
                (col != col
                    || {
                        glp_assert_(
                            b"col != col\0" as *const u8 as *const i8,
                            b"api/prob5.c\0" as *const u8 as *const i8,
                            121 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
    }
    if (*col).stat == 1 as i32 && stat != 1 as i32
        || (*col).stat != 1 as i32 && stat == 1 as i32
    {
        (*lp).valid = 0 as i32;
    }
    (*col).stat = stat;
}
#[no_mangle]
pub unsafe extern "C" fn glp_std_basis(mut lp: *mut glp_prob) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 1 as i32;
    while i <= (*lp).m {
        glp_set_row_stat(lp, i, 1 as i32);
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= (*lp).n {
        let mut col: *mut GLPCOL = *((*lp).col).offset(j as isize);
        if (*col).type_0 == 4 as i32 && fabs((*col).lb) > fabs((*col).ub) {
            glp_set_col_stat(lp, j, 3 as i32);
        } else {
            glp_set_col_stat(lp, j, 2 as i32);
        }
        j += 1;
        j;
    }
}