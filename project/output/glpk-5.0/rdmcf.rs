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
    pub type glp_file;
    fn _glp_dmx_check_int(csa: *mut DMX, num: libc::c_double);
    fn _glp_dmx_end_of_line(csa: *mut DMX);
    fn _glp_dmx_read_field(csa: *mut DMX);
    fn _glp_dmx_read_designator(csa: *mut DMX);
    fn _glp_dmx_error(csa: *mut DMX, fmt: *const i8, _: ...);
    fn _glp_close(f: *mut glp_file) -> i32;
    fn _glp_open(name: *const i8, mode: *const i8) -> *mut glp_file;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn _glp_get_err_msg() -> *const i8;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_printf(fmt: *const i8, _: ...);
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn glp_add_vertices(G: *mut glp_graph, nadd: i32) -> i32;
    fn glp_add_arc(G: *mut glp_graph, i: i32, j: i32) -> *mut glp_arc;
    fn glp_erase_graph(G: *mut glp_graph, v_size: i32, a_size: i32);
    fn _glp_str2int(str: *const i8, val: *mut i32) -> i32;
    fn _glp_str2num(str: *const i8, val: *mut libc::c_double) -> i32;
}
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DMX {
    pub jump: jmp_buf,
    pub fname: *const i8,
    pub fp: *mut glp_file,
    pub count: i32,
    pub c: i32,
    pub field: [i8; 256],
    pub empty: i32,
    pub nonint: i32,
}
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
#[no_mangle]
pub unsafe extern "C" fn glp_read_mincost(
    mut G: *mut glp_graph,
    mut v_rhs: i32,
    mut a_low: i32,
    mut a_cap: i32,
    mut a_cost: i32,
    mut fname: *const i8,
) -> i32 {
    let mut _csa: DMX = DMX {
        jump: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        fname: 0 as *const i8,
        fp: 0 as *mut glp_file,
        count: 0,
        c: 0,
        field: [0; 256],
        empty: 0,
        nonint: 0,
    };
    let mut csa: *mut DMX = &mut _csa;
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut nv: i32 = 0;
    let mut na: i32 = 0;
    let mut ret: i32 = 0 as i32;
    let mut rhs: libc::c_double = 0.;
    let mut low: libc::c_double = 0.;
    let mut cap: libc::c_double = 0.;
    let mut cost: libc::c_double = 0.;
    let mut flag: *mut i8 = 0 as *mut i8;
    if v_rhs >= 0 as i32
        && v_rhs > (*G).v_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/rdmcf.c\0" as *const u8 as *const i8, 63 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_read_mincost: v_rhs = %d; invalid offset\n\0" as *const u8
                as *const i8,
            v_rhs,
        );
    }
    if a_low >= 0 as i32
        && a_low > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/rdmcf.c\0" as *const u8 as *const i8, 66 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_read_mincost: a_low = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_low,
        );
    }
    if a_cap >= 0 as i32
        && a_cap > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/rdmcf.c\0" as *const u8 as *const i8, 69 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_read_mincost: a_cap = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_cap,
        );
    }
    if a_cost >= 0 as i32
        && a_cost > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/rdmcf.c\0" as *const u8 as *const i8, 72 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_read_mincost: a_cost = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_cost,
        );
    }
    glp_erase_graph(G, (*G).v_size, (*G).a_size);
    if _setjmp(((*csa).jump).as_mut_ptr()) != 0 {
        ret = 1 as i32;
    } else {
        (*csa).fname = fname;
        (*csa).fp = 0 as *mut glp_file;
        (*csa).count = 0 as i32;
        (*csa).c = '\n' as i32;
        (*csa).field[0 as i32 as usize] = '\0' as i32 as i8;
        (*csa).nonint = 0 as i32;
        (*csa).empty = (*csa).nonint;
        glp_printf(
            b"Reading min-cost flow problem data from '%s'...\n\0" as *const u8
                as *const i8,
            fname,
        );
        (*csa).fp = _glp_open(fname, b"r\0" as *const u8 as *const i8);
        if ((*csa).fp).is_null() {
            glp_printf(
                b"Unable to open '%s' - %s\n\0" as *const u8 as *const i8,
                fname,
                _glp_get_err_msg(),
            );
            longjmp(((*csa).jump).as_mut_ptr(), 1 as i32);
        }
        _glp_dmx_read_designator(csa);
        if strcmp(((*csa).field).as_mut_ptr(), b"p\0" as *const u8 as *const i8)
            != 0 as i32
        {
            _glp_dmx_error(
                csa,
                b"problem line missing or invalid\0" as *const u8 as *const i8,
            );
        }
        _glp_dmx_read_field(csa);
        if strcmp(((*csa).field).as_mut_ptr(), b"min\0" as *const u8 as *const i8)
            != 0 as i32
        {
            _glp_dmx_error(
                csa,
                b"wrong problem designator; 'min' expected\0" as *const u8 as *const i8,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut nv) == 0 as i32
            && nv >= 0 as i32)
        {
            _glp_dmx_error(
                csa,
                b"number of nodes missing or invalid\0" as *const u8 as *const i8,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut na) == 0 as i32
            && na >= 0 as i32)
        {
            _glp_dmx_error(
                csa,
                b"number of arcs missing or invalid\0" as *const u8 as *const i8,
            );
        }
        glp_printf(
            b"Flow network has %d node%s and %d arc%s\n\0" as *const u8 as *const i8,
            nv,
            if nv == 1 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"s\0" as *const u8 as *const i8
            },
            na,
            if na == 1 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"s\0" as *const u8 as *const i8
            },
        );
        if nv > 0 as i32 {
            glp_add_vertices(G, nv);
        }
        _glp_dmx_end_of_line(csa);
        flag = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i8>() as u64 as i32)
            as *mut i8;
        memset(
            &mut *flag.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
            0 as i32,
            (nv as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
        if v_rhs >= 0 as i32 {
            rhs = 0.0f64;
            i = 1 as i32;
            while i <= nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    ((*v).data as *mut i8).offset(v_rhs as isize) as *mut libc::c_void,
                    &mut rhs as *mut libc::c_double as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
                i += 1;
                i;
            }
        }
        loop {
            _glp_dmx_read_designator(csa);
            if strcmp(((*csa).field).as_mut_ptr(), b"n\0" as *const u8 as *const i8)
                != 0 as i32
            {
                break;
            }
            _glp_dmx_read_field(csa);
            if _glp_str2int(((*csa).field).as_mut_ptr(), &mut i) != 0 as i32 {
                _glp_dmx_error(
                    csa,
                    b"node number missing or invalid\0" as *const u8 as *const i8,
                );
            }
            if !(1 as i32 <= i && i <= nv) {
                _glp_dmx_error(
                    csa,
                    b"node number %d out of range\0" as *const u8 as *const i8,
                    i,
                );
            }
            if *flag.offset(i as isize) != 0 {
                _glp_dmx_error(
                    csa,
                    b"duplicate descriptor of node %d\0" as *const u8 as *const i8,
                    i,
                );
            }
            _glp_dmx_read_field(csa);
            if _glp_str2num(((*csa).field).as_mut_ptr(), &mut rhs) != 0 as i32 {
                _glp_dmx_error(
                    csa,
                    b"node supply/demand missing or invalid\0" as *const u8 as *const i8,
                );
            }
            _glp_dmx_check_int(csa, rhs);
            if v_rhs >= 0 as i32 {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    ((*v).data as *mut i8).offset(v_rhs as isize) as *mut libc::c_void,
                    &mut rhs as *mut libc::c_double as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
            }
            *flag.offset(i as isize) = 1 as i32 as i8;
            _glp_dmx_end_of_line(csa);
        }
        glp_free(flag as *mut libc::c_void);
        flag = 0 as *mut i8;
        k = 1 as i32;
        while k <= na {
            if k > 1 as i32 {
                _glp_dmx_read_designator(csa);
            }
            if strcmp(((*csa).field).as_mut_ptr(), b"a\0" as *const u8 as *const i8)
                != 0 as i32
            {
                _glp_dmx_error(
                    csa,
                    b"wrong line designator; 'a' expected\0" as *const u8 as *const i8,
                );
            }
            _glp_dmx_read_field(csa);
            if _glp_str2int(((*csa).field).as_mut_ptr(), &mut i) != 0 as i32 {
                _glp_dmx_error(
                    csa,
                    b"starting node number missing or invalid\0" as *const u8
                        as *const i8,
                );
            }
            if !(1 as i32 <= i && i <= nv) {
                _glp_dmx_error(
                    csa,
                    b"starting node number %d out of range\0" as *const u8 as *const i8,
                    i,
                );
            }
            _glp_dmx_read_field(csa);
            if _glp_str2int(((*csa).field).as_mut_ptr(), &mut j) != 0 as i32 {
                _glp_dmx_error(
                    csa,
                    b"ending node number missing or invalid\0" as *const u8 as *const i8,
                );
            }
            if !(1 as i32 <= j && j <= nv) {
                _glp_dmx_error(
                    csa,
                    b"ending node number %d out of range\0" as *const u8 as *const i8,
                    j,
                );
            }
            _glp_dmx_read_field(csa);
            if !(_glp_str2num(((*csa).field).as_mut_ptr(), &mut low) == 0 as i32
                && low >= 0.0f64)
            {
                _glp_dmx_error(
                    csa,
                    b"lower bound of arc flow missing or invalid\0" as *const u8
                        as *const i8,
                );
            }
            _glp_dmx_check_int(csa, low);
            _glp_dmx_read_field(csa);
            if !(_glp_str2num(((*csa).field).as_mut_ptr(), &mut cap) == 0 as i32
                && cap >= low)
            {
                _glp_dmx_error(
                    csa,
                    b"upper bound of arc flow missing or invalid\0" as *const u8
                        as *const i8,
                );
            }
            _glp_dmx_check_int(csa, cap);
            _glp_dmx_read_field(csa);
            if _glp_str2num(((*csa).field).as_mut_ptr(), &mut cost) != 0 as i32 {
                _glp_dmx_error(
                    csa,
                    b"per-unit cost of arc flow missing or invalid\0" as *const u8
                        as *const i8,
                );
            }
            _glp_dmx_check_int(csa, cost);
            a = glp_add_arc(G, i, j);
            if a_low >= 0 as i32 {
                memcpy(
                    ((*a).data as *mut i8).offset(a_low as isize) as *mut libc::c_void,
                    &mut low as *mut libc::c_double as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
            }
            if a_cap >= 0 as i32 {
                memcpy(
                    ((*a).data as *mut i8).offset(a_cap as isize) as *mut libc::c_void,
                    &mut cap as *mut libc::c_double as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
            }
            if a_cost >= 0 as i32 {
                memcpy(
                    ((*a).data as *mut i8).offset(a_cost as isize) as *mut libc::c_void,
                    &mut cost as *mut libc::c_double as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
            }
            _glp_dmx_end_of_line(csa);
            k += 1;
            k;
        }
        glp_printf(b"%d lines were read\n\0" as *const u8 as *const i8, (*csa).count);
    }
    if ret != 0 {
        glp_erase_graph(G, (*G).v_size, (*G).a_size);
    }
    if !((*csa).fp).is_null() {
        _glp_close((*csa).fp);
    }
    if !flag.is_null() {
        glp_free(flag as *mut libc::c_void);
    }
    return ret;
}