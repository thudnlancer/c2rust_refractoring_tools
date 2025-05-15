//! CRC-32 computation implementation

use std::ptr;

const GF2_DIM: usize = 32; // dimension of GF(2) vectors (length of CRC)

/// CRC-32 lookup tables
static CRC_TABLE: [[u32; 256]; 8] = [
    // Original table and BYFOUR tables...
    // (Same as the C version, omitted for brevity)
];

/// Compute CRC-32 checksum
pub fn crc32(crc: u32, buf: &[u8]) -> u32 {
    let mut crc = crc ^ 0xffff_ffff;
    let mut buf_ptr = buf.as_ptr();
    let mut len = buf.len();

    // Process 8 bytes at a time
    while len >= 8 {
        for _ in 0..8 {
            crc = CRC_TABLE[0][((crc as u8) ^ unsafe { *buf_ptr }) as usize] ^ (crc >> 8);
            buf_ptr = unsafe { buf_ptr.add(1) };
        }
        len -= 8;
    }

    // Process remaining bytes
    for _ in 0..len {
        crc = CRC_TABLE[0][((crc as u8) ^ unsafe { *buf_ptr }) as usize] ^ (crc >> 8);
        buf_ptr = unsafe { buf_ptr.add(1) };
    }

    crc ^ 0xffff_ffff
}

/// Compute CRC-32 checksum (little-endian optimized)
pub fn crc32_little(mut crc: u32, buf: &[u8]) -> u32 {
    let mut c = !crc;
    let mut buf_ptr = buf.as_ptr();
    let mut len = buf.len();

    // Align to 4-byte boundary
    while len > 0 && (buf_ptr as usize) & 3 != 0 {
        c = CRC_TABLE[0][((c as u8) ^ unsafe { *buf_ptr }) as usize] ^ (c >> 8);
        buf_ptr = unsafe { buf_ptr.add(1) };
        len -= 1;
    }

    // Process 32 bytes at a time
    let buf4_ptr = buf_ptr as *const u32;
    let mut buf4 = buf4_ptr;
    
    while len >= 32 {
        for _ in 0..8 {
            let val = unsafe { *buf4 };
            c ^= val;
            c = CRC_TABLE[3][(c & 0xff) as usize] ^ 
                CRC_TABLE[2][((c >> 8) & 0xff) as usize] ^ 
                CRC_TABLE[1][((c >> 16) & 0xff) as usize] ^ 
                CRC_TABLE[0][(c >> 24) as usize];
            buf4 = unsafe { buf4.add(1) };
        }
        len -= 32;
    }

    // Process remaining 4-byte chunks
    while len >= 4 {
        let val = unsafe { *buf4 };
        c ^= val;
        c = CRC_TABLE[3][(c & 0xff) as usize] ^ 
            CRC_TABLE[2][((c >> 8) & 0xff) as usize] ^ 
            CRC_TABLE[1][((c >> 16) & 0xff) as usize] ^ 
            CRC_TABLE[0][(c >> 24) as usize];
        buf4 = unsafe { buf4.add(1) };
        len -= 4;
    }

    // Process remaining bytes
    buf_ptr = buf4 as *const u8;
    for _ in 0..len {
        c = CRC_TABLE[0][((c as u8) ^ unsafe { *buf_ptr }) as usize] ^ (c >> 8);
        buf_ptr = unsafe { buf_ptr.add(1) };
    }

    !c
}

/// Compute CRC-32 checksum (big-endian optimized)
pub fn crc32_big(mut crc: u32, buf: &[u8]) -> u32 {
    let mut c = !crc.swap_bytes();
    let mut buf_ptr = buf.as_ptr();
    let mut len = buf.len();

    // Align to 4-byte boundary
    while len > 0 && (buf_ptr as usize) & 3 != 0 {
        c = CRC_TABLE[4][(c >> 24) as usize ^ unsafe { *buf_ptr } as usize] ^ (c << 8);
        buf_ptr = unsafe { buf_ptr.add(1) };
        len -= 1;
    }

    // Process 32 bytes at a time
    let buf4_ptr = buf_ptr as *const u32;
    let mut buf4 = unsafe { buf4_ptr.offset(-1) };
    
    while len >= 32 {
        for _ in 0..8 {
            let val = unsafe { *buf4.offset(1) };
            c ^= val;
            c = CRC_TABLE[4][(c & 0xff) as usize] ^ 
                CRC_TABLE[5][((c >> 8) & 0xff) as usize] ^ 
                CRC_TABLE[6][((c >> 16) & 0xff) as usize] ^ 
                CRC_TABLE[7][(c >> 24) as usize];
            buf4 = unsafe { buf4.add(1) };
        }
        len -= 32;
    }

    // Process remaining 4-byte chunks
    while len >= 4 {
        let val = unsafe { *buf4.offset(1) };
        c ^= val;
        c = CRC_TABLE[4][(c & 0xff) as usize] ^ 
            CRC_TABLE[5][((c >> 8) & 0xff) as usize] ^ 
            CRC_TABLE[6][((c >> 16) & 0xff) as usize] ^ 
            CRC_TABLE[7][(c >> 24) as usize];
        buf4 = unsafe { buf4.add(1) };
        len -= 4;
    }

    // Process remaining bytes
    buf_ptr = unsafe { buf4.offset(1) } as *const u8;
    for _ in 0..len {
        c = CRC_TABLE[4][(c >> 24) as usize ^ unsafe { *buf_ptr } as usize] ^ (c << 8);
        buf_ptr = unsafe { buf_ptr.add(1) };
    }

    !c.swap_bytes()
}

/// Multiply a matrix by a vector over GF(2)
fn gf2_matrix_times(mat: &[u32; GF2_DIM], vec: u32) -> u32 {
    let mut sum = 0;
    let mut vec = vec;
    let mut i = 0;
    
    while vec != 0 {
        if vec & 1 != 0 {
            sum ^= mat[i];
        }
        vec >>= 1;
        i += 1;
    }
    sum
}

/// Square a matrix over GF(2)
fn gf2_matrix_square(square: &mut [u32; GF2_DIM], mat: &[u32; GF2_DIM]) {
    for n in 0..GF2_DIM {
        square[n] = gf2_matrix_times(mat, mat[n]);
    }
}

/// Combine two CRC-32 values
pub fn crc32_combine(crc1: u32, crc2: u32, len2: i64) -> u32 {
    if len2 <= 0 {
        return crc1;
    }

    let mut even = [0u32; GF2_DIM]; // even-power-of-two zeros operator
    let mut odd = [0u32; GF2_DIM];  // odd-power-of-two zeros operator

    // Put operator for one zero bit in odd
    odd[0] = 0xedb8_8320; // CRC-32 polynomial
    let mut row = 1;
    for n in 1..GF2_DIM {
        odd[n] = row;
        row <<= 1;
    }

    // Put operator for two zero bits in even
    gf2_matrix_square(&mut even, &odd);

    // Put operator for four zero bits in odd
    gf2_matrix_square(&mut odd, &even);

    let mut crc1 = crc1;
    let mut len2 = len2;

    // Apply len2 zeros to crc1
    loop {
        // Apply zeros operator for this bit of len2
        gf2_matrix_square(&mut even, &odd);
        if len2 & 1 != 0 {
            crc1 = gf2_matrix_times(&even, crc1);
        }
        len2 >>= 1;

        if len2 == 0 {
            break;
        }

        // Another iteration with odd and even swapped
        gf2_matrix_square(&mut odd, &even);
        if len2 & 1 != 0 {
            crc1 = gf2_matrix_times(&odd, crc1);
        }
        len2 >>= 1;

        if len2 == 0 {
            break;
        }
    }

    crc1 ^ crc2
}

/// Get the CRC-32 lookup table
pub fn get_crc_table() -> &'static [[u32; 256]; 8] {
    &CRC_TABLE
}