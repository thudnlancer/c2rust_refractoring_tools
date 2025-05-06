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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type uInt = u32;
pub type uLong = u64;
pub type voidpf = *mut libc::c_void;
#[no_mangle]
pub static mut _glp_zlib_z_errmsg: [*const i8; 10] = [
    b"need dictionary\0" as *const u8 as *const i8,
    b"stream end\0" as *const u8 as *const i8,
    b"\0" as *const u8 as *const i8,
    b"file error\0" as *const u8 as *const i8,
    b"stream error\0" as *const u8 as *const i8,
    b"data error\0" as *const u8 as *const i8,
    b"insufficient memory\0" as *const u8 as *const i8,
    b"buffer error\0" as *const u8 as *const i8,
    b"incompatible version\0" as *const u8 as *const i8,
    b"\0" as *const u8 as *const i8,
];
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_zlibVersion() -> *const i8 {
    return b"1.2.5\0" as *const u8 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_zlibCompileFlags() -> uLong {
    let mut flags: uLong = 0;
    flags = 0 as i32 as uLong;
    match ::core::mem::size_of::<uInt>() as u64 as i32 {
        2 => {}
        4 => {
            flags = (flags as u64).wrapping_add(1 as i32 as u64) as uLong as uLong;
        }
        8 => {
            flags = (flags as u64).wrapping_add(2 as i32 as u64) as uLong as uLong;
        }
        _ => {
            flags = (flags as u64).wrapping_add(3 as i32 as u64) as uLong as uLong;
        }
    }
    match ::core::mem::size_of::<uLong>() as u64 as i32 {
        2 => {}
        4 => {
            flags = (flags as u64).wrapping_add(((1 as i32) << 2 as i32) as u64) as uLong
                as uLong;
        }
        8 => {
            flags = (flags as u64).wrapping_add(((2 as i32) << 2 as i32) as u64) as uLong
                as uLong;
        }
        _ => {
            flags = (flags as u64).wrapping_add(((3 as i32) << 2 as i32) as u64) as uLong
                as uLong;
        }
    }
    match ::core::mem::size_of::<voidpf>() as u64 as i32 {
        2 => {}
        4 => {
            flags = (flags as u64).wrapping_add(((1 as i32) << 4 as i32) as u64) as uLong
                as uLong;
        }
        8 => {
            flags = (flags as u64).wrapping_add(((2 as i32) << 4 as i32) as u64) as uLong
                as uLong;
        }
        _ => {
            flags = (flags as u64).wrapping_add(((3 as i32) << 4 as i32) as u64) as uLong
                as uLong;
        }
    }
    match ::core::mem::size_of::<i64>() as u64 as i32 {
        2 => {}
        4 => {
            flags = (flags as u64).wrapping_add(((1 as i32) << 6 as i32) as u64) as uLong
                as uLong;
        }
        8 => {
            flags = (flags as u64).wrapping_add(((2 as i32) << 6 as i32) as u64) as uLong
                as uLong;
        }
        _ => {
            flags = (flags as u64).wrapping_add(((3 as i32) << 6 as i32) as u64) as uLong
                as uLong;
        }
    }
    flags = (flags as u64).wrapping_add(((1 as i64) << 25 as i32) as u64) as uLong
        as uLong;
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_zError(mut err: i32) -> *const i8 {
    return _glp_zlib_z_errmsg[(2 as i32 - err) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_zcalloc(
    mut opaque: voidpf,
    mut items: u32,
    mut size: u32,
) -> voidpf {
    if !opaque.is_null() {
        items = items.wrapping_add(size.wrapping_sub(size));
    }
    return if ::core::mem::size_of::<uInt>() as u64 > 2 as i32 as u64 {
        malloc(items.wrapping_mul(size) as u64)
    } else {
        calloc(items as u64, size as u64)
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_zcfree(mut opaque: voidpf, mut ptr: voidpf) {
    free(ptr);
    if !opaque.is_null() {
        return;
    }
}