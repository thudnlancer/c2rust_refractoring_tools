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
    pub type __locale_data;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn rpl_free(ptr: *mut libc::c_void);
    fn uselocale(__dataset: locale_t) -> locale_t;
    fn setlocale_null(category: i32) -> *const i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn nl_langinfo(__item: nl_item) -> *mut i8;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_mutexattr_gettype(
        __attr: *const pthread_mutexattr_t,
        __kind: *mut i32,
    ) -> i32;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> i32;
}
pub type nl_item = i32;
pub type locale_t = __locale_t;
pub type __locale_t = *mut __locale_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const libc::c_ushort,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const i8; 13],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct struniq_hash_node {
    pub next: *mut struniq_hash_node,
    pub contents: [i8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [i8; 40],
    pub __align: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [i8; 4],
    pub __align: i32,
}
pub type size_t = u64;
unsafe extern "C" fn string_hash(mut x: *const libc::c_void) -> size_t {
    let mut s: *const i8 = x as *const i8;
    let mut h: size_t = 0 as i32 as size_t;
    while *s != 0 {
        h = (*s as u64)
            .wrapping_add(
                h << 9 as i32
                    | h
                        >> (::core::mem::size_of::<size_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(9 as i32 as u64),
            );
        s = s.offset(1);
        s;
    }
    return h;
}
static mut struniq_hash_table: [*mut struniq_hash_node; 257] = [0
    as *const struniq_hash_node as *mut struniq_hash_node; 257];
static mut struniq_lock: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as i32,
            __count: 0 as i32 as u32,
            __owner: 0 as i32,
            __nusers: 0 as i32 as u32,
            __kind: 0 as i32,
            __spins: 0 as i32 as libc::c_short,
            __elision: 0 as i32 as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
unsafe extern "C" fn struniq(mut string: *const i8) -> *const i8 {
    let mut hashcode: size_t = string_hash(string as *const libc::c_void);
    let mut slot: size_t = hashcode.wrapping_rem(257 as i32 as u64);
    let mut size: size_t = 0;
    let mut new_node: *mut struniq_hash_node = 0 as *mut struniq_hash_node;
    let mut p: *mut struniq_hash_node = 0 as *mut struniq_hash_node;
    p = struniq_hash_table[slot as usize];
    while !p.is_null() {
        if strcmp(((*p).contents).as_mut_ptr(), string) == 0 as i32 {
            return ((*p).contents).as_mut_ptr();
        }
        p = (*p).next;
    }
    size = (strlen(string)).wrapping_add(1 as i32 as u64);
    new_node = malloc(
        (8 as u64)
            .wrapping_add(8 as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_add(size) & !(8 as u64).wrapping_sub(1 as i32 as u64),
    ) as *mut struniq_hash_node;
    if new_node.is_null() {
        return b"C\0" as *const u8 as *const i8;
    }
    memcpy(
        ((*new_node).contents).as_mut_ptr() as *mut libc::c_void,
        string as *const libc::c_void,
        size,
    );
    let mut current_block_23: u64;
    let mut mt: bool = 1 as i32 != 0;
    if mt {
        if if (Some(
            pthread_mutexattr_gettype
                as unsafe extern "C" fn(*const pthread_mutexattr_t, *mut i32) -> i32,
        ))
            .is_some() || 0 as i32 != 0
        {
            pthread_mutex_lock(&mut struniq_lock)
        } else {
            0 as i32
        } != 0
        {
            abort();
        }
    }
    p = struniq_hash_table[slot as usize];
    loop {
        if p.is_null() {
            current_block_23 = 5689001924483802034;
            break;
        }
        if strcmp(((*p).contents).as_mut_ptr(), string) == 0 as i32 {
            rpl_free(new_node as *mut libc::c_void);
            new_node = p;
            current_block_23 = 199334024179629906;
            break;
        } else {
            p = (*p).next;
        }
    }
    match current_block_23 {
        5689001924483802034 => {
            ::core::ptr::write_volatile(
                &mut (*new_node).next as *mut *mut struniq_hash_node,
                struniq_hash_table[slot as usize],
            );
            ::core::ptr::write_volatile(
                &mut struniq_hash_table[slot as usize] as *mut *mut struniq_hash_node,
                new_node,
            );
        }
        _ => {}
    }
    if mt {
        if if (Some(
            pthread_mutexattr_gettype
                as unsafe extern "C" fn(*const pthread_mutexattr_t, *mut i32) -> i32,
        ))
            .is_some() || 0 as i32 != 0
        {
            pthread_mutex_unlock(&mut struniq_lock)
        } else {
            0 as i32
        } != 0
        {
            abort();
        }
    }
    return ((*new_node).contents).as_mut_ptr();
}
unsafe extern "C" fn gl_locale_name_thread_unsafe(
    mut category: i32,
    mut categoryname: *const i8,
) -> *const i8 {
    let mut thread_locale: locale_t = uselocale(0 as locale_t);
    if thread_locale != -(1 as i64) as locale_t {
        let mut name: *const i8 = nl_langinfo(
            category << 16 as i32 | -(1 as i32) & 0xffff as i32,
        );
        if *name.offset(0 as i32 as isize) as i32 == '\0' as i32 {
            name = (*thread_locale).__names[category as usize];
        }
        return name;
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn gl_locale_name_thread(
    mut category: i32,
    mut categoryname: *const i8,
) -> *const i8 {
    let mut name: *const i8 = gl_locale_name_thread_unsafe(category, categoryname);
    if !name.is_null() {
        return struniq(name);
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn gl_locale_name_posix(
    mut category: i32,
    mut categoryname: *const i8,
) -> *const i8 {
    let mut locname: *const i8 = 0 as *const i8;
    locname = setlocale_null(category);
    return locname;
}
#[no_mangle]
pub unsafe extern "C" fn gl_locale_name_environ(
    mut category: i32,
    mut categoryname: *const i8,
) -> *const i8 {
    let mut retval: *const i8 = 0 as *const i8;
    retval = getenv(b"LC_ALL\0" as *const u8 as *const i8);
    if !retval.is_null() && *retval.offset(0 as i32 as isize) as i32 != '\0' as i32 {
        return retval;
    }
    retval = getenv(categoryname);
    if !retval.is_null() && *retval.offset(0 as i32 as isize) as i32 != '\0' as i32 {
        return retval;
    }
    retval = getenv(b"LANG\0" as *const u8 as *const i8);
    if !retval.is_null() && *retval.offset(0 as i32 as isize) as i32 != '\0' as i32 {
        return retval;
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn gl_locale_name_default() -> *const i8 {
    return b"C\0" as *const u8 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn gl_locale_name(
    mut category: i32,
    mut categoryname: *const i8,
) -> *const i8 {
    let mut retval: *const i8 = 0 as *const i8;
    retval = gl_locale_name_thread(category, categoryname);
    if !retval.is_null() {
        return retval;
    }
    retval = gl_locale_name_posix(category, categoryname);
    if !retval.is_null() {
        return retval;
    }
    return gl_locale_name_default();
}