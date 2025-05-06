#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn rpl_free(ptr: *mut libc::c_void);
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn read_file_system_list(need_fs_type: bool) -> *mut mount_entry;
    fn free_mount_entry(entry: *mut mount_entry);
    fn set_stat_placeholders(p: *mut stat);
    static mut options: options;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn extendbuf(
        existing: *mut libc::c_void,
        wanted: size_t,
        allocated: *mut size_t,
    ) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type dev_t = __dev_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mount_entry {
    pub me_devname: *mut i8,
    pub me_mountdir: *mut i8,
    pub me_mntroot: *mut i8,
    pub me_type: *mut i8,
    pub me_dev: dev_t,
    #[bitfield(name = "me_dummy", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "me_remote", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "me_type_malloced", ty = "libc::c_uint", bits = "2..=2")]
    pub me_dummy_me_remote_me_type_malloced: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub me_next: *mut mount_entry,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum quoting_style {
    literal_quoting_style,
    shell_quoting_style,
    shell_always_quoting_style,
    shell_escape_quoting_style,
    shell_escape_always_quoting_style,
    c_quoting_style,
    c_maybe_quoting_style,
    escape_quoting_style,
    locale_quoting_style,
    clocale_quoting_style,
    custom_quoting_style,
}
impl quoting_style {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            quoting_style::literal_quoting_style => 0,
            quoting_style::shell_quoting_style => 1,
            quoting_style::shell_always_quoting_style => 2,
            quoting_style::shell_escape_quoting_style => 3,
            quoting_style::shell_escape_always_quoting_style => 4,
            quoting_style::c_quoting_style => 5,
            quoting_style::c_maybe_quoting_style => 6,
            quoting_style::escape_quoting_style => 7,
            quoting_style::locale_quoting_style => 8,
            quoting_style::clocale_quoting_style => 9,
            quoting_style::custom_quoting_style => 10,
        }
    }
    fn from_libc_c_uint(value: u32) -> quoting_style {
        match value {
            0 => quoting_style::literal_quoting_style,
            1 => quoting_style::shell_quoting_style,
            2 => quoting_style::shell_always_quoting_style,
            3 => quoting_style::shell_escape_quoting_style,
            4 => quoting_style::shell_escape_always_quoting_style,
            5 => quoting_style::c_quoting_style,
            6 => quoting_style::c_maybe_quoting_style,
            7 => quoting_style::escape_quoting_style,
            8 => quoting_style::locale_quoting_style,
            9 => quoting_style::clocale_quoting_style,
            10 => quoting_style::custom_quoting_style,
            _ => panic!("Invalid value for quoting_style: {}", value),
        }
    }
}
impl AddAssign<u32> for quoting_style {
    fn add_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for quoting_style {
    fn sub_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for quoting_style {
    fn mul_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for quoting_style {
    fn div_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for quoting_style {
    fn rem_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for quoting_style {
    type Output = quoting_style;
    fn add(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for quoting_style {
    type Output = quoting_style;
    fn sub(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for quoting_style {
    type Output = quoting_style;
    fn mul(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for quoting_style {
    type Output = quoting_style;
    fn div(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for quoting_style {
    type Output = quoting_style;
    fn rem(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options {
    pub do_dir_first: bool,
    pub explicit_depth: bool,
    pub maxdepth: i32,
    pub mindepth: i32,
    pub no_leaf_check: bool,
    pub stay_on_filesystem: bool,
    pub ignore_readdir_race: bool,
    pub literal_control_chars: bool,
    pub warnings: bool,
    pub posixly_correct: bool,
    pub start_time: timespec,
    pub cur_day_start: timespec,
    pub full_days: bool,
    pub output_block_size: i32,
    pub debug_options: u64,
    pub symlink_handling: SymlinkOption,
    pub xstat: Option<unsafe extern "C" fn(*const i8, *mut stat) -> i32>,
    pub open_nofollow_available: bool,
    pub regex_options: i32,
    pub x_getfilecon: Option<unsafe extern "C" fn(i32, *const i8, *mut *mut i8) -> i32>,
    pub optimisation_level: libc::c_ushort,
    pub err_quoting_style: quoting_style,
    pub files0_from: *const i8,
    pub ok_prompt_stdin: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum SymlinkOption {
    SYMLINK_NEVER_DEREF,
    SYMLINK_ALWAYS_DEREF,
    SYMLINK_DEREF_ARGSONLY,
}
impl SymlinkOption {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            SymlinkOption::SYMLINK_NEVER_DEREF => 0,
            SymlinkOption::SYMLINK_ALWAYS_DEREF => 1,
            SymlinkOption::SYMLINK_DEREF_ARGSONLY => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> SymlinkOption {
        match value {
            0 => SymlinkOption::SYMLINK_NEVER_DEREF,
            1 => SymlinkOption::SYMLINK_ALWAYS_DEREF,
            2 => SymlinkOption::SYMLINK_DEREF_ARGSONLY,
            _ => panic!("Invalid value for SymlinkOption: {}", value),
        }
    }
}
impl AddAssign<u32> for SymlinkOption {
    fn add_assign(&mut self, rhs: u32) {
        *self = SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for SymlinkOption {
    fn sub_assign(&mut self, rhs: u32) {
        *self = SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for SymlinkOption {
    fn mul_assign(&mut self, rhs: u32) {
        *self = SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for SymlinkOption {
    fn div_assign(&mut self, rhs: u32) {
        *self = SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for SymlinkOption {
    fn rem_assign(&mut self, rhs: u32) {
        *self = SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for SymlinkOption {
    type Output = SymlinkOption;
    fn add(self, rhs: u32) -> SymlinkOption {
        SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for SymlinkOption {
    type Output = SymlinkOption;
    fn sub(self, rhs: u32) -> SymlinkOption {
        SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for SymlinkOption {
    type Output = SymlinkOption;
    fn mul(self, rhs: u32) -> SymlinkOption {
        SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for SymlinkOption {
    type Output = SymlinkOption;
    fn div(self, rhs: u32) -> SymlinkOption {
        SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for SymlinkOption {
    type Output = SymlinkOption;
    fn rem(self, rhs: u32) -> SymlinkOption {
        SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _gl_dummy: i32,
}
unsafe extern "C" fn free_file_system_list(mut p: *mut mount_entry) {
    while !p.is_null() {
        let mut pnext: *mut mount_entry = (*p).me_next;
        free_mount_entry(p);
        p = pnext;
    }
}
unsafe extern "C" fn get_file_system_list(mut need_fs_type: bool) -> *mut mount_entry {
    static mut mount_list: *mut mount_entry = 0 as *const mount_entry
        as *mut mount_entry;
    static mut has_fstype: bool = 0 as i32 != 0;
    if !mount_list.is_null() && !has_fstype && need_fs_type as i32 != 0 {
        free_file_system_list(mount_list);
        mount_list = 0 as *mut mount_entry;
    }
    if mount_list.is_null() {
        mount_list = read_file_system_list(need_fs_type);
        has_fstype = need_fs_type;
    }
    return mount_list;
}
#[no_mangle]
pub unsafe extern "C" fn filesystem_type(
    mut statp: *const stat,
    mut path: *const i8,
) -> *mut i8 {
    static mut fstype_known: bool = 0 as i32 != 0;
    static mut current_fstype: *mut i8 = 0 as *const i8 as *mut i8;
    static mut current_dev: dev_t = 0;
    if !current_fstype.is_null() {
        if fstype_known as i32 != 0 && (*statp).st_dev == current_dev {
            return current_fstype;
        }
        rpl_free(current_fstype as *mut libc::c_void);
    }
    current_dev = (*statp).st_dev;
    current_fstype = file_system_type_uncached(statp, path, &mut fstype_known);
    return current_fstype;
}
#[no_mangle]
pub unsafe extern "C" fn is_used_fs_type(mut name: *const i8) -> bool {
    if 0 as i32 == strcmp(b"afs\0" as *const u8 as *const i8, name) {
        return 1 as i32 != 0
    } else {
        let mut entries: *const mount_entry = get_file_system_list(0 as i32 != 0);
        if !entries.is_null() {
            let mut entry: *const mount_entry = 0 as *const mount_entry;
            entry = entries;
            while !entry.is_null() {
                if 0 as i32 == strcmp(name, (*entry).me_type) {
                    return 1 as i32 != 0;
                }
                entry = (*entry).me_next;
            }
        } else {
            return 1 as i32 != 0
        }
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn set_fstype_devno(mut p: *mut mount_entry) -> i32 {
    let mut stbuf: stat = stat {
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
    if (*p).me_dev == -(1 as i32) as dev_t {
        set_stat_placeholders(&mut stbuf);
        if 0 as i32
            == (options.xstat)
                .expect("non-null function pointer")((*p).me_mountdir, &mut stbuf)
        {
            (*p).me_dev = stbuf.st_dev;
            return 0 as i32;
        } else {
            return -(1 as i32)
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn file_system_type_uncached(
    mut statp: *const stat,
    mut path: *const i8,
    mut fstype_known: *mut bool,
) -> *mut i8 {
    let mut entries: *mut mount_entry = 0 as *mut mount_entry;
    let mut entry: *mut mount_entry = 0 as *mut mount_entry;
    let mut best: *mut mount_entry = 0 as *mut mount_entry;
    let mut type_0: *mut i8 = 0 as *mut i8;
    best = 0 as *mut mount_entry;
    entries = get_file_system_list(1 as i32 != 0);
    if entries.is_null() {
        if ::core::mem::size_of::<C2RustUnnamed>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Cannot read mounted file system list\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Cannot read mounted file system list\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    type_0 = 0 as *mut i8;
    entry = entries;
    while !entry.is_null() {
        if !(strcmp((*entry).me_type, b"ignore\0" as *const u8 as *const i8) == 0) {
            if 0 as i32 == set_fstype_devno(entry) {
                if (*entry).me_dev == (*statp).st_dev {
                    best = entry;
                }
            }
        }
        entry = (*entry).me_next;
    }
    if !best.is_null() {
        type_0 = xstrdup((*best).me_type);
    }
    *fstype_known = !type_0.is_null();
    return if !type_0.is_null() {
        type_0
    } else {
        xstrdup(
            dcgettext(0 as *const i8, b"unknown\0" as *const u8 as *const i8, 5 as i32),
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_mounted_devices(mut n: *mut size_t) -> *mut dev_t {
    let mut alloc_size: size_t = 0 as u32 as size_t;
    let mut used: size_t = 0 as u32 as size_t;
    let mut entries: *mut mount_entry = 0 as *mut mount_entry;
    let mut entry: *mut mount_entry = 0 as *mut mount_entry;
    let mut result: *mut dev_t = 0 as *mut dev_t;
    entries = read_file_system_list(0 as i32 != 0);
    entry = entries;
    while !entry.is_null() {
        let mut p: *mut libc::c_void = extendbuf(
            result as *mut libc::c_void,
            (::core::mem::size_of::<dev_t>() as u64)
                .wrapping_mul(used.wrapping_add(1 as i32 as u64)),
            &mut alloc_size,
        );
        if !p.is_null() {
            result = p as *mut dev_t;
            if 0 as i32 == set_fstype_devno(entry) {
                *result.offset(used as isize) = (*entry).me_dev;
                used = used.wrapping_add(1);
                used;
            }
        } else {
            rpl_free(result as *mut libc::c_void);
            result = 0 as *mut dev_t;
        }
        entry = (*entry).me_next;
    }
    free_file_system_list(entries);
    if !result.is_null() {
        *n = used;
    }
    return result;
}