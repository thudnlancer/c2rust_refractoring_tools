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
    fn floor(_: libc::c_double) -> libc::c_double;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn _glp_getc(f: *mut glp_file) -> i32;
    fn _glp_ioerr(f: *mut glp_file) -> i32;
    fn _glp_get_err_msg() -> *const i8;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_vprintf(fmt: *const i8, arg: ::core::ffi::VaList);
    fn glp_printf(fmt: *const i8, _: ...);
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
#[no_mangle]
pub unsafe extern "C" fn _glp_dmx_error(
    mut csa: *mut DMX,
    mut fmt: *const i8,
    mut args: ...
) {
    let mut arg: ::core::ffi::VaListImpl;
    glp_printf(
        b"%s:%d: error: \0" as *const u8 as *const i8,
        (*csa).fname,
        (*csa).count,
    );
    arg = args.clone();
    glp_vprintf(fmt, arg.as_va_list());
    glp_printf(b"\n\0" as *const u8 as *const i8);
    longjmp(((*csa).jump).as_mut_ptr(), 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dmx_warning(
    mut csa: *mut DMX,
    mut fmt: *const i8,
    mut args: ...
) {
    let mut arg: ::core::ffi::VaListImpl;
    glp_printf(
        b"%s:%d: warning: \0" as *const u8 as *const i8,
        (*csa).fname,
        (*csa).count,
    );
    arg = args.clone();
    glp_vprintf(fmt, arg.as_va_list());
    glp_printf(b"\n\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dmx_read_char(mut csa: *mut DMX) {
    let mut c: i32 = 0;
    if (*csa).c == '\n' as i32 {
        (*csa).count += 1;
        (*csa).count;
    }
    c = _glp_getc((*csa).fp);
    if c < 0 as i32 {
        if _glp_ioerr((*csa).fp) != 0 {
            _glp_dmx_error(
                csa,
                b"read error - %s\0" as *const u8 as *const i8,
                _glp_get_err_msg(),
            );
        } else if (*csa).c == '\n' as i32 {
            _glp_dmx_error(csa, b"unexpected end of file\0" as *const u8 as *const i8);
        } else {
            _glp_dmx_warning(
                csa,
                b"missing final end of line\0" as *const u8 as *const i8,
            );
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
            _glp_dmx_error(
                csa,
                b"invalid control character 0x%02X\0" as *const u8 as *const i8,
                c,
            );
        }
    }
    (*csa).c = c;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dmx_read_designator(mut csa: *mut DMX) {
    ((*csa).c == '\n' as i32
        || {
            glp_assert_(
                b"csa->c == '\\n'\0" as *const u8 as *const i8,
                b"misc/dimacs.c\0" as *const u8 as *const i8,
                74 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_dmx_read_char(csa);
    loop {
        while (*csa).c == ' ' as i32 {
            _glp_dmx_read_char(csa);
        }
        if (*csa).c == '\n' as i32 {
            if (*csa).empty == 0 {
                _glp_dmx_warning(csa, b"empty line ignored\0" as *const u8 as *const i8);
                (*csa).empty = 1 as i32;
            }
            _glp_dmx_read_char(csa);
        } else if (*csa).c == 'c' as i32 {
            while (*csa).c != '\n' as i32 {
                _glp_dmx_read_char(csa);
            }
            _glp_dmx_read_char(csa);
        } else {
            (*csa).field[0 as i32 as usize] = (*csa).c as i8;
            (*csa).field[1 as i32 as usize] = '\0' as i32 as i8;
            _glp_dmx_read_char(csa);
            if !((*csa).c == ' ' as i32 || (*csa).c == '\n' as i32) {
                _glp_dmx_error(
                    csa,
                    b"line designator missing or invalid\0" as *const u8 as *const i8,
                );
            }
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dmx_read_field(mut csa: *mut DMX) {
    let mut len: i32 = 0 as i32;
    while (*csa).c == ' ' as i32 {
        _glp_dmx_read_char(csa);
    }
    if (*csa).c == '\n' as i32 {
        _glp_dmx_error(csa, b"unexpected end of line\0" as *const u8 as *const i8);
    }
    while !((*csa).c == ' ' as i32 || (*csa).c == '\n' as i32) {
        if len as u64
            == (::core::mem::size_of::<[i8; 256]>() as u64).wrapping_sub(1 as i32 as u64)
        {
            _glp_dmx_error(
                csa,
                b"data field '%.15s...' too long\0" as *const u8 as *const i8,
                ((*csa).field).as_mut_ptr(),
            );
        }
        let fresh0 = len;
        len = len + 1;
        (*csa).field[fresh0 as usize] = (*csa).c as i8;
        _glp_dmx_read_char(csa);
    }
    (*csa).field[len as usize] = '\0' as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dmx_end_of_line(mut csa: *mut DMX) {
    while (*csa).c == ' ' as i32 {
        _glp_dmx_read_char(csa);
    }
    if (*csa).c != '\n' as i32 {
        _glp_dmx_error(
            csa,
            b"too many data fields specified\0" as *const u8 as *const i8,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dmx_check_int(mut csa: *mut DMX, mut num: libc::c_double) {
    if (*csa).nonint == 0 && num != floor(num) {
        _glp_dmx_warning(csa, b"non-integer data detected\0" as *const u8 as *const i8);
        (*csa).nonint = 1 as i32;
    }
}