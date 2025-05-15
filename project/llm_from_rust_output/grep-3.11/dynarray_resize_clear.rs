use std::mem;
use std::ptr;
use std::slice;

pub type SizeT = usize;

#[derive(Clone)]
pub struct DynarrayHeader {
    pub used: SizeT,
    pub allocated: SizeT,
    pub array: Vec<u8>,
}

impl DynarrayHeader {
    pub fn resize_clear(&mut self, new_size: SizeT, element_size: SizeT) -> bool {
        let old_size = self.used;
        
        if new_size <= self.allocated {
            self.used = new_size;
        } else {
            let new_capacity = new_size.next_power_of_two();
            let new_bytes = new_capacity * element_size;
            
            if new_bytes > isize::MAX as usize {
                return false;
            }
            
            self.array.resize(new_bytes, 0);
            self.used = new_size;
            self.allocated = new_capacity;
        }
        
        if new_size > old_size {
            let start = old_size * element_size;
            let end = new_size * element_size;
            let new_elements = &mut self.array[start..end];
            new_elements.fill(0);
        }
        
        true
    }
}

pub fn gl_dynarray_resize_clear(
    list: &mut DynarrayHeader,
    size: SizeT,
    element_size: SizeT,
) -> bool {
    list.resize_clear(size, element_size)
}