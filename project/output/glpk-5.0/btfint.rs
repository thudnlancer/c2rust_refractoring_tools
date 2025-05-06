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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_sva_more_space(sva: *mut SVA, m_size: i32);
    fn _glp_sva_enlarge_cap(sva: *mut SVA, k: i32, new_cap: i32, skip: i32);
    fn _glp_sva_reserve_cap(sva: *mut SVA, k: i32, new_cap: i32);
    fn _glp_sva_delete_area(sva: *mut SVA);
    fn _glp_btf_store_a_cols(
        btf: *mut BTF,
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
    fn _glp_btf_make_blocks(btf: *mut BTF) -> i32;
    fn _glp_btf_build_a_rows(btf: *mut BTF, len: *mut i32);
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
pub struct BTF {
    pub n: i32,
    pub sva: *mut SVA,
    pub pp_ind: *mut i32,
    pub pp_inv: *mut i32,
    pub qq_ind: *mut i32,
    pub qq_inv: *mut i32,
    pub num: i32,
    pub beg: *mut i32,
    pub ar_ref: i32,
    pub ac_ref: i32,
    pub fr_ref: i32,
    pub fc_ref: i32,
    pub vr_ref: i32,
    pub vr_piv: *mut libc::c_double,
    pub vc_ref: i32,
    pub p1_ind: *mut i32,
    pub p1_inv: *mut i32,
    pub q1_ind: *mut i32,
    pub q1_inv: *mut i32,
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
pub struct BTFINT {
    pub n_max: i32,
    pub valid: i32,
    pub sva: *mut SVA,
    pub btf: *mut BTF,
    pub sgf: *mut SGF,
    pub sva_n_max: i32,
    pub sva_size: i32,
    pub delta_n0: i32,
    pub delta_n: i32,
    pub sgf_piv_tol: libc::c_double,
    pub sgf_piv_lim: i32,
    pub sgf_suhl: i32,
    pub sgf_eps_tol: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_btfint_create() -> *mut BTFINT {
    let mut fi: *mut BTFINT = 0 as *mut BTFINT;
    fi = glp_alloc(1 as i32, ::core::mem::size_of::<BTFINT>() as u64 as i32)
        as *mut BTFINT;
    (*fi).n_max = 0 as i32;
    (*fi).valid = 0 as i32;
    (*fi).sva = 0 as *mut SVA;
    (*fi).btf = 0 as *mut BTF;
    (*fi).sgf = 0 as *mut SGF;
    (*fi).sva_size = 0 as i32;
    (*fi).sva_n_max = (*fi).sva_size;
    (*fi).delta_n = 0 as i32;
    (*fi).delta_n0 = (*fi).delta_n;
    (*fi).sgf_piv_tol = 0.10f64;
    (*fi).sgf_piv_lim = 4 as i32;
    (*fi).sgf_suhl = 1 as i32;
    (*fi).sgf_eps_tol = 2.2204460492503131e-16f64;
    return fi;
}
unsafe extern "C" fn factorize_triv(
    mut fi: *mut BTFINT,
    mut k: i32,
    mut col: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            i32,
            *mut i32,
            *mut libc::c_double,
        ) -> i32,
    >,
    mut info: *mut libc::c_void,
) {
    let mut sva: *mut SVA = (*fi).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut btf: *mut BTF = (*fi).btf;
    let mut pp_inv: *mut i32 = (*btf).pp_inv;
    let mut qq_ind: *mut i32 = (*btf).qq_ind;
    let mut beg: *mut i32 = (*btf).beg;
    let mut ac_ref: i32 = (*btf).ac_ref;
    let mut ac_ptr: *mut i32 = &mut *((*sva).ptr).offset((ac_ref - 1 as i32) as isize)
        as *mut i32;
    let mut ac_len: *mut i32 = &mut *((*sva).len).offset((ac_ref - 1 as i32) as isize)
        as *mut i32;
    let mut sgf: *mut SGF = (*fi).sgf;
    let mut ind: *mut i32 = (*sgf).vr_max as *mut i32;
    let mut val: *mut libc::c_double = (*sgf).work;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut t: i32 = 0;
    let mut len: i32 = 0;
    let mut ptr: i32 = 0;
    let mut beg_k: i32 = 0;
    beg_k = *beg.offset(k as isize);
    i = *pp_inv.offset(beg_k as isize);
    j = *qq_ind.offset(beg_k as isize);
    len = col.expect("non-null function pointer")(info, j, ind, val);
    t = 1 as i32;
    while t <= len {
        if *ind.offset(t as isize) == i {
            break;
        }
        t += 1;
        t;
    }
    (t <= len
        || {
            glp_assert_(
                b"t <= len\0" as *const u8 as *const i8,
                b"bflib/btfint.c\0" as *const u8 as *const i8,
                74 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*val.offset(t as isize) != 0.0f64
        || {
            glp_assert_(
                b"val[t] != 0.0\0" as *const u8 as *const i8,
                b"bflib/btfint.c\0" as *const u8 as *const i8,
                78 as i32,
            );
            1 as i32 != 0
        }) as i32;
    *((*btf).vr_piv).offset(beg_k as isize) = *val.offset(t as isize);
    let ref mut fresh0 = *((*btf).p1_inv).offset(beg_k as isize);
    *fresh0 = 1 as i32;
    *((*btf).p1_ind).offset(beg_k as isize) = *fresh0;
    let ref mut fresh1 = *((*btf).q1_inv).offset(beg_k as isize);
    *fresh1 = 1 as i32;
    *((*btf).q1_ind).offset(beg_k as isize) = *fresh1;
    memmove(
        &mut *ind.offset(t as isize) as *mut i32 as *mut libc::c_void,
        &mut *ind.offset((t + 1 as i32) as isize) as *mut i32 as *const libc::c_void,
        ((len - t) as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    memmove(
        &mut *val.offset(t as isize) as *mut libc::c_double as *mut libc::c_void,
        &mut *val.offset((t + 1 as i32) as isize) as *mut libc::c_double
            as *const libc::c_void,
        ((len - t) as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    len -= 1;
    len;
    if len > 0 as i32 {
        if (*sva).r_ptr - (*sva).m_ptr < len {
            _glp_sva_more_space(sva, len);
            sv_ind = (*sva).ind;
            sv_val = (*sva).val;
        }
        _glp_sva_reserve_cap(sva, ac_ref + (j - 1 as i32), len);
        ptr = *ac_ptr.offset(j as isize);
        memcpy(
            &mut *sv_ind.offset(ptr as isize) as *mut i32 as *mut libc::c_void,
            &mut *ind.offset(1 as i32 as isize) as *mut i32 as *const libc::c_void,
            (len as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
        );
        memcpy(
            &mut *sv_val.offset(ptr as isize) as *mut libc::c_double
                as *mut libc::c_void,
            &mut *val.offset(1 as i32 as isize) as *mut libc::c_double
                as *const libc::c_void,
            (len as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
        );
        *ac_len.offset(j as isize) = len;
    }
}
unsafe extern "C" fn factorize_block(
    mut fi: *mut BTFINT,
    mut k: i32,
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
    let mut sva: *mut SVA = (*fi).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut btf: *mut BTF = (*fi).btf;
    let mut pp_ind: *mut i32 = (*btf).pp_ind;
    let mut qq_ind: *mut i32 = (*btf).qq_ind;
    let mut beg: *mut i32 = (*btf).beg;
    let mut ac_ref: i32 = (*btf).ac_ref;
    let mut ac_ptr: *mut i32 = &mut *((*sva).ptr).offset((ac_ref - 1 as i32) as isize)
        as *mut i32;
    let mut ac_len: *mut i32 = &mut *((*sva).len).offset((ac_ref - 1 as i32) as isize)
        as *mut i32;
    let mut sgf: *mut SGF = (*fi).sgf;
    let mut ind: *mut i32 = (*sgf).vr_max as *mut i32;
    let mut val: *mut libc::c_double = (*sgf).work;
    let mut luf: LUF = LUF {
        n: 0,
        sva: 0 as *mut SVA,
        fr_ref: 0,
        fc_ref: 0,
        vr_ref: 0,
        vr_piv: 0 as *mut libc::c_double,
        vc_ref: 0,
        pp_ind: 0 as *mut i32,
        pp_inv: 0 as *mut i32,
        qq_ind: 0 as *mut i32,
        qq_inv: 0 as *mut i32,
    };
    let mut vc_ptr: *mut i32 = 0 as *mut i32;
    let mut vc_len: *mut i32 = 0 as *mut i32;
    let mut vc_cap: *mut i32 = 0 as *mut i32;
    let mut i: i32 = 0;
    let mut ii: i32 = 0;
    let mut j: i32 = 0;
    let mut jj: i32 = 0;
    let mut t: i32 = 0;
    let mut len: i32 = 0;
    let mut cnt: i32 = 0;
    let mut ptr: i32 = 0;
    let mut beg_k: i32 = 0;
    (*sgf).luf = &mut luf;
    beg_k = *beg.offset(k as isize);
    luf.n = *beg.offset((k + 1 as i32) as isize) - beg_k;
    luf.sva = sva;
    luf.fr_ref = (*btf).fr_ref + (beg_k - 1 as i32);
    luf.fc_ref = (*btf).fc_ref + (beg_k - 1 as i32);
    luf.vr_ref = (*btf).vr_ref + (beg_k - 1 as i32);
    luf.vr_piv = ((*btf).vr_piv).offset((beg_k - 1 as i32) as isize);
    luf.vc_ref = (*btf).vc_ref + (beg_k - 1 as i32);
    luf.pp_ind = ((*btf).p1_ind).offset((beg_k - 1 as i32) as isize);
    luf.pp_inv = ((*btf).p1_inv).offset((beg_k - 1 as i32) as isize);
    luf.qq_ind = ((*btf).q1_ind).offset((beg_k - 1 as i32) as isize);
    luf.qq_inv = ((*btf).q1_inv).offset((beg_k - 1 as i32) as isize);
    vc_ptr = &mut *((*sva).ptr).offset((luf.vc_ref - 1 as i32) as isize) as *mut i32;
    vc_len = &mut *((*sva).len).offset((luf.vc_ref - 1 as i32) as isize) as *mut i32;
    vc_cap = &mut *((*sva).cap).offset((luf.vc_ref - 1 as i32) as isize) as *mut i32;
    jj = 1 as i32;
    while jj <= luf.n {
        j = *qq_ind.offset((jj + (beg_k - 1 as i32)) as isize);
        len = col.expect("non-null function pointer")(info, j, ind, val);
        cnt = 0 as i32;
        t = 1 as i32;
        while t <= len {
            i = *ind.offset(t as isize);
            ii = *pp_ind.offset(i as isize);
            if ii >= beg_k {
                let mut temp: libc::c_double = 0.;
                cnt += 1;
                cnt;
                *ind.offset(t as isize) = *ind.offset(cnt as isize);
                *ind.offset(cnt as isize) = ii - (beg_k - 1 as i32);
                temp = *val.offset(t as isize);
                *val.offset(t as isize) = *val.offset(cnt as isize);
                *val.offset(cnt as isize) = temp;
            }
            t += 1;
            t;
        }
        if *vc_cap.offset(jj as isize) < cnt {
            if (*sva).r_ptr - (*sva).m_ptr < cnt {
                _glp_sva_more_space(sva, cnt);
                sv_ind = (*sva).ind;
                sv_val = (*sva).val;
            }
            _glp_sva_enlarge_cap(sva, luf.vc_ref + (jj - 1 as i32), cnt, 0 as i32);
        }
        ptr = *vc_ptr.offset(jj as isize);
        memcpy(
            &mut *sv_ind.offset(ptr as isize) as *mut i32 as *mut libc::c_void,
            &mut *ind.offset(1 as i32 as isize) as *mut i32 as *const libc::c_void,
            (cnt as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
        );
        memcpy(
            &mut *sv_val.offset(ptr as isize) as *mut libc::c_double
                as *mut libc::c_void,
            &mut *val.offset(1 as i32 as isize) as *mut libc::c_double
                as *const libc::c_void,
            (cnt as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
        );
        *vc_len.offset(jj as isize) = cnt;
        len -= cnt;
        if len > 0 as i32 {
            if (*sva).r_ptr - (*sva).m_ptr < len {
                _glp_sva_more_space(sva, len);
                sv_ind = (*sva).ind;
                sv_val = (*sva).val;
            }
            _glp_sva_reserve_cap(sva, ac_ref - 1 as i32 + j, len);
            ptr = *ac_ptr.offset(j as isize);
            memcpy(
                &mut *sv_ind.offset(ptr as isize) as *mut i32 as *mut libc::c_void,
                &mut *ind.offset((cnt + 1 as i32) as isize) as *mut i32
                    as *const libc::c_void,
                (len as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
            );
            memcpy(
                &mut *sv_val.offset(ptr as isize) as *mut libc::c_double
                    as *mut libc::c_void,
                &mut *val.offset((cnt + 1 as i32) as isize) as *mut libc::c_double
                    as *const libc::c_void,
                (len as u64)
                    .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
            );
            *ac_len.offset(j as isize) = len;
        }
        jj += 1;
        jj;
    }
    k = _glp_sgf_factorize(sgf, 0 as i32);
    ((*sva).m_ptr == 1 as i32
        || {
            glp_assert_(
                b"sva->m_ptr == 1\0" as *const u8 as *const i8,
                b"bflib/btfint.c\0" as *const u8 as *const i8,
                204 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_btfint_factorize(
    mut fi: *mut BTFINT,
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
    let mut btf: *mut BTF = 0 as *mut BTF;
    let mut sgf: *mut SGF = 0 as *mut SGF;
    let mut k: i32 = 0;
    let mut rank: i32 = 0;
    (n > 0 as i32
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const i8,
                b"bflib/btfint.c\0" as *const u8 as *const i8,
                215 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*fi).valid = 0 as i32;
    sva = (*fi).sva;
    if sva.is_null() {
        let mut sva_n_max: i32 = (*fi).sva_n_max;
        let mut sva_size: i32 = (*fi).sva_size;
        if sva_n_max == 0 as i32 {
            sva_n_max = 6 as i32 * n;
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
                    b"bflib/btfint.c\0" as *const u8 as *const i8,
                    235 as i32,
                );
                1 as i32 != 0
            }) as i32;
        btf = (*fi).btf;
        if btf.is_null() {
            (*fi).btf = glp_alloc(1 as i32, ::core::mem::size_of::<BTF>() as u64 as i32)
                as *mut BTF;
            btf = (*fi).btf;
            memset(
                btf as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<BTF>() as u64,
            );
            (*btf).sva = sva;
        } else {
            glp_free((*btf).pp_ind as *mut libc::c_void);
            glp_free((*btf).pp_inv as *mut libc::c_void);
            glp_free((*btf).qq_ind as *mut libc::c_void);
            glp_free((*btf).qq_inv as *mut libc::c_void);
            glp_free((*btf).beg as *mut libc::c_void);
            glp_free((*btf).vr_piv as *mut libc::c_void);
            glp_free((*btf).p1_ind as *mut libc::c_void);
            glp_free((*btf).p1_inv as *mut libc::c_void);
            glp_free((*btf).q1_ind as *mut libc::c_void);
            glp_free((*btf).q1_inv as *mut libc::c_void);
        }
        (*btf).pp_ind = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*btf).pp_inv = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*btf).qq_ind = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*btf).qq_inv = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*btf).beg = glp_alloc(
            1 as i32 + n_max + 1 as i32,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*btf).vr_piv = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        (*btf).p1_ind = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*btf).p1_inv = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*btf).q1_ind = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*btf).q1_inv = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        sgf = (*fi).sgf;
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
    btf = (*fi).btf;
    (*btf).n = n;
    sgf = (*fi).sgf;
    (*sva).n = 0 as i32;
    (*sva).m_ptr = 1 as i32;
    (*sva).r_ptr = (*sva).size + 1 as i32;
    (*sva).tail = 0 as i32;
    (*sva).head = (*sva).tail;
    (*btf).ac_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    _glp_btf_store_a_cols(btf, col, info, (*btf).pp_ind, (*btf).vr_piv);
    rank = _glp_btf_make_blocks(btf);
    if rank != n {
        return 1 as i32;
    }
    (*sva).n = 0 as i32;
    (*sva).m_ptr = 1 as i32;
    (*sva).r_ptr = (*sva).size + 1 as i32;
    (*sva).tail = 0 as i32;
    (*sva).head = (*sva).tail;
    (*btf).ar_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    (*btf).ac_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    (*btf).fr_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    (*btf).fc_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    (*btf).vr_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    (*btf).vc_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    (*sgf).updat = 0 as i32;
    (*sgf).piv_tol = (*fi).sgf_piv_tol;
    (*sgf).piv_lim = (*fi).sgf_piv_lim;
    (*sgf).suhl = (*fi).sgf_suhl;
    (*sgf).eps_tol = (*fi).sgf_eps_tol;
    k = 1 as i32;
    while k <= (*btf).num {
        if *((*btf).beg).offset((k + 1 as i32) as isize)
            - *((*btf).beg).offset(k as isize) == 1 as i32
        {
            factorize_triv(fi, k, col, info);
        } else if factorize_block(fi, k, col, info) != 0 as i32 {
            return 2 as i32
        }
        k += 1;
        k;
    }
    _glp_btf_build_a_rows((*fi).btf, (*(*fi).sgf).rs_head);
    (*fi).valid = 1 as i32;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_btfint_delete(mut fi: *mut BTFINT) {
    let mut sva: *mut SVA = (*fi).sva;
    let mut btf: *mut BTF = (*fi).btf;
    let mut sgf: *mut SGF = (*fi).sgf;
    if !sva.is_null() {
        _glp_sva_delete_area(sva);
    }
    if !btf.is_null() {
        glp_free((*btf).pp_ind as *mut libc::c_void);
        glp_free((*btf).pp_inv as *mut libc::c_void);
        glp_free((*btf).qq_ind as *mut libc::c_void);
        glp_free((*btf).qq_inv as *mut libc::c_void);
        glp_free((*btf).beg as *mut libc::c_void);
        glp_free((*btf).vr_piv as *mut libc::c_void);
        glp_free((*btf).p1_ind as *mut libc::c_void);
        glp_free((*btf).p1_inv as *mut libc::c_void);
        glp_free((*btf).q1_ind as *mut libc::c_void);
        glp_free((*btf).q1_inv as *mut libc::c_void);
        glp_free(btf as *mut libc::c_void);
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