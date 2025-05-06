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
    fn _glp_amd_valid(n_row: i32, n_col: i32, Ap: *const i32, Ai: *const i32) -> i32;
    fn _glp_amd_preprocess(
        n: i32,
        Ap: *const i32,
        Ai: *const i32,
        Rp: *mut i32,
        Ri: *mut i32,
        W: *mut i32,
        Flag: *mut i32,
    );
    fn _glp_amd_aat(
        n: i32,
        Ap: *const i32,
        Ai: *const i32,
        Len: *mut i32,
        Tp: *mut i32,
        Info: *mut libc::c_double,
    ) -> size_t;
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn _glp_amd_1(
        n: i32,
        Ap: *const i32,
        Ai: *const i32,
        P: *mut i32,
        Pinv: *mut i32,
        Len: *mut i32,
        slen: i32,
        S: *mut i32,
        Control: *mut libc::c_double,
        Info: *mut libc::c_double,
    );
    fn glp_free(ptr: *mut libc::c_void);
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn _glp_amd_order(
    mut n: i32,
    mut Ap: *const i32,
    mut Ai: *const i32,
    mut P: *mut i32,
    mut Control: *mut libc::c_double,
    mut Info: *mut libc::c_double,
) -> i32 {
    let mut Len: *mut i32 = 0 as *mut i32;
    let mut S: *mut i32 = 0 as *mut i32;
    let mut nz: i32 = 0;
    let mut i: i32 = 0;
    let mut Pinv: *mut i32 = 0 as *mut i32;
    let mut info: i32 = 0;
    let mut status: i32 = 0;
    let mut Rp: *mut i32 = 0 as *mut i32;
    let mut Ri: *mut i32 = 0 as *mut i32;
    let mut Cp: *mut i32 = 0 as *mut i32;
    let mut Ci: *mut i32 = 0 as *mut i32;
    let mut ok: i32 = 0;
    let mut nzaat: size_t = 0;
    let mut slen: size_t = 0;
    let mut mem: libc::c_double = 0 as i32 as libc::c_double;
    info = (Info != 0 as *mut libc::c_void as *mut libc::c_double) as i32;
    if info != 0 {
        i = 0 as i32;
        while i < 20 as i32 {
            *Info.offset(i as isize) = -(1 as i32) as libc::c_double;
            i += 1;
            i;
        }
        *Info.offset(1 as i32 as isize) = n as libc::c_double;
        *Info.offset(0 as i32 as isize) = 0 as i32 as libc::c_double;
    }
    if Ai.is_null() || Ap.is_null() || P.is_null() || n < 0 as i32 {
        if info != 0 {
            *Info.offset(0 as i32 as isize) = -(2 as i32) as libc::c_double;
        }
        return -(2 as i32);
    }
    if n == 0 as i32 {
        return 0 as i32;
    }
    nz = *Ap.offset(n as isize);
    if info != 0 {
        *Info.offset(2 as i32 as isize) = nz as libc::c_double;
    }
    if nz < 0 as i32 {
        if info != 0 {
            *Info.offset(0 as i32 as isize) = -(2 as i32) as libc::c_double;
        }
        return -(2 as i32);
    }
    if n as size_t
        >= (!(0 as i32 as size_t)).wrapping_div(::core::mem::size_of::<i32>() as u64)
        || nz as size_t
            >= (!(0 as i32 as size_t)).wrapping_div(::core::mem::size_of::<i32>() as u64)
    {
        if info != 0 {
            *Info.offset(0 as i32 as isize) = -(1 as i32) as libc::c_double;
        }
        return -(1 as i32);
    }
    status = _glp_amd_valid(n, n, Ap, Ai);
    if status == -(2 as i32) {
        if info != 0 {
            *Info.offset(0 as i32 as isize) = -(2 as i32) as libc::c_double;
        }
        return -(2 as i32);
    }
    Len = glp_alloc(
        1 as i32,
        (n as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64) as i32,
    ) as *mut i32;
    Pinv = glp_alloc(
        1 as i32,
        (n as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64) as i32,
    ) as *mut i32;
    mem += n as libc::c_double;
    mem += n as libc::c_double;
    if Len.is_null() || Pinv.is_null() {
        if !Len.is_null() {
            glp_free(Len as *mut libc::c_void);
        }
        if !Pinv.is_null() {
            glp_free(Pinv as *mut libc::c_void);
        }
        if info != 0 {
            *Info.offset(0 as i32 as isize) = -(1 as i32) as libc::c_double;
        }
        return -(1 as i32);
    }
    if status == 1 as i32 {
        Rp = glp_alloc(
            1 as i32,
            ((n + 1 as i32) as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64)
                as i32,
        ) as *mut i32;
        Ri = glp_alloc(
            1 as i32,
            ((if nz > 1 as i32 { nz } else { 1 as i32 }) as u64)
                .wrapping_mul(::core::mem::size_of::<i32>() as u64) as i32,
        ) as *mut i32;
        mem += (n + 1 as i32) as libc::c_double;
        mem += (if nz > 1 as i32 { nz } else { 1 as i32 }) as libc::c_double;
        if Rp.is_null() || Ri.is_null() {
            if !Rp.is_null() {
                glp_free(Rp as *mut libc::c_void);
            }
            if !Ri.is_null() {
                glp_free(Ri as *mut libc::c_void);
            }
            if !Len.is_null() {
                glp_free(Len as *mut libc::c_void);
            }
            if !Pinv.is_null() {
                glp_free(Pinv as *mut libc::c_void);
            }
            if info != 0 {
                *Info.offset(0 as i32 as isize) = -(1 as i32) as libc::c_double;
            }
            return -(1 as i32);
        }
        _glp_amd_preprocess(n, Ap, Ai, Rp, Ri, Len, Pinv);
        Cp = Rp;
        Ci = Ri;
    } else {
        Rp = 0 as *mut i32;
        Ri = 0 as *mut i32;
        Cp = Ap as *mut i32;
        Ci = Ai as *mut i32;
    }
    nzaat = _glp_amd_aat(n, Cp as *const i32, Ci as *const i32, Len, P, Info);
    S = 0 as *mut i32;
    slen = nzaat;
    ok = (slen.wrapping_add(nzaat.wrapping_div(5 as i32 as u64)) >= slen) as i32;
    slen = (slen as u64).wrapping_add(nzaat.wrapping_div(5 as i32 as u64)) as size_t
        as size_t;
    i = 0 as i32;
    while ok != 0 && i < 7 as i32 {
        ok = (slen.wrapping_add(n as u64) > slen) as i32;
        slen = (slen as u64).wrapping_add(n as u64) as size_t as size_t;
        i += 1;
        i;
    }
    mem += slen as libc::c_double;
    ok = (ok != 0
        && slen
            < (!(0 as i32 as size_t)).wrapping_div(::core::mem::size_of::<i32>() as u64))
        as i32;
    ok = (ok != 0 && slen < 2147483647 as i32 as u64) as i32;
    if ok != 0 {
        S = glp_alloc(
            1 as i32,
            slen.wrapping_mul(::core::mem::size_of::<i32>() as u64) as i32,
        ) as *mut i32;
    }
    if S.is_null() {
        if !Rp.is_null() {
            glp_free(Rp as *mut libc::c_void);
        }
        if !Ri.is_null() {
            glp_free(Ri as *mut libc::c_void);
        }
        if !Len.is_null() {
            glp_free(Len as *mut libc::c_void);
        }
        if !Pinv.is_null() {
            glp_free(Pinv as *mut libc::c_void);
        }
        if info != 0 {
            *Info.offset(0 as i32 as isize) = -(1 as i32) as libc::c_double;
        }
        return -(1 as i32);
    }
    if info != 0 {
        *Info.offset(7 as i32 as isize) = mem
            * ::core::mem::size_of::<i32>() as u64 as libc::c_double;
    }
    _glp_amd_1(
        n,
        Cp as *const i32,
        Ci as *const i32,
        P,
        Pinv,
        Len,
        slen as i32,
        S,
        Control,
        Info,
    );
    if !Rp.is_null() {
        glp_free(Rp as *mut libc::c_void);
    }
    if !Ri.is_null() {
        glp_free(Ri as *mut libc::c_void);
    }
    if !Len.is_null() {
        glp_free(Len as *mut libc::c_void);
    }
    if !Pinv.is_null() {
        glp_free(Pinv as *mut libc::c_void);
    }
    if !S.is_null() {
        glp_free(S as *mut libc::c_void);
    }
    if info != 0 {
        *Info.offset(0 as i32 as isize) = status as libc::c_double;
    }
    return status;
}