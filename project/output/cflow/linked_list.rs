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
    fn free(__ptr: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_list_entry {
    pub next: *mut linked_list_entry,
    pub prev: *mut linked_list_entry,
    pub list: *mut linked_list,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_list {
    pub free_data: linked_list_free_data_fp,
    pub head: *mut linked_list_entry,
    pub tail: *mut linked_list_entry,
}
pub type linked_list_free_data_fp = Option<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
unsafe extern "C" fn deref_linked_list(
    mut plist: *mut *mut linked_list,
) -> *mut linked_list {
    if (*plist).is_null() {
        let mut list: *mut linked_list = xmalloc(
            ::core::mem::size_of::<linked_list>() as u64,
        ) as *mut linked_list;
        (*list).free_data = None;
        (*list).tail = 0 as *mut linked_list_entry;
        (*list).head = (*list).tail;
        *plist = list;
    }
    return *plist;
}
#[no_mangle]
pub unsafe extern "C" fn linked_list_create(
    mut fun: linked_list_free_data_fp,
) -> *mut linked_list {
    let mut list: *mut linked_list = xmalloc(
        ::core::mem::size_of::<linked_list>() as u64,
    ) as *mut linked_list;
    (*list).free_data = fun;
    (*list).tail = 0 as *mut linked_list_entry;
    (*list).head = (*list).tail;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn linked_list_append(
    mut plist: *mut *mut linked_list,
    mut data: *mut libc::c_void,
) {
    let mut list: *mut linked_list = deref_linked_list(plist);
    let mut entry: *mut linked_list_entry = xmalloc(
        ::core::mem::size_of::<linked_list_entry>() as u64,
    ) as *mut linked_list_entry;
    (*entry).list = list;
    (*entry).data = data;
    (*entry).next = 0 as *mut linked_list_entry;
    (*entry).prev = (*list).tail;
    if !((*list).tail).is_null() {
        (*(*list).tail).next = entry;
    } else {
        (*list).head = entry;
    }
    (*list).tail = entry;
}
#[no_mangle]
pub unsafe extern "C" fn linked_list_destroy(mut plist: *mut *mut linked_list) {
    if !plist.is_null() && !(*plist).is_null() {
        let mut list: *mut linked_list = *plist;
        let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
        p = (*list).head;
        while !p.is_null() {
            let mut next: *mut linked_list_entry = (*p).next;
            if ((*list).free_data).is_some() {
                ((*list).free_data).expect("non-null function pointer")((*p).data);
            }
            free(p as *mut libc::c_void);
            p = next;
        }
        free(list as *mut libc::c_void);
        *plist = 0 as *mut linked_list;
    }
}
#[no_mangle]
pub unsafe extern "C" fn linked_list_unlink(
    mut list: *mut linked_list,
    mut ent: *mut linked_list_entry,
) {
    let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
    p = (*ent).prev;
    if !p.is_null() {
        (*p).next = (*ent).next;
    } else {
        (*list).head = (*ent).next;
    }
    p = (*ent).next;
    if !p.is_null() {
        (*p).prev = (*ent).prev;
    } else {
        (*list).tail = (*ent).prev;
    }
    if ((*list).free_data).is_some() {
        ((*list).free_data).expect("non-null function pointer")((*ent).data);
    }
    free(ent as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn linked_list_iterate(
    mut plist: *mut *mut linked_list,
    mut itr: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> i32>,
    mut data: *mut libc::c_void,
) {
    let mut list: *mut linked_list = 0 as *mut linked_list;
    let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
    if (*plist).is_null() {
        return;
    }
    list = *plist;
    p = if !list.is_null() { (*list).head } else { 0 as *mut linked_list_entry };
    while !p.is_null() {
        let mut next: *mut linked_list_entry = (*p).next;
        if itr.expect("non-null function pointer")((*p).data, data) != 0 {
            linked_list_unlink(list, p);
        }
        p = next;
    }
    if ((*list).head).is_null() {
        linked_list_destroy(&mut list);
    }
    *plist = list;
}
#[no_mangle]
pub unsafe extern "C" fn data_in_list(
    mut data: *mut libc::c_void,
    mut list: *mut linked_list,
) -> i32 {
    let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
    p = if !list.is_null() { (*list).head } else { 0 as *mut linked_list_entry };
    while !p.is_null() {
        if (*p).data == data {
            return 1 as i32;
        }
        p = (*p).next;
    }
    return 0 as i32;
}