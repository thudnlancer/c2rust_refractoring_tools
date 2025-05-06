#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use num_traits::ToPrimitive;
extern "C" {
    fn gsl_sort_long_double(data: *mut f128::f128, stride: size_t, n: size_t);
    fn gsl_sort(data: *mut libc::c_double, stride: size_t, n: size_t);
    fn gsl_sort_float(data: *mut libc::c_float, stride: size_t, n: size_t);
    fn gsl_sort_char(data: *mut i8, stride: size_t, n: size_t);
    fn gsl_sort_uchar(data: *mut u8, stride: size_t, n: size_t);
    fn gsl_sort_ulong(data: *mut u64, stride: size_t, n: size_t);
    fn gsl_sort_int(data: *mut i32, stride: size_t, n: size_t);
    fn gsl_sort_long(data: *mut i64, stride: size_t, n: size_t);
    fn gsl_sort_uint(data: *mut u32, stride: size_t, n: size_t);
    fn gsl_sort_ushort(data: *mut libc::c_ushort, stride: size_t, n: size_t);
    fn gsl_sort_short(data: *mut libc::c_short, stride: size_t, n: size_t);
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_Sn0_from_sorted_data(
    mut sorted_data: *const u32,
    stride: size_t,
    n: size_t,
    mut work: *mut u32,
) -> u32 {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: i32 = 0;
    let mut diff: i32 = 0;
    let mut half: i32 = 0;
    let mut Amin: i32 = 0;
    let mut Amax: i32 = 0;
    let mut even: i32 = 0;
    let mut length: i32 = 0;
    let mut leftA: i32 = 0;
    let mut leftB: i32 = 0;
    let mut nA: i32 = 0;
    let mut nB: i32 = 0;
    let mut tryA: i32 = 0;
    let mut tryB: i32 = 0;
    let mut rightA: i32 = 0;
    let mut rightB: i32 = 0;
    let mut np1_2: i32 = n.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
        as i32;
    *work.offset(0 as i32 as isize) = (*sorted_data
        .offset(n.wrapping_div(2 as i32 as u64).wrapping_mul(stride) as isize))
        .wrapping_sub(*sorted_data.offset(0 as i32 as isize));
    i = 2 as i32;
    while i <= np1_2 {
        nA = i - 1 as i32;
        nB = n.wrapping_sub(i as u64) as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - tryA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                    as isize,
                            ),
                    ) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((tryB + i - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    ))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - 1 as i32) as u64).wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize),
                );
        } else {
            medA = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - leftA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                as isize,
                        ),
                ) as libc::c_double;
            medB = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize),
                ) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as u32;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as i32;
    while i <= n as i32 - 1 as i32 {
        nA = n.wrapping_sub(i as u64) as i32;
        nB = i - 1 as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i + tryA - Amin) as u64).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - 1 as i32) as u64).wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                medB = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - tryB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ),
                );
        } else {
            medA = (*sorted_data
                .offset(((i + leftA - Amin) as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize),
                ) as libc::c_double;
            medB = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as u32;
        }
        i += 1;
        i;
    }
    *work.offset(n.wrapping_sub(1 as i32 as u64) as isize) = (*sorted_data
        .offset(n.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize))
        .wrapping_sub(
            *sorted_data
                .offset(((np1_2 - 1 as i32) as u64).wrapping_mul(stride) as isize),
        );
    gsl_sort_uint(work, 1 as i32 as size_t, n);
    return *work.offset((np1_2 - 1 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_Sn0_from_sorted_data(
    mut sorted_data: *const u8,
    stride: size_t,
    n: size_t,
    mut work: *mut u8,
) -> u8 {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: i32 = 0;
    let mut diff: i32 = 0;
    let mut half: i32 = 0;
    let mut Amin: i32 = 0;
    let mut Amax: i32 = 0;
    let mut even: i32 = 0;
    let mut length: i32 = 0;
    let mut leftA: i32 = 0;
    let mut leftB: i32 = 0;
    let mut nA: i32 = 0;
    let mut nB: i32 = 0;
    let mut tryA: i32 = 0;
    let mut tryB: i32 = 0;
    let mut rightA: i32 = 0;
    let mut rightB: i32 = 0;
    let mut np1_2: i32 = n.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
        as i32;
    *work.offset(0 as i32 as isize) = (*sorted_data
        .offset(n.wrapping_div(2 as i32 as u64).wrapping_mul(stride) as isize) as i32
        - *sorted_data.offset(0 as i32 as isize) as i32) as u8;
    i = 2 as i32;
    while i <= np1_2 {
        nA = i - 1 as i32;
        nB = n.wrapping_sub(i as u64) as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                as isize,
                        ) as i32) as libc::c_double;
                medB = (*sorted_data
                    .offset(((tryB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                        as i32) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                as i32
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as u8;
        } else {
            medA = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                            as isize,
                    ) as i32) as libc::c_double;
            medB = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                as i32
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as u8;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as i32;
    while i <= n as i32 - 1 as i32 {
        nA = n.wrapping_sub(i as u64) as i32;
        nB = i - 1 as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i + tryA - Amin) as u64).wrapping_mul(stride) as isize)
                    as i32
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                        as i32) as libc::c_double;
                medB = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ) as i32) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    ) as i32) as u8;
        } else {
            medA = (*sorted_data
                .offset(((i + leftA - Amin) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_double;
            medB = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    ) as i32) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as u8;
        }
        i += 1;
        i;
    }
    *work.offset(n.wrapping_sub(1 as i32 as u64) as isize) = (*sorted_data
        .offset(n.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize) as i32
        - *sorted_data.offset(((np1_2 - 1 as i32) as u64).wrapping_mul(stride) as isize)
            as i32) as u8;
    gsl_sort_uchar(work, 1 as i32 as size_t, n);
    return *work.offset((np1_2 - 1 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_Sn0_from_sorted_data(
    mut sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mut work: *mut f128::f128,
) -> f128::f128 {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: i32 = 0;
    let mut diff: i32 = 0;
    let mut half: i32 = 0;
    let mut Amin: i32 = 0;
    let mut Amax: i32 = 0;
    let mut even: i32 = 0;
    let mut length: i32 = 0;
    let mut leftA: i32 = 0;
    let mut leftB: i32 = 0;
    let mut nA: i32 = 0;
    let mut nB: i32 = 0;
    let mut tryA: i32 = 0;
    let mut tryB: i32 = 0;
    let mut rightA: i32 = 0;
    let mut rightB: i32 = 0;
    let mut np1_2: i32 = n.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
        as i32;
    *work.offset(0 as i32 as isize) = *sorted_data
        .offset(n.wrapping_div(2 as i32 as u64).wrapping_mul(stride) as isize)
        - *sorted_data.offset(0 as i32 as isize);
    i = 2 as i32;
    while i <= np1_2 {
        nA = i - 1 as i32;
        nB = n.wrapping_sub(i as u64) as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                as isize,
                        ))
                    .to_f64()
                    .unwrap();
                medB = (*sorted_data
                    .offset(((tryB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                    .to_f64()
                    .unwrap();
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = *sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize);
        } else {
            medA = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                            as isize,
                    ))
                .to_f64()
                .unwrap();
            medB = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                .to_f64()
                .unwrap();
            *work.offset((i - 1 as i32) as isize) = f128::f128::new(
                if medA < medB { medA } else { medB },
            );
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as i32;
    while i <= n as i32 - 1 as i32 {
        nA = n.wrapping_sub(i as u64) as i32;
        nB = i - 1 as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i + tryA - Amin) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                    .to_f64()
                    .unwrap();
                medB = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ))
                    .to_f64()
                    .unwrap();
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = *sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    );
        } else {
            medA = (*sorted_data
                .offset(((i + leftA - Amin) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                .to_f64()
                .unwrap();
            medB = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    ))
                .to_f64()
                .unwrap();
            *work.offset((i - 1 as i32) as isize) = f128::f128::new(
                if medA < medB { medA } else { medB },
            );
        }
        i += 1;
        i;
    }
    *work.offset(n.wrapping_sub(1 as i32 as u64) as isize) = *sorted_data
        .offset(n.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
        - *sorted_data.offset(((np1_2 - 1 as i32) as u64).wrapping_mul(stride) as isize);
    gsl_sort_long_double(work, 1 as i32 as size_t, n);
    return *work.offset((np1_2 - 1 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_ushort,
) -> libc::c_ushort {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: i32 = 0;
    let mut diff: i32 = 0;
    let mut half: i32 = 0;
    let mut Amin: i32 = 0;
    let mut Amax: i32 = 0;
    let mut even: i32 = 0;
    let mut length: i32 = 0;
    let mut leftA: i32 = 0;
    let mut leftB: i32 = 0;
    let mut nA: i32 = 0;
    let mut nB: i32 = 0;
    let mut tryA: i32 = 0;
    let mut tryB: i32 = 0;
    let mut rightA: i32 = 0;
    let mut rightB: i32 = 0;
    let mut np1_2: i32 = n.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
        as i32;
    *work.offset(0 as i32 as isize) = (*sorted_data
        .offset(n.wrapping_div(2 as i32 as u64).wrapping_mul(stride) as isize) as i32
        - *sorted_data.offset(0 as i32 as isize) as i32) as libc::c_ushort;
    i = 2 as i32;
    while i <= np1_2 {
        nA = i - 1 as i32;
        nB = n.wrapping_sub(i as u64) as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                as isize,
                        ) as i32) as libc::c_double;
                medB = (*sorted_data
                    .offset(((tryB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                        as i32) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                as i32
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_ushort;
        } else {
            medA = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                            as isize,
                    ) as i32) as libc::c_double;
            medB = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                as i32
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as libc::c_ushort;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as i32;
    while i <= n as i32 - 1 as i32 {
        nA = n.wrapping_sub(i as u64) as i32;
        nB = i - 1 as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i + tryA - Amin) as u64).wrapping_mul(stride) as isize)
                    as i32
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                        as i32) as libc::c_double;
                medB = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ) as i32) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    ) as i32) as libc::c_ushort;
        } else {
            medA = (*sorted_data
                .offset(((i + leftA - Amin) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_double;
            medB = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    ) as i32) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as libc::c_ushort;
        }
        i += 1;
        i;
    }
    *work.offset(n.wrapping_sub(1 as i32 as u64) as isize) = (*sorted_data
        .offset(n.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize) as i32
        - *sorted_data.offset(((np1_2 - 1 as i32) as u64).wrapping_mul(stride) as isize)
            as i32) as libc::c_ushort;
    gsl_sort_ushort(work, 1 as i32 as size_t, n);
    return *work.offset((np1_2 - 1 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_short,
) -> libc::c_short {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: i32 = 0;
    let mut diff: i32 = 0;
    let mut half: i32 = 0;
    let mut Amin: i32 = 0;
    let mut Amax: i32 = 0;
    let mut even: i32 = 0;
    let mut length: i32 = 0;
    let mut leftA: i32 = 0;
    let mut leftB: i32 = 0;
    let mut nA: i32 = 0;
    let mut nB: i32 = 0;
    let mut tryA: i32 = 0;
    let mut tryB: i32 = 0;
    let mut rightA: i32 = 0;
    let mut rightB: i32 = 0;
    let mut np1_2: i32 = n.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
        as i32;
    *work.offset(0 as i32 as isize) = (*sorted_data
        .offset(n.wrapping_div(2 as i32 as u64).wrapping_mul(stride) as isize) as i32
        - *sorted_data.offset(0 as i32 as isize) as i32) as libc::c_short;
    i = 2 as i32;
    while i <= np1_2 {
        nA = i - 1 as i32;
        nB = n.wrapping_sub(i as u64) as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                as isize,
                        ) as i32) as libc::c_double;
                medB = (*sorted_data
                    .offset(((tryB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                        as i32) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                as i32
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_short;
        } else {
            medA = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                            as isize,
                    ) as i32) as libc::c_double;
            medB = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                as i32
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as libc::c_short;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as i32;
    while i <= n as i32 - 1 as i32 {
        nA = n.wrapping_sub(i as u64) as i32;
        nB = i - 1 as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i + tryA - Amin) as u64).wrapping_mul(stride) as isize)
                    as i32
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                        as i32) as libc::c_double;
                medB = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ) as i32) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    ) as i32) as libc::c_short;
        } else {
            medA = (*sorted_data
                .offset(((i + leftA - Amin) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_double;
            medB = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    ) as i32) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as libc::c_short;
        }
        i += 1;
        i;
    }
    *work.offset(n.wrapping_sub(1 as i32 as u64) as isize) = (*sorted_data
        .offset(n.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize) as i32
        - *sorted_data.offset(((np1_2 - 1 as i32) as u64).wrapping_mul(stride) as isize)
            as i32) as libc::c_short;
    gsl_sort_short(work, 1 as i32 as size_t, n);
    return *work.offset((np1_2 - 1 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_Sn0_from_sorted_data(
    mut sorted_data: *const i8,
    stride: size_t,
    n: size_t,
    mut work: *mut i8,
) -> i8 {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: i32 = 0;
    let mut diff: i32 = 0;
    let mut half: i32 = 0;
    let mut Amin: i32 = 0;
    let mut Amax: i32 = 0;
    let mut even: i32 = 0;
    let mut length: i32 = 0;
    let mut leftA: i32 = 0;
    let mut leftB: i32 = 0;
    let mut nA: i32 = 0;
    let mut nB: i32 = 0;
    let mut tryA: i32 = 0;
    let mut tryB: i32 = 0;
    let mut rightA: i32 = 0;
    let mut rightB: i32 = 0;
    let mut np1_2: i32 = n.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
        as i32;
    *work.offset(0 as i32 as isize) = (*sorted_data
        .offset(n.wrapping_div(2 as i32 as u64).wrapping_mul(stride) as isize) as i32
        - *sorted_data.offset(0 as i32 as isize) as i32) as i8;
    i = 2 as i32;
    while i <= np1_2 {
        nA = i - 1 as i32;
        nB = n.wrapping_sub(i as u64) as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                as isize,
                        ) as i32) as libc::c_double;
                medB = (*sorted_data
                    .offset(((tryB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                        as i32) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                as i32
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as i8;
        } else {
            medA = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                            as isize,
                    ) as i32) as libc::c_double;
            medB = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                as i32
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as i8;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as i32;
    while i <= n as i32 - 1 as i32 {
        nA = n.wrapping_sub(i as u64) as i32;
        nB = i - 1 as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i + tryA - Amin) as u64).wrapping_mul(stride) as isize)
                    as i32
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                        as i32) as libc::c_double;
                medB = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ) as i32) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    ) as i32) as i8;
        } else {
            medA = (*sorted_data
                .offset(((i + leftA - Amin) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_double;
            medB = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    ) as i32) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as i8;
        }
        i += 1;
        i;
    }
    *work.offset(n.wrapping_sub(1 as i32 as u64) as isize) = (*sorted_data
        .offset(n.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize) as i32
        - *sorted_data.offset(((np1_2 - 1 as i32) as u64).wrapping_mul(stride) as isize)
            as i32) as i8;
    gsl_sort_char(work, 1 as i32 as size_t, n);
    return *work.offset((np1_2 - 1 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: i32 = 0;
    let mut diff: i32 = 0;
    let mut half: i32 = 0;
    let mut Amin: i32 = 0;
    let mut Amax: i32 = 0;
    let mut even: i32 = 0;
    let mut length: i32 = 0;
    let mut leftA: i32 = 0;
    let mut leftB: i32 = 0;
    let mut nA: i32 = 0;
    let mut nB: i32 = 0;
    let mut tryA: i32 = 0;
    let mut tryB: i32 = 0;
    let mut rightA: i32 = 0;
    let mut rightB: i32 = 0;
    let mut np1_2: i32 = n.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
        as i32;
    *work.offset(0 as i32 as isize) = *sorted_data
        .offset(n.wrapping_div(2 as i32 as u64).wrapping_mul(stride) as isize)
        - *sorted_data.offset(0 as i32 as isize);
    i = 2 as i32;
    while i <= np1_2 {
        nA = i - 1 as i32;
        nB = n.wrapping_sub(i as u64) as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                as isize,
                        );
                medB = *sorted_data
                    .offset(((tryB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize);
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = *sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize);
        } else {
            medA = *sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                            as isize,
                    );
            medB = *sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize);
            *work.offset((i - 1 as i32) as isize) = if medA < medB {
                medA
            } else {
                medB
            };
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as i32;
    while i <= n as i32 - 1 as i32 {
        nA = n.wrapping_sub(i as u64) as i32;
        nB = i - 1 as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = *sorted_data
                    .offset(((i + tryA - Amin) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize);
                medB = *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        );
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = *sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    );
        } else {
            medA = *sorted_data
                .offset(((i + leftA - Amin) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize);
            medB = *sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    );
            *work.offset((i - 1 as i32) as isize) = if medA < medB {
                medA
            } else {
                medB
            };
        }
        i += 1;
        i;
    }
    *work.offset(n.wrapping_sub(1 as i32 as u64) as isize) = *sorted_data
        .offset(n.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
        - *sorted_data.offset(((np1_2 - 1 as i32) as u64).wrapping_mul(stride) as isize);
    gsl_sort(work, 1 as i32 as size_t, n);
    return *work.offset((np1_2 - 1 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_Sn0_from_sorted_data(
    mut sorted_data: *const i64,
    stride: size_t,
    n: size_t,
    mut work: *mut i64,
) -> i64 {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: i32 = 0;
    let mut diff: i32 = 0;
    let mut half: i32 = 0;
    let mut Amin: i32 = 0;
    let mut Amax: i32 = 0;
    let mut even: i32 = 0;
    let mut length: i32 = 0;
    let mut leftA: i32 = 0;
    let mut leftB: i32 = 0;
    let mut nA: i32 = 0;
    let mut nB: i32 = 0;
    let mut tryA: i32 = 0;
    let mut tryB: i32 = 0;
    let mut rightA: i32 = 0;
    let mut rightB: i32 = 0;
    let mut np1_2: i32 = n.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
        as i32;
    *work.offset(0 as i32 as isize) = *sorted_data
        .offset(n.wrapping_div(2 as i32 as u64).wrapping_mul(stride) as isize)
        - *sorted_data.offset(0 as i32 as isize);
    i = 2 as i32;
    while i <= np1_2 {
        nA = i - 1 as i32;
        nB = n.wrapping_sub(i as u64) as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                as isize,
                        )) as libc::c_double;
                medB = (*sorted_data
                    .offset(((tryB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                    as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = *sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize);
        } else {
            medA = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                            as isize,
                    )) as libc::c_double;
            medB = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as i64;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as i32;
    while i <= n as i32 - 1 as i32 {
        nA = n.wrapping_sub(i as u64) as i32;
        nB = i - 1 as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i + tryA - Amin) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                    as libc::c_double;
                medB = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = *sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    );
        } else {
            medA = (*sorted_data
                .offset(((i + leftA - Amin) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                as libc::c_double;
            medB = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    )) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as i64;
        }
        i += 1;
        i;
    }
    *work.offset(n.wrapping_sub(1 as i32 as u64) as isize) = *sorted_data
        .offset(n.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
        - *sorted_data.offset(((np1_2 - 1 as i32) as u64).wrapping_mul(stride) as isize);
    gsl_sort_long(work, 1 as i32 as size_t, n);
    return *work.offset((np1_2 - 1 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_float,
) -> libc::c_float {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: i32 = 0;
    let mut diff: i32 = 0;
    let mut half: i32 = 0;
    let mut Amin: i32 = 0;
    let mut Amax: i32 = 0;
    let mut even: i32 = 0;
    let mut length: i32 = 0;
    let mut leftA: i32 = 0;
    let mut leftB: i32 = 0;
    let mut nA: i32 = 0;
    let mut nB: i32 = 0;
    let mut tryA: i32 = 0;
    let mut tryB: i32 = 0;
    let mut rightA: i32 = 0;
    let mut rightB: i32 = 0;
    let mut np1_2: i32 = n.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
        as i32;
    *work.offset(0 as i32 as isize) = *sorted_data
        .offset(n.wrapping_div(2 as i32 as u64).wrapping_mul(stride) as isize)
        - *sorted_data.offset(0 as i32 as isize);
    i = 2 as i32;
    while i <= np1_2 {
        nA = i - 1 as i32;
        nB = n.wrapping_sub(i as u64) as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                as isize,
                        )) as libc::c_double;
                medB = (*sorted_data
                    .offset(((tryB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                    as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = *sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize);
        } else {
            medA = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                            as isize,
                    )) as libc::c_double;
            medB = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as libc::c_float;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as i32;
    while i <= n as i32 - 1 as i32 {
        nA = n.wrapping_sub(i as u64) as i32;
        nB = i - 1 as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i + tryA - Amin) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                    as libc::c_double;
                medB = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = *sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    );
        } else {
            medA = (*sorted_data
                .offset(((i + leftA - Amin) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                as libc::c_double;
            medB = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    )) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as libc::c_float;
        }
        i += 1;
        i;
    }
    *work.offset(n.wrapping_sub(1 as i32 as u64) as isize) = *sorted_data
        .offset(n.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
        - *sorted_data.offset(((np1_2 - 1 as i32) as u64).wrapping_mul(stride) as isize);
    gsl_sort_float(work, 1 as i32 as size_t, n);
    return *work.offset((np1_2 - 1 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_Sn0_from_sorted_data(
    mut sorted_data: *const i32,
    stride: size_t,
    n: size_t,
    mut work: *mut i32,
) -> i32 {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: i32 = 0;
    let mut diff: i32 = 0;
    let mut half: i32 = 0;
    let mut Amin: i32 = 0;
    let mut Amax: i32 = 0;
    let mut even: i32 = 0;
    let mut length: i32 = 0;
    let mut leftA: i32 = 0;
    let mut leftB: i32 = 0;
    let mut nA: i32 = 0;
    let mut nB: i32 = 0;
    let mut tryA: i32 = 0;
    let mut tryB: i32 = 0;
    let mut rightA: i32 = 0;
    let mut rightB: i32 = 0;
    let mut np1_2: i32 = n.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
        as i32;
    *work.offset(0 as i32 as isize) = *sorted_data
        .offset(n.wrapping_div(2 as i32 as u64).wrapping_mul(stride) as isize)
        - *sorted_data.offset(0 as i32 as isize);
    i = 2 as i32;
    while i <= np1_2 {
        nA = i - 1 as i32;
        nB = n.wrapping_sub(i as u64) as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                as isize,
                        )) as libc::c_double;
                medB = (*sorted_data
                    .offset(((tryB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                    as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = *sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize);
        } else {
            medA = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                            as isize,
                    )) as libc::c_double;
            medB = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as i32;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as i32;
    while i <= n as i32 - 1 as i32 {
        nA = n.wrapping_sub(i as u64) as i32;
        nB = i - 1 as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i + tryA - Amin) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                    as libc::c_double;
                medB = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = *sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    );
        } else {
            medA = (*sorted_data
                .offset(((i + leftA - Amin) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                as libc::c_double;
            medB = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    )) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as i32;
        }
        i += 1;
        i;
    }
    *work.offset(n.wrapping_sub(1 as i32 as u64) as isize) = *sorted_data
        .offset(n.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
        - *sorted_data.offset(((np1_2 - 1 as i32) as u64).wrapping_mul(stride) as isize);
    gsl_sort_int(work, 1 as i32 as size_t, n);
    return *work.offset((np1_2 - 1 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_Sn0_from_sorted_data(
    mut sorted_data: *const u64,
    stride: size_t,
    n: size_t,
    mut work: *mut u64,
) -> u64 {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: i32 = 0;
    let mut diff: i32 = 0;
    let mut half: i32 = 0;
    let mut Amin: i32 = 0;
    let mut Amax: i32 = 0;
    let mut even: i32 = 0;
    let mut length: i32 = 0;
    let mut leftA: i32 = 0;
    let mut leftB: i32 = 0;
    let mut nA: i32 = 0;
    let mut nB: i32 = 0;
    let mut tryA: i32 = 0;
    let mut tryB: i32 = 0;
    let mut rightA: i32 = 0;
    let mut rightB: i32 = 0;
    let mut np1_2: i32 = n.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
        as i32;
    *work.offset(0 as i32 as isize) = (*sorted_data
        .offset(n.wrapping_div(2 as i32 as u64).wrapping_mul(stride) as isize))
        .wrapping_sub(*sorted_data.offset(0 as i32 as isize));
    i = 2 as i32;
    while i <= np1_2 {
        nA = i - 1 as i32;
        nB = n.wrapping_sub(i as u64) as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - tryA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                    as isize,
                            ),
                    ) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((tryB + i - 1 as i32) as u64).wrapping_mul(stride) as isize,
                    ))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - 1 as i32) as u64).wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize),
                );
        } else {
            medA = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - leftA + Amin - 2 as i32) as u64).wrapping_mul(stride)
                                as isize,
                        ),
                ) as libc::c_double;
            medB = (*sorted_data
                .offset(((leftB + i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize),
                ) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as u64;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as i32;
    while i <= n as i32 - 1 as i32 {
        nA = n.wrapping_sub(i as u64) as i32;
        nB = i - 1 as i32;
        diff = nB - nA;
        leftB = 1 as i32;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as i32 + 1 as i32;
        Amax = diff / 2 as i32 + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as i32;
            even = 1 as i32 - length % 2 as i32;
            half = (length - 1 as i32) / 2 as i32;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(((i + tryA - Amin) as u64).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - 1 as i32) as u64).wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                medB = (*sorted_data
                    .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - tryB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work.offset((i - 1 as i32) as isize) = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ),
                );
        } else {
            medA = (*sorted_data
                .offset(((i + leftA - Amin) as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize),
                ) as libc::c_double;
            medB = (*sorted_data
                .offset(((i - 1 as i32) as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - leftB - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double;
            *work.offset((i - 1 as i32) as isize) = (if medA < medB {
                medA
            } else {
                medB
            }) as u64;
        }
        i += 1;
        i;
    }
    *work.offset(n.wrapping_sub(1 as i32 as u64) as isize) = (*sorted_data
        .offset(n.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize))
        .wrapping_sub(
            *sorted_data
                .offset(((np1_2 - 1 as i32) as u64).wrapping_mul(stride) as isize),
        );
    gsl_sort_ulong(work, 1 as i32 as size_t, n);
    return *work.offset((np1_2 - 1 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_float,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_float_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as i32 as u64 {
        if n == 2 as i32 as u64 {
            cn = 0.743f64;
        } else if n == 3 as i32 as u64 {
            cn = 1.851f64;
        } else if n == 4 as i32 as u64 {
            cn = 0.954f64;
        } else if n == 5 as i32 as u64 {
            cn = 1.351f64;
        } else if n == 6 as i32 as u64 {
            cn = 0.993f64;
        } else if n == 7 as i32 as u64 {
            cn = 1.198f64;
        } else if n == 8 as i32 as u64 {
            cn = 1.005f64;
        } else if n == 9 as i32 as u64 {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_short,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_short_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as i32 as u64 {
        if n == 2 as i32 as u64 {
            cn = 0.743f64;
        } else if n == 3 as i32 as u64 {
            cn = 1.851f64;
        } else if n == 4 as i32 as u64 {
            cn = 0.954f64;
        } else if n == 5 as i32 as u64 {
            cn = 1.351f64;
        } else if n == 6 as i32 as u64 {
            cn = 0.993f64;
        } else if n == 7 as i32 as u64 {
            cn = 1.198f64;
        } else if n == 8 as i32 as u64 {
            cn = 1.005f64;
        } else if n == 9 as i32 as u64 {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_Sn_from_sorted_data(
    mut sorted_data: *const u32,
    stride: size_t,
    n: size_t,
    mut work: *mut u32,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_uint_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as i32 as u64 {
        if n == 2 as i32 as u64 {
            cn = 0.743f64;
        } else if n == 3 as i32 as u64 {
            cn = 1.851f64;
        } else if n == 4 as i32 as u64 {
            cn = 0.954f64;
        } else if n == 5 as i32 as u64 {
            cn = 1.351f64;
        } else if n == 6 as i32 as u64 {
            cn = 0.993f64;
        } else if n == 7 as i32 as u64 {
            cn = 1.198f64;
        } else if n == 8 as i32 as u64 {
            cn = 1.005f64;
        } else if n == 9 as i32 as u64 {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_Sn_from_sorted_data(
    mut sorted_data: *const i8,
    stride: size_t,
    n: size_t,
    mut work: *mut i8,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_char_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as i32 as u64 {
        if n == 2 as i32 as u64 {
            cn = 0.743f64;
        } else if n == 3 as i32 as u64 {
            cn = 1.851f64;
        } else if n == 4 as i32 as u64 {
            cn = 0.954f64;
        } else if n == 5 as i32 as u64 {
            cn = 1.351f64;
        } else if n == 6 as i32 as u64 {
            cn = 0.993f64;
        } else if n == 7 as i32 as u64 {
            cn = 1.198f64;
        } else if n == 8 as i32 as u64 {
            cn = 1.005f64;
        } else if n == 9 as i32 as u64 {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_ushort,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_ushort_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as i32 as u64 {
        if n == 2 as i32 as u64 {
            cn = 0.743f64;
        } else if n == 3 as i32 as u64 {
            cn = 1.851f64;
        } else if n == 4 as i32 as u64 {
            cn = 0.954f64;
        } else if n == 5 as i32 as u64 {
            cn = 1.351f64;
        } else if n == 6 as i32 as u64 {
            cn = 0.993f64;
        } else if n == 7 as i32 as u64 {
            cn = 1.198f64;
        } else if n == 8 as i32 as u64 {
            cn = 1.005f64;
        } else if n == 9 as i32 as u64 {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    );
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as i32 as u64 {
        if n == 2 as i32 as u64 {
            cn = 0.743f64;
        } else if n == 3 as i32 as u64 {
            cn = 1.851f64;
        } else if n == 4 as i32 as u64 {
            cn = 0.954f64;
        } else if n == 5 as i32 as u64 {
            cn = 1.351f64;
        } else if n == 6 as i32 as u64 {
            cn = 0.993f64;
        } else if n == 7 as i32 as u64 {
            cn = 1.198f64;
        } else if n == 8 as i32 as u64 {
            cn = 1.005f64;
        } else if n == 9 as i32 as u64 {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_Sn_from_sorted_data(
    mut sorted_data: *const u8,
    stride: size_t,
    n: size_t,
    mut work: *mut u8,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_uchar_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as i32 as u64 {
        if n == 2 as i32 as u64 {
            cn = 0.743f64;
        } else if n == 3 as i32 as u64 {
            cn = 1.851f64;
        } else if n == 4 as i32 as u64 {
            cn = 0.954f64;
        } else if n == 5 as i32 as u64 {
            cn = 1.351f64;
        } else if n == 6 as i32 as u64 {
            cn = 0.993f64;
        } else if n == 7 as i32 as u64 {
            cn = 1.198f64;
        } else if n == 8 as i32 as u64 {
            cn = 1.005f64;
        } else if n == 9 as i32 as u64 {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_Sn_from_sorted_data(
    mut sorted_data: *const u64,
    stride: size_t,
    n: size_t,
    mut work: *mut u64,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_ulong_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as i32 as u64 {
        if n == 2 as i32 as u64 {
            cn = 0.743f64;
        } else if n == 3 as i32 as u64 {
            cn = 1.851f64;
        } else if n == 4 as i32 as u64 {
            cn = 0.954f64;
        } else if n == 5 as i32 as u64 {
            cn = 1.351f64;
        } else if n == 6 as i32 as u64 {
            cn = 0.993f64;
        } else if n == 7 as i32 as u64 {
            cn = 1.198f64;
        } else if n == 8 as i32 as u64 {
            cn = 1.005f64;
        } else if n == 9 as i32 as u64 {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_Sn_from_sorted_data(
    mut sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mut work: *mut f128::f128,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = (gsl_stats_long_double_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ))
        .to_f64()
        .unwrap();
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as i32 as u64 {
        if n == 2 as i32 as u64 {
            cn = 0.743f64;
        } else if n == 3 as i32 as u64 {
            cn = 1.851f64;
        } else if n == 4 as i32 as u64 {
            cn = 0.954f64;
        } else if n == 5 as i32 as u64 {
            cn = 1.351f64;
        } else if n == 6 as i32 as u64 {
            cn = 0.993f64;
        } else if n == 7 as i32 as u64 {
            cn = 1.198f64;
        } else if n == 8 as i32 as u64 {
            cn = 1.005f64;
        } else if n == 9 as i32 as u64 {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_Sn_from_sorted_data(
    mut sorted_data: *const i32,
    stride: size_t,
    n: size_t,
    mut work: *mut i32,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_int_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as i32 as u64 {
        if n == 2 as i32 as u64 {
            cn = 0.743f64;
        } else if n == 3 as i32 as u64 {
            cn = 1.851f64;
        } else if n == 4 as i32 as u64 {
            cn = 0.954f64;
        } else if n == 5 as i32 as u64 {
            cn = 1.351f64;
        } else if n == 6 as i32 as u64 {
            cn = 0.993f64;
        } else if n == 7 as i32 as u64 {
            cn = 1.198f64;
        } else if n == 8 as i32 as u64 {
            cn = 1.005f64;
        } else if n == 9 as i32 as u64 {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_Sn_from_sorted_data(
    mut sorted_data: *const i64,
    stride: size_t,
    n: size_t,
    mut work: *mut i64,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_long_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as i32 as u64 {
        if n == 2 as i32 as u64 {
            cn = 0.743f64;
        } else if n == 3 as i32 as u64 {
            cn = 1.851f64;
        } else if n == 4 as i32 as u64 {
            cn = 0.954f64;
        } else if n == 5 as i32 as u64 {
            cn = 1.351f64;
        } else if n == 6 as i32 as u64 {
            cn = 0.993f64;
        } else if n == 7 as i32 as u64 {
            cn = 1.198f64;
        } else if n == 8 as i32 as u64 {
            cn = 1.005f64;
        } else if n == 9 as i32 as u64 {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}