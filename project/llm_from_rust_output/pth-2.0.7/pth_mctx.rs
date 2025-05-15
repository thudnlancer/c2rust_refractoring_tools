use std::os::raw::{c_int, c_void, c_char, c_ulong, c_ushort, c_uint, c_ulonglong};
use std::mem::MaybeUninit;

type size_t = c_ulong;
type greg_t = c_ulonglong;
type gregset_t = [greg_t; 23];
type sigset_t = [c_ulong; 16];

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Stack {
    sp: *mut c_void,
    flags: c_int,
    size: size_t,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct FpReg {
    significand: [c_ushort; 4],
    exponent: c_ushort,
    reserved: [c_ushort; 3],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct XmmReg {
    element: [c_uint; 4],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct FpState {
    cwd: c_ushort,
    swd: c_ushort,
    ftw: c_ushort,
    fop: c_ushort,
    rip: c_ulong,
    rdp: c_ulong,
    mxcsr: c_uint,
    mxcr_mask: c_uint,
    st: [FpReg; 8],
    xmm: [XmmReg; 16],
    reserved: [c_uint; 24],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct MContext {
    gregs: gregset_t,
    fpregs: *mut FpState,
    reserved: [c_ulonglong; 8],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct UContext {
    flags: c_ulong,
    link: *mut UContext,
    stack: Stack,
    mcontext: MContext,
    sigmask: sigset_t,
    fpregs_mem: FpState,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct PthMctx {
    uc: UContext,
    restored: c_int,
    sigs: sigset_t,
    error: c_int,
}

fn pth_mctx_set(
    mctx: &mut PthMctx,
    func: fn(),
    sk_addr_lo: *mut c_char,
    sk_addr_hi: *mut c_char,
) -> Result<(), ()> {
    unsafe {
        if libc::getcontext(&mut mctx.uc as *mut _ as *mut libc::ucontext_t) != 0 {
            return Err(());
        }
    }

    mctx.uc.link = std::ptr::null_mut();
    mctx.uc.stack.sp = sk_addr_lo as *mut c_void;
    mctx.uc.stack.size = unsafe { sk_addr_hi.offset_from(sk_addr_lo) as isize as size_t };
    mctx.uc.stack.flags = 0;

    unsafe {
        libc::makecontext(
            &mut mctx.uc as *mut _ as *mut libc::ucontext_t,
            func as *const () as *mut std::ffi::c_void,
            0,
        );
    }

    Ok(())
}