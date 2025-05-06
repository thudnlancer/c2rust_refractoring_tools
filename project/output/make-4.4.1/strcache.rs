#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    static mut stdout: *mut _IO_FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn hash_init(
        ht: *mut hash_table,
        size: u64,
        hash_1: hash_func_t,
        hash_2: hash_func_t,
        hash_cmp: hash_cmp_func_t,
    );
    fn hash_find_slot(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut *mut libc::c_void;
    fn hash_insert_at(
        ht: *mut hash_table,
        item: *const libc::c_void,
        slot: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_print_stats(ht: *mut hash_table, out_FILE: *mut FILE);
    fn jhash_string(key: *const u8) -> u32;
    static mut hash_deleted_item: *mut libc::c_void;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut libc::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: u64,
    pub ht_capacity: u64,
    pub ht_fill: u64,
    pub ht_empty_slots: u64,
    pub ht_collisions: u64,
    pub ht_lookups: u64,
    pub ht_rehashes: u32,
}
pub type hash_cmp_func_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type hash_func_t = Option<unsafe extern "C" fn(*const libc::c_void) -> u64>;
pub type sc_buflen_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strcache {
    pub next: *mut strcache,
    pub end: sc_buflen_t,
    pub bytesfree: sc_buflen_t,
    pub count: sc_buflen_t,
    pub buffer: [i8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hugestring {
    pub next: *mut hugestring,
    pub buffer: [i8; 1],
}
static mut strcache: *mut strcache = 0 as *const strcache as *mut strcache;
static mut fullcache: *mut strcache = 0 as *const strcache as *mut strcache;
static mut total_buffers: u64 = 0 as i32 as u64;
static mut total_strings: u64 = 0 as i32 as u64;
static mut total_size: u64 = 0 as i32 as u64;
unsafe extern "C" fn new_cache(
    mut head: *mut *mut strcache,
    mut buflen: sc_buflen_t,
) -> *mut strcache {
    let mut new: *mut strcache = xmalloc((buflen as u64).wrapping_add(14 as u64))
        as *mut strcache;
    (*new).end = 0 as i32 as sc_buflen_t;
    (*new).count = 0 as i32 as sc_buflen_t;
    (*new).bytesfree = buflen;
    (*new).next = *head;
    *head = new;
    total_buffers = total_buffers.wrapping_add(1);
    total_buffers;
    return new;
}
unsafe extern "C" fn copy_string(
    mut sp: *mut strcache,
    mut str: *const i8,
    mut len: sc_buflen_t,
) -> *const i8 {
    let mut res: *mut i8 = &mut *((*sp).buffer).as_mut_ptr().offset((*sp).end as isize)
        as *mut i8;
    memmove(res as *mut libc::c_void, str as *const libc::c_void, len as u64);
    let fresh0 = len;
    len = len.wrapping_add(1);
    *res.offset(fresh0 as isize) = '\0' as i32 as i8;
    (*sp).end = ((*sp).end as i32 + len as i32) as sc_buflen_t;
    (*sp).bytesfree = ((*sp).bytesfree as i32 - len as i32) as sc_buflen_t;
    (*sp).count = ((*sp).count).wrapping_add(1);
    (*sp).count;
    return res;
}
unsafe extern "C" fn add_string(mut str: *const i8, mut len: sc_buflen_t) -> *const i8 {
    let mut res: *const i8 = 0 as *const i8;
    let mut sp: *mut strcache = 0 as *mut strcache;
    let mut spp: *mut *mut strcache = &mut strcache;
    let mut sz: sc_buflen_t = (len as i32 + 1 as i32) as sc_buflen_t;
    total_strings = total_strings.wrapping_add(1);
    total_strings;
    total_size = total_size.wrapping_add(sz as u64);
    if sz as u64
        > (8192 as i32 as u64)
            .wrapping_sub(
                (2 as i32 as u64).wrapping_mul(::core::mem::size_of::<size_t>() as u64),
            )
            .wrapping_sub(14 as u64)
    {
        sp = new_cache(&mut fullcache, sz);
        return copy_string(sp, str, len);
    }
    while !(*spp).is_null() {
        if (**spp).bytesfree as i32 > sz as i32 {
            break;
        }
        spp = &mut (**spp).next;
    }
    sp = *spp;
    if sp.is_null() {
        sp = new_cache(
            &mut strcache,
            (8192 as i32 as u64)
                .wrapping_sub(
                    (2 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<size_t>() as u64),
                )
                .wrapping_sub(14 as u64) as sc_buflen_t,
        );
        spp = &mut strcache;
    }
    res = copy_string(sp, str, len);
    if total_strings > 20 as i32 as u64
        && ((*sp).bytesfree as u64)
            < total_size.wrapping_div(total_strings).wrapping_add(1 as i32 as u64)
    {
        *spp = (*sp).next;
        (*sp).next = fullcache;
        fullcache = sp;
    }
    return res;
}
static mut hugestrings: *mut hugestring = 0 as *const hugestring as *mut hugestring;
unsafe extern "C" fn add_hugestring(mut str: *const i8, mut len: size_t) -> *const i8 {
    let mut new: *mut hugestring = xmalloc(
        (::core::mem::size_of::<hugestring>() as u64).wrapping_add(len),
    ) as *mut hugestring;
    memcpy(
        ((*new).buffer).as_mut_ptr() as *mut libc::c_void,
        str as *const libc::c_void,
        len,
    );
    *((*new).buffer).as_mut_ptr().offset(len as isize) = '\0' as i32 as i8;
    (*new).next = hugestrings;
    hugestrings = new;
    return ((*new).buffer).as_mut_ptr();
}
unsafe extern "C" fn str_hash_1(mut key: *const libc::c_void) -> u64 {
    let mut _result_: u64 = 0 as i32 as u64;
    let mut _key_: *const u8 = key as *const i8 as *const u8;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as u64);
    return _result_;
}
unsafe extern "C" fn str_hash_2(mut key: *const libc::c_void) -> u64 {
    let mut _result_: u64 = 0 as i32 as u64;
    return _result_;
}
unsafe extern "C" fn str_hash_cmp(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> i32 {
    return if x as *const i8 == y as *const i8 {
        0 as i32
    } else {
        strcmp(x as *const i8, y as *const i8)
    };
}
static mut strings: hash_table = hash_table {
    ht_vec: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
    ht_hash_1: None,
    ht_hash_2: None,
    ht_compare: None,
    ht_size: 0,
    ht_capacity: 0,
    ht_fill: 0,
    ht_empty_slots: 0,
    ht_collisions: 0,
    ht_lookups: 0,
    ht_rehashes: 0,
};
static mut total_adds: u64 = 0 as i32 as u64;
unsafe extern "C" fn add_hash(mut str: *const i8, mut len: size_t) -> *const i8 {
    let mut slot: *const *mut i8 = 0 as *const *mut i8;
    let mut key: *const i8 = 0 as *const i8;
    if len > (32767 as i32 * 2 as i32 + 1 as i32 - 1 as i32) as u64 {
        return add_hugestring(str, len);
    }
    slot = hash_find_slot(&mut strings, str as *const libc::c_void) as *const *mut i8;
    key = *slot;
    total_adds = total_adds.wrapping_add(1);
    total_adds;
    if !(key.is_null() || key as *mut libc::c_void == hash_deleted_item) {
        return key;
    }
    key = add_string(str, len as sc_buflen_t);
    hash_insert_at(
        &mut strings,
        key as *const libc::c_void,
        slot as *const libc::c_void,
    );
    return key;
}
#[no_mangle]
pub unsafe extern "C" fn strcache_iscached(mut str: *const i8) -> i32 {
    let mut sp: *mut strcache = 0 as *mut strcache;
    sp = strcache;
    while !sp.is_null() {
        if str >= ((*sp).buffer).as_mut_ptr()
            && str < ((*sp).buffer).as_mut_ptr().offset((*sp).end as i32 as isize)
        {
            return 1 as i32;
        }
        sp = (*sp).next;
    }
    sp = fullcache;
    while !sp.is_null() {
        if str >= ((*sp).buffer).as_mut_ptr()
            && str < ((*sp).buffer).as_mut_ptr().offset((*sp).end as i32 as isize)
        {
            return 1 as i32;
        }
        sp = (*sp).next;
    }
    let mut hp: *mut hugestring = 0 as *mut hugestring;
    hp = hugestrings;
    while !hp.is_null() {
        if str == ((*hp).buffer).as_mut_ptr() {
            return 1 as i32;
        }
        hp = (*hp).next;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn strcache_add(mut str: *const i8) -> *const i8 {
    return add_hash(str, strlen(str));
}
#[no_mangle]
pub unsafe extern "C" fn strcache_add_len(
    mut str: *const i8,
    mut len: size_t,
) -> *const i8 {
    if *str.offset(len as isize) as i32 != '\0' as i32 {
        let mut fresh1 = ::std::vec::from_elem(
            0,
            len.wrapping_add(1 as i32 as u64) as usize,
        );
        let mut key: *mut i8 = fresh1.as_mut_ptr() as *mut i8;
        memcpy(key as *mut libc::c_void, str as *const libc::c_void, len);
        *key.offset(len as isize) = '\0' as i32 as i8;
        str = key;
    }
    return add_hash(str, len);
}
#[no_mangle]
pub unsafe extern "C" fn strcache_init() {
    hash_init(
        &mut strings,
        8000 as i32 as u64,
        Some(str_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(str_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(
            str_hash_cmp
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn strcache_print_stats(mut prefix: *const i8) {
    let mut sp: *const strcache = 0 as *const strcache;
    let mut numbuffs: u64 = 0 as i32 as u64;
    let mut fullbuffs: u64 = 0 as i32 as u64;
    let mut totfree: u64 = 0 as i32 as u64;
    let mut maxfree: u64 = 0 as i32 as u64;
    let mut minfree: u64 = (8192 as i32 as u64)
        .wrapping_sub(
            (2 as i32 as u64).wrapping_mul(::core::mem::size_of::<size_t>() as u64),
        )
        .wrapping_sub(14 as u64);
    if strcache.is_null() {
        printf(
            dcgettext(
                0 as *const i8,
                b"\n%s No strcache buffers\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            prefix,
        );
        return;
    }
    sp = (*strcache).next;
    while !sp.is_null() {
        let mut bf: sc_buflen_t = (*sp).bytesfree;
        totfree = totfree.wrapping_add(bf as u64);
        maxfree = if bf as u64 > maxfree { bf as u64 } else { maxfree };
        minfree = if (bf as u64) < minfree { bf as u64 } else { minfree };
        numbuffs = numbuffs.wrapping_add(1);
        numbuffs;
        sp = (*sp).next;
    }
    sp = fullcache;
    while !sp.is_null() {
        let mut bf_0: sc_buflen_t = (*sp).bytesfree;
        totfree = totfree.wrapping_add(bf_0 as u64);
        maxfree = if bf_0 as u64 > maxfree { bf_0 as u64 } else { maxfree };
        minfree = if (bf_0 as u64) < minfree { bf_0 as u64 } else { minfree };
        numbuffs = numbuffs.wrapping_add(1);
        numbuffs;
        fullbuffs = fullbuffs.wrapping_add(1);
        fullbuffs;
        sp = (*sp).next;
    }
    printf(
        dcgettext(
            0 as *const i8,
            b"\n%s strcache buffers: %lu (%lu) / strings = %lu / storage = %lu B / avg = %lu B\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        prefix,
        numbuffs.wrapping_add(1 as i32 as u64),
        fullbuffs,
        total_strings,
        total_size,
        total_size.wrapping_div(total_strings),
    );
    printf(
        dcgettext(
            0 as *const i8,
            b"%s current buf: size = %hu B / used = %hu B / count = %hu / avg = %u B\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        prefix,
        (8192 as i32 as u64)
            .wrapping_sub(
                (2 as i32 as u64).wrapping_mul(::core::mem::size_of::<size_t>() as u64),
            )
            .wrapping_sub(14 as u64) as sc_buflen_t as i32,
        (*strcache).end as i32,
        (*strcache).count as i32,
        ((*strcache).end as i32 / (*strcache).count as i32) as u32,
    );
    if numbuffs != 0 {
        let mut sz: u64 = total_size.wrapping_sub((*strcache).end as u64);
        let mut cnt: u64 = total_strings.wrapping_sub((*strcache).count as u64);
        let mut avgfree: sc_buflen_t = totfree.wrapping_div(numbuffs) as sc_buflen_t;
        printf(
            dcgettext(
                0 as *const i8,
                b"%s other used: total = %lu B / count = %lu / avg = %lu B\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            prefix,
            sz,
            cnt,
            sz.wrapping_div(cnt),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"%s other free: total = %lu B / max = %lu B / min = %lu B / avg = %hu B\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            prefix,
            totfree,
            maxfree,
            minfree,
            avgfree as i32,
        );
    }
    printf(
        dcgettext(
            0 as *const i8,
            b"\n%s strcache performance: lookups = %lu / hit rate = %lu%%\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        prefix,
        total_adds,
        (100.0f64 * total_adds.wrapping_sub(total_strings) as libc::c_double
            / total_adds as libc::c_double) as u64,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"# hash-table stats:\n# \0" as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    hash_print_stats(&mut strings, stdout);
}