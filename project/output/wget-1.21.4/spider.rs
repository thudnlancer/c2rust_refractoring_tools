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
    pub type hash_table;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn dcngettext(
        __domainname: *const i8,
        __msgid1: *const i8,
        __msgid2: *const i8,
        __n: u64,
        __category: i32,
    ) -> *mut i8;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn logputs(_: log_options, _: *const i8);
    fn string_set_add(_: *mut hash_table, _: *const i8);
    fn hash_table_iterate(_: *mut hash_table, _: *mut hash_table_iterator);
    fn hash_table_iter_next(_: *mut hash_table_iterator) -> i32;
    fn hash_table_count(_: *const hash_table) -> i32;
    fn make_string_hash_table(_: i32) -> *mut hash_table;
    fn is_robots_txt_url(_: *const i8) -> bool;
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum log_options {
    LOG_VERBOSE,
    LOG_NOTQUIET,
    LOG_NONVERBOSE,
    LOG_ALWAYS,
    LOG_PROGRESS,
}
impl log_options {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            log_options::LOG_VERBOSE => 0,
            log_options::LOG_NOTQUIET => 1,
            log_options::LOG_NONVERBOSE => 2,
            log_options::LOG_ALWAYS => 3,
            log_options::LOG_PROGRESS => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> log_options {
        match value {
            0 => log_options::LOG_VERBOSE,
            1 => log_options::LOG_NOTQUIET,
            2 => log_options::LOG_NONVERBOSE,
            3 => log_options::LOG_ALWAYS,
            4 => log_options::LOG_PROGRESS,
            _ => panic!("Invalid value for log_options: {}", value),
        }
    }
}
impl AddAssign<u32> for log_options {
    fn add_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for log_options {
    fn sub_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for log_options {
    fn mul_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for log_options {
    fn div_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for log_options {
    fn rem_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for log_options {
    type Output = log_options;
    fn add(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for log_options {
    type Output = log_options;
    fn sub(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for log_options {
    type Output = log_options;
    fn mul(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for log_options {
    type Output = log_options;
    fn div(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for log_options {
    type Output = log_options;
    fn rem(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table_iterator {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub pos: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
static mut nonexisting_urls_set: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
#[no_mangle]
pub unsafe extern "C" fn nonexisting_url(mut url: *const i8) {
    if is_robots_txt_url(url) {
        return;
    }
    if nonexisting_urls_set.is_null() {
        nonexisting_urls_set = make_string_hash_table(0 as i32);
    }
    string_set_add(nonexisting_urls_set, url);
}
#[no_mangle]
pub unsafe extern "C" fn print_broken_links() {
    let mut iter: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    let mut num_elems: i32 = 0;
    if nonexisting_urls_set.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Found no broken links.\n\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return;
    }
    num_elems = hash_table_count(nonexisting_urls_set);
    logprintf(
        log_options::LOG_NOTQUIET,
        dcngettext(
            0 as *const i8,
            b"Found %d broken link.\n\n\0" as *const u8 as *const i8,
            b"Found %d broken links.\n\n\0" as *const u8 as *const i8,
            num_elems as u64,
            5 as i32,
        ),
        num_elems,
    );
    hash_table_iterate(nonexisting_urls_set, &mut iter);
    while hash_table_iter_next(&mut iter) != 0 {
        let mut url: *const i8 = iter.key as *const i8;
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(0 as *const i8, b"%s\n\0" as *const u8 as *const i8, 5 as i32),
            url,
        );
    }
    logputs(log_options::LOG_NOTQUIET, b"\n\0" as *const u8 as *const i8);
}