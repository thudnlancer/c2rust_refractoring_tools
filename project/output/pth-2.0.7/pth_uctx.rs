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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn __errno_location() -> *mut i32;
    fn swapcontext(__oucp: *mut ucontext_t, __ucp: *const ucontext_t) -> i32;
    fn setcontext(__ucp: *const ucontext_t) -> i32;
    fn __pth_mctx_set(
        _: *mut pth_mctx_t,
        _: Option<unsafe extern "C" fn() -> ()>,
        _: *mut i8,
        _: *mut i8,
    ) -> i32;
}
pub type size_t = u64;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
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
pub type pth_mctx_t = pth_mctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_mctx_st {
    pub uc: ucontext_t,
    pub restored: i32,
    pub sigs: sigset_t,
    pub error: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_uctx_st {
    pub uc_stack_own: i32,
    pub uc_stack_ptr: *mut i8,
    pub uc_stack_len: size_t,
    pub uc_mctx_set: i32,
    pub uc_mctx: pth_mctx_t,
}
pub type pth_uctx_t = *mut pth_uctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_uctx_trampoline_t {
    pub mctx_parent: *mut pth_mctx_t,
    pub uctx_this: pth_uctx_t,
    pub uctx_after: pth_uctx_t,
    pub start_func: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub start_arg: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn pth_uctx_create(mut puctx: *mut pth_uctx_t) -> i32 {
    let mut uctx: pth_uctx_t = 0 as *mut pth_uctx_st;
    if puctx.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    uctx = malloc(::core::mem::size_of::<pth_uctx_st>() as u64) as pth_uctx_t;
    if uctx.is_null() {
        *__errno_location() = *__errno_location();
        return 0 as i32;
    }
    (*uctx).uc_stack_own = 0 as i32;
    (*uctx).uc_stack_ptr = 0 as *mut i8;
    (*uctx).uc_stack_len = 0 as i32 as size_t;
    (*uctx).uc_mctx_set = 0 as i32;
    memset(
        &mut (*uctx).uc_mctx as *mut pth_mctx_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<pth_mctx_t>() as u64,
    );
    *puctx = uctx;
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub static mut pth_uctx_trampoline_ctx: pth_uctx_trampoline_t = pth_uctx_trampoline_t {
    mctx_parent: 0 as *const pth_mctx_t as *mut pth_mctx_t,
    uctx_this: 0 as *const pth_uctx_st as *mut pth_uctx_st,
    uctx_after: 0 as *const pth_uctx_st as *mut pth_uctx_st,
    start_func: None,
    start_arg: 0 as *const libc::c_void as *mut libc::c_void,
};
unsafe extern "C" fn pth_uctx_trampoline() {
    let mut ctx: pth_uctx_trampoline_t = pth_uctx_trampoline_t {
        mctx_parent: 0 as *const pth_mctx_t as *mut pth_mctx_t,
        uctx_this: 0 as *const pth_uctx_st as *mut pth_uctx_st,
        uctx_after: 0 as *const pth_uctx_st as *mut pth_uctx_st,
        start_func: None,
        start_arg: 0 as *const libc::c_void as *mut libc::c_void,
    };
    ::core::ptr::write_volatile(
        &mut ctx.mctx_parent as *mut *mut pth_mctx_t,
        pth_uctx_trampoline_ctx.mctx_parent,
    );
    ::core::ptr::write_volatile(
        &mut ctx.uctx_this as *mut pth_uctx_t,
        pth_uctx_trampoline_ctx.uctx_this,
    );
    ::core::ptr::write_volatile(
        &mut ctx.uctx_after as *mut pth_uctx_t,
        pth_uctx_trampoline_ctx.uctx_after,
    );
    ::core::ptr::write_volatile(
        &mut ctx.start_func
            as *mut Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        pth_uctx_trampoline_ctx.start_func,
    );
    ::core::ptr::write_volatile(
        &mut ctx.start_arg as *mut *mut libc::c_void,
        pth_uctx_trampoline_ctx.start_arg,
    );
    swapcontext(&mut (*ctx.uctx_this).uc_mctx.uc, &mut (*ctx.mctx_parent).uc);
    (Some((ctx.start_func).expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx.start_arg);
    if !(ctx.uctx_after).is_null() {
        *__errno_location() = (*ctx.uctx_after).uc_mctx.error;
        (*ctx.uctx_after).uc_mctx.restored = 1 as i32;
        setcontext(&mut (*ctx.uctx_after).uc_mctx.uc);
    }
    exit(0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn pth_uctx_make(
    mut uctx: pth_uctx_t,
    mut sk_addr: *mut i8,
    mut sk_size: size_t,
    mut sigmask: *const sigset_t,
    mut start_func: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut start_arg: *mut libc::c_void,
    mut uctx_after: pth_uctx_t,
) -> i32 {
    let mut mctx_parent: pth_mctx_t = pth_mctx_t {
        uc: ucontext_t {
            uc_flags: 0,
            uc_link: 0 as *mut ucontext_t,
            uc_stack: stack_t {
                ss_sp: 0 as *mut libc::c_void,
                ss_flags: 0,
                ss_size: 0,
            },
            uc_mcontext: mcontext_t {
                gregs: [0; 23],
                fpregs: 0 as *mut _libc_fpstate,
                __reserved1: [0; 8],
            },
            uc_sigmask: sigset_t { __val: [0; 16] },
            __fpregs_mem: _libc_fpstate {
                cwd: 0,
                swd: 0,
                ftw: 0,
                fop: 0,
                rip: 0,
                rdp: 0,
                mxcsr: 0,
                mxcr_mask: 0,
                _st: [_libc_fpxreg {
                    significand: [0; 4],
                    exponent: 0,
                    __glibc_reserved1: [0; 3],
                }; 8],
                _xmm: [_libc_xmmreg { element: [0; 4] }; 16],
                __glibc_reserved1: [0; 24],
            },
        },
        restored: 0,
        sigs: sigset_t { __val: [0; 16] },
        error: 0,
    };
    let mut ss: sigset_t = sigset_t { __val: [0; 16] };
    if uctx.is_null() || start_func.is_none()
        || sk_size < (16 as i32 * 1024 as i32) as u64
    {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if sk_addr.is_null() {
        sk_addr = malloc(sk_size) as *mut i8;
        if sk_addr.is_null() {
            *__errno_location() = *__errno_location();
            return 0 as i32;
        }
        (*uctx).uc_stack_own = (0 as i32 == 0) as i32;
    } else {
        (*uctx).uc_stack_own = 0 as i32;
    }
    (*uctx).uc_stack_ptr = sk_addr;
    (*uctx).uc_stack_len = sk_size;
    if __pth_mctx_set(
        &mut (*uctx).uc_mctx,
        Some(pth_uctx_trampoline as unsafe extern "C" fn() -> ()),
        (*uctx).uc_stack_ptr,
        ((*uctx).uc_stack_ptr).offset((*uctx).uc_stack_len as isize),
    ) == 0
    {
        *__errno_location() = *__errno_location();
        return 0 as i32;
    }
    pth_uctx_trampoline_ctx.mctx_parent = &mut mctx_parent;
    pth_uctx_trampoline_ctx.uctx_this = uctx;
    pth_uctx_trampoline_ctx.uctx_after = uctx_after;
    pth_uctx_trampoline_ctx.start_func = start_func;
    pth_uctx_trampoline_ctx.start_arg = start_arg;
    if !sigmask.is_null() {
        sigprocmask(2 as i32, sigmask, &mut ss);
    }
    swapcontext(&mut mctx_parent.uc, &mut (*uctx).uc_mctx.uc);
    if !sigmask.is_null() {
        sigprocmask(2 as i32, &mut ss, 0 as *mut sigset_t);
    }
    (*uctx).uc_mctx_set = (0 as i32 == 0) as i32;
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_uctx_switch(
    mut uctx_from: pth_uctx_t,
    mut uctx_to: pth_uctx_t,
) -> i32 {
    if uctx_from.is_null() || uctx_to.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if (*uctx_to).uc_mctx_set == 0 {
        *__errno_location() = 1 as i32;
        return 0 as i32;
    }
    (*uctx_from).uc_mctx_set = (0 as i32 == 0) as i32;
    swapcontext(&mut (*uctx_from).uc_mctx.uc, &mut (*uctx_to).uc_mctx.uc);
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_uctx_destroy(mut uctx: pth_uctx_t) -> i32 {
    if uctx.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if (*uctx).uc_stack_own != 0 && !((*uctx).uc_stack_ptr).is_null() {
        free((*uctx).uc_stack_ptr as *mut libc::c_void);
    }
    free(uctx as *mut libc::c_void);
    return (0 as i32 == 0) as i32;
}