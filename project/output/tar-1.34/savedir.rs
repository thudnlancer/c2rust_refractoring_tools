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
    pub type __dirstream;
    fn closedir(__dirp: *mut DIR) -> i32;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn __errno_location() -> *mut i32;
    fn opendir_safer(name: *const i8) -> *mut DIR;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn stpcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
}
pub type __ino_t = u64;
pub type __off_t = i64;
pub type ino_t = __ino_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: u8,
    pub d_name: [i8; 256],
}
pub type DIR = __dirstream;
pub type size_t = u64;
pub type savedir_option = u32;
pub const SAVEDIR_SORT_FASTREAD: savedir_option = 2;
pub const SAVEDIR_SORT_INODE: savedir_option = 2;
pub const SAVEDIR_SORT_NAME: savedir_option = 1;
pub const SAVEDIR_SORT_NONE: savedir_option = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub name: *mut i8,
    pub ino: ino_t,
}
pub type comparison_function = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    DEFAULT_MXFAST = 128,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::DEFAULT_MXFAST => 128,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            128 => C2RustUnnamed::DEFAULT_MXFAST,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (C2RustUnnamed::DEFAULT_MXFAST as i32 as u64).wrapping_div(s);
            n = (n as u64).wrapping_add((n == 0) as i32 as u64) as size_t as size_t;
        }
        if (if (9223372036854775807 as i64 as u64) < 18446744073709551615 as u64 {
            9223372036854775807 as i64 as u64
        } else {
            (18446744073709551615 as u64).wrapping_sub(1 as i32 as u64)
        })
            .wrapping_div(s) < n
        {
            xalloc_die();
        }
    } else {
        if (if (9223372036854775807 as i64 as u64) < 18446744073709551615 as u64 {
            9223372036854775807 as i64 as u64
        } else {
            18446744073709551615 as u64
        })
            .wrapping_div(3 as i32 as u64)
            .wrapping_mul(2 as i32 as u64)
            .wrapping_div(s) <= n
        {
            xalloc_die();
        }
        n = (n as u64)
            .wrapping_add(n.wrapping_div(2 as i32 as u64).wrapping_add(1 as i32 as u64))
            as size_t as size_t;
    }
    *pn = n;
    return xrealloc(p, n.wrapping_mul(s));
}
unsafe extern "C" fn direntry_cmp_name(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> i32 {
    let mut dea: *const direntry_t = a as *const direntry_t;
    let mut deb: *const direntry_t = b as *const direntry_t;
    return strcmp((*dea).name, (*deb).name);
}
unsafe extern "C" fn direntry_cmp_inode(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> i32 {
    let mut dea: *const direntry_t = a as *const direntry_t;
    let mut deb: *const direntry_t = b as *const direntry_t;
    return ((*dea).ino > (*deb).ino) as i32 - ((*dea).ino < (*deb).ino) as i32;
}
static mut comparison_function_table: [comparison_function; 3] = unsafe {
    [
        None,
        Some(
            direntry_cmp_name
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
        Some(
            direntry_cmp_inode
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn streamsavedir(
    mut dirp: *mut DIR,
    mut option: savedir_option,
) -> *mut i8 {
    let mut name_space: *mut i8 = 0 as *mut i8;
    let mut allocated: size_t = 0 as i32 as size_t;
    let mut entries: *mut direntry_t = 0 as *mut direntry_t;
    let mut entries_allocated: size_t = 0 as i32 as size_t;
    let mut entries_used: size_t = 0 as i32 as size_t;
    let mut used: size_t = 0 as i32 as size_t;
    let mut readdir_errno: i32 = 0;
    let mut cmp: comparison_function = comparison_function_table[option as usize];
    if dirp.is_null() {
        return 0 as *mut i8;
    }
    loop {
        let mut dp: *const dirent = 0 as *const dirent;
        let mut entry: *const i8 = 0 as *const i8;
        *__errno_location() = 0 as i32;
        dp = readdir(dirp);
        if dp.is_null() {
            break;
        }
        entry = ((*dp).d_name).as_ptr();
        if *entry
            .offset(
                (if *entry.offset(0 as i32 as isize) as i32 != '.' as i32 {
                    0 as i32
                } else {
                    (if *entry.offset(1 as i32 as isize) as i32 != '.' as i32 {
                        1 as i32
                    } else {
                        2 as i32
                    })
                }) as isize,
            ) as i32 != '\0' as i32
        {
            let mut entry_size: size_t = (strlen(((*dp).d_name).as_ptr()))
                .wrapping_add(1 as i32 as u64);
            if cmp.is_some() {
                if entries_allocated == entries_used {
                    let mut n: size_t = entries_allocated;
                    entries = x2nrealloc(
                        entries as *mut libc::c_void,
                        &mut n,
                        ::core::mem::size_of::<direntry_t>() as u64,
                    ) as *mut direntry_t;
                    entries_allocated = n;
                }
                let ref mut fresh0 = (*entries.offset(entries_used as isize)).name;
                *fresh0 = xstrdup(entry);
                (*entries.offset(entries_used as isize)).ino = (*dp).d_ino;
                entries_used = entries_used.wrapping_add(1);
                entries_used;
            } else {
                if allocated.wrapping_sub(used) <= entry_size {
                    let mut n_0: size_t = used.wrapping_add(entry_size);
                    if n_0 < used {
                        xalloc_die();
                    }
                    name_space = x2nrealloc(
                        name_space as *mut libc::c_void,
                        &mut n_0,
                        1 as i32 as size_t,
                    ) as *mut i8;
                    allocated = n_0;
                }
                memcpy(
                    name_space.offset(used as isize) as *mut libc::c_void,
                    entry as *const libc::c_void,
                    entry_size,
                );
            }
            used = (used as u64).wrapping_add(entry_size) as size_t as size_t;
        }
    }
    readdir_errno = *__errno_location();
    if readdir_errno != 0 as i32 {
        rpl_free(entries as *mut libc::c_void);
        rpl_free(name_space as *mut libc::c_void);
        *__errno_location() = readdir_errno;
        return 0 as *mut i8;
    }
    if cmp.is_some() {
        let mut i: size_t = 0;
        if entries_used != 0 {
            qsort(
                entries as *mut libc::c_void,
                entries_used,
                ::core::mem::size_of::<direntry_t>() as u64,
                cmp,
            );
        }
        name_space = xmalloc(used.wrapping_add(1 as i32 as u64)) as *mut i8;
        used = 0 as i32 as size_t;
        i = 0 as i32 as size_t;
        while i < entries_used {
            let mut dest: *mut i8 = name_space.offset(used as isize);
            used = (used as u64)
                .wrapping_add(
                    ((stpcpy(dest, (*entries.offset(i as isize)).name)).offset_from(dest)
                        as i64 + 1 as i32 as i64) as u64,
                ) as size_t as size_t;
            rpl_free((*entries.offset(i as isize)).name as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        rpl_free(entries as *mut libc::c_void);
    } else if used == allocated {
        name_space = xrealloc(
            name_space as *mut libc::c_void,
            used.wrapping_add(1 as i32 as u64),
        ) as *mut i8;
    }
    *name_space.offset(used as isize) = '\0' as i32 as i8;
    return name_space;
}
#[no_mangle]
pub unsafe extern "C" fn savedir(
    mut dir: *const i8,
    mut option: savedir_option,
) -> *mut i8 {
    let mut dirp: *mut DIR = opendir_safer(dir);
    if dirp.is_null() {
        return 0 as *mut i8
    } else {
        let mut name_space: *mut i8 = streamsavedir(dirp, option);
        if closedir(dirp) != 0 as i32 {
            let mut closedir_errno: i32 = *__errno_location();
            rpl_free(name_space as *mut libc::c_void);
            *__errno_location() = closedir_errno;
            return 0 as *mut i8;
        }
        return name_space;
    };
}