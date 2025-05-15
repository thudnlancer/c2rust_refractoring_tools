use std::mem;
use std::cmp;
use std::ptr;
use std::ops::{BitAnd, BitOr, Shl, Shr};
use aes::{Aes128, Aes192, Aes256, Block, BlockEncrypt, NewBlockCipher};
use cipher::generic_array::GenericArray;
use cipher::{BlockCipher, BlockEncryptMut, BlockDecryptMut};
use subtle::ConstantTimeEq;

pub const CCM_BLOCK_SIZE: usize = 16;
pub const CCM_DIGEST_SIZE: usize = 16;
pub const CCM_MIN_NONCE_SIZE: usize = 7;
pub const CCM_MAX_NONCE_SIZE: usize = 13;

const CCM_FLAG_L: u8 = 0x07;
const CCM_FLAG_M: u8 = 0x38;
const CCM_FLAG_ADATA: u8 = 0x40;
const CCM_FLAG_RESERVED: u8 = 0x80;
const CCM_OFFSET_FLAGS: usize = 0;
const CCM_OFFSET_NONCE: usize = 1;

#[derive(Clone)]
pub struct CcmCtx {
    ctr: Block,
    tag: Block,
    blength: usize,
}

impl CcmCtx {
    pub fn new() -> Self {
        Self {
            ctr: Default::default(),
            tag: Default::default(),
            blength: 0,
        }
    }

    fn pad(&mut self, cipher: &impl BlockEncrypt) {
        if self.blength != 0 {
            cipher.encrypt_block(&mut self.tag);
            self.blength = 0;
        }
    }

    fn build_iv(&mut self, noncelen: usize, nonce: &[u8], flags: u8, count: usize) {
        assert!(noncelen >= CCM_MIN_NONCE_SIZE);
        assert!(noncelen <= CCM_MAX_NONCE_SIZE);

        let l_size = CCM_BLOCK_SIZE - CCM_OFFSET_NONCE - noncelen;
        self.ctr[CCM_OFFSET_FLAGS] = flags | ((l_size - 1) as u8 & CCM_FLAG_L);
        self.ctr[CCM_OFFSET_NONCE..CCM_OFFSET_NONCE + noncelen].copy_from_slice(nonce);

        let mut remaining = count;
        for i in (CCM_OFFSET_NONCE + noncelen..CCM_BLOCK_SIZE).rev() {
            self.ctr[i] = (remaining & 0xff) as u8;
            remaining >>= 8;
        }
        assert_eq!(remaining, 0);
    }

    pub fn set_nonce(
        &mut self,
        cipher: &impl BlockEncrypt,
        noncelen: usize,
        nonce: &[u8],
        authlen: usize,
        msglen: usize,
        taglen: usize,
    ) {
        self.blength = 0;
        self.build_iv(noncelen, nonce, ((taglen - 2) << 2) & CCM_FLAG_M, msglen);
        self.build_iv(noncelen, nonce, 0, 1);

        if authlen == 0 {
            cipher.encrypt_block(&mut self.tag);
            return;
        }

        self.tag[CCM_OFFSET_FLAGS] |= CCM_FLAG_ADATA;
        cipher.encrypt_block(&mut self.tag);

        if authlen >= (0x01 << 32) {
            self.tag[self.blength] ^= 0xff;
            self.blength += 1;
            self.tag[self.blength] ^= 0xff;
            self.blength += 1;
            for shift in (32..=56).step_by(8).rev() {
                self.tag[self.blength] ^= ((authlen >> shift) & 0xff) as u8;
                self.blength += 1;
            }
        } else if authlen >= ((0x1 << 16) - (0x1 << 8)) {
            self.tag[self.blength] ^= 0xff;
            self.blength += 1;
            self.tag[self.blength] ^= 0xfe;
            self.blength += 1;
            for shift in (16..=24).step_by(8).rev() {
                self.tag[self.blength] ^= ((authlen >> shift) & 0xff) as u8;
                self.blength += 1;
            }
        }
        for shift in (0..=8).step_by(8).rev() {
            self.tag[self.blength] ^= ((authlen >> shift) & 0xff) as u8;
            self.blength += 1;
        }
    }

    pub fn update(&mut self, cipher: &impl BlockEncrypt, data: &[u8]) {
        let mut pos = 0;
        let len = data.len();

        if self.blength + len < CCM_BLOCK_SIZE {
            xor_bytes(&mut self.tag[self.blength..], &data[pos..pos + len]);
            self.blength += len;
            return;
        }

        if self.blength > 0 {
            let take = CCM_BLOCK_SIZE - self.blength;
            xor_bytes(&mut self.tag[self.blength..], &data[pos..pos + take]);
            pos += take;
            cipher.encrypt_block(&mut self.tag);
            self.blength = 0;
        }

        while pos + CCM_BLOCK_SIZE <= len {
            xor_bytes(&mut self.tag, &data[pos..pos + CCM_BLOCK_SIZE]);
            pos += CCM_BLOCK_SIZE;
            cipher.encrypt_block(&mut self.tag);
        }

        if pos < len {
            self.blength = len - pos;
            xor_bytes(&mut self.tag[..self.blength], &data[pos..]);
        }
    }

    pub fn encrypt(
        &mut self,
        cipher: &impl BlockEncrypt,
        dst: &mut [u8],
        src: &[u8],
    ) {
        self.pad(cipher);
        self.update(cipher, src);
        ctr_crypt(cipher, &mut self.ctr, dst, src);
    }

    pub fn decrypt(
        &mut self,
        cipher: &impl BlockEncrypt,
        dst: &mut [u8],
        src: &[u8],
    ) {
        ctr_crypt(cipher, &mut self.ctr, dst, src);
        self.pad(cipher);
        self.update(cipher, dst);
    }

    pub fn digest(&mut self, cipher: &impl BlockEncrypt, tag: &mut [u8]) {
        let l = (self.ctr[CCM_OFFSET_FLAGS] & CCM_FLAG_L) + 1;
        for i in (CCM_BLOCK_SIZE - l as usize)..CCM_BLOCK_SIZE {
            self.ctr[i] = 0;
        }
        self.pad(cipher);
        ctr_crypt(cipher, &mut self.ctr, tag, &self.tag);
    }
}

fn xor_bytes(dst: &mut [u8], src: &[u8]) {
    assert_eq!(dst.len(), src.len());
    for (d, s) in dst.iter_mut().zip(src) {
        *d ^= *s;
    }
}

fn ctr_crypt(cipher: &impl BlockEncrypt, ctr: &mut Block, dst: &mut [u8], src: &[u8]) {
    assert_eq!(dst.len(), src.len());
    let mut counter = *ctr;
    let mut pos = 0;
    while pos < src.len() {
        let mut block = counter;
        cipher.encrypt_block(&mut block);
        let take = cmp::min(CCM_BLOCK_SIZE, src.len() - pos);
        for i in 0..take {
            dst[pos + i] = src[pos + i] ^ block[i];
        }
        pos += take;
        increment_counter(&mut counter);
    }
    *ctr = counter;
}

fn increment_counter(ctr: &mut Block) {
    for i in (0..CCM_BLOCK_SIZE).rev() {
        ctr[i] = ctr[i].wrapping_add(1);
        if ctr[i] != 0 {
            break;
        }
    }
}

pub fn encrypt_message(
    cipher: &impl BlockEncrypt,
    nonce: &[u8],
    adata: &[u8],
    src: &[u8],
    dst: &mut [u8],
    tag_len: usize,
) {
    assert!(dst.len() >= src.len() + tag_len);
    let mut ctx = CcmCtx::new();
    ctx.set_nonce(cipher, nonce.len(), nonce, adata.len(), src.len(), tag_len);
    ctx.update(cipher, adata);
    ctx.encrypt(cipher, &mut dst[..src.len()], src);
    ctx.digest(cipher, &mut dst[src.len()..]);
}

pub fn decrypt_message(
    cipher: &impl BlockEncrypt,
    nonce: &[u8],
    adata: &[u8],
    src: &[u8],
    dst: &mut [u8],
    tag_len: usize,
) -> bool {
    if src.len() < tag_len {
        return false;
    }
    let msg_len = src.len() - tag_len;
    let mut ctx = CcmCtx::new();
    ctx.set_nonce(cipher, nonce.len(), nonce, adata.len(), msg_len, tag_len);
    ctx.update(cipher, adata);
    ctx.decrypt(cipher, dst, &src[..msg_len]);
    let mut tag = vec![0u8; tag_len];
    ctx.digest(cipher, &mut tag);
    tag[..].ct_eq(&src[msg_len..]).into()
}

// AES-specific implementations would wrap CcmCtx with the appropriate cipher
// Similar to how the C code has ccm_aes128_ctx, ccm_aes192_ctx, etc.
// These would provide type-safe interfaces for each key size