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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn last_component(filename: *const i8) -> *mut i8;
    fn malloc(_: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn dir_len(mut file: *const i8) -> size_t {
    let mut prefix_length: size_t = 0 as i32 as size_t;
    let mut length: size_t = 0;
    prefix_length = (prefix_length as u64)
        .wrapping_add(
            (if prefix_length != 0 as i32 as u64 {
                (0 as i32 != 0
                    && *file.offset(prefix_length as isize) as i32 == '/' as i32) as i32
            } else if *file.offset(0 as i32 as isize) as i32 == '/' as i32 {
                if 0 as i32 != 0 && *file.offset(1 as i32 as isize) as i32 == '/' as i32
                    && !(*file.offset(2 as i32 as isize) as i32 == '/' as i32)
                {
                    2 as i32
                } else {
                    1 as i32
                }
            } else {
                0 as i32
            }) as u64,
        ) as size_t as size_t;
    length = (last_component(file)).offset_from(file) as i64 as size_t;
    while prefix_length < length {
        if !(*file.offset(length.wrapping_sub(1 as i32 as u64) as isize) as i32
            == '/' as i32)
        {
            break;
        }
        length = length.wrapping_sub(1);
        length;
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn mdir_name(mut file: *const i8) -> *mut i8 {
    let mut length: size_t = dir_len(file);
    let mut append_dot: bool = length == 0 as i32 as u64
        || 0 as i32 != 0 && length == 0 as i32 as u64
            && *file.offset(2 as i32 as isize) as i32 != '\0' as i32
            && !(*file.offset(2 as i32 as isize) as i32 == '/' as i32);
    let mut dir: *mut i8 = malloc(
        length.wrapping_add(append_dot as u64).wrapping_add(1 as i32 as u64),
    ) as *mut i8;
    if dir.is_null() {
        return 0 as *mut i8;
    }
    memcpy(dir as *mut libc::c_void, file as *const libc::c_void, length);
    if append_dot {
        let fresh0 = length;
        length = length.wrapping_add(1);
        *dir.offset(fresh0 as isize) = '.' as i32 as i8;
    }
    *dir.offset(length as isize) = '\0' as i32 as i8;
    return dir;
}