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
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn mt_basename(filename: *const i8) -> *const i8;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn setup_signal();
    fn read_config();
    fn mattrib(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mbadblocks(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mcat(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mcd(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mcopy(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mdel(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mdir(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mdoctorfat(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mdu(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mformat(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn minfo(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mlabel(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mmd(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mmount(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mmove(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mpartition(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mshortname(argc: i32, argv: *mut *mut i8, mtype: i32);
    fn mshowfat(argc: i32, argv: *mut *mut i8, mtype: i32);
    fn mtoolstest(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn mzip(argc: i32, argv: *mut *mut i8, type_0: i32);
    fn init_privs();
    static mut mversion: *const i8;
}
pub type __off_t = i64;
pub type __off64_t = i64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dispatch {
    pub cmd: *const i8,
    pub fn_0: Option<unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()>,
    pub type_0: i32,
}
#[no_mangle]
pub static mut progname: *const i8 = 0 as *const i8;
static mut dispatch: [dispatch; 26] = unsafe {
    [
        {
            let mut init = dispatch {
                cmd: b"mattrib\0" as *const u8 as *const i8,
                fn_0: Some(
                    mattrib as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> (),
                ),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mbadblocks\0" as *const u8 as *const i8,
                fn_0: Some(
                    mbadblocks as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> (),
                ),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mcat\0" as *const u8 as *const i8,
                fn_0: Some(mcat as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mcd\0" as *const u8 as *const i8,
                fn_0: Some(mcd as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mcopy\0" as *const u8 as *const i8,
                fn_0: Some(mcopy as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mdel\0" as *const u8 as *const i8,
                fn_0: Some(mdel as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mdeltree\0" as *const u8 as *const i8,
                fn_0: Some(mdel as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 2 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mdir\0" as *const u8 as *const i8,
                fn_0: Some(mdir as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mdoctorfat\0" as *const u8 as *const i8,
                fn_0: Some(
                    mdoctorfat as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> (),
                ),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mdu\0" as *const u8 as *const i8,
                fn_0: Some(mdu as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mformat\0" as *const u8 as *const i8,
                fn_0: Some(
                    mformat as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> (),
                ),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"minfo\0" as *const u8 as *const i8,
                fn_0: Some(minfo as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mlabel\0" as *const u8 as *const i8,
                fn_0: Some(mlabel as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mmd\0" as *const u8 as *const i8,
                fn_0: Some(mmd as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mmount\0" as *const u8 as *const i8,
                fn_0: Some(mmount as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mpartition\0" as *const u8 as *const i8,
                fn_0: Some(
                    mpartition as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> (),
                ),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mrd\0" as *const u8 as *const i8,
                fn_0: Some(mdel as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 1 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mread\0" as *const u8 as *const i8,
                fn_0: Some(mcopy as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mmove\0" as *const u8 as *const i8,
                fn_0: Some(mmove as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mren\0" as *const u8 as *const i8,
                fn_0: Some(mmove as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 1 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mshowfat\0" as *const u8 as *const i8,
                fn_0: Some(
                    mshowfat as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> (),
                ),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mshortname\0" as *const u8 as *const i8,
                fn_0: Some(
                    mshortname as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> (),
                ),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mtoolstest\0" as *const u8 as *const i8,
                fn_0: Some(
                    mtoolstest as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> (),
                ),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mtype\0" as *const u8 as *const i8,
                fn_0: Some(mcopy as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 1 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mwrite\0" as *const u8 as *const i8,
                fn_0: Some(mcopy as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mzip\0" as *const u8 as *const i8,
                fn_0: Some(mzip as unsafe extern "C" fn(i32, *mut *mut i8, i32) -> ()),
                type_0: 0 as i32,
            };
            init
        },
    ]
};
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut i: u32 = 0;
    let mut name: *const i8 = 0 as *const i8;
    let mut locale: *mut i8 = 0 as *mut i8;
    locale = setlocale(6 as i32, b"\0" as *const u8 as *const i8);
    if locale.is_null() || strcmp(locale, b"C\0" as *const u8 as *const i8) == 0 {
        setlocale(6 as i32, b"en_US\0" as *const u8 as *const i8);
    }
    init_privs();
    name = mt_basename(*argv.offset(0 as i32 as isize));
    progname = *argv.offset(0 as i32 as isize);
    if argc >= 3 as i32
        && strcmp(*argv.offset(1 as i32 as isize), b"-c\0" as *const u8 as *const i8)
            == 0 && strcmp(name, b"mtools\0" as *const u8 as *const i8) == 0
    {
        argc -= 2 as i32;
        argv = argv.offset(2 as i32 as isize);
        name = *argv.offset(0 as i32 as isize);
    }
    if argc >= 2 as i32
        && (strcmp(*argv.offset(1 as i32 as isize), b"-V\0" as *const u8 as *const i8)
            == 0 as i32
            || strcmp(
                *argv.offset(1 as i32 as isize),
                b"--version\0" as *const u8 as *const i8,
            ) == 0 as i32)
    {
        printf(b"%s (GNU mtools) %s\n\0" as *const u8 as *const i8, name, mversion);
        printf(b"configured with the following options: \0" as *const u8 as *const i8);
        printf(b"enable-xdf \0" as *const u8 as *const i8);
        printf(b"disable-vold \0" as *const u8 as *const i8);
        printf(b"disable-new-vold \0" as *const u8 as *const i8);
        printf(b"disable-debug \0" as *const u8 as *const i8);
        printf(b"enable-raw-term \0" as *const u8 as *const i8);
        printf(b"\n\0" as *const u8 as *const i8);
        return 0 as i32;
    }
    read_config();
    setup_signal();
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[dispatch; 26]>() as u64)
            .wrapping_div(::core::mem::size_of::<dispatch>() as u64)
    {
        if strcmp(name, dispatch[i as usize].cmd) == 0 {
            (dispatch[i as usize].fn_0)
                .expect(
                    "non-null function pointer",
                )(argc, argv, dispatch[i as usize].type_0);
        }
        i = i.wrapping_add(1);
        i;
    }
    if strcmp(name, b"mtools\0" as *const u8 as *const i8) != 0 {
        fprintf(
            stderr,
            b"Unknown mtools command '%s'\n\0" as *const u8 as *const i8,
            name,
        );
    }
    fprintf(stderr, b"Supported commands:\0" as *const u8 as *const i8);
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[dispatch; 26]>() as u64)
            .wrapping_div(::core::mem::size_of::<dispatch>() as u64)
    {
        if i.wrapping_rem(8 as i32 as u32) == 0 as i32 as u32 {
            _IO_putc('\n' as i32, stderr);
        } else {
            fprintf(stderr, b", \0" as *const u8 as *const i8);
        }
        fprintf(stderr, b"%s\0" as *const u8 as *const i8, dispatch[i as usize].cmd);
        i = i.wrapping_add(1);
        i;
    }
    _IO_putc('\n' as i32, stderr);
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn helpFlag(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    return (argc > 1 as i32
        && strcmp(*argv.offset(1 as i32 as isize), b"--help\0" as *const u8 as *const i8)
            == 0) as i32;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}