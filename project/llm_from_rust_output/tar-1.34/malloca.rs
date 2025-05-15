use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

const SA_ALIGNMENT_MAX: usize = 16;
type SmallT = u8;

pub fn mmalloca(n: usize) -> Option<*mut u8> {
    let nplus = n
        .checked_add(mem::size_of::<SmallT>())?
        .checked_add(2 * SA_ALIGNMENT_MAX)?
        .checked_sub(1)?;

    if nplus >= n {
        let layout = Layout::from_size_align(nplus, 1).ok()?;
        let mem = unsafe { alloc(layout) };
        if !mem.is_null() {
            let p = ((mem as usize)
                .checked_add(mem::size_of::<SmallT>())?
                .checked_add(SA_ALIGNMENT_MAX)?
                .checked_sub(1)?)
                & !((2 * SA_ALIGNMENT_MAX - 1) as usize);
            let p = p.checked_add(SA_ALIGNMENT_MAX)? as *mut u8;
            unsafe {
                ptr::write(p.offset(-1) as *mut SmallT, p.offset_from(mem) as SmallT);
            }
            return Some(p);
        }
    }
    None
}

pub fn freea(p: *mut u8) {
    if (p as usize) & (SA_ALIGNMENT_MAX - 1) != 0 {
        panic!("Unaligned pointer");
    }
    if (p as usize) & SA_ALIGNMENT_MAX != 0 {
        let offset = unsafe { ptr::read(p.offset(-1) as *const SmallT) as usize };
        let mem = unsafe { p.offset(-(offset as isize)) };
        let layout = Layout::from_size_align(offset + 1, 1).unwrap();
        unsafe { dealloc(mem, layout) };
    }
}