use std::os::raw::{c_int, c_void, c_ulong, c_ushort, c_uint, c_long, c_ulonglong};
use std::ptr;
use std::mem;

type size_t = c_ulong;
type uintptr_t = c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
struct Sigset {
    __val: [c_ulong; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct StackT {
    ss_sp: *mut c_void,
    ss_flags: c_int,
    ss_size: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VmaStruct {
    start: uintptr_t,
    end: uintptr_t,
    is_near_this: Option<extern "C" fn(uintptr_t, *mut VmaStruct) -> c_int>,
    prev_end: uintptr_t,
}

type SigsegvHandler = Option<extern "C" fn(*mut c_void, c_int) -> c_int>;
type StackOverflowHandler = Option<extern "C" fn(c_int, *mut ucontext_t) -> ()>;

static mut STACK_TOP: uintptr_t = 0;
static mut STK_USER_HANDLER: StackOverflowHandler = None;
static mut STK_EXTRA_STACK: uintptr_t = 0;
static mut STK_EXTRA_STACK_SIZE: size_t = 0;
static mut USER_HANDLER: SigsegvHandler = None;

extern "C" {
    fn sigemptyset(set: *mut Sigset) -> c_int;
    fn sigaddset(set: *mut Sigset, signo: c_int) -> c_int;
    fn sigaction(sig: c_int, act: *const Sigaction, oact: *mut Sigaction) -> c_int;
    fn sigaltstack(ss: *const StackT, oss: *mut StackT) -> c_int;
    fn __errno_location() -> *mut c_int;
    fn perror(s: *const i8);
    fn sigsegv_get_vma(address: uintptr_t, vma: *mut VmaStruct) -> c_int;
    fn signal(sig: c_int, handler: Option<extern "C" fn(c_int)>) -> Option<extern "C" fn(c_int)>>;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Sigaction {
    sa_sigaction: Option<extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    sa_mask: Sigset,
    sa_flags: c_int,
    sa_restorer: Option<extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct siginfo_t {
    si_signo: c_int,
    si_errno: c_int,
    si_code: c_int,
    __pad0: c_int,
    _sifields: SigFields,
}

#[repr(C)]
#[derive(Copy, Clone)]
union SigFields {
    _pad: [c_int; 28],
    _sigfault: SigFault,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SigFault {
    si_addr: *mut c_void,
    si_addr_lsb: c_short,
    _bounds: SigBounds,
}

#[repr(C)]
#[derive(Copy, Clone)]
union SigBounds {
    _addr_bnd: AddrBound,
    _pkey: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct AddrBound {
    _lower: *mut c_void,
    _upper: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ucontext_t {
    uc_flags: c_ulong,
    uc_link: *mut ucontext_t,
    uc_stack: StackT,
    uc_mcontext: mcontext_t,
    uc_sigmask: Sigset,
    __fpregs_mem: FpState,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct mcontext_t {
    gregs: [c_longlong; 23],
    fpregs: *mut FpState,
    __reserved1: [c_ulonglong; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FpState {
    cwd: c_ushort,
    swd: c_ushort,
    ftw: c_ushort,
    fop: c_ushort,
    rip: c_ulong,
    rdp: c_ulong,
    mxcsr: c_uint,
    mxcr_mask: c_uint,
    _st: [FpxReg; 8],
    _xmm: [XmmReg; 16],
    __glibc_reserved1: [c_uint; 24],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FpxReg {
    significand: [c_ushort; 4],
    exponent: c_ushort,
    __glibc_reserved1: [c_ushort; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct XmmReg {
    element: [c_uint; 4],
}

const SS_DISABLE: c_int = 2;
const REG_RSP: usize = 15;

unsafe extern "C" fn sigsegv_handler(sig: c_int, sip: *mut siginfo_t, ucp: *mut c_void) {
    let address = (*sip)._sigfault.si_addr;
    
    if let Some(handler) = USER_HANDLER {
        if handler(address, 0) != 0 {
            return;
        }
    }

    if let Some(stk_handler) = STK_USER_HANDLER {
        let old_sp = (*(ucp as *mut ucontext_t)).uc_mcontext.gregs[REG_RSP] as uintptr_t;
        
        if STACK_TOP != 0 {
            let saved_errno = *__errno_location();
            let mut vma = VmaStruct {
                start: 0,
                end: 0,
                is_near_this: None,
                prev_end: 0,
            };
            
            let ret = sigsegv_get_vma(STACK_TOP, &mut vma);
            *__errno_location() = saved_errno;
            
            if ret >= 0 {
                let addr = address as uintptr_t;
                let in_range = if addr >= vma.start {
                    addr <= vma.end.wrapping_sub(1)
                } else if let Some(is_near) = vma.is_near_this {
                    is_near(addr, &mut vma) != 0
                } else {
                    false
                };
                
                if in_range {
                    let emergency = (old_sp >= STK_EXTRA_STACK && 
                                   old_sp <= STK_EXTRA_STACK.wrapping_add(STK_EXTRA_STACK_SIZE)) as c_int;
                    stk_handler(emergency, ucp as *mut ucontext_t);
                }
            }
        }
    }

    if let Some(handler) = USER_HANDLER {
        if handler(address, 1) != 0 {
            return;
        }
    }

    signal(sig, None);
}

unsafe fn install_for(sig: c_int) {
    let mut action = Sigaction {
        sa_sigaction: Some(sigsegv_handler),
        sa_mask: Sigset { __val: [0; 16] },
        sa_flags: 4,
        sa_restorer: None,
    };

    sigemptyset(&mut action.sa_mask);
    for &signo in &[1, 2, 3, 10, 12, 13, 14, 15, 17, 23, 24, 25, 26, 27, 28, 29, 30] {
        sigaddset(&mut action.sa_mask, signo);
    }

    if STK_USER_HANDLER.is_some() {
        action.sa_flags |= 0x8000000;
    }

    sigaction(sig, &action, ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn sigsegv_install_handler(handler: SigsegvHandler) -> c_int {
    USER_HANDLER = handler;
    install_for(11);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigsegv_deinstall_handler() {
    USER_HANDLER = None;
    if STK_USER_HANDLER.is_none() {
        signal(11, None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn stackoverflow_install_handler(
    handler: StackOverflowHandler,
    extra_stack: *mut c_void,
    extra_stack_size: size_t,
) -> c_int {
    if STACK_TOP == 0 {
        let mut dummy = 0;
        remember_stack_top(&mut dummy as *mut c_int as *mut c_void);
        if STACK_TOP == 0 {
            return -1;
        }
    }

    STK_USER_HANDLER = handler;
    STK_EXTRA_STACK = extra_stack as uintptr_t;
    STK_EXTRA_STACK_SIZE = extra_stack_size;

    let mut ss = StackT {
        ss_sp: extra_stack,
        ss_flags: 0,
        ss_size: extra_stack_size,
    };

    if sigaltstack(&ss, ptr::null_mut()) < 0 {
        return -1;
    }

    install_for(11);
    0
}

#[no_mangle]
pub unsafe extern "C" fn stackoverflow_deinstall_handler() {
    STK_USER_HANDLER = None;

    if USER_HANDLER.is_some() {
        install_for(11);
    } else {
        signal(11, None);
    }

    let mut ss = StackT {
        ss_sp: ptr::null_mut(),
        ss_flags: SS_DISABLE,
        ss_size: 0,
    };

    if sigaltstack(&ss, ptr::null_mut()) < 0 {
        perror(b"gnulib sigsegv (stackoverflow_deinstall_handler)\0" as *const u8 as *const i8);
    }
}

unsafe extern "C" fn remember_stack_top(some_variable_on_stack: *mut c_void) {
    let mut vma = VmaStruct {
        start: 0,
        end: 0,
        is_near_this: None,
        prev_end: 0,
    };

    if sigsegv_get_vma(some_variable_on_stack as uintptr_t, &mut vma) >= 0 {
        STACK_TOP = vma.end.wrapping_sub(1);
    }
}