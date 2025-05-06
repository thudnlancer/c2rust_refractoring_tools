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
pub unsafe extern "C" fn glp_read_ipt(
    mut P: *mut glp_prob,
    mut fname: *const i8,
) -> i32 {
    let mut dmx_: DMX = DMX {
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
    let mut dmx: *mut DMX = &mut dmx_;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut sst: i32 = 0;
    let mut ret: i32 = 1 as i32;
    let mut stat: *mut i8 = 0 as *mut i8;
    let mut obj: libc::c_double = 0.;
    let mut prim: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dual: *mut libc::c_double = 0 as *mut libc::c_double;
    if fname.is_null() {
        (glp_error_(b"api/rdipt.c\0" as *const u8 as *const i8, 56 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_read_ipt: fname = %d; invalid parameter\n\0" as *const u8 as *const i8,
            fname,
        );
    }
    if !(_setjmp(((*dmx).jump).as_mut_ptr()) != 0) {
        (*dmx).fname = fname;
        (*dmx).fp = 0 as *mut glp_file;
        (*dmx).count = 0 as i32;
        (*dmx).c = '\n' as i32;
        (*dmx).field[0 as i32 as usize] = '\0' as i32 as i8;
        (*dmx).nonint = 0 as i32;
        (*dmx).empty = (*dmx).nonint;
        glp_printf(
            b"Reading interior-point solution from '%s'...\n\0" as *const u8
                as *const i8,
            fname,
        );
        (*dmx).fp = _glp_open(fname, b"r\0" as *const u8 as *const i8);
        if ((*dmx).fp).is_null() {
            glp_printf(
                b"Unable to open '%s' - %s\n\0" as *const u8 as *const i8,
                fname,
                _glp_get_err_msg(),
            );
        } else {
            _glp_dmx_read_designator(dmx);
            if strcmp(((*dmx).field).as_mut_ptr(), b"s\0" as *const u8 as *const i8)
                != 0 as i32
            {
                _glp_dmx_error(
                    dmx,
                    b"solution line missing or invalid\0" as *const u8 as *const i8,
                );
            }
            _glp_dmx_read_field(dmx);
            if strcmp(((*dmx).field).as_mut_ptr(), b"ipt\0" as *const u8 as *const i8)
                != 0 as i32
            {
                _glp_dmx_error(
                    dmx,
                    b"wrong solution designator; 'ipt' expected\0" as *const u8
                        as *const i8,
                );
            }
            _glp_dmx_read_field(dmx);
            if !(_glp_str2int(((*dmx).field).as_mut_ptr(), &mut m) == 0 as i32
                && m >= 0 as i32)
            {
                _glp_dmx_error(
                    dmx,
                    b"number of rows missing or invalid\0" as *const u8 as *const i8,
                );
            }
            if m != (*P).m {
                _glp_dmx_error(
                    dmx,
                    b"number of rows mismatch\0" as *const u8 as *const i8,
                );
            }
            _glp_dmx_read_field(dmx);
            if !(_glp_str2int(((*dmx).field).as_mut_ptr(), &mut n) == 0 as i32
                && n >= 0 as i32)
            {
                _glp_dmx_error(
                    dmx,
                    b"number of columns missing or invalid\0" as *const u8 as *const i8,
                );
            }
            if n != (*P).n {
                _glp_dmx_error(
                    dmx,
                    b"number of columns mismatch\0" as *const u8 as *const i8,
                );
            }
            _glp_dmx_read_field(dmx);
            if strcmp(((*dmx).field).as_mut_ptr(), b"o\0" as *const u8 as *const i8)
                == 0 as i32
            {
                sst = 5 as i32;
            } else if strcmp(
                ((*dmx).field).as_mut_ptr(),
                b"i\0" as *const u8 as *const i8,
            ) == 0 as i32
            {
                sst = 3 as i32;
            } else if strcmp(
                ((*dmx).field).as_mut_ptr(),
                b"n\0" as *const u8 as *const i8,
            ) == 0 as i32
            {
                sst = 4 as i32;
            } else if strcmp(
                ((*dmx).field).as_mut_ptr(),
                b"u\0" as *const u8 as *const i8,
            ) == 0 as i32
            {
                sst = 1 as i32;
            } else {
                _glp_dmx_error(
                    dmx,
                    b"solution status missing or invalid\0" as *const u8 as *const i8,
                );
            }
            _glp_dmx_read_field(dmx);
            if _glp_str2num(((*dmx).field).as_mut_ptr(), &mut obj) != 0 as i32 {
                _glp_dmx_error(
                    dmx,
                    b"objective value missing or invalid\0" as *const u8 as *const i8,
                );
            }
            _glp_dmx_end_of_line(dmx);
            stat = glp_alloc(
                1 as i32 + m + n,
                ::core::mem::size_of::<i8>() as u64 as i32,
            ) as *mut i8;
            k = 1 as i32;
            while k <= m + n {
                *stat.offset(k as isize) = '?' as i32 as i8;
                k += 1;
                k;
            }
            prim = glp_alloc(
                1 as i32 + m + n,
                ::core::mem::size_of::<libc::c_double>() as u64 as i32,
            ) as *mut libc::c_double;
            dual = glp_alloc(
                1 as i32 + m + n,
                ::core::mem::size_of::<libc::c_double>() as u64 as i32,
            ) as *mut libc::c_double;
            loop {
                _glp_dmx_read_designator(dmx);
                if strcmp(((*dmx).field).as_mut_ptr(), b"i\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    _glp_dmx_read_field(dmx);
                    if _glp_str2int(((*dmx).field).as_mut_ptr(), &mut i) != 0 as i32 {
                        _glp_dmx_error(
                            dmx,
                            b"row number missing or invalid\0" as *const u8 as *const i8,
                        );
                    }
                    if !(1 as i32 <= i && i <= m) {
                        _glp_dmx_error(
                            dmx,
                            b"row number out of range\0" as *const u8 as *const i8,
                        );
                    }
                    if *stat.offset(i as isize) as i32 != '?' as i32 {
                        _glp_dmx_error(
                            dmx,
                            b"duplicate row solution descriptor\0" as *const u8
                                as *const i8,
                        );
                    }
                    *stat.offset(i as isize) = 1 as i32 as i8;
                    _glp_dmx_read_field(dmx);
                    if _glp_str2num(
                        ((*dmx).field).as_mut_ptr(),
                        &mut *prim.offset(i as isize),
                    ) != 0 as i32
                    {
                        _glp_dmx_error(
                            dmx,
                            b"row primal value missing or invalid\0" as *const u8
                                as *const i8,
                        );
                    }
                    _glp_dmx_read_field(dmx);
                    if _glp_str2num(
                        ((*dmx).field).as_mut_ptr(),
                        &mut *dual.offset(i as isize),
                    ) != 0 as i32
                    {
                        _glp_dmx_error(
                            dmx,
                            b"row dual value missing or invalid\0" as *const u8
                                as *const i8,
                        );
                    }
                    _glp_dmx_end_of_line(dmx);
                } else if strcmp(
                    ((*dmx).field).as_mut_ptr(),
                    b"j\0" as *const u8 as *const i8,
                ) == 0 as i32
                {
                    _glp_dmx_read_field(dmx);
                    if _glp_str2int(((*dmx).field).as_mut_ptr(), &mut j) != 0 as i32 {
                        _glp_dmx_error(
                            dmx,
                            b"column number missing or invalid\0" as *const u8
                                as *const i8,
                        );
                    }
                    if !(1 as i32 <= j && j <= n) {
                        _glp_dmx_error(
                            dmx,
                            b"column number out of range\0" as *const u8 as *const i8,
                        );
                    }
                    if *stat.offset((m + j) as isize) as i32 != '?' as i32 {
                        _glp_dmx_error(
                            dmx,
                            b"duplicate column solution descriptor\0" as *const u8
                                as *const i8,
                        );
                    }
                    *stat.offset((m + j) as isize) = 1 as i32 as i8;
                    _glp_dmx_read_field(dmx);
                    if _glp_str2num(
                        ((*dmx).field).as_mut_ptr(),
                        &mut *prim.offset((m + j) as isize),
                    ) != 0 as i32
                    {
                        _glp_dmx_error(
                            dmx,
                            b"column primal value missing or invalid\0" as *const u8
                                as *const i8,
                        );
                    }
                    _glp_dmx_read_field(dmx);
                    if _glp_str2num(
                        ((*dmx).field).as_mut_ptr(),
                        &mut *dual.offset((m + j) as isize),
                    ) != 0 as i32
                    {
                        _glp_dmx_error(
                            dmx,
                            b"column dual value missing or invalid\0" as *const u8
                                as *const i8,
                        );
                    }
                    _glp_dmx_end_of_line(dmx);
                } else {
                    if strcmp(
                        ((*dmx).field).as_mut_ptr(),
                        b"e\0" as *const u8 as *const i8,
                    ) == 0 as i32
                    {
                        break;
                    }
                    _glp_dmx_error(
                        dmx,
                        b"line designator missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                _glp_dmx_end_of_line(dmx);
            }
            k = 1 as i32;
            while k <= m + n {
                if *stat.offset(k as isize) as i32 == '?' as i32 {
                    _glp_dmx_error(
                        dmx,
                        b"incomplete interior-point solution\0" as *const u8 as *const i8,
                    );
                }
                k += 1;
                k;
            }
            (*P).ipt_stat = sst;
            (*P).ipt_obj = obj;
            i = 1 as i32;
            while i <= m {
                (**((*P).row).offset(i as isize)).pval = *prim.offset(i as isize);
                (**((*P).row).offset(i as isize)).dval = *dual.offset(i as isize);
                i += 1;
                i;
            }
            j = 1 as i32;
            while j <= n {
                (**((*P).col).offset(j as isize)).pval = *prim.offset((m + j) as isize);
                (**((*P).col).offset(j as isize)).dval = *dual.offset((m + j) as isize);
                j += 1;
                j;
            }
            glp_printf(
                b"%d lines were read\n\0" as *const u8 as *const i8,
                (*dmx).count,
            );
            ret = 0 as i32;
        }
    }
    if !((*dmx).fp).is_null() {
        _glp_close((*dmx).fp);
    }
    if !stat.is_null() {
        glp_free(stat as *mut libc::c_void);
    }
    if !prim.is_null() {
        glp_free(prim as *mut libc::c_void);
    }
    if !dual.is_null() {
        glp_free(dual as *mut libc::c_void);
    }
    return ret;
}