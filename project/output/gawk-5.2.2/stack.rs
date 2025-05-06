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
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
static mut size: size_t = 0;
static mut stack: *mut *mut libc::c_void = 0 as *const *mut libc::c_void
    as *mut *mut libc::c_void;
static mut index: i32 = -(1 as i32);
#[no_mangle]
pub unsafe extern "C" fn stack_empty() -> i32 {
    return (index < 0 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn stack_top() -> *mut libc::c_void {
    if stack_empty() != 0 || stack.is_null() {
        return 0 as *mut libc::c_void;
    }
    return *stack.offset(index as isize);
}
#[no_mangle]
pub unsafe extern "C" fn stack_pop() -> *mut libc::c_void {
    if stack_empty() != 0 || stack.is_null() {
        return 0 as *mut libc::c_void;
    }
    let fresh0 = index;
    index = index - 1;
    return *stack.offset(fresh0 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn stack_push(mut object: *mut libc::c_void) -> i32 {
    let mut new_stack: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut new_size: size_t = (2 as i32 as u64).wrapping_mul(size);
    if stack.is_null() {
        stack = malloc(
            (20 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64),
        ) as *mut *mut libc::c_void;
        if stack.is_null() {
            return 0 as i32;
        }
        size = 20 as i32 as size_t;
    } else if (index + 1 as i32) as u64 >= size {
        if new_size < size {
            return 0 as i32;
        }
        new_stack = realloc(
            stack as *mut libc::c_void,
            new_size.wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64),
        ) as *mut *mut libc::c_void;
        if new_stack.is_null() {
            return 0 as i32;
        }
        size = new_size;
        stack = new_stack;
    }
    index += 1;
    let ref mut fresh1 = *stack.offset(index as isize);
    *fresh1 = object;
    return 1 as i32;
}