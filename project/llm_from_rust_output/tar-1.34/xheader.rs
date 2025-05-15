use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_ulong, c_void};
use std::ptr;
use std::mem;
use std::cmp;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{time_t, uid_t, gid_t, off_t, size_t, uintmax_t, intmax_t};

struct Timespec {
    tv_sec: time_t,
    tv_nsec: i64,
}

struct Stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: u32,
    st_uid: uid_t,
    st_gid: gid_t,
    __pad0: c_int,
    st_rdev: u64,
    st_size: off_t,
    st_blksize: i64,
    st_blocks: i64,
    st_atim: Timespec,
    st_mtim: Timespec,
    st_ctim: Timespec,
    __glibc_reserved: [i64; 3],
}

struct SpArray {
    offset: off_t,
    numbytes: off_t,
}

struct XattrArray {
    xkey: *mut c_char,
    xval_ptr: *mut c_char,
    xval_len: size_t,
}

struct Xheader {
    stk: *mut Obstack,
    size: size_t,
    buffer: *mut c_char,
    string_length: uintmax_t,
}

struct TarStatInfo {
    orig_file_name: *mut c_char,
    file_name: *mut c_char,
    had_trailing_slash: bool,
    link_name: *mut c_char,
    uname: *mut c_char,
    gname: *mut c_char,
    cntx_name: *mut c_char,
    acls_a_ptr: *mut c_char,
    acls_a_len: size_t,
    acls_d_ptr: *mut c_char,
    acls_d_len: size_t,
    stat: Stat,
    atime: Timespec,
    mtime: Timespec,
    ctime: Timespec,
    archive_file_size: off_t,
    is_sparse: bool,
    sparse_major: u32,
    sparse_minor: u32,
    sparse_map_avail: size_t,
    sparse_map_size: size_t,
    sparse_map: *mut SpArray,
    real_size: off_t,
    real_size_set: bool,
    sparse_name_done: bool,
    xattr_map_size: size_t,
    xattr_map: *mut XattrArray,
    xhdr: Xheader,
    is_dumpdir: bool,
    skipped: bool,
    dumpdir: *mut c_char,
    parent: *mut TarStatInfo,
    dirstream: *mut libc::DIR,
    fd: c_int,
    exclude_list: *mut Exclist,
}

struct KeywordList {
    next: *mut KeywordList,
    pattern: *mut c_char,
    value: *mut c_char,
}

struct XhdrTab {
    keyword: *const c_char,
    coder: Option<unsafe extern "C" fn(*const TarStatInfo, *const c_char, *mut Xheader, *const c_void)>,
    decoder: Option<unsafe extern "C" fn(*mut TarStatInfo, *const c_char, *const c_char, size_t)>,
    flags: c_int,
    prefix: bool,
}

static mut GLOBAL_HEADER_COUNT: size_t = 0;
static mut KEYWORD_PATTERN_LIST: *mut KeywordList = ptr::null_mut();
static mut KEYWORD_GLOBAL_OVERRIDE_LIST: *mut KeywordList = ptr::null_mut();
static mut KEYWORD_OVERRIDE_LIST: *mut KeywordList = ptr::null_mut();
static mut GLOBAL_HEADER_OVERRIDE_LIST: *mut KeywordList = ptr::null_mut();
static mut EXTHDR_NAME: *mut c_char = ptr::null_mut();
static mut EXTHDR_MTIME_OPTION: *mut c_char = ptr::null_mut();
static mut EXTHDR_MTIME: time_t = 0;
static mut GLOBEXTHDR_NAME: *mut c_char = ptr::null_mut();
static mut GLOBEXTHDR_MTIME_OPTION: *mut c_char = ptr::null_mut();
static mut GLOBEXTHDR_MTIME: time_t = 0;

unsafe fn xheader_keyword_deleted_p(kw: *const c_char) -> bool {
    let mut kp = KEYWORD_PATTERN_LIST;
    while !kp.is_null() {
        if libc::fnmatch((*kp).pattern, kw, 0) == 0 {
            return true;
        }
        kp = (*kp).next;
    }
    false
}

unsafe fn xheader_keyword_override_p(keyword: *const c_char) -> bool {
    let mut kp = KEYWORD_OVERRIDE_LIST;
    while !kp.is_null() {
        if libc::strcmp((*kp).pattern, keyword) == 0 {
            return true;
        }
        kp = (*kp).next;
    }
    false
}

unsafe fn xheader_list_append(
    root: *mut *mut KeywordList,
    kw: *const c_char,
    value: *const c_char,
) {
    let kp = libc::malloc(mem::size_of::<KeywordList>()) as *mut KeywordList;
    (*kp).pattern = libc::strdup(kw);
    (*kp).value = if !value.is_null() { libc::strdup(value) } else { ptr::null_mut() };
    (*kp).next = *root;
    *root = kp;
}

unsafe fn xheader_list_destroy(root: *mut *mut KeywordList) {
    if !root.is_null() {
        let mut kw = *root;
        while !kw.is_null() {
            let next = (*kw).next;
            libc::free((*kw).pattern as *mut c_void);
            libc::free((*kw).value as *mut c_void);
            libc::free(kw as *mut c_void);
            kw = next;
        }
        *root = ptr::null_mut();
    }
}

unsafe fn xheader_set_single_keyword(kw: *mut c_char) -> ! {
    if let Some(hook) = ERROR_HOOK {
        hook();
    }
    error(
        0,
        0,
        dcgettext(
            ptr::null(),
            b"Keyword %s is unknown or not yet implemented\0".as_ptr() as *const c_char,
            5,
        ),
        kw,
    );
    usage(2);
}

unsafe fn assign_time_option(
    sval: *mut *mut c_char,
    tval: *mut time_t,
    input: *const c_char,
) {
    let mut p = ptr::null_mut();
    let t = decode_timespec(input, &mut p, true);
    if !valid_timespec(t) || *p != 0 {
        if let Some(hook) = ERROR_HOOK {
            hook();
        }
        error(
            0,
            0,
            dcgettext(
                ptr::null(),
                b"Time stamp is out of allowed range\0".as_ptr() as *const c_char,
                5,
            ),
        );
        EXIT_STATUS = 2;
    } else {
        *tval = t.tv_sec;
        assign_string(sval, input);
    }
}

unsafe fn xheader_set_keyword_equal(
    kw: *mut c_char,
    eq: *mut c_char,
) {
    let mut global = true;
    let mut p = eq;
    if eq == kw {
        if let Some(hook) = ERROR_HOOK {
            hook();
        }
        error(
            0,
            0,
            dcgettext(
                ptr::null(),
                b"Malformed pax option: %s\0".as_ptr() as *const c_char,
                5,
            ),
            quote(kw),
        );
        usage(2);
    }
    if *eq.offset(-1) as c_int == ':' as i32 {
        p = p.offset(-1);
        global = false;
    }
    while p > kw && libc::isspace(*p as c_int) != 0 {
        p = p.offset(-1);
    }
    *p = 0;
    p = eq.offset(1);
    while *p != 0 && libc::isspace(*p as c_int) != 0 {
        p = p.offset(1);
    }
    if libc::strcmp(kw, b"delete\0".as_ptr() as *const c_char) == 0 {
        if xheader_protected_pattern_p(p) {
            if let Some(hook) = ERROR_HOOK {
                hook();
            }
            error(
                0,
                0,
                dcgettext(
                    ptr::null(),
                    b"Pattern %s cannot be used\0".as_ptr() as *const c_char,
                    5,
                ),
                quote(p),
            );
            usage(2);
        }
        xheader_list_append(&mut KEYWORD_PATTERN_LIST, p, ptr::null());
    } else if libc::strcmp(kw, b"exthdr.name\0".as_ptr() as *const c_char) == 0 {
        assign_string(&mut EXTHDR_NAME, p);
    } else if libc::strcmp(kw, b"globexthdr.name\0".as_ptr() as *const c_char) == 0 {
        assign_string(&mut GLOBEXTHDR_NAME, p);
    } else if libc::strcmp(kw, b"exthdr.mtime\0".as_ptr() as *const c_char) == 0 {
        assign_time_option(&mut EXTHDR_MTIME_OPTION, &mut EXTHDR_MTIME, p);
    } else if libc::strcmp(kw, b"globexthdr.mtime\0".as_ptr() as *const c_char) == 0 {
        assign_time_option(&mut GLOBEXTHDR_MTIME_OPTION, &mut GLOBEXTHDR_MTIME, p);
    } else {
        if xheader_protected_keyword_p(kw) {
            if let Some(hook) = ERROR_HOOK {
                hook();
            }
            error(
                0,
                0,
                dcgettext(
                    ptr::null(),
                    b"Keyword %s cannot be overridden\0".as_ptr() as *const c_char,
                    5,
                ),
                kw,
            );
            usage(2);
        }
        if global {
            xheader_list_append(&mut KEYWORD_GLOBAL_OVERRIDE_LIST, kw, p);
        } else {
            xheader_list_append(&mut KEYWORD_OVERRIDE_LIST, kw, p);
        }
    }
}

pub unsafe extern "C" fn xheader_set_option(string: *mut c_char) {
    let mut token = libc::strtok(string, b",\0".as_ptr() as *const c_char);
    while !token.is_null() {
        let p = libc::strchr(token, '=' as i32);
        if p.is_null() {
            xheader_set_single_keyword(token);
        } else {
            xheader_set_keyword_equal(token, p);
        }
        token = libc::strtok(ptr::null_mut(), b",\0".as_ptr() as *const c_char);
    }
}

// ... (additional functions would follow the same pattern of conversion)

// Note: This is a partial conversion focusing on the core data structures and functions.
// A complete conversion would require implementing all the remaining functions,
// handling memory management safely, and providing Rust alternatives to C-specific constructs.