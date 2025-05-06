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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
}
pub const _ISdigit: C2RustUnnamed = 2048;
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[no_mangle]
pub unsafe extern "C" fn _glp_str2num(
    mut str: *const i8,
    mut val_: *mut libc::c_double,
) -> i32 {
    let mut k: i32 = 0;
    let mut val: libc::c_double = 0.;
    k = if *str.offset(0 as i32 as isize) as i32 == '+' as i32
        || *str.offset(0 as i32 as isize) as i32 == '-' as i32
    {
        1 as i32
    } else {
        0 as i32
    };
    if *str.offset(k as isize) as i32 == '.' as i32 {
        k += 1;
        k;
        if *(*__ctype_b_loc()).offset(*str.offset(k as isize) as u8 as i32 as isize)
            as i32 & _ISdigit as i32 as libc::c_ushort as i32 == 0
        {
            return 2 as i32;
        }
        k += 1;
        k;
    } else {
        if *(*__ctype_b_loc()).offset(*str.offset(k as isize) as u8 as i32 as isize)
            as i32 & _ISdigit as i32 as libc::c_ushort as i32 == 0
        {
            return 2 as i32;
        }
        while *(*__ctype_b_loc()).offset(*str.offset(k as isize) as u8 as i32 as isize)
            as i32 & _ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            k += 1;
            k;
        }
        if *str.offset(k as isize) as i32 == '.' as i32 {
            k += 1;
            k;
        }
    }
    while *(*__ctype_b_loc()).offset(*str.offset(k as isize) as u8 as i32 as isize)
        as i32 & _ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        k += 1;
        k;
    }
    if *str.offset(k as isize) as i32 == 'E' as i32
        || *str.offset(k as isize) as i32 == 'e' as i32
    {
        k += 1;
        k;
        if *str.offset(k as isize) as i32 == '+' as i32
            || *str.offset(k as isize) as i32 == '-' as i32
        {
            k += 1;
            k;
        }
        if *(*__ctype_b_loc()).offset(*str.offset(k as isize) as u8 as i32 as isize)
            as i32 & _ISdigit as i32 as libc::c_ushort as i32 == 0
        {
            return 2 as i32;
        }
    }
    while *(*__ctype_b_loc()).offset(*str.offset(k as isize) as u8 as i32 as isize)
        as i32 & _ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        k += 1;
        k;
    }
    if *str.offset(k as isize) as i32 != '\0' as i32 {
        return 2 as i32;
    }
    let mut endptr: *mut i8 = 0 as *mut i8;
    val = strtod(str, &mut endptr);
    if *endptr as i32 != '\0' as i32 {
        return 2 as i32;
    }
    if !(-1.7976931348623157e+308f64 <= val && val <= 1.7976931348623157e+308f64) {
        return 1 as i32;
    }
    if -2.2250738585072014e-308f64 < val && val < 2.2250738585072014e-308f64 {
        val = 0.0f64;
    }
    *val_ = val;
    return 0 as i32;
}