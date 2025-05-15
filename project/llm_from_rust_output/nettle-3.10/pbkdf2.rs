use std::ptr;

type SizeT = usize;
type Uint8T = u8;

pub trait NettleHash {
    fn update(&mut self, data: &[u8]);
    fn digest(&mut self, output: &mut [u8]);
}

pub fn pbkdf2<H: NettleHash>(
    mac_ctx: &mut H,
    digest_size: SizeT,
    iterations: u32,
    salt: &[u8],
    length: SizeT,
    dst: &mut [u8],
) {
    assert!(iterations > 0);
    if length == 0 {
        return;
    }

    let mut u_buf = vec![0u8; digest_size];
    let mut t_buf = vec![0u8; digest_size];
    let mut i = 1u32;

    let mut remaining = length;
    let mut dst_offset = 0;

    loop {
        let mut tmp = [
            (i >> 24 & 0xff) as u8,
            (i >> 16 & 0xff) as u8,
            (i >> 8 & 0xff) as u8,
            (i & 0xff) as u8,
        ];

        mac_ctx.update(salt);
        mac_ctx.update(&tmp);
        mac_ctx.digest(&mut t_buf);

        let mut prev = &t_buf[..];
        for _ in 1..iterations {
            mac_ctx.update(prev);
            mac_ctx.digest(&mut u_buf);

            for (t, u) in t_buf.iter_mut().zip(&u_buf) {
                *t ^= *u;
            }

            prev = &u_buf[..];
        }

        let copy_len = std::cmp::min(remaining, digest_size);
        dst[dst_offset..dst_offset + copy_len].copy_from_slice(&t_buf[..copy_len]);

        if remaining <= digest_size {
            return;
        }

        remaining -= digest_size;
        dst_offset += digest_size;
        i += 1;
    }
}