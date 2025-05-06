#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type glp_file;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn _glp_close(f: *mut glp_file) -> i32;
    fn _glp_format(f: *mut glp_file, fmt: *const i8, _: ...) -> i32;
    fn _glp_getc(f: *mut glp_file) -> i32;
    fn _glp_ioerr(f: *mut glp_file) -> i32;
    fn _glp_open(name: *const i8, mode: *const i8) -> *mut glp_file;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn _glp_get_err_msg() -> *const i8;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_vprintf(fmt: *const i8, arg: ::core::ffi::VaList);
    fn glp_printf(fmt: *const i8, _: ...);
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn _glp_str2num(str: *const i8, val: *mut libc::c_double) -> i32;
    fn _glp_strspx(str: *mut i8) -> *mut i8;
    fn _glp_strtrim(str: *mut i8) -> *mut i8;
    fn glp_set_prob_name(P: *mut glp_prob, name: *const i8);
    fn glp_set_obj_name(P: *mut glp_prob, name: *const i8);
    fn glp_add_rows(P: *mut glp_prob, nrs: i32) -> i32;
    fn glp_add_cols(P: *mut glp_prob, ncs: i32) -> i32;
    fn glp_set_row_name(P: *mut glp_prob, i: i32, name: *const i8);
    fn glp_set_col_name(P: *mut glp_prob, j: i32, name: *const i8);
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_obj_coef(P: *mut glp_prob, j: i32, coef: libc::c_double);
    fn glp_set_mat_col(
        P: *mut glp_prob,
        j: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
    );
    fn glp_sort_matrix(P: *mut glp_prob);
    fn glp_del_rows(P: *mut glp_prob, nrs: i32, num: *const i32);
    fn glp_erase_prob(P: *mut glp_prob);
    fn glp_create_index(P: *mut glp_prob);
    fn glp_find_row(P: *mut glp_prob, name: *const i8) -> i32;
    fn glp_find_col(P: *mut glp_prob, name: *const i8) -> i32;
    fn glp_delete_index(P: *mut glp_prob);
    fn glp_set_col_kind(P: *mut glp_prob, j: i32, kind: i32);
    fn glp_get_num_int(P: *mut glp_prob) -> i32;
    fn glp_get_num_bin(P: *mut glp_prob) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type va_list = __builtin_va_list;
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_mpscp {
    pub blank: i32,
    pub obj_name: *mut i8,
    pub tol_mps: libc::c_double,
    pub foo_bar: [libc::c_double; 17],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub P: *mut glp_prob,
    pub deck: i32,
    pub parm: *const glp_mpscp,
    pub fname: *const i8,
    pub fp: *mut glp_file,
    pub jump: jmp_buf,
    pub recno: i32,
    pub recpos: i32,
    pub c: i32,
    pub fldno: i32,
    pub field: [i8; 256],
    pub w80: i32,
    pub wef: i32,
    pub obj_row: i32,
    pub work1: *mut libc::c_void,
    pub work2: *mut libc::c_void,
    pub work3: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa1 {
    pub P: *mut glp_prob,
    pub deck: i32,
    pub parm: *const glp_mpscp,
    pub field: [i8; 256],
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_init_mpscp(mut parm: *mut glp_mpscp) {
    (*parm).blank = '\0' as i32;
    (*parm).obj_name = 0 as *mut i8;
    (*parm).tol_mps = 1e-12f64;
}
unsafe extern "C" fn check_parm(mut func: *const i8, mut parm: *const glp_mpscp) {
    if !(0 as i32 <= (*parm).blank && (*parm).blank <= 0xff as i32)
        || !((*parm).blank == '\0' as i32
            || *(*__ctype_b_loc()).offset((*parm).blank as isize) as i32
                & _ISprint as i32 as libc::c_ushort as i32 != 0)
    {
        (glp_error_(b"api/mps.c\0" as *const u8 as *const i8, 57 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"%s: blank = 0x%02X; invalid parameter\n\0" as *const u8 as *const i8,
            func,
            (*parm).blank,
        );
    }
    if !(((*parm).obj_name).is_null() || strlen((*parm).obj_name) <= 255 as i32 as u64) {
        (glp_error_(b"api/mps.c\0" as *const u8 as *const i8, 60 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"%s: obj_name = \"%.12s...\"; parameter too long\n\0" as *const u8
                as *const i8,
            func,
            (*parm).obj_name,
        );
    }
    if !(0.0f64 <= (*parm).tol_mps && (*parm).tol_mps < 1.0f64) {
        (glp_error_(b"api/mps.c\0" as *const u8 as *const i8, 63 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"%s: tol_mps = %g; invalid parameter\n\0" as *const u8 as *const i8,
            func,
            (*parm).tol_mps,
        );
    }
}
unsafe extern "C" fn error(mut csa_0: *mut csa, mut fmt: *const i8, mut args: ...) {
    let mut arg: ::core::ffi::VaListImpl;
    glp_printf(b"%s:%d: \0" as *const u8 as *const i8, (*csa_0).fname, (*csa_0).recno);
    arg = args.clone();
    glp_vprintf(fmt, arg.as_va_list());
    longjmp(((*csa_0).jump).as_mut_ptr(), 1 as i32);
}
unsafe extern "C" fn warning(mut csa_0: *mut csa, mut fmt: *const i8, mut args: ...) {
    let mut arg: ::core::ffi::VaListImpl;
    glp_printf(
        b"%s:%d: warning: \0" as *const u8 as *const i8,
        (*csa_0).fname,
        (*csa_0).recno,
    );
    arg = args.clone();
    glp_vprintf(fmt, arg.as_va_list());
}
unsafe extern "C" fn read_char(mut csa_0: *mut csa) {
    let mut current_block: u64;
    let mut c: i32 = 0;
    if (*csa_0).c == '\n' as i32 {
        (*csa_0).recno += 1;
        (*csa_0).recno;
        (*csa_0).recpos = 0 as i32;
    }
    (*csa_0).recpos += 1;
    (*csa_0).recpos;
    loop {
        c = _glp_getc((*csa_0).fp);
        if c < 0 as i32 {
            if _glp_ioerr((*csa_0).fp) != 0 {
                error(
                    csa_0,
                    b"read error - %s\n\0" as *const u8 as *const i8,
                    _glp_get_err_msg(),
                );
            } else if (*csa_0).c == '\n' as i32 {
                error(csa_0, b"unexpected end of file\n\0" as *const u8 as *const i8);
            } else {
                warning(
                    csa_0,
                    b"missing final end of line\n\0" as *const u8 as *const i8,
                );
                c = '\n' as i32;
            }
            current_block = 11194104282611034094;
            break;
        } else {
            if c == '\n' as i32 {
                current_block = 11194104282611034094;
                break;
            }
            if (*csa_0).c == '\r' as i32 {
                c = '\r' as i32;
                current_block = 15007878687160444872;
                break;
            } else if (*csa_0).deck != 0 && c == '\r' as i32 {
                (*csa_0).c = '\r' as i32;
            } else if c == ' ' as i32 {
                current_block = 11194104282611034094;
                break;
            } else {
                current_block = 13056961889198038528;
                break;
            }
        }
    }
    match current_block {
        13056961889198038528 => {
            if *(*__ctype_b_loc()).offset(c as isize) as i32
                & _ISspace as i32 as libc::c_ushort as i32 != 0
            {
                if (*csa_0).deck != 0 {
                    current_block = 15007878687160444872;
                } else {
                    current_block = 2370887241019905314;
                }
            } else {
                if *(*__ctype_b_loc()).offset(c as isize) as i32
                    & _IScntrl as i32 as libc::c_ushort as i32 != 0
                {
                    error(
                        csa_0,
                        b"invalid control character 0x%02X\n\0" as *const u8
                            as *const i8,
                        c,
                    );
                }
                current_block = 11194104282611034094;
            }
        }
        _ => {}
    }
    match current_block {
        15007878687160444872 => {
            error(
                csa_0,
                b"in fixed MPS format white-space character 0x%02X is not allowed\n\0"
                    as *const u8 as *const i8,
                c,
            );
            current_block = 2370887241019905314;
        }
        _ => {}
    }
    match current_block {
        2370887241019905314 => {
            c = ' ' as i32;
        }
        _ => {}
    }
    if (*csa_0).deck != 0 && (*csa_0).recpos == 81 as i32 && c != '\n' as i32
        && (*csa_0).w80 < 1 as i32
    {
        warning(
            csa_0,
            b"in fixed MPS format record must not be longer than 80 characters\n\0"
                as *const u8 as *const i8,
        );
        (*csa_0).w80 += 1;
        (*csa_0).w80;
    }
    (*csa_0).c = c;
}
unsafe extern "C" fn indicator(mut csa_0: *mut csa, mut name: i32) -> i32 {
    let mut ret: i32 = 0;
    (*csa_0).fldno = 0 as i32;
    loop {
        ((*csa_0).c == '\n' as i32
            || {
                glp_assert_(
                    b"csa->c == '\\n'\0" as *const u8 as *const i8,
                    b"api/mps.c\0" as *const u8 as *const i8,
                    210 as i32,
                );
                1 as i32 != 0
            }) as i32;
        read_char(csa_0);
        if (*csa_0).c == ' ' as i32 || (*csa_0).c == '\n' as i32 {
            ret = 0 as i32;
            break;
        } else if (*csa_0).c == '*' as i32 {
            while (*csa_0).c != '\n' as i32 {
                read_char(csa_0);
            }
        } else {
            let mut len: i32 = 0 as i32;
            while (*csa_0).c != ' ' as i32 && (*csa_0).c != '\n' as i32
                && len < 12 as i32
            {
                let fresh0 = len;
                len = len + 1;
                (*csa_0).field[fresh0 as usize] = (*csa_0).c as i8;
                read_char(csa_0);
            }
            (*csa_0).field[len as usize] = '\0' as i32 as i8;
            if !(strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"NAME\0" as *const u8 as *const i8,
            ) == 0 as i32
                || strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"ROWS\0" as *const u8 as *const i8,
                ) == 0 as i32
                || strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"COLUMNS\0" as *const u8 as *const i8,
                ) == 0 as i32
                || strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"RHS\0" as *const u8 as *const i8,
                ) == 0 as i32
                || strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"RANGES\0" as *const u8 as *const i8,
                ) == 0 as i32
                || strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"BOUNDS\0" as *const u8 as *const i8,
                ) == 0 as i32
                || strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"ENDATA\0" as *const u8 as *const i8,
                ) == 0 as i32)
            {
                error(csa_0, b"invalid indicator record\n\0" as *const u8 as *const i8);
            }
            if name == 0 {
                while (*csa_0).c != '\n' as i32 {
                    read_char(csa_0);
                }
            }
            ret = 1 as i32;
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn read_field(mut csa_0: *mut csa) {
    (*csa_0).fldno += 1;
    (*csa_0).fldno;
    if (*csa_0).deck != 0 {
        let mut beg: i32 = 0;
        let mut end: i32 = 0;
        let mut pos: i32 = 0;
        if (*csa_0).fldno == 1 as i32 {
            beg = 2 as i32;
            end = 3 as i32;
        } else if (*csa_0).fldno == 2 as i32 {
            beg = 5 as i32;
            end = 12 as i32;
        } else if (*csa_0).fldno == 3 as i32 {
            beg = 15 as i32;
            end = 22 as i32;
        } else if (*csa_0).fldno == 4 as i32 {
            beg = 25 as i32;
            end = 36 as i32;
        } else if (*csa_0).fldno == 5 as i32 {
            beg = 40 as i32;
            end = 47 as i32;
        } else if (*csa_0).fldno == 6 as i32 {
            beg = 50 as i32;
            end = 61 as i32;
        } else {
            (csa_0 != csa_0
                || {
                    glp_assert_(
                        b"csa != csa\0" as *const u8 as *const i8,
                        b"api/mps.c\0" as *const u8 as *const i8,
                        267 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        if (*csa_0).c != '\n' as i32 {
            pos = (*csa_0).recpos;
            while (*csa_0).recpos < beg {
                if !((*csa_0).c == ' ' as i32) {
                    if (*csa_0).c == '\n' as i32 {
                        break;
                    }
                    error(
                        csa_0,
                        b"in fixed MPS format positions %d-%d must be blank\n\0"
                            as *const u8 as *const i8,
                        pos,
                        beg - 1 as i32,
                    );
                }
                read_char(csa_0);
            }
        }
        if ((*csa_0).fldno == 3 as i32 || (*csa_0).fldno == 5 as i32)
            && (*csa_0).c == '$' as i32
        {
            while (*csa_0).c != '\n' as i32 {
                read_char(csa_0);
            }
        }
        pos = beg;
        while pos <= end {
            if (*csa_0).c == '\n' as i32 {
                break;
            }
            (*csa_0).field[(pos - beg) as usize] = (*csa_0).c as i8;
            read_char(csa_0);
            pos += 1;
            pos;
        }
        (*csa_0).field[(pos - beg) as usize] = '\0' as i32 as i8;
        _glp_strtrim(((*csa_0).field).as_mut_ptr());
        if (*csa_0).fldno == 6 as i32 && (*csa_0).c != '\n' as i32 {
            while (*csa_0).recpos <= 72 as i32 {
                if !((*csa_0).c == ' ' as i32) {
                    if (*csa_0).c == '\n' as i32 {
                        break;
                    }
                    error(
                        csa_0,
                        b"in fixed MPS format positions 62-72 must be blank\n\0"
                            as *const u8 as *const i8,
                    );
                }
                read_char(csa_0);
            }
            while (*csa_0).c != '\n' as i32 {
                read_char(csa_0);
            }
        }
    } else {
        let mut len: i32 = 0;
        while (*csa_0).c == ' ' as i32 {
            read_char(csa_0);
        }
        if (*csa_0).c == '$' as i32 {
            while (*csa_0).c != '\n' as i32 {
                read_char(csa_0);
            }
        }
        len = 0 as i32;
        while !((*csa_0).c == ' ' as i32 || (*csa_0).c == '\n' as i32) {
            if len == 255 as i32 {
                let fresh1 = (*csa_0).fldno;
                (*csa_0).fldno = (*csa_0).fldno + 1;
                error(
                    csa_0,
                    b"length of field %d exceeds 255 characters\n\0" as *const u8
                        as *const i8,
                    fresh1,
                );
            }
            let fresh2 = len;
            len = len + 1;
            (*csa_0).field[fresh2 as usize] = (*csa_0).c as i8;
            read_char(csa_0);
        }
        (*csa_0).field[len as usize] = '\0' as i32 as i8;
        if (*csa_0).fldno == 6 as i32 {
            while (*csa_0).c == ' ' as i32 {
                read_char(csa_0);
            }
            if (*csa_0).c != '$' as i32 && (*csa_0).c != '\n' as i32
                && (*csa_0).wef < 1 as i32
            {
                warning(
                    csa_0,
                    b"some extra field(s) detected beyond field 6; field(s) ignored\n\0"
                        as *const u8 as *const i8,
                );
                (*csa_0).wef += 1;
                (*csa_0).wef;
            }
            while (*csa_0).c != '\n' as i32 {
                read_char(csa_0);
            }
        }
    };
}
unsafe extern "C" fn patch_name(mut csa_0: *mut csa, mut name: *mut i8) {
    let mut blank: i32 = (*(*csa_0).parm).blank;
    if blank == '\0' as i32 {
        _glp_strspx(name);
    } else {
        while *name as i32 != '\0' as i32 {
            if *name as i32 == ' ' as i32 {
                *name = blank as i8;
            }
            name = name.offset(1);
            name;
        }
    };
}
unsafe extern "C" fn read_number(mut csa_0: *mut csa) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut s: *mut i8 = 0 as *mut i8;
    read_field(csa_0);
    ((*csa_0).fldno == 4 as i32 || (*csa_0).fldno == 6 as i32
        || {
            glp_assert_(
                b"csa->fldno == 4 || csa->fldno == 6\0" as *const u8 as *const i8,
                b"api/mps.c\0" as *const u8 as *const i8,
                370 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
        error(
            csa_0,
            b"missing numeric value in field %d\n\0" as *const u8 as *const i8,
            (*csa_0).fldno,
        );
    }
    s = ((*csa_0).field).as_mut_ptr();
    while *s as i32 == ' ' as i32 {
        s = s.offset(1);
        s;
    }
    if _glp_str2num(s, &mut x) != 0 as i32 {
        error(
            csa_0,
            b"cannot convert '%s' to floating-point number\n\0" as *const u8
                as *const i8,
            s,
        );
    }
    return x;
}
unsafe extern "C" fn skip_field(mut csa_0: *mut csa) {
    read_field(csa_0);
    if (*csa_0).field[0 as i32 as usize] as i32 != '\0' as i32 {
        error(
            csa_0,
            b"field %d must be blank\n\0" as *const u8 as *const i8,
            (*csa_0).fldno,
        );
    }
}
unsafe extern "C" fn read_name(mut csa_0: *mut csa) {
    if !(indicator(csa_0, 1 as i32) != 0
        && strcmp(((*csa_0).field).as_mut_ptr(), b"NAME\0" as *const u8 as *const i8)
            == 0 as i32)
    {
        error(csa_0, b"missing NAME indicator record\n\0" as *const u8 as *const i8);
    }
    (*csa_0).fldno = 2 as i32;
    read_field(csa_0);
    patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
    if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
        warning(csa_0, b"missing model name in field 3\n\0" as *const u8 as *const i8);
    } else {
        glp_set_prob_name((*csa_0).P, ((*csa_0).field).as_mut_ptr());
    }
    while (*csa_0).c != '\n' as i32 {
        read_char(csa_0);
    }
}
unsafe extern "C" fn read_rows(mut csa_0: *mut csa) {
    let mut i: i32 = 0;
    let mut type_0: i32 = 0;
    while !(indicator(csa_0, 0 as i32) != 0) {
        read_field(csa_0);
        _glp_strspx(((*csa_0).field).as_mut_ptr());
        if strcmp(((*csa_0).field).as_mut_ptr(), b"N\0" as *const u8 as *const i8)
            == 0 as i32
        {
            type_0 = 1 as i32;
        } else if strcmp(((*csa_0).field).as_mut_ptr(), b"G\0" as *const u8 as *const i8)
            == 0 as i32
        {
            type_0 = 2 as i32;
        } else if strcmp(((*csa_0).field).as_mut_ptr(), b"L\0" as *const u8 as *const i8)
            == 0 as i32
        {
            type_0 = 3 as i32;
        } else if strcmp(((*csa_0).field).as_mut_ptr(), b"E\0" as *const u8 as *const i8)
            == 0 as i32
        {
            type_0 = 5 as i32;
        } else if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
            error(csa_0, b"missing row type in field 1\n\0" as *const u8 as *const i8);
        } else {
            error(csa_0, b"invalid row type in field 1\n\0" as *const u8 as *const i8);
        }
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
            error(csa_0, b"missing row name in field 2\n\0" as *const u8 as *const i8);
        }
        if glp_find_row((*csa_0).P, ((*csa_0).field).as_mut_ptr()) != 0 as i32 {
            error(
                csa_0,
                b"row '%s' multiply specified\n\0" as *const u8 as *const i8,
                ((*csa_0).field).as_mut_ptr(),
            );
        }
        i = glp_add_rows((*csa_0).P, 1 as i32);
        glp_set_row_name((*csa_0).P, i, ((*csa_0).field).as_mut_ptr());
        glp_set_row_bnds((*csa_0).P, i, type_0, 0.0f64, 0.0f64);
        skip_field(csa_0);
        skip_field(csa_0);
        skip_field(csa_0);
        skip_field(csa_0);
    }
}
unsafe extern "C" fn read_columns(mut csa_0: *mut csa) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut f: i32 = 0;
    let mut len: i32 = 0;
    let mut kind: i32 = 1 as i32;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut aij: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut name: [i8; 256] = [0; 256];
    let mut flag: *mut i8 = 0 as *mut i8;
    ind = glp_alloc(
        1 as i32 + (*(*csa_0).P).m,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*csa_0).work1 = ind as *mut libc::c_void;
    val = glp_alloc(
        1 as i32 + (*(*csa_0).P).m,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    (*csa_0).work2 = val as *mut libc::c_void;
    flag = glp_alloc(
        1 as i32 + (*(*csa_0).P).m,
        ::core::mem::size_of::<i8>() as u64 as i32,
    ) as *mut i8;
    (*csa_0).work3 = flag as *mut libc::c_void;
    memset(
        &mut *flag.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
        0 as i32,
        (*(*csa_0).P).m as u64,
    );
    j = 0 as i32;
    len = 0 as i32;
    while !(indicator(csa_0, 0 as i32) != 0) {
        if (*csa_0).deck != 0 {
            read_field(csa_0);
            if (*csa_0).field[0 as i32 as usize] as i32 != '\0' as i32 {
                error(csa_0, b"field 1 must be blank\n\0" as *const u8 as *const i8);
            }
        } else {
            (*csa_0).fldno += 1;
            (*csa_0).fldno;
        }
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        strcpy(name.as_mut_ptr(), ((*csa_0).field).as_mut_ptr());
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        if strcmp(((*csa_0).field).as_mut_ptr(), b"'MARKER'\0" as *const u8 as *const i8)
            == 0 as i32
        {
            if (*csa_0).deck != 0 {
                read_field(csa_0);
                if (*csa_0).field[0 as i32 as usize] as i32 != '\0' as i32 {
                    error(csa_0, b"field 4 must be blank\n\0" as *const u8 as *const i8);
                }
            } else {
                (*csa_0).fldno += 1;
                (*csa_0).fldno;
            }
            read_field(csa_0);
            patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
            if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"'INTORG'\0" as *const u8 as *const i8,
            ) == 0 as i32
            {
                kind = 2 as i32;
            } else if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"'INTEND'\0" as *const u8 as *const i8,
            ) == 0 as i32
            {
                kind = 1 as i32;
            } else if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
                error(
                    csa_0,
                    b"missing keyword in field 5\n\0" as *const u8 as *const i8,
                );
            } else {
                error(
                    csa_0,
                    b"invalid keyword in field 5\n\0" as *const u8 as *const i8,
                );
            }
            skip_field(csa_0);
        } else {
            if name[0 as i32 as usize] as i32 == '\0' as i32 {
                if j == 0 as i32 {
                    error(
                        csa_0,
                        b"missing column name in field 2\n\0" as *const u8 as *const i8,
                    );
                }
            } else if j != 0 as i32
                && strcmp(
                    name.as_mut_ptr(),
                    (**((*(*csa_0).P).col).offset(j as isize)).name,
                ) == 0 as i32
            {
                (j != 0 as i32
                    || {
                        glp_assert_(
                            b"j != 0\0" as *const u8 as *const i8,
                            b"api/mps.c\0" as *const u8 as *const i8,
                            503 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            } else {
                if j != 0 as i32 {
                    glp_set_mat_col(
                        (*csa_0).P,
                        j,
                        len,
                        ind as *const i32,
                        val as *const libc::c_double,
                    );
                    while len > 0 as i32 {
                        let fresh3 = len;
                        len = len - 1;
                        *flag.offset(*ind.offset(fresh3 as isize) as isize) = 0 as i32
                            as i8;
                    }
                }
                if glp_find_col((*csa_0).P, name.as_mut_ptr()) != 0 as i32 {
                    error(
                        csa_0,
                        b"column '%s' multiply specified\n\0" as *const u8 as *const i8,
                        name.as_mut_ptr(),
                    );
                }
                j = glp_add_cols((*csa_0).P, 1 as i32);
                glp_set_col_name((*csa_0).P, j, name.as_mut_ptr());
                glp_set_col_kind((*csa_0).P, j, kind);
                if kind == 1 as i32 {
                    glp_set_col_bnds((*csa_0).P, j, 2 as i32, 0.0f64, 0.0f64);
                } else if kind == 2 as i32 {
                    glp_set_col_bnds((*csa_0).P, j, 4 as i32, 0.0f64, 1.0f64);
                } else {
                    (kind != kind
                        || {
                            glp_assert_(
                                b"kind != kind\0" as *const u8 as *const i8,
                                b"api/mps.c\0" as *const u8 as *const i8,
                                522 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
            let mut current_block_66: u64;
            f = 3 as i32;
            while f <= 5 as i32 {
                if f == 3 as i32 {
                    if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
                        error(
                            csa_0,
                            b"missing row name in field 3\n\0" as *const u8 as *const i8,
                        );
                    }
                    current_block_66 = 5892776923941496671;
                } else {
                    read_field(csa_0);
                    patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
                    if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
                        skip_field(csa_0);
                        current_block_66 = 9441801433784995173;
                    } else {
                        current_block_66 = 5892776923941496671;
                    }
                }
                match current_block_66 {
                    5892776923941496671 => {
                        i = glp_find_row((*csa_0).P, ((*csa_0).field).as_mut_ptr());
                        if i == 0 as i32 {
                            error(
                                csa_0,
                                b"row '%s' not found\n\0" as *const u8 as *const i8,
                                ((*csa_0).field).as_mut_ptr(),
                            );
                        }
                        if *flag.offset(i as isize) != 0 {
                            error(
                                csa_0,
                                b"duplicate coefficient in row '%s'\n\0" as *const u8
                                    as *const i8,
                                ((*csa_0).field).as_mut_ptr(),
                            );
                        }
                        aij = read_number(csa_0);
                        if fabs(aij) < (*(*csa_0).parm).tol_mps {
                            aij = 0.0f64;
                        }
                        len += 1;
                        len;
                        *ind.offset(len as isize) = i;
                        *val.offset(len as isize) = aij;
                        *flag.offset(i as isize) = 1 as i32 as i8;
                    }
                    _ => {}
                }
                f += 2 as i32;
            }
        }
    }
    if j != 0 as i32 {
        glp_set_mat_col(
            (*csa_0).P,
            j,
            len,
            ind as *const i32,
            val as *const libc::c_double,
        );
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(flag as *mut libc::c_void);
    (*csa_0).work3 = 0 as *mut libc::c_void;
    (*csa_0).work2 = (*csa_0).work3;
    (*csa_0).work1 = (*csa_0).work2;
}
unsafe extern "C" fn read_rhs(mut csa_0: *mut csa) {
    let mut i: i32 = 0;
    let mut f: i32 = 0;
    let mut v: i32 = 0;
    let mut type_0: i32 = 0;
    let mut rhs: libc::c_double = 0.;
    let mut name: [i8; 256] = [0; 256];
    let mut flag: *mut i8 = 0 as *mut i8;
    flag = glp_alloc(
        1 as i32 + (*(*csa_0).P).m,
        ::core::mem::size_of::<i8>() as u64 as i32,
    ) as *mut i8;
    (*csa_0).work3 = flag as *mut libc::c_void;
    memset(
        &mut *flag.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
        0 as i32,
        (*(*csa_0).P).m as u64,
    );
    v = 0 as i32;
    while !(indicator(csa_0, 0 as i32) != 0) {
        if (*csa_0).deck != 0 {
            read_field(csa_0);
            if (*csa_0).field[0 as i32 as usize] as i32 != '\0' as i32 {
                error(csa_0, b"field 1 must be blank\n\0" as *const u8 as *const i8);
            }
        } else {
            (*csa_0).fldno += 1;
            (*csa_0).fldno;
        }
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        let mut current_block_17: u64;
        if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
            if v == 0 as i32 {
                warning(
                    csa_0,
                    b"missing RHS vector name in field 2\n\0" as *const u8 as *const i8,
                );
                current_block_17 = 6066440303963259102;
            } else {
                current_block_17 = 7175849428784450219;
            }
        } else if v != 0 as i32
            && strcmp(((*csa_0).field).as_mut_ptr(), name.as_mut_ptr()) == 0 as i32
        {
            (v != 0 as i32
                || {
                    glp_assert_(
                        b"v != 0\0" as *const u8 as *const i8,
                        b"api/mps.c\0" as *const u8 as *const i8,
                        592 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            current_block_17 = 7175849428784450219;
        } else {
            current_block_17 = 6066440303963259102;
        }
        match current_block_17 {
            6066440303963259102 => {
                if v != 0 as i32 {
                    error(
                        csa_0,
                        b"multiple RHS vectors not supported\n\0" as *const u8
                            as *const i8,
                    );
                }
                v += 1;
                v;
                strcpy(name.as_mut_ptr(), ((*csa_0).field).as_mut_ptr());
            }
            _ => {}
        }
        let mut current_block_39: u64;
        f = 3 as i32;
        while f <= 5 as i32 {
            read_field(csa_0);
            patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
            if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
                if f == 3 as i32 {
                    error(
                        csa_0,
                        b"missing row name in field 3\n\0" as *const u8 as *const i8,
                    );
                    current_block_39 = 4068382217303356765;
                } else {
                    skip_field(csa_0);
                    current_block_39 = 11584701595673473500;
                }
            } else {
                current_block_39 = 4068382217303356765;
            }
            match current_block_39 {
                4068382217303356765 => {
                    i = glp_find_row((*csa_0).P, ((*csa_0).field).as_mut_ptr());
                    if i == 0 as i32 {
                        error(
                            csa_0,
                            b"row '%s' not found\n\0" as *const u8 as *const i8,
                            ((*csa_0).field).as_mut_ptr(),
                        );
                    }
                    if *flag.offset(i as isize) != 0 {
                        error(
                            csa_0,
                            b"duplicate right-hand side for row '%s'\n\0" as *const u8
                                as *const i8,
                            ((*csa_0).field).as_mut_ptr(),
                        );
                    }
                    rhs = read_number(csa_0);
                    if fabs(rhs) < (*(*csa_0).parm).tol_mps {
                        rhs = 0.0f64;
                    }
                    type_0 = (**((*(*csa_0).P).row).offset(i as isize)).type_0;
                    if type_0 == 1 as i32 {
                        if i == (*csa_0).obj_row {
                            glp_set_obj_coef((*csa_0).P, 0 as i32, rhs);
                        } else if rhs != 0.0f64 {
                            warning(
                                csa_0,
                                b"non-zero right-hand side for free row '%s' ignored\n\0"
                                    as *const u8 as *const i8,
                                (**((*(*csa_0).P).row).offset(i as isize)).name,
                            );
                        }
                    } else {
                        glp_set_row_bnds((*csa_0).P, i, type_0, rhs, rhs);
                    }
                    *flag.offset(i as isize) = 1 as i32 as i8;
                }
                _ => {}
            }
            f += 2 as i32;
        }
    }
    glp_free(flag as *mut libc::c_void);
    (*csa_0).work3 = 0 as *mut libc::c_void;
}
unsafe extern "C" fn read_ranges(mut csa_0: *mut csa) {
    let mut i: i32 = 0;
    let mut f: i32 = 0;
    let mut v: i32 = 0;
    let mut type_0: i32 = 0;
    let mut rhs: libc::c_double = 0.;
    let mut rng: libc::c_double = 0.;
    let mut name: [i8; 256] = [0; 256];
    let mut flag: *mut i8 = 0 as *mut i8;
    flag = glp_alloc(
        1 as i32 + (*(*csa_0).P).m,
        ::core::mem::size_of::<i8>() as u64 as i32,
    ) as *mut i8;
    (*csa_0).work3 = flag as *mut libc::c_void;
    memset(
        &mut *flag.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
        0 as i32,
        (*(*csa_0).P).m as u64,
    );
    v = 0 as i32;
    while !(indicator(csa_0, 0 as i32) != 0) {
        if (*csa_0).deck != 0 {
            read_field(csa_0);
            if (*csa_0).field[0 as i32 as usize] as i32 != '\0' as i32 {
                error(csa_0, b"field 1 must be blank\n\0" as *const u8 as *const i8);
            }
        } else {
            (*csa_0).fldno += 1;
            (*csa_0).fldno;
        }
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        let mut current_block_17: u64;
        if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
            if v == 0 as i32 {
                warning(
                    csa_0,
                    b"missing RANGES vector name in field 2\n\0" as *const u8
                        as *const i8,
                );
                current_block_17 = 12444404377125944386;
            } else {
                current_block_17 = 7175849428784450219;
            }
        } else if v != 0 as i32
            && strcmp(((*csa_0).field).as_mut_ptr(), name.as_mut_ptr()) == 0 as i32
        {
            (v != 0 as i32
                || {
                    glp_assert_(
                        b"v != 0\0" as *const u8 as *const i8,
                        b"api/mps.c\0" as *const u8 as *const i8,
                        672 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            current_block_17 = 7175849428784450219;
        } else {
            current_block_17 = 12444404377125944386;
        }
        match current_block_17 {
            12444404377125944386 => {
                if v != 0 as i32 {
                    error(
                        csa_0,
                        b"multiple RANGES vectors not supported\n\0" as *const u8
                            as *const i8,
                    );
                }
                v += 1;
                v;
                strcpy(name.as_mut_ptr(), ((*csa_0).field).as_mut_ptr());
            }
            _ => {}
        }
        let mut current_block_50: u64;
        f = 3 as i32;
        while f <= 5 as i32 {
            read_field(csa_0);
            patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
            if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
                if f == 3 as i32 {
                    error(
                        csa_0,
                        b"missing row name in field 3\n\0" as *const u8 as *const i8,
                    );
                    current_block_50 = 4068382217303356765;
                } else {
                    skip_field(csa_0);
                    current_block_50 = 11584701595673473500;
                }
            } else {
                current_block_50 = 4068382217303356765;
            }
            match current_block_50 {
                4068382217303356765 => {
                    i = glp_find_row((*csa_0).P, ((*csa_0).field).as_mut_ptr());
                    if i == 0 as i32 {
                        error(
                            csa_0,
                            b"row '%s' not found\n\0" as *const u8 as *const i8,
                            ((*csa_0).field).as_mut_ptr(),
                        );
                    }
                    if *flag.offset(i as isize) != 0 {
                        error(
                            csa_0,
                            b"duplicate range for row '%s'\n\0" as *const u8
                                as *const i8,
                            ((*csa_0).field).as_mut_ptr(),
                        );
                    }
                    rng = read_number(csa_0);
                    if fabs(rng) < (*(*csa_0).parm).tol_mps {
                        rng = 0.0f64;
                    }
                    type_0 = (**((*(*csa_0).P).row).offset(i as isize)).type_0;
                    if type_0 == 1 as i32 {
                        warning(
                            csa_0,
                            b"range for free row '%s' ignored\n\0" as *const u8
                                as *const i8,
                            (**((*(*csa_0).P).row).offset(i as isize)).name,
                        );
                    } else if type_0 == 2 as i32 {
                        rhs = (**((*(*csa_0).P).row).offset(i as isize)).lb;
                        glp_set_row_bnds(
                            (*csa_0).P,
                            i,
                            if rng == 0.0f64 { 5 as i32 } else { 4 as i32 },
                            rhs,
                            rhs + fabs(rng),
                        );
                    } else if type_0 == 3 as i32 {
                        rhs = (**((*(*csa_0).P).row).offset(i as isize)).ub;
                        glp_set_row_bnds(
                            (*csa_0).P,
                            i,
                            if rng == 0.0f64 { 5 as i32 } else { 4 as i32 },
                            rhs - fabs(rng),
                            rhs,
                        );
                    } else if type_0 == 5 as i32 {
                        rhs = (**((*(*csa_0).P).row).offset(i as isize)).lb;
                        if rng > 0.0f64 {
                            glp_set_row_bnds((*csa_0).P, i, 4 as i32, rhs, rhs + rng);
                        } else if rng < 0.0f64 {
                            glp_set_row_bnds((*csa_0).P, i, 4 as i32, rhs + rng, rhs);
                        }
                    } else {
                        (type_0 != type_0
                            || {
                                glp_assert_(
                                    b"type != type\0" as *const u8 as *const i8,
                                    b"api/mps.c\0" as *const u8 as *const i8,
                                    732 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                    *flag.offset(i as isize) = 1 as i32 as i8;
                }
                _ => {}
            }
            f += 2 as i32;
        }
    }
    glp_free(flag as *mut libc::c_void);
    (*csa_0).work3 = 0 as *mut libc::c_void;
}
unsafe extern "C" fn read_bounds(mut csa_0: *mut csa) {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut j: i32 = 0;
    let mut v: i32 = 0;
    let mut mask: i32 = 0;
    let mut data: i32 = 0;
    let mut bnd: libc::c_double = 0.;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut type_0: [i8; 3] = [0; 3];
    let mut name: [i8; 256] = [0; 256];
    let mut flag: *mut i8 = 0 as *mut i8;
    flag = glp_alloc(
        1 as i32 + (*(*csa_0).P).n,
        ::core::mem::size_of::<i8>() as u64 as i32,
    ) as *mut i8;
    (*csa_0).work3 = flag as *mut libc::c_void;
    memset(
        &mut *flag.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
        0 as i32,
        (*(*csa_0).P).n as u64,
    );
    v = 0 as i32;
    while !(indicator(csa_0, 0 as i32) != 0) {
        read_field(csa_0);
        if strcmp(((*csa_0).field).as_mut_ptr(), b"LO\0" as *const u8 as *const i8)
            == 0 as i32
        {
            mask = 0x1 as i32;
            data = 1 as i32;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"UP\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            mask = 0x10 as i32;
            data = 1 as i32;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"FX\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            mask = 0x11 as i32;
            data = 1 as i32;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"FR\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            mask = 0x11 as i32;
            data = 0 as i32;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"MI\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            mask = 0x1 as i32;
            data = 0 as i32;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"PL\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            mask = 0x10 as i32;
            data = 0 as i32;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"LI\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            mask = 0x1 as i32;
            data = 1 as i32;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"UI\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            mask = 0x10 as i32;
            data = 1 as i32;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"BV\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            mask = 0x11 as i32;
            data = 0 as i32;
        } else if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
            error(csa_0, b"missing bound type in field 1\n\0" as *const u8 as *const i8);
        } else {
            error(csa_0, b"invalid bound type in field 1\n\0" as *const u8 as *const i8);
        }
        strcpy(type_0.as_mut_ptr(), ((*csa_0).field).as_mut_ptr());
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        let mut current_block_34: u64;
        if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
            if v == 0 as i32 {
                warning(
                    csa_0,
                    b"missing BOUNDS vector name in field 2\n\0" as *const u8
                        as *const i8,
                );
                current_block_34 = 13495302235225487424;
            } else {
                current_block_34 = 9520865839495247062;
            }
        } else if v != 0 as i32
            && strcmp(((*csa_0).field).as_mut_ptr(), name.as_mut_ptr()) == 0 as i32
        {
            (v != 0 as i32
                || {
                    glp_assert_(
                        b"v != 0\0" as *const u8 as *const i8,
                        b"api/mps.c\0" as *const u8 as *const i8,
                        790 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            current_block_34 = 9520865839495247062;
        } else {
            current_block_34 = 13495302235225487424;
        }
        match current_block_34 {
            13495302235225487424 => {
                if v != 0 as i32 {
                    error(
                        csa_0,
                        b"multiple BOUNDS vectors not supported\n\0" as *const u8
                            as *const i8,
                    );
                }
                v += 1;
                v;
                strcpy(name.as_mut_ptr(), ((*csa_0).field).as_mut_ptr());
            }
            _ => {}
        }
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        if (*csa_0).field[0 as i32 as usize] as i32 == '\0' as i32 {
            error(
                csa_0,
                b"missing column name in field 3\n\0" as *const u8 as *const i8,
            );
        }
        j = glp_find_col((*csa_0).P, ((*csa_0).field).as_mut_ptr());
        if j == 0 as i32 {
            error(
                csa_0,
                b"column '%s' not found\n\0" as *const u8 as *const i8,
                ((*csa_0).field).as_mut_ptr(),
            );
        }
        if *flag.offset(j as isize) as i32 & mask == 0x1 as i32 {
            error(
                csa_0,
                b"duplicate lower bound for column '%s'\n\0" as *const u8 as *const i8,
                ((*csa_0).field).as_mut_ptr(),
            );
        }
        if *flag.offset(j as isize) as i32 & mask == 0x10 as i32 {
            error(
                csa_0,
                b"duplicate upper bound for column '%s'\n\0" as *const u8 as *const i8,
                ((*csa_0).field).as_mut_ptr(),
            );
        }
        (*flag.offset(j as isize) as i32 & mask == 0 as i32
            || {
                glp_assert_(
                    b"(flag[j] & mask) == 0x00\0" as *const u8 as *const i8,
                    b"api/mps.c\0" as *const u8 as *const i8,
                    812 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if data != 0 {
            bnd = read_number(csa_0);
            if fabs(bnd) < (*(*csa_0).parm).tol_mps {
                bnd = 0.0f64;
            }
        } else {
            read_field(csa_0);
            bnd = 0.0f64;
        }
        col = *((*(*csa_0).P).col).offset(j as isize);
        if (*col).type_0 == 1 as i32 {
            lb = -1.7976931348623157e+308f64;
            ub = 1.7976931348623157e+308f64;
        } else if (*col).type_0 == 2 as i32 {
            lb = (*col).lb;
            ub = 1.7976931348623157e+308f64;
        } else if (*col).type_0 == 3 as i32 {
            lb = -1.7976931348623157e+308f64;
            ub = (*col).ub;
        } else if (*col).type_0 == 4 as i32 {
            lb = (*col).lb;
            ub = (*col).ub;
        } else if (*col).type_0 == 5 as i32 {
            ub = (*col).lb;
            lb = ub;
        } else {
            (col != col
                || {
                    glp_assert_(
                        b"col != col\0" as *const u8 as *const i8,
                        b"api/mps.c\0" as *const u8 as *const i8,
                        833 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        if strcmp(type_0.as_mut_ptr(), b"LO\0" as *const u8 as *const i8) == 0 as i32 {
            lb = bnd;
        } else if strcmp(type_0.as_mut_ptr(), b"UP\0" as *const u8 as *const i8)
            == 0 as i32
        {
            ub = bnd;
        } else if strcmp(type_0.as_mut_ptr(), b"FX\0" as *const u8 as *const i8)
            == 0 as i32
        {
            ub = bnd;
            lb = ub;
        } else if strcmp(type_0.as_mut_ptr(), b"FR\0" as *const u8 as *const i8)
            == 0 as i32
        {
            lb = -1.7976931348623157e+308f64;
            ub = 1.7976931348623157e+308f64;
        } else if strcmp(type_0.as_mut_ptr(), b"MI\0" as *const u8 as *const i8)
            == 0 as i32
        {
            lb = -1.7976931348623157e+308f64;
        } else if strcmp(type_0.as_mut_ptr(), b"PL\0" as *const u8 as *const i8)
            == 0 as i32
        {
            ub = 1.7976931348623157e+308f64;
        } else if strcmp(type_0.as_mut_ptr(), b"LI\0" as *const u8 as *const i8)
            == 0 as i32
        {
            glp_set_col_kind((*csa_0).P, j, 2 as i32);
            lb = ceil(bnd);
            if *flag.offset(j as isize) as i32 & 0x10 as i32 == 0 {
                ub = 1.7976931348623157e+308f64;
            }
        } else if strcmp(type_0.as_mut_ptr(), b"UI\0" as *const u8 as *const i8)
            == 0 as i32
        {
            glp_set_col_kind((*csa_0).P, j, 2 as i32);
            ub = floor(bnd);
        } else if strcmp(type_0.as_mut_ptr(), b"BV\0" as *const u8 as *const i8)
            == 0 as i32
        {
            glp_set_col_kind((*csa_0).P, j, 2 as i32);
            lb = 0.0f64;
            ub = 1.0f64;
        } else {
            (type_0.as_mut_ptr() != type_0.as_mut_ptr()
                || {
                    glp_assert_(
                        b"type != type\0" as *const u8 as *const i8,
                        b"api/mps.c\0" as *const u8 as *const i8,
                        866 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        if lb == -1.7976931348623157e+308f64 && ub == 1.7976931348623157e+308f64 {
            glp_set_col_bnds((*csa_0).P, j, 1 as i32, lb, ub);
        } else if ub == 1.7976931348623157e+308f64 {
            glp_set_col_bnds((*csa_0).P, j, 2 as i32, lb, ub);
        } else if lb == -1.7976931348623157e+308f64 {
            glp_set_col_bnds((*csa_0).P, j, 3 as i32, lb, ub);
        } else if lb != ub {
            glp_set_col_bnds((*csa_0).P, j, 4 as i32, lb, ub);
        } else {
            glp_set_col_bnds((*csa_0).P, j, 5 as i32, lb, ub);
        }
        let ref mut fresh4 = *flag.offset(j as isize);
        *fresh4 = (*fresh4 as i32 | mask as i8 as i32) as i8;
        skip_field(csa_0);
        skip_field(csa_0);
    }
    glp_free(flag as *mut libc::c_void);
    (*csa_0).work3 = 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn glp_read_mps(
    mut P: *mut glp_prob,
    mut fmt: i32,
    mut parm: *const glp_mpscp,
    mut fname: *const i8,
) -> i32 {
    let mut _parm: glp_mpscp = glp_mpscp {
        blank: 0,
        obj_name: 0 as *mut i8,
        tol_mps: 0.,
        foo_bar: [0.; 17],
    };
    let mut _csa: csa = csa {
        P: 0 as *mut glp_prob,
        deck: 0,
        parm: 0 as *const glp_mpscp,
        fname: 0 as *const i8,
        fp: 0 as *mut glp_file,
        jump: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        recno: 0,
        recpos: 0,
        c: 0,
        fldno: 0,
        field: [0; 256],
        w80: 0,
        wef: 0,
        obj_row: 0,
        work1: 0 as *mut libc::c_void,
        work2: 0 as *mut libc::c_void,
        work3: 0 as *mut libc::c_void,
    };
    let mut csa_0: *mut csa = &mut _csa;
    let mut ret: i32 = 0;
    glp_printf(
        b"Reading problem data from '%s'...\n\0" as *const u8 as *const i8,
        fname,
    );
    if !(fmt == 1 as i32 || fmt == 2 as i32) {
        (glp_error_(b"api/mps.c\0" as *const u8 as *const i8, 897 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_read_mps: fmt = %d; invalid parameter\n\0" as *const u8 as *const i8,
            fmt,
        );
    }
    if parm.is_null() {
        glp_init_mpscp(&mut _parm);
        parm = &mut _parm;
    }
    check_parm(b"glp_read_mps\0" as *const u8 as *const i8, parm);
    (*csa_0).P = P;
    (*csa_0).deck = (fmt == 1 as i32) as i32;
    (*csa_0).parm = parm;
    (*csa_0).fname = fname;
    (*csa_0).fp = 0 as *mut glp_file;
    if _setjmp(((*csa_0).jump).as_mut_ptr()) != 0 {
        ret = 1 as i32;
    } else {
        (*csa_0).recpos = 0 as i32;
        (*csa_0).recno = (*csa_0).recpos;
        (*csa_0).c = '\n' as i32;
        (*csa_0).fldno = 0 as i32;
        (*csa_0).field[0 as i32 as usize] = '\0' as i32 as i8;
        (*csa_0).wef = 0 as i32;
        (*csa_0).w80 = (*csa_0).wef;
        (*csa_0).obj_row = 0 as i32;
        (*csa_0).work3 = 0 as *mut libc::c_void;
        (*csa_0).work2 = (*csa_0).work3;
        (*csa_0).work1 = (*csa_0).work2;
        glp_erase_prob(P);
        glp_create_index(P);
        (*csa_0).fp = _glp_open(fname, b"r\0" as *const u8 as *const i8);
        if ((*csa_0).fp).is_null() {
            glp_printf(
                b"Unable to open '%s' - %s\n\0" as *const u8 as *const i8,
                fname,
                _glp_get_err_msg(),
            );
            ret = 1 as i32;
        } else {
            read_name(csa_0);
            if !((*P).name).is_null() {
                glp_printf(b"Problem: %s\n\0" as *const u8 as *const i8, (*P).name);
            }
            if !(indicator(csa_0, 0 as i32) != 0
                && strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"ROWS\0" as *const u8 as *const i8,
                ) == 0 as i32)
            {
                error(
                    csa_0,
                    b"missing ROWS indicator record\n\0" as *const u8 as *const i8,
                );
            }
            read_rows(csa_0);
            if ((*parm).obj_name).is_null()
                || *((*parm).obj_name).offset(0 as i32 as isize) as i32 == '\0' as i32
            {
                let mut i: i32 = 0;
                i = 1 as i32;
                while i <= (*P).m {
                    if (**((*P).row).offset(i as isize)).type_0 == 1 as i32 {
                        (*csa_0).obj_row = i;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
                if (*csa_0).obj_row == 0 as i32 {
                    warning(
                        csa_0,
                        b"unable to determine objective row\n\0" as *const u8
                            as *const i8,
                    );
                }
            } else {
                let mut i_0: i32 = 0;
                i_0 = 1 as i32;
                while i_0 <= (*P).m {
                    (!((**((*P).row).offset(i_0 as isize)).name).is_null()
                        || {
                            glp_assert_(
                                b"P->row[i]->name != NULL\0" as *const u8 as *const i8,
                                b"api/mps.c\0" as *const u8 as *const i8,
                                954 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if strcmp((*parm).obj_name, (**((*P).row).offset(i_0 as isize)).name)
                        == 0 as i32
                    {
                        (*csa_0).obj_row = i_0;
                        break;
                    } else {
                        i_0 += 1;
                        i_0;
                    }
                }
                if (*csa_0).obj_row == 0 as i32 {
                    error(
                        csa_0,
                        b"objective row '%s' not found\n\0" as *const u8 as *const i8,
                        (*parm).obj_name,
                    );
                }
            }
            if (*csa_0).obj_row != 0 as i32 {
                glp_set_obj_name(
                    P,
                    (**((*P).row).offset((*csa_0).obj_row as isize)).name,
                );
                glp_printf(b"Objective: %s\n\0" as *const u8 as *const i8, (*P).obj);
            }
            if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"COLUMNS\0" as *const u8 as *const i8,
            ) != 0 as i32
            {
                error(
                    csa_0,
                    b"missing COLUMNS indicator record\n\0" as *const u8 as *const i8,
                );
            }
            read_columns(csa_0);
            if (*csa_0).obj_row != 0 as i32 {
                let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
                aij = (**((*P).row).offset((*csa_0).obj_row as isize)).ptr;
                while !aij.is_null() {
                    glp_set_obj_coef(P, (*(*aij).col).j, (*aij).val);
                    aij = (*aij).r_next;
                }
            }
            if strcmp(((*csa_0).field).as_mut_ptr(), b"RHS\0" as *const u8 as *const i8)
                == 0 as i32
            {
                read_rhs(csa_0);
            }
            if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"RANGES\0" as *const u8 as *const i8,
            ) == 0 as i32
            {
                read_ranges(csa_0);
            }
            if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"BOUNDS\0" as *const u8 as *const i8,
            ) == 0 as i32
            {
                read_bounds(csa_0);
            }
            if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"ENDATA\0" as *const u8 as *const i8,
            ) != 0 as i32
            {
                error(
                    csa_0,
                    b"invalid use of %s indicator record\n\0" as *const u8 as *const i8,
                    ((*csa_0).field).as_mut_ptr(),
                );
            }
            glp_printf(
                b"%d row%s, %d column%s, %d non-zero%s\n\0" as *const u8 as *const i8,
                (*P).m,
                if (*P).m == 1 as i32 {
                    b"\0" as *const u8 as *const i8
                } else {
                    b"s\0" as *const u8 as *const i8
                },
                (*P).n,
                if (*P).n == 1 as i32 {
                    b"\0" as *const u8 as *const i8
                } else {
                    b"s\0" as *const u8 as *const i8
                },
                (*P).nnz,
                if (*P).nnz == 1 as i32 {
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
                        glp_printf(
                            b"One variable is integer\n\0" as *const u8 as *const i8,
                        );
                    } else {
                        glp_printf(
                            b"One variable is binary\n\0" as *const u8 as *const i8,
                        );
                    }
                } else {
                    glp_printf(
                        b"%d integer variables, \0" as *const u8 as *const i8,
                        ni,
                    );
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
            glp_printf(
                b"%d records were read\n\0" as *const u8 as *const i8,
                (*csa_0).recno,
            );
            let mut i_1: i32 = 0;
            let mut nrs: i32 = 0;
            let mut num: *mut i32 = 0 as *mut i32;
            num = glp_alloc(
                1 as i32 + (*P).m,
                ::core::mem::size_of::<i32>() as u64 as i32,
            ) as *mut i32;
            nrs = 0 as i32;
            i_1 = 1 as i32;
            while i_1 <= (*P).m {
                if (**((*P).row).offset(i_1 as isize)).type_0 == 1 as i32 {
                    nrs += 1;
                    *num.offset(nrs as isize) = i_1;
                }
                i_1 += 1;
                i_1;
            }
            if nrs > 0 as i32 {
                glp_del_rows(P, nrs, num as *const i32);
                if nrs == 1 as i32 {
                    glp_printf(
                        b"One free row was removed\n\0" as *const u8 as *const i8,
                    );
                } else {
                    glp_printf(
                        b"%d free rows were removed\n\0" as *const u8 as *const i8,
                        nrs,
                    );
                }
            }
            glp_free(num as *mut libc::c_void);
            glp_delete_index(P);
            glp_sort_matrix(P);
            ret = 0 as i32;
        }
    }
    if !((*csa_0).fp).is_null() {
        _glp_close((*csa_0).fp);
    }
    if !((*csa_0).work1).is_null() {
        glp_free((*csa_0).work1);
    }
    if !((*csa_0).work2).is_null() {
        glp_free((*csa_0).work2);
    }
    if !((*csa_0).work3).is_null() {
        glp_free((*csa_0).work3);
    }
    if ret != 0 as i32 {
        glp_erase_prob(P);
    }
    return ret;
}
unsafe extern "C" fn mps_name(mut csa1: *mut csa1) -> *mut i8 {
    let mut f: *mut i8 = 0 as *mut i8;
    if ((*(*csa1).P).name).is_null() {
        (*csa1).field[0 as i32 as usize] = '\0' as i32 as i8;
    } else if (*csa1).deck != 0 {
        strncpy(((*csa1).field).as_mut_ptr(), (*(*csa1).P).name, 8 as i32 as u64);
        (*csa1).field[8 as i32 as usize] = '\0' as i32 as i8;
    } else {
        strcpy(((*csa1).field).as_mut_ptr(), (*(*csa1).P).name);
    }
    f = ((*csa1).field).as_mut_ptr();
    while *f as i32 != '\0' as i32 {
        if *f as i32 == ' ' as i32 {
            *f = '_' as i32 as i8;
        }
        f = f.offset(1);
        f;
    }
    return ((*csa1).field).as_mut_ptr();
}
unsafe extern "C" fn row_name(mut csa1: *mut csa1, mut i: i32) -> *mut i8 {
    let mut f: *mut i8 = 0 as *mut i8;
    (0 as i32 <= i && i <= (*(*csa1).P).m
        || {
            glp_assert_(
                b"0 <= i && i <= csa->P->m\0" as *const u8 as *const i8,
                b"api/mps.c\0" as *const u8 as *const i8,
                1126 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if i == 0 as i32 || ((**((*(*csa1).P).row).offset(i as isize)).name).is_null()
        || (*csa1).deck != 0
            && strlen((**((*(*csa1).P).row).offset(i as isize)).name) > 8 as i32 as u64
    {
        sprintf(((*csa1).field).as_mut_ptr(), b"R%07d\0" as *const u8 as *const i8, i);
    } else {
        strcpy(
            ((*csa1).field).as_mut_ptr(),
            (**((*(*csa1).P).row).offset(i as isize)).name,
        );
        f = ((*csa1).field).as_mut_ptr();
        while *f as i32 != '\0' as i32 {
            if *f as i32 == ' ' as i32 {
                *f = '_' as i32 as i8;
            }
            f = f.offset(1);
            f;
        }
    }
    return ((*csa1).field).as_mut_ptr();
}
unsafe extern "C" fn col_name(mut csa1: *mut csa1, mut j: i32) -> *mut i8 {
    let mut f: *mut i8 = 0 as *mut i8;
    (1 as i32 <= j && j <= (*(*csa1).P).n
        || {
            glp_assert_(
                b"1 <= j && j <= csa->P->n\0" as *const u8 as *const i8,
                b"api/mps.c\0" as *const u8 as *const i8,
                1141 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if ((**((*(*csa1).P).col).offset(j as isize)).name).is_null()
        || (*csa1).deck != 0
            && strlen((**((*(*csa1).P).col).offset(j as isize)).name) > 8 as i32 as u64
    {
        sprintf(((*csa1).field).as_mut_ptr(), b"C%07d\0" as *const u8 as *const i8, j);
    } else {
        strcpy(
            ((*csa1).field).as_mut_ptr(),
            (**((*(*csa1).P).col).offset(j as isize)).name,
        );
        f = ((*csa1).field).as_mut_ptr();
        while *f as i32 != '\0' as i32 {
            if *f as i32 == ' ' as i32 {
                *f = '_' as i32 as i8;
            }
            f = f.offset(1);
            f;
        }
    }
    return ((*csa1).field).as_mut_ptr();
}
unsafe extern "C" fn mps_numb(mut csa1: *mut csa1, mut val: libc::c_double) -> *mut i8 {
    let mut dig: i32 = 0;
    let mut exp: *mut i8 = 0 as *mut i8;
    dig = 12 as i32;
    while dig >= 6 as i32 {
        if val != 0.0f64 && fabs(val) < 0.002f64 {
            sprintf(
                ((*csa1).field).as_mut_ptr(),
                b"%.*E\0" as *const u8 as *const i8,
                dig - 1 as i32,
                val,
            );
        } else {
            sprintf(
                ((*csa1).field).as_mut_ptr(),
                b"%.*G\0" as *const u8 as *const i8,
                dig,
                val,
            );
        }
        exp = strchr(((*csa1).field).as_mut_ptr(), 'E' as i32);
        if !exp.is_null() {
            sprintf(
                exp.offset(1 as i32 as isize),
                b"%d\0" as *const u8 as *const i8,
                atoi(exp.offset(1 as i32 as isize)),
            );
        }
        if strlen(((*csa1).field).as_mut_ptr()) <= 12 as i32 as u64 {
            break;
        }
        dig -= 1;
        dig;
    }
    (strlen(((*csa1).field).as_mut_ptr()) <= 12 as i32 as u64
        || {
            glp_assert_(
                b"strlen(csa->field) <= 12\0" as *const u8 as *const i8,
                b"api/mps.c\0" as *const u8 as *const i8,
                1167 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return ((*csa1).field).as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn glp_write_mps(
    mut P: *mut glp_prob,
    mut fmt: i32,
    mut parm: *const glp_mpscp,
    mut fname: *const i8,
) -> i32 {
    let mut _parm: glp_mpscp = glp_mpscp {
        blank: 0,
        obj_name: 0 as *mut i8,
        tol_mps: 0.,
        foo_bar: [0.; 17],
    };
    let mut _csa: csa1 = csa1 {
        P: 0 as *mut glp_prob,
        deck: 0,
        parm: 0 as *const glp_mpscp,
        field: [0; 256],
    };
    let mut csa1: *mut csa1 = &mut _csa;
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut out_obj: i32 = 0;
    let mut one_col: i32 = 0 as i32;
    let mut empty: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut recno: i32 = 0;
    let mut marker: i32 = 0;
    let mut count: i32 = 0;
    let mut gap: i32 = 0;
    let mut ret: i32 = 0;
    glp_printf(b"Writing problem data to '%s'...\n\0" as *const u8 as *const i8, fname);
    if !(fmt == 1 as i32 || fmt == 2 as i32) {
        (glp_error_(b"api/mps.c\0" as *const u8 as *const i8, 1181 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_write_mps: fmt = %d; invalid parameter\n\0" as *const u8 as *const i8,
            fmt,
        );
    }
    if parm.is_null() {
        glp_init_mpscp(&mut _parm);
        parm = &mut _parm;
    }
    check_parm(b"glp_write_mps\0" as *const u8 as *const i8, parm);
    (*csa1).P = P;
    (*csa1).deck = (fmt == 1 as i32) as i32;
    (*csa1).parm = parm;
    fp = _glp_open(fname, b"w\0" as *const u8 as *const i8);
    recno = 0 as i32;
    if fp.is_null() {
        glp_printf(
            b"Unable to create '%s' - %s\n\0" as *const u8 as *const i8,
            fname,
            _glp_get_err_msg(),
        );
        ret = 1 as i32;
    } else {
        _glp_format(
            fp,
            b"* %-*s%s\n\0" as *const u8 as *const i8,
            (if ((*P).name).is_null() { 1 as i32 } else { 12 as i32 }),
            b"Problem:\0" as *const u8 as *const i8,
            (if ((*P).name).is_null() {
                b"\0" as *const u8 as *const i8
            } else {
                (*P).name
            }),
        );
        recno += 1;
        recno;
        _glp_format(
            fp,
            b"* %-12s%s\n\0" as *const u8 as *const i8,
            b"Class:\0" as *const u8 as *const i8,
            (if glp_get_num_int(P) == 0 as i32 {
                b"LP\0" as *const u8 as *const i8
            } else {
                b"MIP\0" as *const u8 as *const i8
            }),
        );
        recno += 1;
        recno;
        _glp_format(
            fp,
            b"* %-12s%d\n\0" as *const u8 as *const i8,
            b"Rows:\0" as *const u8 as *const i8,
            (*P).m,
        );
        recno += 1;
        recno;
        if glp_get_num_int(P) == 0 as i32 {
            _glp_format(
                fp,
                b"* %-12s%d\n\0" as *const u8 as *const i8,
                b"Columns:\0" as *const u8 as *const i8,
                (*P).n,
            );
            recno += 1;
            recno;
        } else {
            _glp_format(
                fp,
                b"* %-12s%d (%d integer, %d binary)\n\0" as *const u8 as *const i8,
                b"Columns:\0" as *const u8 as *const i8,
                (*P).n,
                glp_get_num_int(P),
                glp_get_num_bin(P),
            );
            recno += 1;
            recno;
        }
        _glp_format(
            fp,
            b"* %-12s%d\n\0" as *const u8 as *const i8,
            b"Non-zeros:\0" as *const u8 as *const i8,
            (*P).nnz,
        );
        recno += 1;
        recno;
        _glp_format(
            fp,
            b"* %-12s%s\n\0" as *const u8 as *const i8,
            b"Format:\0" as *const u8 as *const i8,
            (if (*csa1).deck != 0 {
                b"Fixed MPS\0" as *const u8 as *const i8
            } else {
                b"Free MPS\0" as *const u8 as *const i8
            }),
        );
        recno += 1;
        recno;
        let fresh5 = recno;
        recno = recno + 1;
        _glp_format(fp, b"*\n\0" as *const u8 as *const i8, fresh5);
        _glp_format(
            fp,
            b"NAME%*s%s\n\0" as *const u8 as *const i8,
            (if ((*P).name).is_null() {
                0 as i32
            } else {
                (if (*csa1).deck != 0 { 10 as i32 } else { 1 as i32 })
            }),
            b"\0" as *const u8 as *const i8,
            mps_name(csa1),
        );
        recno += 1;
        recno;
        out_obj = 1 as i32;
        i = 1 as i32;
        while i <= (*P).m {
            if (**((*P).row).offset(i as isize)).type_0 == 1 as i32 {
                out_obj = 0 as i32;
                break;
            } else {
                i += 1;
                i;
            }
        }
        _glp_format(fp, b"ROWS\n\0" as *const u8 as *const i8);
        recno += 1;
        recno;
        i = if out_obj != 0 { 0 as i32 } else { 1 as i32 };
        while i <= (*P).m {
            let mut type_0: i32 = 0;
            type_0 = if i == 0 as i32 {
                1 as i32
            } else {
                (**((*P).row).offset(i as isize)).type_0
            };
            if type_0 == 1 as i32 {
                type_0 = 'N' as i32;
            } else if type_0 == 2 as i32 {
                type_0 = 'G' as i32;
            } else if type_0 == 3 as i32 {
                type_0 = 'L' as i32;
            } else if type_0 == 4 as i32 || type_0 == 5 as i32 {
                type_0 = 'E' as i32;
            } else {
                (type_0 != type_0
                    || {
                        glp_assert_(
                            b"type != type\0" as *const u8 as *const i8,
                            b"api/mps.c\0" as *const u8 as *const i8,
                            1241 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            _glp_format(
                fp,
                b" %c%*s%s\n\0" as *const u8 as *const i8,
                type_0,
                (if (*csa1).deck != 0 { 2 as i32 } else { 1 as i32 }),
                b"\0" as *const u8 as *const i8,
                row_name(csa1, i),
            );
            recno += 1;
            recno;
            i += 1;
            i;
        }
        _glp_format(fp, b"COLUMNS\n\0" as *const u8 as *const i8);
        recno += 1;
        recno;
        marker = 0 as i32;
        j = 1 as i32;
        while j <= (*P).n {
            let mut cj: GLPAIJ = GLPAIJ {
                row: 0 as *mut GLPROW,
                col: 0 as *mut GLPCOL,
                val: 0.,
                r_prev: 0 as *mut GLPAIJ,
                r_next: 0 as *mut GLPAIJ,
                c_prev: 0 as *mut GLPAIJ,
                c_next: 0 as *mut GLPAIJ,
            };
            let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
            let mut kind: i32 = 0;
            kind = (**((*P).col).offset(j as isize)).kind;
            if kind == 1 as i32 {
                if marker % 2 as i32 == 1 as i32 {
                    marker += 1;
                    marker;
                    _glp_format(
                        fp,
                        b"%*sM%07d%*s'MARKER'%*s'INTEND'\n\0" as *const u8 as *const i8,
                        (if (*csa1).deck != 0 { 4 as i32 } else { 1 as i32 }),
                        b"\0" as *const u8 as *const i8,
                        marker,
                        (if (*csa1).deck != 0 { 2 as i32 } else { 1 as i32 }),
                        b"\0" as *const u8 as *const i8,
                        (if (*csa1).deck != 0 { 17 as i32 } else { 1 as i32 }),
                        b"\0" as *const u8 as *const i8,
                    );
                    recno += 1;
                    recno;
                }
            } else if kind == 2 as i32 {
                if marker % 2 as i32 == 0 as i32 {
                    marker += 1;
                    marker;
                    _glp_format(
                        fp,
                        b"%*sM%07d%*s'MARKER'%*s'INTORG'\n\0" as *const u8 as *const i8,
                        (if (*csa1).deck != 0 { 4 as i32 } else { 1 as i32 }),
                        b"\0" as *const u8 as *const i8,
                        marker,
                        (if (*csa1).deck != 0 { 2 as i32 } else { 1 as i32 }),
                        b"\0" as *const u8 as *const i8,
                        (if (*csa1).deck != 0 { 17 as i32 } else { 1 as i32 }),
                        b"\0" as *const u8 as *const i8,
                    );
                    recno += 1;
                    recno;
                }
            } else {
                (kind != kind
                    || {
                        glp_assert_(
                            b"kind != kind\0" as *const u8 as *const i8,
                            b"api/mps.c\0" as *const u8 as *const i8,
                            1271 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if out_obj != 0 && (**((*P).col).offset(j as isize)).coef != 0.0f64 {
                aij = &mut cj;
                (*aij).row = 0 as *mut GLPROW;
                (*aij).val = (**((*P).col).offset(j as isize)).coef;
                (*aij).c_next = (**((*P).col).offset(j as isize)).ptr;
            } else {
                aij = (**((*P).col).offset(j as isize)).ptr;
            }
            if aij.is_null() {
                empty += 1;
                empty;
                _glp_format(
                    fp,
                    b"%*s%-*s\0" as *const u8 as *const i8,
                    if (*csa1).deck != 0 { 4 as i32 } else { 1 as i32 },
                    b"\0" as *const u8 as *const i8,
                    if (*csa1).deck != 0 { 8 as i32 } else { 1 as i32 },
                    col_name(csa1, j),
                );
                ((*P).m > 0 as i32
                    || {
                        glp_assert_(
                            b"P->m > 0\0" as *const u8 as *const i8,
                            b"api/mps.c\0" as *const u8 as *const i8,
                            1288 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                _glp_format(
                    fp,
                    b"%*s%-*s\0" as *const u8 as *const i8,
                    if (*csa1).deck != 0 { 2 as i32 } else { 1 as i32 },
                    b"\0" as *const u8 as *const i8,
                    if (*csa1).deck != 0 { 8 as i32 } else { 1 as i32 },
                    row_name(csa1, 1 as i32),
                );
                _glp_format(
                    fp,
                    b"%*s0%*s$ empty column\n\0" as *const u8 as *const i8,
                    (if (*csa1).deck != 0 { 13 as i32 } else { 1 as i32 }),
                    b"\0" as *const u8 as *const i8,
                    (if (*csa1).deck != 0 { 3 as i32 } else { 1 as i32 }),
                    b"\0" as *const u8 as *const i8,
                );
                recno += 1;
                recno;
            }
            count = 0 as i32;
            aij = aij;
            while !aij.is_null() {
                if one_col != 0 || count % 2 as i32 == 0 as i32 {
                    _glp_format(
                        fp,
                        b"%*s%-*s\0" as *const u8 as *const i8,
                        if (*csa1).deck != 0 { 4 as i32 } else { 1 as i32 },
                        b"\0" as *const u8 as *const i8,
                        if (*csa1).deck != 0 { 8 as i32 } else { 1 as i32 },
                        col_name(csa1, j),
                    );
                }
                gap = if one_col != 0 || count % 2 as i32 == 0 as i32 {
                    2 as i32
                } else {
                    3 as i32
                };
                _glp_format(
                    fp,
                    b"%*s%-*s\0" as *const u8 as *const i8,
                    if (*csa1).deck != 0 { gap } else { 1 as i32 },
                    b"\0" as *const u8 as *const i8,
                    if (*csa1).deck != 0 { 8 as i32 } else { 1 as i32 },
                    row_name(
                        csa1,
                        if ((*aij).row).is_null() { 0 as i32 } else { (*(*aij).row).i },
                    ),
                );
                _glp_format(
                    fp,
                    b"%*s%*s\0" as *const u8 as *const i8,
                    (if (*csa1).deck != 0 { 2 as i32 } else { 1 as i32 }),
                    b"\0" as *const u8 as *const i8,
                    (if (*csa1).deck != 0 { 12 as i32 } else { 1 as i32 }),
                    mps_numb(csa1, (*aij).val),
                );
                count += 1;
                count;
                if one_col != 0 || count % 2 as i32 == 0 as i32 {
                    _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                    recno += 1;
                    recno;
                }
                aij = (*aij).c_next;
            }
            if !(one_col != 0 || count % 2 as i32 == 0 as i32) {
                _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                recno += 1;
                recno;
            }
            j += 1;
            j;
        }
        if marker % 2 as i32 == 1 as i32 {
            marker += 1;
            marker;
            _glp_format(
                fp,
                b"%*sM%07d%*s'MARKER'%*s'INTEND'\n\0" as *const u8 as *const i8,
                (if (*csa1).deck != 0 { 4 as i32 } else { 1 as i32 }),
                b"\0" as *const u8 as *const i8,
                marker,
                (if (*csa1).deck != 0 { 2 as i32 } else { 1 as i32 }),
                b"\0" as *const u8 as *const i8,
                (if (*csa1).deck != 0 { 17 as i32 } else { 1 as i32 }),
                b"\0" as *const u8 as *const i8,
            );
            recno += 1;
            recno;
        }
        if empty > 0 as i32 {
            glp_printf(
                b"Warning: problem has %d empty column(s)\n\0" as *const u8 as *const i8,
                empty,
            );
        }
        _glp_format(fp, b"RHS\n\0" as *const u8 as *const i8);
        recno += 1;
        recno;
        count = 0 as i32;
        i = if out_obj != 0 { 0 as i32 } else { 1 as i32 };
        while i <= (*P).m {
            let mut type_1: i32 = 0;
            let mut rhs: libc::c_double = 0.;
            if i == 0 as i32 {
                rhs = (*P).c0;
            } else {
                type_1 = (**((*P).row).offset(i as isize)).type_0;
                if type_1 == 1 as i32 {
                    rhs = 0.0f64;
                } else if type_1 == 2 as i32 {
                    rhs = (**((*P).row).offset(i as isize)).lb;
                } else if type_1 == 3 as i32 {
                    rhs = (**((*P).row).offset(i as isize)).ub;
                } else if type_1 == 4 as i32 || type_1 == 5 as i32 {
                    rhs = (**((*P).row).offset(i as isize)).lb;
                } else {
                    (type_1 != type_1
                        || {
                            glp_assert_(
                                b"type != type\0" as *const u8 as *const i8,
                                b"api/mps.c\0" as *const u8 as *const i8,
                                1344 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
            if rhs != 0.0f64 {
                if one_col != 0 || count % 2 as i32 == 0 as i32 {
                    _glp_format(
                        fp,
                        b"%*s%-*s\0" as *const u8 as *const i8,
                        if (*csa1).deck != 0 { 4 as i32 } else { 1 as i32 },
                        b"\0" as *const u8 as *const i8,
                        if (*csa1).deck != 0 { 8 as i32 } else { 1 as i32 },
                        b"RHS1\0" as *const u8 as *const i8,
                    );
                }
                gap = if one_col != 0 || count % 2 as i32 == 0 as i32 {
                    2 as i32
                } else {
                    3 as i32
                };
                _glp_format(
                    fp,
                    b"%*s%-*s\0" as *const u8 as *const i8,
                    if (*csa1).deck != 0 { gap } else { 1 as i32 },
                    b"\0" as *const u8 as *const i8,
                    if (*csa1).deck != 0 { 8 as i32 } else { 1 as i32 },
                    row_name(csa1, i),
                );
                _glp_format(
                    fp,
                    b"%*s%*s\0" as *const u8 as *const i8,
                    (if (*csa1).deck != 0 { 2 as i32 } else { 1 as i32 }),
                    b"\0" as *const u8 as *const i8,
                    (if (*csa1).deck != 0 { 12 as i32 } else { 1 as i32 }),
                    mps_numb(csa1, rhs),
                );
                count += 1;
                count;
                if one_col != 0 || count % 2 as i32 == 0 as i32 {
                    _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                    recno += 1;
                    recno;
                }
            }
            i += 1;
            i;
        }
        if !(one_col != 0 || count % 2 as i32 == 0 as i32) {
            _glp_format(fp, b"\n\0" as *const u8 as *const i8);
            recno += 1;
            recno;
        }
        i = (*P).m;
        while i >= 1 as i32 {
            if (**((*P).row).offset(i as isize)).type_0 == 4 as i32 {
                break;
            }
            i -= 1;
            i;
        }
        if !(i == 0 as i32) {
            _glp_format(fp, b"RANGES\n\0" as *const u8 as *const i8);
            recno += 1;
            recno;
            count = 0 as i32;
            i = 1 as i32;
            while i <= (*P).m {
                if (**((*P).row).offset(i as isize)).type_0 == 4 as i32 {
                    if one_col != 0 || count % 2 as i32 == 0 as i32 {
                        _glp_format(
                            fp,
                            b"%*s%-*s\0" as *const u8 as *const i8,
                            if (*csa1).deck != 0 { 4 as i32 } else { 1 as i32 },
                            b"\0" as *const u8 as *const i8,
                            if (*csa1).deck != 0 { 8 as i32 } else { 1 as i32 },
                            b"RNG1\0" as *const u8 as *const i8,
                        );
                    }
                    gap = if one_col != 0 || count % 2 as i32 == 0 as i32 {
                        2 as i32
                    } else {
                        3 as i32
                    };
                    _glp_format(
                        fp,
                        b"%*s%-*s\0" as *const u8 as *const i8,
                        if (*csa1).deck != 0 { gap } else { 1 as i32 },
                        b"\0" as *const u8 as *const i8,
                        if (*csa1).deck != 0 { 8 as i32 } else { 1 as i32 },
                        row_name(csa1, i),
                    );
                    _glp_format(
                        fp,
                        b"%*s%*s\0" as *const u8 as *const i8,
                        (if (*csa1).deck != 0 { 2 as i32 } else { 1 as i32 }),
                        b"\0" as *const u8 as *const i8,
                        (if (*csa1).deck != 0 { 12 as i32 } else { 1 as i32 }),
                        mps_numb(
                            csa1,
                            (**((*P).row).offset(i as isize)).ub
                                - (**((*P).row).offset(i as isize)).lb,
                        ),
                    );
                    count += 1;
                    count;
                    if one_col != 0 || count % 2 as i32 == 0 as i32 {
                        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                        recno += 1;
                        recno;
                    }
                }
                i += 1;
                i;
            }
            if !(one_col != 0 || count % 2 as i32 == 0 as i32) {
                _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                recno += 1;
                recno;
            }
        }
        j = (*P).n;
        while j >= 1 as i32 {
            if !((**((*P).col).offset(j as isize)).kind == 1 as i32
                && (**((*P).col).offset(j as isize)).type_0 == 2 as i32
                && (**((*P).col).offset(j as isize)).lb == 0.0f64)
            {
                break;
            }
            j -= 1;
            j;
        }
        if !(j == 0 as i32) {
            _glp_format(fp, b"BOUNDS\n\0" as *const u8 as *const i8);
            recno += 1;
            recno;
            j = 1 as i32;
            while j <= (*P).n {
                let mut type_2: i32 = 0;
                let mut data: [i32; 2] = [0; 2];
                let mut bnd: [libc::c_double; 2] = [0.; 2];
                let mut spec: [*mut i8; 2] = [0 as *mut i8; 2];
                spec[1 as i32 as usize] = 0 as *mut i8;
                spec[0 as i32 as usize] = spec[1 as i32 as usize];
                type_2 = (**((*P).col).offset(j as isize)).type_0;
                if type_2 == 1 as i32 {
                    spec[0 as i32 as usize] = b"FR\0" as *const u8 as *const i8
                        as *mut i8;
                    data[0 as i32 as usize] = 0 as i32;
                } else if type_2 == 2 as i32 {
                    if (**((*P).col).offset(j as isize)).lb != 0.0f64 {
                        spec[0 as i32 as usize] = b"LO\0" as *const u8 as *const i8
                            as *mut i8;
                        data[0 as i32 as usize] = 1 as i32;
                        bnd[0 as i32 as usize] = (**((*P).col).offset(j as isize)).lb;
                    }
                    if (**((*P).col).offset(j as isize)).kind == 2 as i32 {
                        spec[1 as i32 as usize] = b"PL\0" as *const u8 as *const i8
                            as *mut i8;
                        data[1 as i32 as usize] = 0 as i32;
                    }
                } else if type_2 == 3 as i32 {
                    spec[0 as i32 as usize] = b"MI\0" as *const u8 as *const i8
                        as *mut i8;
                    data[0 as i32 as usize] = 0 as i32;
                    spec[1 as i32 as usize] = b"UP\0" as *const u8 as *const i8
                        as *mut i8;
                    data[1 as i32 as usize] = 1 as i32;
                    bnd[1 as i32 as usize] = (**((*P).col).offset(j as isize)).ub;
                } else if type_2 == 4 as i32 {
                    if (**((*P).col).offset(j as isize)).lb != 0.0f64 {
                        spec[0 as i32 as usize] = b"LO\0" as *const u8 as *const i8
                            as *mut i8;
                        data[0 as i32 as usize] = 1 as i32;
                        bnd[0 as i32 as usize] = (**((*P).col).offset(j as isize)).lb;
                    }
                    spec[1 as i32 as usize] = b"UP\0" as *const u8 as *const i8
                        as *mut i8;
                    data[1 as i32 as usize] = 1 as i32;
                    bnd[1 as i32 as usize] = (**((*P).col).offset(j as isize)).ub;
                } else if type_2 == 5 as i32 {
                    spec[0 as i32 as usize] = b"FX\0" as *const u8 as *const i8
                        as *mut i8;
                    data[0 as i32 as usize] = 1 as i32;
                    bnd[0 as i32 as usize] = (**((*P).col).offset(j as isize)).lb;
                } else {
                    (type_2 != type_2
                        || {
                            glp_assert_(
                                b"type != type\0" as *const u8 as *const i8,
                                b"api/mps.c\0" as *const u8 as *const i8,
                                1420 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
                i = 0 as i32;
                while i <= 1 as i32 {
                    if !(spec[i as usize]).is_null() {
                        _glp_format(
                            fp,
                            b" %s %-*s%*s%-*s\0" as *const u8 as *const i8,
                            spec[i as usize],
                            if (*csa1).deck != 0 { 8 as i32 } else { 1 as i32 },
                            b"BND1\0" as *const u8 as *const i8,
                            if (*csa1).deck != 0 { 2 as i32 } else { 1 as i32 },
                            b"\0" as *const u8 as *const i8,
                            if (*csa1).deck != 0 { 8 as i32 } else { 1 as i32 },
                            col_name(csa1, j),
                        );
                        if data[i as usize] != 0 {
                            _glp_format(
                                fp,
                                b"%*s%*s\0" as *const u8 as *const i8,
                                if (*csa1).deck != 0 { 2 as i32 } else { 1 as i32 },
                                b"\0" as *const u8 as *const i8,
                                if (*csa1).deck != 0 { 12 as i32 } else { 1 as i32 },
                                mps_numb(csa1, bnd[i as usize]),
                            );
                        }
                        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                        recno += 1;
                        recno;
                    }
                    i += 1;
                    i;
                }
                j += 1;
                j;
            }
        }
        _glp_format(fp, b"ENDATA\n\0" as *const u8 as *const i8);
        recno += 1;
        recno;
        if _glp_ioerr(fp) != 0 {
            glp_printf(
                b"Write error on '%s' - %s\n\0" as *const u8 as *const i8,
                fname,
                _glp_get_err_msg(),
            );
            ret = 1 as i32;
        } else {
            glp_printf(b"%d records were written\n\0" as *const u8 as *const i8, recno);
            ret = 0 as i32;
        }
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}