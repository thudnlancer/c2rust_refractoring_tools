use std::mem;
use std::cmp::Ordering;
use std::convert::TryInto;
use aes::{Aes128, Aes192, Aes256};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::NoPadding;
use generic_array::GenericArray;
use subtle::ConstantTimeEq;

type Aes128Cbc = Cbc<Aes128, NoPadding>;
type Aes192Cbc = Cbc<Aes192, NoPadding>;
type Aes256Cbc = Cbc<Aes256, NoPadding>;

#[derive(Debug)]
pub enum KeyWrapError {
    InvalidInputLength,
    VerificationFailed,
}

pub fn nist_keywrap16(
    key: &[u8],
    iv: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, KeyWrapError> {
    if plaintext.len() % 8 != 0 || plaintext.is_empty() {
        return Err(KeyWrapError::InvalidInputLength);
    }
    if iv.len() != 8 {
        return Err(KeyWrapError::InvalidInputLength);
    }

    let n = plaintext.len() / 8;
    let mut a = iv.to_vec();
    let mut r = plaintext.to_vec();

    for j in 0..6 {
        for i in 0..n {
            let t = (n * j) + (i + 1);
            let mut block = Vec::with_capacity(16);
            block.extend_from_slice(&a);
            block.extend_from_slice(&r[i*8..(i+1)*8]);

            let cipher = match key.len() {
                16 => Aes128Cbc::new_from_slices(key, &[0; 16]).unwrap(),
                24 => Aes192Cbc::new_from_slices(key, &[0; 16]).unwrap(),
                32 => Aes256Cbc::new_from_slices(key, &[0; 16]).unwrap(),
                _ => return Err(KeyWrapError::InvalidInputLength),
            };

            let mut buf = GenericArray::clone_from_slice(&block);
            cipher.encrypt_block(&mut buf);

            let b = buf.as_slice();
            a = (u64::from_be_bytes(b[..8].try_into().unwrap()) ^ (t as u64).to_be_bytes().to_vec();
            r[i*8..(i+1)*8].copy_from_slice(&b[8..16]);
        }
    }

    let mut ciphertext = Vec::with_capacity(plaintext.len() + 8);
    ciphertext.extend_from_slice(&a);
    ciphertext.extend_from_slice(&r);
    Ok(ciphertext)
}

pub fn nist_keyunwrap16(
    key: &[u8],
    iv: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, KeyWrapError> {
    if ciphertext.len() % 8 != 0 || ciphertext.len() < 16 {
        return Err(KeyWrapError::InvalidInputLength);
    }
    if iv.len() != 8 {
        return Err(KeyWrapError::InvalidInputLength);
    }

    let n = (ciphertext.len() - 8) / 8;
    let mut a = ciphertext[..8].to_vec();
    let mut r = ciphertext[8..].to_vec();

    for j in (0..6).rev() {
        for i in (0..n).rev() {
            let t = (n * j) + (i + 1);
            a = (u64::from_be_bytes(a[..8].try_into().unwrap()) ^ (t as u64)).to_be_bytes().to_vec();

            let mut block = Vec::with_capacity(16);
            block.extend_from_slice(&a);
            block.extend_from_slice(&r[i*8..(i+1)*8]);

            let cipher = match key.len() {
                16 => Aes128Cbc::new_from_slices(key, &[0; 16]).unwrap(),
                24 => Aes192Cbc::new_from_slices(key, &[0; 16]).unwrap(),
                32 => Aes256Cbc::new_from_slices(key, &[0; 16]).unwrap(),
                _ => return Err(KeyWrapError::InvalidInputLength),
            };

            let mut buf = GenericArray::clone_from_slice(&block);
            cipher.decrypt_block(&mut buf);

            let b = buf.as_slice();
            a = b[..8].to_vec();
            r[i*8..(i+1)*8].copy_from_slice(&b[8..16]);
        }
    }

    if a.ct_eq(iv).unwrap_u8() == 1 {
        Ok(r)
    } else {
        Err(KeyWrapError::VerificationFailed)
    }
}

pub fn aes128_keywrap(
    key: &[u8],
    iv: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, KeyWrapError> {
    nist_keywrap16(key, iv, plaintext)
}

pub fn aes192_keywrap(
    key: &[u8],
    iv: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, KeyWrapError> {
    nist_keywrap16(key, iv, plaintext)
}

pub fn aes256_keywrap(
    key: &[u8],
    iv: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, KeyWrapError> {
    nist_keywrap16(key, iv, plaintext)
}

pub fn aes128_keyunwrap(
    key: &[u8],
    iv: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, KeyWrapError> {
    nist_keyunwrap16(key, iv, ciphertext)
}

pub fn aes192_keyunwrap(
    key: &[u8],
    iv: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, KeyWrapError> {
    nist_keyunwrap16(key, iv, ciphertext)
}

pub fn aes256_keyunwrap(
    key: &[u8],
    iv: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, KeyWrapError> {
    nist_keyunwrap16(key, iv, ciphertext)
}