use std::convert::TryInto;
use std::ptr;

pub type size_t = usize;
pub type uint8_t = u8;

pub trait Cipher {
    fn encrypt(&self, block_size: size_t, dst: &mut [u8], src: &[u8]);
    fn decrypt(&self, block_size: size_t, dst: &mut [u8], src: &[u8]);
}

pub fn cbc_encrypt(
    cipher: &dyn Cipher,
    block_size: size_t,
    iv: &mut [u8],
    length: size_t,
    dst: &mut [u8],
    src: &[u8],
) {
    assert_eq!(length % block_size, 0);
    assert_eq!(dst.len(), length);
    assert_eq!(src.len(), length);
    assert_eq!(iv.len(), block_size);

    let mut chunks = src.chunks_exact(block_size)
        .zip(dst.chunks_exact_mut(block_size));

    for (src_chunk, dst_chunk) in chunks.by_ref() {
        xor_blocks(iv, src_chunk);
        cipher.encrypt(block_size, dst_chunk, iv);
        iv.copy_from_slice(dst_chunk);
    }
}

pub fn cbc_decrypt(
    cipher: &dyn Cipher,
    block_size: size_t,
    iv: &mut [u8],
    length: size_t,
    dst: &mut [u8],
    src: &[u8],
) {
    assert_eq!(length % block_size, 0);
    assert_eq!(dst.len(), length);
    assert_eq!(src.len(), length);
    assert_eq!(iv.len(), block_size);

    if length == 0 {
        return;
    }

    if !ptr::eq(src, dst) {
        cipher.decrypt(length, dst, src);
        xor_blocks(dst, &iv[..block_size]);

        let (first, rest) = dst.split_at_mut(block_size);
        xor_blocks(rest, &src[..rest.len()]);

        iv.copy_from_slice(&src[length - block_size..]);
    } else {
        let buffer_size = if length <= 512 {
            length
        } else {
            512 - (512 % block_size)
        };

        let mut buffer = vec![0u8; buffer_size];
        let mut initial_iv = vec![0u8; block_size];

        let mut remaining_length = length;
        let mut offset = 0;

        while remaining_length > buffer_size {
            cipher.decrypt(buffer_size, &mut buffer, &dst[offset..offset + buffer_size]);

            initial_iv.copy_from_slice(iv);
            iv.copy_from_slice(&dst[offset + buffer_size - block_size..offset + buffer_size]);

            let (dst_first, dst_rest) = dst[offset..].split_at_mut(block_size);
            let (buf_first, buf_rest) = buffer.split_at(block_size);

            xor_blocks_3(
                &mut dst_rest[..buffer_size - block_size],
                &buf_rest[..buffer_size - block_size],
                &dst_first,
            );
            xor_blocks_3(dst_first, buf_first, &initial_iv);

            remaining_length -= buffer_size;
            offset += buffer_size;
        }

        cipher.decrypt(remaining_length, &mut buffer[..remaining_length], &dst[offset..]);

        initial_iv.copy_from_slice(iv);
        iv.copy_from_slice(&dst[offset + remaining_length - block_size..offset + remaining_length]);

        let (dst_first, dst_rest) = dst[offset..].split_at_mut(block_size);
        let (buf_first, buf_rest) = buffer.split_at(block_size);

        xor_blocks_3(
            &mut dst_rest[..remaining_length - block_size],
            &buf_rest[..remaining_length - block_size],
            &dst_first,
        );
        xor_blocks_3(dst_first, buf_first, &initial_iv);
    }
}

fn xor_blocks(dst: &mut [u8], src: &[u8]) {
    assert_eq!(dst.len(), src.len());
    for (d, s) in dst.iter_mut().zip(src) {
        *d ^= *s;
    }
}

fn xor_blocks_3(dst: &mut [u8], a: &[u8], b: &[u8]) {
    assert_eq!(dst.len(), a.len());
    assert_eq!(dst.len(), b.len());
    for ((d, a), b) in dst.iter_mut().zip(a).zip(b) {
        *d = *a ^ *b;
    }
}