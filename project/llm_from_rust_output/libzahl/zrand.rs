use std::fs::File;
use std::io::{Read, Error as IoError};
use std::mem::size_of;
use std::ptr::null_mut;
use std::time::{SystemTime, UNIX_EPOCH};

type ZahlCharT = u64;
type SizeT = usize;
type SSizeT = isize;
type IntPtrT = isize;

#[derive(Debug, Clone)]
struct Zahl {
    sign: i32,
    used: SizeT,
    alloced: SizeT,
    chars: Vec<ZahlCharT>,
}

impl Zahl {
    fn new() -> Self {
        Zahl {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: Vec::new(),
        }
    }

    fn zero(&self) -> bool {
        self.sign == 0
    }

    fn signum(&self) -> i32 {
        self.sign
    }

    fn bits(&mut self) -> SizeT {
        if self.zero() {
            return 1;
        }

        while self.chars[self.used - 1] == 0 {
            self.used -= 1;
        }

        let mut rc = self.used * 8 * size_of::<ZahlCharT>();
        rc -= self.chars[self.used - 1].leading_zeros() as SizeT;
        rc
    }

    fn cmp_mag(&mut self, other: &mut Zahl) -> i32 {
        if self.zero() {
            return -(!other.zero() as i32);
        }
        if other.zero() {
            return 1;
        }

        let mut i = self.used - 1;
        let mut j = other.used - 1;

        while i > j {
            if self.chars[i] != 0 {
                return 1;
            }
            self.used -= 1;
            i -= 1;
        }

        while j > i {
            if other.chars[j] != 0 {
                return -1;
            }
            other.used -= 1;
            j -= 1;
        }

        while i != 0 && self.chars[i] == other.chars[i] {
            i -= 1;
        }

        if self.chars[i] < other.chars[i] {
            -1
        } else {
            (self.chars[i] > other.chars[i]) as i32
        }
    }

    fn mul(&mut self, b: &mut Zahl, c: &mut Zahl) {
        let b_sign = b.sign;
        b.sign *= b_sign;
        let c_sign = c.sign;
        c.sign *= c_sign;

        // TODO: Implement zmul_ll equivalent
        // zmul_ll(self, b, c);

        c.sign = c_sign;
        b.sign = b_sign;
        self.sign = b.signum() * c.signum();
    }

    fn add(&mut self, a: &Zahl, b: &Zahl) {
        // TODO: Implement zadd equivalent
    }

    fn sub(&mut self, a: &Zahl, b: &Zahl) {
        // TODO: Implement zsub equivalent
    }

    fn rsh(&mut self, a: &Zahl, bits: SizeT) {
        // TODO: Implement zrsh equivalent
    }

    fn realloc(&mut self, new_size: SizeT) {
        if self.alloced < new_size {
            self.chars.resize(new_size, 0);
            self.alloced = new_size;
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ZRandDev {
    FastRandom,
    SecureRandom,
    DefaultRandom,
    FastestRandom,
    LibcRandRandom,
    LibcRandomRandom,
    LibcRand48Random,
}

#[derive(Debug, Clone, Copy)]
enum ZRandDist {
    QuasiUniform,
    Uniform,
    ModUniform,
}

fn zrand_libc_rand(out: &mut [u8], _statep: &mut ()) {
    static mut INITED: bool = false;
    unsafe {
        if !INITED {
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32;
            libc::srand(seed);
            INITED = true;
        }
    }

    for chunk in out.chunks_mut(2) {
        let ri = unsafe { libc::rand() as u32 };
        let rd = ri as f64 / (2147483647.0 + 1.0);
        let ri = (rd * (256.0 * 256.0)) as u32;

        if !chunk.is_empty() {
            chunk[0] = (ri >> 0 & 0xFF) as u8;
        }
        if chunk.len() > 1 {
            chunk[1] = (ri >> 8 & 0xFF) as u8;
        }
    }
}

fn zrand_libc_rand48(out: &mut [u8], _statep: &mut ()) {
    static mut INITED: bool = false;
    unsafe {
        if !INITED {
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i32;
            libc::srand48(seed as i64);
            INITED = true;
        }
    }

    for byte in out.iter_mut() {
        let r0 = unsafe { libc::lrand48() } & 0xF;
        let r1 = unsafe { libc::lrand48() } & 0xF;
        *byte = (r0 << 4 | r1) as u8;
    }
}

fn zrand_libc_random(out: &mut [u8], _statep: &mut ()) {
    static mut INITED: bool = false;
    unsafe {
        if !INITED {
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32;
            libc::srandom(seed);
            INITED = true;
        }
    }

    for chunk in out.chunks_mut(3) {
        let ri = unsafe { libc::random() };
        if !chunk.is_empty() {
            chunk[0] = (ri >> 0 & 0xFF) as u8;
        }
        if chunk.len() > 1 {
            chunk[1] = (ri >> 8 & 0xFF) as u8;
        }
        if chunk.len() > 2 {
            chunk[2] = (ri >> 16 & 0xFF) as u8;
        }
    }
}

fn zrand_fd(out: &mut [u8], statep: &mut File) -> Result<(), IoError> {
    statep.read_exact(out)
}

fn get_random_bits(
    r: &mut Zahl,
    bits: SizeT,
    fun: impl Fn(&mut [u8], &mut ()) -> (),
    statep: &mut (),
) {
    let chars = (bits + 63) >> 6;
    r.realloc(chars);

    let byte_len = chars * size_of::<ZahlCharT>();
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(r.chars.as_mut_ptr() as *mut u8, byte_len)
    };

    fun(bytes, statep);

    let bits = bits & 63;
    let mask = if bits == 0 {
        0
    } else {
        (1 << bits) - 1
    };

    if chars > 0 {
        r.chars[chars - 1] &= mask;
    }

    for i in (0..chars).rev() {
        if r.chars[i] != 0 {
            r.used = i + 1;
            r.sign = 1;
            return;
        }
    }

    r.sign = 0;
}

pub fn zrand(r: &mut Zahl, dev: ZRandDev, dist: ZRandDist, n: &mut Zahl) -> Result<(), String> {
    if n.zero() {
        r.sign = 0;
        return Ok(());
    }

    let (random_fun, mut statep) = match dev {
        ZRandDev::FastRandom => {
            let mut file = File::open("/dev/urandom").map_err(|e| e.to_string())?;
            (|out, state| {
                zrand_fd(out, state).map_err(|e| e.to_string())
            }, &mut file)
        }
        ZRandDev::SecureRandom => {
            let mut file = File::open("/dev/random").map_err(|e| e.to_string())?;
            (|out, state| {
                zrand_fd(out, state).map_err(|e| e.to_string())
            }, &mut file)
        }
        ZRandDev::LibcRandRandom => (zrand_libc_rand, &mut ()),
        ZRandDev::LibcRandomRandom | ZRandDev::DefaultRandom | ZRandDev::FastestRandom => {
            (zrand_libc_random, &mut ())
        }
        ZRandDev::LibcRand48Random => (zrand_libc_rand48, &mut ()),
    };

    let bits = n.bits();

    match dist {
        ZRandDist::QuasiUniform => {
            if n.signum() < 0 {
                return Err("Negative not allowed".to_string());
            }
            loop {
                get_random_bits(r, bits, random_fun, statep);
                if r.cmp_mag(n) <= 0 {
                    break;
                }
            }
            let mut one = Zahl::new();
            one.sign = 1;
            one.used = 1;
            one.chars = vec![1];
            r.add(r, &one);
            r.mul(r, n);
            r.rsh(r, bits);
        }
        ZRandDist::Uniform => {
            if n.signum() < 0 {
                return Err("Negative not allowed".to_string());
            }
            loop {
                get_random_bits(r, bits, random_fun, statep);
                if r.cmp_mag(n) <= 0 {
                    break;
                }
            }
        }
        ZRandDist::ModUniform => {
            if n.signum() < 0 {
                return Err("Negative not allowed".to_string());
            }
            loop {
                get_random_bits(r, bits, random_fun, statep);
                if r.cmp_mag(n) <= 0 {
                    break;
                }
            }
            if r.cmp_mag(n) > 0 {
                r.sub(r, n);
            }
        }
    }

    Ok(())
}