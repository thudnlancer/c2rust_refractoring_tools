#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type Byte = u8;
pub type uInt = u32;
pub type uLong = u64;
pub type Bytef = Byte;
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_adler32(
    mut adler: uLong,
    mut buf: *const Bytef,
    mut len: uInt,
) -> uLong {
    let mut sum2: u64 = 0;
    let mut n: u32 = 0;
    sum2 = adler >> 16 as i32 & 0xffff as i32 as u64;
    adler &= 0xffff as i32 as u64;
    if len == 1 as i32 as u32 {
        adler = (adler as u64).wrapping_add(*buf.offset(0 as i32 as isize) as u64)
            as uLong as uLong;
        if adler >= 65521 as u64 {
            adler = (adler as u64).wrapping_sub(65521 as u64) as uLong as uLong;
        }
        sum2 = sum2.wrapping_add(adler);
        if sum2 >= 65521 as u64 {
            sum2 = sum2.wrapping_sub(65521 as u64);
        }
        return adler | sum2 << 16 as i32;
    }
    if buf.is_null() {
        return 1 as i64 as uLong;
    }
    if len < 16 as i32 as u32 {
        loop {
            let fresh0 = len;
            len = len.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            let fresh1 = buf;
            buf = buf.offset(1);
            adler = (adler as u64).wrapping_add(*fresh1 as u64) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
        }
        if adler >= 65521 as u64 {
            adler = (adler as u64).wrapping_sub(65521 as u64) as uLong as uLong;
        }
        sum2 = sum2.wrapping_rem(65521 as u64);
        return adler | sum2 << 16 as i32;
    }
    while len >= 5552 as i32 as u32 {
        len = (len as u32).wrapping_sub(5552 as i32 as u32) as uInt as uInt;
        n = (5552 as i32 / 16 as i32) as u32;
        loop {
            adler = (adler as u64).wrapping_add(*buf.offset(0 as i32 as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(*buf.offset((0 as i32 + 1 as i32) as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(*buf.offset((0 as i32 + 2 as i32) as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((0 as i32 + 2 as i32 + 1 as i32) as isize) as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(*buf.offset((0 as i32 + 4 as i32) as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((0 as i32 + 4 as i32 + 1 as i32) as isize) as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((0 as i32 + 4 as i32 + 2 as i32) as isize) as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((0 as i32 + 4 as i32 + 2 as i32 + 1 as i32) as isize)
                        as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64).wrapping_add(*buf.offset(8 as i32 as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(*buf.offset((8 as i32 + 1 as i32) as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(*buf.offset((8 as i32 + 2 as i32) as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((8 as i32 + 2 as i32 + 1 as i32) as isize) as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(*buf.offset((8 as i32 + 4 as i32) as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((8 as i32 + 4 as i32 + 1 as i32) as isize) as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((8 as i32 + 4 as i32 + 2 as i32) as isize) as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((8 as i32 + 4 as i32 + 2 as i32 + 1 as i32) as isize)
                        as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            buf = buf.offset(16 as i32 as isize);
            n = n.wrapping_sub(1);
            if !(n != 0) {
                break;
            }
        }
        adler = (adler as u64).wrapping_rem(65521 as u64) as uLong as uLong;
        sum2 = sum2.wrapping_rem(65521 as u64);
    }
    if len != 0 {
        while len >= 16 as i32 as u32 {
            len = (len as u32).wrapping_sub(16 as i32 as u32) as uInt as uInt;
            adler = (adler as u64).wrapping_add(*buf.offset(0 as i32 as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(*buf.offset((0 as i32 + 1 as i32) as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(*buf.offset((0 as i32 + 2 as i32) as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((0 as i32 + 2 as i32 + 1 as i32) as isize) as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(*buf.offset((0 as i32 + 4 as i32) as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((0 as i32 + 4 as i32 + 1 as i32) as isize) as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((0 as i32 + 4 as i32 + 2 as i32) as isize) as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((0 as i32 + 4 as i32 + 2 as i32 + 1 as i32) as isize)
                        as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64).wrapping_add(*buf.offset(8 as i32 as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(*buf.offset((8 as i32 + 1 as i32) as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(*buf.offset((8 as i32 + 2 as i32) as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((8 as i32 + 2 as i32 + 1 as i32) as isize) as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(*buf.offset((8 as i32 + 4 as i32) as isize) as u64)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((8 as i32 + 4 as i32 + 1 as i32) as isize) as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((8 as i32 + 4 as i32 + 2 as i32) as isize) as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as u64)
                .wrapping_add(
                    *buf.offset((8 as i32 + 4 as i32 + 2 as i32 + 1 as i32) as isize)
                        as u64,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            buf = buf.offset(16 as i32 as isize);
        }
        loop {
            let fresh2 = len;
            len = len.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            let fresh3 = buf;
            buf = buf.offset(1);
            adler = (adler as u64).wrapping_add(*fresh3 as u64) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
        }
        adler = (adler as u64).wrapping_rem(65521 as u64) as uLong as uLong;
        sum2 = sum2.wrapping_rem(65521 as u64);
    }
    return adler | sum2 << 16 as i32;
}
unsafe extern "C" fn adler32_combine_(
    mut adler1: uLong,
    mut adler2: uLong,
    mut len2: i64,
) -> uLong {
    let mut sum1: u64 = 0;
    let mut sum2: u64 = 0;
    let mut rem: u32 = 0;
    rem = (len2 as u64).wrapping_rem(65521 as u64) as u32;
    sum1 = adler1 & 0xffff as i32 as u64;
    sum2 = (rem as u64).wrapping_mul(sum1);
    sum2 = sum2.wrapping_rem(65521 as u64);
    sum1 = sum1
        .wrapping_add(
            (adler2 & 0xffff as i32 as u64)
                .wrapping_add(65521 as u64)
                .wrapping_sub(1 as i32 as u64),
        );
    sum2 = sum2
        .wrapping_add(
            (adler1 >> 16 as i32 & 0xffff as i32 as u64)
                .wrapping_add(adler2 >> 16 as i32 & 0xffff as i32 as u64)
                .wrapping_add(65521 as u64)
                .wrapping_sub(rem as u64),
        );
    if sum1 >= 65521 as u64 {
        sum1 = sum1.wrapping_sub(65521 as u64);
    }
    if sum1 >= 65521 as u64 {
        sum1 = sum1.wrapping_sub(65521 as u64);
    }
    if sum2 >= (65521 as u64) << 1 as i32 {
        sum2 = sum2.wrapping_sub((65521 as u64) << 1 as i32);
    }
    if sum2 >= 65521 as u64 {
        sum2 = sum2.wrapping_sub(65521 as u64);
    }
    return sum1 | sum2 << 16 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_adler32_combine(
    mut adler1: uLong,
    mut adler2: uLong,
    mut len2: i64,
) -> uLong {
    return adler32_combine_(adler1, adler2, len2);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_adler32_combine64(
    mut adler1: uLong,
    mut adler2: uLong,
    mut len2: i64,
) -> uLong {
    return adler32_combine_(adler1, adler2, len2);
}