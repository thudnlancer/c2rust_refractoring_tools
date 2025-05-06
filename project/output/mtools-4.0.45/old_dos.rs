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
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type off_t = __off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OldDos_t {
    pub tracks: u32,
    pub sectors: uint16_t,
    pub heads: uint16_t,
    pub dir_len: uint16_t,
    pub cluster_size: uint8_t,
    pub fat_len: uint32_t,
    pub media: uint8_t,
}
static mut old_dos: [OldDos_t; 11] = [
    {
        let mut init = OldDos_t {
            tracks: 40 as i32 as u32,
            sectors: 9 as i32 as uint16_t,
            heads: 1 as i32 as uint16_t,
            dir_len: 4 as i32 as uint16_t,
            cluster_size: 1 as i32 as uint8_t,
            fat_len: 2 as i32 as uint32_t,
            media: 0xfc as i32 as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 40 as i32 as u32,
            sectors: 9 as i32 as uint16_t,
            heads: 2 as i32 as uint16_t,
            dir_len: 7 as i32 as uint16_t,
            cluster_size: 2 as i32 as uint8_t,
            fat_len: 2 as i32 as uint32_t,
            media: 0xfd as i32 as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 40 as i32 as u32,
            sectors: 8 as i32 as uint16_t,
            heads: 1 as i32 as uint16_t,
            dir_len: 4 as i32 as uint16_t,
            cluster_size: 1 as i32 as uint8_t,
            fat_len: 1 as i32 as uint32_t,
            media: 0xfe as i32 as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 40 as i32 as u32,
            sectors: 8 as i32 as uint16_t,
            heads: 2 as i32 as uint16_t,
            dir_len: 7 as i32 as uint16_t,
            cluster_size: 2 as i32 as uint8_t,
            fat_len: 1 as i32 as uint32_t,
            media: 0xff as i32 as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as i32 as u32,
            sectors: 9 as i32 as uint16_t,
            heads: 2 as i32 as uint16_t,
            dir_len: 7 as i32 as uint16_t,
            cluster_size: 2 as i32 as uint8_t,
            fat_len: 3 as i32 as uint32_t,
            media: 0xf9 as i32 as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as i32 as u32,
            sectors: 15 as i32 as uint16_t,
            heads: 2 as i32 as uint16_t,
            dir_len: 14 as i32 as uint16_t,
            cluster_size: 1 as i32 as uint8_t,
            fat_len: 7 as i32 as uint32_t,
            media: 0xf9 as i32 as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as i32 as u32,
            sectors: 18 as i32 as uint16_t,
            heads: 2 as i32 as uint16_t,
            dir_len: 14 as i32 as uint16_t,
            cluster_size: 1 as i32 as uint8_t,
            fat_len: 9 as i32 as uint32_t,
            media: 0xf0 as i32 as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as i32 as u32,
            sectors: 36 as i32 as uint16_t,
            heads: 2 as i32 as uint16_t,
            dir_len: 15 as i32 as uint16_t,
            cluster_size: 2 as i32 as uint8_t,
            fat_len: 9 as i32 as uint32_t,
            media: 0xf0 as i32 as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as i32 as u32,
            sectors: 8 as i32 as uint16_t,
            heads: 2 as i32 as uint16_t,
            dir_len: 7 as i32 as uint16_t,
            cluster_size: 2 as i32 as uint8_t,
            fat_len: 2 as i32 as uint32_t,
            media: 0xfb as i32 as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as i32 as u32,
            sectors: 8 as i32 as uint16_t,
            heads: 1 as i32 as uint16_t,
            dir_len: 7 as i32 as uint16_t,
            cluster_size: 2 as i32 as uint8_t,
            fat_len: 2 as i32 as uint32_t,
            media: 0xfa as i32 as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as i32 as u32,
            sectors: 9 as i32 as uint16_t,
            heads: 1 as i32 as uint16_t,
            dir_len: 7 as i32 as uint16_t,
            cluster_size: 2 as i32 as uint8_t,
            fat_len: 2 as i32 as uint32_t,
            media: 0xf8 as i32 as uint8_t,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn getOldDosBySize(mut size: size_t) -> *mut OldDos_t {
    let mut i: size_t = 0;
    size = size.wrapping_mul(2 as i32 as u64);
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[OldDos_t; 11]>() as u64)
            .wrapping_div(::core::mem::size_of::<OldDos_t>() as u64)
    {
        if (old_dos[i as usize].sectors as u32)
            .wrapping_mul(old_dos[i as usize].tracks)
            .wrapping_mul(old_dos[i as usize].heads as u32) as u64 == size
        {
            return &mut *old_dos.as_mut_ptr().offset(i as isize) as *mut OldDos_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut OldDos_t;
}
#[no_mangle]
pub unsafe extern "C" fn getOldDosByMedia(mut media: i32) -> *mut OldDos_t {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[OldDos_t; 11]>() as u64)
            .wrapping_div(::core::mem::size_of::<OldDos_t>() as u64)
    {
        if old_dos[i as usize].media as i32 == media {
            return &mut *old_dos.as_mut_ptr().offset(i as isize) as *mut OldDos_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    fprintf(stderr, b"Unknown media type %02x\n\0" as *const u8 as *const i8, media);
    return 0 as *mut OldDos_t;
}
#[no_mangle]
pub unsafe extern "C" fn getOldDosByParams(
    mut tracks: u32,
    mut heads: u32,
    mut sectors: u32,
    mut dir_len: u32,
    mut cluster_size: u32,
) -> *mut OldDos_t {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[OldDos_t; 11]>() as u64)
            .wrapping_div(::core::mem::size_of::<OldDos_t>() as u64)
    {
        if sectors == old_dos[i as usize].sectors as u32
            && tracks == old_dos[i as usize].tracks
            && heads == old_dos[i as usize].heads as u32
            && (dir_len == 0 as i32 as u32
                || dir_len == old_dos[i as usize].dir_len as u32)
            && (cluster_size == 0 as i32 as u32
                || cluster_size == old_dos[i as usize].cluster_size as u32)
        {
            return &mut *old_dos.as_mut_ptr().offset(i as isize) as *mut OldDos_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut OldDos_t;
}
#[no_mangle]
pub unsafe extern "C" fn setDeviceFromOldDos(
    mut media: i32,
    mut dev: *mut device,
) -> i32 {
    let mut params: *mut OldDos_t = getOldDosByMedia(media);
    if params.is_null() {
        return -(1 as i32);
    }
    (*dev).heads = (*params).heads;
    (*dev).tracks = (*params).tracks;
    (*dev).sectors = (*params).sectors;
    (*dev).ssize = 0x80 as i32 as uint8_t;
    (*dev).use_2m = !(1 as u32);
    return 0 as i32;
}