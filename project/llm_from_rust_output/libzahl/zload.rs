use std::mem::size_of;
use std::ptr::NonNull;
use std::slice;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Zahl {
    pub sign: i32,
    padding__: i32,
    pub used: usize,
    pub alloced: usize,
    chars: NonNull<u64>,
}

impl Zahl {
    pub fn chars(&self) -> &[u64] {
        if self.used == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.chars.as_ptr(), self.used) }
        }
    }

    pub fn chars_mut(&mut self) -> &mut [u64] {
        if self.used == 0 {
            &mut []
        } else {
            unsafe { slice::from_raw_parts_mut(self.chars.as_ptr(), self.used) }
        }
    }
}

pub fn zzero(a: &Zahl) -> bool {
    a.sign == 0
}

pub fn zload(a: &mut Zahl, buffer: &[u8]) -> usize {
    let mut offset = 0;
    
    // Read sign
    a.sign = i32::from_ne_bytes(buffer[offset..offset + size_of::<i32>()].try_into().unwrap());
    offset += size_of::<i32>();
    
    // Read used
    a.used = usize::from_ne_bytes(buffer[offset..offset + size_of::<usize>()].try_into().unwrap());
    offset += size_of::<usize>();
    
    if a.sign != 0 {
        // Ensure capacity
        if a.alloced < a.used {
            // This would need to be implemented safely
            panic!("libzahl_realloc needs safe implementation");
        }
        
        // Copy data
        let char_size = size_of::<u64>();
        let needed_bytes = a.used * char_size;
        let chars_data = &buffer[offset..offset + needed_bytes];
        
        let chars = a.chars_mut();
        for (i, chunk) in chars_data.chunks_exact(char_size).enumerate() {
            chars[i] = u64::from_ne_bytes(chunk.try_into().unwrap());
        }
        
        offset += needed_bytes;
    }
    
    let base_size = size_of::<i32>() + size_of::<usize>();
    let data_size = if zzero(a) { 0 } else { (a.used + 3) & !3 * size_of::<u64>() };
    
    base_size + data_size
}

fn libzahl_memcpy(dest: &mut [u64], src: &[u64]) {
    dest.copy_from_slice(src);
}