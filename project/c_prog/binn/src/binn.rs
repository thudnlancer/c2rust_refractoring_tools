use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type BOOL = libc::c_int;
pub type int64 = libc::c_longlong;
pub type uint64 = libc::c_ulonglong;
pub type binn_mem_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binn_struct {
    pub header: libc::c_int,
    pub allocated: BOOL,
    pub writable: BOOL,
    pub dirty: BOOL,
    pub pbuf: *mut libc::c_void,
    pub pre_allocated: BOOL,
    pub alloc_size: libc::c_int,
    pub used_size: libc::c_int,
    pub type_0: libc::c_int,
    pub ptr: *mut libc::c_void,
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub freefn: binn_mem_free,
    pub c2rust_unnamed: C2RustUnnamed,
    pub disable_int_compression: BOOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub vint8: libc::c_schar,
    pub vint16: libc::c_short,
    pub vint32: libc::c_int,
    pub vint64: int64,
    pub vuint8: libc::c_uchar,
    pub vuint16: libc::c_ushort,
    pub vuint32: libc::c_uint,
    pub vuint64: uint64,
    pub vchar: libc::c_schar,
    pub vuchar: libc::c_uchar,
    pub vshort: libc::c_short,
    pub vushort: libc::c_ushort,
    pub vint: libc::c_int,
    pub vuint: libc::c_uint,
    pub vfloat: libc::c_float,
    pub vdouble: libc::c_double,
    pub vbool: BOOL,
}
pub type binn = binn_struct;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type u16_0 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binn_iter_struct {
    pub pnext: *mut libc::c_uchar,
    pub plimit: *mut libc::c_uchar,
    pub type_0: libc::c_int,
    pub count: libc::c_int,
    pub current: libc::c_int,
}
pub type binn_iter = binn_iter_struct;
#[inline(always)]
unsafe extern "C" fn binn_list_add_value(
    mut list: *mut binn,
    mut value: *mut binn,
) -> BOOL {
    return binn_list_add(
        list,
        (*value).type_0,
        binn_ptr(value as *const libc::c_void),
        binn_size(value as *const libc::c_void),
    );
}
#[inline(always)]
unsafe extern "C" fn binn_map_set_value(
    mut map: *mut binn,
    mut id: libc::c_int,
    mut value: *mut binn,
) -> BOOL {
    return binn_map_set(
        map,
        id,
        (*value).type_0,
        binn_ptr(value as *const libc::c_void),
        binn_size(value as *const libc::c_void),
    );
}
#[inline(always)]
unsafe extern "C" fn binn_object_set_value(
    mut obj: *mut binn,
    mut key: *const libc::c_char,
    mut value: *mut binn,
) -> BOOL {
    return binn_object_set(
        obj,
        key,
        (*value).type_0,
        binn_ptr(value as *const libc::c_void),
        binn_size(value as *const libc::c_void),
    );
}
#[no_mangle]
pub static mut malloc_fn: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void> = None;
#[no_mangle]
pub static mut realloc_fn: Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
> = None;
#[no_mangle]
pub static mut free_fn: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()> = None;
unsafe extern "C" fn copy_be16(mut pdest: *mut u16_0, mut psource: *mut u16_0) {
    let mut source: *mut libc::c_uchar = psource as *mut libc::c_uchar;
    let mut dest: *mut libc::c_uchar = pdest as *mut libc::c_uchar;
    *dest.offset(0 as libc::c_int as isize) = *source.offset(1 as libc::c_int as isize);
    *dest.offset(1 as libc::c_int as isize) = *source.offset(0 as libc::c_int as isize);
}
unsafe extern "C" fn copy_be32(mut pdest: *mut u32_0, mut psource: *mut u32_0) {
    let mut source: *mut libc::c_uchar = psource as *mut libc::c_uchar;
    let mut dest: *mut libc::c_uchar = pdest as *mut libc::c_uchar;
    *dest.offset(0 as libc::c_int as isize) = *source.offset(3 as libc::c_int as isize);
    *dest.offset(1 as libc::c_int as isize) = *source.offset(2 as libc::c_int as isize);
    *dest.offset(2 as libc::c_int as isize) = *source.offset(1 as libc::c_int as isize);
    *dest.offset(3 as libc::c_int as isize) = *source.offset(0 as libc::c_int as isize);
}
unsafe extern "C" fn copy_be64(mut pdest: *mut u64_0, mut psource: *mut u64_0) {
    let mut source: *mut libc::c_uchar = psource as *mut libc::c_uchar;
    let mut dest: *mut libc::c_uchar = pdest as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *dest.offset(i as isize) = *source.offset((7 as libc::c_int - i) as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn binn_version() -> *mut libc::c_char {
    return b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn binn_set_alloc_functions(
    mut new_malloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    mut new_realloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    mut new_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    malloc_fn = new_malloc;
    realloc_fn = new_realloc;
    free_fn = new_free;
}
unsafe extern "C" fn check_alloc_functions() {
    if malloc_fn.is_none() {
        malloc_fn = Some(
            malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void,
        );
    }
    if realloc_fn.is_none() {
        realloc_fn = Some(
            realloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> *mut libc::c_void,
        );
    }
    if free_fn.is_none() {
        free_fn = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    }
}
unsafe extern "C" fn binn_malloc(mut size: libc::c_int) -> *mut libc::c_void {
    check_alloc_functions();
    return malloc_fn.expect("non-null function pointer")(size as size_t);
}
unsafe extern "C" fn binn_memdup(
    mut src: *const libc::c_void,
    mut size: libc::c_int,
) -> *mut libc::c_void {
    let mut dest: *mut libc::c_void = 0 as *mut libc::c_void;
    if src == 0 as *mut libc::c_void || size <= 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    dest = binn_malloc(size);
    if dest.is_null() {
        return 0 as *mut libc::c_void;
    }
    memcpy(dest, src, size as libc::c_ulong);
    return dest;
}
unsafe extern "C" fn strlen2(mut str: *mut libc::c_char) -> size_t {
    if str.is_null() {
        return 0 as libc::c_int as size_t;
    }
    return strlen(str);
}
#[no_mangle]
pub unsafe extern "C" fn binn_create_type(
    mut storage_type: libc::c_int,
    mut data_type_index: libc::c_int,
) -> libc::c_int {
    if data_type_index < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if storage_type < 0 as libc::c_int || storage_type > 0xe0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if data_type_index < 16 as libc::c_int {
        return storage_type | data_type_index
    } else if data_type_index < 4096 as libc::c_int {
        storage_type |= 0x10 as libc::c_int;
        storage_type <<= 8 as libc::c_int;
        data_type_index >>= 4 as libc::c_int;
        return storage_type | data_type_index;
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_type_info(
    mut long_type: libc::c_int,
    mut pstorage_type: *mut libc::c_int,
    mut pextra_type: *mut libc::c_int,
) -> BOOL {
    let mut storage_type: libc::c_int = 0;
    let mut extra_type: libc::c_int = 0;
    let mut retval: BOOL = 1 as libc::c_int;
    let mut current_block_11: u64;
    loop {
        if long_type < 0 as libc::c_int {
            current_block_11 = 1343253133452569337;
            break;
        }
        if long_type <= 0xff as libc::c_int {
            storage_type = long_type & 0xe0 as libc::c_int;
            extra_type = long_type & 0xf as libc::c_int;
            current_block_11 = 10048703153582371463;
            break;
        } else if long_type <= 0xffff as libc::c_int {
            storage_type = long_type & 0xe000 as libc::c_int;
            storage_type >>= 8 as libc::c_int;
            extra_type = long_type & 0xfff as libc::c_int;
            extra_type >>= 4 as libc::c_int;
            current_block_11 = 10048703153582371463;
            break;
        } else {
            if !(long_type & 0x80000 as libc::c_int != 0) {
                current_block_11 = 1343253133452569337;
                break;
            }
            long_type &= 0xffff as libc::c_int;
        }
    }
    match current_block_11 {
        1343253133452569337 => {
            storage_type = -(1 as libc::c_int);
            extra_type = -(1 as libc::c_int);
            retval = 0 as libc::c_int;
        }
        _ => {}
    }
    if !pstorage_type.is_null() {
        *pstorage_type = storage_type;
    }
    if !pextra_type.is_null() {
        *pextra_type = extra_type;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_create(
    mut item: *mut binn,
    mut type_0: libc::c_int,
    mut size: libc::c_int,
    mut pointer: *mut libc::c_void,
) -> BOOL {
    let mut current_block: u64;
    let mut retval: BOOL = 0 as libc::c_int;
    match type_0 {
        224 | 225 | 226 => {
            if !(item.is_null() || size < 0 as libc::c_int) {
                if size < 3 as libc::c_int {
                    if !pointer.is_null() {
                        current_block = 15072273856331234935;
                    } else {
                        size = 0 as libc::c_int;
                        current_block = 10879442775620481940;
                    }
                } else {
                    current_block = 10879442775620481940;
                }
                match current_block {
                    15072273856331234935 => {}
                    _ => {
                        memset(
                            item as *mut libc::c_void,
                            0 as libc::c_int,
                            ::core::mem::size_of::<binn>() as libc::c_ulong,
                        );
                        if !pointer.is_null() {
                            (*item).pre_allocated = 1 as libc::c_int;
                        } else {
                            (*item).pre_allocated = 0 as libc::c_int;
                            if size == 0 as libc::c_int {
                                size = 256 as libc::c_int;
                            }
                            pointer = binn_malloc(size);
                            if pointer.is_null() {
                                return 0 as libc::c_int;
                            }
                        }
                        (*item).pbuf = pointer;
                        (*item).alloc_size = size;
                        (*item).header = 0x1f22b11f as libc::c_int;
                        (*item).writable = 1 as libc::c_int;
                        (*item).used_size = 9 as libc::c_int;
                        (*item).type_0 = type_0;
                        (*item).dirty = 1 as libc::c_int;
                        retval = 1 as libc::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_new(
    mut type_0: libc::c_int,
    mut size: libc::c_int,
    mut pointer: *mut libc::c_void,
) -> *mut binn {
    let mut item: *mut binn = 0 as *mut binn;
    item = binn_malloc(::core::mem::size_of::<binn>() as libc::c_ulong as libc::c_int)
        as *mut binn;
    if binn_create(item, type_0, size, pointer) == 0 as libc::c_int {
        free_fn.expect("non-null function pointer")(item as *mut libc::c_void);
        return 0 as *mut binn;
    }
    (*item).allocated = 1 as libc::c_int;
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_create_list(mut list: *mut binn) -> BOOL {
    return binn_create(
        list,
        0xe0 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_create_map(mut map: *mut binn) -> BOOL {
    return binn_create(
        map,
        0xe1 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_create_object(mut object: *mut binn) -> BOOL {
    return binn_create(
        object,
        0xe2 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_list() -> *mut binn {
    return binn_new(0xe0 as libc::c_int, 0 as libc::c_int, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map() -> *mut binn {
    return binn_new(0xe1 as libc::c_int, 0 as libc::c_int, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object() -> *mut binn {
    return binn_new(0xe2 as libc::c_int, 0 as libc::c_int, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn binn_copy(mut old: *const libc::c_void) -> *mut binn {
    let mut type_0: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut header_size: libc::c_int = 0;
    let mut old_ptr: *mut libc::c_uchar = binn_ptr(old) as *mut libc::c_uchar;
    let mut item: *mut binn = 0 as *mut binn;
    size = 0 as libc::c_int;
    if IsValidBinnHeader(
        old_ptr as *const libc::c_void,
        &mut type_0,
        &mut count,
        &mut size,
        &mut header_size,
    ) == 0
    {
        return 0 as *mut binn;
    }
    item = binn_new(
        type_0,
        size - header_size + 9 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if !item.is_null() {
        let mut dest: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        dest = ((*item).pbuf as *mut libc::c_uchar).offset(9 as libc::c_int as isize);
        memcpy(
            dest as *mut libc::c_void,
            old_ptr.offset(header_size as isize) as *const libc::c_void,
            (size - header_size) as libc::c_ulong,
        );
        (*item).used_size = 9 as libc::c_int + size - header_size;
        (*item).count = count;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_load(
    mut data: *const libc::c_void,
    mut value: *mut binn,
) -> BOOL {
    if data == 0 as *mut libc::c_void || value.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        value as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<binn>() as libc::c_ulong,
    );
    (*value).header = 0x1f22b11f as libc::c_int;
    if binn_is_valid(data, &mut (*value).type_0, &mut (*value).count, &mut (*value).size)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    (*value).ptr = data as *mut libc::c_void;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_load_ex(
    mut data: *const libc::c_void,
    mut size: libc::c_int,
    mut value: *mut binn,
) -> BOOL {
    if data == 0 as *mut libc::c_void || value.is_null() || size <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    memset(
        value as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<binn>() as libc::c_ulong,
    );
    (*value).header = 0x1f22b11f as libc::c_int;
    if binn_is_valid_ex(data, &mut (*value).type_0, &mut (*value).count, &mut size)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    (*value).ptr = data as *mut libc::c_void;
    (*value).size = size;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_open(mut data: *const libc::c_void) -> *mut binn {
    let mut item: *mut binn = 0 as *mut binn;
    item = binn_malloc(::core::mem::size_of::<binn>() as libc::c_ulong as libc::c_int)
        as *mut binn;
    if binn_load(data, item) == 0 as libc::c_int {
        free_fn.expect("non-null function pointer")(item as *mut libc::c_void);
        return 0 as *mut binn;
    }
    (*item).allocated = 1 as libc::c_int;
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_open_ex(
    mut data: *const libc::c_void,
    mut size: libc::c_int,
) -> *mut binn {
    let mut item: *mut binn = 0 as *mut binn;
    item = binn_malloc(::core::mem::size_of::<binn>() as libc::c_ulong as libc::c_int)
        as *mut binn;
    if binn_load_ex(data, size, item) == 0 as libc::c_int {
        free_fn.expect("non-null function pointer")(item as *mut libc::c_void);
        return 0 as *mut binn;
    }
    (*item).allocated = 1 as libc::c_int;
    return item;
}
unsafe extern "C" fn binn_get_ptr_type(mut ptr: *const libc::c_void) -> libc::c_int {
    if ptr == 0 as *mut libc::c_void {
        return 0 as libc::c_int;
    }
    match *(ptr as *mut libc::c_uint) {
        522367263 => return 1 as libc::c_int,
        _ => return 2 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_is_struct(mut ptr: *const libc::c_void) -> BOOL {
    if ptr == 0 as *mut libc::c_void {
        return 0 as libc::c_int;
    }
    if *(ptr as *mut libc::c_uint) == 0x1f22b11f as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn CalcAllocation(
    mut needed_size: libc::c_int,
    mut alloc_size: libc::c_int,
) -> libc::c_int {
    let mut calc_size: libc::c_int = 0;
    calc_size = alloc_size;
    while calc_size < needed_size {
        calc_size <<= 1 as libc::c_int;
    }
    return calc_size;
}
unsafe extern "C" fn CheckAllocation(
    mut item: *mut binn,
    mut add_size: libc::c_int,
) -> BOOL {
    let mut alloc_size: libc::c_int = 0;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*item).used_size + add_size > (*item).alloc_size {
        if (*item).pre_allocated != 0 {
            return 0 as libc::c_int;
        }
        alloc_size = CalcAllocation((*item).used_size + add_size, (*item).alloc_size);
        ptr = realloc_fn
            .expect("non-null function pointer")((*item).pbuf, alloc_size as size_t);
        if ptr.is_null() {
            return 0 as libc::c_int;
        }
        (*item).pbuf = ptr;
        (*item).alloc_size = alloc_size;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn AdvanceDataPos(
    mut p: *mut libc::c_uchar,
    mut plimit: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    let mut byte: libc::c_uchar = 0;
    let mut storage_type: libc::c_int = 0;
    let mut DataSize: libc::c_int = 0;
    if p > plimit {
        return 0 as *mut libc::c_uchar;
    }
    byte = *p;
    p = p.offset(1);
    p;
    storage_type = byte as libc::c_int & 0xe0 as libc::c_int;
    if byte as libc::c_int & 0x10 as libc::c_int != 0 {
        p = p.offset(1);
        p;
    }
    match storage_type {
        0 => {}
        32 => {
            p = p.offset(1);
            p;
        }
        64 => {
            p = p.offset(2 as libc::c_int as isize);
        }
        96 => {
            p = p.offset(4 as libc::c_int as isize);
        }
        128 => {
            p = p.offset(8 as libc::c_int as isize);
        }
        192 | 160 => {
            if p > plimit {
                return 0 as *mut libc::c_uchar;
            }
            DataSize = *p as libc::c_int;
            if DataSize & 0x80 as libc::c_int != 0 {
                if p
                    .offset(
                        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as isize,
                    )
                    .offset(-(1 as libc::c_int as isize)) > plimit
                {
                    return 0 as *mut libc::c_uchar;
                }
                copy_be32(
                    &mut DataSize as *mut libc::c_int as *mut u32_0,
                    p as *mut u32_0,
                );
                DataSize &= 0x7fffffff as libc::c_int;
                p = p.offset(4 as libc::c_int as isize);
            } else {
                p = p.offset(1);
                p;
            }
            p = p.offset(DataSize as isize);
            if storage_type == 0xa0 as libc::c_int {
                p = p.offset(1);
                p;
            }
        }
        224 => {
            if p > plimit {
                return 0 as *mut libc::c_uchar;
            }
            DataSize = *p as libc::c_int;
            if DataSize & 0x80 as libc::c_int != 0 {
                if p
                    .offset(
                        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as isize,
                    )
                    .offset(-(1 as libc::c_int as isize)) > plimit
                {
                    return 0 as *mut libc::c_uchar;
                }
                copy_be32(
                    &mut DataSize as *mut libc::c_int as *mut u32_0,
                    p as *mut u32_0,
                );
                DataSize &= 0x7fffffff as libc::c_int;
            }
            DataSize -= 1;
            DataSize;
            p = p.offset(DataSize as isize);
        }
        _ => return 0 as *mut libc::c_uchar,
    }
    return p;
}
unsafe extern "C" fn read_map_id(
    mut pp: *mut *mut libc::c_uchar,
    mut plimit: *mut libc::c_uchar,
) -> libc::c_int {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut c: libc::c_uchar = 0;
    let mut sign: libc::c_uchar = 0;
    let mut type_0: libc::c_uchar = 0;
    let mut id: libc::c_int = 0;
    let mut extra_bytes: libc::c_int = 0;
    p = *pp;
    if p > plimit {
        return 0 as libc::c_int;
    }
    let fresh0 = p;
    p = p.offset(1);
    c = *fresh0;
    if c as libc::c_int & 0x80 as libc::c_int != 0 {
        extra_bytes = ((c as libc::c_int & 0x60 as libc::c_int) >> 5 as libc::c_int)
            + 1 as libc::c_int;
        if p.offset(extra_bytes as isize) > plimit {
            *pp = p.offset(extra_bytes as isize);
            return 0 as libc::c_int;
        }
    }
    type_0 = (c as libc::c_int & 0xe0 as libc::c_int) as libc::c_uchar;
    sign = (c as libc::c_int & 0x10 as libc::c_int) as libc::c_uchar;
    if c as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
        sign = (c as libc::c_int & 0x40 as libc::c_int) as libc::c_uchar;
        id = c as libc::c_int & 0x3f as libc::c_int;
    } else if type_0 as libc::c_int == 0x80 as libc::c_int {
        id = c as libc::c_int & 0xf as libc::c_int;
        let fresh1 = p;
        p = p.offset(1);
        id = id << 8 as libc::c_int | *fresh1 as libc::c_int;
    } else if type_0 as libc::c_int == 0xa0 as libc::c_int {
        id = c as libc::c_int & 0xf as libc::c_int;
        let fresh2 = p;
        p = p.offset(1);
        id = id << 8 as libc::c_int | *fresh2 as libc::c_int;
        let fresh3 = p;
        p = p.offset(1);
        id = id << 8 as libc::c_int | *fresh3 as libc::c_int;
    } else if type_0 as libc::c_int == 0xc0 as libc::c_int {
        id = c as libc::c_int & 0xf as libc::c_int;
        let fresh4 = p;
        p = p.offset(1);
        id = id << 8 as libc::c_int | *fresh4 as libc::c_int;
        let fresh5 = p;
        p = p.offset(1);
        id = id << 8 as libc::c_int | *fresh5 as libc::c_int;
        let fresh6 = p;
        p = p.offset(1);
        id = id << 8 as libc::c_int | *fresh6 as libc::c_int;
    } else if type_0 as libc::c_int == 0xe0 as libc::c_int {
        copy_be32(&mut id as *mut libc::c_int as *mut u32_0, p as *mut u32_0);
        p = p.offset(4 as libc::c_int as isize);
    } else {
        *pp = plimit.offset(2 as libc::c_int as isize);
        return 0 as libc::c_int;
    }
    if sign != 0 {
        id = -id;
    }
    *pp = p;
    return id;
}
unsafe extern "C" fn SearchForID(
    mut p: *mut libc::c_uchar,
    mut header_size: libc::c_int,
    mut size: libc::c_int,
    mut numitems: libc::c_int,
    mut id: libc::c_int,
) -> *mut libc::c_uchar {
    let mut plimit: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut base: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut int32: libc::c_int = 0;
    base = p;
    plimit = p.offset(size as isize).offset(-(1 as libc::c_int as isize));
    p = p.offset(header_size as isize);
    i = 0 as libc::c_int;
    while i < numitems {
        int32 = read_map_id(&mut p, plimit);
        if p > plimit {
            break;
        }
        if int32 == id {
            return p;
        }
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            break;
        }
        i += 1;
        i;
    }
    return 0 as *mut libc::c_uchar;
}
unsafe extern "C" fn SearchForKey(
    mut p: *mut libc::c_uchar,
    mut header_size: libc::c_int,
    mut size: libc::c_int,
    mut numitems: libc::c_int,
    mut key: *const libc::c_char,
) -> *mut libc::c_uchar {
    let mut len: libc::c_uchar = 0;
    let mut plimit: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut base: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut keylen: libc::c_int = 0;
    base = p;
    plimit = p.offset(size as isize).offset(-(1 as libc::c_int as isize));
    p = p.offset(header_size as isize);
    keylen = strlen(key) as libc::c_int;
    i = 0 as libc::c_int;
    while i < numitems {
        if p > plimit {
            break;
        }
        len = *p;
        p = p.offset(1);
        p;
        if p.offset(len as libc::c_int as isize) > plimit {
            break;
        }
        if len as libc::c_int > 0 as libc::c_int {
            if strncasecmp(p as *mut libc::c_char, key, len as libc::c_ulong)
                == 0 as libc::c_int
            {
                if keylen == len as libc::c_int {
                    p = p.offset(len as libc::c_int as isize);
                    return p;
                }
            }
            p = p.offset(len as libc::c_int as isize);
        } else if len as libc::c_int == keylen {
            return p
        }
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            break;
        }
        i += 1;
        i;
    }
    return 0 as *mut libc::c_uchar;
}
unsafe extern "C" fn binn_list_add_raw(
    mut item: *mut binn,
    mut type_0: libc::c_int,
    mut pvalue: *mut libc::c_void,
    mut size: libc::c_int,
) -> BOOL {
    if item.is_null() || (*item).type_0 != 0xe0 as libc::c_int
        || (*item).writable == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if AddValue(item, type_0, pvalue, size) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*item).count += 1;
    (*item).count;
    return 1 as libc::c_int;
}
unsafe extern "C" fn binn_object_set_raw(
    mut item: *mut binn,
    mut key: *const libc::c_char,
    mut type_0: libc::c_int,
    mut pvalue: *mut libc::c_void,
    mut size: libc::c_int,
) -> BOOL {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: libc::c_uchar = 0;
    let mut int32: libc::c_int = 0;
    if item.is_null() || (*item).type_0 != 0xe2 as libc::c_int
        || (*item).writable == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if key.is_null() {
        return 0 as libc::c_int;
    }
    int32 = strlen(key) as libc::c_int;
    if int32 > 255 as libc::c_int {
        return 0 as libc::c_int;
    }
    p = SearchForKey(
        (*item).pbuf as *mut libc::c_uchar,
        9 as libc::c_int,
        (*item).used_size,
        (*item).count,
        key,
    );
    if !p.is_null() {
        return 0 as libc::c_int;
    }
    if CheckAllocation(item, 1 as libc::c_int + int32) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    p = ((*item).pbuf as *mut libc::c_uchar).offset((*item).used_size as isize);
    len = int32 as libc::c_uchar;
    *p = len;
    p = p.offset(1);
    p;
    memcpy(p as *mut libc::c_void, key as *const libc::c_void, int32 as libc::c_ulong);
    int32 += 1;
    int32;
    (*item).used_size += int32;
    if AddValue(item, type_0, pvalue, size) == 0 as libc::c_int {
        (*item).used_size -= int32;
        return 0 as libc::c_int;
    }
    (*item).count += 1;
    (*item).count;
    return 1 as libc::c_int;
}
unsafe extern "C" fn binn_map_set_raw(
    mut item: *mut binn,
    mut id: libc::c_int,
    mut type_0: libc::c_int,
    mut pvalue: *mut libc::c_void,
    mut size: libc::c_int,
) -> BOOL {
    let mut base: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sign: libc::c_uchar = 0;
    let mut id_size: libc::c_int = 0;
    if item.is_null() || (*item).type_0 != 0xe1 as libc::c_int
        || (*item).writable == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    p = SearchForID(
        (*item).pbuf as *mut libc::c_uchar,
        9 as libc::c_int,
        (*item).used_size,
        (*item).count,
        id,
    );
    if !p.is_null() {
        return 0 as libc::c_int;
    }
    if CheckAllocation(item, 5 as libc::c_int) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    base = ((*item).pbuf as *mut libc::c_uchar).offset((*item).used_size as isize);
    p = base;
    sign = (id < 0 as libc::c_int) as libc::c_int as libc::c_uchar;
    if sign != 0 {
        id = -id;
    }
    if id <= 0x3f as libc::c_int {
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = ((sign as libc::c_int) << 6 as libc::c_int | id) as libc::c_uchar;
    } else if id <= 0xfff as libc::c_int {
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = (0x80 as libc::c_int | (sign as libc::c_int) << 4 as libc::c_int
            | (id & 0xf00 as libc::c_int) >> 8 as libc::c_int) as libc::c_uchar;
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = (id & 0xff as libc::c_int) as libc::c_uchar;
    } else if id <= 0xfffff as libc::c_int {
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = (0xa0 as libc::c_int | (sign as libc::c_int) << 4 as libc::c_int
            | (id & 0xf0000 as libc::c_int) >> 16 as libc::c_int) as libc::c_uchar;
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = ((id & 0xff00 as libc::c_int) >> 8 as libc::c_int) as libc::c_uchar;
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = (id & 0xff as libc::c_int) as libc::c_uchar;
    } else if id <= 0xfffffff as libc::c_int {
        let fresh13 = p;
        p = p.offset(1);
        *fresh13 = (0xc0 as libc::c_int | (sign as libc::c_int) << 4 as libc::c_int
            | (id & 0xf000000 as libc::c_int) >> 24 as libc::c_int) as libc::c_uchar;
        let fresh14 = p;
        p = p.offset(1);
        *fresh14 = ((id & 0xff0000 as libc::c_int) >> 16 as libc::c_int)
            as libc::c_uchar;
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = ((id & 0xff00 as libc::c_int) >> 8 as libc::c_int) as libc::c_uchar;
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = (id & 0xff as libc::c_int) as libc::c_uchar;
    } else {
        let fresh17 = p;
        p = p.offset(1);
        *fresh17 = 0xe0 as libc::c_int as libc::c_uchar;
        if sign != 0 {
            id = -id;
        }
        copy_be32(p as *mut u32_0, &mut id as *mut libc::c_int as *mut u32_0);
        p = p.offset(4 as libc::c_int as isize);
    }
    id_size = p.offset_from(base) as libc::c_long as libc::c_int;
    (*item).used_size += id_size;
    if AddValue(item, type_0, pvalue, size) == 0 as libc::c_int {
        (*item).used_size -= id_size;
        return 0 as libc::c_int;
    }
    (*item).count += 1;
    (*item).count;
    return 1 as libc::c_int;
}
unsafe extern "C" fn compress_int(
    mut pstorage_type: *mut libc::c_int,
    mut ptype: *mut libc::c_int,
    mut psource: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut storage_type: libc::c_int = 0;
    let mut storage_type2: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut type2: libc::c_int = 0 as libc::c_int;
    let mut vint: int64 = 0 as libc::c_int as int64;
    let mut vuint: uint64 = 0;
    let mut pvalue: *mut libc::c_char = 0 as *mut libc::c_char;
    storage_type = *pstorage_type;
    if storage_type == 0x20 as libc::c_int {
        return psource;
    }
    type_0 = *ptype;
    match type_0 {
        129 => {
            vint = *(psource as *mut int64);
            current_block = 10112269340892363896;
        }
        97 => {
            vint = *(psource as *mut libc::c_int) as int64;
            current_block = 10112269340892363896;
        }
        65 => {
            vint = *(psource as *mut libc::c_short) as int64;
            current_block = 10112269340892363896;
        }
        128 => {
            vuint = *(psource as *mut uint64);
            current_block = 12570103875192772683;
        }
        96 => {
            vuint = *(psource as *mut libc::c_uint) as uint64;
            current_block = 12570103875192772683;
        }
        64 => {
            vuint = *(psource as *mut libc::c_ushort) as uint64;
            current_block = 12570103875192772683;
        }
        _ => {
            current_block = 10112269340892363896;
        }
    }
    match current_block {
        10112269340892363896 => {
            if vint >= 0 as libc::c_int as libc::c_longlong {
                vuint = vint as uint64;
                current_block = 12570103875192772683;
            } else {
                if vint >= -(128 as libc::c_int) as libc::c_longlong {
                    type2 = 0x21 as libc::c_int;
                } else if vint
                    >= (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_longlong
                {
                    type2 = 0x41 as libc::c_int;
                } else if vint
                    >= (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_longlong
                {
                    type2 = 0x61 as libc::c_int;
                }
                current_block = 4915007825643337524;
            }
        }
        _ => {}
    }
    match current_block {
        12570103875192772683 => {
            if vuint <= 255 as libc::c_int as libc::c_ulonglong {
                type2 = 0x20 as libc::c_int;
            } else if vuint <= 65535 as libc::c_int as libc::c_ulonglong {
                type2 = 0x40 as libc::c_int;
            } else if vuint <= 4294967295 as libc::c_uint as libc::c_ulonglong {
                type2 = 0x60 as libc::c_int;
            }
        }
        _ => {}
    }
    pvalue = psource as *mut libc::c_char;
    if type2 != 0 && type2 != type_0 {
        *ptype = type2;
        storage_type2 = binn_get_write_storage(type2);
        *pstorage_type = storage_type2;
    }
    return pvalue as *mut libc::c_void;
}
unsafe extern "C" fn AddValue(
    mut item: *mut binn,
    mut type_0: libc::c_int,
    mut pvalue: *mut libc::c_void,
    mut size: libc::c_int,
) -> BOOL {
    let mut int32: libc::c_int = 0;
    let mut ArgSize: libc::c_int = 0;
    let mut storage_type: libc::c_int = 0;
    let mut extra_type: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    binn_get_type_info(type_0, &mut storage_type, &mut extra_type);
    if pvalue.is_null() {
        let mut current_block_1: u64;
        match storage_type {
            0 => {
                current_block_1 = 6873731126896040597;
            }
            192 | 160 => {
                if size == 0 as libc::c_int {
                    current_block_1 = 6873731126896040597;
                } else {
                    current_block_1 = 6246763041535604849;
                }
            }
            _ => {
                current_block_1 = 6246763041535604849;
            }
        }
        match current_block_1 {
            6246763041535604849 => return 0 as libc::c_int,
            _ => {}
        }
    }
    if type_family(type_0) == 0xf2 as libc::c_int
        && (*item).disable_int_compression == 0 as libc::c_int
    {
        pvalue = compress_int(&mut storage_type, &mut type_0, pvalue);
    }
    match storage_type {
        0 => {
            size = 0 as libc::c_int;
            ArgSize = size;
        }
        32 => {
            size = 1 as libc::c_int;
            ArgSize = size;
        }
        64 => {
            size = 2 as libc::c_int;
            ArgSize = size;
        }
        96 => {
            size = 4 as libc::c_int;
            ArgSize = size;
        }
        128 => {
            size = 8 as libc::c_int;
            ArgSize = size;
        }
        192 => {
            if size < 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            ArgSize = size + 4 as libc::c_int;
        }
        160 => {
            if size < 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            if size == 0 as libc::c_int {
                size = strlen2(pvalue as *mut libc::c_char) as libc::c_int;
            }
            ArgSize = size + 5 as libc::c_int;
        }
        224 => {
            if size <= 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            ArgSize = size;
        }
        _ => return 0 as libc::c_int,
    }
    ArgSize += 2 as libc::c_int;
    if CheckAllocation(item, ArgSize) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    p = ((*item).pbuf as *mut libc::c_uchar).offset((*item).used_size as isize);
    if storage_type != 0xe0 as libc::c_int {
        if type_0 > 255 as libc::c_int {
            let mut type16: u16_0 = type_0 as u16_0;
            copy_be16(p as *mut u16_0, &mut type16 as *mut u16_0);
            p = p.offset(2 as libc::c_int as isize);
            (*item).used_size += 2 as libc::c_int;
        } else {
            *p = type_0 as libc::c_uchar;
            p = p.offset(1);
            p;
            (*item).used_size += 1;
            (*item).used_size;
        }
    }
    match storage_type {
        32 => {
            *(p as *mut libc::c_char) = *(pvalue as *mut libc::c_char);
            (*item).used_size += 1 as libc::c_int;
        }
        64 => {
            copy_be16(p as *mut u16_0, pvalue as *mut u16_0);
            (*item).used_size += 2 as libc::c_int;
        }
        96 => {
            copy_be32(p as *mut u32_0, pvalue as *mut u32_0);
            (*item).used_size += 4 as libc::c_int;
        }
        128 => {
            copy_be64(p as *mut u64_0, pvalue as *mut u64_0);
            (*item).used_size += 8 as libc::c_int;
        }
        192 | 160 => {
            if size > 127 as libc::c_int {
                int32 = (size as libc::c_uint | 0x80000000 as libc::c_uint)
                    as libc::c_int;
                copy_be32(p as *mut u32_0, &mut int32 as *mut libc::c_int as *mut u32_0);
                p = p.offset(4 as libc::c_int as isize);
                (*item).used_size += 4 as libc::c_int;
            } else {
                *p = size as libc::c_uchar;
                p = p.offset(1);
                p;
                (*item).used_size += 1;
                (*item).used_size;
            }
            memcpy(p as *mut libc::c_void, pvalue, size as libc::c_ulong);
            if storage_type == 0xa0 as libc::c_int {
                p = p.offset(size as isize);
                *(p as *mut libc::c_char) = 0 as libc::c_int as libc::c_char;
                size += 1;
                size;
            }
            (*item).used_size += size;
        }
        224 => {
            memcpy(p as *mut libc::c_void, pvalue, size as libc::c_ulong);
            (*item).used_size += size;
        }
        0 | _ => {}
    }
    (*item).dirty = 1 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn binn_save_header(mut item: *mut binn) -> BOOL {
    let mut byte: libc::c_uchar = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut int32: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if item.is_null() {
        return 0 as libc::c_int;
    }
    p = ((*item).pbuf as *mut libc::c_uchar).offset(9 as libc::c_int as isize);
    size = (*item).used_size - 9 as libc::c_int + 3 as libc::c_int;
    if (*item).count > 127 as libc::c_int {
        p = p.offset(-(4 as libc::c_int as isize));
        size += 3 as libc::c_int;
        int32 = ((*item).count as libc::c_uint | 0x80000000 as libc::c_uint)
            as libc::c_int;
        copy_be32(p as *mut u32_0, &mut int32 as *mut libc::c_int as *mut u32_0);
    } else {
        p = p.offset(-1);
        p;
        *p = (*item).count as libc::c_uchar;
    }
    if size > 127 as libc::c_int {
        p = p.offset(-(4 as libc::c_int as isize));
        size += 3 as libc::c_int;
        int32 = (size as libc::c_uint | 0x80000000 as libc::c_uint) as libc::c_int;
        copy_be32(p as *mut u32_0, &mut int32 as *mut libc::c_int as *mut u32_0);
    } else {
        p = p.offset(-1);
        p;
        *p = size as libc::c_uchar;
    }
    p = p.offset(-1);
    p;
    *p = (*item).type_0 as libc::c_uchar;
    (*item).ptr = p as *mut libc::c_void;
    (*item).size = size;
    (*item).dirty = 0 as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_free(mut item: *mut binn) {
    if item.is_null() {
        return;
    }
    if (*item).writable != 0 && (*item).pre_allocated == 0 as libc::c_int {
        free_fn.expect("non-null function pointer")((*item).pbuf);
    }
    if ((*item).freefn).is_some() {
        ((*item).freefn).expect("non-null function pointer")((*item).ptr);
    }
    if (*item).allocated != 0 {
        free_fn.expect("non-null function pointer")(item as *mut libc::c_void);
    } else {
        memset(
            item as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<binn>() as libc::c_ulong,
        );
        (*item).header = 0x1f22b11f as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_release(mut item: *mut binn) -> *mut libc::c_void {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    if item.is_null() {
        return 0 as *mut libc::c_void;
    }
    data = binn_ptr(item as *const libc::c_void);
    if data > (*item).pbuf {
        memmove((*item).pbuf, data, (*item).size as libc::c_ulong);
        data = (*item).pbuf;
    }
    if (*item).allocated != 0 {
        free_fn.expect("non-null function pointer")(item as *mut libc::c_void);
    } else {
        memset(
            item as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<binn>() as libc::c_ulong,
        );
        (*item).header = 0x1f22b11f as libc::c_int;
    }
    return data;
}
unsafe extern "C" fn IsValidBinnHeader(
    mut pbuf: *const libc::c_void,
    mut ptype: *mut libc::c_int,
    mut pcount: *mut libc::c_int,
    mut psize: *mut libc::c_int,
    mut pheadersize: *mut libc::c_int,
) -> BOOL {
    let mut byte: libc::c_uchar = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut plimit: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut int32: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    if pbuf == 0 as *mut libc::c_void {
        return 0 as libc::c_int;
    }
    p = pbuf as *mut libc::c_uchar;
    if !psize.is_null() && *psize > 0 as libc::c_int {
        if *psize < 3 as libc::c_int {
            return 0 as libc::c_int;
        }
        plimit = p.offset(*psize as isize).offset(-(1 as libc::c_int as isize));
    }
    byte = *p;
    p = p.offset(1);
    p;
    if byte as libc::c_int & 0xe0 as libc::c_int != 0xe0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if byte as libc::c_int & 0x10 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    type_0 = byte as libc::c_int;
    match type_0 {
        224 | 225 | 226 => {}
        _ => return 0 as libc::c_int,
    }
    if !plimit.is_null() && p > plimit {
        return 0 as libc::c_int;
    }
    int32 = *p as libc::c_int;
    if int32 & 0x80 as libc::c_int != 0 {
        if !plimit.is_null()
            && p
                .offset(::core::mem::size_of::<libc::c_int>() as libc::c_ulong as isize)
                .offset(-(1 as libc::c_int as isize)) > plimit
        {
            return 0 as libc::c_int;
        }
        copy_be32(&mut int32 as *mut libc::c_int as *mut u32_0, p as *mut u32_0);
        int32 &= 0x7fffffff as libc::c_int;
        p = p.offset(4 as libc::c_int as isize);
    } else {
        p = p.offset(1);
        p;
    }
    size = int32;
    if !plimit.is_null() && p > plimit {
        return 0 as libc::c_int;
    }
    int32 = *p as libc::c_int;
    if int32 & 0x80 as libc::c_int != 0 {
        if !plimit.is_null()
            && p
                .offset(::core::mem::size_of::<libc::c_int>() as libc::c_ulong as isize)
                .offset(-(1 as libc::c_int as isize)) > plimit
        {
            return 0 as libc::c_int;
        }
        copy_be32(&mut int32 as *mut libc::c_int as *mut u32_0, p as *mut u32_0);
        int32 &= 0x7fffffff as libc::c_int;
        p = p.offset(4 as libc::c_int as isize);
    } else {
        p = p.offset(1);
        p;
    }
    count = int32;
    if size < 3 as libc::c_int || count < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if !ptype.is_null() {
        *ptype = type_0;
    }
    if !pcount.is_null() {
        *pcount = count;
    }
    if !psize.is_null() {
        *psize = size;
    }
    if !pheadersize.is_null() {
        *pheadersize = p.offset_from(pbuf as *mut libc::c_uchar) as libc::c_long
            as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn binn_buf_type(mut pbuf: *const libc::c_void) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    if IsValidBinnHeader(
        pbuf,
        &mut type_0,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return type_0;
}
unsafe extern "C" fn binn_buf_count(mut pbuf: *const libc::c_void) -> libc::c_int {
    let mut nitems: libc::c_int = 0;
    if IsValidBinnHeader(
        pbuf,
        0 as *mut libc::c_int,
        &mut nitems,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return nitems;
}
unsafe extern "C" fn binn_buf_size(mut pbuf: *const libc::c_void) -> libc::c_int {
    let mut size: libc::c_int = 0 as libc::c_int;
    if IsValidBinnHeader(
        pbuf,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
        &mut size,
        0 as *mut libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn binn_ptr(mut ptr: *const libc::c_void) -> *mut libc::c_void {
    let mut item: *mut binn = 0 as *mut binn;
    match binn_get_ptr_type(ptr) {
        1 => {
            item = ptr as *mut binn;
            if (*item).writable != 0 && (*item).dirty != 0 {
                binn_save_header(item);
            }
            return (*item).ptr;
        }
        2 => return ptr as *mut libc::c_void,
        _ => return 0 as *mut libc::c_void,
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_size(mut ptr: *const libc::c_void) -> libc::c_int {
    let mut item: *mut binn = 0 as *mut binn;
    match binn_get_ptr_type(ptr) {
        1 => {
            item = ptr as *mut binn;
            if (*item).writable != 0 && (*item).dirty != 0 {
                binn_save_header(item);
            }
            return (*item).size;
        }
        2 => return binn_buf_size(ptr),
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_type(mut ptr: *const libc::c_void) -> libc::c_int {
    let mut item: *mut binn = 0 as *mut binn;
    match binn_get_ptr_type(ptr) {
        1 => {
            item = ptr as *mut binn;
            return (*item).type_0;
        }
        2 => return binn_buf_type(ptr),
        _ => return -(1 as libc::c_int),
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_count(mut ptr: *const libc::c_void) -> libc::c_int {
    let mut item: *mut binn = 0 as *mut binn;
    match binn_get_ptr_type(ptr) {
        1 => {
            item = ptr as *mut binn;
            return (*item).count;
        }
        2 => return binn_buf_count(ptr),
        _ => return -(1 as libc::c_int),
    };
}
unsafe extern "C" fn binn_is_valid_ex2(
    mut ptr: *const libc::c_void,
    mut ptype: *mut libc::c_int,
    mut pcount: *mut libc::c_int,
    mut psize: *mut libc::c_int,
) -> BOOL {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut header_size: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut plimit: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut base: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: libc::c_uchar = 0;
    if ptr == 0 as *mut libc::c_void {
        return 0 as libc::c_int;
    }
    if !psize.is_null() && *psize > 0 as libc::c_int {
        size = *psize;
    } else {
        size = 0 as libc::c_int;
    }
    if IsValidBinnHeader(ptr, &mut type_0, &mut count, &mut size, &mut header_size) == 0
    {
        return 0 as libc::c_int;
    }
    if !psize.is_null() && *psize > 0 as libc::c_int {
        if size > *psize {
            return 0 as libc::c_int;
        }
    }
    if !pcount.is_null() && *pcount > 0 as libc::c_int {
        if count != *pcount {
            return 0 as libc::c_int;
        }
    }
    if !ptype.is_null() && *ptype != 0 as libc::c_int {
        if type_0 != *ptype {
            return 0 as libc::c_int;
        }
    }
    p = ptr as *mut libc::c_uchar;
    base = p;
    plimit = p.offset(size as isize).offset(-(1 as libc::c_int as isize));
    p = p.offset(header_size as isize);
    i = 0 as libc::c_int;
    loop {
        if !(i < count) {
            current_block = 2604890879466389055;
            break;
        }
        match type_0 {
            226 => {
                if p > plimit {
                    current_block = 10967187093506341854;
                    break;
                }
                len = *p;
                p = p.offset(1);
                p;
                p = p.offset(len as libc::c_int as isize);
            }
            225 => {
                read_map_id(&mut p, plimit);
            }
            224 => {}
            _ => {
                current_block = 10967187093506341854;
                break;
            }
        }
        if p > plimit {
            current_block = 10967187093506341854;
            break;
        }
        if *p as libc::c_int & 0xe0 as libc::c_int == 0xe0 as libc::c_int {
            let mut size2: libc::c_int = (plimit.offset_from(p) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_int;
            if binn_is_valid_ex2(
                p as *const libc::c_void,
                0 as *mut libc::c_int,
                0 as *mut libc::c_int,
                &mut size2,
            ) == 0 as libc::c_int
            {
                current_block = 10967187093506341854;
                break;
            }
            p = p.offset(size2 as isize);
        } else {
            p = AdvanceDataPos(p, plimit);
            if p.is_null() || p < base {
                current_block = 10967187093506341854;
                break;
            }
        }
        i += 1;
        i;
    }
    match current_block {
        10967187093506341854 => return 0 as libc::c_int,
        _ => {
            if !ptype.is_null() && *ptype == 0 as libc::c_int {
                *ptype = type_0;
            }
            if !pcount.is_null() && *pcount == 0 as libc::c_int {
                *pcount = count;
            }
            if !psize.is_null() {
                *psize = size;
            }
            return 1 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_is_valid_ex(
    mut ptr: *const libc::c_void,
    mut ptype: *mut libc::c_int,
    mut pcount: *mut libc::c_int,
    mut psize: *mut libc::c_int,
) -> BOOL {
    let mut size: libc::c_int = 0;
    if !psize.is_null() && *psize > 0 as libc::c_int {
        size = *psize;
    } else {
        size = 0 as libc::c_int;
    }
    if binn_is_valid_ex2(ptr, ptype, pcount, &mut size) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if !psize.is_null() {
        if *psize > 0 as libc::c_int {
            if size != *psize {
                return 0 as libc::c_int;
            }
        } else if *psize == 0 as libc::c_int {
            *psize = size;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_is_valid(
    mut ptr: *const libc::c_void,
    mut ptype: *mut libc::c_int,
    mut pcount: *mut libc::c_int,
    mut psize: *mut libc::c_int,
) -> BOOL {
    if !ptype.is_null() {
        *ptype = 0 as libc::c_int;
    }
    if !pcount.is_null() {
        *pcount = 0 as libc::c_int;
    }
    if !psize.is_null() {
        *psize = 0 as libc::c_int;
    }
    return binn_is_valid_ex(ptr, ptype, pcount, psize);
}
unsafe extern "C" fn GetValue(
    mut p: *mut libc::c_uchar,
    mut plimit: *mut libc::c_uchar,
    mut value: *mut binn,
) -> BOOL {
    let mut byte: libc::c_uchar = 0;
    let mut data_type: libc::c_int = 0;
    let mut storage_type: libc::c_int = 0;
    let mut DataSize: libc::c_int = 0;
    let mut p2: *mut libc::c_void = 0 as *mut libc::c_void;
    if value.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        value as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<binn>() as libc::c_ulong,
    );
    (*value).header = 0x1f22b11f as libc::c_int;
    p2 = p as *mut libc::c_void;
    if p > plimit {
        return 0 as libc::c_int;
    }
    byte = *p;
    p = p.offset(1);
    p;
    storage_type = byte as libc::c_int & 0xe0 as libc::c_int;
    if byte as libc::c_int & 0x10 as libc::c_int != 0 {
        data_type = (byte as libc::c_int) << 8 as libc::c_int;
        if p > plimit {
            return 0 as libc::c_int;
        }
        byte = *p;
        p = p.offset(1);
        p;
        data_type |= byte as libc::c_int;
    } else {
        data_type = byte as libc::c_int;
    }
    (*value).type_0 = data_type;
    match storage_type {
        0 => {}
        32 => {
            if p > plimit {
                return 0 as libc::c_int;
            }
            (*value).c2rust_unnamed.vuint8 = *p;
            (*value).ptr = p as *mut libc::c_void;
        }
        64 => {
            if p.offset(1 as libc::c_int as isize) > plimit {
                return 0 as libc::c_int;
            }
            copy_be16(
                &mut (*value).c2rust_unnamed.vint16 as *mut libc::c_short as *mut u16_0,
                p as *mut u16_0,
            );
            (*value)
                .ptr = &mut (*value).c2rust_unnamed.vint16 as *mut libc::c_short
                as *mut libc::c_void;
        }
        96 => {
            if p.offset(3 as libc::c_int as isize) > plimit {
                return 0 as libc::c_int;
            }
            copy_be32(
                &mut (*value).c2rust_unnamed.vint32 as *mut libc::c_int as *mut u32_0,
                p as *mut u32_0,
            );
            (*value)
                .ptr = &mut (*value).c2rust_unnamed.vint32 as *mut libc::c_int
                as *mut libc::c_void;
        }
        128 => {
            if p.offset(7 as libc::c_int as isize) > plimit {
                return 0 as libc::c_int;
            }
            copy_be64(
                &mut (*value).c2rust_unnamed.vint64 as *mut int64 as *mut u64_0,
                p as *mut u64_0,
            );
            (*value)
                .ptr = &mut (*value).c2rust_unnamed.vint64 as *mut int64
                as *mut libc::c_void;
        }
        192 | 160 => {
            if p > plimit {
                return 0 as libc::c_int;
            }
            DataSize = *p as libc::c_int;
            if DataSize & 0x80 as libc::c_int != 0 {
                if p.offset(3 as libc::c_int as isize) > plimit {
                    return 0 as libc::c_int;
                }
                copy_be32(
                    &mut DataSize as *mut libc::c_int as *mut u32_0,
                    p as *mut u32_0,
                );
                DataSize &= 0x7fffffff as libc::c_int;
                p = p.offset(4 as libc::c_int as isize);
            } else {
                p = p.offset(1);
                p;
            }
            if p.offset(DataSize as isize).offset(-(1 as libc::c_int as isize)) > plimit
            {
                return 0 as libc::c_int;
            }
            (*value).size = DataSize;
            (*value).ptr = p as *mut libc::c_void;
        }
        224 => {
            (*value).ptr = p2;
            if IsValidBinnHeader(
                p2,
                0 as *mut libc::c_int,
                &mut (*value).count,
                &mut (*value).size,
                0 as *mut libc::c_int,
            ) == 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        }
        _ => return 0 as libc::c_int,
    }
    match (*value).type_0 {
        1 => {
            (*value).type_0 = 0x80061 as libc::c_int;
            (*value).c2rust_unnamed.vbool = 1 as libc::c_int;
            (*value)
                .ptr = &mut (*value).c2rust_unnamed.vbool as *mut BOOL
                as *mut libc::c_void;
        }
        2 => {
            (*value).type_0 = 0x80061 as libc::c_int;
            (*value).c2rust_unnamed.vbool = 0 as libc::c_int;
            (*value)
                .ptr = &mut (*value).c2rust_unnamed.vbool as *mut BOOL
                as *mut libc::c_void;
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub static mut local_value: binn = binn {
    header: 0,
    allocated: 0,
    writable: 0,
    dirty: 0,
    pbuf: 0 as *const libc::c_void as *mut libc::c_void,
    pre_allocated: 0,
    alloc_size: 0,
    used_size: 0,
    type_0: 0,
    ptr: 0 as *const libc::c_void as *mut libc::c_void,
    size: 0,
    count: 0,
    freefn: None,
    c2rust_unnamed: C2RustUnnamed { vint8: 0 },
    disable_int_compression: 0,
};
unsafe extern "C" fn store_value(mut value: *mut binn) -> *mut libc::c_void {
    memcpy(
        &mut local_value as *mut binn as *mut libc::c_void,
        value as *const libc::c_void,
        ::core::mem::size_of::<binn>() as libc::c_ulong,
    );
    's_15: {
        match binn_get_read_storage((*value).type_0) {
            0 => {}
            64 | 96 | 128 => {}
            _ => {
                break 's_15;
            }
        }
        return &mut local_value.c2rust_unnamed.vint32 as *mut libc::c_int
            as *mut libc::c_void;
    }
    return (*value).ptr;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_get_value(
    mut ptr: *const libc::c_void,
    mut key: *const libc::c_char,
    mut value: *mut binn,
) -> BOOL {
    let mut type_0: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut header_size: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut plimit: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    ptr = binn_ptr(ptr);
    if ptr == 0 as *mut libc::c_void || key.is_null() || value.is_null() {
        return 0 as libc::c_int;
    }
    if IsValidBinnHeader(ptr, &mut type_0, &mut count, &mut size, &mut header_size)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if type_0 != 0xe2 as libc::c_int {
        return 0 as libc::c_int;
    }
    if count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    p = ptr as *mut libc::c_uchar;
    plimit = p.offset(size as isize).offset(-(1 as libc::c_int as isize));
    p = SearchForKey(p, header_size, size, count, key);
    if p.is_null() {
        return 0 as libc::c_int;
    }
    return GetValue(p, plimit, value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_get_value(
    mut ptr: *const libc::c_void,
    mut id: libc::c_int,
    mut value: *mut binn,
) -> BOOL {
    let mut type_0: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut header_size: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut plimit: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    ptr = binn_ptr(ptr);
    if ptr == 0 as *mut libc::c_void || value.is_null() {
        return 0 as libc::c_int;
    }
    if IsValidBinnHeader(ptr, &mut type_0, &mut count, &mut size, &mut header_size)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if type_0 != 0xe1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    p = ptr as *mut libc::c_uchar;
    plimit = p.offset(size as isize).offset(-(1 as libc::c_int as isize));
    p = SearchForID(p, header_size, size, count, id);
    if p.is_null() {
        return 0 as libc::c_int;
    }
    return GetValue(p, plimit, value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_get_value(
    mut ptr: *const libc::c_void,
    mut pos: libc::c_int,
    mut value: *mut binn,
) -> BOOL {
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut header_size: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut plimit: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut base: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    ptr = binn_ptr(ptr);
    if ptr == 0 as *mut libc::c_void || value.is_null() {
        return 0 as libc::c_int;
    }
    if IsValidBinnHeader(ptr, &mut type_0, &mut count, &mut size, &mut header_size)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if type_0 != 0xe0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if pos <= 0 as libc::c_int || pos > count {
        return 0 as libc::c_int;
    }
    pos -= 1;
    pos;
    p = ptr as *mut libc::c_uchar;
    base = p;
    plimit = p.offset(size as isize).offset(-(1 as libc::c_int as isize));
    p = p.offset(header_size as isize);
    i = 0 as libc::c_int;
    while i < pos {
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return GetValue(p, plimit, value);
}
unsafe extern "C" fn binn_read_pair(
    mut expected_type: libc::c_int,
    mut ptr: *const libc::c_void,
    mut pos: libc::c_int,
    mut pid: *mut libc::c_int,
    mut pkey: *mut libc::c_char,
    mut value: *mut binn,
) -> BOOL {
    let mut current_block: u64;
    let mut type_0: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut header_size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut int32: libc::c_int = 0;
    let mut id: libc::c_int = 0 as libc::c_int;
    let mut counter: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut plimit: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut base: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    ptr = binn_ptr(ptr);
    if IsValidBinnHeader(ptr, &mut type_0, &mut count, &mut size, &mut header_size)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if type_0 != expected_type || count == 0 as libc::c_int || pos < 1 as libc::c_int
        || pos > count
    {
        return 0 as libc::c_int;
    }
    p = ptr as *mut libc::c_uchar;
    base = p;
    plimit = p.offset(size as isize).offset(-(1 as libc::c_int as isize));
    p = p.offset(header_size as isize);
    i = 0 as libc::c_int;
    loop {
        if !(i < count) {
            current_block = 11298138898191919651;
            break;
        }
        match type_0 {
            225 => {
                int32 = read_map_id(&mut p, plimit);
                if p > plimit {
                    return 0 as libc::c_int;
                }
                id = int32;
            }
            226 => {
                len = *p;
                p = p.offset(1);
                p;
                if p > plimit {
                    return 0 as libc::c_int;
                }
                key = p;
                p = p.offset(len as libc::c_int as isize);
                if p > plimit {
                    return 0 as libc::c_int;
                }
            }
            _ => {}
        }
        counter += 1;
        counter;
        if counter == pos {
            current_block = 11952725860927050584;
            break;
        }
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    match current_block {
        11298138898191919651 => return 0 as libc::c_int,
        _ => {
            match type_0 {
                225 => {
                    if !pid.is_null() {
                        *pid = id;
                    }
                }
                226 => {
                    if !pkey.is_null() {
                        memcpy(
                            pkey as *mut libc::c_void,
                            key as *const libc::c_void,
                            len as libc::c_ulong,
                        );
                        *pkey.offset(len as isize) = 0 as libc::c_int as libc::c_char;
                    }
                }
                _ => {}
            }
            return GetValue(p, plimit, value);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_get_pair(
    mut ptr: *const libc::c_void,
    mut pos: libc::c_int,
    mut pid: *mut libc::c_int,
    mut value: *mut binn,
) -> BOOL {
    return binn_read_pair(
        0xe1 as libc::c_int,
        ptr,
        pos,
        pid,
        0 as *mut libc::c_char,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_get_pair(
    mut ptr: *const libc::c_void,
    mut pos: libc::c_int,
    mut pkey: *mut libc::c_char,
    mut value: *mut binn,
) -> BOOL {
    return binn_read_pair(
        0xe2 as libc::c_int,
        ptr,
        pos,
        0 as *mut libc::c_int,
        pkey,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_pair(
    mut map: *const libc::c_void,
    mut pos: libc::c_int,
    mut pid: *mut libc::c_int,
) -> *mut binn {
    let mut value: *mut binn = 0 as *mut binn;
    value = binn_malloc(::core::mem::size_of::<binn>() as libc::c_ulong as libc::c_int)
        as *mut binn;
    if binn_read_pair(0xe1 as libc::c_int, map, pos, pid, 0 as *mut libc::c_char, value)
        == 0 as libc::c_int
    {
        free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as libc::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_pair(
    mut obj: *const libc::c_void,
    mut pos: libc::c_int,
    mut pkey: *mut libc::c_char,
) -> *mut binn {
    let mut value: *mut binn = 0 as *mut binn;
    value = binn_malloc(::core::mem::size_of::<binn>() as libc::c_ulong as libc::c_int)
        as *mut binn;
    if binn_read_pair(0xe2 as libc::c_int, obj, pos, 0 as *mut libc::c_int, pkey, value)
        == 0 as libc::c_int
    {
        free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as libc::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_read_pair(
    mut ptr: *const libc::c_void,
    mut pos: libc::c_int,
    mut pid: *mut libc::c_int,
    mut ptype: *mut libc::c_int,
    mut psize: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const libc::c_void as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const libc::c_void as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_map_get_pair(ptr, pos, pid, &mut value) == 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_read_pair(
    mut ptr: *const libc::c_void,
    mut pos: libc::c_int,
    mut pkey: *mut libc::c_char,
    mut ptype: *mut libc::c_int,
    mut psize: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const libc::c_void as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const libc::c_void as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_object_get_pair(ptr, pos, pkey, &mut value) == 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_iter_init(
    mut iter: *mut binn_iter,
    mut ptr: *const libc::c_void,
    mut expected_type: libc::c_int,
) -> BOOL {
    let mut type_0: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut header_size: libc::c_int = 0;
    ptr = binn_ptr(ptr);
    if ptr == 0 as *mut libc::c_void || iter.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        iter as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<binn_iter>() as libc::c_ulong,
    );
    if IsValidBinnHeader(ptr, &mut type_0, &mut count, &mut size, &mut header_size)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if type_0 != expected_type {
        return 0 as libc::c_int;
    }
    (*iter)
        .plimit = (ptr as *mut libc::c_uchar)
        .offset(size as isize)
        .offset(-(1 as libc::c_int as isize));
    (*iter).pnext = (ptr as *mut libc::c_uchar).offset(header_size as isize);
    (*iter).count = count;
    (*iter).current = 0 as libc::c_int;
    (*iter).type_0 = type_0;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_next(
    mut iter: *mut binn_iter,
    mut value: *mut binn,
) -> BOOL {
    let mut pnow: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if iter.is_null() || ((*iter).pnext).is_null() || (*iter).pnext > (*iter).plimit
        || (*iter).current > (*iter).count || (*iter).type_0 != 0xe0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    (*iter).current += 1;
    (*iter).current;
    if (*iter).current > (*iter).count {
        return 0 as libc::c_int;
    }
    pnow = (*iter).pnext;
    (*iter).pnext = AdvanceDataPos(pnow, (*iter).plimit);
    if !((*iter).pnext).is_null() && (*iter).pnext < pnow {
        return 0 as libc::c_int;
    }
    return GetValue(pnow, (*iter).plimit, value);
}
unsafe extern "C" fn binn_read_next_pair(
    mut expected_type: libc::c_int,
    mut iter: *mut binn_iter,
    mut pid: *mut libc::c_int,
    mut pkey: *mut libc::c_char,
    mut value: *mut binn,
) -> BOOL {
    let mut int32: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: libc::c_ushort = 0;
    if iter.is_null() || ((*iter).pnext).is_null() || (*iter).pnext > (*iter).plimit
        || (*iter).current > (*iter).count || (*iter).type_0 != expected_type
    {
        return 0 as libc::c_int;
    }
    (*iter).current += 1;
    (*iter).current;
    if (*iter).current > (*iter).count {
        return 0 as libc::c_int;
    }
    p = (*iter).pnext;
    match expected_type {
        225 => {
            int32 = read_map_id(&mut p, (*iter).plimit);
            if p > (*iter).plimit {
                return 0 as libc::c_int;
            }
            id = int32;
            if !pid.is_null() {
                *pid = id;
            }
        }
        226 => {
            len = *p as libc::c_ushort;
            p = p.offset(1);
            p;
            key = p;
            p = p.offset(len as libc::c_int as isize);
            if p > (*iter).plimit {
                return 0 as libc::c_int;
            }
            if !pkey.is_null() {
                memcpy(
                    pkey as *mut libc::c_void,
                    key as *const libc::c_void,
                    len as libc::c_ulong,
                );
                *pkey.offset(len as isize) = 0 as libc::c_int as libc::c_char;
            }
        }
        _ => {}
    }
    (*iter).pnext = AdvanceDataPos(p, (*iter).plimit);
    if !((*iter).pnext).is_null() && (*iter).pnext < p {
        return 0 as libc::c_int;
    }
    return GetValue(p, (*iter).plimit, value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_next(
    mut iter: *mut binn_iter,
    mut pid: *mut libc::c_int,
    mut value: *mut binn,
) -> BOOL {
    return binn_read_next_pair(
        0xe1 as libc::c_int,
        iter,
        pid,
        0 as *mut libc::c_char,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_next(
    mut iter: *mut binn_iter,
    mut pkey: *mut libc::c_char,
    mut value: *mut binn,
) -> BOOL {
    return binn_read_next_pair(
        0xe2 as libc::c_int,
        iter,
        0 as *mut libc::c_int,
        pkey,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_next_value(mut iter: *mut binn_iter) -> *mut binn {
    let mut value: *mut binn = 0 as *mut binn;
    value = binn_malloc(::core::mem::size_of::<binn>() as libc::c_ulong as libc::c_int)
        as *mut binn;
    if binn_list_next(iter, value) == 0 as libc::c_int {
        free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as libc::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_next_value(
    mut iter: *mut binn_iter,
    mut pid: *mut libc::c_int,
) -> *mut binn {
    let mut value: *mut binn = 0 as *mut binn;
    value = binn_malloc(::core::mem::size_of::<binn>() as libc::c_ulong as libc::c_int)
        as *mut binn;
    if binn_map_next(iter, pid, value) == 0 as libc::c_int {
        free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as libc::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_next_value(
    mut iter: *mut binn_iter,
    mut pkey: *mut libc::c_char,
) -> *mut binn {
    let mut value: *mut binn = 0 as *mut binn;
    value = binn_malloc(::core::mem::size_of::<binn>() as libc::c_ulong as libc::c_int)
        as *mut binn;
    if binn_object_next(iter, pkey, value) == 0 as libc::c_int {
        free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as libc::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_read_next(
    mut iter: *mut binn_iter,
    mut ptype: *mut libc::c_int,
    mut psize: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const libc::c_void as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const libc::c_void as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_list_next(iter, &mut value) == 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_read_next(
    mut iter: *mut binn_iter,
    mut pid: *mut libc::c_int,
    mut ptype: *mut libc::c_int,
    mut psize: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const libc::c_void as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const libc::c_void as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_map_next(iter, pid, &mut value) == 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_read_next(
    mut iter: *mut binn_iter,
    mut pkey: *mut libc::c_char,
    mut ptype: *mut libc::c_int,
    mut psize: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const libc::c_void as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const libc::c_void as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_object_next(iter, pkey, &mut value) == 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_write_storage(mut type_0: libc::c_int) -> libc::c_int {
    let mut storage_type: libc::c_int = 0;
    match type_0 {
        166 | 167 => return 0xa0 as libc::c_int,
        524385 => return 0 as libc::c_int,
        _ => {
            binn_get_type_info(type_0, &mut storage_type, 0 as *mut libc::c_int);
            return storage_type;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_read_storage(mut type_0: libc::c_int) -> libc::c_int {
    let mut storage_type: libc::c_int = 0;
    match type_0 {
        524385 | 1 | 2 => return 0x60 as libc::c_int,
        _ => {
            binn_get_type_info(type_0, &mut storage_type, 0 as *mut libc::c_int);
            return storage_type;
        }
    };
}
unsafe extern "C" fn GetWriteConvertedData(
    mut ptype: *mut libc::c_int,
    mut ppvalue: *mut *mut libc::c_void,
    mut psize: *mut libc::c_int,
) -> BOOL {
    let mut type_0: libc::c_int = 0;
    let mut f1: libc::c_float = 0.;
    let mut d1: libc::c_double = 0.;
    let mut pstr: [libc::c_char; 128] = [0; 128];
    type_0 = *ptype;
    if (*ppvalue).is_null() {
        let mut current_block_4: u64;
        match type_0 {
            0 | 1 | 2 => {
                current_block_4 = 13513818773234778473;
            }
            160 | 192 => {
                if *psize == 0 as libc::c_int {
                    current_block_4 = 13513818773234778473;
                } else {
                    current_block_4 = 14156331183014003305;
                }
            }
            _ => {
                current_block_4 = 14156331183014003305;
            }
        }
        match current_block_4 {
            14156331183014003305 => return 0 as libc::c_int,
            _ => {}
        }
    }
    match type_0 {
        164 | 165 => return 1 as libc::c_int,
        162 | 161 | 163 => return 1 as libc::c_int,
        524385 => {
            if **(ppvalue as *mut *mut BOOL) == 0 as libc::c_int {
                type_0 = 0x2 as libc::c_int;
            } else {
                type_0 = 0x1 as libc::c_int;
            }
            *ptype = type_0;
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn type_family(mut type_0: libc::c_int) -> libc::c_int {
    match type_0 {
        224 | 225 | 226 => return 0xf7 as libc::c_int,
        33 | 65 | 97 | 129 | 32 | 64 | 96 | 128 => return 0xf2 as libc::c_int,
        98 | 130 => {}
        166 | 167 => {}
        160 | 45057 | 45061 | 45058 | 45059 | 45060 => return 0xf4 as libc::c_int,
        192 | 53249 | 53250 | 53251 | 53252 => return 0xf5 as libc::c_int,
        164 | 131 | 162 | 163 | 161 => return 0xf4 as libc::c_int,
        524385 => return 0xf6 as libc::c_int,
        0 => return 0xf1 as libc::c_int,
        _ => return 0 as libc::c_int,
    }
    return 0xf3 as libc::c_int;
}
unsafe extern "C" fn int_type(mut type_0: libc::c_int) -> libc::c_int {
    match type_0 {
        33 | 65 | 97 | 129 => return 11 as libc::c_int,
        32 | 64 | 96 | 128 => return 22 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn copy_raw_value(
    mut psource: *const libc::c_void,
    mut pdest: *mut libc::c_void,
    mut data_store: libc::c_int,
) -> BOOL {
    match data_store {
        0 => {}
        32 => {
            *(pdest as *mut libc::c_char) = *(psource as *mut libc::c_char);
        }
        64 => {
            *(pdest as *mut libc::c_short) = *(psource as *mut libc::c_short);
        }
        96 => {
            *(pdest as *mut libc::c_int) = *(psource as *mut libc::c_int);
        }
        128 => {
            *(pdest as *mut uint64) = *(psource as *mut uint64);
        }
        192 | 160 | 224 => {
            let ref mut fresh18 = *(pdest as *mut *mut libc::c_char);
            *fresh18 = psource as *mut libc::c_char;
        }
        _ => return 0 as libc::c_int,
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn copy_int_value(
    mut psource: *const libc::c_void,
    mut pdest: *mut libc::c_void,
    mut source_type: libc::c_int,
    mut dest_type: libc::c_int,
) -> BOOL {
    let mut vuint64: uint64 = 0 as libc::c_int as uint64;
    let mut vint64: int64 = 0 as libc::c_int as int64;
    match source_type {
        33 => {
            vint64 = *(psource as *mut libc::c_schar) as int64;
        }
        65 => {
            vint64 = *(psource as *mut libc::c_short) as int64;
        }
        97 => {
            vint64 = *(psource as *mut libc::c_int) as int64;
        }
        129 => {
            vint64 = *(psource as *mut int64);
        }
        32 => {
            vuint64 = *(psource as *mut libc::c_uchar) as uint64;
        }
        64 => {
            vuint64 = *(psource as *mut libc::c_ushort) as uint64;
        }
        96 => {
            vuint64 = *(psource as *mut libc::c_uint) as uint64;
        }
        128 => {
            vuint64 = *(psource as *mut uint64);
        }
        _ => return 0 as libc::c_int,
    }
    if int_type(source_type) == 22 as libc::c_int
        && int_type(dest_type) == 11 as libc::c_int
    {
        if vuint64 > 9223372036854775807 as libc::c_long as libc::c_ulonglong {
            return 0 as libc::c_int;
        }
        vint64 = vuint64 as int64;
    } else if int_type(source_type) == 11 as libc::c_int
        && int_type(dest_type) == 22 as libc::c_int
    {
        if vint64 < 0 as libc::c_int as libc::c_longlong {
            return 0 as libc::c_int;
        }
        vuint64 = vint64 as uint64;
    }
    match dest_type {
        33 => {
            if vint64 < -(128 as libc::c_int) as libc::c_longlong
                || vint64 > 127 as libc::c_int as libc::c_longlong
            {
                return 0 as libc::c_int;
            }
            *(pdest as *mut libc::c_schar) = vint64 as libc::c_schar;
        }
        65 => {
            if vint64 < (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_longlong
                || vint64 > 32767 as libc::c_int as libc::c_longlong
            {
                return 0 as libc::c_int;
            }
            *(pdest as *mut libc::c_short) = vint64 as libc::c_short;
        }
        97 => {
            if vint64
                < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_longlong
                || vint64 > 2147483647 as libc::c_int as libc::c_longlong
            {
                return 0 as libc::c_int;
            }
            *(pdest as *mut libc::c_int) = vint64 as libc::c_int;
        }
        129 => {
            *(pdest as *mut int64) = vint64;
        }
        32 => {
            if vuint64 > 255 as libc::c_int as libc::c_ulonglong {
                return 0 as libc::c_int;
            }
            *(pdest as *mut libc::c_uchar) = vuint64 as libc::c_uchar;
        }
        64 => {
            if vuint64 > 65535 as libc::c_int as libc::c_ulonglong {
                return 0 as libc::c_int;
            }
            *(pdest as *mut libc::c_ushort) = vuint64 as libc::c_ushort;
        }
        96 => {
            if vuint64 > 4294967295 as libc::c_uint as libc::c_ulonglong {
                return 0 as libc::c_int;
            }
            *(pdest as *mut libc::c_uint) = vuint64 as libc::c_uint;
        }
        128 => {
            *(pdest as *mut uint64) = vuint64;
        }
        _ => return 0 as libc::c_int,
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn copy_float_value(
    mut psource: *const libc::c_void,
    mut pdest: *mut libc::c_void,
    mut source_type: libc::c_int,
    mut dest_type: libc::c_int,
) -> BOOL {
    match source_type {
        98 => {
            *(pdest
                as *mut libc::c_double) = *(psource as *mut libc::c_float)
                as libc::c_double;
        }
        130 => {
            *(pdest
                as *mut libc::c_float) = *(psource as *mut libc::c_double)
                as libc::c_float;
        }
        _ => return 0 as libc::c_int,
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn zero_value(
    mut pvalue: *const libc::c_void,
    mut type_0: libc::c_int,
) {
    match binn_get_read_storage(type_0) {
        32 => {
            *(pvalue as *mut libc::c_char) = 0 as libc::c_int as libc::c_char;
        }
        64 => {
            *(pvalue as *mut libc::c_short) = 0 as libc::c_int as libc::c_short;
        }
        96 => {
            *(pvalue as *mut libc::c_int) = 0 as libc::c_int;
        }
        128 => {
            *(pvalue as *mut uint64) = 0 as libc::c_int as uint64;
        }
        192 | 160 | 224 => {
            let ref mut fresh19 = *(pvalue as *mut *mut libc::c_char);
            *fresh19 = 0 as *mut libc::c_char;
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn copy_value(
    mut psource: *mut libc::c_void,
    mut pdest: *mut libc::c_void,
    mut source_type: libc::c_int,
    mut dest_type: libc::c_int,
    mut data_store: libc::c_int,
) -> BOOL {
    if type_family(source_type) != type_family(dest_type) {
        return 0 as libc::c_int;
    }
    if type_family(source_type) == 0xf2 as libc::c_int && source_type != dest_type {
        return copy_int_value(psource, pdest, source_type, dest_type)
    } else if type_family(source_type) == 0xf3 as libc::c_int && source_type != dest_type
    {
        return copy_float_value(psource, pdest, source_type, dest_type)
    } else {
        return copy_raw_value(psource, pdest, data_store)
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_add(
    mut list: *mut binn,
    mut type_0: libc::c_int,
    mut pvalue: *mut libc::c_void,
    mut size: libc::c_int,
) -> BOOL {
    if GetWriteConvertedData(&mut type_0, &mut pvalue, &mut size) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return binn_list_add_raw(list, type_0, pvalue, size);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_set(
    mut map: *mut binn,
    mut id: libc::c_int,
    mut type_0: libc::c_int,
    mut pvalue: *mut libc::c_void,
    mut size: libc::c_int,
) -> BOOL {
    if GetWriteConvertedData(&mut type_0, &mut pvalue, &mut size) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return binn_map_set_raw(map, id, type_0, pvalue, size);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_set(
    mut obj: *mut binn,
    mut key: *const libc::c_char,
    mut type_0: libc::c_int,
    mut pvalue: *mut libc::c_void,
    mut size: libc::c_int,
) -> BOOL {
    if GetWriteConvertedData(&mut type_0, &mut pvalue, &mut size) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return binn_object_set_raw(obj, key, type_0, pvalue, size);
}
#[no_mangle]
pub unsafe extern "C" fn binn_add_value(
    mut item: *mut binn,
    mut binn_type_0: libc::c_int,
    mut id: libc::c_int,
    mut name: *mut libc::c_char,
    mut type_0: libc::c_int,
    mut pvalue: *mut libc::c_void,
    mut size: libc::c_int,
) -> BOOL {
    match binn_type_0 {
        224 => return binn_list_add(item, type_0, pvalue, size),
        225 => return binn_map_set(item, id, type_0, pvalue, size),
        226 => return binn_object_set(item, name, type_0, pvalue, size),
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_add_new(
    mut list: *mut binn,
    mut value: *mut binn,
) -> BOOL {
    let mut retval: BOOL = 0;
    retval = binn_list_add_value(list, value);
    if !value.is_null() {
        free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_set_new(
    mut map: *mut binn,
    mut id: libc::c_int,
    mut value: *mut binn,
) -> BOOL {
    let mut retval: BOOL = 0;
    retval = binn_map_set_value(map, id, value);
    if !value.is_null() {
        free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_set_new(
    mut obj: *mut binn,
    mut key: *const libc::c_char,
    mut value: *mut binn,
) -> BOOL {
    let mut retval: BOOL = 0;
    retval = binn_object_set_value(obj, key, value);
    if !value.is_null() {
        free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_value(
    mut ptr: *const libc::c_void,
    mut pos: libc::c_int,
) -> *mut binn {
    let mut value: *mut binn = 0 as *mut binn;
    value = binn_malloc(::core::mem::size_of::<binn>() as libc::c_ulong as libc::c_int)
        as *mut binn;
    if binn_list_get_value(ptr, pos, value) == 0 as libc::c_int {
        free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as libc::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_value(
    mut ptr: *const libc::c_void,
    mut id: libc::c_int,
) -> *mut binn {
    let mut value: *mut binn = 0 as *mut binn;
    value = binn_malloc(::core::mem::size_of::<binn>() as libc::c_ulong as libc::c_int)
        as *mut binn;
    if binn_map_get_value(ptr, id, value) == 0 as libc::c_int {
        free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as libc::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_value(
    mut ptr: *const libc::c_void,
    mut key: *const libc::c_char,
) -> *mut binn {
    let mut value: *mut binn = 0 as *mut binn;
    value = binn_malloc(::core::mem::size_of::<binn>() as libc::c_ulong as libc::c_int)
        as *mut binn;
    if binn_object_get_value(ptr, key, value) == 0 as libc::c_int {
        free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as libc::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_read(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
    mut ptype: *mut libc::c_int,
    mut psize: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const libc::c_void as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const libc::c_void as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_list_get_value(list, pos, &mut value) == 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_read(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
    mut ptype: *mut libc::c_int,
    mut psize: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const libc::c_void as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const libc::c_void as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_map_get_value(map, id, &mut value) == 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_read(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
    mut ptype: *mut libc::c_int,
    mut psize: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const libc::c_void as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const libc::c_void as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_object_get_value(obj, key, &mut value) == 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_get(
    mut ptr: *const libc::c_void,
    mut pos: libc::c_int,
    mut type_0: libc::c_int,
    mut pvalue: *mut libc::c_void,
    mut psize: *mut libc::c_int,
) -> BOOL {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const libc::c_void as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const libc::c_void as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    let mut storage_type: libc::c_int = 0;
    storage_type = binn_get_read_storage(type_0);
    if storage_type != 0 as libc::c_int && pvalue.is_null() {
        return 0 as libc::c_int;
    }
    zero_value(pvalue, type_0);
    if binn_list_get_value(ptr, pos, &mut value) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if copy_value(value.ptr, pvalue, value.type_0, type_0, storage_type)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_get(
    mut ptr: *const libc::c_void,
    mut id: libc::c_int,
    mut type_0: libc::c_int,
    mut pvalue: *mut libc::c_void,
    mut psize: *mut libc::c_int,
) -> BOOL {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const libc::c_void as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const libc::c_void as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    let mut storage_type: libc::c_int = 0;
    storage_type = binn_get_read_storage(type_0);
    if storage_type != 0 as libc::c_int && pvalue.is_null() {
        return 0 as libc::c_int;
    }
    zero_value(pvalue, type_0);
    if binn_map_get_value(ptr, id, &mut value) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if copy_value(value.ptr, pvalue, value.type_0, type_0, storage_type)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_get(
    mut ptr: *const libc::c_void,
    mut key: *const libc::c_char,
    mut type_0: libc::c_int,
    mut pvalue: *mut libc::c_void,
    mut psize: *mut libc::c_int,
) -> BOOL {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const libc::c_void as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const libc::c_void as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    let mut storage_type: libc::c_int = 0;
    storage_type = binn_get_read_storage(type_0);
    if storage_type != 0 as libc::c_int && pvalue.is_null() {
        return 0 as libc::c_int;
    }
    zero_value(pvalue, type_0);
    if binn_object_get_value(ptr, key, &mut value) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if copy_value(value.ptr, pvalue, value.type_0, type_0, storage_type)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_int8(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> libc::c_schar {
    let mut value: libc::c_schar = 0;
    binn_list_get(
        list,
        pos,
        0x21 as libc::c_int,
        &mut value as *mut libc::c_schar as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_int16(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> libc::c_short {
    let mut value: libc::c_short = 0;
    binn_list_get(
        list,
        pos,
        0x41 as libc::c_int,
        &mut value as *mut libc::c_short as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_int32(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> libc::c_int {
    let mut value: libc::c_int = 0;
    binn_list_get(
        list,
        pos,
        0x61 as libc::c_int,
        &mut value as *mut libc::c_int as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_int64(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> int64 {
    let mut value: int64 = 0;
    binn_list_get(
        list,
        pos,
        0x81 as libc::c_int,
        &mut value as *mut int64 as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_uint8(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> libc::c_uchar {
    let mut value: libc::c_uchar = 0;
    binn_list_get(
        list,
        pos,
        0x20 as libc::c_int,
        &mut value as *mut libc::c_uchar as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_uint16(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> libc::c_ushort {
    let mut value: libc::c_ushort = 0;
    binn_list_get(
        list,
        pos,
        0x40 as libc::c_int,
        &mut value as *mut libc::c_ushort as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_uint32(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> libc::c_uint {
    let mut value: libc::c_uint = 0;
    binn_list_get(
        list,
        pos,
        0x60 as libc::c_int,
        &mut value as *mut libc::c_uint as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_uint64(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> uint64 {
    let mut value: uint64 = 0;
    binn_list_get(
        list,
        pos,
        0x80 as libc::c_int,
        &mut value as *mut uint64 as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_float(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> libc::c_float {
    let mut value: libc::c_float = 0.;
    binn_list_get(
        list,
        pos,
        0x62 as libc::c_int,
        &mut value as *mut libc::c_float as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_double(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> libc::c_double {
    let mut value: libc::c_double = 0.;
    binn_list_get(
        list,
        pos,
        0x82 as libc::c_int,
        &mut value as *mut libc::c_double as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_bool(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> BOOL {
    let mut value: BOOL = 0;
    binn_list_get(
        list,
        pos,
        0x80061 as libc::c_int,
        &mut value as *mut BOOL as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_null(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> BOOL {
    return binn_list_get(
        list,
        pos,
        0 as libc::c_int,
        0 as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_str(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    binn_list_get(
        list,
        pos,
        0xa0 as libc::c_int,
        &mut value as *mut *mut libc::c_char as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_blob(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
    mut psize: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    binn_list_get(
        list,
        pos,
        0xc0 as libc::c_int,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
        psize,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_list(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    binn_list_get(
        list,
        pos,
        0xe0 as libc::c_int,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_map(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    binn_list_get(
        list,
        pos,
        0xe1 as libc::c_int,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_object(
    mut list: *const libc::c_void,
    mut pos: libc::c_int,
) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    binn_list_get(
        list,
        pos,
        0xe2 as libc::c_int,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_int8(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> libc::c_schar {
    let mut value: libc::c_schar = 0;
    binn_map_get(
        map,
        id,
        0x21 as libc::c_int,
        &mut value as *mut libc::c_schar as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_int16(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> libc::c_short {
    let mut value: libc::c_short = 0;
    binn_map_get(
        map,
        id,
        0x41 as libc::c_int,
        &mut value as *mut libc::c_short as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_int32(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> libc::c_int {
    let mut value: libc::c_int = 0;
    binn_map_get(
        map,
        id,
        0x61 as libc::c_int,
        &mut value as *mut libc::c_int as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_int64(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> int64 {
    let mut value: int64 = 0;
    binn_map_get(
        map,
        id,
        0x81 as libc::c_int,
        &mut value as *mut int64 as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_uint8(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> libc::c_uchar {
    let mut value: libc::c_uchar = 0;
    binn_map_get(
        map,
        id,
        0x20 as libc::c_int,
        &mut value as *mut libc::c_uchar as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_uint16(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> libc::c_ushort {
    let mut value: libc::c_ushort = 0;
    binn_map_get(
        map,
        id,
        0x40 as libc::c_int,
        &mut value as *mut libc::c_ushort as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_uint32(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> libc::c_uint {
    let mut value: libc::c_uint = 0;
    binn_map_get(
        map,
        id,
        0x60 as libc::c_int,
        &mut value as *mut libc::c_uint as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_uint64(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> uint64 {
    let mut value: uint64 = 0;
    binn_map_get(
        map,
        id,
        0x80 as libc::c_int,
        &mut value as *mut uint64 as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_float(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> libc::c_float {
    let mut value: libc::c_float = 0.;
    binn_map_get(
        map,
        id,
        0x62 as libc::c_int,
        &mut value as *mut libc::c_float as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_double(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> libc::c_double {
    let mut value: libc::c_double = 0.;
    binn_map_get(
        map,
        id,
        0x82 as libc::c_int,
        &mut value as *mut libc::c_double as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_bool(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> BOOL {
    let mut value: BOOL = 0;
    binn_map_get(
        map,
        id,
        0x80061 as libc::c_int,
        &mut value as *mut BOOL as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_null(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> BOOL {
    return binn_map_get(
        map,
        id,
        0 as libc::c_int,
        0 as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_str(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> *mut libc::c_char {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    binn_map_get(
        map,
        id,
        0xa0 as libc::c_int,
        &mut value as *mut *mut libc::c_char as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_blob(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
    mut psize: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    binn_map_get(
        map,
        id,
        0xc0 as libc::c_int,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
        psize,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_list(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    binn_map_get(
        map,
        id,
        0xe0 as libc::c_int,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_map(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    binn_map_get(
        map,
        id,
        0xe1 as libc::c_int,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_object(
    mut map: *const libc::c_void,
    mut id: libc::c_int,
) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    binn_map_get(
        map,
        id,
        0xe2 as libc::c_int,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_int8(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> libc::c_schar {
    let mut value: libc::c_schar = 0;
    binn_object_get(
        obj,
        key,
        0x21 as libc::c_int,
        &mut value as *mut libc::c_schar as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_int16(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> libc::c_short {
    let mut value: libc::c_short = 0;
    binn_object_get(
        obj,
        key,
        0x41 as libc::c_int,
        &mut value as *mut libc::c_short as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_int32(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> libc::c_int {
    let mut value: libc::c_int = 0;
    binn_object_get(
        obj,
        key,
        0x61 as libc::c_int,
        &mut value as *mut libc::c_int as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_int64(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> int64 {
    let mut value: int64 = 0;
    binn_object_get(
        obj,
        key,
        0x81 as libc::c_int,
        &mut value as *mut int64 as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_uint8(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> libc::c_uchar {
    let mut value: libc::c_uchar = 0;
    binn_object_get(
        obj,
        key,
        0x20 as libc::c_int,
        &mut value as *mut libc::c_uchar as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_uint16(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> libc::c_ushort {
    let mut value: libc::c_ushort = 0;
    binn_object_get(
        obj,
        key,
        0x40 as libc::c_int,
        &mut value as *mut libc::c_ushort as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_uint32(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> libc::c_uint {
    let mut value: libc::c_uint = 0;
    binn_object_get(
        obj,
        key,
        0x60 as libc::c_int,
        &mut value as *mut libc::c_uint as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_uint64(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> uint64 {
    let mut value: uint64 = 0;
    binn_object_get(
        obj,
        key,
        0x80 as libc::c_int,
        &mut value as *mut uint64 as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_float(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> libc::c_float {
    let mut value: libc::c_float = 0.;
    binn_object_get(
        obj,
        key,
        0x62 as libc::c_int,
        &mut value as *mut libc::c_float as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_double(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> libc::c_double {
    let mut value: libc::c_double = 0.;
    binn_object_get(
        obj,
        key,
        0x82 as libc::c_int,
        &mut value as *mut libc::c_double as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_bool(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> BOOL {
    let mut value: BOOL = 0;
    binn_object_get(
        obj,
        key,
        0x80061 as libc::c_int,
        &mut value as *mut BOOL as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_null(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> BOOL {
    return binn_object_get(
        obj,
        key,
        0 as libc::c_int,
        0 as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_str(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> *mut libc::c_char {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    binn_object_get(
        obj,
        key,
        0xa0 as libc::c_int,
        &mut value as *mut *mut libc::c_char as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_blob(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
    mut psize: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    binn_object_get(
        obj,
        key,
        0xc0 as libc::c_int,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
        psize,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_list(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    binn_object_get(
        obj,
        key,
        0xe0 as libc::c_int,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_map(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    binn_object_get(
        obj,
        key,
        0xe1 as libc::c_int,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_object(
    mut obj: *const libc::c_void,
    mut key: *const libc::c_char,
) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    binn_object_get(
        obj,
        key,
        0xe2 as libc::c_int,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
    return value;
}
unsafe extern "C" fn binn_alloc_item() -> *mut binn {
    let mut item: *mut binn = 0 as *mut binn;
    item = binn_malloc(::core::mem::size_of::<binn>() as libc::c_ulong as libc::c_int)
        as *mut binn;
    if !item.is_null() {
        memset(
            item as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<binn>() as libc::c_ulong,
        );
        (*item).header = 0x1f22b11f as libc::c_int;
        (*item).allocated = 1 as libc::c_int;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_value(
    mut type_0: libc::c_int,
    mut pvalue: *mut libc::c_void,
    mut size: libc::c_int,
    mut freefn: binn_mem_free,
) -> *mut binn {
    let mut storage_type: libc::c_int = 0;
    let mut item: *mut binn = binn_alloc_item();
    if !item.is_null() {
        (*item).type_0 = type_0;
        binn_get_type_info(type_0, &mut storage_type, 0 as *mut libc::c_int);
        let mut current_block_19: u64;
        match storage_type {
            0 => {
                current_block_19 = 2370887241019905314;
            }
            160 => {
                if size == 0 as libc::c_int {
                    size = (strlen(pvalue as *mut libc::c_char))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                }
                current_block_19 = 15029624368332882957;
            }
            192 | 224 => {
                current_block_19 = 15029624368332882957;
            }
            _ => {
                (*item)
                    .ptr = &mut (*item).c2rust_unnamed.vint32 as *mut libc::c_int
                    as *mut libc::c_void;
                copy_raw_value(pvalue, (*item).ptr, storage_type);
                current_block_19 = 2370887241019905314;
            }
        }
        match current_block_19 {
            15029624368332882957 => {
                if freefn
                    == ::core::mem::transmute::<
                        libc::intptr_t,
                        binn_mem_free,
                    >(-(1 as libc::c_int) as libc::intptr_t)
                {
                    (*item).ptr = binn_memdup(pvalue, size);
                    if ((*item).ptr).is_null() {
                        free_fn
                            .expect(
                                "non-null function pointer",
                            )(item as *mut libc::c_void);
                        return 0 as *mut binn;
                    }
                    (*item).freefn = free_fn;
                    if storage_type == 0xa0 as libc::c_int {
                        size -= 1;
                        size;
                    }
                } else {
                    (*item).ptr = pvalue;
                    (*item).freefn = freefn;
                }
                (*item).size = size;
            }
            _ => {}
        }
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_set_string(
    mut item: *mut binn,
    mut str: *mut libc::c_char,
    mut pfree: binn_mem_free,
) -> BOOL {
    if item.is_null() || str.is_null() {
        return 0 as libc::c_int;
    }
    if pfree
        == ::core::mem::transmute::<
            libc::intptr_t,
            binn_mem_free,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        (*item)
            .ptr = binn_memdup(
            str as *const libc::c_void,
            (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        );
        if ((*item).ptr).is_null() {
            return 0 as libc::c_int;
        }
        (*item).freefn = free_fn;
    } else {
        (*item).ptr = str as *mut libc::c_void;
        (*item).freefn = pfree;
    }
    (*item).type_0 = 0xa0 as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_set_blob(
    mut item: *mut binn,
    mut ptr: *mut libc::c_void,
    mut size: libc::c_int,
    mut pfree: binn_mem_free,
) -> BOOL {
    if item.is_null() || ptr.is_null() {
        return 0 as libc::c_int;
    }
    if pfree
        == ::core::mem::transmute::<
            libc::intptr_t,
            binn_mem_free,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        (*item).ptr = binn_memdup(ptr, size);
        if ((*item).ptr).is_null() {
            return 0 as libc::c_int;
        }
        (*item).freefn = free_fn;
    } else {
        (*item).ptr = ptr;
        (*item).freefn = pfree;
    }
    (*item).type_0 = 0xc0 as libc::c_int;
    (*item).size = size;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn atoi64(mut str: *mut libc::c_char) -> int64 {
    let mut retval: int64 = 0;
    let mut is_negative: libc::c_int = 0 as libc::c_int;
    if *str as libc::c_int == '-' as i32 {
        is_negative = 1 as libc::c_int;
        str = str.offset(1);
        str;
    }
    retval = 0 as libc::c_int as int64;
    while *str != 0 {
        retval = 10 as libc::c_int as libc::c_longlong * retval
            + (*str as libc::c_int - '0' as i32) as libc::c_longlong;
        str = str.offset(1);
        str;
    }
    if is_negative != 0 {
        retval *= -(1 as libc::c_int) as libc::c_longlong;
    }
    return retval;
}
unsafe extern "C" fn is_integer(mut p: *mut libc::c_char) -> BOOL {
    let mut retval: BOOL = 0;
    if p.is_null() {
        return 0 as libc::c_int;
    }
    if *p as libc::c_int == '-' as i32 {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    retval = 1 as libc::c_int;
    while *p != 0 {
        if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '9' as i32 {
            retval = 0 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    return retval;
}
unsafe extern "C" fn is_float(mut p: *mut libc::c_char) -> BOOL {
    let mut retval: BOOL = 0;
    let mut number_found: BOOL = 0 as libc::c_int;
    if p.is_null() {
        return 0 as libc::c_int;
    }
    if *p as libc::c_int == '-' as i32 {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    retval = 1 as libc::c_int;
    while *p != 0 {
        if *p as libc::c_int == '.' as i32 || *p as libc::c_int == ',' as i32 {
            if number_found == 0 {
                retval = 0 as libc::c_int;
            }
        } else if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
            number_found = 1 as libc::c_int;
        } else {
            return 0 as libc::c_int
        }
        p = p.offset(1);
        p;
    }
    return retval;
}
unsafe extern "C" fn is_bool_str(
    mut str: *mut libc::c_char,
    mut pbool: *mut BOOL,
) -> BOOL {
    let mut vint: int64 = 0;
    let mut vdouble: libc::c_double = 0.;
    if str.is_null() || pbool.is_null() {
        return 0 as libc::c_int;
    }
    if !(strcasecmp(str, b"true\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int)
    {
        if !(strcasecmp(str, b"yes\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int)
        {
            if !(strcasecmp(str, b"on\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int)
            {
                if !(strcasecmp(str, b"false\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int)
                {
                    if !(strcasecmp(str, b"no\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int)
                    {
                        if !(strcasecmp(
                            str,
                            b"off\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int)
                        {
                            if is_integer(str) != 0 {
                                vint = atoi64(str);
                                *pbool = if vint != 0 as libc::c_int as libc::c_longlong {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                };
                                return 1 as libc::c_int;
                            } else if is_float(str) != 0 {
                                vdouble = atof(str);
                                *pbool = if vdouble != 0 as libc::c_int as libc::c_double {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                };
                                return 1 as libc::c_int;
                            }
                            return 0 as libc::c_int;
                        }
                    }
                }
                *pbool = 0 as libc::c_int;
                return 1 as libc::c_int;
            }
        }
    }
    *pbool = 1 as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_int32(
    mut value: *mut binn,
    mut pint: *mut libc::c_int,
) -> BOOL {
    if value.is_null() || pint.is_null() {
        return 0 as libc::c_int;
    }
    if type_family((*value).type_0) == 0xf2 as libc::c_int {
        return copy_int_value(
            (*value).ptr,
            pint as *mut libc::c_void,
            (*value).type_0,
            0x61 as libc::c_int,
        );
    }
    match (*value).type_0 {
        98 => {
            if (*value).c2rust_unnamed.vfloat
                < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_float
                || (*value).c2rust_unnamed.vfloat
                    > 2147483647 as libc::c_int as libc::c_float
            {
                return 0 as libc::c_int;
            }
            *pint = if (*value).c2rust_unnamed.vfloat as libc::c_double >= 0.0f64 {
                ((*value).c2rust_unnamed.vfloat as libc::c_double + 0.5f64)
                    as libc::c_int
            } else if (*value).c2rust_unnamed.vfloat as libc::c_double
                - (*value).c2rust_unnamed.vfloat as libc::c_int as libc::c_double
                <= -0.5f64
            {
                (*value).c2rust_unnamed.vfloat as libc::c_int
            } else {
                ((*value).c2rust_unnamed.vfloat as libc::c_double - 0.5f64)
                    as libc::c_int
            };
        }
        130 => {
            if (*value).c2rust_unnamed.vdouble
                < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
                || (*value).c2rust_unnamed.vdouble
                    > 2147483647 as libc::c_int as libc::c_double
            {
                return 0 as libc::c_int;
            }
            *pint = if (*value).c2rust_unnamed.vdouble >= 0.0f64 {
                ((*value).c2rust_unnamed.vdouble + 0.5f64) as libc::c_int
            } else if (*value).c2rust_unnamed.vdouble
                - (*value).c2rust_unnamed.vdouble as libc::c_int as libc::c_double
                <= -0.5f64
            {
                (*value).c2rust_unnamed.vdouble as libc::c_int
            } else {
                ((*value).c2rust_unnamed.vdouble - 0.5f64) as libc::c_int
            };
        }
        160 => {
            if is_integer((*value).ptr as *mut libc::c_char) != 0 {
                *pint = atoi((*value).ptr as *mut libc::c_char);
            } else if is_float((*value).ptr as *mut libc::c_char) != 0 {
                *pint = if atof((*value).ptr as *mut libc::c_char) >= 0.0f64 {
                    (atof((*value).ptr as *mut libc::c_char) + 0.5f64) as libc::c_int
                } else if atof((*value).ptr as *mut libc::c_char)
                    - atof((*value).ptr as *mut libc::c_char) as libc::c_int
                        as libc::c_double <= -0.5f64
                {
                    atof((*value).ptr as *mut libc::c_char) as libc::c_int
                } else {
                    (atof((*value).ptr as *mut libc::c_char) - 0.5f64) as libc::c_int
                };
            } else {
                return 0 as libc::c_int
            }
        }
        524385 => {
            *pint = (*value).c2rust_unnamed.vbool;
        }
        _ => return 0 as libc::c_int,
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_int64(
    mut value: *mut binn,
    mut pint: *mut int64,
) -> BOOL {
    if value.is_null() || pint.is_null() {
        return 0 as libc::c_int;
    }
    if type_family((*value).type_0) == 0xf2 as libc::c_int {
        return copy_int_value(
            (*value).ptr,
            pint as *mut libc::c_void,
            (*value).type_0,
            0x81 as libc::c_int,
        );
    }
    match (*value).type_0 {
        98 => {
            if (*value).c2rust_unnamed.vfloat
                < (-(9223372036854775807 as libc::c_long)
                    - 1 as libc::c_int as libc::c_long) as libc::c_float
                || (*value).c2rust_unnamed.vfloat
                    > 9223372036854775807 as libc::c_long as libc::c_float
            {
                return 0 as libc::c_int;
            }
            *pint = (if (*value).c2rust_unnamed.vfloat as libc::c_double >= 0.0f64 {
                ((*value).c2rust_unnamed.vfloat as libc::c_double + 0.5f64)
                    as libc::c_int
            } else if (*value).c2rust_unnamed.vfloat as libc::c_double
                - (*value).c2rust_unnamed.vfloat as libc::c_int as libc::c_double
                <= -0.5f64
            {
                (*value).c2rust_unnamed.vfloat as libc::c_int
            } else {
                ((*value).c2rust_unnamed.vfloat as libc::c_double - 0.5f64)
                    as libc::c_int
            }) as int64;
        }
        130 => {
            if (*value).c2rust_unnamed.vdouble
                < (-(9223372036854775807 as libc::c_long)
                    - 1 as libc::c_int as libc::c_long) as libc::c_double
                || (*value).c2rust_unnamed.vdouble
                    > 9223372036854775807 as libc::c_long as libc::c_double
            {
                return 0 as libc::c_int;
            }
            *pint = (if (*value).c2rust_unnamed.vdouble >= 0.0f64 {
                ((*value).c2rust_unnamed.vdouble + 0.5f64) as libc::c_int
            } else if (*value).c2rust_unnamed.vdouble
                - (*value).c2rust_unnamed.vdouble as libc::c_int as libc::c_double
                <= -0.5f64
            {
                (*value).c2rust_unnamed.vdouble as libc::c_int
            } else {
                ((*value).c2rust_unnamed.vdouble - 0.5f64) as libc::c_int
            }) as int64;
        }
        160 => {
            if is_integer((*value).ptr as *mut libc::c_char) != 0 {
                *pint = atoi64((*value).ptr as *mut libc::c_char);
            } else if is_float((*value).ptr as *mut libc::c_char) != 0 {
                *pint = (if atof((*value).ptr as *mut libc::c_char) >= 0.0f64 {
                    (atof((*value).ptr as *mut libc::c_char) + 0.5f64) as libc::c_int
                } else if atof((*value).ptr as *mut libc::c_char)
                    - atof((*value).ptr as *mut libc::c_char) as libc::c_int
                        as libc::c_double <= -0.5f64
                {
                    atof((*value).ptr as *mut libc::c_char) as libc::c_int
                } else {
                    (atof((*value).ptr as *mut libc::c_char) - 0.5f64) as libc::c_int
                }) as int64;
            } else {
                return 0 as libc::c_int
            }
        }
        524385 => {
            *pint = (*value).c2rust_unnamed.vbool as int64;
        }
        _ => return 0 as libc::c_int,
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_double(
    mut value: *mut binn,
    mut pfloat: *mut libc::c_double,
) -> BOOL {
    let mut vint: int64 = 0;
    if value.is_null() || pfloat.is_null() {
        return 0 as libc::c_int;
    }
    if type_family((*value).type_0) == 0xf2 as libc::c_int {
        if copy_int_value(
            (*value).ptr,
            &mut vint as *mut int64 as *mut libc::c_void,
            (*value).type_0,
            0x81 as libc::c_int,
        ) == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        *pfloat = vint as libc::c_double;
        return 1 as libc::c_int;
    }
    match (*value).type_0 {
        98 => {
            *pfloat = (*value).c2rust_unnamed.vfloat as libc::c_double;
        }
        130 => {
            *pfloat = (*value).c2rust_unnamed.vdouble;
        }
        160 => {
            if is_integer((*value).ptr as *mut libc::c_char) != 0 {
                *pfloat = atoi64((*value).ptr as *mut libc::c_char) as libc::c_double;
            } else if is_float((*value).ptr as *mut libc::c_char) != 0 {
                *pfloat = atof((*value).ptr as *mut libc::c_char);
            } else {
                return 0 as libc::c_int
            }
        }
        524385 => {
            *pfloat = (*value).c2rust_unnamed.vbool as libc::c_double;
        }
        _ => return 0 as libc::c_int,
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_bool(
    mut value: *mut binn,
    mut pbool: *mut BOOL,
) -> BOOL {
    let mut vint: int64 = 0;
    if value.is_null() || pbool.is_null() {
        return 0 as libc::c_int;
    }
    if type_family((*value).type_0) == 0xf2 as libc::c_int {
        if copy_int_value(
            (*value).ptr,
            &mut vint as *mut int64 as *mut libc::c_void,
            (*value).type_0,
            0x81 as libc::c_int,
        ) == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        *pbool = if vint != 0 as libc::c_int as libc::c_longlong {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        return 1 as libc::c_int;
    }
    match (*value).type_0 {
        524385 => {
            *pbool = (*value).c2rust_unnamed.vbool;
        }
        98 => {
            *pbool = if (*value).c2rust_unnamed.vfloat
                != 0 as libc::c_int as libc::c_float
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
        }
        130 => {
            *pbool = if (*value).c2rust_unnamed.vdouble
                != 0 as libc::c_int as libc::c_double
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
        }
        160 => return is_bool_str((*value).ptr as *mut libc::c_char, pbool),
        _ => return 0 as libc::c_int,
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_str(mut value: *mut binn) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut vint: int64 = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    if value.is_null() {
        return 0 as *mut libc::c_char;
    }
    if type_family((*value).type_0) == 0xf2 as libc::c_int {
        if copy_int_value(
            (*value).ptr,
            &mut vint as *mut int64 as *mut libc::c_void,
            (*value).type_0,
            0x81 as libc::c_int,
        ) == 0 as libc::c_int
        {
            return 0 as *mut libc::c_char;
        }
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"%lli\0" as *const u8 as *const libc::c_char,
            vint,
        );
    } else {
        match (*value).type_0 {
            98 => {
                (*value)
                    .c2rust_unnamed
                    .vdouble = (*value).c2rust_unnamed.vfloat as libc::c_double;
                current_block = 3590440023938386005;
            }
            130 => {
                current_block = 3590440023938386005;
            }
            160 => return (*value).ptr as *mut libc::c_char,
            524385 => {
                if (*value).c2rust_unnamed.vbool != 0 {
                    strcpy(
                        buf.as_mut_ptr(),
                        b"true\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    strcpy(
                        buf.as_mut_ptr(),
                        b"false\0" as *const u8 as *const libc::c_char,
                    );
                }
                current_block = 16314702126279054567;
            }
            _ => return 0 as *mut libc::c_char,
        }
        match current_block {
            16314702126279054567 => {}
            _ => {
                snprintf(
                    buf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    b"%g\0" as *const u8 as *const libc::c_char,
                    (*value).c2rust_unnamed.vdouble,
                );
            }
        }
    }
    (*value).ptr = strdup(buf.as_mut_ptr()) as *mut libc::c_void;
    if ((*value).ptr).is_null() {
        return 0 as *mut libc::c_char;
    }
    (*value).freefn = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*value).type_0 = 0xa0 as libc::c_int;
    return (*value).ptr as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn binn_is_container(mut item: *mut binn) -> BOOL {
    if item.is_null() {
        return 0 as libc::c_int;
    }
    match (*item).type_0 {
        224 | 225 | 226 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
