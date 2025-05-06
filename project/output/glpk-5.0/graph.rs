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
    pub type AVL;
    pub type AVLNODE;
    pub type DMP;
    fn _glp_avl_create_tree(
        fcmp: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_void,
                *const libc::c_void,
            ) -> i32,
        >,
        info: *mut libc::c_void,
    ) -> *mut AVL;
    fn _glp_avl_strcmp(
        info: *mut libc::c_void,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> i32;
    fn _glp_avl_insert_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
    fn _glp_avl_set_node_link(node: *mut AVLNODE, link: *mut libc::c_void);
    fn _glp_avl_find_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
    fn _glp_avl_get_node_link(node: *mut AVLNODE) -> *mut libc::c_void;
    fn _glp_avl_delete_node(tree: *mut AVL, node: *mut AVLNODE);
    fn _glp_avl_delete_tree(tree: *mut AVL);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: i32);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn strlen(_: *const i8) -> u64;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
}
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_graph {
    pub pool: *mut libc::c_void,
    pub name: *mut i8,
    pub nv_max: i32,
    pub nv: i32,
    pub na: i32,
    pub v: *mut *mut glp_vertex,
    pub index: *mut libc::c_void,
    pub v_size: i32,
    pub a_size: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_vertex {
    pub i: i32,
    pub name: *mut i8,
    pub entry: *mut libc::c_void,
    pub data: *mut libc::c_void,
    pub temp: *mut libc::c_void,
    pub in_0: *mut glp_arc,
    pub out: *mut glp_arc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_arc {
    pub tail: *mut glp_vertex,
    pub head: *mut glp_vertex,
    pub data: *mut libc::c_void,
    pub temp: *mut libc::c_void,
    pub t_prev: *mut glp_arc,
    pub t_next: *mut glp_arc,
    pub h_prev: *mut glp_arc,
    pub h_next: *mut glp_arc,
}
unsafe extern "C" fn create_graph(
    mut G: *mut glp_graph,
    mut v_size: i32,
    mut a_size: i32,
) {
    (*G).pool = _glp_dmp_create_pool() as *mut libc::c_void;
    (*G).name = 0 as *mut i8;
    (*G).nv_max = 50 as i32;
    (*G).na = 0 as i32;
    (*G).nv = (*G).na;
    (*G).v = glp_alloc(
        1 as i32 + (*G).nv_max,
        ::core::mem::size_of::<*mut glp_vertex>() as u64 as i32,
    ) as *mut *mut glp_vertex;
    (*G).index = 0 as *mut libc::c_void;
    (*G).v_size = v_size;
    (*G).a_size = a_size;
}
#[no_mangle]
pub unsafe extern "C" fn glp_create_graph(
    mut v_size: i32,
    mut a_size: i32,
) -> *mut glp_graph {
    let mut G: *mut glp_graph = 0 as *mut glp_graph;
    if !(0 as i32 <= v_size && v_size <= 256 as i32) {
        (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 74 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_create_graph: v_size = %d; invalid size of vertex data\n\0"
                as *const u8 as *const i8,
            v_size,
        );
    }
    if !(0 as i32 <= a_size && a_size <= 256 as i32) {
        (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 77 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_create_graph: a_size = %d; invalid size of arc data\n\0" as *const u8
                as *const i8,
            a_size,
        );
    }
    G = glp_alloc(1 as i32, ::core::mem::size_of::<glp_graph>() as u64 as i32)
        as *mut glp_graph;
    create_graph(G, v_size, a_size);
    return G;
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_graph_name(mut G: *mut glp_graph, mut name: *const i8) {
    if !((*G).name).is_null() {
        _glp_dmp_free_atom(
            (*G).pool as *mut DMP,
            (*G).name as *mut libc::c_void,
            (strlen((*G).name)).wrapping_add(1 as i32 as u64) as i32,
        );
        (*G).name = 0 as *mut i8;
    }
    if !(name.is_null() || *name.offset(0 as i32 as isize) as i32 == '\0' as i32) {
        let mut j: i32 = 0;
        j = 0 as i32;
        while *name.offset(j as isize) as i32 != '\0' as i32 {
            if j == 256 as i32 {
                (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 110 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_set_graph_name: graph name too long\n\0" as *const u8
                        as *const i8,
                );
            }
            if *(*__ctype_b_loc()).offset(*name.offset(j as isize) as u8 as i32 as isize)
                as i32 & _IScntrl as i32 as libc::c_ushort as i32 != 0
            {
                (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 112 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_set_graph_name: graph name contains invalid character(s)\n\0"
                        as *const u8 as *const i8,
                );
            }
            j += 1;
            j;
        }
        (*G).name = _glp_dmp_get_atom(
            (*G).pool as *mut DMP,
            (strlen(name)).wrapping_add(1 as i32 as u64) as i32,
        ) as *mut i8;
        strcpy((*G).name, name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_add_vertices(mut G: *mut glp_graph, mut nadd: i32) -> i32 {
    let mut i: i32 = 0;
    let mut nv_new: i32 = 0;
    if nadd < 1 as i32 {
        (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 146 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_add_vertices: nadd = %d; invalid number of vertices\n\0" as *const u8
                as *const i8,
            nadd,
        );
    }
    if nadd > 100000000 as i32 - (*G).nv {
        (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 149 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_add_vertices: nadd = %d; too many vertices\n\0" as *const u8
                as *const i8,
            nadd,
        );
    }
    nv_new = (*G).nv + nadd;
    if (*G).nv_max < nv_new {
        let mut save: *mut *mut glp_vertex = (*G).v;
        while (*G).nv_max < nv_new {
            (*G).nv_max += (*G).nv_max;
            ((*G).nv_max > 0 as i32
                || {
                    glp_assert_(
                        b"G->nv_max > 0\0" as *const u8 as *const i8,
                        b"api/graph.c\0" as *const u8 as *const i8,
                        158 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        (*G).v = glp_alloc(
            1 as i32 + (*G).nv_max,
            ::core::mem::size_of::<*mut glp_vertex>() as u64 as i32,
        ) as *mut *mut glp_vertex;
        memcpy(
            &mut *((*G).v).offset(1 as i32 as isize) as *mut *mut glp_vertex
                as *mut libc::c_void,
            &mut *save.offset(1 as i32 as isize) as *mut *mut glp_vertex
                as *const libc::c_void,
            ((*G).nv as u64)
                .wrapping_mul(::core::mem::size_of::<*mut glp_vertex>() as u64),
        );
        glp_free(save as *mut libc::c_void);
    }
    i = (*G).nv + 1 as i32;
    while i <= nv_new {
        let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
        v = _glp_dmp_get_atom(
            (*G).pool as *mut DMP,
            ::core::mem::size_of::<glp_vertex>() as u64 as i32,
        ) as *mut glp_vertex;
        let ref mut fresh0 = *((*G).v).offset(i as isize);
        *fresh0 = v;
        (*v).i = i;
        (*v).name = 0 as *mut i8;
        (*v).entry = 0 as *mut libc::c_void;
        if (*G).v_size == 0 as i32 {
            (*v).data = 0 as *mut libc::c_void;
        } else {
            (*v).data = _glp_dmp_get_atom((*G).pool as *mut DMP, (*G).v_size);
            memset((*v).data, 0 as i32, (*G).v_size as u64);
        }
        (*v).temp = 0 as *mut libc::c_void;
        (*v).out = 0 as *mut glp_arc;
        (*v).in_0 = (*v).out;
        i += 1;
        i;
    }
    (*G).nv = nv_new;
    return nv_new - nadd + 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_vertex_name(
    mut G: *mut glp_graph,
    mut i: i32,
    mut name: *const i8,
) {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    if !(1 as i32 <= i && i <= (*G).nv) {
        (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 192 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_vertex_name: i = %d; vertex number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    v = *((*G).v).offset(i as isize);
    if !((*v).name).is_null() {
        if !((*v).entry).is_null() {
            (!((*G).index).is_null()
                || {
                    glp_assert_(
                        b"G->index != NULL\0" as *const u8 as *const i8,
                        b"api/graph.c\0" as *const u8 as *const i8,
                        197 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            _glp_avl_delete_node((*G).index as *mut AVL, (*v).entry as *mut AVLNODE);
            (*v).entry = 0 as *mut libc::c_void;
        }
        _glp_dmp_free_atom(
            (*G).pool as *mut DMP,
            (*v).name as *mut libc::c_void,
            (strlen((*v).name)).wrapping_add(1 as i32 as u64) as i32,
        );
        (*v).name = 0 as *mut i8;
    }
    if !(name.is_null() || *name.offset(0 as i32 as isize) as i32 == '\0' as i32) {
        let mut k: i32 = 0;
        k = 0 as i32;
        while *name.offset(k as isize) as i32 != '\0' as i32 {
            if k == 256 as i32 {
                (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 208 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_set_vertex_name: i = %d; vertex name too long\n\0" as *const u8
                        as *const i8,
                    i,
                );
            }
            if *(*__ctype_b_loc()).offset(*name.offset(k as isize) as u8 as i32 as isize)
                as i32 & _IScntrl as i32 as libc::c_ushort as i32 != 0
            {
                (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 211 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_set_vertex_name: i = %d; vertex name contains invalid character(s)\n\0"
                        as *const u8 as *const i8,
                    i,
                );
            }
            k += 1;
            k;
        }
        (*v).name = _glp_dmp_get_atom(
            (*G).pool as *mut DMP,
            (strlen(name)).wrapping_add(1 as i32 as u64) as i32,
        ) as *mut i8;
        strcpy((*v).name, name);
        if !((*G).index).is_null() {
            (((*v).entry).is_null()
                || {
                    glp_assert_(
                        b"v->entry == NULL\0" as *const u8 as *const i8,
                        b"api/graph.c\0" as *const u8 as *const i8,
                        217 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*v).entry = _glp_avl_insert_node(
                (*G).index as *mut AVL,
                (*v).name as *const libc::c_void,
            ) as *mut libc::c_void;
            _glp_avl_set_node_link((*v).entry as *mut AVLNODE, v as *mut libc::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_add_arc(
    mut G: *mut glp_graph,
    mut i: i32,
    mut j: i32,
) -> *mut glp_arc {
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    if !(1 as i32 <= i && i <= (*G).nv) {
        (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 249 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_add_arc: i = %d; tail vertex number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    if !(1 as i32 <= j && j <= (*G).nv) {
        (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 252 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_add_arc: j = %d; head vertex number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    if (*G).na == 500000000 as i32 {
        (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 255 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_add_arc: too many arcs\n\0" as *const u8 as *const i8);
    }
    a = _glp_dmp_get_atom(
        (*G).pool as *mut DMP,
        ::core::mem::size_of::<glp_arc>() as u64 as i32,
    ) as *mut glp_arc;
    (*a).tail = *((*G).v).offset(i as isize);
    (*a).head = *((*G).v).offset(j as isize);
    if (*G).a_size == 0 as i32 {
        (*a).data = 0 as *mut libc::c_void;
    } else {
        (*a).data = _glp_dmp_get_atom((*G).pool as *mut DMP, (*G).a_size);
        memset((*a).data, 0 as i32, (*G).a_size as u64);
    }
    (*a).temp = 0 as *mut libc::c_void;
    (*a).t_prev = 0 as *mut glp_arc;
    (*a).t_next = (**((*G).v).offset(i as isize)).out;
    if !((*a).t_next).is_null() {
        (*(*a).t_next).t_prev = a;
    }
    (*a).h_prev = 0 as *mut glp_arc;
    (*a).h_next = (**((*G).v).offset(j as isize)).in_0;
    if !((*a).h_next).is_null() {
        (*(*a).h_next).h_prev = a;
    }
    let ref mut fresh1 = (**((*G).v).offset(j as isize)).in_0;
    *fresh1 = a;
    let ref mut fresh2 = (**((*G).v).offset(i as isize)).out;
    *fresh2 = *fresh1;
    (*G).na += 1;
    (*G).na;
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn glp_del_vertices(
    mut G: *mut glp_graph,
    mut ndel: i32,
    mut num: *const i32,
) {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut nv_new: i32 = 0;
    if !(1 as i32 <= ndel && ndel <= (*G).nv) {
        (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 303 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_del_vertices: ndel = %d; invalid number of vertices\n\0" as *const u8
                as *const i8,
            ndel,
        );
    }
    k = 1 as i32;
    while k <= ndel {
        i = *num.offset(k as isize);
        if !(1 as i32 <= i && i <= (*G).nv) {
            (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 310 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_del_vertices: num[%d] = %d; vertex number out of range\n\0"
                    as *const u8 as *const i8,
                k,
                i,
            );
        }
        v = *((*G).v).offset(i as isize);
        if (*v).i == 0 as i32 {
            (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 315 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_del_vertices: num[%d] = %d; duplicate vertex numbers not allowed\n\0"
                    as *const u8 as *const i8,
                k,
                i,
            );
        }
        glp_set_vertex_name(G, i, 0 as *const i8);
        (((*v).name).is_null()
            || {
                glp_assert_(
                    b"v->name == NULL\0" as *const u8 as *const i8,
                    b"api/graph.c\0" as *const u8 as *const i8,
                    319 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (((*v).entry).is_null()
            || {
                glp_assert_(
                    b"v->entry == NULL\0" as *const u8 as *const i8,
                    b"api/graph.c\0" as *const u8 as *const i8,
                    320 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if !((*v).data).is_null() {
            _glp_dmp_free_atom((*G).pool as *mut DMP, (*v).data, (*G).v_size);
        }
        while !((*v).in_0).is_null() {
            glp_del_arc(G, (*v).in_0);
        }
        while !((*v).out).is_null() {
            glp_del_arc(G, (*v).out);
        }
        (*v).i = 0 as i32;
        k += 1;
        k;
    }
    nv_new = 0 as i32;
    i = 1 as i32;
    while i <= (*G).nv {
        v = *((*G).v).offset(i as isize);
        if (*v).i == 0 as i32 {
            _glp_dmp_free_atom(
                (*G).pool as *mut DMP,
                v as *mut libc::c_void,
                ::core::mem::size_of::<glp_vertex>() as u64 as i32,
            );
        } else {
            nv_new += 1;
            (*v).i = nv_new;
            let ref mut fresh3 = *((*G).v).offset((*v).i as isize);
            *fresh3 = v;
        }
        i += 1;
        i;
    }
    (*G).nv = nv_new;
}
#[no_mangle]
pub unsafe extern "C" fn glp_del_arc(mut G: *mut glp_graph, mut a: *mut glp_arc) {
    ((*G).na > 0 as i32
        || {
            glp_assert_(
                b"G->na > 0\0" as *const u8 as *const i8,
                b"api/graph.c\0" as *const u8 as *const i8,
                370 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= (*(*a).tail).i && (*(*a).tail).i <= (*G).nv
        || {
            glp_assert_(
                b"1 <= a->tail->i && a->tail->i <= G->nv\0" as *const u8 as *const i8,
                b"api/graph.c\0" as *const u8 as *const i8,
                371 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*a).tail == *((*G).v).offset((*(*a).tail).i as isize)
        || {
            glp_assert_(
                b"a->tail == G->v[a->tail->i]\0" as *const u8 as *const i8,
                b"api/graph.c\0" as *const u8 as *const i8,
                372 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= (*(*a).head).i && (*(*a).head).i <= (*G).nv
        || {
            glp_assert_(
                b"1 <= a->head->i && a->head->i <= G->nv\0" as *const u8 as *const i8,
                b"api/graph.c\0" as *const u8 as *const i8,
                373 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*a).head == *((*G).v).offset((*(*a).head).i as isize)
        || {
            glp_assert_(
                b"a->head == G->v[a->head->i]\0" as *const u8 as *const i8,
                b"api/graph.c\0" as *const u8 as *const i8,
                374 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if ((*a).h_prev).is_null() {
        (*(*a).head).in_0 = (*a).h_next;
    } else {
        (*(*a).h_prev).h_next = (*a).h_next;
    }
    if !((*a).h_next).is_null() {
        (*(*a).h_next).h_prev = (*a).h_prev;
    }
    if ((*a).t_prev).is_null() {
        (*(*a).tail).out = (*a).t_next;
    } else {
        (*(*a).t_prev).t_next = (*a).t_next;
    }
    if !((*a).t_next).is_null() {
        (*(*a).t_next).t_prev = (*a).t_prev;
    }
    if !((*a).data).is_null() {
        _glp_dmp_free_atom((*G).pool as *mut DMP, (*a).data, (*G).a_size);
    }
    _glp_dmp_free_atom(
        (*G).pool as *mut DMP,
        a as *mut libc::c_void,
        ::core::mem::size_of::<glp_arc>() as u64 as i32,
    );
    (*G).na -= 1;
    (*G).na;
}
unsafe extern "C" fn delete_graph(mut G: *mut glp_graph) {
    _glp_dmp_delete_pool((*G).pool as *mut DMP);
    glp_free((*G).v as *mut libc::c_void);
    if !((*G).index).is_null() {
        _glp_avl_delete_tree((*G).index as *mut AVL);
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_erase_graph(
    mut G: *mut glp_graph,
    mut v_size: i32,
    mut a_size: i32,
) {
    if !(0 as i32 <= v_size && v_size <= 256 as i32) {
        (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 428 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_erase_graph: v_size = %d; invalid size of vertex data\n\0" as *const u8
                as *const i8,
            v_size,
        );
    }
    if !(0 as i32 <= a_size && a_size <= 256 as i32) {
        (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 431 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_erase_graph: a_size = %d; invalid size of arc data\n\0" as *const u8
                as *const i8,
            a_size,
        );
    }
    delete_graph(G);
    create_graph(G, v_size, a_size);
}
#[no_mangle]
pub unsafe extern "C" fn glp_delete_graph(mut G: *mut glp_graph) {
    delete_graph(G);
    glp_free(G as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn glp_create_v_index(mut G: *mut glp_graph) {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut i: i32 = 0;
    if ((*G).index).is_null() {
        (*G).index = _glp_avl_create_tree(
            Some(
                _glp_avl_strcmp
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
            0 as *mut libc::c_void,
        ) as *mut libc::c_void;
        i = 1 as i32;
        while i <= (*G).nv {
            v = *((*G).v).offset(i as isize);
            (((*v).entry).is_null()
                || {
                    glp_assert_(
                        b"v->entry == NULL\0" as *const u8 as *const i8,
                        b"api/graph.c\0" as *const u8 as *const i8,
                        468 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if !((*v).name).is_null() {
                (*v).entry = _glp_avl_insert_node(
                    (*G).index as *mut AVL,
                    (*v).name as *const libc::c_void,
                ) as *mut libc::c_void;
                _glp_avl_set_node_link(
                    (*v).entry as *mut AVLNODE,
                    v as *mut libc::c_void,
                );
            }
            i += 1;
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_find_vertex(
    mut G: *mut glp_graph,
    mut name: *const i8,
) -> i32 {
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut i: i32 = 0 as i32;
    if ((*G).index).is_null() {
        (glp_error_(b"api/graph.c\0" as *const u8 as *const i8, 483 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_find_vertex: vertex name index does not exist\n\0" as *const u8
                as *const i8,
        );
    }
    if !(name.is_null() || *name.offset(0 as i32 as isize) as i32 == '\0' as i32
        || strlen(name) > 255 as i32 as u64)
    {
        node = _glp_avl_find_node((*G).index as *mut AVL, name as *const libc::c_void);
        if !node.is_null() {
            i = (*(_glp_avl_get_node_link(node) as *mut glp_vertex)).i;
        }
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn glp_delete_v_index(mut G: *mut glp_graph) {
    let mut i: i32 = 0;
    if !((*G).index).is_null() {
        _glp_avl_delete_tree((*G).index as *mut AVL);
        (*G).index = 0 as *mut libc::c_void;
        i = 1 as i32;
        while i <= (*G).nv {
            let ref mut fresh4 = (**((*G).v).offset(i as isize)).entry;
            *fresh4 = 0 as *mut libc::c_void;
            i += 1;
            i;
        }
    }
}