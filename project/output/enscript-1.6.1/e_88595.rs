#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encoding_table_st {
    pub code: i32,
    pub character: *mut i8,
}
pub type AFMEncodingTable = encoding_table_st;
#[no_mangle]
pub static mut afm_88595_encoding: [AFMEncodingTable; 257] = [
    {
        let mut init = encoding_table_st {
            code: 0 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x10 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x11 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x12 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x13 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x14 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x15 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x16 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x17 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x18 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x19 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1a as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1b as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1c as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1d as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1e as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1f as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x20 as i32,
            character: b"space\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x21 as i32,
            character: b"exclam\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x22 as i32,
            character: b"quotedbl\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x23 as i32,
            character: b"numbersign\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x24 as i32,
            character: b"dollar\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x25 as i32,
            character: b"percent\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x26 as i32,
            character: b"ampersand\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x27 as i32,
            character: b"quoteright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x28 as i32,
            character: b"parenleft\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x29 as i32,
            character: b"parenright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2a as i32,
            character: b"asterisk\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2b as i32,
            character: b"plus\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2c as i32,
            character: b"comma\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2d as i32,
            character: b"minus\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2e as i32,
            character: b"period\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2f as i32,
            character: b"slash\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x30 as i32,
            character: b"zero\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x31 as i32,
            character: b"one\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x32 as i32,
            character: b"two\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x33 as i32,
            character: b"three\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x34 as i32,
            character: b"four\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x35 as i32,
            character: b"five\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x36 as i32,
            character: b"six\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x37 as i32,
            character: b"seven\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x38 as i32,
            character: b"eight\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x39 as i32,
            character: b"nine\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3a as i32,
            character: b"colon\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3b as i32,
            character: b"semicolon\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3c as i32,
            character: b"less\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3d as i32,
            character: b"equal\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3e as i32,
            character: b"greater\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3f as i32,
            character: b"question\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x40 as i32,
            character: b"at\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x41 as i32,
            character: b"A\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x42 as i32,
            character: b"B\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x43 as i32,
            character: b"C\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x44 as i32,
            character: b"D\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x45 as i32,
            character: b"E\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x46 as i32,
            character: b"F\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x47 as i32,
            character: b"G\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x48 as i32,
            character: b"H\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x49 as i32,
            character: b"I\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4a as i32,
            character: b"J\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4b as i32,
            character: b"K\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4c as i32,
            character: b"L\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4d as i32,
            character: b"M\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4e as i32,
            character: b"N\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4f as i32,
            character: b"O\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x50 as i32,
            character: b"P\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x51 as i32,
            character: b"Q\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x52 as i32,
            character: b"R\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x53 as i32,
            character: b"S\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x54 as i32,
            character: b"T\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x55 as i32,
            character: b"U\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x56 as i32,
            character: b"V\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x57 as i32,
            character: b"W\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x58 as i32,
            character: b"X\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x59 as i32,
            character: b"Y\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5a as i32,
            character: b"Z\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5b as i32,
            character: b"bracketleft\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5c as i32,
            character: b"backslash\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5d as i32,
            character: b"bracketright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5e as i32,
            character: b"asciicircum\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5f as i32,
            character: b"underscore\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x60 as i32,
            character: b"quoteleft\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x61 as i32,
            character: b"a\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x62 as i32,
            character: b"b\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x63 as i32,
            character: b"c\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x64 as i32,
            character: b"d\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x65 as i32,
            character: b"e\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x66 as i32,
            character: b"f\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x67 as i32,
            character: b"g\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x68 as i32,
            character: b"h\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x69 as i32,
            character: b"i\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6a as i32,
            character: b"j\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6b as i32,
            character: b"k\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6c as i32,
            character: b"l\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6d as i32,
            character: b"m\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6e as i32,
            character: b"n\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6f as i32,
            character: b"o\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x70 as i32,
            character: b"p\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x71 as i32,
            character: b"q\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x72 as i32,
            character: b"r\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x73 as i32,
            character: b"s\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x74 as i32,
            character: b"t\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x75 as i32,
            character: b"u\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x76 as i32,
            character: b"v\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x77 as i32,
            character: b"w\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x78 as i32,
            character: b"x\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x79 as i32,
            character: b"y\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7a as i32,
            character: b"z\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7b as i32,
            character: b"braceleft\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7c as i32,
            character: b"bar\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7d as i32,
            character: b"braceright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7e as i32,
            character: b"tilde\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7f as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x80 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x81 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x82 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x83 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x84 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x85 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x86 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x87 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x88 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x89 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8a as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8b as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8c as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8d as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8e as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8f as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x90 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x91 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x92 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x93 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x94 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x95 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x96 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x97 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x98 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x99 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9a as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9b as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9c as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9d as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9e as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9f as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa0 as i32,
            character: b"space\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa1 as i32,
            character: b"afii10023\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa2 as i32,
            character: b"afii10051\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa3 as i32,
            character: b"afii10052\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa4 as i32,
            character: b"afii10053\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa5 as i32,
            character: b"afii10054\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa6 as i32,
            character: b"afii10055\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa7 as i32,
            character: b"afii10056\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa8 as i32,
            character: b"afii10057\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa9 as i32,
            character: b"afii10058\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xaa as i32,
            character: b"afii10059\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xab as i32,
            character: b"afii10060\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xac as i32,
            character: b"afii10061\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xad as i32,
            character: b"hyphen\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xae as i32,
            character: b"afii10062\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xaf as i32,
            character: b"afii10145\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb0 as i32,
            character: b"afii10017\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb1 as i32,
            character: b"afii10018\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb2 as i32,
            character: b"afii10019\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb3 as i32,
            character: b"afii10020\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb4 as i32,
            character: b"afii10021\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb5 as i32,
            character: b"afii10022\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb6 as i32,
            character: b"afii10024\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb7 as i32,
            character: b"afii10025\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb8 as i32,
            character: b"afii10026\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb9 as i32,
            character: b"afii10027\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xba as i32,
            character: b"afii10028\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbb as i32,
            character: b"afii10029\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbc as i32,
            character: b"afii10030\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbd as i32,
            character: b"afii10031\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbe as i32,
            character: b"afii10032\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbf as i32,
            character: b"afii10033\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc0 as i32,
            character: b"afii10034\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc1 as i32,
            character: b"afii10035\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc2 as i32,
            character: b"afii10036\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc3 as i32,
            character: b"afii10037\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc4 as i32,
            character: b"afii10038\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc5 as i32,
            character: b"afii10039\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc6 as i32,
            character: b"afii10040\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc7 as i32,
            character: b"afii10041\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc8 as i32,
            character: b"afii10042\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc9 as i32,
            character: b"afii10043\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xca as i32,
            character: b"afii10044\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xcb as i32,
            character: b"afii10045\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xcc as i32,
            character: b"afii10046\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xcd as i32,
            character: b"afii10047\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xce as i32,
            character: b"afii10048\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xcf as i32,
            character: b"afii10049\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd0 as i32,
            character: b"afii10065\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd1 as i32,
            character: b"afii10066\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd2 as i32,
            character: b"afii10067\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd3 as i32,
            character: b"afii10068\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd4 as i32,
            character: b"afii10069\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd5 as i32,
            character: b"afii10070\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd6 as i32,
            character: b"afii10072\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd7 as i32,
            character: b"afii10073\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd8 as i32,
            character: b"afii10074\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd9 as i32,
            character: b"afii10075\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xda as i32,
            character: b"afii10076\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xdb as i32,
            character: b"afii10077\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xdc as i32,
            character: b"afii10078\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xdd as i32,
            character: b"afii10079\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xde as i32,
            character: b"afii10080\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xdf as i32,
            character: b"afii10081\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe0 as i32,
            character: b"afii10082\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe1 as i32,
            character: b"afii10083\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe2 as i32,
            character: b"afii10084\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe3 as i32,
            character: b"afii10085\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe4 as i32,
            character: b"afii10086\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe5 as i32,
            character: b"afii10087\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe6 as i32,
            character: b"afii10088\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe7 as i32,
            character: b"afii10089\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe8 as i32,
            character: b"afii10090\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe9 as i32,
            character: b"afii10091\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xea as i32,
            character: b"afii10092\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xeb as i32,
            character: b"afii10093\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xec as i32,
            character: b"afii10094\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xed as i32,
            character: b"afii10095\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xee as i32,
            character: b"afii10096\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xef as i32,
            character: b"afii10097\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf0 as i32,
            character: b"afii61352\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf1 as i32,
            character: b"afii10071\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf2 as i32,
            character: b"afii10099\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf3 as i32,
            character: b"afii10100\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf4 as i32,
            character: b"afii10101\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf5 as i32,
            character: b"afii10102\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf6 as i32,
            character: b"afii10103\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf7 as i32,
            character: b"afii10104\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf8 as i32,
            character: b"afii10105\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf9 as i32,
            character: b"afii10106\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfa as i32,
            character: b"afii10107\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfb as i32,
            character: b"afii10108\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfc as i32,
            character: b"afii10109\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfd as i32,
            character: b"yacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfe as i32,
            character: b"afii10110\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xff as i32,
            character: b"afii10193\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
];