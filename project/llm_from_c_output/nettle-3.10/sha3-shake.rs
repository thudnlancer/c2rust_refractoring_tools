// sha3-shake.rs

use std::mem;

/// SHA3 SHAKE function implementation
pub fn sha3_shake(
    state: &mut sha3::Sha3State,
    block_size: usize,
    block: &mut [u8],
    index: usize,
    length: usize,
    dst: &mut [u8],
) {
    sha3::sha3_pad_shake(state, block_size, block, index);

    let mut remaining = length;
    let mut output_pos = 0;

    while remaining > block_size {
        state.permute();
        write_le64(state.a.as_ref(), &mut dst[output_pos..output_pos + block_size]);
        remaining -= block_size;
        output_pos += block_size;
    }

    state.permute();
    write_le64(state.a.as_ref(), &mut dst[output_pos..output_pos + remaining]);
}

/// SHA3 SHAKE output function
pub fn sha3_shake_output(
    state: &mut sha3::Sha3State,
    block_size: usize,
    block: &mut [u8],
    index: usize,
    length: usize,
    dst: &mut [u8],
) -> usize {
    let mut new_index = index;

    // We use one's complement of the index value to indicate SHAKE is initialized.
    if index < block_size {
        // First call of shake_output
        sha3::sha3_pad_shake(state, block_size, block, index);
        // Point at end of block to trigger fill in of buffer
        new_index = block_size;
    } else {
        new_index = !index;
    }

    debug_assert!(new_index <= block_size);

    let mut remaining = length;
    let mut output_pos = 0;

    // Write remaining data from buffer
    let left = block_size - new_index;
    if remaining <= left {
        dst[..remaining].copy_from_slice(&block[new_index..new_index + remaining]);
        return !(new_index + remaining);
    } else {
        dst[..left].copy_from_slice(&block[new_index..new_index + left]);
        remaining -= left;
        output_pos += left;
    }

    // Write full blocks
    while remaining > block_size {
        state.permute();
        write_le64(state.a.as_ref(), &mut dst[output_pos..output_pos + block_size]);
        remaining -= block_size;
        output_pos += block_size;
    }

    state.permute();
    // Fill buffer for next call
    write_le64(state.a.as_ref(), block);
    dst[output_pos..output_pos + remaining].copy_from_slice(&block[..remaining]);
    !remaining
}

/// Helper function to write u64 values in little-endian order
fn write_le64(src: &[u64], dst: &mut [u8]) {
    for (i, word) in src.iter().enumerate() {
        let bytes = word.to_le_bytes();
        let start = i * 8;
        dst[start..start + 8].copy_from_slice(&bytes);
    }
}

mod sha3 {
    pub struct Sha3State {
        pub a: [u64; 25],
        // ... other SHA3 state fields
    }

    impl Sha3State {
        pub fn permute(&mut self) {
            // SHA3 permutation implementation
            // ...
        }
    }

    pub fn sha3_pad_shake(state: &mut Sha3State, block_size: usize, block: &mut [u8], index: usize) {
        // SHA3 padding implementation for SHAKE
        // ...
    }
}