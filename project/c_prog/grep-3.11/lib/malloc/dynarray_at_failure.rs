use ::libc;
extern "C" {
    fn abort() -> !;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn gl_dynarray_at_failure(mut size: size_t, mut index: size_t) {
    abort();
}
