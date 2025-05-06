#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FVS {
    pub n: i32,
    pub nnz: i32,
    pub ind: *mut i32,
    pub vec: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fvs_alloc_vec(mut x: *mut FVS, mut n: i32) {
    let mut j: i32 = 0;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"misc/fvs.c\0" as *const u8 as *const i8,
                28 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*x).n = n;
    (*x).nnz = 0 as i32;
    (*x).ind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*x).vec = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    j = 1 as i32;
    while j <= n {
        *((*x).vec).offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fvs_check_vec(mut x: *const FVS) {
    let mut n: i32 = (*x).n;
    let mut nnz: i32 = (*x).nnz;
    let mut ind: *mut i32 = (*x).ind;
    let mut vec: *mut libc::c_double = (*x).vec;
    let mut map: *mut i8 = 0 as *mut i8;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"misc/fvs.c\0" as *const u8 as *const i8,
                47 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (0 as i32 <= nnz && nnz <= n
        || {
            glp_assert_(
                b"0 <= nnz && nnz <= n\0" as *const u8 as *const i8,
                b"misc/fvs.c\0" as *const u8 as *const i8,
                48 as i32,
            );
            1 as i32 != 0
        }) as i32;
    map = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i8>() as u64 as i32) as *mut i8;
    j = 1 as i32;
    while j <= n {
        *map.offset(j as isize) = (*vec.offset(j as isize) != 0.0f64) as i32 as i8;
        j += 1;
        j;
    }
    k = 1 as i32;
    while k <= nnz {
        j = *ind.offset(k as isize);
        (1 as i32 <= j && j <= n
            || {
                glp_assert_(
                    b"1 <= j && j <= n\0" as *const u8 as *const i8,
                    b"misc/fvs.c\0" as *const u8 as *const i8,
                    54 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*map.offset(j as isize) as i32 != 0
            || {
                glp_assert_(
                    b"map[j]\0" as *const u8 as *const i8,
                    b"misc/fvs.c\0" as *const u8 as *const i8,
                    55 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *map.offset(j as isize) = 0 as i32 as i8;
        k += 1;
        k;
    }
    j = 1 as i32;
    while j <= n {
        (*map.offset(j as isize) == 0
            || {
                glp_assert_(
                    b"!map[j]\0" as *const u8 as *const i8,
                    b"misc/fvs.c\0" as *const u8 as *const i8,
                    59 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j += 1;
        j;
    }
    glp_free(map as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fvs_gather_vec(mut x: *mut FVS, mut eps: libc::c_double) {
    let mut n: i32 = (*x).n;
    let mut ind: *mut i32 = (*x).ind;
    let mut vec: *mut libc::c_double = (*x).vec;
    let mut j: i32 = 0;
    let mut nnz: i32 = 0 as i32;
    j = n;
    while j >= 1 as i32 {
        if -eps < *vec.offset(j as isize) && *vec.offset(j as isize) < eps {
            *vec.offset(j as isize) = 0.0f64;
        } else {
            nnz += 1;
            *ind.offset(nnz as isize) = j;
        }
        j -= 1;
        j;
    }
    (*x).nnz = nnz;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fvs_clear_vec(mut x: *mut FVS) {
    let mut ind: *mut i32 = (*x).ind;
    let mut vec: *mut libc::c_double = (*x).vec;
    let mut k: i32 = 0;
    k = (*x).nnz;
    while k >= 1 as i32 {
        *vec.offset(*ind.offset(k as isize) as isize) = 0.0f64;
        k -= 1;
        k;
    }
    (*x).nnz = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fvs_copy_vec(mut x: *mut FVS, mut y: *const FVS) {
    let mut x_ind: *mut i32 = (*x).ind;
    let mut x_vec: *mut libc::c_double = (*x).vec;
    let mut y_ind: *mut i32 = (*y).ind;
    let mut y_vec: *mut libc::c_double = (*y).vec;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    (x != y as *mut FVS
        || {
            glp_assert_(
                b"x != y\0" as *const u8 as *const i8,
                b"misc/fvs.c\0" as *const u8 as *const i8,
                98 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*x).n == (*y).n
        || {
            glp_assert_(
                b"x->n == y->n\0" as *const u8 as *const i8,
                b"misc/fvs.c\0" as *const u8 as *const i8,
                99 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_fvs_clear_vec(x);
    (*x).nnz = (*y).nnz;
    k = (*x).nnz;
    while k >= 1 as i32 {
        let ref mut fresh0 = *x_ind.offset(k as isize);
        *fresh0 = *y_ind.offset(k as isize);
        j = *fresh0;
        *x_vec.offset(j as isize) = *y_vec.offset(j as isize);
        k -= 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fvs_adjust_vec(mut x: *mut FVS, mut eps: libc::c_double) {
    let mut nnz: i32 = (*x).nnz;
    let mut ind: *mut i32 = (*x).ind;
    let mut vec: *mut libc::c_double = (*x).vec;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut cnt: i32 = 0 as i32;
    k = 1 as i32;
    while k <= nnz {
        j = *ind.offset(k as isize);
        if -eps < *vec.offset(j as isize) && *vec.offset(j as isize) < eps {
            *vec.offset(j as isize) = 0.0f64;
        } else {
            cnt += 1;
            *ind.offset(cnt as isize) = j;
        }
        k += 1;
        k;
    }
    (*x).nnz = cnt;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fvs_free_vec(mut x: *mut FVS) {
    glp_free((*x).ind as *mut libc::c_void);
    glp_free((*x).vec as *mut libc::c_void);
    (*x).nnz = -(1 as i32);
    (*x).n = (*x).nnz;
    (*x).ind = 0 as *mut i32;
    (*x).vec = 0 as *mut libc::c_double;
}