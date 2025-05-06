#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn acl_errno_valid(mut errnum: i32) -> bool {
    match errnum {
        16 => return 0 as i32 != 0,
        22 => return 0 as i32 != 0,
        38 => return 0 as i32 != 0,
        95 => return 0 as i32 != 0,
        _ => return 1 as i32 != 0,
    };
}