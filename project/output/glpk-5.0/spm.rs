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
    pub type DMP;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_hbm_read_mat(fname: *const i8) -> *mut HBM;
    fn _glp_hbm_free_mat(hbm: *mut HBM);
    fn _glp_rgr_write_bmp16(fname: *const i8, m: i32, n: i32, map: *const i8) -> i32;
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: i32);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HBM {
    pub title: [i8; 73],
    pub key: [i8; 9],
    pub mxtype: [i8; 4],
    pub rhstyp: [i8; 4],
    pub ptrfmt: [i8; 17],
    pub indfmt: [i8; 17],
    pub valfmt: [i8; 21],
    pub rhsfmt: [i8; 21],
    pub totcrd: i32,
    pub ptrcrd: i32,
    pub indcrd: i32,
    pub valcrd: i32,
    pub rhscrd: i32,
    pub nrow: i32,
    pub ncol: i32,
    pub nnzero: i32,
    pub neltvl: i32,
    pub nrhs: i32,
    pub nrhsix: i32,
    pub nrhsvl: i32,
    pub nguess: i32,
    pub nexact: i32,
    pub colptr: *mut i32,
    pub rowind: *mut i32,
    pub rhsptr: *mut i32,
    pub rhsind: *mut i32,
    pub values: *mut libc::c_double,
    pub rhsval: *mut libc::c_double,
    pub sguess: *mut libc::c_double,
    pub xexact: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPM {
    pub m: i32,
    pub n: i32,
    pub pool: *mut DMP,
    pub row: *mut *mut SPME,
    pub col: *mut *mut SPME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPME {
    pub i: i32,
    pub j: i32,
    pub val: libc::c_double,
    pub r_prev: *mut SPME,
    pub r_next: *mut SPME,
    pub c_prev: *mut SPME,
    pub c_next: *mut SPME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PER {
    pub n: i32,
    pub row: *mut i32,
    pub col: *mut i32,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_create_mat(mut m: i32, mut n: i32) -> *mut SPM {
    let mut A: *mut SPM = 0 as *mut SPM;
    (0 as i32 <= m && m < 2147483647 as i32
        || {
            glp_assert_(
                b"0 <= m && m < INT_MAX\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                49 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (0 as i32 <= n && n < 2147483647 as i32
        || {
            glp_assert_(
                b"0 <= n && n < INT_MAX\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                50 as i32,
            );
            1 as i32 != 0
        }) as i32;
    A = glp_alloc(1 as i32, ::core::mem::size_of::<SPM>() as u64 as i32) as *mut SPM;
    (*A).m = m;
    (*A).n = n;
    if m == 0 as i32 || n == 0 as i32 {
        (*A).pool = 0 as *mut DMP;
        (*A).row = 0 as *mut *mut SPME;
        (*A).col = 0 as *mut *mut SPME;
    } else {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        (*A).pool = _glp_dmp_create_pool();
        (*A).row = glp_alloc(
            1 as i32 + m,
            ::core::mem::size_of::<*mut SPME>() as u64 as i32,
        ) as *mut *mut SPME;
        i = 1 as i32;
        while i <= m {
            let ref mut fresh0 = *((*A).row).offset(i as isize);
            *fresh0 = 0 as *mut SPME;
            i += 1;
            i;
        }
        (*A).col = glp_alloc(
            1 as i32 + n,
            ::core::mem::size_of::<*mut SPME>() as u64 as i32,
        ) as *mut *mut SPME;
        j = 1 as i32;
        while j <= n {
            let ref mut fresh1 = *((*A).col).offset(j as isize);
            *fresh1 = 0 as *mut SPME;
            j += 1;
            j;
        }
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_new_elem(
    mut A: *mut SPM,
    mut i: i32,
    mut j: i32,
    mut val: libc::c_double,
) -> *mut SPME {
    let mut e: *mut SPME = 0 as *mut SPME;
    (1 as i32 <= i && i <= (*A).m
        || {
            glp_assert_(
                b"1 <= i && i <= A->m\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                92 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= j && j <= (*A).n
        || {
            glp_assert_(
                b"1 <= j && j <= A->n\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                93 as i32,
            );
            1 as i32 != 0
        }) as i32;
    e = _glp_dmp_get_atom((*A).pool, ::core::mem::size_of::<SPME>() as u64 as i32)
        as *mut SPME;
    (*e).i = i;
    (*e).j = j;
    (*e).val = val;
    (*e).r_prev = 0 as *mut SPME;
    (*e).r_next = *((*A).row).offset(i as isize);
    if !((*e).r_next).is_null() {
        (*(*e).r_next).r_prev = e;
    }
    (*e).c_prev = 0 as *mut SPME;
    (*e).c_next = *((*A).col).offset(j as isize);
    if !((*e).c_next).is_null() {
        (*(*e).c_next).c_prev = e;
    }
    let ref mut fresh2 = *((*A).col).offset(j as isize);
    *fresh2 = e;
    let ref mut fresh3 = *((*A).row).offset(i as isize);
    *fresh3 = *fresh2;
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_delete_mat(mut A: *mut SPM) {
    if !((*A).pool).is_null() {
        _glp_dmp_delete_pool((*A).pool);
    }
    if !((*A).row).is_null() {
        glp_free((*A).row as *mut libc::c_void);
    }
    if !((*A).col).is_null() {
        glp_free((*A).col as *mut libc::c_void);
    }
    glp_free(A as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_test_mat_e(mut n: i32, mut c: i32) -> *mut SPM {
    let mut A: *mut SPM = 0 as *mut SPM;
    let mut i: i32 = 0;
    (n >= 3 as i32 && 2 as i32 <= c && c <= n - 1 as i32
        || {
            glp_assert_(
                b"n >= 3 && 2 <= c && c <= n-1\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                163 as i32,
            );
            1 as i32 != 0
        }) as i32;
    A = _glp_spm_create_mat(n, n);
    i = 1 as i32;
    while i <= n {
        _glp_spm_new_elem(A, i, i, 4.0f64);
        i += 1;
        i;
    }
    i = 1 as i32;
    while i <= n - 1 as i32 {
        _glp_spm_new_elem(A, i, i + 1 as i32, -1.0f64);
        _glp_spm_new_elem(A, i + 1 as i32, i, -1.0f64);
        i += 1;
        i;
    }
    i = 1 as i32;
    while i <= n - c {
        _glp_spm_new_elem(A, i, i + c, -1.0f64);
        _glp_spm_new_elem(A, i + c, i, -1.0f64);
        i += 1;
        i;
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_test_mat_d(mut n: i32, mut c: i32) -> *mut SPM {
    let mut A: *mut SPM = 0 as *mut SPM;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    (n >= 14 as i32 && 1 as i32 <= c && c <= n - 13 as i32
        || {
            glp_assert_(
                b"n >= 14 && 1 <= c && c <= n-13\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                209 as i32,
            );
            1 as i32 != 0
        }) as i32;
    A = _glp_spm_create_mat(n, n);
    i = 1 as i32;
    while i <= n {
        _glp_spm_new_elem(A, i, i, 1.0f64);
        i += 1;
        i;
    }
    i = 1 as i32;
    while i <= n - c {
        _glp_spm_new_elem(A, i, i + c, (i + 1 as i32) as libc::c_double);
        i += 1;
        i;
    }
    i = n - c + 1 as i32;
    while i <= n {
        _glp_spm_new_elem(A, i, i - n + c, (i + 1 as i32) as libc::c_double);
        i += 1;
        i;
    }
    i = 1 as i32;
    while i <= n - c - 1 as i32 {
        _glp_spm_new_elem(A, i, i + c + 1 as i32, -i as libc::c_double);
        i += 1;
        i;
    }
    i = n - c;
    while i <= n {
        _glp_spm_new_elem(A, i, i - n + c + 1 as i32, -i as libc::c_double);
        i += 1;
        i;
    }
    i = 1 as i32;
    while i <= n - c - 2 as i32 {
        _glp_spm_new_elem(A, i, i + c + 2 as i32, 16.0f64);
        i += 1;
        i;
    }
    i = n - c - 1 as i32;
    while i <= n {
        _glp_spm_new_elem(A, i, i - n + c + 2 as i32, 16.0f64);
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= 10 as i32 {
        i = 1 as i32;
        while i <= 11 as i32 - j {
            _glp_spm_new_elem(
                A,
                i,
                n - 11 as i32 + i + j,
                100.0f64 * j as libc::c_double,
            );
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_show_mat(
    mut A: *const SPM,
    mut fname: *const i8,
) -> i32 {
    let mut m: i32 = (*A).m;
    let mut n: i32 = (*A).n;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ret: i32 = 0;
    let mut map: *mut i8 = 0 as *mut i8;
    glp_printf(
        b"spm_show_mat: writing matrix pattern to '%s'...\n\0" as *const u8 as *const i8,
        fname,
    );
    (1 as i32 <= m && m <= 32767 as i32
        || {
            glp_assert_(
                b"1 <= m && m <= 32767\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                268 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= n && n <= 32767 as i32
        || {
            glp_assert_(
                b"1 <= n && n <= 32767\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                269 as i32,
            );
            1 as i32 != 0
        }) as i32;
    map = glp_alloc(1 as i32, m * n) as *mut i8;
    memset(map as *mut libc::c_void, 0x8 as i32, (m * n) as u64);
    i = 1 as i32;
    while i <= m {
        let mut e: *mut SPME = 0 as *mut SPME;
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            j = (*e).j;
            (1 as i32 <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const i8,
                        b"misc/spm.c\0" as *const u8 as *const i8,
                        276 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            k = n * (i - 1 as i32) + (j - 1 as i32);
            if *map.offset(k as isize) as i32 != 0x8 as i32 {
                *map.offset(k as isize) = 0xc as i32 as i8;
            } else if (*e).val > 0.0f64 {
                *map.offset(k as isize) = 0xf as i32 as i8;
            } else if (*e).val < 0.0f64 {
                *map.offset(k as isize) = 0xb as i32 as i8;
            } else {
                *map.offset(k as isize) = 0xa as i32 as i8;
            }
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    ret = _glp_rgr_write_bmp16(fname, m, n, map as *const i8);
    glp_free(map as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_read_hbm(mut fname: *const i8) -> *mut SPM {
    let mut A: *mut SPM = 0 as *mut SPM;
    let mut hbm: *mut HBM = 0 as *mut HBM;
    let mut nrow: i32 = 0;
    let mut ncol: i32 = 0;
    let mut nnzero: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut beg: i32 = 0;
    let mut end: i32 = 0;
    let mut ptr: i32 = 0;
    let mut colptr: *mut i32 = 0 as *mut i32;
    let mut rowind: *mut i32 = 0 as *mut i32;
    let mut val: libc::c_double = 0.;
    let mut values: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut mxtype: *mut i8 = 0 as *mut i8;
    hbm = _glp_hbm_read_mat(fname);
    if hbm.is_null() {
        glp_printf(b"spm_read_hbm: unable to read matrix\n\0" as *const u8 as *const i8);
    } else {
        mxtype = ((*hbm).mxtype).as_mut_ptr();
        nrow = (*hbm).nrow;
        ncol = (*hbm).ncol;
        nnzero = (*hbm).nnzero;
        colptr = (*hbm).colptr;
        rowind = (*hbm).rowind;
        values = (*hbm).values;
        if !(strcmp(mxtype, b"RSA\0" as *const u8 as *const i8) == 0 as i32
            || strcmp(mxtype, b"PSA\0" as *const u8 as *const i8) == 0 as i32
            || strcmp(mxtype, b"RUA\0" as *const u8 as *const i8) == 0 as i32
            || strcmp(mxtype, b"PUA\0" as *const u8 as *const i8) == 0 as i32
            || strcmp(mxtype, b"RRA\0" as *const u8 as *const i8) == 0 as i32
            || strcmp(mxtype, b"PRA\0" as *const u8 as *const i8) == 0 as i32)
        {
            glp_printf(
                b"spm_read_hbm: matrix type '%s' not supported\n\0" as *const u8
                    as *const i8,
                mxtype,
            );
        } else {
            A = _glp_spm_create_mat(nrow, ncol);
            if *mxtype.offset(1 as i32 as isize) as i32 == 'S' as i32
                || *mxtype.offset(1 as i32 as isize) as i32 == 'U' as i32
            {
                (nrow == ncol
                    || {
                        glp_assert_(
                            b"nrow == ncol\0" as *const u8 as *const i8,
                            b"misc/spm.c\0" as *const u8 as *const i8,
                            352 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            j = 1 as i32;
            while j <= ncol {
                beg = *colptr.offset(j as isize);
                end = *colptr.offset((j + 1 as i32) as isize);
                (1 as i32 <= beg && beg <= end && end <= nnzero + 1 as i32
                    || {
                        glp_assert_(
                            b"1 <= beg && beg <= end && end <= nnzero + 1\0" as *const u8
                                as *const i8,
                            b"misc/spm.c\0" as *const u8 as *const i8,
                            356 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                ptr = beg;
                while ptr < end {
                    i = *rowind.offset(ptr as isize);
                    (1 as i32 <= i && i <= nrow
                        || {
                            glp_assert_(
                                b"1 <= i && i <= nrow\0" as *const u8 as *const i8,
                                b"misc/spm.c\0" as *const u8 as *const i8,
                                359 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if *mxtype.offset(0 as i32 as isize) as i32 == 'R' as i32 {
                        val = *values.offset(ptr as isize);
                    } else {
                        val = 1.0f64;
                    }
                    _glp_spm_new_elem(A, i, j, val);
                    if *mxtype.offset(1 as i32 as isize) as i32 == 'S' as i32 && i != j {
                        _glp_spm_new_elem(A, j, i, val);
                    }
                    ptr += 1;
                    ptr;
                }
                j += 1;
                j;
            }
        }
    }
    if !hbm.is_null() {
        _glp_hbm_free_mat(hbm);
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_count_nnz(mut A: *const SPM) -> i32 {
    let mut e: *mut SPME = 0 as *mut SPME;
    let mut i: i32 = 0;
    let mut nnz: i32 = 0 as i32;
    i = 1 as i32;
    while i <= (*A).m {
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            nnz += 1;
            nnz;
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    return nnz;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_drop_zeros(
    mut A: *mut SPM,
    mut eps: libc::c_double,
) -> i32 {
    let mut e: *mut SPME = 0 as *mut SPME;
    let mut next: *mut SPME = 0 as *mut SPME;
    let mut i: i32 = 0;
    let mut count: i32 = 0 as i32;
    i = 1 as i32;
    while i <= (*A).m {
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            next = (*e).r_next;
            if (*e).val == 0.0f64 || fabs((*e).val) < eps {
                if ((*e).r_prev).is_null() {
                    let ref mut fresh4 = *((*A).row).offset((*e).i as isize);
                    *fresh4 = (*e).r_next;
                } else {
                    (*(*e).r_prev).r_next = (*e).r_next;
                }
                if !((*e).r_next).is_null() {
                    (*(*e).r_next).r_prev = (*e).r_prev;
                }
                if ((*e).c_prev).is_null() {
                    let ref mut fresh5 = *((*A).col).offset((*e).j as isize);
                    *fresh5 = (*e).c_next;
                } else {
                    (*(*e).c_prev).c_next = (*e).c_next;
                }
                if !((*e).c_next).is_null() {
                    (*(*e).c_next).c_prev = (*e).c_prev;
                }
                _glp_dmp_free_atom(
                    (*A).pool,
                    e as *mut libc::c_void,
                    ::core::mem::size_of::<SPME>() as u64 as i32,
                );
                count += 1;
                count;
            }
            e = next;
        }
        i += 1;
        i;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_read_mat(mut fname: *const i8) -> *mut SPM {
    (fname != fname
        || {
            glp_assert_(
                b"fname != fname\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                476 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return 0 as *mut SPM;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_write_mat(
    mut A: *const SPM,
    mut fname: *const i8,
) -> i32 {
    (A != A
        || {
            glp_assert_(
                b"A != A\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                574 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (fname != fname
        || {
            glp_assert_(
                b"fname != fname\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                575 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_transpose(mut A: *const SPM) -> *mut SPM {
    let mut B: *mut SPM = 0 as *mut SPM;
    let mut i: i32 = 0;
    B = _glp_spm_create_mat((*A).n, (*A).m);
    i = 1 as i32;
    while i <= (*A).m {
        let mut e: *mut SPME = 0 as *mut SPME;
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            _glp_spm_new_elem(B, (*e).j, i, (*e).val);
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    return B;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_add_sym(
    mut A: *const SPM,
    mut B: *const SPM,
) -> *mut SPM {
    let mut C: *mut SPM = 0 as *mut SPM;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut flag: *mut i32 = 0 as *mut i32;
    ((*A).m == (*B).m
        || {
            glp_assert_(
                b"A->m == B->m\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                646 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*A).n == (*B).n
        || {
            glp_assert_(
                b"A->n == B->n\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                647 as i32,
            );
            1 as i32 != 0
        }) as i32;
    C = _glp_spm_create_mat((*A).m, (*A).n);
    flag = glp_alloc(1 as i32 + (*C).n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    j = 1 as i32;
    while j <= (*C).n {
        *flag.offset(j as isize) = 0 as i32;
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= (*C).m {
        let mut e: *mut SPME = 0 as *mut SPME;
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            j = (*e).j;
            if *flag.offset(j as isize) == 0 {
                _glp_spm_new_elem(C, i, j, 0.0f64);
                *flag.offset(j as isize) = 1 as i32;
            }
            e = (*e).r_next;
        }
        e = *((*B).row).offset(i as isize);
        while !e.is_null() {
            j = (*e).j;
            if *flag.offset(j as isize) == 0 {
                _glp_spm_new_elem(C, i, j, 0.0f64);
                *flag.offset(j as isize) = 1 as i32;
            }
            e = (*e).r_next;
        }
        e = *((*C).row).offset(i as isize);
        while !e.is_null() {
            *flag.offset((*e).j as isize) = 0 as i32;
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= (*C).n {
        (*flag.offset(j as isize) == 0
            || {
                glp_assert_(
                    b"!flag[j]\0" as *const u8 as *const i8,
                    b"misc/spm.c\0" as *const u8 as *const i8,
                    682 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j += 1;
        j;
    }
    glp_free(flag as *mut libc::c_void);
    return C;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_add_num(
    mut C: *mut SPM,
    mut alfa: libc::c_double,
    mut A: *const SPM,
    mut beta: libc::c_double,
    mut B: *const SPM,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    work = glp_alloc(
        1 as i32 + (*C).n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    j = 1 as i32;
    while j <= (*C).n {
        *work.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= (*C).n {
        let mut e: *mut SPME = 0 as *mut SPME;
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            *work.offset((*e).j as isize) += alfa * (*e).val;
            e = (*e).r_next;
        }
        e = *((*B).row).offset(i as isize);
        while !e.is_null() {
            *work.offset((*e).j as isize) += beta * (*e).val;
            e = (*e).r_next;
        }
        e = *((*C).row).offset(i as isize);
        while !e.is_null() {
            j = (*e).j;
            (*e).val = *work.offset(j as isize);
            *work.offset(j as isize) = 0.0f64;
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= (*C).n {
        (*work.offset(j as isize) == 0.0f64
            || {
                glp_assert_(
                    b"work[j] == 0.0\0" as *const u8 as *const i8,
                    b"misc/spm.c\0" as *const u8 as *const i8,
                    714 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j += 1;
        j;
    }
    glp_free(work as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_add_mat(
    mut alfa: libc::c_double,
    mut A: *const SPM,
    mut beta: libc::c_double,
    mut B: *const SPM,
) -> *mut SPM {
    let mut C: *mut SPM = 0 as *mut SPM;
    C = _glp_spm_add_sym(A, B);
    _glp_spm_add_num(C, alfa, A, beta, B);
    return C;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_mul_sym(
    mut A: *const SPM,
    mut B: *const SPM,
) -> *mut SPM {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut flag: *mut i32 = 0 as *mut i32;
    let mut C: *mut SPM = 0 as *mut SPM;
    ((*A).n == (*B).m
        || {
            glp_assert_(
                b"A->n == B->m\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                731 as i32,
            );
            1 as i32 != 0
        }) as i32;
    C = _glp_spm_create_mat((*A).m, (*B).n);
    flag = glp_alloc(1 as i32 + (*C).n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    j = 1 as i32;
    while j <= (*C).n {
        *flag.offset(j as isize) = 0 as i32;
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= (*C).m {
        let mut e: *mut SPME = 0 as *mut SPME;
        let mut ee: *mut SPME = 0 as *mut SPME;
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            k = (*e).j;
            ee = *((*B).row).offset(k as isize);
            while !ee.is_null() {
                j = (*ee).j;
                if *flag.offset(j as isize) == 0 {
                    _glp_spm_new_elem(C, i, j, 0.0f64);
                    *flag.offset(j as isize) = 1 as i32;
                }
                ee = (*ee).r_next;
            }
            e = (*e).r_next;
        }
        e = *((*C).row).offset(i as isize);
        while !e.is_null() {
            *flag.offset((*e).j as isize) = 0 as i32;
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= (*C).n {
        (*flag.offset(j as isize) == 0
            || {
                glp_assert_(
                    b"!flag[j]\0" as *const u8 as *const i8,
                    b"misc/spm.c\0" as *const u8 as *const i8,
                    760 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j += 1;
        j;
    }
    glp_free(flag as *mut libc::c_void);
    return C;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_mul_num(
    mut C: *mut SPM,
    mut A: *const SPM,
    mut B: *const SPM,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    work = glp_alloc(
        1 as i32 + (*A).n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    j = 1 as i32;
    while j <= (*A).n {
        *work.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= (*C).m {
        let mut e: *mut SPME = 0 as *mut SPME;
        let mut ee: *mut SPME = 0 as *mut SPME;
        let mut temp: libc::c_double = 0.;
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            *work.offset((*e).j as isize) += (*e).val;
            e = (*e).r_next;
        }
        e = *((*C).row).offset(i as isize);
        while !e.is_null() {
            j = (*e).j;
            temp = 0.0f64;
            ee = *((*B).col).offset(j as isize);
            while !ee.is_null() {
                temp += *work.offset((*ee).i as isize) * (*ee).val;
                ee = (*ee).c_next;
            }
            (*e).val = temp;
            e = (*e).r_next;
        }
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            *work.offset((*e).j as isize) = 0.0f64;
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= (*A).n {
        (*work.offset(j as isize) == 0.0f64
            || {
                glp_assert_(
                    b"work[j] == 0.0\0" as *const u8 as *const i8,
                    b"misc/spm.c\0" as *const u8 as *const i8,
                    796 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j += 1;
        j;
    }
    glp_free(work as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_mul_mat(
    mut A: *const SPM,
    mut B: *const SPM,
) -> *mut SPM {
    let mut C: *mut SPM = 0 as *mut SPM;
    C = _glp_spm_mul_sym(A, B);
    _glp_spm_mul_num(C, A, B);
    return C;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_create_per(mut n: i32) -> *mut PER {
    let mut P: *mut PER = 0 as *mut PER;
    let mut k: i32 = 0;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                813 as i32,
            );
            1 as i32 != 0
        }) as i32;
    P = glp_alloc(1 as i32, ::core::mem::size_of::<PER>() as u64 as i32) as *mut PER;
    (*P).n = n;
    (*P).row = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*P).col = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    k = 1 as i32;
    while k <= n {
        let ref mut fresh6 = *((*P).col).offset(k as isize);
        *fresh6 = k;
        *((*P).row).offset(k as isize) = *fresh6;
        k += 1;
        k;
    }
    return P;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_check_per(mut P: *mut PER) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    ((*P).n >= 0 as i32
        || {
            glp_assert_(
                b"P->n >= 0\0" as *const u8 as *const i8,
                b"misc/spm.c\0" as *const u8 as *const i8,
                827 as i32,
            );
            1 as i32 != 0
        }) as i32;
    i = 1 as i32;
    while i <= (*P).n {
        j = *((*P).row).offset(i as isize);
        (1 as i32 <= j && j <= (*P).n
            || {
                glp_assert_(
                    b"1 <= j && j <= P->n\0" as *const u8 as *const i8,
                    b"misc/spm.c\0" as *const u8 as *const i8,
                    830 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*((*P).col).offset(j as isize) == i
            || {
                glp_assert_(
                    b"P->col[j] == i\0" as *const u8 as *const i8,
                    b"misc/spm.c\0" as *const u8 as *const i8,
                    831 as i32,
                );
                1 as i32 != 0
            }) as i32;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spm_delete_per(mut P: *mut PER) {
    glp_free((*P).row as *mut libc::c_void);
    glp_free((*P).col as *mut libc::c_void);
    glp_free(P as *mut libc::c_void);
}