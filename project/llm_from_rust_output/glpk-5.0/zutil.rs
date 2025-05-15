pub type uInt = u32;
pub type uLong = u64;
pub type voidpf = *mut std::ffi::c_void;

pub static GLP_ZLIB_Z_ERRMSG: [&str; 10] = [
    "need dictionary",
    "stream end",
    "",
    "file error",
    "stream error",
    "data error",
    "insufficient memory",
    "buffer error",
    "incompatible version",
    "",
];

pub fn glp_zlib_zlib_version() -> &'static str {
    "1.2.5"
}

pub fn glp_zlib_zlib_compile_flags() -> uLong {
    let mut flags = 0u64;

    match std::mem::size_of::<uInt>() {
        2 => {}
        4 => flags += 1,
        8 => flags += 2,
        _ => flags += 3,
    }

    match std::mem::size_of::<uLong>() {
        2 => {}
        4 => flags += (1 << 2),
        8 => flags += (2 << 2),
        _ => flags += (3 << 2),
    }

    match std::mem::size_of::<voidpf>() {
        2 => {}
        4 => flags += (1 << 4),
        8 => flags += (2 << 4),
        _ => flags += (3 << 4),
    }

    match std::mem::size_of::<usize>() {
        2 => {}
        4 => flags += (1 << 6),
        8 => flags += (2 << 6),
        _ => flags += (3 << 6),
    }

    flags += (1 << 25);
    flags
}

pub fn glp_zlib_z_error(err: i32) -> &'static str {
    let idx = (2 - err) as usize;
    if idx < GLP_ZLIB_Z_ERRMSG.len() {
        GLP_ZLIB_Z_ERRMSG[idx]
    } else {
        ""
    }
}

pub fn glp_zlib_zcalloc(items: u32, size: u32) -> Option<std::ptr::NonNull<std::ffi::c_void>> {
    let total_size = items as usize * size as usize;
    let ptr = if std::mem::size_of::<uInt>() > 2 {
        unsafe { libc::malloc(total_size) }
    } else {
        unsafe { libc::calloc(items as usize, size as usize) }
    };

    if ptr.is_null() {
        None
    } else {
        Some(unsafe { std::ptr::NonNull::new_unchecked(ptr) })
    }
}

pub fn glp_zlib_zcfree(ptr: Option<std::ptr::NonNull<std::ffi::c_void>>) {
    if let Some(ptr) = ptr {
        unsafe { libc::free(ptr.as_ptr()) };
    }
}