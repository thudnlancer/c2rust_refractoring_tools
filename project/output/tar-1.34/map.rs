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
extern "C" {
    pub type hash_table;
    pub type wordsplit_node;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn __getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn __strtoul_internal(
        __nptr: *const i8,
        __endptr: *mut *mut i8,
        __base: i32,
        __group: i32,
    ) -> u64;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn getpwnam(__name: *const i8) -> *mut passwd;
    fn getgrnam(__name: *const i8) -> *mut group;
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    fn open_fatal(_: *const i8) -> !;
    fn fatal_exit() -> !;
    static mut group_name_option: *const i8;
    static mut group_option: gid_t;
    static mut owner_name_option: *const i8;
    static mut owner_option: uid_t;
    fn wordsplit(s: *const i8, ws: *mut wordsplit_t, flags: u32) -> i32;
    fn wordsplit_free(ws: *mut wordsplit_t);
    fn wordsplit_strerror(ws: *mut wordsplit_t) -> *const i8;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
}
pub type __uintmax_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type ssize_t = __ssize_t;
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
pub type uintmax_t = __uintmax_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut i8,
    pub gr_passwd: *mut i8,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut i8,
}
pub type Hash_table = hash_table;
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
pub struct mapentry {
    pub orig_id: uintmax_t,
    pub new_id: uintmax_t,
    pub new_name: *mut i8,
}
pub type Hash_data_freer = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_tuning = hash_tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_comparator = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut i8,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn strtoumax(
    mut nptr: *const i8,
    mut endptr: *mut *mut i8,
    mut base: i32,
) -> uintmax_t {
    return __strtoul_internal(nptr, endptr, base, 0 as i32);
}
unsafe extern "C" fn map_hash(
    mut entry: *const libc::c_void,
    mut nbuckets: size_t,
) -> size_t {
    let mut map: *const mapentry = entry as *const mapentry;
    return ((*map).orig_id).wrapping_rem(nbuckets);
}
unsafe extern "C" fn map_compare(
    mut entry1: *const libc::c_void,
    mut entry2: *const libc::c_void,
) -> bool {
    let mut map1: *const mapentry = entry1 as *const mapentry;
    let mut map2: *const mapentry = entry2 as *const mapentry;
    return (*map1).orig_id == (*map2).orig_id;
}
unsafe extern "C" fn parse_id(
    mut retval: *mut uintmax_t,
    mut arg: *const i8,
    mut what: *const i8,
    mut maxval: uintmax_t,
    mut file: *const i8,
    mut line: u32,
) -> i32 {
    let mut v: uintmax_t = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    *__errno_location() = 0 as i32;
    v = strtoumax(arg, &mut p, 10 as i32);
    if *p as i32 != 0 || *__errno_location() != 0 {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"%s:%u: invalid %s: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            file,
            line,
            what,
            arg,
        );
        return -(1 as i32);
    }
    if v > maxval {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"%s:%u: %s out of range: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            file,
            line,
            what,
            arg,
        );
        return -(1 as i32);
    }
    *retval = v;
    return 0 as i32;
}
unsafe extern "C" fn map_read(
    mut ptab: *mut *mut Hash_table,
    mut file: *const i8,
    mut name_to_id: Option<unsafe extern "C" fn(*const i8) -> uintmax_t>,
    mut what: *const i8,
    mut maxval: uintmax_t,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut bufsize: size_t = 0 as i32 as size_t;
    let mut n: ssize_t = 0;
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
    let mut wsopt: i32 = 0;
    let mut line: u32 = 0;
    let mut err: i32 = 0 as i32;
    fp = fopen(file, b"r\0" as *const u8 as *const i8);
    if fp.is_null() {
        open_fatal(file);
    }
    ws.ws_comment = b"#\0" as *const u8 as *const i8;
    wsopt = 0x8000 as i32 | 0x40 as i32 | 0x4 as i32 | 0x800 as i32
        | (0x200 as i32 | 0x400 as i32);
    line = 0 as i32 as u32;
    loop {
        n = getline(&mut buf, &mut bufsize, fp);
        if !(n > 0 as i32 as i64) {
            break;
        }
        let mut ent: *mut mapentry = 0 as *mut mapentry;
        let mut orig_id: uintmax_t = 0;
        let mut new_id: uintmax_t = 0;
        let mut name: *mut i8 = 0 as *mut i8;
        let mut colon: *mut i8 = 0 as *mut i8;
        line = line.wrapping_add(1);
        line;
        if wordsplit(buf, &mut ws, wsopt as u32) != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s:%u: cannot split line: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                file,
                line,
                wordsplit_strerror(&mut ws),
            );
            fatal_exit();
        }
        wsopt |= 0x8 as i32;
        if ws.ws_wordc == 0 as i32 as u64 {
            continue;
        }
        if ws.ws_wordc != 2 as i32 as u64 {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s:%u: malformed line\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                file,
                line,
            );
            err = 1 as i32;
        } else {
            if *(*(ws.ws_wordv).offset(0 as i32 as isize)).offset(0 as i32 as isize)
                as i32 == '+' as i32
            {
                if parse_id(
                    &mut orig_id,
                    (*(ws.ws_wordv).offset(0 as i32 as isize)).offset(1 as i32 as isize),
                    what,
                    maxval,
                    file,
                    line,
                ) != 0
                {
                    err = 1 as i32;
                    continue;
                }
            } else if name_to_id.is_some() {
                orig_id = name_to_id
                    .expect(
                        "non-null function pointer",
                    )(*(ws.ws_wordv).offset(0 as i32 as isize));
                if orig_id == 18446744073709551615 as u64 {
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"%s:%u: can't obtain %s of %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        file,
                        line,
                        what,
                        *(ws.ws_wordv).offset(0 as i32 as isize),
                    );
                    err = 1 as i32;
                    continue;
                }
            }
            colon = strchr(*(ws.ws_wordv).offset(1 as i32 as isize), ':' as i32);
            if !colon.is_null() {
                if colon > *(ws.ws_wordv).offset(1 as i32 as isize) {
                    name = *(ws.ws_wordv).offset(1 as i32 as isize);
                }
                let fresh0 = colon;
                colon = colon.offset(1);
                *fresh0 = 0 as i32 as i8;
                if parse_id(&mut new_id, colon, what, maxval, file, line) != 0 {
                    err = 1 as i32;
                    continue;
                }
            } else if *(*(ws.ws_wordv).offset(1 as i32 as isize))
                .offset(0 as i32 as isize) as i32 == '+' as i32
            {
                if parse_id(
                    &mut new_id,
                    *(ws.ws_wordv).offset(1 as i32 as isize),
                    what,
                    maxval,
                    file,
                    line,
                ) != 0
                {
                    err = 1 as i32;
                    continue;
                }
            } else {
                name = *(ws.ws_wordv).offset(1 as i32 as isize);
                new_id = name_to_id
                    .expect(
                        "non-null function pointer",
                    )(*(ws.ws_wordv).offset(1 as i32 as isize));
                if new_id == 18446744073709551615 as u64 {
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"%s:%u: can't obtain %s of %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        file,
                        line,
                        what,
                        *(ws.ws_wordv).offset(1 as i32 as isize),
                    );
                    err = 1 as i32;
                    continue;
                }
            }
            ent = xmalloc(::core::mem::size_of::<mapentry>() as u64) as *mut mapentry;
            (*ent).orig_id = orig_id;
            (*ent).new_id = new_id;
            (*ent).new_name = if !name.is_null() { xstrdup(name) } else { 0 as *mut i8 };
            if !((!(*ptab).is_null()
                || {
                    *ptab = hash_initialize(
                        0 as i32 as size_t,
                        0 as *const Hash_tuning,
                        Some(
                            map_hash
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    size_t,
                                ) -> size_t,
                        ),
                        Some(
                            map_compare
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> bool,
                        ),
                        None,
                    );
                    !(*ptab).is_null()
                }) && !(hash_insert(*ptab, ent as *const libc::c_void)).is_null())
            {
                xalloc_die();
            }
        }
    }
    if wsopt & 0x8 as i32 != 0 {
        wordsplit_free(&mut ws);
    }
    fclose(fp);
    if err != 0 {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"errors reading map file\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fatal_exit();
    }
}
static mut owner_map: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
unsafe extern "C" fn name_to_uid(mut name: *const i8) -> uintmax_t {
    let mut pw: *mut passwd = getpwnam(name);
    return if !pw.is_null() { (*pw).pw_uid as u64 } else { 18446744073709551615 as u64 };
}
#[no_mangle]
pub unsafe extern "C" fn owner_map_read(mut file: *const i8) {
    map_read(
        &mut owner_map,
        file,
        Some(name_to_uid as unsafe extern "C" fn(*const i8) -> uintmax_t),
        b"UID\0" as *const u8 as *const i8,
        (if (0 as i32 as uid_t) < -(1 as i32) as uid_t {
            -(1 as i32) as uid_t
        } else {
            ((1 as i32 as uid_t)
                << (::core::mem::size_of::<uid_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64))
                .wrapping_sub(1 as i32 as u32)
                .wrapping_mul(2 as i32 as u32)
                .wrapping_add(1 as i32 as u32)
        }) as uintmax_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn owner_map_translate(
    mut uid: uid_t,
    mut new_uid: *mut uid_t,
    mut new_name: *mut *const i8,
) -> i32 {
    let mut rc: i32 = 1 as i32;
    if !owner_map.is_null() {
        let mut ent: mapentry = mapentry {
            orig_id: 0,
            new_id: 0,
            new_name: 0 as *mut i8,
        };
        let mut res: *mut mapentry = 0 as *mut mapentry;
        ent.orig_id = uid as uintmax_t;
        res = hash_lookup(owner_map, &mut ent as *mut mapentry as *const libc::c_void)
            as *mut mapentry;
        if !res.is_null() {
            *new_uid = (*res).new_id as uid_t;
            *new_name = (*res).new_name;
            return 0 as i32;
        }
    }
    if owner_option != -(1 as i32) as uid_t {
        *new_uid = owner_option;
        rc = 0 as i32;
    }
    if !owner_name_option.is_null() {
        *new_name = owner_name_option;
        rc = 0 as i32;
    }
    return rc;
}
static mut group_map: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
unsafe extern "C" fn name_to_gid(mut name: *const i8) -> uintmax_t {
    let mut gr: *mut group = getgrnam(name);
    return if !gr.is_null() { (*gr).gr_gid as u64 } else { 18446744073709551615 as u64 };
}
#[no_mangle]
pub unsafe extern "C" fn group_map_read(mut file: *const i8) {
    map_read(
        &mut group_map,
        file,
        Some(name_to_gid as unsafe extern "C" fn(*const i8) -> uintmax_t),
        b"GID\0" as *const u8 as *const i8,
        (if (0 as i32 as gid_t) < -(1 as i32) as gid_t {
            -(1 as i32) as gid_t
        } else {
            ((1 as i32 as gid_t)
                << (::core::mem::size_of::<gid_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64))
                .wrapping_sub(1 as i32 as u32)
                .wrapping_mul(2 as i32 as u32)
                .wrapping_add(1 as i32 as u32)
        }) as uintmax_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn group_map_translate(
    mut gid: gid_t,
    mut new_gid: *mut gid_t,
    mut new_name: *mut *const i8,
) -> i32 {
    let mut rc: i32 = 1 as i32;
    if !group_map.is_null() {
        let mut ent: mapentry = mapentry {
            orig_id: 0,
            new_id: 0,
            new_name: 0 as *mut i8,
        };
        let mut res: *mut mapentry = 0 as *mut mapentry;
        ent.orig_id = gid as uintmax_t;
        res = hash_lookup(group_map, &mut ent as *mut mapentry as *const libc::c_void)
            as *mut mapentry;
        if !res.is_null() {
            *new_gid = (*res).new_id as gid_t;
            *new_name = (*res).new_name;
            return 0 as i32;
        }
    }
    if group_option != -(1 as i32) as uid_t {
        *new_gid = group_option;
        rc = 0 as i32;
    }
    if !group_name_option.is_null() {
        *new_name = group_name_option;
        rc = 0 as i32;
    }
    return rc;
}