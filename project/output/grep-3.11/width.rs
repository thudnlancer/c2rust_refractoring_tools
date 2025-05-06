#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strcmp(_: *const i8, _: *const i8) -> i32;
}
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub header: [i32; 1],
    pub level1: [i32; 4],
    pub level2: [libc::c_short; 384],
    pub level3: [u32; 448],
}
#[inline]
unsafe extern "C" fn streq3(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s23: i8,
    mut s24: i8,
    mut s25: i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if *s1.offset(3 as i32 as isize) as i32 == s23 as i32 {
        if s23 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return streq4(s1, s2, s24, s25, s26, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
unsafe extern "C" fn is_cjk_encoding(mut encoding: *const i8) -> i32 {
    if 0 as i32 != 0
        || streq0(
            encoding,
            b"EUC-JP\0" as *const u8 as *const i8,
            'E' as i32 as i8,
            'U' as i32 as i8,
            'C' as i32 as i8,
            '-' as i32 as i8,
            'J' as i32 as i8,
            'P' as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
        ) != 0
        || streq0(
            encoding,
            b"GB2312\0" as *const u8 as *const i8,
            'G' as i32 as i8,
            'B' as i32 as i8,
            '2' as i32 as i8,
            '3' as i32 as i8,
            '1' as i32 as i8,
            '2' as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
        ) != 0
        || streq0(
            encoding,
            b"GBK\0" as *const u8 as *const i8,
            'G' as i32 as i8,
            'B' as i32 as i8,
            'K' as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
        ) != 0
        || streq0(
            encoding,
            b"EUC-TW\0" as *const u8 as *const i8,
            'E' as i32 as i8,
            'U' as i32 as i8,
            'C' as i32 as i8,
            '-' as i32 as i8,
            'T' as i32 as i8,
            'W' as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
        ) != 0
        || streq0(
            encoding,
            b"BIG5\0" as *const u8 as *const i8,
            'B' as i32 as i8,
            'I' as i32 as i8,
            'G' as i32 as i8,
            '5' as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
        ) != 0
        || streq0(
            encoding,
            b"EUC-KR\0" as *const u8 as *const i8,
            'E' as i32 as i8,
            'U' as i32 as i8,
            'C' as i32 as i8,
            '-' as i32 as i8,
            'K' as i32 as i8,
            'R' as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
        ) != 0
        || streq0(
            encoding,
            b"CP949\0" as *const u8 as *const i8,
            'C' as i32 as i8,
            'P' as i32 as i8,
            '9' as i32 as i8,
            '4' as i32 as i8,
            '9' as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
        ) != 0
        || streq0(
            encoding,
            b"JOHAB\0" as *const u8 as *const i8,
            'J' as i32 as i8,
            'O' as i32 as i8,
            'H' as i32 as i8,
            'A' as i32 as i8,
            'B' as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
            0 as i32 as i8,
        ) != 0
    {
        return 1 as i32;
    }
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn streq9(mut s1: *const i8, mut s2: *const i8) -> i32 {
    return (strcmp(s1.offset(9 as i32 as isize), s2.offset(9 as i32 as isize))
        == 0 as i32) as i32;
}
#[inline]
unsafe extern "C" fn streq7(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if *s1.offset(7 as i32 as isize) as i32 == s27 as i32 {
        if s27 as i32 == 0 as i32 { return 1 as i32 } else { return streq8(s1, s2, s28) }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn streq4(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s24: i8,
    mut s25: i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if *s1.offset(4 as i32 as isize) as i32 == s24 as i32 {
        if s24 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return streq5(s1, s2, s25, s26, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn streq5(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s25: i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if *s1.offset(5 as i32 as isize) as i32 == s25 as i32 {
        if s25 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return streq6(s1, s2, s26, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn streq6(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if *s1.offset(6 as i32 as isize) as i32 == s26 as i32 {
        if s26 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return streq7(s1, s2, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn streq8(mut s1: *const i8, mut s2: *const i8, mut s28: i8) -> i32 {
    if *s1.offset(8 as i32 as isize) as i32 == s28 as i32 {
        if s28 as i32 == 0 as i32 { return 1 as i32 } else { return streq9(s1, s2) }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn streq2(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s22: i8,
    mut s23: i8,
    mut s24: i8,
    mut s25: i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if *s1.offset(2 as i32 as isize) as i32 == s22 as i32 {
        if s22 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return streq3(s1, s2, s23, s24, s25, s26, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn streq1(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s21: i8,
    mut s22: i8,
    mut s23: i8,
    mut s24: i8,
    mut s25: i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if *s1.offset(1 as i32 as isize) as i32 == s21 as i32 {
        if s21 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return streq2(s1, s2, s22, s23, s24, s25, s26, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn streq0(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s20: i8,
    mut s21: i8,
    mut s22: i8,
    mut s23: i8,
    mut s24: i8,
    mut s25: i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if *s1.offset(0 as i32 as isize) as i32 == s20 as i32 {
        if s20 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return streq1(s1, s2, s21, s22, s23, s24, s25, s26, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
static mut nonspacing_table_data: [u8; 3072] = [
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0 as i32 as u8,
    0x20 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf8 as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xfe as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xbf as i32 as u8,
    0xb6 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x3f as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0x17 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf8 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x1 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc0 as i32 as u8,
    0xbf as i32 as u8,
    0x9f as i32 as u8,
    0x3d as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0x2 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0x7 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc0 as i32 as u8,
    0xff as i32 as u8,
    0x1 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf8 as i32 as u8,
    0xf as i32 as u8,
    0x20 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc0 as i32 as u8,
    0xfb as i32 as u8,
    0xef as i32 as u8,
    0x3e as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xe as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x3 as i32 as u8,
    0xff as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xfc as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0x7 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x14 as i32 as u8,
    0xfe as i32 as u8,
    0x21 as i32 as u8,
    0xfe as i32 as u8,
    0 as i32 as u8,
    0xc as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x2 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x10 as i32 as u8,
    0x1e as i32 as u8,
    0x20 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x40 as i32 as u8,
    0x6 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x10 as i32 as u8,
    0x86 as i32 as u8,
    0x39 as i32 as u8,
    0x2 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x23 as i32 as u8,
    0 as i32 as u8,
    0x6 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x10 as i32 as u8,
    0xbe as i32 as u8,
    0x21 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xfc as i32 as u8,
    0x2 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x90 as i32 as u8,
    0x1e as i32 as u8,
    0x20 as i32 as u8,
    0x60 as i32 as u8,
    0 as i32 as u8,
    0xc as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x4 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x1 as i32 as u8,
    0x20 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x11 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xd0 as i32 as u8,
    0xc1 as i32 as u8,
    0x3d as i32 as u8,
    0x60 as i32 as u8,
    0 as i32 as u8,
    0xc as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x2 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x10 as i32 as u8,
    0 as i32 as u8,
    0x30 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x18 as i32 as u8,
    0x1e as i32 as u8,
    0x20 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x2 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x4 as i32 as u8,
    0x5c as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf2 as i32 as u8,
    0x7 as i32 as u8,
    0x80 as i32 as u8,
    0x7f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf2 as i32 as u8,
    0x1f as i32 as u8,
    0 as i32 as u8,
    0x7f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xa0 as i32 as u8,
    0x2 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xfe as i32 as u8,
    0x7f as i32 as u8,
    0xdf as i32 as u8,
    0xe0 as i32 as u8,
    0xff as i32 as u8,
    0xfe as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0x1f as i32 as u8,
    0x40 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xe0 as i32 as u8,
    0xfd as i32 as u8,
    0x66 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc3 as i32 as u8,
    0x1 as i32 as u8,
    0 as i32 as u8,
    0x1e as i32 as u8,
    0 as i32 as u8,
    0x64 as i32 as u8,
    0x20 as i32 as u8,
    0 as i32 as u8,
    0x20 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xe0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x1c as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xb0 as i32 as u8,
    0x3f as i32 as u8,
    0x40 as i32 as u8,
    0xfe as i32 as u8,
    0xf as i32 as u8,
    0x20 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf8 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x60 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x2 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x87 as i32 as u8,
    0x1 as i32 as u8,
    0x4 as i32 as u8,
    0xe as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0x9 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x40 as i32 as u8,
    0x7f as i32 as u8,
    0xe5 as i32 as u8,
    0x1f as i32 as u8,
    0xf8 as i32 as u8,
    0x9f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0x7f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xd0 as i32 as u8,
    0x17 as i32 as u8,
    0x4 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf8 as i32 as u8,
    0xf as i32 as u8,
    0 as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x3c as i32 as u8,
    0x3b as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x40 as i32 as u8,
    0xa3 as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf0 as i32 as u8,
    0xcf as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf7 as i32 as u8,
    0xff as i32 as u8,
    0xfd as i32 as u8,
    0x21 as i32 as u8,
    0x10 as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0 as i32 as u8,
    0xf8 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x7c as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xdf as i32 as u8,
    0xff as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0x1 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x3c as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x6 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0xf7 as i32 as u8,
    0x3f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x44 as i32 as u8,
    0x8 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x60 as i32 as u8,
    0x10 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x30 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0x3 as i32 as u8,
    0x80 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc0 as i32 as u8,
    0x3f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0xff as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x7 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc8 as i32 as u8,
    0x33 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x20 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x7e as i32 as u8,
    0x66 as i32 as u8,
    0 as i32 as u8,
    0x8 as i32 as u8,
    0x10 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x10 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x9d as i32 as u8,
    0xc1 as i32 as u8,
    0x2 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x30 as i32 as u8,
    0x40 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x20 as i32 as u8,
    0x21 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0x7f as i32 as u8,
    0xf8 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xf as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x40 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xe as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x20 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x1 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc0 as i32 as u8,
    0x7 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x6e as i32 as u8,
    0xf0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x87 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x60 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x18 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xe0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc0 as i32 as u8,
    0xff as i32 as u8,
    0x1 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x3c as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x2 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0x7f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x19 as i32 as u8,
    0x80 as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x78 as i32 as u8,
    0x26 as i32 as u8,
    0x4 as i32 as u8,
    0x20 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x7 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0xef as i32 as u8,
    0x1f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x8 as i32 as u8,
    0 as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc0 as i32 as u8,
    0x7f as i32 as u8,
    0 as i32 as u8,
    0x9e as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0xd3 as i32 as u8,
    0x40 as i32 as u8,
    0x2 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0xf8 as i32 as u8,
    0x7 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x18 as i32 as u8,
    0x1 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc0 as i32 as u8,
    0x1f as i32 as u8,
    0x1f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0x5c as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x40 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf8 as i32 as u8,
    0x85 as i32 as u8,
    0xd as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x3c as i32 as u8,
    0xb0 as i32 as u8,
    0x1 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x30 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf8 as i32 as u8,
    0xa7 as i32 as u8,
    0x1 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x28 as i32 as u8,
    0xbf as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xe0 as i32 as u8,
    0xbc as i32 as u8,
    0xf as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0xff as i32 as u8,
    0x6 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x58 as i32 as u8,
    0x8 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf0 as i32 as u8,
    0xc as i32 as u8,
    0x1 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x7e as i32 as u8,
    0x6 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf8 as i32 as u8,
    0x79 as i32 as u8,
    0x80 as i32 as u8,
    0 as i32 as u8,
    0x7e as i32 as u8,
    0xe as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xfc as i32 as u8,
    0x7f as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x7f as i32 as u8,
    0x3f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xfc as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xfc as i32 as u8,
    0x6d as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x7e as i32 as u8,
    0xb4 as i32 as u8,
    0xbf as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xa3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x18 as i32 as u8,
    0 as i32 as u8,
    0x3 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xc0 as i32 as u8,
    0x7 as i32 as u8,
    0x5 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0x81 as i32 as u8,
    0xff as i32 as u8,
    0x3f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x1f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x7f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0x7 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x10 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x60 as i32 as u8,
    0xf as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0x3f as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0x7f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0x3 as i32 as u8,
    0xf8 as i32 as u8,
    0xff as i32 as u8,
    0xe7 as i32 as u8,
    0xf as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x3c as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x1c as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0x7f as i32 as u8,
    0xf8 as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0x1f as i32 as u8,
    0x20 as i32 as u8,
    0 as i32 as u8,
    0x10 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf8 as i32 as u8,
    0xfe as i32 as u8,
    0xff as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x7f as i32 as u8,
    0xff as i32 as u8,
    0xff as i32 as u8,
    0xf9 as i32 as u8,
    0xdb as i32 as u8,
    0x7 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x7f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x40 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x7f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf0 as i32 as u8,
    0x7 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
];
static mut nonspacing_table_ind: [libc::c_schar; 248] = [
    0 as i32 as libc::c_schar,
    1 as i32 as libc::c_schar,
    2 as i32 as libc::c_schar,
    3 as i32 as libc::c_schar,
    4 as i32 as libc::c_schar,
    5 as i32 as libc::c_schar,
    6 as i32 as libc::c_schar,
    7 as i32 as libc::c_schar,
    8 as i32 as libc::c_schar,
    9 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    10 as i32 as libc::c_schar,
    11 as i32 as libc::c_schar,
    12 as i32 as libc::c_schar,
    13 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    14 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    15 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    16 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    17 as i32 as libc::c_schar,
    18 as i32 as libc::c_schar,
    19 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    20 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    21 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    22 as i32 as libc::c_schar,
    23 as i32 as libc::c_schar,
    24 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    25 as i32 as libc::c_schar,
    26 as i32 as libc::c_schar,
    27 as i32 as libc::c_schar,
    28 as i32 as libc::c_schar,
    29 as i32 as libc::c_schar,
    30 as i32 as libc::c_schar,
    31 as i32 as libc::c_schar,
    32 as i32 as libc::c_schar,
    33 as i32 as libc::c_schar,
    34 as i32 as libc::c_schar,
    35 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    36 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    37 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    38 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    39 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    40 as i32 as libc::c_schar,
    41 as i32 as libc::c_schar,
    42 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    43 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    44 as i32 as libc::c_schar,
    45 as i32 as libc::c_schar,
    46 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    47 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
];
static mut u_width2: C2RustUnnamed = C2RustUnnamed {
    header: [0; 1],
    level1: [0; 4],
    level2: [0; 384],
    level3: [0; 448],
};
#[inline]
unsafe extern "C" fn bitmap_lookup(
    mut table: *const libc::c_void,
    mut uc: ucs4_t,
) -> i32 {
    let mut index1: u32 = uc >> 16 as i32;
    if index1 < *(table as *const i32).offset(0 as i32 as isize) as u32 {
        let mut lookup1: i32 = *(table as *const i32)
            .offset((1 as i32 as u32).wrapping_add(index1) as isize);
        if lookup1 >= 0 as i32 {
            let mut index2: u32 = uc >> 9 as i32 & 127 as i32 as u32;
            let mut lookup2: i32 = *(table as *const libc::c_short)
                .offset((lookup1 as u32).wrapping_add(index2) as isize) as i32;
            if lookup2 >= 0 as i32 {
                let mut index3: u32 = uc >> 5 as i32 & 15 as i32 as u32;
                let mut lookup3: u32 = *(table as *const u32)
                    .offset((lookup2 as u32).wrapping_add(index3) as isize);
                return (lookup3 >> (uc & 0x1f as i32 as u32) & 1 as i32 as u32) as i32;
            }
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn uc_width(mut uc: ucs4_t, mut encoding: *const i8) -> i32 {
    if ((uc >> 9 as i32) as u64)
        < (::core::mem::size_of::<[libc::c_schar; 248]>() as u64)
            .wrapping_div(::core::mem::size_of::<libc::c_schar>() as u64)
    {
        let mut ind: i32 = nonspacing_table_ind[(uc >> 9 as i32) as usize] as i32;
        if ind >= 0 as i32 {
            if nonspacing_table_data[((64 as i32 * ind) as u32)
                .wrapping_add(uc >> 3 as i32 & 63 as i32 as u32) as usize] as i32
                >> (uc & 7 as i32 as u32) & 1 as i32 != 0
            {
                if uc > 0 as i32 as u32 && uc < 0xa0 as i32 as u32 {
                    return -(1 as i32)
                } else {
                    return 0 as i32
                }
            }
        }
    } else if uc >> 9 as i32 == (0xe0000 as i32 >> 9 as i32) as u32 {
        if uc >= 0xe0100 as i32 as u32 {
            if uc <= 0xe01ef as i32 as u32 {
                return 0 as i32;
            }
        } else if if uc >= 0xe0020 as i32 as u32 {
            (uc <= 0xe007f as i32 as u32) as i32
        } else {
            (uc == 0xe0001 as i32 as u32) as i32
        } != 0
        {
            return 0 as i32
        }
    }
    if bitmap_lookup(&u_width2 as *const C2RustUnnamed as *const libc::c_void, uc) != 0 {
        return 2 as i32;
    }
    if uc >= 0xa1 as i32 as u32 && uc < 0xff61 as i32 as u32
        && uc != 0x20a9 as i32 as u32 && is_cjk_encoding(encoding) != 0
    {
        return 2 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn run_static_initializers() {
    u_width2 = {
        let mut init = C2RustUnnamed {
            header: [4 as i32],
            level1: [
                (5 as i32 as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64)
                    .wrapping_div(::core::mem::size_of::<libc::c_short>() as u64)
                    .wrapping_add(0 as i32 as u64) as i32,
                (5 as i32 as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64)
                    .wrapping_div(::core::mem::size_of::<libc::c_short>() as u64)
                    .wrapping_add(128 as i32 as u64) as i32,
                (5 as i32 as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64)
                    .wrapping_div(::core::mem::size_of::<libc::c_short>() as u64)
                    .wrapping_add(256 as i32 as u64) as i32,
                (5 as i32 as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64)
                    .wrapping_div(::core::mem::size_of::<libc::c_short>() as u64)
                    .wrapping_add(256 as i32 as u64) as i32,
            ],
            level2: [
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(0 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(16 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(32 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(48 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(64 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(80 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(96 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(112 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(144 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(160 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(176 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(192 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(208 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(224 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(240 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(256 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(272 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(288 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(304 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(320 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(336 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(352 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(368 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(384 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(400 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(416 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(432 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                (5 as i32 as u64)
                    .wrapping_add(
                        (384 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
            ],
            level3: [
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xc000000 as u32,
                0x600 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x91e00 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x60000000 as u32,
                0x300000 as u32,
                0 as u32,
                0xfff00 as u32,
                0x80000000 as u32,
                0x80000 as u32,
                0x60000c02 as u32,
                0x104030 as u32,
                0x242c0400 as u32,
                0xc20 as u32,
                0x100 as u32,
                0xb85000 as u32,
                0 as u32,
                0xe00000 as u32,
                0x80010000 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x18000000 as u32,
                0 as u32,
                0x210000 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0x7fffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffff00ff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x1fffffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xf as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffff0000 as u32,
                0xffff0000 as u32,
                0xffffffff as u32,
                0xffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0x1 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x7f as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x3000f as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0x3fffff as u32,
                0 as u32,
                0x1ff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x6fef0000 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0x7 as u32,
                0x70000 as u32,
                0xffff00f0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xfffffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x10 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x8000 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x7fe4000 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffbfe001 as u32,
                0xffffffff as u32,
                0xdfffffff as u32,
                0xfffff as u32,
                0xffffffff as u32,
                0xf87ff as u32,
                0xff11ffff as u32,
                0xffffffff as u32,
                0x7fffffff as u32,
                0xfffffffd as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0x9fffffff as u32,
                0xffffffff as u32,
                0x3fffffff as u32,
                0xffff7800 as u32,
                0x40000ff as u32,
                0x600000 as u32,
                0x10 as u32,
                0 as u32,
                0xf8000000 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffff as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xe0e7103f as u32,
                0x1ff01800 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x10fff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xfffff000 as u32,
                0xf7ffffff as u32,
                0xffffffbf as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x1f1f0000 as u32,
                0xffff007f as u32,
                0x7ff1fff as u32,
                0x3ff003f as u32,
                0x7f00ff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
            ],
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];