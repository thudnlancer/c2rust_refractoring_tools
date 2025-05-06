#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type pth_st;
    pub type pth_attr_st;
    pub type pth_event_st;
    fn printf(_: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaddset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn pth_spawn(
        _: pth_attr_t,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> pth_t;
    fn pth_cancel(_: pth_t) -> i32;
    fn pth_join(_: pth_t, _: *mut *mut libc::c_void) -> i32;
    fn pth_timeout(_: i64, _: i64) -> pth_time_t;
    fn pth_event(_: u64, _: ...) -> pth_event_t;
    fn pth_event_free(_: pth_event_t, _: i32) -> i32;
    fn pth_mutex_init(_: *mut pth_mutex_t) -> i32;
    fn pth_mutex_acquire(_: *mut pth_mutex_t, _: i32, _: pth_event_t) -> i32;
    fn pth_mutex_release(_: *mut pth_mutex_t) -> i32;
    fn pth_cond_init(_: *mut pth_cond_t) -> i32;
    fn pth_cond_await(_: *mut pth_cond_t, _: *mut pth_mutex_t, _: pth_event_t) -> i32;
    fn pth_cond_notify(_: *mut pth_cond_t, _: i32) -> i32;
    fn pth_init() -> i32;
    fn pth_kill() -> i32;
    fn pth_sigwait_ev(_: *const sigset_t, _: *mut i32, _: pth_event_t) -> i32;
    fn pth_sleep(_: u32) -> u32;
}
pub type __time_t = i64;
pub type __suseconds_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pth_time_t = timeval;
pub type pth_t = *mut pth_st;
pub type pth_attr_t = *mut pth_attr_st;
pub type pth_event_t = *mut pth_event_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    PTH_FREE_THIS,
    PTH_FREE_ALL,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::PTH_FREE_THIS => 0,
            C2RustUnnamed::PTH_FREE_ALL => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            0 => C2RustUnnamed::PTH_FREE_THIS,
            1 => C2RustUnnamed::PTH_FREE_ALL,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ringnode_st {
    pub rn_next: *mut pth_ringnode_t,
    pub rn_prev: *mut pth_ringnode_t,
}
pub type pth_ringnode_t = pth_ringnode_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_mutex_st {
    pub mx_node: pth_ringnode_t,
    pub mx_state: i32,
    pub mx_owner: pth_t,
    pub mx_count: u64,
}
pub type pth_mutex_t = pth_mutex_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_cond_st {
    pub cn_state: u64,
    pub cn_waiters: u32,
}
pub type pth_cond_t = pth_cond_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum philstat {
    thinking,
    hungry,
    eating,
}
impl philstat {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            philstat::thinking => 0,
            philstat::hungry => 1,
            philstat::eating => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> philstat {
        match value {
            0 => philstat::thinking,
            1 => philstat::hungry,
            2 => philstat::eating,
            _ => panic!("Invalid value for philstat: {}", value),
        }
    }
}
impl AddAssign<u32> for philstat {
    fn add_assign(&mut self, rhs: u32) {
        *self = philstat::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for philstat {
    fn sub_assign(&mut self, rhs: u32) {
        *self = philstat::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for philstat {
    fn mul_assign(&mut self, rhs: u32) {
        *self = philstat::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for philstat {
    fn div_assign(&mut self, rhs: u32) {
        *self = philstat::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for philstat {
    fn rem_assign(&mut self, rhs: u32) {
        *self = philstat::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for philstat {
    type Output = philstat;
    fn add(self, rhs: u32) -> philstat {
        philstat::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for philstat {
    type Output = philstat;
    fn sub(self, rhs: u32) -> philstat {
        philstat::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for philstat {
    type Output = philstat;
    fn mul(self, rhs: u32) -> philstat {
        philstat::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for philstat {
    type Output = philstat;
    fn div(self, rhs: u32) -> philstat {
        philstat::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for philstat {
    type Output = philstat;
    fn rem(self, rhs: u32) -> philstat {
        philstat::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tablestruct {
    pub tid: [pth_t; 5],
    pub self_0: [i32; 5],
    pub mutex: pth_mutex_t,
    pub condition: [pth_cond_t; 5],
    pub status: [philstat; 5],
}
pub type table = tablestruct;
static mut philstatstr: [*const i8; 3] = [
    b"philstat::thinking\0" as *const u8 as *const i8,
    b"philstat::hungry  \0" as *const u8 as *const i8,
    b"EATING  \0" as *const u8 as *const i8,
];
static mut tab: *mut table = 0 as *const table as *mut table;
unsafe extern "C" fn printstate() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 5 as i32 {
        printf(
            b"| %s \0" as *const u8 as *const i8,
            philstatstr[(*tab).status[i as usize] as i32 as usize],
        );
        i += 1;
        i;
    }
    printf(b"|\n\0" as *const u8 as *const i8);
}
unsafe extern "C" fn test(mut i: u32) -> i32 {
    if (*tab).status[i as usize] as u32 == philstat::hungry as i32 as u32
        && (*tab)
            .status[i.wrapping_add(1 as i32 as u32).wrapping_rem(5 as i32 as u32)
            as usize] as u32 != philstat::eating as i32 as u32
        && (*tab)
            .status[i
            .wrapping_sub(1 as i32 as u32)
            .wrapping_add(5 as i32 as u32)
            .wrapping_rem(5 as i32 as u32) as usize] as u32
            != philstat::eating as i32 as u32
    {
        (*tab).status[i as usize] = philstat::eating;
        pth_cond_notify(
            &mut *((*tab).condition).as_mut_ptr().offset(i as isize),
            0 as i32,
        );
        return (0 as i32 == 0) as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn pickup(mut k: u32) {
    pth_mutex_acquire(&mut (*tab).mutex, 0 as i32, 0 as pth_event_t);
    (*tab).status[k as usize] = philstat::hungry;
    printstate();
    if test(k) == 0 {
        pth_cond_await(
            &mut *((*tab).condition).as_mut_ptr().offset(k as isize),
            &mut (*tab).mutex,
            0 as pth_event_t,
        );
    }
    printstate();
    pth_mutex_release(&mut (*tab).mutex);
}
unsafe extern "C" fn putdown(mut k: u32) {
    pth_mutex_acquire(&mut (*tab).mutex, 0 as i32, 0 as pth_event_t);
    (*tab).status[k as usize] = philstat::thinking;
    printstate();
    test(k.wrapping_add(1 as i32 as u32).wrapping_rem(5 as i32 as u32));
    test(
        k
            .wrapping_sub(1 as i32 as u32)
            .wrapping_add(5 as i32 as u32)
            .wrapping_rem(5 as i32 as u32),
    );
    pth_mutex_release(&mut (*tab).mutex);
}
unsafe extern "C" fn philosopher(mut _who: *mut libc::c_void) -> *mut libc::c_void {
    let mut who: *mut u32 = _who as *mut u32;
    loop {
        pth_sleep((*who).wrapping_add(1 as i32 as u32));
        pickup(*who);
        pth_sleep(1 as i32 as u32);
        putdown(*who);
    };
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut i: i32 = 0;
    let mut ss: sigset_t = sigset_t { __val: [0; 16] };
    let mut sig: i32 = 0;
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    pth_init();
    printf(
        b"This is TEST_PHILO, a Pth test showing the Five Dining Philosophers\n\0"
            as *const u8 as *const i8,
    );
    printf(b"\n\0" as *const u8 as *const i8);
    printf(
        b"This is a demonstration showing the famous concurrency problem of the\n\0"
            as *const u8 as *const i8,
    );
    printf(
        b"Five Dining Philosophers as analysed 1965 by E.W.Dijkstra:\n\0" as *const u8
            as *const i8,
    );
    printf(b"\n\0" as *const u8 as *const i8);
    printf(
        b"Five philosophers are sitting around a round table, each with a bowl of\n\0"
            as *const u8 as *const i8,
    );
    printf(
        b"Chinese food in front of him. Between periods of talking they may start\n\0"
            as *const u8 as *const i8,
    );
    printf(
        b"philstat::eating whenever they want to, with their bowls being filled frequently.\n\0"
            as *const u8 as *const i8,
    );
    printf(
        b"But there are only five chopsticks available, one each to the left of\n\0"
            as *const u8 as *const i8,
    );
    printf(
        b"each bowl - and for philstat::eating Chinese food one needs two chopsticks. When\n\0"
            as *const u8 as *const i8,
    );
    printf(
        b"a philosopher wants to start philstat::eating, he must pick up the chopstick to\n\0"
            as *const u8 as *const i8,
    );
    printf(
        b"the left of his bowl and the chopstick to the right of his bowl. He\n\0"
            as *const u8 as *const i8,
    );
    printf(
        b"may find, however, that either one (or even both) of the chopsticks is\n\0"
            as *const u8 as *const i8,
    );
    printf(
        b"unavailable as it is being used by another philosopher sitting on his\n\0"
            as *const u8 as *const i8,
    );
    printf(b"right or left, so he has to wait.\n\0" as *const u8 as *const i8);
    printf(b"\n\0" as *const u8 as *const i8);
    printf(
        b"This situation shows classical contention under concurrency (the\n\0"
            as *const u8 as *const i8,
    );
    printf(
        b"philosophers want to grab the chopsticks) and the possibility of a\n\0"
            as *const u8 as *const i8,
    );
    printf(
        b"deadlock (all philosophers wait that the chopstick to their left becomes\n\0"
            as *const u8 as *const i8,
    );
    printf(b"available).\n\0" as *const u8 as *const i8);
    printf(b"\n\0" as *const u8 as *const i8);
    printf(
        b"The demonstration runs max. 60 seconds. To stop before, press CTRL-C.\n\0"
            as *const u8 as *const i8,
    );
    printf(b"\n\0" as *const u8 as *const i8);
    printf(
        b"+----P1----+----P2----+----P3----+----P4----+----P5----+\n\0" as *const u8
            as *const i8,
    );
    tab = malloc(::core::mem::size_of::<table>() as u64) as *mut table;
    if pth_mutex_init(&mut (*tab).mutex) == 0 {
        perror(b"pth_mutex_init\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    i = 0 as i32;
    while i < 5 as i32 {
        (*tab).self_0[i as usize] = i;
        (*tab).status[i as usize] = philstat::thinking;
        if pth_cond_init(&mut *((*tab).condition).as_mut_ptr().offset(i as isize)) == 0 {
            perror(b"pth_cond_init\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < 5 as i32 {
        (*tab).tid[i as usize] = pth_spawn(
            0 as pth_attr_t,
            Some(
                philosopher
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            &mut *((*tab).self_0).as_mut_ptr().offset(i as isize) as *mut i32
                as *mut libc::c_void,
        );
        if ((*tab).tid[i as usize]).is_null() {
            perror(b"pth_spawn\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        i += 1;
        i;
    }
    sigemptyset(&mut ss);
    sigaddset(&mut ss, 2 as i32);
    ev = pth_event(
        ((1 as i32) << 4 as i32) as u64,
        pth_timeout(60 as i32 as i64, 0 as i32 as i64),
    );
    pth_sigwait_ev(&mut ss, &mut sig, ev);
    pth_event_free(ev, C2RustUnnamed::PTH_FREE_ALL as i32);
    i = 0 as i32;
    while i < 5 as i32 {
        pth_cancel((*tab).tid[i as usize]);
        i += 1;
        i;
    }
    while pth_join(0 as pth_t, 0 as *mut *mut libc::c_void) != 0 {}
    printf(
        b"+----------+----------+----------+----------+----------+\n\0" as *const u8
            as *const i8,
    );
    free(tab as *mut libc::c_void);
    pth_kill();
    return 0 as i32;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}