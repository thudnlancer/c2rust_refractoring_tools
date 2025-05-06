#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn _glp_amd_preprocess(
    mut n: i32,
    mut Ap: *const i32,
    mut Ai: *const i32,
    mut Rp: *mut i32,
    mut Ri: *mut i32,
    mut W: *mut i32,
    mut Flag: *mut i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p: i32 = 0;
    let mut p2: i32 = 0;
    i = 0 as i32;
    while i < n {
        *W.offset(i as isize) = 0 as i32;
        *Flag.offset(i as isize) = -(1 as i32);
        i += 1;
        i;
    }
    j = 0 as i32;
    while j < n {
        p2 = *Ap.offset((j + 1 as i32) as isize);
        p = *Ap.offset(j as isize);
        while p < p2 {
            i = *Ai.offset(p as isize);
            if *Flag.offset(i as isize) != j {
                let ref mut fresh0 = *W.offset(i as isize);
                *fresh0 += 1;
                *fresh0;
                *Flag.offset(i as isize) = j;
            }
            p += 1;
            p;
        }
        j += 1;
        j;
    }
    *Rp.offset(0 as i32 as isize) = 0 as i32;
    i = 0 as i32;
    while i < n {
        *Rp.offset((i + 1 as i32) as isize) = *Rp.offset(i as isize)
            + *W.offset(i as isize);
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < n {
        *W.offset(i as isize) = *Rp.offset(i as isize);
        *Flag.offset(i as isize) = -(1 as i32);
        i += 1;
        i;
    }
    j = 0 as i32;
    while j < n {
        p2 = *Ap.offset((j + 1 as i32) as isize);
        p = *Ap.offset(j as isize);
        while p < p2 {
            i = *Ai.offset(p as isize);
            if *Flag.offset(i as isize) != j {
                let ref mut fresh1 = *W.offset(i as isize);
                let fresh2 = *fresh1;
                *fresh1 = *fresh1 + 1;
                *Ri.offset(fresh2 as isize) = j;
                *Flag.offset(i as isize) = j;
            }
            p += 1;
            p;
        }
        j += 1;
        j;
    }
}