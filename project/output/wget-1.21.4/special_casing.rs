#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
}
pub type size_t = u64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    SCC_AFTER_I = 5,
    SCC_BEFORE_DOT = 4,
    SCC_MORE_ABOVE = 3,
    SCC_AFTER_SOFT_DOTTED = 2,
    SCC_FINAL_SIGMA = 1,
    SCC_ALWAYS = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::SCC_AFTER_I => 5,
            C2RustUnnamed::SCC_BEFORE_DOT => 4,
            C2RustUnnamed::SCC_MORE_ABOVE => 3,
            C2RustUnnamed::SCC_AFTER_SOFT_DOTTED => 2,
            C2RustUnnamed::SCC_FINAL_SIGMA => 1,
            C2RustUnnamed::SCC_ALWAYS => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            5 => C2RustUnnamed::SCC_AFTER_I,
            4 => C2RustUnnamed::SCC_BEFORE_DOT,
            3 => C2RustUnnamed::SCC_MORE_ABOVE,
            2 => C2RustUnnamed::SCC_AFTER_SOFT_DOTTED,
            1 => C2RustUnnamed::SCC_FINAL_SIGMA,
            0 => C2RustUnnamed::SCC_ALWAYS,
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct special_casing_rule {
    pub code: [i8; 3],
    #[bitfield(name = "has_next", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "context", ty = "libc::c_int", bits = "1..=7")]
    pub has_next_context: [u8; 1],
    pub language: [i8; 2],
    pub upper: [libc::c_ushort; 3],
    pub lower: [libc::c_ushort; 3],
    pub title: [libc::c_ushort; 3],
    pub casefold: [libc::c_ushort; 3],
}
#[inline]
unsafe extern "C" fn gl_unicase_special_hash(
    mut str: *const i8,
    mut len: size_t,
) -> u32 {
    static mut asso_values: [u8; 257] = [
        2 as i32 as u8,
        0 as i32 as u8,
        4 as i32 as u8,
        5 as i32 as u8,
        37 as i32 as u8,
        12 as i32 as u8,
        121 as i32 as u8,
        4 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        120 as i32 as u8,
        119 as i32 as u8,
        118 as i32 as u8,
        117 as i32 as u8,
        116 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        5 as i32 as u8,
        3 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        115 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        114 as i32 as u8,
        122 as i32 as u8,
        6 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        1 as i32 as u8,
        111 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        109 as i32 as u8,
        122 as i32 as u8,
        108 as i32 as u8,
        122 as i32 as u8,
        107 as i32 as u8,
        122 as i32 as u8,
        106 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        33 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        105 as i32 as u8,
        104 as i32 as u8,
        103 as i32 as u8,
        102 as i32 as u8,
        101 as i32 as u8,
        100 as i32 as u8,
        99 as i32 as u8,
        31 as i32 as u8,
        98 as i32 as u8,
        97 as i32 as u8,
        96 as i32 as u8,
        95 as i32 as u8,
        94 as i32 as u8,
        93 as i32 as u8,
        92 as i32 as u8,
        91 as i32 as u8,
        28 as i32 as u8,
        90 as i32 as u8,
        89 as i32 as u8,
        88 as i32 as u8,
        87 as i32 as u8,
        86 as i32 as u8,
        27 as i32 as u8,
        24 as i32 as u8,
        23 as i32 as u8,
        20 as i32 as u8,
        19 as i32 as u8,
        85 as i32 as u8,
        84 as i32 as u8,
        83 as i32 as u8,
        16 as i32 as u8,
        82 as i32 as u8,
        81 as i32 as u8,
        80 as i32 as u8,
        79 as i32 as u8,
        15 as i32 as u8,
        78 as i32 as u8,
        77 as i32 as u8,
        76 as i32 as u8,
        75 as i32 as u8,
        74 as i32 as u8,
        73 as i32 as u8,
        72 as i32 as u8,
        71 as i32 as u8,
        70 as i32 as u8,
        69 as i32 as u8,
        68 as i32 as u8,
        67 as i32 as u8,
        64 as i32 as u8,
        122 as i32 as u8,
        65 as i32 as u8,
        64 as i32 as u8,
        63 as i32 as u8,
        122 as i32 as u8,
        62 as i32 as u8,
        61 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        60 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        59 as i32 as u8,
        58 as i32 as u8,
        57 as i32 as u8,
        122 as i32 as u8,
        56 as i32 as u8,
        55 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        14 as i32 as u8,
        55 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        53 as i32 as u8,
        52 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        51 as i32 as u8,
        50 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        50 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        48 as i32 as u8,
        47 as i32 as u8,
        46 as i32 as u8,
        122 as i32 as u8,
        45 as i32 as u8,
        44 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        46 as i32 as u8,
        122 as i32 as u8,
        42 as i32 as u8,
        41 as i32 as u8,
        39 as i32 as u8,
        122 as i32 as u8,
        38 as i32 as u8,
        35 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        0 as i32 as u8,
        33 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
        122 as i32 as u8,
    ];
    return (asso_values[(*str.offset(2 as i32 as isize) as u8 as i32 + 1 as i32)
        as usize] as i32
        + asso_values[*str.offset(1 as i32 as isize) as u8 as usize] as i32
        + asso_values[*str.offset(0 as i32 as isize) as u8 as usize] as i32) as u32;
}
static mut wordlist: [special_casing_rule; 122] = [special_casing_rule {
    code: [0; 3],
    has_next_context: [0; 1],
    language: [0; 2],
    upper: [0; 3],
    lower: [0; 3],
    title: [0; 3],
    casefold: [0; 3],
}; 122];
#[no_mangle]
pub unsafe extern "C" fn gl_unicase_special_lookup(
    mut str: *const i8,
    mut len: size_t,
) -> *const special_casing_rule {
    static mut lengthtable: [u8; 122] = [
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
    ];
    if len <= 3 as i32 as u64 && len >= 3 as i32 as u64 {
        let mut key: u32 = gl_unicase_special_hash(str, len);
        if key <= 121 as i32 as u32 {
            if len == lengthtable[key as usize] as u64 {
                let mut s: *const i8 = (wordlist[key as usize].code).as_ptr();
                if *str as i32 == *s as i32
                    && memcmp(
                        str.offset(1 as i32 as isize) as *const libc::c_void,
                        s.offset(1 as i32 as isize) as *const libc::c_void,
                        len.wrapping_sub(1 as i32 as u64),
                    ) == 0
                {
                    return &*wordlist.as_ptr().offset(key as isize)
                        as *const special_casing_rule;
                }
            }
        }
    }
    return 0 as *const special_casing_rule;
}
unsafe extern "C" fn run_static_initializers() {
    wordlist = [
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\xFB\x01\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x46 as i32 as libc::c_ushort,
                    0x49 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xfb01 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x46 as i32 as libc::c_ushort,
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x66 as i32 as libc::c_ushort,
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x01I\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x2bc as i32 as libc::c_ushort,
                    0x4e as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x149 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x2bc as i32 as libc::c_ushort,
                    0x4e as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x2bc as i32 as libc::c_ushort,
                    0x6e as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\xFB\0\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x46 as i32 as libc::c_ushort,
                    0x46 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xfb00 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x46 as i32 as libc::c_ushort,
                    0x66 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x66 as i32 as libc::c_ushort,
                    0x66 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\0I\0"),
                language: ['l' as i32 as i8, 't' as i32 as i8],
                upper: [
                    0x49 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x69 as i32 as libc::c_ushort,
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x49 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_MORE_ABOVE as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\xFB\x02\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x46 as i32 as libc::c_ushort,
                    0x4c as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xfb02 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x46 as i32 as libc::c_ushort,
                    0x6c as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x66 as i32 as libc::c_ushort,
                    0x6c as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\xFB\x03\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x46 as i32 as libc::c_ushort,
                    0x46 as i32 as libc::c_ushort,
                    0x49 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xfb03 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x46 as i32 as libc::c_ushort,
                    0x66 as i32 as libc::c_ushort,
                    0x69 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x66 as i32 as libc::c_ushort,
                    0x66 as i32 as libc::c_ushort,
                    0x69 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x010\0"),
                language: ['t' as i32 as i8, 'r' as i32 as i8],
                upper: [
                    0x130 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x130 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\0I\x01"),
                language: ['t' as i32 as i8, 'r' as i32 as i8],
                upper: [
                    0x49 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x131 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x49 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x131 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as i32 as u32);
            init.set_context(-(C2RustUnnamed::SCC_BEFORE_DOT as i32));
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\0I\x02"),
                language: ['a' as i32 as i8, 'z' as i32 as i8],
                upper: [
                    0x49 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x131 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x49 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x131 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as i32 as u32);
            init.set_context(-(C2RustUnnamed::SCC_BEFORE_DOT as i32));
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x03\x07\0"),
                language: ['l' as i32 as i8, 't' as i32 as i8],
                upper: [
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_AFTER_SOFT_DOTTED as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x010\x01"),
                language: ['a' as i32 as i8, 'z' as i32 as i8],
                upper: [
                    0x130 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x130 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x010\x02"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x130 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x69 as i32 as libc::c_ushort,
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x130 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x69 as i32 as libc::c_ushort,
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\xFB\x05\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x53 as i32 as libc::c_ushort,
                    0x54 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xfb05 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x53 as i32 as libc::c_ushort,
                    0x74 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x73 as i32 as libc::c_ushort,
                    0x74 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x03\x07\x01"),
                language: ['t' as i32 as i8, 'r' as i32 as i8],
                upper: [
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_AFTER_I as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x03\x07\x02"),
                language: ['a' as i32 as i8, 'z' as i32 as i8],
                upper: [
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_AFTER_I as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\0I\x04"),
                language: ['a' as i32 as i8, 'z' as i32 as i8],
                upper: [
                    0x49 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x49 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x131 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\0\xCC\0"),
                language: ['l' as i32 as i8, 't' as i32 as i8],
                upper: [
                    0xcc as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x69 as i32 as libc::c_ushort,
                    0x307 as i32 as libc::c_ushort,
                    0x300 as i32 as libc::c_ushort,
                ],
                title: [
                    0xcc as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0xec as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xCC\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x397 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fc3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fcc as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b7 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xA3\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f6b as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fab as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f63 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x9E\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f2e as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f96 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f9e as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f26 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x03\xA3\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x3c2 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_FINAL_SIGMA as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1E\x9E\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1e9e as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xdf as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1e9e as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x73 as i32 as libc::c_ushort,
                    0x73 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x9A\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f2a as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f92 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f9a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f22 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x99\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f29 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f91 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f99 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f21 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1E\x9A\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x41 as i32 as libc::c_ushort,
                    0x2be as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1e9a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x41 as i32 as libc::c_ushort,
                    0x2be as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x61 as i32 as libc::c_ushort,
                    0x2be as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1E\x99\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x59 as i32 as libc::c_ushort,
                    0x30a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1e99 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x59 as i32 as libc::c_ushort,
                    0x30a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x79 as i32 as libc::c_ushort,
                    0x30a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x98\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f28 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f90 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f98 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f20 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x97\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f2f as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f97 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f9f as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f27 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1E\x98\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x57 as i32 as libc::c_ushort,
                    0x30a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1e98 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x57 as i32 as libc::c_ushort,
                    0x30a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x77 as i32 as libc::c_ushort,
                    0x30a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1E\x97\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x54 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1e97 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x54 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x74 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x96\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f2e as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f96 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f9e as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f26 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x90\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f28 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f90 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f98 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f20 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1E\x96\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x48 as i32 as libc::c_ushort,
                    0x331 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1e96 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x48 as i32 as libc::c_ushort,
                    0x331 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x68 as i32 as libc::c_ushort,
                    0x331 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x03\x90\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x399 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x390 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x399 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b9 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x87\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f0f as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f87 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f8f as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f07 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\0i\0"),
                language: ['t' as i32 as i8, 'r' as i32 as i8],
                upper: [
                    0x130 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x130 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xFC\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a9 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1ff3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1ffc as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c9 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\xFB\x04\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x46 as i32 as libc::c_ushort,
                    0x46 as i32 as libc::c_ushort,
                    0x4c as i32 as libc::c_ushort,
                ],
                lower: [
                    0xfb04 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x46 as i32 as libc::c_ushort,
                    0x66 as i32 as libc::c_ushort,
                    0x6c as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x66 as i32 as libc::c_ushort,
                    0x66 as i32 as libc::c_ushort,
                    0x6c as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xF7\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a9 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1ff7 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a9 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0x345 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c9 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\0i\x01"),
                language: ['a' as i32 as i8, 'z' as i32 as i8],
                upper: [
                    0x130 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x130 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\0I\x03"),
                language: ['t' as i32 as i8, 'r' as i32 as i8],
                upper: [
                    0x49 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x69 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x49 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x131 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xF6\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a9 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1ff6 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a9 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c9 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xF4\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x38f as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1ff4 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x38f as i32 as libc::c_ushort,
                    0x345 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3ce as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x05\x87\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x535 as i32 as libc::c_ushort,
                    0x552 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x587 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x535 as i32 as libc::c_ushort,
                    0x582 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x565 as i32 as libc::c_ushort,
                    0x582 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xF3\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a9 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1ff3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1ffc as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c9 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xF2\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1ffa as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1ff2 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1ffa as i32 as libc::c_ushort,
                    0x345 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f7c as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x01\xF0\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x4a as i32 as libc::c_ushort,
                    0x30c as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x4a as i32 as libc::c_ushort,
                    0x30c as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x6a as i32 as libc::c_ushort,
                    0x30c as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xE7\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fe7 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xE6\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fe6 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xE4\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a1 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fe4 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a1 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c1 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xE3\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fe3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xE2\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x300 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fe2 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x300 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x300 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\0\xDF\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x53 as i32 as libc::c_ushort,
                    0x53 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xdf as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x53 as i32 as libc::c_ushort,
                    0x73 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x73 as i32 as libc::c_ushort,
                    0x73 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xD7\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x399 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fd7 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x399 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b9 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xD6\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x399 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fd6 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x399 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b9 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xD3\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x399 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fd3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x399 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b9 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xD2\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x399 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x300 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fd2 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x399 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x300 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b9 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x300 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\0\xCD\0"),
                language: ['l' as i32 as i8, 't' as i32 as i8],
                upper: [
                    0xcd as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x69 as i32 as libc::c_ushort,
                    0x307 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
                title: [
                    0xcd as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0xed as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xC7\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x397 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fc7 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x397 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0x345 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b7 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xC6\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x397 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fc6 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x397 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b7 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xC4\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x389 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fc4 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x389 as i32 as libc::c_ushort,
                    0x345 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3ae as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xC3\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x397 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fc3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fcc as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b7 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xC2\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1fca as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fc2 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fca as i32 as libc::c_ushort,
                    0x345 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f74 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xBC\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x391 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fb3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fbc as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b1 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xB7\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x391 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fb7 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x391 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0x345 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b1 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xB6\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x391 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fb6 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x391 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b1 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xB4\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x386 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fb4 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x386 as i32 as libc::c_ushort,
                    0x345 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3ac as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xB3\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x391 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fb3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fbc as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3b1 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xB2\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1fba as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fb2 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fba as i32 as libc::c_ushort,
                    0x345 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f70 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x03\xB0\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x3b0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as i32 as libc::c_ushort,
                    0x308 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xAF\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f6f as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa7 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1faf as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f67 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xAE\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f6e as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa6 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fae as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f66 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xAD\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f6d as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa5 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fad as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f65 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xAC\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f6c as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa4 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fac as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f64 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xAB\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f6b as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa3 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fab as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f63 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xAA\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f6a as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa2 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1faa as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f62 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xA9\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f69 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa1 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fa9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f61 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xA8\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f68 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fa8 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f60 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xA7\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f6f as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa7 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1faf as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f67 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xA6\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f6e as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa6 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fae as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f66 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xA5\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f6d as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa5 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fad as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f65 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xA4\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f6c as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa4 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fac as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f64 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xA2\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f6a as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa2 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1faa as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f62 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xA1\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f69 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa1 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fa9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f61 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\xA0\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f68 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1fa0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1fa8 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f60 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x9F\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f2f as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f97 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f9f as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f27 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x9D\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f2d as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f95 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f9d as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f25 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x9C\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f2c as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f94 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f9c as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f24 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x9B\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f2b as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f93 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f9b as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f23 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x95\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f2d as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f95 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f9d as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f25 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x94\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f2c as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f94 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f9c as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f24 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x93\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f2b as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f93 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f9b as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f23 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x92\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f2a as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f92 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f9a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f22 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x91\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f29 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f91 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f99 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f21 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x8F\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f0f as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f87 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f8f as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f07 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x8E\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f0e as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f86 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f8e as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f06 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x8D\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f0d as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f85 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f8d as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f05 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x8C\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f0c as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f84 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f8c as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f04 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x8B\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f0b as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f83 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f8b as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f03 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x8A\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f0a as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f82 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f8a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f02 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x89\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f09 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f81 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f89 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f01 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x88\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f08 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f80 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f88 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f00 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x86\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f0e as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f86 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f8e as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f06 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x85\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f0d as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f85 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f8d as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f05 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x84\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f0c as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f84 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f8c as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f04 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x83\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f0b as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f83 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f8b as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f03 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x82\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f0a as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f82 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f8a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f02 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x81\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f09 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f81 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f89 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f01 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1F\x80\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x1f08 as i32 as libc::c_ushort,
                    0x399 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f80 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x1f88 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x1f00 as i32 as libc::c_ushort,
                    0x3b9 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1FV\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f56 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0x342 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1FT\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f54 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0x301 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1FR\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0x300 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f52 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0x300 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0x300 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x1FP\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x1f50 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x3a5 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as i32 as libc::c_ushort,
                    0x313 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\0J\0"),
                language: ['l' as i32 as i8, 't' as i32 as i8],
                upper: [
                    0x4a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x6a as i32 as libc::c_ushort,
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x4a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x6a as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_MORE_ABOVE as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x01.\0"),
                language: ['l' as i32 as i8, 't' as i32 as i8],
                upper: [
                    0x12e as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x12f as i32 as libc::c_ushort,
                    0x307 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x12e as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x12f as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_MORE_ABOVE as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\x01(\0"),
                language: ['l' as i32 as i8, 't' as i32 as i8],
                upper: [
                    0x128 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0x69 as i32 as libc::c_ushort,
                    0x307 as i32 as libc::c_ushort,
                    0x303 as i32 as libc::c_ushort,
                ],
                title: [
                    0x128 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x129 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\xFB\x17\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x544 as i32 as libc::c_ushort,
                    0x53d as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xfb17 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x544 as i32 as libc::c_ushort,
                    0x56d as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x574 as i32 as libc::c_ushort,
                    0x56d as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\xFB\x16\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x54e as i32 as libc::c_ushort,
                    0x546 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xfb16 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x54e as i32 as libc::c_ushort,
                    0x576 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x57e as i32 as libc::c_ushort,
                    0x576 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\xFB\x15\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x544 as i32 as libc::c_ushort,
                    0x53b as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xfb15 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x544 as i32 as libc::c_ushort,
                    0x56b as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x574 as i32 as libc::c_ushort,
                    0x56b as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\xFB\x14\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x544 as i32 as libc::c_ushort,
                    0x535 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xfb14 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x544 as i32 as libc::c_ushort,
                    0x565 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x574 as i32 as libc::c_ushort,
                    0x565 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\xFB\x13\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x544 as i32 as libc::c_ushort,
                    0x546 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xfb13 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x544 as i32 as libc::c_ushort,
                    0x576 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x574 as i32 as libc::c_ushort,
                    0x576 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::core::mem::transmute::<&[u8; 3], &mut [i8; 3]>(b"\xFB\x06\0"),
                language: ['\0' as i32 as i8, '\0' as i32 as i8],
                upper: [
                    0x53 as i32 as libc::c_ushort,
                    0x54 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                lower: [
                    0xfb06 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                title: [
                    0x53 as i32 as libc::c_ushort,
                    0x74 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
                casefold: [
                    0x73 as i32 as libc::c_ushort,
                    0x74 as i32 as libc::c_ushort,
                    0 as i32 as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as i32 as u32);
            init.set_context(C2RustUnnamed::SCC_ALWAYS as i32);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];