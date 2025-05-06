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
    fn concatenated_filename(
        directory: *const i8,
        filename: *const i8,
        suffix: *const i8,
    ) -> *mut i8;
    fn xalloc_die();
}
#[no_mangle]
pub unsafe extern "C" fn xconcatenated_filename(
    mut directory: *const i8,
    mut filename: *const i8,
    mut suffix: *const i8,
) -> *mut i8 {
    let mut result: *mut i8 = 0 as *mut i8;
    result = concatenated_filename(directory, filename, suffix);
    if result.is_null() {
        xalloc_die();
    }
    return result;
}