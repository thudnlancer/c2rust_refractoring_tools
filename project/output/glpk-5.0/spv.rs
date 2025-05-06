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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPV {
    pub n: i32,
    pub nnz: i32,
    pub pos: *mut i32,
    pub ind: *mut i32,
    pub val: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spv_create_vec(mut n: i32) -> *mut SPV {
    let mut v: *mut SPV = 0 as *mut SPV;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"intopt/spv.c\0" as *const u8 as *const i8,
                46 as i32,
            );
            1 as i32 != 0
        }) as i32;
    v = glp_alloc(1 as i32, ::core::mem::size_of::<SPV>() as u64 as i32) as *mut SPV;
    (*v).n = n;
    (*v).nnz = 0 as i32;
    (*v).pos = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    memset(
        &mut *((*v).pos).offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        0 as i32,
        (n as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    (*v).ind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*v).val = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spv_check_vec(mut v: *mut SPV) {
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut nnz: i32 = 0;
    ((*v).n >= 0 as i32
        || {
            glp_assert_(
                b"v->n >= 0\0" as *const u8 as *const i8,
                b"intopt/spv.c\0" as *const u8 as *const i8,
                78 as i32,
            );
            1 as i32 != 0
        }) as i32;
    nnz = 0 as i32;
    j = (*v).n;
    while j >= 1 as i32 {
        k = *((*v).pos).offset(j as isize);
        (0 as i32 <= k && k <= (*v).nnz
            || {
                glp_assert_(
                    b"0 <= k && k <= v->nnz\0" as *const u8 as *const i8,
                    b"intopt/spv.c\0" as *const u8 as *const i8,
                    82 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if k != 0 as i32 {
            (*((*v).ind).offset(k as isize) == j
                || {
                    glp_assert_(
                        b"v->ind[k] == j\0" as *const u8 as *const i8,
                        b"intopt/spv.c\0" as *const u8 as *const i8,
                        84 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            nnz += 1;
            nnz;
        }
        j -= 1;
        j;
    }
    ((*v).nnz == nnz
        || {
            glp_assert_(
                b"v->nnz == nnz\0" as *const u8 as *const i8,
                b"intopt/spv.c\0" as *const u8 as *const i8,
                88 as i32,
            );
            1 as i32 != 0
        }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spv_get_vj(mut v: *mut SPV, mut j: i32) -> libc::c_double {
    let mut k: i32 = 0;
    (1 as i32 <= j && j <= (*v).n
        || {
            glp_assert_(
                b"1 <= j && j <= v->n\0" as *const u8 as *const i8,
                b"intopt/spv.c\0" as *const u8 as *const i8,
                109 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *((*v).pos).offset(j as isize);
    (0 as i32 <= k && k <= (*v).nnz
        || {
            glp_assert_(
                b"0 <= k && k <= v->nnz\0" as *const u8 as *const i8,
                b"intopt/spv.c\0" as *const u8 as *const i8,
                111 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return if k == 0 as i32 { 0.0f64 } else { *((*v).val).offset(k as isize) };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spv_set_vj(
    mut v: *mut SPV,
    mut j: i32,
    mut val: libc::c_double,
) {
    let mut k: i32 = 0;
    (1 as i32 <= j && j <= (*v).n
        || {
            glp_assert_(
                b"1 <= j && j <= v->n\0" as *const u8 as *const i8,
                b"intopt/spv.c\0" as *const u8 as *const i8,
                132 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *((*v).pos).offset(j as isize);
    if val == 0.0f64 {
        if k != 0 as i32 {
            *((*v).pos).offset(j as isize) = 0 as i32;
            if k < (*v).nnz {
                *((*v).pos).offset(*((*v).ind).offset((*v).nnz as isize) as isize) = k;
                *((*v).ind).offset(k as isize) = *((*v).ind).offset((*v).nnz as isize);
                *((*v).val).offset(k as isize) = *((*v).val).offset((*v).nnz as isize);
            }
            (*v).nnz -= 1;
            (*v).nnz;
        }
    } else {
        if k == 0 as i32 {
            (*v).nnz += 1;
            k = (*v).nnz;
            *((*v).pos).offset(j as isize) = k;
            *((*v).ind).offset(k as isize) = j;
        }
        *((*v).val).offset(k as isize) = val;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spv_clear_vec(mut v: *mut SPV) {
    let mut k: i32 = 0;
    k = 1 as i32;
    while k <= (*v).nnz {
        *((*v).pos).offset(*((*v).ind).offset(k as isize) as isize) = 0 as i32;
        k += 1;
        k;
    }
    (*v).nnz = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spv_clean_vec(mut v: *mut SPV, mut eps: libc::c_double) {
    let mut k: i32 = 0;
    let mut nnz: i32 = 0;
    nnz = 0 as i32;
    k = 1 as i32;
    while k <= (*v).nnz {
        if fabs(*((*v).val).offset(k as isize)) == 0.0f64
            || fabs(*((*v).val).offset(k as isize)) < eps
        {
            *((*v).pos).offset(*((*v).ind).offset(k as isize) as isize) = 0 as i32;
        } else {
            nnz += 1;
            nnz;
            *((*v).pos).offset(*((*v).ind).offset(k as isize) as isize) = nnz;
            *((*v).ind).offset(nnz as isize) = *((*v).ind).offset(k as isize);
            *((*v).val).offset(nnz as isize) = *((*v).val).offset(k as isize);
        }
        k += 1;
        k;
    }
    (*v).nnz = nnz;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spv_copy_vec(mut x: *mut SPV, mut y: *mut SPV) {
    let mut j: i32 = 0;
    (x != y
        || {
            glp_assert_(
                b"x != y\0" as *const u8 as *const i8,
                b"intopt/spv.c\0" as *const u8 as *const i8,
                234 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*x).n == (*y).n
        || {
            glp_assert_(
                b"x->n == y->n\0" as *const u8 as *const i8,
                b"intopt/spv.c\0" as *const u8 as *const i8,
                235 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_spv_clear_vec(x);
    (*x).nnz = (*y).nnz;
    memcpy(
        &mut *((*x).ind).offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        &mut *((*y).ind).offset(1 as i32 as isize) as *mut i32 as *const libc::c_void,
        ((*x).nnz as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    memcpy(
        &mut *((*x).val).offset(1 as i32 as isize) as *mut libc::c_double
            as *mut libc::c_void,
        &mut *((*y).val).offset(1 as i32 as isize) as *mut libc::c_double
            as *const libc::c_void,
        ((*x).nnz as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    j = 1 as i32;
    while j <= (*x).nnz {
        *((*x).pos).offset(*((*x).ind).offset(j as isize) as isize) = j;
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spv_linear_comb(
    mut x: *mut SPV,
    mut a: libc::c_double,
    mut y: *mut SPV,
) {
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut xj: libc::c_double = 0.;
    let mut yj: libc::c_double = 0.;
    (x != y
        || {
            glp_assert_(
                b"x != y\0" as *const u8 as *const i8,
                b"intopt/spv.c\0" as *const u8 as *const i8,
                266 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*x).n == (*y).n
        || {
            glp_assert_(
                b"x->n == y->n\0" as *const u8 as *const i8,
                b"intopt/spv.c\0" as *const u8 as *const i8,
                267 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = 1 as i32;
    while k <= (*y).nnz {
        j = *((*y).ind).offset(k as isize);
        xj = _glp_spv_get_vj(x, j);
        yj = *((*y).val).offset(k as isize);
        _glp_spv_set_vj(x, j, xj + a * yj);
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spv_delete_vec(mut v: *mut SPV) {
    glp_free((*v).pos as *mut libc::c_void);
    glp_free((*v).ind as *mut libc::c_void);
    glp_free((*v).val as *mut libc::c_void);
    glp_free(v as *mut libc::c_void);
}