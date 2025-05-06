#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type hash_table;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: i32) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strspn(_: *const i8, _: *const i8) -> u64;
    fn mempcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn faccessat(__fd: i32, __file: *const i8, __type: i32, __flag: i32) -> i32;
    fn getcwd(__buf: *mut i8, __size: size_t) -> *mut i8;
    fn readlink(__path: *const i8, __buf: *mut i8, __len: size_t) -> ssize_t;
    fn gl_scratch_buffer_grow(buffer: *mut scratch_buffer) -> bool;
    fn gl_scratch_buffer_grow_preserve(buffer: *mut scratch_buffer) -> bool;
    fn gl_scratch_buffer_dupfree(
        buffer: *mut scratch_buffer,
        size: size_t,
    ) -> *mut libc::c_void;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn seen_file(ht: *const Hash_table, file: *const i8, stats: *const stat) -> bool;
    fn record_file(ht: *mut Hash_table, file: *const i8, stats: *const stat);
    fn hash_free(table: *mut Hash_table);
    fn triple_hash(x: *const libc::c_void, table_size: size_t) -> size_t;
    fn triple_compare_ino_str(x: *const libc::c_void, y: *const libc::c_void) -> bool;
    fn triple_free(x: *mut libc::c_void);
    fn xalloc_die();
}
pub type size_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: libc::c_longlong,
    pub __clang_max_align_nonce2: f128::f128,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum canonicalize_mode_t {
    CAN_EXISTING = 0,
    CAN_ALL_BUT_LAST = 1,
    CAN_MISSING = 2,
    CAN_NOLINKS = 4,
}
impl canonicalize_mode_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            canonicalize_mode_t::CAN_EXISTING => 0,
            canonicalize_mode_t::CAN_ALL_BUT_LAST => 1,
            canonicalize_mode_t::CAN_MISSING => 2,
            canonicalize_mode_t::CAN_NOLINKS => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> canonicalize_mode_t {
        match value {
            0 => canonicalize_mode_t::CAN_EXISTING,
            1 => canonicalize_mode_t::CAN_ALL_BUT_LAST,
            2 => canonicalize_mode_t::CAN_MISSING,
            4 => canonicalize_mode_t::CAN_NOLINKS,
            _ => panic!("Invalid value for canonicalize_mode_t: {}", value),
        }
    }
}
impl AddAssign<u32> for canonicalize_mode_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for canonicalize_mode_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for canonicalize_mode_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for canonicalize_mode_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for canonicalize_mode_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for canonicalize_mode_t {
    type Output = canonicalize_mode_t;
    fn add(self, rhs: u32) -> canonicalize_mode_t {
        canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for canonicalize_mode_t {
    type Output = canonicalize_mode_t;
    fn sub(self, rhs: u32) -> canonicalize_mode_t {
        canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for canonicalize_mode_t {
    type Output = canonicalize_mode_t;
    fn mul(self, rhs: u32) -> canonicalize_mode_t {
        canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for canonicalize_mode_t {
    type Output = canonicalize_mode_t;
    fn div(self, rhs: u32) -> canonicalize_mode_t {
        canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for canonicalize_mode_t {
    type Output = canonicalize_mode_t;
    fn rem(self, rhs: u32) -> canonicalize_mode_t {
        canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scratch_buffer {
    pub data: *mut libc::c_void,
    pub length: size_t,
    pub __space: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __align: max_align_t,
    pub __c: [i8; 1024],
}
pub type Hash_table = hash_table;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type Hash_tuning = hash_tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_data_freer = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_comparator = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn scratch_buffer_init(mut buffer: *mut scratch_buffer) {
    (*buffer).data = ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void;
    (*buffer).length = ::core::mem::size_of::<C2RustUnnamed>() as u64;
}
#[inline]
unsafe extern "C" fn scratch_buffer_free(mut buffer: *mut scratch_buffer) {
    if (*buffer).data != ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void {
        rpl_free((*buffer).data);
    }
}
#[inline(always)]
unsafe extern "C" fn scratch_buffer_grow(mut buffer: *mut scratch_buffer) -> bool {
    return gl_scratch_buffer_grow(buffer) as i64 != 0;
}
#[inline(always)]
unsafe extern "C" fn scratch_buffer_grow_preserve(
    mut buffer: *mut scratch_buffer,
) -> bool {
    return gl_scratch_buffer_grow_preserve(buffer) as i64 != 0;
}
#[inline(always)]
unsafe extern "C" fn scratch_buffer_dupfree(
    mut buffer: *mut scratch_buffer,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut r: *mut libc::c_void = gl_scratch_buffer_dupfree(buffer, size);
    return if (r != 0 as *mut libc::c_void) as i32 as i64 != 0 {
        r
    } else {
        0 as *mut libc::c_void
    };
}
unsafe extern "C" fn file_accessible(mut file: *const i8) -> bool {
    return faccessat(-(100 as i32), file, 0 as i32, 0x200 as i32) == 0 as i32;
}
unsafe extern "C" fn suffix_requires_dir_check(mut end: *const i8) -> bool {
    while *end as i32 == '/' as i32 {
        loop {
            end = end.offset(1);
            end;
            if !(*end as i32 == '/' as i32) {
                break;
            }
        }
        let fresh0 = end;
        end = end.offset(1);
        match *fresh0 as i32 {
            0 => return 1 as i32 != 0,
            46 => {}
            _ => return 0 as i32 != 0,
        }
        if *end == 0
            || *end as i32 == '.' as i32
                && (*end.offset(1 as i32 as isize) == 0
                    || *end.offset(1 as i32 as isize) as i32 == '/' as i32)
        {
            return 1 as i32 != 0;
        }
    }
    return 0 as i32 != 0;
}
static mut dir_suffix: [i8; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &[i8; 2]>(b"/\0")
};
unsafe extern "C" fn dir_check(mut dir: *mut i8, mut dirend: *mut i8) -> bool {
    strcpy(dirend, dir_suffix.as_ptr());
    return file_accessible(dir);
}
unsafe extern "C" fn multiple_bits_set(mut i: canonicalize_mode_t) -> bool {
    return i as u32 & (i as u32).wrapping_sub(1 as i32 as u32) != 0 as i32 as u32;
}
unsafe extern "C" fn seen_triple(
    mut ht: *mut *mut Hash_table,
    mut filename: *const i8,
    mut st: *const stat,
) -> bool {
    if (*ht).is_null() {
        let mut initial_capacity: idx_t = 7 as i32 as idx_t;
        *ht = hash_initialize(
            initial_capacity as size_t,
            0 as *const Hash_tuning,
            Some(
                triple_hash
                    as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
            ),
            Some(
                triple_compare_ino_str
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> bool,
            ),
            Some(triple_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        if (*ht).is_null() {
            xalloc_die();
        }
    }
    if seen_file(*ht, filename, st) {
        return 1 as i32 != 0;
    }
    record_file(*ht, filename, st);
    return 0 as i32 != 0;
}
unsafe extern "C" fn canonicalize_filename_mode_stk(
    mut name: *const i8,
    mut can_mode: canonicalize_mode_t,
    mut rname_buf: *mut scratch_buffer,
) -> *mut i8 {
    let mut current_block: u64;
    let mut dest: *mut i8 = 0 as *mut i8;
    let mut start: *const i8 = 0 as *const i8;
    let mut end: *const i8 = 0 as *const i8;
    let mut ht: *mut Hash_table = 0 as *mut Hash_table;
    let mut logical: bool = can_mode as u32
        & canonicalize_mode_t::CAN_NOLINKS as i32 as u32 != 0 as i32 as u32;
    let mut num_links: i32 = 0 as i32;
    let mut can_exist: canonicalize_mode_t = canonicalize_mode_t::from_libc_c_uint(
        (can_mode as u32
            & (canonicalize_mode_t::CAN_EXISTING as i32
                | canonicalize_mode_t::CAN_ALL_BUT_LAST as i32
                | canonicalize_mode_t::CAN_MISSING as i32) as u32) as u32,
    );
    if multiple_bits_set(can_exist) {
        *__errno_location() = 22 as i32;
        return 0 as *mut i8;
    }
    if name.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as *mut i8;
    }
    if *name.offset(0 as i32 as isize) as i32 == '\0' as i32 {
        *__errno_location() = 2 as i32;
        return 0 as *mut i8;
    }
    let mut extra_buffer: scratch_buffer = scratch_buffer {
        data: 0 as *mut libc::c_void,
        length: 0,
        __space: C2RustUnnamed {
            __align: max_align_t {
                __clang_max_align_nonce1: 0,
                __clang_max_align_nonce2: f128::f128::ZERO,
            },
        },
    };
    let mut link_buffer: scratch_buffer = scratch_buffer {
        data: 0 as *mut libc::c_void,
        length: 0,
        __space: C2RustUnnamed {
            __align: max_align_t {
                __clang_max_align_nonce1: 0,
                __clang_max_align_nonce2: f128::f128::ZERO,
            },
        },
    };
    scratch_buffer_init(&mut extra_buffer);
    scratch_buffer_init(&mut link_buffer);
    scratch_buffer_init(rname_buf);
    let mut rname_on_stack: *mut i8 = (*rname_buf).data as *mut i8;
    let mut rname: *mut i8 = rname_on_stack;
    let mut end_in_extra_buffer: bool = 0 as i32 != 0;
    let mut failed: bool = 1 as i32 != 0;
    let mut prefix_len: idx_t = 0 as i32 as idx_t;
    if !(*name.offset(0 as i32 as isize) as i32 == '/' as i32) {
        loop {
            if !(getcwd(rname, (*rname_buf).length)).is_null() {
                current_block = 14401909646449704462;
                break;
            }
            match *__errno_location() {
                34 => {
                    if scratch_buffer_grow(rname_buf) {
                        rname = (*rname_buf).data as *mut i8;
                        continue;
                    } else {
                        current_block = 16828662993660315854;
                    }
                }
                12 => {
                    current_block = 16828662993660315854;
                }
                _ => {
                    current_block = 10806129942658850455;
                }
            }
            match current_block {
                16828662993660315854 => {
                    xalloc_die();
                }
                _ => {}
            }
            dest = rname;
            current_block = 3632074250920186344;
            break;
        }
        match current_block {
            3632074250920186344 => {}
            _ => {
                dest = rawmemchr(rname as *const libc::c_void, '\0' as i32) as *mut i8;
                start = name;
                prefix_len = 0 as i32 as idx_t;
                current_block = 8704759739624374314;
            }
        }
    } else {
        dest = mempcpy(
            rname as *mut libc::c_void,
            name as *const libc::c_void,
            prefix_len as size_t,
        ) as *mut i8;
        let fresh1 = dest;
        dest = dest.offset(1);
        *fresh1 = '/' as i32 as i8;
        start = name.offset(prefix_len as isize);
        current_block = 8704759739624374314;
    }
    loop {
        match current_block {
            3632074250920186344 => {
                if !ht.is_null() {
                    hash_free(ht);
                }
                break;
            }
            _ => {
                if *start != 0 {
                    while *start as i32 == '/' as i32 {
                        start = start.offset(1);
                        start;
                    }
                    end = start;
                    while *end as i32 != 0 && !(*end as i32 == '/' as i32) {
                        end = end.offset(1);
                        end;
                    }
                    let mut startlen: idx_t = end.offset_from(start) as i64;
                    if !(startlen == 0 as i32 as i64) {
                        if !(startlen == 1 as i32 as i64
                            && *start.offset(0 as i32 as isize) as i32 == '.' as i32)
                        {
                            if startlen == 2 as i32 as i64
                                && *start.offset(0 as i32 as isize) as i32 == '.' as i32
                                && *start.offset(1 as i32 as isize) as i32 == '.' as i32
                            {
                                if dest
                                    > rname
                                        .offset(prefix_len as isize)
                                        .offset(1 as i32 as isize)
                                {
                                    dest = dest.offset(-1);
                                    dest;
                                    while dest > rname
                                        && !(*dest.offset(-(1 as i32) as isize) as i32
                                            == '/' as i32)
                                    {
                                        dest = dest.offset(-1);
                                        dest;
                                    }
                                }
                                if 0 as i32 != 0 && dest == rname.offset(1 as i32 as isize)
                                    && prefix_len == 0 && *dest as i32 == '/' as i32
                                    && !(*dest.offset(1 as i32 as isize) as i32 == '/' as i32)
                                {
                                    dest = dest.offset(1);
                                    dest;
                                }
                            } else {
                                if !(*dest.offset(-(1 as i32) as isize) as i32
                                    == '/' as i32)
                                {
                                    let fresh3 = dest;
                                    dest = dest.offset(1);
                                    *fresh3 = '/' as i32 as i8;
                                }
                                while (rname
                                    .offset((*rname_buf).length as isize)
                                    .offset_from(dest) as i64 as u64)
                                    < (startlen as u64)
                                        .wrapping_add(::core::mem::size_of::<[i8; 2]>() as u64)
                                {
                                    let mut dest_offset: idx_t = dest.offset_from(rname) as i64;
                                    if !scratch_buffer_grow_preserve(rname_buf) {
                                        xalloc_die();
                                    }
                                    rname = (*rname_buf).data as *mut i8;
                                    dest = rname.offset(dest_offset as isize);
                                }
                                dest = mempcpy(
                                    dest as *mut libc::c_void,
                                    start as *const libc::c_void,
                                    startlen as size_t,
                                ) as *mut i8;
                                *dest = '\0' as i32 as i8;
                                let mut buf: *mut i8 = 0 as *mut i8;
                                let mut n: ssize_t = -(1 as i32) as ssize_t;
                                if !logical {
                                    loop {
                                        buf = link_buffer.data as *mut i8;
                                        let mut bufsize: idx_t = link_buffer.length as idx_t;
                                        n = readlink(
                                            rname,
                                            buf,
                                            (bufsize - 1 as i32 as i64) as size_t,
                                        );
                                        if n < bufsize - 1 as i32 as i64 {
                                            break;
                                        }
                                        if !scratch_buffer_grow(&mut link_buffer) {
                                            xalloc_die();
                                        }
                                    }
                                }
                                if 0 as i32 as i64 <= n {
                                    if num_links < 20 as i32 {
                                        num_links += 1;
                                        num_links;
                                        current_block = 9437375157805982253;
                                    } else if *start != 0 {
                                        let mut st: stat = stat {
                                            st_dev: 0,
                                            st_ino: 0,
                                            st_nlink: 0,
                                            st_mode: 0,
                                            st_uid: 0,
                                            st_gid: 0,
                                            __pad0: 0,
                                            st_rdev: 0,
                                            st_size: 0,
                                            st_blksize: 0,
                                            st_blocks: 0,
                                            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                                            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                                            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                                            __glibc_reserved: [0; 3],
                                        };
                                        *dest.offset(-startlen as isize) = '\0' as i32 as i8;
                                        if stat(
                                            (if *rname as i32 != 0 {
                                                rname
                                            } else {
                                                b".\0" as *const u8 as *const i8
                                            }),
                                            &mut st,
                                        ) != 0 as i32
                                        {
                                            current_block = 3632074250920186344;
                                            continue;
                                        }
                                        *dest.offset(-startlen as isize) = *start;
                                        if seen_triple(&mut ht, start, &mut st) {
                                            if can_exist as u32
                                                == canonicalize_mode_t::CAN_MISSING as i32 as u32
                                            {
                                                current_block = 1118134448028020070;
                                            } else {
                                                *__errno_location() = 40 as i32;
                                                current_block = 3632074250920186344;
                                                continue;
                                            }
                                        } else {
                                            current_block = 9437375157805982253;
                                        }
                                    } else {
                                        current_block = 9437375157805982253;
                                    }
                                    match current_block {
                                        1118134448028020070 => {}
                                        _ => {
                                            *buf.offset(n as isize) = '\0' as i32 as i8;
                                            let mut extra_buf: *mut i8 = extra_buffer.data as *mut i8;
                                            let mut end_idx: idx_t = 0;
                                            if end_in_extra_buffer {
                                                end_idx = end.offset_from(extra_buf) as i64;
                                            }
                                            let mut len: size_t = strlen(end);
                                            if if (if (if 1 as i32 != 0 {
                                                0 as i32 as u64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { len })
                                                    .wrapping_add(n as u64)
                                            })
                                                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                                            {
                                                !((if 1 as i32 != 0 {
                                                    0 as i32 as u64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { len })
                                                        .wrapping_add(n as u64)
                                                })
                                                    .wrapping_add(1 as i32 as u64)
                                                    << (::core::mem::size_of::<u64>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64))
                                                    .wrapping_sub(1 as i32 as u64)
                                                    .wrapping_mul(2 as i32 as u64)
                                                    .wrapping_add(1 as i32 as u64)
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as u64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { len })
                                                        .wrapping_add(n as u64)
                                                })
                                                    .wrapping_add(0 as i32 as u64)
                                            }) < 0 as i32 as u64
                                            {
                                                if n < 0 as i32 as i64 {
                                                    (len
                                                        < (if (if 1 as i32 != 0 {
                                                            0 as i32 as u64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { len })
                                                                .wrapping_add(n as u64)
                                                        })
                                                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                                                        {
                                                            !((if 1 as i32 != 0 {
                                                                0 as i32 as u64
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { len })
                                                                    .wrapping_add(n as u64)
                                                            })
                                                                .wrapping_add(1 as i32 as u64)
                                                                << (::core::mem::size_of::<u64>() as u64)
                                                                    .wrapping_mul(8 as i32 as u64)
                                                                    .wrapping_sub(2 as i32 as u64))
                                                                .wrapping_sub(1 as i32 as u64)
                                                                .wrapping_mul(2 as i32 as u64)
                                                                .wrapping_add(1 as i32 as u64)
                                                        } else {
                                                            (if 1 as i32 != 0 {
                                                                0 as i32 as u64
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { len })
                                                                    .wrapping_add(n as u64)
                                                            })
                                                                .wrapping_add(0 as i32 as u64)
                                                        })
                                                            .wrapping_sub(n as u64)) as i32
                                                } else {
                                                    ((if (if 1 as i32 != 0 {
                                                        0 as i32 as u64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { len })
                                                            .wrapping_add(n as u64)
                                                    })
                                                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                                                    {
                                                        ((if 1 as i32 != 0 {
                                                            0 as i32 as u64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { len })
                                                                .wrapping_add(n as u64)
                                                        })
                                                            .wrapping_add(1 as i32 as u64)
                                                            << (::core::mem::size_of::<u64>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64))
                                                            .wrapping_sub(1 as i32 as u64)
                                                            .wrapping_mul(2 as i32 as u64)
                                                            .wrapping_add(1 as i32 as u64)
                                                    } else {
                                                        (if 1 as i32 != 0 {
                                                            0 as i32 as u64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { len })
                                                                .wrapping_add(n as u64)
                                                        })
                                                            .wrapping_sub(1 as i32 as u64)
                                                    })
                                                        .wrapping_sub(n as u64) < len) as i32
                                                }
                                            } else if len < 0 as i32 as u64 {
                                                (n as u64 <= len.wrapping_add(n as u64)) as i32
                                            } else if n < 0 as i32 as i64 {
                                                (len <= len.wrapping_add(n as u64)) as i32
                                            } else {
                                                (len.wrapping_add(n as u64) < n as u64) as i32
                                            } != 0
                                            {
                                                xalloc_die();
                                            }
                                            while extra_buffer.length <= len.wrapping_add(n as u64) {
                                                if !scratch_buffer_grow_preserve(&mut extra_buffer) {
                                                    xalloc_die();
                                                }
                                                extra_buf = extra_buffer.data as *mut i8;
                                            }
                                            if end_in_extra_buffer {
                                                end = extra_buf.offset(end_idx as isize);
                                            }
                                            memmove(
                                                &mut *extra_buf.offset(n as isize) as *mut i8
                                                    as *mut libc::c_void,
                                                end as *const libc::c_void,
                                                len.wrapping_add(1 as i32 as u64),
                                            );
                                            end = memcpy(
                                                extra_buf as *mut libc::c_void,
                                                buf as *const libc::c_void,
                                                n as u64,
                                            ) as *const i8;
                                            name = end;
                                            end_in_extra_buffer = 1 as i32 != 0;
                                            if *buf.offset(0 as i32 as isize) as i32 == '/' as i32 {
                                                let mut pfxlen: idx_t = 0 as i32 as idx_t;
                                                dest = mempcpy(
                                                    rname as *mut libc::c_void,
                                                    buf as *const libc::c_void,
                                                    pfxlen as size_t,
                                                ) as *mut i8;
                                                let fresh4 = dest;
                                                dest = dest.offset(1);
                                                *fresh4 = '/' as i32 as i8;
                                                prefix_len = pfxlen;
                                            } else {
                                                if dest
                                                    > rname
                                                        .offset(prefix_len as isize)
                                                        .offset(1 as i32 as isize)
                                                {
                                                    dest = dest.offset(-1);
                                                    dest;
                                                    while dest > rname
                                                        && !(*dest.offset(-(1 as i32) as isize) as i32
                                                            == '/' as i32)
                                                    {
                                                        dest = dest.offset(-1);
                                                        dest;
                                                    }
                                                }
                                                if 0 as i32 != 0 && dest == rname.offset(1 as i32 as isize)
                                                    && *dest as i32 == '/' as i32
                                                    && !(*dest.offset(1 as i32 as isize) as i32 == '/' as i32)
                                                    && prefix_len == 0
                                                {
                                                    dest = dest.offset(1);
                                                    dest;
                                                }
                                            }
                                        }
                                    }
                                } else if !(can_exist as u32
                                    == canonicalize_mode_t::CAN_MISSING as i32 as u32
                                    || (if suffix_requires_dir_check(end) as i32 != 0 {
                                        dir_check(rname, dest) as i32
                                    } else {
                                        (if !logical {
                                            (*__errno_location() == 22 as i32) as i32
                                        } else {
                                            (*end as i32 != 0 || file_accessible(rname) as i32 != 0)
                                                as i32
                                        })
                                    }) != 0
                                    || can_exist as u32
                                        == canonicalize_mode_t::CAN_ALL_BUT_LAST as i32 as u32
                                        && *__errno_location() == 2 as i32
                                        && *end
                                            .offset(
                                                strspn(end, b"/\0" as *const u8 as *const i8) as isize,
                                            ) == 0)
                                {
                                    current_block = 3632074250920186344;
                                    continue;
                                }
                            }
                        }
                        start = end;
                        current_block = 8704759739624374314;
                        continue;
                    }
                }
                if dest > rname.offset(prefix_len as isize).offset(1 as i32 as isize)
                    && *dest.offset(-(1 as i32) as isize) as i32 == '/' as i32
                {
                    dest = dest.offset(-1);
                    dest;
                }
                if 0 as i32 != 0 && dest == rname.offset(1 as i32 as isize)
                    && prefix_len == 0 && *dest as i32 == '/' as i32
                    && !(*dest.offset(1 as i32 as isize) as i32 == '/' as i32)
                {
                    dest = dest.offset(1);
                    dest;
                }
                failed = 0 as i32 != 0;
                current_block = 3632074250920186344;
            }
        }
    }
    scratch_buffer_free(&mut extra_buffer);
    scratch_buffer_free(&mut link_buffer);
    if failed {
        scratch_buffer_free(rname_buf);
        return 0 as *mut i8;
    }
    let fresh6 = dest;
    dest = dest.offset(1);
    *fresh6 = '\0' as i32 as i8;
    let mut result: *mut i8 = scratch_buffer_dupfree(
        rname_buf,
        dest.offset_from(rname) as i64 as size_t,
    ) as *mut i8;
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn canonicalize_filename_mode(
    mut name: *const i8,
    mut can_mode: canonicalize_mode_t,
) -> *mut i8 {
    let mut rname_buffer: scratch_buffer = scratch_buffer {
        data: 0 as *mut libc::c_void,
        length: 0,
        __space: C2RustUnnamed {
            __align: max_align_t {
                __clang_max_align_nonce1: 0,
                __clang_max_align_nonce2: f128::f128::ZERO,
            },
        },
    };
    return canonicalize_filename_mode_stk(name, can_mode, &mut rname_buffer);
}