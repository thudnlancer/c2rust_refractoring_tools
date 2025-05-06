#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn __errno_location() -> *mut i32;
    fn raise(__sig: i32) -> i32;
    fn abort() -> !;
    fn getprogname() -> *const i8;
    fn strlen(_: *const i8) -> u64;
    fn mempcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn _exit(_: i32) -> !;
    fn sigsegv_install_handler(handler: sigsegv_handler_t) -> i32;
    fn stackoverflow_install_handler(
        handler: stackoverflow_handler_t,
        extra_stack: *mut libc::c_void,
        extra_stack_size: size_t,
    ) -> i32;
    static mut exit_failure: i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type ssize_t = __ssize_t;
pub type __ssize_t = i64;
pub type size_t = u64;
pub type idx_t = ptrdiff_t;
pub type ptrdiff_t = i64;
pub type sigsegv_handler_t = Option<unsafe extern "C" fn(*mut libc::c_void, i32) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: libc::c_longlong,
    pub __clang_max_align_nonce2: f128::f128,
}
pub type stackoverflow_context_t = *mut ucontext_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucontext_t {
    pub uc_flags: u64,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: sigset_t,
    pub __fpregs_mem: _libc_fpstate,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpstate {
    pub cwd: __uint16_t,
    pub swd: __uint16_t,
    pub ftw: __uint16_t,
    pub fop: __uint16_t,
    pub rip: __uint64_t,
    pub rdp: __uint64_t,
    pub mxcsr: __uint32_t,
    pub mxcr_mask: __uint32_t,
    pub _st: [_libc_fpxreg; 8],
    pub _xmm: [_libc_xmmreg; 16],
    pub __glibc_reserved1: [__uint32_t; 24],
}
pub type __uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_xmmreg {
    pub element: [__uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpxreg {
    pub significand: [libc::c_ushort; 4],
    pub exponent: libc::c_ushort,
    pub __glibc_reserved1: [libc::c_ushort; 3],
}
pub type __uint64_t = u64;
pub type __uint16_t = libc::c_ushort;
pub type __sigset_t = sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigset_t {
    pub __val: [u64; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcontext_t {
    pub gregs: gregset_t,
    pub fpregs: fpregset_t,
    pub __reserved1: [libc::c_ulonglong; 8],
}
pub type fpregset_t = *mut _libc_fpstate;
pub type gregset_t = [greg_t; 23];
pub type greg_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut libc::c_void,
    pub ss_flags: i32,
    pub ss_size: size_t,
}
pub type stackoverflow_handler_t = Option<
    unsafe extern "C" fn(i32, stackoverflow_context_t) -> (),
>;
static mut alternate_signal_stack: [max_align_t; 2048] = [max_align_t {
    __clang_max_align_nonce1: 0,
    __clang_max_align_nonce2: f128::f128::ZERO,
}; 2048];
static mut segv_action: Option<unsafe extern "C" fn(i32) -> ()> = None;
static mut program_error_message: *const i8 = 0 as *const i8;
static mut stack_overflow_message: *const i8 = 0 as *const i8;
static mut progname: *const i8 = 0 as *const i8;
unsafe extern "C" fn die(mut signo: i32) {
    segv_action.expect("non-null function pointer")(signo);
    let mut message: *const i8 = if signo != 0 {
        program_error_message
    } else {
        stack_overflow_message
    };
    let mut prognamelen: size_t = strlen(progname);
    let mut messagelen: size_t = strlen(message);
    static mut separator: [i8; 2] = [':' as i32 as i8, ' ' as i32 as i8];
    let mut buf: [i8; 4098] = [0; 4098];
    let mut buflen: idx_t = 0;
    if prognamelen.wrapping_add(messagelen)
        < (::core::mem::size_of::<[i8; 4098]>() as u64)
            .wrapping_sub(::core::mem::size_of::<[i8; 2]>() as u64)
    {
        let mut p: *mut i8 = mempcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            progname as *const libc::c_void,
            prognamelen,
        ) as *mut i8;
        p = mempcpy(
            p as *mut libc::c_void,
            separator.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[i8; 2]>() as u64,
        ) as *mut i8;
        p = mempcpy(p as *mut libc::c_void, message as *const libc::c_void, messagelen)
            as *mut i8;
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = '\n' as i32 as i8;
        buflen = p.offset_from(buf.as_mut_ptr()) as i64;
    } else {
        write(2 as i32, progname as *const libc::c_void, prognamelen);
        write(
            2 as i32,
            separator.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[i8; 2]>() as u64,
        );
        write(2 as i32, message as *const libc::c_void, messagelen);
        buf[0 as i32 as usize] = '\n' as i32 as i8;
        buflen = 1 as i32 as idx_t;
    }
    write(2 as i32, buf.as_mut_ptr() as *const libc::c_void, buflen as size_t);
    if signo == 0 {
        _exit(exit_failure);
    }
    raise(signo);
    abort();
}
unsafe extern "C" fn null_action(mut signo: i32) {}
static mut segv_handler_missing: i32 = 0;
unsafe extern "C" fn segv_handler(
    mut address: *mut libc::c_void,
    mut serious: i32,
) -> i32 {
    if serious == 0 {
        return 0 as i32;
    }
    die(11 as i32);
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn overflow_handler(
    mut emergency: i32,
    mut context: stackoverflow_context_t,
) {
    die(if emergency == 0 || segv_handler_missing != 0 { 0 as i32 } else { 11 as i32 });
}
#[no_mangle]
pub unsafe extern "C" fn c_stack_action(
    mut action: Option<unsafe extern "C" fn(i32) -> ()>,
) -> i32 {
    ::core::ptr::write_volatile(
        &mut segv_action as *mut Option<unsafe extern "C" fn(i32) -> ()>,
        if action.is_some() {
            action
        } else {
            Some(null_action as unsafe extern "C" fn(i32) -> ())
        },
    );
    ::core::ptr::write_volatile(
        &mut program_error_message as *mut *const i8,
        dcgettext(0 as *const i8, b"program error\0" as *const u8 as *const i8, 5 as i32),
    );
    ::core::ptr::write_volatile(
        &mut stack_overflow_message as *mut *const i8,
        dcgettext(
            0 as *const i8,
            b"stack overflow\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    ::core::ptr::write_volatile(&mut progname as *mut *const i8, getprogname());
    if stackoverflow_install_handler(
        Some(
            overflow_handler as unsafe extern "C" fn(i32, stackoverflow_context_t) -> (),
        ),
        alternate_signal_stack.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[max_align_t; 2048]>() as u64,
    ) != 0
    {
        *__errno_location() = 95 as i32;
        return -(1 as i32);
    }
    ::core::ptr::write_volatile(
        &mut segv_handler_missing as *mut i32,
        sigsegv_install_handler(
            Some(segv_handler as unsafe extern "C" fn(*mut libc::c_void, i32) -> i32),
        ),
    );
    return 0 as i32;
}