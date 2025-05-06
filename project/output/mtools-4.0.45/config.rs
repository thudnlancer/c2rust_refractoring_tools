#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type __locale_data;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn __toupper_l(__c: i32, __l: locale_t) -> i32;
    fn __errno_location() -> *mut i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strdup(_: *const i8) -> *mut i8;
    fn strndup(_: *const i8, _: u64) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strcspn(_: *const i8, _: *const i8) -> u64;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strncasecmp_l(
        __s1: *const i8,
        __s2: *const i8,
        __n: size_t,
        __loc: locale_t,
    ) -> i32;
    fn newlocale(
        __category_mask: i32,
        __locale: *const i8,
        __base: locale_t,
    ) -> locale_t;
    fn str_to_off_with_end(str: *const i8, endp: *mut *mut i8) -> mt_off_t;
    fn get_homedir() -> *mut i8;
    fn strtosi(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> i32;
    fn strtoui(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> u32;
    fn strtou8(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> uint8_t;
    fn strtou16(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> uint16_t;
    fn str_to_offset(str: *mut i8) -> mt_off_t;
    fn printOom();
    static mut const_devices: [device; 0];
    static nr_const_devices: u32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __caddr_t = *mut i8;
pub type off_t = __off_t;
pub type caddr_t = __caddr_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const libc::c_ushort,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const i8; 13],
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
pub type mt_off_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const i8,
    pub drive: i8,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: u32,
    pub offset: mt_off_t,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: uint8_t,
    pub use_2m: u32,
    pub precmd: *mut i8,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: *const i8,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut i8,
    pub cfg_filename: *const i8,
}
pub type switches_t = switches_l;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct switches_l {
    pub name: *const i8,
    pub address: caddr_t,
    pub type_0: C2RustUnnamed,
}
pub type C2RustUnnamed = u32;
pub const T_OFFS: C2RustUnnamed = 6;
pub const T_UQSTRING: C2RustUnnamed = 5;
pub const T_UINT16: C2RustUnnamed = 4;
pub const T_UINT8: C2RustUnnamed = 3;
pub const T_UINT: C2RustUnnamed = 2;
pub const T_STRING: C2RustUnnamed = 1;
pub const T_INT: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub name: *const i8,
    pub fat_bits: libc::c_schar,
    pub tracks: u32,
    pub heads: libc::c_ushort,
    pub sectors: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flags_t {
    pub name: *const i8,
    pub flag: u32,
}
#[inline]
unsafe extern "C" fn ptrdiff(mut end: *const i8, mut begin: *const i8) -> size_t {
    return end.offset_from(begin) as i64 as size_t;
}
static mut buffer: [i8; 257] = [0; 257];
static mut pos: *mut i8 = 0 as *const i8 as *mut i8;
static mut token: *mut i8 = 0 as *const i8 as *mut i8;
static mut token_length: size_t = 0;
static mut fp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut linenumber: i32 = 0;
static mut lastTokenLinenumber: i32 = 0;
static mut filename: *const i8 = 0 as *const i8;
static mut file_nr: i32 = 0 as i32;
static mut flag_mask: u32 = 0;
static mut cur_devs: u32 = 0;
static mut cur_dev: i32 = 0;
static mut trusted: i32 = 0 as i32;
static mut nr_dev: u32 = 0;
#[no_mangle]
pub static mut devices: *mut device = 0 as *const device as *mut device;
static mut token_nr: i32 = 0;
static mut default_drive: i8 = '\0' as i32 as i8;
#[no_mangle]
pub static mut mtools_skip_check: u32 = 0 as i32 as u32;
#[no_mangle]
pub static mut mtools_fat_compatibility: u32 = 0 as i32 as u32;
#[no_mangle]
pub static mut mtools_ignore_short_case: u32 = 0 as i32 as u32;
#[no_mangle]
pub static mut mtools_rate_0: uint8_t = 0 as i32 as uint8_t;
#[no_mangle]
pub static mut mtools_rate_any: uint8_t = 0 as i32 as uint8_t;
#[no_mangle]
pub static mut mtools_no_vfat: u32 = 0 as i32 as u32;
#[no_mangle]
pub static mut mtools_numeric_tail: u32 = 1 as i32 as u32;
#[no_mangle]
pub static mut mtools_dotted_dir: u32 = 0 as i32 as u32;
#[no_mangle]
pub static mut mtools_twenty_four_hour_clock: u32 = 1 as i32 as u32;
#[no_mangle]
pub static mut mtools_lock_timeout: u32 = 30 as i32 as u32;
#[no_mangle]
pub static mut mtools_default_codepage: u32 = 850 as i32 as u32;
#[no_mangle]
pub static mut mtools_date_string: *const i8 = b"yyyy-mm-dd\0" as *const u8 as *const i8;
static mut global_switches: [switches_t; 12] = unsafe {
    [
        {
            let mut init = switches_l {
                name: b"MTOOLS_LOWER_CASE\0" as *const u8 as *const i8,
                address: &mtools_ignore_short_case as *const u32 as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_FAT_COMPATIBILITY\0" as *const u8 as *const i8,
                address: &mtools_fat_compatibility as *const u32 as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_SKIP_CHECK\0" as *const u8 as *const i8,
                address: &mtools_skip_check as *const u32 as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_NO_VFAT\0" as *const u8 as *const i8,
                address: &mtools_no_vfat as *const u32 as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_RATE_0\0" as *const u8 as *const i8,
                address: &mtools_rate_0 as *const uint8_t as *mut uint8_t as caddr_t,
                type_0: T_UINT8,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_RATE_ANY\0" as *const u8 as *const i8,
                address: &mtools_rate_any as *const uint8_t as *mut uint8_t as caddr_t,
                type_0: T_UINT8,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_NAME_NUMERIC_TAIL\0" as *const u8 as *const i8,
                address: &mtools_numeric_tail as *const u32 as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_DOTTED_DIR\0" as *const u8 as *const i8,
                address: &mtools_dotted_dir as *const u32 as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_TWENTY_FOUR_HOUR_CLOCK\0" as *const u8 as *const i8,
                address: &mtools_twenty_four_hour_clock as *const u32 as *mut u32
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_DATE_STRING\0" as *const u8 as *const i8,
                address: &mtools_date_string as *const *const i8 as *mut *const i8
                    as caddr_t,
                type_0: T_STRING,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_LOCK_TIMEOUT\0" as *const u8 as *const i8,
                address: &mtools_lock_timeout as *const u32 as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"DEFAULT_CODEPAGE\0" as *const u8 as *const i8,
                address: &mtools_default_codepage as *const u32 as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
    ]
};
static mut openflags: [flags_t; 4] = [
    {
        let mut init = flags_t {
            name: b"sync\0" as *const u8 as *const i8,
            flag: 0o4010000 as i32 as u32,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"nodelay\0" as *const u8 as *const i8,
            flag: 0o4000 as i32 as u32,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"exclusive\0" as *const u8 as *const i8,
            flag: 0o200 as i32 as u32,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"none\0" as *const u8 as *const i8,
            flag: 0 as i32 as u32,
        };
        init
    },
];
static mut misc_flags: [flags_t; 9] = [
    {
        let mut init = flags_t {
            name: b"use_xdf\0" as *const u8 as *const i8,
            flag: 0x8 as u32,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"scsi\0" as *const u8 as *const i8,
            flag: 0x1 as u32,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"nolock\0" as *const u8 as *const i8,
            flag: 0x4 as u32,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"mformat_only\0" as *const u8 as *const i8,
            flag: 0x10 as u32,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"filter\0" as *const u8 as *const i8,
            flag: 0x80 as u32,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"privileged\0" as *const u8 as *const i8,
            flag: 0x2 as u32,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"vold\0" as *const u8 as *const i8,
            flag: 0x20 as u32,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"remote\0" as *const u8 as *const i8,
            flag: 0x40 as u32,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"swap\0" as *const u8 as *const i8,
            flag: 0x100 as u32,
        };
        init
    },
];
static mut default_formats: [C2RustUnnamed_0; 15] = [
    {
        let mut init = C2RustUnnamed_0 {
            name: b"hd514\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 80 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 15 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"high-density-5-1/4\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 80 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 15 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"1.2m\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 80 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 15 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"hd312\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 80 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 18 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"high-density-3-1/2\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 80 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 18 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"1.44m\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 80 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 18 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"dd312\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 80 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 9 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"double-density-3-1/2\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 80 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 9 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"720k\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 80 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 9 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"dd514\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 40 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 9 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"double-density-5-1/4\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 40 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 9 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"360k\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 40 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 9 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"320k\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 40 as i32 as u32,
            heads: 2 as i32 as libc::c_ushort,
            sectors: 8 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"180k\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 40 as i32 as u32,
            heads: 1 as i32 as libc::c_ushort,
            sectors: 9 as i32 as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"160k\0" as *const u8 as *const i8,
            fat_bits: 12 as i32 as libc::c_schar,
            tracks: 40 as i32 as u32,
            heads: 1 as i32 as libc::c_ushort,
            sectors: 8 as i32 as libc::c_ushort,
        };
        init
    },
];
static mut dswitches: [switches_t; 16] = [switches_t {
    name: 0 as *const i8,
    address: 0 as *const i8 as *mut i8,
    type_0: T_INT,
}; 16];
static mut C: locale_t = 0 as *const __locale_struct as locale_t;
unsafe extern "C" fn init_canon() {
    if C.is_null() {
        C = newlocale(
            (1 as i32) << 0 as i32,
            b"C\0" as *const u8 as *const i8,
            0 as locale_t,
        );
    }
}
unsafe extern "C" fn canon_drv(mut drive: i32) -> i32 {
    let mut ret: i32 = 0;
    init_canon();
    ret = ({
        let mut __res: i32 = 0;
        if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
            if 0 != 0 {
                let mut __c: i32 = drive;
                __res = if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *((*C).__ctype_toupper).offset(__c as isize)
                };
            } else {
                __res = __toupper_l(drive, C);
            }
        } else {
            __res = *((*C).__ctype_toupper).offset(drive as isize);
        }
        __res
    });
    return ret;
}
unsafe extern "C" fn cmp_tok(
    mut a: *const i8,
    mut b: *const i8,
    mut len: size_t,
) -> i32 {
    init_canon();
    return strncasecmp_l(a, b, len, C);
}
unsafe extern "C" fn ch_canon_drv(mut drive: i8) -> i8 {
    return canon_drv(drive as u8 as i32) as i8;
}
unsafe extern "C" fn maintain_default_drive(mut drive: i8) {
    if default_drive as i32 == ':' as i32 {
        return;
    }
    if default_drive as i32 == '\0' as i32 || default_drive as i32 > drive as i32 {
        default_drive = drive;
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_default_drive() -> i8 {
    if default_drive as i32 != '\0' as i32 {
        return default_drive
    } else {
        return 'A' as i32 as i8
    };
}
unsafe extern "C" fn syntax(mut msg: *const i8, mut thisLine: i32) -> ! {
    let mut drive: i8 = '\0' as i32 as i8;
    if thisLine != 0 {
        lastTokenLinenumber = linenumber;
    }
    if cur_dev >= 0 as i32 {
        drive = (*devices.offset(cur_dev as isize)).drive;
    }
    fprintf(
        stderr,
        b"Syntax error at line %d \0" as *const u8 as *const i8,
        lastTokenLinenumber,
    );
    if drive != 0 {
        fprintf(stderr, b"for drive %c: \0" as *const u8 as *const i8, drive as i32);
    }
    if !token.is_null() {
        fprintf(
            stderr,
            b"column %ld \0" as *const u8 as *const i8,
            token.offset_from(buffer.as_mut_ptr()) as i64,
        );
    }
    fprintf(stderr, b"in file %s: %s\0" as *const u8 as *const i8, filename, msg);
    if *__errno_location() != 0 as i32 {
        fprintf(
            stderr,
            b" (%s)\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    exit(1 as i32);
}
unsafe extern "C" fn get_env_conf() {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[switches_t; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<switches_t>() as u64)
    {
        s = getenv(global_switches[i as usize].name);
        if !s.is_null() {
            *__errno_location() = 0 as i32;
            match global_switches[i as usize].type_0 as u32 {
                0 => {
                    *(global_switches[i as usize].address as *mut i32) = strtosi(
                        s,
                        0 as *mut *mut i8,
                        0 as i32,
                    );
                }
                2 => {
                    *(global_switches[i as usize].address as *mut u32) = strtoui(
                        s,
                        0 as *mut *mut i8,
                        0 as i32,
                    );
                }
                3 => {
                    *(global_switches[i as usize].address as *mut uint8_t) = strtou8(
                        s,
                        0 as *mut *mut i8,
                        0 as i32,
                    );
                }
                4 => {
                    *(global_switches[i as usize].address as *mut uint16_t) = strtou16(
                        s,
                        0 as *mut *mut i8,
                        0 as i32,
                    );
                }
                1 | 5 => {
                    let ref mut fresh0 = *(global_switches[i as usize].address
                        as *mut *mut i8);
                    *fresh0 = s;
                }
                6 => {
                    *(global_switches[i as usize].address as *mut mt_off_t) = str_to_offset(
                        s,
                    );
                }
                _ => {}
            }
            if *__errno_location() != 0 as i32 {
                fprintf(
                    stderr,
                    b"Bad number %s for %s (%s)\n\0" as *const u8 as *const i8,
                    s,
                    global_switches[i as usize].name,
                    strerror(*__errno_location()),
                );
                exit(1 as i32);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn mtools_getline() -> i32 {
    if fp.is_null() || (fgets(buffer.as_mut_ptr(), 256 as i32 + 1 as i32, fp)).is_null()
    {
        return -(1 as i32);
    }
    linenumber += 1;
    linenumber;
    pos = buffer.as_mut_ptr();
    token_nr = 0 as i32;
    buffer[256 as i32 as usize] = '\0' as i32 as i8;
    if strlen(buffer.as_mut_ptr()) == 256 as i32 as u64 {
        syntax(b"line too long\0" as *const u8 as *const i8, 1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn skip_junk(mut expect: i32) {
    lastTokenLinenumber = linenumber;
    while pos.is_null() || *pos == 0
        || !(strchr(b" #\n\t\0" as *const u8 as *const i8, *pos as i32)).is_null()
    {
        if pos.is_null() || *pos == 0 || *pos as i32 == '#' as i32 {
            if mtools_getline() != 0 {
                pos = 0 as *mut i8;
                if expect != 0 {
                    syntax(
                        b"end of file unexpected\0" as *const u8 as *const i8,
                        1 as i32,
                    );
                }
                return;
            }
        } else {
            pos = pos.offset(1);
            pos;
        }
    }
    token_nr += 1;
    token_nr;
}
unsafe extern "C" fn get_next_token() -> *mut i8 {
    skip_junk(0 as i32);
    if pos.is_null() {
        token_length = 0 as i32 as size_t;
        token = 0 as *mut i8;
        return 0 as *mut i8;
    }
    token = pos;
    token_length = strcspn(token, b" \t\n#:=\0" as *const u8 as *const i8);
    pos = pos.offset(token_length as isize);
    return token;
}
unsafe extern "C" fn match_token(mut template: *const i8) -> i32 {
    return (strlen(template) == token_length
        && cmp_tok(template, token, token_length) == 0) as i32;
}
unsafe extern "C" fn expect_char(mut c: i8) {
    let mut buf: [i8; 11] = [0; 11];
    skip_junk(1 as i32);
    if *pos as i32 != c as i32 {
        sprintf(buf.as_mut_ptr(), b"expected %c\0" as *const u8 as *const i8, c as i32);
        syntax(buf.as_mut_ptr(), 1 as i32);
    }
    pos = pos.offset(1);
    pos;
}
unsafe extern "C" fn get_string() -> *mut i8 {
    let mut end: *mut i8 = 0 as *mut i8;
    let mut str: *mut i8 = 0 as *mut i8;
    skip_junk(1 as i32);
    if *pos as i32 != '"' as i32 {
        syntax(b" \" expected\0" as *const u8 as *const i8, 0 as i32);
    }
    str = pos.offset(1 as i32 as isize);
    end = strchr(str, '"' as i32);
    if end.is_null() {
        syntax(b"unterminated string constant\0" as *const u8 as *const i8, 1 as i32);
    }
    str = strndup(str, ptrdiff(end, str));
    pos = end.offset(1 as i32 as isize);
    return str;
}
unsafe extern "C" fn get_unquoted_string() -> *mut i8 {
    if *pos as i32 == '"' as i32 {
        return get_string()
    } else {
        let mut str: *mut i8 = get_next_token();
        return strndup(str, token_length);
    };
}
unsafe extern "C" fn get_unumber(mut max: u64) -> u64 {
    let mut last: *mut i8 = 0 as *mut i8;
    let mut n: u64 = 0;
    skip_junk(1 as i32);
    last = pos;
    n = strtoul(pos, &mut pos, 0 as i32);
    if *__errno_location() != 0 {
        syntax(b"bad number\0" as *const u8 as *const i8, 0 as i32);
    }
    if last == pos {
        syntax(b"numeral expected\0" as *const u8 as *const i8, 0 as i32);
    }
    if n > max {
        syntax(b"number too big\0" as *const u8 as *const i8, 0 as i32);
    }
    pos = pos.offset(1);
    pos;
    token_nr += 1;
    token_nr;
    return n;
}
unsafe extern "C" fn get_number() -> i32 {
    let mut last: *mut i8 = 0 as *mut i8;
    let mut n: i32 = 0;
    skip_junk(1 as i32);
    last = pos;
    n = strtol(pos, &mut pos, 0 as i32) as i32;
    if *__errno_location() != 0 {
        syntax(b"bad number\0" as *const u8 as *const i8, 0 as i32);
    }
    if last == pos {
        syntax(b"numeral expected\0" as *const u8 as *const i8, 0 as i32);
    }
    pos = pos.offset(1);
    pos;
    token_nr += 1;
    token_nr;
    return n;
}
unsafe extern "C" fn get_offset() -> mt_off_t {
    let mut last: *mut i8 = 0 as *mut i8;
    let mut n: mt_off_t = 0;
    skip_junk(1 as i32);
    last = pos;
    n = str_to_off_with_end(pos, &mut pos);
    if *__errno_location() != 0 {
        syntax(b"bad number\0" as *const u8 as *const i8, 0 as i32);
    }
    if last == pos {
        syntax(b"numeral expected\0" as *const u8 as *const i8, 0 as i32);
    }
    pos = pos.offset(1);
    pos;
    token_nr += 1;
    token_nr;
    return n;
}
unsafe extern "C" fn purge(mut drive: i8, mut fn_0: i32) {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    drive = ch_canon_drv(drive);
    j = 0 as i32 as u32;
    i = 0 as i32 as u32;
    while i < cur_devs {
        if (*devices.offset(i as isize)).drive as i32 != drive as i32
            || (*devices.offset(i as isize)).file_nr == fn_0
        {
            let fresh1 = j;
            j = j.wrapping_add(1);
            *devices.offset(fresh1 as isize) = *devices.offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    cur_devs = j;
}
unsafe extern "C" fn grow() {
    if cur_devs >= nr_dev.wrapping_sub(2 as i32 as u32) {
        nr_dev = cur_devs.wrapping_add(2 as i32 as u32) << 1 as i32;
        devices = realloc(
            devices as *mut i8 as *mut libc::c_void,
            (nr_dev as u64).wrapping_mul(::core::mem::size_of::<device>() as u64),
        ) as *mut device;
        if devices.is_null() {
            printOom();
            exit(1 as i32);
        }
    }
}
unsafe extern "C" fn init_drive() {
    memset(
        &mut *devices.offset(cur_dev as isize) as *mut device as *mut i8
            as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<device>() as u64,
    );
    (*devices.offset(cur_dev as isize)).ssize = 2 as i32 as uint8_t;
}
unsafe extern "C" fn prepend() {
    let mut i: u32 = 0;
    grow();
    i = cur_devs;
    while i > 0 as i32 as u32 {
        *devices.offset(i as isize) = *devices
            .offset(i.wrapping_sub(1 as i32 as u32) as isize);
        i = i.wrapping_sub(1);
        i;
    }
    cur_dev = 0 as i32;
    cur_devs = cur_devs.wrapping_add(1);
    cur_devs;
    init_drive();
}
unsafe extern "C" fn append() {
    grow();
    cur_dev = cur_devs as i32;
    cur_devs = cur_devs.wrapping_add(1);
    cur_devs;
    init_drive();
}
unsafe extern "C" fn finish_drive_clause() {
    if cur_dev == -(1 as i32) {
        trusted = 0 as i32;
        return;
    }
    if ((*devices.offset(cur_dev as isize)).name).is_null() {
        syntax(b"missing filename\0" as *const u8 as *const i8, 0 as i32);
    }
    if (*devices.offset(cur_dev as isize)).tracks != 0
        || (*devices.offset(cur_dev as isize)).heads as i32 != 0
        || (*devices.offset(cur_dev as isize)).sectors as i32 != 0
    {
        if (*devices.offset(cur_dev as isize)).tracks == 0
            || (*devices.offset(cur_dev as isize)).heads == 0
            || (*devices.offset(cur_dev as isize)).sectors == 0
        {
            syntax(
                b"incomplete geometry: either indicate all of track/heads/sectors or none of them\0"
                    as *const u8 as *const i8,
                0 as i32,
            );
        }
        if (*devices.offset(cur_dev as isize)).misc_flags & (0x10 as u32 | 0x80 as u32)
            == 0
        {
            syntax(
                b"if you supply a geometry, you also must supply one of the `mformat_only' or `filter' flags\0"
                    as *const u8 as *const i8,
                0 as i32,
            );
        }
    }
    (*devices.offset(cur_dev as isize)).file_nr = file_nr;
    let ref mut fresh2 = (*devices.offset(cur_dev as isize)).cfg_filename;
    *fresh2 = filename;
    if trusted == 0 && (*devices.offset(cur_dev as isize)).misc_flags & 0x2 as u32 != 0 {
        fprintf(
            stderr,
            b"Warning: privileged flag ignored for drive %c: defined in file %s\n\0"
                as *const u8 as *const i8,
            canon_drv((*devices.offset(cur_dev as isize)).drive as i32),
            filename,
        );
        (*devices.offset(cur_dev as isize)).misc_flags &= !(0x2 as u32);
    }
    trusted = 0 as i32;
    cur_dev = -(1 as i32);
}
unsafe extern "C" fn set_var(
    mut switches: *mut switches_l,
    mut nr: i32,
    mut base_address: caddr_t,
) -> i32 {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < nr {
        if match_token((*switches.offset(i as isize)).name) != 0 {
            expect_char('=' as i32 as i8);
            if (*switches.offset(i as isize)).type_0 as u32 == T_UINT as i32 as u32 {
                *(base_address
                    .offset((*switches.offset(i as isize)).address as i64 as isize)
                    as *mut u32) = get_unumber(
                    (2147483647 as i32 as u32)
                        .wrapping_mul(2 as u32)
                        .wrapping_add(1 as u32) as u64,
                ) as u32;
            } else if (*switches.offset(i as isize)).type_0 as u32
                == T_UINT8 as i32 as u32
            {
                *(base_address
                    .offset((*switches.offset(i as isize)).address as i64 as isize)
                    as *mut uint8_t) = get_unumber(255 as i32 as u64) as uint8_t;
            } else if (*switches.offset(i as isize)).type_0 as u32
                == T_UINT16 as i32 as u32
            {
                *(base_address
                    .offset((*switches.offset(i as isize)).address as i64 as isize)
                    as *mut uint16_t) = get_unumber(65535 as i32 as u64) as uint16_t;
            } else if (*switches.offset(i as isize)).type_0 as u32 == T_INT as i32 as u32
            {
                *(base_address
                    .offset((*switches.offset(i as isize)).address as i64 as isize)
                    as *mut i32) = get_number();
            } else if (*switches.offset(i as isize)).type_0 as u32
                == T_STRING as i32 as u32
            {
                let ref mut fresh3 = *(base_address
                    .offset((*switches.offset(i as isize)).address as i64 as isize)
                    as *mut *mut i8);
                *fresh3 = get_string();
            } else if (*switches.offset(i as isize)).type_0 as u32
                == T_UQSTRING as i32 as u32
            {
                let ref mut fresh4 = *(base_address
                    .offset((*switches.offset(i as isize)).address as i64 as isize)
                    as *mut *mut i8);
                *fresh4 = get_unquoted_string();
            } else if (*switches.offset(i as isize)).type_0 as u32
                == T_OFFS as i32 as u32
            {
                *(base_address
                    .offset((*switches.offset(i as isize)).address as i64 as isize)
                    as *mut mt_off_t) = get_offset();
            }
            return 0 as i32;
        }
        i += 1;
        i;
    }
    return 1 as i32;
}
unsafe extern "C" fn set_openflags(mut dev: *mut device) -> i32 {
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[flags_t; 4]>() as u64)
            .wrapping_div(::core::mem::size_of::<flags_t>() as u64)
    {
        if match_token(openflags[i as usize].name) != 0 {
            (*dev).mode = ((*dev).mode as u32 | openflags[i as usize].flag) as i32;
            return 0 as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
unsafe extern "C" fn set_misc_flags(mut dev: *mut device) -> i32 {
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[flags_t; 9]>() as u64)
            .wrapping_div(::core::mem::size_of::<flags_t>() as u64)
    {
        if match_token(misc_flags[i as usize].name) != 0 {
            flag_mask |= misc_flags[i as usize].flag;
            skip_junk(0 as i32);
            if !pos.is_null() && *pos as i32 == '=' as i32 {
                pos = pos.offset(1);
                pos;
                match get_number() {
                    0 => return 0 as i32,
                    1 => {}
                    _ => {
                        syntax(b"expected 0 or 1\0" as *const u8 as *const i8, 0 as i32);
                    }
                }
            }
            (*dev).misc_flags |= misc_flags[i as usize].flag;
            return 0 as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
unsafe extern "C" fn set_def_format(mut dev: *mut device) -> i32 {
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[C2RustUnnamed_0; 15]>() as u64)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as u64)
    {
        if match_token(default_formats[i as usize].name) != 0 {
            if (*dev).ssize == 0 {
                (*dev).ssize = 2 as i32 as uint8_t;
            }
            if (*dev).tracks == 0 {
                (*dev).tracks = default_formats[i as usize].tracks;
            }
            if (*dev).heads == 0 {
                (*dev).heads = default_formats[i as usize].heads;
            }
            if (*dev).sectors == 0 {
                (*dev).sectors = default_formats[i as usize].sectors;
            }
            if (*dev).fat_bits == 0 {
                (*dev).fat_bits = default_formats[i as usize].fat_bits as i32;
            }
            return 0 as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn set_cmd_line_image(mut img: *mut i8) {
    let mut ofsp: *mut i8 = 0 as *mut i8;
    prepend();
    (*devices.offset(cur_dev as isize)).drive = ':' as i32 as i8;
    default_drive = ':' as i32 as i8;
    ofsp = strstr(img, b"@@\0" as *const u8 as *const i8);
    if ofsp.is_null() {
        let ref mut fresh5 = (*devices.offset(cur_dev as isize)).name;
        *fresh5 = strdup(img);
        (*devices.offset(cur_dev as isize)).offset = 0 as i32 as mt_off_t;
    } else {
        let ref mut fresh6 = (*devices.offset(cur_dev as isize)).name;
        *fresh6 = strndup(img, ptrdiff(ofsp, img));
        (*devices.offset(cur_dev as isize)).offset = str_to_offset(
            ofsp.offset(2 as i32 as isize),
        );
    }
    (*devices.offset(cur_dev as isize)).fat_bits = 0 as i32;
    (*devices.offset(cur_dev as isize)).tracks = 0 as i32 as u32;
    (*devices.offset(cur_dev as isize)).heads = 0 as i32 as uint16_t;
    (*devices.offset(cur_dev as isize)).sectors = 0 as i32 as uint16_t;
    if !(strchr((*devices.offset(cur_dev as isize)).name, '|' as i32)).is_null() {
        let mut pipechar: *mut i8 = strchr(
            (*devices.offset(cur_dev as isize)).name,
            '|' as i32,
        );
        *pipechar = 0 as i32 as i8;
        strncpy(
            buffer.as_mut_ptr(),
            pipechar.offset(1 as i32 as isize),
            256 as i32 as u64,
        );
        buffer[256 as i32 as usize] = '\0' as i32 as i8;
        fp = 0 as *mut FILE;
        filename = b"{command line}\0" as *const u8 as *const i8;
        linenumber = 0 as i32;
        lastTokenLinenumber = 0 as i32;
        pos = buffer.as_mut_ptr();
        token = 0 as *mut i8;
        parse_all(0 as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn check_number_parse_errno(
    mut c: i8,
    mut oarg: *const i8,
    mut endptr: *mut i8,
) {
    if !endptr.is_null() && *endptr as i32 != 0 {
        fprintf(stderr, b"Bad number %s\n\0" as *const u8 as *const i8, oarg);
        exit(1 as i32);
    }
    if *__errno_location() != 0 {
        fprintf(
            stderr,
            b"Bad number %s for -%c (%s)\n\0" as *const u8 as *const i8,
            oarg,
            c as i32,
            strerror(*__errno_location()),
        );
        exit(1 as i32);
    }
}
unsafe extern "C" fn tou16(mut in_0: i32, mut comment: *const i8) -> uint16_t {
    if in_0 > 65535 as i32 {
        fprintf(
            stderr,
            b"Number of %s %d too big\n\0" as *const u8 as *const i8,
            comment,
            in_0,
        );
        exit(1 as i32);
    }
    if in_0 < 0 as i32 {
        fprintf(
            stderr,
            b"Number of %s %d negative\n\0" as *const u8 as *const i8,
            comment,
            in_0,
        );
        exit(1 as i32);
    }
    return in_0 as uint16_t;
}
unsafe extern "C" fn parse_old_device_line(mut drive: i8) {
    let mut name: [i8; 4096] = [0; 4096];
    let mut items: i32 = 0;
    let mut offset: mt_off_t = 0;
    let mut heads: i32 = 0;
    let mut sectors: i32 = 0;
    let mut tracks: i32 = 0;
    finish_drive_clause();
    purge(drive, file_nr);
    append();
    items = sscanf(
        token,
        b"%c %s %i %i %i %i %li\0" as *const u8 as *const i8,
        &mut (*devices.offset(cur_dev as isize)).drive as *mut i8,
        name.as_mut_ptr(),
        &mut (*devices.offset(cur_dev as isize)).fat_bits as *mut i32,
        &mut tracks as *mut i32,
        &mut heads as *mut i32,
        &mut sectors as *mut i32,
        &mut offset as *mut mt_off_t,
    );
    (*devices.offset(cur_dev as isize)).heads = tou16(
        heads,
        b"heads\0" as *const u8 as *const i8,
    );
    (*devices.offset(cur_dev as isize)).sectors = tou16(
        sectors,
        b"sectors\0" as *const u8 as *const i8,
    );
    (*devices.offset(cur_dev as isize)).tracks = tracks as u32;
    (*devices.offset(cur_dev as isize)).offset = offset;
    let mut current_block_13: u64;
    match items {
        2 => {
            (*devices.offset(cur_dev as isize)).fat_bits = 0 as i32;
            current_block_13 = 7764823385890549880;
        }
        3 => {
            current_block_13 = 7764823385890549880;
        }
        6 => {
            current_block_13 = 12065147691658613968;
        }
        0 | 1 | 4 | 5 => {
            syntax(b"bad number of parameters\0" as *const u8 as *const i8, 1 as i32);
        }
        _ => {
            current_block_13 = 13586036798005543211;
        }
    }
    match current_block_13 {
        7764823385890549880 => {
            (*devices.offset(cur_dev as isize)).sectors = 0 as i32 as uint16_t;
            (*devices.offset(cur_dev as isize)).heads = 0 as i32 as uint16_t;
            (*devices.offset(cur_dev as isize)).tracks = 0 as i32 as u32;
            current_block_13 = 12065147691658613968;
        }
        _ => {}
    }
    match current_block_13 {
        12065147691658613968 => {
            (*devices.offset(cur_dev as isize)).offset = 0 as i32 as mt_off_t;
        }
        _ => {}
    }
    if (*devices.offset(cur_dev as isize)).tracks == 0 {
        (*devices.offset(cur_dev as isize)).sectors = 0 as i32 as uint16_t;
        (*devices.offset(cur_dev as isize)).heads = 0 as i32 as uint16_t;
    }
    (*devices.offset(cur_dev as isize)).drive = ch_canon_drv(
        (*devices.offset(cur_dev as isize)).drive,
    );
    maintain_default_drive((*devices.offset(cur_dev as isize)).drive);
    let ref mut fresh7 = (*devices.offset(cur_dev as isize)).name;
    *fresh7 = strdup(name.as_mut_ptr());
    if (*fresh7).is_null() {
        printOom();
        exit(1 as i32);
    }
    (*devices.offset(cur_dev as isize)).misc_flags |= 0x10 as u32;
    finish_drive_clause();
    pos = 0 as *mut i8;
}
unsafe extern "C" fn parse_one(mut privilege: i32) -> i32 {
    let mut action: i32 = 0 as i32;
    get_next_token();
    if token.is_null() {
        return 0 as i32;
    }
    if match_token(b"drive\0" as *const u8 as *const i8) != 0
        && {
            action = 1 as i32;
            action != 0
        }
        || match_token(b"drive+\0" as *const u8 as *const i8) != 0
            && {
                action = 2 as i32;
                action != 0
            }
        || match_token(b"+drive\0" as *const u8 as *const i8) != 0
            && {
                action = 3 as i32;
                action != 0
            }
        || match_token(b"clear_drive\0" as *const u8 as *const i8) != 0
            && {
                action = 4 as i32;
                action != 0
            }
    {
        finish_drive_clause();
        get_next_token();
        if token_length != 1 as i32 as u64 {
            syntax(b"drive letter expected\0" as *const u8 as *const i8, 0 as i32);
        }
        if action == 1 as i32 || action == 4 as i32 {
            purge(*token.offset(0 as i32 as isize), file_nr);
        }
        if action == 4 as i32 {
            return 1 as i32;
        }
        if action == 3 as i32 {
            prepend();
        } else {
            append();
        }
        memset(
            devices.offset(cur_dev as isize) as *mut i8 as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<device>() as u64,
        );
        trusted = privilege;
        flag_mask = 0 as i32 as u32;
        (*devices.offset(cur_dev as isize)).drive = ch_canon_drv(
            *token.offset(0 as i32 as isize),
        );
        maintain_default_drive((*devices.offset(cur_dev as isize)).drive);
        expect_char(':' as i32 as i8);
        return 1 as i32;
    }
    if token_nr == 1 as i32 && token_length == 1 as i32 as u64 {
        parse_old_device_line(*token.offset(0 as i32 as isize));
        return 1 as i32;
    }
    if (cur_dev < 0 as i32
        || set_var(
            dswitches.as_mut_ptr(),
            (::core::mem::size_of::<[switches_t; 16]>() as u64)
                .wrapping_div(::core::mem::size_of::<switches_t>() as u64) as i32,
            &mut *devices.offset(cur_dev as isize) as *mut device as caddr_t,
        ) != 0 && set_openflags(&mut *devices.offset(cur_dev as isize)) != 0
            && set_misc_flags(&mut *devices.offset(cur_dev as isize)) != 0
            && set_def_format(&mut *devices.offset(cur_dev as isize)) != 0)
        && set_var(
            global_switches.as_mut_ptr(),
            (::core::mem::size_of::<[switches_t; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<switches_t>() as u64) as i32,
            0 as caddr_t,
        ) != 0
    {
        syntax(b"unrecognized keyword\0" as *const u8 as *const i8, 1 as i32);
    }
    return 1 as i32;
}
unsafe extern "C" fn parse_all(mut privilege: i32) {
    *__errno_location() = 0 as i32;
    while parse_one(privilege) != 0 {}
}
unsafe extern "C" fn parse(mut name: *const i8, mut privilege: i32) -> i32 {
    if !fp.is_null() {
        fprintf(stderr, b"File descriptor already set!\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    fp = fopen(name, b"r\0" as *const u8 as *const i8);
    if fp.is_null() {
        return 0 as i32;
    }
    file_nr += 1;
    file_nr;
    filename = name;
    linenumber = 0 as i32;
    lastTokenLinenumber = 0 as i32;
    pos = 0 as *mut i8;
    token = 0 as *mut i8;
    cur_dev = -(1 as i32);
    parse_all(privilege);
    finish_drive_clause();
    fclose(fp);
    filename = 0 as *const i8;
    fp = 0 as *mut FILE;
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn read_config() {
    let mut homedir: *mut i8 = 0 as *mut i8;
    let mut envConfFile: *mut i8 = 0 as *mut i8;
    static mut conf_file: [i8; 4107] = [0; 4107];
    file_nr = 0 as i32;
    cur_devs = nr_const_devices;
    nr_dev = nr_const_devices.wrapping_add(2 as i32 as u32);
    devices = calloc(nr_dev as u64, ::core::mem::size_of::<device>() as u64)
        as *mut device;
    if devices.is_null() {
        printOom();
        exit(1 as i32);
    }
    if nr_const_devices != 0 {
        memcpy(
            devices as *mut libc::c_void,
            const_devices.as_mut_ptr() as *const libc::c_void,
            (nr_const_devices as u64)
                .wrapping_mul(::core::mem::size_of::<device>() as u64),
        );
    }
    (parse(b"/etc/mtools.conf\0" as *const u8 as *const i8, 1 as i32)
        | parse(b"/etc/default/mtools.conf\0" as *const u8 as *const i8, 1 as i32)
        | parse(b"/usr/local/etc/mtools.conf\0" as *const u8 as *const i8, 1 as i32) != 0
        || parse(b"/etc/mtools\0" as *const u8 as *const i8, 1 as i32)
            | parse(b"/etc/default/mtools\0" as *const u8 as *const i8, 1 as i32) != 0)
        as i32;
    homedir = get_homedir();
    if !homedir.is_null() {
        strncpy(conf_file.as_mut_ptr(), homedir, 4096 as i32 as u64);
        conf_file[4096 as i32 as usize] = '\0' as i32 as i8;
        strcat(conf_file.as_mut_ptr(), b"/.mtoolsrc\0" as *const u8 as *const i8);
        parse(conf_file.as_mut_ptr(), 0 as i32);
    }
    memset(
        &mut *devices.offset(cur_devs as isize) as *mut device as *mut i8
            as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<device>() as u64,
    );
    envConfFile = getenv(b"MTOOLSRC\0" as *const u8 as *const i8);
    if !envConfFile.is_null() {
        parse(envConfFile, 0 as i32);
    }
    get_env_conf();
    if mtools_skip_check != 0 {
        mtools_fat_compatibility = 1 as i32 as u32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mtoolstest(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut type_0: i32,
) {
    let mut dev: *mut device = 0 as *mut device;
    let mut drive: i8 = '\0' as i32 as i8;
    if argc > 1 as i32
        && *(*argv.offset(1 as i32 as isize)).offset(0 as i32 as isize) as i32 != 0
        && *(*argv.offset(1 as i32 as isize)).offset(1 as i32 as isize) as i32
            == ':' as i32
    {
        drive = ch_canon_drv(
            *(*argv.offset(1 as i32 as isize)).offset(0 as i32 as isize),
        );
    }
    dev = devices;
    while !((*dev).name).is_null() {
        if !(drive as i32 != 0 && drive as i32 != (*dev).drive as i32) {
            printf(b"drive %c:\n\0" as *const u8 as *const i8, (*dev).drive as i32);
            printf(
                b"\t#fn=%d mode=%d \0" as *const u8 as *const i8,
                (*dev).file_nr,
                (*dev).mode,
            );
            if !((*dev).cfg_filename).is_null() {
                printf(
                    b"defined in %s\n\0" as *const u8 as *const i8,
                    (*dev).cfg_filename,
                );
            } else {
                printf(b"builtin\n\0" as *const u8 as *const i8);
            }
            printf(
                b"\tfile=\"%s\" fat_bits=%d \n\0" as *const u8 as *const i8,
                (*dev).name,
                (*dev).fat_bits,
            );
            printf(
                b"\ttracks=%d heads=%d sectors=%d hidden=%d\n\0" as *const u8
                    as *const i8,
                (*dev).tracks,
                (*dev).heads as i32,
                (*dev).sectors as i32,
                (*dev).hidden,
            );
            printf(b"\toffset=%ld\n\0" as *const u8 as *const i8, (*dev).offset);
            printf(b"\tpartition=%d\n\0" as *const u8 as *const i8, (*dev).partition);
            if (*dev).misc_flags != 0 {
                printf(b"\t\0" as *const u8 as *const i8);
            }
            if !dev.is_null() && (*dev).misc_flags & 0x100 as u32 != 0 {
                printf(b"swap \0" as *const u8 as *const i8);
            }
            if !dev.is_null() && (*dev).misc_flags & 0x1 as u32 != 0 {
                printf(b"scsi \0" as *const u8 as *const i8);
            }
            if !dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0 {
                printf(b"privileged\0" as *const u8 as *const i8);
            }
            if !dev.is_null() && (*dev).misc_flags & 0x10 as u32 != 0 {
                printf(b"mformat_only \0" as *const u8 as *const i8);
            }
            if !dev.is_null() && (*dev).misc_flags & 0x20 as u32 != 0 {
                printf(b"vold \0" as *const u8 as *const i8);
            }
            if !dev.is_null() && (*dev).misc_flags & 0x8 as u32 != 0 {
                printf(b"use_xdf \0" as *const u8 as *const i8);
            }
            if (*dev).misc_flags != 0 {
                printf(b"\n\0" as *const u8 as *const i8);
            }
            if (*dev).mode != 0 {
                printf(b"\t\0" as *const u8 as *const i8);
            }
            if (*dev).mode & 0o4010000 as i32 != 0 {
                printf(b"sync \0" as *const u8 as *const i8);
            }
            if (*dev).mode & 0o4000 as i32 != 0 {
                printf(b"nodelay \0" as *const u8 as *const i8);
            }
            if (*dev).mode & 0o200 as i32 != 0 {
                printf(b"exclusive \0" as *const u8 as *const i8);
            }
            if (*dev).mode != 0 {
                printf(b"\n\0" as *const u8 as *const i8);
            }
            if !((*dev).precmd).is_null() {
                printf(b"\tprecmd=%s\n\0" as *const u8 as *const i8, (*dev).precmd);
            }
            printf(b"\n\0" as *const u8 as *const i8);
        }
        dev = dev.offset(1);
        dev;
    }
    printf(
        b"mtools_fat_compatibility=%d\n\0" as *const u8 as *const i8,
        mtools_fat_compatibility,
    );
    printf(b"mtools_skip_check=%d\n\0" as *const u8 as *const i8, mtools_skip_check);
    printf(
        b"mtools_lower_case=%d\n\0" as *const u8 as *const i8,
        mtools_ignore_short_case,
    );
    exit(0 as i32);
}
unsafe extern "C" fn run_static_initializers() {
    dswitches = [
        {
            let mut init = switches_l {
                name: b"FILE\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).name as *mut *const i8 as caddr_t,
                type_0: T_STRING,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"OFFSET\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).offset as *mut mt_off_t as caddr_t,
                type_0: T_OFFS,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"PARTITION\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).partition as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"FAT\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).fat_bits as *mut i32 as caddr_t,
                type_0: T_INT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"FAT_BITS\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).fat_bits as *mut i32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MODE\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).mode as *mut i32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"TRACKS\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).tracks as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"CYLINDERS\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).tracks as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"HEADS\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).heads as *mut uint16_t as caddr_t,
                type_0: T_UINT16,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"SECTORS\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).sectors as *mut uint16_t as caddr_t,
                type_0: T_UINT16,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"HIDDEN\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).hidden as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"PRECMD\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).precmd as *mut *mut i8 as caddr_t,
                type_0: T_STRING,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"POSTCMD\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).postcmd as *mut *mut i8 as caddr_t,
                type_0: T_STRING,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"BLOCKSIZE\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).blocksize as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"CODEPAGE\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).codepage as *mut u32 as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"DATA_MAP\0" as *const u8 as *const i8,
                address: &mut (*(0 as *mut device)).data_map as *mut *const i8
                    as caddr_t,
                type_0: T_UQSTRING,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];