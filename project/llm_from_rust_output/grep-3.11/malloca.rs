use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

const SA_ALIGNMENT_MAX: usize = 16;
const SA_ALIGNMENT_LONGDOUBLE: usize = 16;
const SA_ALIGNMENT_LONGLONG: usize = 8;
const SA_ALIGNMENT_DOUBLE: usize = 8;
const SA_ALIGNMENT_LONG: usize = 8;

type SmallT = u8;
type IdxT = isize;

pub fn mmalloca(n: usize) -> Option<*mut u8> {
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
    unsafe { ptr::write(p.sub(1), offset as SmallT) };

    Some(p as *mut u8)
}

pub fn freea(p: *mut u8) {
    if p as usize & (SA_ALIGNMENT_MAX - 1) != 0 {
        panic!("Invalid alignment");
    }

    if p as usize & SA_ALIGNMENT_MAX != 0 {
        let offset = unsafe { ptr::read((p as *mut SmallT).sub(1)) as isize };
        let mem = unsafe { (p as *mut u8).offset(-offset) };
        let layout = Layout::from_size_align(offset as usize, 1).unwrap();
        unsafe { dealloc(mem, layout) };
    }
}