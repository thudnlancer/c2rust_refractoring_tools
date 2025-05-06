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
    pub type variable_set_list;
    pub type commands;
    static mut not_parallel: i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn free(__ptr: *mut libc::c_void);
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    fn make_toui(_: *const i8, _: *mut *const i8) -> u32;
    fn make_seed(_: u32);
    fn make_rand() -> u32;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __uintmax_t = u64;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const i8,
    pub hname: *const i8,
    pub vpath: *const i8,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const i8,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: u32,
    pub command_flags: i32,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "low_resolution_time", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "tried_implicit", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "libc::c_uint", bits = "24..=24")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum cmd_state {
    cs_finished = 3,
    cs_running = 2,
    cs_deps_running = 1,
    cs_not_started = 0,
}
impl cmd_state {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            cmd_state::cs_finished => 3,
            cmd_state::cs_running => 2,
            cmd_state::cs_deps_running => 1,
            cmd_state::cs_not_started => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> cmd_state {
        match value {
            3 => cmd_state::cs_finished,
            2 => cmd_state::cs_running,
            1 => cmd_state::cs_deps_running,
            0 => cmd_state::cs_not_started,
            _ => panic!("Invalid value for cmd_state: {}", value),
        }
    }
}
impl AddAssign<u32> for cmd_state {
    fn add_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for cmd_state {
    fn sub_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for cmd_state {
    fn mul_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for cmd_state {
    fn div_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for cmd_state {
    fn rem_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for cmd_state {
    type Output = cmd_state;
    fn add(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for cmd_state {
    type Output = cmd_state;
    fn sub(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for cmd_state {
    type Output = cmd_state;
    fn mul(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for cmd_state {
    type Output = cmd_state;
    fn div(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for cmd_state {
    type Output = cmd_state;
    fn rem(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum update_status {
    us_failed = 3,
    us_question = 2,
    us_none = 1,
    us_success = 0,
}
impl update_status {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            update_status::us_failed => 3,
            update_status::us_question => 2,
            update_status::us_none => 1,
            update_status::us_success => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> update_status {
        match value {
            3 => update_status::us_failed,
            2 => update_status::us_question,
            1 => update_status::us_none,
            0 => update_status::us_success,
            _ => panic!("Invalid value for update_status: {}", value),
        }
    }
}
impl AddAssign<u32> for update_status {
    fn add_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for update_status {
    fn sub_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for update_status {
    fn mul_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for update_status {
    fn div_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for update_status {
    fn rem_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for update_status {
    type Output = update_status;
    fn add(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for update_status {
    type Output = update_status;
    fn sub(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for update_status {
    type Output = update_status;
    fn mul(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for update_status {
    type Output = update_status;
    fn div(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for update_status {
    type Output = update_status;
    fn rem(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct dep {
    pub next: *mut dep,
    pub name: *const i8,
    pub file: *mut file,
    pub shuf: *mut dep,
    pub stem: *const i8,
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "need_2nd_expansion", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "ignore_automatic_vars", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "libc::c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const i8,
    pub lineno: u64,
    pub offset: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub mode: shuffle_mode,
    pub seed: u32,
    pub shuffler: Option<unsafe extern "C" fn(*mut *mut libc::c_void, size_t) -> ()>,
    pub strval: [i8; 23],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum shuffle_mode {
    sm_none,
    sm_random,
    sm_reverse,
    sm_identity,
}
impl shuffle_mode {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            shuffle_mode::sm_none => 0,
            shuffle_mode::sm_random => 1,
            shuffle_mode::sm_reverse => 2,
            shuffle_mode::sm_identity => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> shuffle_mode {
        match value {
            0 => shuffle_mode::sm_none,
            1 => shuffle_mode::sm_random,
            2 => shuffle_mode::sm_reverse,
            3 => shuffle_mode::sm_identity,
            _ => panic!("Invalid value for shuffle_mode: {}", value),
        }
    }
}
impl AddAssign<u32> for shuffle_mode {
    fn add_assign(&mut self, rhs: u32) {
        *self = shuffle_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for shuffle_mode {
    fn sub_assign(&mut self, rhs: u32) {
        *self = shuffle_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for shuffle_mode {
    fn mul_assign(&mut self, rhs: u32) {
        *self = shuffle_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for shuffle_mode {
    fn div_assign(&mut self, rhs: u32) {
        *self = shuffle_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for shuffle_mode {
    fn rem_assign(&mut self, rhs: u32) {
        *self = shuffle_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for shuffle_mode {
    type Output = shuffle_mode;
    fn add(self, rhs: u32) -> shuffle_mode {
        shuffle_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for shuffle_mode {
    type Output = shuffle_mode;
    fn sub(self, rhs: u32) -> shuffle_mode {
        shuffle_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for shuffle_mode {
    type Output = shuffle_mode;
    fn mul(self, rhs: u32) -> shuffle_mode {
        shuffle_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for shuffle_mode {
    type Output = shuffle_mode;
    fn div(self, rhs: u32) -> shuffle_mode {
        shuffle_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for shuffle_mode {
    type Output = shuffle_mode;
    fn rem(self, rhs: u32) -> shuffle_mode {
        shuffle_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
static mut config: C2RustUnnamed = unsafe {
    {
        let mut init = C2RustUnnamed {
            mode: shuffle_mode::sm_none,
            seed: 0 as i32 as u32,
            shuffler: None,
            strval: *::core::mem::transmute::<
                &[u8; 23],
                &mut [i8; 23],
            >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn shuffle_get_mode() -> *const i8 {
    return if config.strval[0 as i32 as usize] as i32 == '\0' as i32 {
        0 as *mut i8
    } else {
        (config.strval).as_mut_ptr()
    };
}
#[no_mangle]
pub unsafe extern "C" fn shuffle_set_mode(mut cmdarg: *const i8) {
    if strcasecmp(cmdarg, b"reverse\0" as *const u8 as *const i8) == 0 as i32 {
        config.mode = shuffle_mode::sm_reverse;
        config.shuffler = Some(
            reverse_shuffle_array
                as unsafe extern "C" fn(*mut *mut libc::c_void, size_t) -> (),
        );
        strcpy((config.strval).as_mut_ptr(), b"reverse\0" as *const u8 as *const i8);
    } else if strcasecmp(cmdarg, b"identity\0" as *const u8 as *const i8) == 0 as i32 {
        config.mode = shuffle_mode::sm_identity;
        config.shuffler = Some(
            identity_shuffle_array
                as unsafe extern "C" fn(*mut *mut libc::c_void, size_t) -> (),
        );
        strcpy((config.strval).as_mut_ptr(), b"identity\0" as *const u8 as *const i8);
    } else if strcasecmp(cmdarg, b"none\0" as *const u8 as *const i8) == 0 as i32 {
        config.mode = shuffle_mode::sm_none;
        config.shuffler = None;
        config.strval[0 as i32 as usize] = '\0' as i32 as i8;
    } else {
        if strcasecmp(cmdarg, b"random\0" as *const u8 as *const i8) == 0 as i32 {
            config.seed = make_rand();
        } else {
            let mut err: *const i8 = 0 as *const i8;
            config.seed = make_toui(cmdarg, &mut err);
            if !err.is_null() {
                fatal(
                    0 as *mut floc,
                    (strlen(err)).wrapping_add(strlen(cmdarg)),
                    dcgettext(
                        0 as *const i8,
                        b"invalid shuffle mode: %s: '%s'\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    err,
                    cmdarg,
                );
            }
        }
        config.mode = shuffle_mode::sm_random;
        config.shuffler = Some(
            random_shuffle_array
                as unsafe extern "C" fn(*mut *mut libc::c_void, size_t) -> (),
        );
        sprintf(
            (config.strval).as_mut_ptr(),
            b"%u\0" as *const u8 as *const i8,
            config.seed,
        );
    };
}
unsafe extern "C" fn random_shuffle_array(
    mut a: *mut *mut libc::c_void,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < len {
        let mut t: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut j: u32 = (make_rand() as u64).wrapping_rem(len) as u32;
        if !(i == j as u64) {
            t = *a.offset(i as isize);
            let ref mut fresh0 = *a.offset(i as isize);
            *fresh0 = *a.offset(j as isize);
            let ref mut fresh1 = *a.offset(j as isize);
            *fresh1 = t;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn reverse_shuffle_array(
    mut a: *mut *mut libc::c_void,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < len.wrapping_div(2 as i32 as u64) {
        let mut t: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut j: size_t = len.wrapping_sub(1 as i32 as u64).wrapping_sub(i);
        t = *a.offset(i as isize);
        let ref mut fresh2 = *a.offset(i as isize);
        *fresh2 = *a.offset(j as isize);
        let ref mut fresh3 = *a.offset(j as isize);
        *fresh3 = t;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn identity_shuffle_array(
    mut a: *mut *mut libc::c_void,
    mut len: size_t,
) {}
unsafe extern "C" fn shuffle_deps(mut deps: *mut dep) {
    let mut ndeps: size_t = 0 as i32 as size_t;
    let mut dep: *mut dep = 0 as *mut dep;
    let mut da: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut dp: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    dep = deps;
    while !dep.is_null() {
        if (*dep).wait_here() != 0 {
            return;
        }
        ndeps = ndeps.wrapping_add(1);
        ndeps;
        dep = (*dep).next;
    }
    if ndeps == 0 as i32 as u64 {
        return;
    }
    da = xmalloc((::core::mem::size_of::<*mut dep>() as u64).wrapping_mul(ndeps))
        as *mut *mut libc::c_void;
    dep = deps;
    dp = da;
    while !dep.is_null() {
        *dp = dep as *mut libc::c_void;
        dep = (*dep).next;
        dp = dp.offset(1);
        dp;
    }
    (config.shuffler).expect("non-null function pointer")(da, ndeps);
    dep = deps;
    dp = da;
    while !dep.is_null() {
        (*dep).shuf = *dp as *mut dep;
        dep = (*dep).next;
        dp = dp.offset(1);
        dp;
    }
    free(da as *mut libc::c_void);
}
unsafe extern "C" fn shuffle_file_deps_recursive(mut f: *mut file) {
    let mut dep: *mut dep = 0 as *mut dep;
    if f.is_null() {
        return;
    }
    if (*f).was_shuffled() != 0 {
        return;
    }
    (*f).set_was_shuffled(1 as i32 as u32);
    shuffle_deps((*f).deps);
    dep = (*f).deps;
    while !dep.is_null() {
        shuffle_file_deps_recursive((*dep).file);
        dep = (*dep).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn shuffle_deps_recursive(mut deps: *mut dep) {
    let mut dep: *mut dep = 0 as *mut dep;
    if config.mode as u32 == shuffle_mode::sm_none as i32 as u32 {
        return;
    }
    if not_parallel != 0 {
        return;
    }
    if config.mode as u32 == shuffle_mode::sm_random as i32 as u32 {
        make_seed(config.seed);
    }
    shuffle_deps(deps);
    dep = deps;
    while !dep.is_null() {
        shuffle_file_deps_recursive((*dep).file);
        dep = (*dep).next;
    }
}