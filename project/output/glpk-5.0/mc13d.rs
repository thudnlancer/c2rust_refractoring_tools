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
pub unsafe extern "C" fn _glp_mc13d(
    mut n: i32,
    mut icn: *const i32,
    mut ip: *const i32,
    mut lenr: *const i32,
    mut ior: *mut i32,
    mut ib: *mut i32,
    mut lowl: *mut i32,
    mut numb: *mut i32,
    mut prev: *mut i32,
) -> i32 {
    let mut current_block: u64;
    let mut arp: *mut i32 = ior;
    let mut dummy: i32 = 0;
    let mut i: i32 = 0;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut icnt: i32 = 0;
    let mut ii: i32 = 0;
    let mut isn: i32 = 0;
    let mut ist: i32 = 0;
    let mut ist1: i32 = 0;
    let mut iv: i32 = 0;
    let mut iw: i32 = 0;
    let mut j: i32 = 0;
    let mut lcnt: i32 = 0;
    let mut nnm1: i32 = 0;
    let mut num: i32 = 0;
    let mut stp: i32 = 0;
    icnt = 0 as i32;
    num = 0 as i32;
    nnm1 = n + n - 1 as i32;
    j = 1 as i32;
    while j <= n {
        *numb.offset(j as isize) = 0 as i32;
        *arp.offset(j as isize) = *lenr.offset(j as isize) - 1 as i32;
        j += 1;
        j;
    }
    isn = 1 as i32;
    's_36: while isn <= n {
        if !(*numb.offset(isn as isize) != 0 as i32) {
            iv = isn;
            ist = 1 as i32;
            let ref mut fresh0 = *numb.offset(iv as isize);
            *fresh0 = 1 as i32;
            *lowl.offset(iv as isize) = *fresh0;
            *ib.offset(n as isize) = iv;
            dummy = 1 as i32;
            while dummy <= nnm1 {
                i1 = *arp.offset(iv as isize);
                if i1 >= 0 as i32 {
                    i2 = *ip.offset(iv as isize) + *lenr.offset(iv as isize) - 1 as i32;
                    i1 = i2 - i1;
                    ii = i1;
                    loop {
                        if !(ii <= i2) {
                            current_block = 5783071609795492627;
                            break;
                        }
                        iw = *icn.offset(ii as isize);
                        if *numb.offset(iw as isize) == 0 as i32 {
                            current_block = 17973567135127694340;
                            break;
                        }
                        if *lowl.offset(iw as isize) < *lowl.offset(iv as isize) {
                            *lowl.offset(iv as isize) = *lowl.offset(iw as isize);
                        }
                        ii += 1;
                        ii;
                    }
                    match current_block {
                        17973567135127694340 => {
                            *arp.offset(iv as isize) = i2 - ii - 1 as i32;
                            *prev.offset(iw as isize) = iv;
                            iv = iw;
                            ist += 1;
                            let ref mut fresh1 = *numb.offset(iv as isize);
                            *fresh1 = ist;
                            *lowl.offset(iv as isize) = *fresh1;
                            *ib.offset((n + 1 as i32 - ist) as isize) = iv;
                            current_block = 1856101646708284338;
                        }
                        _ => {
                            *arp.offset(iv as isize) = -(1 as i32);
                            current_block = 5634871135123216486;
                        }
                    }
                } else {
                    current_block = 5634871135123216486;
                }
                match current_block {
                    5634871135123216486 => {
                        if !(*lowl.offset(iv as isize) < *numb.offset(iv as isize)) {
                            num += 1;
                            num;
                            ist1 = n + 1 as i32 - ist;
                            lcnt = icnt + 1 as i32;
                            stp = ist1;
                            while stp <= n {
                                iw = *ib.offset(stp as isize);
                                *lowl.offset(iw as isize) = n + 1 as i32;
                                icnt += 1;
                                *numb.offset(iw as isize) = icnt;
                                if iw == iv {
                                    break;
                                }
                                stp += 1;
                                stp;
                            }
                            ist = n - stp;
                            *ib.offset(num as isize) = lcnt;
                            if !(ist != 0 as i32) {
                                if icnt < n {
                                    break;
                                } else {
                                    break 's_36;
                                }
                            }
                        }
                        iw = iv;
                        iv = *prev.offset(iv as isize);
                        if *lowl.offset(iw as isize) < *lowl.offset(iv as isize) {
                            *lowl.offset(iv as isize) = *lowl.offset(iw as isize);
                        }
                    }
                    _ => {}
                }
                dummy += 1;
                dummy;
            }
        }
        isn += 1;
        isn;
    }
    i = 1 as i32;
    while i <= n {
        *arp.offset(*numb.offset(i as isize) as isize) = i;
        i += 1;
        i;
    }
    return num;
}