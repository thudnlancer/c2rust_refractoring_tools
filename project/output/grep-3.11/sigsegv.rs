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
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaddset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigaction(__sig: i32, __act: *const sigaction, __oact: *mut sigaction) -> i32;
    fn sigaltstack(__ss: *const stack_t, __oss: *mut stack_t) -> i32;
    fn __errno_location() -> *mut i32;
    fn perror(__s: *const i8);
    fn sigsegv_get_vma(address: uintptr_t, vma: *mut vma_struct) -> i32;
}
pub type size_t = u64;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __uid_t = u32;
pub type __pid_t = i32;
pub type __clock_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut libc::c_void,
    pub ss_flags: i32,
    pub ss_size: size_t,
}
pub type greg_t = libc::c_longlong;
pub type gregset_t = [greg_t; 23];
pub type C2RustUnnamed = u32;
pub const REG_CR2: C2RustUnnamed = 22;
pub const REG_OLDMASK: C2RustUnnamed = 21;
pub const REG_TRAPNO: C2RustUnnamed = 20;
pub const REG_ERR: C2RustUnnamed = 19;
pub const REG_CSGSFS: C2RustUnnamed = 18;
pub const REG_EFL: C2RustUnnamed = 17;
pub const REG_RIP: C2RustUnnamed = 16;
pub const REG_RSP: C2RustUnnamed = 15;
pub const REG_RCX: C2RustUnnamed = 14;
pub const REG_RAX: C2RustUnnamed = 13;
pub const REG_RDX: C2RustUnnamed = 12;
pub const REG_RBX: C2RustUnnamed = 11;
pub const REG_RBP: C2RustUnnamed = 10;
pub const REG_RSI: C2RustUnnamed = 9;
pub const REG_RDI: C2RustUnnamed = 8;
pub const REG_R15: C2RustUnnamed = 7;
pub const REG_R14: C2RustUnnamed = 6;
pub const REG_R13: C2RustUnnamed = 5;
pub const REG_R12: C2RustUnnamed = 4;
pub const REG_R11: C2RustUnnamed = 3;
pub const REG_R10: C2RustUnnamed = 2;
pub const REG_R9: C2RustUnnamed = 1;
pub const REG_R8: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpxreg {
    pub significand: [libc::c_ushort; 4],
    pub exponent: libc::c_ushort,
    pub __glibc_reserved1: [libc::c_ushort; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_xmmreg {
    pub element: [__uint32_t; 4],
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
pub type fpregset_t = *mut _libc_fpstate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcontext_t {
    pub gregs: gregset_t,
    pub fpregs: fpregset_t,
    pub __reserved1: [libc::c_ulonglong; 8],
}
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
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _pad: [i32; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: i32,
    pub _arch: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_band: i64,
    pub si_fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: i32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type C2RustUnnamed_11 = u32;
pub const SS_DISABLE: C2RustUnnamed_11 = 2;
pub const SS_ONSTACK: C2RustUnnamed_11 = 1;
pub type sigsegv_handler_t = Option<unsafe extern "C" fn(*mut libc::c_void, i32) -> i32>;
pub type stackoverflow_handler_t = Option<
    unsafe extern "C" fn(i32, stackoverflow_context_t) -> (),
>;
pub type stackoverflow_context_t = *mut ucontext_t;
pub type uintptr_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vma_struct {
    pub start: uintptr_t,
    pub end: uintptr_t,
    pub is_near_this: Option<unsafe extern "C" fn(uintptr_t, *mut vma_struct) -> i32>,
    pub prev_end: uintptr_t,
}
#[no_mangle]
pub static mut libsigsegv_version: i32 = 0x20d as i32;
unsafe extern "C" fn sigsegv_reset_onstack_flag() {}
static mut stack_top: uintptr_t = 0 as i32 as uintptr_t;
unsafe extern "C" fn remember_stack_top(mut some_variable_on_stack: *mut libc::c_void) {
    let mut vma: vma_struct = vma_struct {
        start: 0,
        end: 0,
        is_near_this: None,
        prev_end: 0,
    };
    if sigsegv_get_vma(some_variable_on_stack as uintptr_t, &mut vma) >= 0 as i32 {
        stack_top = (vma.end).wrapping_sub(1 as i32 as u64);
    }
}
static mut stk_user_handler: stackoverflow_handler_t = unsafe {
    ::core::mem::transmute::<
        *mut libc::c_void,
        stackoverflow_handler_t,
    >(0 as *const libc::c_void as *mut libc::c_void)
};
static mut stk_extra_stack: uintptr_t = 0;
static mut stk_extra_stack_size: size_t = 0;
static mut user_handler: sigsegv_handler_t = unsafe {
    ::core::mem::transmute::<
        *mut libc::c_void,
        sigsegv_handler_t,
    >(0 as *const libc::c_void as *mut libc::c_void)
};
unsafe extern "C" fn sigsegv_handler(
    mut sig: i32,
    mut sip: *mut siginfo_t,
    mut ucp: *mut libc::c_void,
) {
    let mut address: *mut libc::c_void = (*sip)._sifields._sigfault.si_addr;
    if !(user_handler.is_some()
        && (Some(user_handler.expect("non-null function pointer")))
            .expect("non-null function pointer")(address, 0 as i32) != 0)
    {
        if stk_user_handler.is_some() {
            let mut old_sp: uintptr_t = (*(ucp as *mut ucontext_t))
                .uc_mcontext
                .gregs[REG_RSP as i32 as usize] as uintptr_t;
            if stack_top != 0 {
                let mut saved_errno: i32 = 0;
                let mut vma: vma_struct = vma_struct {
                    start: 0,
                    end: 0,
                    is_near_this: None,
                    prev_end: 0,
                };
                let mut ret: i32 = 0;
                saved_errno = *__errno_location();
                ret = sigsegv_get_vma(stack_top, &mut vma);
                *__errno_location() = saved_errno;
                if ret >= 0 as i32 {
                    let mut addr: uintptr_t = address as uintptr_t;
                    if if addr >= vma.start {
                        (addr <= (vma.end).wrapping_sub(1 as i32 as u64)) as i32
                    } else {
                        (vma.is_near_this)
                            .expect("non-null function pointer")(addr, &mut vma)
                    } != 0
                    {
                        let mut emergency: i32 = (old_sp >= stk_extra_stack
                            && old_sp
                                <= stk_extra_stack.wrapping_add(stk_extra_stack_size))
                            as i32;
                        let mut context: stackoverflow_context_t = ucp
                            as *mut ucontext_t;
                        (Some(stk_user_handler.expect("non-null function pointer")))
                            .expect("non-null function pointer")(emergency, context);
                    }
                }
            }
        }
        if !(user_handler.is_some()
            && (Some(user_handler.expect("non-null function pointer")))
                .expect("non-null function pointer")(address, 1 as i32) != 0)
        {
            let mut signo: i32 = 0;
            signo = 11 as i32;
            signal(signo, None);
        }
    }
}
unsafe extern "C" fn install_for(mut sig: i32) {
    let mut action: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    action.__sigaction_handler.sa_sigaction = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void) -> ()>,
        Option<unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void) -> ()>,
    >(
        Some(
            sigsegv_handler
                as unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void) -> (),
        ),
    );
    sigemptyset(&mut action.sa_mask);
    sigaddset(&mut action.sa_mask, 1 as i32);
    sigaddset(&mut action.sa_mask, 2 as i32);
    sigaddset(&mut action.sa_mask, 3 as i32);
    sigaddset(&mut action.sa_mask, 13 as i32);
    sigaddset(&mut action.sa_mask, 14 as i32);
    sigaddset(&mut action.sa_mask, 15 as i32);
    sigaddset(&mut action.sa_mask, 10 as i32);
    sigaddset(&mut action.sa_mask, 12 as i32);
    sigaddset(&mut action.sa_mask, 17 as i32);
    sigaddset(&mut action.sa_mask, 17 as i32);
    sigaddset(&mut action.sa_mask, 23 as i32);
    sigaddset(&mut action.sa_mask, 29 as i32);
    sigaddset(&mut action.sa_mask, 29 as i32);
    sigaddset(&mut action.sa_mask, 24 as i32);
    sigaddset(&mut action.sa_mask, 25 as i32);
    sigaddset(&mut action.sa_mask, 26 as i32);
    sigaddset(&mut action.sa_mask, 27 as i32);
    sigaddset(&mut action.sa_mask, 30 as i32);
    sigaddset(&mut action.sa_mask, 28 as i32);
    action.sa_flags = 4 as i32;
    if stk_user_handler.is_some() {
        action.sa_flags |= 0x8000000 as i32;
    }
    sigaction(sig, &mut action, 0 as *mut libc::c_void as *mut sigaction);
}
#[no_mangle]
pub unsafe extern "C" fn sigsegv_install_handler(mut handler: sigsegv_handler_t) -> i32 {
    user_handler = handler;
    let mut sig: i32 = 0;
    sig = 11 as i32;
    install_for(sig);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn sigsegv_deinstall_handler() {
    user_handler = ::core::mem::transmute::<
        *mut libc::c_void,
        sigsegv_handler_t,
    >(0 as *mut libc::c_void);
    if stk_user_handler.is_none() {
        let mut sig: i32 = 0;
        sig = 11 as i32;
        signal(sig, None);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sigsegv_leave_handler(
    mut continuation: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> (),
    >,
    mut cont_arg1: *mut libc::c_void,
    mut cont_arg2: *mut libc::c_void,
    mut cont_arg3: *mut libc::c_void,
) -> i32 {
    sigsegv_reset_onstack_flag();
    (Some(continuation.expect("non-null function pointer")))
        .expect("non-null function pointer")(cont_arg1, cont_arg2, cont_arg3);
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn stackoverflow_install_handler(
    mut handler: stackoverflow_handler_t,
    mut extra_stack: *mut libc::c_void,
    mut extra_stack_size: size_t,
) -> i32 {
    if stack_top == 0 {
        let mut dummy: i32 = 0;
        remember_stack_top(&mut dummy as *mut i32 as *mut libc::c_void);
        if stack_top == 0 {
            return -(1 as i32);
        }
    }
    stk_user_handler = handler;
    stk_extra_stack = extra_stack as uintptr_t;
    stk_extra_stack_size = extra_stack_size;
    let mut ss: stack_t = stack_t {
        ss_sp: 0 as *mut libc::c_void,
        ss_flags: 0,
        ss_size: 0,
    };
    ss.ss_sp = extra_stack;
    ss.ss_size = extra_stack_size;
    ss.ss_flags = 0 as i32;
    if sigaltstack(&mut ss, 0 as *mut stack_t) < 0 as i32 {
        return -(1 as i32);
    }
    let mut sig: i32 = 0;
    sig = 11 as i32;
    install_for(sig);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn stackoverflow_deinstall_handler() {
    stk_user_handler = ::core::mem::transmute::<
        *mut libc::c_void,
        stackoverflow_handler_t,
    >(0 as *mut libc::c_void);
    if user_handler.is_some() {
        let mut sig: i32 = 0;
        sig = 11 as i32;
        install_for(sig);
    } else {
        let mut sig_0: i32 = 0;
        sig_0 = 11 as i32;
        signal(sig_0, None);
    }
    let mut ss: stack_t = stack_t {
        ss_sp: 0 as *mut libc::c_void,
        ss_flags: 0,
        ss_size: 0,
    };
    ss.ss_flags = SS_DISABLE as i32;
    if sigaltstack(&mut ss, 0 as *mut stack_t) < 0 as i32 {
        perror(
            b"gnulib sigsegv (stackoverflow_deinstall_handler)\0" as *const u8
                as *const i8,
        );
    }
}