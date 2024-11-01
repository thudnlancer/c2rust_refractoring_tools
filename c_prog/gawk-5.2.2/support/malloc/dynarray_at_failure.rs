#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn abort() -> !;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn __libc_dynarray_at_failure(
    mut size: size_t,
    mut index: size_t,
) -> ! {
    abort();
}
