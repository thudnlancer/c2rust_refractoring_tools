use std::assert;

fn bigmul(n: i32, m: i32, x: &mut [u16], y: &[u16]) {
    assert!(n >= 1, "n >= 1");
    assert!(m >= 1, "m >= 1");
    
    for j in 0..m as usize {
        x[j] = 0;
    }
    
    for i in 0..n as usize {
        if x[i + m as usize] != 0 {
            let mut t = 0u32;
            for j in 0..m as usize {
                t += x[i + m as usize] as u32 * y[j] as u32 + x[i + j] as u32;
                x[i + j] = t as u16;
                t >>= 16;
            }
            x[i + m as usize] = t as u16;
        }
    }
}

fn bigdiv(n: i32, m: i32, x: &mut [u16], y: &mut [u16]) {
    assert!(n >= 0, "n >= 0");
    assert!(m >= 1, "m >= 1");
    assert!(y[m as usize - 1] != 0, "y[m-1] != 0");
    
    if m == 1 {
        let mut d = 0u16;
        for i in (0..=n as usize).rev() {
            let t = (d as u32) << 16 | x[i] as u32;
            x[i + 1] = (t / y[0] as u32) as u16;
            d = (t % y[0] as u32) as u16;
        }
        x[0] = d;
    } else {
        let d = (0x10000u32 / (y[m as usize - 1] as u32 + 1)) as u16;
        
        if d == 1 {
            x[n as usize + m as usize] = 0;
        } else {
            let mut t = 0u32;
            for i in 0..(n + m) as usize {
                t += x[i] as u32 * d as u32;
                x[i] = t as u16;
                t >>= 16;
            }
            x[n as usize + m as usize] = t as u16;
            
            t = 0;
            for j in 0..m as usize {
                t += y[j] as u32 * d as u32;
                y[j] = t as u16;
                t >>= 16;
            }
        }
        
        for i in (0..=n as usize).rev() {
            let (mut q, mut r) = if x[i + m as usize] < y[m as usize - 1] {
                let t = (x[i + m as usize] as u32) << 16 | x[i + m as usize - 1] as u32;
                (
                    (t / y[m as usize - 1] as u32) as u16,
                    (t % y[m as usize - 1] as u32) as u16,
                )
            } else {
                (0, x[i + m as usize - 1])
            };
            
            loop {
                if q == 0 {
                    x[i + m as usize] = q;
                    break;
                }
                
                let t = (y[m as usize - 2] as u32) * (q as u32);
                if (t >> 16) as u16 > r || 
                   ((t >> 16) as u16 == r && (t as u16) > x[i + m as usize - 2]) {
                    q -= 1;
                    r = (r as u32 + y[m as usize - 1] as u32) as u16;
                    if r as u32 <= 0xFFFF {
                        continue;
                    }
                }
                
                let mut t = 0u32;
                let mut borrow = false;
                for j in 0..m as usize {
                    t += y[j] as u32 * q as u32;
                    if x[i + j] < t as u16 {
                        t += 0x10000;
                    }
                    x[i + j] = x[i + j].wrapping_sub(t as u16);
                    t >>= 16;
                }
                
                if x[i + m as usize] >= t as u16 {
                    x[i + m as usize] = x[i + m as usize].wrapping_sub(t as u16);
                    break;
                }
                
                q -= 1;
                t = 0;
                for j in 0..m as usize {
                    t += x[i + j] as u32 + y[j] as u32;
                    x[i + j] = t as u16;
                    t >>= 16;
                }
            }
        }
        
        if d > 1 {
            let mut t = 0u32;
            for i in (0..m as usize).rev() {
                t = (t << 16) | x[i] as u32;
                x[i] = (t / d as u32) as u16;
                t %= d as u32;
            }
            
            t = 0;
            for j in (0..m as usize).rev() {
                t = (t << 16) | y[j] as u32;
                y[j] = (t / d as u32) as u16;
                t %= d as u32;
            }
        }
    }
}