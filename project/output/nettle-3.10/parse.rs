use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn abort() -> !;
    fn die(format: *const i8, _: ...) -> !;
    fn nettle_buffer_init(buffer: *mut nettle_buffer);
    fn nettle_buffer_clear(buffer: *mut nettle_buffer);
    fn sexp_get_token(
        input: *mut sexp_input,
        mode: sexp_mode,
        string: *mut nettle_buffer,
    );
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
pub struct nettle_buffer {
    pub contents: *mut uint8_t,
    pub alloc: size_t,
    pub realloc_ctx: *mut libc::c_void,
    pub realloc: Option<nettle_realloc_func>,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_compound_token {
    pub type_0: sexp_token,
    pub display: nettle_buffer,
    pub string: nettle_buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_parser {
    pub input: *mut sexp_input,
    pub mode: sexp_mode,
    pub level: u32,
    pub transport: u32,
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
pub type FILE = _IO_FILE;
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
#[no_mangle]
pub unsafe extern "C" fn sexp_compound_token_init(mut token: *mut sexp_compound_token) {
    (*token).type_0 = sexp_token::SEXP_STRING;
    nettle_buffer_init(&mut (*token).display);
    nettle_buffer_init(&mut (*token).string);
}
#[no_mangle]
pub unsafe extern "C" fn sexp_compound_token_clear(mut token: *mut sexp_compound_token) {
    nettle_buffer_clear(&mut (*token).display);
    nettle_buffer_clear(&mut (*token).string);
}
#[no_mangle]
pub unsafe extern "C" fn sexp_parse_init(
    mut parser: *mut sexp_parser,
    mut input: *mut sexp_input,
    mut mode: sexp_mode,
) {
    (*parser).input = input;
    (*parser).mode = mode;
    (*parser).level = 1 as i32 as u32;
    (*parser).transport = 0 as i32 as u32;
}
unsafe extern "C" fn sexp_check_token(
    mut parser: *mut sexp_parser,
    mut token: sexp_token,
    mut string: *mut nettle_buffer,
) {
    sexp_get_token(
        (*parser).input,
        sexp_mode::from_libc_c_uint(
            (if (*parser).transport != 0 {
                sexp_mode::SEXP_CANONICAL as i32 as u32
            } else {
                (*parser).mode as u32
            }) as u32,
        ),
        string,
    );
    if (*(*parser).input).token as u32 != token as u32 {
        die(b"Syntax error.\n\0" as *const u8 as *const i8);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sexp_parse(
    mut parser: *mut sexp_parser,
    mut token: *mut sexp_compound_token,
) {
    loop {
        sexp_get_token(
            (*parser).input,
            sexp_mode::from_libc_c_uint(
                (if (*parser).transport != 0 {
                    sexp_mode::SEXP_CANONICAL as i32 as u32
                } else {
                    (*parser).mode as u32
                }) as u32,
            ),
            &mut (*token).string,
        );
        match (*(*parser).input).token as u32 {
            4 => {
                if (*parser).level == (*parser).transport {
                    die(
                        b"Unmatched end of list in transport encoded data.\n\0"
                            as *const u8 as *const i8,
                    );
                }
                (*parser).level = ((*parser).level).wrapping_sub(1);
                (*parser).level;
                if (*parser).level == 0 {
                    die(b"Unmatched end of list.\n\0" as *const u8 as *const i8);
                }
                (*token).type_0 = sexp_token::SEXP_LIST_END;
            }
            5 => {
                if (*parser).level > 1 as i32 as u32 {
                    die(b"Unexpected end of file.\n\0" as *const u8 as *const i8);
                }
                (*token).type_0 = sexp_token::SEXP_EOF;
                return;
            }
            3 => {
                (*parser).level = ((*parser).level).wrapping_add(1);
                (*parser).level;
                (*token).type_0 = sexp_token::SEXP_LIST_START;
                return;
            }
            6 => {
                sexp_check_token(parser, sexp_token::SEXP_STRING, &mut (*token).display);
                sexp_check_token(
                    parser,
                    sexp_token::SEXP_DISPLAY_END,
                    &mut (*token).display,
                );
                sexp_check_token(parser, sexp_token::SEXP_STRING, &mut (*token).string);
                (*token).type_0 = sexp_token::SEXP_DISPLAY;
            }
            0 => {
                (*token).type_0 = sexp_token::SEXP_STRING;
            }
            2 => {
                (*token).type_0 = sexp_token::SEXP_COMMENT;
                return;
            }
            8 => {
                if (*parser).mode as u32 == sexp_mode::SEXP_CANONICAL as i32 as u32 {
                    die(
                        b"Base64 not allowed in canonical mode.\n\0" as *const u8
                            as *const i8,
                    );
                }
                (*parser).level = ((*parser).level).wrapping_add(1);
                (*parser).level;
                (*parser).transport = (*parser).level;
                continue;
            }
            9 => {
                die(
                    b"Unexpected end of transport encoding.\n\0" as *const u8
                        as *const i8,
                );
            }
            7 => {
                die(b"Unexpected end of display tag.\n\0" as *const u8 as *const i8);
            }
            1 => {
                abort();
            }
            _ => {
                continue;
            }
        }
        if (*parser).level == (*parser).transport {
            sexp_check_token(parser, sexp_token::SEXP_CODING_END, &mut (*token).string);
            if (*parser).transport != 0 {} else {
                __assert_fail(
                    b"parser->transport\0" as *const u8 as *const i8,
                    b"parse.c\0" as *const u8 as *const i8,
                    121 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 68],
                        &[i8; 68],
                    >(
                        b"void sexp_parse(struct sexp_parser *, struct sexp_compound_token *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_1683: {
                if (*parser).transport != 0 {} else {
                    __assert_fail(
                        b"parser->transport\0" as *const u8 as *const i8,
                        b"parse.c\0" as *const u8 as *const i8,
                        121 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 68],
                            &[i8; 68],
                        >(
                            b"void sexp_parse(struct sexp_parser *, struct sexp_compound_token *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if (*parser).level == (*parser).transport {} else {
                __assert_fail(
                    b"parser->level == parser->transport\0" as *const u8 as *const i8,
                    b"parse.c\0" as *const u8 as *const i8,
                    122 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 68],
                        &[i8; 68],
                    >(
                        b"void sexp_parse(struct sexp_parser *, struct sexp_compound_token *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_1632: {
                if (*parser).level == (*parser).transport {} else {
                    __assert_fail(
                        b"parser->level == parser->transport\0" as *const u8
                            as *const i8,
                        b"parse.c\0" as *const u8 as *const i8,
                        122 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 68],
                            &[i8; 68],
                        >(
                            b"void sexp_parse(struct sexp_parser *, struct sexp_compound_token *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            (*parser).level = ((*parser).level).wrapping_sub(1);
            (*parser).level;
            (*parser).transport = 0 as i32 as u32;
        }
        return;
    };
}