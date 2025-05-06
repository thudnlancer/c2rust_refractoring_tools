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
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn _nc_free(ptr: *mut libc::c_void, name: *const i8, line: i32);
    fn _nc_alloc(size: size_t, name: *const i8, line: i32) -> *mut libc::c_void;
    fn _nc_realloc(
        ptr: *mut libc::c_void,
        size: size_t,
        name: *const i8,
        line: i32,
    ) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type rstatus_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub nelem: uint32_t,
    pub elem: *mut libc::c_void,
    pub size: size_t,
    pub nalloc: uint32_t,
}
pub type uint32_t = __uint32_t;
pub type uint8_t = __uint8_t;
pub type array_compare_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type array_each_t = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> rstatus_t,
>;
#[inline]
unsafe extern "C" fn array_n(mut a: *const array) -> uint32_t {
    return (*a).nelem;
}
#[no_mangle]
pub unsafe extern "C" fn array_create(mut n: uint32_t, mut size: size_t) -> *mut array {
    let mut a: *mut array = 0 as *mut array;
    a = _nc_alloc(
        ::core::mem::size_of::<array>() as u64,
        b"nc_array.c\0" as *const u8 as *const i8,
        29 as i32,
    ) as *mut array;
    if a.is_null() {
        return 0 as *mut array;
    }
    (*a).elem = _nc_alloc(
        (n as u64).wrapping_mul(size),
        b"nc_array.c\0" as *const u8 as *const i8,
        34 as i32,
    );
    if ((*a).elem).is_null() {
        _nc_free(
            a as *mut libc::c_void,
            b"nc_array.c\0" as *const u8 as *const i8,
            36 as i32,
        );
        a = 0 as *mut array;
        return 0 as *mut array;
    }
    (*a).nelem = 0 as i32 as uint32_t;
    (*a).size = size;
    (*a).nalloc = n;
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn array_destroy(mut a: *mut array) {
    array_deinit(a);
    _nc_free(
        a as *mut libc::c_void,
        b"nc_array.c\0" as *const u8 as *const i8,
        51 as i32,
    );
    a = 0 as *mut array;
}
#[no_mangle]
pub unsafe extern "C" fn array_init(
    mut a: *mut array,
    mut n: uint32_t,
    mut size: size_t,
) -> rstatus_t {
    (*a).elem = _nc_alloc(
        (n as u64).wrapping_mul(size),
        b"nc_array.c\0" as *const u8 as *const i8,
        59 as i32,
    );
    if ((*a).elem).is_null() {
        return -(3 as i32);
    }
    (*a).nelem = 0 as i32 as uint32_t;
    (*a).size = size;
    (*a).nalloc = n;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn array_deinit(mut a: *mut array) {
    if !((*a).elem).is_null() {
        _nc_free((*a).elem, b"nc_array.c\0" as *const u8 as *const i8, 77 as i32);
        (*a).elem = 0 as *mut libc::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn array_idx(
    mut a: *const array,
    mut elem: *const libc::c_void,
) -> uint32_t {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut q: *const uint8_t = 0 as *const uint8_t;
    let mut off: uint32_t = 0;
    let mut idx: uint32_t = 0;
    p = (*a).elem as *const uint8_t;
    q = elem as *const uint8_t;
    off = q.offset_from(p) as i64 as uint32_t;
    idx = off.wrapping_div((*a).size as uint32_t);
    return idx;
}
#[no_mangle]
pub unsafe extern "C" fn array_push(mut a: *mut array) -> *mut libc::c_void {
    let mut elem: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut size: size_t = 0;
    if (*a).nelem == (*a).nalloc {
        size = ((*a).size).wrapping_mul((*a).nalloc as u64);
        new = _nc_realloc(
            (*a).elem,
            (2 as i32 as u64).wrapping_mul(size),
            b"nc_array.c\0" as *const u8 as *const i8,
            110 as i32,
        );
        if new.is_null() {
            return 0 as *mut libc::c_void;
        }
        (*a).elem = new;
        (*a).nalloc = ((*a).nalloc as u32).wrapping_mul(2 as i32 as u32) as uint32_t
            as uint32_t;
    }
    elem = ((*a).elem as *mut uint8_t)
        .offset(((*a).size).wrapping_mul((*a).nelem as u64) as isize)
        as *mut libc::c_void;
    (*a).nelem = ((*a).nelem).wrapping_add(1);
    (*a).nelem;
    return elem;
}
#[no_mangle]
pub unsafe extern "C" fn array_pop(mut a: *mut array) -> *mut libc::c_void {
    let mut elem: *mut libc::c_void = 0 as *mut libc::c_void;
    (*a).nelem = ((*a).nelem).wrapping_sub(1);
    (*a).nelem;
    elem = ((*a).elem as *mut uint8_t)
        .offset(((*a).size).wrapping_mul((*a).nelem as u64) as isize)
        as *mut libc::c_void;
    return elem;
}
#[no_mangle]
pub unsafe extern "C" fn array_get(
    mut a: *const array,
    mut idx: uint32_t,
) -> *mut libc::c_void {
    let mut elem: *mut libc::c_void = 0 as *mut libc::c_void;
    elem = ((*a).elem as *mut uint8_t)
        .offset(((*a).size).wrapping_mul(idx as u64) as isize) as *mut libc::c_void;
    return elem;
}
#[no_mangle]
pub unsafe extern "C" fn array_top(mut a: *const array) -> *mut libc::c_void {
    return array_get(a, ((*a).nelem).wrapping_sub(1 as i32 as u32));
}
#[no_mangle]
pub unsafe extern "C" fn array_swap(mut a: *mut array, mut b: *mut array) {
    let mut tmp: array = array {
        nelem: 0,
        elem: 0 as *mut libc::c_void,
        size: 0,
        nalloc: 0,
    };
    tmp = *a;
    *a = *b;
    *b = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn array_sort(mut a: *mut array, mut compare: array_compare_t) {
    qsort((*a).elem, (*a).nelem as size_t, (*a).size, compare);
}
#[no_mangle]
pub unsafe extern "C" fn array_each(
    mut a: *const array,
    mut func: array_each_t,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    let mut i: uint32_t = 0;
    let mut nelem: uint32_t = 0;
    i = 0 as i32 as uint32_t;
    nelem = array_n(a);
    while i < nelem {
        let mut elem: *mut libc::c_void = array_get(a, i);
        let mut status: rstatus_t = 0;
        status = func.expect("non-null function pointer")(elem, data);
        if status != 0 as i32 {
            return status;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as i32;
}