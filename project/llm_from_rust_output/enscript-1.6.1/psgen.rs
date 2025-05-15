use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_uint, c_void, c_double, c_float, c_ulong};
use std::ptr::{null, null_mut};
use std::mem;
use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;
use std::process::Command;
use libc::{FILE, fopen, fclose, fread, fwrite, fseek, fflush, fputs, fprintf, sprintf, sscanf, 
           fgets, popen, pclose, remove, tempnam, strerror, strlen, strrchr, strchr, 
           strncmp, strcmp, strncpy, strcpy, memcpy, exit, abort, free, strtol, strtod,
           atoi, atof, __errno_location, __assert_fail, __ctype_b_loc, dcgettext};
use libc::{_IO_FILE, _IO_marker, size_t, __uid_t, __gid_t, __off_t, __off64_t, _IO_lock_t};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

// Constants and types from original C code
const _ISalnum: c_uint = 8;
const _ISpunct: c_uint = 4;
const _IScntrl: c_uint = 2;
const _ISblank: c_uint = 1;
const _ISgraph: c_uint = 32768;
const _ISprint: c_uint = 16384;
const _ISspace: c_uint = 8192;
const _ISxdigit: c_uint = 4096;
const _ISdigit: c_uint = 2048;
const _ISalpha: c_uint = 1024;
const _ISlower: c_uint = 512;
const _ISupper: c_uint = 256;

#[repr(C)]
struct Passwd {
    pw_name: *mut c_char,
    pw_passwd: *mut c_char,
    pw_uid: __uid_t,
    pw_gid: __gid_t,
    pw_gecos: *mut c_char,
    pw_dir: *mut c_char,
    pw_shell: *mut c_char,
}

#[repr(C)]
struct StringHash {
    // Implementation details omitted
}

type StringHashPtr = *mut StringHash;

#[repr(C)]
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

#[repr(u32)]
enum HeaderType {
    HdrNone = 0,
    HdrSimple = 1,
    HdrFancy = 2,
}

#[repr(u32)]
enum PageLabelFormat {
    LabelShort = 0,
    LabelLong = 1,
}

#[repr(u32)]
enum MarkWrappedLinesStyle {
    MwlsNone = 0,
    MwlsPlus = 1,
    MwlsBox = 2,
    MwlsArrow = 3,
}

#[repr(u32)]
enum NonPrintableFormat {
    NpfSpace = 0,
    NpfQuestionmark = 1,
    NpfCaret = 2,
    NpfOctal = 3,
}

#[repr(u32)]
enum FormFeedType {
    FormfeedColumn = 0,
    FormfeedPage = 1,
    FormfeedHcolumn = 2,
}

#[repr(u32)]
enum LineEndType {
    LeTruncate = 0,
    LeCharWrap = 1,
    LeWordWrap = 2,
}

#[repr(C)]
struct FileLookupCtx {
    name: [c_char; 256],
    suffix: [c_char; 256],
    fullname: [c_char; 512],
}

type PathWalkProc = Option<unsafe extern "C" fn(*mut c_char, *mut c_void) -> c_int>;

#[repr(C)]
struct InputStream {
    is_pipe: c_int,
    fp: *mut FILE,
    buf: [c_uchar; 4096],
    data_in_buf: c_uint,
    bufpos: c_uint,
    nreads: c_uint,
    unget_ch: *mut c_uchar,
    unget_pos: c_uint,
    unget_alloc: c_uint,
}

#[repr(C)]
struct PageRange {
    next: *mut PageRange,
    odd: c_int,
    even: c_int,
    start: c_uint,
    end: c_uint,
}

#[repr(C)]
struct FontPoint {
    w: c_double,
    h: c_double,
}

#[repr(C)]
struct Color {
    r: c_float,
    g: c_float,
    b: c_float,
}

#[repr(u32)]
enum TokenType {
    TNone = 0,
    TEof = 1,
    TString = 2,
    TFormfeed = 3,
    TNewline = 4,
    TCarriageReturn = 5,
    TWrappedNewline = 6,
    TEpsf = 7,
    TSetfilename = 8,
    TSetpagenumber = 9,
    TNewpage = 10,
    TFont = 11,
    TColor = 12,
    TPs = 13,
}

#[repr(C)]
struct Token {
    type_: TokenType,
    flags: c_uint,
    new_x: c_double,
    new_y: c_double,
    new_col: c_int,
    u: TokenUnion,
}

#[repr(C)]
union TokenUnion {
    i: c_int,
    str_: *mut c_char,
    epsf: EpsfData,
    color: Color,
    font: FontData,
    filename: [c_char; 512],
}

#[repr(C)]
struct EpsfData {
    x: c_double,
    y: c_double,
    w: c_double,
    h: c_double,
    xscale: c_double,
    yscale: c_double,
    llx: c_int,
    lly: c_int,
    urx: c_int,
    ury: c_int,
    filename: [c_char; 512],
    skipbuf: *mut c_char,
    skipbuf_len: c_uint,
    skipbuf_pos: c_uint,
    fp: *mut FILE,
    pipe: c_int,
}

#[repr(C)]
struct FontData {
    name: [c_char; 512],
    size: FontPoint,
}

#[repr(u32)]
enum SpecialEscape {
    EscComment = 0,
    EscEpsf = 1,
    EscFont = 2,
    EscColor = 3,
    EscNewpage = 4,
    EscSetfilename = 5,
    EscSetpagenumber = 6,
    EscShade = 7,
    EscBggray = 8,
    EscEscape = 9,
    EscPs = 10,
}

#[repr(C)]
struct EscapeDef {
    name: *mut c_char,
    escape: SpecialEscape,
}

#[repr(C)]
struct PassThroughMagic {
    magic: *mut c_char,
    magiclen: c_uint,
    name: *mut c_char,
    revert_delta: c_int,
}

// Global variables
static mut current_pagenum: c_uint = 0;
static mut total_pages_in_file: c_uint = 0;
static mut input_filenum: c_uint = 0;
static mut current_file_linenum: c_uint = 0;
static mut fname: [c_char; 1024] = [0; 1024];
static mut ps_header_dumped: c_int = 0;
static mut divertfp: *mut FILE = null_mut();
static mut cofp: *mut FILE = null_mut();
static mut do_print: c_int = 1;
static mut user_fontp: c_int = 0;
static mut user_font_name: [c_char; 256] = [0; 256];
static mut user_font_pt: FontPoint = FontPoint { w: 0.0, h: 0.0 };
static mut user_colorp: c_int = 0;
static mut user_color: Color = Color { r: 0.0, g: 0.0, b: 0.0 };
static mut print_line_number_last: c_uint = 0;

// Main functions
unsafe fn dump_ps_header() {
    // Implementation omitted for brevity
}

unsafe fn dump_ps_trailer() {
    // Implementation omitted for brevity
}

unsafe fn process_file(fname_arg: *mut c_char, is: *mut InputStream) {
    // Implementation omitted for brevity
}

// Helper functions
unsafe fn read_special_escape(is: *mut InputStream, token: *mut Token) {
    // Implementation omitted for brevity
}

unsafe fn get_next_token(
    is: *mut InputStream,
    linestart: c_double,
    linepos: c_double,
    col: c_uint,
    linew: c_double,
    token: *mut Token,
) {
    // Implementation omitted for brevity
}

unsafe fn dump_ps_page_header(fname: *mut c_char, empty: c_int) {
    // Implementation omitted for brevity
}

unsafe fn dump_ps_page_trailer() {
    // Implementation omitted for brevity
}

unsafe fn dump_empty_page() {
    // Implementation omitted for brevity
}

unsafe fn recognize_eps_file(token: *mut Token) -> c_int {
    // Implementation omitted for brevity
    0
}

unsafe fn paste_epsf(token: *mut Token) {
    // Implementation omitted for brevity
}

unsafe fn read_float(is: *mut InputStream, units: c_int, horizontal: c_int) -> c_double {
    // Implementation omitted for brevity
    0.0
}

unsafe fn do_pass_through(fname: *mut c_char, is: *mut InputStream) -> c_int {
    // Implementation omitted for brevity
    0
}

unsafe fn print_line_number(
    x: c_double,
    y: c_double,
    space: c_double,
    margin: c_double,
    linenum: c_uint,
) {
    // Implementation omitted for brevity
}

static mut divertfname: [c_char; 512] = [0; 512];

unsafe fn divert() {
    // Implementation omitted for brevity
}

unsafe fn undivert() {
    // Implementation omitted for brevity
}

unsafe fn handle_two_side_options() {
    // Implementation omitted for brevity
}