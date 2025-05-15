use std::convert::TryInto;

const PHI: u32 = 0x9E3779B9;

macro_rules! SBOX0 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $w:expr, $x:expr, $y:expr, $z:expr) => {
        let t01 = $b ^ $c;
        let t02 = $a | $d;
        let t03 = $a ^ $b;
        $z = t02 ^ t01;
        let t05 = $c | $z;
        let t06 = $a ^ $d;
        let t07 = $b | $c;
        let t08 = $d & t05;
        let t09 = t03 & t07;
        $y = t09 ^ t08;
        let t11 = t09 & $y;
        let t12 = $c ^ $d;
        let t13 = t07 ^ t11;
        let t14 = $b & t06;
        let t15 = t06 ^ t13;
        $w = !t15;
        let t17 = $w ^ t14;
        $x = t12 ^ t17;
    };
}

macro_rules! SBOX1 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $w:expr, $x:expr, $y:expr, $z:expr) => {
        let t01 = $a | $d;
        let t02 = $c ^ $d;
        let t03 = !$b;
        let t04 = $a ^ $c;
        let t05 = $a | t03;
        let t06 = $d & t04;
        let t07 = t01 & t02;
        let t08 = $b | t06;
        $y = t02 ^ t05;
        let t10 = t07 ^ t08;
        let t11 = t01 ^ t10;
        let t12 = $y ^ t11;
        let t13 = $b & $d;
        $z = !t10;
        $x = t13 ^ t12;
        let t16 = t10 | $x;
        let t17 = t05 & t16;
        $w = $c ^ t17;
    };
}

macro_rules! SBOX2 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $w:expr, $x:expr, $y:expr, $z:expr) => {
        let t01 = $a | $c;
        let t02 = $a ^ $b;
        let t03 = $d ^ t01;
        $w = t02 ^ t03;
        let t05 = $c ^ $w;
        let t06 = $b ^ t05;
        let t07 = $b | t05;
        let t08 = t01 & t06;
        let t09 = t03 ^ t07;
        let t10 = t02 | t09;
        $x = t10 ^ t08;
        let t12 = $a | $d;
        let t13 = t09 ^ $x;
        let t14 = $b ^ t13;
        $z = !t09;
        $y = t12 ^ t14;
    };
}

macro_rules! SBOX3 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $w:expr, $x:expr, $y:expr, $z:expr) => {
        let t01 = $a ^ $c;
        let t02 = $a | $d;
        let t03 = $a & $d;
        let t04 = t01 & t02;
        let t05 = $b | t03;
        let t06 = $a & $b;
        let t07 = $d ^ t04;
        let t08 = $c | t06;
        let t09 = $b ^ t07;
        let t10 = $d & t05;
        let t11 = t02 ^ t10;
        $z = t08 ^ t09;
        let t13 = $d | $z;
        let t14 = $a | t07;
        let t15 = $b & t13;
        $y = t08 ^ t11;
        $w = t14 ^ t15;
        $x = t05 ^ t04;
    };
}

macro_rules! SBOX4 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $w:expr, $x:expr, $y:expr, $z:expr) => {
        let t01 = $a | $b;
        let t02 = $b | $c;
        let t03 = $a ^ t02;
        let t04 = $b ^ $d;
        let t05 = $d | t03;
        let t06 = $d & t01;
        $z = t03 ^ t06;
        let t08 = $z & t04;
        let t09 = t04 & t05;
        let t10 = $c ^ t06;
        let t11 = $b & $c;
        let t12 = t04 ^ t08;
        let t13 = t11 | t03;
        let t14 = t10 ^ t09;
        $y = t13 ^ t08;
        let t15 = $a & t05;
        let t16 = t11 | t12;
        $x = t15 ^ t16;
        $w = !t14;
    };
}

macro_rules! SBOX5 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $w:expr, $x:expr, $y:expr, $z:expr) => {
        let t01 = $b ^ $d;
        let t02 = $b | $d;
        let t03 = $a & t01;
        let t04 = $c ^ t02;
        let t05 = t03 ^ t04;
        $w = !t05;
        let t07 = $a ^ t01;
        let t08 = $d | $w;
        let t09 = $b | t05;
        let t10 = $d ^ t08;
        let t11 = $b | t07;
        let t12 = t03 | $w;
        let t13 = t07 | t10;
        let t14 = t01 ^ t11;
        $y = t09 ^ t13;
        $x = t07 ^ t08;
        $z = t12 ^ t14;
    };
}

macro_rules! SBOX6 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $w:expr, $x:expr, $y:expr, $z:expr) => {
        let t01 = $a & $d;
        let t02 = $b ^ $c;
        let t03 = $a ^ $d;
        let t04 = t01 ^ t02;
        let t05 = $b | $c;
        $x = !t04;
        let t07 = t03 & t05;
        let t08 = $b & $x;
        let t09 = $a | $c;
        let t10 = t07 ^ t08;
        let t11 = $b | $d;
        let t12 = $c ^ t11;
        let t13 = t09 ^ t10;
        $y = !t13;
        let t15 = $x & t03;
        $z = t12 ^ t07;
        let t17 = $a ^ $b;
        let t18 = $y ^ t15;
        $w = t17 ^ t18;
    };
}

macro_rules! SBOX7 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $w:expr, $x:expr, $y:expr, $z:expr) => {
        let t01 = $a & $c;
        let t02 = !$d;
        let t03 = $a & t02;
        let t04 = $b | t01;
        let t05 = $a & $b;
        let t06 = $c ^ t04;
        $z = t03 ^ t06;
        let t08 = $c | $z;
        let t09 = $d | t05;
        let t10 = $a ^ t08;
        let t11 = t04 & $z;
        $x = t09 ^ t10;
        let t13 = $b ^ $x;
        let t14 = t01 ^ $x;
        let t15 = $c ^ t05;
        let t16 = t11 | t13;
        let t17 = t02 | t14;
        $w = t15 ^ t17;
        $y = $a ^ t16;
    };
}

fn rotl32(x: u32, n: u32) -> u32 {
    x.rotate_left(n)
}

#[derive(Debug, Clone)]
pub struct SerpentCtx {
    keys: [[u32; 4]; 33],
}

impl SerpentCtx {
    pub fn new() -> Self {
        SerpentCtx {
            keys: [[0; 4]; 33],
        }
    }

    fn serpent_key_pad(key: &[u8], w: &mut [u32; 8]) {
        assert!(key.len() <= 32);

        let mut i = 0;
        let mut chunks = key.chunks_exact(4);
        
        for chunk in &mut chunks {
            w[i] = u32::from_le_bytes(chunk.try_into().unwrap());
            i += 1;
        }

        let remainder = chunks.remainder();
        if i < 8 && !remainder.is_empty() {
            let mut pad = 0x01u32;
            for &byte in remainder.iter().rev() {
                pad = (pad << 8) | u32::from(byte);
            }
            w[i] = pad;
            i += 1;
        }

        while i < 8 {
            w[i] = 0;
            i += 1;
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        let mut w = [0u32; 8];
        Self::serpent_key_pad(key, &mut w);

        let mut keys = &mut self.keys[..];
        let mut k = 0;

        loop {
            ks(&mut keys, 3, &mut w, 0, &mut k);
            if k == 132 {
                break;
            }
            ks(&mut keys, 2, &mut w, 4, &mut k);
            ks(&mut keys, 1, &mut w, 0, &mut k);
            ks(&mut keys, 0, &mut w, 4, &mut k);
            ks(&mut keys, 7, &mut w, 0, &mut k);
            ks(&mut keys, 6, &mut w, 4, &mut k);
            ks(&mut keys, 5, &mut w, 0, &mut k);
            ks(&mut keys, 4, &mut w, 4, &mut k);
        }
    }

    pub fn set_key_128(&mut self, key: &[u8; 16]) {
        self.set_key(key);
    }

    pub fn set_key_192(&mut self, key: &[u8; 24]) {
        self.set_key(key);
    }

    pub fn set_key_256(&mut self, key: &[u8; 32]) {
        self.set_key(key);
    }
}

fn ks_recurrence(w: &mut [u32; 8], i: usize, k: &mut u32) {
    let wn = w[i] ^ w[(i + 3) & 7] ^ w[(i + 5) & 7] ^ w[(i + 7) & 7] ^ PHI ^ *k;
    *k += 1;
    w[i] = rotl32(wn, 11);
}

fn ks(keys: &mut &mut [[u32; 4]], s: u8, w: &mut [u32; 8], i: usize, k: &mut u32) {
    ks_recurrence(w, i, k);
    ks_recurrence(w, i + 1, k);
    ks_recurrence(w, i + 2, k);
    ks_recurrence(w, i + 3, k);

    let key = &mut keys[0];
    match s {
        0 => SBOX0!(w[i], w[i+1], w[i+2], w[i+3], key[0], key[1], key[2], key[3]),
        1 => SBOX1!(w[i], w[i+1], w[i+2], w[i+3], key[0], key[1], key[2], key[3]),
        2 => SBOX2!(w[i], w[i+1], w[i+2], w[i+3], key[0], key[1], key[2], key[3]),
        3 => SBOX3!(w[i], w[i+1], w[i+2], w[i+3], key[0], key[1], key[2], key[3]),
        4 => SBOX4!(w[i], w[i+1], w[i+2], w[i+3], key[0], key[1], key[2], key[3]),
        5 => SBOX5!(w[i], w[i+1], w[i+2], w[i+3], key[0], key[1], key[2], key[3]),
        6 => SBOX6!(w[i], w[i+1], w[i+2], w[i+3], key[0], key[1], key[2], key[3]),
        7 => SBOX7!(w[i], w[i+1], w[i+2], w[i+3], key[0], key[1], key[2], key[3]),
        _ => unreachable!(),
    }
    *keys = &mut keys[1..];
}