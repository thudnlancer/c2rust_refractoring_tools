// balloon.rs

// Balloon password-hashing algorithm.

// For a description of the algorithm, see:
// Boneh, D., Corrigan-Gibbs, H., Schechter, S. (2017, May 12). Balloon Hashing:
// A Memory-Hard Function Providing Provable Protection Against Sequential Attacks.
// Retrieved Sep 1, 2022, from https://eprint.iacr.org/2016/027.pdf

use std::convert::TryInto;

const DELTA: usize = 3;

pub trait HashContext {
    fn update(&mut self, data: &[u8]);
    fn digest(&mut self, output: &mut [u8]);
}

fn hash<H: HashContext>(
    ctx: &mut H,
    cnt: u64,
    a: Option<&[u8]>,
    b: Option<&[u8]>,
    dst: &mut [u8],
) {
    let mut tmp = [0u8; 8];
    tmp.copy_from_slice(&cnt.to_le_bytes());
    ctx.update(&tmp);
    
    if let Some(a_data) = a {
        ctx.update(a_data);
    }
    
    if let Some(b_data) = b {
        ctx.update(b_data);
    }
    
    ctx.digest(dst);
}

fn hash_ints<H: HashContext>(
    ctx: &mut H,
    i: u64,
    j: u64,
    k: u64,
    dst: &mut [u8],
) {
    let mut tmp = [0u8; 24];
    tmp[0..8].copy_from_slice(&i.to_le_bytes());
    tmp[8..16].copy_from_slice(&j.to_le_bytes());
    tmp[16..24].copy_from_slice(&k.to_le_bytes());
    ctx.update(&tmp);
    ctx.digest(dst);
}

fn block_to_int(block: &[u8], mod_val: usize) -> usize {
    let mut r = 0;
    for &byte in block.iter().rev() {
        r = (r << 8) + byte as usize;
        r %= mod_val;
    }
    r
}

pub fn balloon<H: HashContext>(
    hash_ctx: &mut H,
    digest_size: usize,
    s_cost: usize,
    t_cost: usize,
    passwd: &[u8],
    salt: &[u8],
    scratch: &mut [u8],
    dst: &mut [u8],
) {
    let bs = digest_size;
    let (block, buf) = scratch.split_at_mut(bs);
    let mut cnt = 0;

    hash(hash_ctx, cnt, Some(passwd), Some(salt), &mut buf[0..bs]);
    cnt += 1;

    for i in 1..s_cost {
        let prev_block = &buf[(i - 1) * bs..i * bs];
        let current_block = &mut buf[i * bs..(i + 1) * bs];
        hash(hash_ctx, cnt, Some(prev_block), None, current_block);
        cnt += 1;
    }

    for i in 0..t_cost {
        for j in 0..s_cost {
            let prev_idx = if j > 0 { j - 1 } else { s_cost - 1 };
            let prev_block = &buf[prev_idx * bs..(prev_idx + 1) * bs];
            let current_block = &mut buf[j * bs..(j + 1) * bs];
            
            hash(hash_ctx, cnt, Some(prev_block), Some(current_block), current_block);
            cnt += 1;

            for k in 0..DELTA {
                hash_ints(hash_ctx, i as u64, j as u64, k as u64, block);
                hash(hash_ctx, cnt, Some(salt), Some(block), block);
                cnt += 1;

                let other_idx = block_to_int(block, s_cost);
                let other_block = &buf[other_idx * bs..(other_idx + 1) * bs];
                hash(hash_ctx, cnt, Some(current_block), Some(other_block), current_block);
                cnt += 1;
            }
        }
    }

    dst.copy_from_slice(&buf[(s_cost - 1) * bs..s_cost * bs]);
}

pub fn balloon_itch(digest_size: usize, s_cost: usize) -> usize {
    (s_cost + 1) * digest_size
}

pub fn balloon_sha1(
    s_cost: usize,
    t_cost: usize,
    passwd: &[u8],
    salt: &[u8],
    scratch: &mut [u8],
    dst: &mut [u8],
) {
    let mut ctx = sha1::Sha1::new();
    balloon(&mut ctx, 20, s_cost, t_cost, passwd, salt, scratch, dst);
}

pub fn balloon_sha256(
    s_cost: usize,
    t_cost: usize,
    passwd: &[u8],
    salt: &[u8],
    scratch: &mut [u8],
    dst: &mut [u8],
) {
    let mut ctx = sha2::Sha256::new();
    balloon(&mut ctx, 32, s_cost, t_cost, passwd, salt, scratch, dst);
}

pub fn balloon_sha384(
    s_cost: usize,
    t_cost: usize,
    passwd: &[u8],
    salt: &[u8],
    scratch: &mut [u8],
    dst: &mut [u8],
) {
    let mut ctx = sha2::Sha384::new();
    balloon(&mut ctx, 48, s_cost, t_cost, passwd, salt, scratch, dst);
}

pub fn balloon_sha512(
    s_cost: usize,
    t_cost: usize,
    passwd: &[u8],
    salt: &[u8],
    scratch: &mut [u8],
    dst: &mut [u8],
) {
    let mut ctx = sha2::Sha512::new();
    balloon(&mut ctx, 64, s_cost, t_cost, passwd, salt, scratch, dst);
}

impl HashContext for sha1::Sha1 {
    fn update(&mut self, data: &[u8]) {
        sha1::Digest::update(self, data);
    }
    fn digest(&mut self, output: &mut [u8]) {
        output.copy_from_slice(&sha1::Digest::finalize_reset(self));
    }
}

impl HashContext for sha2::Sha256 {
    fn update(&mut self, data: &[u8]) {
        sha2::Digest::update(self, data);
    }
    fn digest(&mut self, output: &mut [u8]) {
        output.copy_from_slice(&sha2::Digest::finalize_reset(self));
    }
}

impl HashContext for sha2::Sha384 {
    fn update(&mut self, data: &[u8]) {
        sha2::Digest::update(self, data);
    }
    fn digest(&mut self, output: &mut [u8]) {
        output.copy_from_slice(&sha2::Digest::finalize_reset(self));
    }
}

impl HashContext for sha2::Sha512 {
    fn update(&mut self, data: &[u8]) {
        sha2::Digest::update(self, data);
    }
    fn digest(&mut self, output: &mut [u8]) {
        output.copy_from_slice(&sha2::Digest::finalize_reset(self));
    }
}