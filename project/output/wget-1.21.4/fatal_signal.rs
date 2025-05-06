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
    fn raise(__sig: i32) -> i32;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaddset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigaction(__sig: i32, __act: *const sigaction, __oact: *mut sigaction) -> i32;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> i32;
    fn glthread_once_singlethreaded(once_control: *mut pthread_once_t) -> i32;
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C" fn() -> ()>,
    ) -> i32;
    fn pthread_mutexattr_gettype(
        __attr: *const pthread_mutexattr_t,
        __kind: *mut i32,
    ) -> i32;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> i32;
}
pub type __uint32_t = u32;
pub type __uid_t = u32;
pub type __pid_t = i32;
pub type __clock_t = i64;
pub type __sig_atomic_t = i32;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [i8; 4],
    pub __align: i32,
}
pub type pthread_once_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [i8; 40],
    pub __align: i64,
}
pub type sig_atomic_t = __sig_atomic_t;
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
pub type action_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct actions_entry_t {
    pub action: action_t,
}
pub type sa_handler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[inline]
unsafe extern "C" fn get_handler(mut a: *const sigaction) -> sa_handler_t {
    return (*a).__sigaction_handler.sa_handler;
}
static mut fatal_signals: [i32; 7] = [
    2 as i32,
    15 as i32,
    1 as i32,
    13 as i32,
    24 as i32,
    25 as i32,
    0 as i32,
];
unsafe extern "C" fn init_fatal_signals() {
    static mut fatal_signals_initialized: bool = 0 as i32 != 0;
    if !fatal_signals_initialized {
        let mut i: size_t = 0;
        i = 0 as i32 as size_t;
        while i
            < (::core::mem::size_of::<[i32; 7]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                .wrapping_sub(1 as i32 as u64)
        {
            let mut action: sigaction = sigaction {
                __sigaction_handler: C2RustUnnamed_9 {
                    sa_handler: None,
                },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0,
                sa_restorer: None,
            };
            if sigaction(fatal_signals[i as usize], 0 as *const sigaction, &mut action)
                >= 0 as i32
                && get_handler(&mut action)
                    == ::core::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as i32 as libc::intptr_t)
            {
                fatal_signals[i as usize] = -(1 as i32);
            }
            i = i.wrapping_add(1);
            i;
        }
        fatal_signals_initialized = 1 as i32 != 0;
    }
}
static mut static_actions: [actions_entry_t; 32] = [actions_entry_t {
    action: None,
}; 32];
static mut actions: *mut actions_entry_t = unsafe { static_actions.as_ptr() as *mut _ };
static mut actions_count: sig_atomic_t = 0 as i32;
static mut actions_allocated: size_t = 0;
static mut saved_sigactions: [sigaction; 64] = [sigaction {
    __sigaction_handler: C2RustUnnamed_9 {
        sa_handler: None,
    },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
}; 64];
unsafe extern "C" fn uninstall_handlers() {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[i32; 7]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64)
            .wrapping_sub(1 as i32 as u64)
    {
        if fatal_signals[i as usize] >= 0 as i32 {
            let mut sig: i32 = fatal_signals[i as usize];
            if saved_sigactions[sig as usize].__sigaction_handler.sa_handler
                == ::core::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as i32 as libc::intptr_t)
            {
                saved_sigactions[sig as usize].__sigaction_handler.sa_handler = None;
            }
            sigaction(
                sig,
                &mut *saved_sigactions.as_mut_ptr().offset(sig as isize),
                0 as *mut sigaction,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn fatal_signal_handler(mut sig: i32) {
    loop {
        let mut action: action_t = None;
        let mut n: size_t = actions_count as size_t;
        if n == 0 as i32 as u64 {
            break;
        }
        n = n.wrapping_sub(1);
        n;
        ::core::ptr::write_volatile(
            &mut actions_count as *mut sig_atomic_t,
            n as sig_atomic_t,
        );
        action = (*actions.offset(n as isize)).action;
        action.expect("non-null function pointer")(sig);
    }
    uninstall_handlers();
    raise(sig);
}
unsafe extern "C" fn install_handlers() {
    let mut i: size_t = 0;
    let mut action: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    action.__sigaction_handler.sa_handler = Some(
        fatal_signal_handler as unsafe extern "C" fn(i32) -> (),
    );
    action.sa_flags = 0x40000000 as i32;
    sigemptyset(&mut action.sa_mask);
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[i32; 7]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64)
            .wrapping_sub(1 as i32 as u64)
    {
        if fatal_signals[i as usize] >= 0 as i32 {
            let mut sig: i32 = fatal_signals[i as usize];
            if !((sig as u64)
                < (::core::mem::size_of::<[sigaction; 64]>() as u64)
                    .wrapping_div(::core::mem::size_of::<sigaction>() as u64))
            {
                abort();
            }
            sigaction(
                sig,
                &mut action,
                &mut *saved_sigactions.as_mut_ptr().offset(sig as isize),
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
static mut at_fatal_signal_lock: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as i32,
            __count: 0 as i32 as u32,
            __owner: 0 as i32,
            __nusers: 0 as i32 as u32,
            __kind: 0 as i32,
            __spins: 0 as i32 as libc::c_short,
            __elision: 0 as i32 as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
#[no_mangle]
pub unsafe extern "C" fn at_fatal_signal(mut action: action_t) -> i32 {
    let mut current_block: u64;
    let mut mt: bool = 1 as i32 != 0;
    if mt {
        if if (Some(
            pthread_mutexattr_gettype
                as unsafe extern "C" fn(*const pthread_mutexattr_t, *mut i32) -> i32,
        ))
            .is_some() || 0 as i32 != 0
        {
            pthread_mutex_lock(&mut at_fatal_signal_lock)
        } else {
            0 as i32
        } != 0
        {
            abort();
        }
    }
    static mut cleanup_initialized: bool = 0 as i32 != 0;
    if !cleanup_initialized {
        init_fatal_signals();
        install_handlers();
        cleanup_initialized = 1 as i32 != 0;
    }
    let mut ret: i32 = 0 as i32;
    if actions_count as u64 == actions_allocated {
        let mut old_actions: *mut actions_entry_t = actions;
        let mut old_actions_allocated: size_t = actions_allocated;
        let mut new_actions_allocated: size_t = (2 as i32 as u64)
            .wrapping_mul(actions_allocated);
        let mut new_actions: *mut actions_entry_t = malloc(
            new_actions_allocated
                .wrapping_mul(::core::mem::size_of::<actions_entry_t>() as u64),
        ) as *mut actions_entry_t;
        if new_actions.is_null() {
            ret = -(1 as i32);
            current_block = 17203466822048039139;
        } else {
            let mut k: size_t = 0;
            k = 0 as i32 as size_t;
            while k < old_actions_allocated {
                *new_actions.offset(k as isize) = *old_actions.offset(k as isize);
                k = k.wrapping_add(1);
                k;
            }
            ::core::ptr::write_volatile(
                &mut actions as *mut *mut actions_entry_t,
                new_actions,
            );
            actions_allocated = new_actions_allocated;
            current_block = 5948590327928692120;
        }
    } else {
        current_block = 5948590327928692120;
    }
    match current_block {
        5948590327928692120 => {
            let ref mut fresh0 = (*actions.offset(actions_count as isize)).action;
            ::core::ptr::write_volatile(fresh0, action);
            ::core::ptr::write_volatile(
                &mut actions_count as *mut sig_atomic_t,
                ::core::ptr::read_volatile::<
                    sig_atomic_t,
                >(&actions_count as *const sig_atomic_t) + 1,
            );
            ::core::ptr::read_volatile::<
                sig_atomic_t,
            >(&actions_count as *const sig_atomic_t);
        }
        _ => {}
    }
    if mt {
        if if (Some(
            pthread_mutexattr_gettype
                as unsafe extern "C" fn(*const pthread_mutexattr_t, *mut i32) -> i32,
        ))
            .is_some() || 0 as i32 != 0
        {
            pthread_mutex_unlock(&mut at_fatal_signal_lock)
        } else {
            0 as i32
        } != 0
        {
            abort();
        }
    }
    return ret;
}
static mut fatal_signal_set: sigset_t = __sigset_t { __val: [0; 16] };
unsafe extern "C" fn do_init_fatal_signal_set() {
    let mut i: size_t = 0;
    init_fatal_signals();
    sigemptyset(&mut fatal_signal_set);
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[i32; 7]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64)
            .wrapping_sub(1 as i32 as u64)
    {
        if fatal_signals[i as usize] >= 0 as i32 {
            sigaddset(&mut fatal_signal_set, fatal_signals[i as usize]);
        }
        i = i.wrapping_add(1);
        i;
    }
}
static mut fatal_signal_set_once: pthread_once_t = 0 as i32;
unsafe extern "C" fn init_fatal_signal_set() {
    if if (Some(
        pthread_mutexattr_gettype
            as unsafe extern "C" fn(*const pthread_mutexattr_t, *mut i32) -> i32,
    ))
        .is_some() || 0 as i32 != 0
    {
        pthread_once(
            &mut fatal_signal_set_once,
            Some(do_init_fatal_signal_set as unsafe extern "C" fn() -> ()),
        )
    } else if glthread_once_singlethreaded(&mut fatal_signal_set_once) != 0 {
        do_init_fatal_signal_set();
        0 as i32
    } else {
        0 as i32
    } != 0
    {
        abort();
    }
}
static mut fatal_signals_block_lock: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as i32,
            __count: 0 as i32 as u32,
            __owner: 0 as i32,
            __nusers: 0 as i32 as u32,
            __kind: 0 as i32,
            __spins: 0 as i32 as libc::c_short,
            __elision: 0 as i32 as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut fatal_signals_block_counter: u32 = 0 as i32 as u32;
#[no_mangle]
pub unsafe extern "C" fn block_fatal_signals() {
    let mut mt: bool = 1 as i32 != 0;
    if mt {
        if if (Some(
            pthread_mutexattr_gettype
                as unsafe extern "C" fn(*const pthread_mutexattr_t, *mut i32) -> i32,
        ))
            .is_some() || 0 as i32 != 0
        {
            pthread_mutex_lock(&mut fatal_signals_block_lock)
        } else {
            0 as i32
        } != 0
        {
            abort();
        }
    }
    let fresh1 = fatal_signals_block_counter;
    fatal_signals_block_counter = fatal_signals_block_counter.wrapping_add(1);
    if fresh1 == 0 as i32 as u32 {
        init_fatal_signal_set();
        sigprocmask(0 as i32, &mut fatal_signal_set, 0 as *mut sigset_t);
    }
    if mt {
        if if (Some(
            pthread_mutexattr_gettype
                as unsafe extern "C" fn(*const pthread_mutexattr_t, *mut i32) -> i32,
        ))
            .is_some() || 0 as i32 != 0
        {
            pthread_mutex_unlock(&mut fatal_signals_block_lock)
        } else {
            0 as i32
        } != 0
        {
            abort();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn unblock_fatal_signals() {
    let mut mt: bool = 1 as i32 != 0;
    if mt {
        if if (Some(
            pthread_mutexattr_gettype
                as unsafe extern "C" fn(*const pthread_mutexattr_t, *mut i32) -> i32,
        ))
            .is_some() || 0 as i32 != 0
        {
            pthread_mutex_lock(&mut fatal_signals_block_lock)
        } else {
            0 as i32
        } != 0
        {
            abort();
        }
    }
    if fatal_signals_block_counter == 0 as i32 as u32 {
        abort();
    }
    fatal_signals_block_counter = fatal_signals_block_counter.wrapping_sub(1);
    if fatal_signals_block_counter == 0 as i32 as u32 {
        init_fatal_signal_set();
        sigprocmask(1 as i32, &mut fatal_signal_set, 0 as *mut sigset_t);
    }
    if mt {
        if if (Some(
            pthread_mutexattr_gettype
                as unsafe extern "C" fn(*const pthread_mutexattr_t, *mut i32) -> i32,
        ))
            .is_some() || 0 as i32 != 0
        {
            pthread_mutex_unlock(&mut fatal_signals_block_lock)
        } else {
            0 as i32
        } != 0
        {
            abort();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_fatal_signals(mut signals: *mut i32) -> u32 {
    init_fatal_signal_set();
    let mut p: *mut i32 = signals;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[i32; 7]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64)
            .wrapping_sub(1 as i32 as u64)
    {
        if fatal_signals[i as usize] >= 0 as i32 {
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = fatal_signals[i as usize];
        }
        i = i.wrapping_add(1);
        i;
    }
    return p.offset_from(signals) as i64 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn get_fatal_signal_set() -> *const sigset_t {
    init_fatal_signal_set();
    return &mut fatal_signal_set;
}
unsafe extern "C" fn run_static_initializers() {
    actions_allocated = (::core::mem::size_of::<[actions_entry_t; 32]>() as u64)
        .wrapping_div(::core::mem::size_of::<actions_entry_t>() as u64);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];