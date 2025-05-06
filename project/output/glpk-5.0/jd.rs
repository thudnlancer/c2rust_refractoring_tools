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
pub unsafe extern "C" fn _glp_jday(mut d: i32, mut m: i32, mut y: i32) -> i32 {
    let mut c: i32 = 0;
    let mut ya: i32 = 0;
    let mut j: i32 = 0;
    let mut dd: i32 = 0;
    if !(1 as i32 <= d && d <= 31 as i32 && 1 as i32 <= m && m <= 12 as i32
        && 1 as i32 <= y && y <= 4000 as i32)
    {
        return -(1 as i32);
    }
    if m >= 3 as i32 {
        m -= 3 as i32;
    } else {
        m += 9 as i32;
        y -= 1;
        y;
    }
    c = y / 100 as i32;
    ya = y - 100 as i32 * c;
    j = 146097 as i32 * c / 4 as i32 + 1461 as i32 * ya / 4 as i32
        + (153 as i32 * m + 2 as i32) / 5 as i32 + d + 1721119 as i32;
    _glp_jdate(j, &mut dd, 0 as *mut i32, 0 as *mut i32);
    if d != dd {
        return -(1 as i32);
    }
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_jdate(
    mut j: i32,
    mut d_: *mut i32,
    mut m_: *mut i32,
    mut y_: *mut i32,
) -> i32 {
    let mut d: i32 = 0;
    let mut m: i32 = 0;
    let mut y: i32 = 0;
    if !(1721426 as i32 <= j && j <= 3182395 as i32) {
        return 1 as i32;
    }
    j -= 1721119 as i32;
    y = (4 as i32 * j - 1 as i32) / 146097 as i32;
    j = (4 as i32 * j - 1 as i32) % 146097 as i32;
    d = j / 4 as i32;
    j = (4 as i32 * d + 3 as i32) / 1461 as i32;
    d = (4 as i32 * d + 3 as i32) % 1461 as i32;
    d = (d + 4 as i32) / 4 as i32;
    m = (5 as i32 * d - 3 as i32) / 153 as i32;
    d = (5 as i32 * d - 3 as i32) % 153 as i32;
    d = (d + 5 as i32) / 5 as i32;
    y = 100 as i32 * y + j;
    if m <= 9 as i32 {
        m += 3 as i32;
    } else {
        m -= 9 as i32;
        y += 1;
        y;
    }
    if !d_.is_null() {
        *d_ = d;
    }
    if !m_.is_null() {
        *m_ = m;
    }
    if !y_.is_null() {
        *y_ = y;
    }
    return 0 as i32;
}