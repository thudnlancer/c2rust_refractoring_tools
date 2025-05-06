#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_printf(fmt: *const i8, _: ...);
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DMP {
    pub avail: [*mut libc::c_void; 32],
    pub block: *mut libc::c_void,
    pub used: i32,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prefix {
    pub pool: *mut DMP,
    pub size: i32,
}
#[no_mangle]
pub static mut _glp_dmp_debug: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn _glp_dmp_create_pool() -> *mut DMP {
    let mut pool: *mut DMP = 0 as *mut DMP;
    let mut k: i32 = 0;
    (::core::mem::size_of::<*mut libc::c_void>() as u64 <= 8 as i32 as u64
        || {
            glp_assert_(
                b"sizeof(void *) <= 8\0" as *const u8 as *const i8,
                b"misc/dmp.c\0" as *const u8 as *const i8,
                79 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if _glp_dmp_debug != 0 {
        glp_printf(
            b"dmp_create_pool: warning: debug mode is on\n\0" as *const u8 as *const i8,
        );
    }
    pool = glp_alloc(1 as i32, ::core::mem::size_of::<DMP>() as u64 as i32) as *mut DMP;
    k = 0 as i32;
    while k <= 31 as i32 {
        (*pool).avail[k as usize] = 0 as *mut libc::c_void;
        k += 1;
        k;
    }
    (*pool).block = 0 as *mut libc::c_void;
    (*pool).used = 8000 as i32;
    (*pool).count = 0 as i32 as size_t;
    return pool;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dmp_get_atom(
    mut pool: *mut DMP,
    mut size: i32,
) -> *mut libc::c_void {
    let mut atom: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut k: i32 = 0;
    let mut need: i32 = 0;
    (1 as i32 <= size && size <= 256 as i32
        || {
            glp_assert_(
                b"1 <= size && size <= 256\0" as *const u8 as *const i8,
                b"misc/dmp.c\0" as *const u8 as *const i8,
                117 as i32,
            );
            1 as i32 != 0
        }) as i32;
    need = size + 7 as i32 & !(7 as i32);
    k = (need >> 3 as i32) - 1 as i32;
    if ((*pool).avail[k as usize]).is_null() {
        if _glp_dmp_debug != 0 {
            need = (need as u64)
                .wrapping_add(
                    (::core::mem::size_of::<prefix>() as u64)
                        .wrapping_add(7 as i32 as u64) & !(7 as i32) as u64,
                ) as i32 as i32;
        }
        if (*pool).used + need > 8000 as i32 {
            let mut block: *mut libc::c_void = glp_alloc(
                8000 as i32,
                ::core::mem::size_of::<i8>() as u64 as i32,
            ) as *mut i8 as *mut libc::c_void;
            let ref mut fresh0 = *(block as *mut *mut libc::c_void);
            *fresh0 = (*pool).block;
            (*pool).block = block;
            (*pool).used = 8 as i32;
        }
        atom = ((*pool).block as *mut i8).offset((*pool).used as isize)
            as *mut libc::c_void;
        (*pool).used += need;
    } else {
        atom = (*pool).avail[k as usize];
        (*pool).avail[k as usize] = *(atom as *mut *mut libc::c_void);
    }
    if _glp_dmp_debug != 0 {
        let ref mut fresh1 = (*(atom as *mut prefix)).pool;
        *fresh1 = pool;
        (*(atom as *mut prefix)).size = size;
        atom = (atom as *mut i8)
            .offset(
                ((::core::mem::size_of::<prefix>() as u64).wrapping_add(7 as i32 as u64)
                    & !(7 as i32) as u64) as isize,
            ) as *mut libc::c_void;
    }
    (*pool).count = ((*pool).count).wrapping_add(1);
    (*pool).count;
    return atom;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dmp_free_atom(
    mut pool: *mut DMP,
    mut atom: *mut libc::c_void,
    mut size: i32,
) {
    let mut k: i32 = 0;
    (1 as i32 <= size && size <= 256 as i32
        || {
            glp_assert_(
                b"1 <= size && size <= 256\0" as *const u8 as *const i8,
                b"misc/dmp.c\0" as *const u8 as *const i8,
                178 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = (size + 7 as i32 >> 3 as i32) - 1 as i32;
    if _glp_dmp_debug != 0 {
        atom = (atom as *mut i8)
            .offset(
                -(((::core::mem::size_of::<prefix>() as u64)
                    .wrapping_add(7 as i32 as u64) & !(7 as i32) as u64) as isize),
            ) as *mut libc::c_void;
        ((*(atom as *mut prefix)).pool == pool
            || {
                glp_assert_(
                    b"((struct prefix *)atom)->pool == pool\0" as *const u8 as *const i8,
                    b"misc/dmp.c\0" as *const u8 as *const i8,
                    184 as i32,
                );
                1 as i32 != 0
            }) as i32;
        ((*(atom as *mut prefix)).size == size
            || {
                glp_assert_(
                    b"((struct prefix *)atom)->size == size\0" as *const u8 as *const i8,
                    b"misc/dmp.c\0" as *const u8 as *const i8,
                    185 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    let ref mut fresh2 = *(atom as *mut *mut libc::c_void);
    *fresh2 = (*pool).avail[k as usize];
    (*pool).avail[k as usize] = atom;
    ((*pool).count > 0 as i32 as u64
        || {
            glp_assert_(
                b"pool->count > 0\0" as *const u8 as *const i8,
                b"misc/dmp.c\0" as *const u8 as *const i8,
                191 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*pool).count = ((*pool).count).wrapping_sub(1);
    (*pool).count;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dmp_in_use(mut pool: *mut DMP) -> size_t {
    return (*pool).count;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dmp_delete_pool(mut pool: *mut DMP) {
    while !((*pool).block).is_null() {
        let mut block: *mut libc::c_void = (*pool).block;
        (*pool).block = *(block as *mut *mut libc::c_void);
        glp_free(block);
    }
    glp_free(pool as *mut libc::c_void);
}