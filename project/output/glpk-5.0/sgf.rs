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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_luf_build_v_rows(luf: *mut LUF, len: *mut i32);
    fn _glp_luf_build_f_rows(luf: *mut LUF, len: *mut i32);
    fn _glp_luf_build_v_cols(luf: *mut LUF, updat: i32, len: *mut i32);
    fn _glp_sva_defrag_area(sva: *mut SVA);
    fn _glp_sva_more_space(sva: *mut SVA, m_size: i32);
    fn _glp_sva_enlarge_cap(sva: *mut SVA, k: i32, new_cap: i32, skip: i32);
    fn _glp_sva_reserve_cap(sva: *mut SVA, k: i32, new_cap: i32);
    fn _glp_sva_make_static(sva: *mut SVA, k: i32);
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
#[no_mangle]
pub unsafe extern "C" fn _glp_sgf_reduce_nuc(
    mut luf: *mut LUF,
    mut k1_: *mut i32,
    mut k2_: *mut i32,
    mut cnt: *mut i32,
    mut list: *mut i32,
) -> i32 {
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut vr_ref: i32 = (*luf).vr_ref;
    let mut vr_ptr: *mut i32 = &mut *((*sva).ptr).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_len: *mut i32 = &mut *((*sva).len).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vc_ref: i32 = (*luf).vc_ref;
    let mut vc_ptr: *mut i32 = &mut *((*sva).ptr).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vc_len: *mut i32 = &mut *((*sva).len).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut pp_ind: *mut i32 = (*luf).pp_ind;
    let mut pp_inv: *mut i32 = (*luf).pp_inv;
    let mut qq_ind: *mut i32 = (*luf).qq_ind;
    let mut qq_inv: *mut i32 = (*luf).qq_inv;
    let mut i: i32 = 0;
    let mut ii: i32 = 0;
    let mut j: i32 = 0;
    let mut jj: i32 = 0;
    let mut k1: i32 = 0;
    let mut k2: i32 = 0;
    let mut ns: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    k1 = 1 as i32;
    k2 = n;
    ns = 0 as i32;
    j = 1 as i32;
    while j <= n {
        let ref mut fresh0 = *cnt.offset(j as isize);
        *fresh0 = *vc_len.offset(j as isize);
        if *fresh0 == 1 as i32 {
            ns += 1;
            *list.offset(ns as isize) = j;
        }
        j += 1;
        j;
    }
    while ns > 0 as i32 {
        let fresh1 = ns;
        ns = ns - 1;
        j = *list.offset(fresh1 as isize);
        if *cnt.offset(j as isize) == 0 as i32 {
            return 1 as i32;
        }
        ptr = *vc_ptr.offset(j as isize);
        end = ptr + *vc_len.offset(j as isize);
        loop {
            i = *sv_ind.offset(ptr as isize);
            if !(*pp_ind.offset(i as isize) < k1) {
                break;
            }
            ptr += 1;
            ptr;
        }
        (ptr < end
            || {
                glp_assert_(
                    b"ptr < end\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    112 as i32,
                );
                1 as i32 != 0
            }) as i32;
        ii = *pp_ind.offset(i as isize);
        let mut j1: i32 = 0;
        let mut j2: i32 = 0;
        j1 = *pp_inv.offset(k1 as isize);
        j2 = *pp_inv.offset(ii as isize);
        *pp_ind.offset(j1 as isize) = ii;
        *pp_inv.offset(ii as isize) = j1;
        *pp_ind.offset(j2 as isize) = k1;
        *pp_inv.offset(k1 as isize) = j2;
        jj = *qq_inv.offset(j as isize);
        let mut i1: i32 = 0;
        let mut i2: i32 = 0;
        i1 = *qq_ind.offset(k1 as isize);
        i2 = *qq_ind.offset(jj as isize);
        *qq_ind.offset(k1 as isize) = i2;
        *qq_inv.offset(i2 as isize) = k1;
        *qq_ind.offset(jj as isize) = i1;
        *qq_inv.offset(i1 as isize) = jj;
        k1 += 1;
        k1;
        ptr = *vr_ptr.offset(i as isize);
        end = ptr + *vr_len.offset(i as isize);
        while ptr < end {
            j = *sv_ind.offset(ptr as isize);
            let ref mut fresh2 = *cnt.offset(j as isize);
            *fresh2 -= 1;
            if *fresh2 == 1 as i32 {
                ns += 1;
                *list.offset(ns as isize) = j;
            }
            ptr += 1;
            ptr;
        }
    }
    if !(k1 > n) {
        ns = 0 as i32;
        i = 1 as i32;
        while i <= n {
            if *pp_ind.offset(i as isize) < k1 {
                *cnt.offset(i as isize) = 0 as i32;
            } else {
                let ref mut fresh3 = *cnt.offset(i as isize);
                *fresh3 = *vr_len.offset(i as isize);
                if *fresh3 == 1 as i32 {
                    ns += 1;
                    *list.offset(ns as isize) = i;
                }
            }
            i += 1;
            i;
        }
        while ns > 0 as i32 {
            let fresh4 = ns;
            ns = ns - 1;
            i = *list.offset(fresh4 as isize);
            if *cnt.offset(i as isize) == 0 as i32 {
                return 2 as i32;
            }
            ptr = *vr_ptr.offset(i as isize);
            end = ptr + *vr_len.offset(i as isize);
            loop {
                j = *sv_ind.offset(ptr as isize);
                if !(*qq_inv.offset(j as isize) > k2) {
                    break;
                }
                ptr += 1;
                ptr;
            }
            (ptr < end
                || {
                    glp_assert_(
                        b"ptr < end\0" as *const u8 as *const i8,
                        b"bflib/sgf.c\0" as *const u8 as *const i8,
                        166 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            ii = *pp_ind.offset(i as isize);
            let mut j1_0: i32 = 0;
            let mut j2_0: i32 = 0;
            j1_0 = *pp_inv.offset(k2 as isize);
            j2_0 = *pp_inv.offset(ii as isize);
            *pp_ind.offset(j1_0 as isize) = ii;
            *pp_inv.offset(ii as isize) = j1_0;
            *pp_ind.offset(j2_0 as isize) = k2;
            *pp_inv.offset(k2 as isize) = j2_0;
            jj = *qq_inv.offset(j as isize);
            let mut i1_0: i32 = 0;
            let mut i2_0: i32 = 0;
            i1_0 = *qq_ind.offset(k2 as isize);
            i2_0 = *qq_ind.offset(jj as isize);
            *qq_ind.offset(k2 as isize) = i2_0;
            *qq_inv.offset(i2_0 as isize) = k2;
            *qq_ind.offset(jj as isize) = i1_0;
            *qq_inv.offset(i1_0 as isize) = jj;
            k2 -= 1;
            k2;
            ptr = *vc_ptr.offset(j as isize);
            end = ptr + *vc_len.offset(j as isize);
            while ptr < end {
                i = *sv_ind.offset(ptr as isize);
                let ref mut fresh5 = *cnt.offset(i as isize);
                *fresh5 -= 1;
                if *fresh5 == 1 as i32 {
                    ns += 1;
                    *list.offset(ns as isize) = i;
                }
                ptr += 1;
                ptr;
            }
        }
        (k1 < k2
            || {
                glp_assert_(
                    b"k1 < k2\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    185 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    *k1_ = k1;
    *k2_ = k2;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sgf_singl_phase(
    mut luf: *mut LUF,
    mut k1: i32,
    mut k2: i32,
    mut updat: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fc_ref: i32 = (*luf).fc_ref;
    let mut fc_ptr: *mut i32 = &mut *((*sva).ptr).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut fc_len: *mut i32 = &mut *((*sva).len).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_ref: i32 = (*luf).vr_ref;
    let mut vr_ptr: *mut i32 = &mut *((*sva).ptr).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_len: *mut i32 = &mut *((*sva).len).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_piv: *mut libc::c_double = (*luf).vr_piv;
    let mut vc_ref: i32 = (*luf).vc_ref;
    let mut vc_ptr: *mut i32 = &mut *((*sva).ptr).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vc_len: *mut i32 = &mut *((*sva).len).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut pp_ind: *mut i32 = (*luf).pp_ind;
    let mut pp_inv: *mut i32 = (*luf).pp_inv;
    let mut qq_ind: *mut i32 = (*luf).qq_ind;
    let mut qq_inv: *mut i32 = (*luf).qq_inv;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut ptr1: i32 = 0;
    let mut end: i32 = 0;
    let mut len: i32 = 0;
    let mut piv: libc::c_double = 0.;
    (1 as i32 <= k1 && k1 < k2 && k2 <= n || k1 == n + 1 as i32 && k2 == n
        || {
            glp_assert_(
                b"(1 <= k1 && k1 < k2 && k2 <= n) || (k1 == n+1 && k2 == n)\0"
                    as *const u8 as *const i8,
                b"bflib/sgf.c\0" as *const u8 as *const i8,
                282 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = k1;
    while k <= k2 {
        let ref mut fresh6 = *qq_inv.offset(*qq_ind.offset(k as isize) as isize);
        *fresh6 = k - k2 + n;
        *pp_ind.offset(*pp_inv.offset(k as isize) as isize) = *fresh6;
        k += 1;
        k;
    }
    k = k2 + 1 as i32;
    while k <= n {
        let ref mut fresh7 = *qq_inv.offset(*qq_ind.offset(k as isize) as isize);
        *fresh7 = n - k + k1;
        *pp_ind.offset(*pp_inv.offset(k as isize) as isize) = *fresh7;
        k += 1;
        k;
    }
    k = 1 as i32;
    while k <= n {
        let ref mut fresh8 = *qq_ind.offset(*qq_inv.offset(k as isize) as isize);
        *fresh8 = k;
        *pp_inv.offset(*pp_ind.offset(k as isize) as isize) = *fresh8;
        k += 1;
        k;
    }
    k2 = n - k2 + k1;
    k = 1 as i32;
    while k < k1 {
        i = *pp_inv.offset(k as isize);
        ptr = *vr_ptr.offset(i as isize);
        end = ptr + *vr_len.offset(i as isize);
        while *qq_inv.offset(*sv_ind.offset(ptr as isize) as isize) != k {
            ptr += 1;
            ptr;
        }
        (ptr < end
            || {
                glp_assert_(
                    b"ptr < end\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    302 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *vr_piv.offset(i as isize) = *sv_val.offset(ptr as isize);
        *sv_ind.offset(ptr as isize) = *sv_ind.offset((end - 1 as i32) as isize);
        *sv_val.offset(ptr as isize) = *sv_val.offset((end - 1 as i32) as isize);
        let ref mut fresh9 = *vr_len.offset(i as isize);
        *fresh9 -= 1;
        *fresh9;
        *vc_len.offset(*qq_ind.offset(k as isize) as isize) = 0 as i32;
        k += 1;
        k;
    }
    k = k1;
    while k < k2 {
        *vr_len.offset(*pp_inv.offset(k as isize) as isize) = 0 as i32;
        k += 1;
        k;
    }
    k = k2;
    while k <= n {
        i = *pp_inv.offset(k as isize);
        ptr1 = *vr_ptr.offset(i as isize);
        ptr = ptr1;
        end = ptr + *vr_len.offset(i as isize);
        while ptr < end {
            if *qq_inv.offset(*sv_ind.offset(ptr as isize) as isize) >= k2 {
                *sv_ind.offset(ptr1 as isize) = *sv_ind.offset(ptr as isize);
                *sv_val.offset(ptr1 as isize) = *sv_val.offset(ptr as isize);
                ptr1 += 1;
                ptr1;
            }
            ptr += 1;
            ptr;
        }
        *vr_len.offset(i as isize) = ptr1 - *vr_ptr.offset(i as isize);
        j = *qq_ind.offset(k as isize);
        ptr1 = *vc_ptr.offset(j as isize);
        ptr = ptr1;
        end = ptr + *vc_len.offset(j as isize);
        while ptr < end {
            if *pp_ind.offset(*sv_ind.offset(ptr as isize) as isize) >= k2 {
                let fresh10 = ptr1;
                ptr1 = ptr1 + 1;
                *sv_ind.offset(fresh10 as isize) = *sv_ind.offset(ptr as isize);
            }
            ptr += 1;
            ptr;
        }
        *vc_len.offset(j as isize) = ptr1 - *vc_ptr.offset(j as isize);
        k += 1;
        k;
    }
    k = k1;
    while k < k2 {
        j = *qq_ind.offset(k as isize);
        len = 0 as i32;
        piv = 0.0f64;
        ptr = *vc_ptr.offset(j as isize);
        end = ptr + *vc_len.offset(j as isize);
        while ptr < end {
            i = *sv_ind.offset(ptr as isize);
            if *pp_ind.offset(i as isize) == k {
                let ref mut fresh11 = *vr_piv.offset(i as isize);
                *fresh11 = *sv_val.offset(ptr as isize);
                piv = *fresh11;
            } else if *pp_ind.offset(i as isize) > k {
                len += 1;
                len;
                *ind.offset(len as isize) = i;
                *val.offset(len as isize) = *sv_val.offset(ptr as isize);
            }
            ptr += 1;
            ptr;
        }
        *vc_len.offset(j as isize) = 0 as i32;
        j = *pp_inv.offset(k as isize);
        (piv != 0.0f64
            || {
                glp_assert_(
                    b"piv != 0.0\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    375 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if len > 0 as i32 {
            if (*sva).r_ptr - (*sva).m_ptr < len {
                _glp_sva_more_space(sva, len);
                sv_ind = (*sva).ind;
                sv_val = (*sva).val;
            }
            _glp_sva_reserve_cap(sva, fc_ref - 1 as i32 + j, len);
            ptr = *fc_ptr.offset(j as isize);
            ptr1 = 1 as i32;
            while ptr1 <= len {
                *sv_ind.offset(ptr as isize) = *ind.offset(ptr1 as isize);
                *sv_val.offset(ptr as isize) = *val.offset(ptr1 as isize) / piv;
                ptr += 1;
                ptr;
                ptr1 += 1;
                ptr1;
            }
            *fc_len.offset(j as isize) = len;
        }
        k += 1;
        k;
    }
    if updat == 0 {
        k = 1 as i32;
        while k < k2 {
            i = *pp_inv.offset(k as isize);
            len = *vr_len.offset(i as isize);
            if (*sva).r_ptr - (*sva).m_ptr < len {
                _glp_sva_more_space(sva, len);
                sv_ind = (*sva).ind;
                sv_val = (*sva).val;
            }
            _glp_sva_make_static(sva, vr_ref - 1 as i32 + i);
            k += 1;
            k;
        }
    }
    return k2;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sgf_choose_pivot(
    mut sgf: *mut SGF,
    mut p_: *mut i32,
    mut q_: *mut i32,
) -> i32 {
    let mut luf: *mut LUF = (*sgf).luf;
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vr_ref: i32 = (*luf).vr_ref;
    let mut vr_ptr: *mut i32 = &mut *((*sva).ptr).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_len: *mut i32 = &mut *((*sva).len).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vc_ref: i32 = (*luf).vc_ref;
    let mut vc_ptr: *mut i32 = &mut *((*sva).ptr).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vc_len: *mut i32 = &mut *((*sva).len).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut rs_head: *mut i32 = (*sgf).rs_head;
    let mut rs_next: *mut i32 = (*sgf).rs_next;
    let mut cs_head: *mut i32 = (*sgf).cs_head;
    let mut cs_prev: *mut i32 = (*sgf).cs_prev;
    let mut cs_next: *mut i32 = (*sgf).cs_next;
    let mut vr_max: *mut libc::c_double = (*sgf).vr_max;
    let mut piv_tol: libc::c_double = (*sgf).piv_tol;
    let mut piv_lim: i32 = (*sgf).piv_lim;
    let mut suhl: i32 = (*sgf).suhl;
    let mut i: i32 = 0;
    let mut i_ptr: i32 = 0;
    let mut i_end: i32 = 0;
    let mut j: i32 = 0;
    let mut j_ptr: i32 = 0;
    let mut j_end: i32 = 0;
    let mut len: i32 = 0;
    let mut min_i: i32 = 0;
    let mut min_j: i32 = 0;
    let mut min_len: i32 = 0;
    let mut ncand: i32 = 0;
    let mut next_j: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut best: libc::c_double = 0.;
    let mut big: libc::c_double = 0.;
    let mut cost: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    q = 0 as i32;
    p = q;
    best = 1.7976931348623157e+308f64;
    ncand = 0 as i32;
    j = *cs_head.offset(1 as i32 as isize);
    if j != 0 as i32 {
        (*vc_len.offset(j as isize) == 1 as i32
            || {
                glp_assert_(
                    b"vc_len[j] == 1\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    498 as i32,
                );
                1 as i32 != 0
            }) as i32;
        p = *sv_ind.offset(*vc_ptr.offset(j as isize) as isize);
        q = j;
    } else {
        i = *rs_head.offset(1 as i32 as isize);
        if i != 0 as i32 {
            (*vr_len.offset(i as isize) == 1 as i32
                || {
                    glp_assert_(
                        b"vr_len[i] == 1\0" as *const u8 as *const i8,
                        b"bflib/sgf.c\0" as *const u8 as *const i8,
                        506 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            p = i;
            q = *sv_ind.offset(*vr_ptr.offset(i as isize) as isize);
        } else {
            len = 2 as i32;
            's_81: while len <= n {
                j = *cs_head.offset(len as isize);
                while j != 0 as i32 {
                    next_j = *cs_next.offset(j as isize);
                    min_j = 0 as i32;
                    min_i = min_j;
                    min_len = 2147483647 as i32;
                    j_ptr = *vc_ptr.offset(j as isize);
                    j_end = j_ptr + *vc_len.offset(j as isize);
                    while j_ptr < j_end {
                        i = *sv_ind.offset(j_ptr as isize);
                        if !(*vr_len.offset(i as isize) >= min_len) {
                            big = *vr_max.offset(i as isize);
                            if big < 0.0f64 {
                                i_ptr = *vr_ptr.offset(i as isize);
                                i_end = i_ptr + *vr_len.offset(i as isize);
                                while i_ptr < i_end {
                                    temp = *sv_val.offset(i_ptr as isize);
                                    if temp < 0.0f64 {
                                        temp = -temp;
                                    }
                                    if big < temp {
                                        big = temp;
                                    }
                                    i_ptr += 1;
                                    i_ptr;
                                }
                                (big > 0.0f64
                                    || {
                                        glp_assert_(
                                            b"big > 0.0\0" as *const u8 as *const i8,
                                            b"bflib/sgf.c\0" as *const u8 as *const i8,
                                            538 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                                *vr_max.offset(i as isize) = big;
                            }
                            i_ptr = *vr_ptr.offset(i as isize);
                            i_end = i_ptr + *vr_len.offset(i as isize);
                            while *sv_ind.offset(i_ptr as isize) != j {
                                i_ptr += 1;
                                i_ptr;
                            }
                            (i_ptr < i_end
                                || {
                                    glp_assert_(
                                        b"i_ptr < i_end\0" as *const u8 as *const i8,
                                        b"bflib/sgf.c\0" as *const u8 as *const i8,
                                        545 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            temp = *sv_val.offset(i_ptr as isize);
                            if temp < 0.0f64 {
                                temp = -temp;
                            }
                            if !(temp < piv_tol * big) {
                                min_i = i;
                                min_j = j;
                                min_len = *vr_len.offset(i as isize);
                                if min_len <= len {
                                    p = min_i;
                                    q = min_j;
                                    break 's_81;
                                }
                            }
                        }
                        j_ptr += 1;
                        j_ptr;
                    }
                    if min_i != 0 as i32 {
                        ncand += 1;
                        ncand;
                        cost = (min_len - 1 as i32) as libc::c_double
                            * (len - 1 as i32) as libc::c_double;
                        if cost < best {
                            p = min_i;
                            q = min_j;
                            best = cost;
                        }
                        if ncand == piv_lim {
                            break 's_81;
                        }
                    } else if suhl != 0 {
                        if *cs_prev.offset(j as isize) == 0 as i32 {
                            *cs_head.offset(*vc_len.offset(j as isize) as isize) = *cs_next
                                .offset(j as isize);
                        } else {
                            *cs_next.offset(*cs_prev.offset(j as isize) as isize) = *cs_next
                                .offset(j as isize);
                        }
                        if !(*cs_next.offset(j as isize) == 0 as i32) {
                            *cs_prev.offset(*cs_next.offset(j as isize) as isize) = *cs_prev
                                .offset(j as isize);
                        }
                        let ref mut fresh12 = *cs_next.offset(j as isize);
                        *fresh12 = -(1 as i32);
                        *cs_prev.offset(j as isize) = *fresh12;
                        let ref mut fresh13 = *cs_next.offset(j as isize);
                        *fresh13 = j;
                        *cs_prev.offset(j as isize) = *fresh13;
                    }
                    j = next_j;
                }
                i = *rs_head.offset(len as isize);
                while i != 0 as i32 {
                    big = *vr_max.offset(i as isize);
                    if big < 0.0f64 {
                        i_ptr = *vr_ptr.offset(i as isize);
                        i_end = i_ptr + *vr_len.offset(i as isize);
                        while i_ptr < i_end {
                            temp = *sv_val.offset(i_ptr as isize);
                            if temp < 0.0f64 {
                                temp = -temp;
                            }
                            if big < temp {
                                big = temp;
                            }
                            i_ptr += 1;
                            i_ptr;
                        }
                        (big > 0.0f64
                            || {
                                glp_assert_(
                                    b"big > 0.0\0" as *const u8 as *const i8,
                                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                                    600 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        *vr_max.offset(i as isize) = big;
                    }
                    min_j = 0 as i32;
                    min_i = min_j;
                    min_len = 2147483647 as i32;
                    i_ptr = *vr_ptr.offset(i as isize);
                    i_end = i_ptr + *vr_len.offset(i as isize);
                    while i_ptr < i_end {
                        j = *sv_ind.offset(i_ptr as isize);
                        if !(*vc_len.offset(j as isize) >= min_len) {
                            temp = *sv_val.offset(i_ptr as isize);
                            if temp < 0.0f64 {
                                temp = -temp;
                            }
                            if !(temp < piv_tol * big) {
                                min_i = i;
                                min_j = j;
                                min_len = *vc_len.offset(j as isize);
                                if min_len <= len {
                                    p = min_i;
                                    q = min_j;
                                    break 's_81;
                                }
                            }
                        }
                        i_ptr += 1;
                        i_ptr;
                    }
                    if min_i != 0 as i32 {
                        ncand += 1;
                        ncand;
                        cost = (len - 1 as i32) as libc::c_double
                            * (min_len - 1 as i32) as libc::c_double;
                        if cost < best {
                            p = min_i;
                            q = min_j;
                            best = cost;
                        }
                        if ncand == piv_lim {
                            break 's_81;
                        }
                    } else {
                        (min_i != min_i
                            || {
                                glp_assert_(
                                    b"min_i != min_i\0" as *const u8 as *const i8,
                                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                                    647 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                    i = *rs_next.offset(i as isize);
                }
                len += 1;
                len;
            }
        }
    }
    *p_ = p;
    *q_ = q;
    return (p == 0 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sgf_eliminate(
    mut sgf: *mut SGF,
    mut p: i32,
    mut q: i32,
) -> i32 {
    let mut luf: *mut LUF = (*sgf).luf;
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fc_ref: i32 = (*luf).fc_ref;
    let mut fc_ptr: *mut i32 = &mut *((*sva).ptr).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut fc_len: *mut i32 = &mut *((*sva).len).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_ref: i32 = (*luf).vr_ref;
    let mut vr_ptr: *mut i32 = &mut *((*sva).ptr).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_len: *mut i32 = &mut *((*sva).len).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_cap: *mut i32 = &mut *((*sva).cap).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_piv: *mut libc::c_double = (*luf).vr_piv;
    let mut vc_ref: i32 = (*luf).vc_ref;
    let mut vc_ptr: *mut i32 = &mut *((*sva).ptr).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vc_len: *mut i32 = &mut *((*sva).len).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vc_cap: *mut i32 = &mut *((*sva).cap).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut rs_head: *mut i32 = (*sgf).rs_head;
    let mut rs_prev: *mut i32 = (*sgf).rs_prev;
    let mut rs_next: *mut i32 = (*sgf).rs_next;
    let mut cs_head: *mut i32 = (*sgf).cs_head;
    let mut cs_prev: *mut i32 = (*sgf).cs_prev;
    let mut cs_next: *mut i32 = (*sgf).cs_next;
    let mut vr_max: *mut libc::c_double = (*sgf).vr_max;
    let mut flag: *mut i8 = (*sgf).flag;
    let mut work: *mut libc::c_double = (*sgf).work;
    let mut eps_tol: libc::c_double = (*sgf).eps_tol;
    let mut nnz_diff: i32 = 0 as i32;
    let mut fill: i32 = 0;
    let mut i: i32 = 0;
    let mut i_ptr: i32 = 0;
    let mut i_end: i32 = 0;
    let mut j: i32 = 0;
    let mut j_ptr: i32 = 0;
    let mut j_end: i32 = 0;
    let mut ptr: i32 = 0;
    let mut len: i32 = 0;
    let mut loc: i32 = 0;
    let mut loc1: i32 = 0;
    let mut vpq: libc::c_double = 0.;
    let mut fip: libc::c_double = 0.;
    let mut vij: libc::c_double = 0.;
    (1 as i32 <= p && p <= n
        || {
            glp_assert_(
                b"1 <= p && p <= n\0" as *const u8 as *const i8,
                b"bflib/sgf.c\0" as *const u8 as *const i8,
                742 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const i8,
                b"bflib/sgf.c\0" as *const u8 as *const i8,
                743 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if *rs_prev.offset(p as isize) == 0 as i32 {
        *rs_head.offset(*vr_len.offset(p as isize) as isize) = *rs_next
            .offset(p as isize);
    } else {
        *rs_next.offset(*rs_prev.offset(p as isize) as isize) = *rs_next
            .offset(p as isize);
    }
    if !(*rs_next.offset(p as isize) == 0 as i32) {
        *rs_prev.offset(*rs_next.offset(p as isize) as isize) = *rs_prev
            .offset(p as isize);
    }
    let ref mut fresh14 = *rs_next.offset(p as isize);
    *fresh14 = -(1 as i32);
    *rs_prev.offset(p as isize) = *fresh14;
    ptr = 0 as i32;
    i_ptr = *vr_ptr.offset(p as isize);
    i_end = i_ptr + *vr_len.offset(p as isize);
    while i_ptr < i_end {
        j = *sv_ind.offset(i_ptr as isize);
        if j == q {
            ptr = i_ptr;
        } else {
            *flag.offset(j as isize) = 1 as i32 as i8;
            *work.offset(j as isize) = *sv_val.offset(i_ptr as isize);
        }
        if *cs_next.offset(j as isize) == j {
            (*cs_prev.offset(j as isize) == j
                || {
                    glp_assert_(
                        b"cs_prev[j] == j\0" as *const u8 as *const i8,
                        b"bflib/sgf.c\0" as *const u8 as *const i8,
                        768 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        } else {
            if *cs_prev.offset(j as isize) == 0 as i32 {
                *cs_head.offset(*vc_len.offset(j as isize) as isize) = *cs_next
                    .offset(j as isize);
            } else {
                *cs_next.offset(*cs_prev.offset(j as isize) as isize) = *cs_next
                    .offset(j as isize);
            }
            if !(*cs_next.offset(j as isize) == 0 as i32) {
                *cs_prev.offset(*cs_next.offset(j as isize) as isize) = *cs_prev
                    .offset(j as isize);
            }
            let ref mut fresh15 = *cs_next.offset(j as isize);
            *fresh15 = -(1 as i32);
            *cs_prev.offset(j as isize) = *fresh15;
        }
        nnz_diff -= *vc_len.offset(j as isize);
        j_ptr = *vc_ptr.offset(j as isize);
        j_end = j_ptr + *vc_len.offset(j as isize);
        while *sv_ind.offset(j_ptr as isize) != p {
            j_ptr += 1;
            j_ptr;
        }
        (j_ptr < j_end
            || {
                glp_assert_(
                    b"j_ptr < j_end\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    777 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *sv_ind.offset(j_ptr as isize) = *sv_ind.offset((j_end - 1 as i32) as isize);
        let ref mut fresh16 = *vc_len.offset(j as isize);
        *fresh16 -= 1;
        *fresh16;
        i_ptr += 1;
        i_ptr;
    }
    (ptr > 0 as i32
        || {
            glp_assert_(
                b"ptr > 0\0" as *const u8 as *const i8,
                b"bflib/sgf.c\0" as *const u8 as *const i8,
                782 as i32,
            );
            1 as i32 != 0
        }) as i32;
    let ref mut fresh17 = *vr_piv.offset(p as isize);
    *fresh17 = *sv_val.offset(ptr as isize);
    vpq = *fresh17;
    *sv_ind.offset(ptr as isize) = *sv_ind.offset((i_end - 1 as i32) as isize);
    *sv_val.offset(ptr as isize) = *sv_val.offset((i_end - 1 as i32) as isize);
    let ref mut fresh18 = *vr_len.offset(p as isize);
    *fresh18 -= 1;
    *fresh18;
    if (*sgf).updat == 0 {
        len = *vr_len.offset(p as isize);
        if (*sva).r_ptr - (*sva).m_ptr < len {
            _glp_sva_more_space(sva, len);
            sv_ind = (*sva).ind;
            sv_val = (*sva).val;
        }
        _glp_sva_make_static(sva, vr_ref - 1 as i32 + p);
    }
    len = *vc_len.offset(q as isize);
    if len > 0 as i32 {
        if (*sva).r_ptr - (*sva).m_ptr < len {
            _glp_sva_more_space(sva, len);
            sv_ind = (*sva).ind;
            sv_val = (*sva).val;
        }
        _glp_sva_reserve_cap(sva, fc_ref - 1 as i32 + p, len);
        memcpy(
            &mut *sv_ind.offset(*fc_ptr.offset(p as isize) as isize) as *mut i32
                as *mut libc::c_void,
            &mut *sv_ind.offset(*vc_ptr.offset(q as isize) as isize) as *mut i32
                as *const libc::c_void,
            (len as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
        );
        *fc_len.offset(p as isize) = len;
    }
    *vc_len.offset(q as isize) = 0 as i32;
    loc = *fc_len.offset(p as isize) - 1 as i32;
    while loc >= 0 as i32 {
        i = *sv_ind.offset((*fc_ptr.offset(p as isize) + loc) as isize);
        (i != p
            || {
                glp_assert_(
                    b"i != p\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    819 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *rs_prev.offset(i as isize) == 0 as i32 {
            *rs_head.offset(*vr_len.offset(i as isize) as isize) = *rs_next
                .offset(i as isize);
        } else {
            *rs_next.offset(*rs_prev.offset(i as isize) as isize) = *rs_next
                .offset(i as isize);
        }
        if !(*rs_next.offset(i as isize) == 0 as i32) {
            *rs_prev.offset(*rs_next.offset(i as isize) as isize) = *rs_prev
                .offset(i as isize);
        }
        let ref mut fresh19 = *rs_next.offset(i as isize);
        *fresh19 = -(1 as i32);
        *rs_prev.offset(i as isize) = *fresh19;
        i_ptr = *vr_ptr.offset(i as isize);
        i_end = i_ptr + *vr_len.offset(i as isize);
        while *sv_ind.offset(i_ptr as isize) != q {
            i_ptr += 1;
            i_ptr;
        }
        (i_ptr < i_end
            || {
                glp_assert_(
                    b"i_ptr < i_end\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    827 as i32,
                );
                1 as i32 != 0
            }) as i32;
        let ref mut fresh20 = *sv_val
            .offset((*fc_ptr.offset(p as isize) + loc) as isize);
        *fresh20 = *sv_val.offset(i_ptr as isize) / vpq;
        fip = *fresh20;
        *sv_ind.offset(i_ptr as isize) = *sv_ind.offset((i_end - 1 as i32) as isize);
        *sv_val.offset(i_ptr as isize) = *sv_val.offset((i_end - 1 as i32) as isize);
        let ref mut fresh21 = *vr_len.offset(i as isize);
        *fresh21 -= 1;
        *fresh21;
        fill = *vr_len.offset(p as isize);
        let mut current_block_101: u64;
        ptr = *vr_ptr.offset(i as isize);
        i_ptr = ptr;
        i_end = i_ptr + *vr_len.offset(i as isize);
        while i_ptr < i_end {
            j = *sv_ind.offset(i_ptr as isize);
            vij = *sv_val.offset(i_ptr as isize);
            if *flag.offset(j as isize) != 0 {
                *flag.offset(j as isize) = 0 as i32 as i8;
                fill -= 1;
                fill;
                vij -= fip * *work.offset(j as isize);
                if -eps_tol < vij && vij < eps_tol {
                    j_ptr = *vc_ptr.offset(j as isize);
                    j_end = j_ptr + *vc_len.offset(j as isize);
                    while *sv_ind.offset(j_ptr as isize) != i {
                        j_ptr += 1;
                        j_ptr;
                    }
                    (j_ptr < j_end
                        || {
                            glp_assert_(
                                b"j_ptr < j_end\0" as *const u8 as *const i8,
                                b"bflib/sgf.c\0" as *const u8 as *const i8,
                                858 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    *sv_ind.offset(j_ptr as isize) = *sv_ind
                        .offset((j_end - 1 as i32) as isize);
                    let ref mut fresh22 = *vc_len.offset(j as isize);
                    *fresh22 -= 1;
                    *fresh22;
                    current_block_101 = 16231175055492490595;
                } else {
                    current_block_101 = 496303045384785551;
                }
            } else {
                current_block_101 = 496303045384785551;
            }
            match current_block_101 {
                496303045384785551 => {
                    *sv_ind.offset(ptr as isize) = j;
                    *sv_val.offset(ptr as isize) = vij;
                    ptr += 1;
                    ptr;
                }
                _ => {}
            }
            i_ptr += 1;
            i_ptr;
        }
        len = ptr - *vr_ptr.offset(i as isize);
        *vr_len.offset(i as isize) = len;
        if fill == 0 as i32 {
            i_ptr = *vr_ptr.offset(p as isize);
            i_end = i_ptr + *vr_len.offset(p as isize);
            while i_ptr < i_end {
                *flag.offset(*sv_ind.offset(i_ptr as isize) as isize) = 1 as i32 as i8;
                i_ptr += 1;
                i_ptr;
            }
        } else {
            if *vr_cap.offset(i as isize) < len + fill {
                if (*sva).r_ptr - (*sva).m_ptr < len + fill {
                    _glp_sva_more_space(sva, len + fill);
                    sv_ind = (*sva).ind;
                    sv_val = (*sva).val;
                }
                _glp_sva_enlarge_cap(sva, vr_ref - 1 as i32 + i, len + fill, 0 as i32);
            }
            *vr_len.offset(i as isize) += fill;
            loc1 = *vr_len.offset(p as isize) - 1 as i32;
            while loc1 >= 0 as i32 {
                j = *sv_ind.offset((*vr_ptr.offset(p as isize) + loc1) as isize);
                if *flag.offset(j as isize) == 0 {
                    *flag.offset(j as isize) = 1 as i32 as i8;
                } else {
                    vij = -fip * *work.offset(j as isize);
                    if !(-eps_tol < vij && vij < eps_tol) {
                        let fresh23 = len;
                        len = len + 1;
                        ptr = *vr_ptr.offset(i as isize) + fresh23;
                        *sv_ind.offset(ptr as isize) = j;
                        *sv_val.offset(ptr as isize) = vij;
                        if *vc_cap.offset(j as isize) == *vc_len.offset(j as isize) {
                            let mut need: i32 = *vc_len.offset(j as isize) + 10 as i32;
                            if (*sva).r_ptr - (*sva).m_ptr < need {
                                _glp_sva_more_space(sva, need);
                                sv_ind = (*sva).ind;
                                sv_val = (*sva).val;
                            }
                            _glp_sva_enlarge_cap(
                                sva,
                                vc_ref - 1 as i32 + j,
                                need,
                                1 as i32,
                            );
                        }
                        let ref mut fresh24 = *vc_len.offset(j as isize);
                        let fresh25 = *fresh24;
                        *fresh24 = *fresh24 + 1;
                        *sv_ind
                            .offset((*vc_ptr.offset(j as isize) + fresh25) as isize) = i;
                    }
                }
                loc1 -= 1;
                loc1;
            }
            (len <= *vr_len.offset(i as isize)
                || {
                    glp_assert_(
                        b"len <= vr_len[i]\0" as *const u8 as *const i8,
                        b"bflib/sgf.c\0" as *const u8 as *const i8,
                        934 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *vr_len.offset(i as isize) = len;
        }
        let mut len_0: i32 = *vr_len.offset(i as isize);
        *rs_prev.offset(i as isize) = 0 as i32;
        *rs_next.offset(i as isize) = *rs_head.offset(len_0 as isize);
        if *rs_next.offset(i as isize) != 0 as i32 {
            *rs_prev.offset(*rs_next.offset(i as isize) as isize) = i;
        }
        *rs_head.offset(len_0 as isize) = i;
        *vr_max.offset(i as isize) = -1.0f64;
        loc -= 1;
        loc;
    }
    i_ptr = *vr_ptr.offset(p as isize);
    i_end = i_ptr + *vr_len.offset(p as isize);
    while i_ptr < i_end {
        j = *sv_ind.offset(i_ptr as isize);
        (j != q
            || {
                glp_assert_(
                    b"j != q\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    947 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *cs_next.offset(j as isize) == j && *vc_len.offset(j as isize) != 1 as i32 {
            (*cs_prev.offset(j as isize) == j
                || {
                    glp_assert_(
                        b"cs_prev[j] == j\0" as *const u8 as *const i8,
                        b"bflib/sgf.c\0" as *const u8 as *const i8,
                        952 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        } else {
            let mut len_1: i32 = *vc_len.offset(j as isize);
            *cs_prev.offset(j as isize) = 0 as i32;
            *cs_next.offset(j as isize) = *cs_head.offset(len_1 as isize);
            if *cs_next.offset(j as isize) != 0 as i32 {
                *cs_prev.offset(*cs_next.offset(j as isize) as isize) = j;
            }
            *cs_head.offset(len_1 as isize) = j;
        }
        nnz_diff += *vc_len.offset(j as isize);
        *flag.offset(j as isize) = 0 as i32 as i8;
        *work.offset(j as isize) = 0.0f64;
        i_ptr += 1;
        i_ptr;
    }
    return nnz_diff;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sgf_dense_lu(
    mut n: i32,
    mut a_: *mut libc::c_double,
    mut r: *mut i32,
    mut c: *mut i32,
    mut eps: libc::c_double,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut ref_0: i32 = 0;
    let mut akk: libc::c_double = 0.;
    let mut big: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    k = 0 as i32;
    while k < n {
        q = -(1 as i32);
        p = q;
        big = eps;
        i = k;
        while i < n {
            j = k;
            while j < n {
                temp = *a_.offset((i * n + j) as isize);
                if temp < 0.0f64 {
                    temp = -temp;
                }
                if big < temp {
                    p = i;
                    q = j;
                    big = temp;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        if p < 0 as i32 {
            return k + 1 as i32;
        }
        if k != p {
            j = 0 as i32;
            while j < n {
                temp = *a_.offset((k * n + j) as isize);
                *a_.offset((k * n + j) as isize) = *a_.offset((p * n + j) as isize);
                *a_.offset((p * n + j) as isize) = temp;
                j += 1;
                j;
            }
            ref_0 = *r.offset(k as isize);
            *r.offset(k as isize) = *r.offset(p as isize);
            *r.offset(p as isize) = ref_0;
        }
        if k != q {
            i = 0 as i32;
            while i < n {
                temp = *a_.offset((i * n + k) as isize);
                *a_.offset((i * n + k) as isize) = *a_.offset((i * n + q) as isize);
                *a_.offset((i * n + q) as isize) = temp;
                i += 1;
                i;
            }
            ref_0 = *c.offset(k as isize);
            *c.offset(k as isize) = *c.offset(q as isize);
            *c.offset(q as isize) = ref_0;
        }
        akk = *a_.offset((k * n + k) as isize);
        i = k + 1 as i32;
        while i < n {
            if *a_.offset((i * n + k) as isize) != 0.0f64 {
                let ref mut fresh26 = *a_.offset((i * n + k) as isize);
                *fresh26 /= akk;
                temp = *fresh26;
                j = k + 1 as i32;
                while j < n {
                    *a_.offset((i * n + j) as isize)
                        -= temp * *a_.offset((k * n + j) as isize);
                    j += 1;
                    j;
                }
            }
            i += 1;
            i;
        }
        k += 1;
        k;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sgf_dense_phase(
    mut luf: *mut LUF,
    mut k: i32,
    mut updat: i32,
) -> i32 {
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fc_ref: i32 = (*luf).fc_ref;
    let mut fc_ptr: *mut i32 = &mut *((*sva).ptr).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut fc_len: *mut i32 = &mut *((*sva).len).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut fc_cap: *mut i32 = &mut *((*sva).cap).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_ref: i32 = (*luf).vr_ref;
    let mut vr_ptr: *mut i32 = &mut *((*sva).ptr).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_len: *mut i32 = &mut *((*sva).len).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_cap: *mut i32 = &mut *((*sva).cap).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_piv: *mut libc::c_double = (*luf).vr_piv;
    let mut vc_ref: i32 = (*luf).vc_ref;
    let mut vc_len: *mut i32 = &mut *((*sva).len).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut pp_inv: *mut i32 = (*luf).pp_inv;
    let mut pp_ind: *mut i32 = (*luf).pp_ind;
    let mut qq_ind: *mut i32 = (*luf).qq_ind;
    let mut qq_inv: *mut i32 = (*luf).qq_inv;
    let mut a_end: i32 = 0;
    let mut a_ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut i: i32 = 0;
    let mut ia: i32 = 0;
    let mut ii: i32 = 0;
    let mut j: i32 = 0;
    let mut ja: i32 = 0;
    let mut jj: i32 = 0;
    let mut ka: i32 = 0;
    let mut len: i32 = 0;
    let mut na: i32 = 0;
    let mut ne: i32 = 0;
    let mut need: i32 = 0;
    let mut ptr: i32 = 0;
    let mut a_: *mut libc::c_double = 0 as *mut libc::c_double;
    (1 as i32 <= k && k <= n
        || {
            glp_assert_(
                b"1 <= k && k <= n\0" as *const u8 as *const i8,
                b"bflib/sgf.c\0" as *const u8 as *const i8,
                1125 as i32,
            );
            1 as i32 != 0
        }) as i32;
    jj = k;
    while jj <= n {
        *vc_len.offset(*qq_ind.offset(jj as isize) as isize) = 0 as i32;
        jj += 1;
        jj;
    }
    na = n - k + 1 as i32;
    (1 as i32 <= na && na <= n
        || {
            glp_assert_(
                b"1 <= na && na <= n\0" as *const u8 as *const i8,
                b"bflib/sgf.c\0" as *const u8 as *const i8,
                1133 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ne = na * (na - 1 as i32) / 2 as i32;
    need = na * na + ne + ne;
    if (*sva).r_ptr - (*sva).m_ptr < need {
        _glp_sva_more_space(sva, need);
        sv_ind = (*sva).ind;
        sv_val = (*sva).val;
    }
    a_ptr = (*sva).m_ptr + ne;
    a_end = a_ptr + na * na;
    a_ = &mut *((*sva).val).offset(a_ptr as isize) as *mut libc::c_double;
    ia = 1 as i32;
    while ia <= na {
        ja = 1 as i32;
        while ja <= na {
            *a_.offset(((ia - 1 as i32) * na + (ja - 1 as i32)) as isize) = 0.0f64;
            ja += 1;
            ja;
        }
        i = *pp_inv.offset((k - 1 as i32 + ia) as isize);
        ptr = *vr_ptr.offset(i as isize);
        end = ptr + *vr_len.offset(i as isize);
        while ptr < end {
            *a_
                .offset(
                    ((ia - 1 as i32) * na
                        + (*qq_inv.offset(*sv_ind.offset(ptr as isize) as isize) - k
                            + 1 as i32 - 1 as i32)) as isize,
                ) = *sv_val.offset(ptr as isize);
            ptr += 1;
            ptr;
        }
        *vr_len.offset(i as isize) = 0 as i32;
        ia += 1;
        ia;
    }
    ka = _glp_sgf_dense_lu(
        na,
        &mut *a_.offset(((1 as i32 - 1 as i32) * na + (1 as i32 - 1 as i32)) as isize),
        &mut *pp_inv.offset(k as isize),
        &mut *qq_ind.offset(k as isize),
        1e-20f64,
    );
    ii = k;
    while ii <= n {
        *pp_ind.offset(*pp_inv.offset(ii as isize) as isize) = ii;
        ii += 1;
        ii;
    }
    jj = k;
    while jj <= n {
        *qq_inv.offset(*qq_ind.offset(jj as isize) as isize) = jj;
        jj += 1;
        jj;
    }
    if ka != 0 as i32 {
        (1 as i32 <= ka && ka <= na
            || {
                glp_assert_(
                    b"1 <= ka && ka <= na\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    1190 as i32,
                );
                1 as i32 != 0
            }) as i32;
        return k - 1 as i32 + ka;
    }
    ia = 1 as i32;
    while ia <= na {
        i = *pp_inv.offset((k - 1 as i32 + ia) as isize);
        (*vr_len.offset(i as isize) == 0 as i32
            || {
                glp_assert_(
                    b"vr_len[i] == 0\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    1197 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *vr_piv.offset(i as isize) = *a_
            .offset(((ia - 1 as i32) * na + (ia - 1 as i32)) as isize);
        len = 0 as i32;
        ja = ia + 1 as i32;
        while ja <= na {
            if *a_.offset(((ia - 1 as i32) * na + (ja - 1 as i32)) as isize) != 0.0f64 {
                len += 1;
                len;
            }
            ja += 1;
            ja;
        }
        if *vr_cap.offset(i as isize) < len {
            ((*sva).r_ptr - (*sva).m_ptr >= len
                || {
                    glp_assert_(
                        b"sva->r_ptr - sva->m_ptr >= len\0" as *const u8 as *const i8,
                        b"bflib/sgf.c\0" as *const u8 as *const i8,
                        1211 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            _glp_sva_enlarge_cap(sva, vr_ref - 1 as i32 + i, len, 0 as i32);
            ((*sva).m_ptr <= a_ptr
                || {
                    glp_assert_(
                        b"sva->m_ptr <= a_ptr\0" as *const u8 as *const i8,
                        b"bflib/sgf.c\0" as *const u8 as *const i8,
                        1214 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        ptr = *vr_ptr.offset(i as isize);
        ja = ia + 1 as i32;
        while ja <= na {
            if *a_.offset(((ia - 1 as i32) * na + (ja - 1 as i32)) as isize) != 0.0f64 {
                *sv_ind.offset(ptr as isize) = *qq_ind
                    .offset((k - 1 as i32 + ja) as isize);
                *sv_val.offset(ptr as isize) = *a_
                    .offset(((ia - 1 as i32) * na + (ja - 1 as i32)) as isize);
                ptr += 1;
                ptr;
            }
            ja += 1;
            ja;
        }
        (ptr - *vr_ptr.offset(i as isize) == len
            || {
                glp_assert_(
                    b"ptr - vr_ptr[i] == len\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    1226 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *vr_len.offset(i as isize) = len;
        ia += 1;
        ia;
    }
    ja = 1 as i32;
    while ja <= na {
        j = *pp_inv.offset((k - 1 as i32 + ja) as isize);
        (*fc_len.offset(j as isize) == 0 as i32
            || {
                glp_assert_(
                    b"fc_len[j] == 0\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    1234 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*fc_cap.offset(j as isize) == 0 as i32
            || {
                glp_assert_(
                    b"fc_cap[j] == 0\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    1235 as i32,
                );
                1 as i32 != 0
            }) as i32;
        len = 0 as i32;
        ia = ja + 1 as i32;
        while ia <= na {
            if *a_.offset(((ia - 1 as i32) * na + (ja - 1 as i32)) as isize) != 0.0f64 {
                len += 1;
                len;
            }
            ia += 1;
            ia;
        }
        ((*sva).r_ptr - (*sva).m_ptr >= len
            || {
                glp_assert_(
                    b"sva->r_ptr - sva->m_ptr >= len\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    1246 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if len > 0 as i32 {
            _glp_sva_reserve_cap(sva, fc_ref - 1 as i32 + j, len);
        }
        (a_end <= (*sva).r_ptr
            || {
                glp_assert_(
                    b"a_end <= sva->r_ptr\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    1250 as i32,
                );
                1 as i32 != 0
            }) as i32;
        ptr = *fc_ptr.offset(j as isize);
        ia = ja + 1 as i32;
        while ia <= na {
            if *a_.offset(((ia - 1 as i32) * na + (ja - 1 as i32)) as isize) != 0.0f64 {
                *sv_ind.offset(ptr as isize) = *pp_inv
                    .offset((k - 1 as i32 + ia) as isize);
                *sv_val.offset(ptr as isize) = *a_
                    .offset(((ia - 1 as i32) * na + (ja - 1 as i32)) as isize);
                ptr += 1;
                ptr;
            }
            ia += 1;
            ia;
        }
        (ptr - *fc_ptr.offset(j as isize) == len
            || {
                glp_assert_(
                    b"ptr - fc_ptr[j] == len\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    1261 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *fc_len.offset(j as isize) = len;
        ja += 1;
        ja;
    }
    if updat == 0 {
        ia = 1 as i32;
        while ia <= na {
            i = *pp_inv.offset((k - 1 as i32 + ia) as isize);
            len = *vr_len.offset(i as isize);
            if (*sva).r_ptr - (*sva).m_ptr < len {
                _glp_sva_more_space(sva, len);
                sv_ind = (*sva).ind;
                sv_val = (*sva).val;
            }
            _glp_sva_make_static(sva, vr_ref - 1 as i32 + i);
            ia += 1;
            ia;
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sgf_factorize(mut sgf: *mut SGF, mut singl: i32) -> i32 {
    let mut luf: *mut LUF = (*sgf).luf;
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut vr_ref: i32 = (*luf).vr_ref;
    let mut vr_len: *mut i32 = &mut *((*sva).len).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_piv: *mut libc::c_double = (*luf).vr_piv;
    let mut vc_ref: i32 = (*luf).vc_ref;
    let mut vc_len: *mut i32 = &mut *((*sva).len).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut pp_ind: *mut i32 = (*luf).pp_ind;
    let mut pp_inv: *mut i32 = (*luf).pp_inv;
    let mut qq_ind: *mut i32 = (*luf).qq_ind;
    let mut qq_inv: *mut i32 = (*luf).qq_inv;
    let mut rs_head: *mut i32 = (*sgf).rs_head;
    let mut rs_prev: *mut i32 = (*sgf).rs_prev;
    let mut rs_next: *mut i32 = (*sgf).rs_next;
    let mut cs_head: *mut i32 = (*sgf).cs_head;
    let mut cs_prev: *mut i32 = (*sgf).cs_prev;
    let mut cs_next: *mut i32 = (*sgf).cs_next;
    let mut vr_max: *mut libc::c_double = (*sgf).vr_max;
    let mut flag: *mut i8 = (*sgf).flag;
    let mut work: *mut libc::c_double = (*sgf).work;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut k1: i32 = 0;
    let mut k2: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut nnz: i32 = 0;
    _glp_luf_build_v_rows(luf, rs_prev);
    k = 1 as i32;
    while k <= n {
        *vr_piv.offset(k as isize) = 0.0f64;
        let ref mut fresh27 = *qq_inv.offset(k as isize);
        *fresh27 = k;
        let ref mut fresh28 = *qq_ind.offset(k as isize);
        *fresh28 = *fresh27;
        let ref mut fresh29 = *pp_inv.offset(k as isize);
        *fresh29 = *fresh28;
        *pp_ind.offset(k as isize) = *fresh29;
        k += 1;
        k;
    }
    if singl == 0 {
        k2 = 1 as i32;
    } else {
        if _glp_sgf_reduce_nuc(luf, &mut k1, &mut k2, rs_prev, rs_next) != 0 {
            return -(1 as i32);
        }
        k2 = _glp_sgf_singl_phase(luf, k1, k2, (*sgf).updat, rs_prev, work);
    }
    let ref mut fresh30 = *cs_head.offset(0 as i32 as isize);
    *fresh30 = 0 as i32;
    *rs_head.offset(0 as i32 as isize) = *fresh30;
    k = 1 as i32;
    while k <= n {
        let ref mut fresh31 = *cs_head.offset(k as isize);
        *fresh31 = 0 as i32;
        *rs_head.offset(k as isize) = *fresh31;
        *vr_max.offset(k as isize) = -1.0f64;
        *flag.offset(k as isize) = 0 as i32 as i8;
        *work.offset(k as isize) = 0.0f64;
        k += 1;
        k;
    }
    nnz = 0 as i32;
    k = k2;
    while k <= n {
        i = *pp_inv.offset(k as isize);
        let mut len: i32 = *vr_len.offset(i as isize);
        *rs_prev.offset(i as isize) = 0 as i32;
        *rs_next.offset(i as isize) = *rs_head.offset(len as isize);
        if *rs_next.offset(i as isize) != 0 as i32 {
            *rs_prev.offset(*rs_next.offset(i as isize) as isize) = i;
        }
        *rs_head.offset(len as isize) = i;
        nnz += *vr_len.offset(i as isize);
        j = *qq_ind.offset(k as isize);
        let mut len_0: i32 = *vc_len.offset(j as isize);
        *cs_prev.offset(j as isize) = 0 as i32;
        *cs_next.offset(j as isize) = *cs_head.offset(len_0 as isize);
        if *cs_next.offset(j as isize) != 0 as i32 {
            *cs_prev.offset(*cs_next.offset(j as isize) as isize) = j;
        }
        *cs_head.offset(len_0 as isize) = j;
        k += 1;
        k;
    }
    k = k2;
    while k <= n {
        let mut na: i32 = 0;
        let mut den: libc::c_double = 0.;
        na = n - k + 1 as i32;
        den = nnz as libc::c_double / (na as libc::c_double * na as libc::c_double);
        if na >= 5 as i32 && den >= 0.71f64 {
            break;
        }
        if _glp_sgf_choose_pivot(sgf, &mut p, &mut q) != 0 as i32 {
            return k;
        }
        i = *pp_ind.offset(p as isize);
        (k <= i && i <= n
            || {
                glp_assert_(
                    b"k <= i && i <= n\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    1405 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j = *qq_inv.offset(q as isize);
        (k <= j && j <= n
            || {
                glp_assert_(
                    b"k <= j && j <= n\0" as *const u8 as *const i8,
                    b"bflib/sgf.c\0" as *const u8 as *const i8,
                    1407 as i32,
                );
                1 as i32 != 0
            }) as i32;
        let mut j1: i32 = 0;
        let mut j2: i32 = 0;
        j1 = *pp_inv.offset(k as isize);
        j2 = *pp_inv.offset(i as isize);
        *pp_ind.offset(j1 as isize) = i;
        *pp_inv.offset(i as isize) = j1;
        *pp_ind.offset(j2 as isize) = k;
        *pp_inv.offset(k as isize) = j2;
        let mut i1: i32 = 0;
        let mut i2: i32 = 0;
        i1 = *qq_ind.offset(k as isize);
        i2 = *qq_ind.offset(j as isize);
        *qq_ind.offset(k as isize) = i2;
        *qq_inv.offset(i2 as isize) = k;
        *qq_ind.offset(j as isize) = i1;
        *qq_inv.offset(i1 as isize) = j;
        nnz += _glp_sgf_eliminate(sgf, p, q);
        k += 1;
        k;
    }
    if k <= n {
        k = _glp_sgf_dense_phase(luf, k, (*sgf).updat);
        if k != 0 as i32 {
            return k;
        }
    }
    _glp_sva_defrag_area(sva);
    _glp_luf_build_f_rows(luf, rs_head);
    _glp_luf_build_v_cols(luf, (*sgf).updat, rs_head);
    return 0 as i32;
}