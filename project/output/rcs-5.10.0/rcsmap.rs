use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum tokens {
    DELIM,
    DIGIT,
    IDCHAR,
    NEWLN,
    LETTER,
    Letter,
    PERIOD,
    SBEGIN,
    SPACE,
    UNKN,
    COLON,
    ID,
    NUM,
    SEMI,
    STRING,
}
impl tokens {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            tokens::DELIM => 0,
            tokens::DIGIT => 1,
            tokens::IDCHAR => 2,
            tokens::NEWLN => 3,
            tokens::LETTER => 4,
            tokens::Letter => 5,
            tokens::PERIOD => 6,
            tokens::SBEGIN => 7,
            tokens::SPACE => 8,
            tokens::UNKN => 9,
            tokens::COLON => 10,
            tokens::ID => 11,
            tokens::NUM => 12,
            tokens::SEMI => 13,
            tokens::STRING => 14,
        }
    }
    fn from_libc_c_uint(value: u32) -> tokens {
        match value {
            0 => tokens::DELIM,
            1 => tokens::DIGIT,
            2 => tokens::IDCHAR,
            3 => tokens::NEWLN,
            4 => tokens::LETTER,
            5 => tokens::Letter,
            6 => tokens::PERIOD,
            7 => tokens::SBEGIN,
            8 => tokens::SPACE,
            9 => tokens::UNKN,
            10 => tokens::COLON,
            11 => tokens::ID,
            12 => tokens::NUM,
            13 => tokens::SEMI,
            14 => tokens::STRING,
            _ => panic!("Invalid value for tokens: {}", value),
        }
    }
}
impl AddAssign<u32> for tokens {
    fn add_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for tokens {
    fn sub_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for tokens {
    fn mul_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for tokens {
    fn div_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for tokens {
    fn rem_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for tokens {
    type Output = tokens;
    fn add(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for tokens {
    type Output = tokens;
    fn sub(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for tokens {
    type Output = tokens;
    fn mul(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for tokens {
    type Output = tokens;
    fn div(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for tokens {
    type Output = tokens;
    fn rem(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[no_mangle]
pub static mut ctab: [tokens; 256] = [
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::SPACE,
    tokens::SPACE,
    tokens::NEWLN,
    tokens::SPACE,
    tokens::SPACE,
    tokens::SPACE,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::SPACE,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::DELIM,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::DELIM,
    tokens::IDCHAR,
    tokens::PERIOD,
    tokens::IDCHAR,
    tokens::DIGIT,
    tokens::DIGIT,
    tokens::DIGIT,
    tokens::DIGIT,
    tokens::DIGIT,
    tokens::DIGIT,
    tokens::DIGIT,
    tokens::DIGIT,
    tokens::DIGIT,
    tokens::DIGIT,
    tokens::COLON,
    tokens::SEMI,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::SBEGIN,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::UNKN,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::IDCHAR,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::IDCHAR,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::LETTER,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::IDCHAR,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
    tokens::Letter,
];
unsafe extern "C" fn checkidentifier(
    mut id: *const i8,
    mut delimiter: i32,
    mut dotok: bool,
) -> *const i8 {
    let mut temp: *const i8 = 0 as *const i8;
    let mut c: i8 = 0;
    let mut delim: i8 = delimiter as i8;
    let mut isid: bool = 0 as i32 != 0;
    temp = id;
    loop {
        c = *id;
        match ctab[c as u8 as usize] as u32 {
            1 | 2 | 4 | 5 => {
                isid = 1 as i32 != 0;
            }
            6 => {
                if !dotok {
                    break;
                }
            }
            _ => {
                break;
            }
        }
        id = id.offset(1);
        id;
    }
    if !isid
        || c as i32 != 0
            && (delim == 0
                || c as i32 != delim as i32 && c as i32 != ' ' as i32
                    && c as i32 != '\t' as i32 && c as i32 != '\n' as i32)
    {
        loop {
            c = *id;
            if !(c as i32 != 0 && c as i32 != ' ' as i32 && c as i32 != '\t' as i32
                && c as i32 != '\n' as i32 && c as i32 != delim as i32)
            {
                break;
            }
            id = id.offset(1);
            id;
        }
        generic_fatal(
            0 as *const i8,
            b"invalid %s `%.*s'\0" as *const u8 as *const i8,
            if dotok as i32 != 0 {
                b"identifier\0" as *const u8 as *const i8
            } else {
                b"symbol\0" as *const u8 as *const i8
            },
            id.offset_from(temp) as i64 as i32,
            temp,
        );
    }
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn checkid(mut id: *const i8, mut delimiter: i32) -> *const i8 {
    return checkidentifier(id, delimiter, 1 as i32 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn checksym(mut sym: *const i8, mut delimiter: i32) -> *const i8 {
    return checkidentifier(sym, delimiter, 0 as i32 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn checksid(mut id: *const i8) {
    checkid(id, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn checkssym(mut sym: *const i8) {
    checksym(sym, 0 as i32);
}