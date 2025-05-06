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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn _glp_btf_a_solve(
        btf: *mut BTF,
        b: *mut libc::c_double,
        x: *mut libc::c_double,
        w1: *mut libc::c_double,
        w2: *mut libc::c_double,
    );
    fn _glp_btf_at_solve(
        btf: *mut BTF,
        b: *mut libc::c_double,
        x: *mut libc::c_double,
        w1: *mut libc::c_double,
        w2: *mut libc::c_double,
    );
    fn _glp_sva_more_space(sva: *mut SVA, m_size: i32);
    fn _glp_sva_reserve_cap(sva: *mut SVA, k: i32, new_cap: i32);
    fn _glp_ifu_bg_update(
        ifu: *mut IFU,
        c: *mut libc::c_double,
        r: *mut libc::c_double,
        d: libc::c_double,
    ) -> i32;
    fn _glp_ifu_gr_update(
        ifu: *mut IFU,
        c: *mut libc::c_double,
        r: *mut libc::c_double,
        d: libc::c_double,
    ) -> i32;
    fn _glp_ifu_a_solve(ifu: *mut IFU, x: *mut libc::c_double, w: *mut libc::c_double);
    fn _glp_ifu_at_solve(ifu: *mut IFU, x: *mut libc::c_double, w: *mut libc::c_double);
    fn _glp_luf_f_solve(luf: *mut LUF, x: *mut libc::c_double);
    fn _glp_luf_ft_solve(luf: *mut LUF, x: *mut libc::c_double);
    fn _glp_luf_v_solve(luf: *mut LUF, b: *mut libc::c_double, x: *mut libc::c_double);
    fn _glp_luf_vt_solve(luf: *mut LUF, b: *mut libc::c_double, x: *mut libc::c_double);
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
pub struct IFU {
    pub n_max: i32,
    pub n: i32,
    pub f: *mut libc::c_double,
    pub u: *mut libc::c_double,
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
pub struct SCF {
    pub n: i32,
    pub n0: i32,
    pub type_0: i32,
    pub a0: C2RustUnnamed,
    pub nn_max: i32,
    pub nn: i32,
    pub sva: *mut SVA,
    pub rr_ref: i32,
    pub ss_ref: i32,
    pub ifu: IFU,
    pub pp_ind: *mut i32,
    pub pp_inv: *mut i32,
    pub qq_ind: *mut i32,
    pub qq_inv: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub luf: *mut LUF,
    pub btf: *mut BTF,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scf_r0_solve(
    mut scf: *mut SCF,
    mut tr: i32,
    mut x: *mut libc::c_double,
) {
    match (*scf).type_0 {
        1 => {
            if tr == 0 {
                _glp_luf_f_solve((*scf).a0.luf, x);
            } else {
                _glp_luf_ft_solve((*scf).a0.luf, x);
            }
        }
        2 => {}
        _ => {
            (scf != scf
                || {
                    glp_assert_(
                        b"scf != scf\0" as *const u8 as *const i8,
                        b"bflib/scf.c\0" as *const u8 as *const i8,
                        50 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scf_s0_solve(
    mut scf: *mut SCF,
    mut tr: i32,
    mut x: *mut libc::c_double,
    mut w1: *mut libc::c_double,
    mut w2: *mut libc::c_double,
    mut w3: *mut libc::c_double,
) {
    let mut n0: i32 = (*scf).n0;
    match (*scf).type_0 {
        1 => {
            if tr == 0 {
                _glp_luf_v_solve((*scf).a0.luf, x, w1);
            } else {
                _glp_luf_vt_solve((*scf).a0.luf, x, w1);
            }
        }
        2 => {
            if tr == 0 {
                _glp_btf_a_solve((*scf).a0.btf, x, w1, w2, w3);
            } else {
                _glp_btf_at_solve((*scf).a0.btf, x, w1, w2, w3);
            }
        }
        _ => {
            (scf != scf
                || {
                    glp_assert_(
                        b"scf != scf\0" as *const u8 as *const i8,
                        b"bflib/scf.c\0" as *const u8 as *const i8,
                        90 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    memcpy(
        &mut *x.offset(1 as i32 as isize) as *mut libc::c_double as *mut libc::c_void,
        &mut *w1.offset(1 as i32 as isize) as *mut libc::c_double as *const libc::c_void,
        (n0 as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scf_r_prod(
    mut scf: *mut SCF,
    mut y: *mut libc::c_double,
    mut a: libc::c_double,
    mut x: *const libc::c_double,
) {
    let mut nn: i32 = (*scf).nn;
    let mut sva: *mut SVA = (*scf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut rr_ref: i32 = (*scf).rr_ref;
    let mut rr_ptr: *mut i32 = &mut *((*sva).ptr).offset((rr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut rr_len: *mut i32 = &mut *((*sva).len).offset((rr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut i: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut t: libc::c_double = 0.;
    i = 1 as i32;
    while i <= nn {
        t = 0.0f64;
        ptr = *rr_ptr.offset(i as isize);
        end = ptr + *rr_len.offset(i as isize);
        while ptr < end {
            t
                += *sv_val.offset(ptr as isize)
                    * *x.offset(*sv_ind.offset(ptr as isize) as isize);
            ptr += 1;
            ptr;
        }
        *y.offset(i as isize) += a * t;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scf_rt_prod(
    mut scf: *mut SCF,
    mut y: *mut libc::c_double,
    mut a: libc::c_double,
    mut x: *const libc::c_double,
) {
    let mut nn: i32 = (*scf).nn;
    let mut sva: *mut SVA = (*scf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut rr_ref: i32 = (*scf).rr_ref;
    let mut rr_ptr: *mut i32 = &mut *((*sva).ptr).offset((rr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut rr_len: *mut i32 = &mut *((*sva).len).offset((rr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut i: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut t: libc::c_double = 0.;
    i = 1 as i32;
    while i <= nn {
        if !(*x.offset(i as isize) == 0.0f64) {
            t = a * *x.offset(i as isize);
            ptr = *rr_ptr.offset(i as isize);
            end = ptr + *rr_len.offset(i as isize);
            while ptr < end {
                *y.offset(*sv_ind.offset(ptr as isize) as isize)
                    += *sv_val.offset(ptr as isize) * t;
                ptr += 1;
                ptr;
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scf_s_prod(
    mut scf: *mut SCF,
    mut y: *mut libc::c_double,
    mut a: libc::c_double,
    mut x: *const libc::c_double,
) {
    let mut nn: i32 = (*scf).nn;
    let mut sva: *mut SVA = (*scf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut ss_ref: i32 = (*scf).ss_ref;
    let mut ss_ptr: *mut i32 = &mut *((*sva).ptr).offset((ss_ref - 1 as i32) as isize)
        as *mut i32;
    let mut ss_len: *mut i32 = &mut *((*sva).len).offset((ss_ref - 1 as i32) as isize)
        as *mut i32;
    let mut j: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut t: libc::c_double = 0.;
    j = 1 as i32;
    while j <= nn {
        if !(*x.offset(j as isize) == 0.0f64) {
            t = a * *x.offset(j as isize);
            ptr = *ss_ptr.offset(j as isize);
            end = ptr + *ss_len.offset(j as isize);
            while ptr < end {
                *y.offset(*sv_ind.offset(ptr as isize) as isize)
                    += *sv_val.offset(ptr as isize) * t;
                ptr += 1;
                ptr;
            }
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scf_st_prod(
    mut scf: *mut SCF,
    mut y: *mut libc::c_double,
    mut a: libc::c_double,
    mut x: *const libc::c_double,
) {
    let mut nn: i32 = (*scf).nn;
    let mut sva: *mut SVA = (*scf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut ss_ref: i32 = (*scf).ss_ref;
    let mut ss_ptr: *mut i32 = &mut *((*sva).ptr).offset((ss_ref - 1 as i32) as isize)
        as *mut i32;
    let mut ss_len: *mut i32 = &mut *((*sva).len).offset((ss_ref - 1 as i32) as isize)
        as *mut i32;
    let mut j: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut t: libc::c_double = 0.;
    j = 1 as i32;
    while j <= nn {
        t = 0.0f64;
        ptr = *ss_ptr.offset(j as isize);
        end = ptr + *ss_len.offset(j as isize);
        while ptr < end {
            t
                += *sv_val.offset(ptr as isize)
                    * *x.offset(*sv_ind.offset(ptr as isize) as isize);
            ptr += 1;
            ptr;
        }
        *y.offset(j as isize) += a * t;
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scf_a_solve(
    mut scf: *mut SCF,
    mut x: *mut libc::c_double,
    mut w: *mut libc::c_double,
    mut work1: *mut libc::c_double,
    mut work2: *mut libc::c_double,
    mut work3: *mut libc::c_double,
) {
    let mut n: i32 = (*scf).n;
    let mut n0: i32 = (*scf).n0;
    let mut nn: i32 = (*scf).nn;
    let mut pp_ind: *mut i32 = (*scf).pp_ind;
    let mut qq_inv: *mut i32 = (*scf).qq_inv;
    let mut i: i32 = 0;
    let mut ii: i32 = 0;
    ii = 1 as i32;
    while ii <= n0 + nn {
        i = *pp_ind.offset(ii as isize);
        (i == ii
            || {
                glp_assert_(
                    b"i == ii\0" as *const u8 as *const i8,
                    b"bflib/scf.c\0" as *const u8 as *const i8,
                    264 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *w.offset(ii as isize) = if i <= n { *x.offset(i as isize) } else { 0.0f64 };
        ii += 1;
        ii;
    }
    _glp_scf_r0_solve(scf, 0 as i32, &mut *w.offset(0 as i32 as isize));
    _glp_scf_r_prod(
        scf,
        &mut *w.offset(n0 as isize),
        -1.0f64,
        &mut *w.offset(0 as i32 as isize) as *mut libc::c_double as *const libc::c_double,
    );
    _glp_ifu_a_solve(&mut (*scf).ifu, &mut *w.offset(n0 as isize), work1);
    _glp_scf_s_prod(
        scf,
        &mut *w.offset(0 as i32 as isize),
        -1.0f64,
        &mut *w.offset(n0 as isize) as *mut libc::c_double as *const libc::c_double,
    );
    _glp_scf_s0_solve(
        scf,
        0 as i32,
        &mut *w.offset(0 as i32 as isize),
        work1,
        work2,
        work3,
    );
    i = 1 as i32;
    while i <= n {
        *x.offset(i as isize) = *w.offset(*qq_inv.offset(i as isize) as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scf_at_solve(
    mut scf: *mut SCF,
    mut x: *mut libc::c_double,
    mut w: *mut libc::c_double,
    mut work1: *mut libc::c_double,
    mut work2: *mut libc::c_double,
    mut work3: *mut libc::c_double,
) {
    let mut n: i32 = (*scf).n;
    let mut n0: i32 = (*scf).n0;
    let mut nn: i32 = (*scf).nn;
    let mut pp_inv: *mut i32 = (*scf).pp_inv;
    let mut qq_ind: *mut i32 = (*scf).qq_ind;
    let mut i: i32 = 0;
    let mut ii: i32 = 0;
    ii = 1 as i32;
    while ii <= n0 + nn {
        i = *qq_ind.offset(ii as isize);
        *w.offset(ii as isize) = if i <= n { *x.offset(i as isize) } else { 0.0f64 };
        ii += 1;
        ii;
    }
    _glp_scf_s0_solve(
        scf,
        1 as i32,
        &mut *w.offset(0 as i32 as isize),
        work1,
        work2,
        work3,
    );
    _glp_scf_st_prod(
        scf,
        &mut *w.offset(n0 as isize),
        -1.0f64,
        &mut *w.offset(0 as i32 as isize) as *mut libc::c_double as *const libc::c_double,
    );
    _glp_ifu_at_solve(&mut (*scf).ifu, &mut *w.offset(n0 as isize), work1);
    _glp_scf_rt_prod(
        scf,
        &mut *w.offset(0 as i32 as isize),
        -1.0f64,
        &mut *w.offset(n0 as isize) as *mut libc::c_double as *const libc::c_double,
    );
    _glp_scf_r0_solve(scf, 1 as i32, &mut *w.offset(0 as i32 as isize));
    i = 1 as i32;
    while i <= n {
        (*pp_inv.offset(i as isize) == i
            || {
                glp_assert_(
                    b"pp_inv[i] == i\0" as *const u8 as *const i8,
                    b"bflib/scf.c\0" as *const u8 as *const i8,
                    324 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *x.offset(i as isize) = *w.offset(*pp_inv.offset(i as isize) as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scf_add_r_row(
    mut scf: *mut SCF,
    mut w: *const libc::c_double,
) {
    let mut n0: i32 = (*scf).n0;
    let mut nn: i32 = (*scf).nn;
    let mut sva: *mut SVA = (*scf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut rr_ref: i32 = (*scf).rr_ref;
    let mut rr_ptr: *mut i32 = &mut *((*sva).ptr).offset((rr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut rr_len: *mut i32 = &mut *((*sva).len).offset((rr_ref - 1 as i32) as isize)
        as *mut i32;
    let mut j: i32 = 0;
    let mut len: i32 = 0;
    let mut ptr: i32 = 0;
    (0 as i32 <= nn && nn < (*scf).nn_max
        || {
            glp_assert_(
                b"0 <= nn && nn < scf->nn_max\0" as *const u8 as *const i8,
                b"bflib/scf.c\0" as *const u8 as *const i8,
                347 as i32,
            );
            1 as i32 != 0
        }) as i32;
    len = 0 as i32;
    j = 1 as i32;
    while j <= n0 {
        if *w.offset(j as isize) != 0.0f64 {
            len += 1;
            len;
        }
        j += 1;
        j;
    }
    if len > 0 as i32 {
        if (*sva).r_ptr - (*sva).m_ptr < len {
            _glp_sva_more_space(sva, len);
            sv_ind = (*sva).ind;
            sv_val = (*sva).val;
        }
        _glp_sva_reserve_cap(sva, rr_ref + nn, len);
    }
    ptr = *rr_ptr.offset((nn + 1 as i32) as isize);
    j = 1 as i32;
    while j <= n0 {
        if *w.offset(j as isize) != 0.0f64 {
            *sv_ind.offset(ptr as isize) = j;
            *sv_val.offset(ptr as isize) = *w.offset(j as isize);
            ptr += 1;
            ptr;
        }
        j += 1;
        j;
    }
    (ptr - *rr_ptr.offset((nn + 1 as i32) as isize) == len
        || {
            glp_assert_(
                b"ptr - rr_ptr[nn+1] == len\0" as *const u8 as *const i8,
                b"bflib/scf.c\0" as *const u8 as *const i8,
                372 as i32,
            );
            1 as i32 != 0
        }) as i32;
    *rr_len.offset((nn + 1 as i32) as isize) = len;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scf_add_s_col(
    mut scf: *mut SCF,
    mut v: *const libc::c_double,
) {
    let mut n0: i32 = (*scf).n0;
    let mut nn: i32 = (*scf).nn;
    let mut sva: *mut SVA = (*scf).sva;
    let mut sv_ind: *mut i32 = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut ss_ref: i32 = (*scf).ss_ref;
    let mut ss_ptr: *mut i32 = &mut *((*sva).ptr).offset((ss_ref - 1 as i32) as isize)
        as *mut i32;
    let mut ss_len: *mut i32 = &mut *((*sva).len).offset((ss_ref - 1 as i32) as isize)
        as *mut i32;
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    let mut ptr: i32 = 0;
    (0 as i32 <= nn && nn < (*scf).nn_max
        || {
            glp_assert_(
                b"0 <= nn && nn < scf->nn_max\0" as *const u8 as *const i8,
                b"bflib/scf.c\0" as *const u8 as *const i8,
                396 as i32,
            );
            1 as i32 != 0
        }) as i32;
    len = 0 as i32;
    i = 1 as i32;
    while i <= n0 {
        if *v.offset(i as isize) != 0.0f64 {
            len += 1;
            len;
        }
        i += 1;
        i;
    }
    if len > 0 as i32 {
        if (*sva).r_ptr - (*sva).m_ptr < len {
            _glp_sva_more_space(sva, len);
            sv_ind = (*sva).ind;
            sv_val = (*sva).val;
        }
        _glp_sva_reserve_cap(sva, ss_ref + nn, len);
    }
    ptr = *ss_ptr.offset((nn + 1 as i32) as isize);
    i = 1 as i32;
    while i <= n0 {
        if *v.offset(i as isize) != 0.0f64 {
            *sv_ind.offset(ptr as isize) = i;
            *sv_val.offset(ptr as isize) = *v.offset(i as isize);
            ptr += 1;
            ptr;
        }
        i += 1;
        i;
    }
    (ptr - *ss_ptr.offset((nn + 1 as i32) as isize) == len
        || {
            glp_assert_(
                b"ptr - ss_ptr[nn+1] == len\0" as *const u8 as *const i8,
                b"bflib/scf.c\0" as *const u8 as *const i8,
                421 as i32,
            );
            1 as i32 != 0
        }) as i32;
    *ss_len.offset((nn + 1 as i32) as isize) = len;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scf_update_aug(
    mut scf: *mut SCF,
    mut b: *mut libc::c_double,
    mut d: *mut libc::c_double,
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut h: libc::c_double,
    mut upd: i32,
    mut w1: *mut libc::c_double,
    mut w2: *mut libc::c_double,
    mut w3: *mut libc::c_double,
) -> i32 {
    let mut n0: i32 = (*scf).n0;
    let mut k: i32 = 0;
    let mut ret: i32 = 0;
    let mut v: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut w: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut z: libc::c_double = 0.;
    if (*scf).nn == (*scf).nn_max {
        return 1 as i32;
    }
    v = b;
    _glp_scf_r0_solve(scf, 0 as i32, v);
    w = d;
    _glp_scf_s0_solve(scf, 1 as i32, w, w1, w2, w3);
    x = f;
    _glp_scf_r_prod(scf, x, -1.0f64, v as *const libc::c_double);
    y = g;
    _glp_scf_st_prod(scf, y, -1.0f64, w as *const libc::c_double);
    z = h;
    k = 1 as i32;
    while k <= n0 {
        z -= *v.offset(k as isize) * *w.offset(k as isize);
        k += 1;
        k;
    }
    _glp_scf_add_r_row(scf, w as *const libc::c_double);
    _glp_scf_add_s_col(scf, v as *const libc::c_double);
    match upd {
        1 => {
            ret = _glp_ifu_bg_update(&mut (*scf).ifu, x, y, z);
        }
        2 => {
            ret = _glp_ifu_gr_update(&mut (*scf).ifu, x, y, z);
        }
        _ => {
            (upd != upd
                || {
                    glp_assert_(
                        b"upd != upd\0" as *const u8 as *const i8,
                        b"bflib/scf.c\0" as *const u8 as *const i8,
                        505 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    if ret != 0 as i32 {
        return 2 as i32;
    }
    (*scf).nn += 1;
    (*scf).nn;
    k = n0 + (*scf).nn;
    let ref mut fresh0 = *((*scf).pp_inv).offset(k as isize);
    *fresh0 = k;
    *((*scf).pp_ind).offset(k as isize) = *fresh0;
    let ref mut fresh1 = *((*scf).qq_inv).offset(k as isize);
    *fresh1 = k;
    *((*scf).qq_ind).offset(k as isize) = *fresh1;
    return 0 as i32;
}