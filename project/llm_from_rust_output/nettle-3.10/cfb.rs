use std::ptr;

type NettleCipherFunc = fn(key: &[u8], block_size: usize, dst: &mut [u8], src: &[u8]);

pub fn nettle_cfb_encrypt(
    ctx: &[u8],
    f: NettleCipherFunc,
    block_size: usize,
    iv: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    let mut buffer = vec![0u8; block_size];
    let mut p = iv.as_mut_ptr();
    let mut remaining = length;
    let mut offset = 0;

    if src.as_ptr() != dst.as_ptr() {
        while remaining >= block_size {
            f(ctx, block_size, &mut buffer, unsafe {
                std::slice::from_raw_parts(p, block_size)
            });
            xor_bytes(
                &mut dst[offset..offset + block_size],
                &src[offset..offset + block_size],
                &buffer,
            );
            p = dst[offset..].as_mut_ptr();
            offset += block_size;
            remaining -= block_size;
        }
    } else {
        while remaining >= block_size {
            f(ctx, block_size, &mut buffer, unsafe {
                std::slice::from_raw_parts(p, block_size)
            });
            xor_bytes(
                &mut dst[offset..offset + block_size],
                &buffer,
                &buffer,
            );
            p = dst[offset..].as_mut_ptr();
            offset += block_size;
            remaining -= block_size;
        }
    }

    if !ptr::eq(p, iv.as_ptr()) {
        iv.copy_from_slice(unsafe { std::slice::from_raw_parts(p, block_size) });
    }

    if remaining > 0 {
        f(ctx, block_size, &mut buffer, iv);
        xor_bytes(
            &mut dst[offset..offset + remaining],
            &src[offset..offset + remaining],
            &buffer[..remaining],
        );
    }
}

pub fn nettle_cfb_decrypt(
    ctx: &[u8],
    f: NettleCipherFunc,
    block_size: usize,
    iv: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    if src.as_ptr() != dst.as_ptr() {
        let mut remaining = length % block_size;
        let mut processed = length - remaining;

        if processed > 0 {
            f(ctx, block_size, &mut dst[..block_size], iv);
            f(
                ctx,
                processed - block_size,
                &mut dst[block_size..processed],
                &src[..processed - block_size],
            );
            iv.copy_from_slice(&src[processed - block_size..processed]);
            xor_bytes(&mut dst[..processed], &src[..processed], &dst[..processed]);
        }

        if remaining > 0 {
            let mut buffer = vec![0u8; block_size];
            f(ctx, block_size, &mut buffer, iv);
            xor_bytes(
                &mut dst[processed..],
                &src[processed..],
                &buffer[..remaining],
            );
        }
    } else {
        let buffer_size = 512 - (512 % block_size);
        let mut buffer = vec![0u8; buffer_size];
        let mut initial_iv = vec![0u8; block_size];
        let mut remaining = length % block_size;
        let mut processed = length - remaining;
        let mut offset = 0;

        while processed > 0 {
            let part = std::cmp::min(processed, buffer_size);
            f(ctx, block_size, &mut buffer[..block_size], iv);
            f(
                ctx,
                part - block_size,
                &mut buffer[block_size..part],
                &dst[offset..offset + part - block_size],
            );
            iv.copy_from_slice(&dst[offset + part - block_size..offset + part]);
            xor_bytes(
                &mut dst[offset..offset + part],
                &buffer[..part],
                &buffer[..part],
            );
            offset += part;
            processed -= part;
        }

        if remaining > 0 {
            f(ctx, block_size, &mut buffer[..block_size], iv);
            xor_bytes(
                &mut dst[offset..],
                &buffer[..remaining],
                &buffer[..remaining],
            );
        }
    }
}

pub fn nettle_cfb8_encrypt(
    ctx: &[u8],
    f: NettleCipherFunc,
    block_size: usize,
    iv: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    let mut buffer = vec![0u8; block_size * 2];
    let mut outbuf = vec![0u8; block_size];
    buffer[..block_size].copy_from_slice(iv);
    let mut pos = 0;

    for i in 0..length {
        if pos == block_size {
            buffer.copy_within(block_size..block_size * 2, 0);
            pos = 0;
        }

        f(ctx, block_size, &mut outbuf, &buffer[pos..pos + block_size]);
        dst[i] = src[i] ^ outbuf[0];
        buffer[pos + block_size] = dst[i];
        pos += 1;
    }

    iv.copy_from_slice(&buffer[pos..pos + block_size]);
}

pub fn nettle_cfb8_decrypt(
    ctx: &[u8],
    f: NettleCipherFunc,
    block_size: usize,
    iv: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    let mut buffer = vec![0u8; block_size * 2];
    let mut outbuf = vec![0u8; block_size * 2];
    buffer[..block_size].copy_from_slice(iv);
    buffer[block_size..block_size * 2].copy_from_slice(&src[..std::cmp::min(length, block_size)]);
    let mut i = 0;

    while i < length {
        let chunk_size = std::cmp::min(length - i, block_size);
        for j in 0..chunk_size {
            f(ctx, block_size, &mut outbuf[j..j + block_size], &buffer[j..j + block_size]);
        }
        xor_bytes(&mut dst[i..i + chunk_size], &src[i..i + chunk_size], &outbuf[..chunk_size]);
        i += chunk_size;

        if chunk_size == block_size {
            buffer.copy_within(block_size..block_size * 2, 0);
            buffer[block_size..block_size * 2].copy_from_slice(&src[i..std::cmp::min(i + block_size, length)]);
        }
    }

    iv.copy_from_slice(&buffer[i..i + block_size]);
}

fn xor_bytes(dst: &mut [u8], a: &[u8], b: &[u8]) {
    assert_eq!(dst.len(), a.len());
    assert_eq!(dst.len(), b.len());
    for ((d, a), b) in dst.iter_mut().zip(a).zip(b) {
        *d = a ^ b;
    }
}