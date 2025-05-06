#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type hash_table;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncasecmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn rpl_free(_: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_pair {
    pub name: *mut i8,
    pub value: *mut i8,
    pub value_raw_beginning: *const i8,
    pub value_raw_size: i32,
    pub name_pool_index: i32,
    pub value_pool_index: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taginfo {
    pub name: *mut i8,
    pub end_tag_p: i32,
    pub nattrs: i32,
    pub attrs: *mut attr_pair,
    pub start_position: *const i8,
    pub end_position: *const i8,
    pub contents_begin: *const i8,
    pub contents_end: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagstack_item {
    pub tagname_begin: *const i8,
    pub tagname_end: *const i8,
    pub contents_begin: *const i8,
    pub prev: *mut tagstack_item,
    pub next: *mut tagstack_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool {
    pub contents: *mut i8,
    pub size: i32,
    pub tail: i32,
    pub resized: bool,
    pub orig_contents: *mut i8,
    pub orig_size: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    AP_DOWNCASE = 1,
    AP_TRIM_BLANKS = 4,
    AP_DECODE_ENTITIES = 2,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::AP_DOWNCASE => 1,
            C2RustUnnamed_0::AP_TRIM_BLANKS => 4,
            C2RustUnnamed_0::AP_DECODE_ENTITIES => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            1 => C2RustUnnamed_0::AP_DOWNCASE,
            4 => C2RustUnnamed_0::AP_TRIM_BLANKS,
            2 => C2RustUnnamed_0::AP_DECODE_ENTITIES,
            _ => panic!("Invalid value for C2RustUnnamed_0: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_0 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_0 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_0 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_0 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_0 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn add(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn sub(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn mul(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn div(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn rem(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    AC_S_BACKOUT = 1,
    AC_S_QUOTE2 = 12,
    AC_S_IN_QUOTE = 11,
    AC_S_QUOTE1 = 10,
    AC_S_DASH4 = 9,
    AC_S_DASH3 = 8,
    AC_S_COMMENT = 7,
    AC_S_DASH2 = 6,
    AC_S_DASH1 = 5,
    AC_S_DCLNAME = 4,
    AC_S_DEFAULT = 3,
    AC_S_BANG = 2,
    AC_S_DONE = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::AC_S_BACKOUT => 1,
            C2RustUnnamed::AC_S_QUOTE2 => 12,
            C2RustUnnamed::AC_S_IN_QUOTE => 11,
            C2RustUnnamed::AC_S_QUOTE1 => 10,
            C2RustUnnamed::AC_S_DASH4 => 9,
            C2RustUnnamed::AC_S_DASH3 => 8,
            C2RustUnnamed::AC_S_COMMENT => 7,
            C2RustUnnamed::AC_S_DASH2 => 6,
            C2RustUnnamed::AC_S_DASH1 => 5,
            C2RustUnnamed::AC_S_DCLNAME => 4,
            C2RustUnnamed::AC_S_DEFAULT => 3,
            C2RustUnnamed::AC_S_BANG => 2,
            C2RustUnnamed::AC_S_DONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            1 => C2RustUnnamed::AC_S_BACKOUT,
            12 => C2RustUnnamed::AC_S_QUOTE2,
            11 => C2RustUnnamed::AC_S_IN_QUOTE,
            10 => C2RustUnnamed::AC_S_QUOTE1,
            9 => C2RustUnnamed::AC_S_DASH4,
            8 => C2RustUnnamed::AC_S_DASH3,
            7 => C2RustUnnamed::AC_S_COMMENT,
            6 => C2RustUnnamed::AC_S_DASH2,
            5 => C2RustUnnamed::AC_S_DASH1,
            4 => C2RustUnnamed::AC_S_DCLNAME,
            3 => C2RustUnnamed::AC_S_DEFAULT,
            2 => C2RustUnnamed::AC_S_BANG,
            0 => C2RustUnnamed::AC_S_DONE,
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
#[inline]
unsafe extern "C" fn c_isalnum(mut c: i32) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: i32) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: i32) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isxdigit(mut c: i32) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 65 | 66 | 67 | 68 | 69 | 70 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn c_tolower(mut c: i32) -> i32 {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
#[inline]
unsafe extern "C" fn _unhex(mut c: u8) -> u8 {
    return (if c as i32 <= '9' as i32 {
        c as i32 - '0' as i32
    } else if c as i32 <= 'F' as i32 {
        c as i32 - 'A' as i32 + 10 as i32
    } else {
        c as i32 - 'a' as i32 + 10 as i32
    }) as u8;
}
unsafe extern "C" fn tagstack_push(
    mut head: *mut *mut tagstack_item,
    mut tail: *mut *mut tagstack_item,
) -> *mut tagstack_item {
    let mut ts: *mut tagstack_item = xmalloc(
        ::core::mem::size_of::<tagstack_item>() as u64,
    ) as *mut tagstack_item;
    if (*head).is_null() {
        *tail = ts;
        *head = *tail;
        (*ts).next = 0 as *mut tagstack_item;
        (*ts).prev = (*ts).next;
    } else {
        (**tail).next = ts;
        (*ts).prev = *tail;
        *tail = ts;
        (*ts).next = 0 as *mut tagstack_item;
    }
    return ts;
}
unsafe extern "C" fn tagstack_pop(
    mut head: *mut *mut tagstack_item,
    mut tail: *mut *mut tagstack_item,
    mut ts: *mut tagstack_item,
) {
    if (*head).is_null() {
        return;
    }
    if ts == *tail {
        if ts == *head {
            rpl_free(ts as *mut libc::c_void);
            ts = 0 as *mut tagstack_item;
            *tail = 0 as *mut tagstack_item;
            *head = *tail;
        } else {
            (*(*ts).prev).next = 0 as *mut tagstack_item;
            *tail = (*ts).prev;
            rpl_free(ts as *mut libc::c_void);
            ts = 0 as *mut tagstack_item;
        }
    } else {
        if ts == *head {
            *head = 0 as *mut tagstack_item;
        }
        *tail = (*ts).prev;
        if !((*ts).prev).is_null() {
            (*(*ts).prev).next = 0 as *mut tagstack_item;
        }
        while !ts.is_null() {
            let mut p: *mut tagstack_item = (*ts).next;
            rpl_free(ts as *mut libc::c_void);
            ts = 0 as *mut tagstack_item;
            ts = p;
        }
    };
}
unsafe extern "C" fn tagstack_find(
    mut tail: *mut tagstack_item,
    mut tagname_begin: *const i8,
    mut tagname_end: *const i8,
) -> *mut tagstack_item {
    let mut len: i32 = tagname_end.offset_from(tagname_begin) as i64 as i32;
    while !tail.is_null() {
        if len as i64 == ((*tail).tagname_end).offset_from((*tail).tagname_begin) as i64
        {
            if 0 as i32 == strncasecmp((*tail).tagname_begin, tagname_begin, len as u64)
            {
                return tail;
            }
        }
        tail = (*tail).prev;
    }
    return 0 as *mut tagstack_item;
}
unsafe extern "C" fn decode_entity(mut ptr: *mut *const i8, mut end: *const i8) -> i32 {
    let mut p: *const i8 = *ptr;
    let mut value: i32 = -(1 as i32);
    p = p.offset(1);
    if p == end {
        return -(1 as i32);
    }
    let fresh0 = p;
    p = p.offset(1);
    match *fresh0 as i32 {
        35 => {
            let mut digits: i32 = 0 as i32;
            value = 0 as i32;
            if *p as i32 == 'x' as i32 {
                p = p.offset(1);
                p;
                while value < 256 as i32 && p < end && c_isxdigit(*p as i32) as i32 != 0
                {
                    value = (value << 4 as i32) + _unhex(*p as u8) as i32;
                    p = p.offset(1);
                    p;
                    digits += 1;
                    digits;
                }
            } else {
                while value < 256 as i32 && p < end && c_isdigit(*p as i32) as i32 != 0 {
                    value = value * 10 as i32 + (*p as i32 - '0' as i32);
                    p = p.offset(1);
                    p;
                    digits += 1;
                    digits;
                }
            }
            if digits == 0 {
                return -(1 as i32);
            }
            if value == 0 || value & !(0x7f as i32) != 0 {
                return -(1 as i32);
            }
            p = p.offset(0 as i32 as isize);
            *ptr = (if p < end && *p as i32 == ';' as i32 {
                p = p.offset(1);
                p
            } else {
                p
            });
            return value;
        }
        103 => {
            if (p.offset(1 as i32 as isize) == end
                || p.offset(1 as i32 as isize) < end
                    && !c_isalnum(*p.offset(1 as i32 as isize) as i32))
                && *p.offset(0 as i32 as isize) as i32 == 't' as i32
            {
                value = '>' as i32;
                p = p.offset(1 as i32 as isize);
                *ptr = (if p < end && *p as i32 == ';' as i32 {
                    p = p.offset(1);
                    p
                } else {
                    p
                });
            }
        }
        108 => {
            if (p.offset(1 as i32 as isize) == end
                || p.offset(1 as i32 as isize) < end
                    && !c_isalnum(*p.offset(1 as i32 as isize) as i32))
                && *p.offset(0 as i32 as isize) as i32 == 't' as i32
            {
                value = '<' as i32;
                p = p.offset(1 as i32 as isize);
                *ptr = (if p < end && *p as i32 == ';' as i32 {
                    p = p.offset(1);
                    p
                } else {
                    p
                });
            }
        }
        97 => {
            if (p.offset(2 as i32 as isize) == end
                || p.offset(2 as i32 as isize) < end
                    && !c_isalnum(*p.offset(2 as i32 as isize) as i32))
                && *p.offset(0 as i32 as isize) as i32 == 'm' as i32
                && *p.offset(1 as i32 as isize) as i32 == 'p' as i32
            {
                value = '&' as i32;
                p = p.offset(2 as i32 as isize);
                *ptr = (if p < end && *p as i32 == ';' as i32 {
                    p = p.offset(1);
                    p
                } else {
                    p
                });
            } else if (p.offset(3 as i32 as isize) == end
                || p.offset(3 as i32 as isize) < end
                    && !c_isalnum(*p.offset(3 as i32 as isize) as i32))
                && *p.offset(0 as i32 as isize) as i32 == 'p' as i32
                && *p.offset(1 as i32 as isize) as i32 == 'o' as i32
                && *p.offset(2 as i32 as isize) as i32 == 's' as i32
            {
                value = '\'' as i32;
                p = p.offset(3 as i32 as isize);
                *ptr = (if p < end && *p as i32 == ';' as i32 {
                    p = p.offset(1);
                    p
                } else {
                    p
                });
            }
        }
        113 => {
            if (p.offset(3 as i32 as isize) == end
                || p.offset(3 as i32 as isize) < end
                    && !c_isalnum(*p.offset(3 as i32 as isize) as i32))
                && *p.offset(0 as i32 as isize) as i32 == 'u' as i32
                && *p.offset(1 as i32 as isize) as i32 == 'o' as i32
                && *p.offset(2 as i32 as isize) as i32 == 't' as i32
            {
                value = '"' as i32;
                p = p.offset(3 as i32 as isize);
                *ptr = (if p < end && *p as i32 == ';' as i32 {
                    p = p.offset(1);
                    p
                } else {
                    p
                });
            }
        }
        _ => {}
    }
    return value;
}
unsafe extern "C" fn convert_and_copy(
    mut pool: *mut pool,
    mut beg: *const i8,
    mut end: *const i8,
    mut flags: i32,
) {
    let mut old_tail: i32 = (*pool).tail;
    if flags & C2RustUnnamed_0::AP_TRIM_BLANKS as i32 != 0 {
        while beg < end && c_isspace(*beg as i32) as i32 != 0 {
            beg = beg.offset(1);
            beg;
        }
        while end > beg
            && c_isspace(*end.offset(-(1 as i32) as isize) as i32) as i32 != 0
        {
            end = end.offset(-1);
            end;
        }
    }
    if flags & C2RustUnnamed_0::AP_DECODE_ENTITIES as i32 != 0 {
        let mut from: *const i8 = beg;
        let mut to: *mut i8 = 0 as *mut i8;
        let mut squash_newlines: bool = flags & C2RustUnnamed_0::AP_TRIM_BLANKS as i32
            != 0;
        let mut ga_needed_size: i64 = (*pool).tail as i64 + end.offset_from(beg) as i64;
        let mut ga_newsize: i64 = (*pool).size as i64;
        while ga_newsize < ga_needed_size {
            ga_newsize <<= 1 as i32;
        }
        if ga_newsize != (*pool).size as i64 {
            if (*pool).resized {
                (*pool).contents = xrealloc(
                    (*pool).contents as *mut libc::c_void,
                    (ga_newsize as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
                ) as *mut i8;
            } else {
                let mut ga_new: *mut libc::c_void = xmalloc(
                    (ga_newsize as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                memcpy(
                    ga_new,
                    (*pool).contents as *const libc::c_void,
                    ((*pool).size as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                (*pool).contents = ga_new as *mut i8;
                (*pool).resized = 1 as i32 != 0;
            }
            (*pool).size = ga_newsize as i32;
        }
        to = ((*pool).contents).offset((*pool).tail as isize);
        while from < end {
            if *from as i32 == '&' as i32 {
                let mut entity: i32 = decode_entity(&mut from, end);
                if entity != -(1 as i32) {
                    let fresh1 = to;
                    to = to.offset(1);
                    *fresh1 = entity as i8;
                } else {
                    let fresh2 = from;
                    from = from.offset(1);
                    let fresh3 = to;
                    to = to.offset(1);
                    *fresh3 = *fresh2;
                }
            } else if (*from as i32 == '\n' as i32 || *from as i32 == '\r' as i32)
                && squash_newlines as i32 != 0
            {
                from = from.offset(1);
                from;
            } else {
                let fresh4 = from;
                from = from.offset(1);
                let fresh5 = to;
                to = to.offset(1);
                *fresh5 = *fresh4;
            }
        }
        (*pool).tail = to.offset_from((*pool).contents) as i64 as i32;
        let mut PAC_char: i8 = '\0' as i32 as i8;
        let mut ga_needed_size_0: i64 = ((*pool).tail + 1 as i32) as i64;
        let mut ga_newsize_0: i64 = (*pool).size as i64;
        while ga_newsize_0 < ga_needed_size_0 {
            ga_newsize_0 <<= 1 as i32;
        }
        if ga_newsize_0 != (*pool).size as i64 {
            if (*pool).resized {
                (*pool).contents = xrealloc(
                    (*pool).contents as *mut libc::c_void,
                    (ga_newsize_0 as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                ) as *mut i8;
            } else {
                let mut ga_new_0: *mut libc::c_void = xmalloc(
                    (ga_newsize_0 as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                memcpy(
                    ga_new_0,
                    (*pool).contents as *const libc::c_void,
                    ((*pool).size as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                (*pool).contents = ga_new_0 as *mut i8;
                (*pool).resized = 1 as i32 != 0;
            }
            (*pool).size = ga_newsize_0 as i32;
        }
        let fresh6 = (*pool).tail;
        (*pool).tail = (*pool).tail + 1;
        *((*pool).contents).offset(fresh6 as isize) = PAC_char;
    } else {
        let mut PA_beg: *const i8 = beg;
        let mut PA_size: i32 = end.offset_from(PA_beg) as i64 as i32;
        let mut ga_needed_size_1: i64 = ((*pool).tail + PA_size) as i64;
        let mut ga_newsize_1: i64 = (*pool).size as i64;
        while ga_newsize_1 < ga_needed_size_1 {
            ga_newsize_1 <<= 1 as i32;
        }
        if ga_newsize_1 != (*pool).size as i64 {
            if (*pool).resized {
                (*pool).contents = xrealloc(
                    (*pool).contents as *mut libc::c_void,
                    (ga_newsize_1 as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                ) as *mut i8;
            } else {
                let mut ga_new_1: *mut libc::c_void = xmalloc(
                    (ga_newsize_1 as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                memcpy(
                    ga_new_1,
                    (*pool).contents as *const libc::c_void,
                    ((*pool).size as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                (*pool).contents = ga_new_1 as *mut i8;
                (*pool).resized = 1 as i32 != 0;
            }
            (*pool).size = ga_newsize_1 as i32;
        }
        memcpy(
            ((*pool).contents).offset((*pool).tail as isize) as *mut libc::c_void,
            PA_beg as *const libc::c_void,
            PA_size as u64,
        );
        (*pool).tail += PA_size;
        let mut PAC_char_0: i8 = '\0' as i32 as i8;
        let mut ga_needed_size_2: i64 = ((*pool).tail + 1 as i32) as i64;
        let mut ga_newsize_2: i64 = (*pool).size as i64;
        while ga_newsize_2 < ga_needed_size_2 {
            ga_newsize_2 <<= 1 as i32;
        }
        if ga_newsize_2 != (*pool).size as i64 {
            if (*pool).resized {
                (*pool).contents = xrealloc(
                    (*pool).contents as *mut libc::c_void,
                    (ga_newsize_2 as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                ) as *mut i8;
            } else {
                let mut ga_new_2: *mut libc::c_void = xmalloc(
                    (ga_newsize_2 as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                memcpy(
                    ga_new_2,
                    (*pool).contents as *const libc::c_void,
                    ((*pool).size as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                (*pool).contents = ga_new_2 as *mut i8;
                (*pool).resized = 1 as i32 != 0;
            }
            (*pool).size = ga_newsize_2 as i32;
        }
        let fresh7 = (*pool).tail;
        (*pool).tail = (*pool).tail + 1;
        *((*pool).contents).offset(fresh7 as isize) = PAC_char_0;
    }
    if flags & C2RustUnnamed_0::AP_DOWNCASE as i32 != 0 {
        let mut p: *mut i8 = ((*pool).contents).offset(old_tail as isize);
        while *p != 0 {
            *p = c_tolower(*p as i32) as i8;
            p = p.offset(1);
            p;
        }
    }
}
unsafe extern "C" fn advance_declaration(
    mut beg: *const i8,
    mut end: *const i8,
) -> *const i8 {
    let mut p: *const i8 = beg;
    let mut quote_char: i8 = '\0' as i32 as i8;
    let mut ch: i8 = 0;
    let mut state: C2RustUnnamed = C2RustUnnamed::AC_S_BANG;
    if beg == end {
        return beg;
    }
    let fresh8 = p;
    p = p.offset(1);
    ch = *fresh8;
    while state as u32 != C2RustUnnamed::AC_S_DONE as i32 as u32
        && state as u32 != C2RustUnnamed::AC_S_BACKOUT as i32 as u32
    {
        if p == end {
            state = C2RustUnnamed::AC_S_BACKOUT;
        }
        match state as u32 {
            2 => {
                if ch as i32 == '!' as i32 {
                    let fresh9 = p;
                    p = p.offset(1);
                    ch = *fresh9;
                    state = C2RustUnnamed::AC_S_DEFAULT;
                } else {
                    state = C2RustUnnamed::AC_S_BACKOUT;
                }
            }
            3 => {
                match ch as i32 {
                    45 => {
                        state = C2RustUnnamed::AC_S_DASH1;
                    }
                    32 | 9 | 13 | 10 => {
                        let fresh10 = p;
                        p = p.offset(1);
                        ch = *fresh10;
                    }
                    60 | 62 => {
                        state = C2RustUnnamed::AC_S_DONE;
                    }
                    39 | 34 => {
                        state = C2RustUnnamed::AC_S_QUOTE1;
                    }
                    _ => {
                        if ch as i32 > 32 as i32 && (ch as i32) < 127 as i32
                            && ch as i32 != '=' as i32 && ch as i32 != '<' as i32
                            && ch as i32 != '>' as i32 && ch as i32 != '/' as i32
                        {
                            state = C2RustUnnamed::AC_S_DCLNAME;
                        } else {
                            state = C2RustUnnamed::AC_S_BACKOUT;
                        }
                    }
                }
            }
            4 => {
                if ch as i32 == '-' as i32 {
                    state = C2RustUnnamed::AC_S_DASH1;
                } else if ch as i32 > 32 as i32 && (ch as i32) < 127 as i32
                    && ch as i32 != '=' as i32 && ch as i32 != '<' as i32
                    && ch as i32 != '>' as i32 && ch as i32 != '/' as i32
                {
                    let fresh11 = p;
                    p = p.offset(1);
                    ch = *fresh11;
                } else {
                    state = C2RustUnnamed::AC_S_DEFAULT;
                }
            }
            10 => {
                quote_char = ch;
                let fresh12 = p;
                p = p.offset(1);
                ch = *fresh12;
                state = C2RustUnnamed::AC_S_IN_QUOTE;
            }
            11 => {
                if ch as i32 == quote_char as i32 {
                    state = C2RustUnnamed::AC_S_QUOTE2;
                } else {
                    let fresh13 = p;
                    p = p.offset(1);
                    ch = *fresh13;
                }
            }
            12 => {
                let fresh14 = p;
                p = p.offset(1);
                ch = *fresh14;
                state = C2RustUnnamed::AC_S_DEFAULT;
            }
            5 => {
                let fresh15 = p;
                p = p.offset(1);
                ch = *fresh15;
                state = C2RustUnnamed::AC_S_DASH2;
            }
            6 => {
                match ch as i32 {
                    45 => {
                        let fresh16 = p;
                        p = p.offset(1);
                        ch = *fresh16;
                        state = C2RustUnnamed::AC_S_COMMENT;
                    }
                    _ => {
                        state = C2RustUnnamed::AC_S_BACKOUT;
                    }
                }
            }
            7 => {
                match ch as i32 {
                    45 => {
                        state = C2RustUnnamed::AC_S_DASH3;
                    }
                    _ => {
                        let fresh17 = p;
                        p = p.offset(1);
                        ch = *fresh17;
                    }
                }
            }
            8 => {
                let fresh18 = p;
                p = p.offset(1);
                ch = *fresh18;
                state = C2RustUnnamed::AC_S_DASH4;
            }
            9 => {
                match ch as i32 {
                    45 => {
                        let fresh19 = p;
                        p = p.offset(1);
                        ch = *fresh19;
                        state = C2RustUnnamed::AC_S_DEFAULT;
                    }
                    _ => {
                        state = C2RustUnnamed::AC_S_COMMENT;
                    }
                }
            }
            0 | 1 | _ => {}
        }
    }
    if state as u32 == C2RustUnnamed::AC_S_BACKOUT as i32 as u32 {
        return beg.offset(1 as i32 as isize);
    }
    return p;
}
unsafe extern "C" fn find_comment_end(
    mut beg: *const i8,
    mut end: *const i8,
) -> *const i8 {
    let mut p: *const i8 = beg.offset(-(1 as i32 as isize));
    loop {
        p = p.offset(3 as i32 as isize);
        if !(p < end) {
            break;
        }
        match *p.offset(0 as i32 as isize) as i32 {
            62 => {
                if *p.offset(-(1 as i32) as isize) as i32 == '-' as i32
                    && *p.offset(-(2 as i32) as isize) as i32 == '-' as i32
                {
                    return p.offset(1 as i32 as isize);
                }
            }
            45 => {
                loop {
                    if *p.offset(-(1 as i32) as isize) as i32 == '-' as i32 {
                        's_46: {
                            loop {
                                p = p.offset(1);
                                if p == end {
                                    return 0 as *const i8;
                                }
                                match *p.offset(0 as i32 as isize) as i32 {
                                    62 => return p.offset(1 as i32 as isize),
                                    45 => {}
                                    _ => {
                                        break 's_46;
                                    }
                                }
                            }
                        }
                        break;
                    } else {
                        p = p.offset(2 as i32 as isize);
                        if p >= end {
                            return 0 as *const i8;
                        }
                        match *p.offset(0 as i32 as isize) as i32 {
                            62 => {}
                            45 => {
                                continue;
                            }
                            _ => {
                                break;
                            }
                        }
                        if *p.offset(-(1 as i32) as isize) as i32 == '-' as i32 {
                            return p.offset(1 as i32 as isize);
                        }
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    return 0 as *const i8;
}
unsafe extern "C" fn name_allowed(
    mut ht: *const hash_table,
    mut b: *const i8,
    mut e: *const i8,
) -> bool {
    let mut buf: [i8; 256] = [0; 256];
    let mut copy: *mut i8 = 0 as *mut i8;
    let mut len: size_t = e.offset_from(b) as i64 as size_t;
    let mut ret: bool = false;
    if ht.is_null() {
        return 1 as i32 != 0;
    }
    if len < ::core::mem::size_of::<[i8; 256]>() as u64 {
        copy = buf.as_mut_ptr();
    } else {
        copy = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
    }
    memcpy(copy as *mut libc::c_void, b as *const libc::c_void, len);
    *copy.offset(len as isize) = 0 as i32 as i8;
    ret = !(hash_table_get(ht, copy as *const libc::c_void)).is_null();
    if copy != buf.as_mut_ptr() {
        rpl_free(copy as *mut libc::c_void);
        copy = 0 as *mut i8;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn map_html_tags(
    mut text: *const i8,
    mut size: i32,
    mut mapfun: Option<unsafe extern "C" fn(*mut taginfo, *mut libc::c_void) -> ()>,
    mut maparg: *mut libc::c_void,
    mut flags: i32,
    mut allowed_tags: *const hash_table,
    mut allowed_attributes: *const hash_table,
) {
    let mut current_block: u64;
    let mut pool_initial_storage: [i8; 256] = [0; 256];
    let mut pool: pool = pool {
        contents: 0 as *mut i8,
        size: 0,
        tail: 0,
        resized: false,
        orig_contents: 0 as *mut i8,
        orig_size: 0,
    };
    let mut p: *const i8 = text;
    let mut end: *const i8 = text.offset(size as isize);
    let mut attr_pair_initial_storage: [attr_pair; 8] = [attr_pair {
        name: 0 as *mut i8,
        value: 0 as *mut i8,
        value_raw_beginning: 0 as *const i8,
        value_raw_size: 0,
        name_pool_index: 0,
        value_pool_index: 0,
    }; 8];
    let mut attr_pair_size: i32 = (::core::mem::size_of::<[attr_pair; 8]>() as u64)
        .wrapping_div(::core::mem::size_of::<attr_pair>() as u64) as i32;
    let mut attr_pair_resized: bool = 0 as i32 != 0;
    let mut pairs: *mut attr_pair = attr_pair_initial_storage.as_mut_ptr();
    let mut head: *mut tagstack_item = 0 as *mut tagstack_item;
    let mut tail: *mut tagstack_item = 0 as *mut tagstack_item;
    if size == 0 {
        return;
    }
    let mut P: *mut pool = &mut pool;
    (*P).contents = pool_initial_storage.as_mut_ptr();
    (*P).size = (::core::mem::size_of::<[i8; 256]>() as u64)
        .wrapping_div(::core::mem::size_of::<i8>() as u64) as i32;
    (*P).tail = 0 as i32;
    (*P).resized = 0 as i32 != 0;
    (*P).orig_contents = (*P).contents;
    (*P).orig_size = (*P).size;
    let mut nattrs: i32 = 0;
    let mut end_tag: i32 = 0;
    let mut tag_name_begin: *const i8 = 0 as *const i8;
    let mut tag_name_end: *const i8 = 0 as *const i8;
    let mut tag_start_position: *const i8 = 0 as *const i8;
    let mut uninteresting_tag: bool = false;
    '_look_for_tag: loop {
        pool.tail = 0 as i32;
        nattrs = 0 as i32;
        end_tag = 0 as i32;
        p = memchr(
            p as *const libc::c_void,
            '<' as i32,
            end.offset_from(p) as i64 as u64,
        ) as *const i8;
        if p.is_null() {
            break;
        }
        tag_start_position = p;
        p = p.offset(1);
        p;
        if p >= end {
            break;
        }
        if *p as i32 == '!' as i32 {
            if flags & 1 as i32 == 0 && p.offset(3 as i32 as isize) < end
                && *p.offset(1 as i32 as isize) as i32 == '-' as i32
                && *p.offset(2 as i32 as isize) as i32 == '-' as i32
            {
                let mut comment_end: *const i8 = find_comment_end(
                    p.offset(3 as i32 as isize),
                    end,
                );
                if !comment_end.is_null() {
                    p = comment_end;
                }
            } else {
                p = advance_declaration(p, end);
            }
            if p == end {
                break;
            }
        } else {
            if *p as i32 == '/' as i32 {
                end_tag = 1 as i32;
                p = p.offset(1);
                p;
                if p >= end {
                    break;
                }
            }
            tag_name_begin = p;
            while *p as i32 > 32 as i32 && (*p as i32) < 127 as i32
                && *p as i32 != '=' as i32 && *p as i32 != '<' as i32
                && *p as i32 != '>' as i32 && *p as i32 != '/' as i32
            {
                p = p.offset(1);
                p;
                if p >= end {
                    break '_look_for_tag;
                }
            }
            if p == tag_name_begin {
                continue;
            }
            tag_name_end = p;
            while c_isspace(*p as i32) {
                p = p.offset(1);
                p;
                if p >= end {
                    break '_look_for_tag;
                }
            }
            if end_tag == 0 {
                let mut ts: *mut tagstack_item = tagstack_push(&mut head, &mut tail);
                if !ts.is_null() {
                    (*ts).tagname_begin = tag_name_begin;
                    (*ts).tagname_end = tag_name_end;
                    (*ts).contents_begin = 0 as *const i8;
                }
            }
            if !(end_tag != 0 && *p as i32 != '>' as i32 && *p as i32 != '<' as i32) {
                if !name_allowed(allowed_tags, tag_name_begin, tag_name_end) {
                    uninteresting_tag = 1 as i32 != 0;
                } else {
                    uninteresting_tag = 0 as i32 != 0;
                    convert_and_copy(
                        &mut pool,
                        tag_name_begin,
                        tag_name_end,
                        C2RustUnnamed_0::AP_DOWNCASE as i32,
                    );
                }
                loop {
                    let mut attr_name_begin: *const i8 = 0 as *const i8;
                    let mut attr_name_end: *const i8 = 0 as *const i8;
                    let mut attr_value_begin: *const i8 = 0 as *const i8;
                    let mut attr_value_end: *const i8 = 0 as *const i8;
                    let mut attr_raw_value_begin: *const i8 = 0 as *const i8;
                    let mut attr_raw_value_end: *const i8 = 0 as *const i8;
                    let mut operation: i32 = C2RustUnnamed_0::AP_DOWNCASE as i32;
                    while c_isspace(*p as i32) {
                        p = p.offset(1);
                        p;
                        if p >= end {
                            break '_look_for_tag;
                        }
                    }
                    if *p as i32 == '/' as i32 {
                        p = p.offset(1);
                        p;
                        if p >= end {
                            break '_look_for_tag;
                        }
                        while c_isspace(*p as i32) {
                            p = p.offset(1);
                            p;
                            if p >= end {
                                break '_look_for_tag;
                            }
                        }
                        if *p as i32 != '<' as i32 && *p as i32 != '>' as i32 {
                            current_block = 6354269463029205598;
                            break;
                        }
                    }
                    if *p as i32 == '<' as i32 || *p as i32 == '>' as i32 {
                        current_block = 9270770154621591809;
                        break;
                    }
                    attr_name_begin = p;
                    while *p as i32 > 32 as i32 && (*p as i32) < 127 as i32
                        && *p as i32 != '=' as i32 && *p as i32 != '<' as i32
                        && *p as i32 != '>' as i32 && *p as i32 != '/' as i32
                    {
                        p = p.offset(1);
                        p;
                        if p >= end {
                            break '_look_for_tag;
                        }
                    }
                    attr_name_end = p;
                    if attr_name_begin == attr_name_end {
                        current_block = 6354269463029205598;
                        break;
                    }
                    while c_isspace(*p as i32) {
                        p = p.offset(1);
                        p;
                        if p >= end {
                            break '_look_for_tag;
                        }
                    }
                    if *p as i32 > 32 as i32 && (*p as i32) < 127 as i32
                        && *p as i32 != '=' as i32 && *p as i32 != '<' as i32
                        && *p as i32 != '>' as i32 && *p as i32 != '/' as i32
                        || *p as i32 == '/' as i32 || *p as i32 == '<' as i32
                        || *p as i32 == '>' as i32
                    {
                        attr_value_begin = attr_name_begin;
                        attr_raw_value_begin = attr_value_begin;
                        attr_value_end = attr_name_end;
                        attr_raw_value_end = attr_value_end;
                    } else if *p as i32 == '=' as i32 {
                        p = p.offset(1);
                        p;
                        if p >= end {
                            break '_look_for_tag;
                        }
                        while c_isspace(*p as i32) {
                            p = p.offset(1);
                            p;
                            if p >= end {
                                break '_look_for_tag;
                            }
                        }
                        if *p as i32 == '"' as i32 || *p as i32 == '\'' as i32 {
                            let mut newline_seen: bool = 0 as i32 != 0;
                            let mut quote_char: i8 = *p;
                            attr_raw_value_begin = p;
                            p = p.offset(1);
                            p;
                            if p >= end {
                                break '_look_for_tag;
                            }
                            attr_value_begin = p;
                            while *p as i32 != quote_char as i32 {
                                if !newline_seen && *p as i32 == '\n' as i32 {
                                    p = attr_value_begin;
                                    newline_seen = 1 as i32 != 0;
                                } else {
                                    if newline_seen as i32 != 0
                                        && (*p as i32 == '<' as i32 || *p as i32 == '>' as i32)
                                    {
                                        break;
                                    }
                                    p = p.offset(1);
                                    p;
                                    if p >= end {
                                        break '_look_for_tag;
                                    }
                                }
                            }
                            attr_value_end = p;
                            if !(*p as i32 == quote_char as i32) {
                                continue '_look_for_tag;
                            }
                            p = p.offset(1);
                            p;
                            if p >= end {
                                break '_look_for_tag;
                            }
                            attr_raw_value_end = p;
                            operation = C2RustUnnamed_0::AP_DECODE_ENTITIES as i32;
                            if flags & 2 as i32 != 0 {
                                operation |= C2RustUnnamed_0::AP_TRIM_BLANKS as i32;
                            }
                        } else {
                            attr_value_begin = p;
                            while !c_isspace(*p as i32) && *p as i32 != '<' as i32
                                && *p as i32 != '>' as i32
                            {
                                p = p.offset(1);
                                p;
                                if p >= end {
                                    break '_look_for_tag;
                                }
                            }
                            attr_value_end = p;
                            if attr_value_begin == attr_value_end {
                                current_block = 6354269463029205598;
                                break;
                            } else {
                                attr_raw_value_begin = attr_value_begin;
                                attr_raw_value_end = attr_value_end;
                                operation = C2RustUnnamed_0::AP_DECODE_ENTITIES as i32;
                            }
                        }
                    } else {
                        current_block = 6354269463029205598;
                        break;
                    }
                    if uninteresting_tag {
                        continue;
                    }
                    if !name_allowed(
                        allowed_attributes,
                        attr_name_begin,
                        attr_name_end,
                    ) {
                        continue;
                    }
                    let mut ga_needed_size: i64 = (nattrs + 1 as i32) as i64;
                    let mut ga_newsize: i64 = attr_pair_size as i64;
                    while ga_newsize < ga_needed_size {
                        ga_newsize <<= 1 as i32;
                    }
                    if ga_newsize != attr_pair_size as i64 {
                        if attr_pair_resized {
                            pairs = xrealloc(
                                pairs as *mut libc::c_void,
                                (ga_newsize as u64)
                                    .wrapping_mul(::core::mem::size_of::<attr_pair>() as u64),
                            ) as *mut attr_pair;
                        } else {
                            let mut ga_new: *mut libc::c_void = xmalloc(
                                (ga_newsize as u64)
                                    .wrapping_mul(::core::mem::size_of::<attr_pair>() as u64),
                            );
                            memcpy(
                                ga_new,
                                pairs as *const libc::c_void,
                                (attr_pair_size as u64)
                                    .wrapping_mul(::core::mem::size_of::<attr_pair>() as u64),
                            );
                            pairs = ga_new as *mut attr_pair;
                            attr_pair_resized = 1 as i32 != 0;
                        }
                        attr_pair_size = ga_newsize as i32;
                    }
                    (*pairs.offset(nattrs as isize)).name_pool_index = pool.tail;
                    convert_and_copy(
                        &mut pool,
                        attr_name_begin,
                        attr_name_end,
                        C2RustUnnamed_0::AP_DOWNCASE as i32,
                    );
                    (*pairs.offset(nattrs as isize)).value_pool_index = pool.tail;
                    convert_and_copy(
                        &mut pool,
                        attr_value_begin,
                        attr_value_end,
                        operation,
                    );
                    let ref mut fresh20 = (*pairs.offset(nattrs as isize))
                        .value_raw_beginning;
                    *fresh20 = attr_raw_value_begin;
                    (*pairs.offset(nattrs as isize)).value_raw_size = attr_raw_value_end
                        .offset_from(attr_raw_value_begin) as i64 as i32;
                    nattrs += 1;
                    nattrs;
                }
                match current_block {
                    6354269463029205598 => {}
                    _ => {
                        if end_tag == 0 && !tail.is_null()
                            && (*tail).tagname_begin == tag_name_begin
                        {
                            (*tail).contents_begin = p.offset(1 as i32 as isize);
                        }
                        if uninteresting_tag {
                            p = p.offset(1);
                            p;
                            if p >= end {
                                break;
                            } else {
                                continue;
                            }
                        } else {
                            let mut i: i32 = 0;
                            let mut taginfo: taginfo = taginfo {
                                name: 0 as *mut i8,
                                end_tag_p: 0,
                                nattrs: 0,
                                attrs: 0 as *mut attr_pair,
                                start_position: 0 as *const i8,
                                end_position: 0 as *const i8,
                                contents_begin: 0 as *const i8,
                                contents_end: 0 as *const i8,
                            };
                            let mut ts_0: *mut tagstack_item = 0 as *mut tagstack_item;
                            taginfo.name = pool.contents;
                            taginfo.end_tag_p = end_tag;
                            taginfo.nattrs = nattrs;
                            i = 0 as i32;
                            while i < nattrs {
                                let ref mut fresh21 = (*pairs.offset(i as isize)).name;
                                *fresh21 = (pool.contents)
                                    .offset(
                                        (*pairs.offset(i as isize)).name_pool_index as isize,
                                    );
                                let ref mut fresh22 = (*pairs.offset(i as isize)).value;
                                *fresh22 = (pool.contents)
                                    .offset(
                                        (*pairs.offset(i as isize)).value_pool_index as isize,
                                    );
                                i += 1;
                                i;
                            }
                            taginfo.attrs = pairs;
                            taginfo.start_position = tag_start_position;
                            taginfo.end_position = p.offset(1 as i32 as isize);
                            taginfo.contents_begin = 0 as *const i8;
                            taginfo.contents_end = 0 as *const i8;
                            if end_tag != 0 {
                                ts_0 = tagstack_find(tail, tag_name_begin, tag_name_end);
                                if !ts_0.is_null() {
                                    if !((*ts_0).contents_begin).is_null() {
                                        taginfo.contents_begin = (*ts_0).contents_begin;
                                        taginfo.contents_end = tag_start_position;
                                    }
                                    tagstack_pop(&mut head, &mut tail, ts_0);
                                }
                            }
                            mapfun
                                .expect("non-null function pointer")(&mut taginfo, maparg);
                            if !(*p as i32 != '<' as i32) {
                                continue;
                            }
                            p = p.offset(1);
                            p;
                            if p >= end {
                                break;
                            } else {
                                continue;
                            }
                        }
                    }
                }
            }
            p = tag_start_position.offset(1 as i32 as isize);
        }
    }
    let mut P_0: *mut pool = &mut pool;
    if (*P_0).resized {
        rpl_free((*P_0).contents as *mut libc::c_void);
        (*P_0).contents = 0 as *mut i8;
    }
    (*P_0).contents = (*P_0).orig_contents;
    (*P_0).size = (*P_0).orig_size;
    (*P_0).tail = 0 as i32;
    (*P_0).resized = 0 as i32 != 0;
    if attr_pair_resized {
        rpl_free(pairs as *mut libc::c_void);
        pairs = 0 as *mut attr_pair;
    }
    tagstack_pop(&mut head, &mut tail, head);
}