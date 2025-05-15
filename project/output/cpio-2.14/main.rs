use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn exit(_: i32) -> !;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn pax_exit();
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn close(__fd: i32) -> i32;
    fn geteuid() -> __uid_t;
    static mut stderr: *mut _IO_FILE;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn textdomain(__domainname: *const i8) -> *mut i8;
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    fn argp_parse(
        __argp: *const argp,
        _: i32,
        _: *mut *mut i8,
        __flags: u32,
        __arg_index: *mut i32,
        __input: *mut libc::c_void,
    ) -> error_t;
    fn argp_help(
        __argp: *const argp,
        __stream: *mut FILE,
        __flags: u32,
        __name: *mut i8,
    );
    fn argp_version_setup(name: *const i8, authors: *const *const i8);
    static mut program_name: *const i8;
    fn set_program_name(argv0: *const i8);
    fn close_stdout();
    static mut to_stdout_option: bool;
    static mut archive_format: archive_format;
    static mut reset_time_flag: i32;
    static mut io_block_size: i32;
    static mut create_dir_flag: i32;
    static mut rename_flag: i32;
    static mut rename_batch_file: *mut i8;
    static mut table_flag: i32;
    static mut unconditional_flag: i32;
    static mut verbose_flag: i32;
    static mut dot_flag: i32;
    static mut link_flag: i32;
    static mut retain_time_flag: i32;
    static mut crc_i_flag: i32;
    static mut append_flag: i32;
    static mut swap_bytes_flag: i32;
    static mut swap_halfwords_flag: i32;
    static mut set_owner_flag: i32;
    static mut set_owner: uid_t;
    static mut set_group_flag: i32;
    static mut set_group: gid_t;
    static mut no_chown_flag: i32;
    static mut sparse_flag: i32;
    static mut quiet_flag: i32;
    static mut only_verify_crc_flag: i32;
    static mut no_abs_paths_flag: i32;
    static mut warn_option: u32;
    static mut renumber_inodes_option: i32;
    static mut copy_matching_files: i32;
    static mut numeric_uid: i32;
    static mut pattern_file_name: *mut i8;
    static mut archive_des: i32;
    static mut archive_name: *mut i8;
    static mut rsh_command_option: *mut i8;
    static mut input_buffer: *mut i8;
    static mut output_buffer: *mut i8;
    static mut in_buff: *mut i8;
    static mut out_buff: *mut i8;
    static mut input_buffer_size: size_t;
    static mut input_size: size_t;
    static mut output_size: size_t;
    static mut input_bytes: off_t;
    static mut output_bytes: off_t;
    static mut directory_name: *mut i8;
    static mut save_patterns: *mut *mut i8;
    static mut num_patterns: i32;
    static mut name_end: i8;
    static mut xstat: Option<unsafe extern "C" fn() -> i32>;
    static mut copy_function: Option<unsafe extern "C" fn() -> ()>;
    static mut change_directory_option: *mut i8;
    fn process_copy_in();
    fn process_copy_out();
    fn process_copy_pass();
    static mut ignore_dirnlink_option: i32;
    static mut ignore_devno_option: i32;
    fn quotearg_colon(arg: *const i8) -> *mut i8;
    fn open_archive(file: *mut i8) -> i32;
    fn set_new_media_message(message: *mut i8);
    fn parse_user_spec(
        spec_arg: *const i8,
        uid: *mut uid_t,
        gid: *mut gid_t,
        username_arg: *mut *mut i8,
        groupname_arg: *mut *mut i8,
    ) -> *const i8;
    fn arf_stores_inode_p(arf: archive_format) -> i32;
    fn rmt_close__(_: i32) -> i32;
    static mut force_local_option: bool;
}
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
pub type __syscall_slong_t = i64;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type error_t = i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_option {
    pub name: *const i8,
    pub key: i32,
    pub arg: *const i8,
    pub flags: i32,
    pub doc: *const i8,
    pub group: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: argp_parser_t,
    pub args_doc: *const i8,
    pub doc: *const i8,
    pub children: *const argp_child,
    pub help_filter: Option<
        unsafe extern "C" fn(i32, *const i8, *mut libc::c_void) -> *mut i8,
    >,
    pub argp_domain: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: i32,
    pub header: *const i8,
    pub group: i32,
}
pub type argp_parser_t = Option<
    unsafe extern "C" fn(i32, *mut i8, *mut argp_state) -> error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_state {
    pub root_argp: *const argp,
    pub argc: i32,
    pub argv: *mut *mut i8,
    pub next: i32,
    pub flags: u32,
    pub arg_num: u32,
    pub quoted: i32,
    pub input: *mut libc::c_void,
    pub child_inputs: *mut *mut libc::c_void,
    pub hook: *mut libc::c_void,
    pub name: *mut i8,
    pub err_stream: *mut FILE,
    pub out_stream: *mut FILE,
    pub pstate: *mut libc::c_void,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum cpio_options {
    NO_ABSOLUTE_FILENAMES_OPTION = 256,
    ABSOLUTE_FILENAMES_OPTION,
    NO_PRESERVE_OWNER_OPTION,
    ONLY_VERIFY_CRC_OPTION,
    RENAME_BATCH_FILE_OPTION,
    RSH_COMMAND_OPTION,
    QUIET_OPTION,
    SPARSE_OPTION,
    FORCE_LOCAL_OPTION,
    DEBUG_OPTION,
    BLOCK_SIZE_OPTION,
    TO_STDOUT_OPTION,
    RENUMBER_INODES_OPTION,
    IGNORE_DEVNO_OPTION,
    IGNORE_DIRNLINK_OPTION,
    DEVICE_INDEPENDENT_OPTION,
}
impl cpio_options {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            cpio_options::NO_ABSOLUTE_FILENAMES_OPTION => 256,
            cpio_options::ABSOLUTE_FILENAMES_OPTION => 257,
            cpio_options::NO_PRESERVE_OWNER_OPTION => 258,
            cpio_options::ONLY_VERIFY_CRC_OPTION => 259,
            cpio_options::RENAME_BATCH_FILE_OPTION => 260,
            cpio_options::RSH_COMMAND_OPTION => 261,
            cpio_options::QUIET_OPTION => 262,
            cpio_options::SPARSE_OPTION => 263,
            cpio_options::FORCE_LOCAL_OPTION => 264,
            cpio_options::DEBUG_OPTION => 265,
            cpio_options::BLOCK_SIZE_OPTION => 266,
            cpio_options::TO_STDOUT_OPTION => 267,
            cpio_options::RENUMBER_INODES_OPTION => 268,
            cpio_options::IGNORE_DEVNO_OPTION => 269,
            cpio_options::IGNORE_DIRNLINK_OPTION => 270,
            cpio_options::DEVICE_INDEPENDENT_OPTION => 271,
        }
    }
    fn from_libc_c_uint(value: u32) -> cpio_options {
        match value {
            256 => cpio_options::NO_ABSOLUTE_FILENAMES_OPTION,
            257 => cpio_options::ABSOLUTE_FILENAMES_OPTION,
            258 => cpio_options::NO_PRESERVE_OWNER_OPTION,
            259 => cpio_options::ONLY_VERIFY_CRC_OPTION,
            260 => cpio_options::RENAME_BATCH_FILE_OPTION,
            261 => cpio_options::RSH_COMMAND_OPTION,
            262 => cpio_options::QUIET_OPTION,
            263 => cpio_options::SPARSE_OPTION,
            264 => cpio_options::FORCE_LOCAL_OPTION,
            265 => cpio_options::DEBUG_OPTION,
            266 => cpio_options::BLOCK_SIZE_OPTION,
            267 => cpio_options::TO_STDOUT_OPTION,
            268 => cpio_options::RENUMBER_INODES_OPTION,
            269 => cpio_options::IGNORE_DEVNO_OPTION,
            270 => cpio_options::IGNORE_DIRNLINK_OPTION,
            271 => cpio_options::DEVICE_INDEPENDENT_OPTION,
            _ => panic!("Invalid value for cpio_options: {}", value),
        }
    }
}
impl AddAssign<u32> for cpio_options {
    fn add_assign(&mut self, rhs: u32) {
        *self = cpio_options::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for cpio_options {
    fn sub_assign(&mut self, rhs: u32) {
        *self = cpio_options::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for cpio_options {
    fn mul_assign(&mut self, rhs: u32) {
        *self = cpio_options::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for cpio_options {
    fn div_assign(&mut self, rhs: u32) {
        *self = cpio_options::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for cpio_options {
    fn rem_assign(&mut self, rhs: u32) {
        *self = cpio_options::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for cpio_options {
    type Output = cpio_options;
    fn add(self, rhs: u32) -> cpio_options {
        cpio_options::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for cpio_options {
    type Output = cpio_options;
    fn sub(self, rhs: u32) -> cpio_options {
        cpio_options::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for cpio_options {
    type Output = cpio_options;
    fn mul(self, rhs: u32) -> cpio_options {
        cpio_options::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for cpio_options {
    type Output = cpio_options;
    fn div(self, rhs: u32) -> cpio_options {
        cpio_options::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for cpio_options {
    type Output = cpio_options;
    fn rem(self, rhs: u32) -> cpio_options {
        cpio_options::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct warn_tab {
    pub name: *mut i8,
    pub flag: i32,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
#[no_mangle]
pub static mut program_authors: [*const i8; 5] = [
    b"Phil Nelson\0" as *const u8 as *const i8,
    b"David MacKenzie\0" as *const u8 as *const i8,
    b"John Oleynick\0" as *const u8 as *const i8,
    b"Sergey Poznyakoff\0" as *const u8 as *const i8,
    0 as *const i8,
];
#[no_mangle]
pub static mut argp_program_bug_address: *const i8 = b"<bug-cpio@gnu.org>\0" as *const u8
    as *const i8;
static mut doc: [i8; 300] = unsafe {
    *::core::mem::transmute::<
        &[u8; 300],
        &mut [i8; 300],
    >(
        b"GNU `cpio' copies files to and from archives\n\nExamples:\n  # Copy files named in name-list to the archive\n  cpio -o < name-list [> archive]\n  # Extract files from the archive\n  cpio -i [< archive]\n  # Copy files named in name-list to destination-directory\n  cpio -p destination-directory < name-list\n\0",
    )
};
static mut options: [argp_option; 58] = [
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Main operation mode:\0" as *const u8 as *const i8,
            group: 10 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"create\0" as *const u8 as *const i8,
            key: 'o' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Create the archive (run in copy-out mode)\0" as *const u8
                as *const i8,
            group: 10 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"extract\0" as *const u8 as *const i8,
            key: 'i' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Extract files from an archive (run in copy-in mode)\0" as *const u8
                as *const i8,
            group: 10 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"pass-through\0" as *const u8 as *const i8,
            key: 'p' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Run in copy-pass mode\0" as *const u8 as *const i8,
            group: 10 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"list\0" as *const u8 as *const i8,
            key: 't' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Print a table of contents of the input\0" as *const u8 as *const i8,
            group: 10 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Operation modifiers valid in any mode:\0" as *const u8 as *const i8,
            group: 100 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"directory\0" as *const u8 as *const i8,
            key: 'D' as i32,
            arg: b"DIR\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Change to directory DIR\0" as *const u8 as *const i8,
            group: 100 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"force-local\0" as *const u8 as *const i8,
            key: cpio_options::FORCE_LOCAL_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Archive file is local, even if its name contains colons\0"
                as *const u8 as *const i8,
            group: 100 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"format\0" as *const u8 as *const i8,
            key: 'H' as i32,
            arg: b"FORMAT\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Use given archive FORMAT\0" as *const u8 as *const i8,
            group: 100 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 'B' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Set the I/O block size to 5120 bytes\0" as *const u8 as *const i8,
            group: 100 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"block-size\0" as *const u8 as *const i8,
            key: cpio_options::BLOCK_SIZE_OPTION as i32,
            arg: b"BLOCK-SIZE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Set the I/O block size to BLOCK-SIZE * 512 bytes\0" as *const u8
                as *const i8,
            group: 100 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 'c' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Use the old portable (ASCII) archive format\0" as *const u8
                as *const i8,
            group: 100 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"dot\0" as *const u8 as *const i8,
            key: 'V' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Print a \".\" for each file processed\0" as *const u8 as *const i8,
            group: 100 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"io-size\0" as *const u8 as *const i8,
            key: 'C' as i32,
            arg: b"NUMBER\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Set the I/O block size to the given NUMBER of bytes\0" as *const u8
                as *const i8,
            group: 100 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"quiet\0" as *const u8 as *const i8,
            key: cpio_options::QUIET_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Do not print the number of blocks copied\0" as *const u8 as *const i8,
            group: 100 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"verbose\0" as *const u8 as *const i8,
            key: 'v' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Verbosely list the files processed\0" as *const u8 as *const i8,
            group: 100 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"warning\0" as *const u8 as *const i8,
            key: 'W' as i32,
            arg: b"FLAG\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Control warning display. Currently FLAG is one of 'none', 'truncate', 'all'. Multiple options accumulate.\0"
                as *const u8 as *const i8,
            group: 100 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"owner\0" as *const u8 as *const i8,
            key: 'R' as i32,
            arg: b"[USER][:.][GROUP]\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Set the ownership of all files created to the specified USER and/or GROUP\0"
                as *const u8 as *const i8,
            group: 100 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Operation modifiers valid in copy-in and copy-out modes\0"
                as *const u8 as *const i8,
            group: 110 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"file\0" as *const u8 as *const i8,
            key: 'F' as i32,
            arg: b"[[USER@]HOST:]FILE-NAME\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Use this FILE-NAME instead of standard input or output. Optional USER and HOST specify the user and host names in case of a remote archive\0"
                as *const u8 as *const i8,
            group: 110 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"message\0" as *const u8 as *const i8,
            key: 'M' as i32,
            arg: b"STRING\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Print STRING when the end of a volume of the backup media is reached\0"
                as *const u8 as *const i8,
            group: 110 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"rsh-command\0" as *const u8 as *const i8,
            key: cpio_options::RSH_COMMAND_OPTION as i32,
            arg: b"COMMAND\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Use COMMAND instead of rsh\0" as *const u8 as *const i8,
            group: 110 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Operation modifiers valid only in copy-in mode:\0" as *const u8
                as *const i8,
            group: 200 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"nonmatching\0" as *const u8 as *const i8,
            key: 'f' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Only copy files that do not match any of the given patterns\0"
                as *const u8 as *const i8,
            group: 200 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"numeric-uid-gid\0" as *const u8 as *const i8,
            key: 'n' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"In the verbose table of contents listing, show numeric UID and GID\0"
                as *const u8 as *const i8,
            group: 200 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"pattern-file\0" as *const u8 as *const i8,
            key: 'E' as i32,
            arg: b"FILE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Read additional patterns specifying filenames to extract or list from FILE\0"
                as *const u8 as *const i8,
            group: 210 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"only-verify-crc\0" as *const u8 as *const i8,
            key: cpio_options::ONLY_VERIFY_CRC_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"When reading a CRC format archive, only verify the CRC's of each file in the archive, don't actually extract the files\0"
                as *const u8 as *const i8,
            group: 210 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"rename\0" as *const u8 as *const i8,
            key: 'r' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Interactively rename files\0" as *const u8 as *const i8,
            group: 200 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"rename-batch-file\0" as *const u8 as *const i8,
            key: cpio_options::RENAME_BATCH_FILE_OPTION as i32,
            arg: b"FILE\0" as *const u8 as *const i8,
            flags: 0x2 as i32,
            doc: b"\0" as *const u8 as *const i8,
            group: 200 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"swap\0" as *const u8 as *const i8,
            key: 'b' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Swap both halfwords of words and bytes of halfwords in the data. Equivalent to -sS\0"
                as *const u8 as *const i8,
            group: 200 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"swap-bytes\0" as *const u8 as *const i8,
            key: 's' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Swap the bytes of each halfword in the files\0" as *const u8
                as *const i8,
            group: 200 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"swap-halfwords\0" as *const u8 as *const i8,
            key: 'S' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Swap the halfwords of each word (4 bytes) in the files\0" as *const u8
                as *const i8,
            group: 200 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"to-stdout\0" as *const u8 as *const i8,
            key: cpio_options::TO_STDOUT_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Extract files to standard output\0" as *const u8 as *const i8,
            group: 200 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 'I' as i32,
            arg: b"[[USER@]HOST:]FILE-NAME\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Archive filename to use instead of standard input. Optional USER and HOST specify the user and host names in case of a remote archive\0"
                as *const u8 as *const i8,
            group: 200 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Operation modifiers valid only in copy-out mode:\0" as *const u8
                as *const i8,
            group: 300 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"append\0" as *const u8 as *const i8,
            key: 'A' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Append to an existing archive.\0" as *const u8 as *const i8,
            group: 300 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 'O' as i32,
            arg: b"[[USER@]HOST:]FILE-NAME\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Archive filename to use instead of standard output. Optional USER and HOST specify the user and host names in case of a remote archive\0"
                as *const u8 as *const i8,
            group: 300 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"renumber-inodes\0" as *const u8 as *const i8,
            key: cpio_options::RENUMBER_INODES_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Renumber inodes\0" as *const u8 as *const i8,
            group: 0,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"ignore-devno\0" as *const u8 as *const i8,
            key: cpio_options::IGNORE_DEVNO_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Don't store device numbers\0" as *const u8 as *const i8,
            group: 0,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"ignore-dirnlink\0" as *const u8 as *const i8,
            key: cpio_options::IGNORE_DIRNLINK_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"ignore number of links of a directory; always assume 2\0" as *const u8
                as *const i8,
            group: 0,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"device-independent\0" as *const u8 as *const i8,
            key: cpio_options::DEVICE_INDEPENDENT_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Create device-independent (reproducible) archives\0" as *const u8
                as *const i8,
            group: 0,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"reproducible\0" as *const u8 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0x4 as i32,
            doc: 0 as *const i8,
            group: 0,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Operation modifiers valid only in copy-pass mode:\0" as *const u8
                as *const i8,
            group: 400 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"link\0" as *const u8 as *const i8,
            key: 'l' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Link files instead of copying them, when  possible\0" as *const u8
                as *const i8,
            group: 400 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Operation modifiers valid in copy-in and copy-out modes:\0"
                as *const u8 as *const i8,
            group: 500 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"absolute-filenames\0" as *const u8 as *const i8,
            key: cpio_options::ABSOLUTE_FILENAMES_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Do not strip file system prefix components from the file names\0"
                as *const u8 as *const i8,
            group: 500 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-absolute-filenames\0" as *const u8 as *const i8,
            key: cpio_options::NO_ABSOLUTE_FILENAMES_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Create all files relative to the current directory\0" as *const u8
                as *const i8,
            group: 500 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Operation modifiers valid in copy-out and copy-pass modes:\0"
                as *const u8 as *const i8,
            group: 600 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"null\0" as *const u8 as *const i8,
            key: '0' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Filenames in the list are delimited by null characters instead of newlines\0"
                as *const u8 as *const i8,
            group: 600 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"dereference\0" as *const u8 as *const i8,
            key: 'L' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Dereference  symbolic  links  (copy  the files that they point to instead of copying the links).\0"
                as *const u8 as *const i8,
            group: 600 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"reset-access-time\0" as *const u8 as *const i8,
            key: 'a' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Reset the access times of files after reading them\0" as *const u8
                as *const i8,
            group: 600 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Operation modifiers valid in copy-in and copy-pass modes:\0"
                as *const u8 as *const i8,
            group: 700 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"preserve-modification-time\0" as *const u8 as *const i8,
            key: 'm' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Retain previous file modification times when creating files\0"
                as *const u8 as *const i8,
            group: 700 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"make-directories\0" as *const u8 as *const i8,
            key: 'd' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Create leading directories where needed\0" as *const u8 as *const i8,
            group: 700 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-preserve-owner\0" as *const u8 as *const i8,
            key: cpio_options::NO_PRESERVE_OWNER_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Do not change the ownership of the files\0" as *const u8 as *const i8,
            group: 700 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"unconditional\0" as *const u8 as *const i8,
            key: 'u' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Replace all files unconditionally\0" as *const u8 as *const i8,
            group: 700 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"sparse\0" as *const u8 as *const i8,
            key: cpio_options::SPARSE_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Write files with large blocks of zeros as sparse files\0" as *const u8
                as *const i8,
            group: 700 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: 0 as *const i8,
            group: 0,
        };
        init
    },
];
static mut input_archive_name: *mut i8 = 0 as *const i8 as *mut i8;
static mut output_archive_name: *mut i8 = 0 as *const i8 as *mut i8;
unsafe extern "C" fn warn_control(mut arg: *mut i8) -> i32 {
    static mut warn_tab_0: [warn_tab; 5] = [
        {
            let mut init = warn_tab {
                name: b"none\0" as *const u8 as *const i8 as *mut i8,
                flag: -(1 as i32) as u32 as i32,
            };
            init
        },
        {
            let mut init = warn_tab {
                name: b"truncate\0" as *const u8 as *const i8 as *mut i8,
                flag: 0x1 as i32,
            };
            init
        },
        {
            let mut init = warn_tab {
                name: b"all\0" as *const u8 as *const i8 as *mut i8,
                flag: -(1 as i32) as u32 as i32,
            };
            init
        },
        {
            let mut init = warn_tab {
                name: b"interdir\0" as *const u8 as *const i8 as *mut i8,
                flag: 0x2 as i32,
            };
            init
        },
        {
            let mut init = warn_tab {
                name: 0 as *const i8 as *mut i8,
                flag: 0,
            };
            init
        },
    ];
    let mut wt: *mut warn_tab = 0 as *mut warn_tab;
    let mut offset: i32 = 0 as i32;
    if strcmp(arg, b"none\0" as *const u8 as *const i8) == 0 as i32 {
        warn_option = 0 as i32 as u32;
        return 0 as i32;
    }
    if strlen(arg) > 2 as i32 as u64
        && memcmp(
            arg as *const libc::c_void,
            b"no-\0" as *const u8 as *const i8 as *const libc::c_void,
            3 as i32 as u64,
        ) == 0 as i32
    {
        offset = 3 as i32;
    }
    wt = warn_tab_0.as_mut_ptr();
    while !((*wt).name).is_null() {
        if strcmp(arg.offset(offset as isize), (*wt).name) == 0 as i32 {
            if offset != 0 {
                warn_option &= !(*wt).flag as u32;
            } else {
                warn_option |= (*wt).flag as u32;
            }
            return 0 as i32;
        }
        wt = wt.offset(1);
        wt;
    }
    return 1 as i32;
}
unsafe extern "C" fn parse_opt(
    mut key: i32,
    mut arg: *mut i8,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        48 => {
            name_end = '\0' as i32 as i8;
        }
        97 => {
            reset_time_flag = 1 as i32;
        }
        65 => {
            append_flag = 1 as i32;
        }
        98 => {
            swap_bytes_flag = 1 as i32;
            swap_halfwords_flag = 1 as i32;
        }
        66 => {
            io_block_size = 5120 as i32;
        }
        266 => {
            io_block_size = atoi(arg);
            if io_block_size < 1 as i32 || io_block_size > 2147483647 as i32 / 512 as i32
            {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"invalid block size\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                usage(2 as i32);
            }
            io_block_size *= 512 as i32;
        }
        99 => {
            if archive_format as u32 != archive_format::arf_unknown as i32 as u32 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"Archive format multiply defined\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                usage(2 as i32);
            }
            archive_format = archive_format::arf_oldascii;
        }
        67 => {
            io_block_size = atoi(arg);
            if io_block_size < 1 as i32 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"invalid block size\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                usage(2 as i32);
            }
        }
        100 => {
            create_dir_flag = 1 as i32;
        }
        68 => {
            change_directory_option = arg;
        }
        102 => {
            copy_matching_files = 0 as i32;
        }
        69 => {
            pattern_file_name = arg;
        }
        70 => {
            archive_name = arg;
        }
        72 => {
            if archive_format as u32 != archive_format::arf_unknown as i32 as u32 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"Archive format multiply defined\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                usage(2 as i32);
            }
            if strcasecmp(arg, b"crc\0" as *const u8 as *const i8) == 0 {
                archive_format = archive_format::arf_crcascii;
            } else if strcasecmp(arg, b"newc\0" as *const u8 as *const i8) == 0 {
                archive_format = archive_format::arf_newascii;
            } else if strcasecmp(arg, b"odc\0" as *const u8 as *const i8) == 0 {
                archive_format = archive_format::arf_oldascii;
            } else if strcasecmp(arg, b"bin\0" as *const u8 as *const i8) == 0 {
                archive_format = archive_format::arf_binary;
            } else if strcasecmp(arg, b"ustar\0" as *const u8 as *const i8) == 0 {
                archive_format = archive_format::arf_ustar;
            } else if strcasecmp(arg, b"tar\0" as *const u8 as *const i8) == 0 {
                archive_format = archive_format::arf_tar;
            } else if strcasecmp(arg, b"hpodc\0" as *const u8 as *const i8) == 0 {
                archive_format = archive_format::arf_hpoldascii;
            } else if strcasecmp(arg, b"hpbin\0" as *const u8 as *const i8) == 0 {
                archive_format = archive_format::arf_hpbinary;
            } else {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"invalid archive format `%s'; valid formats are:\ncrc newc odc bin ustar tar (all-caps also recognized)\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    arg,
                );
                usage(2 as i32);
            }
        }
        105 => {
            if copy_function.is_some() {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"Mode already defined\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                usage(2 as i32);
            }
            copy_function = ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(process_copy_in as unsafe extern "C" fn() -> ()));
        }
        73 => {
            input_archive_name = arg;
        }
        108 => {
            link_flag = 1 as i32;
        }
        76 => {
            xstat = ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*const i8, *mut stat) -> i32>,
                Option<unsafe extern "C" fn() -> i32>,
            >(Some(stat as unsafe extern "C" fn(*const i8, *mut stat) -> i32));
        }
        109 => {
            retain_time_flag = 1 as i32;
        }
        77 => {
            set_new_media_message(arg);
        }
        110 => {
            numeric_uid = 1 as i32;
        }
        256 => {
            no_abs_paths_flag = 1 as i32;
        }
        257 => {
            no_abs_paths_flag = 0 as i32;
        }
        258 => {
            if set_owner_flag != 0 || set_group_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"--no-preserve-owner cannot be used with --owner\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                usage(2 as i32);
            }
            no_chown_flag = 1 as i32;
        }
        111 => {
            if copy_function.is_some() {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"Mode already defined\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                usage(2 as i32);
            }
            copy_function = ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(process_copy_out as unsafe extern "C" fn() -> ()));
        }
        79 => {
            output_archive_name = arg;
        }
        259 => {
            only_verify_crc_flag = 1 as i32;
        }
        112 => {
            if copy_function.is_some() {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"Mode already defined\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                usage(2 as i32);
            }
            copy_function = ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(process_copy_pass as unsafe extern "C" fn() -> ()));
        }
        269 => {
            ignore_devno_option = 1 as i32;
        }
        268 => {
            renumber_inodes_option = 1 as i32;
        }
        270 => {
            ignore_dirnlink_option = 1 as i32;
        }
        271 => {
            ignore_dirnlink_option = 1 as i32;
            renumber_inodes_option = ignore_dirnlink_option;
            ignore_devno_option = renumber_inodes_option;
        }
        261 => {
            rsh_command_option = arg;
        }
        114 => {
            rename_flag = 1 as i32;
        }
        260 => {
            rename_batch_file = arg;
        }
        262 => {
            quiet_flag = 1 as i32;
        }
        82 => {
            if no_chown_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"--owner cannot be used with --no-preserve-owner\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                usage(2 as i32);
            } else {
                let mut e: *const i8 = 0 as *const i8;
                let mut u: *mut i8 = 0 as *mut i8;
                let mut g: *mut i8 = 0 as *mut i8;
                e = parse_user_spec(arg, &mut set_owner, &mut set_group, &mut u, &mut g);
                if !e.is_null() {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    error(
                        0 as i32,
                        0 as i32,
                        b"%s: %s\0" as *const u8 as *const i8,
                        arg,
                        e,
                    );
                    usage(2 as i32);
                }
                if !u.is_null() {
                    rpl_free(u as *mut libc::c_void);
                    set_owner_flag = 1 as i32;
                }
                if !g.is_null() {
                    rpl_free(g as *mut libc::c_void);
                    set_group_flag = 1 as i32;
                }
            }
        }
        115 => {
            swap_bytes_flag = 1 as i32;
        }
        83 => {
            swap_halfwords_flag = 1 as i32;
        }
        116 => {
            table_flag = 1 as i32;
        }
        117 => {
            unconditional_flag = 1 as i32;
        }
        118 => {
            verbose_flag = 1 as i32;
        }
        86 => {
            dot_flag = 1 as i32;
        }
        87 => {
            if warn_control(arg) != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"Invalid value for --warning option: %s\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    arg,
                );
                usage(2 as i32);
            }
        }
        263 => {
            sparse_flag = 1 as i32;
        }
        264 => {
            force_local_option = 1 as i32 != 0;
        }
        267 => {
            to_stdout_option = 1 as i32 != 0;
        }
        _ => return 7 as i32,
    }
    return 0 as i32;
}
static mut argp: argp = unsafe {
    {
        let mut init = argp {
            options: options.as_ptr() as *mut _,
            parser: Some(
                parse_opt
                    as unsafe extern "C" fn(i32, *mut i8, *mut argp_state) -> error_t,
            ),
            args_doc: b"[destination-directory]\0" as *const u8 as *const i8,
            doc: doc.as_ptr() as *mut _,
            children: 0 as *const argp_child,
            help_filter: None,
            argp_domain: 0 as *const i8,
        };
        init
    }
};
unsafe extern "C" fn usage(mut status: i32) {
    argp_help(&mut argp, stderr, 0x4 as i32 as u32, program_name as *mut i8);
    close_stdout();
    exit(status);
}
#[no_mangle]
pub unsafe extern "C" fn process_args(mut argc: i32, mut argv: *mut *mut i8) {
    let mut index: i32 = 0;
    xstat = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const i8, *mut stat) -> i32>,
        Option<unsafe extern "C" fn() -> i32>,
    >(Some(lstat as unsafe extern "C" fn(*const i8, *mut stat) -> i32));
    if argp_parse(
        &mut argp,
        argc,
        argv,
        0x8 as i32 as u32,
        &mut index,
        0 as *mut libc::c_void,
    ) != 0
    {
        exit(2 as i32);
    }
    if copy_function.is_none() {
        if table_flag != 0 {
            copy_function = ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(process_copy_in as unsafe extern "C" fn() -> ()));
        } else {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"You must specify one of -oipt options.\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            usage(2 as i32);
        }
    }
    if copy_function
        == ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(process_copy_in as unsafe extern "C" fn() -> ()))
    {
        archive_des = 0 as i32;
        if link_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--link\0" as *const u8 as *const i8,
                b"--extract\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if reset_time_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--reset\0" as *const u8 as *const i8,
                b"--extract\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if xstat
            != ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*const i8, *mut stat) -> i32>,
                Option<unsafe extern "C" fn() -> i32>,
            >(Some(lstat as unsafe extern "C" fn(*const i8, *mut stat) -> i32))
        {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--dereference\0" as *const u8 as *const i8,
                b"--extract\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if append_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--append\0" as *const u8 as *const i8,
                b"--extract\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if !output_archive_name.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"-O\0" as *const u8 as *const i8,
                b"--extract\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if renumber_inodes_option != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--renumber-inodes\0" as *const u8 as *const i8,
                b"--extract\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if ignore_devno_option != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--ignore-devno\0" as *const u8 as *const i8,
                b"--extract\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if to_stdout_option {
            if create_dir_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s is meaningless with %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    b"--make-directories\0" as *const u8 as *const i8,
                    b"--to-stdout\0" as *const u8 as *const i8,
                );
                usage(2 as i32);
            }
            if rename_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s is meaningless with %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    b"--rename\0" as *const u8 as *const i8,
                    b"--to-stdout\0" as *const u8 as *const i8,
                );
                usage(2 as i32);
            }
            if no_chown_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s is meaningless with %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    b"--no-preserve-owner\0" as *const u8 as *const i8,
                    b"--to-stdout\0" as *const u8 as *const i8,
                );
                usage(2 as i32);
            }
            if set_owner_flag != 0 || set_group_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s is meaningless with %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    b"--owner\0" as *const u8 as *const i8,
                    b"--to-stdout\0" as *const u8 as *const i8,
                );
                usage(2 as i32);
            }
            if retain_time_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s is meaningless with %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    b"--preserve-modification-time\0" as *const u8 as *const i8,
                    b"--to-stdout\0" as *const u8 as *const i8,
                );
                usage(2 as i32);
            }
        }
        if !archive_name.is_null() && !input_archive_name.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Both -I and -F are used in copy-in mode\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            usage(2 as i32);
        }
        if archive_format as u32 == archive_format::arf_crcascii as i32 as u32 {
            crc_i_flag = 1 as i32;
        }
        num_patterns = argc - index;
        save_patterns = &mut *argv.offset(index as isize) as *mut *mut i8;
        if !input_archive_name.is_null() {
            archive_name = input_archive_name;
        }
    } else if copy_function
        == ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(process_copy_out as unsafe extern "C" fn() -> ()))
    {
        if index != argc {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Too many arguments\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            usage(2 as i32);
        }
        archive_des = 1 as i32;
        if create_dir_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--make-directories\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if rename_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--rename\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if table_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--list\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if unconditional_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--unconditional\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if link_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--link\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if sparse_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--sparse\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if retain_time_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--preserve-modification-time\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if no_chown_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--no-preserve-owner\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if swap_bytes_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--swap-bytes (--swap)\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if swap_halfwords_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--swap-halfwords (--swap)\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if to_stdout_option {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--to-stdout\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if append_flag != 0
            && !(!archive_name.is_null() || !output_archive_name.is_null())
        {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"--append is used but no archive file name is given (use -F or -O options)\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            usage(2 as i32);
        }
        if !rename_batch_file.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--rename-batch-file\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if !input_archive_name.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"-I\0" as *const u8 as *const i8,
                b"--create\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if !archive_name.is_null() && !output_archive_name.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Both -O and -F are used in copy-out mode\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            usage(2 as i32);
        }
        if archive_format as u32 == archive_format::arf_unknown as i32 as u32 {
            archive_format = archive_format::arf_binary;
        }
        if !output_archive_name.is_null() {
            archive_name = output_archive_name;
        }
        if arf_stores_inode_p(archive_format) == 0 {
            ignore_devno_option = 0 as i32;
            renumber_inodes_option = ignore_devno_option;
        }
    } else {
        if index < argc - 1 as i32 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Too many arguments\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            usage(2 as i32);
        } else if index > argc - 1 as i32 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Not enough arguments\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            usage(2 as i32);
        }
        if archive_format as u32 != archive_format::arf_unknown as i32 as u32 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Archive format is not specified in copy-pass mode (use --format option)\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            usage(2 as i32);
        }
        if swap_bytes_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--swap-bytes (--swap)\0" as *const u8 as *const i8,
                b"--pass-through\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if swap_halfwords_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--swap-halfwords (--swap)\0" as *const u8 as *const i8,
                b"--pass-through\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if table_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--list\0" as *const u8 as *const i8,
                b"--pass-through\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if rename_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--rename\0" as *const u8 as *const i8,
                b"--pass-through\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if append_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--append\0" as *const u8 as *const i8,
                b"--pass-through\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if !rename_batch_file.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--rename-batch-file\0" as *const u8 as *const i8,
                b"--pass-through\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if no_abs_paths_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--no-absolute-pathnames\0" as *const u8 as *const i8,
                b"--pass-through\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if no_abs_paths_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--absolute-pathnames\0" as *const u8 as *const i8,
                b"--pass-through\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if to_stdout_option {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--to-stdout\0" as *const u8 as *const i8,
                b"--pass-through\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if renumber_inodes_option != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--renumber-inodes\0" as *const u8 as *const i8,
                b"--pass-through\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        if ignore_devno_option != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is meaningless with %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"--ignore-devno\0" as *const u8 as *const i8,
                b"--pass-through\0" as *const u8 as *const i8,
            );
            usage(2 as i32);
        }
        directory_name = *argv.offset(index as isize);
    }
    if !archive_name.is_null() {
        if copy_function
            != ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(process_copy_in as unsafe extern "C" fn() -> ()))
            && copy_function
                != ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(process_copy_out as unsafe extern "C" fn() -> ()))
        {
            error(
                2 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"-F can be used only with --create or --extract\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        archive_des = open_archive(archive_name);
        if archive_des < 0 as i32 {
            error(
                2 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Cannot open %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quotearg_colon(archive_name),
            );
        }
    }
    if set_owner_flag == 0 as i32 && set_group_flag == 0 as i32 && geteuid() != 0 {
        no_chown_flag = 1 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn initialize_buffers() {
    let mut in_buf_size: i32 = 0;
    let mut out_buf_size: i32 = 0;
    if copy_function
        == ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(process_copy_in as unsafe extern "C" fn() -> ()))
    {
        if io_block_size >= 512 as i32 {
            in_buf_size = 2 as i32 * io_block_size;
        } else {
            in_buf_size = 1024 as i32;
        }
        out_buf_size = 512 as i32;
    } else if copy_function
        == ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(process_copy_out as unsafe extern "C" fn() -> ()))
    {
        in_buf_size = 512 as i32;
        out_buf_size = io_block_size;
    } else {
        in_buf_size = 512 as i32;
        out_buf_size = 512 as i32;
    }
    input_buffer = xmalloc(in_buf_size as size_t) as *mut i8;
    in_buff = input_buffer;
    input_buffer_size = in_buf_size as size_t;
    input_size = 0 as i32 as size_t;
    input_bytes = 0 as i32 as off_t;
    output_buffer = xmalloc(out_buf_size as size_t) as *mut i8;
    out_buff = output_buffer;
    output_size = 0 as i32 as size_t;
    output_bytes = 0 as i32 as off_t;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    setlocale(6 as i32, b"\0" as *const u8 as *const i8);
    bindtextdomain(
        b"cpio\0" as *const u8 as *const i8,
        b"/usr/local/share/locale\0" as *const u8 as *const i8,
    );
    textdomain(b"cpio\0" as *const u8 as *const i8);
    set_program_name(*argv.offset(0 as i32 as isize));
    argp_version_setup(
        b"cpio\0" as *const u8 as *const i8,
        program_authors.as_mut_ptr(),
    );
    process_args(argc, argv);
    initialize_buffers();
    ::core::mem::transmute::<
        _,
        fn(),
    >(
        (Some(copy_function.expect("non-null function pointer")))
            .expect("non-null function pointer"),
    )();
    if archive_des >= 0 as i32
        && (if archive_des >= (1 as i32) << 30 as i32 {
            rmt_close__(archive_des - ((1 as i32) << 30 as i32))
        } else {
            close(archive_des)
        }) == -(1 as i32)
    {
        error(
            2 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"error closing archive\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    pax_exit();
    return 0;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}