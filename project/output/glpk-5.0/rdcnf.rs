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
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn _glp_dmx_end_of_line(csa: *mut DMX);
    fn _glp_dmx_read_field(csa: *mut DMX);
    fn _glp_dmx_read_designator(csa: *mut DMX);
    fn _glp_dmx_read_char(csa: *mut DMX);
    fn _glp_dmx_error(csa: *mut DMX, fmt: *const i8, _: ...);
    fn _glp_close(f: *mut glp_file) -> i32;
    fn _glp_open(name: *const i8, mode: *const i8) -> *mut glp_file;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn _glp_get_err_msg() -> *const i8;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_printf(fmt: *const i8, _: ...);
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn _glp_str2int(str: *const i8, val: *mut i32) -> i32;
    fn glp_add_rows(P: *mut glp_prob, nrs: i32) -> i32;
    fn glp_add_cols(P: *mut glp_prob, ncs: i32) -> i32;
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_mat_row(
        P: *mut glp_prob,
        i: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
    );
    fn glp_sort_matrix(P: *mut glp_prob);
    fn glp_erase_prob(P: *mut glp_prob);
    fn glp_set_col_kind(P: *mut glp_prob, j: i32, kind: i32);
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
pub struct glp_prob {
    pub pool: *mut DMP,
    pub tree: *mut glp_tree,
    pub name: *mut i8,
    pub obj: *mut i8,
    pub dir: i32,
    pub c0: libc::c_double,
    pub m_max: i32,
    pub n_max: i32,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut AVL,
    pub c_tree: *mut AVL,
    pub valid: i32,
    pub head: *mut i32,
    pub bfd: *mut BFD,
    pub pbs_stat: i32,
    pub dbs_stat: i32,
    pub obj_val: libc::c_double,
    pub it_cnt: i32,
    pub some: i32,
    pub ipt_stat: i32,
    pub ipt_obj: libc::c_double,
    pub mip_stat: i32,
    pub mip_obj: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPCOL {
    pub j: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub kind: i32,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPAIJ {
    pub row: *mut GLPROW,
    pub col: *mut GLPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut GLPAIJ,
    pub r_next: *mut GLPAIJ,
    pub c_prev: *mut GLPAIJ,
    pub c_next: *mut GLPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPROW {
    pub i: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub level: i32,
    pub origin: u8,
    pub klass: u8,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn glp_read_cnfsat(
    mut P: *mut glp_prob,
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
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut len: i32 = 0;
    let mut neg: i32 = 0;
    let mut rhs: i32 = 0;
    let mut ret: i32 = 0 as i32;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut map: *mut i8 = 0 as *mut i8;
    if fname.is_null() {
        (glp_error_(b"api/rdcnf.c\0" as *const u8 as *const i8, 47 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_read_cnfsat: fname = %p; invalid parameter\n\0" as *const u8
                as *const i8,
            fname,
        );
    }
    glp_erase_prob(P);
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
            b"Reading CNF-SAT problem data from '%s'...\n\0" as *const u8 as *const i8,
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
        if strcmp(((*csa).field).as_mut_ptr(), b"cnf\0" as *const u8 as *const i8)
            != 0 as i32
        {
            _glp_dmx_error(
                csa,
                b"wrong problem designator; 'cnf' expected\n\0" as *const u8 as *const i8,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut n) == 0 as i32
            && n >= 0 as i32)
        {
            _glp_dmx_error(
                csa,
                b"number of variables missing or invalid\n\0" as *const u8 as *const i8,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut m) == 0 as i32
            && m >= 0 as i32)
        {
            _glp_dmx_error(
                csa,
                b"number of clauses missing or invalid\n\0" as *const u8 as *const i8,
            );
        }
        glp_printf(
            b"Instance has %d variable%s and %d clause%s\n\0" as *const u8 as *const i8,
            n,
            if n == 1 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"s\0" as *const u8 as *const i8
            },
            m,
            if m == 1 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"s\0" as *const u8 as *const i8
            },
        );
        _glp_dmx_end_of_line(csa);
        if m > 0 as i32 {
            glp_add_rows(P, m);
        }
        if n > 0 as i32 {
            glp_add_cols(P, n);
            j = 1 as i32;
            while j <= n {
                glp_set_col_kind(P, j, 3 as i32);
                j += 1;
                j;
            }
        }
        ind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        val = glp_alloc(
            1 as i32 + n,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        map = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i8>() as u64 as i32)
            as *mut i8;
        j = 1 as i32;
        while j <= n {
            *map.offset(j as isize) = 0 as i32 as i8;
            j += 1;
            j;
        }
        i = 1 as i32;
        while i <= m {
            len = 0 as i32;
            rhs = 1 as i32;
            loop {
                while (*csa).c == ' ' as i32 || (*csa).c == '\n' as i32 {
                    _glp_dmx_read_char(csa);
                }
                _glp_dmx_read_field(csa);
                if _glp_str2int(((*csa).field).as_mut_ptr(), &mut j) != 0 as i32 {
                    _glp_dmx_error(
                        csa,
                        b"variable number missing or invalid\n\0" as *const u8
                            as *const i8,
                    );
                }
                if j > 0 as i32 {
                    neg = 0 as i32;
                } else {
                    if !(j < 0 as i32) {
                        break;
                    }
                    neg = 1 as i32;
                    j = -j;
                    rhs -= 1;
                    rhs;
                }
                if !(1 as i32 <= j && j <= n) {
                    _glp_dmx_error(
                        csa,
                        b"variable number out of range\n\0" as *const u8 as *const i8,
                    );
                }
                if *map.offset(j as isize) != 0 {
                    _glp_dmx_error(
                        csa,
                        b"duplicate variable number\n\0" as *const u8 as *const i8,
                    );
                }
                len += 1;
                len;
                *ind.offset(len as isize) = j;
                *val.offset(len as isize) = (if neg != 0 { -1.0f64 } else { 1.0f64 });
                *map.offset(j as isize) = 1 as i32 as i8;
            }
            glp_set_row_bnds(P, i, 2 as i32, rhs as libc::c_double, 0.0f64);
            glp_set_mat_row(P, i, len, ind as *const i32, val as *const libc::c_double);
            while len > 0 as i32 {
                let fresh0 = len;
                len = len - 1;
                *map.offset(*ind.offset(fresh0 as isize) as isize) = 0 as i32 as i8;
            }
            i += 1;
            i;
        }
        glp_printf(b"%d lines were read\n\0" as *const u8 as *const i8, (*csa).count);
        glp_sort_matrix(P);
    }
    if !((*csa).fp).is_null() {
        _glp_close((*csa).fp);
    }
    if !ind.is_null() {
        glp_free(ind as *mut libc::c_void);
    }
    if !val.is_null() {
        glp_free(val as *mut libc::c_void);
    }
    if !map.is_null() {
        glp_free(map as *mut libc::c_void);
    }
    if ret != 0 {
        glp_erase_prob(P);
    }
    return ret;
}