use libc::{c_int, c_char, c_void, c_ulong, c_long, c_uint, off_t, size_t, ssize_t};
use std::ptr;
use std::mem;
use std::ffi::CString;
use std::os::unix::io::RawFd;

const DEFAULT_MXFAST: usize = 128;

#[repr(C)]
struct Timespec {
    tv_sec: libc::time_t,
    tv_nsec: libc::c_long,
}

#[repr(C)]
struct Stat {
    st_dev: libc::dev_t,
    st_ino: libc::ino_t,
    st_nlink: libc::nlink_t,
    st_mode: libc::mode_t,
    st_uid: libc::uid_t,
    st_gid: libc::gid_t,
    __pad0: c_int,
    st_rdev: libc::dev_t,
    st_size: off_t,
    st_blksize: libc::blksize_t,
    st_blocks: libc::blkcnt_t,
    st_atim: Timespec,
    st_mtim: Timespec,
    st_ctim: Timespec,
    __glibc_reserved: [libc::c_long; 3],
}

#[repr(C)]
struct SpArray {
    offset: off_t,
    numbytes: off_t,
}

#[repr(C)]
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
    sparse_major: c_uint,
    sparse_minor: c_uint,
    sparse_map_avail: size_t,
    sparse_map_size: size_t,
    sparse_map: *mut SpArray,
    real_size: off_t,
    real_size_set: bool,
    sparse_name_done: bool,
    xattr_map_size: size_t,
    xattr_map: *mut XattrArray,
    xhdr: XHeader,
    is_dumpdir: bool,
    skipped: bool,
    dumpdir: *mut c_char,
    parent: *mut TarStatInfo,
    dirstream: *mut libc::DIR,
    fd: RawFd,
    exclude_list: *mut libc::c_void,
}

#[repr(C)]
struct TarSparseFile {
    fd: RawFd,
    seekable: bool,
    offset: off_t,
    dumped_size: off_t,
    stat_info: *mut TarStatInfo,
    optab: *const TarSparseOptab,
    closure: *mut c_void,
}

#[repr(C)]
struct TarSparseOptab {
    init: Option<unsafe extern "C" fn(*mut TarSparseFile) -> bool>,
    done: Option<unsafe extern "C" fn(*mut TarSparseFile) -> bool>,
    sparse_member_p: Option<unsafe extern "C" fn(*mut TarSparseFile) -> bool>,
    dump_header: Option<unsafe extern "C" fn(*mut TarSparseFile) -> bool>,
    fixup_header: Option<unsafe extern "C" fn(*mut TarSparseFile) -> bool>,
    decode_header: Option<unsafe extern "C" fn(*mut TarSparseFile) -> bool>,
    scan_block: Option<unsafe extern "C" fn(*mut TarSparseFile, SparseScanState, *mut c_void) -> bool>,
    dump_region: Option<unsafe extern "C" fn(*mut TarSparseFile, size_t) -> bool>,
    extract_region: Option<unsafe extern "C" fn(*mut TarSparseFile, size_t) -> bool>,
}

#[repr(u32)]
enum SparseScanState {
    ScanBegin = 0,
    ScanBlock = 1,
    ScanEnd = 2,
}

#[repr(u32)]
enum OldgnuAddStatus {
    AddOk = 0,
    AddFinish = 1,
    AddFail = 2,
}

#[repr(u32)]
enum DumpStatus {
    DumpOk = 0,
    DumpShort = 1,
    DumpFail = 2,
    DumpNotImplemented = 3,
}

#[repr(u32)]
enum ArchiveFormat {
    DefaultFormat = 0,
    V7Format = 1,
    OldgnuFormat = 2,
    UstarFormat = 3,
    PosixFormat = 4,
    StarFormat = 5,
    GnuFormat = 6,
}

#[repr(u32)]
enum HoleDetectionMethod {
    HoleDetectionDefault = 0,
    HoleDetectionRaw = 1,
    HoleDetectionSeek = 2,
}

static mut error_hook: Option<unsafe extern "C" fn()> = None;
static mut exit_status: c_int = 0;
static mut archive_format: ArchiveFormat = ArchiveFormat::DefaultFormat;
static mut ignore_failed_read_option: bool = false;
static mut tar_sparse_major: c_uint = 0;
static mut tar_sparse_minor: c_uint = 0;
static mut hole_detection: HoleDetectionMethod = HoleDetectionMethod::HoleDetectionDefault;
static mut current_stat_info: TarStatInfo = unsafe { mem::zeroed() };
static mut current_header: *mut Block = ptr::null_mut();
static mut current_format: ArchiveFormat = ArchiveFormat::DefaultFormat;
static mut warning_option: c_int = 0;

#[repr(C)]
struct Block {
    buffer: [c_char; 512],
}

extern "C" {
    fn __fxstat(ver: c_int, fildes: RawFd, stat_buf: *mut Stat) -> c_int;
    fn lseek(fd: RawFd, offset: off_t, whence: c_int) -> off_t;
    fn write(fd: RawFd, buf: *const c_void, count: size_t) -> ssize_t;
    fn rpl_free(ptr: *mut c_void);
    fn __strtoul_internal(
        nptr: *const c_char,
        endptr: *mut *mut c_char,
        base: c_int,
        group: c_int,
    ) -> c_ulong;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn xalloc_die();
    fn xcalloc(n: size_t, s: size_t) -> *mut c_void;
    fn xrealloc(p: *mut c_void, s: size_t) -> *mut c_void;
    fn dcgettext(domainname: *const c_char, msgid: *const c_char, category: c_int) -> *mut c_char;
    fn dcngettext(
        domainname: *const c_char,
        msgid1: *const c_char,
        msgid2: *const c_char,
        n: c_ulong,
        category: c_int,
    ) -> *mut c_char;
    fn offtostr(off: off_t, buf: *mut c_char) -> *mut c_char;
    fn umaxtostr(num: u64, buf: *mut c_char) -> *mut c_char;
    fn truncate_warn(name: *const c_char);
    fn write_error_details(name: *const c_char, count: size_t, wrbytes: size_t);
    fn fatal_exit() -> !;
    fn quotearg_colon(arg: *const c_char) -> *mut c_char;
    fn current_block_ordinal() -> off_t;
    fn find_next_block() -> *mut Block;
    fn set_next_block_after(block: *mut Block);
    fn mv_begin_write(file_name: *const c_char, totsize: off_t, sizeleft: off_t);
    fn mv_begin_read(st: *mut TarStatInfo);
    fn mv_end();
    fn mv_size_left(size: off_t);
    fn pad_archive(size_left: off_t);
    fn start_header(st: *mut TarStatInfo) -> *mut Block;
    fn finish_header(st: *mut TarStatInfo, header: *mut Block, block_ordinal: off_t);
    fn off_to_chars(off: off_t, buf: *mut c_char, size: size_t) -> bool;
    fn off_from_header(buf: *const c_char, size: size_t) -> off_t;
    fn skip_file(size: off_t);
    fn blocking_read(fd: RawFd, buf: *mut c_void, count: size_t) -> size_t;
    fn blocking_write(fd: RawFd, buf: *const c_void, count: size_t) -> size_t;
    fn read_diag_details(name: *const c_char, offset: off_t, size: size_t);
    fn seek_diag_details(name: *const c_char, offset: off_t);
    fn set_exit_status(val: c_int);
    fn xheader_store(keyword: *const c_char, st: *mut TarStatInfo, data: *const c_void);
    fn xheader_string_begin(xhdr: *mut XHeader);
    fn xheader_string_add(xhdr: *mut XHeader, s: *const c_char);
    fn xheader_string_end(xhdr: *mut XHeader, keyword: *const c_char) -> bool;
    fn xheader_keyword_deleted_p(kw: *const c_char) -> bool;
    fn xheader_format_name(st: *mut TarStatInfo, fmt: *const c_char, n: size_t) -> *mut c_char;
    fn sys_truncate(fd: RawFd) -> c_int;
    fn report_difference(st: *mut TarStatInfo, message: *const c_char, ...);
    fn safe_read(fd: RawFd, buf: *mut c_void, count: size_t) -> size_t;
}

#[inline]
unsafe fn fstat(fd: RawFd, statbuf: *mut Stat) -> c_int {
    __fxstat(1, fd, statbuf)
}

#[inline]
unsafe fn strtoumax(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64 {
    __strtoul_internal(nptr, endptr, base, 0) as u64
}

#[inline]
unsafe fn x2nrealloc(p: *mut c_void, pn: *mut size_t, s: size_t) -> *mut c_void {
    let mut n = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as c_ulong).wrapping_div(s);
            n = n.wrapping_add((n == 0) as usize);
        }
        if (u64::MAX / s) < n {
            xalloc_die();
        }
    } else {
        if (u64::MAX / 3 * 2 / s) <= n {
            xalloc_die();
        }
        n = n.wrapping_add(n / 2 + 1);
    }
    *pn = n;
    xrealloc(p, n.wrapping_mul(s))
}

unsafe fn dump_zeros(file: *mut TarSparseFile, offset: off_t) -> bool {
    static mut zero_buf: [c_char; 512] = [0; 512];
    if offset < (*file).offset {
        *libc::__errno_location() = libc::EINVAL;
        return false;
    }
    while (*file).offset < offset {
        let size = (512.min(offset - (*file).offset) as size_t;
        let wrbytes = write((*file).fd, zero_buf.as_ptr() as *const c_void, size);
        if wrbytes <= 0 {
            if wrbytes == 0 {
                *libc::__errno_location() = libc::EINVAL;
            }
            return false;
        }
        (*file).offset += wrbytes;
    }
    true
}

unsafe fn tar_sparse_member_p(file: *mut TarSparseFile) -> bool {
    if let Some(func) = (*(*file).optab).sparse_member_p {
        return func(file);
    }
    false
}

unsafe fn tar_sparse_init(file: *mut TarSparseFile) -> bool {
    libc::memset(
        file as *mut c_void,
        0,
        mem::size_of::<TarSparseFile>(),
    );
    if !sparse_select_optab(file) {
        return false;
    }
    if let Some(func) = (*(*file).optab).init {
        return func(file);
    }
    true
}

unsafe fn tar_sparse_done(file: *mut TarSparseFile) -> bool {
    if let Some(func) = (*(*file).optab).done {
        return func(file);
    }
    true
}

unsafe fn tar_sparse_scan(
    file: *mut TarSparseFile,
    state: SparseScanState,
    block: *mut c_void,
) -> bool {
    if let Some(func) = (*(*file).optab).scan_block {
        return func(file, state, block);
    }
    true
}

unsafe fn tar_sparse_dump_region(file: *mut TarSparseFile, i: size_t) -> bool {
    if let Some(func) = (*(*file).optab).dump_region {
        return func(file, i);
    }
    false
}

unsafe fn tar_sparse_extract_region(file: *mut TarSparseFile, i: size_t) -> bool {
    if let Some(func) = (*(*file).optab).extract_region {
        return func(file, i);
    }
    false
}

unsafe fn tar_sparse_dump_header(file: *mut TarSparseFile) -> bool {
    if let Some(func) = (*(*file).optab).dump_header {
        return func(file);
    }
    false
}

unsafe fn tar_sparse_decode_header(file: *mut TarSparseFile) -> bool {
    if let Some(func) = (*(*file).optab).decode_header {
        return func(file);
    }
    true
}

unsafe fn tar_sparse_fixup_header(file: *mut TarSparseFile) -> bool {
    if let Some(func) = (*(*file).optab).fixup_header {
        return func(file);
    }
    true
}

unsafe fn lseek_or_error(file: *mut TarSparseFile, offset: off_t) -> bool {
    if (*file).seekable {
        if lseek((*file).fd, offset, libc::SEEK_SET) < 0 {
            seek_diag_details((*(*file).stat_info).orig_file_name, offset);
            return false;
        }
    } else if !dump_zeros(file, offset) {
        seek_diag_details((*(*file).stat_info).orig_file_name, offset);
        return false;
    }
    true
}

unsafe fn zero_block_p(buffer: *const c_char, size: size_t) -> bool {
    for i in 0..size {
        if *buffer.add(i) != 0 {
            return false;
        }
    }
    true
}

unsafe fn sparse_add_map(st: *mut TarStatInfo, sp: *const SpArray) {
    let mut sparse_map = (*st).sparse_map;
    let avail = (*st).sparse_map_avail;
    if avail == (*st).sparse_map_size {
        sparse_map = x2nrealloc(
            sparse_map as *mut c_void,
            &mut (*st).sparse_map_size,
            mem::size_of::<SpArray>(),
        ) as *mut SpArray;
        (*st).sparse_map = sparse_map;
    }
    *sparse_map.add(avail) = *sp;
    (*st).sparse_map_avail = avail + 1;
}

unsafe fn sparse_scan_file_raw(file: *mut TarSparseFile) -> bool {
    let st = (*file).stat_info;
    let fd = (*file).fd;
    let mut buffer = [0; 512];
    let mut count = 0;
    let mut offset = 0;
    let mut sp = SpArray {
        offset: 0,
        numbytes: 0,
    };
    (*st).archive_file_size = 0;

    if !tar_sparse_scan(file, SparseScanState::ScanBegin, ptr::null_mut()) {
        return false;
    }

    loop {
        count = blocking_read(fd, buffer.as_mut_ptr() as *mut c_void, 512);
        if count == 0 || count == !0 {
            break;
        }

        if zero_block_p(buffer.as_ptr(), count) {
            if sp.numbytes != 0 {
                sparse_add_map(st, &sp);
                sp.numbytes = 0;
                if !tar_sparse_scan(file, SparseScanState::ScanBlock, ptr::null_mut()) {
                    return false;
                }
            }
        } else {
            if sp.numbytes == 0 {
                sp.offset = offset;
            }
            sp.numbytes += count as off_t;
            (*st).archive_file_size += count as off_t;
            if !tar_sparse_scan(
                file,
                SparseScanState::ScanBlock,
                buffer.as_mut_ptr() as *mut c_void,
            ) {
                return false;
            }
        }
        offset += count as off_t;
    }

    if sp.numbytes == 0 {
        sp.offset = offset;
    }
    sparse_add_map(st, &sp);
    (*st).archive_file_size += count as off_t;
    tar_sparse_scan(file, SparseScanState::ScanEnd, ptr::null_mut())
}

unsafe fn sparse_scan_file_wholesparse(file: *mut TarSparseFile) -> bool {
    let st = (*file).stat_info;
    let mut sp = SpArray {
        offset: 0,
        numbytes: