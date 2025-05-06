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
    fn _glp_amd_2(
        n: i32,
        Pe: *mut i32,
        Iw: *mut i32,
        Len: *mut i32,
        iwlen: i32,
        pfree: i32,
        Nv: *mut i32,
        Next: *mut i32,
        Last: *mut i32,
        Head: *mut i32,
        Elen: *mut i32,
        Degree: *mut i32,
        W: *mut i32,
        Control: *mut libc::c_double,
        Info: *mut libc::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_amd_1(
    mut n: i32,
    mut Ap: *const i32,
    mut Ai: *const i32,
    mut P: *mut i32,
    mut Pinv: *mut i32,
    mut Len: *mut i32,
    mut slen: i32,
    mut S: *mut i32,
    mut Control: *mut libc::c_double,
    mut Info: *mut libc::c_double,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut p: i32 = 0;
    let mut pfree: i32 = 0;
    let mut iwlen: i32 = 0;
    let mut pj: i32 = 0;
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;
    let mut pj2: i32 = 0;
    let mut Iw: *mut i32 = 0 as *mut i32;
    let mut Pe: *mut i32 = 0 as *mut i32;
    let mut Nv: *mut i32 = 0 as *mut i32;
    let mut Head: *mut i32 = 0 as *mut i32;
    let mut Elen: *mut i32 = 0 as *mut i32;
    let mut Degree: *mut i32 = 0 as *mut i32;
    let mut s: *mut i32 = 0 as *mut i32;
    let mut W: *mut i32 = 0 as *mut i32;
    let mut Sp: *mut i32 = 0 as *mut i32;
    let mut Tp: *mut i32 = 0 as *mut i32;
    iwlen = slen - 6 as i32 * n;
    s = S;
    Pe = s;
    s = s.offset(n as isize);
    Nv = s;
    s = s.offset(n as isize);
    Head = s;
    s = s.offset(n as isize);
    Elen = s;
    s = s.offset(n as isize);
    Degree = s;
    s = s.offset(n as isize);
    W = s;
    s = s.offset(n as isize);
    Iw = s;
    s = s.offset(iwlen as isize);
    Sp = Nv;
    Tp = W;
    pfree = 0 as i32;
    j = 0 as i32;
    while j < n {
        *Pe.offset(j as isize) = pfree;
        *Sp.offset(j as isize) = pfree;
        pfree += *Len.offset(j as isize);
        j += 1;
        j;
    }
    k = 0 as i32;
    while k < n {
        p1 = *Ap.offset(k as isize);
        p2 = *Ap.offset((k + 1 as i32) as isize);
        p = p1;
        while p < p2 {
            j = *Ai.offset(p as isize);
            if j < k {
                let ref mut fresh0 = *Sp.offset(j as isize);
                let fresh1 = *fresh0;
                *fresh0 = *fresh0 + 1;
                *Iw.offset(fresh1 as isize) = k;
                let ref mut fresh2 = *Sp.offset(k as isize);
                let fresh3 = *fresh2;
                *fresh2 = *fresh2 + 1;
                *Iw.offset(fresh3 as isize) = j;
                p += 1;
                p;
                pj2 = *Ap.offset((j + 1 as i32) as isize);
                pj = *Tp.offset(j as isize);
                while pj < pj2 {
                    i = *Ai.offset(pj as isize);
                    if i < k {
                        let ref mut fresh4 = *Sp.offset(i as isize);
                        let fresh5 = *fresh4;
                        *fresh4 = *fresh4 + 1;
                        *Iw.offset(fresh5 as isize) = j;
                        let ref mut fresh6 = *Sp.offset(j as isize);
                        let fresh7 = *fresh6;
                        *fresh6 = *fresh6 + 1;
                        *Iw.offset(fresh7 as isize) = i;
                        pj += 1;
                        pj;
                    } else if i == k {
                        pj += 1;
                        pj;
                        break;
                    } else {
                        break;
                    }
                }
                *Tp.offset(j as isize) = pj;
            } else if j == k {
                p += 1;
                p;
                break;
            } else {
                break;
            }
        }
        *Tp.offset(k as isize) = p;
        k += 1;
        k;
    }
    j = 0 as i32;
    while j < n {
        pj = *Tp.offset(j as isize);
        while pj < *Ap.offset((j + 1 as i32) as isize) {
            i = *Ai.offset(pj as isize);
            let ref mut fresh8 = *Sp.offset(i as isize);
            let fresh9 = *fresh8;
            *fresh8 = *fresh8 + 1;
            *Iw.offset(fresh9 as isize) = j;
            let ref mut fresh10 = *Sp.offset(j as isize);
            let fresh11 = *fresh10;
            *fresh10 = *fresh10 + 1;
            *Iw.offset(fresh11 as isize) = i;
            pj += 1;
            pj;
        }
        j += 1;
        j;
    }
    _glp_amd_2(
        n,
        Pe,
        Iw,
        Len,
        iwlen,
        pfree,
        Nv,
        Pinv,
        P,
        Head,
        Elen,
        Degree,
        W,
        Control,
        Info,
    );
}