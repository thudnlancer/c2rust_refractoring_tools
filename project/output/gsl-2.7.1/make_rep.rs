#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type C2RustUnnamed = u32;
pub const GSL_IEEE_TYPE_ZERO: C2RustUnnamed = 5;
pub const GSL_IEEE_TYPE_DENORMAL: C2RustUnnamed = 4;
pub const GSL_IEEE_TYPE_NORMAL: C2RustUnnamed = 3;
pub const GSL_IEEE_TYPE_INF: C2RustUnnamed = 2;
pub const GSL_IEEE_TYPE_NAN: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ieee_float_rep {
    pub sign: i32,
    pub mantissa: [i8; 24],
    pub exponent: i32,
    pub type_0: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ieee_double_rep {
    pub sign: i32,
    pub mantissa: [i8; 53],
    pub exponent: i32,
    pub type_0: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub byte: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub f: libc::c_float,
    pub ieee: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub f: libc::c_float,
    pub b: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub l: i64,
    pub c: [i8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub byte: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub d: libc::c_double,
    pub ieee: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub d: libc::c_double,
    pub b: [u8; 8],
}
unsafe extern "C" fn little_endian_p() -> i32 {
    let mut u: C2RustUnnamed_3 = C2RustUnnamed_3 { l: 0 };
    u.l = 1 as i32 as i64;
    return (u
        .c[(::core::mem::size_of::<i64>() as u64).wrapping_sub(1 as i32 as u64) as usize]
        as i32 == 1 as i32) as i32;
}
unsafe extern "C" fn make_float_bigendian(mut x: *mut libc::c_float) {
    let mut u: C2RustUnnamed_2 = C2RustUnnamed_2 { f: 0. };
    let mut v: C2RustUnnamed_2 = C2RustUnnamed_2 { f: 0. };
    u.f = *x;
    v.b[0 as i32 as usize] = u.b[3 as i32 as usize];
    v.b[1 as i32 as usize] = u.b[2 as i32 as usize];
    v.b[2 as i32 as usize] = u.b[1 as i32 as usize];
    v.b[3 as i32 as usize] = u.b[0 as i32 as usize];
    *x = v.f;
}
unsafe extern "C" fn make_double_bigendian(mut x: *mut libc::c_double) {
    let mut u: C2RustUnnamed_6 = C2RustUnnamed_6 { d: 0. };
    let mut v: C2RustUnnamed_6 = C2RustUnnamed_6 { d: 0. };
    u.d = *x;
    v.b[0 as i32 as usize] = u.b[7 as i32 as usize];
    v.b[1 as i32 as usize] = u.b[6 as i32 as usize];
    v.b[2 as i32 as usize] = u.b[5 as i32 as usize];
    v.b[3 as i32 as usize] = u.b[4 as i32 as usize];
    v.b[4 as i32 as usize] = u.b[3 as i32 as usize];
    v.b[5 as i32 as usize] = u.b[2 as i32 as usize];
    v.b[6 as i32 as usize] = u.b[1 as i32 as usize];
    v.b[7 as i32 as usize] = u.b[0 as i32 as usize];
    *x = v.d;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ieee_float_to_rep(
    mut x: *const libc::c_float,
    mut r: *mut gsl_ieee_float_rep,
) {
    let mut e: i32 = 0;
    let mut non_zero: i32 = 0;
    let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { f: 0. };
    u.f = *x;
    if little_endian_p() != 0 {
        make_float_bigendian(&mut u.f);
    }
    if u.ieee.byte[3 as i32 as usize] as i32 >> 7 as i32 != 0 {
        (*r).sign = 1 as i32;
    } else {
        (*r).sign = 0 as i32;
    }
    e = (u.ieee.byte[3 as i32 as usize] as i32 & 0x7f as i32) << 1 as i32
        | (u.ieee.byte[2 as i32 as usize] as i32 & 0x80 as i32) >> 7 as i32;
    (*r).exponent = e - 127 as i32;
    sprint_byte(
        (u.ieee.byte[2 as i32 as usize] as i32 & 0x7f as i32) << 1 as i32,
        ((*r).mantissa).as_mut_ptr(),
    );
    sprint_byte(
        u.ieee.byte[1 as i32 as usize] as i32,
        ((*r).mantissa).as_mut_ptr().offset(7 as i32 as isize),
    );
    sprint_byte(
        u.ieee.byte[0 as i32 as usize] as i32,
        ((*r).mantissa).as_mut_ptr().offset(15 as i32 as isize),
    );
    (*r).mantissa[23 as i32 as usize] = '\0' as i32 as i8;
    non_zero = (u.ieee.byte[0 as i32 as usize] as i32 != 0
        || u.ieee.byte[1 as i32 as usize] as i32 != 0
        || u.ieee.byte[2 as i32 as usize] as i32 & 0x7f as i32 != 0) as i32;
    (*r).type_0 = determine_ieee_type(non_zero, e, 255 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ieee_double_to_rep(
    mut x: *const libc::c_double,
    mut r: *mut gsl_ieee_double_rep,
) {
    let mut e: i32 = 0;
    let mut non_zero: i32 = 0;
    let mut u: C2RustUnnamed_5 = C2RustUnnamed_5 { d: 0. };
    u.d = *x;
    if little_endian_p() != 0 {
        make_double_bigendian(&mut u.d);
    }
    if u.ieee.byte[7 as i32 as usize] as i32 >> 7 as i32 != 0 {
        (*r).sign = 1 as i32;
    } else {
        (*r).sign = 0 as i32;
    }
    e = (u.ieee.byte[7 as i32 as usize] as i32 & 0x7f as i32) << 4 as i32
        ^ (u.ieee.byte[6 as i32 as usize] as i32 & 0xf0 as i32) >> 4 as i32;
    (*r).exponent = e - 1023 as i32;
    sprint_nybble(u.ieee.byte[6 as i32 as usize] as i32, ((*r).mantissa).as_mut_ptr());
    sprint_byte(
        u.ieee.byte[5 as i32 as usize] as i32,
        ((*r).mantissa).as_mut_ptr().offset(4 as i32 as isize),
    );
    sprint_byte(
        u.ieee.byte[4 as i32 as usize] as i32,
        ((*r).mantissa).as_mut_ptr().offset(12 as i32 as isize),
    );
    sprint_byte(
        u.ieee.byte[3 as i32 as usize] as i32,
        ((*r).mantissa).as_mut_ptr().offset(20 as i32 as isize),
    );
    sprint_byte(
        u.ieee.byte[2 as i32 as usize] as i32,
        ((*r).mantissa).as_mut_ptr().offset(28 as i32 as isize),
    );
    sprint_byte(
        u.ieee.byte[1 as i32 as usize] as i32,
        ((*r).mantissa).as_mut_ptr().offset(36 as i32 as isize),
    );
    sprint_byte(
        u.ieee.byte[0 as i32 as usize] as i32,
        ((*r).mantissa).as_mut_ptr().offset(44 as i32 as isize),
    );
    (*r).mantissa[52 as i32 as usize] = '\0' as i32 as i8;
    non_zero = (u.ieee.byte[0 as i32 as usize] as i32 != 0
        || u.ieee.byte[1 as i32 as usize] as i32 != 0
        || u.ieee.byte[2 as i32 as usize] as i32 != 0
        || u.ieee.byte[3 as i32 as usize] as i32 != 0
        || u.ieee.byte[4 as i32 as usize] as i32 != 0
        || u.ieee.byte[5 as i32 as usize] as i32 != 0
        || u.ieee.byte[6 as i32 as usize] as i32 & 0xf as i32 != 0) as i32;
    (*r).type_0 = determine_ieee_type(non_zero, e, 2047 as i32);
}
static mut nybble: [[i8; 5]; 16] = unsafe {
    [
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"0000\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"0001\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"0010\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"0011\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"0100\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"0101\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"0110\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"0111\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"1000\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"1001\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"1010\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"1011\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"1100\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"1101\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"1110\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"1111\0"),
    ]
};
unsafe extern "C" fn sprint_nybble(mut i: i32, mut s: *mut i8) {
    let mut c: *mut i8 = 0 as *mut i8;
    c = (nybble[(i & 0xf as i32) as usize]).as_mut_ptr();
    *s = *c.offset(0 as i32 as isize);
    *s.offset(1 as i32 as isize) = *c.offset(1 as i32 as isize);
    *s.offset(2 as i32 as isize) = *c.offset(2 as i32 as isize);
    *s.offset(3 as i32 as isize) = *c.offset(3 as i32 as isize);
}
unsafe extern "C" fn sprint_byte(mut i: i32, mut s: *mut i8) {
    let mut c: *mut i8 = 0 as *mut i8;
    c = (nybble[((i & 0xf0 as i32) >> 4 as i32) as usize]).as_mut_ptr();
    *s = *c.offset(0 as i32 as isize);
    *s.offset(1 as i32 as isize) = *c.offset(1 as i32 as isize);
    *s.offset(2 as i32 as isize) = *c.offset(2 as i32 as isize);
    *s.offset(3 as i32 as isize) = *c.offset(3 as i32 as isize);
    c = (nybble[(i & 0xf as i32) as usize]).as_mut_ptr();
    *s.offset(4 as i32 as isize) = *c.offset(0 as i32 as isize);
    *s.offset(5 as i32 as isize) = *c.offset(1 as i32 as isize);
    *s.offset(6 as i32 as isize) = *c.offset(2 as i32 as isize);
    *s.offset(7 as i32 as isize) = *c.offset(3 as i32 as isize);
}
unsafe extern "C" fn determine_ieee_type(
    mut non_zero: i32,
    mut exponent: i32,
    mut max_exponent: i32,
) -> i32 {
    if exponent == max_exponent {
        if non_zero != 0 {
            return GSL_IEEE_TYPE_NAN as i32
        } else {
            return GSL_IEEE_TYPE_INF as i32
        }
    } else if exponent == 0 as i32 {
        if non_zero != 0 {
            return GSL_IEEE_TYPE_DENORMAL as i32
        } else {
            return GSL_IEEE_TYPE_ZERO as i32
        }
    } else {
        return GSL_IEEE_TYPE_NORMAL as i32
    };
}