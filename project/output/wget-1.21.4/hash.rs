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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn rpl_free(_: *mut libc::c_void);
    fn abort() -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type uintptr_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub hash_function: hashfun_t,
    pub test_function: testfun_t,
    pub cells: *mut cell,
    pub size: i32,
    pub count: i32,
    pub resize_threshold: i32,
    pub prime_offset: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
}
pub type testfun_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type hashfun_t = Option<unsafe extern "C" fn(*const libc::c_void) -> u64>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table_iterator {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub pos: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn c_tolower(mut c: i32) -> i32 {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
unsafe extern "C" fn prime_size(mut size: i32, mut prime_offset: *mut i32) -> i32 {
    static mut primes: [i32; 71] = [
        13 as i32,
        19 as i32,
        29 as i32,
        41 as i32,
        59 as i32,
        79 as i32,
        107 as i32,
        149 as i32,
        197 as i32,
        263 as i32,
        347 as i32,
        457 as i32,
        599 as i32,
        787 as i32,
        1031 as i32,
        1361 as i32,
        1777 as i32,
        2333 as i32,
        3037 as i32,
        3967 as i32,
        5167 as i32,
        6719 as i32,
        8737 as i32,
        11369 as i32,
        14783 as i32,
        19219 as i32,
        24989 as i32,
        32491 as i32,
        42257 as i32,
        54941 as i32,
        71429 as i32,
        92861 as i32,
        120721 as i32,
        156941 as i32,
        204047 as i32,
        265271 as i32,
        344857 as i32,
        448321 as i32,
        582821 as i32,
        757693 as i32,
        985003 as i32,
        1280519 as i32,
        1664681 as i32,
        2164111 as i32,
        2813353 as i32,
        3657361 as i32,
        4754591 as i32,
        6180989 as i32,
        8035301 as i32,
        10445899 as i32,
        13579681 as i32,
        17653589 as i32,
        22949669 as i32,
        29834603 as i32,
        38784989 as i32,
        50420551 as i32,
        65546729 as i32,
        85210757 as i32,
        110774011 as i32,
        144006217 as i32,
        187208107 as i32,
        243370577 as i32,
        316381771 as i32,
        411296309 as i32,
        534685237 as i32,
        695090819 as i32,
        903618083 as i32,
        1174703521 as i32,
        1527114613 as i32,
        1837299131 as i32,
        2147483647 as i32,
    ];
    let mut i: size_t = 0;
    i = *prime_offset as size_t;
    while i
        < (::core::mem::size_of::<[i32; 71]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64)
    {
        if primes[i as usize] >= size {
            *prime_offset = i.wrapping_add(1 as i32 as u64) as i32;
            return primes[i as usize];
        }
        i = i.wrapping_add(1);
        i;
    }
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn hash_table_new(
    mut items: i32,
    mut hash_function: Option<unsafe extern "C" fn(*const libc::c_void) -> u64>,
    mut test_function: Option<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
    >,
) -> *mut hash_table {
    let mut size: i32 = 0;
    let mut ht: *mut hash_table = xmalloc(::core::mem::size_of::<hash_table>() as u64)
        as *mut hash_table;
    (*ht).hash_function = if hash_function.is_some() {
        hash_function
    } else {
        Some(hash_pointer as unsafe extern "C" fn(*const libc::c_void) -> u64)
    };
    (*ht).test_function = if test_function.is_some() {
        test_function
    } else {
        Some(
            cmp_pointer
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        )
    };
    (*ht).prime_offset = 0 as i32;
    size = (1 as i32 as libc::c_double + items as libc::c_double / 0.75f64) as i32;
    size = prime_size(size, &mut (*ht).prime_offset);
    (*ht).size = size;
    (*ht).resize_threshold = (size as libc::c_double * 0.75f64) as i32;
    (*ht).cells = xmalloc(
        ((*ht).size as u64).wrapping_mul(::core::mem::size_of::<cell>() as u64),
    ) as *mut cell;
    memset(
        (*ht).cells as *mut libc::c_void,
        127 as i32 * 2 as i32 + 1 as i32,
        (size as u64).wrapping_mul(::core::mem::size_of::<cell>() as u64),
    );
    (*ht).count = 0 as i32;
    return ht;
}
#[no_mangle]
pub unsafe extern "C" fn hash_table_destroy(mut ht: *mut hash_table) {
    rpl_free((*ht).cells as *mut libc::c_void);
    (*ht).cells = 0 as *mut cell;
    rpl_free(ht as *mut libc::c_void);
    ht = 0 as *mut hash_table;
}
#[inline]
unsafe extern "C" fn find_cell(
    mut ht: *const hash_table,
    mut key: *const libc::c_void,
) -> *mut cell {
    let mut cells: *mut cell = (*ht).cells;
    let mut size: i32 = (*ht).size;
    let mut c: *mut cell = cells
        .offset(
            (((*ht).hash_function).expect("non-null function pointer")(key))
                .wrapping_rem(size as u64) as isize,
        );
    let mut equals: testfun_t = (*ht).test_function;
    while (*c).key != !(0 as i32 as uintptr_t) as *mut libc::c_void {
        if equals.expect("non-null function pointer")(key, (*c).key) != 0 {
            break;
        }
        c = if c != cells.offset((size - 1 as i32) as isize) {
            c.offset(1 as i32 as isize)
        } else {
            cells
        };
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn hash_table_get(
    mut ht: *const hash_table,
    mut key: *const libc::c_void,
) -> *mut libc::c_void {
    let mut c: *mut cell = find_cell(ht, key);
    if (*c).key != !(0 as i32 as uintptr_t) as *mut libc::c_void {
        return (*c).value
    } else {
        return 0 as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn hash_table_get_pair(
    mut ht: *const hash_table,
    mut lookup_key: *const libc::c_void,
    mut orig_key: *mut libc::c_void,
    mut value: *mut libc::c_void,
) -> i32 {
    let mut c: *mut cell = find_cell(ht, lookup_key);
    if (*c).key != !(0 as i32 as uintptr_t) as *mut libc::c_void {
        if !orig_key.is_null() {
            let ref mut fresh0 = *(orig_key as *mut *mut libc::c_void);
            *fresh0 = (*c).key;
        }
        if !value.is_null() {
            let ref mut fresh1 = *(value as *mut *mut libc::c_void);
            *fresh1 = (*c).value;
        }
        return 1 as i32;
    } else {
        return 0 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn hash_table_contains(
    mut ht: *const hash_table,
    mut key: *const libc::c_void,
) -> i32 {
    let mut c: *mut cell = find_cell(ht, key);
    return ((*c).key != !(0 as i32 as uintptr_t) as *mut libc::c_void) as i32;
}
unsafe extern "C" fn grow_hash_table(mut ht: *mut hash_table) {
    let mut hasher: hashfun_t = (*ht).hash_function;
    let mut old_cells: *mut cell = (*ht).cells;
    let mut old_end: *mut cell = ((*ht).cells).offset((*ht).size as isize);
    let mut c: *mut cell = 0 as *mut cell;
    let mut cells: *mut cell = 0 as *mut cell;
    let mut newsize: i32 = 0;
    newsize = prime_size((*ht).size * 2 as i32, &mut (*ht).prime_offset);
    (*ht).size = newsize;
    (*ht).resize_threshold = (newsize as libc::c_double * 0.75f64) as i32;
    cells = xmalloc((newsize as u64).wrapping_mul(::core::mem::size_of::<cell>() as u64))
        as *mut cell;
    memset(
        cells as *mut libc::c_void,
        127 as i32 * 2 as i32 + 1 as i32,
        (newsize as u64).wrapping_mul(::core::mem::size_of::<cell>() as u64),
    );
    (*ht).cells = cells;
    c = old_cells;
    while c < old_end {
        if (*c).key != !(0 as i32 as uintptr_t) as *mut libc::c_void {
            let mut new_c: *mut cell = 0 as *mut cell;
            new_c = cells
                .offset(
                    (hasher.expect("non-null function pointer")((*c).key))
                        .wrapping_rem(newsize as u64) as isize,
                );
            while (*new_c).key != !(0 as i32 as uintptr_t) as *mut libc::c_void {
                new_c = if new_c != cells.offset((newsize - 1 as i32) as isize) {
                    new_c.offset(1 as i32 as isize)
                } else {
                    cells
                };
            }
            *new_c = *c;
        }
        c = c.offset(1);
        c;
    }
    rpl_free(old_cells as *mut libc::c_void);
    old_cells = 0 as *mut cell;
}
#[no_mangle]
pub unsafe extern "C" fn hash_table_put(
    mut ht: *mut hash_table,
    mut key: *const libc::c_void,
    mut value: *const libc::c_void,
) {
    let mut c: *mut cell = find_cell(ht, key);
    if (*c).key != !(0 as i32 as uintptr_t) as *mut libc::c_void {
        (*c).key = key as *mut libc::c_void;
        (*c).value = value as *mut libc::c_void;
        return;
    }
    if (*ht).count >= (*ht).resize_threshold {
        grow_hash_table(ht);
        c = find_cell(ht, key);
    }
    (*ht).count += 1;
    (*ht).count;
    (*c).key = key as *mut libc::c_void;
    (*c).value = value as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn hash_table_remove(
    mut ht: *mut hash_table,
    mut key: *const libc::c_void,
) -> i32 {
    let mut c: *mut cell = find_cell(ht, key);
    if !((*c).key != !(0 as i32 as uintptr_t) as *mut libc::c_void) {
        return 0 as i32
    } else {
        let mut size: i32 = (*ht).size;
        let mut cells: *mut cell = (*ht).cells;
        let mut hasher: hashfun_t = (*ht).hash_function;
        (*c).key = !(0 as i32 as uintptr_t) as *mut libc::c_void;
        (*ht).count -= 1;
        (*ht).count;
        c = if c != cells.offset((size - 1 as i32) as isize) {
            c.offset(1 as i32 as isize)
        } else {
            cells
        };
        while (*c).key != !(0 as i32 as uintptr_t) as *mut libc::c_void {
            let mut current_block_7: u64;
            let mut key2: *const libc::c_void = (*c).key;
            let mut c_new: *mut cell = 0 as *mut cell;
            c_new = cells
                .offset(
                    (hasher.expect("non-null function pointer")(key2))
                        .wrapping_rem(size as u64) as isize,
                );
            loop {
                if !((*c_new).key != !(0 as i32 as uintptr_t) as *mut libc::c_void) {
                    current_block_7 = 13183875560443969876;
                    break;
                }
                if key2 == (*c_new).key {
                    current_block_7 = 12209867499936983673;
                    break;
                }
                c_new = if c_new != cells.offset((size - 1 as i32) as isize) {
                    c_new.offset(1 as i32 as isize)
                } else {
                    cells
                };
            }
            match current_block_7 {
                13183875560443969876 => {
                    *c_new = *c;
                    (*c).key = !(0 as i32 as uintptr_t) as *mut libc::c_void;
                }
                _ => {}
            }
            c = if c != cells.offset((size - 1 as i32) as isize) {
                c.offset(1 as i32 as isize)
            } else {
                cells
            };
        }
        return 1 as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn hash_table_clear(mut ht: *mut hash_table) {
    memset(
        (*ht).cells as *mut libc::c_void,
        127 as i32 * 2 as i32 + 1 as i32,
        ((*ht).size as u64).wrapping_mul(::core::mem::size_of::<cell>() as u64),
    );
    (*ht).count = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn hash_table_for_each(
    mut ht: *mut hash_table,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> i32,
    >,
    mut arg: *mut libc::c_void,
) {
    let mut c: *mut cell = (*ht).cells;
    let mut end: *mut cell = ((*ht).cells).offset((*ht).size as isize);
    while c < end {
        if (*c).key != !(0 as i32 as uintptr_t) as *mut libc::c_void {
            let mut key: *mut libc::c_void = 0 as *mut libc::c_void;
            loop {
                key = (*c).key;
                if fn_0.expect("non-null function pointer")(key, (*c).value, arg) != 0 {
                    return;
                }
                if !((*c).key != key
                    && (*c).key != !(0 as i32 as uintptr_t) as *mut libc::c_void)
                {
                    break;
                }
            }
        }
        c = c.offset(1);
        c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn hash_table_iterate(
    mut ht: *mut hash_table,
    mut iter: *mut hash_table_iterator,
) {
    (*iter).pos = (*ht).cells as *mut libc::c_void;
    (*iter).end = ((*ht).cells).offset((*ht).size as isize) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn hash_table_iter_next(
    mut iter: *mut hash_table_iterator,
) -> i32 {
    let mut c: *mut cell = (*iter).pos as *mut cell;
    let mut end: *mut cell = (*iter).end as *mut cell;
    while c < end {
        if (*c).key != !(0 as i32 as uintptr_t) as *mut libc::c_void {
            (*iter).key = (*c).key;
            (*iter).value = (*c).value;
            (*iter).pos = c.offset(1 as i32 as isize) as *mut libc::c_void;
            return 1 as i32;
        }
        c = c.offset(1);
        c;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn hash_table_count(mut ht: *const hash_table) -> i32 {
    return (*ht).count;
}
unsafe extern "C" fn hash_string(mut key: *const libc::c_void) -> u64 {
    let mut p: *const i8 = key as *const i8;
    let mut h: u32 = *p as u32;
    if h != 0 {
        p = p.offset(1 as i32 as isize);
        while *p as i32 != '\0' as i32 {
            h = (h << 5 as i32).wrapping_sub(h).wrapping_add(*p as u32);
            p = p.offset(1);
            p;
        }
    }
    return h as u64;
}
unsafe extern "C" fn cmp_string(
    mut s1: *const libc::c_void,
    mut s2: *const libc::c_void,
) -> i32 {
    return (strcmp(s1 as *const i8, s2 as *const i8) == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn make_string_hash_table(mut items: i32) -> *mut hash_table {
    return hash_table_new(
        items,
        Some(hash_string as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(
            cmp_string
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
}
unsafe extern "C" fn hash_string_nocase(mut key: *const libc::c_void) -> u64 {
    let mut p: *const i8 = key as *const i8;
    let mut h: u32 = c_tolower(*p as i32) as u32;
    if h != 0 {
        p = p.offset(1 as i32 as isize);
        while *p as i32 != '\0' as i32 {
            h = (h << 5 as i32)
                .wrapping_sub(h)
                .wrapping_add(c_tolower(*p as i32) as u32);
            p = p.offset(1);
            p;
        }
    }
    return h as u64;
}
unsafe extern "C" fn string_cmp_nocase(
    mut s1: *const libc::c_void,
    mut s2: *const libc::c_void,
) -> i32 {
    return (strcasecmp(s1 as *const i8, s2 as *const i8) == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn make_nocase_string_hash_table(
    mut items: i32,
) -> *mut hash_table {
    return hash_table_new(
        items,
        Some(hash_string_nocase as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(
            string_cmp_nocase
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn hash_pointer(mut ptr: *const libc::c_void) -> u64 {
    let mut key: uintptr_t = ptr as uintptr_t;
    key = (key as u64).wrapping_add(key << 12 as i32) as uintptr_t as uintptr_t;
    key ^= key >> 22 as i32;
    key = (key as u64).wrapping_add(key << 4 as i32) as uintptr_t as uintptr_t;
    key ^= key >> 9 as i32;
    key = (key as u64).wrapping_add(key << 10 as i32) as uintptr_t as uintptr_t;
    key ^= key >> 2 as i32;
    key = (key as u64).wrapping_add(key << 7 as i32) as uintptr_t as uintptr_t;
    key ^= key >> 12 as i32;
    return key;
}
unsafe extern "C" fn cmp_pointer(
    mut ptr1: *const libc::c_void,
    mut ptr2: *const libc::c_void,
) -> i32 {
    return (ptr1 == ptr2) as i32;
}