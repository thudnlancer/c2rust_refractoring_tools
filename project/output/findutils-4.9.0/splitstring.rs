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
    fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
}
pub type size_t = u64;
unsafe extern "C" fn field_length(
    mut str: *const i8,
    mut separators: *const i8,
) -> size_t {
    if *separators != 0 {
        let mut end: *const i8 = strpbrk(str, separators);
        if !end.is_null() {
            return end.offset_from(str) as i64 as size_t;
        }
    }
    return strlen(str);
}
#[no_mangle]
pub unsafe extern "C" fn splitstring(
    mut s: *const i8,
    mut separators: *const i8,
    mut first: bool,
    mut pos: *mut size_t,
    mut len: *mut size_t,
) -> bool {
    if first {
        *pos = 0 as u32 as size_t;
        *len = 0 as u32 as size_t;
    } else {
        *pos = (*pos as u64).wrapping_add(*len) as size_t as size_t;
        if *s.offset(*pos as isize) != 0 {
            *pos = (*pos).wrapping_add(1);
            *pos;
        } else {
            return 0 as i32 != 0
        }
    }
    *len = field_length(&*s.offset(*pos as isize), separators);
    return 1 as i32 != 0;
}