#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __errno_location() -> *mut i32;
    fn abort() -> !;
    fn strerror(_: i32) -> *mut i8;
    fn nettle_buffer_reset(buffer: *mut nettle_buffer);
    fn nettle_buffer_grow(buffer: *mut nettle_buffer, length: size_t) -> i32;
    static nettle_base64: nettle_armor;
    static nettle_base16: nettle_armor;
    fn die(format: *const i8, _: ...) -> !;
    static sexp_token_chars: [i8; 128];
    fn _IO_getc(__fp: *mut _IO_FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __off_t = i64;
pub type __off64_t = i64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum sexp_mode {
    SEXP_CANONICAL = 0,
    SEXP_ADVANCED = 1,
    SEXP_TRANSPORT = 2,
}
impl sexp_mode {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            sexp_mode::SEXP_CANONICAL => 0,
            sexp_mode::SEXP_ADVANCED => 1,
            sexp_mode::SEXP_TRANSPORT => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> sexp_mode {
        match value {
            0 => sexp_mode::SEXP_CANONICAL,
            1 => sexp_mode::SEXP_ADVANCED,
            2 => sexp_mode::SEXP_TRANSPORT,
            _ => panic!("Invalid value for sexp_mode: {}", value),
        }
    }
}
impl AddAssign<u32> for sexp_mode {
    fn add_assign(&mut self, rhs: u32) {
        *self = sexp_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for sexp_mode {
    fn sub_assign(&mut self, rhs: u32) {
        *self = sexp_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for sexp_mode {
    fn mul_assign(&mut self, rhs: u32) {
        *self = sexp_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for sexp_mode {
    fn div_assign(&mut self, rhs: u32) {
        *self = sexp_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for sexp_mode {
    fn rem_assign(&mut self, rhs: u32) {
        *self = sexp_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for sexp_mode {
    type Output = sexp_mode;
    fn add(self, rhs: u32) -> sexp_mode {
        sexp_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for sexp_mode {
    type Output = sexp_mode;
    fn sub(self, rhs: u32) -> sexp_mode {
        sexp_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for sexp_mode {
    type Output = sexp_mode;
    fn mul(self, rhs: u32) -> sexp_mode {
        sexp_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for sexp_mode {
    type Output = sexp_mode;
    fn div(self, rhs: u32) -> sexp_mode {
        sexp_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for sexp_mode {
    type Output = sexp_mode;
    fn rem(self, rhs: u32) -> sexp_mode {
        sexp_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum sexp_token {
    SEXP_STRING,
    SEXP_DISPLAY,
    SEXP_COMMENT,
    SEXP_LIST_START,
    SEXP_LIST_END,
    SEXP_EOF,
    SEXP_DISPLAY_START,
    SEXP_DISPLAY_END,
    SEXP_TRANSPORT_START,
    SEXP_CODING_END,
}
impl sexp_token {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            sexp_token::SEXP_STRING => 0,
            sexp_token::SEXP_DISPLAY => 1,
            sexp_token::SEXP_COMMENT => 2,
            sexp_token::SEXP_LIST_START => 3,
            sexp_token::SEXP_LIST_END => 4,
            sexp_token::SEXP_EOF => 5,
            sexp_token::SEXP_DISPLAY_START => 6,
            sexp_token::SEXP_DISPLAY_END => 7,
            sexp_token::SEXP_TRANSPORT_START => 8,
            sexp_token::SEXP_CODING_END => 9,
        }
    }
    fn from_libc_c_uint(value: u32) -> sexp_token {
        match value {
            0 => sexp_token::SEXP_STRING,
            1 => sexp_token::SEXP_DISPLAY,
            2 => sexp_token::SEXP_COMMENT,
            3 => sexp_token::SEXP_LIST_START,
            4 => sexp_token::SEXP_LIST_END,
            5 => sexp_token::SEXP_EOF,
            6 => sexp_token::SEXP_DISPLAY_START,
            7 => sexp_token::SEXP_DISPLAY_END,
            8 => sexp_token::SEXP_TRANSPORT_START,
            9 => sexp_token::SEXP_CODING_END,
            _ => panic!("Invalid value for sexp_token: {}", value),
        }
    }
}
impl AddAssign<u32> for sexp_token {
    fn add_assign(&mut self, rhs: u32) {
        *self = sexp_token::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for sexp_token {
    fn sub_assign(&mut self, rhs: u32) {
        *self = sexp_token::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for sexp_token {
    fn mul_assign(&mut self, rhs: u32) {
        *self = sexp_token::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for sexp_token {
    fn div_assign(&mut self, rhs: u32) {
        *self = sexp_token::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for sexp_token {
    fn rem_assign(&mut self, rhs: u32) {
        *self = sexp_token::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for sexp_token {
    type Output = sexp_token;
    fn add(self, rhs: u32) -> sexp_token {
        sexp_token::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for sexp_token {
    type Output = sexp_token;
    fn sub(self, rhs: u32) -> sexp_token {
        sexp_token::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for sexp_token {
    type Output = sexp_token;
    fn mul(self, rhs: u32) -> sexp_token {
        sexp_token::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for sexp_token {
    type Output = sexp_token;
    fn div(self, rhs: u32) -> sexp_token {
        sexp_token::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for sexp_token {
    type Output = sexp_token;
    fn rem(self, rhs: u32) -> sexp_token {
        sexp_token::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type uint8_t = __uint8_t;
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
pub type nettle_armor_length_func = unsafe extern "C" fn(size_t) -> size_t;
pub type nettle_armor_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_armor_encode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut i8,
    size_t,
    *const uint8_t,
) -> size_t;
pub type nettle_armor_encode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut i8,
) -> size_t;
pub type nettle_armor_decode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut size_t,
    *mut uint8_t,
    size_t,
    *const i8,
) -> i32;
pub type nettle_armor_decode_final_func = unsafe extern "C" fn(*mut libc::c_void) -> i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base16_decode_ctx {
    pub word: u8,
    pub bits: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: u8,
    pub padding: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_buffer {
    pub contents: *mut uint8_t,
    pub alloc: size_t,
    pub realloc_ctx: *mut libc::c_void,
    pub realloc: Option<nettle_realloc_func>,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_armor {
    pub name: *const i8,
    pub encode_context_size: u32,
    pub decode_context_size: u32,
    pub encode_final_length: u32,
    pub encode_init: Option<nettle_armor_init_func>,
    pub encode_length: Option<nettle_armor_length_func>,
    pub encode_update: Option<nettle_armor_encode_update_func>,
    pub encode_final: Option<nettle_armor_encode_final_func>,
    pub decode_init: Option<nettle_armor_init_func>,
    pub decode_length: Option<nettle_armor_length_func>,
    pub decode_update: Option<nettle_armor_decode_update_func>,
    pub decode_final: Option<nettle_armor_decode_final_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum sexp_char_type {
    SEXP_NORMAL_CHAR = 0,
    SEXP_EOF_CHAR,
    SEXP_END_CHAR,
}
impl sexp_char_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            sexp_char_type::SEXP_NORMAL_CHAR => 0,
            sexp_char_type::SEXP_EOF_CHAR => 1,
            sexp_char_type::SEXP_END_CHAR => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> sexp_char_type {
        match value {
            0 => sexp_char_type::SEXP_NORMAL_CHAR,
            1 => sexp_char_type::SEXP_EOF_CHAR,
            2 => sexp_char_type::SEXP_END_CHAR,
            _ => panic!("Invalid value for sexp_char_type: {}", value),
        }
    }
}
impl AddAssign<u32> for sexp_char_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for sexp_char_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for sexp_char_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for sexp_char_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for sexp_char_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for sexp_char_type {
    type Output = sexp_char_type;
    fn add(self, rhs: u32) -> sexp_char_type {
        sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for sexp_char_type {
    type Output = sexp_char_type;
    fn sub(self, rhs: u32) -> sexp_char_type {
        sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for sexp_char_type {
    type Output = sexp_char_type;
    fn mul(self, rhs: u32) -> sexp_char_type {
        sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for sexp_char_type {
    type Output = sexp_char_type;
    fn div(self, rhs: u32) -> sexp_char_type {
        sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for sexp_char_type {
    type Output = sexp_char_type;
    fn rem(self, rhs: u32) -> sexp_char_type {
        sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_input {
    pub f: *mut FILE,
    pub ctype: sexp_char_type,
    pub c: uint8_t,
    pub coding: *const nettle_armor,
    pub state: C2RustUnnamed,
    pub terminator: uint8_t,
    pub token: sexp_token,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub base64: base64_decode_ctx,
    pub hex: base16_decode_ctx,
}
#[no_mangle]
pub unsafe extern "C" fn sexp_input_init(mut input: *mut sexp_input, mut f: *mut FILE) {
    (*input).f = f;
    (*input).coding = 0 as *const nettle_armor;
}
unsafe extern "C" fn sexp_get_raw_char(mut input: *mut sexp_input) {
    let mut c: i32 = _IO_getc((*input).f);
    if c < 0 as i32 {
        if ferror((*input).f) != 0 {
            die(
                b"Read error: %s\n\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        (*input).ctype = sexp_char_type::SEXP_EOF_CHAR;
    } else {
        (*input).ctype = sexp_char_type::SEXP_NORMAL_CHAR;
        (*input).c = c as uint8_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn sexp_get_char(mut input: *mut sexp_input) {
    if !((*input).coding).is_null() {
        loop {
            let mut done: size_t = 0;
            sexp_get_raw_char(input);
            if (*input).ctype as u32 == sexp_char_type::SEXP_EOF_CHAR as i32 as u32 {
                die(
                    b"Unexpected end of file in coded data.\n\0" as *const u8
                        as *const i8,
                );
            }
            if (*input).c as i32 == (*input).terminator as i32 {
                (*input).ctype = sexp_char_type::SEXP_END_CHAR;
                return;
            }
            done = 1 as i32 as size_t;
            if ((*(*input).coding).decode_update)
                .expect(
                    "non-null function pointer",
                )(
                &mut (*input).state as *mut C2RustUnnamed as *mut libc::c_void,
                &mut done,
                &mut (*input).c,
                1 as i32 as size_t,
                &mut (*input).c as *mut uint8_t as *const i8,
            ) == 0
            {
                die(b"Invalid coded data.\n\0" as *const u8 as *const i8);
            }
            if done != 0 {
                return;
            }
        }
    } else {
        sexp_get_raw_char(input);
    };
}
unsafe extern "C" fn sexp_next_char(mut input: *mut sexp_input) -> uint8_t {
    sexp_get_char(input);
    if (*input).ctype as u32 != sexp_char_type::SEXP_NORMAL_CHAR as i32 as u32 {
        die(b"Unexpected end of file.\n\0" as *const u8 as *const i8);
    }
    return (*input).c;
}
unsafe extern "C" fn sexp_push_char(
    mut input: *mut sexp_input,
    mut string: *mut nettle_buffer,
) {
    if (*input).ctype as u32 == sexp_char_type::SEXP_NORMAL_CHAR as i32 as u32 {} else {
        __assert_fail(
            b"input->ctype == sexp_char_type::SEXP_NORMAL_CHAR\0" as *const u8
                as *const i8,
            b"input.c\0" as *const u8 as *const i8,
            117 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[i8; 65],
            >(b"void sexp_push_char(struct sexp_input *, struct nettle_buffer *)\0"))
                .as_ptr(),
        );
    }
    'c_3098: {
        if (*input).ctype as u32 == sexp_char_type::SEXP_NORMAL_CHAR as i32 as u32
        {} else {
            __assert_fail(
                b"input->ctype == sexp_char_type::SEXP_NORMAL_CHAR\0" as *const u8
                    as *const i8,
                b"input.c\0" as *const u8 as *const i8,
                117 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[i8; 65],
                >(b"void sexp_push_char(struct sexp_input *, struct nettle_buffer *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(((*string).size < (*string).alloc
        || nettle_buffer_grow(string, 1 as i32 as size_t) != 0)
        && {
            let fresh0 = (*string).size;
            (*string).size = ((*string).size).wrapping_add(1);
            *((*string).contents).offset(fresh0 as isize) = (*input).c;
            1 as i32 != 0
        })
    {
        die(b"Virtual memory exhasuted.\n\0" as *const u8 as *const i8);
    }
}
unsafe extern "C" fn sexp_input_start_coding(
    mut input: *mut sexp_input,
    mut coding: *const nettle_armor,
    mut terminator: uint8_t,
) {
    if ((*input).coding).is_null() {} else {
        __assert_fail(
            b"!input->coding\0" as *const u8 as *const i8,
            b"input.c\0" as *const u8 as *const i8,
            128 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 88],
                &[i8; 88],
            >(
                b"void sexp_input_start_coding(struct sexp_input *, const struct nettle_armor *, uint8_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3424: {
        if ((*input).coding).is_null() {} else {
            __assert_fail(
                b"!input->coding\0" as *const u8 as *const i8,
                b"input.c\0" as *const u8 as *const i8,
                128 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 88],
                    &[i8; 88],
                >(
                    b"void sexp_input_start_coding(struct sexp_input *, const struct nettle_armor *, uint8_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*input).coding = coding;
    ((*(*input).coding).decode_init)
        .expect(
            "non-null function pointer",
        )(&mut (*input).state as *mut C2RustUnnamed as *mut libc::c_void);
    (*input).terminator = terminator;
}
unsafe extern "C" fn sexp_input_end_coding(mut input: *mut sexp_input) {
    if !((*input).coding).is_null() {} else {
        __assert_fail(
            b"input->coding\0" as *const u8 as *const i8,
            b"input.c\0" as *const u8 as *const i8,
            138 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[i8; 48],
            >(b"void sexp_input_end_coding(struct sexp_input *)\0"))
                .as_ptr(),
        );
    }
    'c_3311: {
        if !((*input).coding).is_null() {} else {
            __assert_fail(
                b"input->coding\0" as *const u8 as *const i8,
                b"input.c\0" as *const u8 as *const i8,
                138 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[i8; 48],
                >(b"void sexp_input_end_coding(struct sexp_input *)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*(*input).coding).decode_final)
        .expect(
            "non-null function pointer",
        )(&mut (*input).state as *mut C2RustUnnamed as *mut libc::c_void) == 0
    {
        die(b"Invalid coded data.\n\0" as *const u8 as *const i8);
    }
    (*input).coding = 0 as *const nettle_armor;
}
unsafe extern "C" fn sexp_get_quoted_char(mut input: *mut sexp_input) -> i32 {
    sexp_next_char(input);
    match (*input).c as i32 {
        34 => return 0 as i32,
        92 => {
            sexp_next_char(input);
            match (*input).c as i32 {
                98 => {
                    (*input).c = '\u{8}' as i32 as uint8_t;
                    return 1 as i32;
                }
                116 => {
                    (*input).c = '\t' as i32 as uint8_t;
                    return 1 as i32;
                }
                110 => {
                    (*input).c = '\n' as i32 as uint8_t;
                    return 1 as i32;
                }
                102 => {
                    (*input).c = '\u{c}' as i32 as uint8_t;
                    return 1 as i32;
                }
                114 => {
                    (*input).c = '\r' as i32 as uint8_t;
                    return 1 as i32;
                }
                92 => {
                    (*input).c = '\\' as i32 as uint8_t;
                    return 1 as i32;
                }
                111 | 120 => {
                    abort();
                }
                10 => {
                    if sexp_next_char(input) as i32 == '\r' as i32 {
                        sexp_next_char(input);
                    }
                }
                13 => {
                    if sexp_next_char(input) as i32 == '\n' as i32 {
                        sexp_next_char(input);
                    }
                }
                _ => {}
            }
            return 1 as i32;
        }
        _ => return 1 as i32,
    };
}
unsafe extern "C" fn sexp_get_token_string(
    mut input: *mut sexp_input,
    mut string: *mut nettle_buffer,
) {
    if ((*input).coding).is_null() {} else {
        __assert_fail(
            b"!input->coding\0" as *const u8 as *const i8,
            b"input.c\0" as *const u8 as *const i8,
            193 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[i8; 72],
            >(
                b"void sexp_get_token_string(struct sexp_input *, struct nettle_buffer *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3215: {
        if ((*input).coding).is_null() {} else {
            __assert_fail(
                b"!input->coding\0" as *const u8 as *const i8,
                b"input.c\0" as *const u8 as *const i8,
                193 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[i8; 72],
                >(
                    b"void sexp_get_token_string(struct sexp_input *, struct nettle_buffer *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*input).ctype as u32 == sexp_char_type::SEXP_NORMAL_CHAR as i32 as u32 {} else {
        __assert_fail(
            b"input->ctype == sexp_char_type::SEXP_NORMAL_CHAR\0" as *const u8
                as *const i8,
            b"input.c\0" as *const u8 as *const i8,
            194 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[i8; 72],
            >(
                b"void sexp_get_token_string(struct sexp_input *, struct nettle_buffer *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3171: {
        if (*input).ctype as u32 == sexp_char_type::SEXP_NORMAL_CHAR as i32 as u32
        {} else {
            __assert_fail(
                b"input->ctype == sexp_char_type::SEXP_NORMAL_CHAR\0" as *const u8
                    as *const i8,
                b"input.c\0" as *const u8 as *const i8,
                194 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[i8; 72],
                >(
                    b"void sexp_get_token_string(struct sexp_input *, struct nettle_buffer *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !(((*input).c as i32) < 0x80 as i32
        && sexp_token_chars[(*input).c as usize] as i32 != 0)
    {
        die(b"Invalid token.\n\0" as *const u8 as *const i8);
    }
    loop {
        sexp_push_char(input, string);
        sexp_get_char(input);
        if !((*input).ctype as u32 == sexp_char_type::SEXP_NORMAL_CHAR as i32 as u32
            && (((*input).c as i32) < 0x80 as i32
                && sexp_token_chars[(*input).c as usize] as i32 != 0))
        {
            break;
        }
    }
    if (*string).size != 0 {} else {
        __assert_fail(
            b"string->size\0" as *const u8 as *const i8,
            b"input.c\0" as *const u8 as *const i8,
            206 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[i8; 72],
            >(
                b"void sexp_get_token_string(struct sexp_input *, struct nettle_buffer *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2952: {
        if (*string).size != 0 {} else {
            __assert_fail(
                b"string->size\0" as *const u8 as *const i8,
                b"input.c\0" as *const u8 as *const i8,
                206 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[i8; 72],
                >(
                    b"void sexp_get_token_string(struct sexp_input *, struct nettle_buffer *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn sexp_get_string(
    mut input: *mut sexp_input,
    mut string: *mut nettle_buffer,
) {
    nettle_buffer_reset(string);
    (*input).token = sexp_token::SEXP_STRING;
    let mut current_block_16: u64;
    match (*input).c as i32 {
        34 => {
            while sexp_get_quoted_char(input) != 0 {
                sexp_push_char(input, string);
            }
            sexp_get_char(input);
            current_block_16 = 6009453772311597924;
        }
        35 => {
            sexp_input_start_coding(input, &nettle_base16, '#' as i32 as uint8_t);
            current_block_16 = 11657864150791474432;
        }
        124 => {
            sexp_input_start_coding(input, &nettle_base64, '|' as i32 as uint8_t);
            current_block_16 = 11657864150791474432;
        }
        _ => {
            sexp_get_token_string(input, string);
            current_block_16 = 6009453772311597924;
        }
    }
    match current_block_16 {
        11657864150791474432 => {
            loop {
                sexp_get_char(input);
                match (*input).ctype as u32 {
                    0 => {
                        sexp_push_char(input, string);
                    }
                    1 => {
                        die(
                            b"Unexpected end of file in coded string.\n\0" as *const u8
                                as *const i8,
                        );
                    }
                    2 => {
                        sexp_input_end_coding(input);
                        sexp_get_char(input);
                        return;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn sexp_get_string_length(
    mut input: *mut sexp_input,
    mut mode: sexp_mode,
    mut string: *mut nettle_buffer,
) {
    let mut length: u32 = 0;
    nettle_buffer_reset(string);
    (*input).token = sexp_token::SEXP_STRING;
    length = ((*input).c as i32 - '0' as i32) as u32;
    if length == 0 {
        sexp_next_char(input);
    } else {
        if length < 10 as i32 as u32 {} else {
            __assert_fail(
                b"length < 10\0" as *const u8 as *const i8,
                b"input.c\0" as *const u8 as *const i8,
                275 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 89],
                    &[i8; 89],
                >(
                    b"void sexp_get_string_length(struct sexp_input *, enum sexp_mode, struct nettle_buffer *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4246: {
            if length < 10 as i32 as u32 {} else {
                __assert_fail(
                    b"length < 10\0" as *const u8 as *const i8,
                    b"input.c\0" as *const u8 as *const i8,
                    275 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 89],
                        &[i8; 89],
                    >(
                        b"void sexp_get_string_length(struct sexp_input *, enum sexp_mode, struct nettle_buffer *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        loop {
            sexp_next_char(input);
            if ((*input).c as i32) < '0' as i32 || (*input).c as i32 > '9' as i32 {
                break;
            }
            length = length
                .wrapping_mul(10 as i32 as u32)
                .wrapping_add((*input).c as u32)
                .wrapping_sub('0' as i32 as u32);
        }
    }
    if (*input).c as i32 == ':' as i32 {
        while length != 0 {
            sexp_next_char(input);
            sexp_push_char(input, string);
            length = length.wrapping_sub(1);
            length;
        }
    } else if mode as u32 != sexp_mode::SEXP_ADVANCED as i32 as u32 {
        die(
            b"Encountered advanced string in canonical mode.\n\0" as *const u8
                as *const i8,
        );
    } else {
        let mut current_block_31: u64;
        match (*input).c as i32 {
            34 => {
                while length != 0 {
                    if sexp_get_quoted_char(input) != 0 {
                        sexp_push_char(input, string);
                    } else {
                        die(b"Unexpected end of string.\n\0" as *const u8 as *const i8);
                    }
                    length = length.wrapping_sub(1);
                    length;
                }
                if sexp_get_quoted_char(input) != 0 {
                    die(
                        b"Quoted string longer than expected.\n\0" as *const u8
                            as *const i8,
                    );
                }
                current_block_31 = 5494826135382683477;
            }
            35 => {
                sexp_input_start_coding(input, &nettle_base16, '#' as i32 as uint8_t);
                current_block_31 = 5414350703348826794;
            }
            124 => {
                sexp_input_start_coding(input, &nettle_base64, '|' as i32 as uint8_t);
                current_block_31 = 5414350703348826794;
            }
            _ => {
                die(b"Invalid string.\n\0" as *const u8 as *const i8);
            }
        }
        match current_block_31 {
            5414350703348826794 => {
                while length != 0 {
                    sexp_next_char(input);
                    sexp_push_char(input, string);
                    length = length.wrapping_sub(1);
                    length;
                }
                sexp_get_char(input);
                if (*input).ctype as u32 != sexp_char_type::SEXP_END_CHAR as i32 as u32 {
                    die(b"Coded string too long.\n\0" as *const u8 as *const i8);
                }
                sexp_input_end_coding(input);
            }
            _ => {}
        }
    }
    sexp_get_char(input);
}
unsafe extern "C" fn sexp_get_comment(
    mut input: *mut sexp_input,
    mut string: *mut nettle_buffer,
) {
    nettle_buffer_reset(string);
    if (*input).ctype as u32 == sexp_char_type::SEXP_NORMAL_CHAR as i32 as u32 {} else {
        __assert_fail(
            b"input->ctype == sexp_char_type::SEXP_NORMAL_CHAR\0" as *const u8
                as *const i8,
            b"input.c\0" as *const u8 as *const i8,
            349 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[i8; 67],
            >(b"void sexp_get_comment(struct sexp_input *, struct nettle_buffer *)\0"))
                .as_ptr(),
        );
    }
    'c_3804: {
        if (*input).ctype as u32 == sexp_char_type::SEXP_NORMAL_CHAR as i32 as u32
        {} else {
            __assert_fail(
                b"input->ctype == sexp_char_type::SEXP_NORMAL_CHAR\0" as *const u8
                    as *const i8,
                b"input.c\0" as *const u8 as *const i8,
                349 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 67],
                    &[i8; 67],
                >(
                    b"void sexp_get_comment(struct sexp_input *, struct nettle_buffer *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*input).c as i32 == ';' as i32 {} else {
        __assert_fail(
            b"input->c == ';'\0" as *const u8 as *const i8,
            b"input.c\0" as *const u8 as *const i8,
            350 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[i8; 67],
            >(b"void sexp_get_comment(struct sexp_input *, struct nettle_buffer *)\0"))
                .as_ptr(),
        );
    }
    'c_3761: {
        if (*input).c as i32 == ';' as i32 {} else {
            __assert_fail(
                b"input->c == ';'\0" as *const u8 as *const i8,
                b"input.c\0" as *const u8 as *const i8,
                350 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 67],
                    &[i8; 67],
                >(
                    b"void sexp_get_comment(struct sexp_input *, struct nettle_buffer *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    loop {
        sexp_push_char(input, string);
        sexp_get_raw_char(input);
        if !((*input).ctype as u32 == sexp_char_type::SEXP_NORMAL_CHAR as i32 as u32
            && (*input).c as i32 != '\n' as i32)
        {
            break;
        }
    }
    (*input).token = sexp_token::SEXP_COMMENT;
}
#[no_mangle]
pub unsafe extern "C" fn sexp_get_token(
    mut input: *mut sexp_input,
    mut mode: sexp_mode,
    mut string: *mut nettle_buffer,
) {
    loop {
        match (*input).ctype as u32 {
            1 => {
                (*input).token = sexp_token::SEXP_EOF;
                return;
            }
            2 => {
                (*input).token = sexp_token::SEXP_CODING_END;
                sexp_input_end_coding(input);
                sexp_get_char(input);
                return;
            }
            0 => {
                match (*input).c as i32 {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        sexp_get_string_length(input, mode, string);
                        return;
                    }
                    40 => {
                        (*input).token = sexp_token::SEXP_LIST_START;
                        sexp_get_char(input);
                        return;
                    }
                    41 => {
                        (*input).token = sexp_token::SEXP_LIST_END;
                        sexp_get_char(input);
                        return;
                    }
                    91 => {
                        (*input).token = sexp_token::SEXP_DISPLAY_START;
                        sexp_get_char(input);
                        return;
                    }
                    93 => {
                        (*input).token = sexp_token::SEXP_DISPLAY_END;
                        sexp_get_char(input);
                        return;
                    }
                    123 => {
                        if mode as u32 == sexp_mode::SEXP_CANONICAL as i32 as u32 {
                            die(
                                b"Unexpected transport data in canonical mode.\n\0"
                                    as *const u8 as *const i8,
                            );
                        }
                        sexp_input_start_coding(
                            input,
                            &nettle_base64,
                            '}' as i32 as uint8_t,
                        );
                        sexp_get_char(input);
                        (*input).token = sexp_token::SEXP_TRANSPORT_START;
                        return;
                    }
                    32 | 9 | 10 | 13 => {
                        if mode as u32 == sexp_mode::SEXP_CANONICAL as i32 as u32 {
                            die(
                                b"Whitespace encountered in canonical mode.\n\0"
                                    as *const u8 as *const i8,
                            );
                        }
                        sexp_get_char(input);
                    }
                    59 => {
                        if mode as u32 == sexp_mode::SEXP_CANONICAL as i32 as u32 {
                            die(
                                b"Comment encountered in canonical mode.\n\0" as *const u8
                                    as *const i8,
                            );
                        }
                        sexp_get_comment(input, string);
                        return;
                    }
                    _ => {
                        if mode as u32 != sexp_mode::SEXP_ADVANCED as i32 as u32 {
                            die(
                                b"Encountered advanced string in canonical mode.\n\0"
                                    as *const u8 as *const i8,
                            );
                        }
                        sexp_get_string(input, string);
                        return;
                    }
                }
            }
            _ => {}
        }
    };
}