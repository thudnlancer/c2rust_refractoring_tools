pub fn acl_errno_valid(errnum: i32) -> bool {
    match errnum {
        16 | 22 | 38 | 95 => false,
        _ => true,
    }
}