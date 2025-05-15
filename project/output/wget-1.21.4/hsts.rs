use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type hash_table;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn rpl_free(_: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn __getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn rpl_fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
    fn rpl_fseek(fp: *mut FILE, offset: i64, whence: i32) -> i32;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn xstrdup_lower(_: *const i8) -> *mut i8;
    fn file_exists_p(_: *const i8, _: *mut file_stats_t) -> bool;
    fn fopen_stat(_: *const i8, _: *const i8, _: *mut file_stats_t) -> *mut FILE;
    fn is_valid_ip_address(name: *const i8) -> bool;
    fn hash_table_new(
        _: i32,
        _: Option<unsafe extern "C" fn(*const libc::c_void) -> u64>,
        _: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>,
    ) -> *mut hash_table;
    fn hash_table_destroy(_: *mut hash_table);
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_contains(_: *const hash_table, _: *const libc::c_void) -> i32;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_remove(_: *mut hash_table, _: *const libc::c_void) -> i32;
    fn hash_table_iterate(_: *mut hash_table, _: *mut hash_table_iterator);
    fn hash_table_iter_next(_: *mut hash_table_iterator) -> i32;
    fn hash_table_count(_: *const hash_table) -> i32;
    fn ftruncate(__fd: i32, __length: __off_t) -> i32;
    fn flock(__fd: i32, __operation: i32) -> i32;
}
pub type __int64_t = i64;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type time_t = __time_t;
pub type size_t = u64;
pub type int64_t = __int64_t;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum log_options {
    LOG_VERBOSE,
    LOG_NOTQUIET,
    LOG_NONVERBOSE,
    LOG_ALWAYS,
    LOG_PROGRESS,
}
impl log_options {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            log_options::LOG_VERBOSE => 0,
            log_options::LOG_NOTQUIET => 1,
            log_options::LOG_NONVERBOSE => 2,
            log_options::LOG_ALWAYS => 3,
            log_options::LOG_PROGRESS => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> log_options {
        match value {
            0 => log_options::LOG_VERBOSE,
            1 => log_options::LOG_NOTQUIET,
            2 => log_options::LOG_NONVERBOSE,
            3 => log_options::LOG_ALWAYS,
            4 => log_options::LOG_PROGRESS,
            _ => panic!("Invalid value for log_options: {}", value),
        }
    }
}
impl AddAssign<u32> for log_options {
    fn add_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for log_options {
    fn sub_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for log_options {
    fn mul_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for log_options {
    fn div_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for log_options {
    fn rem_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for log_options {
    type Output = log_options;
    fn add(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for log_options {
    type Output = log_options;
    fn sub(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for log_options {
    type Output = log_options;
    fn mul(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for log_options {
    type Output = log_options;
    fn div(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for log_options {
    type Output = log_options;
    fn rem(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum url_scheme {
    SCHEME_HTTP,
    SCHEME_HTTPS,
    SCHEME_FTP,
    SCHEME_FTPS,
    SCHEME_INVALID,
}
impl url_scheme {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            url_scheme::SCHEME_HTTP => 0,
            url_scheme::SCHEME_HTTPS => 1,
            url_scheme::SCHEME_FTP => 2,
            url_scheme::SCHEME_FTPS => 3,
            url_scheme::SCHEME_INVALID => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> url_scheme {
        match value {
            0 => url_scheme::SCHEME_HTTP,
            1 => url_scheme::SCHEME_HTTPS,
            2 => url_scheme::SCHEME_FTP,
            3 => url_scheme::SCHEME_FTPS,
            4 => url_scheme::SCHEME_INVALID,
            _ => panic!("Invalid value for url_scheme: {}", value),
        }
    }
}
impl AddAssign<u32> for url_scheme {
    fn add_assign(&mut self, rhs: u32) {
        *self = url_scheme::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for url_scheme {
    fn sub_assign(&mut self, rhs: u32) {
        *self = url_scheme::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for url_scheme {
    fn mul_assign(&mut self, rhs: u32) {
        *self = url_scheme::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for url_scheme {
    fn div_assign(&mut self, rhs: u32) {
        *self = url_scheme::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for url_scheme {
    fn rem_assign(&mut self, rhs: u32) {
        *self = url_scheme::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for url_scheme {
    type Output = url_scheme;
    fn add(self, rhs: u32) -> url_scheme {
        url_scheme::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for url_scheme {
    type Output = url_scheme;
    fn sub(self, rhs: u32) -> url_scheme {
        url_scheme::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for url_scheme {
    type Output = url_scheme;
    fn mul(self, rhs: u32) -> url_scheme {
        url_scheme::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for url_scheme {
    type Output = url_scheme;
    fn div(self, rhs: u32) -> url_scheme {
        url_scheme::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for url_scheme {
    type Output = url_scheme;
    fn rem(self, rhs: u32) -> url_scheme {
        url_scheme::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct url {
    pub url: *mut i8,
    pub scheme: url_scheme,
    pub host: *mut i8,
    pub port: i32,
    pub path: *mut i8,
    pub params: *mut i8,
    pub query: *mut i8,
    pub fragment: *mut i8,
    pub dir: *mut i8,
    pub file: *mut i8,
    pub user: *mut i8,
    pub passwd: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hsts_store {
    pub table: *mut hash_table,
    pub last_mtime: time_t,
    pub changed: bool,
}
pub type hsts_store_t = *mut hsts_store;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table_iterator {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub pos: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hsts_kh {
    pub host: *mut i8,
    pub explicit_port: i32,
}
pub type file_stats_t = file_stat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stat_s {
    pub access_err: i32,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hsts_kh_info {
    pub created: int64_t,
    pub max_age: int64_t,
    pub include_subdomains: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum hsts_kh_match {
    NO_MATCH,
    SUPERDOMAIN_MATCH,
    CONGRUENT_MATCH,
}
impl hsts_kh_match {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            hsts_kh_match::NO_MATCH => 0,
            hsts_kh_match::SUPERDOMAIN_MATCH => 1,
            hsts_kh_match::CONGRUENT_MATCH => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> hsts_kh_match {
        match value {
            0 => hsts_kh_match::NO_MATCH,
            1 => hsts_kh_match::SUPERDOMAIN_MATCH,
            2 => hsts_kh_match::CONGRUENT_MATCH,
            _ => panic!("Invalid value for hsts_kh_match: {}", value),
        }
    }
}
impl AddAssign<u32> for hsts_kh_match {
    fn add_assign(&mut self, rhs: u32) {
        *self = hsts_kh_match::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for hsts_kh_match {
    fn sub_assign(&mut self, rhs: u32) {
        *self = hsts_kh_match::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for hsts_kh_match {
    fn mul_assign(&mut self, rhs: u32) {
        *self = hsts_kh_match::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for hsts_kh_match {
    fn div_assign(&mut self, rhs: u32) {
        *self = hsts_kh_match::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for hsts_kh_match {
    fn rem_assign(&mut self, rhs: u32) {
        *self = hsts_kh_match::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for hsts_kh_match {
    type Output = hsts_kh_match;
    fn add(self, rhs: u32) -> hsts_kh_match {
        hsts_kh_match::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for hsts_kh_match {
    type Output = hsts_kh_match;
    fn sub(self, rhs: u32) -> hsts_kh_match {
        hsts_kh_match::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for hsts_kh_match {
    type Output = hsts_kh_match;
    fn mul(self, rhs: u32) -> hsts_kh_match {
        hsts_kh_match::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for hsts_kh_match {
    type Output = hsts_kh_match;
    fn div(self, rhs: u32) -> hsts_kh_match {
        hsts_kh_match::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for hsts_kh_match {
    type Output = hsts_kh_match;
    fn rem(self, rhs: u32) -> hsts_kh_match {
        hsts_kh_match::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: i32) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut i8,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
unsafe extern "C" fn hsts_hash_func(mut key: *const libc::c_void) -> u64 {
    let mut k: *mut hsts_kh = key as *mut hsts_kh;
    let mut h: *const i8 = 0 as *const i8;
    let mut hash: u32 = (*k).explicit_port as u32;
    h = (*k).host;
    while *h != 0 {
        hash = hash.wrapping_mul(31 as i32 as u32).wrapping_add(*h as u32);
        h = h.offset(1);
        h;
    }
    return hash as u64;
}
unsafe extern "C" fn hsts_cmp_func(
    mut h1: *const libc::c_void,
    mut h2: *const libc::c_void,
) -> i32 {
    let mut kh1: *mut hsts_kh = h1 as *mut hsts_kh;
    let mut kh2: *mut hsts_kh = h2 as *mut hsts_kh;
    return (strcmp((*kh1).host, (*kh2).host) == 0
        && (*kh1).explicit_port == (*kh2).explicit_port) as i32;
}
unsafe extern "C" fn hsts_find_entry(
    mut store: hsts_store_t,
    mut host: *const i8,
    mut explicit_port: i32,
    mut match_type: *mut hsts_kh_match,
    mut kh: *mut hsts_kh,
) -> *mut hsts_kh_info {
    let mut k: *mut hsts_kh = 0 as *mut hsts_kh;
    let mut khi: *mut hsts_kh_info = 0 as *mut hsts_kh_info;
    let mut match_0: hsts_kh_match = hsts_kh_match::NO_MATCH;
    let mut pos: *mut i8 = 0 as *mut i8;
    let mut org_ptr: *mut i8 = 0 as *mut i8;
    k = xmalloc(::core::mem::size_of::<hsts_kh>() as u64) as *mut hsts_kh;
    (*k).host = xstrdup_lower(host);
    (*k).explicit_port = explicit_port;
    org_ptr = (*k).host;
    khi = hash_table_get((*store).table, k as *const libc::c_void) as *mut hsts_kh_info;
    if !khi.is_null() {
        match_0 = hsts_kh_match::CONGRUENT_MATCH;
    } else {
        while match_0 as u32 == hsts_kh_match::NO_MATCH as i32 as u32
            && {
                pos = strchr((*k).host, '.' as i32);
                !pos.is_null()
            } && pos.offset_from((*k).host) as i64 > 0 as i32 as i64
            && !(strchr(pos.offset(1 as i32 as isize), '.' as i32)).is_null()
        {
            (*k).host = ((*k).host)
                .offset((pos.offset_from((*k).host) as i64 + 1 as i32 as i64) as isize);
            khi = hash_table_get((*store).table, k as *const libc::c_void)
                as *mut hsts_kh_info;
            if !khi.is_null() {
                match_0 = hsts_kh_match::SUPERDOMAIN_MATCH;
            }
        }
    }
    (*k).host = org_ptr;
    if !match_type.is_null() {
        *match_type = match_0;
    }
    if !kh.is_null() {
        memcpy(
            kh as *mut libc::c_void,
            k as *const libc::c_void,
            ::core::mem::size_of::<hsts_kh>() as u64,
        );
    } else {
        rpl_free((*k).host as *mut libc::c_void);
        (*k).host = 0 as *mut i8;
    }
    rpl_free(k as *mut libc::c_void);
    k = 0 as *mut hsts_kh;
    return khi;
}
unsafe extern "C" fn hsts_new_entry_internal(
    mut store: hsts_store_t,
    mut host: *const i8,
    mut port: i32,
    mut created: int64_t,
    mut max_age: int64_t,
    mut include_subdomains: bool,
    mut check_validity: bool,
    mut check_expired: bool,
    mut check_duplicates: bool,
) -> bool {
    let mut kh: *mut hsts_kh = xmalloc(::core::mem::size_of::<hsts_kh>() as u64)
        as *mut hsts_kh;
    let mut khi: *mut hsts_kh_info = xcalloc(
        1 as i32 as size_t,
        ::core::mem::size_of::<hsts_kh_info>() as u64,
    ) as *mut hsts_kh_info;
    let mut success: bool = 0 as i32 != 0;
    (*kh).host = xstrdup_lower(host);
    (*kh).explicit_port = if url_scheme::SCHEME_HTTPS as i32
        == url_scheme::SCHEME_HTTPS as i32
    {
        if port == 443 as i32 { 0 as i32 } else { port }
    } else if port == 80 as i32 {
        0 as i32
    } else {
        port
    };
    (*khi).created = created;
    (*khi).max_age = max_age;
    (*khi).include_subdomains = include_subdomains;
    if !(check_validity as i32 != 0 && is_valid_ip_address(host)) {
        if !(check_expired as i32 != 0
            && (*khi).created + (*khi).max_age < (*khi).created)
        {
            if !(check_duplicates as i32 != 0
                && hash_table_contains((*store).table, kh as *const libc::c_void) != 0)
            {
                hash_table_put(
                    (*store).table,
                    kh as *const libc::c_void,
                    khi as *const libc::c_void,
                );
                success = 1 as i32 != 0;
            }
        }
    }
    if !success {
        rpl_free((*kh).host as *mut libc::c_void);
        (*kh).host = 0 as *mut i8;
        rpl_free(kh as *mut libc::c_void);
        kh = 0 as *mut hsts_kh;
        rpl_free(khi as *mut libc::c_void);
        khi = 0 as *mut hsts_kh_info;
    }
    return success;
}
unsafe extern "C" fn hsts_add_entry(
    mut store: hsts_store_t,
    mut host: *const i8,
    mut port: i32,
    mut max_age: int64_t,
    mut include_subdomains: bool,
) -> bool {
    let mut t: int64_t = time(0 as *mut time_t);
    return if t == -(1 as i32) as i64 {
        0 as i32
    } else {
        hsts_new_entry_internal(
            store,
            host,
            port,
            t,
            max_age,
            include_subdomains,
            0 as i32 != 0,
            1 as i32 != 0,
            0 as i32 != 0,
        ) as i32
    } != 0;
}
unsafe extern "C" fn hsts_new_entry(
    mut store: hsts_store_t,
    mut host: *const i8,
    mut port: i32,
    mut created: int64_t,
    mut max_age: int64_t,
    mut include_subdomains: bool,
) -> bool {
    return hsts_new_entry_internal(
        store,
        host,
        port,
        created,
        max_age,
        include_subdomains,
        1 as i32 != 0,
        1 as i32 != 0,
        1 as i32 != 0,
    );
}
unsafe extern "C" fn hsts_remove_entry(mut store: hsts_store_t, mut kh: *mut hsts_kh) {
    hash_table_remove((*store).table, kh as *const libc::c_void);
}
unsafe extern "C" fn hsts_store_merge(
    mut store: hsts_store_t,
    mut host: *const i8,
    mut port: i32,
    mut created: int64_t,
    mut max_age: int64_t,
    mut include_subdomains: bool,
) -> bool {
    let mut match_type: hsts_kh_match = hsts_kh_match::NO_MATCH;
    let mut khi: *mut hsts_kh_info = 0 as *mut hsts_kh_info;
    let mut success: bool = 0 as i32 != 0;
    port = if url_scheme::SCHEME_HTTPS as i32 == url_scheme::SCHEME_HTTPS as i32 {
        if port == 443 as i32 { 0 as i32 } else { port }
    } else if port == 80 as i32 {
        0 as i32
    } else {
        port
    };
    khi = hsts_find_entry(store, host, port, &mut match_type, 0 as *mut hsts_kh);
    if !khi.is_null()
        && match_type as u32 == hsts_kh_match::CONGRUENT_MATCH as i32 as u32
        && created > (*khi).created
    {
        (*khi).created = created;
        (*khi).max_age = max_age;
        (*khi).include_subdomains = include_subdomains;
        success = 1 as i32 != 0;
    } else if khi.is_null() {
        success = hsts_new_entry(
            store,
            host,
            port,
            created,
            max_age,
            include_subdomains,
        );
    }
    return success;
}
unsafe extern "C" fn hsts_read_database(
    mut store: hsts_store_t,
    mut fp: *mut FILE,
    mut merge_with_existing_entries: bool,
) -> bool {
    let mut line: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0 as i32 as size_t;
    let mut items_read: i32 = 0;
    let mut result: bool = 0 as i32 != 0;
    let mut func: Option<
        unsafe extern "C" fn(
            hsts_store_t,
            *const i8,
            i32,
            int64_t,
            int64_t,
            bool,
        ) -> bool,
    > = None;
    let mut host: [i8; 256] = [0; 256];
    let mut port: i32 = 0;
    let mut created: int64_t = 0;
    let mut max_age: int64_t = 0;
    let mut include_subdomains: i32 = 0;
    func = if merge_with_existing_entries as i32 != 0 {
        Some(
            hsts_store_merge
                as unsafe extern "C" fn(
                    hsts_store_t,
                    *const i8,
                    i32,
                    int64_t,
                    int64_t,
                    bool,
                ) -> bool,
        )
    } else {
        Some(
            hsts_new_entry
                as unsafe extern "C" fn(
                    hsts_store_t,
                    *const i8,
                    i32,
                    int64_t,
                    int64_t,
                    bool,
                ) -> bool,
        )
    };
    while getline(&mut line, &mut len, fp) > 0 as i32 as i64 {
        p = line;
        while c_isspace(*p as i32) {
            p = p.offset(1);
            p;
        }
        if *p as i32 == '#' as i32 {
            continue;
        }
        items_read = sscanf(
            p,
            b"%255s %d %d %ld %ld\0" as *const u8 as *const i8,
            host.as_mut_ptr(),
            &mut port as *mut i32,
            &mut include_subdomains as *mut i32,
            &mut created as *mut int64_t,
            &mut max_age as *mut int64_t,
        );
        if items_read == 5 as i32 {
            func
                .expect(
                    "non-null function pointer",
                )(
                store,
                host.as_mut_ptr(),
                port,
                created,
                max_age,
                include_subdomains != 0,
            );
        }
    }
    rpl_free(line as *mut libc::c_void);
    line = 0 as *mut i8;
    result = 1 as i32 != 0;
    return result;
}
unsafe extern "C" fn hsts_store_dump(mut store: hsts_store_t, mut fp: *mut FILE) {
    let mut it: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    fputs(
        b"# HSTS 1.0 Known Hosts database for GNU Wget.\n\0" as *const u8 as *const i8,
        fp,
    );
    fputs(b"# Edit at your own risk.\n\0" as *const u8 as *const i8, fp);
    fputs(
        b"# <hostname>\t<port>\t<incl. subdomains>\t<created>\t<max-age>\n\0"
            as *const u8 as *const i8,
        fp,
    );
    hash_table_iterate((*store).table, &mut it);
    while hash_table_iter_next(&mut it) != 0 {
        let mut kh: *mut hsts_kh = it.key as *mut hsts_kh;
        let mut khi: *mut hsts_kh_info = it.value as *mut hsts_kh_info;
        if !(fprintf(
            fp,
            b"%s\t%d\t%d\t%ld\t%ld\n\0" as *const u8 as *const i8,
            (*kh).host,
            (*kh).explicit_port,
            (*khi).include_subdomains as i32,
            (*khi).created,
            (*khi).max_age,
        ) < 0 as i32)
        {
            continue;
        }
        logprintf(
            log_options::LOG_ALWAYS,
            b"Could not write the HSTS database correctly.\n\0" as *const u8 as *const i8,
        );
        break;
    }
}
unsafe extern "C" fn hsts_file_access_valid(mut filename: *const i8) -> bool {
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
    if stat(filename, &mut st) == -(1 as i32) {
        return 0 as i32 != 0;
    }
    return st.st_mode & (0o200 as i32 >> 3 as i32 >> 3 as i32) as u32 == 0
        && st.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn hsts_match(mut store: hsts_store_t, mut u: *mut url) -> bool {
    let mut url_changed: bool = 0 as i32 != 0;
    let mut entry: *mut hsts_kh_info = 0 as *mut hsts_kh_info;
    let mut kh: *mut hsts_kh = xmalloc(::core::mem::size_of::<hsts_kh>() as u64)
        as *mut hsts_kh;
    let mut match_0: hsts_kh_match = hsts_kh_match::NO_MATCH;
    let mut port: i32 = if (*u).scheme as u32 == url_scheme::SCHEME_HTTPS as i32 as u32 {
        if (*u).port == 443 as i32 { 0 as i32 } else { (*u).port }
    } else if (*u).port == 80 as i32 {
        0 as i32
    } else {
        (*u).port
    };
    if !((*u).scheme as u32 == url_scheme::SCHEME_HTTPS as i32 as u32) {
        entry = hsts_find_entry(store, (*u).host, port, &mut match_0, kh);
        if !entry.is_null() {
            if (*entry).created + (*entry).max_age >= time(0 as *mut time_t) {
                if match_0 as u32 == hsts_kh_match::CONGRUENT_MATCH as i32 as u32
                    || match_0 as u32 == hsts_kh_match::SUPERDOMAIN_MATCH as i32 as u32
                        && (*entry).include_subdomains as i32 != 0
                {
                    (*u).scheme = url_scheme::SCHEME_HTTPS;
                    if (*u).port == 80 as i32 {
                        (*u).port = 443 as i32;
                    }
                    url_changed = 1 as i32 != 0;
                    (*store).changed = 1 as i32 != 0;
                }
            } else {
                hsts_remove_entry(store, kh);
                (*store).changed = 1 as i32 != 0;
            }
        }
        rpl_free((*kh).host as *mut libc::c_void);
        (*kh).host = 0 as *mut i8;
    }
    rpl_free(kh as *mut libc::c_void);
    kh = 0 as *mut hsts_kh;
    return url_changed;
}
#[no_mangle]
pub unsafe extern "C" fn hsts_store_entry(
    mut store: hsts_store_t,
    mut scheme: url_scheme,
    mut host: *const i8,
    mut port: i32,
    mut max_age: int64_t,
    mut include_subdomains: bool,
) -> bool {
    let mut result: bool = 0 as i32 != 0;
    let mut match_0: hsts_kh_match = hsts_kh_match::NO_MATCH;
    let mut kh: *mut hsts_kh = xmalloc(::core::mem::size_of::<hsts_kh>() as u64)
        as *mut hsts_kh;
    let mut entry: *mut hsts_kh_info = 0 as *mut hsts_kh_info;
    if scheme as u32 == url_scheme::SCHEME_HTTPS as i32 as u32
        && !is_valid_ip_address(host)
    {
        port = if scheme as u32 == url_scheme::SCHEME_HTTPS as i32 as u32 {
            if port == 443 as i32 { 0 as i32 } else { port }
        } else if port == 80 as i32 {
            0 as i32
        } else {
            port
        };
        entry = hsts_find_entry(store, host, port, &mut match_0, kh);
        if !entry.is_null()
            && match_0 as u32 == hsts_kh_match::CONGRUENT_MATCH as i32 as u32
        {
            if max_age == 0 as i32 as i64 {
                hsts_remove_entry(store, kh);
                (*store).changed = 1 as i32 != 0;
            } else if max_age > 0 as i32 as i64 {
                let mut t: int64_t = time(0 as *mut time_t);
                if t != -(1 as i32) as i64 && t != (*entry).created {
                    (*entry).created = t;
                    (*entry).max_age = max_age;
                    (*entry).include_subdomains = include_subdomains;
                    (*store).changed = 1 as i32 != 0;
                }
            }
        } else if entry.is_null()
            || match_0 as u32 == hsts_kh_match::SUPERDOMAIN_MATCH as i32 as u32
        {
            result = hsts_add_entry(store, host, port, max_age, include_subdomains);
            if result {
                (*store).changed = 1 as i32 != 0;
            }
        }
        rpl_free((*kh).host as *mut libc::c_void);
        (*kh).host = 0 as *mut i8;
    }
    rpl_free(kh as *mut libc::c_void);
    kh = 0 as *mut hsts_kh;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn hsts_store_open(mut filename: *const i8) -> hsts_store_t {
    let mut store: hsts_store_t = 0 as hsts_store_t;
    let mut fstats: file_stats_t = file_stats_t {
        access_err: 0,
        st_ino: 0,
        st_dev: 0,
    };
    store = xcalloc(1 as i32 as size_t, ::core::mem::size_of::<hsts_store>() as u64)
        as hsts_store_t;
    (*store).table = hash_table_new(
        0 as i32,
        Some(hsts_hash_func as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(
            hsts_cmp_func
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    (*store).last_mtime = 0 as i32 as time_t;
    (*store).changed = 0 as i32 != 0;
    if file_exists_p(filename, &mut fstats) {
        if hsts_file_access_valid(filename) {
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
            let mut fp: *mut FILE = fopen_stat(
                filename,
                b"r\0" as *const u8 as *const i8,
                &mut fstats,
            );
            if fp.is_null() || !hsts_read_database(store, fp, 0 as i32 != 0) {
                hsts_store_close(store);
                rpl_free(store as *mut libc::c_void);
                store = 0 as hsts_store_t;
                if !fp.is_null() {
                    fclose(fp);
                }
            } else {
                if fstat(fileno(fp), &mut st) == 0 as i32 {
                    (*store).last_mtime = st.st_mtim.tv_sec;
                }
                fclose(fp);
            }
        } else {
            hsts_store_close(store);
            rpl_free(store as *mut libc::c_void);
            store = 0 as hsts_store_t;
            logprintf(
                log_options::LOG_NOTQUIET,
                b"Will not apply HSTS. The HSTS database must be a regular and non-world-writable file.\n\0"
                    as *const u8 as *const i8,
            );
        }
    }
    return store;
}
#[no_mangle]
pub unsafe extern "C" fn hsts_store_save(
    mut store: hsts_store_t,
    mut filename: *const i8,
) {
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
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fd: i32 = 0 as i32;
    if !filename.is_null() && hash_table_count((*store).table) > 0 as i32 {
        fp = rpl_fopen(filename, b"a+\0" as *const u8 as *const i8);
        if !fp.is_null() {
            fd = fileno(fp);
            flock(fd, 2 as i32);
            if (*store).last_mtime != 0 && stat(filename, &mut st) == 0 as i32
                && st.st_mtim.tv_sec > (*store).last_mtime
            {
                hsts_read_database(store, fp, 1 as i32 != 0);
            }
            rpl_fseek(fp, 0 as i32 as i64, 0 as i32);
            ftruncate(fd, 0 as i32 as __off_t);
            hsts_store_dump(store, fp);
            fclose(fp);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn hsts_store_has_changed(mut store: hsts_store_t) -> bool {
    return if !store.is_null() { (*store).changed as i32 } else { 0 as i32 } != 0;
}
#[no_mangle]
pub unsafe extern "C" fn hsts_store_close(mut store: hsts_store_t) {
    let mut it: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    hash_table_iterate((*store).table, &mut it);
    while hash_table_iter_next(&mut it) != 0 {
        rpl_free((*(it.key as *mut hsts_kh)).host as *mut libc::c_void);
        let ref mut fresh0 = (*(it.key as *mut hsts_kh)).host;
        *fresh0 = 0 as *mut i8;
        rpl_free(it.key);
        it.key = 0 as *mut libc::c_void;
        rpl_free(it.value);
        it.value = 0 as *mut libc::c_void;
    }
    hash_table_destroy((*store).table);
}