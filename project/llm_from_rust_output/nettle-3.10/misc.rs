use std::alloc::{alloc, Layout};
use std::io::{stderr, Write};
use std::process;

#[derive(Debug)]
pub enum Error {
    AllocationFailed,
    IoError(std::io::Error),
}

pub fn die(message: &str) -> ! {
    let _ = writeln!(stderr(), "{}", message);
    process::exit(1);
}

pub fn werror(message: &str) -> Result<(), Error> {
    stderr()
        .write_all(message.as_bytes())
        .map_err(Error::IoError)
}

pub fn xalloc(size: usize) -> Result<*mut u8, Error> {
    let layout = Layout::from_size_align(size, std::mem::align_of::<u8>())
        .map_err(|_| Error::AllocationFailed)?;
    
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            die("Virtual memory exhausted.");
        }
        Ok(ptr)
    }
}

pub static SEXP_TOKEN_CHARS: [u8; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0,
    0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
];