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
    pub type BFD;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_fvs_clear_vec(x: *mut FVS);
    fn _glp_fvs_adjust_vec(x: *mut FVS, eps: libc::c_double);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FVS {
    pub n: i32,
    pub nnz: i32,
    pub ind: *mut i32,
    pub vec: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPXLP {
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub A_ptr: *mut i32,
    pub A_ind: *mut i32,
    pub A_val: *mut libc::c_double,
    pub b: *mut libc::c_double,
    pub c: *mut libc::c_double,
    pub l: *mut libc::c_double,
    pub u: *mut libc::c_double,
    pub head: *mut i32,
    pub flag: *mut i8,
    pub valid: i32,
    pub bfd: *mut BFD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPXNT {
    pub ptr: *mut i32,
    pub len: *mut i32,
    pub ind: *mut i32,
    pub val: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_alloc_nt(mut lp: *mut SPXLP, mut nt: *mut SPXNT) {
    let mut m: i32 = (*lp).m;
    let mut nnz: i32 = (*lp).nnz;
    (*nt).ptr = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*nt).len = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*nt).ind = glp_alloc(1 as i32 + nnz, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*nt).val = glp_alloc(
        1 as i32 + nnz,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_init_nt(mut lp: *mut SPXLP, mut nt: *mut SPXNT) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut nnz: i32 = (*lp).nnz;
    let mut A_ptr: *mut i32 = (*lp).A_ptr;
    let mut A_ind: *mut i32 = (*lp).A_ind;
    let mut NT_ptr: *mut i32 = (*nt).ptr;
    let mut NT_len: *mut i32 = (*nt).len;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    memset(
        &mut *NT_len.offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        0 as i32,
        (m as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    k = 1 as i32;
    while k <= n {
        ptr = *A_ptr.offset(k as isize);
        end = *A_ptr.offset((k + 1 as i32) as isize);
        while ptr < end {
            let ref mut fresh0 = *NT_len.offset(*A_ind.offset(ptr as isize) as isize);
            *fresh0 += 1;
            *fresh0;
            ptr += 1;
            ptr;
        }
        k += 1;
        k;
    }
    *NT_ptr.offset(1 as i32 as isize) = 1 as i32;
    i = 2 as i32;
    while i <= m {
        *NT_ptr.offset(i as isize) = *NT_ptr.offset((i - 1 as i32) as isize)
            + *NT_len.offset((i - 1 as i32) as isize);
        i += 1;
        i;
    }
    (*NT_ptr.offset(m as isize) + *NT_len.offset(m as isize) == nnz + 1 as i32
        || {
            glp_assert_(
                b"NT_ptr[m] + NT_len[m] == nnz+1\0" as *const u8 as *const i8,
                b"simplex/spxnt.c\0" as *const u8 as *const i8,
                71 as i32,
            );
            1 as i32 != 0
        }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_nt_add_col(
    mut lp: *mut SPXLP,
    mut nt: *mut SPXNT,
    mut j: i32,
    mut k: i32,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut nnz: i32 = (*lp).nnz;
    let mut A_ptr: *mut i32 = (*lp).A_ptr;
    let mut A_ind: *mut i32 = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut NT_ptr: *mut i32 = (*nt).ptr;
    let mut NT_len: *mut i32 = (*nt).len;
    let mut NT_ind: *mut i32 = (*nt).ind;
    let mut NT_val: *mut libc::c_double = (*nt).val;
    let mut i: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut pos: i32 = 0;
    (1 as i32 <= j && j <= n - m
        || {
            glp_assert_(
                b"1 <= j && j <= n-m\0" as *const u8 as *const i8,
                b"simplex/spxnt.c\0" as *const u8 as *const i8,
                95 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= k && k <= n
        || {
            glp_assert_(
                b"1 <= k && k <= n\0" as *const u8 as *const i8,
                b"simplex/spxnt.c\0" as *const u8 as *const i8,
                96 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ptr = *A_ptr.offset(k as isize);
    end = *A_ptr.offset((k + 1 as i32) as isize);
    while ptr < end {
        i = *A_ind.offset(ptr as isize);
        let ref mut fresh1 = *NT_len.offset(i as isize);
        let fresh2 = *fresh1;
        *fresh1 = *fresh1 + 1;
        pos = *NT_ptr.offset(i as isize) + fresh2;
        if i < m {
            (pos < *NT_ptr.offset((i + 1 as i32) as isize)
                || {
                    glp_assert_(
                        b"pos < NT_ptr[i+1]\0" as *const u8 as *const i8,
                        b"simplex/spxnt.c\0" as *const u8 as *const i8,
                        104 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        } else {
            (pos <= nnz
                || {
                    glp_assert_(
                        b"pos <= nnz\0" as *const u8 as *const i8,
                        b"simplex/spxnt.c\0" as *const u8 as *const i8,
                        106 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        *NT_ind.offset(pos as isize) = j;
        *NT_val.offset(pos as isize) = *A_val.offset(ptr as isize);
        ptr += 1;
        ptr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_build_nt(mut lp: *mut SPXLP, mut nt: *mut SPXNT) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut head: *mut i32 = (*lp).head;
    let mut NT_len: *mut i32 = (*nt).len;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    memset(
        &mut *NT_len.offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        0 as i32,
        (m as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    j = 1 as i32;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        _glp_spx_nt_add_col(lp, nt, j, k);
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_nt_del_col(
    mut lp: *mut SPXLP,
    mut nt: *mut SPXNT,
    mut j: i32,
    mut k: i32,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut A_ptr: *mut i32 = (*lp).A_ptr;
    let mut A_ind: *mut i32 = (*lp).A_ind;
    let mut NT_ptr: *mut i32 = (*nt).ptr;
    let mut NT_len: *mut i32 = (*nt).len;
    let mut NT_ind: *mut i32 = (*nt).ind;
    let mut NT_val: *mut libc::c_double = (*nt).val;
    let mut i: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut ptr1: i32 = 0;
    let mut end1: i32 = 0;
    (1 as i32 <= j && j <= n - m
        || {
            glp_assert_(
                b"1 <= j && j <= n-m\0" as *const u8 as *const i8,
                b"simplex/spxnt.c\0" as *const u8 as *const i8,
                154 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= k && k <= n
        || {
            glp_assert_(
                b"1 <= k && k <= n\0" as *const u8 as *const i8,
                b"simplex/spxnt.c\0" as *const u8 as *const i8,
                155 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ptr = *A_ptr.offset(k as isize);
    end = *A_ptr.offset((k + 1 as i32) as isize);
    while ptr < end {
        i = *A_ind.offset(ptr as isize);
        ptr1 = *NT_ptr.offset(i as isize);
        end1 = ptr1 + *NT_len.offset(i as isize);
        while *NT_ind.offset(ptr1 as isize) != j {
            ptr1 += 1;
            ptr1;
        }
        (ptr1 < end1
            || {
                glp_assert_(
                    b"ptr1 < end1\0" as *const u8 as *const i8,
                    b"simplex/spxnt.c\0" as *const u8 as *const i8,
                    165 as i32,
                );
                1 as i32 != 0
            }) as i32;
        let ref mut fresh3 = *NT_len.offset(i as isize);
        *fresh3 -= 1;
        *fresh3;
        *NT_ind.offset(ptr1 as isize) = *NT_ind.offset((end1 - 1 as i32) as isize);
        *NT_val.offset(ptr1 as isize) = *NT_val.offset((end1 - 1 as i32) as isize);
        ptr += 1;
        ptr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_update_nt(
    mut lp: *mut SPXLP,
    mut nt: *mut SPXNT,
    mut p: i32,
    mut q: i32,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut head: *mut i32 = (*lp).head;
    (1 as i32 <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const i8,
                b"simplex/spxnt.c\0" as *const u8 as *const i8,
                185 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= q && q <= n - m
        || {
            glp_assert_(
                b"1 <= q && q <= n-m\0" as *const u8 as *const i8,
                b"simplex/spxnt.c\0" as *const u8 as *const i8,
                186 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_spx_nt_del_col(lp, nt, q, *head.offset((m + q) as isize));
    _glp_spx_nt_add_col(lp, nt, q, *head.offset(p as isize));
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_nt_prod(
    mut lp: *mut SPXLP,
    mut nt: *mut SPXNT,
    mut y: *mut libc::c_double,
    mut ign: i32,
    mut s: libc::c_double,
    mut x: *const libc::c_double,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut NT_ptr: *mut i32 = (*nt).ptr;
    let mut NT_len: *mut i32 = (*nt).len;
    let mut NT_ind: *mut i32 = (*nt).ind;
    let mut NT_val: *mut libc::c_double = (*nt).val;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut t: libc::c_double = 0.;
    if ign != 0 {
        j = 1 as i32;
        while j <= n - m {
            *y.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
    }
    i = 1 as i32;
    while i <= m {
        if *x.offset(i as isize) != 0.0f64 {
            t = s * *x.offset(i as isize);
            ptr = *NT_ptr.offset(i as isize);
            end = ptr + *NT_len.offset(i as isize);
            while ptr < end {
                *y.offset(*NT_ind.offset(ptr as isize) as isize)
                    += *NT_val.offset(ptr as isize) * t;
                ptr += 1;
                ptr;
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_nt_prod_s(
    mut lp: *mut SPXLP,
    mut nt: *mut SPXNT,
    mut y: *mut FVS,
    mut ign: i32,
    mut s: libc::c_double,
    mut x: *const FVS,
    mut eps: libc::c_double,
) {
    let mut NT_ptr: *mut i32 = (*nt).ptr;
    let mut NT_len: *mut i32 = (*nt).len;
    let mut NT_ind: *mut i32 = (*nt).ind;
    let mut NT_val: *mut libc::c_double = (*nt).val;
    let mut x_ind: *mut i32 = (*x).ind;
    let mut x_vec: *mut libc::c_double = (*x).vec;
    let mut y_ind: *mut i32 = (*y).ind;
    let mut y_vec: *mut libc::c_double = (*y).vec;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut nnz: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut t: libc::c_double = 0.;
    ((*x).n == (*lp).m
        || {
            glp_assert_(
                b"x->n == lp->m\0" as *const u8 as *const i8,
                b"simplex/spxnt.c\0" as *const u8 as *const i8,
                257 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*y).n == (*lp).n - (*lp).m
        || {
            glp_assert_(
                b"y->n == lp->n-lp->m\0" as *const u8 as *const i8,
                b"simplex/spxnt.c\0" as *const u8 as *const i8,
                258 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if ign != 0 {
        _glp_fvs_clear_vec(y);
    }
    nnz = (*y).nnz;
    k = (*x).nnz;
    while k >= 1 as i32 {
        i = *x_ind.offset(k as isize);
        t = s * *x_vec.offset(i as isize);
        ptr = *NT_ptr.offset(i as isize);
        end = ptr + *NT_len.offset(i as isize);
        while ptr < end {
            j = *NT_ind.offset(ptr as isize);
            if *y_vec.offset(j as isize) == 0.0f64 {
                nnz += 1;
                *y_ind.offset(nnz as isize) = j;
            }
            *y_vec.offset(j as isize) += *NT_val.offset(ptr as isize) * t;
            if *y_vec.offset(j as isize) == 0.0f64 {
                *y_vec.offset(j as isize) = 2.2250738585072014e-308f64;
            }
            ptr += 1;
            ptr;
        }
        k -= 1;
        k;
    }
    (*y).nnz = nnz;
    _glp_fvs_adjust_vec(y, eps);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_free_nt(mut lp: *mut SPXLP, mut nt: *mut SPXNT) {
    (lp == lp
        || {
            glp_assert_(
                b"lp == lp\0" as *const u8 as *const i8,
                b"simplex/spxnt.c\0" as *const u8 as *const i8,
                293 as i32,
            );
            1 as i32 != 0
        }) as i32;
    glp_free((*nt).ptr as *mut libc::c_void);
    glp_free((*nt).len as *mut libc::c_void);
    glp_free((*nt).ind as *mut libc::c_void);
    glp_free((*nt).val as *mut libc::c_void);
}