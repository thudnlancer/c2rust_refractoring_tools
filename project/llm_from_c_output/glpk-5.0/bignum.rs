//! BigNum arithmetic operations (multiplication and division) for unsigned integers of arbitrary precision.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BigNumError {
    details: String,
}

impl BigNumError {
    fn new(msg: &str) -> BigNumError {
        BigNumError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for BigNumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for BigNumError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Multiplies two unsigned integer numbers of arbitrary precision.
///
/// # Arguments
/// * `n` - Number of digits in multiplicand (must be >= 1)
/// * `m` - Number of digits in multiplier (must be >= 1)
/// * `x` - Array containing digits of multiplicand in x[m..m+n], other elements ignored on entry
/// * `y` - Array containing digits of multiplier in y[0..m]
///
/// # Returns
/// The product is stored in x[0..n+m]. The array y is unchanged.
pub fn bigmul(
    n: usize,
    m: usize,
    x: &mut [u16],
    y: &[u16],
) -> Result<(), BigNumError> {
    if n < 1 || m < 1 {
        return Err(BigNumError::new("n and m must be >= 1"));
    }
    if x.len() < n + m || y.len() < m {
        return Err(BigNumError::new("arrays too small"));
    }

    // Initialize lower part of x to zero
    for j in 0..m {
        x[j] = 0;
    }

    for i in 0..n {
        if x[i + m] != 0 {
            let mut t: u32 = 0;
            for j in 0..m {
                t += u32::from(x[i + m]) * u32::from(y[j]) + u32::from(x[i + j]);
                x[i + j] = (t & 0xFFFF) as u16;
                t >>= 16;
            }
            x[i + m] = t as u16;
        }
    }

    Ok(())
}

/// Divides one unsigned integer number of arbitrary precision by another.
///
/// # Arguments
/// * `n` - Difference between dividend and divisor digit counts (must be >= 0)
/// * `m` - Number of digits in divisor (must be >= 1)
/// * `x` - Array containing digits of dividend in x[0..n+m]
/// * `y` - Array containing digits of divisor in y[0..m] (y[m-1] must be non-zero)
///
/// # Returns
/// On exit, quotient is stored in x[m..n+m+1] and remainder in x[0..m].
/// The array y is changed but then restored.
pub fn bigdiv(
    n: usize,
    m: usize,
    x: &mut [u16],
    y: &mut [u16],
) -> Result<(), BigNumError> {
    if n == 0 && m < 1 {
        return Err(BigNumError::new("n must be >= 0 and m >= 1"));
    }
    if y[m - 1] == 0 {
        return Err(BigNumError::new("highest divisor digit must be non-zero"));
    }
    if x.len() < n + m || y.len() < m {
        return Err(BigNumError::new("arrays too small"));
    }

    // Special case when divisor has only one digit
    if m == 1 {
        let mut d: u16 = 0;
        for i in (0..=n).rev() {
            let t = (u32::from(d) << 16) + u32::from(x[i]);
            x[i + 1] = (t / u32::from(y[0])) as u16;
            d = (t % u32::from(y[0])) as u16;
        }
        x[0] = d;
        return Ok(());
    }

    // Multiply dividend and divisor by normalizing coefficient
    let d = (0x10000 / (u32::from(y[m - 1]) + 1)) as u16;
    let mut t: u32;
    
    if d == 1 {
        if x.len() > n + m {
            x[n + m] = 0;
        }
    } else {
        t = 0;
        for i in 0..n + m {
            t += u32::from(x[i]) * u32::from(d);
            x[i] = (t & 0xFFFF) as u16;
            t >>= 16;
        }
        if x.len() > n + m {
            x[n + m] = t as u16;
        }

        t = 0;
        for j in 0..m {
            t += u32::from(y[j]) * u32::from(d);
            y[j] = (t & 0xFFFF) as u16;
            t >>= 16;
        }
    }

    // Main loop
    for i in (0..=n).rev() {
        let (q, r) = if x[i + m] < y[m - 1] {
            let t = (u32::from(x[i + m]) << 16) + u32::from(x[i + m - 1]);
            let q = (t / u32::from(y[m - 1])) as u16;
            let r = (t % u32::from(y[m - 1])) as u16;
            if q == 0 {
                (0, 0)
            } else {
                (q, r)
            }
        } else {
            (0, x[i + m - 1])
        };

        let (mut q, mut r) = (q, r);
        
        // Correction loop
        loop {
            if q == 0 && r == 0 {
                break;
            }
            
            let test = u32::from(y[m - 2]) * u32::from(q);
            if (test >> 16) as u16 > r {
                q = q.wrapping_sub(1);
                r = r.wrapping_add(y[m - 1]);
                continue;
            }
            if (test >> 16) as u16 < r {
                break;
            }
            if (test as u16) > x[i + m - 2] {
                q = q.wrapping_sub(1);
                r = r.wrapping_add(y[m - 1]);
                continue;
            }
            break;
        }

        // Subtract divisor multiplied by current quotient digit
        if q != 0 {
            t = 0;
            for j in 0..m {
                t += u32::from(y[j]) * u32::from(q);
                if x[i + j] < (t & 0xFFFF) as u16 {
                    t += 0x10000;
                }
                x[i + j] = x[i + j].wrapping_sub((t & 0xFFFF) as u16);
                t >>= 16;
            }
            
            if x[i + m] < (t & 0xFFFF) as u16 {
                // Correcting addition
                q = q.wrapping_sub(1);
                t = 0;
                for j in 0..m {
                    t += u32::from(x[i + j]) + u32::from(y[j]);
                    x[i + j] = (t & 0xFFFF) as u16;
                    t >>= 16;
                }
            }
        }

        // Store quotient digit
        if x.len() > i + m {
            x[i + m] = q;
        }
    }

    // Divide divisor and remainder by normalizing coefficient
    if d > 1 {
        t = 0;
        for i in (0..m).rev() {
            t = (t << 16) + u32::from(x[i]);
            x[i] = (t / u32::from(d)) as u16;
            t %= u32::from(d);
        }

        t = 0;
        for j in (0..m).rev() {
            t = (t << 16) + u32::from(y[j]);
            y[j] = (t / u32::from(d)) as u16;
            t %= u32::from(d);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    const N_MAX: usize = 7;
    const M_MAX: usize = 5;
    const N_TEST: usize = 1000;

    #[test]
    fn test_big_ops() {
        let mut rng = rand::thread_rng();
        
        for _ in 0..N_TEST {
            // Test multiplication and division
            
            // Generate multiplicand
            let n = rng.gen_range(1..=N_MAX);
            let mut x = vec![0u16; N_MAX + M_MAX];
            for j in 0..n {
                x[M_MAX + j] = rng.gen_range(0..=65535);
            }
            
            // Generate multiplier
            let m = rng.gen_range(1..=M_MAX);
            let mut y = vec![0u16; m];
            for j in 0..m {
                y[j] = rng.gen_range(0..=65535);
            }
            if y[m - 1] == 0 {
                y[m - 1] = 1;
            }
            
            // Multiply
            let mut z = x.clone();
            bigmul(n, m, &mut z, &y).unwrap();
            
            // Divide
            bigdiv(n, m, &mut z, &mut y.clone()).unwrap();
            
            // Check remainder is zero
            for j in 0..m {
                assert_eq!(z[j], 0);
            }
            
            // Check quotient matches original x
            for j in 0..n {
                assert_eq!(z[m + j], x[M_MAX + j]);
            }
        }
    }
}