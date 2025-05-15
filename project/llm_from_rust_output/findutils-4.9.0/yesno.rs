use std::io::{self, BufRead};
use std::ffi::CString;

pub fn yesno() -> bool {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut response = String::new();

    match handle.read_line(&mut response) {
        Ok(0) => false,  // EOF or empty input
        Ok(_) => {
            if response.ends_with('\n') {
                response.pop();
            }
            
            let c_str = match CString::new(response) {
                Ok(s) => s,
                Err(_) => return false,
            };
            
            // Note: rpmatch is still unsafe C function, but we isolate the unsafe block
            unsafe { rpmatch(c_str.as_ptr()) > 0 }
        }
        Err(_) => false,
    }
}

// External C function declaration
extern "C" {
    fn rpmatch(__response: *const libc::c_char) -> libc::c_int;
}