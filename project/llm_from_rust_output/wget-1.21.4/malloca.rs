use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr::NonNull;

const SA_ALIGNMENT_MAX: usize = 16;
type SmallT = u8;
type IdxT = isize;

pub fn mmalloca(n: usize) -> Option<NonNull<u8>> {
    let alignment2_mask = (2 * SA_ALIGNMENT_MAX - 1) as usize;
    let plus = mem::size_of::<SmallT>() + alignment2_mask;
    
    let nplus = n.checked_add(plus)?;
    if nplus > isize::MAX as usize {
        return None;
    }

    let layout = Layout::from_size_align(nplus, 1).ok()?;
    let mem = unsafe { alloc(layout) };
    if mem.is_null() {
        return None;
    }

    let umem = mem as usize;
    let umemplus = umem
        .checked_add(mem::size_of::<SmallT>() + SA_ALIGNMENT_MAX - 1)?;
    
    let offset = (umemplus & !alignment2_mask)
        .checked_add(SA_ALIGNMENT_MAX)?
        .checked_sub(umem)? as isize;
    
    let vp = unsafe { mem.add(offset as usize) };
    let p = vp as *mut SmallT;
    unsafe { *p.sub(1) = offset as SmallT };
    
    NonNull::new(p as *mut u8)
}

pub fn freea(p: Option<NonNull<u8>>) {
    let p = match p {
        Some(p) => p.as_ptr(),
        None => return,
    };

    let p_addr = p as usize;
    if p_addr & (SA_ALIGNMENT_MAX - 1) != 0 {
        panic!("Unaligned pointer");
    }

    if p_addr & SA_ALIGNMENT_MAX != 0 {
        let offset = unsafe { *(p as *const SmallT).sub(1) } as isize;
        let mem = unsafe { (p as *mut u8).offset(-offset) };
        let size = offset as usize + mem::size_of::<SmallT>();
        let layout = Layout::from_size_align(size, 1).unwrap();
        unsafe { dealloc(mem, layout) };
    }
}