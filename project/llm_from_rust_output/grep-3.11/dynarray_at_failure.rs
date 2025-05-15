use std::process;

pub type size_t = usize;

#[no_mangle]
pub extern "C" fn gl_dynarray_at_failure(size: size_t, index: size_t) -> ! {
    process::abort();
}