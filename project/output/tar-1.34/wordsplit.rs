#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type stat;
    pub type dirent;
    fn __errno_location() -> *mut i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn getuid() -> __uid_t;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strdup(_: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const i8) -> *mut passwd;
    fn glob(
        __pattern: *const i8,
        __flags: i32,
        __errfunc: Option<unsafe extern "C" fn(*const i8, i32) -> i32>,
        __pglob: *mut glob_t,
    ) -> i32;
    fn globfree(__pglob: *mut glob_t);
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int32_t = i32;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
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
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
pub type __size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut i8,
    pub gl_offs: __size_t,
    pub gl_flags: i32,
    pub gl_closedir: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option<unsafe extern "C" fn(*const i8) -> *mut libc::c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(*const i8, *mut stat) -> i32>,
    pub gl_stat: Option<unsafe extern "C" fn(*const i8, *mut stat) -> i32>,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordsplit_node {
    pub prev: *mut wordsplit_node,
    pub next: *mut wordsplit_node,
    pub flags: u32,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub segm: C2RustUnnamed_0,
    pub word: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub beg: size_t,
    pub end: size_t,
}
pub type wordsplit_t = wordsplit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exptab {
    pub descr: *const i8,
    pub flag: i32,
    pub opt: i32,
    pub expansion: Option<unsafe extern "C" fn(*mut wordsplit) -> i32>,
}
pub type C2RustUnnamed_1 = u32;
pub const st_dquote: C2RustUnnamed_1 = 2;
pub const st_squote: C2RustUnnamed_1 = 1;
pub const st_init: C2RustUnnamed_1 = 0;
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn _wsplt_alloc_die(mut wsp: *mut wordsplit) {
    ((*wsp).ws_error)
        .expect(
            "non-null function pointer",
        )(
        b"%s\0" as *const u8 as *const i8,
        dcgettext(
            0 as *const i8,
            b"memory exhausted\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    abort();
}
unsafe extern "C" fn _wsplt_error(mut fmt: *const i8, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fputc('\n' as i32, stderr);
}
unsafe extern "C" fn _wsplt_seterr(mut wsp: *mut wordsplit, mut ec: i32) -> i32 {
    (*wsp).ws_errno = ec;
    if (*wsp).ws_flags & 0x10 as i32 as u32 != 0 {
        wordsplit_perror(wsp);
    }
    return ec;
}
unsafe extern "C" fn _wsplt_nomem(mut wsp: *mut wordsplit) -> i32 {
    *__errno_location() = 12 as i32;
    (*wsp).ws_errno = 2 as i32;
    if (*wsp).ws_flags & 0x80 as i32 as u32 != 0 {
        ((*wsp).ws_alloc_die).expect("non-null function pointer")(wsp);
    }
    if (*wsp).ws_flags & 0x10 as i32 as u32 != 0 {
        wordsplit_perror(wsp);
    }
    if (*wsp).ws_flags & 0x8 as i32 as u32 == 0 {
        wordsplit_free(wsp);
    }
    wordsplit_free_nodes(wsp);
    return (*wsp).ws_errno;
}
unsafe extern "C" fn _wsplt_subsplit(
    mut wsp: *mut wordsplit,
    mut wss: *mut wordsplit,
    mut str: *const i8,
    mut len: i32,
    mut flags: u32,
    mut finalize: i32,
) -> i32 {
    let mut rc: i32 = 0;
    (*wss).ws_delim = (*wsp).ws_delim;
    (*wss).ws_debug = (*wsp).ws_debug;
    (*wss).ws_error = (*wsp).ws_error;
    (*wss).ws_alloc_die = (*wsp).ws_alloc_die;
    if flags & 0x40 as i32 as u32 == 0 {
        (*wss).ws_env = (*wsp).ws_env;
        (*wss).ws_getvar = (*wsp).ws_getvar;
        flags
            |= (*wsp).ws_flags
                & (0x80000 as i32 | 0x8000000 as i32 | 0x100000 as i32) as u32;
    }
    if flags & 0x4 as i32 as u32 == 0 {
        (*wss).ws_command = (*wsp).ws_command;
    }
    if flags & (0x40 as i32 | 0x4 as i32) as u32 != (0x40 as i32 | 0x4 as i32) as u32 {
        (*wss).ws_closure = (*wsp).ws_closure;
        flags |= (*wsp).ws_flags & 0x4000000 as i32 as u32;
    }
    (*wss).ws_options = (*wsp).ws_options;
    flags
        |= (0x4000 as i32 | 0x10000 as i32 | 0x20000 as i32 | 0x40000 as i32) as u32
            | (*wsp).ws_flags
                & ((0x200000 as i32 | 0x10 as i32) as u32 | 0x80000000 as u32);
    rc = wordsplit_init(wss, str, len as size_t, flags);
    if rc != 0 {
        return rc;
    }
    (*wss).ws_lvl = (*wsp).ws_lvl + 1 as i32;
    rc = wordsplit_process_list(wss, 0 as i32 as size_t);
    if rc != 0 {
        wordsplit_free_nodes(wss);
        return rc;
    }
    if finalize != 0 {
        rc = wordsplit_finish(wss);
        wordsplit_free_nodes(wss);
    }
    return rc;
}
unsafe extern "C" fn _wsplt_seterr_sub(
    mut wsp: *mut wordsplit,
    mut wss: *mut wordsplit,
) {
    if (*wsp).ws_errno == 9 as i32 {
        rpl_free((*wsp).ws_usererr as *mut libc::c_void);
    }
    (*wsp).ws_errno = (*wss).ws_errno;
    if (*wss).ws_errno == 9 as i32 {
        (*wsp).ws_usererr = (*wss).ws_usererr;
        (*wss).ws_errno = 0 as i32;
        (*wss).ws_usererr = 0 as *mut i8;
    }
}
unsafe extern "C" fn wordsplit_init0(mut wsp: *mut wordsplit) {
    if (*wsp).ws_flags & 0x8 as i32 as u32 != 0 {
        if (*wsp).ws_flags & 0x1 as i32 as u32 == 0 {
            wordsplit_free_words(wsp);
        }
        wordsplit_clearerr(wsp);
    } else {
        (*wsp).ws_wordv = 0 as *mut *mut i8;
        (*wsp).ws_wordc = 0 as i32 as size_t;
        (*wsp).ws_wordn = 0 as i32 as size_t;
    }
    (*wsp).ws_errno = 0 as i32;
}
#[no_mangle]
pub static mut wordsplit_c_escape_tab: [i8; 19] = unsafe {
    *::core::mem::transmute::<
        &[u8; 19],
        &mut [i8; 19],
    >(b"\\\\\"\"a\x07b\x08f\x0Cn\nr\rt\tv\x0B\0")
};
unsafe extern "C" fn wordsplit_init(
    mut wsp: *mut wordsplit,
    mut input: *const i8,
    mut len: size_t,
    mut flags: u32,
) -> i32 {
    (*wsp).ws_flags = flags;
    if (*wsp).ws_flags & 0x10000 as i32 as u32 == 0 {
        (*wsp).ws_alloc_die = Some(
            _wsplt_alloc_die as unsafe extern "C" fn(*mut wordsplit) -> (),
        );
    }
    if (*wsp).ws_flags & 0x20000 as i32 as u32 == 0 {
        (*wsp).ws_error = Some(
            _wsplt_error as unsafe extern "C" fn(*const i8, ...) -> (),
        );
    }
    if (*wsp).ws_flags & 0x40 as i32 as u32 == 0 {
        (*wsp).ws_envsiz = 0 as i32 as size_t;
        (*wsp).ws_envidx = (*wsp).ws_envsiz;
        (*wsp).ws_envbuf = 0 as *mut *mut i8;
    }
    if (*wsp).ws_flags & 0x4 as i32 as u32 == 0 {
        if ((*wsp).ws_command).is_none() {
            _wsplt_seterr(wsp, 3 as i32);
            *__errno_location() = 22 as i32;
            return (*wsp).ws_errno;
        }
    }
    if (*wsp).ws_flags & 0x200000 as i32 as u32 != 0 {
        if (*wsp).ws_flags & 0x40000 as i32 as u32 == 0 {
            if (*wsp).ws_flags & 0x20000 as i32 as u32 != 0 {
                (*wsp).ws_debug = (*wsp).ws_error;
            } else if (*wsp).ws_flags & 0x10 as i32 as u32 != 0 {
                (*wsp).ws_debug = Some(
                    _wsplt_error as unsafe extern "C" fn(*const i8, ...) -> (),
                );
            } else {
                (*wsp).ws_flags &= !(0x200000 as i32) as u32;
            }
        }
    }
    (*wsp).ws_input = input;
    (*wsp).ws_len = len;
    if (*wsp).ws_flags & 0x2 as i32 as u32 == 0 {
        (*wsp).ws_offs = 0 as i32 as size_t;
    }
    if (*wsp).ws_flags & 0x4000 as i32 as u32 == 0 {
        (*wsp).ws_delim = b" \t\n\0" as *const u8 as *const i8;
    }
    if (*wsp).ws_flags & 0x8000 as i32 as u32 == 0 {
        (*wsp).ws_comment = 0 as *const i8;
    }
    if (*wsp).ws_flags & 0x4000000 as i32 as u32 == 0 {
        (*wsp).ws_closure = 0 as *mut libc::c_void;
    }
    if (*wsp).ws_flags & 0x80000000 as u32 == 0 {
        (*wsp).ws_options = 0 as i32 as u32;
    }
    if (*wsp).ws_flags & 0x10000000 as i32 as u32 != 0 {
        if ((*wsp).ws_escape[0 as i32 as usize]).is_null() {
            (*wsp).ws_escape[0 as i32 as usize] = b"\0" as *const u8 as *const i8;
        }
        if ((*wsp).ws_escape[1 as i32 as usize]).is_null() {
            (*wsp).ws_escape[1 as i32 as usize] = b"\0" as *const u8 as *const i8;
        }
    } else if (*wsp).ws_flags & 0x2000000 as i32 as u32 != 0 {
        (*wsp).ws_escape[0 as i32 as usize] = wordsplit_c_escape_tab.as_mut_ptr();
        (*wsp).ws_escape[1 as i32 as usize] = wordsplit_c_escape_tab.as_mut_ptr();
        (*wsp).ws_options
            |= (0x200 as i32 | 0x20 as i32 | 0x400 as i32 | 0x40 as i32) as u32;
    } else {
        (*wsp).ws_escape[0 as i32 as usize] = b"\0" as *const u8 as *const i8;
        (*wsp).ws_escape[1 as i32 as usize] = b"\\\\\"\"\0" as *const u8 as *const i8;
        (*wsp).ws_options |= 0x100 as i32 as u32;
    }
    (*wsp).ws_endp = 0 as i32 as size_t;
    (*wsp).ws_wordi = 0 as i32 as size_t;
    if (*wsp).ws_flags & 0x8 as i32 as u32 != 0 {
        wordsplit_free_nodes(wsp);
    }
    (*wsp).ws_tail = 0 as *mut wordsplit_node;
    (*wsp).ws_head = (*wsp).ws_tail;
    wordsplit_init0(wsp);
    return 0 as i32;
}
unsafe extern "C" fn alloc_space(mut wsp: *mut wordsplit, mut count: size_t) -> i32 {
    let mut offs: size_t = if (*wsp).ws_flags & 0x2 as i32 as u32 != 0 {
        (*wsp).ws_offs
    } else {
        0 as i32 as u64
    };
    let mut ptr: *mut *mut i8 = 0 as *mut *mut i8;
    let mut newalloc: size_t = 0;
    if ((*wsp).ws_wordv).is_null() {
        newalloc = if offs.wrapping_add(count) > 128 as i32 as u64 {
            count
        } else {
            128 as i32 as u64
        };
        ptr = calloc(newalloc, ::core::mem::size_of::<*mut i8>() as u64) as *mut *mut i8;
    } else if (*wsp).ws_wordn < offs.wrapping_add((*wsp).ws_wordc).wrapping_add(count) {
        newalloc = offs
            .wrapping_add((*wsp).ws_wordc)
            .wrapping_add(
                (if count > 128 as i32 as u64 { count } else { 128 as i32 as u64 }),
            );
        ptr = realloc(
            (*wsp).ws_wordv as *mut libc::c_void,
            newalloc.wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
        ) as *mut *mut i8;
    } else {
        return 0 as i32
    }
    if !ptr.is_null() {
        (*wsp).ws_wordn = newalloc;
        (*wsp).ws_wordv = ptr;
    } else {
        return _wsplt_nomem(wsp)
    }
    return 0 as i32;
}
unsafe extern "C" fn wsnode_flagstr(mut flags: u32) -> *const i8 {
    static mut retbuf: [i8; 7] = [0; 7];
    let mut p: *mut i8 = retbuf.as_mut_ptr();
    if flags & 0x2 as i32 as u32 != 0 {
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = 'w' as i32 as i8;
    } else if flags & 0x1 as i32 as u32 != 0 {
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = 'n' as i32 as i8;
    } else {
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = '-' as i32 as i8;
    }
    if flags & 0x4 as i32 as u32 != 0 {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = 'q' as i32 as i8;
    } else {
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = '-' as i32 as i8;
    }
    if flags & 0x8 as i32 as u32 != 0 {
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = 'E' as i32 as i8;
    } else {
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = '-' as i32 as i8;
    }
    if flags & 0x10 as i32 as u32 != 0 {
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = 'j' as i32 as i8;
    } else {
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = '-' as i32 as i8;
    }
    if flags & 0x20 as i32 as u32 != 0 {
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = 's' as i32 as i8;
    } else {
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = '-' as i32 as i8;
    }
    if flags & 0x40 as i32 as u32 != 0 {
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = 'd' as i32 as i8;
    } else {
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = '-' as i32 as i8;
    }
    *p = 0 as i32 as i8;
    return retbuf.as_mut_ptr();
}
unsafe extern "C" fn wsnode_ptr(
    mut wsp: *mut wordsplit,
    mut p: *mut wordsplit_node,
) -> *const i8 {
    if (*p).flags & 0x1 as i32 as u32 != 0 {
        return b"\0" as *const u8 as *const i8
    } else if (*p).flags & 0x2 as i32 as u32 != 0 {
        return (*p).v.word
    } else {
        return ((*wsp).ws_input).offset((*p).v.segm.beg as isize)
    };
}
unsafe extern "C" fn wsnode_len(mut p: *mut wordsplit_node) -> size_t {
    if (*p).flags & 0x1 as i32 as u32 != 0 {
        return 0 as i32 as size_t
    } else if (*p).flags & 0x2 as i32 as u32 != 0 {
        return strlen((*p).v.word)
    } else {
        return ((*p).v.segm.end).wrapping_sub((*p).v.segm.beg)
    };
}
unsafe extern "C" fn wsnode_new(
    mut wsp: *mut wordsplit,
    mut pnode: *mut *mut wordsplit_node,
) -> i32 {
    let mut node: *mut wordsplit_node = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<wordsplit_node>() as u64,
    ) as *mut wordsplit_node;
    if node.is_null() {
        return _wsplt_nomem(wsp);
    }
    *pnode = node;
    return 0 as i32;
}
unsafe extern "C" fn wsnode_free(mut p: *mut wordsplit_node) {
    if (*p).flags & 0x2 as i32 as u32 != 0 {
        rpl_free((*p).v.word as *mut libc::c_void);
    }
    rpl_free(p as *mut libc::c_void);
}
unsafe extern "C" fn wsnode_append(
    mut wsp: *mut wordsplit,
    mut node: *mut wordsplit_node,
) {
    (*node).next = 0 as *mut wordsplit_node;
    (*node).prev = (*wsp).ws_tail;
    if !((*wsp).ws_tail).is_null() {
        (*(*wsp).ws_tail).next = node;
    } else {
        (*wsp).ws_head = node;
    }
    (*wsp).ws_tail = node;
}
unsafe extern "C" fn wsnode_remove(
    mut wsp: *mut wordsplit,
    mut node: *mut wordsplit_node,
) {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*node).prev;
    if !p.is_null() {
        (*p).next = (*node).next;
        if ((*node).next).is_null() {
            (*p).flags &= !(0x10 as i32) as u32;
        }
    } else {
        (*wsp).ws_head = (*node).next;
    }
    p = (*node).next;
    if !p.is_null() {
        (*p).prev = (*node).prev;
    } else {
        (*wsp).ws_tail = (*node).prev;
    }
    (*node).prev = 0 as *mut wordsplit_node;
    (*node).next = (*node).prev;
}
unsafe extern "C" fn wsnode_tail(mut p: *mut wordsplit_node) -> *mut wordsplit_node {
    while !p.is_null() && !((*p).next).is_null() {
        p = (*p).next;
    }
    return p;
}
unsafe extern "C" fn wsnode_insert(
    mut wsp: *mut wordsplit,
    mut node: *mut wordsplit_node,
    mut anchor: *mut wordsplit_node,
    mut before: i32,
) {
    if ((*wsp).ws_head).is_null() {
        (*node).prev = 0 as *mut wordsplit_node;
        (*node).next = (*node).prev;
        (*wsp).ws_tail = node;
        (*wsp).ws_head = (*wsp).ws_tail;
    } else if before != 0 {
        if !((*anchor).prev).is_null() {
            wsnode_insert(wsp, node, (*anchor).prev, 0 as i32);
        } else {
            let mut tail: *mut wordsplit_node = wsnode_tail(node);
            (*node).prev = 0 as *mut wordsplit_node;
            (*tail).next = anchor;
            (*anchor).prev = tail;
            (*wsp).ws_head = node;
        }
    } else {
        let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
        let mut tail_0: *mut wordsplit_node = wsnode_tail(node);
        p = (*anchor).next;
        if !p.is_null() {
            (*p).prev = tail_0;
        } else {
            (*wsp).ws_tail = tail_0;
        }
        (*tail_0).next = p;
        (*node).prev = anchor;
        (*anchor).next = node;
    };
}
unsafe extern "C" fn wordsplit_add_segm(
    mut wsp: *mut wordsplit,
    mut beg: size_t,
    mut end: size_t,
    mut flg: i32,
) -> i32 {
    let mut node: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut rc: i32 = 0;
    if end == beg && flg & 0x100 as i32 == 0 {
        return 0 as i32;
    }
    rc = wsnode_new(wsp, &mut node);
    if rc != 0 {
        return rc;
    }
    (*node).flags = (flg & !(0x2 as i32 | 0x100 as i32)) as u32;
    (*node).v.segm.beg = beg;
    (*node).v.segm.end = end;
    wsnode_append(wsp, node);
    return 0 as i32;
}
unsafe extern "C" fn wordsplit_free_nodes(mut wsp: *mut wordsplit) {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut next: *mut wordsplit_node = (*p).next;
        wsnode_free(p);
        p = next;
    }
    (*wsp).ws_tail = 0 as *mut wordsplit_node;
    (*wsp).ws_head = (*wsp).ws_tail;
}
unsafe extern "C" fn wordsplit_dump_nodes(mut wsp: *mut wordsplit) {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut n: i32 = 0 as i32;
    p = (*wsp).ws_head;
    n = 0 as i32;
    while !p.is_null() {
        if (*p).flags & 0x2 as i32 as u32 != 0 {
            ((*wsp).ws_debug)
                .expect(
                    "non-null function pointer",
                )(
                b"(%02d) %4d: %p: %#04x (%s):%s;\0" as *const u8 as *const i8,
                (*wsp).ws_lvl,
                n,
                p,
                (*p).flags,
                wsnode_flagstr((*p).flags),
                (*p).v.word,
            );
        } else {
            ((*wsp).ws_debug)
                .expect(
                    "non-null function pointer",
                )(
                b"(%02d) %4d: %p: %#04x (%s):%.*s;\0" as *const u8 as *const i8,
                (*wsp).ws_lvl,
                n,
                p,
                (*p).flags,
                wsnode_flagstr((*p).flags),
                ((*p).v.segm.end).wrapping_sub((*p).v.segm.beg) as i32,
                ((*wsp).ws_input).offset((*p).v.segm.beg as isize),
            );
        }
        p = (*p).next;
        n += 1;
        n;
    }
}
unsafe extern "C" fn coalesce_segment(
    mut wsp: *mut wordsplit,
    mut node: *mut wordsplit_node,
) -> i32 {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut end: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut len: size_t = 0 as i32 as size_t;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut cur: *mut i8 = 0 as *mut i8;
    let mut stop: i32 = 0;
    if (*node).flags & 0x10 as i32 as u32 == 0 {
        return 0 as i32;
    }
    p = node;
    while !p.is_null() && (*p).flags & 0x10 as i32 as u32 != 0 {
        len = (len as u64).wrapping_add(wsnode_len(p)) as size_t as size_t;
        p = (*p).next;
    }
    if !p.is_null() {
        len = (len as u64).wrapping_add(wsnode_len(p)) as size_t as size_t;
    }
    end = p;
    buf = malloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
    if buf.is_null() {
        return _wsplt_nomem(wsp);
    }
    cur = buf;
    p = node;
    stop = 0 as i32;
    while stop == 0 {
        let mut next: *mut wordsplit_node = (*p).next;
        let mut str: *const i8 = wsnode_ptr(wsp, p);
        let mut slen: size_t = wsnode_len(p);
        memcpy(cur as *mut libc::c_void, str as *const libc::c_void, slen);
        cur = cur.offset(slen as isize);
        if p != node {
            (*node).flags |= (*p).flags & 0x4 as i32 as u32;
            wsnode_remove(wsp, p);
            stop = (p == end) as i32;
            wsnode_free(p);
        }
        p = next;
    }
    *cur = 0 as i32 as i8;
    (*node).flags &= !(0x10 as i32) as u32;
    if (*node).flags & 0x2 as i32 as u32 != 0 {
        rpl_free((*node).v.word as *mut libc::c_void);
    } else {
        (*node).flags |= 0x2 as i32 as u32;
    }
    (*node).v.word = buf;
    return 0 as i32;
}
unsafe extern "C" fn wsnode_quoteremoval(mut wsp: *mut wordsplit) -> i32 {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut str: *const i8 = wsnode_ptr(wsp, p);
        let mut slen: size_t = wsnode_len(p);
        let mut unquote: i32 = 0;
        if (*wsp).ws_flags & (0x200 as i32 | 0x400 as i32) as u32 != 0 {
            unquote = ((*p).flags & 0x8 as i32 as u32 == 0) as i32;
        } else {
            unquote = 0 as i32;
        }
        if unquote != 0 {
            if (*p).flags & 0x2 as i32 as u32 == 0 {
                let mut newstr: *mut i8 = malloc(slen.wrapping_add(1 as i32 as u64))
                    as *mut i8;
                if newstr.is_null() {
                    return _wsplt_nomem(wsp);
                }
                memcpy(newstr as *mut libc::c_void, str as *const libc::c_void, slen);
                *newstr.offset(slen as isize) = 0 as i32 as i8;
                (*p).v.word = newstr;
                (*p).flags |= 0x2 as i32 as u32;
            }
            wordsplit_string_unquote_copy(
                wsp,
                ((*p).flags & 0x4 as i32 as u32) as i32,
                (*p).v.word,
                str,
                slen,
            );
        }
        p = (*p).next;
    }
    return 0 as i32;
}
unsafe extern "C" fn wsnode_coalesce(mut wsp: *mut wordsplit) -> i32 {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        if (*p).flags & 0x10 as i32 as u32 != 0 {
            if coalesce_segment(wsp, p) != 0 {
                return 1 as i32;
            }
        }
        p = (*p).next;
    }
    return 0 as i32;
}
unsafe extern "C" fn wsnode_tail_coalesce(
    mut wsp: *mut wordsplit,
    mut p: *mut wordsplit_node,
) -> i32 {
    if !((*p).next).is_null() {
        let mut np: *mut wordsplit_node = p;
        while !np.is_null() && !((*np).next).is_null() {
            (*np).flags |= 0x10 as i32 as u32;
            np = (*np).next;
        }
        if coalesce_segment(wsp, p) != 0 {
            return 1 as i32;
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn wordsplit_finish(mut wsp: *mut wordsplit) -> i32 {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut n: size_t = 0;
    let mut delim: i32 = 0;
    loop {
        delim = 0 as i32;
        n = 0 as i32 as size_t;
        p = (*wsp).ws_head;
        while !p.is_null() {
            let mut next: *mut wordsplit_node = (*p).next;
            if (*p).flags & 0x40 as i32 as u32 != 0 {
                if (*wsp).ws_flags & 0x1000 as i32 as u32 != 0 {
                    if (*wsp).ws_flags & 0x800 as i32 as u32 != 0 {
                        let mut s: *const i8 = wsnode_ptr(wsp, p);
                        if delim != 0 {
                            if delim == *s as i32 {
                                wsnode_remove(wsp, p);
                                p = next;
                                continue;
                            } else {
                                delim = 0 as i32;
                                n = n.wrapping_add(1);
                                n;
                            }
                        } else {
                            delim = *s as i32;
                            p = next;
                            continue;
                        }
                    }
                } else if (*wsp).ws_options & 0x80 as i32 as u32 != 0 {
                    wsnode_remove(wsp, p);
                    p = next;
                    continue;
                }
            } else {
                if delim != 0 {
                    n = n.wrapping_add(1);
                    n;
                    delim = 0 as i32;
                }
                if (*wsp).ws_options & 0x80 as i32 as u32 != 0 {
                    if ((*wsp).ws_wordi).wrapping_add(n).wrapping_add(1 as i32 as u64)
                        == (*wsp).ws_maxwords
                    {
                        break;
                    }
                }
            }
            n = n.wrapping_add(1);
            n;
            if (*wsp).ws_flags & 0x20000000 as i32 as u32 != 0 {
                p = 0 as *mut wordsplit_node;
            } else {
                p = next;
            }
        }
        if !p.is_null() {
            if wsnode_tail_coalesce(wsp, p) != 0 {
                return (*wsp).ws_errno;
            }
            n = n.wrapping_add(1);
            n;
        }
        if !(n == 0 as i32 as u64 && (*wsp).ws_flags & 0x20000000 as i32 as u32 != 0) {
            break;
        }
        if (*wsp).ws_endp < (*wsp).ws_len {
            let mut rc: i32 = 0;
            if (*wsp).ws_flags & 0x200000 as i32 as u32 != 0 {
                ((*wsp).ws_debug)
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"Restarting\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            rc = wordsplit_process_list(wsp, skip_delim(wsp));
            if rc != 0 {
                return rc;
            }
        } else {
            (*wsp).ws_error = None;
            return 0 as i32;
        }
    }
    if alloc_space(wsp, n.wrapping_add(1 as i32 as u64)) != 0 {
        return (*wsp).ws_errno;
    }
    while !((*wsp).ws_head).is_null() {
        let mut str: *const i8 = wsnode_ptr(wsp, (*wsp).ws_head);
        let mut slen: size_t = wsnode_len((*wsp).ws_head);
        let mut newstr: *mut i8 = malloc(slen.wrapping_add(1 as i32 as u64)) as *mut i8;
        let ref mut fresh13 = *((*wsp).ws_wordv)
            .offset(((*wsp).ws_offs).wrapping_add((*wsp).ws_wordc) as isize);
        *fresh13 = newstr;
        if newstr.is_null() {
            return _wsplt_nomem(wsp);
        }
        memcpy(newstr as *mut libc::c_void, str as *const libc::c_void, slen);
        *newstr.offset(slen as isize) = 0 as i32 as i8;
        wsnode_remove(wsp, (*wsp).ws_head);
        (*wsp).ws_wordc = ((*wsp).ws_wordc).wrapping_add(1);
        (*wsp).ws_wordc;
        (*wsp).ws_wordi = ((*wsp).ws_wordi).wrapping_add(1);
        (*wsp).ws_wordi;
        if (*wsp).ws_flags & 0x20000000 as i32 as u32 != 0 {
            break;
        }
    }
    let ref mut fresh14 = *((*wsp).ws_wordv)
        .offset(((*wsp).ws_offs).wrapping_add((*wsp).ws_wordc) as isize);
    *fresh14 = 0 as *mut i8;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit_append(
    mut wsp: *mut wordsplit_t,
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> i32 {
    let mut rc: i32 = 0;
    let mut i: size_t = 0;
    rc = alloc_space(
        wsp,
        ((*wsp).ws_wordc).wrapping_add(argc as u64).wrapping_add(1 as i32 as u64),
    );
    if rc != 0 {
        return rc;
    }
    i = 0 as i32 as size_t;
    while i < argc as u64 {
        let mut newstr: *mut i8 = strdup(*argv.offset(i as isize));
        if newstr.is_null() {
            while i > 0 as i32 as u64 {
                rpl_free(
                    *((*wsp).ws_wordv)
                        .offset(
                            ((*wsp).ws_offs)
                                .wrapping_add((*wsp).ws_wordc)
                                .wrapping_add(i)
                                .wrapping_sub(1 as i32 as u64) as isize,
                        ) as *mut libc::c_void,
                );
                let ref mut fresh15 = *((*wsp).ws_wordv)
                    .offset(
                        ((*wsp).ws_offs)
                            .wrapping_add((*wsp).ws_wordc)
                            .wrapping_add(i)
                            .wrapping_sub(1 as i32 as u64) as isize,
                    );
                *fresh15 = 0 as *mut i8;
                i = i.wrapping_sub(1);
                i;
            }
            return _wsplt_nomem(wsp);
        }
        let ref mut fresh16 = *((*wsp).ws_wordv)
            .offset(
                ((*wsp).ws_offs).wrapping_add((*wsp).ws_wordc).wrapping_add(i) as isize,
            );
        *fresh16 = newstr;
        i = i.wrapping_add(1);
        i;
    }
    (*wsp).ws_wordc = ((*wsp).ws_wordc as u64).wrapping_add(i) as size_t as size_t;
    let ref mut fresh17 = *((*wsp).ws_wordv)
        .offset(((*wsp).ws_offs).wrapping_add((*wsp).ws_wordc) as isize);
    *fresh17 = 0 as *mut i8;
    return 0 as i32;
}
unsafe extern "C" fn node_split_prefix(
    mut wsp: *mut wordsplit,
    mut ptail: *mut *mut wordsplit_node,
    mut node: *mut wordsplit_node,
    mut beg: size_t,
    mut len: size_t,
    mut flg: i32,
) -> i32 {
    let mut newnode: *mut wordsplit_node = 0 as *mut wordsplit_node;
    if len == 0 as i32 as u64 {
        return 0 as i32;
    }
    if wsnode_new(wsp, &mut newnode) != 0 {
        return 1 as i32;
    }
    wsnode_insert(wsp, newnode, *ptail, 0 as i32);
    if (*node).flags & 0x2 as i32 as u32 != 0 {
        let mut str: *const i8 = wsnode_ptr(wsp, node);
        let mut newstr: *mut i8 = malloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
        if newstr.is_null() {
            return _wsplt_nomem(wsp);
        }
        memcpy(
            newstr as *mut libc::c_void,
            str.offset(beg as isize) as *const libc::c_void,
            len,
        );
        *newstr.offset(len as isize) = 0 as i32 as i8;
        (*newnode).flags = 0x2 as i32 as u32;
        (*newnode).v.word = newstr;
    } else {
        (*newnode).v.segm.beg = ((*node).v.segm.beg).wrapping_add(beg);
        (*newnode).v.segm.end = ((*newnode).v.segm.beg).wrapping_add(len);
    }
    (*newnode).flags |= flg as u32;
    *ptail = newnode;
    return 0 as i32;
}
unsafe extern "C" fn find_closing_paren(
    mut str: *const i8,
    mut i: size_t,
    mut len: size_t,
    mut poff: *mut size_t,
    mut paren: *const i8,
) -> i32 {
    let mut state: C2RustUnnamed_1 = st_init;
    let mut level: size_t = 1 as i32 as size_t;
    while i < len {
        match state as u32 {
            0 => {
                match *str.offset(i as isize) as i32 {
                    34 => {
                        state = st_dquote;
                    }
                    39 => {
                        state = st_squote;
                    }
                    _ => {
                        if *str.offset(i as isize) as i32
                            == *paren.offset(0 as i32 as isize) as i32
                        {
                            level = level.wrapping_add(1);
                            level;
                        } else if *str.offset(i as isize) as i32
                            == *paren.offset(1 as i32 as isize) as i32
                        {
                            level = level.wrapping_sub(1);
                            if level == 0 as i32 as u64 {
                                *poff = i;
                                return 0 as i32;
                            }
                        }
                    }
                }
            }
            1 => {
                if *str.offset(i as isize) as i32 == '\'' as i32 {
                    state = st_init;
                }
            }
            2 => {
                if *str.offset(i as isize) as i32 == '\\' as i32 {
                    i = i.wrapping_add(1);
                    i;
                } else if *str.offset(i as isize) as i32 == '"' as i32 {
                    state = st_init;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
unsafe extern "C" fn wordsplit_find_env(
    mut wsp: *mut wordsplit,
    mut name: *const i8,
    mut len: size_t,
    mut ret: *mut *const i8,
) -> i32 {
    let mut i: size_t = 0;
    if (*wsp).ws_flags & 0x80000 as i32 as u32 == 0 {
        return 5 as i32;
    }
    if (*wsp).ws_flags & 0x8000000 as i32 as u32 != 0 {
        i = 0 as i32 as size_t;
        while !(*((*wsp).ws_env).offset(i as isize)).is_null() {
            let mut elen: size_t = strlen(*((*wsp).ws_env).offset(i as isize));
            if elen == len
                && memcmp(
                    *((*wsp).ws_env).offset(i as isize) as *const libc::c_void,
                    name as *const libc::c_void,
                    elen,
                ) == 0 as i32
            {
                *ret = *((*wsp).ws_env).offset(i.wrapping_add(1 as i32 as u64) as isize);
                return 0 as i32;
            }
            i = i.wrapping_add(1);
            i;
            if (*((*wsp).ws_env).offset(i as isize)).is_null() {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if !((*wsp).ws_env).is_null() {
        i = 0 as i32 as size_t;
        while !(*((*wsp).ws_env).offset(i as isize)).is_null() {
            let mut j: size_t = 0;
            let mut var: *const i8 = *((*wsp).ws_env).offset(i as isize);
            j = 0 as i32 as size_t;
            while j < len {
                if *name.offset(j as isize) as i32 != *var.offset(j as isize) as i32 {
                    break;
                }
                j = j.wrapping_add(1);
                j;
            }
            if j == len && *var.offset(j as isize) as i32 == '=' as i32 {
                *ret = var.offset(j as isize).offset(1 as i32 as isize);
                return 0 as i32;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 5 as i32;
}
unsafe extern "C" fn wsplt_assign_var(
    mut wsp: *mut wordsplit,
    mut name: *const i8,
    mut namelen: size_t,
    mut value: *mut i8,
) -> i32 {
    let mut n: i32 = if (*wsp).ws_flags & 0x8000000 as i32 as u32 != 0 {
        2 as i32
    } else {
        1 as i32
    };
    let mut v: *mut i8 = 0 as *mut i8;
    if ((*wsp).ws_envidx).wrapping_add(n as u64) >= (*wsp).ws_envsiz {
        let mut sz: size_t = 0;
        let mut newenv: *mut *mut i8 = 0 as *mut *mut i8;
        if ((*wsp).ws_envbuf).is_null() {
            if (*wsp).ws_flags & 0x80000 as i32 as u32 != 0 {
                let mut i: size_t = 0 as i32 as size_t;
                let mut j: size_t = 0;
                if !((*wsp).ws_env).is_null() {
                    while !(*((*wsp).ws_env).offset(i as isize)).is_null() {
                        i = i.wrapping_add(1);
                        i;
                    }
                }
                sz = i.wrapping_add(n as u64).wrapping_add(1 as i32 as u64);
                newenv = calloc(sz, ::core::mem::size_of::<*mut i8>() as u64)
                    as *mut *mut i8;
                if newenv.is_null() {
                    return _wsplt_nomem(wsp);
                }
                j = 0 as i32 as size_t;
                while j < i {
                    let ref mut fresh18 = *newenv.offset(j as isize);
                    *fresh18 = strdup(*((*wsp).ws_env).offset(j as isize));
                    if (*newenv.offset(j as isize)).is_null() {
                        while j > 1 as i32 as u64 {
                            rpl_free(
                                *newenv.offset(j.wrapping_sub(1 as i32 as u64) as isize)
                                    as *mut libc::c_void,
                            );
                            j = j.wrapping_sub(1);
                            j;
                        }
                        rpl_free(
                            *newenv.offset(j.wrapping_sub(1 as i32 as u64) as isize)
                                as *mut libc::c_void,
                        );
                        rpl_free(newenv as *mut libc::c_void);
                        return _wsplt_nomem(wsp);
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                let ref mut fresh19 = *newenv.offset(j as isize);
                *fresh19 = 0 as *mut i8;
                (*wsp).ws_envbuf = newenv;
                (*wsp).ws_envidx = i;
                (*wsp).ws_envsiz = sz;
                (*wsp).ws_env = (*wsp).ws_envbuf as *mut *const i8;
            } else {
                newenv = calloc(
                    16 as i32 as u64,
                    ::core::mem::size_of::<*mut i8>() as u64,
                ) as *mut *mut i8;
                if newenv.is_null() {
                    return _wsplt_nomem(wsp);
                }
                (*wsp).ws_envbuf = newenv;
                (*wsp).ws_envidx = 0 as i32 as size_t;
                (*wsp).ws_envsiz = 16 as i32 as size_t;
                (*wsp).ws_env = (*wsp).ws_envbuf as *mut *const i8;
                (*wsp).ws_flags |= 0x80000 as i32 as u32;
            }
        } else {
            (*wsp).ws_envsiz = ((*wsp).ws_envsiz as u64).wrapping_mul(2 as i32 as u64)
                as size_t as size_t;
            newenv = realloc(
                (*wsp).ws_envbuf as *mut libc::c_void,
                ((*wsp).ws_envsiz).wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
            ) as *mut *mut i8;
            if newenv.is_null() {
                return _wsplt_nomem(wsp);
            }
            (*wsp).ws_envbuf = newenv;
            (*wsp).ws_env = (*wsp).ws_envbuf as *mut *const i8;
        }
    }
    if (*wsp).ws_flags & 0x8000000 as i32 as u32 != 0 {
        let mut p: *mut i8 = malloc(namelen.wrapping_add(1 as i32 as u64)) as *mut i8;
        if p.is_null() {
            return _wsplt_nomem(wsp);
        }
        memcpy(p as *mut libc::c_void, name as *const libc::c_void, namelen);
        *p.offset(namelen as isize) = 0 as i32 as i8;
        v = strdup(value);
        if v.is_null() {
            rpl_free(p as *mut libc::c_void);
            return _wsplt_nomem(wsp);
        }
        let fresh20 = (*wsp).ws_envidx;
        (*wsp).ws_envidx = ((*wsp).ws_envidx).wrapping_add(1);
        let ref mut fresh21 = *((*wsp).ws_env).offset(fresh20 as isize);
        *fresh21 = p;
        let fresh22 = (*wsp).ws_envidx;
        (*wsp).ws_envidx = ((*wsp).ws_envidx).wrapping_add(1);
        let ref mut fresh23 = *((*wsp).ws_env).offset(fresh22 as isize);
        *fresh23 = v;
    } else {
        v = malloc(namelen.wrapping_add(strlen(value)).wrapping_add(2 as i32 as u64))
            as *mut i8;
        if v.is_null() {
            return _wsplt_nomem(wsp);
        }
        memcpy(v as *mut libc::c_void, name as *const libc::c_void, namelen);
        let fresh24 = namelen;
        namelen = namelen.wrapping_add(1);
        *v.offset(fresh24 as isize) = '=' as i32 as i8;
        strcpy(v.offset(namelen as isize), value);
        let fresh25 = (*wsp).ws_envidx;
        (*wsp).ws_envidx = ((*wsp).ws_envidx).wrapping_add(1);
        let ref mut fresh26 = *((*wsp).ws_env).offset(fresh25 as isize);
        *fresh26 = v;
    }
    let fresh27 = (*wsp).ws_envidx;
    (*wsp).ws_envidx = ((*wsp).ws_envidx).wrapping_add(1);
    let ref mut fresh28 = *((*wsp).ws_env).offset(fresh27 as isize);
    *fresh28 = 0 as *const i8;
    return 0 as i32;
}
unsafe extern "C" fn expvar(
    mut wsp: *mut wordsplit,
    mut str: *const i8,
    mut len: size_t,
    mut ptail: *mut *mut wordsplit_node,
    mut pend: *mut *const i8,
    mut flg: u32,
) -> i32 {
    let mut i: size_t = 0 as i32 as size_t;
    let mut defstr: *const i8 = 0 as *const i8;
    let mut value: *mut i8 = 0 as *mut i8;
    let mut vptr: *const i8 = 0 as *const i8;
    let mut newnode: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut start: *const i8 = str.offset(-(1 as i32 as isize));
    let mut rc: i32 = 0;
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
    if 'A' as i32 as u32 <= *str.offset(0 as i32 as isize) as u32
        && *str.offset(0 as i32 as isize) as u32 <= 'Z' as i32 as u32
        || 'a' as i32 as u32 <= *str.offset(0 as i32 as isize) as u32
            && *str.offset(0 as i32 as isize) as u32 <= 'z' as i32 as u32
        || *str.offset(0 as i32 as isize) as i32 == '_' as i32
    {
        i = 1 as i32 as size_t;
        while i < len {
            if !('A' as i32 as u32 <= *str.offset(i as isize) as u32
                && *str.offset(i as isize) as u32 <= 'Z' as i32 as u32
                || 'a' as i32 as u32 <= *str.offset(i as isize) as u32
                    && *str.offset(i as isize) as u32 <= 'z' as i32 as u32
                || '0' as i32 as u32 <= *str.offset(i as isize) as u32
                    && *str.offset(i as isize) as u32 <= '9' as i32 as u32
                || *str.offset(i as isize) as i32 == '_' as i32)
            {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        *pend = str.offset(i as isize).offset(-(1 as i32 as isize));
    } else if *str.offset(0 as i32 as isize) as i32 == '{' as i32 {
        str = str.offset(1);
        str;
        len = len.wrapping_sub(1);
        len;
        i = 1 as i32 as size_t;
        while i < len {
            if *str.offset(i as isize) as i32 == ':' as i32 {
                let mut j: size_t = 0;
                defstr = str.offset(i as isize).offset(1 as i32 as isize);
                if find_closing_paren(
                    str,
                    i.wrapping_add(1 as i32 as u64),
                    len,
                    &mut j,
                    b"{}\0" as *const u8 as *const i8,
                ) != 0
                {
                    return _wsplt_seterr(wsp, 4 as i32);
                }
                *pend = str.offset(j as isize);
                break;
            } else if *str.offset(i as isize) as i32 == '}' as i32 {
                defstr = 0 as *const i8;
                *pend = str.offset(i as isize);
                break;
            } else if !(strchr(
                b"-+?=\0" as *const u8 as *const i8,
                *str.offset(i as isize) as i32,
            ))
                .is_null()
            {
                let mut j_0: size_t = 0;
                defstr = str.offset(i as isize);
                if find_closing_paren(
                    str,
                    i,
                    len,
                    &mut j_0,
                    b"{}\0" as *const u8 as *const i8,
                ) != 0
                {
                    return _wsplt_seterr(wsp, 4 as i32);
                }
                *pend = str.offset(j_0 as isize);
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        if i == len {
            return _wsplt_seterr(wsp, 4 as i32);
        }
    } else {
        if wsnode_new(wsp, &mut newnode) != 0 {
            return 1 as i32;
        }
        wsnode_insert(wsp, newnode, *ptail, 0 as i32);
        *ptail = newnode;
        (*newnode).flags = 0x2 as i32 as u32 | flg;
        (*newnode).v.word = malloc(3 as i32 as u64) as *mut i8;
        if ((*newnode).v.word).is_null() {
            return _wsplt_nomem(wsp);
        }
        *((*newnode).v.word).offset(0 as i32 as isize) = '$' as i32 as i8;
        *((*newnode).v.word).offset(1 as i32 as isize) = *str.offset(0 as i32 as isize);
        *((*newnode).v.word).offset(2 as i32 as isize) = 0 as i32 as i8;
        *pend = str;
        return 0 as i32;
    }
    if !defstr.is_null()
        && (strchr(
            b"-+?=\0" as *const u8 as *const i8,
            *defstr.offset(0 as i32 as isize) as i32,
        ))
            .is_null()
    {
        rc = 5 as i32;
        defstr = 0 as *const i8;
    } else {
        rc = wordsplit_find_env(wsp, str, i, &mut vptr);
        if rc == 0 as i32 {
            if !vptr.is_null() {
                value = strdup(vptr);
                if value.is_null() {
                    rc = 2 as i32;
                }
            } else {
                rc = 5 as i32;
            }
        } else if (*wsp).ws_flags & 0x100000 as i32 as u32 != 0 {
            rc = ((*wsp).ws_getvar)
                .expect(
                    "non-null function pointer",
                )(&mut value, str, i, (*wsp).ws_closure);
        } else {
            rc = 5 as i32;
        }
        if rc == 0 as i32
            && (value.is_null() || *value.offset(0 as i32 as isize) as i32 == 0 as i32)
            && !defstr.is_null()
            && *defstr.offset(-(1 as i32) as isize) as i32 == ':' as i32
        {
            rpl_free(value as *mut libc::c_void);
            rc = 5 as i32;
        }
    }
    let mut current_block_112: u64;
    match rc {
        0 => {
            if !defstr.is_null() && *defstr as i32 == '+' as i32 {
                defstr = defstr.offset(1);
                let mut size: size_t = (*pend).offset_from(defstr) as i64 as size_t;
                rc = _wsplt_subsplit(
                    wsp,
                    &mut ws,
                    defstr,
                    size as i32,
                    (0x400000 as i32 | 0x100 as i32 | (0x200 as i32 | 0x400 as i32))
                        as u32 | (*wsp).ws_flags & (0x40 as i32 | 0x4 as i32) as u32,
                    1 as i32,
                );
                if rc != 0 {
                    return rc;
                }
                rpl_free(value as *mut libc::c_void);
                value = *(ws.ws_wordv).offset(0 as i32 as isize);
                let ref mut fresh29 = *(ws.ws_wordv).offset(0 as i32 as isize);
                *fresh29 = 0 as *mut i8;
                wordsplit_free(&mut ws);
            }
            current_block_112 = 14865402277128115059;
        }
        5 => {
            if !defstr.is_null() {
                let mut size_0: size_t = 0;
                if *defstr as i32 == '-' as i32 || *defstr as i32 == '=' as i32 {
                    defstr = defstr.offset(1);
                    size_0 = (*pend).offset_from(defstr) as i64 as size_t;
                    rc = _wsplt_subsplit(
                        wsp,
                        &mut ws,
                        defstr,
                        size_0 as i32,
                        (0x400000 as i32 | 0x100 as i32 | (0x200 as i32 | 0x400 as i32))
                            as u32 | (*wsp).ws_flags & (0x40 as i32 | 0x4 as i32) as u32,
                        1 as i32,
                    );
                    if rc != 0 {
                        return rc;
                    }
                    value = *(ws.ws_wordv).offset(0 as i32 as isize);
                    let ref mut fresh30 = *(ws.ws_wordv).offset(0 as i32 as isize);
                    *fresh30 = 0 as *mut i8;
                    wordsplit_free(&mut ws);
                    if *defstr.offset(-(1 as i32) as isize) as i32 == '=' as i32 {
                        wsplt_assign_var(wsp, str, i, value);
                    }
                } else {
                    if *defstr as i32 == '?' as i32 {
                        defstr = defstr.offset(1);
                        size_0 = (*pend).offset_from(defstr) as i64 as size_t;
                        if size_0 == 0 as i32 as u64 {
                            ((*wsp).ws_error)
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"%.*s: variable null or not set\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                i as i32,
                                str,
                            );
                        } else {
                            rc = _wsplt_subsplit(
                                wsp,
                                &mut ws,
                                defstr,
                                size_0 as i32,
                                (0x400000 as i32 | 0x100 as i32
                                    | (0x200 as i32 | 0x400 as i32)) as u32
                                    | (*wsp).ws_flags & (0x40 as i32 | 0x4 as i32) as u32,
                                1 as i32,
                            );
                            if rc == 0 as i32 {
                                ((*wsp).ws_error)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    b"%.*s: %s\0" as *const u8 as *const i8,
                                    i as i32,
                                    str,
                                    *(ws.ws_wordv).offset(0 as i32 as isize),
                                );
                            } else {
                                ((*wsp).ws_error)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    b"%.*s: %.*s\0" as *const u8 as *const i8,
                                    i as i32,
                                    str,
                                    size_0 as i32,
                                    defstr,
                                );
                            }
                            wordsplit_free(&mut ws);
                        }
                    }
                    value = 0 as *mut i8;
                }
            } else if (*wsp).ws_flags & 0x20 as i32 as u32 != 0 {
                _wsplt_seterr(wsp, 5 as i32);
                return 1 as i32;
            } else {
                if (*wsp).ws_flags & 0x1000000 as i32 as u32 != 0 {
                    ((*wsp).ws_error)
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"warning: undefined variable `%.*s'\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        i as i32,
                        str,
                    );
                }
                if (*wsp).ws_flags & 0x800000 as i32 as u32 != 0 {
                    value = 0 as *mut i8;
                } else {
                    value = strdup(b"\0" as *const u8 as *const i8);
                    if value.is_null() {
                        return _wsplt_nomem(wsp);
                    }
                }
            }
            current_block_112 = 14865402277128115059;
        }
        2 => return _wsplt_nomem(wsp),
        9 => {
            if (*wsp).ws_errno == 9 as i32 {
                rpl_free((*wsp).ws_usererr as *mut libc::c_void);
            }
            (*wsp).ws_usererr = value;
            current_block_112 = 3458014403435832185;
        }
        _ => {
            current_block_112 = 3458014403435832185;
        }
    }
    match current_block_112 {
        14865402277128115059 => {}
        _ => {
            _wsplt_seterr(wsp, rc);
            return 1 as i32;
        }
    }
    if !value.is_null() {
        if flg & 0x4 as i32 as u32 != 0 {
            if wsnode_new(wsp, &mut newnode) != 0 {
                rpl_free(value as *mut libc::c_void);
                return 1 as i32;
            }
            wsnode_insert(wsp, newnode, *ptail, 0 as i32);
            *ptail = newnode;
            (*newnode).flags = (0x2 as i32 | 0x8 as i32) as u32 | flg;
            (*newnode).v.word = value;
        } else if *value as i32 == 0 as i32 {
            rpl_free(value as *mut libc::c_void);
            if wsnode_new(wsp, &mut newnode) != 0 {
                return 1 as i32;
            }
            wsnode_insert(wsp, newnode, *ptail, 0 as i32);
            *ptail = newnode;
            (*newnode).flags = 0x1 as i32 as u32;
        } else {
            let mut ws_0: wordsplit = wordsplit {
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
            let mut rc_0: i32 = 0;
            rc_0 = _wsplt_subsplit(
                wsp,
                &mut ws_0,
                value,
                strlen(value) as i32,
                (0x40 as i32 | 0x4 as i32 | (0x200 as i32 | 0x400 as i32)
                    | (if (*wsp).ws_flags & 0x1000 as i32 as u32 != 0
                        || (*wsp).ws_options & 0x80 as i32 as u32 != 0
                    {
                        0x1000 as i32
                    } else {
                        0 as i32
                    })) as u32,
                0 as i32,
            );
            rpl_free(value as *mut libc::c_void);
            if rc_0 != 0 {
                _wsplt_seterr_sub(wsp, &mut ws_0);
                wordsplit_free(&mut ws_0);
                return 1 as i32;
            }
            wsnode_insert(wsp, ws_0.ws_head, *ptail, 0 as i32);
            *ptail = ws_0.ws_tail;
            ws_0.ws_tail = 0 as *mut wordsplit_node;
            ws_0.ws_head = ws_0.ws_tail;
            wordsplit_free(&mut ws_0);
        }
    } else if (*wsp).ws_flags & 0x800000 as i32 as u32 != 0 {
        let mut size_1: size_t = ((*pend).offset_from(start) as i64 + 1 as i32 as i64)
            as size_t;
        if wsnode_new(wsp, &mut newnode) != 0 {
            return 1 as i32;
        }
        wsnode_insert(wsp, newnode, *ptail, 0 as i32);
        *ptail = newnode;
        (*newnode).flags = (0x2 as i32 | 0x8 as i32) as u32 | flg;
        (*newnode).v.word = malloc(size_1.wrapping_add(1 as i32 as u64)) as *mut i8;
        if ((*newnode).v.word).is_null() {
            return _wsplt_nomem(wsp);
        }
        memcpy(
            (*newnode).v.word as *mut libc::c_void,
            start as *const libc::c_void,
            size_1,
        );
        *((*newnode).v.word).offset(size_1 as isize) = 0 as i32 as i8;
    } else {
        if wsnode_new(wsp, &mut newnode) != 0 {
            return 1 as i32;
        }
        wsnode_insert(wsp, newnode, *ptail, 0 as i32);
        *ptail = newnode;
        (*newnode).flags = 0x1 as i32 as u32;
    }
    return 0 as i32;
}
unsafe extern "C" fn begin_var_p(mut c: i32) -> i32 {
    return (c == '{' as i32
        || ('A' as i32 as u32 <= c as u32 && c as u32 <= 'Z' as i32 as u32
            || 'a' as i32 as u32 <= c as u32 && c as u32 <= 'z' as i32 as u32
            || c == '_' as i32)) as i32;
}
unsafe extern "C" fn node_expand(
    mut wsp: *mut wordsplit,
    mut node: *mut wordsplit_node,
    mut beg_p: Option<unsafe extern "C" fn(i32) -> i32>,
    mut ws_exp_fn: Option<
        unsafe extern "C" fn(
            *mut wordsplit,
            *const i8,
            size_t,
            *mut *mut wordsplit_node,
            *mut *const i8,
            u32,
        ) -> i32,
    >,
) -> i32 {
    let mut str: *const i8 = wsnode_ptr(wsp, node);
    let mut slen: size_t = wsnode_len(node);
    let mut end: *const i8 = str.offset(slen as isize);
    let mut p: *const i8 = 0 as *const i8;
    let mut off: size_t = 0 as i32 as size_t;
    let mut tail: *mut wordsplit_node = node;
    p = str;
    while p < end {
        if *p as i32 == '\\' as i32 {
            p = p.offset(1);
            p;
        } else if *p as i32 == '$' as i32
            && beg_p
                .expect("non-null function pointer")(*p.offset(1 as i32 as isize) as i32)
                != 0
        {
            let mut n: size_t = p.offset_from(str) as i64 as size_t;
            if tail != node {
                (*tail).flags |= 0x10 as i32 as u32;
            }
            if node_split_prefix(wsp, &mut tail, node, off, n, 0x10 as i32) != 0 {
                return 1 as i32;
            }
            p = p.offset(1);
            p;
            if ws_exp_fn
                .expect(
                    "non-null function pointer",
                )(
                wsp,
                p,
                slen.wrapping_sub(n),
                &mut tail,
                &mut p,
                (*node).flags & (0x10 as i32 | 0x4 as i32) as u32,
            ) != 0
            {
                return 1 as i32;
            }
            off = (off as u64)
                .wrapping_add((p.offset_from(str) as i64 + 1 as i32 as i64) as u64)
                as size_t as size_t;
            str = p.offset(1 as i32 as isize);
        }
        p = p.offset(1);
        p;
    }
    if p > str {
        if tail != node {
            (*tail).flags |= 0x10 as i32 as u32;
        }
        if node_split_prefix(
            wsp,
            &mut tail,
            node,
            off,
            p.offset_from(str) as i64 as size_t,
            ((*node).flags & (0x10 as i32 | 0x4 as i32) as u32) as i32,
        ) != 0
        {
            return 1 as i32;
        }
    }
    if tail != node {
        wsnode_remove(wsp, node);
        wsnode_free(node);
    }
    return 0 as i32;
}
unsafe extern "C" fn wsnode_nullelim(mut wsp: *mut wordsplit) {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut next: *mut wordsplit_node = (*p).next;
        if (*p).flags & 0x40 as i32 as u32 != 0 && !((*p).prev).is_null() {
            (*(*p).prev).flags &= !(0x10 as i32) as u32;
        }
        if (*p).flags & 0x1 as i32 as u32 != 0 {
            wsnode_remove(wsp, p);
            wsnode_free(p);
        }
        p = next;
    }
}
unsafe extern "C" fn wordsplit_varexp(mut wsp: *mut wordsplit) -> i32 {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut next: *mut wordsplit_node = (*p).next;
        if (*p).flags & (0x8 as i32 | 0x40 as i32) as u32 == 0 {
            if node_expand(
                wsp,
                p,
                Some(begin_var_p as unsafe extern "C" fn(i32) -> i32),
                Some(
                    expvar
                        as unsafe extern "C" fn(
                            *mut wordsplit,
                            *const i8,
                            size_t,
                            *mut *mut wordsplit_node,
                            *mut *const i8,
                            u32,
                        ) -> i32,
                ),
            ) != 0
            {
                return 1 as i32;
            }
        }
        p = next;
    }
    wsnode_nullelim(wsp);
    return 0 as i32;
}
unsafe extern "C" fn begin_cmd_p(mut c: i32) -> i32 {
    return (c == '(' as i32) as i32;
}
unsafe extern "C" fn expcmd(
    mut wsp: *mut wordsplit,
    mut str: *const i8,
    mut len: size_t,
    mut ptail: *mut *mut wordsplit_node,
    mut pend: *mut *const i8,
    mut flg: u32,
) -> i32 {
    let mut rc: i32 = 0;
    let mut j: size_t = 0;
    let mut value: *mut i8 = 0 as *mut i8;
    let mut newnode: *mut wordsplit_node = 0 as *mut wordsplit_node;
    str = str.offset(1);
    str;
    len = len.wrapping_sub(1);
    len;
    if find_closing_paren(
        str,
        0 as i32 as size_t,
        len,
        &mut j,
        b"()\0" as *const u8 as *const i8,
    ) != 0
    {
        _wsplt_seterr(wsp, 7 as i32);
        return 1 as i32;
    }
    *pend = str.offset(j as isize);
    if (*wsp).ws_options & 0x8 as i32 as u32 != 0 {
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
        rc = _wsplt_subsplit(
            wsp,
            &mut ws,
            str,
            j as i32,
            (0x100 as i32 | (0x200 as i32 | 0x400 as i32)) as u32,
            1 as i32,
        );
        if rc != 0 {
            _wsplt_seterr_sub(wsp, &mut ws);
            wordsplit_free(&mut ws);
            return 1 as i32;
        }
        rc = ((*wsp).ws_command)
            .expect(
                "non-null function pointer",
            )(&mut value, str, j, ws.ws_wordv, (*wsp).ws_closure);
        wordsplit_free(&mut ws);
    } else {
        rc = ((*wsp).ws_command)
            .expect(
                "non-null function pointer",
            )(&mut value, str, j, 0 as *mut *mut i8, (*wsp).ws_closure);
    }
    if rc == 2 as i32 {
        return _wsplt_nomem(wsp)
    } else if rc != 0 {
        if rc == 9 as i32 {
            if (*wsp).ws_errno == 9 as i32 {
                rpl_free((*wsp).ws_usererr as *mut libc::c_void);
            }
            (*wsp).ws_usererr = value;
        }
        _wsplt_seterr(wsp, rc);
        return 1 as i32;
    }
    if !value.is_null() {
        if flg & 0x4 as i32 as u32 != 0 {
            if wsnode_new(wsp, &mut newnode) != 0 {
                return 1 as i32;
            }
            wsnode_insert(wsp, newnode, *ptail, 0 as i32);
            *ptail = newnode;
            (*newnode).flags = (0x2 as i32 | 0x8 as i32) as u32 | flg;
            (*newnode).v.word = value;
        } else if *value as i32 == 0 as i32 {
            rpl_free(value as *mut libc::c_void);
            if wsnode_new(wsp, &mut newnode) != 0 {
                return 1 as i32;
            }
            wsnode_insert(wsp, newnode, *ptail, 0 as i32);
            *ptail = newnode;
            (*newnode).flags = 0x1 as i32 as u32;
        } else {
            let mut ws_0: wordsplit = wordsplit {
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
            let mut rc_0: i32 = 0;
            rc_0 = _wsplt_subsplit(
                wsp,
                &mut ws_0,
                value,
                strlen(value) as i32,
                (0x40 as i32 | 0x4 as i32 | 0x100 as i32 | (0x200 as i32 | 0x400 as i32)
                    | (if (*wsp).ws_flags & 0x1000 as i32 as u32 != 0
                        || (*wsp).ws_options & 0x80 as i32 as u32 != 0
                    {
                        0x1000 as i32
                    } else {
                        0 as i32
                    })) as u32,
                0 as i32,
            );
            rpl_free(value as *mut libc::c_void);
            if rc_0 != 0 {
                _wsplt_seterr_sub(wsp, &mut ws_0);
                wordsplit_free(&mut ws_0);
                return 1 as i32;
            }
            wsnode_insert(wsp, ws_0.ws_head, *ptail, 0 as i32);
            *ptail = ws_0.ws_tail;
            ws_0.ws_tail = 0 as *mut wordsplit_node;
            ws_0.ws_head = ws_0.ws_tail;
            wordsplit_free(&mut ws_0);
        }
    } else {
        if wsnode_new(wsp, &mut newnode) != 0 {
            return 1 as i32;
        }
        wsnode_insert(wsp, newnode, *ptail, 0 as i32);
        *ptail = newnode;
        (*newnode).flags = 0x1 as i32 as u32;
    }
    return 0 as i32;
}
unsafe extern "C" fn wordsplit_cmdexp(mut wsp: *mut wordsplit) -> i32 {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut next: *mut wordsplit_node = (*p).next;
        if (*p).flags & 0x8 as i32 as u32 == 0 {
            if node_expand(
                wsp,
                p,
                Some(begin_cmd_p as unsafe extern "C" fn(i32) -> i32),
                Some(
                    expcmd
                        as unsafe extern "C" fn(
                            *mut wordsplit,
                            *const i8,
                            size_t,
                            *mut *mut wordsplit_node,
                            *mut *const i8,
                            u32,
                        ) -> i32,
                ),
            ) != 0
            {
                return 1 as i32;
            }
        }
        p = next;
    }
    wsnode_nullelim(wsp);
    return 0 as i32;
}
unsafe extern "C" fn wordsplit_trimws(mut wsp: *mut wordsplit) -> i32 {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut n: size_t = 0;
        if (*p).flags & 0x4 as i32 as u32 == 0 {
            n = (*p).v.segm.beg;
            while n < (*p).v.segm.end
                && (*((*wsp).ws_input).offset(n as isize) as i32 == ' ' as i32
                    || *((*wsp).ws_input).offset(n as isize) as i32 == '\t' as i32
                    || *((*wsp).ws_input).offset(n as isize) as i32 == '\n' as i32)
            {
                n = n.wrapping_add(1);
                n;
            }
            (*p).v.segm.beg = n;
        }
        while !((*p).next).is_null() && (*p).flags & 0x10 as i32 as u32 != 0 {
            p = (*p).next;
        }
        if !((*p).flags & 0x4 as i32 as u32 != 0) {
            n = (*p).v.segm.end;
            while n > (*p).v.segm.beg
                && (*((*wsp).ws_input).offset(n.wrapping_sub(1 as i32 as u64) as isize)
                    as i32 == ' ' as i32
                    || *((*wsp).ws_input)
                        .offset(n.wrapping_sub(1 as i32 as u64) as isize) as i32
                        == '\t' as i32
                    || *((*wsp).ws_input)
                        .offset(n.wrapping_sub(1 as i32 as u64) as isize) as i32
                        == '\n' as i32)
            {
                n = n.wrapping_sub(1);
                n;
            }
            (*p).v.segm.end = n;
            if (*p).v.segm.beg == (*p).v.segm.end {
                (*p).flags |= 0x1 as i32 as u32;
            }
        }
        p = (*p).next;
    }
    wsnode_nullelim(wsp);
    return 0 as i32;
}
unsafe extern "C" fn wordsplit_tildexpand(mut wsp: *mut wordsplit) -> i32 {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut uname: *mut i8 = 0 as *mut i8;
    let mut usize: size_t = 0 as i32 as size_t;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut str: *const i8 = 0 as *const i8;
        if !((*p).flags & 0x4 as i32 as u32 != 0) {
            str = wsnode_ptr(wsp, p);
            if *str.offset(0 as i32 as isize) as i32 == '~' as i32 {
                let mut i: size_t = 0;
                let mut size: size_t = 0;
                let mut dlen: size_t = 0;
                let mut slen: size_t = wsnode_len(p);
                let mut pw: *mut passwd = 0 as *mut passwd;
                let mut newstr: *mut i8 = 0 as *mut i8;
                i = 1 as i32 as size_t;
                while i < slen && *str.offset(i as isize) as i32 != '/' as i32 {
                    i = i.wrapping_add(1);
                    i;
                }
                if !(i == slen) {
                    if i > 1 as i32 as u64 {
                        if i > usize {
                            let mut p_0: *mut i8 = realloc(uname as *mut libc::c_void, i)
                                as *mut i8;
                            if p_0.is_null() {
                                rpl_free(uname as *mut libc::c_void);
                                return _wsplt_nomem(wsp);
                            }
                            uname = p_0;
                            usize = i;
                        }
                        i = i.wrapping_sub(1);
                        i;
                        memcpy(
                            uname as *mut libc::c_void,
                            str.offset(1 as i32 as isize) as *const libc::c_void,
                            i,
                        );
                        *uname.offset(i as isize) = 0 as i32 as i8;
                        pw = getpwnam(uname);
                    } else {
                        pw = getpwuid(getuid());
                    }
                    if !pw.is_null() {
                        dlen = strlen((*pw).pw_dir);
                        size = slen.wrapping_sub(i).wrapping_add(dlen);
                        newstr = malloc(size) as *mut i8;
                        if newstr.is_null() {
                            rpl_free(uname as *mut libc::c_void);
                            return _wsplt_nomem(wsp);
                        }
                        size = size.wrapping_sub(1);
                        size;
                        memcpy(
                            newstr as *mut libc::c_void,
                            (*pw).pw_dir as *const libc::c_void,
                            dlen,
                        );
                        memcpy(
                            newstr.offset(dlen as isize) as *mut libc::c_void,
                            str.offset(i as isize).offset(1 as i32 as isize)
                                as *const libc::c_void,
                            slen.wrapping_sub(i).wrapping_sub(1 as i32 as u64),
                        );
                        *newstr.offset(size as isize) = 0 as i32 as i8;
                        if (*p).flags & 0x2 as i32 as u32 != 0 {
                            rpl_free((*p).v.word as *mut libc::c_void);
                        }
                        (*p).v.word = newstr;
                        (*p).flags |= 0x2 as i32 as u32;
                    }
                }
            }
        }
        p = (*p).next;
    }
    rpl_free(uname as *mut libc::c_void);
    return 0 as i32;
}
unsafe extern "C" fn isglob(mut s: *const i8, mut l: i32) -> i32 {
    loop {
        let fresh31 = l;
        l = l - 1;
        if !(fresh31 != 0) {
            break;
        }
        let fresh32 = s;
        s = s.offset(1);
        if !(strchr(b"*?[\0" as *const u8 as *const i8, *fresh32 as i32)).is_null() {
            return 1 as i32;
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn wordsplit_pathexpand(mut wsp: *mut wordsplit) -> i32 {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut next: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut slen: size_t = 0;
    let mut flags: i32 = 0 as i32;
    if (*wsp).ws_options & 0x4 as i32 as u32 != 0 {
        flags = (1 as i32) << 7 as i32;
    }
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut str: *const i8 = 0 as *const i8;
        next = (*p).next;
        if !((*p).flags & 0x4 as i32 as u32 != 0) {
            str = wsnode_ptr(wsp, p);
            slen = wsnode_len(p);
            if isglob(str, slen as i32) != 0 {
                let mut i: i32 = 0;
                let mut g: glob_t = glob_t {
                    gl_pathc: 0,
                    gl_pathv: 0 as *mut *mut i8,
                    gl_offs: 0,
                    gl_flags: 0,
                    gl_closedir: None,
                    gl_readdir: None,
                    gl_opendir: None,
                    gl_lstat: None,
                    gl_stat: None,
                };
                let mut prev: *mut wordsplit_node = 0 as *mut wordsplit_node;
                let mut pattern: *mut i8 = 0 as *mut i8;
                pattern = malloc(slen.wrapping_add(1 as i32 as u64)) as *mut i8;
                if pattern.is_null() {
                    return _wsplt_nomem(wsp);
                }
                memcpy(pattern as *mut libc::c_void, str as *const libc::c_void, slen);
                *pattern.offset(slen as isize) = 0 as i32 as i8;
                match glob(pattern, flags, None, &mut g) {
                    0 => {
                        rpl_free(pattern as *mut libc::c_void);
                        prev = p;
                        i = 0 as i32;
                        while (i as u64) < g.gl_pathc {
                            let mut newnode: *mut wordsplit_node = 0
                                as *mut wordsplit_node;
                            let mut newstr: *mut i8 = 0 as *mut i8;
                            if wsnode_new(wsp, &mut newnode) != 0 {
                                return 1 as i32;
                            }
                            newstr = strdup(*(g.gl_pathv).offset(i as isize));
                            if newstr.is_null() {
                                wsnode_free(newnode);
                                return _wsplt_nomem(wsp);
                            }
                            (*newnode).v.word = newstr;
                            (*newnode).flags |= (0x2 as i32 | 0x4 as i32) as u32;
                            wsnode_insert(wsp, newnode, prev, 0 as i32);
                            prev = newnode;
                            i += 1;
                            i;
                        }
                        globfree(&mut g);
                        wsnode_remove(wsp, p);
                        wsnode_free(p);
                    }
                    1 => {
                        rpl_free(pattern as *mut libc::c_void);
                        return _wsplt_nomem(wsp);
                    }
                    3 => {
                        if (*wsp).ws_options & 0x1 as i32 as u32 != 0 {
                            wsnode_remove(wsp, p);
                            wsnode_free(p);
                        } else if (*wsp).ws_options & 0x2 as i32 as u32 != 0 {
                            let mut buf: [i8; 128] = [0; 128];
                            if (*wsp).ws_errno == 9 as i32 {
                                rpl_free((*wsp).ws_usererr as *mut libc::c_void);
                            }
                            snprintf(
                                buf.as_mut_ptr(),
                                ::core::mem::size_of::<[i8; 128]>() as u64,
                                dcgettext(
                                    0 as *const i8,
                                    b"no files match pattern %s\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                pattern,
                            );
                            rpl_free(pattern as *mut libc::c_void);
                            (*wsp).ws_usererr = strdup(buf.as_mut_ptr());
                            if ((*wsp).ws_usererr).is_null() {
                                return _wsplt_nomem(wsp)
                            } else {
                                return _wsplt_seterr(wsp, 9 as i32)
                            }
                        }
                        rpl_free(pattern as *mut libc::c_void);
                    }
                    _ => {
                        rpl_free(pattern as *mut libc::c_void);
                        return _wsplt_seterr(wsp, 8 as i32);
                    }
                }
            }
        }
        p = next;
    }
    return 0 as i32;
}
unsafe extern "C" fn skip_sed_expr(
    mut command: *const i8,
    mut i: size_t,
    mut len: size_t,
) -> i32 {
    let mut state: i32 = 0;
    loop {
        let mut delim: i32 = 0;
        if *command.offset(i as isize) as i32 == ';' as i32 {
            i = i.wrapping_add(1);
            i;
        }
        if !(*command.offset(i as isize) as i32 == 's' as i32
            && i.wrapping_add(3 as i32 as u64) < len
            && !(strchr(
                b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~\0" as *const u8 as *const i8,
                *command.offset(i.wrapping_add(1 as i32 as u64) as isize) as i32,
            ))
                .is_null())
        {
            break;
        }
        i = i.wrapping_add(1);
        delim = *command.offset(i as isize) as i32;
        state = 1 as i32;
        i = i.wrapping_add(1);
        i;
        while i < len {
            if state == 3 as i32 {
                if *command.offset(i as isize) as i32 == delim
                    || !('A' as i32 as u32 <= *command.offset(i as isize) as u32
                        && *command.offset(i as isize) as u32 <= 'Z' as i32 as u32
                        || 'a' as i32 as u32 <= *command.offset(i as isize) as u32
                            && *command.offset(i as isize) as u32 <= 'z' as i32 as u32
                        || '0' as i32 as u32 <= *command.offset(i as isize) as u32
                            && *command.offset(i as isize) as u32 <= '9' as i32 as u32)
                {
                    break;
                }
            } else if *command.offset(i as isize) as i32 == '\\' as i32 {
                i = i.wrapping_add(1);
                i;
            } else if *command.offset(i as isize) as i32 == delim {
                state += 1;
                state;
            }
            i = i.wrapping_add(1);
            i;
        }
        if !(state == 3 as i32 && i < len
            && *command.offset(i as isize) as i32 == ';' as i32)
        {
            break;
        }
    }
    return i as i32;
}
#[inline]
unsafe extern "C" fn skip_delim_internal(
    mut wsp: *mut wordsplit,
    mut return_delims: i32,
) -> size_t {
    return if return_delims != 0 {
        (*wsp).ws_endp
    } else {
        ((*wsp).ws_endp).wrapping_add(1 as i32 as u64)
    };
}
#[inline]
unsafe extern "C" fn skip_delim(mut wsp: *mut wordsplit) -> size_t {
    return skip_delim_internal(
        wsp,
        ((*wsp).ws_flags & 0x1000 as i32 as u32 != 0
            || (*wsp).ws_options & 0x80 as i32 as u32 != 0) as i32,
    );
}
#[inline]
unsafe extern "C" fn skip_delim_real(mut wsp: *mut wordsplit) -> size_t {
    return skip_delim_internal(wsp, ((*wsp).ws_flags & 0x1000 as i32 as u32) as i32);
}
unsafe extern "C" fn scan_qstring(
    mut wsp: *mut wordsplit,
    mut start: size_t,
    mut end: *mut size_t,
) -> i32 {
    let mut j: size_t = 0;
    let mut command: *const i8 = (*wsp).ws_input;
    let mut len: size_t = (*wsp).ws_len;
    let mut q: i8 = *command.offset(start as isize);
    j = start.wrapping_add(1 as i32 as u64);
    while j < len && *command.offset(j as isize) as i32 != q as i32 {
        if q as i32 == '"' as i32 && *command.offset(j as isize) as i32 == '\\' as i32 {
            j = j.wrapping_add(1);
            j;
        }
        j = j.wrapping_add(1);
        j;
    }
    if j < len && *command.offset(j as isize) as i32 == q as i32 {
        let mut flags: u32 = (0x4 as i32 | 0x100 as i32) as u32;
        if q as i32 == '\'' as i32 {
            flags |= 0x8 as i32 as u32;
        }
        if wordsplit_add_segm(wsp, start.wrapping_add(1 as i32 as u64), j, flags as i32)
            != 0
        {
            return 2 as i32;
        }
        *end = j;
    } else {
        (*wsp).ws_endp = start;
        _wsplt_seterr(wsp, 1 as i32);
        return 2 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn scan_word(
    mut wsp: *mut wordsplit,
    mut start: size_t,
    mut consume_all: i32,
) -> i32 {
    let mut len: size_t = (*wsp).ws_len;
    let mut command: *const i8 = (*wsp).ws_input;
    let mut comment: *const i8 = (*wsp).ws_comment;
    let mut join: i32 = 0 as i32;
    let mut flags: u32 = 0 as i32 as u32;
    let mut np: *mut wordsplit_node = (*wsp).ws_tail;
    let mut i: size_t = start;
    if i >= len {
        (*wsp).ws_errno = 0 as i32;
        return 0 as i32;
    }
    start = i;
    if (*wsp).ws_flags & 0x2000 as i32 as u32 != 0
        && *command.offset(i as isize) as i32 == 's' as i32
        && i.wrapping_add(3 as i32 as u64) < len
        && !(strchr(
            b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~\0" as *const u8 as *const i8,
            *command.offset(i.wrapping_add(1 as i32 as u64) as isize) as i32,
        ))
            .is_null()
    {
        flags = 0x20 as i32 as u32;
        i = skip_sed_expr(command, i, len) as size_t;
    } else if consume_all != 0
        || (strchr((*wsp).ws_delim, *command.offset(i as isize) as i32)).is_null()
    {
        while i < len {
            if !comment.is_null()
                && !(strchr(comment, *command.offset(i as isize) as i32)).is_null()
            {
                let mut j: size_t = 0;
                j = i.wrapping_add(1 as i32 as u64);
                while j < len && *command.offset(j as isize) as i32 != '\n' as i32 {
                    j = j.wrapping_add(1);
                    j;
                }
                if wordsplit_add_segm(wsp, start, i, 0 as i32) != 0 {
                    return 2 as i32;
                }
                (*wsp).ws_endp = j;
                return 1 as i32;
            }
            if (*wsp).ws_flags & (0x200 as i32 | 0x400 as i32) as u32 != 0 {
                if *command.offset(i as isize) as i32 == '\\' as i32 {
                    i = i.wrapping_add(1);
                    if i == len {
                        break;
                    }
                    i = i.wrapping_add(1);
                    i;
                    continue;
                } else if (*wsp).ws_flags & 0x200 as i32 as u32 != 0
                    && *command.offset(i as isize) as i32 == '\'' as i32
                    || (*wsp).ws_flags & 0x400 as i32 as u32 != 0
                        && *command.offset(i as isize) as i32 == '"' as i32
                {
                    if join != 0 && !((*wsp).ws_tail).is_null() {
                        (*(*wsp).ws_tail).flags |= 0x10 as i32 as u32;
                    }
                    if wordsplit_add_segm(wsp, start, i, 0x10 as i32) != 0 {
                        return 2 as i32;
                    }
                    if scan_qstring(wsp, i, &mut i) != 0 {
                        return 2 as i32;
                    }
                    start = i.wrapping_add(1 as i32 as u64);
                    join = 1 as i32;
                }
            }
            if *command.offset(i as isize) as i32 == '$' as i32 {
                if (*wsp).ws_flags & 0x40 as i32 as u32 == 0
                    && *command.offset(i.wrapping_add(1 as i32 as u64) as isize) as i32
                        == '{' as i32
                    && find_closing_paren(
                        command,
                        i.wrapping_add(2 as i32 as u64),
                        len,
                        &mut i,
                        b"{}\0" as *const u8 as *const i8,
                    ) == 0 as i32
                {
                    continue;
                }
                if (*wsp).ws_flags & 0x4 as i32 as u32 == 0
                    && *command.offset(i.wrapping_add(1 as i32 as u64) as isize) as i32
                        == '(' as i32
                    && find_closing_paren(
                        command,
                        i.wrapping_add(2 as i32 as u64),
                        len,
                        &mut i,
                        b"()\0" as *const u8 as *const i8,
                    ) == 0 as i32
                {
                    continue;
                }
            }
            if consume_all == 0
                && !(strchr((*wsp).ws_delim, *command.offset(i as isize) as i32))
                    .is_null()
            {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if (*wsp).ws_flags & 0x1000 as i32 as u32 != 0
        || (*wsp).ws_options & 0x80 as i32 as u32 != 0
    {
        i = i.wrapping_add(1);
        i;
        flags |= 0x40 as i32 as u32;
    } else if (*wsp).ws_flags & 0x800 as i32 as u32 == 0 {
        flags |= 0x100 as i32 as u32;
    }
    if join != 0 && i > start && !((*wsp).ws_tail).is_null() {
        (*(*wsp).ws_tail).flags |= 0x10 as i32 as u32;
    }
    if wordsplit_add_segm(wsp, start, i, flags as i32) != 0 {
        return 2 as i32;
    }
    (*wsp).ws_endp = i;
    if (*wsp).ws_flags & 0x20000000 as i32 as u32 != 0 {
        return 0 as i32;
    }
    if consume_all != 0 {
        if np.is_null() {
            np = (*wsp).ws_head;
        }
        while !np.is_null() {
            (*np).flags |= 0x4 as i32 as u32;
            np = (*np).next;
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn xtonum(
    mut pval: *mut i32,
    mut src: *const i8,
    mut base: i32,
    mut cnt: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut val: i32 = 0;
    i = 0 as i32;
    val = 0 as i32;
    while i < cnt {
        let mut n: i32 = *(src as *mut u8) as i32;
        if n > 127 as i32
            || {
                n = (if '0' as i32 as u32 <= n as u32 && n as u32 <= '9' as i32 as u32 {
                    n - '0' as i32
                } else {
                    (if !(strchr(b"abcdefABCDEF\0" as *const u8 as *const i8, n))
                        .is_null()
                    {
                        ({
                            let mut __res: i32 = 0;
                            if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                                if 0 != 0 {
                                    let mut __c: i32 = n;
                                    __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                        __c
                                    } else {
                                        *(*__ctype_toupper_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = toupper(n);
                                }
                            } else {
                                __res = *(*__ctype_toupper_loc()).offset(n as isize);
                            }
                            __res
                        }) - 'A' as i32 + 10 as i32
                    } else {
                        255 as i32
                    })
                });
                n >= base
            }
        {
            break;
        }
        val = val * base + n;
        i += 1;
        i;
        src = src.offset(1);
        src;
    }
    *pval = val;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit_c_quoted_length(
    mut str: *const i8,
    mut quote_hex: i32,
    mut quote: *mut i32,
) -> size_t {
    let mut len: size_t = 0 as i32 as size_t;
    *quote = 0 as i32;
    while *str != 0 {
        if !(strchr(b" \"\0" as *const u8 as *const i8, *str as i32)).is_null() {
            *quote = 1 as i32;
        }
        if *str as i32 == ' ' as i32 {
            len = len.wrapping_add(1);
            len;
        } else if *str as i32 == '"' as i32 {
            len = (len as u64).wrapping_add(2 as i32 as u64) as size_t as size_t;
        } else if *str as i32 != '\t' as i32 && *str as i32 != '\\' as i32
            && (' ' as i32 as u32 <= *str as u32 && *str as u32 <= 127 as i32 as u32)
        {
            len = len.wrapping_add(1);
            len;
        } else if quote_hex != 0 {
            len = (len as u64).wrapping_add(3 as i32 as u64) as size_t as size_t;
        } else if wordsplit_c_quote_char(*str as i32) != 0 {
            len = (len as u64).wrapping_add(2 as i32 as u64) as size_t as size_t;
        } else {
            len = (len as u64).wrapping_add(4 as i32 as u64) as size_t as size_t;
        }
        str = str.offset(1);
        str;
    }
    return len;
}
unsafe extern "C" fn wsplt_unquote_char(mut transtab: *const i8, mut c: i32) -> i32 {
    while *transtab as i32 != 0 && *transtab.offset(1 as i32 as isize) as i32 != 0 {
        let fresh33 = transtab;
        transtab = transtab.offset(1);
        if *fresh33 as i32 == c {
            return *transtab as i32;
        }
        transtab = transtab.offset(1);
        transtab;
    }
    return 0 as i32;
}
unsafe extern "C" fn wsplt_quote_char(mut transtab: *const i8, mut c: i32) -> i32 {
    while *transtab as i32 != 0 && *transtab.offset(1 as i32 as isize) as i32 != 0 {
        if *transtab.offset(1 as i32 as isize) as i32 == c {
            return *transtab as i32;
        }
        transtab = transtab.offset(2 as i32 as isize);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit_c_unquote_char(mut c: i32) -> i32 {
    return wsplt_unquote_char(wordsplit_c_escape_tab.as_mut_ptr(), c);
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit_c_quote_char(mut c: i32) -> i32 {
    return wsplt_quote_char(wordsplit_c_escape_tab.as_mut_ptr(), c);
}
unsafe extern "C" fn wordsplit_string_unquote_copy(
    mut ws: *mut wordsplit,
    mut inquote: i32,
    mut dst: *mut i8,
    mut src: *const i8,
    mut n: size_t,
) {
    let mut i: i32 = 0 as i32;
    let mut c: i32 = 0;
    inquote = (inquote != 0) as i32;
    while (i as u64) < n {
        if *src.offset(i as isize) as i32 == '\\' as i32 {
            i += 1;
            i;
            if (*ws).ws_options & ((0x40 as i32) << 4 as i32 * inquote) as u32 != 0
                && (*src.offset(i as isize) as i32 == 'x' as i32
                    || *src.offset(i as isize) as i32 == 'X' as i32)
            {
                if n.wrapping_sub(i as u64) < 2 as i32 as u64 {
                    let fresh34 = dst;
                    dst = dst.offset(1);
                    *fresh34 = '\\' as i32 as i8;
                    let fresh35 = i;
                    i = i + 1;
                    let fresh36 = dst;
                    dst = dst.offset(1);
                    *fresh36 = *src.offset(fresh35 as isize);
                } else {
                    let mut off: i32 = xtonum(
                        &mut c,
                        src.offset(i as isize).offset(1 as i32 as isize),
                        16 as i32,
                        2 as i32,
                    );
                    if off == 0 as i32 {
                        let fresh37 = dst;
                        dst = dst.offset(1);
                        *fresh37 = '\\' as i32 as i8;
                        let fresh38 = i;
                        i = i + 1;
                        let fresh39 = dst;
                        dst = dst.offset(1);
                        *fresh39 = *src.offset(fresh38 as isize);
                    } else {
                        let fresh40 = dst;
                        dst = dst.offset(1);
                        *fresh40 = c as i8;
                        i += off + 1 as i32;
                    }
                }
            } else if (*ws).ws_options & ((0x20 as i32) << 4 as i32 * inquote) as u32
                != 0 && (*src.offset(i as isize) as u8 as i32) < 128 as i32
                && ('0' as i32 as u32 <= *src.offset(i as isize) as u32
                    && *src.offset(i as isize) as u32 <= '9' as i32 as u32)
            {
                if n.wrapping_sub(i as u64) < 1 as i32 as u64 {
                    let fresh41 = dst;
                    dst = dst.offset(1);
                    *fresh41 = '\\' as i32 as i8;
                    let fresh42 = i;
                    i = i + 1;
                    let fresh43 = dst;
                    dst = dst.offset(1);
                    *fresh43 = *src.offset(fresh42 as isize);
                } else {
                    let mut off_0: i32 = xtonum(
                        &mut c,
                        src.offset(i as isize),
                        8 as i32,
                        3 as i32,
                    );
                    if off_0 == 0 as i32 {
                        let fresh44 = dst;
                        dst = dst.offset(1);
                        *fresh44 = '\\' as i32 as i8;
                        let fresh45 = i;
                        i = i + 1;
                        let fresh46 = dst;
                        dst = dst.offset(1);
                        *fresh46 = *src.offset(fresh45 as isize);
                    } else {
                        let fresh47 = dst;
                        dst = dst.offset(1);
                        *fresh47 = c as i8;
                        i += off_0;
                    }
                }
            } else {
                c = wsplt_unquote_char(
                    (*ws).ws_escape[inquote as usize],
                    *src.offset(i as isize) as i32,
                );
                if c != 0 {
                    let fresh48 = dst;
                    dst = dst.offset(1);
                    *fresh48 = c as i8;
                    i += 1;
                    i;
                } else {
                    if (*ws).ws_options & ((0x10 as i32) << 4 as i32 * inquote) as u32
                        != 0
                    {
                        let fresh49 = dst;
                        dst = dst.offset(1);
                        *fresh49 = '\\' as i32 as i8;
                    }
                    let fresh50 = i;
                    i = i + 1;
                    let fresh51 = dst;
                    dst = dst.offset(1);
                    *fresh51 = *src.offset(fresh50 as isize);
                }
            }
        } else {
            let fresh52 = i;
            i = i + 1;
            let fresh53 = dst;
            dst = dst.offset(1);
            *fresh53 = *src.offset(fresh52 as isize);
        }
    }
    *dst = 0 as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit_c_quote_copy(
    mut dst: *mut i8,
    mut src: *const i8,
    mut quote_hex: i32,
) {
    while *src != 0 {
        if *src as i32 == '"' as i32 {
            let fresh54 = dst;
            dst = dst.offset(1);
            *fresh54 = '\\' as i32 as i8;
            let fresh55 = dst;
            dst = dst.offset(1);
            *fresh55 = *src;
        } else if *src as i32 != '\t' as i32 && *src as i32 != '\\' as i32
            && (' ' as i32 as u32 <= *src as u32 && *src as u32 <= 127 as i32 as u32)
        {
            let fresh56 = dst;
            dst = dst.offset(1);
            *fresh56 = *src;
        } else {
            let mut tmp: [i8; 4] = [0; 4];
            if quote_hex != 0 {
                snprintf(
                    tmp.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 4]>() as u64,
                    b"%%%02X\0" as *const u8 as *const i8,
                    *(src as *mut u8) as i32,
                );
                memcpy(
                    dst as *mut libc::c_void,
                    tmp.as_mut_ptr() as *const libc::c_void,
                    3 as i32 as u64,
                );
                dst = dst.offset(3 as i32 as isize);
            } else {
                let mut c: i32 = wordsplit_c_quote_char(*src as i32);
                let fresh57 = dst;
                dst = dst.offset(1);
                *fresh57 = '\\' as i32 as i8;
                if c != 0 {
                    let fresh58 = dst;
                    dst = dst.offset(1);
                    *fresh58 = c as i8;
                } else {
                    snprintf(
                        tmp.as_mut_ptr(),
                        ::core::mem::size_of::<[i8; 4]>() as u64,
                        b"%03o\0" as *const u8 as *const i8,
                        *(src as *mut u8) as i32,
                    );
                    memcpy(
                        dst as *mut libc::c_void,
                        tmp.as_mut_ptr() as *const libc::c_void,
                        3 as i32 as u64,
                    );
                    dst = dst.offset(3 as i32 as isize);
                }
            }
        }
        src = src.offset(1);
        src;
    }
}
static mut exptab: [exptab; 9] = unsafe {
    [
        {
            let mut init = exptab {
                descr: b"WS trimming\0" as *const u8 as *const i8,
                flag: 0x100 as i32,
                opt: 0 as i32,
                expansion: Some(
                    wordsplit_trimws as unsafe extern "C" fn(*mut wordsplit) -> i32,
                ),
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"command substitution\0" as *const u8 as *const i8,
                flag: 0x4 as i32,
                opt: 0x1 as i32 | 0x4 as i32,
                expansion: Some(
                    wordsplit_cmdexp as unsafe extern "C" fn(*mut wordsplit) -> i32,
                ),
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"coalesce list\0" as *const u8 as *const i8,
                flag: 0 as i32,
                opt: 0x1 as i32 | 0x4 as i32,
                expansion: None,
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"tilde expansion\0" as *const u8 as *const i8,
                flag: 0x40000000 as i32,
                opt: 0 as i32,
                expansion: Some(
                    wordsplit_tildexpand as unsafe extern "C" fn(*mut wordsplit) -> i32,
                ),
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"variable expansion\0" as *const u8 as *const i8,
                flag: 0x40 as i32,
                opt: 0x1 as i32,
                expansion: Some(
                    wordsplit_varexp as unsafe extern "C" fn(*mut wordsplit) -> i32,
                ),
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"quote removal\0" as *const u8 as *const i8,
                flag: 0 as i32,
                opt: 0x1 as i32,
                expansion: Some(
                    wsnode_quoteremoval as unsafe extern "C" fn(*mut wordsplit) -> i32,
                ),
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"coalesce list\0" as *const u8 as *const i8,
                flag: 0 as i32,
                opt: 0x1 as i32 | 0x4 as i32,
                expansion: None,
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"path expansion\0" as *const u8 as *const i8,
                flag: 0x40000000 as i32,
                opt: 0 as i32,
                expansion: Some(
                    wordsplit_pathexpand as unsafe extern "C" fn(*mut wordsplit) -> i32,
                ),
            };
            init
        },
        {
            let mut init = exptab {
                descr: 0 as *const i8,
                flag: 0,
                opt: 0,
                expansion: None,
            };
            init
        },
    ]
};
#[inline]
unsafe extern "C" fn exptab_matches(mut p: *mut exptab, mut wsp: *mut wordsplit) -> i32 {
    let mut result: i32 = 0;
    result = ((*wsp).ws_flags & (*p).flag as u32) as i32;
    if (*p).opt & 0x2 as i32 != 0 {
        result = (result == (*p).flag) as i32;
    }
    if (*p).opt & 0x1 as i32 != 0 {
        result = (result == 0) as i32;
    }
    return result;
}
unsafe extern "C" fn wordsplit_process_list(
    mut wsp: *mut wordsplit,
    mut start: size_t,
) -> i32 {
    let mut p: *mut exptab = 0 as *mut exptab;
    if (*wsp).ws_flags & 0x200000 as i32 as u32 != 0 {
        ((*wsp).ws_debug)
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"(%02d) Input:%.*s;\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*wsp).ws_lvl,
            (*wsp).ws_len as i32,
            (*wsp).ws_input,
        );
    }
    if (*wsp).ws_flags & 0x400000 as i32 as u32 != 0
        || (*wsp).ws_options & 0x80 as i32 as u32 != 0
            && ((*wsp).ws_wordi).wrapping_add(1 as i32 as u64) == (*wsp).ws_maxwords
    {
        if scan_word(wsp, start, 1 as i32) == 2 as i32 {
            return (*wsp).ws_errno;
        }
    } else {
        let mut rc: i32 = 0;
        loop {
            rc = scan_word(wsp, start, 0 as i32);
            if !(rc == 1 as i32) {
                break;
            }
            start = skip_delim(wsp);
        }
        if !((*wsp).ws_tail).is_null() {
            (*(*wsp).ws_tail).flags &= !(0x10 as i32) as u32;
        }
        if rc == 2 as i32 {
            return (*wsp).ws_errno;
        }
    }
    if (*wsp).ws_flags & 0x200000 as i32 as u32 != 0 {
        ((*wsp).ws_debug)
            .expect(
                "non-null function pointer",
            )(
            b"(%02d) %s\0" as *const u8 as *const i8,
            (*wsp).ws_lvl,
            dcgettext(
                0 as *const i8,
                b"Initial list:\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        wordsplit_dump_nodes(wsp);
    }
    p = exptab.as_mut_ptr();
    while !((*p).descr).is_null() {
        if exptab_matches(p, wsp) != 0 {
            if (*p).opt & 0x4 as i32 != 0 {
                if wsnode_coalesce(wsp) != 0 {
                    break;
                }
                if (*wsp).ws_flags & 0x200000 as i32 as u32 != 0 {
                    ((*wsp).ws_debug)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"(%02d) %s\0" as *const u8 as *const i8,
                        (*wsp).ws_lvl,
                        dcgettext(
                            0 as *const i8,
                            b"Coalesced list:\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    wordsplit_dump_nodes(wsp);
                }
            }
            if ((*p).expansion).is_some() {
                if ((*p).expansion).expect("non-null function pointer")(wsp) != 0 {
                    break;
                }
                if (*wsp).ws_flags & 0x200000 as i32 as u32 != 0 {
                    ((*wsp).ws_debug)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"(%02d) %s\0" as *const u8 as *const i8,
                        (*wsp).ws_lvl,
                        dcgettext(0 as *const i8, (*p).descr, 5 as i32),
                    );
                    wordsplit_dump_nodes(wsp);
                }
            }
        }
        p = p.offset(1);
        p;
    }
    return (*wsp).ws_errno;
}
unsafe extern "C" fn wordsplit_run(
    mut command: *const i8,
    mut length: size_t,
    mut wsp: *mut wordsplit,
    mut flags: u32,
    mut lvl: i32,
) -> i32 {
    let mut rc: i32 = 0;
    let mut start: size_t = 0;
    if command.is_null() {
        if flags & 0x20000000 as i32 as u32 == 0 {
            return _wsplt_seterr(wsp, 3 as i32);
        }
        if !((*wsp).ws_head).is_null() {
            return wordsplit_finish(wsp);
        }
        start = skip_delim_real(wsp);
        if (*wsp).ws_endp == (*wsp).ws_len {
            return _wsplt_seterr(wsp, 6 as i32);
        }
        (*wsp).ws_flags |= 0x8 as i32 as u32;
        wordsplit_init0(wsp);
    } else {
        start = 0 as i32 as size_t;
        rc = wordsplit_init(wsp, command, length, flags);
        if rc != 0 {
            return rc;
        }
        (*wsp).ws_lvl = lvl;
    }
    rc = wordsplit_process_list(wsp, start);
    if rc != 0 {
        return rc;
    }
    return wordsplit_finish(wsp);
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit_len(
    mut command: *const i8,
    mut length: size_t,
    mut wsp: *mut wordsplit,
    mut flags: u32,
) -> i32 {
    return wordsplit_run(command, length, wsp, flags, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit(
    mut command: *const i8,
    mut ws: *mut wordsplit,
    mut flags: u32,
) -> i32 {
    return wordsplit_len(
        command,
        if !command.is_null() { strlen(command) } else { 0 as i32 as u64 },
        ws,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit_free_words(mut ws: *mut wordsplit) {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < (*ws).ws_wordc {
        let mut p: *mut i8 = *((*ws).ws_wordv)
            .offset(((*ws).ws_offs).wrapping_add(i) as isize);
        if !p.is_null() {
            rpl_free(p as *mut libc::c_void);
            let ref mut fresh59 = *((*ws).ws_wordv)
                .offset(((*ws).ws_offs).wrapping_add(i) as isize);
            *fresh59 = 0 as *mut i8;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*ws).ws_wordc = 0 as i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit_free_envbuf(mut ws: *mut wordsplit) {
    if (*ws).ws_flags & 0x4 as i32 as u32 != 0 {
        return;
    }
    if !((*ws).ws_envbuf).is_null() {
        let mut i: size_t = 0;
        i = 0 as i32 as size_t;
        while !(*((*ws).ws_envbuf).offset(i as isize)).is_null() {
            rpl_free(*((*ws).ws_envbuf).offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        rpl_free((*ws).ws_envbuf as *mut libc::c_void);
        (*ws).ws_envsiz = 0 as i32 as size_t;
        (*ws).ws_envidx = (*ws).ws_envsiz;
        (*ws).ws_envbuf = 0 as *mut *mut i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit_clearerr(mut ws: *mut wordsplit) {
    if (*ws).ws_errno == 9 as i32 {
        rpl_free((*ws).ws_usererr as *mut libc::c_void);
    }
    (*ws).ws_usererr = 0 as *mut i8;
    (*ws).ws_errno = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit_free(mut ws: *mut wordsplit) {
    wordsplit_free_nodes(ws);
    wordsplit_free_words(ws);
    rpl_free((*ws).ws_wordv as *mut libc::c_void);
    (*ws).ws_wordv = 0 as *mut *mut i8;
    wordsplit_free_envbuf(ws);
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit_get_words(
    mut ws: *mut wordsplit,
    mut wordc: *mut size_t,
    mut wordv: *mut *mut *mut i8,
) -> i32 {
    let mut p: *mut *mut i8 = realloc(
        (*ws).ws_wordv as *mut libc::c_void,
        ((*ws).ws_wordc)
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
    ) as *mut *mut i8;
    if p.is_null() {
        return -(1 as i32);
    }
    *wordv = p;
    *wordc = (*ws).ws_wordc;
    (*ws).ws_wordv = 0 as *mut *mut i8;
    (*ws).ws_wordc = 0 as i32 as size_t;
    (*ws).ws_wordn = 0 as i32 as size_t;
    return 0 as i32;
}
#[no_mangle]
pub static mut _wordsplit_errstr: [*const i8; 9] = [
    b"no error\0" as *const u8 as *const i8,
    b"missing closing quote\0" as *const u8 as *const i8,
    b"memory exhausted\0" as *const u8 as *const i8,
    b"invalid wordsplit usage\0" as *const u8 as *const i8,
    b"unbalanced curly brace\0" as *const u8 as *const i8,
    b"undefined variable\0" as *const u8 as *const i8,
    b"input exhausted\0" as *const u8 as *const i8,
    b"unbalanced parenthesis\0" as *const u8 as *const i8,
    b"globbing error\0" as *const u8 as *const i8,
];
#[no_mangle]
pub static mut _wordsplit_nerrs: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn wordsplit_strerror(mut ws: *mut wordsplit) -> *const i8 {
    if (*ws).ws_errno == 9 as i32 {
        return (*ws).ws_usererr;
    }
    if (*ws).ws_errno < _wordsplit_nerrs {
        return _wordsplit_errstr[(*ws).ws_errno as usize];
    }
    return b"unknown error\0" as *const u8 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn wordsplit_perror(mut wsp: *mut wordsplit) {
    match (*wsp).ws_errno {
        1 => {
            ((*wsp).ws_error)
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"missing closing %c (start near #%lu)\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                *((*wsp).ws_input).offset((*wsp).ws_endp as isize) as i32,
                (*wsp).ws_endp,
            );
        }
        _ => {
            ((*wsp).ws_error)
                .expect(
                    "non-null function pointer",
                )(b"%s\0" as *const u8 as *const i8, wordsplit_strerror(wsp));
        }
    };
}
unsafe extern "C" fn run_static_initializers() {
    _wordsplit_nerrs = (::core::mem::size_of::<[*const i8; 9]>() as u64)
        .wrapping_div(::core::mem::size_of::<*const i8>() as u64) as i32;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];