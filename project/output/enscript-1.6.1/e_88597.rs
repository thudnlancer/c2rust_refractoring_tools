#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encoding_table_st {
    pub code: libc::c_int,
    pub character: *mut libc::c_char,
}
pub type AFMEncodingTable = encoding_table_st;
#[no_mangle]
pub static mut afm_88597_encoding: [AFMEncodingTable; 257] = [
    {
        let mut init = encoding_table_st {
            code: 0 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x10 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x11 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x12 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x13 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x14 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x15 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x16 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x17 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x18 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x19 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1a as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1b as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1c as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1d as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1e as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1f as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x20 as libc::c_int,
            character: b"space\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x21 as libc::c_int,
            character: b"exclam\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x22 as libc::c_int,
            character: b"quotedbl\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x23 as libc::c_int,
            character: b"numbersign\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x24 as libc::c_int,
            character: b"dollar\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x25 as libc::c_int,
            character: b"percent\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x26 as libc::c_int,
            character: b"ampersand\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x27 as libc::c_int,
            character: b"quoteright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x28 as libc::c_int,
            character: b"parenleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x29 as libc::c_int,
            character: b"parenright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2a as libc::c_int,
            character: b"asterisk\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2b as libc::c_int,
            character: b"plus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2c as libc::c_int,
            character: b"comma\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2d as libc::c_int,
            character: b"hyphen\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2e as libc::c_int,
            character: b"period\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2f as libc::c_int,
            character: b"slash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x30 as libc::c_int,
            character: b"zero\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x31 as libc::c_int,
            character: b"one\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x32 as libc::c_int,
            character: b"two\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x33 as libc::c_int,
            character: b"three\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x34 as libc::c_int,
            character: b"four\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x35 as libc::c_int,
            character: b"five\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x36 as libc::c_int,
            character: b"six\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x37 as libc::c_int,
            character: b"seven\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x38 as libc::c_int,
            character: b"eight\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x39 as libc::c_int,
            character: b"nine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3a as libc::c_int,
            character: b"colon\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3b as libc::c_int,
            character: b"semicolon\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3c as libc::c_int,
            character: b"less\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3d as libc::c_int,
            character: b"equal\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3e as libc::c_int,
            character: b"greater\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3f as libc::c_int,
            character: b"question\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x40 as libc::c_int,
            character: b"at\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x41 as libc::c_int,
            character: b"A\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x42 as libc::c_int,
            character: b"B\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x43 as libc::c_int,
            character: b"C\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x44 as libc::c_int,
            character: b"D\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x45 as libc::c_int,
            character: b"E\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x46 as libc::c_int,
            character: b"F\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x47 as libc::c_int,
            character: b"G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x48 as libc::c_int,
            character: b"H\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x49 as libc::c_int,
            character: b"I\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4a as libc::c_int,
            character: b"J\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4b as libc::c_int,
            character: b"K\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4c as libc::c_int,
            character: b"L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4d as libc::c_int,
            character: b"M\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4e as libc::c_int,
            character: b"N\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4f as libc::c_int,
            character: b"O\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x50 as libc::c_int,
            character: b"P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x51 as libc::c_int,
            character: b"Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x52 as libc::c_int,
            character: b"R\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x53 as libc::c_int,
            character: b"S\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x54 as libc::c_int,
            character: b"T\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x55 as libc::c_int,
            character: b"U\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x56 as libc::c_int,
            character: b"V\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x57 as libc::c_int,
            character: b"W\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x58 as libc::c_int,
            character: b"X\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x59 as libc::c_int,
            character: b"Y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5a as libc::c_int,
            character: b"Z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5b as libc::c_int,
            character: b"bracketleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5c as libc::c_int,
            character: b"backslash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5d as libc::c_int,
            character: b"bracketright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5e as libc::c_int,
            character: b"asciicircum\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5f as libc::c_int,
            character: b"underscore\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x60 as libc::c_int,
            character: b"quoteleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x61 as libc::c_int,
            character: b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x62 as libc::c_int,
            character: b"b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x63 as libc::c_int,
            character: b"c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x64 as libc::c_int,
            character: b"d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x65 as libc::c_int,
            character: b"e\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x66 as libc::c_int,
            character: b"f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x67 as libc::c_int,
            character: b"g\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x68 as libc::c_int,
            character: b"h\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x69 as libc::c_int,
            character: b"i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6a as libc::c_int,
            character: b"j\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6b as libc::c_int,
            character: b"k\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6c as libc::c_int,
            character: b"l\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6d as libc::c_int,
            character: b"m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6e as libc::c_int,
            character: b"n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6f as libc::c_int,
            character: b"o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x70 as libc::c_int,
            character: b"p\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x71 as libc::c_int,
            character: b"q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x72 as libc::c_int,
            character: b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x73 as libc::c_int,
            character: b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x74 as libc::c_int,
            character: b"t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x75 as libc::c_int,
            character: b"u\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x76 as libc::c_int,
            character: b"v\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x77 as libc::c_int,
            character: b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x78 as libc::c_int,
            character: b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x79 as libc::c_int,
            character: b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7a as libc::c_int,
            character: b"z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7b as libc::c_int,
            character: b"braceleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7c as libc::c_int,
            character: b"bar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7d as libc::c_int,
            character: b"braceright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7e as libc::c_int,
            character: b"tilde\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7f as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x80 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x81 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x82 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x83 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x84 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x85 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x86 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x87 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x88 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x89 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8a as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8b as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8c as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8d as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8e as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8f as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x90 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x91 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x92 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x93 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x94 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x95 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x96 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x97 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x98 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x99 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9a as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9b as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9c as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9d as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9e as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9f as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa0 as libc::c_int,
            character: b"space\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa1 as libc::c_int,
            character: b"quoteleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa2 as libc::c_int,
            character: b"quoteright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa3 as libc::c_int,
            character: b"sterling\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa4 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa5 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa6 as libc::c_int,
            character: b"brokenbar\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa7 as libc::c_int,
            character: b"section\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa8 as libc::c_int,
            character: b"dieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa9 as libc::c_int,
            character: b"copyright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xaa as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xab as libc::c_int,
            character: b"guillemotleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xac as libc::c_int,
            character: b"logicalnot\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xad as libc::c_int,
            character: b"hyphen\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xae as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xaf as libc::c_int,
            character: b"endash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb0 as libc::c_int,
            character: b"degree\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb1 as libc::c_int,
            character: b"plusminus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb2 as libc::c_int,
            character: b"twosuperior\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb3 as libc::c_int,
            character: b"threesuperior\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb4 as libc::c_int,
            character: b"acute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb5 as libc::c_int,
            character: b"dieresisacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb6 as libc::c_int,
            character: b"afii9936\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb7 as libc::c_int,
            character: b"periodcentered\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb8 as libc::c_int,
            character: b"afii9937\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb9 as libc::c_int,
            character: b"afii9938\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xba as libc::c_int,
            character: b"afii9939\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbb as libc::c_int,
            character: b"guillemotright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbc as libc::c_int,
            character: b"afii9940\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbd as libc::c_int,
            character: b"onehalf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbe as libc::c_int,
            character: b"afii9941\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbf as libc::c_int,
            character: b"afii9942\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc0 as libc::c_int,
            character: b"afii9977\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc1 as libc::c_int,
            character: b"afii9793\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc2 as libc::c_int,
            character: b"afii9794\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc3 as libc::c_int,
            character: b"afii9796\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc4 as libc::c_int,
            character: b"afii9797\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc5 as libc::c_int,
            character: b"afii9798\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc6 as libc::c_int,
            character: b"afii9801\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc7 as libc::c_int,
            character: b"afii9802\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc8 as libc::c_int,
            character: b"afii9803\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc9 as libc::c_int,
            character: b"afii9804\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xca as libc::c_int,
            character: b"afii9805\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xcb as libc::c_int,
            character: b"afii9806\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xcc as libc::c_int,
            character: b"afii9807\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xcd as libc::c_int,
            character: b"afii9808\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xce as libc::c_int,
            character: b"afii9809\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xcf as libc::c_int,
            character: b"afii9810\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd0 as libc::c_int,
            character: b"afii9811\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd1 as libc::c_int,
            character: b"afii9813\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd2 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd3 as libc::c_int,
            character: b"afii9814\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd4 as libc::c_int,
            character: b"afii9816\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd5 as libc::c_int,
            character: b"afii9817\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd6 as libc::c_int,
            character: b"afii9818\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd7 as libc::c_int,
            character: b"afii9819\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd8 as libc::c_int,
            character: b"afii9820\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd9 as libc::c_int,
            character: b"afii9821\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xda as libc::c_int,
            character: b"afii9943\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xdb as libc::c_int,
            character: b"afii9944\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xdc as libc::c_int,
            character: b"afii9968\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xdd as libc::c_int,
            character: b"afii9969\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xde as libc::c_int,
            character: b"afii9970\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xdf as libc::c_int,
            character: b"afii9971\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe0 as libc::c_int,
            character: b"afii9978\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe1 as libc::c_int,
            character: b"afii9825\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe2 as libc::c_int,
            character: b"afii9826\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe3 as libc::c_int,
            character: b"afii9828\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe4 as libc::c_int,
            character: b"afii9829\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe5 as libc::c_int,
            character: b"afii9830\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe6 as libc::c_int,
            character: b"afii9833\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe7 as libc::c_int,
            character: b"afii9834\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe8 as libc::c_int,
            character: b"afii9835\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe9 as libc::c_int,
            character: b"afii9836\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xea as libc::c_int,
            character: b"afii9837\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xeb as libc::c_int,
            character: b"afii9838\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xec as libc::c_int,
            character: b"afii9839\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xed as libc::c_int,
            character: b"afii9840\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xee as libc::c_int,
            character: b"afii9841\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xef as libc::c_int,
            character: b"afii9842\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf0 as libc::c_int,
            character: b"afii9843\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf1 as libc::c_int,
            character: b"afii9845\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf2 as libc::c_int,
            character: b"afii9847\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf3 as libc::c_int,
            character: b"afii9846\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf4 as libc::c_int,
            character: b"afii9848\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf5 as libc::c_int,
            character: b"afii9849\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf6 as libc::c_int,
            character: b"afii9850\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf7 as libc::c_int,
            character: b"afii9851\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf8 as libc::c_int,
            character: b"afii9852\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf9 as libc::c_int,
            character: b"afii9853\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfa as libc::c_int,
            character: b"afii9975\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfb as libc::c_int,
            character: b"afii9976\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfc as libc::c_int,
            character: b"afii9972\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfd as libc::c_int,
            character: b"afii9973\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfe as libc::c_int,
            character: b"afii9974\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xff as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
