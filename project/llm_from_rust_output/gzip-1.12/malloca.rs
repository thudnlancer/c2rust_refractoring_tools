use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

const SA_ALIGNMENT_MAX: usize = 16;
type IdxT = isize;
type SmallT = u8;

pub fn mmalloca(n: usize) -> Option<NonNull<u8>> {
    let alignment2_mask = (2 * SA_ALIGNMENT_MAX - 1) as usize;
    let plus = std::mem::size_of::<SmallT>() + alignment2_mask;
    let nplus = n.checked_add(plus)?;

    let layout = Layout::from_size_align(nplus, 1).ok()?;
    let mem = unsafe { alloc(layout) };
    if mem.is_null() {
        return None;
    }

    let umem = mem as usize;
    let umemplus = umem
        .checked_add(std::mem::size_of::<SmallT>() + SA_ALIGNMENT_MAX - 1)?;
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

    if (p as usize) & (SA_ALIGNMENT_MAX - 1) != 0 {
        panic!("Invalid alignment");
    }

    if (p as usize) & SA_ALIGNMENT_MAX != 0 {
        let offset = unsafe { *(p as *const SmallT).sub(1) } as isize;
        let mem = unsafe { (p as *mut u8).offset(-offset) };
        let layout = Layout::from_size_align(1, 1).unwrap();
        unsafe { dealloc(mem, layout) };
    }
}