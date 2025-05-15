use std::mem;

#[repr(C)]
#[derive(Debug)]
pub struct GslIeeeFloatRep {
    pub sign: i32,
    pub mantissa: [u8; 24],
    pub exponent: i32,
    pub type_: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct GslIeeeDoubleRep {
    pub sign: i32,
    pub mantissa: [u8; 53],
    pub exponent: i32,
    pub type_: i32,
}

pub const GSL_IEEE_TYPE_NAN: i32 = 0;
pub const GSL_IEEE_TYPE_INF: i32 = 1;
pub const GSL_IEEE_TYPE_NORMAL: i32 = 2;
pub const GSL_IEEE_TYPE_DENORMAL: i32 = 3;
pub const GSL_IEEE_TYPE_ZERO: i32 = 4;

fn little_endian_p() -> bool {
    cfg!(target_endian = "little")
}

fn make_float_bigendian(f: &mut f32) {
    if little_endian_p() {
        let bytes = f.to_be_bytes();
        *f = f32::from_be_bytes(bytes);
    }
}

fn make_double_bigendian(d: &mut f64) {
    if little_endian_p() {
        let bytes = d.to_be_bytes();
        *d = f64::from_be_bytes(bytes);
    }
}

static NYBBLE: [&str; 16] = [
    "0000", "0001", "0010", "0011",
    "0100", "0101", "0110", "0111",
    "1000", "1001", "1010", "1011",
    "1100", "1101", "1110", "1111",
];

fn sprint_nybble(i: u8, s: &mut [u8]) {
    let c = NYBBLE[(i & 0x0f) as usize].as_bytes();
    s[..4].copy_from_slice(c);
}

fn sprint_byte(i: u8, s: &mut [u8]) {
    let c = NYBBLE[((i & 0xf0) >> 4) as usize].as_bytes();
    s[..4].copy_from_slice(c);
    let c = NYBBLE[(i & 0x0f) as usize].as_bytes();
    s[4..8].copy_from_slice(c);
}

fn determine_ieee_type(non_zero: bool, exponent: i32, max_exponent: i32) -> i32 {
    if exponent == max_exponent {
        if non_zero {
            GSL_IEEE_TYPE_NAN
        } else {
            GSL_IEEE_TYPE_INF
        }
    } else if exponent == 0 {
        if non_zero {
            GSL_IEEE_TYPE_DENORMAL
        } else {
            GSL_IEEE_TYPE_ZERO
        }
    } else {
        GSL_IEEE_TYPE_NORMAL
    }
}

pub fn gsl_ieee_float_to_rep(x: &f32, r: &mut GslIeeeFloatRep) {
    let mut u = x.to_bits();
    
    if little_endian_p() {
        u = u.to_be();
    }

    r.sign = if (u >> 31) != 0 { 1 } else { 0 };
    
    let e = ((u >> 23) & 0xff) as i32;
    r.exponent = e - 127;

    let mantissa_bits = u & 0x007fffff;
    let bytes = mantissa_bits.to_be_bytes();
    
    sprint_byte((bytes[1] & 0x7f) << 1, &mut r.mantissa[0..8]);
    sprint_byte(bytes[2], &mut r.mantissa[7..15]);
    sprint_byte(bytes[3], &mut r.mantissa[15..23]);
    
    r.mantissa[23] = b'\0';

    let non_zero = bytes[1] != 0 || bytes[2] != 0 || (bytes[3] & 0x7f) != 0;
    r.type_ = determine_ieee_type(non_zero, e, 255);
}

pub fn gsl_ieee_double_to_rep(x: &f64, r: &mut GslIeeeDoubleRep) {
    let mut u = x.to_bits();
    
    if little_endian_p() {
        u = u.to_be();
    }

    r.sign = if (u >> 63) != 0 { 1 } else { 0 };
    
    let e = ((u >> 52) & 0x7ff) as i32;
    r.exponent = e - 1023;

    let mantissa_bits = u & 0x000fffffffffffff;
    let bytes = mantissa_bits.to_be_bytes();
    
    sprint_nybble(bytes[0], &mut r.mantissa[0..4]);
    sprint_byte(bytes[1], &mut r.mantissa[4..12]);
    sprint_byte(bytes[2], &mut r.mantissa[12..20]);
    sprint_byte(bytes[3], &mut r.mantissa[20..28]);
    sprint_byte(bytes[4], &mut r.mantissa[28..36]);
    sprint_byte(bytes[5], &mut r.mantissa[36..44]);
    sprint_byte(bytes[6], &mut r.mantissa[44..52]);
    
    r.mantissa[52] = b'\0';

    let non_zero = bytes[0] != 0 || bytes[1] != 0 || bytes[2] != 0 ||
                   bytes[3] != 0 || bytes[4] != 0 || bytes[5] != 0 ||
                   (bytes[6] & 0x0f) != 0;
    r.type_ = determine_ieee_type(non_zero, e, 2047);
}