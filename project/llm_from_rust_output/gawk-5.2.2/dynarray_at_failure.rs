use std::process;

pub type SizeT = usize;

#[no_mangle]
pub extern "C" fn __libc_dynarray_at_failure(size: SizeT, index: SizeT) -> ! {
    process::abort();
}