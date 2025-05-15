use std::ptr;
use std::alloc::{self, Layout};
use std::process;

pub type SizeT = usize;

fn decide_size(current: SizeT, wanted: SizeT) -> SizeT {
    let mut newsize = if current == 0 { 16 } else { current };
    
    while newsize < wanted {
        if newsize.checked_mul(2).is_none() {
            return wanted;
        }
        newsize *= 2;
    }
    newsize
}

pub fn extendbuf(
    existing: Option<ptr::NonNull<u8>>,
    wanted: SizeT,
    allocated: &mut SizeT,
) -> Option<ptr::NonNull<u8>> {
    assert!(wanted > 0, "wanted must be greater than 0");
    
    let newsize = decide_size(*allocated, wanted);
    let result = if *allocated == 0 {
        assert!(existing.is_none(), "existing must be NULL when allocated is 0");
        *allocated = newsize;
        unsafe {
            let layout = Layout::from_size_align(newsize, 1).unwrap();
            alloc::alloc(layout)
        }
    } else if newsize != *allocated {
        *allocated = newsize;
        unsafe {
            let layout = Layout::from_size_align(*allocated, 1).unwrap();
            alloc::realloc(existing.unwrap().as_ptr(), layout, newsize)
        }
    } else {
        return existing;
    };

    ptr::NonNull::new(result)
}

pub fn xextendbuf(
    existing: Option<ptr::NonNull<u8>>,
    wanted: SizeT,
    allocated: &mut SizeT,
) -> ptr::NonNull<u8> {
    match extendbuf(existing, wanted, allocated) {
        Some(p) => p,
        None => {
            if let Some(p) = existing {
                unsafe {
                    let layout = Layout::from_size_align(*allocated, 1).unwrap();
                    alloc::dealloc(p.as_ptr(), layout);
                }
            }
            process::abort();
        }
    }
}