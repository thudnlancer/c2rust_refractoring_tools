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
    fn gsl_sort_uchar(data: *mut u8, stride: size_t, n: size_t);
    fn gsl_sort_char(data: *mut i8, stride: size_t, n: size_t);
    fn gsl_sort_float(data: *mut libc::c_float, stride: size_t, n: size_t);
    fn gsl_sort_ulong(data: *mut u64, stride: size_t, n: size_t);
    fn gsl_sort_uint(data: *mut u32, stride: size_t, n: size_t);
    fn gsl_sort_long(data: *mut i64, stride: size_t, n: size_t);
    fn gsl_sort_int(data: *mut i32, stride: size_t, n: size_t);
    fn gsl_sort_ushort(data: *mut libc::c_ushort, stride: size_t, n: size_t);
    fn gsl_sort_short(data: *mut libc::c_short, stride: size_t, n: size_t);
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_Qn0_from_sorted_data(
    mut sorted_data: *const i64,
    stride: size_t,
    n: size_t,
    mut work: *mut i64,
    mut work_int: *mut i32,
) -> i64 {
    let ni: i32 = n as i32;
    let mut a_srt: *mut i64 = &mut *work.offset(n as isize) as *mut i64;
    let mut a_cand: *mut i64 = &mut *work
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i64;
    let mut left: *mut i32 = &mut *work_int.offset(0 as i32 as isize) as *mut i32;
    let mut right: *mut i32 = &mut *work_int.offset(n as isize) as *mut i32;
    let mut p: *mut i32 = &mut *work_int
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut q: *mut i32 = &mut *work_int
        .offset((3 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut weight: *mut i32 = &mut *work_int
        .offset((4 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut trial: i64 = 0.0f64 as i64;
    let mut found: i32 = 0 as i32;
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jh: i32 = 0;
    let mut k: i64 = 0;
    let mut knew: i64 = 0;
    let mut nl: i64 = 0;
    let mut nr: i64 = 0;
    let mut sump: i64 = 0;
    let mut sumq: i64 = 0;
    if n < 2 as i32 as u64 {
        return 0.0f64 as i64;
    }
    h = n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64) as i32;
    k = h as i64 * (h - 1 as i32) as i64 / 2 as i32 as i64;
    i = 0 as i32;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as i32;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as i64 as u64)
        .wrapping_mul(n.wrapping_add(1 as i32 as u64))
        .wrapping_div(2 as i32 as u64) as i64;
    nr = (n as i64 as u64).wrapping_mul(n) as i64;
    knew = k + nl;
    while found == 0 && nr - nl > ni as i64 {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight.offset(j as isize) = *right.offset(i as isize)
                    - *left.offset(i as isize) + 1 as i32;
                jh = *left.offset(i as isize) + *weight.offset(j as isize) / 2 as i32;
                *work.offset(j as isize) = *sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((ni - jh) as u64).wrapping_mul(stride) as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_long_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as i32;
        i = ni - 1 as i32;
        while i >= 0 as i32 {
            while j < ni
                && ((*sorted_data.offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        )) as libc::c_double) < trial as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as i32;
        i = 0 as i32;
        while i < ni {
            while (*sorted_data.offset((i as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((ni - j + 1 as i32) as u64).wrapping_mul(stride) as isize))
                as libc::c_double > trial as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as i32 as i64;
        sumq = 0 as i32 as i64;
        i = 0 as i32;
        while i < ni {
            sump += *p.offset(i as isize) as i64;
            sumq += (*q.offset(i as isize) - 1 as i32) as i64;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as i32;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as i32;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as i32;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            let mut jj: i32 = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work.offset(j as isize) = *sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((ni - jj) as u64).wrapping_mul(stride) as isize);
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as i32 as i64;
        gsl_sort_long(work, 1 as i32 as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_ushort,
    mut work_int: *mut i32,
) -> libc::c_ushort {
    let ni: i32 = n as i32;
    let mut a_srt: *mut libc::c_ushort = &mut *work.offset(n as isize)
        as *mut libc::c_ushort;
    let mut a_cand: *mut libc::c_ushort = &mut *work
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut libc::c_ushort;
    let mut left: *mut i32 = &mut *work_int.offset(0 as i32 as isize) as *mut i32;
    let mut right: *mut i32 = &mut *work_int.offset(n as isize) as *mut i32;
    let mut p: *mut i32 = &mut *work_int
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut q: *mut i32 = &mut *work_int
        .offset((3 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut weight: *mut i32 = &mut *work_int
        .offset((4 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut trial: libc::c_ushort = 0.0f64 as libc::c_ushort;
    let mut found: i32 = 0 as i32;
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jh: i32 = 0;
    let mut k: i64 = 0;
    let mut knew: i64 = 0;
    let mut nl: i64 = 0;
    let mut nr: i64 = 0;
    let mut sump: i64 = 0;
    let mut sumq: i64 = 0;
    if n < 2 as i32 as u64 {
        return 0.0f64 as libc::c_ushort;
    }
    h = n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64) as i32;
    k = h as i64 * (h - 1 as i32) as i64 / 2 as i32 as i64;
    i = 0 as i32;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as i32;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as i64 as u64)
        .wrapping_mul(n.wrapping_add(1 as i32 as u64))
        .wrapping_div(2 as i32 as u64) as i64;
    nr = (n as i64 as u64).wrapping_mul(n) as i64;
    knew = k + nl;
    while found == 0 && nr - nl > ni as i64 {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight.offset(j as isize) = *right.offset(i as isize)
                    - *left.offset(i as isize) + 1 as i32;
                jh = *left.offset(i as isize) + *weight.offset(j as isize) / 2 as i32;
                *work.offset(j as isize) = (*sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(((ni - jh) as u64).wrapping_mul(stride) as isize) as i32)
                    as libc::c_ushort;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_ushort_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as i32;
        i = ni - 1 as i32;
        while i >= 0 as i32 {
            while j < ni
                && ((*sorted_data.offset((i as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ) as i32) as libc::c_double) < trial as i32 as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as i32;
        i = 0 as i32;
        while i < ni {
            while (*sorted_data.offset((i as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(((ni - j + 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_double > trial as i32 as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as i32 as i64;
        sumq = 0 as i32 as i64;
        i = 0 as i32;
        while i < ni {
            sump += *p.offset(i as isize) as i64;
            sumq += (*q.offset(i as isize) - 1 as i32) as i64;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as i32;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as i32;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as i32;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            let mut jj: i32 = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work.offset(j as isize) = (*sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(((ni - jj) as u64).wrapping_mul(stride) as isize) as i32)
                    as libc::c_ushort;
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as i32 as i64;
        gsl_sort_ushort(work, 1 as i32 as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_short,
    mut work_int: *mut i32,
) -> libc::c_short {
    let ni: i32 = n as i32;
    let mut a_srt: *mut libc::c_short = &mut *work.offset(n as isize)
        as *mut libc::c_short;
    let mut a_cand: *mut libc::c_short = &mut *work
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut libc::c_short;
    let mut left: *mut i32 = &mut *work_int.offset(0 as i32 as isize) as *mut i32;
    let mut right: *mut i32 = &mut *work_int.offset(n as isize) as *mut i32;
    let mut p: *mut i32 = &mut *work_int
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut q: *mut i32 = &mut *work_int
        .offset((3 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut weight: *mut i32 = &mut *work_int
        .offset((4 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut trial: libc::c_short = 0.0f64 as libc::c_short;
    let mut found: i32 = 0 as i32;
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jh: i32 = 0;
    let mut k: i64 = 0;
    let mut knew: i64 = 0;
    let mut nl: i64 = 0;
    let mut nr: i64 = 0;
    let mut sump: i64 = 0;
    let mut sumq: i64 = 0;
    if n < 2 as i32 as u64 {
        return 0.0f64 as libc::c_short;
    }
    h = n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64) as i32;
    k = h as i64 * (h - 1 as i32) as i64 / 2 as i32 as i64;
    i = 0 as i32;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as i32;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as i64 as u64)
        .wrapping_mul(n.wrapping_add(1 as i32 as u64))
        .wrapping_div(2 as i32 as u64) as i64;
    nr = (n as i64 as u64).wrapping_mul(n) as i64;
    knew = k + nl;
    while found == 0 && nr - nl > ni as i64 {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight.offset(j as isize) = *right.offset(i as isize)
                    - *left.offset(i as isize) + 1 as i32;
                jh = *left.offset(i as isize) + *weight.offset(j as isize) / 2 as i32;
                *work.offset(j as isize) = (*sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(((ni - jh) as u64).wrapping_mul(stride) as isize) as i32)
                    as libc::c_short;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_short_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as i32;
        i = ni - 1 as i32;
        while i >= 0 as i32 {
            while j < ni
                && ((*sorted_data.offset((i as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ) as i32) as libc::c_double) < trial as i32 as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as i32;
        i = 0 as i32;
        while i < ni {
            while (*sorted_data.offset((i as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(((ni - j + 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_double > trial as i32 as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as i32 as i64;
        sumq = 0 as i32 as i64;
        i = 0 as i32;
        while i < ni {
            sump += *p.offset(i as isize) as i64;
            sumq += (*q.offset(i as isize) - 1 as i32) as i64;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as i32;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as i32;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as i32;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            let mut jj: i32 = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work.offset(j as isize) = (*sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(((ni - jj) as u64).wrapping_mul(stride) as isize) as i32)
                    as libc::c_short;
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as i32 as i64;
        gsl_sort_short(work, 1 as i32 as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_Qn0_from_sorted_data(
    mut sorted_data: *const u8,
    stride: size_t,
    n: size_t,
    mut work: *mut u8,
    mut work_int: *mut i32,
) -> u8 {
    let ni: i32 = n as i32;
    let mut a_srt: *mut u8 = &mut *work.offset(n as isize) as *mut u8;
    let mut a_cand: *mut u8 = &mut *work
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut u8;
    let mut left: *mut i32 = &mut *work_int.offset(0 as i32 as isize) as *mut i32;
    let mut right: *mut i32 = &mut *work_int.offset(n as isize) as *mut i32;
    let mut p: *mut i32 = &mut *work_int
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut q: *mut i32 = &mut *work_int
        .offset((3 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut weight: *mut i32 = &mut *work_int
        .offset((4 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut trial: u8 = 0.0f64 as u8;
    let mut found: i32 = 0 as i32;
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jh: i32 = 0;
    let mut k: i64 = 0;
    let mut knew: i64 = 0;
    let mut nl: i64 = 0;
    let mut nr: i64 = 0;
    let mut sump: i64 = 0;
    let mut sumq: i64 = 0;
    if n < 2 as i32 as u64 {
        return 0.0f64 as u8;
    }
    h = n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64) as i32;
    k = h as i64 * (h - 1 as i32) as i64 / 2 as i32 as i64;
    i = 0 as i32;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as i32;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as i64 as u64)
        .wrapping_mul(n.wrapping_add(1 as i32 as u64))
        .wrapping_div(2 as i32 as u64) as i64;
    nr = (n as i64 as u64).wrapping_mul(n) as i64;
    knew = k + nl;
    while found == 0 && nr - nl > ni as i64 {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight.offset(j as isize) = *right.offset(i as isize)
                    - *left.offset(i as isize) + 1 as i32;
                jh = *left.offset(i as isize) + *weight.offset(j as isize) / 2 as i32;
                *work.offset(j as isize) = (*sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(((ni - jh) as u64).wrapping_mul(stride) as isize) as i32)
                    as u8;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_uchar_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as i32;
        i = ni - 1 as i32;
        while i >= 0 as i32 {
            while j < ni
                && ((*sorted_data.offset((i as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ) as i32) as libc::c_double) < trial as i32 as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as i32;
        i = 0 as i32;
        while i < ni {
            while (*sorted_data.offset((i as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(((ni - j + 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_double > trial as i32 as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as i32 as i64;
        sumq = 0 as i32 as i64;
        i = 0 as i32;
        while i < ni {
            sump += *p.offset(i as isize) as i64;
            sumq += (*q.offset(i as isize) - 1 as i32) as i64;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as i32;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as i32;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as i32;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            let mut jj: i32 = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work.offset(j as isize) = (*sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(((ni - jj) as u64).wrapping_mul(stride) as isize) as i32)
                    as u8;
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as i32 as i64;
        gsl_sort_uchar(work, 1 as i32 as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
    mut work_int: *mut i32,
) -> libc::c_double {
    let ni: i32 = n as i32;
    let mut a_srt: *mut libc::c_double = &mut *work.offset(n as isize)
        as *mut libc::c_double;
    let mut a_cand: *mut libc::c_double = &mut *work
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut libc::c_double;
    let mut left: *mut i32 = &mut *work_int.offset(0 as i32 as isize) as *mut i32;
    let mut right: *mut i32 = &mut *work_int.offset(n as isize) as *mut i32;
    let mut p: *mut i32 = &mut *work_int
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut q: *mut i32 = &mut *work_int
        .offset((3 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut weight: *mut i32 = &mut *work_int
        .offset((4 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut trial: libc::c_double = 0.0f64;
    let mut found: i32 = 0 as i32;
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jh: i32 = 0;
    let mut k: i64 = 0;
    let mut knew: i64 = 0;
    let mut nl: i64 = 0;
    let mut nr: i64 = 0;
    let mut sump: i64 = 0;
    let mut sumq: i64 = 0;
    if n < 2 as i32 as u64 {
        return 0.0f64;
    }
    h = n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64) as i32;
    k = h as i64 * (h - 1 as i32) as i64 / 2 as i32 as i64;
    i = 0 as i32;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as i32;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as i64 as u64)
        .wrapping_mul(n.wrapping_add(1 as i32 as u64))
        .wrapping_div(2 as i32 as u64) as i64;
    nr = (n as i64 as u64).wrapping_mul(n) as i64;
    knew = k + nl;
    while found == 0 && nr - nl > ni as i64 {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight.offset(j as isize) = *right.offset(i as isize)
                    - *left.offset(i as isize) + 1 as i32;
                jh = *left.offset(i as isize) + *weight.offset(j as isize) / 2 as i32;
                *work.offset(j as isize) = *sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((ni - jh) as u64).wrapping_mul(stride) as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as i32;
        i = ni - 1 as i32;
        while i >= 0 as i32 {
            while j < ni
                && *sorted_data.offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ) < trial
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as i32;
        i = 0 as i32;
        while i < ni {
            while *sorted_data.offset((i as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((ni - j + 1 as i32) as u64).wrapping_mul(stride) as isize)
                > trial
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as i32 as i64;
        sumq = 0 as i32 as i64;
        i = 0 as i32;
        while i < ni {
            sump += *p.offset(i as isize) as i64;
            sumq += (*q.offset(i as isize) - 1 as i32) as i64;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as i32;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as i32;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as i32;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            let mut jj: i32 = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work.offset(j as isize) = *sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((ni - jj) as u64).wrapping_mul(stride) as isize);
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as i32 as i64;
        gsl_sort(work, 1 as i32 as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_Qn0_from_sorted_data(
    mut sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mut work: *mut f128::f128,
    mut work_int: *mut i32,
) -> f128::f128 {
    let ni: i32 = n as i32;
    let mut a_srt: *mut f128::f128 = &mut *work.offset(n as isize) as *mut f128::f128;
    let mut a_cand: *mut f128::f128 = &mut *work
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut f128::f128;
    let mut left: *mut i32 = &mut *work_int.offset(0 as i32 as isize) as *mut i32;
    let mut right: *mut i32 = &mut *work_int.offset(n as isize) as *mut i32;
    let mut p: *mut i32 = &mut *work_int
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut q: *mut i32 = &mut *work_int
        .offset((3 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut weight: *mut i32 = &mut *work_int
        .offset((4 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut trial: f128::f128 = f128::f128::new(0.0f64);
    let mut found: i32 = 0 as i32;
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jh: i32 = 0;
    let mut k: i64 = 0;
    let mut knew: i64 = 0;
    let mut nl: i64 = 0;
    let mut nr: i64 = 0;
    let mut sump: i64 = 0;
    let mut sumq: i64 = 0;
    if n < 2 as i32 as u64 {
        return f128::f128::new(0.0f64);
    }
    h = n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64) as i32;
    k = h as i64 * (h - 1 as i32) as i64 / 2 as i32 as i64;
    i = 0 as i32;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as i32;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as i64 as u64)
        .wrapping_mul(n.wrapping_add(1 as i32 as u64))
        .wrapping_div(2 as i32 as u64) as i64;
    nr = (n as i64 as u64).wrapping_mul(n) as i64;
    knew = k + nl;
    while found == 0 && nr - nl > ni as i64 {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight.offset(j as isize) = *right.offset(i as isize)
                    - *left.offset(i as isize) + 1 as i32;
                jh = *left.offset(i as isize) + *weight.offset(j as isize) / 2 as i32;
                *work.offset(j as isize) = *sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((ni - jh) as u64).wrapping_mul(stride) as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_long_double_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as i32;
        i = ni - 1 as i32;
        while i >= 0 as i32 {
            while j < ni
                && f128::f128::new(
                    (*sorted_data.offset((i as u64).wrapping_mul(stride) as isize)
                        - *sorted_data
                            .offset(
                                ((ni - j - 1 as i32) as u64).wrapping_mul(stride) as isize,
                            ))
                        .to_f64()
                        .unwrap(),
                ) < trial
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as i32;
        i = 0 as i32;
        while i < ni {
            while f128::f128::new(
                (*sorted_data.offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - j + 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ))
                    .to_f64()
                    .unwrap(),
            ) > trial
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as i32 as i64;
        sumq = 0 as i32 as i64;
        i = 0 as i32;
        while i < ni {
            sump += *p.offset(i as isize) as i64;
            sumq += (*q.offset(i as isize) - 1 as i32) as i64;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as i32;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as i32;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as i32;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            let mut jj: i32 = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work.offset(j as isize) = *sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((ni - jj) as u64).wrapping_mul(stride) as isize);
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as i32 as i64;
        gsl_sort_long_double(work, 1 as i32 as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_float,
    mut work_int: *mut i32,
) -> libc::c_float {
    let ni: i32 = n as i32;
    let mut a_srt: *mut libc::c_float = &mut *work.offset(n as isize)
        as *mut libc::c_float;
    let mut a_cand: *mut libc::c_float = &mut *work
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut libc::c_float;
    let mut left: *mut i32 = &mut *work_int.offset(0 as i32 as isize) as *mut i32;
    let mut right: *mut i32 = &mut *work_int.offset(n as isize) as *mut i32;
    let mut p: *mut i32 = &mut *work_int
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut q: *mut i32 = &mut *work_int
        .offset((3 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut weight: *mut i32 = &mut *work_int
        .offset((4 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut trial: libc::c_float = 0.0f64 as libc::c_float;
    let mut found: i32 = 0 as i32;
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jh: i32 = 0;
    let mut k: i64 = 0;
    let mut knew: i64 = 0;
    let mut nl: i64 = 0;
    let mut nr: i64 = 0;
    let mut sump: i64 = 0;
    let mut sumq: i64 = 0;
    if n < 2 as i32 as u64 {
        return 0.0f64 as libc::c_float;
    }
    h = n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64) as i32;
    k = h as i64 * (h - 1 as i32) as i64 / 2 as i32 as i64;
    i = 0 as i32;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as i32;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as i64 as u64)
        .wrapping_mul(n.wrapping_add(1 as i32 as u64))
        .wrapping_div(2 as i32 as u64) as i64;
    nr = (n as i64 as u64).wrapping_mul(n) as i64;
    knew = k + nl;
    while found == 0 && nr - nl > ni as i64 {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight.offset(j as isize) = *right.offset(i as isize)
                    - *left.offset(i as isize) + 1 as i32;
                jh = *left.offset(i as isize) + *weight.offset(j as isize) / 2 as i32;
                *work.offset(j as isize) = *sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((ni - jh) as u64).wrapping_mul(stride) as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_float_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as i32;
        i = ni - 1 as i32;
        while i >= 0 as i32 {
            while j < ni
                && ((*sorted_data.offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        )) as libc::c_double) < trial as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as i32;
        i = 0 as i32;
        while i < ni {
            while (*sorted_data.offset((i as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((ni - j + 1 as i32) as u64).wrapping_mul(stride) as isize))
                as libc::c_double > trial as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as i32 as i64;
        sumq = 0 as i32 as i64;
        i = 0 as i32;
        while i < ni {
            sump += *p.offset(i as isize) as i64;
            sumq += (*q.offset(i as isize) - 1 as i32) as i64;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as i32;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as i32;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as i32;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            let mut jj: i32 = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work.offset(j as isize) = *sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((ni - jj) as u64).wrapping_mul(stride) as isize);
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as i32 as i64;
        gsl_sort_float(work, 1 as i32 as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_Qn0_from_sorted_data(
    mut sorted_data: *const i32,
    stride: size_t,
    n: size_t,
    mut work: *mut i32,
    mut work_int: *mut i32,
) -> i32 {
    let ni: i32 = n as i32;
    let mut a_srt: *mut i32 = &mut *work.offset(n as isize) as *mut i32;
    let mut a_cand: *mut i32 = &mut *work
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut left: *mut i32 = &mut *work_int.offset(0 as i32 as isize) as *mut i32;
    let mut right: *mut i32 = &mut *work_int.offset(n as isize) as *mut i32;
    let mut p: *mut i32 = &mut *work_int
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut q: *mut i32 = &mut *work_int
        .offset((3 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut weight: *mut i32 = &mut *work_int
        .offset((4 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut trial: i32 = 0.0f64 as i32;
    let mut found: i32 = 0 as i32;
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jh: i32 = 0;
    let mut k: i64 = 0;
    let mut knew: i64 = 0;
    let mut nl: i64 = 0;
    let mut nr: i64 = 0;
    let mut sump: i64 = 0;
    let mut sumq: i64 = 0;
    if n < 2 as i32 as u64 {
        return 0.0f64 as i32;
    }
    h = n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64) as i32;
    k = h as i64 * (h - 1 as i32) as i64 / 2 as i32 as i64;
    i = 0 as i32;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as i32;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as i64 as u64)
        .wrapping_mul(n.wrapping_add(1 as i32 as u64))
        .wrapping_div(2 as i32 as u64) as i64;
    nr = (n as i64 as u64).wrapping_mul(n) as i64;
    knew = k + nl;
    while found == 0 && nr - nl > ni as i64 {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight.offset(j as isize) = *right.offset(i as isize)
                    - *left.offset(i as isize) + 1 as i32;
                jh = *left.offset(i as isize) + *weight.offset(j as isize) / 2 as i32;
                *work.offset(j as isize) = *sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((ni - jh) as u64).wrapping_mul(stride) as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_int_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as i32;
        i = ni - 1 as i32;
        while i >= 0 as i32 {
            while j < ni
                && ((*sorted_data.offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        )) as libc::c_double) < trial as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as i32;
        i = 0 as i32;
        while i < ni {
            while (*sorted_data.offset((i as u64).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(((ni - j + 1 as i32) as u64).wrapping_mul(stride) as isize))
                as libc::c_double > trial as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as i32 as i64;
        sumq = 0 as i32 as i64;
        i = 0 as i32;
        while i < ni {
            sump += *p.offset(i as isize) as i64;
            sumq += (*q.offset(i as isize) - 1 as i32) as i64;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as i32;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as i32;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as i32;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            let mut jj: i32 = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work.offset(j as isize) = *sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(((ni - jj) as u64).wrapping_mul(stride) as isize);
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as i32 as i64;
        gsl_sort_int(work, 1 as i32 as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_Qn0_from_sorted_data(
    mut sorted_data: *const i8,
    stride: size_t,
    n: size_t,
    mut work: *mut i8,
    mut work_int: *mut i32,
) -> i8 {
    let ni: i32 = n as i32;
    let mut a_srt: *mut i8 = &mut *work.offset(n as isize) as *mut i8;
    let mut a_cand: *mut i8 = &mut *work
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i8;
    let mut left: *mut i32 = &mut *work_int.offset(0 as i32 as isize) as *mut i32;
    let mut right: *mut i32 = &mut *work_int.offset(n as isize) as *mut i32;
    let mut p: *mut i32 = &mut *work_int
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut q: *mut i32 = &mut *work_int
        .offset((3 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut weight: *mut i32 = &mut *work_int
        .offset((4 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut trial: i8 = 0.0f64 as i8;
    let mut found: i32 = 0 as i32;
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jh: i32 = 0;
    let mut k: i64 = 0;
    let mut knew: i64 = 0;
    let mut nl: i64 = 0;
    let mut nr: i64 = 0;
    let mut sump: i64 = 0;
    let mut sumq: i64 = 0;
    if n < 2 as i32 as u64 {
        return 0.0f64 as i8;
    }
    h = n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64) as i32;
    k = h as i64 * (h - 1 as i32) as i64 / 2 as i32 as i64;
    i = 0 as i32;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as i32;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as i64 as u64)
        .wrapping_mul(n.wrapping_add(1 as i32 as u64))
        .wrapping_div(2 as i32 as u64) as i64;
    nr = (n as i64 as u64).wrapping_mul(n) as i64;
    knew = k + nl;
    while found == 0 && nr - nl > ni as i64 {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight.offset(j as isize) = *right.offset(i as isize)
                    - *left.offset(i as isize) + 1 as i32;
                jh = *left.offset(i as isize) + *weight.offset(j as isize) / 2 as i32;
                *work.offset(j as isize) = (*sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(((ni - jh) as u64).wrapping_mul(stride) as isize) as i32)
                    as i8;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_char_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as i32;
        i = ni - 1 as i32;
        while i >= 0 as i32 {
            while j < ni
                && ((*sorted_data.offset((i as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ) as i32) as libc::c_double) < trial as i32 as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as i32;
        i = 0 as i32;
        while i < ni {
            while (*sorted_data.offset((i as u64).wrapping_mul(stride) as isize) as i32
                - *sorted_data
                    .offset(((ni - j + 1 as i32) as u64).wrapping_mul(stride) as isize)
                    as i32) as libc::c_double > trial as i32 as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as i32 as i64;
        sumq = 0 as i32 as i64;
        i = 0 as i32;
        while i < ni {
            sump += *p.offset(i as isize) as i64;
            sumq += (*q.offset(i as isize) - 1 as i32) as i64;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as i32;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as i32;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as i32;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            let mut jj: i32 = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work.offset(j as isize) = (*sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize) as i32
                    - *sorted_data
                        .offset(((ni - jj) as u64).wrapping_mul(stride) as isize) as i32)
                    as i8;
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as i32 as i64;
        gsl_sort_char(work, 1 as i32 as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_Qn0_from_sorted_data(
    mut sorted_data: *const u32,
    stride: size_t,
    n: size_t,
    mut work: *mut u32,
    mut work_int: *mut i32,
) -> u32 {
    let ni: i32 = n as i32;
    let mut a_srt: *mut u32 = &mut *work.offset(n as isize) as *mut u32;
    let mut a_cand: *mut u32 = &mut *work
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut u32;
    let mut left: *mut i32 = &mut *work_int.offset(0 as i32 as isize) as *mut i32;
    let mut right: *mut i32 = &mut *work_int.offset(n as isize) as *mut i32;
    let mut p: *mut i32 = &mut *work_int
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut q: *mut i32 = &mut *work_int
        .offset((3 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut weight: *mut i32 = &mut *work_int
        .offset((4 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut trial: u32 = 0.0f64 as u32;
    let mut found: i32 = 0 as i32;
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jh: i32 = 0;
    let mut k: i64 = 0;
    let mut knew: i64 = 0;
    let mut nl: i64 = 0;
    let mut nr: i64 = 0;
    let mut sump: i64 = 0;
    let mut sumq: i64 = 0;
    if n < 2 as i32 as u64 {
        return 0.0f64 as u32;
    }
    h = n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64) as i32;
    k = h as i64 * (h - 1 as i32) as i64 / 2 as i32 as i64;
    i = 0 as i32;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as i32;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as i64 as u64)
        .wrapping_mul(n.wrapping_add(1 as i32 as u64))
        .wrapping_div(2 as i32 as u64) as i64;
    nr = (n as i64 as u64).wrapping_mul(n) as i64;
    knew = k + nl;
    while found == 0 && nr - nl > ni as i64 {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight.offset(j as isize) = *right.offset(i as isize)
                    - *left.offset(i as isize) + 1 as i32;
                jh = *left.offset(i as isize) + *weight.offset(j as isize) / 2 as i32;
                *work.offset(j as isize) = (*sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(((ni - jh) as u64).wrapping_mul(stride) as isize),
                    );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_uint_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as i32;
        i = ni - 1 as i32;
        while i >= 0 as i32 {
            while j < ni
                && ((*sorted_data.offset((i as u64).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((ni - j - 1 as i32) as u64).wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double) < trial as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as i32;
        i = 0 as i32;
        while i < ni {
            while (*sorted_data.offset((i as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((ni - j + 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double > trial as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as i32 as i64;
        sumq = 0 as i32 as i64;
        i = 0 as i32;
        while i < ni {
            sump += *p.offset(i as isize) as i64;
            sumq += (*q.offset(i as isize) - 1 as i32) as i64;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as i32;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as i32;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as i32;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            let mut jj: i32 = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work.offset(j as isize) = (*sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(((ni - jj) as u64).wrapping_mul(stride) as isize),
                    );
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as i32 as i64;
        gsl_sort_uint(work, 1 as i32 as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_Qn0_from_sorted_data(
    mut sorted_data: *const u64,
    stride: size_t,
    n: size_t,
    mut work: *mut u64,
    mut work_int: *mut i32,
) -> u64 {
    let ni: i32 = n as i32;
    let mut a_srt: *mut u64 = &mut *work.offset(n as isize) as *mut u64;
    let mut a_cand: *mut u64 = &mut *work
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut u64;
    let mut left: *mut i32 = &mut *work_int.offset(0 as i32 as isize) as *mut i32;
    let mut right: *mut i32 = &mut *work_int.offset(n as isize) as *mut i32;
    let mut p: *mut i32 = &mut *work_int
        .offset((2 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut q: *mut i32 = &mut *work_int
        .offset((3 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut weight: *mut i32 = &mut *work_int
        .offset((4 as i32 as u64).wrapping_mul(n) as isize) as *mut i32;
    let mut trial: u64 = 0.0f64 as u64;
    let mut found: i32 = 0 as i32;
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jh: i32 = 0;
    let mut k: i64 = 0;
    let mut knew: i64 = 0;
    let mut nl: i64 = 0;
    let mut nr: i64 = 0;
    let mut sump: i64 = 0;
    let mut sumq: i64 = 0;
    if n < 2 as i32 as u64 {
        return 0.0f64 as u64;
    }
    h = n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64) as i32;
    k = h as i64 * (h - 1 as i32) as i64 / 2 as i32 as i64;
    i = 0 as i32;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as i32;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as i64 as u64)
        .wrapping_mul(n.wrapping_add(1 as i32 as u64))
        .wrapping_div(2 as i32 as u64) as i64;
    nr = (n as i64 as u64).wrapping_mul(n) as i64;
    knew = k + nl;
    while found == 0 && nr - nl > ni as i64 {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight.offset(j as isize) = *right.offset(i as isize)
                    - *left.offset(i as isize) + 1 as i32;
                jh = *left.offset(i as isize) + *weight.offset(j as isize) / 2 as i32;
                *work.offset(j as isize) = (*sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(((ni - jh) as u64).wrapping_mul(stride) as isize),
                    );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_ulong_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as i32;
        i = ni - 1 as i32;
        while i >= 0 as i32 {
            while j < ni
                && ((*sorted_data.offset((i as u64).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((ni - j - 1 as i32) as u64).wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double) < trial as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as i32;
        i = 0 as i32;
        while i < ni {
            while (*sorted_data.offset((i as u64).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((ni - j + 1 as i32) as u64).wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double > trial as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as i32 as i64;
        sumq = 0 as i32 as i64;
        i = 0 as i32;
        while i < ni {
            sump += *p.offset(i as isize) as i64;
            sumq += (*q.offset(i as isize) - 1 as i32) as i64;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as i32;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as i32;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as i32;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as i32;
        i = 1 as i32;
        while i < ni {
            let mut jj: i32 = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work.offset(j as isize) = (*sorted_data
                    .offset((i as u64).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(((ni - jj) as u64).wrapping_mul(stride) as isize),
                    );
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as i32 as i64;
        gsl_sort_ulong(work, 1 as i32 as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_Qn_from_sorted_data(
    mut sorted_data: *const i32,
    stride: size_t,
    n: size_t,
    mut work: *mut i32,
    mut work_int: *mut i32,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_int_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as i32 as u64 {
        if n == 2 as i32 as u64 {
            dn = 0.399356f64;
        } else if n == 3 as i32 as u64 {
            dn = 0.99365f64;
        } else if n == 4 as i32 as u64 {
            dn = 0.51321f64;
        } else if n == 5 as i32 as u64 {
            dn = 0.84401f64;
        } else if n == 6 as i32 as u64 {
            dn = 0.61220f64;
        } else if n == 7 as i32 as u64 {
            dn = 0.85877f64;
        } else if n == 8 as i32 as u64 {
            dn = 0.66993f64;
        } else if n == 9 as i32 as u64 {
            dn = 0.87344f64;
        } else if n == 10 as i32 as u64 {
            dn = 0.72014f64;
        } else if n == 11 as i32 as u64 {
            dn = 0.88906f64;
        } else if n == 12 as i32 as u64 {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_float,
    mut work_int: *mut i32,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_float_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as i32 as u64 {
        if n == 2 as i32 as u64 {
            dn = 0.399356f64;
        } else if n == 3 as i32 as u64 {
            dn = 0.99365f64;
        } else if n == 4 as i32 as u64 {
            dn = 0.51321f64;
        } else if n == 5 as i32 as u64 {
            dn = 0.84401f64;
        } else if n == 6 as i32 as u64 {
            dn = 0.61220f64;
        } else if n == 7 as i32 as u64 {
            dn = 0.85877f64;
        } else if n == 8 as i32 as u64 {
            dn = 0.66993f64;
        } else if n == 9 as i32 as u64 {
            dn = 0.87344f64;
        } else if n == 10 as i32 as u64 {
            dn = 0.72014f64;
        } else if n == 11 as i32 as u64 {
            dn = 0.88906f64;
        } else if n == 12 as i32 as u64 {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_Qn_from_sorted_data(
    mut sorted_data: *const u32,
    stride: size_t,
    n: size_t,
    mut work: *mut u32,
    mut work_int: *mut i32,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_uint_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as i32 as u64 {
        if n == 2 as i32 as u64 {
            dn = 0.399356f64;
        } else if n == 3 as i32 as u64 {
            dn = 0.99365f64;
        } else if n == 4 as i32 as u64 {
            dn = 0.51321f64;
        } else if n == 5 as i32 as u64 {
            dn = 0.84401f64;
        } else if n == 6 as i32 as u64 {
            dn = 0.61220f64;
        } else if n == 7 as i32 as u64 {
            dn = 0.85877f64;
        } else if n == 8 as i32 as u64 {
            dn = 0.66993f64;
        } else if n == 9 as i32 as u64 {
            dn = 0.87344f64;
        } else if n == 10 as i32 as u64 {
            dn = 0.72014f64;
        } else if n == 11 as i32 as u64 {
            dn = 0.88906f64;
        } else if n == 12 as i32 as u64 {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_Qn_from_sorted_data(
    mut sorted_data: *const i64,
    stride: size_t,
    n: size_t,
    mut work: *mut i64,
    mut work_int: *mut i32,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_long_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as i32 as u64 {
        if n == 2 as i32 as u64 {
            dn = 0.399356f64;
        } else if n == 3 as i32 as u64 {
            dn = 0.99365f64;
        } else if n == 4 as i32 as u64 {
            dn = 0.51321f64;
        } else if n == 5 as i32 as u64 {
            dn = 0.84401f64;
        } else if n == 6 as i32 as u64 {
            dn = 0.61220f64;
        } else if n == 7 as i32 as u64 {
            dn = 0.85877f64;
        } else if n == 8 as i32 as u64 {
            dn = 0.66993f64;
        } else if n == 9 as i32 as u64 {
            dn = 0.87344f64;
        } else if n == 10 as i32 as u64 {
            dn = 0.72014f64;
        } else if n == 11 as i32 as u64 {
            dn = 0.88906f64;
        } else if n == 12 as i32 as u64 {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_Qn_from_sorted_data(
    mut sorted_data: *const u8,
    stride: size_t,
    n: size_t,
    mut work: *mut u8,
    mut work_int: *mut i32,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_uchar_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as i32 as u64 {
        if n == 2 as i32 as u64 {
            dn = 0.399356f64;
        } else if n == 3 as i32 as u64 {
            dn = 0.99365f64;
        } else if n == 4 as i32 as u64 {
            dn = 0.51321f64;
        } else if n == 5 as i32 as u64 {
            dn = 0.84401f64;
        } else if n == 6 as i32 as u64 {
            dn = 0.61220f64;
        } else if n == 7 as i32 as u64 {
            dn = 0.85877f64;
        } else if n == 8 as i32 as u64 {
            dn = 0.66993f64;
        } else if n == 9 as i32 as u64 {
            dn = 0.87344f64;
        } else if n == 10 as i32 as u64 {
            dn = 0.72014f64;
        } else if n == 11 as i32 as u64 {
            dn = 0.88906f64;
        } else if n == 12 as i32 as u64 {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_short,
    mut work_int: *mut i32,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_short_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as i32 as u64 {
        if n == 2 as i32 as u64 {
            dn = 0.399356f64;
        } else if n == 3 as i32 as u64 {
            dn = 0.99365f64;
        } else if n == 4 as i32 as u64 {
            dn = 0.51321f64;
        } else if n == 5 as i32 as u64 {
            dn = 0.84401f64;
        } else if n == 6 as i32 as u64 {
            dn = 0.61220f64;
        } else if n == 7 as i32 as u64 {
            dn = 0.85877f64;
        } else if n == 8 as i32 as u64 {
            dn = 0.66993f64;
        } else if n == 9 as i32 as u64 {
            dn = 0.87344f64;
        } else if n == 10 as i32 as u64 {
            dn = 0.72014f64;
        } else if n == 11 as i32 as u64 {
            dn = 0.88906f64;
        } else if n == 12 as i32 as u64 {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_Qn_from_sorted_data(
    mut sorted_data: *const u64,
    stride: size_t,
    n: size_t,
    mut work: *mut u64,
    mut work_int: *mut i32,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_ulong_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as i32 as u64 {
        if n == 2 as i32 as u64 {
            dn = 0.399356f64;
        } else if n == 3 as i32 as u64 {
            dn = 0.99365f64;
        } else if n == 4 as i32 as u64 {
            dn = 0.51321f64;
        } else if n == 5 as i32 as u64 {
            dn = 0.84401f64;
        } else if n == 6 as i32 as u64 {
            dn = 0.61220f64;
        } else if n == 7 as i32 as u64 {
            dn = 0.85877f64;
        } else if n == 8 as i32 as u64 {
            dn = 0.66993f64;
        } else if n == 9 as i32 as u64 {
            dn = 0.87344f64;
        } else if n == 10 as i32 as u64 {
            dn = 0.72014f64;
        } else if n == 11 as i32 as u64 {
            dn = 0.88906f64;
        } else if n == 12 as i32 as u64 {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_ushort,
    mut work_int: *mut i32,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_ushort_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as i32 as u64 {
        if n == 2 as i32 as u64 {
            dn = 0.399356f64;
        } else if n == 3 as i32 as u64 {
            dn = 0.99365f64;
        } else if n == 4 as i32 as u64 {
            dn = 0.51321f64;
        } else if n == 5 as i32 as u64 {
            dn = 0.84401f64;
        } else if n == 6 as i32 as u64 {
            dn = 0.61220f64;
        } else if n == 7 as i32 as u64 {
            dn = 0.85877f64;
        } else if n == 8 as i32 as u64 {
            dn = 0.66993f64;
        } else if n == 9 as i32 as u64 {
            dn = 0.87344f64;
        } else if n == 10 as i32 as u64 {
            dn = 0.72014f64;
        } else if n == 11 as i32 as u64 {
            dn = 0.88906f64;
        } else if n == 12 as i32 as u64 {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
    mut work_int: *mut i32,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    );
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as i32 as u64 {
        if n == 2 as i32 as u64 {
            dn = 0.399356f64;
        } else if n == 3 as i32 as u64 {
            dn = 0.99365f64;
        } else if n == 4 as i32 as u64 {
            dn = 0.51321f64;
        } else if n == 5 as i32 as u64 {
            dn = 0.84401f64;
        } else if n == 6 as i32 as u64 {
            dn = 0.61220f64;
        } else if n == 7 as i32 as u64 {
            dn = 0.85877f64;
        } else if n == 8 as i32 as u64 {
            dn = 0.66993f64;
        } else if n == 9 as i32 as u64 {
            dn = 0.87344f64;
        } else if n == 10 as i32 as u64 {
            dn = 0.72014f64;
        } else if n == 11 as i32 as u64 {
            dn = 0.88906f64;
        } else if n == 12 as i32 as u64 {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_Qn_from_sorted_data(
    mut sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mut work: *mut f128::f128,
    mut work_int: *mut i32,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = (gsl_stats_long_double_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ))
        .to_f64()
        .unwrap();
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as i32 as u64 {
        if n == 2 as i32 as u64 {
            dn = 0.399356f64;
        } else if n == 3 as i32 as u64 {
            dn = 0.99365f64;
        } else if n == 4 as i32 as u64 {
            dn = 0.51321f64;
        } else if n == 5 as i32 as u64 {
            dn = 0.84401f64;
        } else if n == 6 as i32 as u64 {
            dn = 0.61220f64;
        } else if n == 7 as i32 as u64 {
            dn = 0.85877f64;
        } else if n == 8 as i32 as u64 {
            dn = 0.66993f64;
        } else if n == 9 as i32 as u64 {
            dn = 0.87344f64;
        } else if n == 10 as i32 as u64 {
            dn = 0.72014f64;
        } else if n == 11 as i32 as u64 {
            dn = 0.88906f64;
        } else if n == 12 as i32 as u64 {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_Qn_from_sorted_data(
    mut sorted_data: *const i8,
    stride: size_t,
    n: size_t,
    mut work: *mut i8,
    mut work_int: *mut i32,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_char_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as i32 as u64 {
        if n == 2 as i32 as u64 {
            dn = 0.399356f64;
        } else if n == 3 as i32 as u64 {
            dn = 0.99365f64;
        } else if n == 4 as i32 as u64 {
            dn = 0.51321f64;
        } else if n == 5 as i32 as u64 {
            dn = 0.84401f64;
        } else if n == 6 as i32 as u64 {
            dn = 0.61220f64;
        } else if n == 7 as i32 as u64 {
            dn = 0.85877f64;
        } else if n == 8 as i32 as u64 {
            dn = 0.66993f64;
        } else if n == 9 as i32 as u64 {
            dn = 0.87344f64;
        } else if n == 10 as i32 as u64 {
            dn = 0.72014f64;
        } else if n == 11 as i32 as u64 {
            dn = 0.88906f64;
        } else if n == 12 as i32 as u64 {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
unsafe extern "C" fn Qn_int_whimed(
    mut a: *mut i32,
    mut w: *mut i32,
    mut n: i32,
    mut a_cand: *mut i32,
    mut a_srt: *mut i32,
    mut w_cand: *mut i32,
) -> i32 {
    let mut n2: i32 = 0;
    let mut i: i32 = 0;
    let mut kcand: i32 = 0;
    let mut wleft: i64 = 0;
    let mut wmid: i64 = 0;
    let mut wright: i64 = 0;
    let mut w_tot: i64 = 0;
    let mut wrest: i64 = 0;
    let mut trial: i32 = 0;
    w_tot = 0 as i32 as i64;
    i = 0 as i32;
    while i < n {
        w_tot += *w.offset(i as isize) as i64;
        i += 1;
        i;
    }
    wrest = 0 as i32 as i64;
    loop {
        i = 0 as i32;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as i32;
        gsl_sort_int(a_srt, 1 as i32 as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as i32 as i64;
        wmid = 0 as i32 as i64;
        wright = 0 as i32 as i64;
        i = 0 as i32;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as i64;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as i64;
            } else {
                wmid += *w.offset(i as isize) as i64;
            }
            i += 1;
            i;
        }
        kcand = 0 as i32;
        if 2 as i32 as i64 * (wrest + wleft) > w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as i32 as i64 * (wrest + wleft + wmid) <= w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as i32;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_float_whimed(
    mut a: *mut libc::c_float,
    mut w: *mut i32,
    mut n: i32,
    mut a_cand: *mut libc::c_float,
    mut a_srt: *mut libc::c_float,
    mut w_cand: *mut i32,
) -> libc::c_float {
    let mut n2: i32 = 0;
    let mut i: i32 = 0;
    let mut kcand: i32 = 0;
    let mut wleft: i64 = 0;
    let mut wmid: i64 = 0;
    let mut wright: i64 = 0;
    let mut w_tot: i64 = 0;
    let mut wrest: i64 = 0;
    let mut trial: libc::c_float = 0.;
    w_tot = 0 as i32 as i64;
    i = 0 as i32;
    while i < n {
        w_tot += *w.offset(i as isize) as i64;
        i += 1;
        i;
    }
    wrest = 0 as i32 as i64;
    loop {
        i = 0 as i32;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as i32;
        gsl_sort_float(a_srt, 1 as i32 as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as i32 as i64;
        wmid = 0 as i32 as i64;
        wright = 0 as i32 as i64;
        i = 0 as i32;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as i64;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as i64;
            } else {
                wmid += *w.offset(i as isize) as i64;
            }
            i += 1;
            i;
        }
        kcand = 0 as i32;
        if 2 as i32 as i64 * (wrest + wleft) > w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as i32 as i64 * (wrest + wleft + wmid) <= w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as i32;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_char_whimed(
    mut a: *mut i8,
    mut w: *mut i32,
    mut n: i32,
    mut a_cand: *mut i8,
    mut a_srt: *mut i8,
    mut w_cand: *mut i32,
) -> i8 {
    let mut n2: i32 = 0;
    let mut i: i32 = 0;
    let mut kcand: i32 = 0;
    let mut wleft: i64 = 0;
    let mut wmid: i64 = 0;
    let mut wright: i64 = 0;
    let mut w_tot: i64 = 0;
    let mut wrest: i64 = 0;
    let mut trial: i8 = 0;
    w_tot = 0 as i32 as i64;
    i = 0 as i32;
    while i < n {
        w_tot += *w.offset(i as isize) as i64;
        i += 1;
        i;
    }
    wrest = 0 as i32 as i64;
    loop {
        i = 0 as i32;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as i32;
        gsl_sort_char(a_srt, 1 as i32 as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as i32 as i64;
        wmid = 0 as i32 as i64;
        wright = 0 as i32 as i64;
        i = 0 as i32;
        while i < n {
            if (*a.offset(i as isize) as i32) < trial as i32 {
                wleft += *w.offset(i as isize) as i64;
            } else if *a.offset(i as isize) as i32 > trial as i32 {
                wright += *w.offset(i as isize) as i64;
            } else {
                wmid += *w.offset(i as isize) as i64;
            }
            i += 1;
            i;
        }
        kcand = 0 as i32;
        if 2 as i32 as i64 * (wrest + wleft) > w_tot {
            i = 0 as i32;
            while i < n {
                if (*a.offset(i as isize) as i32) < trial as i32 {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as i32 as i64 * (wrest + wleft + wmid) <= w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) as i32 > trial as i32 {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as i32;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_whimed(
    mut a: *mut libc::c_double,
    mut w: *mut i32,
    mut n: i32,
    mut a_cand: *mut libc::c_double,
    mut a_srt: *mut libc::c_double,
    mut w_cand: *mut i32,
) -> libc::c_double {
    let mut n2: i32 = 0;
    let mut i: i32 = 0;
    let mut kcand: i32 = 0;
    let mut wleft: i64 = 0;
    let mut wmid: i64 = 0;
    let mut wright: i64 = 0;
    let mut w_tot: i64 = 0;
    let mut wrest: i64 = 0;
    let mut trial: libc::c_double = 0.;
    w_tot = 0 as i32 as i64;
    i = 0 as i32;
    while i < n {
        w_tot += *w.offset(i as isize) as i64;
        i += 1;
        i;
    }
    wrest = 0 as i32 as i64;
    loop {
        i = 0 as i32;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as i32;
        gsl_sort(a_srt, 1 as i32 as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as i32 as i64;
        wmid = 0 as i32 as i64;
        wright = 0 as i32 as i64;
        i = 0 as i32;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as i64;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as i64;
            } else {
                wmid += *w.offset(i as isize) as i64;
            }
            i += 1;
            i;
        }
        kcand = 0 as i32;
        if 2 as i32 as i64 * (wrest + wleft) > w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as i32 as i64 * (wrest + wleft + wmid) <= w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as i32;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_uchar_whimed(
    mut a: *mut u8,
    mut w: *mut i32,
    mut n: i32,
    mut a_cand: *mut u8,
    mut a_srt: *mut u8,
    mut w_cand: *mut i32,
) -> u8 {
    let mut n2: i32 = 0;
    let mut i: i32 = 0;
    let mut kcand: i32 = 0;
    let mut wleft: i64 = 0;
    let mut wmid: i64 = 0;
    let mut wright: i64 = 0;
    let mut w_tot: i64 = 0;
    let mut wrest: i64 = 0;
    let mut trial: u8 = 0;
    w_tot = 0 as i32 as i64;
    i = 0 as i32;
    while i < n {
        w_tot += *w.offset(i as isize) as i64;
        i += 1;
        i;
    }
    wrest = 0 as i32 as i64;
    loop {
        i = 0 as i32;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as i32;
        gsl_sort_uchar(a_srt, 1 as i32 as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as i32 as i64;
        wmid = 0 as i32 as i64;
        wright = 0 as i32 as i64;
        i = 0 as i32;
        while i < n {
            if (*a.offset(i as isize) as i32) < trial as i32 {
                wleft += *w.offset(i as isize) as i64;
            } else if *a.offset(i as isize) as i32 > trial as i32 {
                wright += *w.offset(i as isize) as i64;
            } else {
                wmid += *w.offset(i as isize) as i64;
            }
            i += 1;
            i;
        }
        kcand = 0 as i32;
        if 2 as i32 as i64 * (wrest + wleft) > w_tot {
            i = 0 as i32;
            while i < n {
                if (*a.offset(i as isize) as i32) < trial as i32 {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as i32 as i64 * (wrest + wleft + wmid) <= w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) as i32 > trial as i32 {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as i32;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_ulong_whimed(
    mut a: *mut u64,
    mut w: *mut i32,
    mut n: i32,
    mut a_cand: *mut u64,
    mut a_srt: *mut u64,
    mut w_cand: *mut i32,
) -> u64 {
    let mut n2: i32 = 0;
    let mut i: i32 = 0;
    let mut kcand: i32 = 0;
    let mut wleft: i64 = 0;
    let mut wmid: i64 = 0;
    let mut wright: i64 = 0;
    let mut w_tot: i64 = 0;
    let mut wrest: i64 = 0;
    let mut trial: u64 = 0;
    w_tot = 0 as i32 as i64;
    i = 0 as i32;
    while i < n {
        w_tot += *w.offset(i as isize) as i64;
        i += 1;
        i;
    }
    wrest = 0 as i32 as i64;
    loop {
        i = 0 as i32;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as i32;
        gsl_sort_ulong(a_srt, 1 as i32 as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as i32 as i64;
        wmid = 0 as i32 as i64;
        wright = 0 as i32 as i64;
        i = 0 as i32;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as i64;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as i64;
            } else {
                wmid += *w.offset(i as isize) as i64;
            }
            i += 1;
            i;
        }
        kcand = 0 as i32;
        if 2 as i32 as i64 * (wrest + wleft) > w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as i32 as i64 * (wrest + wleft + wmid) <= w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as i32;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_long_double_whimed(
    mut a: *mut f128::f128,
    mut w: *mut i32,
    mut n: i32,
    mut a_cand: *mut f128::f128,
    mut a_srt: *mut f128::f128,
    mut w_cand: *mut i32,
) -> f128::f128 {
    let mut n2: i32 = 0;
    let mut i: i32 = 0;
    let mut kcand: i32 = 0;
    let mut wleft: i64 = 0;
    let mut wmid: i64 = 0;
    let mut wright: i64 = 0;
    let mut w_tot: i64 = 0;
    let mut wrest: i64 = 0;
    let mut trial: f128::f128 = f128::f128::ZERO;
    w_tot = 0 as i32 as i64;
    i = 0 as i32;
    while i < n {
        w_tot += *w.offset(i as isize) as i64;
        i += 1;
        i;
    }
    wrest = 0 as i32 as i64;
    loop {
        i = 0 as i32;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as i32;
        gsl_sort_long_double(a_srt, 1 as i32 as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as i32 as i64;
        wmid = 0 as i32 as i64;
        wright = 0 as i32 as i64;
        i = 0 as i32;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as i64;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as i64;
            } else {
                wmid += *w.offset(i as isize) as i64;
            }
            i += 1;
            i;
        }
        kcand = 0 as i32;
        if 2 as i32 as i64 * (wrest + wleft) > w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as i32 as i64 * (wrest + wleft + wmid) <= w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as i32;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_uint_whimed(
    mut a: *mut u32,
    mut w: *mut i32,
    mut n: i32,
    mut a_cand: *mut u32,
    mut a_srt: *mut u32,
    mut w_cand: *mut i32,
) -> u32 {
    let mut n2: i32 = 0;
    let mut i: i32 = 0;
    let mut kcand: i32 = 0;
    let mut wleft: i64 = 0;
    let mut wmid: i64 = 0;
    let mut wright: i64 = 0;
    let mut w_tot: i64 = 0;
    let mut wrest: i64 = 0;
    let mut trial: u32 = 0;
    w_tot = 0 as i32 as i64;
    i = 0 as i32;
    while i < n {
        w_tot += *w.offset(i as isize) as i64;
        i += 1;
        i;
    }
    wrest = 0 as i32 as i64;
    loop {
        i = 0 as i32;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as i32;
        gsl_sort_uint(a_srt, 1 as i32 as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as i32 as i64;
        wmid = 0 as i32 as i64;
        wright = 0 as i32 as i64;
        i = 0 as i32;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as i64;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as i64;
            } else {
                wmid += *w.offset(i as isize) as i64;
            }
            i += 1;
            i;
        }
        kcand = 0 as i32;
        if 2 as i32 as i64 * (wrest + wleft) > w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as i32 as i64 * (wrest + wleft + wmid) <= w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as i32;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_long_whimed(
    mut a: *mut i64,
    mut w: *mut i32,
    mut n: i32,
    mut a_cand: *mut i64,
    mut a_srt: *mut i64,
    mut w_cand: *mut i32,
) -> i64 {
    let mut n2: i32 = 0;
    let mut i: i32 = 0;
    let mut kcand: i32 = 0;
    let mut wleft: i64 = 0;
    let mut wmid: i64 = 0;
    let mut wright: i64 = 0;
    let mut w_tot: i64 = 0;
    let mut wrest: i64 = 0;
    let mut trial: i64 = 0;
    w_tot = 0 as i32 as i64;
    i = 0 as i32;
    while i < n {
        w_tot += *w.offset(i as isize) as i64;
        i += 1;
        i;
    }
    wrest = 0 as i32 as i64;
    loop {
        i = 0 as i32;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as i32;
        gsl_sort_long(a_srt, 1 as i32 as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as i32 as i64;
        wmid = 0 as i32 as i64;
        wright = 0 as i32 as i64;
        i = 0 as i32;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as i64;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as i64;
            } else {
                wmid += *w.offset(i as isize) as i64;
            }
            i += 1;
            i;
        }
        kcand = 0 as i32;
        if 2 as i32 as i64 * (wrest + wleft) > w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as i32 as i64 * (wrest + wleft + wmid) <= w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as i32;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_ushort_whimed(
    mut a: *mut libc::c_ushort,
    mut w: *mut i32,
    mut n: i32,
    mut a_cand: *mut libc::c_ushort,
    mut a_srt: *mut libc::c_ushort,
    mut w_cand: *mut i32,
) -> libc::c_ushort {
    let mut n2: i32 = 0;
    let mut i: i32 = 0;
    let mut kcand: i32 = 0;
    let mut wleft: i64 = 0;
    let mut wmid: i64 = 0;
    let mut wright: i64 = 0;
    let mut w_tot: i64 = 0;
    let mut wrest: i64 = 0;
    let mut trial: libc::c_ushort = 0;
    w_tot = 0 as i32 as i64;
    i = 0 as i32;
    while i < n {
        w_tot += *w.offset(i as isize) as i64;
        i += 1;
        i;
    }
    wrest = 0 as i32 as i64;
    loop {
        i = 0 as i32;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as i32;
        gsl_sort_ushort(a_srt, 1 as i32 as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as i32 as i64;
        wmid = 0 as i32 as i64;
        wright = 0 as i32 as i64;
        i = 0 as i32;
        while i < n {
            if (*a.offset(i as isize) as i32) < trial as i32 {
                wleft += *w.offset(i as isize) as i64;
            } else if *a.offset(i as isize) as i32 > trial as i32 {
                wright += *w.offset(i as isize) as i64;
            } else {
                wmid += *w.offset(i as isize) as i64;
            }
            i += 1;
            i;
        }
        kcand = 0 as i32;
        if 2 as i32 as i64 * (wrest + wleft) > w_tot {
            i = 0 as i32;
            while i < n {
                if (*a.offset(i as isize) as i32) < trial as i32 {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as i32 as i64 * (wrest + wleft + wmid) <= w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) as i32 > trial as i32 {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as i32;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_short_whimed(
    mut a: *mut libc::c_short,
    mut w: *mut i32,
    mut n: i32,
    mut a_cand: *mut libc::c_short,
    mut a_srt: *mut libc::c_short,
    mut w_cand: *mut i32,
) -> libc::c_short {
    let mut n2: i32 = 0;
    let mut i: i32 = 0;
    let mut kcand: i32 = 0;
    let mut wleft: i64 = 0;
    let mut wmid: i64 = 0;
    let mut wright: i64 = 0;
    let mut w_tot: i64 = 0;
    let mut wrest: i64 = 0;
    let mut trial: libc::c_short = 0;
    w_tot = 0 as i32 as i64;
    i = 0 as i32;
    while i < n {
        w_tot += *w.offset(i as isize) as i64;
        i += 1;
        i;
    }
    wrest = 0 as i32 as i64;
    loop {
        i = 0 as i32;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as i32;
        gsl_sort_short(a_srt, 1 as i32 as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as i32 as i64;
        wmid = 0 as i32 as i64;
        wright = 0 as i32 as i64;
        i = 0 as i32;
        while i < n {
            if (*a.offset(i as isize) as i32) < trial as i32 {
                wleft += *w.offset(i as isize) as i64;
            } else if *a.offset(i as isize) as i32 > trial as i32 {
                wright += *w.offset(i as isize) as i64;
            } else {
                wmid += *w.offset(i as isize) as i64;
            }
            i += 1;
            i;
        }
        kcand = 0 as i32;
        if 2 as i32 as i64 * (wrest + wleft) > w_tot {
            i = 0 as i32;
            while i < n {
                if (*a.offset(i as isize) as i32) < trial as i32 {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as i32 as i64 * (wrest + wleft + wmid) <= w_tot {
            i = 0 as i32;
            while i < n {
                if *a.offset(i as isize) as i32 > trial as i32 {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as i32;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}