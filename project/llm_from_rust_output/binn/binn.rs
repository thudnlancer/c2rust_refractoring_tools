use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void, c_int, c_uchar, c_float, c_double};
use std::ptr::{null_mut, null, copy_nonoverlapping};
use std::cmp::Ordering;
use std::str::FromStr;

type BOOL = c_int;
type size_t = usize;
type int64 = i64;
type uint64 = u64;
type u32_0 = u32;
type u16_0 = u16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binn {
    header: c_int,
    allocated: BOOL,
    writable: BOOL,
    dirty: BOOL,
    pbuf: *mut c_void,
    pre_allocated: BOOL,
    alloc_size: c_int,
    used_size: c_int,
    type_: c_int,
    ptr: *mut c_void,
    size: c_int,
    count: c_int,
    freefn: Option<unsafe extern "C" fn(*mut c_void)>,
    c2rust_unnamed: C2RustUnnamed,
    disable_int_compression: BOOL,
}

#[repr(C)]
#[derive(Copy, Clone)]
union C2RustUnnamed {
    vint8: i8,
    vint16: i16,
    vint32: c_int,
    vint64: int64,
    vuint8: u8,
    vuint16: u16,
    vuint32: u32,
    vuint64: uint64,
    vchar: i8,
    vuchar: u8,
    vshort: i16,
    vushort: u16,
    vint: c_int,
    vuint: u32,
    vfloat: c_float,
    vdouble: c_double,
    vbool: BOOL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binn_iter {
    pnext: *mut c_uchar,
    plimit: *mut c_uchar,
    type_: c_int,
    count: c_int,
    current: c_int,
}

static mut malloc_fn: Option<unsafe extern "C" fn(size_t) -> *mut c_void> = None;
static mut realloc_fn: Option<unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void> = None;
static mut free_fn: Option<unsafe extern "C" fn(*mut c_void)> = None;

pub unsafe fn binn_version() -> *mut c_char {
    b"3.0.0\0".as_ptr() as *mut c_char
}

pub unsafe fn binn_set_alloc_functions(
    new_malloc: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
    new_realloc: Option<unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void>,
    new_free: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    malloc_fn = new_malloc;
    realloc_fn = new_realloc;
    free_fn = new_free;
}

unsafe fn check_alloc_functions() {
    if malloc_fn.is_none() {
        malloc_fn = Some(libc::malloc);
    }
    if realloc_fn.is_none() {
        realloc_fn = Some(libc::realloc);
    }
    if free_fn.is_none() {
        free_fn = Some(libc::free);
    }
}

unsafe fn binn_malloc(size: c_int) -> *mut c_void {
    check_alloc_functions();
    malloc_fn.expect("non-null function pointer")(size as size_t)
}

unsafe fn binn_memdup(src: *const c_void, size: c_int) -> *mut c_void {
    if src.is_null() || size <= 0 {
        return null_mut();
    }
    let dest = binn_malloc(size);
    if dest.is_null() {
        return null_mut();
    }
    libc::memcpy(dest, src, size as size_t);
    dest
}

unsafe fn strlen2(str: *mut c_char) -> size_t {
    if str.is_null() {
        return 0;
    }
    libc::strlen(str)
}

pub unsafe fn binn_create_type(storage_type: c_int, data_type_index: c_int) -> c_int {
    if data_type_index < 0 {
        return -1;
    }
    if storage_type < 0 || storage_type > 0xe0 {
        return -1;
    }
    if data_type_index < 16 {
        storage_type | data_type_index
    } else if data_type_index < 4096 {
        (storage_type | 0x10) << 8 | (data_type_index >> 4)
    } else {
        -1
    }
}

pub unsafe fn binn_get_type_info(
    long_type: c_int,
    pstorage_type: *mut c_int,
    pextra_type: *mut c_int,
) -> BOOL {
    let mut storage_type = 0;
    let mut extra_type = 0;
    let mut retval = 1;

    if long_type < 0 {
        storage_type = -1;
        extra_type = -1;
        retval = 0;
    } else if long_type <= 0xff {
        storage_type = long_type & 0xe0;
        extra_type = long_type & 0xf;
    } else if long_type <= 0xffff {
        storage_type = (long_type & 0xe000) >> 8;
        extra_type = (long_type & 0xfff) >> 4;
    } else if long_type & 0x80000 == 0 {
        storage_type = -1;
        extra_type = -1;
        retval = 0;
    } else {
        let long_type = long_type & 0xffff;
        storage_type = (long_type & 0xe000) >> 8;
        extra_type = (long_type & 0xfff) >> 4;
    }

    if !pstorage_type.is_null() {
        *pstorage_type = storage_type;
    }
    if !pextra_type.is_null() {
        *pextra_type = extra_type;
    }
    retval
}

pub unsafe fn binn_create(
    item: *mut binn,
    type_: c_int,
    size: c_int,
    pointer: *mut c_void,
) -> BOOL {
    let mut retval = 0;
    match type_ {
        224 | 225 | 226 => {
            if !item.is_null() && size >= 0 {
                if size < 3 && !pointer.is_null() {
                    return 0;
                }
                let size = if pointer.is_null() { if size == 0 { 256 } else { size } } else { size };
                libc::memset(
                    item as *mut c_void,
                    0,
                    mem::size_of::<binn>() as size_t,
                );
                let pointer = if pointer.is_null() {
                    let ptr = binn_malloc(size);
                    if ptr.is_null() {
                        return 0;
                    }
                    ptr
                } else {
                    pointer
                };
                (*item).pre_allocated = if pointer.is_null() { 0 } else { 1 };
                (*item).pbuf = pointer;
                (*item).alloc_size = size;
                (*item).header = 0x1f22b11f;
                (*item).writable = 1;
                (*item).used_size = 9;
                (*item).type_ = type_;
                (*item).dirty = 1;
                retval = 1;
            }
        }
        _ => {}
    }
    retval
}

pub unsafe fn binn_new(type_: c_int, size: c_int, pointer: *mut c_void) -> *mut binn {
    let item = binn_malloc(mem::size_of::<binn>() as c_int) as *mut binn;
    if binn_create(item, type_, size, pointer) == 0 {
        free_fn.expect("non-null function pointer")(item as *mut c_void);
        null_mut()
    } else {
        (*item).allocated = 1;
        item
    }
}

pub unsafe fn binn_create_list(list: *mut binn) -> BOOL {
    binn_create(list, 0xe0, 0, null_mut())
}

pub unsafe fn binn_create_map(map: *mut binn) -> BOOL {
    binn_create(map, 0xe1, 0, null_mut())
}

pub unsafe fn binn_create_object(object: *mut binn) -> BOOL {
    binn_create(object, 0xe2, 0, null_mut())
}

pub unsafe fn binn_list() -> *mut binn {
    binn_new(0xe0, 0, null_mut())
}

pub unsafe fn binn_map() -> *mut binn {
    binn_new(0xe1, 0, null_mut())
}

pub unsafe fn binn_object() -> *mut binn {
    binn_new(0xe2, 0, null_mut())
}

pub unsafe fn binn_copy(old: *const c_void) -> *mut binn {
    let mut type_ = 0;
    let mut count = 0;
    let mut size = 0;
    let mut header_size = 0;
    let old_ptr = binn_ptr(old) as *mut c_uchar;
    let mut item = null_mut();
    
    if IsValidBinnHeader(old_ptr as *const c_void, &mut type_, &mut count, &mut size, &mut header_size) == 0 {
        return null_mut();
    }
    
    item = binn_new(type_, size - header_size + 9, null_mut());
    if !item.is_null() {
        let dest = ((*item).pbuf as *mut c_uchar).offset(9);
        libc::memcpy(
            dest as *mut c_void,
            old_ptr.offset(header_size as isize) as *const c_void,
            (size - header_size) as size_t,
        );
        (*item).used_size = 9 + size - header_size;
        (*item).count = count;
    }
    item
}

pub unsafe fn binn_load(data: *const c_void, value: *mut binn) -> BOOL {
    if data.is_null() || value.is_null() {
        return 0;
    }
    libc::memset(
        value as *mut c_void,
        0,
        mem::size_of::<binn>() as size_t,
    );
    (*value).header = 0x1f22b11f;
    if binn_is_valid(data, &mut (*value).type_, &mut (*value).count, &mut (*value).size) == 0 {
        return 0;
    }
    (*value).ptr = data as *mut c_void;
    1
}

pub unsafe fn binn_load_ex(data: *const c_void, size: c_int, value: *mut binn) -> BOOL {
    if data.is_null() || value.is_null() || size <= 0 {
        return 0;
    }
    libc::memset(
        value as *mut c_void,
        0,
        mem::size_of::<binn>() as size_t,
    );
    (*value).header = 0x1f22b11f;
    let mut size = size;
    if binn_is_valid_ex(data, &mut (*value).type_, &mut (*value).count, &mut size) == 0 {
        return 0;
    }
    (*value).ptr = data as *mut c_void;
    (*value).size = size;
    1
}

pub unsafe fn binn_open(data: *const c_void) -> *mut binn {
    let item = binn_malloc(mem::size_of::<binn>() as c_int) as *mut binn;
    if binn_load(data, item) == 0 {
        free_fn.expect("non-null function pointer")(item as *mut c_void);
        null_mut()
    } else {
        (*item).allocated = 1;
        item
    }
}

pub unsafe fn binn_open_ex(data: *const c_void, size: c_int) -> *mut binn {
    let item = binn_malloc(mem::size_of::<binn>() as c_int) as *mut binn;
    if binn_load_ex(data, size, item) == 0 {
        free_fn.expect("non-null function pointer")(item as *mut c_void);
        null_mut()
    } else {
        (*item).allocated = 1;
        item
    }
}

unsafe fn binn_get_ptr_type(ptr: *const c_void) -> c_int {
    if ptr.is_null() {
        return 0;
    }
    match *(ptr as *const u32) {
        522367263 => 1,
        _ => 2,
    }
}

pub unsafe fn binn_is_struct(ptr: *const c_void) -> BOOL {
    if ptr.is_null() {
        return 0;
    }
    if *(ptr as *const u32) == 0x1f22b11f {
        1
    } else {
        0
    }
}

unsafe fn CalcAllocation(needed_size: c_int, alloc_size: c_int) -> c_int {
    let mut calc_size = alloc_size;
    while calc_size < needed_size {
        calc_size <<= 1;
    }
    calc_size
}

unsafe fn CheckAllocation(item: *mut binn, add_size: c_int) -> BOOL {
    if (*item).used_size + add_size > (*item).alloc_size {
        if (*item).pre_allocated != 0 {
            return 0;
        }
        let alloc_size = CalcAllocation((*item).used_size + add_size, (*item).alloc_size);
        let ptr = realloc_fn.expect("non-null function pointer")((*item).pbuf, alloc_size as size_t);
        if ptr.is_null() {
            return 0;
        }
        (*item).pbuf = ptr;
        (*item).alloc_size = alloc_size;
    }
    1
}

unsafe fn AdvanceDataPos(p: *mut c_uchar, plimit: *mut c_uchar) -> *mut c_uchar {
    if p > plimit {
        return null_mut();
    }
    let byte = *p;
    let mut p = p.offset(1);
    let storage_type = byte as c_int & 0xe0;
    if byte as c_int & 0x10 != 0 {
        p = p.offset(1);
    }
    match storage_type {
        0 => {}
        32 => {
            p = p.offset(1);
        }
        64 => {
            p = p.offset(2);
        }
        96 => {
            p = p.offset(4);
        }
        128 => {
            p = p.offset(8);
        }
        192 | 160 => {
            if p > plimit {
                return null_mut();
            }
            let mut DataSize = *p as c_int;
            if DataSize & 0x80 != 0 {
                if p.offset(mem::size_of::<c_int>() as isize - 1) > plimit {
                    return null_mut();
                }
                copy_be32(&mut DataSize as *mut c_int as *mut u32_0, p as *mut u32_0);
                DataSize &= 0x7fffffff;
                p = p.offset(4);
            } else {
                p = p.offset(1);
            }
            p = p.offset(DataSize as isize);
            if storage_type == 0xa0 {
                p = p.offset(1);
            }
        }
        224 => {
            if p > plimit {
                return null_mut();
            }
            let mut DataSize = *p as c_int;
            if DataSize & 0x80 != 0 {
                if p.offset(mem::size_of::<c_int>() as isize - 1) > plimit {
                    return null_mut();
                }
                copy_be32(&mut DataSize as *mut c_int as *mut u32_0, p as *mut u32_0);
                DataSize &= 0x7fffffff;
            }
            DataSize -= 1;
            p = p.offset(DataSize as isize);
        }
        _ => return null_mut(),
    }
    p
}

unsafe fn read_map_id(pp: *mut *mut c_uchar, plimit: *mut c_uchar) -> c_int {
    let mut p = *pp;
    if p > plimit {
        return 0;
    }
    let c = *p;
    p = p.offset(1);
    let mut extra_bytes = 0;
    if c as c_int & 0x80 != 0 {
        extra_bytes = ((c as c_int & 0x60) >> 5) + 1;
        if p.offset(extra_bytes as isize) > plimit {
            *pp = p.offset(extra_bytes as isize);
            return 0;
        }
    }
    let type_ = (c as c_int & 0xe0) as u8;
    let sign = (c as c_int & 0x10) as u8;
    let mut id = 0;
    if c as c_int & 0x80 == 0 {
        let sign = (c as c_int & 0x40) as u8;
        id = c as c_int & 0x3f;
    } else {
        match type_ as c_int {
            0x80 => {
                id = c as c_int & 0xf;
                id = id << 8 | *p as c_int;
                p = p.offset(1);
            }
            0xa0 => {
                id = c as c_int & 0xf;
                id = id << 8 | *p as c_int;
                p = p.offset(1);
                id = id << 8 | *p as c_int;
                p = p.offset(1);
            }
            0xc0 => {
                id = c as c_int