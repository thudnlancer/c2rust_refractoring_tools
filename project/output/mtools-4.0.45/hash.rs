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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
}
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
pub type uint32_t = __uint32_t;
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
pub struct hashtable {
    pub f1: T_HashFunc,
    pub f2: T_HashFunc,
    pub compar: T_ComparFunc,
    pub size: size_t,
    pub fill: size_t,
    pub inuse: size_t,
    pub max: size_t,
    pub entries: *mut *mut libc::c_void,
}
pub type T_ComparFunc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> i32,
>;
pub type T_HashFunc = Option<unsafe extern "C" fn(*mut libc::c_void) -> uint32_t>;
pub type T_HashTable = hashtable;
static mut sizes: [size_t; 30] = [
    5 as i32 as size_t,
    11 as i32 as size_t,
    23 as i32 as size_t,
    47 as i32 as size_t,
    97 as i32 as size_t,
    197 as i32 as size_t,
    397 as i32 as size_t,
    797 as i32 as size_t,
    1597 as i32 as size_t,
    3203 as i32 as size_t,
    6421 as i32 as size_t,
    12853 as i32 as size_t,
    25717 as i32 as size_t,
    51437 as i32 as size_t,
    102877 as i32 as size_t,
    205759 as i32 as size_t,
    411527 as i32 as size_t,
    823117 as i32 as size_t,
    1646237 as i32 as size_t,
    3292489 as i32 as size_t,
    6584983 as i32 as size_t,
    13169977 as i32 as size_t,
    26339969 as i32 as size_t,
    52679969 as i32 as size_t,
    105359939 as i32 as size_t,
    210719881 as i32 as size_t,
    421439783 as i32 as size_t,
    842879579 as i32 as size_t,
    1685759167 as i32 as size_t,
    0 as i32 as size_t,
];
static mut deleted: i32 = 0 as i32;
static mut unallocated: i32 = 0 as i32;
unsafe extern "C" fn alloc_ht(mut H: *mut T_HashTable, mut size: size_t) -> i32 {
    let mut i: i32 = 0;
    let mut ii: size_t = 0;
    i = 0 as i32;
    while sizes[i as usize] != 0 {
        if sizes[i as usize] > size.wrapping_mul(4 as i32 as u64) {
            break;
        }
        i += 1;
        i;
    }
    if sizes[i as usize] == 0 {
        i = 0 as i32;
        while sizes[i as usize] != 0 {
            if sizes[i as usize] > size.wrapping_mul(2 as i32 as u64) {
                break;
            }
            i += 1;
            i;
        }
    }
    if sizes[i as usize] == 0 {
        i = 0 as i32;
        while sizes[i as usize] != 0 {
            if sizes[i as usize] > size {
                break;
            }
            i += 1;
            i;
        }
    }
    if sizes[i as usize] == 0 {
        return -(1 as i32);
    }
    size = sizes[i as usize];
    if size < (*H).size {
        size = (*H).size;
    }
    (*H).max = size
        .wrapping_mul(4 as i32 as u64)
        .wrapping_div(5 as i32 as u64)
        .wrapping_sub(2 as i32 as u64);
    (*H).size = size;
    (*H).fill = 0 as i32 as size_t;
    (*H).inuse = 0 as i32 as size_t;
    (*H).entries = calloc(size, ::core::mem::size_of::<*mut libc::c_void>() as u64)
        as *mut *mut libc::c_void;
    if ((*H).entries).is_null() {
        return -(1 as i32);
    }
    ii = 0 as i32 as size_t;
    while ii < size {
        let ref mut fresh0 = *((*H).entries).offset(ii as isize);
        *fresh0 = &mut unallocated as *mut i32 as *mut libc::c_void;
        ii = ii.wrapping_add(1);
        ii;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn make_ht(
    mut f1: T_HashFunc,
    mut f2: T_HashFunc,
    mut c: T_ComparFunc,
    mut size: size_t,
    mut H: *mut *mut T_HashTable,
) -> i32 {
    *H = calloc(1 as i32 as u64, ::core::mem::size_of::<T_HashTable>() as u64)
        as *mut T_HashTable;
    if (*H).is_null() {
        return -(1 as i32);
    }
    (**H).f1 = f1;
    (**H).f2 = f2;
    (**H).compar = c;
    (**H).size = 0 as i32 as size_t;
    if alloc_ht(*H, size) != 0 {
        return -(1 as i32);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn free_ht(
    mut H: *mut T_HashTable,
    mut entry_free: T_HashFunc,
) -> i32 {
    let mut i: size_t = 0;
    if entry_free.is_some() {
        i = 0 as i32 as size_t;
        while i < (*H).size {
            if *((*H).entries).offset(i as isize)
                != &mut unallocated as *mut i32 as *mut libc::c_void
                && *((*H).entries).offset(i as isize)
                    != &mut deleted as *mut i32 as *mut libc::c_void
            {
                entry_free
                    .expect(
                        "non-null function pointer",
                    )(*((*H).entries).offset(i as isize));
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    free((*H).entries as *mut i8 as *mut libc::c_void);
    free(H as *mut i8 as *mut libc::c_void);
    return 0 as i32;
}
unsafe extern "C" fn mt_hash_add(
    mut H: *mut T_HashTable,
    mut E: *mut libc::c_void,
    mut hint: *mut size_t,
) -> i32 {
    let mut f2: size_t = 0;
    let mut pos: size_t = 0;
    let mut ctr: i32 = 0;
    pos = (((*H).f1).expect("non-null function pointer")(E) as u64)
        .wrapping_rem((*H).size);
    f2 = -(1 as i32) as size_t;
    ctr = 0 as i32;
    while *((*H).entries).offset(pos as isize)
        != &mut unallocated as *mut i32 as *mut libc::c_void
        && *((*H).entries).offset(pos as isize)
            != &mut deleted as *mut i32 as *mut libc::c_void
    {
        if f2 == -(1 as i32) as size_t {
            f2 = (((*H).f2).expect("non-null function pointer")(E) as u64)
                .wrapping_rem(((*H).size).wrapping_sub(1 as i32 as u64));
        }
        pos = pos.wrapping_add(f2).wrapping_add(1 as i32 as u64).wrapping_rem((*H).size);
        ctr += 1;
        ctr;
    }
    if *((*H).entries).offset(pos as isize)
        == &mut unallocated as *mut i32 as *mut libc::c_void
    {
        (*H).fill = ((*H).fill).wrapping_add(1);
        (*H).fill;
    }
    (*H).inuse = ((*H).inuse).wrapping_add(1);
    (*H).inuse;
    let ref mut fresh1 = *((*H).entries).offset(pos as isize);
    *fresh1 = E;
    if !hint.is_null() {
        *hint = pos;
    }
    return 0 as i32;
}
unsafe extern "C" fn rehash(mut H: *mut T_HashTable) -> i32 {
    let mut size: size_t = 0;
    let mut i: size_t = 0;
    let mut oldentries: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    size = (*H).size;
    oldentries = (*H).entries;
    if alloc_ht(
        H,
        ((*H).inuse)
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(4 as i32 as u64)
            .wrapping_add((*H).fill)
            .wrapping_div(5 as i32 as u64),
    ) != 0
    {
        return -(1 as i32);
    }
    i = 0 as i32 as size_t;
    while i < size {
        if *oldentries.offset(i as isize)
            != &mut unallocated as *mut i32 as *mut libc::c_void
            && *oldentries.offset(i as isize)
                != &mut deleted as *mut i32 as *mut libc::c_void
        {
            mt_hash_add(H, *oldentries.offset(i as isize), 0 as *mut size_t);
        }
        i = i.wrapping_add(1);
        i;
    }
    free(oldentries as *mut i8 as *mut libc::c_void);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn hash_add(
    mut H: *mut T_HashTable,
    mut E: *mut libc::c_void,
    mut hint: *mut size_t,
) -> i32 {
    if (*H).fill >= (*H).max {
        rehash(H);
    }
    if (*H).fill == (*H).size {
        return -(1 as i32);
    }
    return mt_hash_add(H, E, hint);
}
unsafe extern "C" fn mt_hash_lookup(
    mut H: *mut T_HashTable,
    mut E: *mut libc::c_void,
    mut E2: *mut *mut libc::c_void,
    mut hint: *mut size_t,
    mut isIdentity: i32,
) -> i32 {
    let mut f2: size_t = 0;
    let mut pos: size_t = 0;
    let mut upos: size_t = 0;
    let mut ttl: size_t = 0;
    pos = (((*H).f1).expect("non-null function pointer")(E) as u64)
        .wrapping_rem((*H).size);
    ttl = (*H).size;
    f2 = -(1 as i32) as size_t;
    upos = -(1 as i32) as size_t;
    while ttl != 0
        && *((*H).entries).offset(pos as isize)
            != &mut unallocated as *mut i32 as *mut libc::c_void
        && (*((*H).entries).offset(pos as isize)
            == &mut deleted as *mut i32 as *mut libc::c_void
            || (isIdentity != 0
                || ((*H).compar)
                    .expect(
                        "non-null function pointer",
                    )(*((*H).entries).offset(pos as isize), E) != 0 as i32)
                && (isIdentity == 0 || *((*H).entries).offset(pos as isize) != E))
    {
        if f2 == -(1 as i32) as size_t {
            f2 = (((*H).f2).expect("non-null function pointer")(E) as u64)
                .wrapping_rem(((*H).size).wrapping_sub(1 as i32 as u64));
        }
        if upos == -(1 as i32) as size_t
            && *((*H).entries).offset(pos as isize)
                == &mut deleted as *mut i32 as *mut libc::c_void
        {
            upos = pos;
        }
        pos = pos.wrapping_add(f2).wrapping_add(1 as i32 as u64).wrapping_rem((*H).size);
        ttl = ttl.wrapping_sub(1);
        ttl;
    }
    if *((*H).entries).offset(pos as isize)
        == &mut unallocated as *mut i32 as *mut libc::c_void || ttl == 0
    {
        return -(1 as i32);
    }
    if upos != -(1 as i32) as size_t {
        let ref mut fresh2 = *((*H).entries).offset(upos as isize);
        *fresh2 = *((*H).entries).offset(pos as isize);
        let ref mut fresh3 = *((*H).entries).offset(pos as isize);
        *fresh3 = &mut deleted as *mut i32 as *mut libc::c_void;
        pos = upos;
    }
    if !hint.is_null() {
        *hint = pos;
    }
    *E2 = *((*H).entries).offset(pos as isize);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn hash_lookup(
    mut H: *mut T_HashTable,
    mut E: *mut libc::c_void,
    mut E2: *mut *mut libc::c_void,
    mut hint: *mut size_t,
) -> i32 {
    return mt_hash_lookup(H, E, E2, hint, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn hash_remove(
    mut H: *mut T_HashTable,
    mut E: *mut libc::c_void,
    mut hint: size_t,
) -> i32 {
    let mut E2: *mut libc::c_void = 0 as *mut libc::c_void;
    if hint < (*H).size && *((*H).entries).offset(hint as isize) == E {
        (*H).inuse = ((*H).inuse).wrapping_sub(1);
        (*H).inuse;
        let ref mut fresh4 = *((*H).entries).offset(hint as isize);
        *fresh4 = &mut deleted as *mut i32 as *mut libc::c_void;
        return 0 as i32;
    }
    if mt_hash_lookup(H, E, &mut E2, &mut hint, 1 as i32) != 0 {
        fprintf(stderr, b"Removing non-existent entry\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    (*H).inuse = ((*H).inuse).wrapping_sub(1);
    (*H).inuse;
    let ref mut fresh5 = *((*H).entries).offset(hint as isize);
    *fresh5 = &mut deleted as *mut i32 as *mut libc::c_void;
    return 0 as i32;
}