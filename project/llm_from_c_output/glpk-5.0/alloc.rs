use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr::{null_mut, NonNull};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const ALIGN: usize = 16;
const MBD_SIZE: usize = ((std::mem::size_of::<MBD>() + (ALIGN - 1)) / ALIGN) * ALIGN;

struct MBD {
    size: usize,
    self_ptr: *mut MBD,
    prev: *mut MBD,
    next: *mut MBD,
}

struct Env {
    mem_ptr: *mut MBD,
    mem_count: usize,
    mem_cpeak: usize,
    mem_total: usize,
    mem_tpeak: usize,
    mem_limit: usize,
}

lazy_static::lazy_static! {
    static ref ENV: Mutex<Env> = Mutex::new(Env {
        mem_ptr: null_mut(),
        mem_count: 0,
        mem_cpeak: 0,
        mem_total: 0,
        mem_tpeak: 0,
        mem_limit: usize::MAX,
    });
}

fn dma(func: &str, ptr: Option<NonNull<u8>>, size: usize) -> Result<Option<NonNull<u8>>, &'static str> {
    let mut env = ENV.lock().unwrap();
    
    let mbd_ptr = match ptr {
        Some(p) => {
            let mbd_ptr = (p.as_ptr() as usize - MBD_SIZE) as *mut MBD;
            unsafe {
                // Validate descriptor
                if (*mbd_ptr).self_ptr != mbd_ptr {
                    return Err("invalid pointer");
                }
                
                // Remove from linked list
                (*mbd_ptr).self_ptr = null_mut();
                if (*mbd_ptr).prev.is_null() {
                    env.mem_ptr = (*mbd_ptr).next;
                } else {
                    (*(*mbd_ptr).prev).next = (*mbd_ptr).next;
                }
                if !(*mbd_ptr).next.is_null() {
                    (*(*mbd_ptr).next).prev = (*mbd_ptr).prev;
                }
                
                // Update stats
                if !(env.mem_count >= 1 && env.mem_total >= (*mbd_ptr).size) {
                    return Err("memory allocation error");
                }
                env.mem_count -= 1;
                env.mem_total -= (*mbd_ptr).size;
                
                if size == 0 {
                    // Free memory
                    let layout = Layout::from_size_align((*mbd_ptr).size, ALIGN).unwrap();
                    dealloc(mbd_ptr as *mut u8, layout);
                    return Ok(None);
                }
                
                Some(NonNull::new_unchecked(mbd_ptr as *mut u8))
            }
        }
        None => None,
    };
    
    // Check size limits
    if size > usize::MAX - MBD_SIZE {
        return Err("block too large");
    }
    let total_size = size + MBD_SIZE;
    if total_size > env.mem_limit - env.mem_total {
        return Err("memory allocation limit exceeded");
    }
    if env.mem_count == usize::MAX {
        return Err("too many memory blocks allocated");
    }
    
    // Allocate/reallocate memory
    let layout = Layout::from_size_align(total_size, ALIGN).unwrap();
    let new_mbd_ptr = match mbd_ptr {
        Some(p) => unsafe { realloc(p.as_ptr(), layout, total_size) },
        None => unsafe { alloc(layout) },
    };
    
    if new_mbd_ptr.is_null() {
        return Err("no memory available");
    }
    
    let new_mbd = new_mbd_ptr as *mut MBD;
    unsafe {
        // Setup descriptor
        (*new_mbd).size = total_size;
        (*new_mbd).self_ptr = new_mbd;
        (*new_mbd).prev = null_mut();
        (*new_mbd).next = env.mem_ptr;
        
        // Add to linked list
        if !(*new_mbd).next.is_null() {
            (*(*new_mbd).next).prev = new_mbd;
        }
        env.mem_ptr = new_mbd;
        
        // Update stats
        env.mem_count += 1;
        if env.mem_cpeak < env.mem_count {
            env.mem_cpeak = env.mem_count;
        }
        env.mem_total += total_size;
        if env.mem_tpeak < env.mem_total {
            env.mem_tpeak = env.mem_total;
        }
        
        Ok(Some(NonNull::new_unchecked(new_mbd_ptr.add(MBD_SIZE))))
    }
}

pub fn glp_alloc(n: usize, size: usize) -> Result<NonNull<u8>, &'static str> {
    if n < 1 || size < 1 {
        return Err("invalid parameters");
    }
    if n > usize::MAX / size {
        return Err("block too large");
    }
    dma("glp_alloc", None, n * size)?.ok_or("allocation failed")
}

pub fn glp_realloc(ptr: NonNull<u8>, n: usize, size: usize) -> Result<NonNull<u8>, &'static str> {
    if n < 1 || size < 1 {
        return Err("invalid parameters");
    }
    if n > usize::MAX / size {
        return Err("block too large");
    }
    dma("glp_realloc", Some(ptr), n * size)?.ok_or("reallocation failed")
}

pub fn glp_free(ptr: NonNull<u8>) -> Result<(), &'static str> {
    dma("glp_free", Some(ptr), 0)?;
    Ok(())
}

pub fn glp_mem_limit(limit: usize) -> Result<(), &'static str> {
    let mut env = ENV.lock().unwrap();
    if limit < 1 {
        return Err("invalid parameter");
    }
    env.mem_limit = limit.saturating_mul(1 << 20);
    Ok(())
}

pub fn glp_mem_usage() -> (usize, usize, usize, usize) {
    let env = ENV.lock().unwrap();
    (env.mem_count, env.mem_cpeak, env.mem_total, env.mem_tpeak)
}