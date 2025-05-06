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
    pub type dep;
    pub type commands;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    static mut stdout: *mut _IO_FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn mempcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn find_percent(_: *mut i8) -> *mut i8;
    fn dir_file_exists_p(_: *const i8, _: *const i8) -> i32;
    fn dir_name(_: *const i8) -> *const i8;
    fn strcache_add(str: *const i8) -> *const i8;
    static mut stopchar_map: [libc::c_ushort; 0];
    fn strcache_add_len(str: *const i8, len: size_t) -> *const i8;
    fn lookup_file(name: *const i8) -> *mut file;
    fn file_timestamp_cons(_: *const i8, _: time_t, _: i64) -> uintmax_t;
    fn variable_expand(line: *const i8) -> *mut i8;
    fn pattern_matches(pattern: *const i8, percent: *const i8, str: *const i8) -> i32;
}
pub type size_t = u64;
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
pub type __syscall_slong_t = i64;
pub type time_t = __time_t;
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
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const i8,
    pub hname: *const i8,
    pub vpath: *const i8,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const i8,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: u32,
    pub command_flags: i32,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "low_resolution_time", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "tried_implicit", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "libc::c_uint", bits = "24..=24")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum cmd_state {
    cs_finished = 3,
    cs_running = 2,
    cs_deps_running = 1,
    cs_not_started = 0,
}
impl cmd_state {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            cmd_state::cs_finished => 3,
            cmd_state::cs_running => 2,
            cmd_state::cs_deps_running => 1,
            cmd_state::cs_not_started => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> cmd_state {
        match value {
            3 => cmd_state::cs_finished,
            2 => cmd_state::cs_running,
            1 => cmd_state::cs_deps_running,
            0 => cmd_state::cs_not_started,
            _ => panic!("Invalid value for cmd_state: {}", value),
        }
    }
}
impl AddAssign<u32> for cmd_state {
    fn add_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for cmd_state {
    fn sub_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for cmd_state {
    fn mul_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for cmd_state {
    fn div_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for cmd_state {
    fn rem_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for cmd_state {
    type Output = cmd_state;
    fn add(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for cmd_state {
    type Output = cmd_state;
    fn sub(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for cmd_state {
    type Output = cmd_state;
    fn mul(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for cmd_state {
    type Output = cmd_state;
    fn div(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for cmd_state {
    type Output = cmd_state;
    fn rem(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum update_status {
    us_failed = 3,
    us_question = 2,
    us_none = 1,
    us_success = 0,
}
impl update_status {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            update_status::us_failed => 3,
            update_status::us_question => 2,
            update_status::us_none => 1,
            update_status::us_success => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> update_status {
        match value {
            3 => update_status::us_failed,
            2 => update_status::us_question,
            1 => update_status::us_none,
            0 => update_status::us_success,
            _ => panic!("Invalid value for update_status: {}", value),
        }
    }
}
impl AddAssign<u32> for update_status {
    fn add_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for update_status {
    fn sub_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for update_status {
    fn mul_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for update_status {
    fn div_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for update_status {
    fn rem_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for update_status {
    type Output = update_status;
    fn add(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for update_status {
    type Output = update_status;
    fn sub(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for update_status {
    type Output = update_status;
    fn mul(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for update_status {
    type Output = update_status;
    fn div(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for update_status {
    type Output = update_status;
    fn rem(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set_list {
    pub next: *mut variable_set_list,
    pub set: *mut variable_set,
    pub next_is_parent: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set {
    pub table: hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut libc::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: u64,
    pub ht_capacity: u64,
    pub ht_fill: u64,
    pub ht_empty_slots: u64,
    pub ht_collisions: u64,
    pub ht_lookups: u64,
    pub ht_rehashes: u32,
}
pub type hash_cmp_func_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type hash_func_t = Option<unsafe extern "C" fn(*const libc::c_void) -> u64>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpath {
    pub next: *mut vpath,
    pub pattern: *const i8,
    pub percent: *const i8,
    pub patlen: size_t,
    pub searchpath: *mut *const i8,
    pub maxlen: size_t,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
static mut vpaths: *mut vpath = 0 as *const vpath as *mut vpath;
static mut general_vpath: *mut vpath = 0 as *const vpath as *mut vpath;
static mut gpaths: *mut vpath = 0 as *const vpath as *mut vpath;
#[no_mangle]
pub unsafe extern "C" fn build_vpath_lists() {
    let mut new: *mut vpath = 0 as *mut vpath;
    let mut old: *mut vpath = 0 as *mut vpath;
    let mut nexto: *mut vpath = 0 as *mut vpath;
    let mut p: *mut i8 = 0 as *mut i8;
    old = vpaths;
    while !old.is_null() {
        nexto = (*old).next;
        (*old).next = new;
        new = old;
        old = nexto;
    }
    vpaths = new;
    p = variable_expand(b"$(strip $(VPATH))\0" as *const u8 as *const i8);
    if *p as i32 != '\0' as i32 {
        let mut save_vpaths: *mut vpath = vpaths;
        let mut gp: [i8; 2] = *::core::mem::transmute::<&[u8; 2], &mut [i8; 2]>(b"%\0");
        vpaths = 0 as *mut vpath;
        construct_vpath_list(gp.as_mut_ptr(), p);
        general_vpath = vpaths;
        vpaths = save_vpaths;
    }
    p = variable_expand(b"$(strip $(GPATH))\0" as *const u8 as *const i8);
    if *p as i32 != '\0' as i32 {
        let mut save_vpaths_0: *mut vpath = vpaths;
        let mut gp_0: [i8; 2] = *::core::mem::transmute::<
            &[u8; 2],
            &mut [i8; 2],
        >(b"%\0");
        vpaths = 0 as *mut vpath;
        construct_vpath_list(gp_0.as_mut_ptr(), p);
        gpaths = vpaths;
        vpaths = save_vpaths_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn construct_vpath_list(
    mut pattern: *mut i8,
    mut dirpath: *mut i8,
) {
    let mut elem: u32 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut vpath: *mut *const i8 = 0 as *mut *const i8;
    let mut maxvpath: size_t = 0;
    let mut maxelem: u32 = 0;
    let mut percent: *const i8 = 0 as *const i8;
    if !pattern.is_null() {
        percent = find_percent(pattern);
    }
    if dirpath.is_null() {
        let mut path: *mut vpath = 0 as *mut vpath;
        let mut lastpath: *mut vpath = 0 as *mut vpath;
        lastpath = 0 as *mut vpath;
        path = vpaths;
        while !path.is_null() {
            let mut next: *mut vpath = (*path).next;
            if pattern.is_null()
                || (percent.is_null() && ((*path).percent).is_null()
                    || percent.offset_from(pattern) as i64
                        == ((*path).percent).offset_from((*path).pattern) as i64)
                    && (pattern == (*path).pattern as *mut i8
                        || *pattern as i32 == *(*path).pattern as i32
                            && (*pattern as i32 == '\0' as i32
                                || strcmp(
                                    pattern.offset(1 as i32 as isize),
                                    ((*path).pattern).offset(1 as i32 as isize),
                                ) == 0))
            {
                if lastpath.is_null() {
                    vpaths = (*path).next;
                } else {
                    (*lastpath).next = next;
                }
                free((*path).searchpath as *mut libc::c_void);
                free(path as *mut libc::c_void);
            } else {
                lastpath = path;
            }
            path = next;
        }
        return;
    }
    while *stopchar_map.as_mut_ptr().offset(*dirpath as u8 as isize) as i32
        & (0x2 as i32 | 0x40 as i32) != 0 as i32
    {
        dirpath = dirpath.offset(1);
        dirpath;
    }
    maxelem = 2 as i32 as u32;
    p = dirpath;
    while *p as i32 != '\0' as i32 {
        let fresh0 = p;
        p = p.offset(1);
        if *stopchar_map.as_mut_ptr().offset(*fresh0 as u8 as isize) as i32
            & (0x2 as i32 | 0x40 as i32) != 0 as i32
        {
            maxelem = maxelem.wrapping_add(1);
            maxelem;
        }
    }
    vpath = xmalloc(
        (maxelem as u64).wrapping_mul(::core::mem::size_of::<*const i8>() as u64),
    ) as *mut *const i8;
    maxvpath = 0 as i32 as size_t;
    elem = 0 as i32 as u32;
    p = dirpath;
    while *p as i32 != '\0' as i32 {
        let mut v: *mut i8 = 0 as *mut i8;
        let mut len: size_t = 0;
        v = p;
        while *p as i32 != '\0' as i32 && *p as i32 != ':' as i32
            && !(*stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32 & 0x2 as i32
                != 0 as i32)
        {
            p = p.offset(1);
            p;
        }
        len = p.offset_from(v) as i64 as size_t;
        if len > 1 as i32 as u64 && *p.offset(-(1 as i32) as isize) as i32 == '/' as i32
        {
            len = len.wrapping_sub(1);
            len;
        }
        if len > 1 as i32 as u64 || *v as i32 != '.' as i32 {
            let fresh1 = elem;
            elem = elem.wrapping_add(1);
            let ref mut fresh2 = *vpath.offset(fresh1 as isize);
            *fresh2 = dir_name(strcache_add_len(v, len));
            if len > maxvpath {
                maxvpath = len;
            }
        }
        while *stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
            & (0x2 as i32 | 0x40 as i32) != 0 as i32
        {
            p = p.offset(1);
            p;
        }
    }
    if elem > 0 as i32 as u32 {
        let mut path_0: *mut vpath = 0 as *mut vpath;
        if elem < maxelem.wrapping_sub(1 as i32 as u32) {
            vpath = xrealloc(
                vpath as *mut libc::c_void,
                (elem.wrapping_add(1 as i32 as u32) as u64)
                    .wrapping_mul(::core::mem::size_of::<*const i8>() as u64),
            ) as *mut *const i8;
        }
        let ref mut fresh3 = *vpath.offset(elem as isize);
        *fresh3 = 0 as *const i8;
        path_0 = xmalloc(::core::mem::size_of::<vpath>() as u64) as *mut vpath;
        (*path_0).searchpath = vpath;
        (*path_0).maxlen = maxvpath;
        (*path_0).next = vpaths;
        vpaths = path_0;
        (*path_0).pattern = strcache_add(pattern);
        (*path_0).patlen = strlen(pattern);
        (*path_0).percent = if !percent.is_null() {
            ((*path_0).pattern).offset(percent.offset_from(pattern) as i64 as isize)
        } else {
            0 as *const i8
        };
    } else {
        free(vpath as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gpath_search(mut file: *const i8, mut len: size_t) -> i32 {
    if !gpaths.is_null() && len <= (*gpaths).maxlen {
        let mut gp: *mut *const i8 = 0 as *mut *const i8;
        gp = (*gpaths).searchpath;
        while !(*gp).is_null() {
            if strncmp(*gp, file, len) == 0 as i32
                && *(*gp).offset(len as isize) as i32 == '\0' as i32
            {
                return 1 as i32;
            }
            gp = gp.offset(1);
            gp;
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn selective_vpath_search(
    mut path: *mut vpath,
    mut file: *const i8,
    mut mtime_ptr: *mut uintmax_t,
    mut path_index: *mut u32,
) -> *const i8 {
    let mut not_target: i32 = 0;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut n: *const i8 = 0 as *const i8;
    let mut filename: *const i8 = 0 as *const i8;
    let mut vpath: *mut *const i8 = (*path).searchpath;
    let mut maxvpath: size_t = (*path).maxlen;
    let mut i: u32 = 0;
    let mut flen: size_t = 0;
    let mut name_dplen: size_t = 0;
    let mut exists: i32 = 0 as i32;
    let mut f: *mut file = lookup_file(file);
    not_target = (f.is_null() || (*f).is_target() == 0) as i32;
    flen = strlen(file);
    n = strrchr(file, '/' as i32);
    name_dplen = (if !n.is_null() {
        n.offset_from(file) as i64
    } else {
        0 as i32 as i64
    }) as size_t;
    filename = if name_dplen > 0 as i32 as u64 {
        n.offset(1 as i32 as isize)
    } else {
        file
    };
    if name_dplen > 0 as i32 as u64 {
        flen = (flen as u64).wrapping_sub(name_dplen.wrapping_add(1 as i32 as u64))
            as size_t as size_t;
    }
    let mut fresh4 = ::std::vec::from_elem(
        0,
        maxvpath
            .wrapping_add(1 as i32 as u64)
            .wrapping_add(name_dplen)
            .wrapping_add(1 as i32 as u64)
            .wrapping_add(flen)
            .wrapping_add(1 as i32 as u64) as usize,
    );
    name = fresh4.as_mut_ptr() as *mut i8;
    let mut current_block_45: u64;
    i = 0 as i32 as u32;
    while !(*vpath.offset(i as isize)).is_null() {
        let mut exists_in_cache: i32 = 0 as i32;
        let mut p: *mut i8 = name;
        let mut vlen: size_t = strlen(*vpath.offset(i as isize));
        p = mempcpy(
            p as *mut libc::c_void,
            *vpath.offset(i as isize) as *const libc::c_void,
            vlen,
        ) as *mut i8;
        if name_dplen > 0 as i32 as u64 {
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = '/' as i32 as i8;
            p = mempcpy(p as *mut libc::c_void, file as *const libc::c_void, name_dplen)
                as *mut i8;
        }
        if p != name && *p.offset(-(1 as i32) as isize) as i32 != '/' as i32 {
            *p = '/' as i32 as i8;
            memcpy(
                p.offset(1 as i32 as isize) as *mut libc::c_void,
                filename as *const libc::c_void,
                flen.wrapping_add(1 as i32 as u64),
            );
        } else {
            memcpy(
                p as *mut libc::c_void,
                filename as *const libc::c_void,
                flen.wrapping_add(1 as i32 as u64),
            );
        }
        let mut f_0: *mut file = lookup_file(name);
        if !f_0.is_null() {
            exists = (not_target != 0 || (*f_0).is_target() as i32 != 0) as i32;
            if exists != 0 && !mtime_ptr.is_null()
                && ((*f_0).last_mtime == 2 as i32 as u64
                    || (*f_0).last_mtime
                        == (!(0 as i32 as uintmax_t))
                            .wrapping_sub(
                                (if !(-(1 as i32) as uintmax_t <= 0 as i32 as u64) {
                                    0 as i32 as uintmax_t
                                } else {
                                    !(0 as i32 as uintmax_t)
                                        << (::core::mem::size_of::<uintmax_t>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                }),
                            ))
            {
                *mtime_ptr = (*f_0).last_mtime;
                mtime_ptr = 0 as *mut uintmax_t;
            }
        }
        if exists == 0 {
            *p = '\0' as i32 as i8;
            exists = dir_file_exists_p(name, filename);
            exists_in_cache = exists;
        }
        if exists != 0 {
            let mut st: stat = stat {
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
            *p = '/' as i32 as i8;
            if exists_in_cache != 0 {
                let mut e: i32 = 0;
                loop {
                    e = stat(name, &mut st);
                    if !(e == -(1 as i32) && *__errno_location() == 4 as i32) {
                        break;
                    }
                }
                if e != 0 as i32 {
                    exists = 0 as i32;
                    current_block_45 = 2868539653012386629;
                } else {
                    if !mtime_ptr.is_null() {
                        *mtime_ptr = file_timestamp_cons(
                            name,
                            st.st_mtim.tv_sec,
                            st.st_mtim.tv_nsec,
                        );
                        mtime_ptr = 0 as *mut uintmax_t;
                    }
                    current_block_45 = 7427571413727699167;
                }
            } else {
                current_block_45 = 7427571413727699167;
            }
            match current_block_45 {
                2868539653012386629 => {}
                _ => {
                    if !mtime_ptr.is_null() {
                        *mtime_ptr = 0 as i32 as uintmax_t;
                    }
                    if !path_index.is_null() {
                        *path_index = i;
                    }
                    return strcache_add_len(
                        name,
                        (p.offset(1 as i32 as isize).offset_from(name) as i64 as u64)
                            .wrapping_add(flen),
                    );
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn vpath_search(
    mut file: *const i8,
    mut mtime_ptr: *mut uintmax_t,
    mut vpath_index: *mut u32,
    mut path_index: *mut u32,
) -> *const i8 {
    let mut v: *mut vpath = 0 as *mut vpath;
    if *file.offset(0 as i32 as isize) as i32 == '/' as i32
        || vpaths.is_null() && general_vpath.is_null()
    {
        return 0 as *const i8;
    }
    if !vpath_index.is_null() {
        *vpath_index = 0 as i32 as u32;
        *path_index = 0 as i32 as u32;
    }
    v = vpaths;
    while !v.is_null() {
        if pattern_matches((*v).pattern, (*v).percent, file) != 0 {
            let mut p: *const i8 = selective_vpath_search(
                v,
                file,
                mtime_ptr,
                path_index,
            );
            if !p.is_null() {
                return p;
            }
        }
        if !vpath_index.is_null() {
            *vpath_index = (*vpath_index).wrapping_add(1);
            *vpath_index;
        }
        v = (*v).next;
    }
    if !general_vpath.is_null() {
        let mut p_0: *const i8 = selective_vpath_search(
            general_vpath,
            file,
            mtime_ptr,
            path_index,
        );
        if !p_0.is_null() {
            return p_0;
        }
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn print_vpath_data_base() {
    let mut nvpaths: u32 = 0;
    let mut v: *mut vpath = 0 as *mut vpath;
    puts(
        dcgettext(
            0 as *const i8,
            b"\n# VPATH Search Paths\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    nvpaths = 0 as i32 as u32;
    v = vpaths;
    while !v.is_null() {
        let mut i: u32 = 0;
        nvpaths = nvpaths.wrapping_add(1);
        nvpaths;
        printf(b"vpath %s \0" as *const u8 as *const i8, (*v).pattern);
        i = 0 as i32 as u32;
        while !(*((*v).searchpath).offset(i as isize)).is_null() {
            printf(
                b"%s%c\0" as *const u8 as *const i8,
                *((*v).searchpath).offset(i as isize),
                if (*((*v).searchpath).offset(i.wrapping_add(1 as i32 as u32) as isize))
                    .is_null()
                {
                    '\n' as i32
                } else {
                    ':' as i32
                },
            );
            i = i.wrapping_add(1);
            i;
        }
        v = (*v).next;
    }
    if vpaths.is_null() {
        puts(
            dcgettext(
                0 as *const i8,
                b"# No 'vpath' search paths.\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else {
        printf(
            dcgettext(
                0 as *const i8,
                b"\n# %u 'vpath' search paths.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            nvpaths,
        );
    }
    if general_vpath.is_null() {
        puts(
            dcgettext(
                0 as *const i8,
                b"\n# No general ('VPATH' variable) search path.\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    } else {
        let mut path: *mut *const i8 = (*general_vpath).searchpath;
        let mut i_0: u32 = 0;
        fputs(
            dcgettext(
                0 as *const i8,
                b"\n# General ('VPATH' variable) search path:\n# \0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            stdout,
        );
        i_0 = 0 as i32 as u32;
        while !(*path.offset(i_0 as isize)).is_null() {
            printf(
                b"%s%c\0" as *const u8 as *const i8,
                *path.offset(i_0 as isize),
                if (*path.offset(i_0.wrapping_add(1 as i32 as u32) as isize)).is_null() {
                    '\n' as i32
                } else {
                    ':' as i32
                },
            );
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    };
}