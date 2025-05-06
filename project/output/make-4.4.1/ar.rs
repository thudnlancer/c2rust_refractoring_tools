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
    fn free(__ptr: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn concat(_: u32, _: ...) -> *const i8;
    fn error(flocp: *const floc, length: size_t, fmt: *const i8, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    fn perror_with_name(_: *const i8, _: *const i8);
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const i8) -> *mut i8;
    fn alpha_compare(_: *const libc::c_void, _: *const libc::c_void) -> i32;
    fn ar_member_touch(arname: *const i8, memname: *const i8) -> i32;
    fn strcache_add(str: *const i8) -> *const i8;
    fn ar_name_equal(name: *const i8, mem: *const i8, truncated: i32) -> i32;
    fn ar_scan(
        archive: *const i8,
        function: ar_member_func_t,
        arg: *const libc::c_void,
    ) -> intmax_t;
    fn file_exists_p(_: *const i8) -> i32;
    fn enter_file(name: *const i8) -> *mut file;
    fn lookup_file(name: *const i8) -> *mut file;
    fn f_mtime(file: *mut file, search: i32) -> uintmax_t;
    fn fnmatch(__pattern: *const i8, __name: *const i8, __flags: i32) -> i32;
}
pub type size_t = u64;
pub type __intmax_t = i64;
pub type __uintmax_t = u64;
pub type __time_t = i64;
pub type time_t = __time_t;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type intmax_t = __intmax_t;
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
pub type ar_member_func_t = Option<
    unsafe extern "C" fn(
        i32,
        *const i8,
        i32,
        i64,
        i64,
        i64,
        intmax_t,
        i32,
        i32,
        u32,
        *const libc::c_void,
    ) -> intmax_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ar_glob_state {
    pub arname: *const i8,
    pub pattern: *const i8,
    pub size: size_t,
    pub chain: *mut nameseq,
    pub n: u32,
}
#[no_mangle]
pub unsafe extern "C" fn ar_name(mut name: *const i8) -> i32 {
    let mut p: *const i8 = strchr(name, '(' as i32);
    let mut end: *const i8 = 0 as *const i8;
    if p.is_null() || p == name {
        return 0 as i32;
    }
    end = p.offset(strlen(p) as isize).offset(-(1 as i32 as isize));
    if *end as i32 != ')' as i32 || end == p.offset(1 as i32 as isize) {
        return 0 as i32;
    }
    if *p.offset(1 as i32 as isize) as i32 == '(' as i32
        && *end.offset(-(1 as i32) as isize) as i32 == ')' as i32
    {
        fatal(
            0 as *mut floc,
            strlen(name),
            dcgettext(
                0 as *const i8,
                b"attempt to use unsupported feature: '%s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            name,
        );
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn ar_parse_name(
    mut name: *const i8,
    mut arname_p: *mut *mut i8,
    mut memname_p: *mut *mut i8,
) {
    let mut p: *mut i8 = 0 as *mut i8;
    *arname_p = xstrdup(name);
    p = strchr(*arname_p, '(' as i32);
    if p.is_null() {
        fatal(
            0 as *mut floc,
            strlen(*arname_p),
            b"Internal: ar_parse_name: bad name '%s'\0" as *const u8 as *const i8,
            *arname_p,
        );
    }
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '\0' as i32 as i8;
    *p.offset((strlen(p)).wrapping_sub(1 as i32 as u64) as isize) = '\0' as i32 as i8;
    *memname_p = p;
}
unsafe extern "C" fn ar_member_date_1(
    mut desc: i32,
    mut mem: *const i8,
    mut truncated: i32,
    mut hdrpos: i64,
    mut datapos: i64,
    mut size: i64,
    mut date: intmax_t,
    mut uid: i32,
    mut gid: i32,
    mut mode: u32,
    mut name: *const libc::c_void,
) -> intmax_t {
    return if ar_name_equal(name as *const i8, mem, truncated) != 0 {
        date
    } else {
        0 as i32 as i64
    };
}
#[no_mangle]
pub unsafe extern "C" fn ar_member_date(mut name: *const i8) -> time_t {
    let mut arname: *mut i8 = 0 as *mut i8;
    let mut memname: *mut i8 = 0 as *mut i8;
    let mut val: intmax_t = 0;
    ar_parse_name(name, &mut arname, &mut memname);
    let mut arfile: *mut file = 0 as *mut file;
    arfile = lookup_file(arname);
    if arfile.is_null() && file_exists_p(arname) != 0 {
        arfile = enter_file(strcache_add(arname));
    }
    if !arfile.is_null() {
        f_mtime(arfile, 0 as i32);
    }
    val = ar_scan(
        arname,
        Some(
            ar_member_date_1
                as unsafe extern "C" fn(
                    i32,
                    *const i8,
                    i32,
                    i64,
                    i64,
                    i64,
                    intmax_t,
                    i32,
                    i32,
                    u32,
                    *const libc::c_void,
                ) -> intmax_t,
        ),
        memname as *const libc::c_void,
    );
    free(arname as *mut libc::c_void);
    return if (0 as i32 as i64) < val
        && val
            <= (if (0 as i32 as time_t) < -(1 as i32) as time_t {
                -(1 as i32) as time_t
            } else {
                (((1 as i32 as time_t)
                    << (::core::mem::size_of::<time_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64
            })
    {
        val
    } else {
        -(1 as i32) as i64
    };
}
#[no_mangle]
pub unsafe extern "C" fn ar_touch(mut name: *const i8) -> i32 {
    let mut arname: *mut i8 = 0 as *mut i8;
    let mut memname: *mut i8 = 0 as *mut i8;
    let mut val: i32 = 0;
    ar_parse_name(name, &mut arname, &mut memname);
    let mut arfile: *mut file = 0 as *mut file;
    arfile = enter_file(strcache_add(arname));
    f_mtime(arfile, 0 as i32);
    val = 1 as i32;
    match ar_member_touch(arname, memname) {
        -1 => {
            error(
                0 as *mut floc,
                strlen(arname),
                dcgettext(
                    0 as *const i8,
                    b"touch: Archive '%s' does not exist\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                arname,
            );
        }
        -2 => {
            error(
                0 as *mut floc,
                strlen(arname),
                dcgettext(
                    0 as *const i8,
                    b"touch: '%s' is not a valid archive\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                arname,
            );
        }
        -3 => {
            perror_with_name(b"touch: \0" as *const u8 as *const i8, arname);
        }
        1 => {
            error(
                0 as *mut floc,
                (strlen(memname)).wrapping_add(strlen(arname)),
                dcgettext(
                    0 as *const i8,
                    b"touch: Member '%s' does not exist in '%s'\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                memname,
                arname,
            );
        }
        0 => {
            val = 0 as i32;
        }
        _ => {
            error(
                0 as *mut floc,
                strlen(name),
                dcgettext(
                    0 as *const i8,
                    b"touch: Bad return code from ar_member_touch on '%s'\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                name,
            );
        }
    }
    free(arname as *mut libc::c_void);
    return val;
}
unsafe extern "C" fn ar_glob_match(
    mut desc: i32,
    mut mem: *const i8,
    mut truncated: i32,
    mut hdrpos: i64,
    mut datapos: i64,
    mut size: i64,
    mut date: intmax_t,
    mut uid: i32,
    mut gid: i32,
    mut mode: u32,
    mut arg: *const libc::c_void,
) -> intmax_t {
    let mut state: *mut ar_glob_state = arg as *mut ar_glob_state;
    if fnmatch((*state).pattern, mem, (1 as i32) << 0 as i32 | (1 as i32) << 2 as i32)
        == 0 as i32
    {
        let mut new: *mut nameseq = xcalloc((*state).size) as *mut nameseq;
        (*new).name = strcache_add(
            concat(
                4 as i32 as u32,
                (*state).arname,
                b"(\0" as *const u8 as *const i8,
                mem,
                b")\0" as *const u8 as *const i8,
            ),
        );
        (*new).next = (*state).chain;
        (*state).chain = new;
        (*state).n = ((*state).n).wrapping_add(1);
        (*state).n;
    }
    return 0 as i32 as intmax_t;
}
unsafe extern "C" fn ar_glob_pattern_p(mut pattern: *const i8, mut quote: i32) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut opened: i32 = 0 as i32;
    p = pattern;
    while *p as i32 != '\0' as i32 {
        match *p as i32 {
            63 | 42 => return 1 as i32,
            92 => {
                if quote != 0 {
                    p = p.offset(1);
                    p;
                }
            }
            91 => {
                opened = 1 as i32;
            }
            93 => {
                if opened != 0 {
                    return 1 as i32;
                }
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn ar_glob(
    mut arname: *const i8,
    mut member_pattern: *const i8,
    mut size: size_t,
) -> *mut nameseq {
    let mut state: ar_glob_state = ar_glob_state {
        arname: 0 as *const i8,
        pattern: 0 as *const i8,
        size: 0,
        chain: 0 as *mut nameseq,
        n: 0,
    };
    let mut n: *mut nameseq = 0 as *mut nameseq;
    let mut names: *mut *const i8 = 0 as *mut *const i8;
    let mut i: u32 = 0;
    if ar_glob_pattern_p(member_pattern, 1 as i32) == 0 {
        return 0 as *mut nameseq;
    }
    state.arname = arname;
    state.pattern = member_pattern;
    state.size = size;
    state.chain = 0 as *mut nameseq;
    state.n = 0 as i32 as u32;
    ar_scan(
        arname,
        Some(
            ar_glob_match
                as unsafe extern "C" fn(
                    i32,
                    *const i8,
                    i32,
                    i64,
                    i64,
                    i64,
                    intmax_t,
                    i32,
                    i32,
                    u32,
                    *const libc::c_void,
                ) -> intmax_t,
        ),
        &mut state as *mut ar_glob_state as *const libc::c_void,
    );
    if (state.chain).is_null() {
        return 0 as *mut nameseq;
    }
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (state.n as u64).wrapping_mul(::core::mem::size_of::<*const i8>() as u64)
            as usize,
    );
    names = fresh1.as_mut_ptr() as *mut *const i8;
    i = 0 as i32 as u32;
    n = state.chain;
    while !n.is_null() {
        let fresh2 = i;
        i = i.wrapping_add(1);
        let ref mut fresh3 = *names.offset(fresh2 as isize);
        *fresh3 = (*n).name;
        n = (*n).next;
    }
    qsort(
        names as *mut libc::c_void,
        i as size_t,
        ::core::mem::size_of::<*const i8>() as u64,
        Some(
            alpha_compare
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    i = 0 as i32 as u32;
    n = state.chain;
    while !n.is_null() {
        let fresh4 = i;
        i = i.wrapping_add(1);
        (*n).name = *names.offset(fresh4 as isize);
        n = (*n).next;
    }
    return state.chain;
}