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
    fn __errno_location() -> *mut i32;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn strerror(_: i32) -> *mut i8;
    static sexp_token_chars: [i8; 128];
    fn die(format: *const i8, _: ...) -> !;
    static nettle_base64: nettle_armor;
    static nettle_base16: nettle_armor;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type uint8_t = __uint8_t;
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
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
pub struct nettle_hash {
    pub name: *const i8,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: Option<nettle_hash_init_func>,
    pub update: Option<nettle_hash_update_func>,
    pub digest: Option<nettle_hash_digest_func>,
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
pub struct sexp_output {
    pub f: *mut FILE,
    pub line_width: u32,
    pub coding: *const nettle_armor,
    pub coding_indent: u32,
    pub prefer_hex: i32,
    pub hash: *const nettle_hash,
    pub ctx: *mut libc::c_void,
    pub base64: base64_decode_ctx,
    pub pos: u32,
    pub soft_newline: i32,
}
#[no_mangle]
pub unsafe extern "C" fn sexp_output_init(
    mut output: *mut sexp_output,
    mut f: *mut FILE,
    mut width: u32,
    mut prefer_hex: i32,
) {
    (*output).f = f;
    (*output).line_width = width;
    (*output).coding = 0 as *const nettle_armor;
    (*output).prefer_hex = prefer_hex;
    (*output).hash = 0 as *const nettle_hash;
    (*output).ctx = 0 as *mut libc::c_void;
    (*output).pos = 0 as i32 as u32;
    (*output).soft_newline = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn sexp_output_hash_init(
    mut output: *mut sexp_output,
    mut hash: *const nettle_hash,
    mut ctx: *mut libc::c_void,
) {
    (*output).hash = hash;
    (*output).ctx = ctx;
    ((*hash).init).expect("non-null function pointer")(ctx);
}
unsafe extern "C" fn sexp_put_raw_char(mut output: *mut sexp_output, mut c: uint8_t) {
    if _IO_putc(c as i32, (*output).f) < 0 as i32 {
        die(
            b"Write failed: %s\n\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
    }
    (*output).pos = ((*output).pos).wrapping_add(1);
    (*output).pos;
    (*output).soft_newline = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn sexp_put_newline(
    mut output: *mut sexp_output,
    mut indent: u32,
) {
    if (*output).soft_newline != 0 {
        (*output).soft_newline = 0 as i32;
    } else {
        let mut i: u32 = 0;
        sexp_put_raw_char(output, '\n' as i32 as uint8_t);
        (*output).pos = 0 as i32 as u32;
        i = 0 as i32 as u32;
        while i < indent {
            sexp_put_raw_char(output, ' ' as i32 as uint8_t);
            i = i.wrapping_add(1);
            i;
        }
        (*output).pos = indent;
    };
}
#[no_mangle]
pub unsafe extern "C" fn sexp_put_soft_newline(
    mut output: *mut sexp_output,
    mut indent: u32,
) {
    sexp_put_newline(output, indent);
    (*output).soft_newline = 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn sexp_put_char(mut output: *mut sexp_output, mut c: uint8_t) {
    if !((*output).coding).is_null() {
        let mut encoded: [i8; 2] = [0; 2];
        let mut done: u32 = 0;
        let mut i: u32 = 0;
        done = ((*(*output).coding).encode_update)
            .expect(
                "non-null function pointer",
            )(
            &mut (*output).base64 as *mut base64_decode_ctx as *mut libc::c_void,
            encoded.as_mut_ptr(),
            1 as i32 as size_t,
            &mut c,
        ) as u32;
        if done as u64 <= ::core::mem::size_of::<[i8; 2]>() as u64 {} else {
            __assert_fail(
                b"done <= sizeof(encoded)\0" as *const u8 as *const i8,
                b"output.c\0" as *const u8 as *const i8,
                124 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[i8; 50],
                >(b"void sexp_put_char(struct sexp_output *, uint8_t)\0"))
                    .as_ptr(),
            );
        }
        'c_2954: {
            if done as u64 <= ::core::mem::size_of::<[i8; 2]>() as u64 {} else {
                __assert_fail(
                    b"done <= sizeof(encoded)\0" as *const u8 as *const i8,
                    b"output.c\0" as *const u8 as *const i8,
                    124 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[i8; 50],
                    >(b"void sexp_put_char(struct sexp_output *, uint8_t)\0"))
                        .as_ptr(),
                );
            }
        };
        i = 0 as i32 as u32;
        while i < done {
            if (*output).line_width != 0 && (*output).pos >= (*output).line_width
                && (*output).pos
                    >= ((*output).coding_indent).wrapping_add(10 as i32 as u32)
            {
                sexp_put_newline(output, (*output).coding_indent);
            }
            sexp_put_raw_char(output, encoded[i as usize] as uint8_t);
            i = i.wrapping_add(1);
            i;
        }
    } else if !((*output).hash).is_null() {
        ((*(*output).hash).update)
            .expect(
                "non-null function pointer",
            )((*output).ctx, 1 as i32 as size_t, &mut c);
    } else {
        sexp_put_raw_char(output, c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn sexp_put_data(
    mut output: *mut sexp_output,
    mut length: u32,
    mut data: *const uint8_t,
) {
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while i < length {
        sexp_put_char(output, *data.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn sexp_put_length(mut output: *mut sexp_output, mut length: u32) {
    let mut digit: u32 = 1 as i32 as u32;
    loop {
        let mut next: u32 = digit.wrapping_mul(10 as i32 as u32);
        if next > length {
            break;
        }
        digit = next;
    }
    while digit != 0 {
        sexp_put_char(
            output,
            ('0' as i32 as u32).wrapping_add(length.wrapping_div(digit)) as uint8_t,
        );
        length = length.wrapping_rem(digit);
        digit = digit.wrapping_div(10 as i32 as u32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sexp_put_code_start(
    mut output: *mut sexp_output,
    mut coding: *const nettle_armor,
) {
    if ((*output).coding).is_null() {} else {
        __assert_fail(
            b"!output->coding\0" as *const u8 as *const i8,
            b"output.c\0" as *const u8 as *const i8,
            174 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[i8; 76],
            >(
                b"void sexp_put_code_start(struct sexp_output *, const struct nettle_armor *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3094: {
        if ((*output).coding).is_null() {} else {
            __assert_fail(
                b"!output->coding\0" as *const u8 as *const i8,
                b"output.c\0" as *const u8 as *const i8,
                174 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[i8; 76],
                >(
                    b"void sexp_put_code_start(struct sexp_output *, const struct nettle_armor *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*output).coding_indent = (*output).pos;
    (*output).coding = coding;
    ((*(*output).coding).encode_init)
        .expect(
            "non-null function pointer",
        )(&mut (*output).base64 as *mut base64_decode_ctx as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sexp_put_code_end(mut output: *mut sexp_output) {
    let mut encoded: [i8; 3] = [0; 3];
    let mut done: u32 = 0;
    if !((*output).coding).is_null() {} else {
        __assert_fail(
            b"output->coding\0" as *const u8 as *const i8,
            b"output.c\0" as *const u8 as *const i8,
            189 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[i8; 45],
            >(b"void sexp_put_code_end(struct sexp_output *)\0"))
                .as_ptr(),
        );
    }
    'c_3227: {
        if !((*output).coding).is_null() {} else {
            __assert_fail(
                b"output->coding\0" as *const u8 as *const i8,
                b"output.c\0" as *const u8 as *const i8,
                189 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[i8; 45],
                >(b"void sexp_put_code_end(struct sexp_output *)\0"))
                    .as_ptr(),
            );
        }
    };
    done = ((*(*output).coding).encode_final)
        .expect(
            "non-null function pointer",
        )(
        &mut (*output).base64 as *mut base64_decode_ctx as *mut libc::c_void,
        encoded.as_mut_ptr(),
    ) as u32;
    if done as u64 <= ::core::mem::size_of::<[i8; 3]>() as u64 {} else {
        __assert_fail(
            b"done <= sizeof(encoded)\0" as *const u8 as *const i8,
            b"output.c\0" as *const u8 as *const i8,
            193 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[i8; 45],
            >(b"void sexp_put_code_end(struct sexp_output *)\0"))
                .as_ptr(),
        );
    }
    'c_3167: {
        if done as u64 <= ::core::mem::size_of::<[i8; 3]>() as u64 {} else {
            __assert_fail(
                b"done <= sizeof(encoded)\0" as *const u8 as *const i8,
                b"output.c\0" as *const u8 as *const i8,
                193 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[i8; 45],
                >(b"void sexp_put_code_end(struct sexp_output *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*output).coding = 0 as *const nettle_armor;
    sexp_put_data(output, done, encoded.as_mut_ptr() as *const uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn sexp_put_string(
    mut output: *mut sexp_output,
    mut mode: sexp_mode,
    mut string: *mut nettle_buffer,
) {
    if (*string).size == 0 {
        sexp_put_data(
            output,
            2 as i32 as u32,
            (if mode as u32 == sexp_mode::SEXP_ADVANCED as i32 as u32 {
                b"\"\"\0" as *const u8 as *const i8
            } else {
                b"0:\0" as *const u8 as *const i8
            }) as *const uint8_t,
        );
    } else if mode as u32 == sexp_mode::SEXP_ADVANCED as i32 as u32 {
        let mut i: u32 = 0;
        let mut token: i32 = ((*((*string).contents).offset(0 as i32 as isize) as i32)
            < '0' as i32
            || *((*string).contents).offset(0 as i32 as isize) as i32 > '9' as i32)
            as i32;
        let mut quote_friendly: i32 = 1 as i32;
        static mut escape_names: [i8; 32] = [
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            'b' as i32 as i8,
            't' as i32 as i8,
            'n' as i32 as i8,
            0 as i32 as i8,
            'f' as i32 as i8,
            'r' as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
        ];
        i = 0 as i32 as u32;
        while (i as u64) < (*string).size {
            let mut c: uint8_t = *((*string).contents).offset(i as isize);
            if token
                & !((c as i32) < 0x80 as i32 && sexp_token_chars[c as usize] as i32 != 0)
                    as i32 != 0
            {
                token = 0 as i32;
            }
            if quote_friendly != 0 {
                if c as i32 >= 0x7f as i32 {
                    quote_friendly = 0 as i32;
                } else if (c as i32) < 0x20 as i32 && escape_names[c as usize] == 0 {
                    quote_friendly = 0 as i32;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        if token != 0 {
            sexp_put_data(output, (*string).size as u32, (*string).contents);
        } else if quote_friendly != 0 {
            sexp_put_char(output, '"' as i32 as uint8_t);
            i = 0 as i32 as u32;
            while (i as u64) < (*string).size {
                let mut escape: i32 = 0 as i32;
                let mut c_0: uint8_t = *((*string).contents).offset(i as isize);
                if (c_0 as i32) < 0x7f as i32 {} else {
                    __assert_fail(
                        b"c < 0x7f\0" as *const u8 as *const i8,
                        b"output.c\0" as *const u8 as *const i8,
                        248 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 83],
                            &[i8; 83],
                        >(
                            b"void sexp_put_string(struct sexp_output *, enum sexp_mode, struct nettle_buffer *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_3651: {
                    if (c_0 as i32) < 0x7f as i32 {} else {
                        __assert_fail(
                            b"c < 0x7f\0" as *const u8 as *const i8,
                            b"output.c\0" as *const u8 as *const i8,
                            248 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 83],
                                &[i8; 83],
                            >(
                                b"void sexp_put_string(struct sexp_output *, enum sexp_mode, struct nettle_buffer *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if c_0 as i32 == '\\' as i32 || c_0 as i32 == '"' as i32 {
                    escape = 1 as i32;
                } else if (c_0 as i32) < 0x20 as i32 {
                    escape = 1 as i32;
                    c_0 = escape_names[c_0 as usize] as uint8_t;
                    if c_0 != 0 {} else {
                        __assert_fail(
                            b"c\0" as *const u8 as *const i8,
                            b"output.c\0" as *const u8 as *const i8,
                            256 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 83],
                                &[i8; 83],
                            >(
                                b"void sexp_put_string(struct sexp_output *, enum sexp_mode, struct nettle_buffer *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_3518: {
                        if c_0 != 0 {} else {
                            __assert_fail(
                                b"c\0" as *const u8 as *const i8,
                                b"output.c\0" as *const u8 as *const i8,
                                256 as i32 as u32,
                                (*::core::mem::transmute::<
                                    &[u8; 83],
                                    &[i8; 83],
                                >(
                                    b"void sexp_put_string(struct sexp_output *, enum sexp_mode, struct nettle_buffer *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                }
                if escape != 0 {
                    sexp_put_char(output, '\\' as i32 as uint8_t);
                }
                sexp_put_char(output, c_0);
                i = i.wrapping_add(1);
                i;
            }
            sexp_put_char(output, '"' as i32 as uint8_t);
        } else {
            let mut delimiter: uint8_t = 0;
            let mut coding: *const nettle_armor = 0 as *const nettle_armor;
            if (*output).prefer_hex != 0 {
                delimiter = '#' as i32 as uint8_t;
                coding = &nettle_base16;
            } else {
                delimiter = '|' as i32 as uint8_t;
                coding = &nettle_base64;
            }
            sexp_put_char(output, delimiter);
            sexp_put_code_start(output, coding);
            sexp_put_data(output, (*string).size as u32, (*string).contents);
            sexp_put_code_end(output);
            sexp_put_char(output, delimiter);
        }
    } else {
        sexp_put_length(output, (*string).size as u32);
        sexp_put_char(output, ':' as i32 as uint8_t);
        sexp_put_data(output, (*string).size as u32, (*string).contents);
    };
}
#[no_mangle]
pub unsafe extern "C" fn sexp_put_digest(mut output: *mut sexp_output) {
    let mut digest: *mut uint8_t = 0 as *mut uint8_t;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::core::mem::size_of::<uint8_t>() as u64)
            .wrapping_mul((*(*output).hash).digest_size as u64) as usize,
    );
    digest = fresh0.as_mut_ptr() as *mut uint8_t;
    if !((*output).hash).is_null() {} else {
        __assert_fail(
            b"output->hash\0" as *const u8 as *const i8,
            b"output.c\0" as *const u8 as *const i8,
            304 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[i8; 43],
            >(b"void sexp_put_digest(struct sexp_output *)\0"))
                .as_ptr(),
        );
    }
    'c_3922: {
        if !((*output).hash).is_null() {} else {
            __assert_fail(
                b"output->hash\0" as *const u8 as *const i8,
                b"output.c\0" as *const u8 as *const i8,
                304 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[i8; 43],
                >(b"void sexp_put_digest(struct sexp_output *)\0"))
                    .as_ptr(),
            );
        }
    };
    ((*(*output).hash).digest)
        .expect(
            "non-null function pointer",
        )((*output).ctx, (*(*output).hash).digest_size as size_t, digest);
    sexp_put_code_start(output, &nettle_base16);
    sexp_put_data(output, (*(*output).hash).digest_size, digest);
    sexp_put_code_end(output);
}