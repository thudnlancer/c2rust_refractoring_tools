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
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type inode_val;
    fn stat_error(_: *const i8);
    fn readlink_warn(_: *const i8);
    fn open_error(_: *const i8);
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn readlink(__path: *const i8, __buf: *mut i8, __len: size_t) -> ssize_t;
    fn abort() -> !;
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn dcngettext(
        __domainname: *const i8,
        __msgid1: *const i8,
        __msgid2: *const i8,
        __n: u64,
        __category: i32,
    ) -> *mut i8;
    fn close_error(_: *const i8);
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn stpcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn cpio_file_stat_free(file_hdr: *mut cpio_file_stat);
    fn cpio_set_c_name(file_hdr: *mut cpio_file_stat, name: *mut i8);
    fn ds_free(string: *mut dynamic_string);
    fn ds_fgetstr(f: *mut FILE, s: *mut dynamic_string, eos: i8) -> *mut i8;
    fn ds_append(s: *mut dynamic_string, c: i32);
    fn ds_endswith(s: *mut dynamic_string, c: i32) -> i32;
    static mut quiet_flag: i32;
    static mut no_abs_paths_flag: i32;
    static mut warn_option: u32;
    static mut ignore_dirnlink_option: i32;
    static mut archive_des: i32;
    static mut output_size: size_t;
    static mut name_end: i8;
    static mut output_is_special: i8;
    static mut output_is_seekable: i8;
    static mut xstat: Option<unsafe extern "C" fn() -> i32>;
    fn process_copy_in();
    static mut append_flag: i32;
    static mut output_bytes: off_t;
    static mut dot_flag: i32;
    static mut verbose_flag: i32;
    static mut io_block_size: i32;
    static mut reset_time_flag: i32;
    static mut archive_format: archive_format;
    fn umaxtostr(_: uintmax_t, _: *mut i8) -> *mut i8;
    fn warn_if_file_changed(
        file_name: *mut i8,
        old_file_size: off_t,
        old_file_mtime: time_t,
    );
    fn copy_files_disk_to_tape(
        in_des: i32,
        out_des: i32,
        num_bytes: off_t,
        filename: *mut i8,
    );
    fn add_inode(
        node_num: ino_t,
        file_name: *mut i8,
        major_num: u64,
        minor_num: u64,
    ) -> *mut inode_val;
    fn tape_empty_output_buffer(out_des: i32);
    fn find_inode_file(node_num: ino_t, major_num: u64, minor_num: u64) -> *mut i8;
    fn tape_buffered_write(in_buf: *mut i8, out_des: i32, num_bytes: off_t);
    fn prepare_append(out_file_des: i32);
    fn write_out_tar_header(file_hdr: *mut cpio_file_stat, out_des: i32) -> i32;
    fn is_tar_filename_too_long(name: *mut i8) -> i32;
    fn write_nuls_to_file(
        num_bytes: off_t,
        out_des: i32,
        writer: Option<unsafe extern "C" fn(*mut i8, i32, off_t) -> ()>,
    );
    fn set_file_times(fd: i32, name: *const i8, atime: u64, mtime: u64);
    fn stat_to_cpio(hdr: *mut cpio_file_stat, st: *mut stat);
    fn cpio_safer_name_suffix(
        name: *mut i8,
        link_target: bool,
        absolute_names: bool,
        strip_leading_dots: bool,
    );
    fn change_dir();
    fn create_deferment(file_hdr: *mut cpio_file_stat) -> *mut deferment;
    fn free_deferment(d: *mut deferment);
}
pub type __uint32_t = u32;
pub type __uintmax_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type uint32_t = __uint32_t;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct old_cpio_header {
    pub c_magic: libc::c_ushort,
    pub c_dev: libc::c_ushort,
    pub c_ino: libc::c_ushort,
    pub c_mode: libc::c_ushort,
    pub c_uid: libc::c_ushort,
    pub c_gid: libc::c_ushort,
    pub c_nlink: libc::c_ushort,
    pub c_rdev: libc::c_ushort,
    pub c_mtimes: [libc::c_ushort; 2],
    pub c_namesize: libc::c_ushort,
    pub c_filesizes: [libc::c_ushort; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpio_file_stat {
    pub c_magic: libc::c_ushort,
    pub c_ino: ino_t,
    pub c_mode: mode_t,
    pub c_uid: uid_t,
    pub c_gid: gid_t,
    pub c_nlink: size_t,
    pub c_mtime: time_t,
    pub c_filesize: off_t,
    pub c_dev_maj: u32,
    pub c_dev_min: u32,
    pub c_rdev_maj: u32,
    pub c_rdev_min: u32,
    pub c_namesize: size_t,
    pub c_chksum: uint32_t,
    pub c_name: *mut i8,
    pub c_name_buflen: size_t,
    pub c_tar_linkname: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynamic_string {
    pub ds_size: size_t,
    pub ds_idx: size_t,
    pub ds_string: *mut i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum archive_format {
    arf_unknown,
    arf_binary,
    arf_oldascii,
    arf_newascii,
    arf_crcascii,
    arf_tar,
    arf_ustar,
    arf_hpoldascii,
    arf_hpbinary,
}
impl archive_format {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            archive_format::arf_unknown => 0,
            archive_format::arf_binary => 1,
            archive_format::arf_oldascii => 2,
            archive_format::arf_newascii => 3,
            archive_format::arf_crcascii => 4,
            archive_format::arf_tar => 5,
            archive_format::arf_ustar => 6,
            archive_format::arf_hpoldascii => 7,
            archive_format::arf_hpbinary => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> archive_format {
        match value {
            0 => archive_format::arf_unknown,
            1 => archive_format::arf_binary,
            2 => archive_format::arf_oldascii,
            3 => archive_format::arf_newascii,
            4 => archive_format::arf_crcascii,
            5 => archive_format::arf_tar,
            6 => archive_format::arf_ustar,
            7 => archive_format::arf_hpoldascii,
            8 => archive_format::arf_hpbinary,
            _ => panic!("Invalid value for archive_format: {}", value),
        }
    }
}
impl AddAssign<u32> for archive_format {
    fn add_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for archive_format {
    fn sub_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for archive_format {
    fn mul_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for archive_format {
    fn div_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for archive_format {
    fn rem_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for archive_format {
    type Output = archive_format;
    fn add(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for archive_format {
    type Output = archive_format;
    fn sub(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for archive_format {
    type Output = archive_format;
    fn mul(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for archive_format {
    type Output = archive_format;
    fn div(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for archive_format {
    type Output = archive_format;
    fn rem(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deferment {
    pub next: *mut deferment,
    pub header: cpio_file_stat,
}
#[inline]
unsafe extern "C" fn gnu_dev_makedev(mut __major: u32, mut __minor: u32) -> __dev_t {
    let mut __dev: __dev_t = 0;
    __dev = ((__major & 0xfff as u32) as __dev_t) << 8 as i32;
    __dev |= ((__major & 0xfffff000 as u32) as __dev_t) << 32 as i32;
    __dev |= ((__minor & 0xff as u32) as __dev_t) << 0 as i32;
    __dev |= ((__minor & 0xffffff00 as u32) as __dev_t) << 12 as i32;
    return __dev;
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn fputc_unlocked(mut __c: i32, mut __stream: *mut FILE) -> i32 {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as i32 as i64 != 0
    {
        __overflow(__stream, __c as u8 as i32)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as i8;
        *fresh0 as u8 as i32
    };
}
unsafe extern "C" fn read_for_checksum(
    mut in_file_des: i32,
    mut file_size: off_t,
    mut file_name: *mut i8,
) -> uint32_t {
    let mut crc: uint32_t = 0;
    let mut buf: [u8; 8192] = [0; 8192];
    let mut bytes_read: ssize_t = 0;
    let mut i: ssize_t = 0;
    crc = 0 as i32 as uint32_t;
    while file_size > 0 as i32 as i64 {
        bytes_read = read(
            in_file_des,
            buf.as_mut_ptr() as *mut libc::c_void,
            8192 as i32 as size_t,
        );
        if bytes_read < 0 as i32 as i64 {
            error(
                2 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"cannot read checksum for %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                file_name,
            );
        }
        if bytes_read == 0 as i32 as i64 {
            break;
        }
        i = 0 as i32 as ssize_t;
        while i < bytes_read {
            crc = (crc as u32)
                .wrapping_add((buf[i as usize] as i32 & 0xff as i32) as u32) as uint32_t
                as uint32_t;
            i += 1;
            i;
        }
        file_size -= bytes_read;
    }
    if lseek(in_file_des, 0 as i64, 0 as i32) != 0 {
        error(
            2 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"cannot read checksum for %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            file_name,
        );
    }
    return crc;
}
unsafe extern "C" fn tape_clear_rest_of_block(mut out_file_des: i32) {
    write_nuls_to_file(
        (io_block_size as u64).wrapping_sub(output_size) as off_t,
        out_file_des,
        Some(tape_buffered_write as unsafe extern "C" fn(*mut i8, i32, off_t) -> ()),
    );
}
unsafe extern "C" fn tape_pad_output(mut out_file_des: i32, mut offset: i32) {
    let mut pad: size_t = 0;
    if archive_format as u32 == archive_format::arf_newascii as i32 as u32
        || archive_format as u32 == archive_format::arf_crcascii as i32 as u32
    {
        pad = ((4 as i32 - offset % 4 as i32) % 4 as i32) as size_t;
    } else if archive_format as u32 == archive_format::arf_tar as i32 as u32
        || archive_format as u32 == archive_format::arf_ustar as i32 as u32
    {
        pad = ((512 as i32 - offset % 512 as i32) % 512 as i32) as size_t;
    } else if archive_format as u32 != archive_format::arf_oldascii as i32 as u32
        && archive_format as u32 != archive_format::arf_hpoldascii as i32 as u32
    {
        pad = ((2 as i32 - offset % 2 as i32) % 2 as i32) as size_t;
    } else {
        pad = 0 as i32 as size_t;
    }
    if pad != 0 as i32 as u64 {
        write_nuls_to_file(
            pad as off_t,
            out_file_des,
            Some(tape_buffered_write as unsafe extern "C" fn(*mut i8, i32, off_t) -> ()),
        );
    }
}
#[no_mangle]
pub static mut deferouts: *mut deferment = 0 as *const deferment as *mut deferment;
unsafe extern "C" fn count_defered_links_to_dev_ino(
    mut file_hdr: *mut cpio_file_stat,
) -> size_t {
    let mut d: *mut deferment = 0 as *mut deferment;
    let mut ino: ino_t = (*file_hdr).c_ino;
    let mut maj: i64 = (*file_hdr).c_dev_maj as i64;
    let mut min: i64 = (*file_hdr).c_dev_min as i64;
    let mut count: size_t = 0 as i32 as size_t;
    d = deferouts;
    while !d.is_null() {
        if (*d).header.c_ino == ino && (*d).header.c_dev_maj as i64 == maj
            && (*d).header.c_dev_min as i64 == min
        {
            count = count.wrapping_add(1);
            count;
        }
        d = (*d).next;
    }
    return count;
}
unsafe extern "C" fn last_link(mut file_hdr: *mut cpio_file_stat) -> i32 {
    return ((*file_hdr).c_nlink
        == (count_defered_links_to_dev_ino(file_hdr)).wrapping_add(1 as i32 as u64))
        as i32;
}
unsafe extern "C" fn add_link_defer(mut file_hdr: *mut cpio_file_stat) {
    let mut d: *mut deferment = 0 as *mut deferment;
    d = create_deferment(file_hdr);
    (*d).next = deferouts;
    deferouts = d;
}
unsafe extern "C" fn writeout_other_defers(
    mut file_hdr: *mut cpio_file_stat,
    mut out_des: i32,
) {
    let mut d: *mut deferment = 0 as *mut deferment;
    let mut d_prev: *mut deferment = 0 as *mut deferment;
    let mut ino: ino_t = 0;
    let mut maj: i32 = 0;
    let mut min: i32 = 0;
    ino = (*file_hdr).c_ino;
    maj = (*file_hdr).c_dev_maj as i32;
    min = (*file_hdr).c_dev_min as i32;
    d_prev = 0 as *mut deferment;
    d = deferouts;
    while !d.is_null() {
        if (*d).header.c_ino == ino && (*d).header.c_dev_maj == maj as u32
            && (*d).header.c_dev_min == min as u32
        {
            let mut d_free: *mut deferment = 0 as *mut deferment;
            (*d).header.c_filesize = 0 as i32 as off_t;
            write_out_header(&mut (*d).header, out_des);
            if !d_prev.is_null() {
                (*d_prev).next = (*d).next;
            } else {
                deferouts = (*d).next;
            }
            d_free = d;
            d = (*d).next;
            free_deferment(d_free);
        } else {
            d_prev = d;
            d = (*d).next;
        }
    }
}
unsafe extern "C" fn writeout_defered_file(
    mut header: *mut cpio_file_stat,
    mut out_file_des: i32,
) {
    let mut in_file_des: i32 = 0;
    let mut file_hdr: cpio_file_stat = cpio_file_stat {
        c_magic: 0,
        c_ino: 0,
        c_mode: 0,
        c_uid: 0,
        c_gid: 0,
        c_nlink: 0,
        c_mtime: 0,
        c_filesize: 0,
        c_dev_maj: 0,
        c_dev_min: 0,
        c_rdev_maj: 0,
        c_rdev_min: 0,
        c_namesize: 0,
        c_chksum: 0,
        c_name: 0 as *mut i8,
        c_name_buflen: 0,
        c_tar_linkname: 0 as *const i8,
    };
    file_hdr = *header;
    in_file_des = open((*header).c_name, 0 as i32 | 0 as i32, 0 as i32);
    if in_file_des < 0 as i32 {
        open_error((*header).c_name);
        return;
    }
    if archive_format as u32 == archive_format::arf_crcascii as i32 as u32 {
        file_hdr.c_chksum = read_for_checksum(
            in_file_des,
            file_hdr.c_filesize,
            (*header).c_name,
        );
    }
    if write_out_header(&mut file_hdr, out_file_des) != 0 {
        return;
    }
    copy_files_disk_to_tape(
        in_file_des,
        out_file_des,
        file_hdr.c_filesize,
        (*header).c_name,
    );
    warn_if_file_changed((*header).c_name, file_hdr.c_filesize, file_hdr.c_mtime);
    if archive_format as u32 == archive_format::arf_tar as i32 as u32
        || archive_format as u32 == archive_format::arf_ustar as i32 as u32
    {
        add_inode(
            file_hdr.c_ino,
            file_hdr.c_name,
            file_hdr.c_dev_maj as u64,
            file_hdr.c_dev_min as u64,
        );
    }
    tape_pad_output(out_file_des, file_hdr.c_filesize as i32);
    if reset_time_flag != 0 {
        set_file_times(
            in_file_des,
            file_hdr.c_name,
            file_hdr.c_mtime as u64,
            file_hdr.c_mtime as u64,
        );
    }
    if close(in_file_des) < 0 as i32 {
        close_error((*header).c_name);
    }
}
unsafe extern "C" fn writeout_final_defers(mut out_des: i32) {
    let mut d: *mut deferment = 0 as *mut deferment;
    let mut other_count: i32 = 0;
    while !deferouts.is_null() {
        d = deferouts;
        other_count = count_defered_links_to_dev_ino(&mut (*d).header) as i32;
        if other_count == 1 as i32 {
            writeout_defered_file(&mut (*d).header, out_des);
        } else {
            let mut file_hdr: cpio_file_stat = cpio_file_stat {
                c_magic: 0,
                c_ino: 0,
                c_mode: 0,
                c_uid: 0,
                c_gid: 0,
                c_nlink: 0,
                c_mtime: 0,
                c_filesize: 0,
                c_dev_maj: 0,
                c_dev_min: 0,
                c_rdev_maj: 0,
                c_rdev_min: 0,
                c_namesize: 0,
                c_chksum: 0,
                c_name: 0 as *mut i8,
                c_name_buflen: 0,
                c_tar_linkname: 0 as *const i8,
            };
            file_hdr = (*d).header;
            file_hdr.c_filesize = 0 as i32 as off_t;
            write_out_header(&mut file_hdr, out_des);
        }
        deferouts = (*deferouts).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn to_ascii(
    mut where_0: *mut i8,
    mut v: uintmax_t,
    mut digits: size_t,
    mut logbase: u32,
    mut nul: bool,
) -> i32 {
    static mut codetab: [i8; 17] = unsafe {
        *::core::mem::transmute::<&[u8; 17], &mut [i8; 17]>(b"0123456789ABCDEF\0")
    };
    if nul {
        digits = digits.wrapping_sub(1);
        *where_0.offset(digits as isize) = 0 as i32 as i8;
    }
    while digits > 0 as i32 as u64 {
        digits = digits.wrapping_sub(1);
        *where_0.offset(digits as isize) = codetab[(v
            & (((1 as i32) << logbase) - 1 as i32) as u64) as usize];
        v >>= logbase;
    }
    return (v != 0 as i32 as u64) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn field_width_error(
    mut filename: *const i8,
    mut fieldname: *const i8,
    mut value: uintmax_t,
    mut width: size_t,
    mut nul: bool,
) {
    let mut valbuf: [i8; 22] = [0; 22];
    let mut maxbuf: [i8; 22] = [0; 22];
    error(
        0 as i32,
        0 as i32,
        dcgettext(
            0 as *const i8,
            b"%s: value %s %s out of allowed range 0..%s\0" as *const u8 as *const i8,
            5 as i32,
        ),
        filename,
        fieldname,
        umaxtostr(value, valbuf.as_mut_ptr()),
        umaxtostr(
            if width.wrapping_sub(nul as u64).wrapping_mul(3 as i32 as u64)
                < (::core::mem::size_of::<uintmax_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
            {
                ((1 as i32 as uintmax_t)
                    << width.wrapping_sub(nul as u64).wrapping_mul(3 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)
            } else {
                -(1 as i32) as uintmax_t
            },
            maxbuf.as_mut_ptr(),
        ),
    );
}
unsafe extern "C" fn field_width_warning(
    mut filename: *const i8,
    mut fieldname: *const i8,
) {
    if warn_option & 0x1 as i32 as u32 != 0 {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"%s: truncating %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            filename,
            fieldname,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn to_ascii_or_warn(
    mut where_0: *mut i8,
    mut n: uintmax_t,
    mut digits: size_t,
    mut logbase: u32,
    mut filename: *const i8,
    mut fieldname: *const i8,
) {
    if to_ascii(where_0, n, digits, logbase, 0 as i32 != 0) != 0 {
        field_width_warning(filename, fieldname);
    }
}
#[no_mangle]
pub unsafe extern "C" fn to_ascii_or_error(
    mut where_0: *mut i8,
    mut n: uintmax_t,
    mut digits: size_t,
    mut logbase: u32,
    mut filename: *const i8,
    mut fieldname: *const i8,
) -> i32 {
    if to_ascii(where_0, n, digits, logbase, 0 as i32 != 0) != 0 {
        field_width_error(filename, fieldname, n, digits, 0 as i32 != 0);
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn write_out_new_ascii_header(
    mut magic_string: *const i8,
    mut file_hdr: *mut cpio_file_stat,
    mut out_des: i32,
) -> i32 {
    let mut ascii_header: [i8; 110] = [0; 110];
    let mut p: *mut i8 = 0 as *mut i8;
    p = stpcpy(ascii_header.as_mut_ptr(), magic_string);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_ino,
        8 as i32 as size_t,
        4 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"inode number\0" as *const u8 as *const i8, 5 as i32),
    );
    p = p.offset(8 as i32 as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_mode as uintmax_t,
        8 as i32 as size_t,
        4 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"file mode\0" as *const u8 as *const i8, 5 as i32),
    );
    p = p.offset(8 as i32 as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_uid as uintmax_t,
        8 as i32 as size_t,
        4 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"uid\0" as *const u8 as *const i8, 5 as i32),
    );
    p = p.offset(8 as i32 as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_gid as uintmax_t,
        8 as i32 as size_t,
        4 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"gid\0" as *const u8 as *const i8, 5 as i32),
    );
    p = p.offset(8 as i32 as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_nlink,
        8 as i32 as size_t,
        4 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const i8,
            b"number of links\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    p = p.offset(8 as i32 as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_mtime as uintmax_t,
        8 as i32 as size_t,
        4 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const i8,
            b"modification time\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    p = p.offset(8 as i32 as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_filesize as uintmax_t,
        8 as i32 as size_t,
        4 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"file size\0" as *const u8 as *const i8, 5 as i32),
    ) != 0
    {
        return 1 as i32;
    }
    p = p.offset(8 as i32 as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_dev_maj as uintmax_t,
        8 as i32 as size_t,
        4 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const i8,
            b"device major number\0" as *const u8 as *const i8,
            5 as i32,
        ),
    ) != 0
    {
        return 1 as i32;
    }
    p = p.offset(8 as i32 as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_dev_min as uintmax_t,
        8 as i32 as size_t,
        4 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const i8,
            b"device minor number\0" as *const u8 as *const i8,
            5 as i32,
        ),
    ) != 0
    {
        return 1 as i32;
    }
    p = p.offset(8 as i32 as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_rdev_maj as uintmax_t,
        8 as i32 as size_t,
        4 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"rdev major\0" as *const u8 as *const i8, 5 as i32),
    ) != 0
    {
        return 1 as i32;
    }
    p = p.offset(8 as i32 as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_rdev_min as uintmax_t,
        8 as i32 as size_t,
        4 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"rdev minor\0" as *const u8 as *const i8, 5 as i32),
    ) != 0
    {
        return 1 as i32;
    }
    p = p.offset(8 as i32 as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_namesize,
        8 as i32 as size_t,
        4 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"name size\0" as *const u8 as *const i8, 5 as i32),
    ) != 0
    {
        return 1 as i32;
    }
    p = p.offset(8 as i32 as isize);
    to_ascii(
        p,
        ((*file_hdr).c_chksum & 0xffffffff as u32) as uintmax_t,
        8 as i32 as size_t,
        4 as i32 as u32,
        0 as i32 != 0,
    );
    tape_buffered_write(
        ascii_header.as_mut_ptr(),
        out_des,
        ::core::mem::size_of::<[i8; 110]>() as u64 as off_t,
    );
    tape_buffered_write((*file_hdr).c_name, out_des, (*file_hdr).c_namesize as i64);
    tape_pad_output(
        out_des,
        ((*file_hdr).c_namesize).wrapping_add(::core::mem::size_of::<[i8; 110]>() as u64)
            as i32,
    );
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn write_out_old_ascii_header(
    mut dev: dev_t,
    mut rdev: dev_t,
    mut file_hdr: *mut cpio_file_stat,
    mut out_des: i32,
) -> i32 {
    let mut ascii_header: [i8; 76] = [0; 76];
    let mut p: *mut i8 = ascii_header.as_mut_ptr();
    to_ascii(
        p,
        (*file_hdr).c_magic as uintmax_t,
        6 as i32 as size_t,
        3 as i32 as u32,
        0 as i32 != 0,
    );
    p = p.offset(6 as i32 as isize);
    to_ascii_or_warn(
        p,
        dev,
        6 as i32 as size_t,
        3 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"device number\0" as *const u8 as *const i8, 5 as i32),
    );
    p = p.offset(6 as i32 as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_ino,
        6 as i32 as size_t,
        3 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"inode number\0" as *const u8 as *const i8, 5 as i32),
    );
    p = p.offset(6 as i32 as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_mode as uintmax_t,
        6 as i32 as size_t,
        3 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"file mode\0" as *const u8 as *const i8, 5 as i32),
    );
    p = p.offset(6 as i32 as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_uid as uintmax_t,
        6 as i32 as size_t,
        3 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"uid\0" as *const u8 as *const i8, 5 as i32),
    );
    p = p.offset(6 as i32 as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_gid as uintmax_t,
        6 as i32 as size_t,
        3 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"gid\0" as *const u8 as *const i8, 5 as i32),
    );
    p = p.offset(6 as i32 as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_nlink,
        6 as i32 as size_t,
        3 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const i8,
            b"number of links\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    p = p.offset(6 as i32 as isize);
    to_ascii_or_warn(
        p,
        rdev,
        6 as i32 as size_t,
        3 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"rdev\0" as *const u8 as *const i8, 5 as i32),
    );
    p = p.offset(6 as i32 as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_mtime as uintmax_t,
        11 as i32 as size_t,
        3 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const i8,
            b"modification time\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    p = p.offset(11 as i32 as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_namesize,
        6 as i32 as size_t,
        3 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"name size\0" as *const u8 as *const i8, 5 as i32),
    ) != 0
    {
        return 1 as i32;
    }
    p = p.offset(6 as i32 as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_filesize as uintmax_t,
        11 as i32 as size_t,
        3 as i32 as u32,
        (*file_hdr).c_name,
        dcgettext(0 as *const i8, b"file size\0" as *const u8 as *const i8, 5 as i32),
    ) != 0
    {
        return 1 as i32;
    }
    tape_buffered_write(
        ascii_header.as_mut_ptr(),
        out_des,
        ::core::mem::size_of::<[i8; 76]>() as u64 as off_t,
    );
    tape_buffered_write((*file_hdr).c_name, out_des, (*file_hdr).c_namesize as off_t);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn hp_compute_dev(
    mut file_hdr: *mut cpio_file_stat,
    mut pdev: *mut dev_t,
    mut prdev: *mut dev_t,
) {
    match (*file_hdr).c_mode & 0o170000 as i32 as u32 {
        8192 | 24576 | 49152 | 4096 => {
            (*file_hdr).c_filesize = gnu_dev_makedev(
                (*file_hdr).c_rdev_maj,
                (*file_hdr).c_rdev_min,
            ) as off_t;
            *prdev = gnu_dev_makedev(0 as i32 as u32, 1 as i32 as u32);
            *pdev = *prdev;
        }
        _ => {
            *pdev = gnu_dev_makedev((*file_hdr).c_dev_maj, (*file_hdr).c_dev_min);
            *prdev = gnu_dev_makedev((*file_hdr).c_rdev_maj, (*file_hdr).c_rdev_min);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn write_out_binary_header(
    mut rdev: dev_t,
    mut file_hdr: *mut cpio_file_stat,
    mut out_des: i32,
) -> i32 {
    let mut short_hdr: old_cpio_header = old_cpio_header {
        c_magic: 0,
        c_dev: 0,
        c_ino: 0,
        c_mode: 0,
        c_uid: 0,
        c_gid: 0,
        c_nlink: 0,
        c_rdev: 0,
        c_mtimes: [0; 2],
        c_namesize: 0,
        c_filesizes: [0; 2],
    };
    short_hdr.c_magic = 0o70707 as i32 as libc::c_ushort;
    short_hdr.c_dev = gnu_dev_makedev((*file_hdr).c_dev_maj, (*file_hdr).c_dev_min)
        as libc::c_ushort;
    if warn_option & 0x1 as i32 as u32 != 0
        && (*file_hdr).c_ino >> 16 as i32 != 0 as i32 as u64
    {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"%s: truncating inode number\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*file_hdr).c_name,
        );
    }
    short_hdr.c_ino = ((*file_hdr).c_ino & 0xffff as i32 as u64) as libc::c_ushort;
    if short_hdr.c_ino as u64 != (*file_hdr).c_ino {
        field_width_warning(
            (*file_hdr).c_name,
            dcgettext(
                0 as *const i8,
                b"inode number\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    short_hdr.c_mode = ((*file_hdr).c_mode & 0xffff as i32 as u32) as libc::c_ushort;
    if short_hdr.c_mode as u32 != (*file_hdr).c_mode {
        field_width_warning(
            (*file_hdr).c_name,
            dcgettext(0 as *const i8, b"file mode\0" as *const u8 as *const i8, 5 as i32),
        );
    }
    short_hdr.c_uid = ((*file_hdr).c_uid & 0xffff as i32 as u32) as libc::c_ushort;
    if short_hdr.c_uid as u32 != (*file_hdr).c_uid {
        field_width_warning(
            (*file_hdr).c_name,
            dcgettext(0 as *const i8, b"uid\0" as *const u8 as *const i8, 5 as i32),
        );
    }
    short_hdr.c_gid = ((*file_hdr).c_gid & 0xffff as i32 as u32) as libc::c_ushort;
    if short_hdr.c_gid as u32 != (*file_hdr).c_gid {
        field_width_warning(
            (*file_hdr).c_name,
            dcgettext(0 as *const i8, b"gid\0" as *const u8 as *const i8, 5 as i32),
        );
    }
    short_hdr.c_nlink = ((*file_hdr).c_nlink & 0xffff as i32 as u64) as libc::c_ushort;
    if short_hdr.c_nlink as u64 != (*file_hdr).c_nlink {
        field_width_warning(
            (*file_hdr).c_name,
            dcgettext(
                0 as *const i8,
                b"number of links\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    short_hdr.c_rdev = rdev as libc::c_ushort;
    short_hdr.c_mtimes[0 as i32 as usize] = ((*file_hdr).c_mtime >> 16 as i32)
        as libc::c_ushort;
    short_hdr.c_mtimes[1 as i32 as usize] = ((*file_hdr).c_mtime & 0xffff as i32 as i64)
        as libc::c_ushort;
    short_hdr.c_namesize = ((*file_hdr).c_namesize & 0xffff as i32 as u64)
        as libc::c_ushort;
    if short_hdr.c_namesize as u64 != (*file_hdr).c_namesize {
        let mut maxbuf: [i8; 22] = [0; 22];
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"%s: value %s %s out of allowed range 0..%u\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            (*file_hdr).c_name,
            dcgettext(
                0 as *const i8,
                b"name size\0" as *const u8 as *const i8,
                5 as i32,
            ),
            umaxtostr((*file_hdr).c_namesize, maxbuf.as_mut_ptr()),
            0xffff as u32,
        );
        return 1 as i32;
    }
    short_hdr.c_filesizes[0 as i32 as usize] = ((*file_hdr).c_filesize >> 16 as i32)
        as libc::c_ushort;
    short_hdr.c_filesizes[1 as i32 as usize] = ((*file_hdr).c_filesize
        & 0xffff as i32 as i64) as libc::c_ushort;
    if ((short_hdr.c_filesizes[0 as i32 as usize] as off_t) << 16 as i32)
        + short_hdr.c_filesizes[1 as i32 as usize] as i64 != (*file_hdr).c_filesize
    {
        let mut maxbuf_0: [i8; 22] = [0; 22];
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"%s: value %s %s out of allowed range 0..%lu\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            (*file_hdr).c_name,
            dcgettext(
                0 as *const i8,
                b"file size\0" as *const u8 as *const i8,
                5 as i32,
            ),
            umaxtostr((*file_hdr).c_namesize, maxbuf_0.as_mut_ptr()),
            0xffffffff as u64,
        );
        return 1 as i32;
    }
    tape_buffered_write(
        &mut short_hdr as *mut old_cpio_header as *mut i8,
        out_des,
        26 as i32 as off_t,
    );
    tape_buffered_write((*file_hdr).c_name, out_des, (*file_hdr).c_namesize as off_t);
    tape_pad_output(
        out_des,
        ((*file_hdr).c_namesize).wrapping_add(26 as i32 as u64) as i32,
    );
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn write_out_header(
    mut file_hdr: *mut cpio_file_stat,
    mut out_des: i32,
) -> i32 {
    let mut dev: dev_t = 0;
    let mut rdev: dev_t = 0;
    match archive_format as u32 {
        3 => {
            return write_out_new_ascii_header(
                b"070701\0" as *const u8 as *const i8,
                file_hdr,
                out_des,
            );
        }
        4 => {
            return write_out_new_ascii_header(
                b"070702\0" as *const u8 as *const i8,
                file_hdr,
                out_des,
            );
        }
        2 => {
            return write_out_old_ascii_header(
                gnu_dev_makedev((*file_hdr).c_dev_maj, (*file_hdr).c_dev_min),
                gnu_dev_makedev((*file_hdr).c_rdev_maj, (*file_hdr).c_rdev_min),
                file_hdr,
                out_des,
            );
        }
        7 => {
            hp_compute_dev(file_hdr, &mut dev, &mut rdev);
            return write_out_old_ascii_header(dev, rdev, file_hdr, out_des);
        }
        5 | 6 => {
            if is_tar_filename_too_long((*file_hdr).c_name) != 0 {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s: file name too long\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*file_hdr).c_name,
                );
                return 1 as i32;
            }
            return write_out_tar_header(file_hdr, out_des);
        }
        1 => {
            return write_out_binary_header(
                gnu_dev_makedev((*file_hdr).c_rdev_maj, (*file_hdr).c_rdev_min),
                file_hdr,
                out_des,
            );
        }
        8 => {
            hp_compute_dev(file_hdr, &mut dev, &mut rdev);
            return write_out_binary_header(rdev, file_hdr, out_des);
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn assign_string(mut pvar: *mut *mut i8, mut value: *mut i8) {
    let mut p: *mut i8 = xrealloc(
        *pvar as *mut libc::c_void,
        (strlen(value)).wrapping_add(1 as i32 as u64),
    ) as *mut i8;
    strcpy(p, value);
    *pvar = p;
}
#[no_mangle]
pub unsafe extern "C" fn process_copy_out() {
    let mut input_name: dynamic_string = {
        let mut init = dynamic_string {
            ds_size: 0 as i32 as size_t,
            ds_idx: 0 as i32 as size_t,
            ds_string: 0 as *mut i8,
        };
        init
    };
    let mut file_stat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut file_hdr: cpio_file_stat = {
        let mut init = cpio_file_stat {
            c_magic: 0 as i32 as libc::c_ushort,
            c_ino: 0 as i32 as ino_t,
            c_mode: 0 as i32 as mode_t,
            c_uid: 0 as i32 as uid_t,
            c_gid: 0 as i32 as gid_t,
            c_nlink: 0 as i32 as size_t,
            c_mtime: 0 as i32 as time_t,
            c_filesize: 0 as i32 as off_t,
            c_dev_maj: 0 as i32 as u32,
            c_dev_min: 0 as i32 as u32,
            c_rdev_maj: 0 as i32 as u32,
            c_rdev_min: 0 as i32 as u32,
            c_namesize: 0 as i32 as size_t,
            c_chksum: 0 as i32 as uint32_t,
            c_name: 0 as *mut i8,
            c_name_buflen: 0 as i32 as size_t,
            c_tar_linkname: 0 as *const i8,
        };
        init
    };
    let mut in_file_des: i32 = 0;
    let mut out_file_des: i32 = 0;
    let mut orig_file_name: *mut i8 = 0 as *mut i8;
    file_hdr.c_magic = 0o70707 as i32 as libc::c_ushort;
    out_file_des = archive_des;
    if out_file_des >= (1 as i32) << 30 as i32 {
        output_is_special = 1 as i32 as i8;
        output_is_seekable = 0 as i32 as i8;
    } else {
        if fstat(out_file_des, &mut file_stat) != 0 {
            error(
                2 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"standard output is closed\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        output_is_special = (file_stat.st_mode & 0o170000 as i32 as u32
            == 0o60000 as i32 as u32
            || file_stat.st_mode & 0o170000 as i32 as u32 == 0o20000 as i32 as u32)
            as i32 as i8;
        output_is_seekable = (file_stat.st_mode & 0o170000 as i32 as u32
            == 0o100000 as i32 as u32) as i32 as i8;
    }
    if append_flag != 0 {
        process_copy_in();
        prepare_append(out_file_des);
    } else {
        change_dir();
    }
    let mut current_block_72: u64;
    while !(ds_fgetstr(stdin, &mut input_name, name_end)).is_null() {
        if *(input_name.ds_string).offset(0 as i32 as isize) as i32 == 0 as i32 {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"blank line ignored\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        } else if ::core::mem::transmute::<
            _,
            fn(_, _) -> i32,
        >(
            (Some(xstat.expect("non-null function pointer")))
                .expect("non-null function pointer"),
        )(input_name.ds_string, &mut file_stat) < 0 as i32
        {
            stat_error(input_name.ds_string);
        } else {
            stat_to_cpio(&mut file_hdr, &mut file_stat);
            if archive_format as u32 == archive_format::arf_tar as i32 as u32
                || archive_format as u32 == archive_format::arf_ustar as i32 as u32
            {
                if file_hdr.c_mode & 0o40000 as i32 as u32 != 0 {
                    if ds_endswith(&mut input_name, '/' as i32) == 0 {
                        ds_append(&mut input_name, '/' as i32);
                    }
                }
            }
            assign_string(&mut orig_file_name, input_name.ds_string);
            cpio_safer_name_suffix(
                input_name.ds_string,
                0 as i32 != 0,
                no_abs_paths_flag == 0,
                1 as i32 != 0,
            );
            cpio_set_c_name(&mut file_hdr, input_name.ds_string);
            match file_hdr.c_mode & 0o170000 as i32 as u32 {
                32768 => {
                    if archive_format as u32 == archive_format::arf_tar as i32 as u32
                        || archive_format as u32
                            == archive_format::arf_ustar as i32 as u32
                    {
                        let mut otherfile: *mut i8 = 0 as *mut i8;
                        otherfile = find_inode_file(
                            file_hdr.c_ino,
                            file_hdr.c_dev_maj as u64,
                            file_hdr.c_dev_min as u64,
                        );
                        if !otherfile.is_null() {
                            file_hdr.c_tar_linkname = otherfile;
                            if write_out_header(&mut file_hdr, out_file_des) != 0 {
                                continue;
                            }
                            current_block_72 = 2310077433060450808;
                        } else {
                            current_block_72 = 15897653523371991391;
                        }
                    } else {
                        current_block_72 = 15897653523371991391;
                    }
                    match current_block_72 {
                        2310077433060450808 => {}
                        _ => {
                            if (archive_format as u32
                                == archive_format::arf_newascii as i32 as u32
                                || archive_format as u32
                                    == archive_format::arf_crcascii as i32 as u32)
                                && file_hdr.c_nlink > 1 as i32 as u64
                            {
                                if last_link(&mut file_hdr) != 0 {
                                    writeout_other_defers(&mut file_hdr, out_file_des);
                                    current_block_72 = 6450636197030046351;
                                } else {
                                    add_link_defer(&mut file_hdr);
                                    current_block_72 = 2310077433060450808;
                                }
                            } else {
                                current_block_72 = 6450636197030046351;
                            }
                            match current_block_72 {
                                2310077433060450808 => {}
                                _ => {
                                    in_file_des = open(
                                        orig_file_name,
                                        0 as i32 | 0 as i32,
                                        0 as i32,
                                    );
                                    if in_file_des < 0 as i32 {
                                        open_error(orig_file_name);
                                        continue;
                                    } else {
                                        if archive_format as u32
                                            == archive_format::arf_crcascii as i32 as u32
                                        {
                                            file_hdr.c_chksum = read_for_checksum(
                                                in_file_des,
                                                file_hdr.c_filesize,
                                                orig_file_name,
                                            );
                                        }
                                        if write_out_header(&mut file_hdr, out_file_des) != 0 {
                                            continue;
                                        }
                                        copy_files_disk_to_tape(
                                            in_file_des,
                                            out_file_des,
                                            file_hdr.c_filesize,
                                            orig_file_name,
                                        );
                                        warn_if_file_changed(
                                            orig_file_name,
                                            file_hdr.c_filesize,
                                            file_hdr.c_mtime,
                                        );
                                        if archive_format as u32
                                            == archive_format::arf_tar as i32 as u32
                                            || archive_format as u32
                                                == archive_format::arf_ustar as i32 as u32
                                        {
                                            add_inode(
                                                file_hdr.c_ino,
                                                orig_file_name,
                                                file_hdr.c_dev_maj as u64,
                                                file_hdr.c_dev_min as u64,
                                            );
                                        }
                                        tape_pad_output(out_file_des, file_hdr.c_filesize as i32);
                                        if reset_time_flag != 0 {
                                            set_file_times(
                                                in_file_des,
                                                orig_file_name,
                                                file_stat.st_atim.tv_sec as u64,
                                                file_stat.st_mtim.tv_sec as u64,
                                            );
                                        }
                                        if close(in_file_des) < 0 as i32 {
                                            close_error(orig_file_name);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                16384 => {
                    file_hdr.c_filesize = 0 as i32 as off_t;
                    if ignore_dirnlink_option != 0 {
                        file_hdr.c_nlink = 2 as i32 as size_t;
                    }
                    if write_out_header(&mut file_hdr, out_file_des) != 0 {
                        continue;
                    }
                }
                8192 | 24576 | 49152 | 4096 => {
                    if archive_format as u32 == archive_format::arf_tar as i32 as u32 {
                        error(
                            0 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"%s not dumped: not a regular file\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            orig_file_name,
                        );
                        continue;
                    } else {
                        if archive_format as u32
                            == archive_format::arf_ustar as i32 as u32
                        {
                            let mut otherfile_0: *mut i8 = 0 as *mut i8;
                            otherfile_0 = find_inode_file(
                                file_hdr.c_ino,
                                file_hdr.c_dev_maj as u64,
                                file_hdr.c_dev_min as u64,
                            );
                            if !otherfile_0.is_null() {
                                file_hdr.c_mode = file_stat.st_mode & 0o7777 as i32 as u32;
                                file_hdr.c_mode |= 0o100000 as i32 as u32;
                                file_hdr.c_tar_linkname = otherfile_0;
                                if write_out_header(&mut file_hdr, out_file_des) != 0 {
                                    continue;
                                }
                                current_block_72 = 2310077433060450808;
                            } else {
                                add_inode(
                                    file_hdr.c_ino,
                                    orig_file_name,
                                    file_hdr.c_dev_maj as u64,
                                    file_hdr.c_dev_min as u64,
                                );
                                current_block_72 = 5807581744382915773;
                            }
                        } else {
                            current_block_72 = 5807581744382915773;
                        }
                        match current_block_72 {
                            2310077433060450808 => {}
                            _ => {
                                file_hdr.c_filesize = 0 as i32 as off_t;
                                if write_out_header(&mut file_hdr, out_file_des) != 0 {
                                    continue;
                                }
                            }
                        }
                    }
                }
                40960 => {
                    let mut link_name: *mut i8 = xmalloc(
                        (file_stat.st_size + 1 as i32 as i64) as size_t,
                    ) as *mut i8;
                    let mut link_size: i32 = 0;
                    link_size = readlink(
                        orig_file_name,
                        link_name,
                        file_stat.st_size as size_t,
                    ) as i32;
                    if link_size < 0 as i32 {
                        readlink_warn(orig_file_name);
                        rpl_free(link_name as *mut libc::c_void);
                        continue;
                    } else {
                        *link_name.offset(link_size as isize) = 0 as i32 as i8;
                        cpio_safer_name_suffix(
                            link_name,
                            0 as i32 != 0,
                            no_abs_paths_flag == 0,
                            1 as i32 != 0,
                        );
                        link_size = strlen(link_name) as i32;
                        file_hdr.c_filesize = link_size as off_t;
                        if archive_format as u32 == archive_format::arf_tar as i32 as u32
                            || archive_format as u32
                                == archive_format::arf_ustar as i32 as u32
                        {
                            if link_size + 1 as i32 > 100 as i32 {
                                error(
                                    0 as i32,
                                    0 as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"%s: symbolic link too long\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    file_hdr.c_name,
                                );
                            } else {
                                *link_name.offset(link_size as isize) = '\0' as i32 as i8;
                                file_hdr.c_tar_linkname = link_name;
                                if write_out_header(&mut file_hdr, out_file_des) != 0 {
                                    continue;
                                }
                            }
                        } else {
                            if write_out_header(&mut file_hdr, out_file_des) != 0 {
                                continue;
                            }
                            tape_buffered_write(
                                link_name,
                                out_file_des,
                                link_size as off_t,
                            );
                            tape_pad_output(out_file_des, link_size);
                        }
                        rpl_free(link_name as *mut libc::c_void);
                    }
                }
                _ => {
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"%s: unknown file type\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        orig_file_name,
                    );
                }
            }
            if verbose_flag != 0 {
                fprintf(stderr, b"%s\n\0" as *const u8 as *const i8, orig_file_name);
            }
            if dot_flag != 0 {
                fputc_unlocked('.' as i32, stderr);
            }
        }
    }
    rpl_free(orig_file_name as *mut libc::c_void);
    writeout_final_defers(out_file_des);
    file_hdr.c_ino = 0 as i32 as ino_t;
    file_hdr.c_mode = 0 as i32 as mode_t;
    file_hdr.c_uid = 0 as i32 as uid_t;
    file_hdr.c_gid = 0 as i32 as gid_t;
    file_hdr.c_nlink = 1 as i32 as size_t;
    file_hdr.c_dev_maj = 0 as i32 as u32;
    file_hdr.c_dev_min = 0 as i32 as u32;
    file_hdr.c_rdev_maj = 0 as i32 as u32;
    file_hdr.c_rdev_min = 0 as i32 as u32;
    file_hdr.c_mtime = 0 as i32 as time_t;
    file_hdr.c_chksum = 0 as i32 as uint32_t;
    file_hdr.c_filesize = 0 as i32 as off_t;
    cpio_set_c_name(&mut file_hdr, b"TRAILER!!!\0" as *const u8 as *const i8 as *mut i8);
    if archive_format as u32 != archive_format::arf_tar as i32 as u32
        && archive_format as u32 != archive_format::arf_ustar as i32 as u32
    {
        write_out_header(&mut file_hdr, out_file_des);
    } else {
        write_nuls_to_file(
            1024 as i32 as off_t,
            out_file_des,
            Some(tape_buffered_write as unsafe extern "C" fn(*mut i8, i32, off_t) -> ()),
        );
    }
    tape_clear_rest_of_block(out_file_des);
    tape_empty_output_buffer(out_file_des);
    if dot_flag != 0 {
        fputc_unlocked('\n' as i32, stderr);
    }
    if quiet_flag == 0 {
        let mut blocks: size_t = ((output_bytes + io_block_size as i64 - 1 as i32 as i64)
            / io_block_size as i64) as size_t;
        fprintf(
            stderr,
            dcngettext(
                0 as *const i8,
                b"%lu block\n\0" as *const u8 as *const i8,
                b"%lu blocks\n\0" as *const u8 as *const i8,
                blocks,
                5 as i32,
            ),
            blocks,
        );
    }
    cpio_file_stat_free(&mut file_hdr);
    ds_free(&mut input_name);
}