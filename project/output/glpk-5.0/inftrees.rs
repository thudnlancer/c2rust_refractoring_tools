use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: u8,
    pub bits: u8,
    pub val: libc::c_ushort,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum codetype {
    CODES,
    LENS,
    DISTS,
}
impl codetype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            codetype::CODES => 0,
            codetype::LENS => 1,
            codetype::DISTS => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> codetype {
        match value {
            0 => codetype::CODES,
            1 => codetype::LENS,
            2 => codetype::DISTS,
            _ => panic!("Invalid value for codetype: {}", value),
        }
    }
}
impl AddAssign<u32> for codetype {
    fn add_assign(&mut self, rhs: u32) {
        *self = codetype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for codetype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = codetype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for codetype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = codetype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for codetype {
    fn div_assign(&mut self, rhs: u32) {
        *self = codetype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for codetype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = codetype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for codetype {
    type Output = codetype;
    fn add(self, rhs: u32) -> codetype {
        codetype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for codetype {
    type Output = codetype;
    fn sub(self, rhs: u32) -> codetype {
        codetype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for codetype {
    type Output = codetype;
    fn mul(self, rhs: u32) -> codetype {
        codetype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for codetype {
    type Output = codetype;
    fn div(self, rhs: u32) -> codetype {
        codetype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for codetype {
    type Output = codetype;
    fn rem(self, rhs: u32) -> codetype {
        codetype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[no_mangle]
pub static mut _glp_zlib_inflate_copyright: [i8; 47] = unsafe {
    *::core::mem::transmute::<
        &[u8; 47],
        &[i8; 47],
    >(b" inflate 1.2.5 Copyright 1995-2010 Mark Adler \0")
};
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflate_table(
    mut type_0: codetype,
    mut lens: *mut libc::c_ushort,
    mut codes: u32,
    mut table: *mut *mut code,
    mut bits: *mut u32,
    mut work: *mut libc::c_ushort,
) -> i32 {
    let mut len: u32 = 0;
    let mut sym: u32 = 0;
    let mut min: u32 = 0;
    let mut max: u32 = 0;
    let mut root: u32 = 0;
    let mut curr: u32 = 0;
    let mut drop_0: u32 = 0;
    let mut left: i32 = 0;
    let mut used: u32 = 0;
    let mut huff: u32 = 0;
    let mut incr: u32 = 0;
    let mut fill: u32 = 0;
    let mut low: u32 = 0;
    let mut mask: u32 = 0;
    let mut here: code = code { op: 0, bits: 0, val: 0 };
    let mut next: *mut code = 0 as *mut code;
    let mut base: *const libc::c_ushort = 0 as *const libc::c_ushort;
    let mut extra: *const libc::c_ushort = 0 as *const libc::c_ushort;
    let mut end: i32 = 0;
    let mut count: [libc::c_ushort; 16] = [0; 16];
    let mut offs: [libc::c_ushort; 16] = [0; 16];
    static mut lbase: [libc::c_ushort; 31] = [
        3 as i32 as libc::c_ushort,
        4 as i32 as libc::c_ushort,
        5 as i32 as libc::c_ushort,
        6 as i32 as libc::c_ushort,
        7 as i32 as libc::c_ushort,
        8 as i32 as libc::c_ushort,
        9 as i32 as libc::c_ushort,
        10 as i32 as libc::c_ushort,
        11 as i32 as libc::c_ushort,
        13 as i32 as libc::c_ushort,
        15 as i32 as libc::c_ushort,
        17 as i32 as libc::c_ushort,
        19 as i32 as libc::c_ushort,
        23 as i32 as libc::c_ushort,
        27 as i32 as libc::c_ushort,
        31 as i32 as libc::c_ushort,
        35 as i32 as libc::c_ushort,
        43 as i32 as libc::c_ushort,
        51 as i32 as libc::c_ushort,
        59 as i32 as libc::c_ushort,
        67 as i32 as libc::c_ushort,
        83 as i32 as libc::c_ushort,
        99 as i32 as libc::c_ushort,
        115 as i32 as libc::c_ushort,
        131 as i32 as libc::c_ushort,
        163 as i32 as libc::c_ushort,
        195 as i32 as libc::c_ushort,
        227 as i32 as libc::c_ushort,
        258 as i32 as libc::c_ushort,
        0 as i32 as libc::c_ushort,
        0 as i32 as libc::c_ushort,
    ];
    static mut lext: [libc::c_ushort; 31] = [
        16 as i32 as libc::c_ushort,
        16 as i32 as libc::c_ushort,
        16 as i32 as libc::c_ushort,
        16 as i32 as libc::c_ushort,
        16 as i32 as libc::c_ushort,
        16 as i32 as libc::c_ushort,
        16 as i32 as libc::c_ushort,
        16 as i32 as libc::c_ushort,
        17 as i32 as libc::c_ushort,
        17 as i32 as libc::c_ushort,
        17 as i32 as libc::c_ushort,
        17 as i32 as libc::c_ushort,
        18 as i32 as libc::c_ushort,
        18 as i32 as libc::c_ushort,
        18 as i32 as libc::c_ushort,
        18 as i32 as libc::c_ushort,
        19 as i32 as libc::c_ushort,
        19 as i32 as libc::c_ushort,
        19 as i32 as libc::c_ushort,
        19 as i32 as libc::c_ushort,
        20 as i32 as libc::c_ushort,
        20 as i32 as libc::c_ushort,
        20 as i32 as libc::c_ushort,
        20 as i32 as libc::c_ushort,
        21 as i32 as libc::c_ushort,
        21 as i32 as libc::c_ushort,
        21 as i32 as libc::c_ushort,
        21 as i32 as libc::c_ushort,
        16 as i32 as libc::c_ushort,
        73 as i32 as libc::c_ushort,
        195 as i32 as libc::c_ushort,
    ];
    static mut dbase: [libc::c_ushort; 32] = [
        1 as i32 as libc::c_ushort,
        2 as i32 as libc::c_ushort,
        3 as i32 as libc::c_ushort,
        4 as i32 as libc::c_ushort,
        5 as i32 as libc::c_ushort,
        7 as i32 as libc::c_ushort,
        9 as i32 as libc::c_ushort,
        13 as i32 as libc::c_ushort,
        17 as i32 as libc::c_ushort,
        25 as i32 as libc::c_ushort,
        33 as i32 as libc::c_ushort,
        49 as i32 as libc::c_ushort,
        65 as i32 as libc::c_ushort,
        97 as i32 as libc::c_ushort,
        129 as i32 as libc::c_ushort,
        193 as i32 as libc::c_ushort,
        257 as i32 as libc::c_ushort,
        385 as i32 as libc::c_ushort,
        513 as i32 as libc::c_ushort,
        769 as i32 as libc::c_ushort,
        1025 as i32 as libc::c_ushort,
        1537 as i32 as libc::c_ushort,
        2049 as i32 as libc::c_ushort,
        3073 as i32 as libc::c_ushort,
        4097 as i32 as libc::c_ushort,
        6145 as i32 as libc::c_ushort,
        8193 as i32 as libc::c_ushort,
        12289 as i32 as libc::c_ushort,
        16385 as i32 as libc::c_ushort,
        24577 as i32 as libc::c_ushort,
        0 as i32 as libc::c_ushort,
        0 as i32 as libc::c_ushort,
    ];
    static mut dext: [libc::c_ushort; 32] = [
        16 as i32 as libc::c_ushort,
        16 as i32 as libc::c_ushort,
        16 as i32 as libc::c_ushort,
        16 as i32 as libc::c_ushort,
        17 as i32 as libc::c_ushort,
        17 as i32 as libc::c_ushort,
        18 as i32 as libc::c_ushort,
        18 as i32 as libc::c_ushort,
        19 as i32 as libc::c_ushort,
        19 as i32 as libc::c_ushort,
        20 as i32 as libc::c_ushort,
        20 as i32 as libc::c_ushort,
        21 as i32 as libc::c_ushort,
        21 as i32 as libc::c_ushort,
        22 as i32 as libc::c_ushort,
        22 as i32 as libc::c_ushort,
        23 as i32 as libc::c_ushort,
        23 as i32 as libc::c_ushort,
        24 as i32 as libc::c_ushort,
        24 as i32 as libc::c_ushort,
        25 as i32 as libc::c_ushort,
        25 as i32 as libc::c_ushort,
        26 as i32 as libc::c_ushort,
        26 as i32 as libc::c_ushort,
        27 as i32 as libc::c_ushort,
        27 as i32 as libc::c_ushort,
        28 as i32 as libc::c_ushort,
        28 as i32 as libc::c_ushort,
        29 as i32 as libc::c_ushort,
        29 as i32 as libc::c_ushort,
        64 as i32 as libc::c_ushort,
        64 as i32 as libc::c_ushort,
    ];
    len = 0 as i32 as u32;
    while len <= 15 as i32 as u32 {
        count[len as usize] = 0 as i32 as libc::c_ushort;
        len = len.wrapping_add(1);
        len;
    }
    sym = 0 as i32 as u32;
    while sym < codes {
        count[*lens.offset(sym as isize) as usize] = (count[*lens.offset(sym as isize)
            as usize])
            .wrapping_add(1);
        count[*lens.offset(sym as isize) as usize];
        sym = sym.wrapping_add(1);
        sym;
    }
    root = *bits;
    max = 15 as i32 as u32;
    while max >= 1 as i32 as u32 {
        if count[max as usize] as i32 != 0 as i32 {
            break;
        }
        max = max.wrapping_sub(1);
        max;
    }
    if root > max {
        root = max;
    }
    if max == 0 as i32 as u32 {
        here.op = 64 as i32 as u8;
        here.bits = 1 as i32 as u8;
        here.val = 0 as i32 as libc::c_ushort;
        let fresh0 = *table;
        *table = (*table).offset(1);
        *fresh0 = here;
        let fresh1 = *table;
        *table = (*table).offset(1);
        *fresh1 = here;
        *bits = 1 as i32 as u32;
        return 0 as i32;
    }
    min = 1 as i32 as u32;
    while min < max {
        if count[min as usize] as i32 != 0 as i32 {
            break;
        }
        min = min.wrapping_add(1);
        min;
    }
    if root < min {
        root = min;
    }
    left = 1 as i32;
    len = 1 as i32 as u32;
    while len <= 15 as i32 as u32 {
        left <<= 1 as i32;
        left -= count[len as usize] as i32;
        if left < 0 as i32 {
            return -(1 as i32);
        }
        len = len.wrapping_add(1);
        len;
    }
    if left > 0 as i32
        && (type_0 as u32 == codetype::CODES as i32 as u32 || max != 1 as i32 as u32)
    {
        return -(1 as i32);
    }
    offs[1 as i32 as usize] = 0 as i32 as libc::c_ushort;
    len = 1 as i32 as u32;
    while len < 15 as i32 as u32 {
        offs[len.wrapping_add(1 as i32 as u32) as usize] = (offs[len as usize] as i32
            + count[len as usize] as i32) as libc::c_ushort;
        len = len.wrapping_add(1);
        len;
    }
    sym = 0 as i32 as u32;
    while sym < codes {
        if *lens.offset(sym as isize) as i32 != 0 as i32 {
            let fresh2 = offs[*lens.offset(sym as isize) as usize];
            offs[*lens.offset(sym as isize) as usize] = (offs[*lens.offset(sym as isize)
                as usize])
                .wrapping_add(1);
            *work.offset(fresh2 as isize) = sym as libc::c_ushort;
        }
        sym = sym.wrapping_add(1);
        sym;
    }
    match type_0 as u32 {
        0 => {
            extra = work;
            base = extra;
            end = 19 as i32;
        }
        1 => {
            base = lbase.as_ptr();
            base = base.offset(-(257 as i32 as isize));
            extra = lext.as_ptr();
            extra = extra.offset(-(257 as i32 as isize));
            end = 256 as i32;
        }
        _ => {
            base = dbase.as_ptr();
            extra = dext.as_ptr();
            end = -(1 as i32);
        }
    }
    huff = 0 as i32 as u32;
    sym = 0 as i32 as u32;
    len = min;
    next = *table;
    curr = root;
    drop_0 = 0 as i32 as u32;
    low = -(1 as i32) as u32;
    used = (1 as u32) << root;
    mask = used.wrapping_sub(1 as i32 as u32);
    if type_0 as u32 == codetype::LENS as i32 as u32 && used >= 852 as i32 as u32
        || type_0 as u32 == codetype::DISTS as i32 as u32 && used >= 592 as i32 as u32
    {
        return 1 as i32;
    }
    loop {
        here.bits = len.wrapping_sub(drop_0) as u8;
        if (*work.offset(sym as isize) as i32) < end {
            here.op = 0 as i32 as u8;
            here.val = *work.offset(sym as isize);
        } else if *work.offset(sym as isize) as i32 > end {
            here.op = *extra.offset(*work.offset(sym as isize) as isize) as u8;
            here.val = *base.offset(*work.offset(sym as isize) as isize);
        } else {
            here.op = (32 as i32 + 64 as i32) as u8;
            here.val = 0 as i32 as libc::c_ushort;
        }
        incr = (1 as u32) << len.wrapping_sub(drop_0);
        fill = (1 as u32) << curr;
        min = fill;
        loop {
            fill = fill.wrapping_sub(incr);
            *next.offset((huff >> drop_0).wrapping_add(fill) as isize) = here;
            if !(fill != 0 as i32 as u32) {
                break;
            }
        }
        incr = (1 as u32) << len.wrapping_sub(1 as i32 as u32);
        while huff & incr != 0 {
            incr >>= 1 as i32;
        }
        if incr != 0 as i32 as u32 {
            huff &= incr.wrapping_sub(1 as i32 as u32);
            huff = huff.wrapping_add(incr);
        } else {
            huff = 0 as i32 as u32;
        }
        sym = sym.wrapping_add(1);
        sym;
        count[len as usize] = (count[len as usize]).wrapping_sub(1);
        if count[len as usize] as i32 == 0 as i32 {
            if len == max {
                break;
            }
            len = *lens.offset(*work.offset(sym as isize) as isize) as u32;
        }
        if len > root && huff & mask != low {
            if drop_0 == 0 as i32 as u32 {
                drop_0 = root;
            }
            next = next.offset(min as isize);
            curr = len.wrapping_sub(drop_0);
            left = (1 as i32) << curr;
            while curr.wrapping_add(drop_0) < max {
                left -= count[curr.wrapping_add(drop_0) as usize] as i32;
                if left <= 0 as i32 {
                    break;
                }
                curr = curr.wrapping_add(1);
                curr;
                left <<= 1 as i32;
            }
            used = used.wrapping_add((1 as u32) << curr);
            if type_0 as u32 == codetype::LENS as i32 as u32 && used >= 852 as i32 as u32
                || type_0 as u32 == codetype::DISTS as i32 as u32
                    && used >= 592 as i32 as u32
            {
                return 1 as i32;
            }
            low = huff & mask;
            (*(*table).offset(low as isize)).op = curr as u8;
            (*(*table).offset(low as isize)).bits = root as u8;
            (*(*table).offset(low as isize)).val = next.offset_from(*table) as i64
                as libc::c_ushort;
        }
    }
    here.op = 64 as i32 as u8;
    here.bits = len.wrapping_sub(drop_0) as u8;
    here.val = 0 as i32 as libc::c_ushort;
    while huff != 0 as i32 as u32 {
        if drop_0 != 0 as i32 as u32 && huff & mask != low {
            drop_0 = 0 as i32 as u32;
            len = root;
            next = *table;
            here.bits = len as u8;
        }
        *next.offset((huff >> drop_0) as isize) = here;
        incr = (1 as u32) << len.wrapping_sub(1 as i32 as u32);
        while huff & incr != 0 {
            incr >>= 1 as i32;
        }
        if incr != 0 as i32 as u32 {
            huff &= incr.wrapping_sub(1 as i32 as u32);
            huff = huff.wrapping_add(incr);
        } else {
            huff = 0 as i32 as u32;
        }
    }
    *table = (*table).offset(used as isize);
    *bits = root;
    return 0 as i32;
}