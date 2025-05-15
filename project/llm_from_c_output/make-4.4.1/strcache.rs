use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::cmp;
use std::alloc::{alloc, dealloc, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

const CACHE_BUFFER_BASE: usize = 8192;
const CACHE_BUFFER_OFFSET: usize = mem::size_of::<StrCacheHeader>();
const BUFSIZE: usize = CACHE_BUFFER_BASE - CACHE_BUFFER_OFFSET;

type ScBuflenT = u16;

struct StrCacheHeader {
    next: *mut StrCache,
    end: ScBuflenT,
    bytesfree: ScBuflenT,
    count: ScBuflenT,
}

struct StrCache {
    header: StrCacheHeader,
    buffer: [u8; BUFSIZE],
}

struct HugeString {
    next: *mut HugeString,
    buffer: [u8],
}

static STRCACHE: AtomicUsize = AtomicUsize::new(0);
static FULLCACHE: AtomicUsize = AtomicUsize::new(0);
static HUGESTRINGS: AtomicUsize = AtomicUsize::new(0);

static TOTAL_BUFFERS: AtomicUsize = AtomicUsize::new(0);
static TOTAL_STRINGS: AtomicUsize = AtomicUsize::new(0);
static TOTAL_SIZE: AtomicUsize = AtomicUsize::new(0);
static TOTAL_ADDS: AtomicUsize = AtomicUsize::new(0);

fn new_cache(head: &AtomicUsize, buflen: ScBuflenT) -> *mut StrCache {
    let layout = Layout::new::<StrCache>();
    let cache = unsafe { alloc(layout) as *mut StrCache };
    
    unsafe {
        (*cache).header.next = head.load(Ordering::Relaxed) as *mut StrCache;
        (*cache).header.end = 0;
        (*cache).header.bytesfree = buflen;
        (*cache).header.count = 0;
    }
    
    head.store(cache as usize, Ordering::Relaxed);
    TOTAL_BUFFERS.fetch_add(1, Ordering::Relaxed);
    
    cache
}

fn copy_string(cache: *mut StrCache, str: &[u8]) -> *const u8 {
    let len = str.len() as ScBuflenT;
    let end = unsafe { (*cache).header.end };
    let buffer_ptr = unsafe { (*cache).buffer.as_ptr().offset(end as isize) };
    
    unsafe {
        ptr::copy_nonoverlapping(str.as_ptr(), buffer_ptr as *mut u8, str.len());
        *buffer_ptr.add(str.len()) = 0;
        (*cache).header.end += len + 1;
        (*cache).header.bytesfree -= len + 1;
        (*cache).header.count += 1;
    }
    
    buffer_ptr
}

fn add_string(str: &[u8]) -> *const u8 {
    let len = str.len();
    let sz = (len + 1) as ScBuflenT;
    
    TOTAL_STRINGS.fetch_add(1, Ordering::Relaxed);
    TOTAL_SIZE.fetch_add(sz as usize, Ordering::Relaxed);
    
    if sz > BUFSIZE as ScBuflenT {
        let cache = new_cache(&FULLCACHE, sz);
        return copy_string(cache, str);
    }
    
    let mut current = STRCACHE.load(Ordering::Relaxed) as *mut StrCache;
    let mut prev = ptr::null_mut();
    
    while !current.is_null() {
        if unsafe { (*current).header.bytesfree > sz } {
            break;
        }
        prev = current;
        current = unsafe { (*current).header.next };
    }
    
    if current.is_null() {
        current = new_cache(&STRCACHE, BUFSIZE as ScBuflenT);
    }
    
    let res = copy_string(current, str);
    
    if TOTAL_STRINGS.load(Ordering::Relaxed) > 20 &&
       unsafe { (*current).header.bytesfree } < 
       (TOTAL_SIZE.load(Ordering::Relaxed) / TOTAL_STRINGS.load(Ordering::Relaxed)) as ScBuflenT + 1 
    {
        if prev.is_null() {
            STRCACHE.store(unsafe { (*current).header.next as usize }, Ordering::Relaxed);
        } else {
            unsafe { (*prev).header.next = (*current).header.next };
        }
        
        unsafe {
            (*current).header.next = FULLCACHE.load(Ordering::Relaxed) as *mut StrCache;
            FULLCACHE.store(current as usize, Ordering::Relaxed);
        }
    }
    
    res
}

fn add_hugestring(str: &[u8]) -> *const u8 {
    let layout = Layout::array::<u8>(mem::size_of::<HugeString>() + str.len()).unwrap();
    let huge = unsafe { alloc(layout) as *mut HugeString };
    
    unsafe {
        (*huge).next = HUGESTRINGS.load(Ordering::Relaxed) as *mut HugeString;
        ptr::copy_nonoverlapping(str.as_ptr(), (*huge).buffer.as_mut_ptr(), str.len());
        *(*huge).buffer.as_mut_ptr().add(str.len()) = 0;
    }
    
    HUGESTRINGS.store(huge as usize, Ordering::Relaxed);
    unsafe { (*huge).buffer.as_ptr() }
}

fn str_hash(str: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    str.hash(&mut hasher);
    hasher.finish()
}

fn str_cache_iscached(str: *const u8) -> bool {
    let mut current = STRCACHE.load(Ordering::Relaxed) as *mut StrCache;
    
    while !current.is_null() {
        let start = unsafe { (*current).buffer.as_ptr() };
        let end = unsafe { start.add((*current).header.end as usize) };
        
        if str >= start && str < end {
            return true;
        }
        current = unsafe { (*current).header.next };
    }
    
    current = FULLCACHE.load(Ordering::Relaxed) as *mut StrCache;
    while !current.is_null() {
        let start = unsafe { (*current).buffer.as_ptr() };
        let end = unsafe { start.add((*current).header.end as usize) };
        
        if str >= start && str < end {
            return true;
        }
        current = unsafe { (*current).header.next };
    }
    
    let mut huge = HUGESTRINGS.load(Ordering::Relaxed) as *mut HugeString;
    while !huge.is_null() {
        if str == unsafe { (*huge).buffer.as_ptr() } {
            return true;
        }
        huge = unsafe { (*huge).next };
    }
    
    false
}

pub fn strcache_add(str: &str) -> *const u8 {
    add_hash(str.as_bytes())
}

pub fn strcache_add_len(str: &[u8]) -> *const u8 {
    if str.last() != Some(&0) {
        let mut owned = Vec::with_capacity(str.len() + 1);
        owned.extend_from_slice(str);
        owned.push(0);
        add_hash(&owned)
    } else {
        add_hash(str)
    }
}

fn add_hash(str: &[u8]) -> *const u8 {
    if str.len() > ScBuflenT::max_value() as usize - 1 {
        return add_hugestring(str);
    }
    
    TOTAL_ADDS.fetch_add(1, Ordering::Relaxed);
    
    // Hash table lookup would go here in a complete implementation
    add_string(str)
}

pub fn strcache_init() {
    // Initialize hash table
}

pub fn strcache_print_stats(prefix: &str) {
    // Implementation would mirror the C version's statistics printing
}