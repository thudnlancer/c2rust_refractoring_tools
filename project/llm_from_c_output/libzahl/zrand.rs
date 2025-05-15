use std::fs::File;
use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{Rng, SeedableRng, rngs::StdRng};
use rand::distributions::Uniform;
use std::os::unix::io::AsRawFd;

const FAST_RANDOM_PATHNAME: &str = "/dev/urandom";
const SECURE_RANDOM_PATHNAME: &str = "/dev/random";

enum ZRandDev {
    FastRandom,
    SecureRandom,
    LibcRandRandom,
    DefaultRandom,
    FastestRandom,
    LibcRandomRandom,
    LibcRand48Random,
}

enum ZRandDist {
    QuasiUniform,
    Uniform,
    ModUniform,
}

struct Z {
    chars: Vec<u64>,
    used: usize,
    signum: i32,
}

impl Z {
    fn new() -> Self {
        Z {
            chars: Vec::new(),
            used: 0,
            signum: 0,
        }
    }

    fn set_signum(&mut self, signum: i32) {
        self.signum = signum;
    }

    fn zero(&self) -> bool {
        self.signum == 0
    }

    fn negative(&self) -> bool {
        self.signum < 0
    }

    fn cmp_mag(&self, other: &Z) -> i32 {
        if self.used > other.used {
            1
        } else if self.used < other.used {
            -1
        } else {
            for i in (0..self.used).rev() {
                if self.chars[i] > other.chars[i] {
                    return 1;
                } else if self.chars[i] < other.chars[i] {
                    return -1;
                }
            }
            0
        }
    }

    fn add(&mut self, a: &Z, b: &Z) {
        // Implementation of addition
    }

    fn mul(&mut self, a: &Z, b: &Z) {
        // Implementation of multiplication
    }

    fn rsh(&mut self, a: &Z, bits: usize) {
        // Implementation of right shift
    }

    fn sub(&mut self, a: &Z, b: &Z) {
        // Implementation of subtraction
    }
}

fn zrand_libc_rand(out: &mut [u8], statep: Option<&mut StdRng>) {
    let mut rng = match statep {
        Some(r) => r,
        None => {
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let mut r = StdRng::seed_from_u64(seed);
            &mut r
        }
    };

    for byte in out.iter_mut().rev() {
        let ri = rng.gen::<u32>();
        let rd = f64::from(ri) / (f64::from(u32::MAX) + 1.0);
        *byte = (rd * 256.0) as u8;
    }
}

fn zrand_libc_rand48(out: &mut [u8], statep: Option<&mut StdRng>) {
    let mut rng = match statep {
        Some(r) => r,
        None => {
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let mut r = StdRng::seed_from_u64(seed);
            &mut r
        }
    };

    for byte in out.iter_mut().rev() {
        let r0 = rng.gen::<u8>() & 0x0F;
        let r1 = rng.gen::<u8>() & 0x0F;
        *byte = (r0 << 4) | r1;
    }
}

fn zrand_libc_random(out: &mut [u8], statep: Option<&mut StdRng>) {
    let mut rng = match statep {
        Some(r) => r,
        None => {
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let mut r = StdRng::seed_from_u64(seed);
            &mut r
        }
    };

    let mut n = out.len();
    while n > 0 {
        let ri = rng.gen::<u32>();
        out[n - 1] = (ri & 0xFF) as u8;
        if n == 1 { break; }
        out[n - 2] = ((ri >> 8) & 0xFF) as u8;
        if n == 2 { break; }
        out[n - 3] = ((ri >> 16) & 0xFF) as u8;
        n = n.saturating_sub(3);
    }
}

fn zrand_fd(out: &mut [u8], fd: &File) -> io::Result<()> {
    let mut total_read = 0;
    while total_read < out.len() {
        let bytes_read = fd.take(out.len() as u64).read(&mut out[total_read..])?;
        if bytes_read == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "End of file"));
        }
        total_read += bytes_read;
    }
    Ok(())
}

fn zrand_get_random_bits(r: &mut Z, bits: usize, fun: fn(&mut [u8], Option<&mut StdRng>), rng: Option<&mut StdRng>) {
    let chars = (bits + 63) / 64;
    r.chars.resize(chars, 0);
    
    let bytes_needed = chars * 8;
    let mut buffer = vec![0u8; bytes_needed];
    fun(&mut buffer, rng);
    
    unsafe {
        std::ptr::copy_nonoverlapping(
            buffer.as_ptr(),
            r.chars.as_mut_ptr() as *mut u8,
            bytes_needed,
        );
    }
    
    let bits_in_last_char = bits % 64;
    if bits_in_last_char != 0 {
        let mask = (1u64 << bits_in_last_char) - 1;
        r.chars[chars - 1] &= mask;
    }
    
    r.used = 0;
    for (i, &ch) in r.chars.iter().enumerate().rev() {
        if ch != 0 {
            r.used = i + 1;
            break;
        }
    }
    
    r.set_signum(if r.used > 0 { 1 } else { 0 });
}

fn zrand(r: &mut Z, dev: ZRandDev, dist: ZRandDist, n: &Z) -> io::Result<()> {
    if n.zero() {
        r.set_signum(0);
        return Ok(());
    }

    let mut rng = StdRng::from_entropy();
    let random_fun = match dev {
        ZRandDev::FastRandom | ZRandDev::SecureRandom => {
            let pathname = match dev {
                ZRandDev::FastRandom => FAST_RANDOM_PATHNAME,
                ZRandDev::SecureRandom => SECURE_RANDOM_PATHNAME,
                _ => unreachable!(),
            };
            let file = File::open(pathname)?;
            return zrand_get_random_bits_fd(r, n, &file, dist);
        }
        ZRandDev::LibcRandRandom => zrand_libc_rand,
        ZRandDev::DefaultRandom | ZRandDev::FastestRandom | ZRandDev::LibcRandomRandom => zrand_libc_random,
        ZRandDev::LibcRand48Random => zrand_libc_rand48,
    };

    let bits = n.bits();
    match dist {
        ZRandDist::QuasiUniform => {
            zrand_get_random_bits(r, bits, random_fun, Some(&mut rng));
            let one = Z { chars: vec![1], used: 1, signum: 1 };
            r.add(r, &one);
            r.mul(r, n);
            r.rsh(r, bits);
        }
        ZRandDist::Uniform => {
            loop {
                zrand_get_random_bits(r, bits, random_fun, Some(&mut rng));
                if r.cmp_mag(n) <= 0 {
                    break;
                }
            }
        }
        ZRandDist::ModUniform => {
            zrand_get_random_bits(r, bits, random_fun, Some(&mut rng));
            if r.cmp_mag(n) > 0 {
                r.sub(r, n);
            }
        }
    }

    Ok(())
}

fn zrand_get_random_bits_fd(r: &mut Z, n: &Z, file: &File, dist: ZRandDist) -> io::Result<()> {
    let bits = n.bits();
    let chars = (bits + 63) / 64;
    r.chars.resize(chars, 0);
    
    let bytes_needed = chars * 8;
    let mut buffer = vec![0u8; bytes_needed];
    zrand_fd(&mut buffer, file)?;
    
    unsafe {
        std::ptr::copy_nonoverlapping(
            buffer.as_ptr(),
            r.chars.as_mut_ptr() as *mut u8,
            bytes_needed,
        );
    }
    
    let bits_in_last_char = bits % 64;
    if bits_in_last_char != 0 {
        let mask = (1u64 << bits_in_last_char) - 1;
        r.chars[chars - 1] &= mask;
    }
    
    r.used = 0;
    for (i, &ch) in r.chars.iter().enumerate().rev() {
        if ch != 0 {
            r.used = i + 1;
            break;
        }
    }
    
    r.set_signum(if r.used > 0 { 1 } else { 0 });
    
    match dist {
        ZRandDist::QuasiUniform => {
            let one = Z { chars: vec![1], used: 1, signum: 1 };
            r.add(r, &one);
            r.mul(r, n);
            r.rsh(r, bits);
        }
        ZRandDist::Uniform => {
            while r.cmp_mag(n) > 0 {
                zrand_get_random_bits_fd(r, n, file, dist)?;
            }
        }
        ZRandDist::ModUniform => {
            if r.cmp_mag(n) > 0 {
                r.sub(r, n);
            }
        }
    }
    
    Ok(())
}

trait Bits {
    fn bits(&self) -> usize;
}

impl Bits for Z {
    fn bits(&self) -> usize {
        if self.used == 0 {
            return 0;
        }
        let mut bits = (self.used - 1) * 64;
        let mut v = self.chars[self.used - 1];
        while v != 0 {
            v >>= 1;
            bits += 1;
        }
        bits
    }
}