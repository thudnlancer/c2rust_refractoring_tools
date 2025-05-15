use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_void, c_ulong, c_double};
use std::ptr;
use std::mem;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use libc::{stat, FILE, fopen, fclose, fread, fwrite, fflush, fgetc, fputc, fgets, fputs, ungetc, pclose, popen};
use libc::{strlen, strcpy, strncpy, strcmp, strncmp, strchr, strrchr, strtok, strerror, sprintf, atoi, atof};
use libc::{getcwd, gethostname, getenv, exit, strtol, strtod, __xstat, __errno_location, __ctype_b_loc};
use libc::{strftime, tm, timespec, passwd, dcgettext, getpwnam};

// Constants and types from original C code
const _ISalnum: c_int = 8;
const _ISpunct: c_int = 4;
const _IScntrl: c_int = 2;
const _ISblank: c_int = 1;
const _ISgraph: c_int = 32768;
const _ISprint: c_int = 16384;
const _ISspace: c_int = 8192;
const _ISxdigit: c_int = 4096;
const _ISdigit: c_int = 2048;
const _ISalpha: c_int = 1024;
const _ISlower: c_int = 512;
const _ISupper: c_int = 256;

const AFMTrue: c_int = 1;
const AFMFalse: c_int = 0;

type AFMString = *mut c_char;
type AFMName = *mut c_char;
type AFMNumber = c_double;
type AFMInteger = libc::c_long;
type AFMBoolean = c_int;

struct AFMArray {
    num_items: AFMNumber,
    items: *mut AFMNode,
}

struct AFMNode {
    type_: c_int,
    u: AFMNodeUnion,
}

union AFMNodeUnion {
    string: AFMString,
    name: AFMName,
    number: AFMNumber,
    integer: AFMInteger,
    array: *mut AFMArray,
    boolean: AFMBoolean,
}

struct AFMGlobalFontInformation {
    FontName: AFMString,
    FullName: AFMString,
    FamilyName: AFMString,
    Weight: AFMString,
    FontBBox_llx: AFMNumber,
    FontBBox_lly: AFMNumber,
    FontBBox_urx: AFMNumber,
    FontBBox_ury: AFMNumber,
    Version: AFMString,
    Notice: AFMString,
    EncodingScheme: AFMString,
    MappingScheme: AFMInteger,
    EscChar: AFMInteger,
    CharacterSet: AFMString,
    Characters: AFMInteger,
    IsBaseFont: AFMBoolean,
    VVector_0: AFMNumber,
    VVector_1: AFMNumber,
    IsFixedV: AFMBoolean,
    CapHeight: AFMNumber,
    XHeight: AFMNumber,
    Ascender: AFMNumber,
    Descender: AFMNumber,
    BlendAxisTypes: *mut AFMArray,
    BlendDesignPositions: *mut AFMArray,
    BlendDesignMap: *mut AFMArray,
    WeightVector: *mut AFMArray,
}

struct AFMWritingDirectionMetrics {
    is_valid: AFMBoolean,
    UnderlinePosition: AFMNumber,
    UnderlineThickness: AFMNumber,
    ItalicAngle: AFMNumber,
    CharWidth_x: AFMNumber,
    CharWidth_y: AFMNumber,
    IsFixedPitch: AFMBoolean,
}

struct AFMLigature {
    successor: AFMName,
    ligature: AFMName,
}

struct AFMIndividualCharacterMetrics {
    character_code: AFMInteger,
    w0x: AFMNumber,
    w0y: AFMNumber,
    w1x: AFMNumber,
    w1y: AFMNumber,
    name: AFMName,
    vv_x: AFMNumber,
    vv_y: AFMNumber,
    llx: AFMNumber,
    lly: AFMNumber,
    urx: AFMNumber,
    ury: AFMNumber,
    num_ligatures: AFMNumber,
    ligatures: *mut AFMLigature,
}

struct AFMTrackKern {
    degree: AFMInteger,
    min_ptsize: AFMNumber,
    min_kern: AFMNumber,
    max_ptsize: AFMNumber,
    max_kern: AFMNumber,
}

struct AFMPairWiseKerning {
    name1: AFMName,
    name2: AFMName,
    kx: AFMNumber,
    ky: AFMNumber,
}

struct AFMCompositeComponent {
    name: AFMName,
    deltax: AFMNumber,
    deltay: AFMNumber,
}

struct AFMComposite {
    name: AFMName,
    num_components: AFMInteger,
    components: *mut AFMCompositeComponent,
}

type AFMError = c_int;
type AFMHandle = *mut c_void;
type AFMEncoding = c_int;

const AFM_ENCODING_KOI8: AFMEncoding = 12;
const AFM_ENCODING_HP8: AFMEncoding = 11;
const AFM_ENCODING_VMS: AFMEncoding = 10;
const AFM_ENCODING_MAC: AFMEncoding = 9;
const AFM_ENCODING_ASCII: AFMEncoding = 8;
const AFM_ENCODING_IBMPC: AFMEncoding = 7;
const AFM_ENCODING_ISO_8859_7: AFMEncoding = 6;
const AFM_ENCODING_ISO_8859_5: AFMEncoding = 5;
const AFM_ENCODING_ISO_8859_4: AFMEncoding = 4;
const AFM_ENCODING_ISO_8859_3: AFMEncoding = 3;
const AFM_ENCODING_ISO_8859_2: AFMEncoding = 2;
const AFM_ENCODING_ISO_8859_1: AFMEncoding = 1;
const AFM_ENCODING_DEFAULT: AFMEncoding = 0;

struct AFMFont {
    private: *mut c_void,
    version: AFMNumber,
    info_level: c_uint,
    encoding: [*mut AFMIndividualCharacterMetrics; 256],
    global_info: AFMGlobalFontInformation,
    writing_direction_metrics: [AFMWritingDirectionMetrics; 2],
    num_character_metrics: AFMInteger,
    character_metrics: *mut AFMIndividualCharacterMetrics,
    num_composites: AFMInteger,
    composites: *mut AFMComposite,
    num_kern_pairs: AFMInteger,
    kern_pairs: *mut AFMPairWiseKerning,
    num_track_kerns: AFMInteger,
    track_kerns: *mut AFMTrackKern,
}

type StringHashPtr = *mut c_void;

struct MediaEntry {
    next: *mut MediaEntry,
    name: *mut c_char,
    w: c_int,
    h: c_int,
    llx: c_int,
    lly: c_int,
    urx: c_int,
    ury: c_int,
}

type InputEncoding = c_int;

const ENC_PS: InputEncoding = 14;
const ENC_KOI8: InputEncoding = 13;
const ENC_HP8: InputEncoding = 12;
const ENC_VMS: InputEncoding = 11;
const ENC_MAC: InputEncoding = 10;
const ENC_IBMPC: InputEncoding = 9;
const ENC_ASCII_DKNO: InputEncoding = 8;
const ENC_ASCII_FISE: InputEncoding = 7;
const ENC_ASCII: InputEncoding = 6;
const ENC_ISO_8859_7: InputEncoding = 5;
const ENC_ISO_8859_5: InputEncoding = 4;
const ENC_ISO_8859_4: InputEncoding = 3;
const ENC_ISO_8859_3: InputEncoding = 2;
const ENC_ISO_8859_2: InputEncoding = 1;
const ENC_ISO_8859_1: InputEncoding = 0;

type FormFeedType = c_int;

const FORMFEED_HCOLUMN: FormFeedType = 2;
const FORMFEED_PAGE: FormFeedType = 1;
const FORMFEED_COLUMN: FormFeedType = 0;

struct FileLookupCtx {
    name: [c_char; 256],
    suffix: [c_char; 256],
    fullname: [c_char; 512],
}

type PathWalkProc = extern "C" fn(*mut c_char, *mut c_void) -> c_int;

struct InputStream {
    is_pipe: c_int,
    fp: *mut FILE,
    buf: [u8; 4096],
    data_in_buf: c_uint,
    bufpos: c_uint,
    nreads: c_uint,
    unget_ch: *mut u8,
    unget_pos: c_uint,
    unget_alloc: c_uint,
}

struct FontPoint {
    w: c_double,
    h: c_double,
}

struct CachedFontInfo {
    font_widths: [c_double; 256],
    font_ctype: [c_char; 256],
    font_is_fixed: AFMBoolean,
    font_bbox_lly: AFMNumber,
}

struct EncodingMap {
    code: c_int,
    name: *mut c_char,
}

static enc_7bit_ascii_fise: [EncodingMap; 7] = [
    EncodingMap { code: b'{' as c_int, name: b"adieresis\0".as_ptr() as *mut c_char },
    EncodingMap { code: b'|' as c_int, name: b"odieresis\0".as_ptr() as *mut c_char },
    EncodingMap { code: b'}' as c_int, name: b"aring\0".as_ptr() as *mut c_char },
    EncodingMap { code: b'[' as c_int, name: b"Adieresis\0".as_ptr() as *mut c_char },
    EncodingMap { code: b'\\' as c_int, name: b"Odieresis\0".as_ptr() as *mut c_char },
    EncodingMap { code: b']' as c_int, name: b"Aring\0".as_ptr() as *mut c_char },
    EncodingMap { code: 0, name: ptr::null_mut() },
];

static enc_7bit_ascii_dkno: [EncodingMap; 7] = [
    EncodingMap { code: b'{' as c_int, name: b"ae\0".as_ptr() as *mut c_char },
    EncodingMap { code: b'|' as c_int, name: b"oslash\0".as_ptr() as *mut c_char },
    EncodingMap { code: b'}' as c_int, name: b"aring\0".as_ptr() as *mut c_char },
    EncodingMap { code: b'[' as c_int, name: b"AE\0".as_ptr() as *mut c_char },
    EncodingMap { code: b'\\' as c_int, name: b"Oslash\0".as_ptr() as *mut c_char },
    EncodingMap { code: b']' as c_int, name: b"Aring\0".as_ptr() as *mut c_char },
    EncodingMap { code: 0, name: ptr::null_mut() },
];

// Global variables from original C code
static mut pslevel: c_uint = 0;
static mut generate_PageSize: c_int = 0;
static mut states_highlight_level: [c_char; 0] = [];
static mut states_config_file: [c_char; 0] = [];
static mut states_color_model: [c_char; 0] = [];
static mut states_path: [c_char; 0] = [];
static mut slice: c_uint = 0;
static mut slicing: c_int = 0;
static mut toc_fmt_string: *mut c_char = ptr::null_mut();
static mut program: *mut c_char = ptr::null_mut();
static mut ofp: *mut FILE = ptr::null_mut();
static mut run_tm: tm = tm {
    tm_sec: 0,
    tm_min: 0,
    tm_hour: 0,
    tm_mday: 0,
    tm_mon: 0,
    tm_year: 0,
    tm_wday: 0,
    tm_yday: 0,
    tm_isdst: 0,
    tm_gmtoff: 0,
    tm_zone: ptr::null(),
};
static mut mod_tm: tm = tm {
    tm_sec: 0,
    tm_min: 0,
    tm_hour: 0,
    tm_mday: 0,
    tm_mon: 0,
    tm_year: 0,
    tm_wday: 0,
    tm_yday: 0,
    tm_isdst: 0,
    tm_gmtoff: 0,
    tm_zone: ptr::null(),
};
static mut passwd: *mut passwd = ptr::null_mut();
static mut libpath: [c_char; 0] = [];
static mut afm_path: *mut c_char = ptr::null_mut();
static mut afm_path_buffer: [c_char; 0] = [];
static mut media_names: *mut MediaEntry = ptr::null_mut();
static mut no_job_header_switch: [c_char; 0] = [];
static mut output_first_line: [c_char; 0] = [];
static mut queue_param: [c_char; 0] = [];
static mut spooler_command: [c_char; 0] = [];
static mut current_pagenum: c_uint = 0;
static mut input_filenum: c_uint = 0;
static mut current_file_linenum: c_uint = 0;
static mut fname: [c_char; 0] = [];
static mut d_header_h: c_int = 0;
static mut d_footer_h: c_int = 0;
static mut res_fonts: StringHashPtr = ptr::null_mut();
static mut download_fonts: StringHashPtr = ptr::null_mut();
static mut pagedevice: StringHashPtr = ptr::null_mut();
static mut statusdict: StringHashPtr = ptr::null_mut();
static mut user_strings: StringHashPtr = ptr::null_mut();
static mut afm_cache: StringHashPtr = ptr::null_mut();
static mut afm_info_cache: StringHashPtr = ptr::null_mut();
static mut afm: AFMHandle = ptr::null_mut();
static mut Fname: *mut c_char = ptr::null_mut();
static mut Fpt: FontPoint = FontPoint { w: 0.0, h: 0.0 };
static mut font_widths: [c_double; 0] = [];
static mut font_ctype: [c_char; 0] = [];
static mut font_is_fixed: c_int = 0;
static mut font_bbox_lly: c_double = 0.0;
static mut printer: *mut c_char = ptr::null_mut();
static mut printer_buf: [c_char; 0] = [];
static mut verbose: c_int = 0;
static mut title: *mut c_char = ptr::null_mut();
static mut quiet: c_int = 0;
static mut fancy_header_default: [c_char; 0] = [];
static mut output_file: *mut c_char = ptr::null_mut();
static mut encoding: InputEncoding = 0;
static mut media_name: *mut c_char = ptr::null_mut();
static mut media_name_buffer: [c_char; 0] = [];
static mut encoding_name: *mut c_char = ptr::null_mut();
static mut encoding_name_buffer: [c_char; 0] = [];
static mut escape_char: c_int = 0;
static mut baselineskip: c_double = 0.0;
static mut ul_ptsize: FontPoint = FontPoint { w: 0.0, h: 0.0 };
static mut ul_gray: c_double = 0.0;
static mut ul_font: *mut c_char = ptr::null_mut();
static mut underlay: *mut c_char = ptr::null_mut();
static mut ul_position_buf: [c_char; 0] = [];
static mut ul_position: *mut c_char = ptr::null_mut();
static mut ul_angle: c_double = 0.0;
static mut ul_style_str: *mut c_char = ptr::null_mut();
static mut ul_style_str_buf: [c_char; 0] = [];
static mut ul_position_p: c_int = 0;
static mut ul_angle_p: c_int = 0;
static mut page_label_format: *mut c_char = ptr::null_mut();
static mut page_label_format_buf: [c_char; 0] = [];
static mut mark_wrapped_lines_style_name: [c_char; 0] = [];
static mut npf_name: *mut c_char = ptr::null_mut();
static mut npf_name_buf: [c_char; 0] = [];
static mut clean_7bit: c_int = 0;
static mut append_ctrl_D: c_int = 0;
static mut highlight_bars: c_uint = 0;
static mut highlight_bar_gray: c_double = 0.0;
static mut page_prefeed: c_int = 0;
static mut accept_composites: c_int = 0;
static mut formfeed_type: FormFeedType = 0;
static mut input_filter_stdin: *mut c_char = ptr::null_mut();
static mut toc: c_int = 0;

// Function implementations would follow here...
// Note: The rest of the functions would need to be similarly converted to Rust