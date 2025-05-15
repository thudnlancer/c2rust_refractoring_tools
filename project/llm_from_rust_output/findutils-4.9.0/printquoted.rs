use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::ptr;

struct QuotingOptions;

extern "C" {
    fn quotearg_buffer(
        buffer: *mut libc::c_char,
        buffersize: libc::size_t,
        arg: *const libc::c_char,
        argsize: libc::size_t,
        o: *const QuotingOptions,
    ) -> libc::size_t;
    fn qmark_chars(buf: *mut libc::c_char, len: libc::size_t) -> libc::size_t;
}

fn print_quoted(
    mut fp: &mut dyn Write,
    qopts: &QuotingOptions,
    dest_is_tty: bool,
    format: &CStr,
    s: &CStr,
) -> io::Result<()> {
    if dest_is_tty {
        const SMALL_BUF_SIZE: usize = 8192;
        let mut smallbuf = vec![0; SMALL_BUF_SIZE];
        
        let len = unsafe {
            quotearg_buffer(
                smallbuf.as_mut_ptr(),
                SMALL_BUF_SIZE as libc::size_t,
                s.as_ptr(),
                !0,
                qopts as *const _,
            )
        };

        let buf = if len < SMALL_BUF_SIZE as libc::size_t {
            unsafe { CStr::from_ptr(smallbuf.as_ptr()) }.to_owned()
        } else {
            let mut buf = vec![0; (len + 1) as usize];
            unsafe {
                quotearg_buffer(
                    buf.as_mut_ptr(),
                    len + 1,
                    s.as_ptr(),
                    !0,
                    qopts as *const _,
                );
            }
            unsafe { CStr::from_ptr(buf.as_ptr()) }.to_owned()
        };

        let mut buf = buf.into_bytes();
        let new_len = unsafe { qmark_chars(buf.as_mut_ptr() as *mut _, len) };
        buf.truncate(new_len as usize);
        buf.push(0);

        write!(fp, "{}", format.to_str()?, unsafe {
            CStr::from_ptr(buf.as_ptr() as *const _).to_str()?
        })?;
    } else {
        write!(fp, "{}", format.to_str()?, s.to_str()?)?;
    }

    Ok(())
}