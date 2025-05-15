use std::mem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IeeeType {
    Nan,
    Inf,
    Normal,
    Denormal,
    Zero,
}

#[derive(Debug, Clone, Copy)]
pub struct IeeeFloatRep {
    pub sign: i32,
    pub mantissa: [u8; 24],
    pub exponent: i32,
    pub type_: IeeeType,
}

#[derive(Debug, Clone, Copy)]
pub struct IeeeDoubleRep {
    pub sign: i32,
    pub mantissa: [u8; 53],
    pub exponent: i32,
    pub type_: IeeeType,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
union FloatUnion {
    f: f32,
    bytes: [u8; 4],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
union DoubleUnion {
    d: f64,
    bytes: [u8; 8],
}

const NYBBLE: [[u8; 5]; 16] = [
    *b"0000\0", *b"0001\0", *b"0010\0", *b"0011\0",
    *b"0100\0", *b"0101\0", *b"0110\0", *b"0111\0",
    *b"1000\0", *b"1001\0", *b"1010\0", *b"1011\0",
    *b"1100\0", *b"1101\0", *b"1110\0", *b"1111\0",
];

fn is_little_endian() -> bool {
    let num: u32 = 1;
    unsafe { *(num as *const _ as *const u8) == 1 }
}

fn make_float_big_endian(x: &mut f32) {
    if is_little_endian() {
        let bytes = x.to_be_bytes();
        *x = f32::from_be_bytes(bytes);
    }
}

fn make_double_big_endian(x: &mut f64) {
    if is_little_endian() {
        let bytes = x.to_be_bytes();
        *x = f64::from_be_bytes(bytes);
    }
}

fn sprint_nybble(i: u8, s: &mut [u8]) {
    let nyb = NYBBLE[(i & 0xF) as usize];
    s[..4].copy_from_slice(&nyb[..4]);
}

fn sprint_byte(i: u8, s: &mut [u8]) {
    sprint_nybble(i >> 4, s);
    sprint_nybble(i & 0xF, &mut s[4..]);
}

fn determine_ieee_type(non_zero: bool, exponent: i32, max_exponent: i32) -> IeeeType {
    match (exponent, non_zero) {
        (e, _) if e == max_exponent => {
            if non_zero { IeeeType::Nan } else { IeeeType::Inf }
        }
        (0, true) => IeeeType::Denormal,
        (0, false) => IeeeType::Zero,
        _ => IeeeType::Normal,
    }
}

pub fn gsl_ieee_float_to_rep(x: f32) -> IeeeFloatRep {
    let mut u = FloatUnion { f: x };
    make_float_big_endian(unsafe { &mut u.f });

    let bytes = unsafe { u.bytes };
    let sign = (bytes[3] >> 7) as i32;
    let e = ((bytes[3] & 0x7F) as i32) << 1 | ((bytes[2] & 0x80) as i32 >> 7);
    let exponent = e - 127;

    let mut mantissa = [0u8; 24];
    sprint_byte(bytes[2] & 0x7F, &mut mantissa[0..8]);
    sprint_byte(bytes[1], &mut mantissa[7..15]);
    sprint_byte(bytes[0], &mut mantissa[15..23]);
    mantissa[23] = b'\0';

    let non_zero = bytes[0] != 0 || bytes[1] != 0 || (bytes[2] & 0x7F) != 0;
    let type_ = determine_ieee_type(non_zero, e, 255);

    IeeeFloatRep {
        sign,
        mantissa,
        exponent,
        type_,
    }
}

pub fn gsl_ieee_double_to_rep(x: f64) -> IeeeDoubleRep {
    let mut u = DoubleUnion { d: x };
    make_double_big_endian(unsafe { &mut u.d });

    let bytes = unsafe { u.bytes };
    let sign = (bytes[7] >> 7) as i32;
    let e = ((bytes[7] & 0x7F) as i32) << 4 | ((bytes[6] & 0xF0) as i32 >> 4);
    let exponent = e - 1023;

    let mut mantissa = [0u8; 53];
    sprint_nybble(bytes[6] & 0x0F, &mut mantissa[0..4]);
    sprint_byte(bytes[5], &mut mantissa[4..12]);
    sprint_byte(bytes[4], &mut mantissa[12..20]);
    sprint_byte(bytes[3], &mut mantissa[20..28]);
    sprint_byte(bytes[2], &mut mantissa[28..36]);
    sprint_byte(bytes[1], &mut mantissa[36..44]);
    sprint_byte(bytes[0], &mut mantissa[44..52]);
    mantissa[52] = b'\0';

    let non_zero = bytes[0] != 0
        || bytes[1] != 0
        || bytes[2] != 0
        || bytes[3] != 0
        || bytes[4] != 0
        || bytes[5] != 0
        || (bytes[6] & 0x0F) != 0;
    let type_ = determine_ieee_type(non_zero, e, 2047);

    IeeeDoubleRep {
        sign,
        mantissa,
        exponent,
        type_,
    }
}