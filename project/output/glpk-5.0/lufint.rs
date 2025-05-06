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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_luf_store_v_cols(
        luf: *mut LUF,
        col: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                i32,
                *mut i32,
                *mut libc::c_double,
            ) -> i32,
        >,
        info: *mut libc::c_void,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
    fn _glp_sva_delete_area(sva: *mut SVA);
    fn _glp_sva_alloc_vecs(sva: *mut SVA, nnn: i32) -> i32;
    fn _glp_sva_create_area(n_max: i32, size: i32) -> *mut SVA;
    fn _glp_sgf_factorize(sgf: *mut SGF, singl: i32) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SVA {
    pub n_max: i32,
    pub n: i32,
    pub ptr: *mut i32,
    pub len: *mut i32,
    pub cap: *mut i32,
    pub size: i32,
    pub m_ptr: i32,
    pub r_ptr: i32,
    pub head: i32,
    pub tail: i32,
    pub prev: *mut i32,
    pub next: *mut i32,
    pub ind: *mut i32,
    pub val: *mut libc::c_double,
    pub talky: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUF {
    pub n: i32,
    pub sva: *mut SVA,
    pub fr_ref: i32,
    pub fc_ref: i32,
    pub vr_ref: i32,
    pub vr_piv: *mut libc::c_double,
    pub vc_ref: i32,
    pub pp_ind: *mut i32,
    pub pp_inv: *mut i32,
    pub qq_ind: *mut i32,
    pub qq_inv: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SGF {
    pub luf: *mut LUF,
    pub rs_head: *mut i32,
    pub rs_prev: *mut i32,
    pub rs_next: *mut i32,
    pub cs_head: *mut i32,
    pub cs_prev: *mut i32,
    pub cs_next: *mut i32,
    pub vr_max: *mut libc::c_double,
    pub flag: *mut i8,
    pub work: *mut libc::c_double,
    pub updat: i32,
    pub piv_tol: libc::c_double,
    pub piv_lim: i32,
    pub suhl: i32,
    pub eps_tol: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUFINT {
    pub n_max: i32,
    pub valid: i32,
    pub sva: *mut SVA,
    pub luf: *mut LUF,
    pub sgf: *mut SGF,
    pub sva_n_max: i32,
    pub sva_size: i32,
    pub delta_n0: i32,
    pub delta_n: i32,
    pub sgf_updat: i32,
    pub sgf_piv_tol: libc::c_double,
    pub sgf_piv_lim: i32,
    pub sgf_suhl: i32,
    pub sgf_eps_tol: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_lufint_create() -> *mut LUFINT {
    let mut fi: *mut LUFINT = 0 as *mut LUFINT;
    fi = glp_alloc(1 as i32, ::core::mem::size_of::<LUFINT>() as u64 as i32)
        as *mut LUFINT;
    (*fi).n_max = 0 as i32;
    (*fi).valid = 0 as i32;
    (*fi).sva = 0 as *mut SVA;
    (*fi).luf = 0 as *mut LUF;
    (*fi).sgf = 0 as *mut SGF;
    (*fi).sva_size = 0 as i32;
    (*fi).sva_n_max = (*fi).sva_size;
    (*fi).delta_n = 0 as i32;
    (*fi).delta_n0 = (*fi).delta_n;
    (*fi).sgf_updat = 0 as i32;
    (*fi).sgf_piv_tol = 0.10f64;
    (*fi).sgf_piv_lim = 4 as i32;
    (*fi).sgf_suhl = 1 as i32;
    (*fi).sgf_eps_tol = 2.2204460492503131e-16f64;
    return fi;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_lufint_factorize(
    mut fi: *mut LUFINT,
    mut n: i32,
    mut col: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            i32,
            *mut i32,
            *mut libc::c_double,
        ) -> i32,
    >,
    mut info: *mut libc::c_void,
) -> i32 {
    let mut sva: *mut SVA = 0 as *mut SVA;
    let mut luf: *mut LUF = 0 as *mut LUF;
    let mut sgf: *mut SGF = 0 as *mut SGF;
    let mut k: i32 = 0;
    (n > 0 as i32
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const i8,
                b"bflib/lufint.c\0" as *const u8 as *const i8,
                51 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*fi).valid = 0 as i32;
    sva = (*fi).sva;
    if sva.is_null() {
        let mut sva_n_max: i32 = (*fi).sva_n_max;
        let mut sva_size: i32 = (*fi).sva_size;
        if sva_n_max == 0 as i32 {
            sva_n_max = 4 as i32 * n;
        }
        if sva_size == 0 as i32 {
            sva_size = 10 as i32 * n;
        }
        (*fi).sva = _glp_sva_create_area(sva_n_max, sva_size);
        sva = (*fi).sva;
    }
    if (*fi).n_max < n {
        let mut n_max: i32 = (*fi).n_max;
        if n_max == 0 as i32 {
            (*fi).n_max = n + (*fi).delta_n0;
            n_max = (*fi).n_max;
        } else {
            (*fi).n_max = n + (*fi).delta_n;
            n_max = (*fi).n_max;
        }
        (n_max >= n
            || {
                glp_assert_(
                    b"n_max >= n\0" as *const u8 as *const i8,
                    b"bflib/lufint.c\0" as *const u8 as *const i8,
                    71 as i32,
                );
                1 as i32 != 0
            }) as i32;
        luf = (*fi).luf;
        if luf.is_null() {
            (*fi).luf = glp_alloc(1 as i32, ::core::mem::size_of::<LUF>() as u64 as i32)
                as *mut LUF;
            luf = (*fi).luf;
            memset(
                luf as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<LUF>() as u64,
            );
            (*luf).sva = sva;
        } else {
            glp_free((*luf).vr_piv as *mut libc::c_void);
            glp_free((*luf).pp_ind as *mut libc::c_void);
            glp_free((*luf).pp_inv as *mut libc::c_void);
            glp_free((*luf).qq_ind as *mut libc::c_void);
            glp_free((*luf).qq_inv as *mut libc::c_void);
        }
        (*luf).vr_piv = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        (*luf).pp_ind = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*luf).pp_inv = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*luf).qq_ind = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*luf).qq_inv = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        sgf = (*fi).sgf;
        if sgf.is_null() {
            (*fi).sgf = glp_alloc(1 as i32, ::core::mem::size_of::<SGF>() as u64 as i32)
                as *mut SGF;
            sgf = (*fi).sgf;
            memset(
                sgf as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<SGF>() as u64,
            );
            (*sgf).luf = luf;
        } else {
            glp_free((*sgf).rs_head as *mut libc::c_void);
            glp_free((*sgf).rs_prev as *mut libc::c_void);
            glp_free((*sgf).rs_next as *mut libc::c_void);
            glp_free((*sgf).cs_head as *mut libc::c_void);
            glp_free((*sgf).cs_prev as *mut libc::c_void);
            glp_free((*sgf).cs_next as *mut libc::c_void);
            glp_free((*sgf).vr_max as *mut libc::c_void);
            glp_free((*sgf).flag as *mut libc::c_void);
            glp_free((*sgf).work as *mut libc::c_void);
        }
        (*sgf).rs_head = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*sgf).rs_prev = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*sgf).rs_next = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*sgf).cs_head = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*sgf).cs_prev = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*sgf).cs_next = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*sgf).vr_max = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        (*sgf).flag = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i8>() as u64 as i32,
        ) as *mut i8;
        (*sgf).work = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
    }
    luf = (*fi).luf;
    sgf = (*fi).sgf;
    (*sva).n = 0 as i32;
    (*sva).m_ptr = 1 as i32;
    (*sva).r_ptr = (*sva).size + 1 as i32;
    (*sva).tail = 0 as i32;
    (*sva).head = (*sva).tail;
    (*luf).n = n;
    (*luf).fr_ref = _glp_sva_alloc_vecs(sva, n);
    (*luf).fc_ref = _glp_sva_alloc_vecs(sva, n);
    (*luf).vr_ref = _glp_sva_alloc_vecs(sva, n);
    (*luf).vc_ref = _glp_sva_alloc_vecs(sva, n);
    _glp_luf_store_v_cols(luf, col, info, (*sgf).rs_prev, (*sgf).work);
    (*sgf).updat = (*fi).sgf_updat;
    (*sgf).piv_tol = (*fi).sgf_piv_tol;
    (*sgf).piv_lim = (*fi).sgf_piv_lim;
    (*sgf).suhl = (*fi).sgf_suhl;
    (*sgf).eps_tol = (*fi).sgf_eps_tol;
    k = _glp_sgf_factorize(sgf, 1 as i32);
    if k == 0 as i32 {
        (*fi).valid = 1 as i32;
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_lufint_delete(mut fi: *mut LUFINT) {
    let mut sva: *mut SVA = (*fi).sva;
    let mut luf: *mut LUF = (*fi).luf;
    let mut sgf: *mut SGF = (*fi).sgf;
    if !sva.is_null() {
        _glp_sva_delete_area(sva);
    }
    if !luf.is_null() {
        glp_free((*luf).vr_piv as *mut libc::c_void);
        glp_free((*luf).pp_ind as *mut libc::c_void);
        glp_free((*luf).pp_inv as *mut libc::c_void);
        glp_free((*luf).qq_ind as *mut libc::c_void);
        glp_free((*luf).qq_inv as *mut libc::c_void);
        glp_free(luf as *mut libc::c_void);
    }
    if !sgf.is_null() {
        glp_free((*sgf).rs_head as *mut libc::c_void);
        glp_free((*sgf).rs_prev as *mut libc::c_void);
        glp_free((*sgf).rs_next as *mut libc::c_void);
        glp_free((*sgf).cs_head as *mut libc::c_void);
        glp_free((*sgf).cs_prev as *mut libc::c_void);
        glp_free((*sgf).cs_next as *mut libc::c_void);
        glp_free((*sgf).vr_max as *mut libc::c_void);
        glp_free((*sgf).flag as *mut libc::c_void);
        glp_free((*sgf).work as *mut libc::c_void);
        glp_free(sgf as *mut libc::c_void);
    }
    glp_free(fi as *mut libc::c_void);
}