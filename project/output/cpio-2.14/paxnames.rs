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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn rpl_free(_: *mut libc::c_void);
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xalloc_die();
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn hash_get_n_entries(table: *const Hash_table) -> size_t;
    fn hash_string(string: *const i8, n_buckets: size_t) -> size_t;
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
pub type size_t = u64;
pub type Hash_table = hash_table;
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
unsafe extern "C" fn hash_string_hasher(
    mut name: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    return hash_string(name as *const i8, n_buckets);
}
unsafe extern "C" fn hash_string_compare(
    mut name1: *const libc::c_void,
    mut name2: *const libc::c_void,
) -> bool {
    return strcmp(name1 as *const i8, name2 as *const i8) == 0 as i32;
}
unsafe extern "C" fn hash_string_insert_prefix(
    mut table: *mut *mut Hash_table,
    mut string: *const i8,
    mut len: size_t,
    mut return_prefix: *mut *const i8,
) -> bool {
    let mut t: *mut Hash_table = *table;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut e: *mut i8 = 0 as *mut i8;
    if len != 0 {
        s = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
        memcpy(s as *mut libc::c_void, string as *const libc::c_void, len);
        *s.offset(len as isize) = 0 as i32 as i8;
    } else {
        s = xstrdup(string);
    }
    if !((!t.is_null()
        || {
            t = hash_initialize(
                0 as i32 as size_t,
                0 as *const Hash_tuning,
                Some(
                    hash_string_hasher
                        as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
                ),
                Some(
                    hash_string_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> bool,
                ),
                None,
            );
            *table = t;
            !(*table).is_null()
        })
        && {
            e = hash_insert(t, s as *const libc::c_void) as *mut i8;
            !e.is_null()
        })
    {
        xalloc_die();
    }
    if e == s {
        if !return_prefix.is_null() {
            *return_prefix = s;
        }
        return 1 as i32 != 0;
    } else {
        rpl_free(s as *mut libc::c_void);
        return 0 as i32 != 0;
    };
}
static mut prefix_table: [*mut Hash_table; 2] = [0 as *const Hash_table
    as *mut Hash_table; 2];
#[no_mangle]
pub unsafe extern "C" fn removed_prefixes_p() -> bool {
    return !(prefix_table[0 as i32 as usize]).is_null()
        && hash_get_n_entries(prefix_table[0 as i32 as usize]) != 0 as i32 as u64
        || !(prefix_table[1 as i32 as usize]).is_null()
            && hash_get_n_entries(prefix_table[1 as i32 as usize]) != 0 as i32 as u64;
}
#[no_mangle]
pub unsafe extern "C" fn safer_name_suffix(
    mut file_name: *const i8,
    mut link_target: bool,
    mut absolute_names: bool,
) -> *mut i8 {
    let mut p: *const i8 = 0 as *const i8;
    if absolute_names {
        p = file_name;
    } else {
        let mut prefix_len: size_t = 0 as i32 as size_t;
        p = file_name.offset(prefix_len as isize);
        while *p != 0 {
            if *p.offset(0 as i32 as isize) as i32 == '.' as i32
                && *p.offset(1 as i32 as isize) as i32 == '.' as i32
                && (*p.offset(2 as i32 as isize) as i32 == '/' as i32
                    || *p.offset(2 as i32 as isize) == 0)
            {
                prefix_len = p.offset(2 as i32 as isize).offset_from(file_name) as i64
                    as size_t;
            }
            loop {
                let fresh0 = p;
                p = p.offset(1);
                let mut c: i8 = *fresh0;
                if c as i32 == '/' as i32 {
                    break;
                }
                if !(*p != 0) {
                    break;
                }
            }
        }
        p = file_name.offset(prefix_len as isize);
        while *p as i32 == '/' as i32 {
            p = p.offset(1);
            p;
        }
        prefix_len = p.offset_from(file_name) as i64 as size_t;
        if prefix_len != 0 {
            let mut prefix: *const i8 = 0 as *const i8;
            if hash_string_insert_prefix(
                &mut *prefix_table.as_mut_ptr().offset(link_target as isize),
                file_name,
                prefix_len,
                &mut prefix,
            ) {
                static mut diagnostic: [*const i8; 2] = [
                    b"Removing leading `%s' from member names\0" as *const u8
                        as *const i8,
                    b"Removing leading `%s' from hard link targets\0" as *const u8
                        as *const i8,
                ];
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        diagnostic[link_target as usize],
                        5 as i32,
                    ),
                    prefix,
                );
            }
        }
    }
    if *p == 0 {
        if p == file_name {
            static mut diagnostic_0: [*const i8; 2] = [
                b"Substituting `.' for empty member name\0" as *const u8 as *const i8,
                b"Substituting `.' for empty hard link target\0" as *const u8
                    as *const i8,
            ];
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                b"%s\0" as *const u8 as *const i8,
                dcgettext(0 as *const i8, diagnostic_0[link_target as usize], 5 as i32),
            );
        }
        p = b".\0" as *const u8 as *const i8;
    }
    return p as *mut i8;
}