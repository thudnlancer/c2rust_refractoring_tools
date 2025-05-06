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
pub unsafe extern "C" fn _glp_str2int(mut str: *const i8, mut val_: *mut i32) -> i32 {
    let mut d: i32 = 0;
    let mut k: i32 = 0;
    let mut s: i32 = 0;
    let mut val: i32 = 0 as i32;
    if *str.offset(0 as i32 as isize) as i32 == '+' as i32 {
        s = 1 as i32;
        k = 1 as i32;
    } else if *str.offset(0 as i32 as isize) as i32 == '-' as i32 {
        s = -(1 as i32);
        k = 1 as i32;
    } else {
        s = 1 as i32;
        k = 0 as i32;
    }
    if *(*__ctype_b_loc()).offset(*str.offset(k as isize) as u8 as i32 as isize) as i32
        & _ISdigit as i32 as libc::c_ushort as i32 == 0
    {
        return 2 as i32;
    }
    while *(*__ctype_b_loc()).offset(*str.offset(k as isize) as u8 as i32 as isize)
        as i32 & _ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        let fresh0 = k;
        k = k + 1;
        d = *str.offset(fresh0 as isize) as i32 - '0' as i32;
        if s > 0 as i32 {
            if val > 2147483647 as i32 / 10 as i32 {
                return 1 as i32;
            }
            val *= 10 as i32;
            if val > 2147483647 as i32 - d {
                return 1 as i32;
            }
            val += d;
        } else {
            if val < (-(2147483647 as i32) - 1 as i32) / 10 as i32 {
                return 1 as i32;
            }
            val *= 10 as i32;
            if val < -(2147483647 as i32) - 1 as i32 + d {
                return 1 as i32;
            }
            val -= d;
        }
    }
    if *str.offset(k as isize) as i32 != '\0' as i32 {
        return 2 as i32;
    }
    *val_ = val;
    return 0 as i32;
}