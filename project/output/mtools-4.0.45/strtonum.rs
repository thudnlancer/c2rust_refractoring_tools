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
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn strtol_with_range(
    mut nptr: *const i8,
    mut endptr: *mut *mut i8,
    mut base: i32,
    mut min: i64,
    mut max: i64,
) -> i64 {
    let mut l: i64 = strtol(nptr, endptr, base);
    if l > max {
        *__errno_location() = 34 as i32;
        return max;
    }
    if l < min {
        *__errno_location() = 34 as i32;
        return min;
    }
    return l;
}
unsafe extern "C" fn strtoul_with_range(
    mut nptr: *const i8,
    mut endptr: *mut *mut i8,
    mut base: i32,
    mut max: u64,
) -> u64 {
    let mut l: u64 = strtoul(nptr, endptr, base);
    if l > max {
        *__errno_location() = 34 as i32;
        return max;
    }
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn strtoui(
    mut nptr: *const i8,
    mut endptr: *mut *mut i8,
    mut base: i32,
) -> u32 {
    return strtoul_with_range(
        nptr,
        endptr,
        base,
        (2147483647 as i32 as u32).wrapping_mul(2 as u32).wrapping_add(1 as u32) as u64,
    ) as u32;
}
#[no_mangle]
pub unsafe extern "C" fn atoui(mut str: *const i8) -> u32 {
    return strtoui(str, 0 as *mut *mut i8, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn strtosi(
    mut nptr: *const i8,
    mut endptr: *mut *mut i8,
    mut base: i32,
) -> i32 {
    return strtol_with_range(
        nptr,
        endptr,
        base,
        (-(2147483647 as i32) - 1 as i32) as i64,
        2147483647 as i32 as i64,
    ) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn atoul(mut str: *const i8) -> u64 {
    return strtoul(str, 0 as *mut *mut i8, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn strtou8(
    mut nptr: *const i8,
    mut endptr: *mut *mut i8,
    mut base: i32,
) -> uint8_t {
    return strtoul_with_range(nptr, endptr, base, 255 as i32 as u64) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn atou8(mut str: *const i8) -> uint8_t {
    return strtou8(str, 0 as *mut *mut i8, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn strtou16(
    mut nptr: *const i8,
    mut endptr: *mut *mut i8,
    mut base: i32,
) -> uint16_t {
    return strtoul_with_range(nptr, endptr, base, 65535 as i32 as u64) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn atou16(mut str: *const i8) -> uint16_t {
    return strtou16(str, 0 as *mut *mut i8, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn strtou32(
    mut nptr: *const i8,
    mut endptr: *mut *mut i8,
    mut base: i32,
) -> uint32_t {
    return strtoul_with_range(nptr, endptr, base, 4294967295 as u32 as u64) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn atou32(mut str: *const i8) -> uint32_t {
    return strtou32(str, 0 as *mut *mut i8, 0 as i32);
}
unsafe extern "C" fn checkOverflow(mut tot_sectors: uint32_t, mut bits: i32) {
    if tot_sectors > 4294967295 as u32 >> bits {
        fprintf(stderr, b"Too many sectors\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn parseSize(mut sizeStr: *mut i8) -> uint32_t {
    let mut eptr: *mut i8 = 0 as *mut i8;
    let mut tot_sectors: uint32_t = strtou32(sizeStr, &mut eptr, 10 as i32);
    if eptr == sizeStr {
        fprintf(stderr, b"Bad size %s\n\0" as *const u8 as *const i8, sizeStr);
        exit(1 as i32);
    }
    let mut current_block_12: u64;
    match ({
        let mut __res: i32 = 0;
        if ::core::mem::size_of::<i8>() as u64 > 1 as i32 as u64 {
            if 0 != 0 {
                let mut __c: i32 = *eptr as i32;
                __res = if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                };
            } else {
                __res = toupper(*eptr as i32);
            }
        } else {
            __res = *(*__ctype_toupper_loc()).offset(*eptr as i32 as isize);
        }
        __res
    }) {
        84 => {
            checkOverflow(tot_sectors, 10 as i32);
            tot_sectors = (tot_sectors as u32).wrapping_mul(1024 as i32 as u32)
                as uint32_t as uint32_t;
            current_block_12 = 17691484140891172101;
        }
        71 => {
            current_block_12 = 17691484140891172101;
        }
        77 => {
            current_block_12 = 13243873874200723537;
        }
        75 => {
            current_block_12 = 2595707941723139929;
        }
        83 => {
            current_block_12 = 14873937869286739677;
        }
        0 => {
            current_block_12 = 7651349459974463963;
        }
        _ => {
            current_block_12 = 7651349459974463963;
        }
    }
    match current_block_12 {
        17691484140891172101 => {
            checkOverflow(tot_sectors, 10 as i32);
            tot_sectors = (tot_sectors as u32).wrapping_mul(1024 as i32 as u32)
                as uint32_t as uint32_t;
            current_block_12 = 13243873874200723537;
        }
        _ => {}
    }
    match current_block_12 {
        13243873874200723537 => {
            checkOverflow(tot_sectors, 10 as i32);
            tot_sectors = (tot_sectors as u32).wrapping_mul(1024 as i32 as u32)
                as uint32_t as uint32_t;
            current_block_12 = 2595707941723139929;
        }
        _ => {}
    }
    match current_block_12 {
        2595707941723139929 => {
            checkOverflow(tot_sectors, 1 as i32);
            tot_sectors = (tot_sectors as u32).wrapping_mul(2 as i32 as u32) as uint32_t
                as uint32_t;
            current_block_12 = 14873937869286739677;
        }
        _ => {}
    }
    match current_block_12 {
        14873937869286739677 => {
            eptr = eptr.offset(1);
            eptr;
        }
        _ => {}
    }
    if *eptr != 0 {
        fprintf(stderr, b"Bad suffix %s\n\0" as *const u8 as *const i8, eptr);
        exit(1 as i32);
    }
    return tot_sectors;
}