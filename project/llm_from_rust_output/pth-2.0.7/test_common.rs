use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::sync::Once;

type SizeT = usize;
type SSizeT = isize;

const READLINE_BUF_SIZE: usize = 1024;

struct ReadlineBuf {
    rl_cnt: c_int,
    rl_bufptr: *mut c_char,
    rl_buf: [c_char; READLINE_BUF_SIZE],
}

impl Default for ReadlineBuf {
    fn default() -> Self {
        ReadlineBuf {
            rl_cnt: 0,
            rl_bufptr: ptr::null_mut(),
            rl_buf: [0; READLINE_BUF_SIZE],
        }
    }
}

static READLINE_ONCE: Once = Once::new();
static mut READLINE_KEY: Option<std::thread::LocalKey<Box<ReadlineBuf>>> = None;

fn readline_init() {
    unsafe {
        READLINE_KEY = Some(std::thread::LocalKey::new());
    }
}

fn get_readline_buf() -> &'static mut ReadlineBuf {
    READLINE_ONCE.call_once(readline_init);
    
    unsafe {
        READLINE_KEY.as_ref().unwrap().with(|buf| {
            if buf.is_null() {
                let new_buf = Box::new(ReadlineBuf::default());
                *buf = Box::into_raw(new_buf);
            }
            &mut **buf
        })
    }
}

fn pth_readline(fd: c_int, buf: *mut c_void, buflen: SizeT) -> SSizeT {
    pth_readline_ev(fd, buf, buflen, ptr::null_mut())
}

fn pth_readline_ev(
    fd: c_int,
    buf: *mut c_void,
    buflen: SizeT,
    ev_extra: *mut c_void,
) -> SSizeT {
    let mut n: SizeT = 0;
    let mut rc: SSizeT = 0;
    let mut c: c_char = 0;
    let mut cp = buf as *mut c_char;
    let rl = get_readline_buf();

    n = 1;
    while n < buflen {
        rc = 1;
        if rl.rl_cnt <= 0 {
            rl.rl_cnt = pth_read_ev(
                fd,
                rl.rl_buf.as_mut_ptr() as *mut c_void,
                READLINE_BUF_SIZE as SizeT,
                ev_extra,
            ) as c_int;
            
            if rl.rl_cnt < 0 {
                rc = -1;
            } else if rl.rl_cnt == 0 {
                rc = 0;
            } else {
                rl.rl_bufptr = rl.rl_buf.as_mut_ptr();
            }
        }

        if rc == 1 {
            rl.rl_cnt -= 1;
            unsafe {
                c = *rl.rl_bufptr;
                rl.rl_bufptr = rl.rl_bufptr.add(1);
            }

            if c as i32 == '\r' as i32 {
                n -= 1;
            } else {
                unsafe {
                    *cp = c;
                    cp = cp.add(1);
                }
                if c as i32 == '\n' as i32 {
                    break;
                }
            }
            n += 1;
        } else if rc == 0 {
            if n != 1 {
                break;
            }
            return 0;
        } else {
            return -1;
        }
    }

    unsafe {
        *cp = 0;
    }
    n as SSizeT
}

// Placeholder for external C function
extern "C" {
    fn pth_read_ev(
        fd: c_int,
        buf: *mut c_void,
        count: SizeT,
        ev_extra: *mut c_void,
    ) -> SSizeT;
}