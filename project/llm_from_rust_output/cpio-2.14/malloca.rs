use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

const SA_ALIGNMENT_MAX: usize = 16;
type SmallT = u8;
type IdxT = isize;

pub fn mmalloca(n: usize) -> Option<*mut u8> {
    let alignment_mask = 2 * SA_ALIGNMENT_MAX - 1;
    let plus = mem::size_of::<SmallT>() + alignment_mask;
    
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
    let offset = (umemplus & !alignment_mask)
        .checked_add(SA_ALIGNMENT_MAX)?
        .checked_sub(umem)? as IdxT;

    let vp = unsafe { mem.add(offset as usize) };
    let p = vp as *mut SmallT;
    unsafe { ptr::write(p.sub(1), offset as SmallT) };

    Some(vp)
}

pub fn freea(p: *mut u8) {
    if p.is_null() {
        return;
    }

    let alignment = p as usize & (SA_ALIGNMENT_MAX - 1);
    if alignment != 0 {
        panic!("Unaligned pointer passed to freea");
    }

    if p as usize & SA_ALIGNMENT_MAX != 0 {
        let offset = unsafe { ptr::read((p as *mut SmallT).sub(1)) } as usize;
        let mem = unsafe { p.sub(offset) };
        let layout = Layout::from_size_align(offset + mem::size_of::<SmallT>(), 1)
            .expect("Invalid layout in freea");
        unsafe { dealloc(mem, layout) };
    }
}