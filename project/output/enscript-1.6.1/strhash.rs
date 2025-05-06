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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
}
pub type StringHashPtr = *mut stringhash_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringhash_st {
    pub hash_table: *mut HashTable,
    pub next_idx: u32,
    pub next_item: *mut HashList,
}
pub type HashList = hash_list_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_list_st {
    pub next: *mut hash_list_st,
    pub key: *mut i8,
    pub keylen: i32,
    pub data: *mut libc::c_void,
}
pub type HashTable = *mut HashList;
#[no_mangle]
pub unsafe extern "C" fn strhash_init() -> StringHashPtr {
    let mut tmp: StringHashPtr = 0 as *mut stringhash_st;
    tmp = calloc(1 as i32 as u64, ::core::mem::size_of::<stringhash_st>() as u64)
        as StringHashPtr;
    if tmp.is_null() {
        return 0 as StringHashPtr;
    }
    (*tmp).hash_table = calloc(
        8192 as i32 as u64,
        ::core::mem::size_of::<HashTable>() as u64,
    ) as *mut HashTable;
    if ((*tmp).hash_table).is_null() {
        free(tmp as *mut libc::c_void);
        return 0 as StringHashPtr;
    }
    return tmp;
}
#[no_mangle]
pub unsafe extern "C" fn strhash_free(mut hash: StringHashPtr) {
    let mut list: *mut HashList = 0 as *mut HashList;
    let mut list_next: *mut HashList = 0 as *mut HashList;
    let mut i: i32 = 0;
    if hash.is_null() {
        return;
    }
    i = 0 as i32;
    while i < 8192 as i32 {
        list = *((*hash).hash_table).offset(i as isize);
        while !list.is_null() {
            list_next = (*list).next;
            free((*list).key as *mut libc::c_void);
            free(list as *mut libc::c_void);
            list = list_next;
        }
        i += 1;
        i;
    }
    free((*hash).hash_table as *mut libc::c_void);
    free(hash as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn strhash_put(
    mut hash: StringHashPtr,
    mut key: *mut i8,
    mut keylen: i32,
    mut data: *mut libc::c_void,
    mut old_data: *mut *mut libc::c_void,
) -> i32 {
    let mut list: *mut HashList = 0 as *mut HashList;
    let mut prev: *mut HashList = 0 as *mut HashList;
    let mut pos: i32 = 0;
    let mut cmp_val: i32 = 0;
    if hash.is_null() || key.is_null() || keylen <= 0 as i32 {
        return 0 as i32;
    }
    if !old_data.is_null() {
        *old_data = 0 as *mut libc::c_void;
    }
    pos = count_hash(key, keylen);
    list = *((*hash).hash_table).offset(pos as isize);
    while !list.is_null() {
        if (*list).keylen == keylen {
            cmp_val = memcmp(
                key as *const libc::c_void,
                (*list).key as *const libc::c_void,
                keylen as u64,
            );
            if cmp_val == 0 as i32 {
                if !old_data.is_null() {
                    *old_data = (*list).data;
                }
                (*list).data = data;
                return 1 as i32;
            } else if cmp_val < 0 as i32 {
                break;
            }
        } else if (*list).keylen > keylen {
            break;
        }
        prev = list;
        list = (*list).next;
    }
    list = calloc(1 as i32 as u64, ::core::mem::size_of::<HashList>() as u64)
        as *mut HashList;
    if list.is_null() {
        return 0 as i32;
    }
    (*list).key = malloc(keylen as u64) as *mut i8;
    if ((*list).key).is_null() {
        free(list as *mut libc::c_void);
        return 0 as i32;
    }
    memcpy((*list).key as *mut libc::c_void, key as *const libc::c_void, keylen as u64);
    (*list).keylen = keylen;
    (*list).data = data;
    if prev.is_null() {
        (*list).next = *((*hash).hash_table).offset(pos as isize);
        let ref mut fresh0 = *((*hash).hash_table).offset(pos as isize);
        *fresh0 = list;
    } else {
        (*list).next = (*prev).next;
        (*prev).next = list;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn strhash_get(
    mut hash: StringHashPtr,
    mut key: *const i8,
    mut keylen: i32,
    mut data: *mut *mut libc::c_void,
) -> i32 {
    let mut list: *mut HashList = 0 as *mut HashList;
    let mut pos: i32 = 0;
    let mut cmp_val: i32 = 0;
    if hash.is_null() || key.is_null() || keylen <= 0 as i32 || data.is_null() {
        return 0 as i32;
    }
    *data = 0 as *mut libc::c_void;
    pos = count_hash(key, keylen);
    list = *((*hash).hash_table).offset(pos as isize);
    while !list.is_null() {
        if (*list).keylen == keylen {
            cmp_val = memcmp(
                key as *const libc::c_void,
                (*list).key as *const libc::c_void,
                keylen as u64,
            );
            if cmp_val == 0 as i32 {
                *data = (*list).data;
                return 1 as i32;
            } else if cmp_val < 0 as i32 {
                break;
            }
        } else if (*list).keylen > keylen {
            break;
        }
        list = (*list).next;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn strhash_delete(
    mut hash: StringHashPtr,
    mut key: *const i8,
    mut keylen: i32,
    mut data: *mut *mut libc::c_void,
) -> i32 {
    let mut list: *mut HashList = 0 as *mut HashList;
    let mut prev: *mut HashList = 0 as *mut HashList;
    let mut pos: i32 = 0;
    let mut cmp_val: i32 = 0;
    if hash.is_null() || key.is_null() || keylen <= 0 as i32 || data.is_null() {
        return 0 as i32;
    }
    *data = 0 as *mut libc::c_void;
    pos = count_hash(key, keylen);
    list = *((*hash).hash_table).offset(pos as isize);
    while !list.is_null() {
        if (*list).keylen == keylen {
            cmp_val = memcmp(
                key as *const libc::c_void,
                (*list).key as *const libc::c_void,
                keylen as u64,
            );
            if cmp_val == 0 as i32 {
                if prev.is_null() {
                    let ref mut fresh1 = *((*hash).hash_table).offset(pos as isize);
                    *fresh1 = (*list).next;
                } else {
                    (*prev).next = (*list).next;
                }
                *data = (*list).data;
                free((*list).key as *mut libc::c_void);
                free(list as *mut libc::c_void);
                (*hash).next_idx = 0 as i32 as u32;
                (*hash).next_item = 0 as *mut HashList;
                return 1 as i32;
            } else if cmp_val < 0 as i32 {
                break;
            }
        } else if (*list).keylen > keylen {
            break;
        }
        prev = list;
        list = (*list).next;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn strhash_get_first(
    mut hash: StringHashPtr,
    mut key_return: *mut *mut i8,
    mut keylen_return: *mut i32,
    mut data_return: *mut *mut libc::c_void,
) -> i32 {
    if hash.is_null() || key_return.is_null() || keylen_return.is_null()
        || data_return.is_null()
    {
        return 0 as i32;
    }
    (*hash).next_idx = 0 as i32 as u32;
    while (*hash).next_idx < 8192 as i32 as u32 {
        (*hash).next_item = *((*hash).hash_table).offset((*hash).next_idx as isize);
        if !((*hash).next_item).is_null() {
            *key_return = (*(*hash).next_item).key;
            *keylen_return = (*(*hash).next_item).keylen;
            *data_return = (*(*hash).next_item).data;
            return 1 as i32;
        }
        (*hash).next_idx = ((*hash).next_idx).wrapping_add(1);
        (*hash).next_idx;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn strhash_get_next(
    mut hash: StringHashPtr,
    mut key_return: *mut *mut i8,
    mut keylen_return: *mut i32,
    mut data_return: *mut *mut libc::c_void,
) -> i32 {
    if hash.is_null() || key_return.is_null() || keylen_return.is_null()
        || data_return.is_null()
    {
        return 0 as i32;
    }
    while (*hash).next_idx < 8192 as i32 as u32 {
        if ((*hash).next_item).is_null() {
            (*hash).next_item = *((*hash).hash_table).offset((*hash).next_idx as isize);
        } else {
            (*hash).next_item = (*(*hash).next_item).next;
        }
        if !((*hash).next_item).is_null() {
            *key_return = (*(*hash).next_item).key;
            *keylen_return = (*(*hash).next_item).keylen;
            *data_return = (*(*hash).next_item).data;
            return 1 as i32;
        }
        (*hash).next_idx = ((*hash).next_idx).wrapping_add(1);
        (*hash).next_idx;
    }
    return 0 as i32;
}
unsafe extern "C" fn count_hash(mut key: *const i8, mut keylen: i32) -> i32 {
    let mut val: u32 = 0 as i32 as u32;
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while i < keylen as u32 {
        val = val << 5 as i32 ^ *key.offset(i as isize) as u8 as u32 ^ val >> 16 as i32
            ^ val >> 7 as i32;
        i = i.wrapping_add(1);
        i;
    }
    return val.wrapping_rem(8192 as i32 as u32) as i32;
}