use std::convert::TryInto;
use std::mem;

const BLOCK_SIZE_16: usize = 16;
const BUFFER_SIZE_MAX: usize = 512;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock16 {
    pub b: [u8; 16],
    pub w: [u64; 2],
    pub u64_0: [u64; 2],
}

pub type NettleCipherFunc = fn(ctx: &[u8], length: usize, dst: &mut [u8], src: &[u8]);
pub type NettleFill16Func = fn(ctr: &mut [u8], blocks: usize, buffer: &mut [NettleBlock16]);

fn ctr_fill(block_size: usize, ctr: &mut [u8], length: usize, buffer: &mut [u8]) -> usize {
    let mut i = 0;
    while i + block_size <= length {
        buffer[i..i + block_size].copy_from_slice(&ctr[..block_size]);
        
        let mut increment_i = block_size - 1;
        ctr[increment_i] = ctr[increment_i].wrapping_add(1);
        
        if ctr[increment_i] == 0 {
            while increment_i > 0 {
                increment_i -= 1;
                ctr[increment_i] = ctr[increment_i].wrapping_add(1);
                if ctr[increment_i] != 0 {
                    break;
                }
            }
        }
        
        i += block_size;
    }
    i
}

fn ctr_fill16(ctr: &mut [u8], blocks: usize, buffer: &mut [NettleBlock16]) {
    let hi = u64::from_be_bytes(ctr[..8].try_into().unwrap());
    let mut lo = u64::from_be_bytes(ctr[8..16].try_into().unwrap());
    
    for i in 0..blocks {
        unsafe {
            buffer[i].u64_0[0] = hi;
            buffer[i].u64_0[1] = lo.to_be();
        }
        lo = lo.wrapping_add(1);
    }
    
    let new_ctr = hi.to_be_bytes()
        .iter()
        .chain(&lo.to_be_bytes())
        .copied()
        .collect::<Vec<u8>>();
    ctr.copy_from_slice(&new_ctr);
}

pub fn nettle_ctr_crypt(
    ctx: &[u8],
    f: NettleCipherFunc,
    block_size: usize,
    ctr: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    assert!(block_size <= ctr.len());
    
    if block_size == BLOCK_SIZE_16 {
        let mut blocks = length / BLOCK_SIZE_16;
        let remainder = length % BLOCK_SIZE_16;
        
        let mut buffer = vec![NettleBlock16 { u64_0: [0; 2] }; blocks];
        ctr_fill16(ctr, blocks, &mut buffer);
        
        let buffer_bytes = unsafe {
            std::slice::from_raw_parts(
                buffer.as_ptr() as *const u8,
                blocks * BLOCK_SIZE_16
            )
        };
        
        f(ctx, blocks * BLOCK_SIZE_16, dst, buffer_bytes);
        
        if remainder > 0 {
            let mut block = vec![0u8; BLOCK_SIZE_16];
            f(ctx, BLOCK_SIZE_16, &mut block, ctr);
            
            for i in 0..remainder {
                dst[blocks * BLOCK_SIZE_16 + i] = 
                    src[blocks * BLOCK_SIZE_16 + i] ^ block[i];
            }
            
            // Increment counter
            let mut increment_i = block_size - 1;
            ctr[increment_i] = ctr[increment_i].wrapping_add(1);
            if ctr[increment_i] == 0 {
                while increment_i > 0 {
                    increment_i -= 1;
                    ctr[increment_i] = ctr[increment_i].wrapping_add(1);
                    if ctr[increment_i] != 0 {
                        break;
                    }
                }
            }
        }
        return;
    }
    
    if src.as_ptr() != dst.as_ptr() {
        let filled = ctr_fill(block_size, ctr, length, dst);
        f(ctx, filled, dst, dst);
        
        for i in 0..filled {
            dst[i] ^= src[i];
        }
        
        if filled < length {
            let mut block = vec![0u8; block_size];
            f(ctx, block_size, &mut block, ctr);
            
            for i in filled..length {
                dst[i] = src[i] ^ block[i - filled];
            }
            
            // Increment counter
            let mut increment_i = block_size - 1;
            ctr[increment_i] = ctr[increment_i].wrapping_add(1);
            if ctr[increment_i] == 0 {
                while increment_i > 0 {
                    increment_i -= 1;
                    ctr[increment_i] = ctr[increment_i].wrapping_add(1);
                    if ctr[increment_i] != 0 {
                        break;
                    }
                }
            }
        }
    } else {
        let buffer_size = if length < block_size {
            block_size
        } else if length <= BUFFER_SIZE_MAX {
            length
        } else {
            BUFFER_SIZE_MAX
        };
        
        let mut buffer = vec![0u8; buffer_size];
        let mut remaining_length = length;
        let mut offset = 0;
        
        while remaining_length >= block_size {
            let chunk_size = std::cmp::min(buffer_size, remaining_length);
            let filled = ctr_fill(block_size, ctr, chunk_size, &mut buffer);
            assert!(filled > 0);
            
            f(ctx, filled, &mut buffer[..filled], &buffer[..filled]);
            
            for i in 0..filled {
                dst[offset + i] ^= buffer[i];
            }
            
            remaining_length -= filled;
            offset += filled;
        }
        
        if remaining_length > 0 {
            f(ctx, block_size, &mut buffer[..block_size], ctr);
            
            for i in 0..remaining_length {
                dst[offset + i] ^= buffer[i];
            }
            
            // Increment counter
            let mut increment_i = block_size - 1;
            ctr[increment_i] = ctr[increment_i].wrapping_add(1);
            if ctr[increment_i] == 0 {
                while increment_i > 0 {
                    increment_i -= 1;
                    ctr[increment_i] = ctr[increment_i].wrapping_add(1);
                    if ctr[increment_i] != 0 {
                        break;
                    }
                }
            }
        }
    }
}