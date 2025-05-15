use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type wordsplit_node;
    pub type exclude;
    fn fclose(__stream: *mut FILE) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn __errno_location() -> *mut i32;
    fn faccessat(__fd: i32, __file: *const i8, __type: i32, __flag: i32) -> i32;
    fn close(__fd: i32) -> i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn base_name(file: *const i8) -> *mut i8;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    static mut exit_status: i32;
    fn open_error(_: *const i8);
    fn fatal_exit() -> !;
    fn quotearg_colon(arg: *const i8) -> *mut i8;
    fn wordsplit(s: *const i8, ws: *mut wordsplit_t, flags: u32) -> i32;
    fn wordsplit_free(ws: *mut wordsplit_t);
    static mut excluded: *mut exclude;
    fn subfile_open(dir: *const tar_stat_info, file: *const i8, flags: i32) -> i32;
    static mut chdir_fd: i32;
    fn exclude_add_pattern_buffer(ex: *mut exclude, buf: *mut i8);
    fn excluded_file_name(_: *const exclude, _: *const i8) -> bool;
    fn add_exclude_fp(
        _: Option<
            unsafe extern "C" fn(*mut exclude, *const i8, i32, *mut libc::c_void) -> (),
        >,
        _: *mut exclude,
        _: *mut FILE,
        _: i32,
        _: i8,
        _: *mut libc::c_void,
    ) -> i32;
    fn add_exclude(_: *mut exclude, _: *const i8, _: i32);
    fn free_exclude(_: *mut exclude);
    fn new_exclude() -> *mut exclude;
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
pub type __syscall_slong_t = i64;
pub type off_t = __off_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            8192 => C2RustUnnamed::_ISspace,
            4096 => C2RustUnnamed::_ISxdigit,
            2048 => C2RustUnnamed::_ISdigit,
            1024 => C2RustUnnamed::_ISalpha,
            512 => C2RustUnnamed::_ISlower,
            256 => C2RustUnnamed::_ISupper,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut i8,
    pub next_free: *mut i8,
    pub chunk_limit: *mut i8,
    pub temp: C2RustUnnamed_2,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_1,
    pub freefun: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
    pub plain: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub plain: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordsplit {
    pub ws_wordc: size_t,
    pub ws_wordv: *mut *mut i8,
    pub ws_offs: size_t,
    pub ws_wordn: size_t,
    pub ws_flags: u32,
    pub ws_options: u32,
    pub ws_maxwords: size_t,
    pub ws_wordi: size_t,
    pub ws_delim: *const i8,
    pub ws_comment: *const i8,
    pub ws_escape: [*const i8; 2],
    pub ws_alloc_die: Option<unsafe extern "C" fn(*mut wordsplit_t) -> ()>,
    pub ws_error: Option<unsafe extern "C" fn(*const i8, ...) -> ()>,
    pub ws_debug: Option<unsafe extern "C" fn(*const i8, ...) -> ()>,
    pub ws_env: *mut *const i8,
    pub ws_envbuf: *mut *mut i8,
    pub ws_envidx: size_t,
    pub ws_envsiz: size_t,
    pub ws_getvar: Option<
        unsafe extern "C" fn(*mut *mut i8, *const i8, size_t, *mut libc::c_void) -> i32,
    >,
    pub ws_closure: *mut libc::c_void,
    pub ws_command: Option<
        unsafe extern "C" fn(
            *mut *mut i8,
            *const i8,
            size_t,
            *mut *mut i8,
            *mut libc::c_void,
        ) -> i32,
    >,
    pub ws_input: *const i8,
    pub ws_len: size_t,
    pub ws_endp: size_t,
    pub ws_errno: i32,
    pub ws_usererr: *mut i8,
    pub ws_head: *mut wordsplit_node,
    pub ws_tail: *mut wordsplit_node,
    pub ws_lvl: i32,
}
pub type wordsplit_t = wordsplit;
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
pub struct exclist {
    pub next: *mut exclist,
    pub prev: *mut exclist,
    pub flags: i32,
    pub excluded: *mut exclude,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct excfile {
    pub next: *mut excfile,
    pub flags: i32,
    pub name: [i8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vcs_ignore_file {
    pub filename: *const i8,
    pub flags: i32,
    pub addfn: add_fn,
    pub initfn: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub data: *mut libc::c_void,
}
pub type add_fn = Option<
    unsafe extern "C" fn(*mut exclude, *const i8, i32, *mut libc::c_void) -> (),
>;
static mut excfile_head: *mut excfile = 0 as *const excfile as *mut excfile;
static mut excfile_tail: *mut excfile = 0 as *const excfile as *mut excfile;
#[no_mangle]
pub unsafe extern "C" fn excfile_add(mut name: *const i8, mut flags: i32) {
    let mut p: *mut excfile = xmalloc(
        (::core::mem::size_of::<excfile>() as u64).wrapping_add(strlen(name)),
    ) as *mut excfile;
    (*p).next = 0 as *mut excfile;
    (*p).flags = flags;
    strcpy(((*p).name).as_mut_ptr(), name);
    if !excfile_tail.is_null() {
        (*excfile_tail).next = p;
    } else {
        excfile_head = p;
    }
    excfile_tail = p;
}
#[no_mangle]
pub unsafe extern "C" fn info_attach_exclist(mut dir: *mut tar_stat_info) {
    let mut file: *mut excfile = 0 as *mut excfile;
    let mut head: *mut exclist = 0 as *mut exclist;
    let mut tail: *mut exclist = 0 as *mut exclist;
    let mut ent: *mut exclist = 0 as *mut exclist;
    let mut vcsfile: *mut vcs_ignore_file = 0 as *mut vcs_ignore_file;
    if !((*dir).exclude_list).is_null() {
        return;
    }
    file = excfile_head;
    while !file.is_null() {
        if faccessat(
            (if !dir.is_null() { (*dir).fd } else { chdir_fd }),
            ((*file).name).as_mut_ptr(),
            0 as i32,
            0 as i32,
        ) == 0 as i32
        {
            let mut fp: *mut FILE = 0 as *mut FILE;
            let mut ex: *mut exclude = 0 as *mut exclude;
            let mut fd: i32 = subfile_open(dir, ((*file).name).as_mut_ptr(), 0 as i32);
            if fd == -(1 as i32) {
                open_error(((*file).name).as_mut_ptr());
            } else {
                fp = fdopen(fd, b"r\0" as *const u8 as *const i8);
                if fp.is_null() {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    error(
                        0 as i32,
                        *__errno_location(),
                        dcgettext(
                            0 as *const i8,
                            b"%s: fdopen failed\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        ((*file).name).as_mut_ptr(),
                    );
                    exit_status = 2 as i32;
                    close(fd);
                } else {
                    if ex.is_null() {
                        ex = new_exclude();
                    }
                    vcsfile = get_vcs_ignore_file(((*file).name).as_mut_ptr());
                    if ((*vcsfile).initfn).is_some() {
                        (*vcsfile).data = ((*vcsfile).initfn)
                            .expect("non-null function pointer")((*vcsfile).data);
                    }
                    if add_exclude_fp(
                        (*vcsfile).addfn,
                        ex,
                        fp,
                        (1 as i32) << 0 as i32 | (1 as i32) << 28 as i32
                            | (1 as i32) << 30 as i32,
                        '\n' as i32 as i8,
                        (*vcsfile).data,
                    ) != 0
                    {
                        let mut e: i32 = *__errno_location();
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        error(
                            0 as i32,
                            e,
                            b"%s\0" as *const u8 as *const i8,
                            quotearg_colon(((*file).name).as_mut_ptr()),
                        );
                        fatal_exit();
                    }
                    fclose(fp);
                    ent = xmalloc(::core::mem::size_of::<exclist>() as u64)
                        as *mut exclist;
                    (*ent).excluded = ex;
                    (*ent).flags = if (*file).flags == 0 as i32 {
                        (*file).flags
                    } else {
                        (*vcsfile).flags
                    };
                    (*ent).prev = tail;
                    (*ent).next = 0 as *mut exclist;
                    if !tail.is_null() {
                        (*tail).next = ent;
                    } else {
                        head = ent;
                    }
                    tail = ent;
                }
            }
        }
        file = (*file).next;
    }
    (*dir).exclude_list = head;
}
#[no_mangle]
pub unsafe extern "C" fn info_free_exclist(mut dir: *mut tar_stat_info) {
    let mut ep: *mut exclist = (*dir).exclude_list;
    while !ep.is_null() {
        let mut next: *mut exclist = (*ep).next;
        free_exclude((*ep).excluded);
        rpl_free(ep as *mut libc::c_void);
        ep = next;
    }
    (*dir).exclude_list = 0 as *mut exclist;
}
#[no_mangle]
pub unsafe extern "C" fn excluded_name(
    mut name: *const i8,
    mut st: *mut tar_stat_info,
) -> bool {
    let mut ep: *mut exclist = 0 as *mut exclist;
    let mut rname: *const i8 = 0 as *const i8;
    let mut bname: *mut i8 = 0 as *mut i8;
    let mut result: bool = false;
    let mut nr: i32 = 0 as i32;
    name = name.offset(0 as i32 as isize);
    if excluded_file_name(excluded, name) {
        return 1 as i32 != 0;
    }
    if st.is_null() {
        return 0 as i32 != 0;
    }
    result = 0 as i32 != 0;
    while !st.is_null() && !result {
        ep = (*st).exclude_list;
        while !ep.is_null() {
            if !((*ep).flags & nr != 0) {
                result = excluded_file_name((*ep).excluded, name);
                if result {
                    break;
                }
                if rname.is_null() {
                    rname = name;
                    while *rname as i32 == '.' as i32
                        && *rname.offset(1 as i32 as isize) as i32 == '/' as i32
                    {
                        rname = rname.offset(2 as i32 as isize);
                    }
                }
                result = excluded_file_name((*ep).excluded, rname);
                if result {
                    break;
                }
                if bname.is_null() {
                    bname = base_name(name);
                }
                result = excluded_file_name((*ep).excluded, bname);
                if result {
                    break;
                }
            }
            ep = (*ep).next;
        }
        st = (*st).parent;
        nr = 0x2 as i32;
    }
    rpl_free(bname as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn cvs_addfn(
    mut ex: *mut exclude,
    mut pattern: *const i8,
    mut options: i32,
    mut data: *mut libc::c_void,
) {
    let mut ws: wordsplit = wordsplit {
        ws_wordc: 0,
        ws_wordv: 0 as *mut *mut i8,
        ws_offs: 0,
        ws_wordn: 0,
        ws_flags: 0,
        ws_options: 0,
        ws_maxwords: 0,
        ws_wordi: 0,
        ws_delim: 0 as *const i8,
        ws_comment: 0 as *const i8,
        ws_escape: [0 as *const i8; 2],
        ws_alloc_die: None,
        ws_error: None,
        ws_debug: None,
        ws_env: 0 as *mut *const i8,
        ws_envbuf: 0 as *mut *mut i8,
        ws_envidx: 0,
        ws_envsiz: 0,
        ws_getvar: None,
        ws_closure: 0 as *mut libc::c_void,
        ws_command: None,
        ws_input: 0 as *const i8,
        ws_len: 0,
        ws_endp: 0,
        ws_errno: 0,
        ws_usererr: 0 as *mut i8,
        ws_head: 0 as *mut wordsplit_node,
        ws_tail: 0 as *mut wordsplit_node,
        ws_lvl: 0,
    };
    let mut i: size_t = 0;
    if wordsplit(pattern, &mut ws, (0x40 as i32 | 0x4 as i32 | 0x800 as i32) as u32) != 0
    {
        return;
    }
    i = 0 as i32 as size_t;
    while i < ws.ws_wordc {
        add_exclude(ex, *(ws.ws_wordv).offset(i as isize), options);
        i = i.wrapping_add(1);
        i;
    }
    wordsplit_free(&mut ws);
}
unsafe extern "C" fn git_addfn(
    mut ex: *mut exclude,
    mut pattern: *const i8,
    mut options: i32,
    mut data: *mut libc::c_void,
) {
    while *(*__ctype_b_loc()).offset(*pattern as i32 as isize) as i32
        & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
    {
        pattern = pattern.offset(1);
        pattern;
    }
    if *pattern as i32 == 0 as i32 || *pattern as i32 == '#' as i32 {
        return;
    }
    if *pattern as i32 == '\\' as i32
        && *pattern.offset(1 as i32 as isize) as i32 == '#' as i32
    {
        pattern = pattern.offset(1);
        pattern;
    }
    add_exclude(ex, pattern, options);
}
unsafe extern "C" fn bzr_addfn(
    mut ex: *mut exclude,
    mut pattern: *const i8,
    mut options: i32,
    mut data: *mut libc::c_void,
) {
    while *(*__ctype_b_loc()).offset(*pattern as i32 as isize) as i32
        & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
    {
        pattern = pattern.offset(1);
        pattern;
    }
    if *pattern as i32 == 0 as i32 || *pattern as i32 == '#' as i32 {
        return;
    }
    if *pattern as i32 == '!' as i32 {
        pattern = pattern.offset(1);
        if *pattern as i32 == '!' as i32 {
            pattern = pattern.offset(1);
            pattern;
        } else {
            options |= (1 as i32) << 29 as i32;
        }
    }
    if strncmp(pattern, b"RE:\0" as *const u8 as *const i8, 3 as i32 as u64) == 0 as i32
    {
        pattern = pattern.offset(3 as i32 as isize);
        options &= !((1 as i32) << 28 as i32);
        options |= (1 as i32) << 27 as i32;
    }
    add_exclude(ex, pattern, options);
}
unsafe extern "C" fn hg_initfn(mut data: *mut libc::c_void) -> *mut libc::c_void {
    static mut hg_options: i32 = 0;
    let mut hgopt: *mut i32 = (if !data.is_null() {
        data
    } else {
        &mut hg_options as *mut i32 as *mut libc::c_void
    }) as *mut i32;
    *hgopt = (1 as i32) << 27 as i32;
    return hgopt as *mut libc::c_void;
}
unsafe extern "C" fn hg_addfn(
    mut ex: *mut exclude,
    mut pattern: *const i8,
    mut options: i32,
    mut data: *mut libc::c_void,
) {
    let mut hgopt: *mut i32 = data as *mut i32;
    let mut len: size_t = 0;
    while *(*__ctype_b_loc()).offset(*pattern as i32 as isize) as i32
        & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
    {
        pattern = pattern.offset(1);
        pattern;
    }
    if *pattern as i32 == 0 as i32 || *pattern as i32 == '#' as i32 {
        return;
    }
    if strncmp(pattern, b"syntax:\0" as *const u8 as *const i8, 7 as i32 as u64)
        == 0 as i32
    {
        pattern = pattern.offset(7 as i32 as isize);
        while *(*__ctype_b_loc()).offset(*pattern as i32 as isize) as i32
            & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
        {
            pattern = pattern.offset(1);
            pattern;
        }
        if strcmp(pattern, b"regexp\0" as *const u8 as *const i8) == 0 as i32 {
            *hgopt = (1 as i32) << 27 as i32;
        } else if strcmp(pattern, b"glob\0" as *const u8 as *const i8) == 0 as i32 {
            *hgopt = (1 as i32) << 28 as i32;
        }
        return;
    }
    len = strlen(pattern);
    if *pattern.offset(len.wrapping_sub(1 as i32 as u64) as isize) as i32 == '/' as i32 {
        let mut p: *mut i8 = 0 as *mut i8;
        len = len.wrapping_sub(1);
        len;
        p = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
        memcpy(p as *mut libc::c_void, pattern as *const libc::c_void, len);
        *p.offset(len as isize) = 0 as i32 as i8;
        pattern = p;
        exclude_add_pattern_buffer(ex, p);
        options |= (1 as i32) << 3 as i32 | (1 as i32) << 26 as i32;
    }
    add_exclude(
        ex,
        pattern,
        (if *hgopt == (1 as i32) << 27 as i32 {
            options & !((1 as i32) << 28 as i32)
        } else {
            options & !((1 as i32) << 27 as i32)
        }) | *hgopt,
    );
}
static mut vcs_ignore_files: [vcs_ignore_file; 5] = unsafe {
    [
        {
            let mut init = vcs_ignore_file {
                filename: b".cvsignore\0" as *const u8 as *const i8,
                flags: 0x2 as i32,
                addfn: Some(
                    cvs_addfn
                        as unsafe extern "C" fn(
                            *mut exclude,
                            *const i8,
                            i32,
                            *mut libc::c_void,
                        ) -> (),
                ),
                initfn: None,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = vcs_ignore_file {
                filename: b".gitignore\0" as *const u8 as *const i8,
                flags: 0 as i32,
                addfn: Some(
                    git_addfn
                        as unsafe extern "C" fn(
                            *mut exclude,
                            *const i8,
                            i32,
                            *mut libc::c_void,
                        ) -> (),
                ),
                initfn: None,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = vcs_ignore_file {
                filename: b".bzrignore\0" as *const u8 as *const i8,
                flags: 0 as i32,
                addfn: Some(
                    bzr_addfn
                        as unsafe extern "C" fn(
                            *mut exclude,
                            *const i8,
                            i32,
                            *mut libc::c_void,
                        ) -> (),
                ),
                initfn: None,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = vcs_ignore_file {
                filename: b".hgignore\0" as *const u8 as *const i8,
                flags: 0 as i32,
                addfn: Some(
                    hg_addfn
                        as unsafe extern "C" fn(
                            *mut exclude,
                            *const i8,
                            i32,
                            *mut libc::c_void,
                        ) -> (),
                ),
                initfn: Some(
                    hg_initfn
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = vcs_ignore_file {
                filename: 0 as *const i8,
                flags: 0 as i32,
                addfn: Some(
                    git_addfn
                        as unsafe extern "C" fn(
                            *mut exclude,
                            *const i8,
                            i32,
                            *mut libc::c_void,
                        ) -> (),
                ),
                initfn: None,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
unsafe extern "C" fn get_vcs_ignore_file(mut name: *const i8) -> *mut vcs_ignore_file {
    let mut p: *mut vcs_ignore_file = 0 as *mut vcs_ignore_file;
    p = vcs_ignore_files.as_mut_ptr();
    while !((*p).filename).is_null() {
        if strcmp((*p).filename, name) == 0 as i32 {
            break;
        }
        p = p.offset(1);
        p;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn exclude_vcs_ignores() {
    let mut p: *mut vcs_ignore_file = 0 as *mut vcs_ignore_file;
    p = vcs_ignore_files.as_mut_ptr();
    while !((*p).filename).is_null() {
        excfile_add((*p).filename, 0 as i32);
        p = p.offset(1);
        p;
    }
}