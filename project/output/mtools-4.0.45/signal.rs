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
    fn exit(_: i32) -> !;
    fn perror(__s: *const i8);
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn sigaction(__sig: i32, __act: *const sigaction, __oact: *mut sigaction) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
pub type __uint32_t = u32;
pub type __uid_t = u32;
pub type __pid_t = i32;
pub type __clock_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [i32; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: i32,
    pub _arch: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: i64,
    pub si_fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: i32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_sig_state {
    pub sa: [sigaction; 4],
}
#[no_mangle]
pub static mut got_signal: i32 = 0 as i32;
unsafe extern "C" fn signal_handler(mut dummy: i32) {
    got_signal = 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn setup_signal() {
    signal(1 as i32, Some(signal_handler as unsafe extern "C" fn(i32) -> ()));
    signal(2 as i32, Some(signal_handler as unsafe extern "C" fn(i32) -> ()));
    signal(15 as i32, Some(signal_handler as unsafe extern "C" fn(i32) -> ()));
    signal(3 as i32, Some(signal_handler as unsafe extern "C" fn(i32) -> ()));
}
unsafe extern "C" fn mt_allow_interrupt(
    mut ss: *mut saved_sig_state,
    mut sig: i32,
    mut slot: i32,
) {
    let mut new: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    memset(
        &mut new as *mut sigaction as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<sigaction>() as u64,
    );
    new.__sigaction_handler.sa_handler = Some(
        signal_handler as unsafe extern "C" fn(i32) -> (),
    );
    new.sa_flags &= !(0x10000000 as i32);
    if sigaction(sig, &mut new, &mut *((*ss).sa).as_mut_ptr().offset(slot as isize))
        < 0 as i32
    {
        perror(b"sigaction\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn allow_interrupts(mut ss: *mut saved_sig_state) {
    mt_allow_interrupt(ss, 2 as i32, 0 as i32);
    mt_allow_interrupt(ss, 2 as i32, 1 as i32);
    mt_allow_interrupt(ss, 2 as i32, 2 as i32);
    mt_allow_interrupt(ss, 2 as i32, 3 as i32);
}
unsafe extern "C" fn mt_restore_interrupt(
    mut ss: *mut saved_sig_state,
    mut sig: i32,
    mut slot: i32,
) {
    if sigaction(
        sig,
        &mut *((*ss).sa).as_mut_ptr().offset(slot as isize),
        0 as *mut sigaction,
    ) < 0 as i32
    {
        perror(b"restore sigaction\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn restore_interrupts(mut ss: *mut saved_sig_state) {
    mt_restore_interrupt(ss, 2 as i32, 0 as i32);
    mt_restore_interrupt(ss, 2 as i32, 1 as i32);
    mt_restore_interrupt(ss, 2 as i32, 2 as i32);
    mt_restore_interrupt(ss, 2 as i32, 3 as i32);
}