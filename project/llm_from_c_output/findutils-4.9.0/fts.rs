use std::{
    ffi::{CStr, CString, OsStr, OsString},
    fs::{self, DirEntry, File, Metadata, OpenOptions},
    io::{self, Error, ErrorKind},
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    ptr,
    sync::Arc,
};

use libc::{self, c_int, c_void, dirent, stat};

const FTS_NOCHDIR: u32 = 0x01;
const FTS_PHYSICAL: u32 = 0x02;
const FTS_LOGICAL: u32 = 0x04;
const FTS_SEEDOT: u32 = 0x08;
const FTS_XDEV: u32 = 0x10;
const FTS_COMFOLLOW: u32 = 0x20;
const FTS_NOSTAT: u32 = 0x40;
const FTS_DEFER_STAT: u32 = 0x80;
const FTS_CWDFD: u32 = 0x100;
const FTS_TIGHT_CYCLE_CHECK: u32 = 0x200;
const FTS_OPTIONMASK: u32 = 0x3ff;

const FTS_AGAIN: u16 = 1;
const FTS_FOLLOW: u16 = 2;
const FTS_NOINSTR: u16 = 3;
const FTS_SKIP: u16 = 4;

const FTS_ROOTPARENTLEVEL: i16 = -1;
const FTS_ROOTLEVEL: i16 = 0;

const FTS_DEFAULT: u16 = 0;
const FTS_D: u16 = 1;
const FTS_DC: u16 = 2;
const FTS_DNR: u16 = 3;
const FTS_DOT: u16 = 4;
const FTS_DP: u16 = 5;
const FTS_ERR: u16 = 6;
const FTS_F: u16 = 7;
const FTS_INIT: u16 = 8;
const FTS_NS: u16 = 9;
const FTS_NSOK: u16 = 10;
const FTS_SL: u16 = 11;
const FTS_SLNONE: u16 = 12;

const FTS_STOP: u32 = 0x1000;

const MAXPATHLEN: usize = 1024;

struct FTSENT {
    fts_link: *mut FTSENT,
    fts_parent: *mut FTSENT,
    fts_path: *mut u8,
    fts_accpath: *mut u8,
    fts_name: *mut u8,
    fts_namelen: usize,
    fts_pathlen: usize,
    fts_level: i16,
    fts_errno: i32,
    fts_number: i64,
    fts_pointer: *mut c_void,
    fts_fts: *mut FTS,
    fts_info: u16,
    fts_flags: u16,
    fts_instr: u16,
    fts_statp: *mut stat,
    fts_dirp: *mut libc::DIR,
    fts_symfd: c_int,
}

struct FTS {
    fts_compar: Option<extern "C" fn(*const *const FTSENT, *const *const FTSENT) -> c_int>,
    fts_options: u32,
    fts_path: *mut u8,
    fts_pathlen: usize,
    fts_nitems: usize,
    fts_array: *mut *mut FTSENT,
    fts_cur: *mut FTSENT,
    fts_child: *mut FTSENT,
    fts_dev: libc::dev_t,
    fts_cwd_fd: c_int,
    fts_rfd: c_int,
    fts_fd_ring: I_RING,
    fts_leaf_optimization_works_ht: *mut c_void,
}

struct I_RING {
    fts_fd_ring: [c_int; 16],
    fts_front: usize,
    fts_back: usize,
}

impl I_RING {
    fn init(&mut self, value: c_int) {
        self.fts_front = 0;
        self.fts_back = 0;
        for i in 0..16 {
            self.fts_fd_ring[i] = value;
        }
    }

    fn empty(&self) -> bool {
        self.fts_front == self.fts_back
    }

    fn push(&mut self, fd: c_int) -> c_int {
        let old = self.fts_fd_ring[self.fts_back];
        self.fts_fd_ring[self.fts_back] = fd;
        self.fts_back = (self.fts_back + 1) % 16;
        old
    }

    fn pop(&mut self) -> c_int {
        let fd = self.fts_fd_ring[self.fts_front];
        self.fts_fd_ring[self.fts_front] = -1;
        self.fts_front = (self.fts_front + 1) % 16;
        fd
    }
}

extern "C" {
    fn opendir(dirname: *const libc::c_char) -> *mut libc::DIR;
    fn readdir(dirp: *mut libc::DIR) -> *mut dirent;
    fn closedir(dirp: *mut libc::DIR) -> c_int;
    fn fchdir(fd: c_int) -> c_int;
    fn open(path: *const libc::c_char, oflag: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn stat(path: *const libc::c_char, buf: *mut stat) -> c_int;
    fn lstat(path: *const libc::c_char, buf: *mut stat) -> c_int;
    fn fstatat(
        dirfd: c_int,
        pathname: *const libc::c_char,
        buf: *mut stat,
        flags: c_int,
    ) -> c_int;
    fn qsort(
        base: *mut c_void,
        nmemb: usize,
        size: usize,
        compar: Option<extern "C" fn(*const c_void, *const c_void) -> c_int>,
    );
    fn malloc(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> c_int;
    fn strlen(s: *const libc::c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize);
    fn memset(s: *mut c_void, c: c_int, n: usize);
}

#[no_mangle]
pub extern "C" fn fts_open(
    argv: *mut *mut libc::c_char,
    options: c_int,
    compar: Option<extern "C" fn(*const *const FTSENT, *const *const FTSENT) -> c_int>,
) -> *mut FTS {
    let options = options as u32;
    if (options & !FTS_OPTIONMASK) != 0 {
        unsafe {
            libc::__errno_location().write(libc::EINVAL);
        }
        return ptr::null_mut();
    }

    if (options & FTS_NOCHDIR) != 0 && (options & FTS_CWDFD) != 0 {
        unsafe {
            libc::__errno_location().write(libc::EINVAL);
        }
        return ptr::null_mut();
    }

    if (options & (FTS_LOGICAL | FTS_PHYSICAL)) == 0 {
        unsafe {
            libc::__errno_location().write(libc::EINVAL);
        }
        return ptr::null_mut();
    }

    let sp = unsafe { malloc(std::mem::size_of::<FTS>()) as *mut FTS };
    if sp.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        (*sp).fts_compar = compar;
        (*sp).fts_options = options;
    }

    if (options & FTS_LOGICAL != 0) {
        unsafe {
            (*sp).fts_options |= FTS_NOCHDIR;
            (*sp).fts_options &= !FTS_CWDFD;
        }
    }

    unsafe {
        (*sp).fts_cwd_fd = libc::AT_FDCWD;
        if (options & FTS_CWDFD != 0 && !HAVE_OPENAT_SUPPORT {
            let fd = open(b".\0".as_ptr() as *const libc::c_char, libc::O_SEARCH | libc::O_CLOEXEC);
            if fd >= 0 {
                close(fd);
            } else if openat_needs_fchdir() {
                (*sp).fts_options |= FTS_NOCHDIR;
                (*sp).fts_options &= !FTS_CWDFD;
            }
        }
    }

    let maxarglen = unsafe { fts_maxarglen(argv) };
    let pathlen = std::cmp::max(maxarglen, MAXPATHLEN);
    if unsafe { fts_palloc(sp, pathlen) } == 0 {
        unsafe { free(sp as *mut c_void) };
        return ptr::null_mut();
    }

    let mut parent = ptr::null_mut();
    if unsafe { !(*argv).is_null() } {
        parent = unsafe { fts_alloc(sp, b"\0".as_ptr(), 0) };
        if parent.is_null() {
            unsafe { free((*sp).fts_path as *mut c_void) };
            unsafe { free(sp as *mut c_void) };
            return ptr::null_mut();
        }
        unsafe { (*parent).fts_level = FTS_ROOTPARENTLEVEL };
    }

    let defer_stat = unsafe { compar.is_none() || ((*sp).fts_options & FTS_DEFER_STAT) != 0 };

    let mut root = ptr::null_mut();
    let mut nitems = 0;
    let mut tmp = ptr::null_mut();
    while unsafe { !(*argv.offset(nitems as isize)).is_null() } {
        let mut len = unsafe { strlen(*argv.offset(nitems as isize)) };
        let name = unsafe { *argv.offset(nitems as isize) };

        if (options & FTS_VERBATIM) == 0 {
            let mut v = unsafe { *argv.offset(nitems as isize) };
            if len > 2 && unsafe { *v.offset((len - 1) as isize) } == b'/' as i8 {
                while len > 1 && unsafe { *v.offset((len - 2) as isize) } == b'/' as i8 {
                    len -= 1;
                }
            }
        }

        let p = unsafe { fts_alloc(sp, name, len) };
        if p.is_null() {
            unsafe { fts_lfree(root) };
            unsafe { free(parent as *mut c_void) };
            unsafe { free((*sp).fts_path as *mut c_void) };
            unsafe { free(sp as *mut c_void) };
            return ptr::null_mut();
        }

        unsafe { (*p).fts_level = FTS_ROOTLEVEL };
        unsafe { (*p).fts_parent = parent };
        unsafe { (*p).fts_accpath = (*p).fts_name };

        if defer_stat && !root.is_null() {
            unsafe { (*p).fts_info = FTS_NSOK };
            unsafe { fts_set_stat_required(p, true) };
        } else {
            unsafe { (*p).fts_info = fts_stat(sp, p, false) };
        }

        if compar.is_some() {
            unsafe { (*p).fts_link = root };
            root = p;
        } else {
            unsafe { (*p).fts_link = ptr::null_mut() };
            if root.is_null() {
                root = p;
                tmp = p;
            } else {
                unsafe { (*tmp).fts_link = p };
                tmp = p;
            }
        }

        nitems += 1;
    }

    if compar.is_some() && nitems > 1 {
        root = unsafe { fts_sort(sp, root, nitems) };
    }

    let cur = unsafe { fts_alloc(sp, b"\0".as_ptr(), 0) };
    if cur.is_null() {
        unsafe { fts_lfree(root) };
        unsafe { free(parent as *mut c_void) };
        unsafe { free((*sp).fts_path as *mut c_void) };
        unsafe { free(sp as *mut c_void) };
        return ptr::null_mut();
    }

    unsafe {
        (*sp).fts_cur = cur;
        (*cur).fts_link = root;
        (*cur).fts_info = FTS_INIT;
        (*cur).fts_level = 1;
    }

    if unsafe { setup_dir(sp) } == 0 {
        unsafe { fts_lfree(root) };
        unsafe { free(parent as *mut c_void) };
        unsafe { free((*sp).fts_path as *mut c_void) };
        unsafe { free(sp as *mut c_void) };
        return ptr::null_mut();
    }

    if (options & FTS_NOCHDIR) == 0 && (options & FTS_CWDFD) == 0 {
        let rfd = unsafe { diropen(sp, b".\0".as_ptr() as *const libc::c_char) };
        if rfd < 0 {
            unsafe { (*sp).fts_options |= FTS_NOCHDIR };
        } else {
            unsafe { (*sp).fts_rfd = rfd };
        }
    }

    unsafe {
        (*sp).fts_fd_ring.init(-1);
    }

    sp
}

#[no_mangle]
pub extern "C" fn fts_close(sp: *mut FTS) -> c_int {
    if sp.is_null() {
        return -1;
    }

    let mut saved_errno = 0;

    unsafe {
        if !(*sp).fts_cur.is_null() {
            let mut p = (*sp).fts_cur;
            while (*p).fts_level >= FTS_ROOTLEVEL {
                let freep = p;
                p = if !(*p).fts_link.is_null() {
                    (*p).fts_link
                } else {
                    (*p).fts_parent
                };
                free(freep as *mut c_void);
            }
            free(p as *mut c_void);
        }

        if !(*sp).fts_child.is_null() {
            fts_lfree((*sp).fts_child);
        }

        free((*sp).fts_array as *mut c_void);
        free((*sp).fts_path as *mut c_void);

        if (*sp).fts_options & FTS_CWDFD != 0 {
            if (*sp).fts_cwd_fd >= 0 {
                if close((*sp).fts_cwd_fd) != 0 {
                    saved_errno = Error::last_os_error().raw_os_error().unwrap_or(0);
                }
            }
        } else if (*sp).fts_options & FTS_NOCHDIR == 0 {
            if fchdir((*sp).fts_rfd) != 0 {
                saved_errno = Error::last_os_error().raw_os_error().unwrap_or(0);
            }
            if close((*sp).fts_rfd) != 0 && saved_errno == 0 {
                saved_errno = Error::last_os_error().raw_os_error().unwrap_or(0);
            }
        }

        fd_ring_clear(&mut (*sp).fts_fd_ring);

        if !(*sp).fts_leaf_optimization_works_ht.is_null() {
            hash_free((*sp).fts_leaf_optimization_works_ht);
        }

        free_dir(sp);
        free(sp as *mut c_void);
    }

    if saved_errno != 0 {
        unsafe {
            libc::__errno_location().write(saved_errno);
        }
        -1
    } else {
        0
    }
}

// ... (其他函数的实现类似，限于篇幅未完全展示)

#[no_mangle]
pub extern "C" fn fts_set(
    _sp: *mut FTS,
    p: *mut FTSENT,
    instr: c_int,
) -> c_int {
    if instr != 0
        && instr != FTS_AGAIN as c_int
        && instr != FTS_FOLLOW as c_int
        && instr != FTS_NOINSTR as c_int
        && instr != FTS_SKIP as c_int
    {
        unsafe {
            libc::__errno_location().write(libc::EINVAL);
        }
        return 1;
    }
    unsafe {
        (*p).fts_instr = instr as u16;
    }
    0
}