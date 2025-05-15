use std::process;

pub type SizeT = usize;

#[no_mangle]
pub fn gl_dynarray_at_failure(size: SizeT, index: SizeT) -> ! {
    process::abort();
}