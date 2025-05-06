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
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_get_err_msg() -> *const i8;
    fn _glp_str2int(str: *const i8, val: *mut i32) -> i32;
    fn _glp_str2num(str: *const i8, val: *mut libc::c_double) -> i32;
    fn glp_set_prob_name(P: *mut glp_prob, name: *const i8);
    fn glp_set_obj_name(P: *mut glp_prob, name: *const i8);
    fn glp_set_obj_dir(P: *mut glp_prob, dir: i32);
    fn glp_add_rows(P: *mut glp_prob, nrs: i32) -> i32;
    fn glp_add_cols(P: *mut glp_prob, ncs: i32) -> i32;
    fn glp_set_row_name(P: *mut glp_prob, i: i32, name: *const i8);
    fn glp_set_col_name(P: *mut glp_prob, j: i32, name: *const i8);
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_obj_coef(P: *mut glp_prob, j: i32, coef: libc::c_double);
    fn glp_load_matrix(
        P: *mut glp_prob,
        ne: i32,
        ia: *const i32,
        ja: *const i32,
        ar: *const libc::c_double,
    );
    fn glp_check_dup(m: i32, n: i32, ne: i32, ia: *const i32, ja: *const i32) -> i32;
    fn glp_sort_matrix(P: *mut glp_prob);
    fn glp_erase_prob(P: *mut glp_prob);
    fn glp_set_col_kind(P: *mut glp_prob, j: i32, kind: i32);
    fn glp_get_num_int(P: *mut glp_prob) -> i32;
    fn glp_get_num_bin(P: *mut glp_prob) -> i32;
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
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
pub unsafe extern "C" fn glp_read_prob(
    mut P: *mut glp_prob,
    mut flags: i32,
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
    let mut mip: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut nnz: i32 = 0;
    let mut ne: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut type_0: i32 = 0;
    let mut kind: i32 = 0;
    let mut ret: i32 = 0;
    let mut ln: *mut i32 = 0 as *mut i32;
    let mut ia: *mut i32 = 0 as *mut i32;
    let mut ja: *mut i32 = 0 as *mut i32;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut ar: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rf: *mut i8 = 0 as *mut i8;
    let mut cf: *mut i8 = 0 as *mut i8;
    if flags != 0 as i32 {
        (glp_error_(b"api/rdprob.c\0" as *const u8 as *const i8, 64 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_read_prob: flags = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            flags,
        );
    }
    if fname.is_null() {
        (glp_error_(b"api/rdprob.c\0" as *const u8 as *const i8, 67 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_read_prob: fname = %d; invalid parameter\n\0" as *const u8
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
            b"Reading problem data from '%s'...\n\0" as *const u8 as *const i8,
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
        if strcmp(((*csa).field).as_mut_ptr(), b"lp\0" as *const u8 as *const i8)
            == 0 as i32
        {
            mip = 0 as i32;
        } else if strcmp(((*csa).field).as_mut_ptr(), b"mip\0" as *const u8 as *const i8)
            == 0 as i32
        {
            mip = 1 as i32;
        } else {
            _glp_dmx_error(
                csa,
                b"wrong problem designator; 'lp' or 'mip' expected\0" as *const u8
                    as *const i8,
            );
        }
        _glp_dmx_read_field(csa);
        if strcmp(((*csa).field).as_mut_ptr(), b"min\0" as *const u8 as *const i8)
            == 0 as i32
        {
            glp_set_obj_dir(P, 1 as i32);
        } else if strcmp(((*csa).field).as_mut_ptr(), b"max\0" as *const u8 as *const i8)
            == 0 as i32
        {
            glp_set_obj_dir(P, 2 as i32);
        } else {
            _glp_dmx_error(
                csa,
                b"objective sense missing or invalid\0" as *const u8 as *const i8,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut m) == 0 as i32
            && m >= 0 as i32)
        {
            _glp_dmx_error(
                csa,
                b"number of rows missing or invalid\0" as *const u8 as *const i8,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut n) == 0 as i32
            && n >= 0 as i32)
        {
            _glp_dmx_error(
                csa,
                b"number of columns missing or invalid\0" as *const u8 as *const i8,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut nnz) == 0 as i32
            && nnz >= 0 as i32)
        {
            _glp_dmx_error(
                csa,
                b"number of constraint coefficients missing or invalid\0" as *const u8
                    as *const i8,
            );
        }
        if m > 0 as i32 {
            glp_add_rows(P, m);
            i = 1 as i32;
            while i <= m {
                glp_set_row_bnds(P, i, 5 as i32, 0.0f64, 0.0f64);
                i += 1;
                i;
            }
        }
        if n > 0 as i32 {
            glp_add_cols(P, n);
            j = 1 as i32;
            while j <= n {
                if mip == 0 {
                    glp_set_col_bnds(P, j, 2 as i32, 0.0f64, 0.0f64);
                } else {
                    glp_set_col_kind(P, j, 3 as i32);
                }
                j += 1;
                j;
            }
        }
        _glp_dmx_end_of_line(csa);
        rf = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i8>() as u64 as i32)
            as *mut i8;
        memset(rf as *mut libc::c_void, 0 as i32, (1 as i32 + m) as u64);
        cf = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i8>() as u64 as i32)
            as *mut i8;
        memset(cf as *mut libc::c_void, 0 as i32, (1 as i32 + n) as u64);
        ln = glp_alloc(1 as i32 + nnz, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        ia = glp_alloc(1 as i32 + nnz, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        ja = glp_alloc(1 as i32 + nnz, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        ar = glp_alloc(
            1 as i32 + nnz,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        ne = 0 as i32;
        loop {
            _glp_dmx_read_designator(csa);
            if strcmp(((*csa).field).as_mut_ptr(), b"i\0" as *const u8 as *const i8)
                == 0 as i32
            {
                _glp_dmx_read_field(csa);
                if _glp_str2int(((*csa).field).as_mut_ptr(), &mut i) != 0 as i32 {
                    _glp_dmx_error(
                        csa,
                        b"row number missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                if !(1 as i32 <= i && i <= m) {
                    _glp_dmx_error(
                        csa,
                        b"row number out of range\0" as *const u8 as *const i8,
                    );
                }
                _glp_dmx_read_field(csa);
                if strcmp(((*csa).field).as_mut_ptr(), b"f\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    type_0 = 1 as i32;
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"l\0" as *const u8 as *const i8,
                ) == 0 as i32
                {
                    type_0 = 2 as i32;
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"u\0" as *const u8 as *const i8,
                ) == 0 as i32
                {
                    type_0 = 3 as i32;
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"d\0" as *const u8 as *const i8,
                ) == 0 as i32
                {
                    type_0 = 4 as i32;
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"s\0" as *const u8 as *const i8,
                ) == 0 as i32
                {
                    type_0 = 5 as i32;
                } else {
                    _glp_dmx_error(
                        csa,
                        b"row type missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                if type_0 == 2 as i32 || type_0 == 4 as i32 || type_0 == 5 as i32 {
                    _glp_dmx_read_field(csa);
                    if _glp_str2num(((*csa).field).as_mut_ptr(), &mut lb) != 0 as i32 {
                        _glp_dmx_error(
                            csa,
                            b"row lower bound/fixed value missing or invalid\0"
                                as *const u8 as *const i8,
                        );
                    }
                } else {
                    lb = 0.0f64;
                }
                if type_0 == 3 as i32 || type_0 == 4 as i32 {
                    _glp_dmx_read_field(csa);
                    if _glp_str2num(((*csa).field).as_mut_ptr(), &mut ub) != 0 as i32 {
                        _glp_dmx_error(
                            csa,
                            b"row upper bound missing or invalid\0" as *const u8
                                as *const i8,
                        );
                    }
                } else {
                    ub = 0.0f64;
                }
                if *rf.offset(i as isize) as i32 & 0x1 as i32 != 0 {
                    _glp_dmx_error(
                        csa,
                        b"duplicate row descriptor\0" as *const u8 as *const i8,
                    );
                }
                glp_set_row_bnds(P, i, type_0, lb, ub);
                let ref mut fresh0 = *rf.offset(i as isize);
                *fresh0 = (*fresh0 as i32 | 0x1 as i32) as i8;
            } else if strcmp(
                ((*csa).field).as_mut_ptr(),
                b"j\0" as *const u8 as *const i8,
            ) == 0 as i32
            {
                let mut current_block_140: u64;
                _glp_dmx_read_field(csa);
                if _glp_str2int(((*csa).field).as_mut_ptr(), &mut j) != 0 as i32 {
                    _glp_dmx_error(
                        csa,
                        b"column number missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                if !(1 as i32 <= j && j <= n) {
                    _glp_dmx_error(
                        csa,
                        b"column number out of range\0" as *const u8 as *const i8,
                    );
                }
                if mip == 0 {
                    kind = 1 as i32;
                    current_block_140 = 6540614962658479183;
                } else {
                    _glp_dmx_read_field(csa);
                    if strcmp(
                        ((*csa).field).as_mut_ptr(),
                        b"c\0" as *const u8 as *const i8,
                    ) == 0 as i32
                    {
                        kind = 1 as i32;
                        current_block_140 = 6540614962658479183;
                    } else if strcmp(
                        ((*csa).field).as_mut_ptr(),
                        b"i\0" as *const u8 as *const i8,
                    ) == 0 as i32
                    {
                        kind = 2 as i32;
                        current_block_140 = 6540614962658479183;
                    } else if strcmp(
                        ((*csa).field).as_mut_ptr(),
                        b"b\0" as *const u8 as *const i8,
                    ) == 0 as i32
                    {
                        kind = 2 as i32;
                        type_0 = 4 as i32;
                        lb = 0.0f64;
                        ub = 1.0f64;
                        current_block_140 = 4404413600511016096;
                    } else {
                        _glp_dmx_error(
                            csa,
                            b"column kind missing or invalid\0" as *const u8 as *const i8,
                        );
                        current_block_140 = 6540614962658479183;
                    }
                }
                match current_block_140 {
                    6540614962658479183 => {
                        _glp_dmx_read_field(csa);
                        if strcmp(
                            ((*csa).field).as_mut_ptr(),
                            b"f\0" as *const u8 as *const i8,
                        ) == 0 as i32
                        {
                            type_0 = 1 as i32;
                        } else if strcmp(
                            ((*csa).field).as_mut_ptr(),
                            b"l\0" as *const u8 as *const i8,
                        ) == 0 as i32
                        {
                            type_0 = 2 as i32;
                        } else if strcmp(
                            ((*csa).field).as_mut_ptr(),
                            b"u\0" as *const u8 as *const i8,
                        ) == 0 as i32
                        {
                            type_0 = 3 as i32;
                        } else if strcmp(
                            ((*csa).field).as_mut_ptr(),
                            b"d\0" as *const u8 as *const i8,
                        ) == 0 as i32
                        {
                            type_0 = 4 as i32;
                        } else if strcmp(
                            ((*csa).field).as_mut_ptr(),
                            b"s\0" as *const u8 as *const i8,
                        ) == 0 as i32
                        {
                            type_0 = 5 as i32;
                        } else {
                            _glp_dmx_error(
                                csa,
                                b"column type missing or invalid\0" as *const u8
                                    as *const i8,
                            );
                        }
                        if type_0 == 2 as i32 || type_0 == 4 as i32 || type_0 == 5 as i32
                        {
                            _glp_dmx_read_field(csa);
                            if _glp_str2num(((*csa).field).as_mut_ptr(), &mut lb)
                                != 0 as i32
                            {
                                _glp_dmx_error(
                                    csa,
                                    b"column lower bound/fixed value missing or invalid\0"
                                        as *const u8 as *const i8,
                                );
                            }
                        } else {
                            lb = 0.0f64;
                        }
                        if type_0 == 3 as i32 || type_0 == 4 as i32 {
                            _glp_dmx_read_field(csa);
                            if _glp_str2num(((*csa).field).as_mut_ptr(), &mut ub)
                                != 0 as i32
                            {
                                _glp_dmx_error(
                                    csa,
                                    b"column upper bound missing or invalid\0" as *const u8
                                        as *const i8,
                                );
                            }
                        } else {
                            ub = 0.0f64;
                        }
                    }
                    _ => {}
                }
                if *cf.offset(j as isize) as i32 & 0x1 as i32 != 0 {
                    _glp_dmx_error(
                        csa,
                        b"duplicate column descriptor\0" as *const u8 as *const i8,
                    );
                }
                glp_set_col_kind(P, j, kind);
                glp_set_col_bnds(P, j, type_0, lb, ub);
                let ref mut fresh1 = *cf.offset(j as isize);
                *fresh1 = (*fresh1 as i32 | 0x1 as i32) as i8;
            } else if strcmp(
                ((*csa).field).as_mut_ptr(),
                b"a\0" as *const u8 as *const i8,
            ) == 0 as i32
            {
                _glp_dmx_read_field(csa);
                if _glp_str2int(((*csa).field).as_mut_ptr(), &mut i) != 0 as i32 {
                    _glp_dmx_error(
                        csa,
                        b"row number missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                if !(0 as i32 <= i && i <= m) {
                    _glp_dmx_error(
                        csa,
                        b"row number out of range\0" as *const u8 as *const i8,
                    );
                }
                _glp_dmx_read_field(csa);
                if _glp_str2int(((*csa).field).as_mut_ptr(), &mut j) != 0 as i32 {
                    _glp_dmx_error(
                        csa,
                        b"column number missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                if !((if i == 0 as i32 { 0 as i32 } else { 1 as i32 }) <= j && j <= n) {
                    _glp_dmx_error(
                        csa,
                        b"column number out of range\0" as *const u8 as *const i8,
                    );
                }
                _glp_dmx_read_field(csa);
                if i == 0 as i32 {
                    if _glp_str2num(((*csa).field).as_mut_ptr(), &mut temp) != 0 as i32 {
                        _glp_dmx_error(
                            csa,
                            b"objective %s missing or invalid\0" as *const u8
                                as *const i8,
                            if j == 0 as i32 {
                                b"constant term\0" as *const u8 as *const i8
                            } else {
                                b"coefficient\0" as *const u8 as *const i8
                            },
                        );
                    }
                    if *cf.offset(j as isize) as i32 & 0x10 as i32 != 0 {
                        _glp_dmx_error(
                            csa,
                            b"duplicate objective %s\0" as *const u8 as *const i8,
                            if j == 0 as i32 {
                                b"constant term\0" as *const u8 as *const i8
                            } else {
                                b"coefficient\0" as *const u8 as *const i8
                            },
                        );
                    }
                    glp_set_obj_coef(P, j, temp);
                    let ref mut fresh2 = *cf.offset(j as isize);
                    *fresh2 = (*fresh2 as i32 | 0x10 as i32) as i8;
                } else {
                    if _glp_str2num(((*csa).field).as_mut_ptr(), &mut temp) != 0 as i32 {
                        _glp_dmx_error(
                            csa,
                            b"constraint coefficient missing or invalid\0" as *const u8
                                as *const i8,
                        );
                    }
                    if ne == nnz {
                        _glp_dmx_error(
                            csa,
                            b"too many constraint coefficient descriptors\0" as *const u8
                                as *const i8,
                        );
                    }
                    ne += 1;
                    *ln.offset(ne as isize) = (*csa).count;
                    *ia.offset(ne as isize) = i;
                    *ja.offset(ne as isize) = j;
                    *ar.offset(ne as isize) = temp;
                }
            } else if strcmp(
                ((*csa).field).as_mut_ptr(),
                b"n\0" as *const u8 as *const i8,
            ) == 0 as i32
            {
                _glp_dmx_read_field(csa);
                if strcmp(((*csa).field).as_mut_ptr(), b"p\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    _glp_dmx_read_field(csa);
                    if !((*P).name).is_null() {
                        _glp_dmx_error(
                            csa,
                            b"duplicate problem name\0" as *const u8 as *const i8,
                        );
                    }
                    glp_set_prob_name(P, ((*csa).field).as_mut_ptr());
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"z\0" as *const u8 as *const i8,
                ) == 0 as i32
                {
                    _glp_dmx_read_field(csa);
                    if !((*P).obj).is_null() {
                        _glp_dmx_error(
                            csa,
                            b"duplicate objective name\0" as *const u8 as *const i8,
                        );
                    }
                    glp_set_obj_name(P, ((*csa).field).as_mut_ptr());
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"i\0" as *const u8 as *const i8,
                ) == 0 as i32
                {
                    _glp_dmx_read_field(csa);
                    if _glp_str2int(((*csa).field).as_mut_ptr(), &mut i) != 0 as i32 {
                        _glp_dmx_error(
                            csa,
                            b"row number missing or invalid\0" as *const u8 as *const i8,
                        );
                    }
                    if !(1 as i32 <= i && i <= m) {
                        _glp_dmx_error(
                            csa,
                            b"row number out of range\0" as *const u8 as *const i8,
                        );
                    }
                    _glp_dmx_read_field(csa);
                    if !((**((*P).row).offset(i as isize)).name).is_null() {
                        _glp_dmx_error(
                            csa,
                            b"duplicate row name\0" as *const u8 as *const i8,
                        );
                    }
                    glp_set_row_name(P, i, ((*csa).field).as_mut_ptr());
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"j\0" as *const u8 as *const i8,
                ) == 0 as i32
                {
                    _glp_dmx_read_field(csa);
                    if _glp_str2int(((*csa).field).as_mut_ptr(), &mut j) != 0 as i32 {
                        _glp_dmx_error(
                            csa,
                            b"column number missing or invalid\0" as *const u8
                                as *const i8,
                        );
                    }
                    if !(1 as i32 <= j && j <= n) {
                        _glp_dmx_error(
                            csa,
                            b"column number out of range\0" as *const u8 as *const i8,
                        );
                    }
                    _glp_dmx_read_field(csa);
                    if !((**((*P).col).offset(j as isize)).name).is_null() {
                        _glp_dmx_error(
                            csa,
                            b"duplicate column name\0" as *const u8 as *const i8,
                        );
                    }
                    glp_set_col_name(P, j, ((*csa).field).as_mut_ptr());
                } else {
                    _glp_dmx_error(
                        csa,
                        b"object designator missing or invalid\0" as *const u8
                            as *const i8,
                    );
                }
            } else {
                if strcmp(((*csa).field).as_mut_ptr(), b"e\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    break;
                }
                _glp_dmx_error(
                    csa,
                    b"line designator missing or invalid\0" as *const u8 as *const i8,
                );
            }
            _glp_dmx_end_of_line(csa);
        }
        if ne < nnz {
            _glp_dmx_error(
                csa,
                b"too few constraint coefficient descriptors\0" as *const u8 as *const i8,
            );
        }
        (ne == nnz
            || {
                glp_assert_(
                    b"ne == nnz\0" as *const u8 as *const i8,
                    b"api/rdprob.c\0" as *const u8 as *const i8,
                    322 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k = glp_check_dup(m, n, ne, ia as *const i32, ja as *const i32);
        (0 as i32 <= k && k <= nnz
            || {
                glp_assert_(
                    b"0 <= k && k <= nnz\0" as *const u8 as *const i8,
                    b"api/rdprob.c\0" as *const u8 as *const i8,
                    324 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if k > 0 as i32 {
            (*csa).count = *ln.offset(k as isize);
            _glp_dmx_error(
                csa,
                b"duplicate constraint coefficient\0" as *const u8 as *const i8,
            );
        }
        glp_load_matrix(
            P,
            ne,
            ia as *const i32,
            ja as *const i32,
            ar as *const libc::c_double,
        );
        if !((*P).name).is_null() {
            glp_printf(b"Problem: %s\n\0" as *const u8 as *const i8, (*P).name);
        }
        if !((*P).obj).is_null() {
            glp_printf(b"Objective: %s\n\0" as *const u8 as *const i8, (*P).obj);
        }
        glp_printf(
            b"%d row%s, %d column%s, %d non-zero%s\n\0" as *const u8 as *const i8,
            m,
            if m == 1 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"s\0" as *const u8 as *const i8
            },
            n,
            if n == 1 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"s\0" as *const u8 as *const i8
            },
            nnz,
            if nnz == 1 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"s\0" as *const u8 as *const i8
            },
        );
        if glp_get_num_int(P) > 0 as i32 {
            let mut ni: i32 = glp_get_num_int(P);
            let mut nb: i32 = glp_get_num_bin(P);
            if ni == 1 as i32 {
                if nb == 0 as i32 {
                    glp_printf(b"One variable is integer\n\0" as *const u8 as *const i8);
                } else {
                    glp_printf(b"One variable is binary\n\0" as *const u8 as *const i8);
                }
            } else {
                glp_printf(b"%d integer variables, \0" as *const u8 as *const i8, ni);
                if nb == 0 as i32 {
                    glp_printf(b"none\0" as *const u8 as *const i8);
                } else if nb == 1 as i32 {
                    glp_printf(b"one\0" as *const u8 as *const i8);
                } else if nb == ni {
                    glp_printf(b"all\0" as *const u8 as *const i8);
                } else {
                    glp_printf(b"%d\0" as *const u8 as *const i8, nb);
                }
                glp_printf(
                    b" of which %s binary\n\0" as *const u8 as *const i8,
                    if nb == 1 as i32 {
                        b"is\0" as *const u8 as *const i8
                    } else {
                        b"are\0" as *const u8 as *const i8
                    },
                );
            }
        }
        glp_printf(b"%d lines were read\n\0" as *const u8 as *const i8, (*csa).count);
        glp_sort_matrix(P);
        ret = 0 as i32;
    }
    if !((*csa).fp).is_null() {
        _glp_close((*csa).fp);
    }
    if !rf.is_null() {
        glp_free(rf as *mut libc::c_void);
    }
    if !cf.is_null() {
        glp_free(cf as *mut libc::c_void);
    }
    if !ln.is_null() {
        glp_free(ln as *mut libc::c_void);
    }
    if !ia.is_null() {
        glp_free(ia as *mut libc::c_void);
    }
    if !ja.is_null() {
        glp_free(ja as *mut libc::c_void);
    }
    if !ar.is_null() {
        glp_free(ar as *mut libc::c_void);
    }
    if ret != 0 {
        glp_erase_prob(P);
    }
    return ret;
}