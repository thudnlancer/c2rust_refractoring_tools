use std::alloc::{alloc, dealloc, Layout};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

struct Obstacle {
    chunk_size: usize,
    chunk: *mut Chunk,
    object_base: *mut c_char,
    next_free: *mut c_char,
    chunk_limit: *mut c_char,
    alignment_mask: usize,
}

struct Chunk {
    limit: *mut c_char,
    prev: *mut Chunk,
    contents: [c_char; 0],
}

struct Divvy {
    name: *const c_char,
    space: Obstacle,
    first: *mut c_void,
    count: usize,
}

static mut PLEXUS: *mut Divvy = ptr::null_mut();
static mut SINGLE: *mut Divvy = ptr::null_mut();

fn make_space(name: &CStr) -> Box<Divvy> {
    let widest = mem::size_of::<*mut c_void>().max(mem::size_of::<i64>());
    let alignment_mask = widest - 1;

    let mut divvy = Box::new(Divvy {
        name: name.as_ptr(),
        space: Obstacle {
            chunk_size: 0,
            chunk: ptr::null_mut(),
            object_base: ptr::null_mut(),
            next_free: ptr::null_mut(),
            chunk_limit: ptr::null_mut(),
            alignment_mask,
        },
        first: ptr::null_mut(),
        count: 0,
    });

    unsafe {
        divvy.first = divvy.space.next_free as *mut c_void;
    }
    divvy
}

fn alloc(divvy: &mut Divvy, len: usize) -> *mut c_void {
    divvy.count += 1;
    
    unsafe {
        if divvy.space.chunk_limit.offset_from(divvy.space.next_free) < len as isize {
            grow_chunk(&mut divvy.space, len);
        }
        
        let result = divvy.space.next_free as *mut c_void;
        divvy.space.next_free = divvy.space.next_free.add(len);
        align_pointer(&mut divvy.space);
        result
    }
}

fn zalloc(divvy: &mut Divvy, len: usize) -> *mut c_void {
    let ptr = alloc(divvy, len);
    unsafe {
        ptr::write_bytes(ptr, 0, len);
    }
    ptr
}

fn intern(divvy: &mut Divvy, s: &CStr) -> *mut c_char {
    let len = s.to_bytes().len();
    divvy.count += 1;
    
    unsafe {
        if divvy.space.chunk_limit.offset_from(divvy.space.next_free) < (len + 1) as isize {
            grow_chunk(&mut divvy.space, len + 1);
        }
        
        ptr::copy_nonoverlapping(s.as_ptr(), divvy.space.next_free, len);
        let result = divvy.space.next_free;
        divvy.space.next_free = divvy.space.next_free.add(len);
        *divvy.space.next_free = 0;
        divvy.space.next_free = divvy.space.next_free.add(1);
        align_pointer(&mut divvy.space);
        result
    }
}

fn brush_off(divvy: &mut Divvy, ptr: *mut c_void) {
    divvy.count -= 1;
    
    unsafe {
        if ptr > divvy.space.chunk as *mut c_void && ptr < divvy.space.chunk_limit as *mut c_void {
            divvy.space.object_base = ptr as *mut c_char;
            divvy.space.next_free = divvy.space.object_base;
        } else {
            free_chunk(&mut divvy.space, ptr);
        }
    }
}

fn forget(divvy: &mut Divvy) {
    unsafe {
        if divvy.first > divvy.space.chunk as *mut c_void && divvy.first < divvy.space.chunk_limit as *mut c_void {
            divvy.space.object_base = divvy.first as *mut c_char;
            divvy.space.next_free = divvy.space.object_base;
        } else {
            free_chunk(&mut divvy.space, divvy.first);
        }
    }
    divvy.count = 0;
}

unsafe fn grow_chunk(obst: &mut Obstacle, len: usize) {
    let new_size = len.next_power_of_two().max(obst.chunk_size);
    let layout = Layout::from_size_align(new_size, obst.alignment_mask + 1).unwrap();
    let new_chunk = alloc(layout) as *mut Chunk;
    
    (*new_chunk).prev = obst.chunk;
    (*new_chunk).limit = (new_chunk as *mut c_char).add(new_size);
    
    obst.chunk = new_chunk;
    obst.object_base = new_chunk as *mut c_char;
    obst.next_free = obst.object_base;
    obst.chunk_limit = (*new_chunk).limit;
    obst.chunk_size = new_size;
}

unsafe fn free_chunk(obst: &mut Obstacle, ptr: *mut c_void) {
    let mut current = obst.chunk;
    while !current.is_null() {
        let prev = (*current).prev;
        let layout = Layout::from_size_align(obst.chunk_size, obst.alignment_mask + 1).unwrap();
        dealloc(current as *mut u8, layout);
        current = prev;
    }
    
    obst.chunk = ptr::null_mut();
    obst.object_base = ptr::null_mut();
    obst.next_free = ptr::null_mut();
    obst.chunk_limit = ptr::null_mut();
    obst.chunk_size = 0;
}

unsafe fn align_pointer(obst: &mut Obstacle) {
    let aligned = ((obst.next_free as usize + obst.alignment_mask) & !obst.alignment_mask) as *mut c_char;
    obst.next_free = aligned.min(obst.chunk_limit);
    obst.object_base = obst.next_free;
}