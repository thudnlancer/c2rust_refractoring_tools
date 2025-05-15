use std::num::Wrapping;

const UMAC_P128_LO: u64 = 159;
const UMAC_P128_HI: u64 = !0;

#[derive(Debug, Clone, Copy)]
struct Poly128State {
    y0: u64,
    y1: u64,
}

fn poly128_mul(k: &[u32; 4], y: &mut Poly128State) {
    let y0 = y.y1 & 0xffffffff;
    let y1 = y.y1 >> 32;
    let y2 = y.y0 & 0xffffffff;
    let y3 = y.y0 >> 32;

    let k0 = Wrapping(k[0] as u64);
    let k1 = Wrapping(k[1] as u64);
    let k2 = Wrapping(k[2] as u64);
    let k3 = Wrapping(k[3] as u64);

    let p0 = Wrapping(y0) * k3;
    let m0 = Wrapping(y0) * k2 + Wrapping(y1) * k3;
    let p1 = Wrapping(y0) * k1 + Wrapping(y1) * k2 + Wrapping(y2) * k3;
    let m1 = Wrapping(y0) * k0 + Wrapping(y1) * k1 + Wrapping(y2) * k2 + Wrapping(y3) * k3;
    let p2 = Wrapping(y1) * k0 + Wrapping(y2) * k1 + Wrapping(y3) * k2;
    let m2 = Wrapping(y2) * k0 + Wrapping(y3) * k1;
    let p3 = Wrapping(y3) * k0;

    let m1 = m1 + Wrapping(UMAC_P128_LO) * Wrapping(p3.0 >> 32);
    let p1 = p1 + Wrapping(UMAC_P128_LO) * Wrapping((p3.0 & 0xffffffff) + (m2.0 >> 32));
    let m0 = m0 + Wrapping(UMAC_P128_LO) * Wrapping((p2.0 >> 32) + (m2.0 & 0xffffffff));
    let p0 = p0 + Wrapping(UMAC_P128_LO) * Wrapping((p2.0 & 0xffffffff) + (m1.0 >> 32));

    let p1 = p1 + Wrapping(m0.0 >> 32);
    let m0 = Wrapping(m0.0 << 32);
    let m1 = Wrapping(m1.0 << 32);

    let p0 = p0 + m0;
    let p1 = p1 + Wrapping((p0 < m0) as u64) + m1;

    let (mut y0, mut y1) = (p0.0, p1.0);
    if p1 < m1 {
        y0 += UMAC_P128_LO;
        y1 += (y0 < UMAC_P128_LO) as u64;
    }

    *y = Poly128State { y0: y1, y1: y0 };
}

pub fn umac_poly128(k: &[u32; 4], y: &mut Poly128State, mh: u64, ml: u64) {
    assert!(mh < UMAC_P128_HI || ml < UMAC_P128_LO.wrapping_neg());

    if mh >> 32 == 0xffffffff {
        poly128_mul(k, y);
        
        if y.y1 > 0 {
            y.y1 -= 1;
        } else if y.y0 > 0 {
            y.y0 -= 1;
            y.y1 = !0;
        } else {
            y.y0 = !0;
            y.y1 = UMAC_P128_LO.wrapping_neg().wrapping_sub(1);
        }

        let mh = mh - (ml < UMAC_P128_LO) as u64;
        let ml = ml - UMAC_P128_LO;
    }

    poly128_mul(k, y);

    let mut yl = y.y1.wrapping_add(ml);
    let mut cy = (yl < ml) as u64;
    let mut yh = y.y0.wrapping_add(cy);
    cy = (yh < cy) as u64;
    yh = yh.wrapping_add(mh);
    cy += (yh < mh) as u64;

    assert!(cy <= 1);

    if cy != 0 {
        yl = yl.wrapping_add(UMAC_P128_LO);
        yh = yh.wrapping_add((yl < UMAC_P128_LO) as u64);
    }

    *y = Poly128State { y0: yh, y1: yl };
}