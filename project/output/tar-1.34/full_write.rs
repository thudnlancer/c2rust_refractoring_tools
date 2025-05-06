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
    fn __errno_location() -> *mut i32;
    fn safe_write(fd: i32, buf: *const libc::c_void, count: size_t) -> size_t;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn full_write(
    mut fd: i32,
    mut buf: *const libc::c_void,
    mut count: size_t,
) -> size_t {
    let mut total: size_t = 0 as i32 as size_t;
    let mut ptr: *const i8 = buf as *const i8;
    while count > 0 as i32 as u64 {
        let mut n_rw: size_t = safe_write(fd, ptr as *const libc::c_void, count);
        if n_rw == -(1 as i32) as size_t {
            break;
        }
        if n_rw == 0 as i32 as u64 {
            *__errno_location() = 28 as i32;
            break;
        } else {
            total = (total as u64).wrapping_add(n_rw) as size_t as size_t;
            ptr = ptr.offset(n_rw as isize);
            count = (count as u64).wrapping_sub(n_rw) as size_t as size_t;
        }
    }
    return total;
}