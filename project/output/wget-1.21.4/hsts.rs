#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type hash_table;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn rpl_fseek(
        fp: *mut FILE,
        offset: libc::c_long,
        whence: libc::c_int,
    ) -> libc::c_int;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn xstrdup_lower(_: *const libc::c_char) -> *mut libc::c_char;
    fn file_exists_p(_: *const libc::c_char, _: *mut file_stats_t) -> bool;
    fn fopen_stat(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut file_stats_t,
    ) -> *mut FILE;
    fn is_valid_ip_address(name: *const libc::c_char) -> bool;
    fn hash_table_new(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong>,
        _: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    ) -> *mut hash_table;
    fn hash_table_destroy(_: *mut hash_table);
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_contains(_: *const hash_table, _: *const libc::c_void) -> libc::c_int;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_remove(_: *mut hash_table, _: *const libc::c_void) -> libc::c_int;
    fn hash_table_iterate(_: *mut hash_table, _: *mut hash_table_iterator);
    fn hash_table_iter_next(_: *mut hash_table_iterator) -> libc::c_int;
    fn hash_table_count(_: *const hash_table) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
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
    pub __pad0: libc::c_int,
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
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
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
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum url_scheme {
    SCHEME_HTTP,
    SCHEME_HTTPS,
    SCHEME_FTP,
    SCHEME_FTPS,
    SCHEME_INVALID,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct url {
    pub url: *mut libc::c_char,
    pub scheme: url_scheme,
    pub host: *mut libc::c_char,
    pub port: libc::c_int,
    pub path: *mut libc::c_char,
    pub params: *mut libc::c_char,
    pub query: *mut libc::c_char,
    pub fragment: *mut libc::c_char,
    pub dir: *mut libc::c_char,
    pub file: *mut libc::c_char,
    pub user: *mut libc::c_char,
    pub passwd: *mut libc::c_char,
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
    pub host: *mut libc::c_char,
    pub explicit_port: libc::c_int,
}
pub type file_stats_t = file_stat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stat_s {
    pub access_err: libc::c_int,
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
pub const CONGRUENT_MATCH: hsts_kh_match = 2;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum hsts_kh_match {
    NO_MATCH,
    SUPERDOMAIN_MATCH,
    CONGRUENT_MATCH,
}  // end of enum

#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
unsafe extern "C" fn hsts_hash_func(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut k: *mut hsts_kh = key as *mut hsts_kh;
    let mut h: *const libc::c_char = 0 as *const libc::c_char;
    let mut hash: libc::c_uint = (*k).explicit_port as libc::c_uint;
    h = (*k).host;
    while *h != 0 {
        hash = hash
            .wrapping_mul(31 as libc::c_int as libc::c_uint)
            .wrapping_add(*h as libc::c_uint);
        h = h.offset(1);
        h;
    }
    return hash as libc::c_ulong;
}
unsafe extern "C" fn hsts_cmp_func(
    mut h1: *const libc::c_void,
    mut h2: *const libc::c_void,
) -> libc::c_int {
    let mut kh1: *mut hsts_kh = h1 as *mut hsts_kh;
    let mut kh2: *mut hsts_kh = h2 as *mut hsts_kh;
    return (strcmp((*kh1).host, (*kh2).host) == 0
        && (*kh1).explicit_port == (*kh2).explicit_port) as libc::c_int;
}
unsafe extern "C" fn hsts_find_entry(
    mut store: hsts_store_t,
    mut host: *const libc::c_char,
    mut explicit_port: libc::c_int,
    mut match_type: *mut hsts_kh_match,
    mut kh: *mut hsts_kh,
) -> *mut hsts_kh_info {
    let mut k: *mut hsts_kh = 0 as *mut hsts_kh;
    let mut khi: *mut hsts_kh_info = 0 as *mut hsts_kh_info;
    let mut match_0: hsts_kh_match = NO_MATCH;
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut org_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    k = xmalloc(::core::mem::size_of::<hsts_kh>() as libc::c_ulong) as *mut hsts_kh;
    (*k).host = xstrdup_lower(host);
    (*k).explicit_port = explicit_port;
    org_ptr = (*k).host;
    khi = hash_table_get((*store).table, k as *const libc::c_void) as *mut hsts_kh_info;
    if !khi.is_null() {
        match_0 = CONGRUENT_MATCH;
    } else {
        while match_0 as libc::c_uint == NO_MATCH as libc::c_int as libc::c_uint
            && {
                pos = strchr((*k).host, '.' as i32);
                !pos.is_null()
            }
            && pos.offset_from((*k).host) as libc::c_long
                > 0 as libc::c_int as libc::c_long
            && !(strchr(pos.offset(1 as libc::c_int as isize), '.' as i32)).is_null()
        {
            (*k)
                .host = ((*k).host)
                .offset(
                    (pos.offset_from((*k).host) as libc::c_long
                        + 1 as libc::c_int as libc::c_long) as isize,
                );
            khi = hash_table_get((*store).table, k as *const libc::c_void)
                as *mut hsts_kh_info;
            if !khi.is_null() {
                match_0 = SUPERDOMAIN_MATCH;
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
            ::core::mem::size_of::<hsts_kh>() as libc::c_ulong,
        );
    } else {
        rpl_free((*k).host as *mut libc::c_void);
        (*k).host = 0 as *mut libc::c_char;
    }
    rpl_free(k as *mut libc::c_void);
    k = 0 as *mut hsts_kh;
    return khi;
}
unsafe extern "C" fn hsts_new_entry_internal(
    mut store: hsts_store_t,
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut created: int64_t,
    mut max_age: int64_t,
    mut include_subdomains: bool,
    mut check_validity: bool,
    mut check_expired: bool,
    mut check_duplicates: bool,
) -> bool {
    let mut kh: *mut hsts_kh = xmalloc(
        ::core::mem::size_of::<hsts_kh>() as libc::c_ulong,
    ) as *mut hsts_kh;
    let mut khi: *mut hsts_kh_info = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<hsts_kh_info>() as libc::c_ulong,
    ) as *mut hsts_kh_info;
    let mut success: bool = 0 as libc::c_int != 0;
    (*kh).host = xstrdup_lower(host);
    (*kh)
        .explicit_port = if SCHEME_HTTPS as libc::c_int == SCHEME_HTTPS as libc::c_int {
        if port == 443 as libc::c_int { 0 as libc::c_int } else { port }
    } else if port == 80 as libc::c_int {
        0 as libc::c_int
    } else {
        port
    };
    (*khi).created = created;
    (*khi).max_age = max_age;
    (*khi).include_subdomains = include_subdomains;
    if !(check_validity as libc::c_int != 0 && is_valid_ip_address(host)) {
        if !(check_expired as libc::c_int != 0
            && (*khi).created + (*khi).max_age < (*khi).created)
        {
            if !(check_duplicates as libc::c_int != 0
                && hash_table_contains((*store).table, kh as *const libc::c_void) != 0)
            {
                hash_table_put(
                    (*store).table,
                    kh as *const libc::c_void,
                    khi as *const libc::c_void,
                );
                success = 1 as libc::c_int != 0;
            }
        }
    }
    if !success {
        rpl_free((*kh).host as *mut libc::c_void);
        (*kh).host = 0 as *mut libc::c_char;
        rpl_free(kh as *mut libc::c_void);
        kh = 0 as *mut hsts_kh;
        rpl_free(khi as *mut libc::c_void);
        khi = 0 as *mut hsts_kh_info;
    }
    return success;
}
unsafe extern "C" fn hsts_add_entry(
    mut store: hsts_store_t,
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut max_age: int64_t,
    mut include_subdomains: bool,
) -> bool {
    let mut t: int64_t = time(0 as *mut time_t);
    return if t == -(1 as libc::c_int) as libc::c_long {
        0 as libc::c_int
    } else {
        hsts_new_entry_internal(
            store,
            host,
            port,
            t,
            max_age,
            include_subdomains,
            0 as libc::c_int != 0,
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        ) as libc::c_int
    } != 0;
}
unsafe extern "C" fn hsts_new_entry(
    mut store: hsts_store_t,
    mut host: *const libc::c_char,
    mut port: libc::c_int,
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
        1 as libc::c_int != 0,
        1 as libc::c_int != 0,
        1 as libc::c_int != 0,
    );
}
unsafe extern "C" fn hsts_remove_entry(mut store: hsts_store_t, mut kh: *mut hsts_kh) {
    hash_table_remove((*store).table, kh as *const libc::c_void);
}
unsafe extern "C" fn hsts_store_merge(
    mut store: hsts_store_t,
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut created: int64_t,
    mut max_age: int64_t,
    mut include_subdomains: bool,
) -> bool {
    let mut match_type: hsts_kh_match = NO_MATCH;
    let mut khi: *mut hsts_kh_info = 0 as *mut hsts_kh_info;
    let mut success: bool = 0 as libc::c_int != 0;
    port = if SCHEME_HTTPS as libc::c_int == SCHEME_HTTPS as libc::c_int {
        if port == 443 as libc::c_int { 0 as libc::c_int } else { port }
    } else if port == 80 as libc::c_int {
        0 as libc::c_int
    } else {
        port
    };
    khi = hsts_find_entry(store, host, port, &mut match_type, 0 as *mut hsts_kh);
    if !khi.is_null()
        && match_type as libc::c_uint == CONGRUENT_MATCH as libc::c_int as libc::c_uint
        && created > (*khi).created
    {
        (*khi).created = created;
        (*khi).max_age = max_age;
        (*khi).include_subdomains = include_subdomains;
        success = 1 as libc::c_int != 0;
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
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut items_read: libc::c_int = 0;
    let mut result: bool = 0 as libc::c_int != 0;
    let mut func: Option::<
        unsafe extern "C" fn(
            hsts_store_t,
            *const libc::c_char,
            libc::c_int,
            int64_t,
            int64_t,
            bool,
        ) -> bool,
    > = None;
    let mut host: [libc::c_char; 256] = [0; 256];
    let mut port: libc::c_int = 0;
    let mut created: int64_t = 0;
    let mut max_age: int64_t = 0;
    let mut include_subdomains: libc::c_int = 0;
    func = if merge_with_existing_entries as libc::c_int != 0 {
        Some(
            hsts_store_merge
                as unsafe extern "C" fn(
                    hsts_store_t,
                    *const libc::c_char,
                    libc::c_int,
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
                    *const libc::c_char,
                    libc::c_int,
                    int64_t,
                    int64_t,
                    bool,
                ) -> bool,
        )
    };
    while getline(&mut line, &mut len, fp) > 0 as libc::c_int as libc::c_long {
        p = line;
        while c_isspace(*p as libc::c_int) {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '#' as i32 {
            continue;
        }
        items_read = sscanf(
            p,
            b"%255s %d %d %ld %ld\0" as *const u8 as *const libc::c_char,
            host.as_mut_ptr(),
            &mut port as *mut libc::c_int,
            &mut include_subdomains as *mut libc::c_int,
            &mut created as *mut int64_t,
            &mut max_age as *mut int64_t,
        );
        if items_read == 5 as libc::c_int {
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
    line = 0 as *mut libc::c_char;
    result = 1 as libc::c_int != 0;
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
        b"# HSTS 1.0 Known Hosts database for GNU Wget.\n\0" as *const u8
            as *const libc::c_char,
        fp,
    );
    fputs(b"# Edit at your own risk.\n\0" as *const u8 as *const libc::c_char, fp);
    fputs(
        b"# <hostname>\t<port>\t<incl. subdomains>\t<created>\t<max-age>\n\0"
            as *const u8 as *const libc::c_char,
        fp,
    );
    hash_table_iterate((*store).table, &mut it);
    while hash_table_iter_next(&mut it) != 0 {
        let mut kh: *mut hsts_kh = it.key as *mut hsts_kh;
        let mut khi: *mut hsts_kh_info = it.value as *mut hsts_kh_info;
        if !(fprintf(
            fp,
            b"%s\t%d\t%d\t%ld\t%ld\n\0" as *const u8 as *const libc::c_char,
            (*kh).host,
            (*kh).explicit_port,
            (*khi).include_subdomains as libc::c_int,
            (*khi).created,
            (*khi).max_age,
        ) < 0 as libc::c_int)
        {
            continue;
        }
        logprintf(
            LOG_ALWAYS,
            b"Could not write the HSTS database correctly.\n\0" as *const u8
                as *const libc::c_char,
        );
        break;
    }
}
unsafe extern "C" fn hsts_file_access_valid(mut filename: *const libc::c_char) -> bool {
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
    if stat(filename, &mut st) == -(1 as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    return st.st_mode
        & (0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        == 0
        && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn hsts_match(mut store: hsts_store_t, mut u: *mut url) -> bool {
    let mut url_changed: bool = 0 as libc::c_int != 0;
    let mut entry: *mut hsts_kh_info = 0 as *mut hsts_kh_info;
    let mut kh: *mut hsts_kh = xmalloc(
        ::core::mem::size_of::<hsts_kh>() as libc::c_ulong,
    ) as *mut hsts_kh;
    let mut match_0: hsts_kh_match = NO_MATCH;
    let mut port: libc::c_int = if (*u).scheme as libc::c_uint
        == SCHEME_HTTPS as libc::c_int as libc::c_uint
    {
        if (*u).port == 443 as libc::c_int { 0 as libc::c_int } else { (*u).port }
    } else if (*u).port == 80 as libc::c_int {
        0 as libc::c_int
    } else {
        (*u).port
    };
    if !((*u).scheme as libc::c_uint == SCHEME_HTTPS as libc::c_int as libc::c_uint) {
        entry = hsts_find_entry(store, (*u).host, port, &mut match_0, kh);
        if !entry.is_null() {
            if (*entry).created + (*entry).max_age >= time(0 as *mut time_t) {
                if match_0 as libc::c_uint
                    == CONGRUENT_MATCH as libc::c_int as libc::c_uint
                    || match_0 as libc::c_uint
                        == SUPERDOMAIN_MATCH as libc::c_int as libc::c_uint
                        && (*entry).include_subdomains as libc::c_int != 0
                {
                    (*u).scheme = SCHEME_HTTPS;
                    if (*u).port == 80 as libc::c_int {
                        (*u).port = 443 as libc::c_int;
                    }
                    url_changed = 1 as libc::c_int != 0;
                    (*store).changed = 1 as libc::c_int != 0;
                }
            } else {
                hsts_remove_entry(store, kh);
                (*store).changed = 1 as libc::c_int != 0;
            }
        }
        rpl_free((*kh).host as *mut libc::c_void);
        (*kh).host = 0 as *mut libc::c_char;
    }
    rpl_free(kh as *mut libc::c_void);
    kh = 0 as *mut hsts_kh;
    return url_changed;
}
#[no_mangle]
pub unsafe extern "C" fn hsts_store_entry(
    mut store: hsts_store_t,
    mut scheme: url_scheme,
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut max_age: int64_t,
    mut include_subdomains: bool,
) -> bool {
    let mut result: bool = 0 as libc::c_int != 0;
    let mut match_0: hsts_kh_match = NO_MATCH;
    let mut kh: *mut hsts_kh = xmalloc(
        ::core::mem::size_of::<hsts_kh>() as libc::c_ulong,
    ) as *mut hsts_kh;
    let mut entry: *mut hsts_kh_info = 0 as *mut hsts_kh_info;
    if scheme as libc::c_uint == SCHEME_HTTPS as libc::c_int as libc::c_uint
        && !is_valid_ip_address(host)
    {
        port = if scheme as libc::c_uint == SCHEME_HTTPS as libc::c_int as libc::c_uint {
            if port == 443 as libc::c_int { 0 as libc::c_int } else { port }
        } else if port == 80 as libc::c_int {
            0 as libc::c_int
        } else {
            port
        };
        entry = hsts_find_entry(store, host, port, &mut match_0, kh);
        if !entry.is_null()
            && match_0 as libc::c_uint == CONGRUENT_MATCH as libc::c_int as libc::c_uint
        {
            if max_age == 0 as libc::c_int as libc::c_long {
                hsts_remove_entry(store, kh);
                (*store).changed = 1 as libc::c_int != 0;
            } else if max_age > 0 as libc::c_int as libc::c_long {
                let mut t: int64_t = time(0 as *mut time_t);
                if t != -(1 as libc::c_int) as libc::c_long && t != (*entry).created {
                    (*entry).created = t;
                    (*entry).max_age = max_age;
                    (*entry).include_subdomains = include_subdomains;
                    (*store).changed = 1 as libc::c_int != 0;
                }
            }
        } else if entry.is_null()
            || match_0 as libc::c_uint
                == SUPERDOMAIN_MATCH as libc::c_int as libc::c_uint
        {
            result = hsts_add_entry(store, host, port, max_age, include_subdomains);
            if result {
                (*store).changed = 1 as libc::c_int != 0;
            }
        }
        rpl_free((*kh).host as *mut libc::c_void);
        (*kh).host = 0 as *mut libc::c_char;
    }
    rpl_free(kh as *mut libc::c_void);
    kh = 0 as *mut hsts_kh;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn hsts_store_open(
    mut filename: *const libc::c_char,
) -> hsts_store_t {
    let mut store: hsts_store_t = 0 as hsts_store_t;
    let mut fstats: file_stats_t = file_stats_t {
        access_err: 0,
        st_ino: 0,
        st_dev: 0,
    };
    store = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<hsts_store>() as libc::c_ulong,
    ) as hsts_store_t;
    (*store)
        .table = hash_table_new(
        0 as libc::c_int,
        Some(
            hsts_hash_func as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            hsts_cmp_func
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    (*store).last_mtime = 0 as libc::c_int as time_t;
    (*store).changed = 0 as libc::c_int != 0;
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
                b"r\0" as *const u8 as *const libc::c_char,
                &mut fstats,
            );
            if fp.is_null() || !hsts_read_database(store, fp, 0 as libc::c_int != 0) {
                hsts_store_close(store);
                rpl_free(store as *mut libc::c_void);
                store = 0 as hsts_store_t;
                if !fp.is_null() {
                    fclose(fp);
                }
            } else {
                if fstat(fileno(fp), &mut st) == 0 as libc::c_int {
                    (*store).last_mtime = st.st_mtim.tv_sec;
                }
                fclose(fp);
            }
        } else {
            hsts_store_close(store);
            rpl_free(store as *mut libc::c_void);
            store = 0 as hsts_store_t;
            logprintf(
                LOG_NOTQUIET,
                b"Will not apply HSTS. The HSTS database must be a regular and non-world-writable file.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    return store;
}
#[no_mangle]
pub unsafe extern "C" fn hsts_store_save(
    mut store: hsts_store_t,
    mut filename: *const libc::c_char,
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
    let mut fd: libc::c_int = 0 as libc::c_int;
    if !filename.is_null() && hash_table_count((*store).table) > 0 as libc::c_int {
        fp = rpl_fopen(filename, b"a+\0" as *const u8 as *const libc::c_char);
        if !fp.is_null() {
            fd = fileno(fp);
            flock(fd, 2 as libc::c_int);
            if (*store).last_mtime != 0 && stat(filename, &mut st) == 0 as libc::c_int
                && st.st_mtim.tv_sec > (*store).last_mtime
            {
                hsts_read_database(store, fp, 1 as libc::c_int != 0);
            }
            rpl_fseek(fp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
            ftruncate(fd, 0 as libc::c_int as __off_t);
            hsts_store_dump(store, fp);
            fclose(fp);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn hsts_store_has_changed(mut store: hsts_store_t) -> bool {
    return if !store.is_null() {
        (*store).changed as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
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
        *fresh0 = 0 as *mut libc::c_char;
        rpl_free(it.key);
        it.key = 0 as *mut libc::c_void;
        rpl_free(it.value);
        it.value = 0 as *mut libc::c_void;
    }
    hash_table_destroy((*store).table);
}
