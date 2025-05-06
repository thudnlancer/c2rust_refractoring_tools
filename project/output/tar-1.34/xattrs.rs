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
    pub type __dirstream;
    pub type exclist;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strcspn(_: *const i8, _: *const i8) -> u64;
    fn strlen(_: *const i8) -> u64;
    fn __errno_location() -> *mut i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    fn call_arg_warn(call: *const i8, name: *const i8);
    fn fnmatch(__pattern: *const i8, __name: *const i8, __flags: i32) -> i32;
    static mut selinux_context_option: i32;
    static mut acls_option: i32;
    static mut xattrs_option: i32;
    static mut verbose_option: i32;
    static mut stdlis: *mut FILE;
    static mut chdir_fd: i32;
    fn xheader_xattr_add(
        st: *mut tar_stat_info,
        key: *const i8,
        val: *const i8,
        len: size_t,
    );
    static mut warning_option: i32;
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> i32;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    fn setxattrat(
        dir_fd: i32,
        path: *const i8,
        name: *const i8,
        value: *const libc::c_void,
        size: size_t,
        flags: i32,
    ) -> i32;
    fn lsetxattrat(
        dir_fd: i32,
        path: *const i8,
        name: *const i8,
        value: *const libc::c_void,
        size: size_t,
        flags: i32,
    ) -> i32;
    fn llistxattrat(
        dir_fd: i32,
        path: *const i8,
        list: *mut i8,
        size: size_t,
    ) -> ssize_t;
    fn lgetxattrat(
        dir_fd: i32,
        path: *const i8,
        name: *const i8,
        value: *mut libc::c_void,
        size: size_t,
    ) -> ssize_t;
    fn fgetxattr(
        __fd: i32,
        __name: *const i8,
        __value: *mut libc::c_void,
        __size: size_t,
    ) -> ssize_t;
    fn flistxattr(__fd: i32, __list: *mut i8, __size: size_t) -> ssize_t;
}
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
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = i64;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut i8,
    pub next_free: *mut i8,
    pub chunk_limit: *mut i8,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut i8,
    pub prev: *mut _obstack_chunk,
    pub contents: [i8; 0],
}
pub type uintmax_t = __uintmax_t;
pub type DIR = __dirstream;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_2 {
    DEFAULT_MXFAST = 128,
}
impl C2RustUnnamed_2 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_2::DEFAULT_MXFAST => 128,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_2 {
        match value {
            128 => C2RustUnnamed_2::DEFAULT_MXFAST,
            _ => panic!("Invalid value for C2RustUnnamed_2: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_2 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_2 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_2 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_2 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_2 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn add(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn sub(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn mul(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn div(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn rem(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sp_array {
    pub offset: off_t,
    pub numbytes: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xheader {
    pub stk: *mut obstack,
    pub size: size_t,
    pub buffer: *mut i8,
    pub string_length: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xattr_array {
    pub xkey: *mut i8,
    pub xval_ptr: *mut i8,
    pub xval_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_stat_info {
    pub orig_file_name: *mut i8,
    pub file_name: *mut i8,
    pub had_trailing_slash: bool,
    pub link_name: *mut i8,
    pub uname: *mut i8,
    pub gname: *mut i8,
    pub cntx_name: *mut i8,
    pub acls_a_ptr: *mut i8,
    pub acls_a_len: size_t,
    pub acls_d_ptr: *mut i8,
    pub acls_d_len: size_t,
    pub stat: stat,
    pub atime: timespec,
    pub mtime: timespec,
    pub ctime: timespec,
    pub archive_file_size: off_t,
    pub is_sparse: bool,
    pub sparse_major: u32,
    pub sparse_minor: u32,
    pub sparse_map_avail: size_t,
    pub sparse_map_size: size_t,
    pub sparse_map: *mut sp_array,
    pub real_size: off_t,
    pub real_size_set: bool,
    pub sparse_name_done: bool,
    pub xattr_map_size: size_t,
    pub xattr_map: *mut xattr_array,
    pub xhdr: xheader,
    pub is_dumpdir: bool,
    pub skipped: bool,
    pub dumpdir: *mut i8,
    pub parent: *mut tar_stat_info,
    pub dirstream: *mut DIR,
    pub fd: i32,
    pub exclude_list: *mut exclist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xattrs_mask_map {
    pub masks: *mut *const i8,
    pub size: size_t,
    pub used: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub incl: xattrs_mask_map,
    pub excl: xattrs_mask_map,
}
#[inline]
unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (C2RustUnnamed_2::DEFAULT_MXFAST as i32 as u64).wrapping_div(s);
            n = (n as u64).wrapping_add((n == 0) as i32 as u64) as size_t as size_t;
        }
        if (if (9223372036854775807 as i64 as u64) < 18446744073709551615 as u64 {
            9223372036854775807 as i64 as u64
        } else {
            (18446744073709551615 as u64).wrapping_sub(1 as i32 as u64)
        })
            .wrapping_div(s) < n
        {
            xalloc_die();
        }
    } else {
        if (if (9223372036854775807 as i64 as u64) < 18446744073709551615 as u64 {
            9223372036854775807 as i64 as u64
        } else {
            18446744073709551615 as u64
        })
            .wrapping_div(3 as i32 as u64)
            .wrapping_mul(2 as i32 as u64)
            .wrapping_div(s) <= n
        {
            xalloc_die();
        }
        n = (n as u64)
            .wrapping_add(n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64))
            as size_t as size_t;
    }
    *pn = n;
    return xrealloc(p, n.wrapping_mul(s));
}
static mut xattrs_setup: C2RustUnnamed_3 = C2RustUnnamed_3 {
    incl: xattrs_mask_map {
        masks: 0 as *const *const i8 as *mut *const i8,
        size: 0,
        used: 0,
    },
    excl: xattrs_mask_map {
        masks: 0 as *const *const i8 as *mut *const i8,
        size: 0,
        used: 0,
    },
};
unsafe extern "C" fn acls_one_line(
    mut prefix: *const i8,
    mut delim: i8,
    mut aclstring: *const i8,
    mut len: size_t,
) {
    let mut stk: obstack = obstack {
        chunk_size: 0,
        chunk: 0 as *mut _obstack_chunk,
        object_base: 0 as *mut i8,
        next_free: 0 as *mut i8,
        chunk_limit: 0 as *mut i8,
        temp: C2RustUnnamed_1 { i: 0 },
        alignment_mask: 0,
        chunkfun: C2RustUnnamed_0 { plain: None },
        freefun: C2RustUnnamed { plain: None },
        extra_arg: 0 as *mut libc::c_void,
        use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut pref_len: i32 = strlen(prefix) as i32;
    let mut oldstring: *const i8 = aclstring;
    let mut pos: i32 = 0 as i32;
    if aclstring.is_null() || len == 0 {
        return;
    }
    _obstack_begin(
        &mut stk,
        0 as i32 as size_t,
        0 as i32 as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    while pos as u64 <= len {
        let mut move_0: i32 = strcspn(aclstring, b",\n\0" as *const u8 as *const i8)
            as i32;
        if move_0 == 0 {
            break;
        }
        if oldstring != aclstring {
            let mut __o: *mut obstack = &mut stk;
            if ({
                let mut __o1: *const obstack = __o;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
            }) < 1 as i32 as u64
            {
                _obstack_newchunk(__o, 1 as i32 as size_t);
            }
            let fresh0 = (*__o).next_free;
            (*__o).next_free = ((*__o).next_free).offset(1);
            *fresh0 = delim;
        }
        let mut __o_0: *mut obstack = &mut stk;
        let mut __len: size_t = pref_len as size_t;
        if ({
            let mut __o1: *const obstack = __o_0;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
        }) < __len
        {
            _obstack_newchunk(__o_0, __len);
        }
        memcpy(
            (*__o_0).next_free as *mut libc::c_void,
            prefix as *const libc::c_void,
            __len,
        );
        (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
        let mut __o_1: *mut obstack = &mut stk;
        let mut __len_0: size_t = move_0 as size_t;
        if ({
            let mut __o1: *const obstack = __o_1;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
        }) < __len_0
        {
            _obstack_newchunk(__o_1, __len_0);
        }
        memcpy(
            (*__o_1).next_free as *mut libc::c_void,
            aclstring as *const libc::c_void,
            __len_0,
        );
        (*__o_1).next_free = ((*__o_1).next_free).offset(__len_0 as isize);
        pos += move_0 + 1 as i32;
        aclstring = aclstring.offset((move_0 + 1 as i32) as isize);
    }
    let mut __o_2: *mut obstack = &mut stk;
    if ({
        let mut __o1: *const obstack = __o_2;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
    }) < 1 as i32 as u64
    {
        _obstack_newchunk(__o_2, 1 as i32 as size_t);
    }
    let fresh1 = (*__o_2).next_free;
    (*__o_2).next_free = ((*__o_2).next_free).offset(1);
    *fresh1 = '\0' as i32 as i8;
    fprintf(
        stdlis,
        b"%s\0" as *const u8 as *const i8,
        ({
            let mut __o1: *mut obstack = &mut stk as *mut obstack;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut i8 {
                (*__o1).set_maybe_empty_object(1 as i32 as u32);
            }
            (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                < ::core::mem::size_of::<*mut libc::c_void>() as u64
            {
                (*__o1).object_base
            } else {
                0 as *mut i8
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                                < ::core::mem::size_of::<*mut libc::c_void>() as u64
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut i8
                            }),
                        ) as i64 as u64)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64 as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
                    as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }) as *mut i8,
    );
    let mut __o_3: *mut obstack = &mut stk;
    let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
    if __obj > (*__o_3).chunk as *mut libc::c_void
        && __obj < (*__o_3).chunk_limit as *mut libc::c_void
    {
        (*__o_3).object_base = __obj as *mut i8;
        (*__o_3).next_free = (*__o_3).object_base;
    } else {
        _obstack_free(__o_3, __obj);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xattrs_acls_get(
    mut parentfd: i32,
    mut file_name: *const i8,
    mut st: *mut tar_stat_info,
    mut fd: i32,
    mut xisfile: i32,
) {
    if acls_option > 0 as i32 {
        static mut done: i32 = 0 as i32;
        if done == 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"POSIX ACL support is not available\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        done = 1 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xattrs_acls_set(
    mut st: *const tar_stat_info,
    mut file_name: *const i8,
    mut typeflag: i8,
) {
    if acls_option > 0 as i32 && typeflag as i32 != '2' as i32 {
        static mut done: i32 = 0 as i32;
        if done == 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"POSIX ACL support is not available\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        done = 1 as i32;
    }
}
unsafe extern "C" fn mask_map_realloc(mut map: *mut xattrs_mask_map) {
    if (*map).used == (*map).size {
        if (*map).size == 0 as i32 as u64 {
            (*map).size = 4 as i32 as size_t;
        }
        (*map).masks = x2nrealloc(
            (*map).masks as *mut libc::c_void,
            &mut (*map).size,
            ::core::mem::size_of::<*const i8>() as u64,
        ) as *mut *const i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xattrs_mask_add(mut mask: *const i8, mut incl: bool) {
    let mut mask_map: *mut xattrs_mask_map = if incl as i32 != 0 {
        &mut xattrs_setup.incl
    } else {
        &mut xattrs_setup.excl
    };
    mask_map_realloc(mask_map);
    let fresh2 = (*mask_map).used;
    (*mask_map).used = ((*mask_map).used).wrapping_add(1);
    let ref mut fresh3 = *((*mask_map).masks).offset(fresh2 as isize);
    *fresh3 = mask;
}
unsafe extern "C" fn clear_mask_map(mut mask_map: *mut xattrs_mask_map) {
    if (*mask_map).size != 0 {
        rpl_free((*mask_map).masks as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xattrs_clear_setup() {
    clear_mask_map(&mut xattrs_setup.incl);
    clear_mask_map(&mut xattrs_setup.excl);
}
#[no_mangle]
pub unsafe extern "C" fn xattrs_xattrs_get(
    mut parentfd: i32,
    mut file_name: *const i8,
    mut st: *mut tar_stat_info,
    mut fd: i32,
) {
    if xattrs_option > 0 as i32 {
        static mut xsz: size_t = 1024 as i32 as size_t;
        static mut xatrs: *mut i8 = 0 as *const i8 as *mut i8;
        let mut xret: ssize_t = -(1 as i32) as ssize_t;
        if xatrs.is_null() {
            xatrs = x2nrealloc(xatrs as *mut libc::c_void, &mut xsz, 1 as i32 as size_t)
                as *mut i8;
        }
        while (if fd == 0 as i32 {
            xret = llistxattrat(parentfd, file_name, xatrs, xsz);
            (xret == -(1 as i32) as i64) as i32
        } else {
            xret = flistxattr(fd, xatrs, xsz);
            (xret == -(1 as i32) as i64) as i32
        }) != 0 && *__errno_location() == 34 as i32
        {
            xatrs = x2nrealloc(xatrs as *mut libc::c_void, &mut xsz, 1 as i32 as size_t)
                as *mut i8;
        }
        if xret == -(1 as i32) as i64 {
            call_arg_warn(
                if fd == 0 as i32 {
                    b"llistxattrat\0" as *const u8 as *const i8
                } else {
                    b"flistxattr\0" as *const u8 as *const i8
                },
                file_name,
            );
        } else {
            let mut attr: *const i8 = xatrs;
            static mut asz: size_t = 1024 as i32 as size_t;
            static mut val: *mut i8 = 0 as *const i8 as *mut i8;
            if val.is_null() {
                val = x2nrealloc(val as *mut libc::c_void, &mut asz, 1 as i32 as size_t)
                    as *mut i8;
            }
            while xret > 0 as i32 as i64 {
                let mut len: size_t = strlen(attr);
                let mut aret: ssize_t = 0 as i32 as ssize_t;
                while (if fd == 0 as i32 {
                    aret = lgetxattrat(
                        parentfd,
                        file_name,
                        attr,
                        val as *mut libc::c_void,
                        asz,
                    );
                    (aret == -(1 as i32) as i64) as i32
                } else {
                    aret = fgetxattr(fd, attr, val as *mut libc::c_void, asz);
                    (aret == -(1 as i32) as i64) as i32
                }) != 0 && *__errno_location() == 34 as i32
                {
                    val = x2nrealloc(
                        val as *mut libc::c_void,
                        &mut asz,
                        1 as i32 as size_t,
                    ) as *mut i8;
                }
                if aret != -(1 as i32) as i64 {
                    if !xattrs_masked_out(attr, 1 as i32 != 0) {
                        xheader_xattr_add(st, attr, val, aret as size_t);
                    }
                } else if *__errno_location() != 61 as i32 {
                    call_arg_warn(
                        if fd == 0 as i32 {
                            b"lgetxattrat\0" as *const u8 as *const i8
                        } else {
                            b"fgetxattr\0" as *const u8 as *const i8
                        },
                        file_name,
                    );
                }
                attr = attr.offset(len.wrapping_add(1 as i32 as u64) as isize);
                xret = (xret as u64).wrapping_sub(len.wrapping_add(1 as i32 as u64))
                    as ssize_t as ssize_t;
            }
        }
    }
}
unsafe extern "C" fn xattrs__fd_set(
    mut st: *const tar_stat_info,
    mut file_name: *const i8,
    mut typeflag: i8,
    mut attr: *const i8,
    mut ptr: *const i8,
    mut len: size_t,
) {
    if !ptr.is_null() {
        let mut sysname: *const i8 = b"setxattrat\0" as *const u8 as *const i8;
        let mut ret: i32 = -(1 as i32);
        if typeflag as i32 != '2' as i32 {
            ret = setxattrat(
                chdir_fd,
                file_name,
                attr,
                ptr as *const libc::c_void,
                len,
                0 as i32,
            );
        } else {
            sysname = b"lsetxattr\0" as *const u8 as *const i8;
            ret = lsetxattrat(
                chdir_fd,
                file_name,
                attr,
                ptr as *const libc::c_void,
                len,
                0 as i32,
            );
        }
        if ret == -(1 as i32) {
            if warning_option & 0x200000 as i32 != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"%s: Cannot set '%s' extended attribute for file '%s'\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    sysname,
                    attr,
                    file_name,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xattrs_selinux_get(
    mut parentfd: i32,
    mut file_name: *const i8,
    mut st: *mut tar_stat_info,
    mut fd: i32,
) {
    if selinux_context_option > 0 as i32 {
        static mut done: i32 = 0 as i32;
        if done == 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"SELinux support is not available\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        done = 1 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xattrs_selinux_set(
    mut st: *const tar_stat_info,
    mut file_name: *const i8,
    mut typeflag: i8,
) {
    if selinux_context_option > 0 as i32 {
        static mut done: i32 = 0 as i32;
        if done == 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"SELinux support is not available\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        done = 1 as i32;
    }
}
unsafe extern "C" fn xattrs_matches_mask(
    mut kw: *const i8,
    mut mm: *mut xattrs_mask_map,
) -> bool {
    let mut i: i32 = 0;
    if (*mm).size == 0 {
        return 0 as i32 != 0;
    }
    i = 0 as i32;
    while (i as u64) < (*mm).used {
        if fnmatch(*((*mm).masks).offset(i as isize), kw, 0 as i32) == 0 as i32 {
            return 1 as i32 != 0;
        }
        i += 1;
        i;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn xattrs_kw_included(mut kw: *const i8, mut archiving: bool) -> bool {
    if xattrs_setup.incl.size != 0 {
        return xattrs_matches_mask(kw, &mut xattrs_setup.incl)
    } else if archiving {
        return 1 as i32 != 0
    } else {
        return strncmp(
            kw,
            b"user.\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64),
        ) == 0 as i32
    };
}
unsafe extern "C" fn xattrs_kw_excluded(mut kw: *const i8, mut archiving: bool) -> bool {
    return if xattrs_setup.excl.size != 0 {
        xattrs_matches_mask(kw, &mut xattrs_setup.excl) as i32
    } else {
        0 as i32
    } != 0;
}
unsafe extern "C" fn xattrs_masked_out(mut kw: *const i8, mut archiving: bool) -> bool {
    return if xattrs_kw_included(kw, archiving) as i32 != 0 {
        xattrs_kw_excluded(kw, archiving) as i32
    } else {
        1 as i32
    } != 0;
}
#[no_mangle]
pub unsafe extern "C" fn xattrs_xattrs_set(
    mut st: *const tar_stat_info,
    mut file_name: *const i8,
    mut typeflag: i8,
    mut later_run: i32,
) {
    if xattrs_option > 0 as i32 {
        let mut scan: size_t = 0 as i32 as size_t;
        if (*st).xattr_map_size == 0 {
            return;
        }
        while scan < (*st).xattr_map_size {
            let mut keyword: *mut i8 = (*((*st).xattr_map).offset(scan as isize)).xkey;
            keyword = keyword
                .offset(strlen(b"SCHILY.xattr.\0" as *const u8 as *const i8) as isize);
            if !(typeflag as i32 == '0' as i32
                && later_run
                    == (strcmp(
                        keyword,
                        b"security.capability\0" as *const u8 as *const i8,
                    ) != 0) as i32)
            {
                if !xattrs_masked_out(keyword, 0 as i32 != 0) {
                    xattrs__fd_set(
                        st,
                        file_name,
                        typeflag,
                        keyword,
                        (*((*st).xattr_map).offset(scan as isize)).xval_ptr,
                        (*((*st).xattr_map).offset(scan as isize)).xval_len,
                    );
                }
            }
            scan = scan.wrapping_add(1);
            scan;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xattrs_print_char(
    mut st: *const tar_stat_info,
    mut output: *mut i8,
) {
    let mut i: i32 = 0;
    if verbose_option < 2 as i32 {
        *output = 0 as i32 as i8;
        return;
    }
    if xattrs_option > 0 as i32 || selinux_context_option > 0 as i32
        || acls_option > 0 as i32
    {
        *output = ' ' as i32 as i8;
        *output.offset(1 as i32 as isize) = 0 as i32 as i8;
    }
    if xattrs_option > 0 as i32 && (*st).xattr_map_size != 0 {
        i = 0 as i32;
        while (i as u64) < (*st).xattr_map_size {
            let mut keyword: *mut i8 = ((*((*st).xattr_map).offset(i as isize)).xkey)
                .offset(strlen(b"SCHILY.xattr.\0" as *const u8 as *const i8) as isize);
            if !xattrs_masked_out(keyword, 0 as i32 != 0) {
                *output = '*' as i32 as i8;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    if selinux_context_option > 0 as i32 && !((*st).cntx_name).is_null() {
        *output = '.' as i32 as i8;
    }
    if acls_option > 0 as i32 && ((*st).acls_a_len != 0 || (*st).acls_d_len != 0) {
        *output = '+' as i32 as i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xattrs_print(mut st: *const tar_stat_info) {
    if verbose_option < 3 as i32 {
        return;
    }
    if selinux_context_option > 0 as i32 && !((*st).cntx_name).is_null() {
        fprintf(stdlis, b"  s: %s\n\0" as *const u8 as *const i8, (*st).cntx_name);
    }
    if acls_option > 0 as i32 && ((*st).acls_a_len != 0 || (*st).acls_d_len != 0) {
        fprintf(stdlis, b"  a: \0" as *const u8 as *const i8);
        acls_one_line(
            b"\0" as *const u8 as *const i8,
            ',' as i32 as i8,
            (*st).acls_a_ptr,
            (*st).acls_a_len,
        );
        if (*st).acls_a_len != 0 && (*st).acls_d_len != 0 {
            fprintf(stdlis, b",\0" as *const u8 as *const i8);
        }
        acls_one_line(
            b"default:\0" as *const u8 as *const i8,
            ',' as i32 as i8,
            (*st).acls_d_ptr,
            (*st).acls_d_len,
        );
        fprintf(stdlis, b"\n\0" as *const u8 as *const i8);
    }
    if xattrs_option > 0 as i32 && (*st).xattr_map_size != 0 {
        let mut i: i32 = 0;
        i = 0 as i32;
        while (i as u64) < (*st).xattr_map_size {
            let mut keyword: *mut i8 = ((*((*st).xattr_map).offset(i as isize)).xkey)
                .offset(strlen(b"SCHILY.xattr.\0" as *const u8 as *const i8) as isize);
            if !xattrs_masked_out(keyword, 0 as i32 != 0) {
                fprintf(
                    stdlis,
                    b"  x: %lu %s\n\0" as *const u8 as *const i8,
                    (*((*st).xattr_map).offset(i as isize)).xval_len,
                    keyword,
                );
            }
            i += 1;
            i;
        }
    }
}