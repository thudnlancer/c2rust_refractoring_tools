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
    fn __errno_location() -> *mut i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_xstrerr(_: i32) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn _glp_str2int(str: *const i8, val: *mut i32) -> i32;
    fn _glp_str2num(str: *const i8, val: *mut libc::c_double) -> i32;
    fn _glp_strspx(str: *mut i8) -> *mut i8;
    fn _glp_strtrim(str: *mut i8) -> *mut i8;
}
pub type __int32_t = i32;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HBM {
    pub title: [i8; 73],
    pub key: [i8; 9],
    pub mxtype: [i8; 4],
    pub rhstyp: [i8; 4],
    pub ptrfmt: [i8; 17],
    pub indfmt: [i8; 17],
    pub valfmt: [i8; 21],
    pub rhsfmt: [i8; 21],
    pub totcrd: i32,
    pub ptrcrd: i32,
    pub indcrd: i32,
    pub valcrd: i32,
    pub rhscrd: i32,
    pub nrow: i32,
    pub ncol: i32,
    pub nnzero: i32,
    pub neltvl: i32,
    pub nrhs: i32,
    pub nrhsix: i32,
    pub nrhsvl: i32,
    pub nguess: i32,
    pub nexact: i32,
    pub colptr: *mut i32,
    pub rowind: *mut i32,
    pub rhsptr: *mut i32,
    pub rhsind: *mut i32,
    pub values: *mut libc::c_double,
    pub rhsval: *mut libc::c_double,
    pub sguess: *mut libc::c_double,
    pub xexact: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa {
    pub fname: *const i8,
    pub fp: *mut FILE,
    pub seqn: i32,
    pub card: [i8; 81],
    pub fmt_p: i32,
    pub fmt_k: i32,
    pub fmt_f: i32,
    pub fmt_w: i32,
    pub fmt_d: i32,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn read_card(mut dsa: *mut dsa) -> i32 {
    let mut current_block: u64;
    let mut c: i32 = 0;
    let mut len: i32 = 0 as i32;
    let mut buf: [i8; 256] = [0; 256];
    (*dsa).seqn += 1;
    (*dsa).seqn;
    loop {
        c = fgetc((*dsa).fp);
        if c == -(1 as i32) {
            if ferror((*dsa).fp) != 0 {
                glp_printf(
                    b"%s:%d: read error\n\0" as *const u8 as *const i8,
                    (*dsa).fname,
                    (*dsa).seqn,
                );
            } else {
                glp_printf(
                    b"%s:%d: unexpected end-of-file\n\0" as *const u8 as *const i8,
                    (*dsa).fname,
                    (*dsa).seqn,
                );
            }
            return 1 as i32;
        } else {
            if c == '\r' as i32 {
                continue;
            }
            if c == '\n' as i32 {
                current_block = 13056961889198038528;
                break;
            }
            if *(*__ctype_b_loc()).offset(c as isize) as i32
                & _IScntrl as i32 as libc::c_ushort as i32 != 0
            {
                glp_printf(
                    b"%s:%d: invalid control character\n\0" as *const u8 as *const i8,
                    (*dsa).fname,
                    (*dsa).seqn,
                    c,
                );
                return 1 as i32;
            } else {
                if len as u64
                    == (::core::mem::size_of::<[i8; 256]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                {
                    current_block = 14160977806067770390;
                    break;
                }
                let fresh0 = len;
                len = len + 1;
                buf[fresh0 as usize] = c as i8;
            }
        }
    }
    match current_block {
        13056961889198038528 => {
            while len > 80 as i32 && buf[(len - 1 as i32) as usize] as i32 == ' ' as i32
            {
                len -= 1;
                len;
            }
            buf[len as usize] = '\0' as i32 as i8;
            if !(len > 80 as i32) {
                strcpy(((*dsa).card).as_mut_ptr(), buf.as_mut_ptr());
                memset(
                    &mut *((*dsa).card).as_mut_ptr().offset(len as isize) as *mut i8
                        as *mut libc::c_void,
                    ' ' as i32,
                    (80 as i32 - len) as u64,
                );
                (*dsa).card[80 as i32 as usize] = '\0' as i32 as i8;
                return 0 as i32;
            }
        }
        _ => {}
    }
    (glp_error_(b"misc/hbm.c\0" as *const u8 as *const i8, 119 as i32))
        .expect(
            "non-null function pointer",
        )(
        b"%s:%d: card image too long\n\0" as *const u8 as *const i8,
        (*dsa).fname,
        (*dsa).seqn,
    );
    return 1 as i32;
}
unsafe extern "C" fn scan_int(
    mut dsa: *mut dsa,
    mut fld: *mut i8,
    mut pos: i32,
    mut width: i32,
    mut val: *mut i32,
) -> i32 {
    let mut str: [i8; 81] = [0; 81];
    (1 as i32 <= width && width <= 80 as i32
        || {
            glp_assert_(
                b"1 <= width && width <= 80\0" as *const u8 as *const i8,
                b"misc/hbm.c\0" as *const u8 as *const i8,
                143 as i32,
            );
            1 as i32 != 0
        }) as i32;
    memcpy(
        str.as_mut_ptr() as *mut libc::c_void,
        ((*dsa).card).as_mut_ptr().offset(pos as isize) as *const libc::c_void,
        width as u64,
    );
    str[width as usize] = '\0' as i32 as i8;
    if _glp_str2int(_glp_strspx(str.as_mut_ptr()), val) != 0 {
        glp_printf(
            b"%s:%d: field '%s' contains invalid value '%s'\n\0" as *const u8
                as *const i8,
            (*dsa).fname,
            (*dsa).seqn,
            fld,
            str.as_mut_ptr(),
        );
        return 1 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn parse_fmt(mut dsa: *mut dsa, mut fmt: *mut i8) -> i32 {
    let mut current_block: u64;
    let mut k: i32 = 0;
    let mut s: i32 = 0;
    let mut val: i32 = 0;
    let mut str: [i8; 81] = [0; 81];
    if !(*fmt.offset(0 as i32 as isize) as i32 != '(' as i32) {
        k = 1 as i32;
        (*dsa).fmt_p = 0 as i32;
        if *(*__ctype_b_loc()).offset(*fmt.offset(k as isize) as u8 as i32 as isize)
            as i32 & _ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            s = 0 as i32;
            loop {
                if !(*(*__ctype_b_loc())
                    .offset(*fmt.offset(k as isize) as u8 as i32 as isize) as i32
                    & _ISdigit as i32 as libc::c_ushort as i32 != 0)
                {
                    current_block = 13513818773234778473;
                    break;
                }
                if s == 80 as i32 {
                    current_block = 10855290414189143302;
                    break;
                }
                let fresh1 = k;
                k = k + 1;
                let fresh2 = s;
                s = s + 1;
                str[fresh2 as usize] = *fmt.offset(fresh1 as isize);
            }
            match current_block {
                10855290414189143302 => {}
                _ => {
                    str[s as usize] = '\0' as i32 as i8;
                    if _glp_str2int(str.as_mut_ptr(), &mut val) != 0 {
                        current_block = 10855290414189143302;
                    } else if ({
                        let mut __res: i32 = 0;
                        if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                            if 0 != 0 {
                                let mut __c: i32 = *fmt.offset(k as isize) as u8 as i32;
                                __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                    __c
                                } else {
                                    *(*__ctype_toupper_loc()).offset(__c as isize)
                                });
                            } else {
                                __res = toupper(*fmt.offset(k as isize) as u8 as i32);
                            }
                        } else {
                            __res = *(*__ctype_toupper_loc())
                                .offset(*fmt.offset(k as isize) as u8 as i32 as isize);
                        }
                        __res
                    }) != 'P' as i32
                    {
                        current_block = 16056372141605515937;
                    } else {
                        (*dsa).fmt_p = val;
                        k += 1;
                        k;
                        if !(0 as i32 <= (*dsa).fmt_p && (*dsa).fmt_p <= 255 as i32) {
                            current_block = 10855290414189143302;
                        } else {
                            if *fmt.offset(k as isize) as i32 == ',' as i32 {
                                k += 1;
                                k;
                            }
                            current_block = 9606288038608642794;
                        }
                    }
                }
            }
        } else {
            current_block = 9606288038608642794;
        }
        match current_block {
            10855290414189143302 => {}
            _ => {
                match current_block {
                    9606288038608642794 => {
                        (*dsa).fmt_k = 1 as i32;
                        if *(*__ctype_b_loc())
                            .offset(*fmt.offset(k as isize) as u8 as i32 as isize) as i32
                            & _ISdigit as i32 as libc::c_ushort as i32 != 0
                        {
                            s = 0 as i32;
                            loop {
                                if !(*(*__ctype_b_loc())
                                    .offset(*fmt.offset(k as isize) as u8 as i32 as isize)
                                    as i32 & _ISdigit as i32 as libc::c_ushort as i32 != 0)
                                {
                                    current_block = 26972500619410423;
                                    break;
                                }
                                if s == 80 as i32 {
                                    current_block = 10855290414189143302;
                                    break;
                                }
                                let fresh3 = k;
                                k = k + 1;
                                let fresh4 = s;
                                s = s + 1;
                                str[fresh4 as usize] = *fmt.offset(fresh3 as isize);
                            }
                            match current_block {
                                10855290414189143302 => {}
                                _ => {
                                    str[s as usize] = '\0' as i32 as i8;
                                    if _glp_str2int(str.as_mut_ptr(), &mut val) != 0 {
                                        current_block = 10855290414189143302;
                                    } else {
                                        current_block = 16056372141605515937;
                                    }
                                }
                            }
                        } else {
                            current_block = 2719512138335094285;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    10855290414189143302 => {}
                    _ => {
                        match current_block {
                            16056372141605515937 => {
                                (*dsa).fmt_k = val;
                                if !(1 as i32 <= (*dsa).fmt_k && (*dsa).fmt_k <= 255 as i32)
                                {
                                    current_block = 10855290414189143302;
                                } else {
                                    current_block = 2719512138335094285;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            10855290414189143302 => {}
                            _ => {
                                (*dsa).fmt_f = ({
                                    let mut __res: i32 = 0;
                                    if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                                        if 0 != 0 {
                                            let fresh5 = k;
                                            k = k + 1;
                                            let mut __c: i32 = *fmt.offset(fresh5 as isize) as u8
                                                as i32;
                                            __res = if __c < -(128 as i32) || __c > 255 as i32 {
                                                __c
                                            } else {
                                                *(*__ctype_toupper_loc()).offset(__c as isize)
                                            };
                                        } else {
                                            let fresh6 = k;
                                            k = k + 1;
                                            __res = toupper(*fmt.offset(fresh6 as isize) as u8 as i32);
                                        }
                                    } else {
                                        let fresh7 = k;
                                        k = k + 1;
                                        __res = *(*__ctype_toupper_loc())
                                            .offset(*fmt.offset(fresh7 as isize) as u8 as i32 as isize);
                                    }
                                    __res
                                });
                                if (*dsa).fmt_f == 'D' as i32 || (*dsa).fmt_f == 'E' as i32
                                    || (*dsa).fmt_f == 'F' as i32 || (*dsa).fmt_f == 'G' as i32
                                    || (*dsa).fmt_f == 'I' as i32
                                {
                                    if !(*(*__ctype_b_loc())
                                        .offset(*fmt.offset(k as isize) as u8 as i32 as isize)
                                        as i32 & _ISdigit as i32 as libc::c_ushort as i32 == 0)
                                    {
                                        s = 0 as i32;
                                        loop {
                                            if !(*(*__ctype_b_loc())
                                                .offset(*fmt.offset(k as isize) as u8 as i32 as isize)
                                                as i32 & _ISdigit as i32 as libc::c_ushort as i32 != 0)
                                            {
                                                current_block = 2569451025026770673;
                                                break;
                                            }
                                            if s == 80 as i32 {
                                                current_block = 10855290414189143302;
                                                break;
                                            }
                                            let fresh8 = k;
                                            k = k + 1;
                                            let fresh9 = s;
                                            s = s + 1;
                                            str[fresh9 as usize] = *fmt.offset(fresh8 as isize);
                                        }
                                        match current_block {
                                            10855290414189143302 => {}
                                            _ => {
                                                str[s as usize] = '\0' as i32 as i8;
                                                if !(_glp_str2int(str.as_mut_ptr(), &mut (*dsa).fmt_w) != 0)
                                                {
                                                    if 1 as i32 <= (*dsa).fmt_w && (*dsa).fmt_w <= 255 as i32 {
                                                        (*dsa).fmt_d = 0 as i32;
                                                        if *fmt.offset(k as isize) as i32 == '.' as i32 {
                                                            k += 1;
                                                            k;
                                                            if *(*__ctype_b_loc())
                                                                .offset(*fmt.offset(k as isize) as u8 as i32 as isize)
                                                                as i32 & _ISdigit as i32 as libc::c_ushort as i32 == 0
                                                            {
                                                                current_block = 10855290414189143302;
                                                            } else {
                                                                s = 0 as i32;
                                                                loop {
                                                                    if !(*(*__ctype_b_loc())
                                                                        .offset(*fmt.offset(k as isize) as u8 as i32 as isize)
                                                                        as i32 & _ISdigit as i32 as libc::c_ushort as i32 != 0)
                                                                    {
                                                                        current_block = 7226443171521532240;
                                                                        break;
                                                                    }
                                                                    if s == 80 as i32 {
                                                                        current_block = 10855290414189143302;
                                                                        break;
                                                                    }
                                                                    let fresh10 = k;
                                                                    k = k + 1;
                                                                    let fresh11 = s;
                                                                    s = s + 1;
                                                                    str[fresh11 as usize] = *fmt.offset(fresh10 as isize);
                                                                }
                                                                match current_block {
                                                                    10855290414189143302 => {}
                                                                    _ => {
                                                                        str[s as usize] = '\0' as i32 as i8;
                                                                        if _glp_str2int(str.as_mut_ptr(), &mut (*dsa).fmt_d) != 0 {
                                                                            current_block = 10855290414189143302;
                                                                        } else if !(0 as i32 <= (*dsa).fmt_d
                                                                            && (*dsa).fmt_d <= 255 as i32)
                                                                        {
                                                                            current_block = 10855290414189143302;
                                                                        } else {
                                                                            current_block = 721385680381463314;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 721385680381463314;
                                                        }
                                                        match current_block {
                                                            10855290414189143302 => {}
                                                            _ => {
                                                                if *fmt.offset(k as isize) as i32 == ')' as i32
                                                                    && *fmt.offset((k + 1 as i32) as isize) as i32
                                                                        == '\0' as i32
                                                                {
                                                                    return 0 as i32;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    glp_printf(
        b"hbm_read_mat: format '%s' not recognised\n\0" as *const u8 as *const i8,
        fmt,
    );
    return 1 as i32;
}
unsafe extern "C" fn read_int_array(
    mut dsa: *mut dsa,
    mut name: *mut i8,
    mut fmt: *mut i8,
    mut n: i32,
    mut val: *mut i32,
) -> i32 {
    let mut k: i32 = 0;
    let mut pos: i32 = 0;
    let mut str: [i8; 81] = [0; 81];
    if parse_fmt(dsa, fmt) != 0 {
        return 1 as i32;
    }
    if !((*dsa).fmt_f == 'I' as i32 && (*dsa).fmt_w <= 80 as i32
        && (*dsa).fmt_k * (*dsa).fmt_w <= 80 as i32)
    {
        glp_printf(
            b"%s:%d: can't read array '%s' - invalid format '%s'\n\0" as *const u8
                as *const i8,
            (*dsa).fname,
            (*dsa).seqn,
            name,
            fmt,
        );
        return 1 as i32;
    }
    k = 1 as i32;
    pos = 2147483647 as i32;
    while k <= n {
        if pos >= (*dsa).fmt_k {
            if read_card(dsa) != 0 {
                return 1 as i32;
            }
            pos = 0 as i32;
        }
        memcpy(
            str.as_mut_ptr() as *mut libc::c_void,
            ((*dsa).card).as_mut_ptr().offset(((*dsa).fmt_w * pos) as isize)
                as *const libc::c_void,
            (*dsa).fmt_w as u64,
        );
        str[(*dsa).fmt_w as usize] = '\0' as i32 as i8;
        _glp_strspx(str.as_mut_ptr());
        if _glp_str2int(str.as_mut_ptr(), &mut *val.offset(k as isize)) != 0 {
            glp_printf(
                b"%s:%d: can't read array '%s' - invalid value '%s'\n\0" as *const u8
                    as *const i8,
                (*dsa).fname,
                (*dsa).seqn,
                name,
                str.as_mut_ptr(),
            );
            return 1 as i32;
        }
        k += 1;
        k;
        pos += 1;
        pos;
    }
    return 0 as i32;
}
unsafe extern "C" fn read_real_array(
    mut dsa: *mut dsa,
    mut name: *mut i8,
    mut fmt: *mut i8,
    mut n: i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut k: i32 = 0;
    let mut pos: i32 = 0;
    let mut str: [i8; 81] = [0; 81];
    let mut ptr: *mut i8 = 0 as *mut i8;
    if parse_fmt(dsa, fmt) != 0 {
        return 1 as i32;
    }
    if !((*dsa).fmt_f != 'I' as i32 && (*dsa).fmt_w <= 80 as i32
        && (*dsa).fmt_k * (*dsa).fmt_w <= 80 as i32)
    {
        glp_printf(
            b"%s:%d: can't read array '%s' - invalid format '%s'\n\0" as *const u8
                as *const i8,
            (*dsa).fname,
            (*dsa).seqn,
            name,
            fmt,
        );
        return 1 as i32;
    }
    k = 1 as i32;
    pos = 2147483647 as i32;
    while k <= n {
        if pos >= (*dsa).fmt_k {
            if read_card(dsa) != 0 {
                return 1 as i32;
            }
            pos = 0 as i32;
        }
        memcpy(
            str.as_mut_ptr() as *mut libc::c_void,
            ((*dsa).card).as_mut_ptr().offset(((*dsa).fmt_w * pos) as isize)
                as *const libc::c_void,
            (*dsa).fmt_w as u64,
        );
        str[(*dsa).fmt_w as usize] = '\0' as i32 as i8;
        _glp_strspx(str.as_mut_ptr());
        if (strchr(str.as_mut_ptr(), '.' as i32)).is_null()
            && strcmp(str.as_mut_ptr(), b"0\0" as *const u8 as *const i8) != 0
        {
            glp_printf(
                b"%s(%d): can't read array '%s' - value '%s' has no decimal point\n\0"
                    as *const u8 as *const i8,
                (*dsa).fname,
                (*dsa).seqn,
                name,
                str.as_mut_ptr(),
            );
            return 1 as i32;
        }
        ptr = str.as_mut_ptr();
        while *ptr != 0 {
            *ptr = ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = *ptr as u8 as i32;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(*ptr as u8 as i32);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(*ptr as u8 as i32 as isize);
                }
                __res
            }) as i8;
            ptr = ptr.offset(1);
            ptr;
        }
        ptr = strchr(str.as_mut_ptr(), 'D' as i32);
        if !ptr.is_null() {
            *ptr = 'E' as i32 as i8;
        }
        ptr = strchr(str.as_mut_ptr().offset(1 as i32 as isize), '+' as i32);
        if ptr.is_null() {
            ptr = strchr(str.as_mut_ptr().offset(1 as i32 as isize), '-' as i32);
        }
        if !ptr.is_null() && *ptr.offset(-(1 as i32 as isize)) as i32 != 'E' as i32 {
            (strlen(str.as_mut_ptr()) < 80 as i32 as u64
                || {
                    glp_assert_(
                        b"strlen(str) < 80\0" as *const u8 as *const i8,
                        b"misc/hbm.c\0" as *const u8 as *const i8,
                        318 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            memmove(
                ptr.offset(1 as i32 as isize) as *mut libc::c_void,
                ptr as *const libc::c_void,
                (strlen(ptr)).wrapping_add(1 as i32 as u64),
            );
            *ptr = 'E' as i32 as i8;
        }
        if _glp_str2num(str.as_mut_ptr(), &mut *val.offset(k as isize)) != 0 {
            glp_printf(
                b"%s:%d: can't read array '%s' - invalid value '%s'\n\0" as *const u8
                    as *const i8,
                (*dsa).fname,
                (*dsa).seqn,
                name,
                str.as_mut_ptr(),
            );
            return 1 as i32;
        }
        k += 1;
        k;
        pos += 1;
        pos;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_hbm_read_mat(mut fname: *const i8) -> *mut HBM {
    let mut current_block: u64;
    let mut _dsa: dsa = dsa {
        fname: 0 as *const i8,
        fp: 0 as *mut FILE,
        seqn: 0,
        card: [0; 81],
        fmt_p: 0,
        fmt_k: 0,
        fmt_f: 0,
        fmt_w: 0,
        fmt_d: 0,
    };
    let mut dsa: *mut dsa = &mut _dsa;
    let mut hbm: *mut HBM = 0 as *mut HBM;
    (*dsa).fname = fname;
    glp_printf(
        b"hbm_read_mat: reading matrix from '%s'...\n\0" as *const u8 as *const i8,
        (*dsa).fname,
    );
    (*dsa).fp = fopen((*dsa).fname, b"r\0" as *const u8 as *const i8);
    if ((*dsa).fp).is_null() {
        glp_printf(
            b"hbm_read_mat: unable to open '%s' - %s\n\0" as *const u8 as *const i8,
            (*dsa).fname,
            _glp_xstrerr(*__errno_location()),
        );
    } else {
        (*dsa).seqn = 0 as i32;
        hbm = glp_alloc(1 as i32, ::core::mem::size_of::<HBM>() as u64 as i32)
            as *mut HBM;
        memset(hbm as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<HBM>() as u64);
        if !(read_card(dsa) != 0) {
            memcpy(
                ((*hbm).title).as_mut_ptr() as *mut libc::c_void,
                ((*dsa).card).as_mut_ptr() as *const libc::c_void,
                72 as i32 as u64,
            );
            (*hbm).title[72 as i32 as usize] = '\0' as i32 as i8;
            _glp_strtrim(((*hbm).title).as_mut_ptr());
            glp_printf(b"%s\n\0" as *const u8 as *const i8, ((*hbm).title).as_mut_ptr());
            memcpy(
                ((*hbm).key).as_mut_ptr() as *mut libc::c_void,
                ((*dsa).card).as_mut_ptr().offset(72 as i32 as isize)
                    as *const libc::c_void,
                8 as i32 as u64,
            );
            (*hbm).key[8 as i32 as usize] = '\0' as i32 as i8;
            _glp_strspx(((*hbm).key).as_mut_ptr());
            glp_printf(
                b"key = %s\n\0" as *const u8 as *const i8,
                ((*hbm).key).as_mut_ptr(),
            );
            if !(read_card(dsa) != 0) {
                if !(scan_int(
                    dsa,
                    b"totcrd\0" as *const u8 as *const i8 as *mut i8,
                    0 as i32,
                    14 as i32,
                    &mut (*hbm).totcrd,
                ) != 0)
                {
                    if !(scan_int(
                        dsa,
                        b"ptrcrd\0" as *const u8 as *const i8 as *mut i8,
                        14 as i32,
                        14 as i32,
                        &mut (*hbm).ptrcrd,
                    ) != 0)
                    {
                        if !(scan_int(
                            dsa,
                            b"indcrd\0" as *const u8 as *const i8 as *mut i8,
                            28 as i32,
                            14 as i32,
                            &mut (*hbm).indcrd,
                        ) != 0)
                        {
                            if !(scan_int(
                                dsa,
                                b"valcrd\0" as *const u8 as *const i8 as *mut i8,
                                42 as i32,
                                14 as i32,
                                &mut (*hbm).valcrd,
                            ) != 0)
                            {
                                if !(scan_int(
                                    dsa,
                                    b"rhscrd\0" as *const u8 as *const i8 as *mut i8,
                                    56 as i32,
                                    14 as i32,
                                    &mut (*hbm).rhscrd,
                                ) != 0)
                                {
                                    glp_printf(
                                        b"totcrd = %d; ptrcrd = %d; indcrd = %d; valcrd = %d; rhscrd = %d\n\0"
                                            as *const u8 as *const i8,
                                        (*hbm).totcrd,
                                        (*hbm).ptrcrd,
                                        (*hbm).indcrd,
                                        (*hbm).valcrd,
                                        (*hbm).rhscrd,
                                    );
                                    if !(read_card(dsa) != 0) {
                                        memcpy(
                                            ((*hbm).mxtype).as_mut_ptr() as *mut libc::c_void,
                                            ((*dsa).card).as_mut_ptr() as *const libc::c_void,
                                            3 as i32 as u64,
                                        );
                                        (*hbm).mxtype[3 as i32 as usize] = '\0' as i32 as i8;
                                        if (strchr(
                                            b"RCP\0" as *const u8 as *const i8,
                                            (*hbm).mxtype[0 as i32 as usize] as i32,
                                        ))
                                            .is_null()
                                            || (strchr(
                                                b"SUHZR\0" as *const u8 as *const i8,
                                                (*hbm).mxtype[1 as i32 as usize] as i32,
                                            ))
                                                .is_null()
                                            || (strchr(
                                                b"AE\0" as *const u8 as *const i8,
                                                (*hbm).mxtype[2 as i32 as usize] as i32,
                                            ))
                                                .is_null()
                                        {
                                            glp_printf(
                                                b"%s:%d: matrix type '%s' not recognised\n\0" as *const u8
                                                    as *const i8,
                                                (*dsa).fname,
                                                (*dsa).seqn,
                                                ((*hbm).mxtype).as_mut_ptr(),
                                            );
                                        } else if !(scan_int(
                                            dsa,
                                            b"nrow\0" as *const u8 as *const i8 as *mut i8,
                                            14 as i32,
                                            14 as i32,
                                            &mut (*hbm).nrow,
                                        ) != 0)
                                        {
                                            if !(scan_int(
                                                dsa,
                                                b"ncol\0" as *const u8 as *const i8 as *mut i8,
                                                28 as i32,
                                                14 as i32,
                                                &mut (*hbm).ncol,
                                            ) != 0)
                                            {
                                                if !(scan_int(
                                                    dsa,
                                                    b"nnzero\0" as *const u8 as *const i8 as *mut i8,
                                                    42 as i32,
                                                    14 as i32,
                                                    &mut (*hbm).nnzero,
                                                ) != 0)
                                                {
                                                    if !(scan_int(
                                                        dsa,
                                                        b"neltvl\0" as *const u8 as *const i8 as *mut i8,
                                                        56 as i32,
                                                        14 as i32,
                                                        &mut (*hbm).neltvl,
                                                    ) != 0)
                                                    {
                                                        glp_printf(
                                                            b"mxtype = %s; nrow = %d; ncol = %d; nnzero = %d; neltvl = %d\n\0"
                                                                as *const u8 as *const i8,
                                                            ((*hbm).mxtype).as_mut_ptr(),
                                                            (*hbm).nrow,
                                                            (*hbm).ncol,
                                                            (*hbm).nnzero,
                                                            (*hbm).neltvl,
                                                        );
                                                        if !(read_card(dsa) != 0) {
                                                            memcpy(
                                                                ((*hbm).ptrfmt).as_mut_ptr() as *mut libc::c_void,
                                                                ((*dsa).card).as_mut_ptr() as *const libc::c_void,
                                                                16 as i32 as u64,
                                                            );
                                                            (*hbm).ptrfmt[16 as i32 as usize] = '\0' as i32 as i8;
                                                            _glp_strspx(((*hbm).ptrfmt).as_mut_ptr());
                                                            memcpy(
                                                                ((*hbm).indfmt).as_mut_ptr() as *mut libc::c_void,
                                                                ((*dsa).card).as_mut_ptr().offset(16 as i32 as isize)
                                                                    as *const libc::c_void,
                                                                16 as i32 as u64,
                                                            );
                                                            (*hbm).indfmt[16 as i32 as usize] = '\0' as i32 as i8;
                                                            _glp_strspx(((*hbm).indfmt).as_mut_ptr());
                                                            memcpy(
                                                                ((*hbm).valfmt).as_mut_ptr() as *mut libc::c_void,
                                                                ((*dsa).card).as_mut_ptr().offset(32 as i32 as isize)
                                                                    as *const libc::c_void,
                                                                20 as i32 as u64,
                                                            );
                                                            (*hbm).valfmt[20 as i32 as usize] = '\0' as i32 as i8;
                                                            _glp_strspx(((*hbm).valfmt).as_mut_ptr());
                                                            memcpy(
                                                                ((*hbm).rhsfmt).as_mut_ptr() as *mut libc::c_void,
                                                                ((*dsa).card).as_mut_ptr().offset(52 as i32 as isize)
                                                                    as *const libc::c_void,
                                                                20 as i32 as u64,
                                                            );
                                                            (*hbm).rhsfmt[20 as i32 as usize] = '\0' as i32 as i8;
                                                            _glp_strspx(((*hbm).rhsfmt).as_mut_ptr());
                                                            glp_printf(
                                                                b"ptrfmt = %s; indfmt = %s; valfmt = %s; rhsfmt = %s\n\0"
                                                                    as *const u8 as *const i8,
                                                                ((*hbm).ptrfmt).as_mut_ptr(),
                                                                ((*hbm).indfmt).as_mut_ptr(),
                                                                ((*hbm).valfmt).as_mut_ptr(),
                                                                ((*hbm).rhsfmt).as_mut_ptr(),
                                                            );
                                                            if (*hbm).rhscrd <= 0 as i32 {
                                                                strcpy(
                                                                    ((*hbm).rhstyp).as_mut_ptr(),
                                                                    b"???\0" as *const u8 as *const i8,
                                                                );
                                                                (*hbm).nrhs = 0 as i32;
                                                                (*hbm).nrhsix = 0 as i32;
                                                                current_block = 7226443171521532240;
                                                            } else if read_card(dsa) != 0 {
                                                                current_block = 12379811654266555223;
                                                            } else {
                                                                memcpy(
                                                                    ((*hbm).rhstyp).as_mut_ptr() as *mut libc::c_void,
                                                                    ((*dsa).card).as_mut_ptr() as *const libc::c_void,
                                                                    3 as i32 as u64,
                                                                );
                                                                (*hbm).rhstyp[3 as i32 as usize] = '\0' as i32 as i8;
                                                                if scan_int(
                                                                    dsa,
                                                                    b"nrhs\0" as *const u8 as *const i8 as *mut i8,
                                                                    14 as i32,
                                                                    14 as i32,
                                                                    &mut (*hbm).nrhs,
                                                                ) != 0
                                                                {
                                                                    current_block = 12379811654266555223;
                                                                } else if scan_int(
                                                                    dsa,
                                                                    b"nrhsix\0" as *const u8 as *const i8 as *mut i8,
                                                                    28 as i32,
                                                                    14 as i32,
                                                                    &mut (*hbm).nrhsix,
                                                                ) != 0
                                                                {
                                                                    current_block = 12379811654266555223;
                                                                } else {
                                                                    glp_printf(
                                                                        b"rhstyp = '%s'; nrhs = %d; nrhsix = %d\n\0" as *const u8
                                                                            as *const i8,
                                                                        ((*hbm).rhstyp).as_mut_ptr(),
                                                                        (*hbm).nrhs,
                                                                        (*hbm).nrhsix,
                                                                    );
                                                                    current_block = 7226443171521532240;
                                                                }
                                                            }
                                                            match current_block {
                                                                12379811654266555223 => {}
                                                                _ => {
                                                                    (*hbm).colptr = glp_alloc(
                                                                        1 as i32 + (*hbm).ncol + 1 as i32,
                                                                        ::core::mem::size_of::<i32>() as u64 as i32,
                                                                    ) as *mut i32;
                                                                    if !(read_int_array(
                                                                        dsa,
                                                                        b"colptr\0" as *const u8 as *const i8 as *mut i8,
                                                                        ((*hbm).ptrfmt).as_mut_ptr(),
                                                                        (*hbm).ncol + 1 as i32,
                                                                        (*hbm).colptr,
                                                                    ) != 0)
                                                                    {
                                                                        (*hbm).rowind = glp_alloc(
                                                                            1 as i32 + (*hbm).nnzero,
                                                                            ::core::mem::size_of::<i32>() as u64 as i32,
                                                                        ) as *mut i32;
                                                                        if !(read_int_array(
                                                                            dsa,
                                                                            b"rowind\0" as *const u8 as *const i8 as *mut i8,
                                                                            ((*hbm).indfmt).as_mut_ptr(),
                                                                            (*hbm).nnzero,
                                                                            (*hbm).rowind,
                                                                        ) != 0)
                                                                        {
                                                                            if (*hbm).valcrd <= 0 as i32 {
                                                                                current_block = 16295177628088055733;
                                                                            } else {
                                                                                if (*hbm).mxtype[2 as i32 as usize] as i32 == 'A' as i32 {
                                                                                    (*hbm).values = glp_alloc(
                                                                                        1 as i32 + (*hbm).nnzero,
                                                                                        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
                                                                                    ) as *mut libc::c_double;
                                                                                    if read_real_array(
                                                                                        dsa,
                                                                                        b"values\0" as *const u8 as *const i8 as *mut i8,
                                                                                        ((*hbm).valfmt).as_mut_ptr(),
                                                                                        (*hbm).nnzero,
                                                                                        (*hbm).values,
                                                                                    ) != 0
                                                                                    {
                                                                                        current_block = 12379811654266555223;
                                                                                    } else {
                                                                                        current_block = 13678349939556791712;
                                                                                    }
                                                                                } else {
                                                                                    (*hbm).values = glp_alloc(
                                                                                        1 as i32 + (*hbm).neltvl,
                                                                                        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
                                                                                    ) as *mut libc::c_double;
                                                                                    if read_real_array(
                                                                                        dsa,
                                                                                        b"values\0" as *const u8 as *const i8 as *mut i8,
                                                                                        ((*hbm).valfmt).as_mut_ptr(),
                                                                                        (*hbm).neltvl,
                                                                                        (*hbm).values,
                                                                                    ) != 0
                                                                                    {
                                                                                        current_block = 12379811654266555223;
                                                                                    } else {
                                                                                        current_block = 13678349939556791712;
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    12379811654266555223 => {}
                                                                                    _ => {
                                                                                        if (*hbm).nrhs <= 0 as i32 {
                                                                                            current_block = 16295177628088055733;
                                                                                        } else {
                                                                                            if (*hbm).rhstyp[0 as i32 as usize] as i32 == 'F' as i32 {
                                                                                                (*hbm).nrhsvl = (*hbm).nrow * (*hbm).nrhs;
                                                                                                (*hbm).rhsval = glp_alloc(
                                                                                                    1 as i32 + (*hbm).nrhsvl,
                                                                                                    ::core::mem::size_of::<libc::c_double>() as u64 as i32,
                                                                                                ) as *mut libc::c_double;
                                                                                                if read_real_array(
                                                                                                    dsa,
                                                                                                    b"rhsval\0" as *const u8 as *const i8 as *mut i8,
                                                                                                    ((*hbm).rhsfmt).as_mut_ptr(),
                                                                                                    (*hbm).nrhsvl,
                                                                                                    (*hbm).rhsval,
                                                                                                ) != 0
                                                                                                {
                                                                                                    current_block = 12379811654266555223;
                                                                                                } else {
                                                                                                    current_block = 981995395831942902;
                                                                                                }
                                                                                            } else if (*hbm).rhstyp[0 as i32 as usize] as i32
                                                                                                == 'M' as i32
                                                                                                && (*hbm).mxtype[2 as i32 as usize] as i32 == 'A' as i32
                                                                                            {
                                                                                                (*hbm).rhsptr = glp_alloc(
                                                                                                    1 as i32 + (*hbm).nrhs + 1 as i32,
                                                                                                    ::core::mem::size_of::<i32>() as u64 as i32,
                                                                                                ) as *mut i32;
                                                                                                if read_int_array(
                                                                                                    dsa,
                                                                                                    b"rhsptr\0" as *const u8 as *const i8 as *mut i8,
                                                                                                    ((*hbm).ptrfmt).as_mut_ptr(),
                                                                                                    (*hbm).nrhs + 1 as i32,
                                                                                                    (*hbm).rhsptr,
                                                                                                ) != 0
                                                                                                {
                                                                                                    current_block = 12379811654266555223;
                                                                                                } else {
                                                                                                    (*hbm).rhsind = glp_alloc(
                                                                                                        1 as i32 + (*hbm).nrhsix,
                                                                                                        ::core::mem::size_of::<i32>() as u64 as i32,
                                                                                                    ) as *mut i32;
                                                                                                    if read_int_array(
                                                                                                        dsa,
                                                                                                        b"rhsind\0" as *const u8 as *const i8 as *mut i8,
                                                                                                        ((*hbm).indfmt).as_mut_ptr(),
                                                                                                        (*hbm).nrhsix,
                                                                                                        (*hbm).rhsind,
                                                                                                    ) != 0
                                                                                                    {
                                                                                                        current_block = 12379811654266555223;
                                                                                                    } else {
                                                                                                        (*hbm).rhsval = glp_alloc(
                                                                                                            1 as i32 + (*hbm).nrhsix,
                                                                                                            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
                                                                                                        ) as *mut libc::c_double;
                                                                                                        if read_real_array(
                                                                                                            dsa,
                                                                                                            b"rhsval\0" as *const u8 as *const i8 as *mut i8,
                                                                                                            ((*hbm).rhsfmt).as_mut_ptr(),
                                                                                                            (*hbm).nrhsix,
                                                                                                            (*hbm).rhsval,
                                                                                                        ) != 0
                                                                                                        {
                                                                                                            current_block = 12379811654266555223;
                                                                                                        } else {
                                                                                                            current_block = 981995395831942902;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else if (*hbm).rhstyp[0 as i32 as usize] as i32
                                                                                                == 'M' as i32
                                                                                                && (*hbm).mxtype[2 as i32 as usize] as i32 == 'E' as i32
                                                                                            {
                                                                                                (*hbm).rhsval = glp_alloc(
                                                                                                    1 as i32 + (*hbm).nrhsvl,
                                                                                                    ::core::mem::size_of::<libc::c_double>() as u64 as i32,
                                                                                                ) as *mut libc::c_double;
                                                                                                if read_real_array(
                                                                                                    dsa,
                                                                                                    b"rhsval\0" as *const u8 as *const i8 as *mut i8,
                                                                                                    ((*hbm).rhsfmt).as_mut_ptr(),
                                                                                                    (*hbm).nrhsvl,
                                                                                                    (*hbm).rhsval,
                                                                                                ) != 0
                                                                                                {
                                                                                                    current_block = 12379811654266555223;
                                                                                                } else {
                                                                                                    current_block = 981995395831942902;
                                                                                                }
                                                                                            } else {
                                                                                                glp_printf(
                                                                                                    b"%s:%d: right-hand side type '%c' not recognised\n\0"
                                                                                                        as *const u8 as *const i8,
                                                                                                    (*dsa).fname,
                                                                                                    (*dsa).seqn,
                                                                                                    (*hbm).rhstyp[0 as i32 as usize] as i32,
                                                                                                );
                                                                                                current_block = 12379811654266555223;
                                                                                            }
                                                                                            match current_block {
                                                                                                12379811654266555223 => {}
                                                                                                _ => {
                                                                                                    if (*hbm).rhstyp[1 as i32 as usize] as i32 == 'G' as i32 {
                                                                                                        (*hbm).nguess = (*hbm).nrow * (*hbm).nrhs;
                                                                                                        (*hbm).sguess = glp_alloc(
                                                                                                            1 as i32 + (*hbm).nguess,
                                                                                                            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
                                                                                                        ) as *mut libc::c_double;
                                                                                                        if read_real_array(
                                                                                                            dsa,
                                                                                                            b"sguess\0" as *const u8 as *const i8 as *mut i8,
                                                                                                            ((*hbm).rhsfmt).as_mut_ptr(),
                                                                                                            (*hbm).nguess,
                                                                                                            (*hbm).sguess,
                                                                                                        ) != 0
                                                                                                        {
                                                                                                            current_block = 12379811654266555223;
                                                                                                        } else {
                                                                                                            current_block = 1868291631715963762;
                                                                                                        }
                                                                                                    } else {
                                                                                                        current_block = 1868291631715963762;
                                                                                                    }
                                                                                                    match current_block {
                                                                                                        12379811654266555223 => {}
                                                                                                        _ => {
                                                                                                            if (*hbm).rhstyp[2 as i32 as usize] as i32 == 'X' as i32 {
                                                                                                                (*hbm).nexact = (*hbm).nrow * (*hbm).nrhs;
                                                                                                                (*hbm).xexact = glp_alloc(
                                                                                                                    1 as i32 + (*hbm).nexact,
                                                                                                                    ::core::mem::size_of::<libc::c_double>() as u64 as i32,
                                                                                                                ) as *mut libc::c_double;
                                                                                                                if read_real_array(
                                                                                                                    dsa,
                                                                                                                    b"xexact\0" as *const u8 as *const i8 as *mut i8,
                                                                                                                    ((*hbm).rhsfmt).as_mut_ptr(),
                                                                                                                    (*hbm).nexact,
                                                                                                                    (*hbm).xexact,
                                                                                                                ) != 0
                                                                                                                {
                                                                                                                    current_block = 12379811654266555223;
                                                                                                                } else {
                                                                                                                    current_block = 16295177628088055733;
                                                                                                                }
                                                                                                            } else {
                                                                                                                current_block = 16295177628088055733;
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12379811654266555223 => {}
                                                                                _ => {
                                                                                    glp_printf(
                                                                                        b"hbm_read_mat: %d cards were read\n\0" as *const u8
                                                                                            as *const i8,
                                                                                        (*dsa).seqn,
                                                                                    );
                                                                                    fclose((*dsa).fp);
                                                                                    return hbm;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !hbm.is_null() {
        if !((*hbm).colptr).is_null() {
            glp_free((*hbm).colptr as *mut libc::c_void);
        }
        if !((*hbm).rowind).is_null() {
            glp_free((*hbm).rowind as *mut libc::c_void);
        }
        if !((*hbm).rhsptr).is_null() {
            glp_free((*hbm).rhsptr as *mut libc::c_void);
        }
        if !((*hbm).rhsind).is_null() {
            glp_free((*hbm).rhsind as *mut libc::c_void);
        }
        if !((*hbm).values).is_null() {
            glp_free((*hbm).values as *mut libc::c_void);
        }
        if !((*hbm).rhsval).is_null() {
            glp_free((*hbm).rhsval as *mut libc::c_void);
        }
        if !((*hbm).sguess).is_null() {
            glp_free((*hbm).sguess as *mut libc::c_void);
        }
        if !((*hbm).xexact).is_null() {
            glp_free((*hbm).xexact as *mut libc::c_void);
        }
        glp_free(hbm as *mut libc::c_void);
    }
    if !((*dsa).fp).is_null() {
        fclose((*dsa).fp);
    }
    return 0 as *mut HBM;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_hbm_free_mat(mut hbm: *mut HBM) {
    if !((*hbm).colptr).is_null() {
        glp_free((*hbm).colptr as *mut libc::c_void);
    }
    if !((*hbm).rowind).is_null() {
        glp_free((*hbm).rowind as *mut libc::c_void);
    }
    if !((*hbm).rhsptr).is_null() {
        glp_free((*hbm).rhsptr as *mut libc::c_void);
    }
    if !((*hbm).rhsind).is_null() {
        glp_free((*hbm).rhsind as *mut libc::c_void);
    }
    if !((*hbm).values).is_null() {
        glp_free((*hbm).values as *mut libc::c_void);
    }
    if !((*hbm).rhsval).is_null() {
        glp_free((*hbm).rhsval as *mut libc::c_void);
    }
    if !((*hbm).sguess).is_null() {
        glp_free((*hbm).sguess as *mut libc::c_void);
    }
    if !((*hbm).xexact).is_null() {
        glp_free((*hbm).xexact as *mut libc::c_void);
    }
    glp_free(hbm as *mut libc::c_void);
}