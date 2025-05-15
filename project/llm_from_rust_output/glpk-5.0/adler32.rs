pub type Byte = u8;
pub type uInt = u32;
pub type uLong = u64;
pub type Bytef = Byte;

const BASE: uLong = 65521;

pub fn adler32(adler: uLong, buf: &[Bytef]) -> uLong {
    let mut a = adler & 0xFFFF;
    let mut b = (adler >> 16) & 0xFFFF;

    for &byte in buf {
        a = a.wrapping_add(byte as uLong);
        b = b.wrapping_add(a);
    }

    a %= BASE;
    b %= BASE;

    (b << 16) | a
}

pub fn adler32_combine(adler1: uLong, adler2: uLong, len2: uLong) -> uLong {
    let rem = len2 % BASE;
    let mut sum1 = adler1 & 0xFFFF;
    let mut sum2 = rem * sum1;
    sum2 %= BASE;
    sum1 = sum1.wrapping_add((adler2 & 0xFFFF) + BASE - 1);
    sum2 = sum2.wrapping_add(
        ((adler1 >> 16) & 0xFFFF)
            .wrapping_add((adler2 >> 16) & 0xFFFF)
            .wrapping_add(BASE)
            .wrapping_sub(rem),
    );

    if sum1 >= BASE {
        sum1 -= BASE;
    }
    if sum1 >= BASE {
        sum1 -= BASE;
    }
    if sum2 >= (BASE << 1) {
        sum2 -= BASE << 1;
    }
    if sum2 >= BASE {
        sum2 -= BASE;
    }

    (sum2 << 16) | sum1
}

#[no_mangle]
pub extern "C" fn _glp_zlib_adler32(adler: uLong, buf: *const Bytef, len: uInt) -> uLong {
    if buf.is_null() {
        return 1;
    }
    let slice = unsafe { std::slice::from_raw_parts(buf, len as usize) };
    adler32(adler, slice)
}

#[no_mangle]
pub extern "C" fn _glp_zlib_adler32_combine(
    adler1: uLong,
    adler2: uLong,
    len2: i64,
) -> uLong {
    adler32_combine(adler1, adler2, len2 as uLong)
}

#[no_mangle]
pub extern "C" fn _glp_zlib_adler32_combine64(
    adler1: uLong,
    adler2: uLong,
    len2: i64,
) -> uLong {
    adler32_combine(adler1, adler2, len2 as uLong)
}