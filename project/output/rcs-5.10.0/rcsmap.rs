#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
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
impl tokens {
    fn to_libc_c_uint(self) -> libc::c_uint {
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
}

#[no_mangle]
pub static mut ctab: [tokens; 256] = [
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    SPACE,
    SPACE,
    NEWLN,
    SPACE,
    SPACE,
    SPACE,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    SPACE,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    DELIM,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    DELIM,
    IDCHAR,
    PERIOD,
    IDCHAR,
    DIGIT,
    DIGIT,
    DIGIT,
    DIGIT,
    DIGIT,
    DIGIT,
    DIGIT,
    DIGIT,
    DIGIT,
    DIGIT,
    COLON,
    SEMI,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    SBEGIN,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    UNKN,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    IDCHAR,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    IDCHAR,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    LETTER,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    IDCHAR,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
    Letter,
];
unsafe extern "C" fn checkidentifier(
    mut id: *const libc::c_char,
    mut delimiter: libc::c_int,
    mut dotok: bool,
) -> *const libc::c_char {
    let mut temp: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_char = 0;
    let mut delim: libc::c_char = delimiter as libc::c_char;
    let mut isid: bool = 0 as libc::c_int != 0;
    temp = id;
    loop {
        c = *id;
        match ctab[c as libc::c_uchar as usize] as libc::c_uint {
            1 | 2 | 4 | 5 => {
                isid = 1 as libc::c_int != 0;
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
        || c as libc::c_int != 0
            && (delim == 0
                || c as libc::c_int != delim as libc::c_int
                    && c as libc::c_int != ' ' as i32 && c as libc::c_int != '\t' as i32
                    && c as libc::c_int != '\n' as i32)
    {
        loop {
            c = *id;
            if !(c as libc::c_int != 0 && c as libc::c_int != ' ' as i32
                && c as libc::c_int != '\t' as i32 && c as libc::c_int != '\n' as i32
                && c as libc::c_int != delim as libc::c_int)
            {
                break;
            }
            id = id.offset(1);
            id;
        }
        generic_fatal(
            0 as *const libc::c_char,
            b"invalid %s `%.*s'\0" as *const u8 as *const libc::c_char,
            if dotok as libc::c_int != 0 {
                b"identifier\0" as *const u8 as *const libc::c_char
            } else {
                b"symbol\0" as *const u8 as *const libc::c_char
            },
            id.offset_from(temp) as libc::c_long as libc::c_int,
            temp,
        );
    }
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn checkid(
    mut id: *const libc::c_char,
    mut delimiter: libc::c_int,
) -> *const libc::c_char {
    return checkidentifier(id, delimiter, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn checksym(
    mut sym: *const libc::c_char,
    mut delimiter: libc::c_int,
) -> *const libc::c_char {
    return checkidentifier(sym, delimiter, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn checksid(mut id: *const libc::c_char) {
    checkid(id, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn checkssym(mut sym: *const libc::c_char) {
    checksym(sym, 0 as libc::c_int);
}
