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
    pub type afm_handle_st;
    pub type afm_font_private_data_st;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn exit(_: i32) -> !;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn textdomain(__domainname: *const i8) -> *mut i8;
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
    fn afm_error_to_string(error: AFMError, buf: *mut i8);
    fn afm_create(
        path: *const i8,
        verbose_level: u32,
        handle_return: *mut AFMHandle,
    ) -> AFMError;
    fn afm_open_file(
        handle: AFMHandle,
        info_level: u32,
        filename: *const i8,
        font_return: *mut AFMFont,
    ) -> AFMError;
    fn afm_close_font(font: AFMFont) -> AFMError;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt_long(
        argc: i32,
        argv: *const *mut i8,
        shortopts: *const i8,
        longopts: *const option,
        longind: *mut i32,
    ) -> i32;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type AFMString = *mut i8;
pub type AFMName = *mut i8;
pub type AFMNumber = libc::c_double;
pub type AFMInteger = i64;
pub type AFMBoolean = u32;
pub const AFMTrue: AFMBoolean = 1;
pub const AFMFalse: AFMBoolean = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afm_array_st {
    pub num_items: AFMNumber,
    pub items: *mut AFMNode,
}
pub type AFMNode = afm_node_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afm_node_st {
    pub type_0: i32,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub string: AFMString,
    pub name: AFMName,
    pub number: AFMNumber,
    pub integer: AFMInteger,
    pub array: AFMArray,
    pub boolean: AFMBoolean,
}
pub type AFMArray = *mut afm_array_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct global_font_information_st {
    pub FontName: AFMString,
    pub FullName: AFMString,
    pub FamilyName: AFMString,
    pub Weight: AFMString,
    pub FontBBox_llx: AFMNumber,
    pub FontBBox_lly: AFMNumber,
    pub FontBBox_urx: AFMNumber,
    pub FontBBox_ury: AFMNumber,
    pub Version: AFMString,
    pub Notice: AFMString,
    pub EncodingScheme: AFMString,
    pub MappingScheme: AFMInteger,
    pub EscChar: AFMInteger,
    pub CharacterSet: AFMString,
    pub Characters: AFMInteger,
    pub IsBaseFont: AFMBoolean,
    pub VVector_0: AFMNumber,
    pub VVector_1: AFMNumber,
    pub IsFixedV: AFMBoolean,
    pub CapHeight: AFMNumber,
    pub XHeight: AFMNumber,
    pub Ascender: AFMNumber,
    pub Descender: AFMNumber,
    pub BlendAxisTypes: AFMArray,
    pub BlendDesignPositions: AFMArray,
    pub BlendDesignMap: AFMArray,
    pub WeightVector: AFMArray,
}
pub type AFMGlobalFontInformation = global_font_information_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct writing_direction_metrics_st {
    pub is_valid: AFMBoolean,
    pub UnderlinePosition: AFMNumber,
    pub UnderlineThickness: AFMNumber,
    pub ItalicAngle: AFMNumber,
    pub CharWidth_x: AFMNumber,
    pub CharWidth_y: AFMNumber,
    pub IsFixedPitch: AFMBoolean,
}
pub type AFMWritingDirectionMetrics = writing_direction_metrics_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ligature_st {
    pub successor: AFMName,
    pub ligature: AFMName,
}
pub type AFMLigature = ligature_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct individual_character_metrics_st {
    pub character_code: AFMInteger,
    pub w0x: AFMNumber,
    pub w0y: AFMNumber,
    pub w1x: AFMNumber,
    pub w1y: AFMNumber,
    pub name: AFMName,
    pub vv_x: AFMNumber,
    pub vv_y: AFMNumber,
    pub llx: AFMNumber,
    pub lly: AFMNumber,
    pub urx: AFMNumber,
    pub ury: AFMNumber,
    pub num_ligatures: AFMNumber,
    pub ligatures: *mut AFMLigature,
}
pub type AFMIndividualCharacterMetrics = individual_character_metrics_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct track_kern_st {
    pub degree: AFMInteger,
    pub min_ptsize: AFMNumber,
    pub min_kern: AFMNumber,
    pub max_ptsize: AFMNumber,
    pub max_kern: AFMNumber,
}
pub type AFMTrackKern = track_kern_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pair_wise_kerning_st {
    pub name1: AFMName,
    pub name2: AFMName,
    pub kx: AFMNumber,
    pub ky: AFMNumber,
}
pub type AFMPairWiseKerning = pair_wise_kerning_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_component_st {
    pub name: AFMName,
    pub deltax: AFMNumber,
    pub deltay: AFMNumber,
}
pub type AFMCompositeComponent = composite_component_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_st {
    pub name: AFMName,
    pub num_components: AFMInteger,
    pub components: *mut AFMCompositeComponent,
}
pub type AFMComposite = composite_st;
pub type AFMError = u32;
pub type AFMHandle = *mut afm_handle_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afm_font_st {
    pub private: *mut afm_font_private_data_st,
    pub version: AFMNumber,
    pub info_level: u32,
    pub encoding: [*mut AFMIndividualCharacterMetrics; 256],
    pub global_info: AFMGlobalFontInformation,
    pub writing_direction_metrics: [AFMWritingDirectionMetrics; 2],
    pub num_character_metrics: AFMInteger,
    pub character_metrics: *mut AFMIndividualCharacterMetrics,
    pub num_composites: AFMInteger,
    pub composites: *mut AFMComposite,
    pub num_kern_pairs: AFMInteger,
    pub kern_pairs: *mut AFMPairWiseKerning,
    pub num_track_kerns: AFMInteger,
    pub track_kerns: *mut AFMTrackKern,
}
pub type AFMFont = *mut afm_font_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
static mut fname: *mut i8 = b"font.map\0" as *const u8 as *const i8 as *mut i8;
static mut program: *mut i8 = 0 as *const i8 as *mut i8;
static mut long_options: [option; 4] = [
    {
        let mut init = option {
            name: b"output-file\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 0 as i32,
        };
        init
    },
];
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut error: AFMError = 0;
    let mut afm: AFMHandle = 0 as *mut afm_handle_st;
    let mut font: AFMFont = 0 as *mut afm_font_st;
    let mut i: i32 = 0;
    let mut ofp: *mut FILE = 0 as *mut FILE;
    let mut mfp: *mut FILE = 0 as *mut FILE;
    program = strrchr(*argv.offset(0 as i32 as isize), '/' as i32);
    if program.is_null() {
        program = *argv.offset(0 as i32 as isize);
    } else {
        program = program.offset(1);
        program;
    }
    let ref mut fresh0 = *argv.offset(0 as i32 as isize);
    *fresh0 = program;
    setlocale(5 as i32, b"\0" as *const u8 as *const i8);
    bindtextdomain(
        b"enscript\0" as *const u8 as *const i8,
        b"/usr/local/share/locale\0" as *const u8 as *const i8,
    );
    textdomain(b"enscript\0" as *const u8 as *const i8);
    loop {
        let mut option_index: i32 = 0 as i32;
        let mut c: i32 = 0;
        c = getopt_long(
            argc,
            argv as *const *mut i8,
            b"p:h\0" as *const u8 as *const i8,
            long_options.as_mut_ptr(),
            &mut option_index,
        );
        if c == -(1 as i32) {
            break;
        }
        match c {
            104 => {
                usage();
                exit(0 as i32);
            }
            112 => {
                if strcmp(optarg, b"-\0" as *const u8 as *const i8) == 0 as i32 {
                    fname = 0 as *mut i8;
                } else {
                    fname = optarg;
                }
            }
            86 => {
                printf(
                    b"%s for GNU %s %s\n\0" as *const u8 as *const i8,
                    program,
                    b"enscript\0" as *const u8 as *const i8,
                    b"1.6.1\0" as *const u8 as *const i8,
                );
                exit(0 as i32);
            }
            63 => {
                usage();
                exit(1 as i32);
            }
            _ => {}
        }
    }
    printf(
        dcgettext(0 as *const i8, b"file=%s\n\0" as *const u8 as *const i8, 5 as i32),
        if !fname.is_null() {
            fname
        } else {
            dcgettext(0 as *const i8, b"stdout\0" as *const u8 as *const i8, 5 as i32)
        },
    );
    if !fname.is_null() {
        ofp = fopen(fname, b"w\0" as *const u8 as *const i8);
        if ofp.is_null() {
            let mut buf: [i8; 256] = [0; 256];
            sprintf(
                buf.as_mut_ptr(),
                dcgettext(
                    0 as *const i8,
                    b"%s: couldn't open output file \"%s\"\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                program,
                fname,
            );
            perror(buf.as_mut_ptr());
            exit(1 as i32);
        }
        mfp = stdout;
    } else {
        ofp = stdout;
        mfp = stderr;
    }
    error = afm_create(0 as *const i8, 0 as i32 as u32, &mut afm);
    if error != 0 as i32 as u32 {
        let mut buf_0: [i8; 256] = [0; 256];
        afm_error_to_string(error, buf_0.as_mut_ptr());
        fprintf(
            stderr,
            b"%s: %s: %s\n\0" as *const u8 as *const i8,
            program,
            dcgettext(
                0 as *const i8,
                b"couldn't create AFM library\0" as *const u8 as *const i8,
                5 as i32,
            ),
            buf_0.as_mut_ptr(),
        );
        exit(1 as i32);
    }
    i = optind;
    while i < argc {
        fprintf(mfp, b"%s...\n\0" as *const u8 as *const i8, *argv.offset(i as isize));
        error = afm_open_file(afm, 0 as i32 as u32, *argv.offset(i as isize), &mut font);
        if error == 0 as i32 as u32 {
            let mut cp: *mut i8 = 0 as *mut i8;
            let mut sf: *mut i8 = 0 as *mut i8;
            let mut len: i32 = 0;
            cp = strrchr(*argv.offset(i as isize), '/' as i32);
            if cp.is_null() {
                cp = *argv.offset(i as isize);
            } else {
                cp = cp.offset(1);
                cp;
            }
            sf = strrchr(*argv.offset(i as isize), '.' as i32);
            if !sf.is_null() {
                len = sf.offset_from(cp) as i64 as i32;
            } else {
                len = strlen(cp) as i32;
            }
            fprintf(
                ofp,
                b"%-30s\t%.*s\n\0" as *const u8 as *const i8,
                (*font).global_info.FontName,
                len,
                cp,
            );
            afm_close_font(font);
        } else {
            let mut buf_1: [i8; 256] = [0; 256];
            afm_error_to_string(error, buf_1.as_mut_ptr());
            fprintf(
                mfp,
                b"%s: %s\n\0" as *const u8 as *const i8,
                program,
                buf_1.as_mut_ptr(),
            );
        }
        i += 1;
        i;
    }
    if !fname.is_null() {
        fclose(ofp);
    }
    return 0 as i32;
}
unsafe extern "C" fn usage() {
    printf(
        dcgettext(
            0 as *const i8,
            b"Usage: %s [OPTION]... FILE...\nMandatory arguments to long options are mandatory for short options too.\n  -h, --help              print this help and exit\n  -p, --output-file=NAME  print output to file NAME (default file is\n                          font.map).  If FILE is `-', leavy output to\n                          stdout.\n  -V, --version           print version number\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        program,
    );
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