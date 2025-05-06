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
    fn glp_realloc(ptr: *mut libc::c_void, n: i32, size: i32) -> *mut libc::c_void;
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_printf(fmt: *const i8, _: ...);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SVA {
    pub n_max: i32,
    pub n: i32,
    pub ptr: *mut i32,
    pub len: *mut i32,
    pub cap: *mut i32,
    pub size: i32,
    pub m_ptr: i32,
    pub r_ptr: i32,
    pub head: i32,
    pub tail: i32,
    pub prev: *mut i32,
    pub next: *mut i32,
    pub ind: *mut i32,
    pub val: *mut libc::c_double,
    pub talky: i32,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sva_create_area(
    mut n_max: i32,
    mut size: i32,
) -> *mut SVA {
    let mut sva: *mut SVA = 0 as *mut SVA;
    ((0 as i32) < n_max && n_max < 2147483647 as i32
        || {
            glp_assert_(
                b"0 < n_max && n_max < INT_MAX\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                41 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((0 as i32) < size && size < 2147483647 as i32
        || {
            glp_assert_(
                b"0 < size && size < INT_MAX\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                42 as i32,
            );
            1 as i32 != 0
        }) as i32;
    sva = glp_alloc(1 as i32, ::core::mem::size_of::<SVA>() as u64 as i32) as *mut SVA;
    (*sva).n_max = n_max;
    (*sva).n = 0 as i32;
    (*sva).ptr = glp_alloc(1 as i32 + n_max, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*sva).len = glp_alloc(1 as i32 + n_max, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*sva).cap = glp_alloc(1 as i32 + n_max, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*sva).size = size;
    (*sva).m_ptr = 1 as i32;
    (*sva).r_ptr = size + 1 as i32;
    (*sva).tail = 0 as i32;
    (*sva).head = (*sva).tail;
    (*sva).prev = glp_alloc(
        1 as i32 + n_max,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*sva).next = glp_alloc(
        1 as i32 + n_max,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*sva).ind = glp_alloc(1 as i32 + size, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*sva).val = glp_alloc(
        1 as i32 + size,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    (*sva).talky = 0 as i32;
    return sva;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sva_alloc_vecs(mut sva: *mut SVA, mut nnn: i32) -> i32 {
    let mut n: i32 = (*sva).n;
    let mut n_max: i32 = (*sva).n_max;
    let mut ptr: *mut i32 = (*sva).ptr;
    let mut len: *mut i32 = (*sva).len;
    let mut cap: *mut i32 = (*sva).cap;
    let mut prev: *mut i32 = (*sva).prev;
    let mut next: *mut i32 = (*sva).next;
    let mut k: i32 = 0;
    let mut new_n: i32 = 0;
    if (*sva).talky != 0 {
        glp_printf(b"sva_alloc_vecs: nnn = %d\n\0" as *const u8 as *const i8, nnn);
    }
    (nnn > 0 as i32
        || {
            glp_assert_(
                b"nnn > 0\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                84 as i32,
            );
            1 as i32 != 0
        }) as i32;
    new_n = n + nnn;
    (new_n > n
        || {
            glp_assert_(
                b"new_n > n\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                87 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if n_max < new_n {
        while n_max < new_n {
            n_max += n_max;
            (n_max > 0 as i32
                || {
                    glp_assert_(
                        b"n_max > 0\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        92 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        (*sva).n_max = n_max;
        ptr = glp_realloc(
            ptr as *mut libc::c_void,
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*sva).ptr = ptr;
        len = glp_realloc(
            len as *mut libc::c_void,
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*sva).len = len;
        cap = glp_realloc(
            cap as *mut libc::c_void,
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*sva).cap = cap;
        prev = glp_realloc(
            prev as *mut libc::c_void,
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*sva).prev = prev;
        next = glp_realloc(
            next as *mut libc::c_void,
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*sva).next = next;
    }
    (*sva).n = new_n;
    k = n + 1 as i32;
    while k <= new_n {
        let ref mut fresh0 = *cap.offset(k as isize);
        *fresh0 = 0 as i32;
        let ref mut fresh1 = *len.offset(k as isize);
        *fresh1 = *fresh0;
        *ptr.offset(k as isize) = *fresh1;
        let ref mut fresh2 = *next.offset(k as isize);
        *fresh2 = -(1 as i32);
        *prev.offset(k as isize) = *fresh2;
        k += 1;
        k;
    }
    if (*sva).talky != 0 {
        glp_printf(
            b"now sva->n_max = %d, sva->n = %d\n\0" as *const u8 as *const i8,
            (*sva).n_max,
            (*sva).n,
        );
    }
    return n + 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sva_resize_area(mut sva: *mut SVA, mut delta: i32) {
    let mut n: i32 = (*sva).n;
    let mut ptr: *mut i32 = (*sva).ptr;
    let mut size: i32 = (*sva).size;
    let mut m_ptr: i32 = (*sva).m_ptr;
    let mut r_ptr: i32 = (*sva).r_ptr;
    let mut k: i32 = 0;
    let mut r_size: i32 = 0;
    if (*sva).talky != 0 {
        glp_printf(b"sva_resize_area: delta = %d\n\0" as *const u8 as *const i8, delta);
    }
    (delta != 0 as i32
        || {
            glp_assert_(
                b"delta != 0\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                144 as i32,
            );
            1 as i32 != 0
        }) as i32;
    r_size = size - r_ptr + 1 as i32;
    if delta < 0 as i32 {
        (delta >= m_ptr - r_ptr
            || {
                glp_assert_(
                    b"delta >= m_ptr - r_ptr\0" as *const u8 as *const i8,
                    b"bflib/sva.c\0" as *const u8 as *const i8,
                    149 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*sva).r_ptr += delta;
        memmove(
            &mut *((*sva).ind).offset((*sva).r_ptr as isize) as *mut i32
                as *mut libc::c_void,
            &mut *((*sva).ind).offset(r_ptr as isize) as *mut i32 as *const libc::c_void,
            (r_size as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
        );
        memmove(
            &mut *((*sva).val).offset((*sva).r_ptr as isize) as *mut libc::c_double
                as *mut libc::c_void,
            &mut *((*sva).val).offset(r_ptr as isize) as *mut libc::c_double
                as *const libc::c_void,
            (r_size as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
        );
    }
    (delta < 2147483647 as i32 - (*sva).size
        || {
            glp_assert_(
                b"delta < INT_MAX - sva->size\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                157 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*sva).size += delta;
    (*sva).ind = glp_realloc(
        (*sva).ind as *mut libc::c_void,
        1 as i32 + (*sva).size,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*sva).val = glp_realloc(
        (*sva).val as *mut libc::c_void,
        1 as i32 + (*sva).size,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    if delta > 0 as i32 {
        (*sva).r_ptr += delta;
        memmove(
            &mut *((*sva).ind).offset((*sva).r_ptr as isize) as *mut i32
                as *mut libc::c_void,
            &mut *((*sva).ind).offset(r_ptr as isize) as *mut i32 as *const libc::c_void,
            (r_size as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
        );
        memmove(
            &mut *((*sva).val).offset((*sva).r_ptr as isize) as *mut libc::c_double
                as *mut libc::c_void,
            &mut *((*sva).val).offset(r_ptr as isize) as *mut libc::c_double
                as *const libc::c_void,
            (r_size as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
        );
    }
    k = 1 as i32;
    while k <= n {
        if *ptr.offset(k as isize) >= r_ptr {
            *ptr.offset(k as isize) += delta;
        }
        k += 1;
        k;
    }
    if (*sva).talky != 0 {
        glp_printf(b"now sva->size = %d\n\0" as *const u8 as *const i8, (*sva).size);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sva_defrag_area(mut sva: *mut SVA) {
    let mut ptr: *mut i32 = (*sva).ptr;
    let mut len: *mut i32 = (*sva).len;
    let mut cap: *mut i32 = (*sva).cap;
    let mut prev: *mut i32 = (*sva).prev;
    let mut next: *mut i32 = (*sva).next;
    let mut ind: *mut i32 = (*sva).ind;
    let mut val: *mut libc::c_double = (*sva).val;
    let mut k: i32 = 0;
    let mut next_k: i32 = 0;
    let mut ptr_k: i32 = 0;
    let mut len_k: i32 = 0;
    let mut m_ptr: i32 = 0;
    let mut head: i32 = 0;
    let mut tail: i32 = 0;
    if (*sva).talky != 0 {
        glp_printf(b"sva_defrag_area:\n\0" as *const u8 as *const i8);
        glp_printf(
            b"before defragmenting = %d %d %d\n\0" as *const u8 as *const i8,
            (*sva).m_ptr - 1 as i32,
            (*sva).r_ptr - (*sva).m_ptr,
            (*sva).size + 1 as i32 - (*sva).r_ptr,
        );
    }
    m_ptr = 1 as i32;
    tail = 0 as i32;
    head = tail;
    k = (*sva).head;
    while k != 0 as i32 {
        next_k = *next.offset(k as isize);
        len_k = *len.offset(k as isize);
        if len_k == 0 as i32 {
            let ref mut fresh3 = *cap.offset(k as isize);
            *fresh3 = 0 as i32;
            *ptr.offset(k as isize) = *fresh3;
            let ref mut fresh4 = *next.offset(k as isize);
            *fresh4 = -(1 as i32);
            *prev.offset(k as isize) = *fresh4;
        } else {
            ptr_k = *ptr.offset(k as isize);
            (m_ptr <= ptr_k
                || {
                    glp_assert_(
                        b"m_ptr <= ptr_k\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        223 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if m_ptr < ptr_k {
                memmove(
                    &mut *ind.offset(m_ptr as isize) as *mut i32 as *mut libc::c_void,
                    &mut *ind.offset(ptr_k as isize) as *mut i32 as *const libc::c_void,
                    (len_k as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
                );
                memmove(
                    &mut *val.offset(m_ptr as isize) as *mut libc::c_double
                        as *mut libc::c_void,
                    &mut *val.offset(ptr_k as isize) as *mut libc::c_double
                        as *const libc::c_void,
                    (len_k as u64)
                        .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
                );
                *ptr.offset(k as isize) = m_ptr;
            }
            *cap.offset(k as isize) = len_k;
            m_ptr += len_k;
            *prev.offset(k as isize) = tail;
            *next.offset(k as isize) = 0 as i32;
            if head == 0 as i32 {
                head = k;
            } else {
                *next.offset(tail as isize) = k;
            }
            tail = k;
        }
        k = next_k;
    }
    (m_ptr <= (*sva).r_ptr
        || {
            glp_assert_(
                b"m_ptr <= sva->r_ptr\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                248 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*sva).m_ptr = m_ptr;
    (*sva).head = head;
    (*sva).tail = tail;
    if (*sva).talky != 0 {
        glp_printf(
            b"after defragmenting = %d %d %d\n\0" as *const u8 as *const i8,
            (*sva).m_ptr - 1 as i32,
            (*sva).r_ptr - (*sva).m_ptr,
            (*sva).size + 1 as i32 - (*sva).r_ptr,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sva_more_space(mut sva: *mut SVA, mut m_size: i32) {
    let mut size: i32 = 0;
    let mut delta: i32 = 0;
    if (*sva).talky != 0 {
        glp_printf(b"sva_more_space: m_size = %d\n\0" as *const u8 as *const i8, m_size);
    }
    (m_size > (*sva).r_ptr - (*sva).m_ptr
        || {
            glp_assert_(
                b"m_size > sva->r_ptr - sva->m_ptr\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                281 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_sva_defrag_area(sva);
    if m_size < (*sva).m_ptr - 1 as i32 {
        m_size = (*sva).m_ptr - 1 as i32;
    }
    if (*sva).r_ptr - (*sva).m_ptr < m_size {
        size = (*sva).size;
        loop {
            delta = size - (*sva).size;
            if (*sva).r_ptr - (*sva).m_ptr + delta >= m_size {
                break;
            }
            size += size;
            (size > 0 as i32
                || {
                    glp_assert_(
                        b"size > 0\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        297 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        _glp_sva_resize_area(sva, delta);
        ((*sva).r_ptr - (*sva).m_ptr >= m_size
            || {
                glp_assert_(
                    b"sva->r_ptr - sva->m_ptr >= m_size\0" as *const u8 as *const i8,
                    b"bflib/sva.c\0" as *const u8 as *const i8,
                    300 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sva_enlarge_cap(
    mut sva: *mut SVA,
    mut k: i32,
    mut new_cap: i32,
    mut skip: i32,
) {
    let mut ptr: *mut i32 = (*sva).ptr;
    let mut len: *mut i32 = (*sva).len;
    let mut cap: *mut i32 = (*sva).cap;
    let mut prev: *mut i32 = (*sva).prev;
    let mut next: *mut i32 = (*sva).next;
    let mut ind: *mut i32 = (*sva).ind;
    let mut val: *mut libc::c_double = (*sva).val;
    (1 as i32 <= k && k <= (*sva).n
        || {
            glp_assert_(
                b"1 <= k && k <= sva->n\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                334 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (new_cap > *cap.offset(k as isize)
        || {
            glp_assert_(
                b"new_cap > cap[k]\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                335 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*sva).r_ptr - (*sva).m_ptr >= new_cap
        || {
            glp_assert_(
                b"sva->r_ptr - sva->m_ptr >= new_cap\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                337 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if *cap.offset(k as isize) == 0 as i32 {
        (*ptr.offset(k as isize) == 0 as i32
            || {
                glp_assert_(
                    b"ptr[k] == 0\0" as *const u8 as *const i8,
                    b"bflib/sva.c\0" as *const u8 as *const i8,
                    341 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*len.offset(k as isize) == 0 as i32
            || {
                glp_assert_(
                    b"len[k] == 0\0" as *const u8 as *const i8,
                    b"bflib/sva.c\0" as *const u8 as *const i8,
                    342 as i32,
                );
                1 as i32 != 0
            }) as i32;
    } else {
        (*ptr.offset(k as isize) + *len.offset(k as isize) <= (*sva).m_ptr
            || {
                glp_assert_(
                    b"ptr[k] + len[k] <= sva->m_ptr\0" as *const u8 as *const i8,
                    b"bflib/sva.c\0" as *const u8 as *const i8,
                    346 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *len.offset(k as isize) > 0 as i32 {
            memcpy(
                &mut *ind.offset((*sva).m_ptr as isize) as *mut i32 as *mut libc::c_void,
                &mut *ind.offset(*ptr.offset(k as isize) as isize) as *mut i32
                    as *const libc::c_void,
                (*len.offset(k as isize) as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64),
            );
            if skip == 0 {
                memcpy(
                    &mut *val.offset((*sva).m_ptr as isize) as *mut libc::c_double
                        as *mut libc::c_void,
                    &mut *val.offset(*ptr.offset(k as isize) as isize)
                        as *mut libc::c_double as *const libc::c_void,
                    (*len.offset(k as isize) as u64)
                        .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
                );
            }
        }
        if *prev.offset(k as isize) == 0 as i32 {
            (*sva).head = *next.offset(k as isize);
        } else {
            *cap.offset(*prev.offset(k as isize) as isize) += *cap.offset(k as isize);
            *next.offset(*prev.offset(k as isize) as isize) = *next.offset(k as isize);
        }
        if *next.offset(k as isize) == 0 as i32 {
            (*sva).tail = *prev.offset(k as isize);
        } else {
            *prev.offset(*next.offset(k as isize) as isize) = *prev.offset(k as isize);
        }
    }
    *ptr.offset(k as isize) = (*sva).m_ptr;
    *cap.offset(k as isize) = new_cap;
    *prev.offset(k as isize) = (*sva).tail;
    *next.offset(k as isize) = 0 as i32;
    if (*sva).head == 0 as i32 {
        (*sva).head = k;
    } else {
        *next.offset((*sva).tail as isize) = k;
    }
    (*sva).tail = k;
    (*sva).m_ptr += new_cap;
    ((*sva).m_ptr <= (*sva).r_ptr
        || {
            glp_assert_(
                b"sva->m_ptr <= sva->r_ptr\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                382 as i32,
            );
            1 as i32 != 0
        }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sva_reserve_cap(
    mut sva: *mut SVA,
    mut k: i32,
    mut new_cap: i32,
) {
    let mut ptr: *mut i32 = (*sva).ptr;
    let mut len: *mut i32 = (*sva).len;
    let mut cap: *mut i32 = (*sva).cap;
    (1 as i32 <= k && k <= (*sva).n
        || {
            glp_assert_(
                b"1 <= k && k <= sva->n\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                405 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (new_cap > 0 as i32
        || {
            glp_assert_(
                b"new_cap > 0\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                406 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*ptr.offset(k as isize) == 0 as i32 && *len.offset(k as isize) == 0 as i32
        && *cap.offset(k as isize) == 0 as i32
        || {
            glp_assert_(
                b"ptr[k] == 0 && len[k] == 0 && cap[k] == 0\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                407 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*sva).r_ptr - (*sva).m_ptr >= new_cap
        || {
            glp_assert_(
                b"sva->r_ptr - sva->m_ptr >= new_cap\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                409 as i32,
            );
            1 as i32 != 0
        }) as i32;
    *ptr.offset(k as isize) = (*sva).r_ptr - new_cap;
    *cap.offset(k as isize) = new_cap;
    (*sva).r_ptr -= new_cap;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sva_make_static(mut sva: *mut SVA, mut k: i32) {
    let mut ptr: *mut i32 = (*sva).ptr;
    let mut len: *mut i32 = (*sva).len;
    let mut cap: *mut i32 = (*sva).cap;
    let mut prev: *mut i32 = (*sva).prev;
    let mut next: *mut i32 = (*sva).next;
    let mut ind: *mut i32 = (*sva).ind;
    let mut val: *mut libc::c_double = (*sva).val;
    let mut ptr_k: i32 = 0;
    let mut len_k: i32 = 0;
    (1 as i32 <= k && k <= (*sva).n
        || {
            glp_assert_(
                b"1 <= k && k <= sva->n\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                442 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if *cap.offset(k as isize) == 0 as i32 {
        (*ptr.offset(k as isize) == 0 as i32
            || {
                glp_assert_(
                    b"ptr[k] == 0\0" as *const u8 as *const i8,
                    b"bflib/sva.c\0" as *const u8 as *const i8,
                    445 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*len.offset(k as isize) == 0 as i32
            || {
                glp_assert_(
                    b"len[k] == 0\0" as *const u8 as *const i8,
                    b"bflib/sva.c\0" as *const u8 as *const i8,
                    446 as i32,
                );
                1 as i32 != 0
            }) as i32;
    } else {
        len_k = *len.offset(k as isize);
        ((*sva).r_ptr - (*sva).m_ptr >= len_k
            || {
                glp_assert_(
                    b"sva->r_ptr - sva->m_ptr >= len_k\0" as *const u8 as *const i8,
                    b"bflib/sva.c\0" as *const u8 as *const i8,
                    451 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *prev.offset(k as isize) == 0 as i32 {
            (*sva).head = *next.offset(k as isize);
        } else {
            *cap.offset(*prev.offset(k as isize) as isize) += *cap.offset(k as isize);
            *next.offset(*prev.offset(k as isize) as isize) = *next.offset(k as isize);
        }
        if *next.offset(k as isize) == 0 as i32 {
            (*sva).tail = *prev.offset(k as isize);
        } else {
            *prev.offset(*next.offset(k as isize) as isize) = *prev.offset(k as isize);
        }
        if len_k == 0 as i32 {
            let ref mut fresh5 = *cap.offset(k as isize);
            *fresh5 = 0 as i32;
            *ptr.offset(k as isize) = *fresh5;
        } else {
            ptr_k = (*sva).r_ptr - len_k;
            memcpy(
                &mut *ind.offset(ptr_k as isize) as *mut i32 as *mut libc::c_void,
                &mut *ind.offset(*ptr.offset(k as isize) as isize) as *mut i32
                    as *const libc::c_void,
                (len_k as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
            );
            memcpy(
                &mut *val.offset(ptr_k as isize) as *mut libc::c_double
                    as *mut libc::c_void,
                &mut *val.offset(*ptr.offset(k as isize) as isize) as *mut libc::c_double
                    as *const libc::c_void,
                (len_k as u64)
                    .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
            );
            *ptr.offset(k as isize) = ptr_k;
            *cap.offset(k as isize) = len_k;
            (*sva).r_ptr -= len_k;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sva_check_area(mut sva: *mut SVA) {
    let mut n_max: i32 = (*sva).n_max;
    let mut n: i32 = (*sva).n;
    let mut ptr: *mut i32 = (*sva).ptr;
    let mut len: *mut i32 = (*sva).len;
    let mut cap: *mut i32 = (*sva).cap;
    let mut size: i32 = (*sva).size;
    let mut m_ptr: i32 = (*sva).m_ptr;
    let mut r_ptr: i32 = (*sva).r_ptr;
    let mut head: i32 = (*sva).head;
    let mut tail: i32 = (*sva).tail;
    let mut prev: *mut i32 = (*sva).prev;
    let mut next: *mut i32 = (*sva).next;
    let mut k: i32 = 0;
    (0 as i32 <= n && n <= n_max
        || {
            glp_assert_(
                b"0 <= n && n <= n_max\0" as *const u8 as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                505 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= m_ptr && m_ptr <= r_ptr && r_ptr <= size + 1 as i32
        || {
            glp_assert_(
                b"1 <= m_ptr && m_ptr <= r_ptr && r_ptr <= size+1\0" as *const u8
                    as *const i8,
                b"bflib/sva.c\0" as *const u8 as *const i8,
                507 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = head;
    while k != 0 as i32 {
        (1 as i32 <= k && k <= n
            || {
                glp_assert_(
                    b"1 <= k && k <= n\0" as *const u8 as *const i8,
                    b"bflib/sva.c\0" as *const u8 as *const i8,
                    511 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*cap.offset(k as isize) > 0 as i32
            || {
                glp_assert_(
                    b"cap[k] > 0\0" as *const u8 as *const i8,
                    b"bflib/sva.c\0" as *const u8 as *const i8,
                    512 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (0 as i32 <= *len.offset(k as isize)
            && *len.offset(k as isize) <= *cap.offset(k as isize)
            || {
                glp_assert_(
                    b"0 <= len[k] && len[k] <= cap[k]\0" as *const u8 as *const i8,
                    b"bflib/sva.c\0" as *const u8 as *const i8,
                    513 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *prev.offset(k as isize) == 0 as i32 {
            (k == head
                || {
                    glp_assert_(
                        b"k == head\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        515 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        } else {
            (1 as i32 <= *prev.offset(k as isize) && *prev.offset(k as isize) <= n
                || {
                    glp_assert_(
                        b"1 <= prev[k] && prev[k] <= n\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        517 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*next.offset(*prev.offset(k as isize) as isize) == k
                || {
                    glp_assert_(
                        b"next[prev[k]] == k\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        518 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        if *next.offset(k as isize) == 0 as i32 {
            (k == tail
                || {
                    glp_assert_(
                        b"k == tail\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        521 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*ptr.offset(k as isize) + *cap.offset(k as isize) <= m_ptr
                || {
                    glp_assert_(
                        b"ptr[k] + cap[k] <= m_ptr\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        522 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        } else {
            (1 as i32 <= *next.offset(k as isize) && *next.offset(k as isize) <= n
                || {
                    glp_assert_(
                        b"1 <= next[k] && next[k] <= n\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        525 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*prev.offset(*next.offset(k as isize) as isize) == k
                || {
                    glp_assert_(
                        b"prev[next[k]] == k\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        526 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*ptr.offset(k as isize) + *cap.offset(k as isize)
                <= *ptr.offset(*next.offset(k as isize) as isize)
                || {
                    glp_assert_(
                        b"ptr[k] + cap[k] <= ptr[next[k]]\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        527 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        *cap.offset(k as isize) = -*cap.offset(k as isize);
        k = *next.offset(k as isize);
    }
    k = 1 as i32;
    while k <= n {
        if *cap.offset(k as isize) < 0 as i32 {
            *cap.offset(k as isize) = -*cap.offset(k as isize);
        } else if *cap.offset(k as isize) == 0 as i32 {
            (*ptr.offset(k as isize) == 0 as i32
                || {
                    glp_assert_(
                        b"ptr[k] == 0\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        540 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*len.offset(k as isize) == 0 as i32
                || {
                    glp_assert_(
                        b"len[k] == 0\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        541 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        } else {
            (0 as i32 <= *len.offset(k as isize)
                && *len.offset(k as isize) <= *cap.offset(k as isize)
                || {
                    glp_assert_(
                        b"0 <= len[k] && len[k] <= cap[k]\0" as *const u8 as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        545 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (r_ptr <= *ptr.offset(k as isize)
                && *ptr.offset(k as isize) + *cap.offset(k as isize) <= size + 1 as i32
                || {
                    glp_assert_(
                        b"r_ptr <= ptr[k] && ptr[k] + cap[k] <= size+1\0" as *const u8
                            as *const i8,
                        b"bflib/sva.c\0" as *const u8 as *const i8,
                        546 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_sva_delete_area(mut sva: *mut SVA) {
    glp_free((*sva).ptr as *mut libc::c_void);
    glp_free((*sva).len as *mut libc::c_void);
    glp_free((*sva).cap as *mut libc::c_void);
    glp_free((*sva).prev as *mut libc::c_void);
    glp_free((*sva).next as *mut libc::c_void);
    glp_free((*sva).ind as *mut libc::c_void);
    glp_free((*sva).val as *mut libc::c_void);
    glp_free(sva as *mut libc::c_void);
}