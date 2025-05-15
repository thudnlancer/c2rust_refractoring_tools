use std::convert::TryInto;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Salsa20Context {
    pub input: [u32; 16],
}

fn salsa20_core(dst: &mut [u32; 16], src: &[u32; 16], rounds: u32) {
    // Implementation of _nettle_salsa20_core
    unimplemented!()
}

fn salsa20_2core(dst: &mut [u32; 32], src: &[u32; 16], rounds: u32) {
    // Implementation of _nettle_salsa20_2core
    unimplemented!()
}

fn memxor3(dst: &mut [u8], a: &[u8], b: &[u8]) {
    // Implementation of nettle_memxor3
    assert_eq!(dst.len(), a.len());
    assert_eq!(dst.len(), b.len());
    for ((d, a), b) in dst.iter_mut().zip(a).zip(b) {
        *d = a ^ b;
    }
}

pub fn salsa20_crypt(
    ctx: &mut Salsa20Context,
    rounds: u32,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    assert_eq!(dst.len(), src.len());
    assert!(dst.len() >= length);

    let mut x = [0u32; 32];
    let mut remaining = length;
    let mut offset = 0;

    while remaining > 64 {
        salsa20_2core(&mut x, &ctx.input, rounds);
        
        ctx.input[8] = ctx.input[8].wrapping_add(2);
        ctx.input[9] = ctx.input[9].wrapping_add(if ctx.input[8] < 2 { 1 } else { 0 });

        if remaining <= 2 * 64 {
            let chunk_len = remaining;
            let x_bytes = unsafe {
                std::slice::from_raw_parts(
                    x.as_ptr() as *const u8,
                    x.len() * std::mem::size_of::<u32>()
                )
            };
            memxor3(&mut dst[offset..offset+chunk_len], &src[offset..offset+chunk_len], &x_bytes[..chunk_len]);
            return;
        }

        let chunk_len = 2 * 64;
        let x_bytes = unsafe {
            std::slice::from_raw_parts(
                x.as_ptr() as *const u8,
                x.len() * std::mem::size_of::<u32>()
            )
        };
        memxor3(
            &mut dst[offset..offset+chunk_len],
            &src[offset..offset+chunk_len],
            &x_bytes[..chunk_len]
        );

        remaining -= chunk_len;
        offset += chunk_len;
    }

    salsa20_core(&mut x[..16].try_into().unwrap(), &ctx.input, rounds);
    ctx.input[8] = ctx.input[8].wrapping_add(1);
    ctx.input[9] = ctx.input[9].wrapping_add(if ctx.input[8] == 0 { 1 } else { 0 });

    let x_bytes = unsafe {
        std::slice::from_raw_parts(
            x.as_ptr() as *const u8,
            x.len() * std::mem::size_of::<u32>()
        )
    };
    memxor3(
        &mut dst[offset..offset+remaining],
        &src[offset..offset+remaining],
        &x_bytes[..remaining]
    );
}