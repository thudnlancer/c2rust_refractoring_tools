/* 
 * "pma", a persistent memory allocator (Rust implementation)
 * Original C code Copyright (C) 2019, 2022 Terence Kelly
 * Rust translation maintains same license: GNU Affero General Public License
 */

use std::{
    ffi::CString,
    fs::{File, OpenOptions},
    io::{self, Error, ErrorKind},
    mem::{size_of, align_of},
    os::unix::prelude::*,
    ptr::{self, NonNull},
    sync::atomic::{AtomicI32, Ordering},
    sync::Once,
};

// Version strings should match original C implementation
const PMA_H_VERSION: &str = "2022.10Oct.30.1667172241 (Avon 8-g1)";
static PMA_VERSION: &str = "2022.10Oct.30.1667172241 (Avon 8-g1)";

// Error state
static PMA_ERRNO: AtomicI32 = AtomicI32::new(0);

// Global state
struct State {
    init: i32,
    vrb: i32,
    file: Option<CString>,
    hdr: Option<NonNull<PmaHdr>>,
}

static STATE: Once = Once::new();
static mut GLOBAL_STATE: Option<State> = None;

fn state() -> &'static mut State {
    unsafe {
        STATE.call_once(|| {
            GLOBAL_STATE = Some(State {
                init: 0,
                vrb: 0,
                file: None,
                hdr: None,
            });
        });
        GLOBAL_STATE.as_mut().unwrap()
    }
}

// Constants
const VERS: u64 = 2;     // backing file format version
const WDSZ: usize = 8;   // word size (bytes)
const NFL: usize = 422;  // number of free lists / size classes
const ALGN: usize = 1024 * 1024 * 1024; // alignment of in-memory image

// Alloc object header
struct Ao {
    anext: *mut Ao,      // singly linked list of all aos in addr order
    fprev: *mut Ao,      // doubly linked free list
    fnext: *mut Ao,
}

// Header flags
const IU: u8 = 0x1;     // in use
const PIU: u8 = 0x2;    // previous in use 
const GROWN: u8 = 0x4;  // has grown via realloc

// PMA header structure
#[repr(C)]
struct PmaHdr {
    mapaddr: *mut (),
    bf_vers: u64,
    nallocs: u64,
    nfrees: u64,
    res_0: u64,
    root: *mut (),
    afirst: *mut Ao,
    abound: *mut Ao,
    free: [Ao; NFL],
}

// Initialize the PMA allocator
pub fn pma_init(verbose: i32, file: Option<&str>) -> io::Result<()> {
    let state = state();
    
    if !(0..=3).contains(&verbose) {
        PMA_ERRNO.store(line!(), Ordering::SeqCst);
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid verbosity level"));
    }

    state.vrb = verbose;
    
    // Check if already initialized
    if state.init != 0 {
        PMA_ERRNO.store(line!(), Ordering::SeqCst);
        return Err(Error::new(ErrorKind::AlreadyExists, "PMA already initialized"));
    }

    // Handle fallback to standard allocator
    if file.is_none() {
        state.init = 2;
        return Ok(());
    }

    let file = file.unwrap();
    let c_file = CString::new(file).map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid filename"))?;

    // Open backing file
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file)?;

    // Get file size and map into memory
    let metadata = fd.metadata()?;
    let file_size = metadata.len() as usize;

    // Check page size alignment
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if file_size % page_size as usize != 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "File size not multiple of page size"));
    }

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            file_size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd.as_raw_fd(),
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        return Err(Error::last_os_error());
    }

    // Initialize global state
    let hdr = ptr as *mut PmaHdr;
    state.init = 1;
    state.file = Some(c_file);
    state.hdr = NonNull::new(hdr);

    // Initialize header if new file
    unsafe {
        if (*hdr).mapaddr.is_null() {
            (*hdr).mapaddr = hdr as *mut _;
            (*hdr).bf_vers = VERS;
            (*hdr).afirst = (hdr as *mut u8).add(size_of::<PmaHdr>()) as *mut Ao;
            (*hdr).abound = (hdr as *mut u8).add(file_size) as *mut Ao;
            
            // Initialize free lists
            for i in 0..NFL {
                (*hdr).free[i].fprev = &mut (*hdr).free[i];
                (*hdr).free[i].fnext = &mut (*hdr).free[i];
            }
            
            // Set up initial wilderness block
            let wilderness = (*hdr).afirst;
            (*wilderness).anext = (*hdr).abound;
            
            // Add to free list
            let fl = sc(AOCAP(wilderness));
            let head = &mut (*hdr).free[fl];
            (*wilderness).fprev = head;
            (*wilderness).fnext = (*head).fnext;
            (*(*head).fnext).fprev = wilderness;
            (*head).fnext = wilderness;
        }
    }

    Ok(())
}

// Helper functions
fn sc(nbytes: usize) -> usize {
    static UB: Once = Once::new();
    static mut UPPER_BOUNDS: [usize; NFL] = [0; NFL];
    
    unsafe {
        UB.call_once(|| {
            UPPER_BOUNDS[0] = 3;
            for c in 1..NFL {
                let hi = ((1.0 + UPPER_BOUNDS[c-1] as f64) * 1.1).floor() as usize;
                UPPER_BOUNDS[c] = hi;
            }
        });
        
        let nwords = (nbytes / WDSZ) + if nbytes % WDSZ != 0 { 1 } else { 0 };
        for c in 0..NFL {
            if nwords <= UPPER_BOUNDS[c] {
                return c;
            }
        }
        NFL - 1
    }
}

// Memory allocation functions
pub fn pma_malloc(size: usize) -> Option<NonNull<u8>> {
    let state = state();
    
    if state.init == 2 {
        let ptr = unsafe { libc::malloc(size) };
        return NonNull::new(ptr);
    }
    
    if size == 0 {
        return None;
    }
    
    // Find appropriate free block
    let hdr = unsafe { state.hdr?.as_ref() };
    let c = sc(size);
    
    for fl in c..NFL {
        let head = &hdr.free[fl];
        let mut curr = unsafe { head.fnext };
        
        while curr != head as *const _ as *mut Ao {
            let ao = unsafe { &*curr };
            if AOCAP(ao) >= size {
                // Found suitable block
                let ptr = remove_from_free_list(ao);
                let split = split_ao(ptr, size);
                hdr.nallocs += 1;
                return NonNull::new(unsafe { &mut (*split).fprev } as *mut _ as *mut u8);
            }
            curr = unsafe { (*curr).fnext };
        }
    }
    
    None
}

// Other PMA functions would follow similar patterns...

// Helper functions would need to be implemented:
// - remove_from_free_list
// - split_ao
// - AOCAP (macro equivalent)
// - coalesce
// - etc.

// Note: This is a partial implementation showing the overall structure.
// A complete implementation would need to handle all the original C functionality
// while adhering to Rust's safety guarantees.