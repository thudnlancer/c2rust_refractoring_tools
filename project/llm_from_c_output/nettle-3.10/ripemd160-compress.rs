use std::convert::TryInto;

const K0: u32 = 0x00000000;
const K1: u32 = 0x5A827999;
const K2: u32 = 0x6ED9EBA1;
const K3: u32 = 0x8F1BBCDC;
const K4: u32 = 0xA953FD4E;
const KK0: u32 = 0x50A28BE6;
const KK1: u32 = 0x5C4DD124;
const KK2: u32 = 0x6D703EF3;
const KK3: u32 = 0x7A6D76E9;
const KK4: u32 = 0x00000000;

fn f0(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }
fn f1(x: u32, y: u32, z: u32) -> u32 { (x & y) | (!x & z) }
fn f2(x: u32, y: u32, z: u32) -> u32 { (x | !y) ^ z }
fn f3(x: u32, y: u32, z: u32) -> u32 { (x & z) | (y & !z) }
fn f4(x: u32, y: u32, z: u32) -> u32 { x ^ (y | !z) }

fn rotl32(n: u32, x: u32) -> u32 {
    x.rotate_left(n)
}

fn r(a: &mut u32, b: u32, c: u32, d: u32, e: &mut u32, f: fn(u32, u32, u32) -> u32, k: u32, x: &[u32], r: usize, s: u32) {
    let t = a.wrapping_add(f(b, c, d))
             .wrapping_add(k)
             .wrapping_add(x[r]);
    *a = rotl32(s, t).wrapping_add(*e);
    *e = rotl32(10, *e);
}

pub fn ripemd160_compress(state: &mut [u32; 5], data: &[u8]) {
    let mut x = [0u32; 16];
    
    if cfg!(target_endian = "big") {
        for i in 0..16 {
            let pos = i * 4;
            x[i] = u32::from_le_bytes(data[pos..pos+4].try_into().unwrap());
        }
    } else {
        x.copy_from_slice(unsafe { std::mem::transmute::<&[u8], &[u32; 16]>(&data[0..64]).as_ref() });
    }

    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];

    // Left lane
    r(&mut a, b, c, d, &mut e, f0, K0, &x, 0, 11);
    r(&mut e, a, b, c, &mut d, f0, K0, &x, 1, 14);
    r(&mut d, e, a, b, &mut c, f0, K0, &x, 2, 15);
    r(&mut c, d, e, a, &mut b, f0, K0, &x, 3, 12);
    r(&mut b, c, d, e, &mut a, f0, K0, &x, 4, 5);
    r(&mut a, b, c, d, &mut e, f0, K0, &x, 5, 8);
    r(&mut e, a, b, c, &mut d, f0, K0, &x, 6, 7);
    r(&mut d, e, a, b, &mut c, f0, K0, &x, 7, 9);
    r(&mut c, d, e, a, &mut b, f0, K0, &x, 8, 11);
    r(&mut b, c, d, e, &mut a, f0, K0, &x, 9, 13);
    r(&mut a, b, c, d, &mut e, f0, K0, &x, 10, 14);
    r(&mut e, a, b, c, &mut d, f0, K0, &x, 11, 15);
    r(&mut d, e, a, b, &mut c, f0, K0, &x, 12, 6);
    r(&mut c, d, e, a, &mut b, f0, K0, &x, 13, 7);
    r(&mut b, c, d, e, &mut a, f0, K0, &x, 14, 9);
    r(&mut a, b, c, d, &mut e, f0, K0, &x, 15, 8);
    r(&mut e, a, b, c, &mut d, f1, K1, &x, 7, 7);
    r(&mut d, e, a, b, &mut c, f1, K1, &x, 4, 6);
    r(&mut c, d, e, a, &mut b, f1, K1, &x, 13, 8);
    r(&mut b, c, d, e, &mut a, f1, K1, &x, 1, 13);
    r(&mut a, b, c, d, &mut e, f1, K1, &x, 10, 11);
    r(&mut e, a, b, c, &mut d, f1, K1, &x, 6, 9);
    r(&mut d, e, a, b, &mut c, f1, K1, &x, 15, 7);
    r(&mut c, d, e, a, &mut b, f1, K1, &x, 3, 15);
    r(&mut b, c, d, e, &mut a, f1, K1, &x, 12, 7);
    r(&mut a, b, c, d, &mut e, f1, K1, &x, 0, 12);
    r(&mut e, a, b, c, &mut d, f1, K1, &x, 9, 15);
    r(&mut d, e, a, b, &mut c, f1, K1, &x, 5, 9);
    r(&mut c, d, e, a, &mut b, f1, K1, &x, 2, 11);
    r(&mut b, c, d, e, &mut a, f1, K1, &x, 14, 7);
    r(&mut a, b, c, d, &mut e, f1, K1, &x, 11, 13);
    r(&mut e, a, b, c, &mut d, f1, K1, &x, 8, 12);
    r(&mut d, e, a, b, &mut c, f2, K2, &x, 3, 11);
    r(&mut c, d, e, a, &mut b, f2, K2, &x, 10, 13);
    r(&mut b, c, d, e, &mut a, f2, K2, &x, 14, 6);
    r(&mut a, b, c, d, &mut e, f2, K2, &x, 4, 7);
    r(&mut e, a, b, c, &mut d, f2, K2, &x, 9, 14);
    r(&mut d, e, a, b, &mut c, f2, K2, &x, 15, 9);
    r(&mut c, d, e, a, &mut b, f2, K2, &x, 8, 13);
    r(&mut b, c, d, e, &mut a, f2, K2, &x, 1, 15);
    r(&mut a, b, c, d, &mut e, f2, K2, &x, 2, 14);
    r(&mut e, a, b, c, &mut d, f2, K2, &x, 7, 8);
    r(&mut d, e, a, b, &mut c, f2, K2, &x, 0, 13);
    r(&mut c, d, e, a, &mut b, f2, K2, &x, 6, 6);
    r(&mut b, c, d, e, &mut a, f2, K2, &x, 13, 5);
    r(&mut a, b, c, d, &mut e, f2, K2, &x, 11, 12);
    r(&mut e, a, b, c, &mut d, f2, K2, &x, 5, 7);
    r(&mut d, e, a, b, &mut c, f2, K2, &x, 12, 5);
    r(&mut c, d, e, a, &mut b, f3, K3, &x, 1, 11);
    r(&mut b, c, d, e, &mut a, f3, K3, &x, 9, 12);
    r(&mut a, b, c, d, &mut e, f3, K3, &x, 11, 14);
    r(&mut e, a, b, c, &mut d, f3, K3, &x, 10, 15);
    r(&mut d, e, a, b, &mut c, f3, K3, &x, 0, 14);
    r(&mut c, d, e, a, &mut b, f3, K3, &x, 8, 15);
    r(&mut b, c, d, e, &mut a, f3, K3, &x, 12, 9);
    r(&mut a, b, c, d, &mut e, f3, K3, &x, 4, 8);
    r(&mut e, a, b, c, &mut d, f3, K3, &x, 13, 9);
    r(&mut d, e, a, b, &mut c, f3, K3, &x, 3, 14);
    r(&mut c, d, e, a, &mut b, f3, K3, &x, 7, 5);
    r(&mut b, c, d, e, &mut a, f3, K3, &x, 15, 6);
    r(&mut a, b, c, d, &mut e, f3, K3, &x, 14, 8);
    r(&mut e, a, b, c, &mut d, f3, K3, &x, 5, 6);
    r(&mut d, e, a, b, &mut c, f3, K3, &x, 6, 5);
    r(&mut c, d, e, a, &mut b, f3, K3, &x, 2, 12);
    r(&mut b, c, d, e, &mut a, f4, K4, &x, 4, 9);
    r(&mut a, b, c, d, &mut e, f4, K4, &x, 0, 15);
    r(&mut e, a, b, c, &mut d, f4, K4, &x, 5, 5);
    r(&mut d, e, a, b, &mut c, f4, K4, &x, 9, 11);
    r(&mut c, d, e, a, &mut b, f4, K4, &x, 7, 6);
    r(&mut b, c, d, e, &mut a, f4, K4, &x, 12, 8);
    r(&mut a, b, c, d, &mut e, f4, K4, &x, 2, 13);
    r(&mut e, a, b, c, &mut d, f4, K4, &x, 10, 12);
    r(&mut d, e, a, b, &mut c, f4, K4, &x, 14, 5);
    r(&mut c, d, e, a, &mut b, f4, K4, &x, 1, 12);
    r(&mut b, c, d, e, &mut a, f4, K4, &x, 3, 13);
    r(&mut a, b, c, d, &mut e, f4, K4, &x, 8, 14);
    r(&mut e, a, b, c, &mut d, f4, K4, &x, 11, 11);
    r(&mut d, e, a, b, &mut c, f4, K4, &x, 6, 8);
    r(&mut c, d, e, a, &mut b, f4, K4, &x, 15, 5);
    r(&mut b, c, d, e, &mut a, f4, K4, &x, 13, 6);

    let aa = a;
    let bb = b;
    let cc = c;
    let dd = d;
    let ee = e;

    // Right lane
    a = state[0];
    b = state[1];
    c = state[2];
    d = state[3];
    e = state[4];

    r(&mut a, b, c, d, &mut e, f4, KK0, &x, 5, 8);
    r(&mut e, a, b, c, &mut d, f4, KK0, &x, 14, 9);
    r(&mut d, e, a, b, &mut c, f4, KK0, &x, 7, 9);
    r(&mut c, d, e, a, &mut b, f4, KK0, &x, 0, 11);
    r(&mut b, c, d, e, &mut a, f4, KK0, &x, 9, 13);
    r(&mut a, b, c, d, &mut e, f4, KK0, &x, 2, 15);
    r(&mut e, a, b, c, &mut d, f4, KK0, &x, 11, 15);
    r(&mut d, e, a, b, &mut c, f4, KK0, &x, 4, 5);
    r(&mut c, d, e, a, &mut b, f4, KK0, &x, 13, 7);
    r(&mut b, c, d, e, &mut a, f4, KK0, &x, 6, 7);
    r(&mut a, b, c, d, &mut e, f4, KK0, &x, 15, 8);
    r(&mut e, a, b, c, &mut d, f4, KK0, &x, 8, 11);
    r(&mut d, e, a, b, &mut c, f4, KK0, &x, 1, 14);
    r(&mut c, d, e, a, &mut b, f4, KK0, &x, 10, 14);
    r(&mut b, c, d, e, &mut a, f4, KK0, &x, 3, 12);
    r(&mut a, b, c, d, &mut e, f4, KK0, &x, 12, 6);
    r(&mut e, a, b, c, &mut d, f3, KK1, &x, 6, 9);
    r(&mut d, e, a, b, &mut c, f3, KK1, &x, 11, 13);
    r(&mut c, d, e, a, &mut b, f3, KK1, &x, 3, 15);
    r(&mut b, c, d, e, &mut a, f3, KK1, &x, 7, 7);
    r(&mut a, b, c, d, &mut e, f3, KK1, &x, 0, 12);
    r(&mut e, a, b, c, &mut d, f3, KK1, &x, 13, 8);
    r(&mut d, e, a, b, &mut c, f3, KK1, &x, 5, 9);
    r(&mut c, d, e, a, &mut b, f3, KK1, &x, 10, 11);
    r(&mut b, c, d, e, &mut a, f3, KK1, &x, 14, 7);
    r(&mut a, b, c, d, &mut e, f3, KK1, &x, 15, 7);
    r(&mut e, a, b, c, &mut d, f3, KK1, &x, 8, 12);
    r(&mut d, e, a, b, &mut c, f3, KK1, &x, 12, 7