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
    fn _glp_sva_more_space(sva: *mut SVA, m_size: i32);
    fn _glp_sva_enlarge_cap(sva: *mut SVA, k: i32, new_cap: i32, skip: i32);
    fn _glp_sva_reserve_cap(sva: *mut SVA, k: i32, new_cap: i32);
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
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_store_v_cols(
    mut luf: *mut LUF,
    mut col: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            i32,
            *mut i32,
            *mut libc::c_double,
        ) -> i32,
    >,
    mut info: *mut libc::c_void,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vc_ref: i32 = (*luf).vc_ref;
    let mut vc_ptr: *mut i32 = &mut *((*sva).ptr).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vc_len: *mut i32 = &mut *((*sva).len).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vc_cap: *mut i32 = &mut *((*sva).cap).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut j: i32 = 0;
    let mut len: i32 = 0;
    let mut ptr: i32 = 0;
    let mut nnz: i32 = 0;
    nnz = 0 as i32;
    j = 1 as i32;
    while j <= n {
        len = col.expect("non-null function pointer")(info, j, ind, val);
        (0 as i32 <= len && len <= n
            || {
                glp_assert_(
                    b"0 <= len && len <= n\0" as *const u8 as *const i8,
                    b"bflib/luf.c\0" as *const u8 as *const i8,
                    48 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *vc_cap.offset(j as isize) < len {
            if (*sva).r_ptr - (*sva).m_ptr < len {
                _glp_sva_more_space(sva, len);
                sv_ind = (*sva).ind;
                sv_val = (*sva).val;
            }
            _glp_sva_enlarge_cap(sva, vc_ref - 1 as i32 + j, len, 0 as i32);
        }
        ptr = *vc_ptr.offset(j as isize);
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
        *vc_len.offset(j as isize) = len;
        nnz += len;
        j += 1;
        j;
    }
    return nnz;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_check_all(mut luf: *mut LUF, mut k: i32) {
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fr_ref: i32 = (*luf).fr_ref;
    let mut fr_len: *mut i32 = &mut *((*sva).len).offset((fr_ref - 1 as i32) as isize)
        as *mut i32;
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
    let mut i_ptr: i32 = 0;
    let mut i_end: i32 = 0;
    let mut j: i32 = 0;
    let mut jj: i32 = 0;
    let mut j_ptr: i32 = 0;
    let mut j_end: i32 = 0;
    (n > 0 as i32
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const i8,
                b"bflib/luf.c\0" as *const u8 as *const i8,
                100 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= k && k <= n + 1 as i32
        || {
            glp_assert_(
                b"1 <= k && k <= n+1\0" as *const u8 as *const i8,
                b"bflib/luf.c\0" as *const u8 as *const i8,
                101 as i32,
            );
            1 as i32 != 0
        }) as i32;
    i = 1 as i32;
    while i <= n {
        ii = *pp_ind.offset(i as isize);
        (1 as i32 <= ii && ii <= n
            || {
                glp_assert_(
                    b"1 <= ii && ii <= n\0" as *const u8 as *const i8,
                    b"bflib/luf.c\0" as *const u8 as *const i8,
                    105 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*pp_inv.offset(ii as isize) == i
            || {
                glp_assert_(
                    b"pp_inv[ii] == i\0" as *const u8 as *const i8,
                    b"bflib/luf.c\0" as *const u8 as *const i8,
                    106 as i32,
                );
                1 as i32 != 0
            }) as i32;
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        jj = *qq_inv.offset(j as isize);
        (1 as i32 <= jj && jj <= n
            || {
                glp_assert_(
                    b"1 <= jj && jj <= n\0" as *const u8 as *const i8,
                    b"bflib/luf.c\0" as *const u8 as *const i8,
                    111 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*qq_ind.offset(jj as isize) == j
            || {
                glp_assert_(
                    b"qq_ind[jj] == j\0" as *const u8 as *const i8,
                    b"bflib/luf.c\0" as *const u8 as *const i8,
                    112 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= n {
        (*fr_len.offset(i as isize) == 0 as i32
            || {
                glp_assert_(
                    b"fr_len[i] == 0\0" as *const u8 as *const i8,
                    b"bflib/luf.c\0" as *const u8 as *const i8,
                    116 as i32,
                );
                1 as i32 != 0
            }) as i32;
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        jj = *pp_ind.offset(j as isize);
        if jj < k {
            j_ptr = *fc_ptr.offset(j as isize);
            j_end = j_ptr + *fc_len.offset(j as isize);
            while j_ptr < j_end {
                i = *sv_ind.offset(j_ptr as isize);
                (1 as i32 <= i && i <= n
                    || {
                        glp_assert_(
                            b"1 <= i && i <= n\0" as *const u8 as *const i8,
                            b"bflib/luf.c\0" as *const u8 as *const i8,
                            126 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                ii = *pp_ind.offset(i as isize);
                (ii > jj
                    || {
                        glp_assert_(
                            b"ii > jj\0" as *const u8 as *const i8,
                            b"bflib/luf.c\0" as *const u8 as *const i8,
                            128 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*sv_val.offset(j_ptr as isize) != 0.0f64
                    || {
                        glp_assert_(
                            b"sv_val[j_ptr] != 0.0\0" as *const u8 as *const i8,
                            b"bflib/luf.c\0" as *const u8 as *const i8,
                            129 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                j_ptr += 1;
                j_ptr;
            }
        } else {
            (*fc_len.offset(j as isize) == 0 as i32
                || {
                    glp_assert_(
                        b"fc_len[j] == 0\0" as *const u8 as *const i8,
                        b"bflib/luf.c\0" as *const u8 as *const i8,
                        133 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= n {
        ii = *pp_ind.offset(i as isize);
        i_ptr = *vr_ptr.offset(i as isize);
        i_end = i_ptr + *vr_len.offset(i as isize);
        while i_ptr < i_end {
            j = *sv_ind.offset(i_ptr as isize);
            (1 as i32 <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const i8,
                        b"bflib/luf.c\0" as *const u8 as *const i8,
                        143 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            jj = *qq_inv.offset(j as isize);
            if ii < k {
                (jj > ii
                    || {
                        glp_assert_(
                            b"jj > ii\0" as *const u8 as *const i8,
                            b"bflib/luf.c\0" as *const u8 as *const i8,
                            146 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            } else {
                (jj >= k
                    || {
                        glp_assert_(
                            b"jj >= k\0" as *const u8 as *const i8,
                            b"bflib/luf.c\0" as *const u8 as *const i8,
                            148 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
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
                            b"bflib/luf.c\0" as *const u8 as *const i8,
                            154 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            (*sv_val.offset(i_ptr as isize) != 0.0f64
                || {
                    glp_assert_(
                        b"sv_val[i_ptr] != 0.0\0" as *const u8 as *const i8,
                        b"bflib/luf.c\0" as *const u8 as *const i8,
                        156 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            i_ptr += 1;
            i_ptr;
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        jj = *qq_inv.offset(j as isize);
        if jj < k {
            (*vc_len.offset(j as isize) == 0 as i32
                || {
                    glp_assert_(
                        b"vc_len[j] == 0\0" as *const u8 as *const i8,
                        b"bflib/luf.c\0" as *const u8 as *const i8,
                        164 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        } else {
            j_ptr = *vc_ptr.offset(j as isize);
            j_end = j_ptr + *vc_len.offset(j as isize);
            while j_ptr < j_end {
                i = *sv_ind.offset(j_ptr as isize);
                ii = *pp_ind.offset(i as isize);
                (ii >= k
                    || {
                        glp_assert_(
                            b"ii >= k\0" as *const u8 as *const i8,
                            b"bflib/luf.c\0" as *const u8 as *const i8,
                            171 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
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
                            b"bflib/luf.c\0" as *const u8 as *const i8,
                            177 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                j_ptr += 1;
                j_ptr;
            }
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_build_v_rows(mut luf: *mut LUF, mut len: *mut i32) {
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
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut end: i32 = 0;
    let mut nnz: i32 = 0;
    let mut ptr: i32 = 0;
    let mut ptr1: i32 = 0;
    nnz = 0 as i32;
    i = 1 as i32;
    while i <= n {
        *len.offset(i as isize) = 0 as i32;
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        nnz += *vc_len.offset(j as isize);
        ptr = *vc_ptr.offset(j as isize);
        end = ptr + *vc_len.offset(j as isize);
        while ptr < end {
            let ref mut fresh0 = *len.offset(*sv_ind.offset(ptr as isize) as isize);
            *fresh0 += 1;
            *fresh0;
            ptr += 1;
            ptr;
        }
        j += 1;
        j;
    }
    if (*sva).r_ptr - (*sva).m_ptr < nnz {
        _glp_sva_more_space(sva, nnz);
        sv_ind = (*sva).ind;
        sv_val = (*sva).val;
    }
    i = 1 as i32;
    while i <= n {
        if *len.offset(i as isize) > 0 as i32 {
            _glp_sva_enlarge_cap(
                sva,
                vr_ref - 1 as i32 + i,
                *len.offset(i as isize),
                0 as i32,
            );
        }
        *vr_len.offset(i as isize) = *len.offset(i as isize);
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        ptr = *vc_ptr.offset(j as isize);
        end = ptr + *vc_len.offset(j as isize);
        while ptr < end {
            i = *sv_ind.offset(ptr as isize);
            let ref mut fresh1 = *len.offset(i as isize);
            *fresh1 -= 1;
            ptr1 = *vr_ptr.offset(i as isize) + *fresh1;
            *sv_ind.offset(ptr1 as isize) = j;
            *sv_val.offset(ptr1 as isize) = *sv_val.offset(ptr as isize);
            ptr += 1;
            ptr;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_build_f_rows(mut luf: *mut LUF, mut len: *mut i32) {
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fr_ref: i32 = (*luf).fr_ref;
    let mut fr_ptr: *mut i32 = &mut *((*sva).ptr).offset((fr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut fr_len: *mut i32 = &mut *((*sva).len).offset((fr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut fc_ref: i32 = (*luf).fc_ref;
    let mut fc_ptr: *mut i32 = &mut *((*sva).ptr).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut fc_len: *mut i32 = &mut *((*sva).len).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut end: i32 = 0;
    let mut nnz: i32 = 0;
    let mut ptr: i32 = 0;
    let mut ptr1: i32 = 0;
    nnz = 0 as i32;
    i = 1 as i32;
    while i <= n {
        *len.offset(i as isize) = 0 as i32;
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        nnz += *fc_len.offset(j as isize);
        ptr = *fc_ptr.offset(j as isize);
        end = ptr + *fc_len.offset(j as isize);
        while ptr < end {
            let ref mut fresh2 = *len.offset(*sv_ind.offset(ptr as isize) as isize);
            *fresh2 += 1;
            *fresh2;
            ptr += 1;
            ptr;
        }
        j += 1;
        j;
    }
    if (*sva).r_ptr - (*sva).m_ptr < nnz {
        _glp_sva_more_space(sva, nnz);
        sv_ind = (*sva).ind;
        sv_val = (*sva).val;
    }
    i = 1 as i32;
    while i <= n {
        if *len.offset(i as isize) > 0 as i32 {
            _glp_sva_reserve_cap(sva, fr_ref - 1 as i32 + i, *len.offset(i as isize));
        }
        *fr_len.offset(i as isize) = *len.offset(i as isize);
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        ptr = *fc_ptr.offset(j as isize);
        end = ptr + *fc_len.offset(j as isize);
        while ptr < end {
            i = *sv_ind.offset(ptr as isize);
            let ref mut fresh3 = *len.offset(i as isize);
            *fresh3 -= 1;
            ptr1 = *fr_ptr.offset(i as isize) + *fresh3;
            *sv_ind.offset(ptr1 as isize) = j;
            *sv_val.offset(ptr1 as isize) = *sv_val.offset(ptr as isize);
            ptr += 1;
            ptr;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_build_v_cols(
    mut luf: *mut LUF,
    mut updat: i32,
    mut len: *mut i32,
) {
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
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut end: i32 = 0;
    let mut nnz: i32 = 0;
    let mut ptr: i32 = 0;
    let mut ptr1: i32 = 0;
    nnz = 0 as i32;
    j = 1 as i32;
    while j <= n {
        *len.offset(j as isize) = 0 as i32;
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= n {
        nnz += *vr_len.offset(i as isize);
        ptr = *vr_ptr.offset(i as isize);
        end = ptr + *vr_len.offset(i as isize);
        while ptr < end {
            let ref mut fresh4 = *len.offset(*sv_ind.offset(ptr as isize) as isize);
            *fresh4 += 1;
            *fresh4;
            ptr += 1;
            ptr;
        }
        i += 1;
        i;
    }
    if (*sva).r_ptr - (*sva).m_ptr < nnz {
        _glp_sva_more_space(sva, nnz);
        sv_ind = (*sva).ind;
        sv_val = (*sva).val;
    }
    j = 1 as i32;
    while j <= n {
        if *len.offset(j as isize) > 0 as i32 {
            if updat != 0 {
                _glp_sva_enlarge_cap(
                    sva,
                    vc_ref - 1 as i32 + j,
                    *len.offset(j as isize),
                    0 as i32,
                );
            } else {
                _glp_sva_reserve_cap(
                    sva,
                    vc_ref - 1 as i32 + j,
                    *len.offset(j as isize),
                );
            }
        }
        *vc_len.offset(j as isize) = *len.offset(j as isize);
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= n {
        ptr = *vr_ptr.offset(i as isize);
        end = ptr + *vr_len.offset(i as isize);
        while ptr < end {
            j = *sv_ind.offset(ptr as isize);
            let ref mut fresh5 = *len.offset(j as isize);
            *fresh5 -= 1;
            ptr1 = *vc_ptr.offset(j as isize) + *fresh5;
            *sv_ind.offset(ptr1 as isize) = i;
            *sv_val.offset(ptr1 as isize) = *sv_val.offset(ptr as isize);
            ptr += 1;
            ptr;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_check_f_rc(mut luf: *mut LUF) {
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fr_ref: i32 = (*luf).fr_ref;
    let mut fr_ptr: *mut i32 = &mut *((*sva).ptr).offset((fr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut fr_len: *mut i32 = &mut *((*sva).len).offset((fr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut fc_ref: i32 = (*luf).fc_ref;
    let mut fc_ptr: *mut i32 = &mut *((*sva).ptr).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut fc_len: *mut i32 = &mut *((*sva).len).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut i: i32 = 0;
    let mut i_end: i32 = 0;
    let mut i_ptr: i32 = 0;
    let mut j: i32 = 0;
    let mut j_end: i32 = 0;
    let mut j_ptr: i32 = 0;
    i = 1 as i32;
    while i <= n {
        i_ptr = *fr_ptr.offset(i as isize);
        i_end = i_ptr + *fr_len.offset(i as isize);
        while i_ptr < i_end {
            j = *sv_ind.offset(i_ptr as isize);
            j_ptr = *fc_ptr.offset(j as isize);
            j_end = j_ptr + *fc_len.offset(j as isize);
            while *sv_ind.offset(j_ptr as isize) != i {
                j_ptr += 1;
                j_ptr;
            }
            (j_ptr < j_end
                || {
                    glp_assert_(
                        b"j_ptr < j_end\0" as *const u8 as *const i8,
                        b"bflib/luf.c\0" as *const u8 as *const i8,
                        389 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*sv_val.offset(i_ptr as isize) == *sv_val.offset(j_ptr as isize)
                || {
                    glp_assert_(
                        b"sv_val[i_ptr] == sv_val[j_ptr]\0" as *const u8 as *const i8,
                        b"bflib/luf.c\0" as *const u8 as *const i8,
                        390 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *sv_ind.offset(j_ptr as isize) = -i;
            i_ptr += 1;
            i_ptr;
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        j_ptr = *fc_ptr.offset(j as isize);
        j_end = j_ptr + *fc_len.offset(j as isize);
        while j_ptr < j_end {
            i = *sv_ind.offset(j_ptr as isize);
            (i < 0 as i32
                || {
                    glp_assert_(
                        b"(i = sv_ind[j_ptr]) < 0\0" as *const u8 as *const i8,
                        b"bflib/luf.c\0" as *const u8 as *const i8,
                        400 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *sv_ind.offset(j_ptr as isize) = -i;
            j_ptr += 1;
            j_ptr;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_check_v_rc(mut luf: *mut LUF) {
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
    let mut i: i32 = 0;
    let mut i_end: i32 = 0;
    let mut i_ptr: i32 = 0;
    let mut j: i32 = 0;
    let mut j_end: i32 = 0;
    let mut j_ptr: i32 = 0;
    i = 1 as i32;
    while i <= n {
        i_ptr = *vr_ptr.offset(i as isize);
        i_end = i_ptr + *vr_len.offset(i as isize);
        while i_ptr < i_end {
            j = *sv_ind.offset(i_ptr as isize);
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
                        b"bflib/luf.c\0" as *const u8 as *const i8,
                        437 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*sv_val.offset(i_ptr as isize) == *sv_val.offset(j_ptr as isize)
                || {
                    glp_assert_(
                        b"sv_val[i_ptr] == sv_val[j_ptr]\0" as *const u8 as *const i8,
                        b"bflib/luf.c\0" as *const u8 as *const i8,
                        438 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *sv_ind.offset(j_ptr as isize) = -i;
            i_ptr += 1;
            i_ptr;
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        j_ptr = *vc_ptr.offset(j as isize);
        j_end = j_ptr + *vc_len.offset(j as isize);
        while j_ptr < j_end {
            i = *sv_ind.offset(j_ptr as isize);
            (i < 0 as i32
                || {
                    glp_assert_(
                        b"(i = sv_ind[j_ptr]) < 0\0" as *const u8 as *const i8,
                        b"bflib/luf.c\0" as *const u8 as *const i8,
                        448 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *sv_ind.offset(j_ptr as isize) = -i;
            j_ptr += 1;
            j_ptr;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_f_solve(
    mut luf: *mut LUF,
    mut x: *mut libc::c_double,
) {
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fc_ref: i32 = (*luf).fc_ref;
    let mut fc_ptr: *mut i32 = &mut *((*sva).ptr).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut fc_len: *mut i32 = &mut *((*sva).len).offset((fc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut pp_inv: *mut i32 = (*luf).pp_inv;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut x_j: libc::c_double = 0.;
    k = 1 as i32;
    while k <= n {
        j = *pp_inv.offset(k as isize);
        x_j = *x.offset(j as isize);
        if x_j != 0.0f64 {
            ptr = *fc_ptr.offset(j as isize);
            end = ptr + *fc_len.offset(j as isize);
            while ptr < end {
                *x.offset(*sv_ind.offset(ptr as isize) as isize)
                    -= *sv_val.offset(ptr as isize) * x_j;
                ptr += 1;
                ptr;
            }
        }
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_ft_solve(
    mut luf: *mut LUF,
    mut x: *mut libc::c_double,
) {
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fr_ref: i32 = (*luf).fr_ref;
    let mut fr_ptr: *mut i32 = &mut *((*sva).ptr).offset((fr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut fr_len: *mut i32 = &mut *((*sva).len).offset((fr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut pp_inv: *mut i32 = (*luf).pp_inv;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut x_i: libc::c_double = 0.;
    k = n;
    while k >= 1 as i32 {
        i = *pp_inv.offset(k as isize);
        x_i = *x.offset(i as isize);
        if x_i != 0.0f64 {
            ptr = *fr_ptr.offset(i as isize);
            end = ptr + *fr_len.offset(i as isize);
            while ptr < end {
                *x.offset(*sv_ind.offset(ptr as isize) as isize)
                    -= *sv_val.offset(ptr as isize) * x_i;
                ptr += 1;
                ptr;
            }
        }
        k -= 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_v_solve(
    mut luf: *mut LUF,
    mut b: *mut libc::c_double,
    mut x: *mut libc::c_double,
) {
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vr_piv: *mut libc::c_double = (*luf).vr_piv;
    let mut vc_ref: i32 = (*luf).vc_ref;
    let mut vc_ptr: *mut i32 = &mut *((*sva).ptr).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vc_len: *mut i32 = &mut *((*sva).len).offset((vc_ref - 1 as i32) as isize)
        as *mut i32;
    let mut pp_inv: *mut i32 = (*luf).pp_inv;
    let mut qq_ind: *mut i32 = (*luf).qq_ind;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut x_j: libc::c_double = 0.;
    k = n;
    while k >= 1 as i32 {
        i = *pp_inv.offset(k as isize);
        j = *qq_ind.offset(k as isize);
        let ref mut fresh6 = *x.offset(j as isize);
        *fresh6 = *b.offset(i as isize) / *vr_piv.offset(i as isize);
        x_j = *fresh6;
        if x_j != 0.0f64 {
            ptr = *vc_ptr.offset(j as isize);
            end = ptr + *vc_len.offset(j as isize);
            while ptr < end {
                *b.offset(*sv_ind.offset(ptr as isize) as isize)
                    -= *sv_val.offset(ptr as isize) * x_j;
                ptr += 1;
                ptr;
            }
        }
        k -= 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_vt_solve(
    mut luf: *mut LUF,
    mut b: *mut libc::c_double,
    mut x: *mut libc::c_double,
) {
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vr_piv: *mut libc::c_double = (*luf).vr_piv;
    let mut vr_ref: i32 = (*luf).vr_ref;
    let mut vr_ptr: *mut i32 = &mut *((*sva).ptr).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_len: *mut i32 = &mut *((*sva).len).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut pp_inv: *mut i32 = (*luf).pp_inv;
    let mut qq_ind: *mut i32 = (*luf).qq_ind;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut x_i: libc::c_double = 0.;
    k = 1 as i32;
    while k <= n {
        i = *pp_inv.offset(k as isize);
        j = *qq_ind.offset(k as isize);
        let ref mut fresh7 = *x.offset(i as isize);
        *fresh7 = *b.offset(j as isize) / *vr_piv.offset(i as isize);
        x_i = *fresh7;
        if x_i != 0.0f64 {
            ptr = *vr_ptr.offset(i as isize);
            end = ptr + *vr_len.offset(i as isize);
            while ptr < end {
                *b.offset(*sv_ind.offset(ptr as isize) as isize)
                    -= *sv_val.offset(ptr as isize) * x_i;
                ptr += 1;
                ptr;
            }
        }
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_vt_solve1(
    mut luf: *mut LUF,
    mut e: *mut libc::c_double,
    mut y: *mut libc::c_double,
) {
    let mut n: i32 = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vr_piv: *mut libc::c_double = (*luf).vr_piv;
    let mut vr_ref: i32 = (*luf).vr_ref;
    let mut vr_ptr: *mut i32 = &mut *((*sva).ptr).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut vr_len: *mut i32 = &mut *((*sva).len).offset((vr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut pp_inv: *mut i32 = (*luf).pp_inv;
    let mut qq_ind: *mut i32 = (*luf).qq_ind;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut e_j: libc::c_double = 0.;
    let mut y_i: libc::c_double = 0.;
    k = 1 as i32;
    while k <= n {
        i = *pp_inv.offset(k as isize);
        j = *qq_ind.offset(k as isize);
        e_j = if *e.offset(j as isize) >= 0.0f64 {
            *e.offset(j as isize) + 1.0f64
        } else {
            *e.offset(j as isize) - 1.0f64
        };
        let ref mut fresh8 = *y.offset(i as isize);
        *fresh8 = e_j / *vr_piv.offset(i as isize);
        y_i = *fresh8;
        ptr = *vr_ptr.offset(i as isize);
        end = ptr + *vr_len.offset(i as isize);
        while ptr < end {
            *e.offset(*sv_ind.offset(ptr as isize) as isize)
                -= *sv_val.offset(ptr as isize) * y_i;
            ptr += 1;
            ptr;
        }
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_luf_estimate_norm(
    mut luf: *mut LUF,
    mut w1: *mut libc::c_double,
    mut w2: *mut libc::c_double,
) -> libc::c_double {
    let mut n: i32 = (*luf).n;
    let mut e: *mut libc::c_double = w1;
    let mut y: *mut libc::c_double = w2;
    let mut z: *mut libc::c_double = w1;
    let mut i: i32 = 0;
    let mut y_norm: libc::c_double = 0.;
    let mut z_norm: libc::c_double = 0.;
    i = 1 as i32;
    while i <= n {
        *e.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    _glp_luf_vt_solve1(luf, e, y);
    _glp_luf_ft_solve(luf, y);
    y_norm = 0.0f64;
    i = 1 as i32;
    while i <= n {
        y_norm
            += if *y.offset(i as isize) >= 0.0f64 {
                *y.offset(i as isize)
            } else {
                -*y.offset(i as isize)
            };
        i += 1;
        i;
    }
    _glp_luf_f_solve(luf, y);
    _glp_luf_v_solve(luf, y, z);
    z_norm = 0.0f64;
    i = 1 as i32;
    while i <= n {
        z_norm
            += if *z.offset(i as isize) >= 0.0f64 {
                *z.offset(i as isize)
            } else {
                -*z.offset(i as isize)
            };
        i += 1;
        i;
    }
    return z_norm / y_norm;
}