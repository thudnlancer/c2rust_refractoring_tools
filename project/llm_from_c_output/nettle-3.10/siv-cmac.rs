use std::mem;
use std::ptr;
use std::cmp;
use std::ops::{BitXor, BitXorAssign};
use std::convert::TryInto;
use aes::{Aes128, Aes256, Block, BlockEncrypt, NewBlockCipher};
use cmac::{Cmac, Mac, NewMac};
use ctr::Ctr128;
use cipher::{FromBlockCipher, StreamCipher};
use subtle::ConstantTimeEq;

pub const SIV_BLOCK_SIZE: usize = 16;
pub const SIV_DIGEST_SIZE: usize = 16;
pub const SIV_MIN_NONCE_SIZE: usize = 1;
pub const SIV_CMAC_AES128_KEY_SIZE: usize = 32;
pub const SIV_CMAC_AES256_KEY_SIZE: usize = 64;

#[derive(Clone)]
pub struct SivCmacAes128 {
    cmac_key: Cmac<Aes128>,
    cmac_cipher: Aes128,
    ctr_cipher: Aes128,
}

#[derive(Clone)]
pub struct SivCmacAes256 {
    cmac_key: Cmac<Aes256>,
    cmac_cipher: Aes256,
    ctr_cipher: Aes256,
}

#[derive(Clone)]
pub struct Block16 {
    b: [u8; 16],
}

impl Block16 {
    pub fn new() -> Self {
        Block16 { b: [0; 16] }
    }

    pub fn mulx_be(&mut self) {
        let overflow = (self.b[0] & 0x80) != 0;
        for i in 0..15 {
            self.b[i] = (self.b[i] << 1) | ((self.b[i + 1] & 0x80) >> 7);
        }
        self.b[15] <<= 1;
        if overflow {
            self.b[15] ^= 0x87;
        }
    }
}

impl BitXor for Block16 {
    type Output = Block16;

    fn bitxor(self, rhs: Block16) -> Block16 {
        let mut res = Block16::new();
        for i in 0..16 {
            res.b[i] = self.b[i] ^ rhs.b[i];
        }
        res
    }
}

impl BitXorAssign for Block16 {
    fn bitxor_assign(&mut self, rhs: Block16) {
        for i in 0..16 {
            self.b[i] ^= rhs.b[i];
        }
    }
}

fn siv_s2v(
    cmac_key: &Cmac<Aes128>,
    cmac_cipher: &Aes128,
    adata: &[u8],
    nonce: &[u8],
    pdata: &[u8],
) -> Block16 {
    let mut D = Block16::new();
    let mut S = Block16::new();
    let mut T = Block16::new();
    let const_zero = Block16::new();
    let mut cmac_ctx = cmac_key.clone();

    assert!(nonce.len() >= SIV_MIN_NONCE_SIZE);

    cmac_ctx.update(&const_zero.b);
    D.b.copy_from_slice(&cmac_ctx.finalize_reset().into_bytes());

    D.mulx_be();
    cmac_ctx.update(adata);
    S.b.copy_from_slice(&cmac_ctx.finalize_reset().into_bytes());
    D ^= S;

    D.mulx_be();
    cmac_ctx.update(nonce);
    S.b.copy_from_slice(&cmac_ctx.finalize_reset().into_bytes());
    D ^= S;

    if pdata.len() >= 16 {
        cmac_ctx.update(&pdata[..pdata.len() - 16]);
        T = D;
        for i in 0..16 {
            T.b[i] ^= pdata[pdata.len() - 16 + i];
        }
    } else {
        let mut pad = [0u8; 16];
        pad[..pdata.len()].copy_from_slice(pdata);
        pad[pdata.len()] = 0x80;
        T = D;
        T.mulx_be();
        for i in 0..16 {
            T.b[i] ^= pad[i];
        }
    }

    cmac_ctx.update(&T.b);
    let mut v = Block16::new();
    v.b.copy_from_slice(&cmac_ctx.finalize().into_bytes());
    v
}

impl SivCmacAes128 {
    pub fn new(key: &[u8; SIV_CMAC_AES128_KEY_SIZE]) -> Self {
        let cmac_cipher = Aes128::new_from_slice(&key[..16]).unwrap();
        let ctr_cipher = Aes128::new_from_slice(&key[16..]).unwrap();
        let cmac_key = Cmac::<Aes128>::new_from_slice(&key[..16]).unwrap();
        SivCmacAes128 {
            cmac_key,
            cmac_cipher,
            ctr_cipher,
        }
    }

    pub fn encrypt_message(
        &self,
        nonce: &[u8],
        adata: &[u8],
        src: &[u8],
        dst: &mut [u8],
    ) {
        assert!(dst.len() >= src.len() + SIV_DIGEST_SIZE);
        let slength = src.len();

        let siv = siv_s2v(&self.cmac_key, &self.cmac_cipher, adata, nonce, src);
        dst[..SIV_DIGEST_SIZE].copy_from_slice(&siv.b);

        let mut ctr_nonce = siv;
        ctr_nonce.b[8] &= !0x80;
        ctr_nonce.b[12] &= !0x80;

        let mut cipher = Ctr128::from_block_cipher(self.ctr_cipher.clone(), &ctr_nonce.b.into());
        cipher.apply_keystream(&mut dst[SIV_DIGEST_SIZE..SIV_DIGEST_SIZE + slength]);
    }

    pub fn decrypt_message(
        &self,
        nonce: &[u8],
        adata: &[u8],
        src: &[u8],
        dst: &mut [u8],
    ) -> bool {
        assert!(src.len() >= SIV_DIGEST_SIZE);
        let mlength = src.len() - SIV_DIGEST_SIZE;
        assert!(dst.len() >= mlength);

        let mut ctr_nonce = Block16::new();
        ctr_nonce.b.copy_from_slice(&src[..SIV_DIGEST_SIZE]);
        ctr_nonce.b[8] &= !0x80;
        ctr_nonce.b[12] &= !0x80;

        let mut cipher = Ctr128::from_block_cipher(self.ctr_cipher.clone(), &ctr_nonce.b.into());
        cipher.apply_keystream(&mut dst[..mlength]);

        let siv = siv_s2v(&self.cmac_key, &self.cmac_cipher, adata, nonce, dst);
        siv.b.ct_eq(&src[..SIV_DIGEST_SIZE]).into()
    }
}

impl SivCmacAes256 {
    pub fn new(key: &[u8; SIV_CMAC_AES256_KEY_SIZE]) -> Self {
        let cmac_cipher = Aes256::new_from_slice(&key[..32]).unwrap();
        let ctr_cipher = Aes256::new_from_slice(&key[32..]).unwrap();
        let cmac_key = Cmac::<Aes256>::new_from_slice(&key[..32]).unwrap();
        SivCmacAes256 {
            cmac_key,
            cmac_cipher,
            ctr_cipher,
        }
    }

    pub fn encrypt_message(
        &self,
        nonce: &[u8],
        adata: &[u8],
        src: &[u8],
        dst: &mut [u8],
    ) {
        assert!(dst.len() >= src.len() + SIV_DIGEST_SIZE);
        let slength = src.len();

        let siv = siv_s2v(&self.cmac_key, &self.cmac_cipher, adata, nonce, src);
        dst[..SIV_DIGEST_SIZE].copy_from_slice(&siv.b);

        let mut ctr_nonce = siv;
        ctr_nonce.b[8] &= !0x80;
        ctr_nonce.b[12] &= !0x80;

        let mut cipher = Ctr128::from_block_cipher(self.ctr_cipher.clone(), &ctr_nonce.b.into());
        cipher.apply_keystream(&mut dst[SIV_DIGEST_SIZE..SIV_DIGEST_SIZE + slength]);
    }

    pub fn decrypt_message(
        &self,
        nonce: &[u8],
        adata: &[u8],
        src: &[u8],
        dst: &mut [u8],
    ) -> bool {
        assert!(src.len() >= SIV_DIGEST_SIZE);
        let mlength = src.len() - SIV_DIGEST_SIZE;
        assert!(dst.len() >= mlength);

        let mut ctr_nonce = Block16::new();
        ctr_nonce.b.copy_from_slice(&src[..SIV_DIGEST_SIZE]);
        ctr_nonce.b[8] &= !0x80;
        ctr_nonce.b[12] &= !0x80;

        let mut cipher = Ctr128::from_block_cipher(self.ctr_cipher.clone(), &ctr_nonce.b.into());
        cipher.apply_keystream(&mut dst[..mlength]);

        let siv = siv_s2v(&self.cmac_key, &self.cmac_cipher, adata, nonce, dst);
        siv.b.ct_eq(&src[..SIV_DIGEST_SIZE]).into()
    }
}