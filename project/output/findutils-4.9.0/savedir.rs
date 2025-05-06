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
extern "C" {
    pub type __dirstream;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn closedir(__dirp: *mut DIR) -> i32;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn rpl_free(ptr: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn opendir_safer(name: *const i8) -> *mut DIR;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn stpcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn xirealloc(p: *mut libc::c_void, s: idx_t) -> *mut libc::c_void;
    fn ximalloc(s: idx_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type ptrdiff_t = i64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum savedir_option {
    SAVEDIR_SORT_NONE,
    SAVEDIR_SORT_NAME,
    SAVEDIR_SORT_INODE,
    SAVEDIR_SORT_FASTREAD,
}
impl savedir_option {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            savedir_option::SAVEDIR_SORT_NONE => 0,
            savedir_option::SAVEDIR_SORT_NAME => 1,
            savedir_option::SAVEDIR_SORT_INODE => 2,
            savedir_option::SAVEDIR_SORT_FASTREAD => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> savedir_option {
        match value {
            0 => savedir_option::SAVEDIR_SORT_NONE,
            1 => savedir_option::SAVEDIR_SORT_NAME,
            2 => savedir_option::SAVEDIR_SORT_INODE,
            2 => savedir_option::SAVEDIR_SORT_FASTREAD,
        }
    }
}
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub name: *mut i8,
    pub ino: ino_t,
}
pub type comparison_function = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
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
    let mut allocated: idx_t = 0 as i32 as idx_t;
    let mut entries: *mut direntry_t = 0 as *mut direntry_t;
    let mut entries_allocated: idx_t = 0 as i32 as idx_t;
    let mut entries_used: idx_t = 0 as i32 as idx_t;
    let mut used: idx_t = 0 as i32 as idx_t;
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
            let mut entry_size: idx_t = (strlen(((*dp).d_name).as_ptr()))
                .wrapping_add(1 as i32 as u64) as idx_t;
            if cmp.is_some() {
                if entries_allocated == entries_used {
                    entries = xpalloc(
                        entries as *mut libc::c_void,
                        &mut entries_allocated,
                        1 as i32 as idx_t,
                        -(1 as i32) as ptrdiff_t,
                        ::core::mem::size_of::<direntry_t>() as u64 as idx_t,
                    ) as *mut direntry_t;
                }
                let ref mut fresh0 = (*entries.offset(entries_used as isize)).name;
                *fresh0 = xstrdup(entry);
                (*entries.offset(entries_used as isize)).ino = (*dp).d_ino;
                entries_used += 1;
                entries_used;
            } else {
                if allocated - used <= entry_size {
                    name_space = xpalloc(
                        name_space as *mut libc::c_void,
                        &mut allocated,
                        entry_size - (allocated - used),
                        9223372036854775807 as i64 - 1 as i32 as i64,
                        ::core::mem::size_of::<i8>() as u64 as idx_t,
                    ) as *mut i8;
                }
                memcpy(
                    name_space.offset(used as isize) as *mut libc::c_void,
                    entry as *const libc::c_void,
                    entry_size as u64,
                );
            }
            used += entry_size;
        }
    }
    if *__errno_location() != 0 as i32 {
        rpl_free(entries as *mut libc::c_void);
        rpl_free(name_space as *mut libc::c_void);
        return 0 as *mut i8;
    }
    if cmp.is_some() {
        if entries_used != 0 {
            qsort(
                entries as *mut libc::c_void,
                entries_used as size_t,
                ::core::mem::size_of::<direntry_t>() as u64,
                cmp,
            );
        }
        name_space = ximalloc(used + 1 as i32 as i64) as *mut i8;
        used = 0 as i32 as idx_t;
        let mut i: idx_t = 0 as i32 as idx_t;
        while i < entries_used {
            let mut dest: *mut i8 = name_space.offset(used as isize);
            used
                += (stpcpy(dest, (*entries.offset(i as isize)).name)).offset_from(dest)
                    as i64 + 1 as i32 as i64;
            rpl_free((*entries.offset(i as isize)).name as *mut libc::c_void);
            i += 1;
            i;
        }
        rpl_free(entries as *mut libc::c_void);
    } else if used == allocated {
        name_space = xirealloc(name_space as *mut libc::c_void, used + 1 as i32 as i64)
            as *mut i8;
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
            rpl_free(name_space as *mut libc::c_void);
            return 0 as *mut i8;
        }
        return name_space;
    };
}