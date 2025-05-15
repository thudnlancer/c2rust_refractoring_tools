use libc::{c_char, c_int, c_uint, c_ulong, c_long, c_void, size_t, off_t, uid_t, gid_t, mode_t, dev_t, ino_t, nlink_t, time_t, ssize_t};
use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::os::unix::fs::{OpenOptionsExt, MetadataExt};
use std::io::{Read, Write};
use std::collections::HashMap;

const IMPOSTOR_ERRNO: c_int = 2;
const FILES_MANY: c_uint = 2;
const FILES_ONE: c_uint = 1;
const FILES_NONE: c_uint = 0;

#[derive(Debug, Clone, Copy)]
struct Timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[derive(Debug)]
struct Stat {
    st_dev: dev_t,
    st_ino: ino_t,
    st_nlink: nlink_t,
    st_mode: mode_t,
    st_uid: uid_t,
    st_gid: gid_t,
    st_rdev: dev_t,
    st_size: off_t,
    st_blksize: c_long,
    st_blocks: c_long,
    st_atim: Timespec,
    st_mtim: Timespec,
    st_ctim: Timespec,
}

#[derive(Debug)]
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
    fd: c_int,
    exclude_list: *mut Exclist,
}

#[derive(Debug)]
struct SpArray {
    offset: off_t,
    numbytes: off_t,
}

#[derive(Debug)]
struct XHeader {
    stk: *mut Obstack,
    size: size_t,
    buffer: *mut c_char,
    string_length: uintmax_t,
}

#[derive(Debug)]
struct XattrArray {
    xkey: *mut c_char,
    xval_ptr: *mut c_char,
    xval_len: size_t,
}

#[derive(Debug)]
struct Obstack {
    chunk_size: size_t,
    chunk: *mut ObstackChunk,
    object_base: *mut c_char,
    next_free: *mut c_char,
    chunk_limit: *mut c_char,
    alignment_mask: size_t,
    chunkfun: ObstackChunkFun,
    freefun: ObstackFreeFun,
    extra_arg: *mut c_void,
}

#[derive(Debug)]
struct ObstackChunk {
    limit: *mut c_char,
    prev: *mut ObstackChunk,
    contents: [c_char; 0],
}

type ObstackChunkFun = Option<unsafe extern "C" fn(size_t) -> *mut c_void>;
type ObstackFreeFun = Option<unsafe extern "C" fn(*mut c_void)>;

#[derive(Debug)]
struct Exclist;

#[derive(Debug, Clone, Copy)]
enum ArchiveFormat {
    V7,
    OldGnu,
    Ustar,
    Posix,
    Star,
    Gnu,
    Default,
}

#[derive(Debug, Clone, Copy)]
enum AtimePreserve {
    No,
    Replace,
    System,
}

#[derive(Debug, Clone, Copy)]
enum ExclusionTagType {
    None,
    Contents,
    Under,
    All,
}

#[derive(Debug, Clone, Copy)]
enum SetMtimeOption {
    UseFile,
    Force,
    Clamp,
}

#[derive(Debug)]
struct Name {
    next: *mut Name,
    prev: *mut Name,
    name: *mut c_char,
    length: size_t,
    matching_flags: c_int,
    cmdline: bool,
    change_dir: c_int,
    found_count: uintmax_t,
    directory: *mut Directory,
    parent: *mut Name,
    child: *mut Name,
    sibling: *mut Name,
    caname: *mut c_char,
}

#[derive(Debug)]
struct Directory;

#[derive(Debug, Clone, Copy)]
enum AccessMode {
    Read,
    Write,
    Update,
}

#[derive(Debug, Clone, Copy)]
enum DumpStatus {
    Ok,
    Short,
    Fail,
    NotImplemented,
}

#[derive(Debug)]
struct ExclusionTag {
    name: *const c_char,
    length: size_t,
    type_: ExclusionTagType,
    predicate: Option<unsafe extern "C" fn(c_int) -> bool>,
    next: *mut ExclusionTag,
}

#[derive(Debug)]
struct Link {
    dev: dev_t,
    ino: ino_t,
    nlink: nlink_t,
    name: [c_char; 1],
}

#[derive(Debug)]
struct HashTable;

#[derive(Debug)]
struct HashTuning {
    shrink_threshold: f32,
    shrink_factor: f32,
    growth_threshold: f32,
    growth_factor: f32,
    is_n_buckets: bool,
}

type HashDataFreer = Option<unsafe extern "C" fn(*mut c_void)>;
type HashHasher = Option<unsafe extern "C" fn(*const c_void, size_t) -> size_t>;
type HashComparator = Option<unsafe extern "C" fn(*const c_void, *const c_void) -> bool>;

static mut exclusion_tags: *mut ExclusionTag = ptr::null_mut();
static mut link_table: *mut HashTable = ptr::null_mut();
static mut trivial_link_count: nlink_t = 0;

fn add_exclusion_tag(name: *const c_char, type_: ExclusionTagType, predicate: Option<unsafe extern "C" fn(c_int) -> bool>) {
    unsafe {
        let tag = libc::malloc(mem::size_of::<ExclusionTag>()) as *mut ExclusionTag;
        (*tag).next = exclusion_tags;
        (*tag).name = name;
        (*tag).type_ = type_;
        (*tag).predicate = predicate;
        (*tag).length = libc::strlen(name);
        exclusion_tags = tag;
    }
}

fn exclusion_tag_warning(dirname: *const c_char, tagname: *const c_char, message: *const c_char) {
    unsafe {
        if verbose_option != 0 {
            if warning_option & 0x4 != 0 {
                if let Some(hook) = error_hook {
                    hook();
                }
                error(
                    0,
                    0,
                    dcgettext(
                        ptr::null(),
                        b"%s: contains a cache directory tag %s; %s\0".as_ptr() as *const c_char,
                        5,
                    ),
                    quotearg_colon(dirname),
                    quotearg_n(1, tagname),
                    message,
                );
            }
        }
    }
}

fn check_exclusion_tags(st: *const TarStatInfo, tag_file_name: *mut *const c_char) -> ExclusionTagType {
    unsafe {
        let mut tag = exclusion_tags;
        while !tag.is_null() {
            let tagfd = subfile_open(st, (*tag).name, open_read_flags);
            if tagfd >= 0 {
                let satisfied = (*tag).predicate.is_none() || (*tag).predicate.unwrap()(tagfd) != 0;
                libc::close(tagfd);
                if satisfied {
                    if !tag_file_name.is_null() {
                        *tag_file_name = (*tag).name;
                    }
                    return (*tag).type_;
                }
            }
            tag = (*tag).next;
        }
        ExclusionTagType::None
    }
}

fn cachedir_file_p(fd: c_int) -> bool {
    unsafe {
        let mut tagbuf = [0; 43];
        let count = libc::read(
            fd,
            tagbuf.as_mut_ptr() as *mut c_void,
            mem::size_of_val(&tagbuf) - 1,
        );
        count as usize == mem::size_of_val(&tagbuf) - 1
            && libc::memcmp(
                tagbuf.as_ptr() as *const c_void,
                b"Signature: 8a477f597d28d172789f06886806bc55\0".as_ptr() as *const c_char as *const c_void,
                mem::size_of_val(&tagbuf) - 1,
            ) == 0
    }
}

fn to_octal(value: uintmax_t, where_: *mut c_char, size: size_t) {
    unsafe {
        let mut v = value;
        let mut i = size;
        loop {
            i -= 1;
            *where_.add(i) = b'0' as c_char + (v & 0o7) as c_char;
            v >>= 3;
            if i == 0 {
                break;
            }
        }
    }
}

fn tar_copy_str(dst: *mut c_char, src: *const c_char, len: size_t) {
    unsafe {
        for i in 0..len {
            *dst.add(i) = *src.add(i);
            if *dst.add(i) == 0 {
                break;
            }
        }
    }
}

fn tar_name_copy_str(dst: *mut c_char, src: *const c_char, len: size_t) {
    tar_copy_str(dst, src, len);
    unsafe {
        if archive_format == ArchiveFormat::OldGnu {
            *dst.add(len - 1) = 0;
        }
    }
}

fn to_base256(negative: c_int, value: uintmax_t, where_: *mut c_char, size: size_t) {
    unsafe {
        let mut v = value;
        let mut i = size;
        loop {
            i -= 1;
            *where_.add(i) = (v & 0xff) as c_char;
            v >>= 8;
            if i == 0 {
                break;
            }
        }
    }
}

fn to_chars_subst(
    negative: c_int,
    gnu_format: c_int,
    value: uintmax_t,
    valsize: size_t,
    substitute: Option<unsafe extern "C" fn(*mut c_int) -> uintmax_t>,
    where_: *mut c_char,
    size: size_t,
    type_: *const c_char,
) -> bool {
    unsafe {
        let maxval = if gnu_format != 0 {
            if (size - 1) * 8 < mem::size_of::<uintmax_t>() * 8 {
                (1 << ((size - 1) * 8)) - 1
            } else {
                !0
            }
        } else if (size - 1) * 3 < mem::size_of::<uintmax_t>() * 8 {
            (1 << ((size - 1) * 3)) - 1
        } else {
            !0
        };

        let mut valbuf = [0; 22];
        let mut maxbuf = [0; 21];
        let mut minbuf = [0; 22];
        let minval_string: *const c_char;
        let maxval_string = umaxtostr(maxval, maxbuf.as_mut_ptr());
        let value_string: *const c_char;

        if gnu_format != 0 {
            let m = if maxval.wrapping_add(1) != 0 {
                maxval.wrapping_add(1)
            } else {
                maxval.wrapping_div(2).wrapping_add(1)
            };
            let mut p = umaxtostr(m, minbuf[1..].as_mut_ptr());
            p = p.offset(-1);
            *p = b'-' as c_char;
            minval_string = p;
        } else {
            minval_string = b"0\0".as_ptr() as *const c_char;
        }

        if negative != 0 {
            let mut p = umaxtostr(value.wrapping_neg(), valbuf[1..].as_mut_ptr());
            p = p.offset(-1);
            *p = b'-' as c_char;
            value_string = p;
        } else {
            value_string = umaxtostr(value, valbuf.as_mut_ptr());
        }

        if let Some(sub) = substitute {
            let mut negsub = 0;
            let sub_val = sub(&mut negsub) & maxval;
            negsub &= (archive_format == ArchiveFormat::Gnu) as c_int;
            let s = if negsub != 0 { sub_val.wrapping_neg() } else { sub_val };
            let mut subbuf = [0; 22];
            let mut sub_string = umaxtostr(s, subbuf[1..].as_mut_ptr());
            if negsub != 0 {
                sub_string = sub_string.offset(-1);
                *sub_string = b'-' as c_char;
            }
            if let Some(hook) = error_hook {
                hook();
            }
            error(
                0,
                0,
                dcgettext(
                    ptr::null(),
                    b"value %s out of %s range %s..%s; substituting %s\0".as_ptr() as *const c_char,
                    5,
                ),
                value_string,
                type_,
                minval_string,
                maxval_string,
                sub_string,
            );
            return to_chars(negsub, s, valsize, None, where_, size, type_);
        } else {
            if let Some(hook) = error_hook {
                hook();
            }
            error(
                0,
                0,
                dcgettext(
                    ptr::null(),
                    b"value %s out of %s range %s..%s\0".as_ptr() as *const c_char,
                    5,
                ),
                value_string,
                type_,
                minval_string,
                maxval_string,
            );
            exit_status = 2;
        }
        false
    }
}

fn to_chars(
    negative: c_int,
    value: uintmax_t,
    valsize: size_t,
    substitute: Option<unsafe extern "C" fn(*mut c_int) -> uintmax_t>,
    where_: *mut c_char,
    size: size_t,
    type_: *const c_char,
) -> bool {
    unsafe {
        let gnu_format = (archive_format == ArchiveFormat::Gnu || archive_format == ArchiveFormat::OldGnu) as c_int;
        if negative == 0
            && value
                <= if (size - 1) * 3 < mem::size_of::<uintmax_t>() * 8 {
                    (1 << ((size - 1) * 3)) - 1
                } else {
                    !0
                }
        {
            *where_.add(size - 1) = 0;
            to_octal(value, where_, size - 1);
            true
        } else if gnu_format != 0 {
            let val = if negative != 0 {
                !0 - value
            } else {
                value
            };
            if val
                <= if (size - 1) * 8 < mem::size_of::<uintmax_t>() * 8 {
                    (1 << ((size - 1) * 8)) - 1
                } else {
                    !0
                }
            {
                *where_ = if negative != 0 {
                    -1
                } else {
                    1 << (8 - 1)
                } as c_char;
                to_base256(negative, value, where_.add(1), size - 1);
                true
            } else if negative != 0
                && valsize * 8 <= (size - 1) * 3
            {
                static mut warned_once: c_int = 0;
                if warned_once == 0 {
                    warned_once = 1;
                    if let Some(hook) = error_hook {
                        hook();
                    }
                    error(
                        0,
                        0,
                        dcgettext(
                            ptr::null(),
                            b"Generating negative octal headers\0".as_ptr() as *const c_char,
                            5,
                        ),
                    );
                }
                *where_.add(size - 1) = 0;
                to_octal(
                    value
                        & if valsize * 8 * 1 < mem::size_of::<uintmax_t>() * 8 {
                            (1 << (valsize * 8 * 1)) - 1
                        } else {
                            !0
                        },
                    where_,
                    size - 1,
                );
                true
            } else {
                substitute = None;
                to_chars_subst(negative, gnu_format, value, valsize, substitute, where_, size, type_)
            }
        } else {
            substitute = None;
            to_chars_subst(negative, gnu_format, value, valsize, substitute, where_, size, type_)
        }
    }
}

fn gid_substitute(negative: *mut c_int) -> uintmax_t {
    unsafe {
        static mut gid_nobody: gid_t = 0;
        if gid_nobody == 0
            && gname_to_gid(b"nobody\0".as_ptr() as *const c_char, &mut gid_nobody) == 0
        {
            gid_nobody = !1;
        }
        let r = gid_nobody