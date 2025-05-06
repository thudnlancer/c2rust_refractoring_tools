#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn _glp_amd_aat(
    mut n: i32,
    mut Ap: *const i32,
    mut Ai: *const i32,
    mut Len: *mut i32,
    mut Tp: *mut i32,
    mut Info: *mut libc::c_double,
) -> size_t {
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;
    let mut p: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pj: i32 = 0;
    let mut pj2: i32 = 0;
    let mut k: i32 = 0;
    let mut nzdiag: i32 = 0;
    let mut nzboth: i32 = 0;
    let mut nz: i32 = 0;
    let mut sym: libc::c_double = 0.;
    let mut nzaat: size_t = 0;
    if !Info.is_null() {
        i = 0 as i32;
        while i < 20 as i32 {
            *Info.offset(i as isize) = -(1 as i32) as libc::c_double;
            i += 1;
            i;
        }
        *Info.offset(0 as i32 as isize) = 0 as i32 as libc::c_double;
    }
    k = 0 as i32;
    while k < n {
        *Len.offset(k as isize) = 0 as i32;
        k += 1;
        k;
    }
    nzdiag = 0 as i32;
    nzboth = 0 as i32;
    nz = *Ap.offset(n as isize);
    k = 0 as i32;
    while k < n {
        p1 = *Ap.offset(k as isize);
        p2 = *Ap.offset((k + 1 as i32) as isize);
        p = p1;
        while p < p2 {
            j = *Ai.offset(p as isize);
            if j < k {
                let ref mut fresh0 = *Len.offset(j as isize);
                *fresh0 += 1;
                *fresh0;
                let ref mut fresh1 = *Len.offset(k as isize);
                *fresh1 += 1;
                *fresh1;
                p += 1;
                p;
                pj2 = *Ap.offset((j + 1 as i32) as isize);
                pj = *Tp.offset(j as isize);
                while pj < pj2 {
                    i = *Ai.offset(pj as isize);
                    if i < k {
                        let ref mut fresh2 = *Len.offset(i as isize);
                        *fresh2 += 1;
                        *fresh2;
                        let ref mut fresh3 = *Len.offset(j as isize);
                        *fresh3 += 1;
                        *fresh3;
                        pj += 1;
                        pj;
                    } else if i == k {
                        pj += 1;
                        pj;
                        nzboth += 1;
                        nzboth;
                        break;
                    } else {
                        break;
                    }
                }
                *Tp.offset(j as isize) = pj;
            } else if j == k {
                p += 1;
                p;
                nzdiag += 1;
                nzdiag;
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
            let ref mut fresh4 = *Len.offset(i as isize);
            *fresh4 += 1;
            *fresh4;
            let ref mut fresh5 = *Len.offset(j as isize);
            *fresh5 += 1;
            *fresh5;
            pj += 1;
            pj;
        }
        j += 1;
        j;
    }
    if nz == nzdiag {
        sym = 1 as i32 as libc::c_double;
    } else {
        sym = 2 as i32 as libc::c_double * nzboth as libc::c_double
            / (nz - nzdiag) as libc::c_double;
    }
    nzaat = 0 as i32 as size_t;
    k = 0 as i32;
    while k < n {
        nzaat = (nzaat as u64).wrapping_add(*Len.offset(k as isize) as u64) as size_t
            as size_t;
        k += 1;
        k;
    }
    if !Info.is_null() {
        *Info.offset(0 as i32 as isize) = 0 as i32 as libc::c_double;
        *Info.offset(1 as i32 as isize) = n as libc::c_double;
        *Info.offset(2 as i32 as isize) = nz as libc::c_double;
        *Info.offset(3 as i32 as isize) = sym;
        *Info.offset(4 as i32 as isize) = nzdiag as libc::c_double;
        *Info.offset(5 as i32 as isize) = nzaat as libc::c_double;
    }
    return nzaat;
}