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
pub unsafe extern "C" fn _glp_mc21a(
    mut n: i32,
    mut icn: *const i32,
    mut ip: *const i32,
    mut lenr: *const i32,
    mut iperm: *mut i32,
    mut pr: *mut i32,
    mut arp: *mut i32,
    mut cv: *mut i32,
    mut out: *mut i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut ii: i32 = 0;
    let mut in1: i32 = 0;
    let mut in2: i32 = 0;
    let mut j: i32 = 0;
    let mut j1: i32 = 0;
    let mut jord: i32 = 0;
    let mut k: i32 = 0;
    let mut kk: i32 = 0;
    let mut numnz: i32 = 0;
    i = 1 as i32;
    while i <= n {
        *arp.offset(i as isize) = *lenr.offset(i as isize) - 1 as i32;
        let ref mut fresh0 = *iperm.offset(i as isize);
        *fresh0 = 0 as i32;
        *cv.offset(i as isize) = *fresh0;
        i += 1;
        i;
    }
    numnz = 0 as i32;
    jord = 1 as i32;
    while jord <= n {
        let mut current_block_39: u64;
        j = jord;
        *pr.offset(j as isize) = -(1 as i32);
        k = 1 as i32;
        's_43: loop {
            if !(k <= jord) {
                current_block_39 = 7356893052265527703;
                break;
            }
            in1 = *arp.offset(j as isize);
            if in1 >= 0 as i32 {
                in2 = *ip.offset(j as isize) + *lenr.offset(j as isize) - 1 as i32;
                in1 = in2 - in1;
                ii = in1;
                while ii <= in2 {
                    i = *icn.offset(ii as isize);
                    if *iperm.offset(i as isize) == 0 as i32 {
                        current_block_39 = 7356893052265527703;
                        break 's_43;
                    }
                    ii += 1;
                    ii;
                }
                *arp.offset(j as isize) = -(1 as i32);
            }
            *out.offset(j as isize) = *lenr.offset(j as isize) - 1 as i32;
            kk = 1 as i32;
            's_95: while kk <= jord {
                in1 = *out.offset(j as isize);
                if in1 >= 0 as i32 {
                    in2 = *ip.offset(j as isize) + *lenr.offset(j as isize) - 1 as i32;
                    in1 = in2 - in1;
                    ii = in1;
                    while ii <= in2 {
                        i = *icn.offset(ii as isize);
                        if *cv.offset(i as isize) != jord {
                            j1 = j;
                            j = *iperm.offset(i as isize);
                            *cv.offset(i as isize) = jord;
                            *pr.offset(j as isize) = j1;
                            *out.offset(j1 as isize) = in2 - ii - 1 as i32;
                            break 's_95;
                        } else {
                            ii += 1;
                            ii;
                        }
                    }
                }
                j = *pr.offset(j as isize);
                if j == -(1 as i32) {
                    current_block_39 = 15004371738079956865;
                    break 's_43;
                }
                kk += 1;
                kk;
            }
            k += 1;
            k;
        }
        match current_block_39 {
            7356893052265527703 => {
                *iperm.offset(i as isize) = j;
                *arp.offset(j as isize) = in2 - ii - 1 as i32;
                numnz += 1;
                numnz;
                k = 1 as i32;
                while k <= jord {
                    j = *pr.offset(j as isize);
                    if j == -(1 as i32) {
                        break;
                    }
                    ii = *ip.offset(j as isize) + *lenr.offset(j as isize)
                        - *out.offset(j as isize) - 2 as i32;
                    i = *icn.offset(ii as isize);
                    *iperm.offset(i as isize) = j;
                    k += 1;
                    k;
                }
            }
            _ => {}
        }
        jord += 1;
        jord;
    }
    if numnz < n {
        i = 1 as i32;
        while i <= n {
            *arp.offset(i as isize) = 0 as i32;
            i += 1;
            i;
        }
        k = 0 as i32;
        i = 1 as i32;
        while i <= n {
            if *iperm.offset(i as isize) == 0 as i32 {
                k += 1;
                *out.offset(k as isize) = i;
            } else {
                *arp.offset(*iperm.offset(i as isize) as isize) = i;
            }
            i += 1;
            i;
        }
        k = 0 as i32;
        i = 1 as i32;
        while i <= n {
            if *arp.offset(i as isize) == 0 as i32 {
                k += 1;
                *iperm.offset(*out.offset(k as isize) as isize) = i;
            }
            i += 1;
            i;
        }
    }
    return numnz;
}