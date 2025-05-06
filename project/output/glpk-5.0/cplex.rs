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
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn _glp_close(f: *mut glp_file) -> i32;
    fn _glp_format(f: *mut glp_file, fmt: *const i8, _: ...) -> i32;
    fn _glp_getc(f: *mut glp_file) -> i32;
    fn _glp_ioerr(f: *mut glp_file) -> i32;
    fn _glp_open(name: *const i8, mode: *const i8) -> *mut glp_file;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn _glp_get_err_msg() -> *const i8;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_vprintf(fmt: *const i8, arg: ::core::ffi::VaList);
    fn glp_printf(fmt: *const i8, _: ...);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strlen(_: *const i8) -> u64;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn _glp_str2num(str: *const i8, val: *mut libc::c_double) -> i32;
    fn glp_set_obj_name(P: *mut glp_prob, name: *const i8);
    fn glp_set_obj_dir(P: *mut glp_prob, dir: i32);
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
    fn glp_set_mat_row(
        P: *mut glp_prob,
        i: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
    );
    fn glp_sort_matrix(P: *mut glp_prob);
    fn glp_erase_prob(P: *mut glp_prob);
    fn glp_get_obj_name(P: *mut glp_prob) -> *const i8;
    fn glp_get_row_name(P: *mut glp_prob, i: i32) -> *const i8;
    fn glp_get_col_name(P: *mut glp_prob, j: i32) -> *const i8;
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
pub type __int32_t = i32;
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
pub struct glp_cpxcp {
    pub foo_bar: [libc::c_double; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub P: *mut glp_prob,
    pub parm: *const glp_cpxcp,
    pub fname: *const i8,
    pub fp: *mut glp_file,
    pub jump: jmp_buf,
    pub count: i32,
    pub c: i32,
    pub token: i32,
    pub image: [i8; 256],
    pub imlen: i32,
    pub value: libc::c_double,
    pub n_max: i32,
    pub ind: *mut i32,
    pub val: *mut libc::c_double,
    pub flag: *mut i8,
    pub lb: *mut libc::c_double,
    pub ub: *mut libc::c_double,
    pub lb_warn: i32,
    pub ub_warn: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa1 {
    pub P: *mut glp_prob,
    pub parm: *const glp_cpxcp,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub unsafe extern "C" fn glp_init_cpxcp(mut parm: *mut glp_cpxcp) {
    (!parm.is_null()
        || {
            glp_assert_(
                b"parm != NULL\0" as *const u8 as *const i8,
                b"api/cplex.c\0" as *const u8 as *const i8,
                45 as i32,
            );
            1 as i32 != 0
        }) as i32;
}
unsafe extern "C" fn check_parm(mut func: *const i8, mut parm: *const glp_cpxcp) {
    (!func.is_null()
        || {
            glp_assert_(
                b"func != NULL\0" as *const u8 as *const i8,
                b"api/cplex.c\0" as *const u8 as *const i8,
                51 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!parm.is_null()
        || {
            glp_assert_(
                b"parm != NULL\0" as *const u8 as *const i8,
                b"api/cplex.c\0" as *const u8 as *const i8,
                52 as i32,
            );
            1 as i32 != 0
        }) as i32;
}
unsafe extern "C" fn error(mut csa_0: *mut csa, mut fmt: *const i8, mut args: ...) {
    let mut arg: ::core::ffi::VaListImpl;
    glp_printf(b"%s:%d: \0" as *const u8 as *const i8, (*csa_0).fname, (*csa_0).count);
    arg = args.clone();
    glp_vprintf(fmt, arg.as_va_list());
    longjmp(((*csa_0).jump).as_mut_ptr(), 1 as i32);
}
unsafe extern "C" fn warning(mut csa_0: *mut csa, mut fmt: *const i8, mut args: ...) {
    let mut arg: ::core::ffi::VaListImpl;
    glp_printf(
        b"%s:%d: warning: \0" as *const u8 as *const i8,
        (*csa_0).fname,
        (*csa_0).count,
    );
    arg = args.clone();
    glp_vprintf(fmt, arg.as_va_list());
}
unsafe extern "C" fn read_char(mut csa_0: *mut csa) {
    let mut c: i32 = 0;
    ((*csa_0).c != -(1 as i32)
        || {
            glp_assert_(
                b"csa->c != EOF\0" as *const u8 as *const i8,
                b"api/cplex.c\0" as *const u8 as *const i8,
                170 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*csa_0).c == '\n' as i32 {
        (*csa_0).count += 1;
        (*csa_0).count;
    }
    c = _glp_getc((*csa_0).fp);
    if c < 0 as i32 {
        if _glp_ioerr((*csa_0).fp) != 0 {
            error(
                csa_0,
                b"read error - %s\n\0" as *const u8 as *const i8,
                _glp_get_err_msg(),
            );
        } else if (*csa_0).c == '\n' as i32 {
            (*csa_0).count -= 1;
            (*csa_0).count;
            c = -(1 as i32);
        } else {
            warning(csa_0, b"missing final end of line\n\0" as *const u8 as *const i8);
            c = '\n' as i32;
        }
    } else if !(c == '\n' as i32) {
        if *(*__ctype_b_loc()).offset(c as isize) as i32
            & _ISspace as i32 as libc::c_ushort as i32 != 0
        {
            c = ' ' as i32;
        } else if *(*__ctype_b_loc()).offset(c as isize) as i32
            & _IScntrl as i32 as libc::c_ushort as i32 != 0
        {
            error(
                csa_0,
                b"invalid control character 0x%02X\n\0" as *const u8 as *const i8,
                c,
            );
        }
    }
    (*csa_0).c = c;
}
unsafe extern "C" fn add_char(mut csa_0: *mut csa) {
    if (*csa_0).imlen as u64
        == (::core::mem::size_of::<[i8; 256]>() as u64).wrapping_sub(1 as i32 as u64)
    {
        error(
            csa_0,
            b"token '%.15s...' too long\n\0" as *const u8 as *const i8,
            ((*csa_0).image).as_mut_ptr(),
        );
    }
    let fresh0 = (*csa_0).imlen;
    (*csa_0).imlen = (*csa_0).imlen + 1;
    (*csa_0).image[fresh0 as usize] = (*csa_0).c as i8;
    (*csa_0).image[(*csa_0).imlen as usize] = '\0' as i32 as i8;
    read_char(csa_0);
}
unsafe extern "C" fn the_same(mut s1: *mut i8, mut s2: *mut i8) -> i32 {
    while *s1 as i32 != '\0' as i32 {
        if ({
            let mut __res: i32 = 0;
            if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                if 0 != 0 {
                    let mut __c: i32 = *s1 as u8 as i32;
                    __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*s1 as u8 as i32);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(*s1 as u8 as i32 as isize);
            }
            __res
        })
            != ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = *s2 as u8 as i32;
                        __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(*s2 as u8 as i32);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(*s2 as u8 as i32 as isize);
                }
                __res
            })
        {
            return 0 as i32;
        }
        s1 = s1.offset(1);
        s1;
        s2 = s2.offset(1);
        s2;
    }
    return 1 as i32;
}
unsafe extern "C" fn scan_token(mut csa_0: *mut csa) {
    let mut current_block: u64;
    let mut flag: i32 = 0;
    (*csa_0).token = -(1 as i32);
    (*csa_0).image[0 as i32 as usize] = '\0' as i32 as i8;
    (*csa_0).imlen = 0 as i32;
    (*csa_0).value = 0.0f64;
    loop {
        flag = 0 as i32;
        while (*csa_0).c == ' ' as i32 {
            read_char(csa_0);
        }
        if (*csa_0).c == -(1 as i32) {
            (*csa_0).token = 0 as i32;
            current_block = 15456862084301247793;
            break;
        } else if (*csa_0).c == '\n' as i32 {
            read_char(csa_0);
            if !(*(*__ctype_b_loc()).offset((*csa_0).c as isize) as i32
                & _ISalpha as i32 as libc::c_ushort as i32 != 0)
            {
                continue;
            }
            flag = 1 as i32;
            current_block = 12599329904712511516;
            break;
        } else if (*csa_0).c == '\\' as i32 {
            while (*csa_0).c != '\n' as i32 {
                read_char(csa_0);
            }
        } else if *(*__ctype_b_loc()).offset((*csa_0).c as isize) as i32
            & _ISalpha as i32 as libc::c_ushort as i32 != 0
            || (*csa_0).c != '.' as i32
                && !(strchr(
                    b"!\"#$%&()/,.;?@_`'{}|~\0" as *const u8 as *const i8,
                    (*csa_0).c,
                ))
                    .is_null()
        {
            current_block = 12599329904712511516;
            break;
        } else {
            current_block = 3512920355445576850;
            break;
        }
    }
    match current_block {
        12599329904712511516 => {
            (*csa_0).token = 0x9 as i32;
            while *(*__ctype_b_loc()).offset((*csa_0).c as isize) as i32
                & _ISalnum as i32 as libc::c_ushort as i32 != 0
                || !(strchr(
                    b"!\"#$%&()/,.;?@_`'{}|~\0" as *const u8 as *const i8,
                    (*csa_0).c,
                ))
                    .is_null()
            {
                add_char(csa_0);
            }
            if flag != 0 {
                if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"minimize\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x1 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"minimum\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x1 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"min\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x1 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"maximize\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x2 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"maximum\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x2 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"max\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x2 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"subject\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    if (*csa_0).c == ' ' as i32 {
                        read_char(csa_0);
                        if ({
                            let mut __res: i32 = 0;
                            if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                                if 0 != 0 {
                                    let mut __c: i32 = (*csa_0).c;
                                    __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = tolower((*csa_0).c);
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset((*csa_0).c as isize);
                            }
                            __res
                        }) == 't' as i32
                        {
                            (*csa_0).token = 0x3 as i32;
                            let fresh1 = (*csa_0).imlen;
                            (*csa_0).imlen = (*csa_0).imlen + 1;
                            (*csa_0).image[fresh1 as usize] = ' ' as i32 as i8;
                            (*csa_0).image[(*csa_0).imlen as usize] = '\0' as i32 as i8;
                            add_char(csa_0);
                            if ({
                                let mut __res: i32 = 0;
                                if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                                    if 0 != 0 {
                                        let mut __c: i32 = (*csa_0).c;
                                        __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        });
                                    } else {
                                        __res = tolower((*csa_0).c);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc())
                                        .offset((*csa_0).c as isize);
                                }
                                __res
                            }) != 'o' as i32
                            {
                                error(
                                    csa_0,
                                    b"keyword 'subject to' incomplete\n\0" as *const u8
                                        as *const i8,
                                );
                            }
                            add_char(csa_0);
                            if *(*__ctype_b_loc()).offset((*csa_0).c as isize) as i32
                                & _ISalpha as i32 as libc::c_ushort as i32 != 0
                            {
                                error(
                                    csa_0,
                                    b"keyword '%s%c...' not recognized\n\0" as *const u8
                                        as *const i8,
                                    ((*csa_0).image).as_mut_ptr(),
                                    (*csa_0).c,
                                );
                            }
                        }
                    }
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"such\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    if (*csa_0).c == ' ' as i32 {
                        read_char(csa_0);
                        if ({
                            let mut __res: i32 = 0;
                            if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                                if 0 != 0 {
                                    let mut __c: i32 = (*csa_0).c;
                                    __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = tolower((*csa_0).c);
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset((*csa_0).c as isize);
                            }
                            __res
                        }) == 't' as i32
                        {
                            (*csa_0).token = 0x3 as i32;
                            let fresh2 = (*csa_0).imlen;
                            (*csa_0).imlen = (*csa_0).imlen + 1;
                            (*csa_0).image[fresh2 as usize] = ' ' as i32 as i8;
                            (*csa_0).image[(*csa_0).imlen as usize] = '\0' as i32 as i8;
                            add_char(csa_0);
                            if ({
                                let mut __res: i32 = 0;
                                if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                                    if 0 != 0 {
                                        let mut __c: i32 = (*csa_0).c;
                                        __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        });
                                    } else {
                                        __res = tolower((*csa_0).c);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc())
                                        .offset((*csa_0).c as isize);
                                }
                                __res
                            }) != 'h' as i32
                            {
                                current_block = 12073580533535933710;
                            } else {
                                current_block = 2480299350034459858;
                            }
                            loop {
                                match current_block {
                                    12073580533535933710 => {
                                        error(
                                            csa_0,
                                            b"keyword 'such that' incomplete\n\0" as *const u8
                                                as *const i8,
                                        );
                                        current_block = 2480299350034459858;
                                    }
                                    _ => {
                                        add_char(csa_0);
                                        if ({
                                            let mut __res: i32 = 0;
                                            if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                                                if 0 != 0 {
                                                    let mut __c: i32 = (*csa_0).c;
                                                    __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                                        __c
                                                    } else {
                                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                                    });
                                                } else {
                                                    __res = tolower((*csa_0).c);
                                                }
                                            } else {
                                                __res = *(*__ctype_tolower_loc())
                                                    .offset((*csa_0).c as isize);
                                            }
                                            __res
                                        }) != 'a' as i32
                                        {
                                            current_block = 12073580533535933710;
                                            continue;
                                        }
                                        add_char(csa_0);
                                        if ({
                                            let mut __res: i32 = 0;
                                            if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                                                if 0 != 0 {
                                                    let mut __c: i32 = (*csa_0).c;
                                                    __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                                        __c
                                                    } else {
                                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                                    });
                                                } else {
                                                    __res = tolower((*csa_0).c);
                                                }
                                            } else {
                                                __res = *(*__ctype_tolower_loc())
                                                    .offset((*csa_0).c as isize);
                                            }
                                            __res
                                        }) != 't' as i32
                                        {
                                            current_block = 12073580533535933710;
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                            add_char(csa_0);
                            if *(*__ctype_b_loc()).offset((*csa_0).c as isize) as i32
                                & _ISalpha as i32 as libc::c_ushort as i32 != 0
                            {
                                error(
                                    csa_0,
                                    b"keyword '%s%c...' not recognized\n\0" as *const u8
                                        as *const i8,
                                    ((*csa_0).image).as_mut_ptr(),
                                    (*csa_0).c,
                                );
                            }
                        }
                    }
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"st\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x3 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"s.t.\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x3 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"st.\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x3 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"bounds\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x4 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"bound\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x4 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"general\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x5 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"generals\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x5 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"gen\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x5 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"integer\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x6 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"integers\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x6 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"int\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x6 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"binary\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x7 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"binaries\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x7 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"bin\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x7 as i32;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"end\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    (*csa_0).token = 0x8 as i32;
                }
            }
        }
        3512920355445576850 => {
            if *(*__ctype_b_loc()).offset((*csa_0).c as isize) as i32
                & _ISdigit as i32 as libc::c_ushort as i32 != 0
                || (*csa_0).c == '.' as i32
            {
                (*csa_0).token = 0xa as i32;
                while *(*__ctype_b_loc()).offset((*csa_0).c as isize) as i32
                    & _ISdigit as i32 as libc::c_ushort as i32 != 0
                {
                    add_char(csa_0);
                }
                if (*csa_0).c == '.' as i32 {
                    add_char(csa_0);
                    if (*csa_0).imlen == 1 as i32
                        && *(*__ctype_b_loc()).offset((*csa_0).c as isize) as i32
                            & _ISdigit as i32 as libc::c_ushort as i32 == 0
                    {
                        error(
                            csa_0,
                            b"invalid use of decimal point\n\0" as *const u8 as *const i8,
                        );
                    }
                    while *(*__ctype_b_loc()).offset((*csa_0).c as isize) as i32
                        & _ISdigit as i32 as libc::c_ushort as i32 != 0
                    {
                        add_char(csa_0);
                    }
                }
                if (*csa_0).c == 'e' as i32 || (*csa_0).c == 'E' as i32 {
                    add_char(csa_0);
                    if (*csa_0).c == '+' as i32 || (*csa_0).c == '-' as i32 {
                        add_char(csa_0);
                    }
                    if *(*__ctype_b_loc()).offset((*csa_0).c as isize) as i32
                        & _ISdigit as i32 as libc::c_ushort as i32 == 0
                    {
                        error(
                            csa_0,
                            b"numeric constant '%s' incomplete\n\0" as *const u8
                                as *const i8,
                            ((*csa_0).image).as_mut_ptr(),
                        );
                    }
                    while *(*__ctype_b_loc()).offset((*csa_0).c as isize) as i32
                        & _ISdigit as i32 as libc::c_ushort as i32 != 0
                    {
                        add_char(csa_0);
                    }
                }
                if _glp_str2num(((*csa_0).image).as_mut_ptr(), &mut (*csa_0).value) != 0
                {
                    error(
                        csa_0,
                        b"numeric constant '%s' out of range\n\0" as *const u8
                            as *const i8,
                        ((*csa_0).image).as_mut_ptr(),
                    );
                }
            } else if (*csa_0).c == '+' as i32 {
                (*csa_0).token = 0xb as i32;
                add_char(csa_0);
            } else if (*csa_0).c == '-' as i32 {
                (*csa_0).token = 0xc as i32;
                add_char(csa_0);
            } else if (*csa_0).c == ':' as i32 {
                (*csa_0).token = 0xd as i32;
                add_char(csa_0);
            } else if (*csa_0).c == '<' as i32 {
                (*csa_0).token = 0xe as i32;
                add_char(csa_0);
                if (*csa_0).c == '=' as i32 {
                    add_char(csa_0);
                }
            } else if (*csa_0).c == '>' as i32 {
                (*csa_0).token = 0xf as i32;
                add_char(csa_0);
                if (*csa_0).c == '=' as i32 {
                    add_char(csa_0);
                }
            } else if (*csa_0).c == '=' as i32 {
                (*csa_0).token = 0x10 as i32;
                add_char(csa_0);
                if (*csa_0).c == '<' as i32 {
                    (*csa_0).token = 0xe as i32;
                    add_char(csa_0);
                } else if (*csa_0).c == '>' as i32 {
                    (*csa_0).token = 0xf as i32;
                    add_char(csa_0);
                }
            } else {
                error(
                    csa_0,
                    b"character '%c' not recognized\n\0" as *const u8 as *const i8,
                    (*csa_0).c,
                );
            }
        }
        _ => {}
    }
    while (*csa_0).c == ' ' as i32 {
        read_char(csa_0);
    }
}
unsafe extern "C" fn find_col(mut csa_0: *mut csa, mut name: *mut i8) -> i32 {
    let mut j: i32 = 0;
    j = glp_find_col((*csa_0).P, name);
    if j == 0 as i32 {
        j = glp_add_cols((*csa_0).P, 1 as i32);
        glp_set_col_name((*csa_0).P, j, name);
        if (*csa_0).n_max < j {
            let mut n_max: i32 = (*csa_0).n_max;
            let mut ind: *mut i32 = (*csa_0).ind;
            let mut val: *mut libc::c_double = (*csa_0).val;
            let mut flag: *mut i8 = (*csa_0).flag;
            let mut lb: *mut libc::c_double = (*csa_0).lb;
            let mut ub: *mut libc::c_double = (*csa_0).ub;
            (*csa_0).n_max += (*csa_0).n_max;
            (*csa_0).ind = glp_alloc(
                1 as i32 + (*csa_0).n_max,
                ::core::mem::size_of::<i32>() as u64 as i32,
            ) as *mut i32;
            memcpy(
                &mut *((*csa_0).ind).offset(1 as i32 as isize) as *mut i32
                    as *mut libc::c_void,
                &mut *ind.offset(1 as i32 as isize) as *mut i32 as *const libc::c_void,
                (n_max as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
            );
            glp_free(ind as *mut libc::c_void);
            (*csa_0).val = glp_alloc(
                1 as i32 + (*csa_0).n_max,
                ::core::mem::size_of::<libc::c_double>() as u64 as i32,
            ) as *mut libc::c_double;
            memcpy(
                &mut *((*csa_0).val).offset(1 as i32 as isize) as *mut libc::c_double
                    as *mut libc::c_void,
                &mut *val.offset(1 as i32 as isize) as *mut libc::c_double
                    as *const libc::c_void,
                (n_max as u64)
                    .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
            );
            glp_free(val as *mut libc::c_void);
            (*csa_0).flag = glp_alloc(
                1 as i32 + (*csa_0).n_max,
                ::core::mem::size_of::<i8>() as u64 as i32,
            ) as *mut i8;
            memset(
                &mut *((*csa_0).flag).offset(1 as i32 as isize) as *mut i8
                    as *mut libc::c_void,
                0 as i32,
                ((*csa_0).n_max as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            memcpy(
                &mut *((*csa_0).flag).offset(1 as i32 as isize) as *mut i8
                    as *mut libc::c_void,
                &mut *flag.offset(1 as i32 as isize) as *mut i8 as *const libc::c_void,
                (n_max as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            glp_free(flag as *mut libc::c_void);
            (*csa_0).lb = glp_alloc(
                1 as i32 + (*csa_0).n_max,
                ::core::mem::size_of::<libc::c_double>() as u64 as i32,
            ) as *mut libc::c_double;
            memcpy(
                &mut *((*csa_0).lb).offset(1 as i32 as isize) as *mut libc::c_double
                    as *mut libc::c_void,
                &mut *lb.offset(1 as i32 as isize) as *mut libc::c_double
                    as *const libc::c_void,
                (n_max as u64)
                    .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
            );
            glp_free(lb as *mut libc::c_void);
            (*csa_0).ub = glp_alloc(
                1 as i32 + (*csa_0).n_max,
                ::core::mem::size_of::<libc::c_double>() as u64 as i32,
            ) as *mut libc::c_double;
            memcpy(
                &mut *((*csa_0).ub).offset(1 as i32 as isize) as *mut libc::c_double
                    as *mut libc::c_void,
                &mut *ub.offset(1 as i32 as isize) as *mut libc::c_double
                    as *const libc::c_void,
                (n_max as u64)
                    .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
            );
            glp_free(ub as *mut libc::c_void);
        }
        *((*csa_0).lb).offset(j as isize) = 1.7976931348623157e+308f64;
        *((*csa_0).ub).offset(j as isize) = -1.7976931348623157e+308f64;
    }
    return j;
}
unsafe extern "C" fn parse_linear_form(mut csa_0: *mut csa) -> i32 {
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut len: i32 = 0 as i32;
    let mut newlen: i32 = 0;
    let mut s: libc::c_double = 0.;
    let mut coef: libc::c_double = 0.;
    loop {
        if (*csa_0).token == 0xb as i32 {
            s = 1.0f64;
            scan_token(csa_0);
        } else if (*csa_0).token == 0xc as i32 {
            s = -1.0f64;
            scan_token(csa_0);
        } else {
            s = 1.0f64;
        }
        if (*csa_0).token == 0xa as i32 {
            coef = (*csa_0).value;
            scan_token(csa_0);
        } else {
            coef = 1.0f64;
        }
        if (*csa_0).token != 0x9 as i32 {
            error(csa_0, b"missing variable name\n\0" as *const u8 as *const i8);
        }
        j = find_col(csa_0, ((*csa_0).image).as_mut_ptr());
        if *((*csa_0).flag).offset(j as isize) != 0 {
            error(
                csa_0,
                b"multiple use of variable '%s' not allowed\n\0" as *const u8
                    as *const i8,
                ((*csa_0).image).as_mut_ptr(),
            );
        }
        len += 1;
        len;
        *((*csa_0).ind).offset(len as isize) = j;
        *((*csa_0).val).offset(len as isize) = s * coef;
        *((*csa_0).flag).offset(j as isize) = 1 as i32 as i8;
        scan_token(csa_0);
        if !((*csa_0).token == 0xb as i32 || (*csa_0).token == 0xc as i32) {
            break;
        }
    }
    k = 1 as i32;
    while k <= len {
        *((*csa_0).flag).offset(*((*csa_0).ind).offset(k as isize) as isize) = 0 as i32
            as i8;
        k += 1;
        k;
    }
    newlen = 0 as i32;
    k = 1 as i32;
    while k <= len {
        if *((*csa_0).val).offset(k as isize) != 0.0f64 {
            newlen += 1;
            newlen;
            *((*csa_0).ind).offset(newlen as isize) = *((*csa_0).ind).offset(k as isize);
            *((*csa_0).val).offset(newlen as isize) = *((*csa_0).val).offset(k as isize);
        }
        k += 1;
        k;
    }
    return newlen;
}
unsafe extern "C" fn parse_objective(mut csa_0: *mut csa) {
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    if (*csa_0).token == 0x1 as i32 {
        glp_set_obj_dir((*csa_0).P, 1 as i32);
    } else if (*csa_0).token == 0x2 as i32 {
        glp_set_obj_dir((*csa_0).P, 2 as i32);
    } else {
        (csa_0 != csa_0
            || {
                glp_assert_(
                    b"csa != csa\0" as *const u8 as *const i8,
                    b"api/cplex.c\0" as *const u8 as *const i8,
                    502 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    scan_token(csa_0);
    if (*csa_0).token == 0x9 as i32 && (*csa_0).c == ':' as i32 {
        glp_set_obj_name((*csa_0).P, ((*csa_0).image).as_mut_ptr());
        scan_token(csa_0);
        ((*csa_0).token == 0xd as i32
            || {
                glp_assert_(
                    b"csa->token == T_COLON\0" as *const u8 as *const i8,
                    b"api/cplex.c\0" as *const u8 as *const i8,
                    509 as i32,
                );
                1 as i32 != 0
            }) as i32;
        scan_token(csa_0);
    } else {
        glp_set_obj_name((*csa_0).P, b"obj\0" as *const u8 as *const i8);
    }
    len = parse_linear_form(csa_0);
    k = 1 as i32;
    while k <= len {
        glp_set_obj_coef(
            (*csa_0).P,
            *((*csa_0).ind).offset(k as isize),
            *((*csa_0).val).offset(k as isize),
        );
        k += 1;
        k;
    }
}
unsafe extern "C" fn parse_constraints(mut csa_0: *mut csa) {
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    let mut type_0: i32 = 0;
    let mut s: libc::c_double = 0.;
    ((*csa_0).token == 0x3 as i32
        || {
            glp_assert_(
                b"csa->token == T_SUBJECT_TO\0" as *const u8 as *const i8,
                b"api/cplex.c\0" as *const u8 as *const i8,
                543 as i32,
            );
            1 as i32 != 0
        }) as i32;
    scan_token(csa_0);
    loop {
        i = glp_add_rows((*csa_0).P, 1 as i32);
        if (*csa_0).token == 0x9 as i32 && (*csa_0).c == ':' as i32 {
            if glp_find_row((*csa_0).P, ((*csa_0).image).as_mut_ptr()) != 0 as i32 {
                error(
                    csa_0,
                    b"constraint '%s' multiply defined\n\0" as *const u8 as *const i8,
                    ((*csa_0).image).as_mut_ptr(),
                );
            }
            glp_set_row_name((*csa_0).P, i, ((*csa_0).image).as_mut_ptr());
            scan_token(csa_0);
            ((*csa_0).token == 0xd as i32
                || {
                    glp_assert_(
                        b"csa->token == T_COLON\0" as *const u8 as *const i8,
                        b"api/cplex.c\0" as *const u8 as *const i8,
                        555 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            scan_token(csa_0);
        } else {
            let mut name: [i8; 50] = [0; 50];
            sprintf(
                name.as_mut_ptr(),
                b"r.%d\0" as *const u8 as *const i8,
                (*csa_0).count,
            );
            glp_set_row_name((*csa_0).P, i, name.as_mut_ptr());
        }
        len = parse_linear_form(csa_0);
        glp_set_mat_row(
            (*csa_0).P,
            i,
            len,
            (*csa_0).ind as *const i32,
            (*csa_0).val as *const libc::c_double,
        );
        if (*csa_0).token == 0xe as i32 {
            type_0 = 3 as i32;
            scan_token(csa_0);
        } else if (*csa_0).token == 0xf as i32 {
            type_0 = 2 as i32;
            scan_token(csa_0);
        } else if (*csa_0).token == 0x10 as i32 {
            type_0 = 5 as i32;
            scan_token(csa_0);
        } else {
            error(csa_0, b"missing constraint sense\n\0" as *const u8 as *const i8);
        }
        if (*csa_0).token == 0xb as i32 {
            s = 1.0f64;
            scan_token(csa_0);
        } else if (*csa_0).token == 0xc as i32 {
            s = -1.0f64;
            scan_token(csa_0);
        } else {
            s = 1.0f64;
        }
        if (*csa_0).token != 0xa as i32 {
            error(csa_0, b"missing right-hand side\n\0" as *const u8 as *const i8);
        }
        glp_set_row_bnds((*csa_0).P, i, type_0, s * (*csa_0).value, s * (*csa_0).value);
        if !((*csa_0).c == '\n' as i32 || (*csa_0).c == -(1 as i32)) {
            error(
                csa_0,
                b"invalid symbol(s) beyond right-hand side\n\0" as *const u8 as *const i8,
            );
        }
        scan_token(csa_0);
        if !((*csa_0).token == 0xb as i32 || (*csa_0).token == 0xc as i32
            || (*csa_0).token == 0xa as i32 || (*csa_0).token == 0x9 as i32)
        {
            break;
        }
    };
}
unsafe extern "C" fn set_lower_bound(
    mut csa_0: *mut csa,
    mut j: i32,
    mut lb: libc::c_double,
) {
    if *((*csa_0).lb).offset(j as isize) != 1.7976931348623157e+308f64
        && (*csa_0).lb_warn == 0
    {
        warning(
            csa_0,
            b"lower bound of variable '%s' redefined\n\0" as *const u8 as *const i8,
            glp_get_col_name((*csa_0).P, j),
        );
        (*csa_0).lb_warn = 1 as i32;
    }
    *((*csa_0).lb).offset(j as isize) = lb;
}
unsafe extern "C" fn set_upper_bound(
    mut csa_0: *mut csa,
    mut j: i32,
    mut ub: libc::c_double,
) {
    if *((*csa_0).ub).offset(j as isize) != -1.7976931348623157e+308f64
        && (*csa_0).ub_warn == 0
    {
        warning(
            csa_0,
            b"upper bound of variable '%s' redefined\n\0" as *const u8 as *const i8,
            glp_get_col_name((*csa_0).P, j),
        );
        (*csa_0).ub_warn = 1 as i32;
    }
    *((*csa_0).ub).offset(j as isize) = ub;
}
unsafe extern "C" fn parse_bounds(mut csa_0: *mut csa) {
    let mut j: i32 = 0;
    let mut lb_flag: i32 = 0;
    let mut lb: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    ((*csa_0).token == 0x4 as i32
        || {
            glp_assert_(
                b"csa->token == T_BOUNDS\0" as *const u8 as *const i8,
                b"api/cplex.c\0" as *const u8 as *const i8,
                641 as i32,
            );
            1 as i32 != 0
        }) as i32;
    scan_token(csa_0);
    while (*csa_0).token == 0xb as i32 || (*csa_0).token == 0xc as i32
        || (*csa_0).token == 0xa as i32 || (*csa_0).token == 0x9 as i32
    {
        if (*csa_0).token == 0xb as i32 || (*csa_0).token == 0xc as i32 {
            lb_flag = 1 as i32;
            s = if (*csa_0).token == 0xb as i32 { 1.0f64 } else { -1.0f64 };
            scan_token(csa_0);
            if (*csa_0).token == 0xa as i32 {
                lb = s * (*csa_0).value;
                scan_token(csa_0);
            } else if the_same(
                ((*csa_0).image).as_mut_ptr(),
                b"infinity\0" as *const u8 as *const i8 as *mut i8,
            ) != 0
                || the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"inf\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
            {
                if s > 0.0f64 {
                    error(
                        csa_0,
                        b"invalid use of '+inf' as lower bound\n\0" as *const u8
                            as *const i8,
                    );
                }
                lb = -1.7976931348623157e+308f64;
                scan_token(csa_0);
            } else {
                error(csa_0, b"missing lower bound\n\0" as *const u8 as *const i8);
            }
        } else if (*csa_0).token == 0xa as i32 {
            lb_flag = 1 as i32;
            lb = (*csa_0).value;
            scan_token(csa_0);
        } else {
            lb_flag = 0 as i32;
        }
        if lb_flag != 0 {
            if (*csa_0).token != 0xe as i32 {
                error(
                    csa_0,
                    b"missing '<', '<=', or '=<' after lower bound\n\0" as *const u8
                        as *const i8,
                );
            }
            scan_token(csa_0);
        }
        if (*csa_0).token != 0x9 as i32 {
            error(csa_0, b"missing variable name\n\0" as *const u8 as *const i8);
        }
        j = find_col(csa_0, ((*csa_0).image).as_mut_ptr());
        if lb_flag != 0 {
            set_lower_bound(csa_0, j, lb);
        }
        scan_token(csa_0);
        if (*csa_0).token == 0xe as i32 {
            scan_token(csa_0);
            if (*csa_0).token == 0xb as i32 || (*csa_0).token == 0xc as i32 {
                s = if (*csa_0).token == 0xb as i32 { 1.0f64 } else { -1.0f64 };
                scan_token(csa_0);
                if (*csa_0).token == 0xa as i32 {
                    set_upper_bound(csa_0, j, s * (*csa_0).value);
                    scan_token(csa_0);
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"infinity\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                    || the_same(
                        ((*csa_0).image).as_mut_ptr(),
                        b"inf\0" as *const u8 as *const i8 as *mut i8,
                    ) != 0
                {
                    if s < 0.0f64 {
                        error(
                            csa_0,
                            b"invalid use of '-inf' as upper bound\n\0" as *const u8
                                as *const i8,
                        );
                    }
                    set_upper_bound(csa_0, j, 1.7976931348623157e+308f64);
                    scan_token(csa_0);
                } else {
                    error(csa_0, b"missing upper bound\n\0" as *const u8 as *const i8);
                }
            } else if (*csa_0).token == 0xa as i32 {
                set_upper_bound(csa_0, j, (*csa_0).value);
                scan_token(csa_0);
            } else {
                error(csa_0, b"missing upper bound\n\0" as *const u8 as *const i8);
            }
        } else if (*csa_0).token == 0xf as i32 {
            if lb_flag != 0 {
                error(csa_0, b"invalid bound definition\n\0" as *const u8 as *const i8);
            }
            scan_token(csa_0);
            if (*csa_0).token == 0xb as i32 || (*csa_0).token == 0xc as i32 {
                s = if (*csa_0).token == 0xb as i32 { 1.0f64 } else { -1.0f64 };
                scan_token(csa_0);
                if (*csa_0).token == 0xa as i32 {
                    set_lower_bound(csa_0, j, s * (*csa_0).value);
                    scan_token(csa_0);
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"infinity\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                    || the_same(
                        ((*csa_0).image).as_mut_ptr(),
                        b"inf\0" as *const u8 as *const i8 as *mut i8,
                    ) == 0 as i32
                {
                    if s > 0.0f64 {
                        error(
                            csa_0,
                            b"invalid use of '+inf' as lower bound\n\0" as *const u8
                                as *const i8,
                        );
                    }
                    set_lower_bound(csa_0, j, -1.7976931348623157e+308f64);
                    scan_token(csa_0);
                } else {
                    error(csa_0, b"missing lower bound\n\0" as *const u8 as *const i8);
                }
            } else if (*csa_0).token == 0xa as i32 {
                set_lower_bound(csa_0, j, (*csa_0).value);
                scan_token(csa_0);
            } else {
                error(csa_0, b"missing lower bound\n\0" as *const u8 as *const i8);
            }
        } else if (*csa_0).token == 0x10 as i32 {
            if lb_flag != 0 {
                error(csa_0, b"invalid bound definition\n\0" as *const u8 as *const i8);
            }
            scan_token(csa_0);
            if (*csa_0).token == 0xb as i32 || (*csa_0).token == 0xc as i32 {
                s = if (*csa_0).token == 0xb as i32 { 1.0f64 } else { -1.0f64 };
                scan_token(csa_0);
                if (*csa_0).token == 0xa as i32 {
                    set_lower_bound(csa_0, j, s * (*csa_0).value);
                    set_upper_bound(csa_0, j, s * (*csa_0).value);
                    scan_token(csa_0);
                } else {
                    error(csa_0, b"missing fixed value\n\0" as *const u8 as *const i8);
                }
            } else if (*csa_0).token == 0xa as i32 {
                set_lower_bound(csa_0, j, (*csa_0).value);
                set_upper_bound(csa_0, j, (*csa_0).value);
                scan_token(csa_0);
            } else {
                error(csa_0, b"missing fixed value\n\0" as *const u8 as *const i8);
            }
        } else if the_same(
            ((*csa_0).image).as_mut_ptr(),
            b"free\0" as *const u8 as *const i8 as *mut i8,
        ) != 0
        {
            if lb_flag != 0 {
                error(csa_0, b"invalid bound definition\n\0" as *const u8 as *const i8);
            }
            set_lower_bound(csa_0, j, -1.7976931348623157e+308f64);
            set_upper_bound(csa_0, j, 1.7976931348623157e+308f64);
            scan_token(csa_0);
        } else if lb_flag == 0 {
            error(csa_0, b"invalid bound definition\n\0" as *const u8 as *const i8);
        }
    }
}
unsafe extern "C" fn parse_integer(mut csa_0: *mut csa) {
    let mut j: i32 = 0;
    let mut binary: i32 = 0;
    if (*csa_0).token == 0x5 as i32 {
        binary = 0 as i32;
        scan_token(csa_0);
    } else if (*csa_0).token == 0x6 as i32 {
        binary = 0 as i32;
        scan_token(csa_0);
    } else if (*csa_0).token == 0x7 as i32 {
        binary = 1 as i32;
        scan_token(csa_0);
    } else {
        (csa_0 != csa_0
            || {
                glp_assert_(
                    b"csa != csa\0" as *const u8 as *const i8,
                    b"api/cplex.c\0" as *const u8 as *const i8,
                    817 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    while (*csa_0).token == 0x9 as i32 {
        j = find_col(csa_0, ((*csa_0).image).as_mut_ptr());
        glp_set_col_kind((*csa_0).P, j, 2 as i32);
        if binary != 0 {
            set_lower_bound(
                csa_0,
                j,
                if *((*csa_0).lb).offset(j as isize) == 1.7976931348623157e+308f64 {
                    0.0f64
                } else {
                    *((*csa_0).lb).offset(j as isize)
                },
            );
            set_upper_bound(
                csa_0,
                j,
                if *((*csa_0).ub).offset(j as isize) == -1.7976931348623157e+308f64 {
                    1.0f64
                } else {
                    *((*csa_0).ub).offset(j as isize)
                },
            );
        }
        scan_token(csa_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_read_lp(
    mut P: *mut glp_prob,
    mut parm: *const glp_cpxcp,
    mut fname: *const i8,
) -> i32 {
    let mut _parm: glp_cpxcp = glp_cpxcp { foo_bar: [0.; 20] };
    let mut _csa: csa = csa {
        P: 0 as *mut glp_prob,
        parm: 0 as *const glp_cpxcp,
        fname: 0 as *const i8,
        fp: 0 as *mut glp_file,
        jump: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        count: 0,
        c: 0,
        token: 0,
        image: [0; 256],
        imlen: 0,
        value: 0.,
        n_max: 0,
        ind: 0 as *mut i32,
        val: 0 as *mut libc::c_double,
        flag: 0 as *mut i8,
        lb: 0 as *mut libc::c_double,
        ub: 0 as *mut libc::c_double,
        lb_warn: 0,
        ub_warn: 0,
    };
    let mut csa_0: *mut csa = &mut _csa;
    let mut ret: i32 = 0;
    glp_printf(
        b"Reading problem data from '%s'...\n\0" as *const u8 as *const i8,
        fname,
    );
    if parm.is_null() {
        glp_init_cpxcp(&mut _parm);
        parm = &mut _parm;
    }
    check_parm(b"glp_read_lp\0" as *const u8 as *const i8, parm);
    (*csa_0).P = P;
    (*csa_0).parm = parm;
    (*csa_0).fname = fname;
    (*csa_0).fp = 0 as *mut glp_file;
    if _setjmp(((*csa_0).jump).as_mut_ptr()) != 0 {
        ret = 1 as i32;
    } else {
        (*csa_0).count = 0 as i32;
        (*csa_0).c = '\n' as i32;
        (*csa_0).token = 0 as i32;
        (*csa_0).image[0 as i32 as usize] = '\0' as i32 as i8;
        (*csa_0).imlen = 0 as i32;
        (*csa_0).value = 0.0f64;
        (*csa_0).n_max = 100 as i32;
        (*csa_0).ind = glp_alloc(
            1 as i32 + (*csa_0).n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*csa_0).val = glp_alloc(
            1 as i32 + (*csa_0).n_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        (*csa_0).flag = glp_alloc(
            1 as i32 + (*csa_0).n_max,
            ::core::mem::size_of::<i8>() as u64 as i32,
        ) as *mut i8;
        memset(
            &mut *((*csa_0).flag).offset(1 as i32 as isize) as *mut i8
                as *mut libc::c_void,
            0 as i32,
            ((*csa_0).n_max as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
        (*csa_0).lb = glp_alloc(
            1 as i32 + (*csa_0).n_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        (*csa_0).ub = glp_alloc(
            1 as i32 + (*csa_0).n_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        (*csa_0).ub_warn = 0 as i32;
        (*csa_0).lb_warn = (*csa_0).ub_warn;
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
            scan_token(csa_0);
            if !((*csa_0).token == 0x1 as i32 || (*csa_0).token == 0x2 as i32) {
                error(
                    csa_0,
                    b"'minimize' or 'maximize' keyword missing\n\0" as *const u8
                        as *const i8,
                );
            }
            parse_objective(csa_0);
            if (*csa_0).token != 0x3 as i32 {
                error(
                    csa_0,
                    b"constraints section missing\n\0" as *const u8 as *const i8,
                );
            }
            parse_constraints(csa_0);
            if (*csa_0).token == 0x4 as i32 {
                parse_bounds(csa_0);
            }
            while (*csa_0).token == 0x5 as i32 || (*csa_0).token == 0x6 as i32
                || (*csa_0).token == 0x7 as i32
            {
                parse_integer(csa_0);
            }
            if (*csa_0).token == 0x8 as i32 {
                scan_token(csa_0);
            } else if (*csa_0).token == 0 as i32 {
                warning(csa_0, b"keyword 'end' missing\n\0" as *const u8 as *const i8);
            } else {
                error(
                    csa_0,
                    b"symbol '%s' in wrong position\n\0" as *const u8 as *const i8,
                    ((*csa_0).image).as_mut_ptr(),
                );
            }
            if (*csa_0).token != 0 as i32 {
                error(
                    csa_0,
                    b"extra symbol(s) detected beyond 'end'\n\0" as *const u8
                        as *const i8,
                );
            }
            let mut j: i32 = 0;
            let mut type_0: i32 = 0;
            let mut lb: libc::c_double = 0.;
            let mut ub: libc::c_double = 0.;
            j = 1 as i32;
            while j <= (*P).n {
                lb = *((*csa_0).lb).offset(j as isize);
                ub = *((*csa_0).ub).offset(j as isize);
                if lb == 1.7976931348623157e+308f64 {
                    lb = 0.0f64;
                }
                if ub == -1.7976931348623157e+308f64 {
                    ub = 1.7976931348623157e+308f64;
                }
                if lb == -1.7976931348623157e+308f64 && ub == 1.7976931348623157e+308f64
                {
                    type_0 = 1 as i32;
                } else if ub == 1.7976931348623157e+308f64 {
                    type_0 = 2 as i32;
                } else if lb == -1.7976931348623157e+308f64 {
                    type_0 = 3 as i32;
                } else if lb != ub {
                    type_0 = 4 as i32;
                } else {
                    type_0 = 5 as i32;
                }
                glp_set_col_bnds((*csa_0).P, j, type_0, lb, ub);
                j += 1;
                j;
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
                b"%d lines were read\n\0" as *const u8 as *const i8,
                (*csa_0).count,
            );
            glp_delete_index(P);
            glp_sort_matrix(P);
            ret = 0 as i32;
        }
    }
    if !((*csa_0).fp).is_null() {
        _glp_close((*csa_0).fp);
    }
    glp_free((*csa_0).ind as *mut libc::c_void);
    glp_free((*csa_0).val as *mut libc::c_void);
    glp_free((*csa_0).flag as *mut libc::c_void);
    glp_free((*csa_0).lb as *mut libc::c_void);
    glp_free((*csa_0).ub as *mut libc::c_void);
    if ret != 0 as i32 {
        glp_erase_prob(P);
    }
    return ret;
}
unsafe extern "C" fn check_name(mut name: *mut i8) -> i32 {
    if *name as i32 == '.' as i32 {
        return 1 as i32;
    }
    if *(*__ctype_b_loc()).offset(*name as u8 as i32 as isize) as i32
        & _ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        return 1 as i32;
    }
    while *name != 0 {
        if *(*__ctype_b_loc()).offset(*name as u8 as i32 as isize) as i32
            & _ISalnum as i32 as libc::c_ushort as i32 == 0
            && (strchr(
                b"!\"#$%&()/,.;?@_`'{}|~\0" as *const u8 as *const i8,
                *name as u8 as i32,
            ))
                .is_null()
        {
            return 1 as i32;
        }
        name = name.offset(1);
        name;
    }
    return 0 as i32;
}
unsafe extern "C" fn adjust_name(mut name: *mut i8) {
    while *name != 0 {
        if *name as i32 == ' ' as i32 {
            *name = '_' as i32 as i8;
        } else if *name as i32 == '-' as i32 {
            *name = '~' as i32 as i8;
        } else if *name as i32 == '[' as i32 {
            *name = '(' as i32 as i8;
        } else if *name as i32 == ']' as i32 {
            *name = ')' as i32 as i8;
        }
        name = name.offset(1);
        name;
    }
}
unsafe extern "C" fn row_name(
    mut csa1: *mut csa1,
    mut i: i32,
    mut rname: *mut i8,
) -> *mut i8 {
    let mut name: *const i8 = 0 as *const i8;
    if i == 0 as i32 {
        name = glp_get_obj_name((*csa1).P);
    } else {
        name = glp_get_row_name((*csa1).P, i);
    }
    if !name.is_null() {
        strcpy(rname, name);
        adjust_name(rname);
        if !(check_name(rname) != 0) {
            return rname;
        }
    }
    if i == 0 as i32 {
        strcpy(rname, b"obj\0" as *const u8 as *const i8);
    } else {
        sprintf(rname, b"r_%d\0" as *const u8 as *const i8, i);
    }
    return rname;
}
unsafe extern "C" fn col_name(
    mut csa1: *mut csa1,
    mut j: i32,
    mut cname: *mut i8,
) -> *mut i8 {
    let mut name: *const i8 = 0 as *const i8;
    name = glp_get_col_name((*csa1).P, j);
    if !name.is_null() {
        strcpy(cname, name);
        adjust_name(cname);
        if !(check_name(cname) != 0) {
            return cname;
        }
    }
    let mut col: *mut GLPCOL = *((*(*csa1).P).col).offset(j as isize);
    if (*col).type_0 == 5 as i32 {
        sprintf(cname, b"s_%d\0" as *const u8 as *const i8, j);
    } else if (*col).kind == 1 as i32 {
        sprintf(cname, b"x_%d\0" as *const u8 as *const i8, j);
    } else if !((*col).lb == 0 as i32 as libc::c_double
        && (*col).ub == 1 as i32 as libc::c_double)
    {
        sprintf(cname, b"y_%d\0" as *const u8 as *const i8, j);
    } else {
        sprintf(cname, b"z_%d\0" as *const u8 as *const i8, j);
    }
    return cname;
}
#[no_mangle]
pub unsafe extern "C" fn glp_write_lp(
    mut P: *mut glp_prob,
    mut parm: *const glp_cpxcp,
    mut fname: *const i8,
) -> i32 {
    let mut _parm: glp_cpxcp = glp_cpxcp { foo_bar: [0.; 20] };
    let mut _csa: csa1 = csa1 {
        P: 0 as *mut glp_prob,
        parm: 0 as *const glp_cpxcp,
    };
    let mut csa1: *mut csa1 = &mut _csa;
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut len: i32 = 0;
    let mut flag: i32 = 0;
    let mut count: i32 = 0;
    let mut ret: i32 = 0;
    let mut line: [i8; 1001] = [0; 1001];
    let mut term: [i8; 501] = [0; 501];
    let mut name: [i8; 256] = [0; 256];
    glp_printf(b"Writing problem data to '%s'...\n\0" as *const u8 as *const i8, fname);
    if parm.is_null() {
        glp_init_cpxcp(&mut _parm);
        parm = &mut _parm;
    }
    check_parm(b"glp_write_lp\0" as *const u8 as *const i8, parm);
    (*csa1).P = P;
    (*csa1).parm = parm;
    fp = _glp_open(fname, b"w\0" as *const u8 as *const i8);
    count = 0 as i32;
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
            b"\\* Problem: %s *\\\n\0" as *const u8 as *const i8,
            (if ((*P).name).is_null() {
                b"Unknown\0" as *const u8 as *const i8
            } else {
                (*P).name
            }),
        );
        count += 1;
        count;
        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
        count += 1;
        count;
        if !((*P).m > 0 as i32 && (*P).n > 0 as i32) {
            glp_printf(
                b"Warning: problem has no rows/columns\n\0" as *const u8 as *const i8,
            );
            _glp_format(
                fp,
                b"\\* WARNING: PROBLEM HAS NO ROWS/COLUMNS *\\\n\0" as *const u8
                    as *const i8,
            );
            count += 1;
            count;
            _glp_format(fp, b"\n\0" as *const u8 as *const i8);
            count += 1;
            count;
        } else {
            if (*P).dir == 1 as i32 {
                _glp_format(fp, b"Minimize\n\0" as *const u8 as *const i8);
                count += 1;
                count;
            } else if (*P).dir == 2 as i32 {
                _glp_format(fp, b"Maximize\n\0" as *const u8 as *const i8);
                count += 1;
                count;
            } else {
                (P != P
                    || {
                        glp_assert_(
                            b"P != P\0" as *const u8 as *const i8,
                            b"api/cplex.c\0" as *const u8 as *const i8,
                            1136 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            row_name(csa1, 0 as i32, name.as_mut_ptr());
            sprintf(
                line.as_mut_ptr(),
                b" %s:\0" as *const u8 as *const i8,
                name.as_mut_ptr(),
            );
            len = 0 as i32;
            j = 1 as i32;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if (*col).coef != 0.0f64 || ((*col).ptr).is_null() {
                    len += 1;
                    len;
                    col_name(csa1, j, name.as_mut_ptr());
                    if (*col).coef == 0.0f64 {
                        sprintf(
                            term.as_mut_ptr(),
                            b" + 0 %s\0" as *const u8 as *const i8,
                            name.as_mut_ptr(),
                        );
                    } else if (*col).coef == 1.0f64 {
                        sprintf(
                            term.as_mut_ptr(),
                            b" + %s\0" as *const u8 as *const i8,
                            name.as_mut_ptr(),
                        );
                    } else if (*col).coef == -1.0f64 {
                        sprintf(
                            term.as_mut_ptr(),
                            b" - %s\0" as *const u8 as *const i8,
                            name.as_mut_ptr(),
                        );
                    } else if (*col).coef > 0.0f64 {
                        sprintf(
                            term.as_mut_ptr(),
                            b" + %.*g %s\0" as *const u8 as *const i8,
                            15 as i32,
                            (*col).coef,
                            name.as_mut_ptr(),
                        );
                    } else {
                        sprintf(
                            term.as_mut_ptr(),
                            b" - %.*g %s\0" as *const u8 as *const i8,
                            15 as i32,
                            -(*col).coef,
                            name.as_mut_ptr(),
                        );
                    }
                    if (strlen(line.as_mut_ptr()))
                        .wrapping_add(strlen(term.as_mut_ptr())) > 72 as i32 as u64
                    {
                        _glp_format(
                            fp,
                            b"%s\n\0" as *const u8 as *const i8,
                            line.as_mut_ptr(),
                        );
                        line[0 as i32 as usize] = '\0' as i32 as i8;
                        count += 1;
                        count;
                    }
                    strcat(line.as_mut_ptr(), term.as_mut_ptr());
                }
                j += 1;
                j;
            }
            if len == 0 as i32 {
                sprintf(
                    term.as_mut_ptr(),
                    b" 0 %s\0" as *const u8 as *const i8,
                    col_name(csa1, 1 as i32, name.as_mut_ptr()),
                );
                strcat(line.as_mut_ptr(), term.as_mut_ptr());
            }
            _glp_format(fp, b"%s\n\0" as *const u8 as *const i8, line.as_mut_ptr());
            count += 1;
            count;
            if (*P).c0 != 0.0f64 {
                _glp_format(
                    fp,
                    b"\\* constant term = %.*g *\\\n\0" as *const u8 as *const i8,
                    15 as i32,
                    (*P).c0,
                );
                count += 1;
                count;
            }
            _glp_format(fp, b"\n\0" as *const u8 as *const i8);
            count += 1;
            count;
            _glp_format(fp, b"Subject To\n\0" as *const u8 as *const i8);
            count += 1;
            count;
            i = 1 as i32;
            while i <= (*P).m {
                row = *((*P).row).offset(i as isize);
                if !((*row).type_0 == 1 as i32) {
                    row_name(csa1, i, name.as_mut_ptr());
                    sprintf(
                        line.as_mut_ptr(),
                        b" %s:\0" as *const u8 as *const i8,
                        name.as_mut_ptr(),
                    );
                    aij = (*row).ptr;
                    while !aij.is_null() {
                        col_name(csa1, (*(*aij).col).j, name.as_mut_ptr());
                        if (*aij).val == 1.0f64 {
                            sprintf(
                                term.as_mut_ptr(),
                                b" + %s\0" as *const u8 as *const i8,
                                name.as_mut_ptr(),
                            );
                        } else if (*aij).val == -1.0f64 {
                            sprintf(
                                term.as_mut_ptr(),
                                b" - %s\0" as *const u8 as *const i8,
                                name.as_mut_ptr(),
                            );
                        } else if (*aij).val > 0.0f64 {
                            sprintf(
                                term.as_mut_ptr(),
                                b" + %.*g %s\0" as *const u8 as *const i8,
                                15 as i32,
                                (*aij).val,
                                name.as_mut_ptr(),
                            );
                        } else {
                            sprintf(
                                term.as_mut_ptr(),
                                b" - %.*g %s\0" as *const u8 as *const i8,
                                15 as i32,
                                -(*aij).val,
                                name.as_mut_ptr(),
                            );
                        }
                        if (strlen(line.as_mut_ptr()))
                            .wrapping_add(strlen(term.as_mut_ptr())) > 72 as i32 as u64
                        {
                            _glp_format(
                                fp,
                                b"%s\n\0" as *const u8 as *const i8,
                                line.as_mut_ptr(),
                            );
                            line[0 as i32 as usize] = '\0' as i32 as i8;
                            count += 1;
                            count;
                        }
                        strcat(line.as_mut_ptr(), term.as_mut_ptr());
                        aij = (*aij).r_next;
                    }
                    if (*row).type_0 == 4 as i32 {
                        sprintf(
                            term.as_mut_ptr(),
                            b" - ~r_%d\0" as *const u8 as *const i8,
                            i,
                        );
                        if (strlen(line.as_mut_ptr()))
                            .wrapping_add(strlen(term.as_mut_ptr())) > 72 as i32 as u64
                        {
                            _glp_format(
                                fp,
                                b"%s\n\0" as *const u8 as *const i8,
                                line.as_mut_ptr(),
                            );
                            line[0 as i32 as usize] = '\0' as i32 as i8;
                            count += 1;
                            count;
                        }
                        strcat(line.as_mut_ptr(), term.as_mut_ptr());
                    } else if ((*row).ptr).is_null() {
                        sprintf(
                            term.as_mut_ptr(),
                            b" 0 %s\0" as *const u8 as *const i8,
                            col_name(csa1, 1 as i32, name.as_mut_ptr()),
                        );
                        strcat(line.as_mut_ptr(), term.as_mut_ptr());
                    }
                    if (*row).type_0 == 2 as i32 {
                        sprintf(
                            term.as_mut_ptr(),
                            b" >= %.*g\0" as *const u8 as *const i8,
                            15 as i32,
                            (*row).lb,
                        );
                    } else if (*row).type_0 == 3 as i32 {
                        sprintf(
                            term.as_mut_ptr(),
                            b" <= %.*g\0" as *const u8 as *const i8,
                            15 as i32,
                            (*row).ub,
                        );
                    } else if (*row).type_0 == 4 as i32 || (*row).type_0 == 5 as i32 {
                        sprintf(
                            term.as_mut_ptr(),
                            b" = %.*g\0" as *const u8 as *const i8,
                            15 as i32,
                            (*row).lb,
                        );
                    } else {
                        (row != row
                            || {
                                glp_assert_(
                                    b"row != row\0" as *const u8 as *const i8,
                                    b"api/cplex.c\0" as *const u8 as *const i8,
                                    1212 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                    if (strlen(line.as_mut_ptr()))
                        .wrapping_add(strlen(term.as_mut_ptr())) > 72 as i32 as u64
                    {
                        _glp_format(
                            fp,
                            b"%s\n\0" as *const u8 as *const i8,
                            line.as_mut_ptr(),
                        );
                        line[0 as i32 as usize] = '\0' as i32 as i8;
                        count += 1;
                        count;
                    }
                    strcat(line.as_mut_ptr(), term.as_mut_ptr());
                    _glp_format(
                        fp,
                        b"%s\n\0" as *const u8 as *const i8,
                        line.as_mut_ptr(),
                    );
                    count += 1;
                    count;
                }
                i += 1;
                i;
            }
            _glp_format(fp, b"\n\0" as *const u8 as *const i8);
            count += 1;
            count;
            flag = 0 as i32;
            i = 1 as i32;
            while i <= (*P).m {
                row = *((*P).row).offset(i as isize);
                if !((*row).type_0 != 4 as i32) {
                    if flag == 0 {
                        _glp_format(fp, b"Bounds\n\0" as *const u8 as *const i8);
                        flag = 1 as i32;
                        count += 1;
                        count;
                    }
                    _glp_format(
                        fp,
                        b" 0 <= ~r_%d <= %.*g\n\0" as *const u8 as *const i8,
                        i,
                        15 as i32,
                        (*row).ub - (*row).lb,
                    );
                    count += 1;
                    count;
                }
                i += 1;
                i;
            }
            j = 1 as i32;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if !((*col).type_0 == 2 as i32 && (*col).lb == 0.0f64) {
                    if flag == 0 {
                        _glp_format(fp, b"Bounds\n\0" as *const u8 as *const i8);
                        flag = 1 as i32;
                        count += 1;
                        count;
                    }
                    col_name(csa1, j, name.as_mut_ptr());
                    if (*col).type_0 == 1 as i32 {
                        _glp_format(
                            fp,
                            b" %s free\n\0" as *const u8 as *const i8,
                            name.as_mut_ptr(),
                        );
                        count += 1;
                        count;
                    } else if (*col).type_0 == 2 as i32 {
                        _glp_format(
                            fp,
                            b" %s >= %.*g\n\0" as *const u8 as *const i8,
                            name.as_mut_ptr(),
                            15 as i32,
                            (*col).lb,
                        );
                        count += 1;
                        count;
                    } else if (*col).type_0 == 3 as i32 {
                        _glp_format(
                            fp,
                            b" -Inf <= %s <= %.*g\n\0" as *const u8 as *const i8,
                            name.as_mut_ptr(),
                            15 as i32,
                            (*col).ub,
                        );
                        count += 1;
                        count;
                    } else if (*col).type_0 == 4 as i32 {
                        _glp_format(
                            fp,
                            b" %.*g <= %s <= %.*g\n\0" as *const u8 as *const i8,
                            15 as i32,
                            (*col).lb,
                            name.as_mut_ptr(),
                            15 as i32,
                            (*col).ub,
                        );
                        count += 1;
                        count;
                    } else if (*col).type_0 == 5 as i32 {
                        _glp_format(
                            fp,
                            b" %s = %.*g\n\0" as *const u8 as *const i8,
                            name.as_mut_ptr(),
                            15 as i32,
                            (*col).lb,
                        );
                        count += 1;
                        count;
                    } else {
                        (col != col
                            || {
                                glp_assert_(
                                    b"col != col\0" as *const u8 as *const i8,
                                    b"api/cplex.c\0" as *const u8 as *const i8,
                                    1250 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                }
                j += 1;
                j;
            }
            if flag != 0 {
                _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                count += 1;
                count;
            }
            flag = 0 as i32;
            j = 1 as i32;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if !((*col).kind == 1 as i32) {
                    ((*col).kind == 2 as i32
                        || {
                            glp_assert_(
                                b"col->kind == GLP_IV\0" as *const u8 as *const i8,
                                b"api/cplex.c\0" as *const u8 as *const i8,
                                1258 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if flag == 0 {
                        _glp_format(fp, b"Generals\n\0" as *const u8 as *const i8);
                        flag = 1 as i32;
                        count += 1;
                        count;
                    }
                    _glp_format(
                        fp,
                        b" %s\n\0" as *const u8 as *const i8,
                        col_name(csa1, j, name.as_mut_ptr()),
                    );
                    count += 1;
                    count;
                }
                j += 1;
                j;
            }
            if flag != 0 {
                _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                count += 1;
                count;
            }
        }
        _glp_format(fp, b"End\n\0" as *const u8 as *const i8);
        count += 1;
        count;
        if _glp_ioerr(fp) != 0 {
            glp_printf(
                b"Write error on '%s' - %s\n\0" as *const u8 as *const i8,
                fname,
                _glp_get_err_msg(),
            );
            ret = 1 as i32;
        } else {
            glp_printf(b"%d lines were written\n\0" as *const u8 as *const i8, count);
            ret = 0 as i32;
        }
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}