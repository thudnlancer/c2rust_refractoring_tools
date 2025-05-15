use std::mem;

const BLOCK_SIZE: usize = 16;
const CTR_BUFFER_LIMIT: usize = 512;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock16 {
    pub b: [u8; BLOCK_SIZE],
    pub w: [usize; 2],
    pub u64_0: [u64; 2],
}

pub type NettleCipherFunc = fn(&[u8], usize, &mut [u8], &[u8]);
pub type NettleFill16Func = fn(&mut [u8], usize, &mut [NettleBlock16]);

pub fn nettle_ctr_crypt16(
    ctx: &[u8],
    f: NettleCipherFunc,
    fill: NettleFill16Func,
    ctr: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    assert!(dst.len() >= length);
    assert!(src.len() >= length);
    assert!(ctr.len() >= BLOCK_SIZE);

    if dst.as_ptr() != src.as_ptr() && (dst.as_ptr() as usize) % mem::size_of::<u64>() == 0 {
        let blocks = length / BLOCK_SIZE;
        let done = blocks * BLOCK_SIZE;

        if blocks > 0 {
            let dst_blocks = unsafe {
                std::slice::from_raw_parts_mut(
                    dst.as_mut_ptr() as *mut NettleBlock16,
                    blocks,
                )
            };
            fill(ctr, blocks, dst_blocks);
            f(ctx, done, dst, dst);
            
            xor_memory(dst, src, done);
        }

        let remaining = length - done;
        if remaining > 0 {
            assert!(remaining < BLOCK_SIZE);
            let mut block = NettleBlock16 { b: [0; BLOCK_SIZE] };
            
            fill(ctr, 1, std::slice::from_mut(&mut block));
            f(ctx, BLOCK_SIZE, &mut block.b, &block.b);
            
            xor_memory(&mut dst[done..], &src[done..], remaining);
        }
    } else {
        let blocks = (length + BLOCK_SIZE - 1) / BLOCK_SIZE;
        let buffer_size = std::cmp::min(blocks, CTR_BUFFER_LIMIT / BLOCK_SIZE);
        let mut buffer = vec![NettleBlock16 { b: [0; BLOCK_SIZE] }; buffer_size];
        
        let mut i = 0;
        while blocks >= CTR_BUFFER_LIMIT / BLOCK_SIZE {
            fill(ctr, CTR_BUFFER_LIMIT / BLOCK_SIZE, &mut buffer);
            f(ctx, CTR_BUFFER_LIMIT, &mut buffer[0].b, &buffer[0].b);
            
            assert!(length - i < CTR_BUFFER_LIMIT);
            xor_memory(
                &mut dst[i..],
                &src[i..],
                CTR_BUFFER_LIMIT,
            );
            
            i += CTR_BUFFER_LIMIT;
        }

        if blocks > 0 {
            assert!(length - i < CTR_BUFFER_LIMIT);
            fill(ctr, blocks, &mut buffer[..blocks]);
            f(ctx, blocks * BLOCK_SIZE, &mut buffer[0].b, &buffer[0].b);
            xor_memory(&mut dst[i..], &src[i..], length - i);
        }
    }
}

fn xor_memory(dst: &mut [u8], src: &[u8], len: usize) {
    for i in 0..len {
        dst[i] ^= src[i];
    }
}