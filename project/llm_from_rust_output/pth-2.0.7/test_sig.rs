use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_ulong, c_void};
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

type PthSt = c_void;
type PthAttrSt = c_void;
type PthT = *mut PthSt;
type PthAttrT = *mut PthAttrSt;

#[repr(C)]
struct Sigset {
    __val: [c_ulong; 16],
}

static CHILD1: AtomicPtr<PthSt> = AtomicPtr::new(ptr::null_mut());
static CHILD2: AtomicPtr<PthSt> = AtomicPtr::new(ptr::null_mut());

extern "C" {
    fn fprintf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn sigemptyset(set: *mut Sigset) -> c_int;
    fn sigaddset(set: *mut Sigset, signo: c_int) -> c_int;
    fn sigdelset(set: *mut Sigset, signo: c_int) -> c_int;
    fn sigismember(set: *const Sigset, signo: c_int) -> c_int;
    fn pth_init() -> c_int;
    fn pth_kill() -> c_int;
    fn pth_attr_new() -> PthAttrT;
    fn pth_attr_set(attr: PthAttrT, field: c_int, ...) -> c_int;
    fn pth_attr_destroy(attr: PthAttrT) -> c_int;
    fn pth_spawn(
        attr: PthAttrT,
        func: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> PthT;
    fn pth_cancel(thread: PthT) -> c_int;
    fn pth_join(thread: PthT, value: *mut *mut c_void) -> c_int;
    fn pth_cleanup_push(
        func: Option<unsafe extern "C" fn(*mut c_void)>,
        arg: *mut c_void,
    ) -> c_int;
    fn pth_sleep(sec: c_uint) -> c_uint;
    fn pth_sigmask(how: c_int, set: *const Sigset, oset: *mut Sigset) -> c_int;
    fn pth_sigwait(set: *const Sigset, sig: *mut c_int) -> c_int;
}

const SIGINT: c_int = 2;
const SIGUSR1: c_int = 10;
const SIGUSR2: c_int = 12;

fn inthandler(_arg: *mut c_void) -> *mut c_void {
    let mut sigs = Sigset { __val: [0; 16] };
    let mut sig = 0;
    let mut n = 0;

    unsafe {
        fprintf(
            libc::stderr,
            b"inthandler: enter\n\0".as_ptr() as *const c_char,
        );
        sigemptyset(&mut sigs);
        sigaddset(&mut sigs, SIGINT);
        pth_sigmask(1, &mut sigs, ptr::null_mut());

        while n < 3 {
            pth_sigwait(&mut sigs, &mut sig);
            fprintf(
                libc::stderr,
                b"inthandler: SIGINT received (#%d)\n\0".as_ptr() as *const c_char,
                n,
            );
            n += 1;
        }

        fprintf(
            libc::stderr,
            b"inthandler: cancelling child1 and child2\n\0".as_ptr() as *const c_char,
        );
        pth_cancel(CHILD1.load(Ordering::SeqCst));
        pth_cancel(CHILD2.load(Ordering::SeqCst));
        fprintf(
            libc::stderr,
            b"inthandler: leave\n\0".as_ptr() as *const c_char,
        );
    }

    ptr::null_mut()
}

fn child_cleanup(arg: *mut c_void) {
    unsafe {
        fprintf(
            libc::stderr,
            b"%s: running cleanup\n\0".as_ptr() as *const c_char,
            arg as *mut c_char,
        );
    }
}

fn child(_arg: *mut c_void) -> *mut c_void {
    let mut sigs = Sigset { __val: [0; 16] };
    let name = unsafe { _arg as *mut c_char };
    let mut i = 0;

    unsafe {
        fprintf(libc::stderr, b"%s: enter\n\0".as_ptr() as *const c_char, name);
        pth_cleanup_push(Some(child_cleanup), name as *mut c_void);
        pth_sigmask(2, ptr::null(), &mut sigs);

        sigaddset(&mut sigs, SIGINT);
        if strcmp(name, b"child1\0".as_ptr() as *const c_char) == 0 {
            sigaddset(&mut sigs, SIGUSR1);
            sigdelset(&mut sigs, SIGUSR2);
        } else {
            sigdelset(&mut sigs, SIGUSR1);
            sigaddset(&mut sigs, SIGUSR2);
        }
        pth_sigmask(2, &mut sigs, ptr::null_mut());

        while i < 10 {
            pth_sigmask(2, ptr::null(), &mut sigs);
            fprintf(
                libc::stderr,
                b"%s: SIGUSR1: %s\n\0".as_ptr() as *const c_char,
                name,
                if sigismember(&mut sigs, SIGUSR1) != 0 {
                    b"blocked\0".as_ptr() as *const c_char
                } else {
                    b"unblocked\0".as_ptr() as *const c_char
                },
            );
            fprintf(
                libc::stderr,
                b"%s: SIGUSR2: %s\n\0".as_ptr() as *const c_char,
                name,
                if sigismember(&mut sigs, SIGUSR2) != 0 {
                    b"blocked\0".as_ptr() as *const c_char
                } else {
                    b"unblocked\0".as_ptr() as *const c_char
                },
            );
            fprintf(
                libc::stderr,
                b"%s: leave to scheduler\n\0".as_ptr() as *const c_char,
                name,
            );
            pth_sleep(1);
            fprintf(
                libc::stderr,
                b"%s: reentered from scheduler\n\0".as_ptr() as *const c_char,
                name,
            );
            i += 1;
        }

        fprintf(libc::stderr, b"%s: leave\n\0".as_ptr() as *const c_char, name);
    }

    ptr::null_mut()
}

fn main() {
    unsafe {
        pth_init();
        fprintf(
            libc::stderr,
            b"This is TEST_SIG, a Pth test using signals.\n\0".as_ptr() as *const c_char,
        );
        fprintf(libc::stderr, b"\n\0".as_ptr() as *const c_char);
        fprintf(
            libc::stderr,
            b"Hit CTRL-C three times to stop this test.\n\0".as_ptr() as *const c_char,
        );
        fprintf(
            libc::stderr,
            b"But only after all threads were terminated.\n\0".as_ptr() as *const c_char,
        );
        fprintf(libc::stderr, b"\n\0".as_ptr() as *const c_char);
        fprintf(
            libc::stderr,
            b"main: init\n\0".as_ptr() as *const c_char,
        );

        let mut sigs = Sigset { __val: [0; 16] };
        sigemptyset(&mut sigs);
        sigaddset(&mut sigs, SIGUSR1);
        sigaddset(&mut sigs, SIGUSR2);
        sigaddset(&mut sigs, SIGINT);
        pth_sigmask(2, &mut sigs, ptr::null_mut());

        let attr = pth_attr_new();
        pth_attr_set(
            attr,
            1,
            b"child1\0".as_ptr() as *const c_char,
        );
        CHILD1.store(
            pth_spawn(attr, Some(child), b"child1\0".as_ptr() as *mut c_void),
            Ordering::SeqCst,
        );

        pth_attr_set(
            attr,
            1,
            b"child2\0".as_ptr() as *const c_char,
        );
        CHILD2.store(
            pth_spawn(attr, Some(child), b"child2\0".as_ptr() as *mut c_void),
            Ordering::SeqCst,
        );

        pth_attr_set(
            attr,
            1,
            b"inthandler\0".as_ptr() as *const c_char,
        );
        pth_spawn(
            attr,
            Some(inthandler),
            b"inthandler\0".as_ptr() as *mut c_void,
        );

        pth_attr_destroy(attr);

        while pth_join(ptr::null_mut(), ptr::null_mut()) != 0 {}

        fprintf(
            libc::stderr,
            b"main: exit\n\0".as_ptr() as *const c_char,
        );
        pth_kill();
    }
}