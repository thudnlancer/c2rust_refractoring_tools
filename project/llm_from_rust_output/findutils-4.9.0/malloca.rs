use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, align_of};
use std::ptr::{NonNull, null_mut};

const SA_ALIGNMENT_MAX: usize = 16;
type SmallT = u8;
type IdxT = isize;

pub fn mmalloca(n: usize) -> Option<NonNull<u8>> {
    let alignment2_mask = 2 * SA_ALIGNMENT_MAX - 1;
    let plus = size_of::<SmallT>() + alignment2_mask;
    
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
    let umemplus = umem.checked_add(size_of::<SmallT>() + SA_ALIGNMENT_MAX - 1)?;
    let offset = (umemplus & !alignment2_mask)
        .checked_add(SA_ALIGNMENT_MAX)?
        .checked_sub(umem)? as IdxT;

    let vp = unsafe { mem.add(offset as usize) };
    unsafe { *(vp.offset(-1) = offset as SmallT; }

    NonNull::new(vp)
}

pub fn freea(p: Option<NonNull<u8>>) {
    let p = match p {
        Some(p) => p.as_ptr(),
        None => return,
    };

    let p_addr = p as usize;
    if p_addr & (SA_ALIGNMENT_MAX - 1) != 0 {
        panic!("Invalid alignment");
    }

    if p_addr & SA_ALIGNMENT_MAX != 0 {
        let offset = unsafe { *(p.cast::<SmallT>().offset(-1)) as isize };
        let mem = unsafe { p.cast::<u8>().offset(-offset) };
        
        let layout = Layout::from_size_align(offset as usize + size_of::<SmallT>(), 1)
            .expect("Invalid layout");
        unsafe { dealloc(mem, layout); }
    }
}