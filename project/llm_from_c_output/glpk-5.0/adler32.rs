// adler32.rs -- compute the Adler-32 checksum of a data stream
// Based on original C code by Mark Adler

const BASE: u32 = 65521; // largest prime smaller than 65536
const NMAX: usize = 5552;
// NMAX is the largest n such that 255n(n+1)/2 + (n+1)(BASE-1) <= 2^32-1

/// Compute Adler-32 checksum
pub fn adler32(adler: u32, buf: &[u8]) -> u32 {
    let mut adler = adler;
    let mut sum2 = (adler >> 16) & 0xffff;
    adler &= 0xffff;

    // Handle empty buffer case
    if buf.is_empty() {
        return 1;
    }

    // Optimize for single byte
    if buf.len() == 1 {
        adler += u32::from(buf[0]);
        if adler >= BASE {
            adler -= BASE;
        }
        sum2 += adler;
        if sum2 >= BASE {
            sum2 -= BASE;
        }
        return adler | (sum2 << 16);
    }

    // Optimize for small buffers
    if buf.len() < 16 {
        for &byte in buf {
            adler += u32::from(byte);
            sum2 += adler;
        }
        if adler >= BASE {
            adler -= BASE;
        }
        sum2 %= BASE;
        return adler | (sum2 << 16);
    }

    let mut buf = buf;
    let mut len = buf.len();

    // Process NMAX-sized blocks
    while len >= NMAX {
        len -= NMAX;
        let n = NMAX / 16;
        for _ in 0..n {
            for i in 0..16 {
                adler += u32::from(buf[i]);
                sum2 += adler;
            }
            buf = &buf[16..];
        }
        adler %= BASE;
        sum2 %= BASE;
    }

    // Process remaining bytes
    if len > 0 {
        while len >= 16 {
            len -= 16;
            for i in 0..16 {
                adler += u32::from(buf[i]);
                sum2 += adler;
            }
            buf = &buf[16..];
        }
        for &byte in buf {
            adler += u32::from(byte);
            sum2 += adler;
        }
        adler %= BASE;
        sum2 %= BASE;
    }

    adler | (sum2 << 16)
}

/// Combine two Adler-32 checksums
pub fn adler32_combine(adler1: u32, adler2: u32, len2: i64) -> u32 {
    let rem = (len2 % i64::from(BASE)) as u32;
    let mut sum1 = adler1 & 0xffff;
    let mut sum2 = rem * sum1;
    sum2 %= BASE;
    sum1 += (adler2 & 0xffff) + BASE - 1;
    sum2 += ((adler1 >> 16) & 0xffff) + ((adler2 >> 16) & 0xffff) + BASE - rem;
    
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
    
    sum1 | (sum2 << 16)
}

/// Combine two Adler-32 checksums (64-bit length variant)
pub fn adler32_combine64(adler1: u32, adler2: u32, len2: i64) -> u32 {
    adler32_combine(adler1, adler2, len2)
}