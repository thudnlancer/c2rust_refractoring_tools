pub type uint32_t = u32;
pub type uint64_t = u64;

fn poly64_mul(kh: uint32_t, kl: uint32_t, y: uint64_t) -> uint64_t {
    let yl = y & 0xffffffff_u64;
    let yh = y >> 32;
    
    let pl = yl.wrapping_mul(kl as u64);
    let ph = yh.wrapping_mul(kh as u64);
    
    let ml = yh.wrapping_mul(kl as u64)
        .wrapping_add(yl.wrapping_mul(kh as u64));
    let mh = ml >> 32;
    let ml = ml << 32;
    
    let pl = pl.wrapping_add(ml);
    let ph = ph.wrapping_add(mh.wrapping_add((pl < ml) as u64));
    
    assert!(ph < (1_u64 << 57), "ph < ((uint64_t) 1 << 57)");
    
    let ph = ph.wrapping_mul(59);
    let mut pl = pl.wrapping_add(ph);
    
    if pl < ph {
        pl = pl.wrapping_add(59);
    }
    
    pl
}

#[no_mangle]
pub fn _nettle_umac_poly64(kh: uint32_t, kl: uint32_t, mut y: uint64_t, mut m: uint64_t) -> uint64_t {
    if m >> 32 == 0xffffffff_u64 {
        y = poly64_mul(kh, kl, y);
        y = if y == 0 {
            (!59_u64).wrapping_sub(1)
        } else {
            y.wrapping_sub(1)
        };
        m = m.wrapping_sub(59);
    }
    
    y = poly64_mul(kh, kl, y);
    y = y.wrapping_add(m);
    
    if y < m {
        y = y.wrapping_add(59);
    }
    
    y
}