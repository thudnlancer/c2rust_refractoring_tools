// umac-poly128.rs

// Copyright (C) 2013 Niels Möller
// Translated to Rust while maintaining original functionality and safety

const UMAC_P128_HI: u64 = 0xFFFFFFFF00000001;
const UMAC_P128_LO: u64 = 0x0000000000000001;
const UMAC_P128_OFFSET: u64 = 0x0000000100000000;

fn hi(x: u64) -> u64 {
    x >> 32
}

fn lo(x: u64) -> u64 {
    x & 0xffffffff
}

fn poly128_mul(k: &[u32; 4], y: &mut [u64; 2]) {
    let y0 = lo(y[1]);
    let y1 = hi(y[1]);
    let y2 = lo(y[0]);
    let y3 = hi(y[0]);

    let k0 = k[0] as u64;
    let k1 = k[1] as u64;
    let k2 = k[2] as u64;
    let k3 = k[3] as u64;

    let p0 = y0 * k3;
    let m0 = y0 * k2 + y1 * k3;
    let p1 = y0 * k1 + y1 * k2 + y2 * k3;
    let m1 = y0 * k0 + y1 * k1 + y2 * k2 + y3 * k3;
    let p2 = y1 * k0 + y2 * k1 + y3 * k2;
    let m2 = y2 * k0 + y3 * k1;
    let p3 = y3 * k0;

    // Collaps to 4 64-bit words
    // +---+---+---+---+
    // | p3| p2| p1| p0|
    // +-+-+-+-+-+-+-+-+
    // +  | m2| m1| m0|
    // -+-+-+-+-+-+-+-+-+
    
    // Reduce (p3,p2,p1,p0) and (m2,m1,m0) mod p first
    let mut m1 = m1 + UMAC_P128_OFFSET * hi(p3);
    let mut p1 = p1 + UMAC_P128_OFFSET * (lo(p3) + hi(m2));
    let mut m0 = m0 + UMAC_P128_OFFSET * (hi(p2) + lo(m2));
    let mut p0 = p0 + UMAC_P128_OFFSET * (lo(p2) + hi(m1));

    // Left to add
    // +---+---+
    // | p1| p0|
    // +-+-+-+-+
    // m1| m0|
    // +-+---+
    
    // First add high parts, with no possibilities for carries
    p1 += m0 >> 32;

    m0 <<= 32;
    m1 <<= 32;

    // Remains:
    // +---+---+
    // | p1| p0|
    // +-+-+---+
    // +| m1| m0|
    // -+---+---+
    p0 += m0;
    p1 += if p0 < m0 { 1 } else { 0 };
    p1 += m1;
    if p1 < m1 {
        p0 += UMAC_P128_OFFSET;
        p1 += if p0 < UMAC_P128_OFFSET { 1 } else { 0 };
    }

    y[0] = p1;
    y[1] = p0;
}

pub fn umac_poly128(k: &[u32; 4], y: &mut [u64; 2], mh: u64, ml: u64) {
    if (mh >> 32) == 0xffffffff {
        poly128_mul(k, y);
        if y[1] > 0 {
            y[1] -= 1;
        } else if y[0] > 0 {
            y[0] -= 1;
            y[1] = UMAC_P128_HI;
        } else {
            y[0] = UMAC_P128_HI;
            y[1] = UMAC_P128_LO - 1;
        }

        let new_mh = mh - if ml < UMAC_P128_OFFSET { 1 } else { 0 };
        let new_ml = ml - UMAC_P128_OFFSET;
        
        assert!(new_mh < UMAC_P128_HI || new_ml < UMAC_P128_LO);
        poly128_mul(k, y);
        
        let yl = y[1].wrapping_add(new_ml);
        let cy = if yl < new_ml { 1 } else { 0 };
        let mut yh = y[0].wrapping_add(cy);
        let mut cy = if yh < cy { 1 } else { 0 };
        yh = yh.wrapping_add(new_mh);
        cy += if yh < new_mh { 1 } else { 0 };
        
        assert!(cy <= 1);
        if cy != 0 {
            let new_yl = yl.wrapping_add(UMAC_P128_OFFSET);
            yh += if new_yl < UMAC_P128_OFFSET { 1 } else { 0 };
            y[0] = yh;
            y[1] = new_yl;
        } else {
            y[0] = yh;
            y[1] = yl;
        }
        return;
    }

    assert!(mh < UMAC_P128_HI || ml < UMAC_P128_LO);
    poly128_mul(k, y);
    
    let yl = y[1].wrapping_add(ml);
    let cy = if yl < ml { 1 } else { 0 };
    let mut yh = y[0].wrapping_add(cy);
    let mut cy = if yh < cy { 1 } else { 0 };
    yh = yh.wrapping_add(mh);
    cy += if yh < mh { 1 } else { 0 };
    
    assert!(cy <= 1);
    if cy != 0 {
        let new_yl = yl.wrapping_add(UMAC_P128_OFFSET);
        yh += if new_yl < UMAC_P128_OFFSET { 1 } else { 0 };
        y[0] = yh;
        y[1] = new_yl;
    } else {
        y[0] = yh;
        y[1] = yl;
    }
}