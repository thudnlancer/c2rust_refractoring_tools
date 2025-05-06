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
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
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
pub struct SPXAT {
    pub ptr: *mut i32,
    pub ind: *mut i32,
    pub val: *mut libc::c_double,
    pub work: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_alloc_at(mut lp: *mut SPXLP, mut at: *mut SPXAT) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut nnz: i32 = (*lp).nnz;
    (*at).ptr = glp_alloc(
        1 as i32 + m + 1 as i32,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*at).ind = glp_alloc(1 as i32 + nnz, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*at).val = glp_alloc(
        1 as i32 + nnz,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    (*at).work = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_build_at(mut lp: *mut SPXLP, mut at: *mut SPXAT) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut nnz: i32 = (*lp).nnz;
    let mut A_ptr: *mut i32 = (*lp).A_ptr;
    let mut A_ind: *mut i32 = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut AT_ptr: *mut i32 = (*at).ptr;
    let mut AT_ind: *mut i32 = (*at).ind;
    let mut AT_val: *mut libc::c_double = (*at).val;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut pos: i32 = 0;
    memset(
        &mut *AT_ptr.offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        0 as i32,
        (m as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    k = 1 as i32;
    while k <= n {
        ptr = *A_ptr.offset(k as isize);
        end = *A_ptr.offset((k + 1 as i32) as isize);
        while ptr < end {
            let ref mut fresh0 = *AT_ptr.offset(*A_ind.offset(ptr as isize) as isize);
            *fresh0 += 1;
            *fresh0;
            ptr += 1;
            ptr;
        }
        k += 1;
        k;
    }
    let ref mut fresh1 = *AT_ptr.offset(1 as i32 as isize);
    *fresh1 += 1;
    *fresh1;
    i = 2 as i32;
    while i <= m {
        *AT_ptr.offset(i as isize) += *AT_ptr.offset((i - 1 as i32) as isize);
        i += 1;
        i;
    }
    (*AT_ptr.offset(m as isize) == nnz + 1 as i32
        || {
            glp_assert_(
                b"AT_ptr[m] == nnz+1\0" as *const u8 as *const i8,
                b"simplex/spxat.c\0" as *const u8 as *const i8,
                72 as i32,
            );
            1 as i32 != 0
        }) as i32;
    *AT_ptr.offset((m + 1 as i32) as isize) = nnz + 1 as i32;
    k = n;
    while k >= 1 as i32 {
        ptr = *A_ptr.offset(k as isize);
        end = *A_ptr.offset((k + 1 as i32) as isize);
        while ptr < end {
            let ref mut fresh2 = *AT_ptr.offset(*A_ind.offset(ptr as isize) as isize);
            *fresh2 -= 1;
            pos = *fresh2;
            *AT_ind.offset(pos as isize) = k;
            *AT_val.offset(pos as isize) = *A_val.offset(ptr as isize);
            ptr += 1;
            ptr;
        }
        k -= 1;
        k;
    }
    (*AT_ptr.offset(1 as i32 as isize) == 1 as i32
        || {
            glp_assert_(
                b"AT_ptr[1] == 1\0" as *const u8 as *const i8,
                b"simplex/spxat.c\0" as *const u8 as *const i8,
                85 as i32,
            );
            1 as i32 != 0
        }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_at_prod(
    mut lp: *mut SPXLP,
    mut at: *mut SPXAT,
    mut y: *mut libc::c_double,
    mut s: libc::c_double,
    mut x: *const libc::c_double,
) {
    let mut m: i32 = (*lp).m;
    let mut AT_ptr: *mut i32 = (*at).ptr;
    let mut AT_ind: *mut i32 = (*at).ind;
    let mut AT_val: *mut libc::c_double = (*at).val;
    let mut i: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut t: libc::c_double = 0.;
    i = 1 as i32;
    while i <= m {
        if *x.offset(i as isize) != 0.0f64 {
            t = s * *x.offset(i as isize);
            ptr = *AT_ptr.offset(i as isize);
            end = *AT_ptr.offset((i + 1 as i32) as isize);
            while ptr < end {
                *y.offset(*AT_ind.offset(ptr as isize) as isize)
                    += *AT_val.offset(ptr as isize) * t;
                ptr += 1;
                ptr;
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_nt_prod1(
    mut lp: *mut SPXLP,
    mut at: *mut SPXAT,
    mut y: *mut libc::c_double,
    mut ign: i32,
    mut s: libc::c_double,
    mut x: *const libc::c_double,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut head: *mut i32 = (*lp).head;
    let mut work: *mut libc::c_double = (*at).work;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    k = 1 as i32;
    while k <= n {
        *work.offset(k as isize) = 0.0f64;
        k += 1;
        k;
    }
    if ign == 0 {
        j = 1 as i32;
        while j <= n - m {
            *work.offset(*head.offset((m + j) as isize) as isize) = *y
                .offset(j as isize);
            j += 1;
            j;
        }
    }
    _glp_spx_at_prod(lp, at, work, s, x);
    j = 1 as i32;
    while j <= n - m {
        *y.offset(j as isize) = *work.offset(*head.offset((m + j) as isize) as isize);
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_eval_trow1(
    mut lp: *mut SPXLP,
    mut at: *mut SPXAT,
    mut rho: *const libc::c_double,
    mut trow: *mut libc::c_double,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut nnz: i32 = (*lp).nnz;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut nnz_rho: i32 = 0;
    let mut cnt1: libc::c_double = 0.;
    let mut cnt2: libc::c_double = 0.;
    nnz_rho = 0 as i32;
    i = 1 as i32;
    while i <= m {
        if *rho.offset(i as isize) != 0.0f64 {
            nnz_rho += 1;
            nnz_rho;
        }
        i += 1;
        i;
    }
    cnt1 = (n - m) as libc::c_double * (nnz as libc::c_double / n as libc::c_double);
    cnt2 = nnz_rho as libc::c_double * (nnz as libc::c_double / m as libc::c_double);
    if cnt1 < cnt2 {
        let mut A_ptr: *mut i32 = (*lp).A_ptr;
        let mut A_ind: *mut i32 = (*lp).A_ind;
        let mut A_val: *mut libc::c_double = (*lp).A_val;
        let mut head: *mut i32 = (*lp).head;
        let mut k: i32 = 0;
        let mut ptr: i32 = 0;
        let mut end: i32 = 0;
        let mut tij: libc::c_double = 0.;
        j = 1 as i32;
        while j <= n - m {
            k = *head.offset((m + j) as isize);
            tij = 0.0f64;
            ptr = *A_ptr.offset(k as isize);
            end = *A_ptr.offset((k + 1 as i32) as isize);
            while ptr < end {
                tij
                    -= *A_val.offset(ptr as isize)
                        * *rho.offset(*A_ind.offset(ptr as isize) as isize);
                ptr += 1;
                ptr;
            }
            *trow.offset(j as isize) = tij;
            j += 1;
            j;
        }
    } else {
        _glp_spx_nt_prod1(lp, at, trow, 1 as i32, -1.0f64, rho);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_free_at(mut lp: *mut SPXLP, mut at: *mut SPXAT) {
    (lp == lp
        || {
            glp_assert_(
                b"lp == lp\0" as *const u8 as *const i8,
                b"simplex/spxat.c\0" as *const u8 as *const i8,
                255 as i32,
            );
            1 as i32 != 0
        }) as i32;
    glp_free((*at).ptr as *mut libc::c_void);
    glp_free((*at).ind as *mut libc::c_void);
    glp_free((*at).val as *mut libc::c_void);
    glp_free((*at).work as *mut libc::c_void);
}