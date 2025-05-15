use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};
use std::process;
use std::ptr;
use std::slice;

const EXIT_FAILURE: c_int = 1;
const SIGSEGV: c_int = 11;
const ENOSYS: c_int = 95;

static mut ALTERNATE_SIGNAL_STACK: [u8; 2048 * std::mem::size_of::<usize>()] = [0; 2048 * std::mem::size_of::<usize>()];
static mut SEGV_ACTION: Option<fn(c_int)> = None;
static mut PROGRAM_ERROR_MESSAGE: *const c_char = ptr::null();
static mut STACK_OVERFLOW_MESSAGE: *const c_char = ptr::null();
static mut PROGNAME: *const c_char = ptr::null();
static mut SEGV_HANDLER_MISSING: c_int = 0;

type SigsegvHandler = unsafe extern "C" fn(*mut c_void, c_int) -> c_int;
type StackoverflowHandler = unsafe extern "C" fn(c_int, *mut c_void);

extern "C" {
    fn getprogname() -> *const c_char;
    fn dcgettext(domainname: *const c_char, msgid: *const c_char, category: c_int) -> *mut c_char;
    fn sigsegv_install_handler(handler: Option<SigsegvHandler>) -> c_int;
    fn stackoverflow_install_handler(
        handler: Option<StackoverflowHandler>,
        extra_stack: *mut c_void,
        extra_stack_size: usize,
    ) -> c_int;
}

fn die(signo: c_int) {
    unsafe {
        if let Some(action) = SEGV_ACTION {
            action(signo);
        }

        let message = if signo != 0 {
            PROGRAM_ERROR_MESSAGE
        } else {
            STACK_OVERFLOW_MESSAGE
        };

        let progname = CStr::from_ptr(PROGNAME).to_bytes();
        let message = CStr::from_ptr(message).to_bytes();
        let separator = b": ";

        let mut buf = Vec::with_capacity(progname.len() + separator.len() + message.len() + 1);
        buf.extend_from_slice(progname);
        buf.extend_from_slice(separator);
        buf.extend_from_slice(message);
        buf.push(b'\n');

        let stderr = std::io::stderr();
        let _ = stderr.lock().write_all(&buf);

        if signo == 0 {
            process::exit(EXIT_FAILURE);
        } else {
            process::abort();
        }
    }
}

fn null_action(_signo: c_int) {}

unsafe extern "C" fn segv_handler(_address: *mut c_void, serious: c_int) -> c_int {
    if serious == 0 {
        return 0;
    }
    die(SIGSEGV);
    0
}

unsafe extern "C" fn overflow_handler(emergency: c_int, _context: *mut c_void) {
    die(if emergency == 0 || SEGV_HANDLER_MISSING != 0 {
        0
    } else {
        SIGSEGV
    });
}

pub fn c_stack_action(action: Option<fn(c_int)>) -> c_int {
    unsafe {
        SEGV_ACTION = action.or(Some(null_action));
        PROGRAM_ERROR_MESSAGE = dcgettext(
            ptr::null(),
            b"program error\0".as_ptr() as *const c_char,
            5,
        );
        STACK_OVERFLOW_MESSAGE = dcgettext(
            ptr::null(),
            b"stack overflow\0".as_ptr() as *const c_char,
            5,
        );
        PROGNAME = getprogname();

        if stackoverflow_install_handler(
            Some(overflow_handler),
            ALTERNATE_SIGNAL_STACK.as_mut_ptr() as *mut c_void,
            ALTERNATE_SIGNAL_STACK.len(),
        ) != 0
        {
            ptr::write_volatile(&mut errno(), ENOSYS);
            return -1;
        }

        SEGV_HANDLER_MISSING = sigsegv_install_handler(Some(segv_handler));
        0
    }
}

fn errno() -> c_int {
    unsafe { *libc::__errno_location() }
}