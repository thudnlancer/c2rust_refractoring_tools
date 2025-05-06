#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type stringhash_st;
    fn strerror(_: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    fn abort() -> !;
    fn free(__ptr: *mut libc::c_void);
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
    fn tilde_subst(from: *mut i8, to: *mut i8);
    fn pathwalk(path: *mut i8, proc_0: PathWalkProc, context: *mut libc::c_void) -> i32;
    fn file_lookup(path: *mut i8, context: *mut libc::c_void) -> i32;
    fn parse_font_spec(
        spec: *mut i8,
        name_return: *mut *mut i8,
        size_return: *mut FontPoint,
    ) -> i32;
    fn escape_string(string: *mut i8) -> *mut i8;
    fn read_font_info();
    fn format_user_string(context_name: *mut i8, string: *mut i8) -> *mut i8;
    fn is_ungetc(ch: i32, is: *mut InputStream) -> i32;
    fn is_getc(is: *mut InputStream) -> i32;
    fn download_font(name: *mut i8);
    fn count_key_value_set(set: StringHashPtr) -> i32;
    fn paste_file(name: *mut i8, suffix: *mut i8) -> i32;
    static mut rotate_even_pages: i32;
    static mut pslevel: u32;
    static mut horizontal_column_height: libc::c_double;
    static mut generate_PageSize: i32;
    static mut output_language_pass_through: i32;
    static mut output_language: *mut i8;
    static mut nup_scale: libc::c_double;
    static mut nup_height: u32;
    static mut nup_width: u32;
    static mut nup_landscape: i32;
    static mut nup_columns: u32;
    static mut nup_rows: u32;
    static mut nup: u32;
    static mut slice: u32;
    static mut slicing: i32;
    static mut file_align: u32;
    static mut toc_fmt_string: *mut i8;
    static mut toc_fp: *mut FILE;
    static mut toc: i32;
    static mut formfeed_type: FormFeedType;
    static mut bggray: libc::c_double;
    static mut line_highlight_gray: libc::c_double;
    static mut borders: i32;
    static mut page_ranges: *mut PageRange;
    static mut page_prefeed: i32;
    static mut highlight_bar_gray: libc::c_double;
    static mut highlight_bars: u32;
    static mut clean_7bit: i32;
    static mut mark_wrapped_lines_style: MarkWrappedLinesStyle;
    static mut non_printable_format: NonPrintableFormat;
    static mut interpret_formfeed: i32;
    static mut start_line_number: u32;
    static mut line_numbers: i32;
    static mut pass_through: i32;
    static mut page_label: PageLabelFormat;
    static mut ul_angle_p: i32;
    static mut ul_position_p: i32;
    static mut ul_style: u32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fclose(__stream: *mut FILE) -> i32;
    fn tempnam(__dir: *const i8, __pfx: *const i8) -> *mut i8;
    fn remove(__filename: *const i8) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn __errno_location() -> *mut i32;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut i8,
        keylen: i32,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> i32;
    fn strhash_get_first(
        hash: StringHashPtr,
        key_return: *mut *mut i8,
        keylen_return: *mut i32,
        data_return: *mut *mut libc::c_void,
    ) -> i32;
    fn strhash_get_next(
        hash: StringHashPtr,
        key_return: *mut *mut i8,
        keylen_return: *mut i32,
        data_return: *mut *mut libc::c_void,
    ) -> i32;
    fn xrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn xstrdup(_: *mut i8) -> *mut i8;
    static mut program: *mut i8;
    static mut ofp: *mut FILE;
    static mut version_string: [i8; 0];
    static mut ps_version_string: [i8; 0];
    static mut date_string: [i8; 0];
    static mut passwd: *mut passwd;
    static mut libpath: [i8; 0];
    static mut media: *mut MediaEntry;
    static mut output_first_line: [i8; 0];
    static mut nl: i32;
    static mut bs: i32;
    static mut ul_angle: libc::c_double;
    static mut ul_y: libc::c_double;
    static mut ul_x: libc::c_double;
    static mut total_pages: i32;
    static mut num_truncated_lines: i32;
    static mut num_missing_chars: i32;
    static mut missing_chars: [i32; 0];
    static mut num_non_printable_chars: i32;
    static mut non_printable_chars: [i32; 0];
    static mut d_page_w: i32;
    static mut d_page_h: i32;
    static mut d_header_w: i32;
    static mut d_header_h: i32;
    static mut d_footer_h: i32;
    static mut d_output_w: i32;
    static mut d_output_h: i32;
    static mut d_output_x_margin: i32;
    static mut d_output_y_margin: i32;
    static mut nup_xpad: u32;
    static mut nup_ypad: u32;
    static mut res_fonts: StringHashPtr;
    static mut download_fonts: StringHashPtr;
    static mut pagedevice: StringHashPtr;
    static mut statusdict: StringHashPtr;
    static mut user_strings: StringHashPtr;
    static mut HFname: *mut i8;
    static mut HFpt: FontPoint;
    static mut Fname: *mut i8;
    static mut Fpt: FontPoint;
    static mut default_Fpt: FontPoint;
    static mut default_Fname: *mut i8;
    static mut font_widths: [libc::c_double; 0];
    static mut font_ctype: [i8; 0];
    static mut font_is_fixed: i32;
    static mut font_bbox_lly: libc::c_double;
    static mut verbose: i32;
    static mut num_copies: i32;
    static mut title: *mut i8;
    static mut num_columns: i32;
    static mut line_end: LineEndType;
    static mut quiet: i32;
    static mut landscape: i32;
    static mut header: HeaderType;
    static mut fancy_header_name: *mut i8;
    static mut line_indent: libc::c_double;
    static mut page_header: *mut i8;
    static mut lines_per_page: u32;
    static mut encoding_name: *mut i8;
    static mut special_escapes: i32;
    static mut escape_char: i32;
    static mut default_escape_char: i32;
    static mut tabsize: i32;
    static mut baselineskip: libc::c_double;
    static mut ul_ptsize: FontPoint;
    static mut ul_gray: libc::c_double;
    static mut ul_font: *mut i8;
    static mut underlay: *mut i8;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fseek(__stream: *mut FILE, __off: i64, __whence: i32) -> i32;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn pclose(__stream: *mut FILE) -> i32;
    fn popen(__command: *const i8, __modes: *const i8) -> *mut FILE;
}
pub type size_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            8192 => C2RustUnnamed::_ISspace,
            4096 => C2RustUnnamed::_ISxdigit,
            2048 => C2RustUnnamed::_ISdigit,
            1024 => C2RustUnnamed::_ISalpha,
            512 => C2RustUnnamed::_ISlower,
            256 => C2RustUnnamed::_ISupper,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
pub type StringHashPtr = *mut stringhash_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct media_entry_st {
    pub next: *mut media_entry_st,
    pub name: *mut i8,
    pub w: i32,
    pub h: i32,
    pub llx: i32,
    pub lly: i32,
    pub urx: i32,
    pub ury: i32,
}
pub type MediaEntry = media_entry_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum HeaderType {
    HDR_NONE,
    HDR_SIMPLE,
    HDR_FANCY,
}
impl HeaderType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            HeaderType::HDR_NONE => 0,
            HeaderType::HDR_SIMPLE => 1,
            HeaderType::HDR_FANCY => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> HeaderType {
        match value {
            0 => HeaderType::HDR_NONE,
            1 => HeaderType::HDR_SIMPLE,
            2 => HeaderType::HDR_FANCY,
            _ => panic!("Invalid value for HeaderType: {}", value),
        }
    }
}
impl AddAssign<u32> for HeaderType {
    fn add_assign(&mut self, rhs: u32) {
        *self = HeaderType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for HeaderType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = HeaderType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for HeaderType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = HeaderType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for HeaderType {
    fn div_assign(&mut self, rhs: u32) {
        *self = HeaderType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for HeaderType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = HeaderType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for HeaderType {
    type Output = HeaderType;
    fn add(self, rhs: u32) -> HeaderType {
        HeaderType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for HeaderType {
    type Output = HeaderType;
    fn sub(self, rhs: u32) -> HeaderType {
        HeaderType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for HeaderType {
    type Output = HeaderType;
    fn mul(self, rhs: u32) -> HeaderType {
        HeaderType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for HeaderType {
    type Output = HeaderType;
    fn div(self, rhs: u32) -> HeaderType {
        HeaderType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for HeaderType {
    type Output = HeaderType;
    fn rem(self, rhs: u32) -> HeaderType {
        HeaderType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum PageLabelFormat {
    LABEL_SHORT,
    LABEL_LONG,
}
impl PageLabelFormat {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            PageLabelFormat::LABEL_SHORT => 0,
            PageLabelFormat::LABEL_LONG => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> PageLabelFormat {
        match value {
            0 => PageLabelFormat::LABEL_SHORT,
            1 => PageLabelFormat::LABEL_LONG,
            _ => panic!("Invalid value for PageLabelFormat: {}", value),
        }
    }
}
impl AddAssign<u32> for PageLabelFormat {
    fn add_assign(&mut self, rhs: u32) {
        *self = PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for PageLabelFormat {
    fn sub_assign(&mut self, rhs: u32) {
        *self = PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for PageLabelFormat {
    fn mul_assign(&mut self, rhs: u32) {
        *self = PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for PageLabelFormat {
    fn div_assign(&mut self, rhs: u32) {
        *self = PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for PageLabelFormat {
    fn rem_assign(&mut self, rhs: u32) {
        *self = PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for PageLabelFormat {
    type Output = PageLabelFormat;
    fn add(self, rhs: u32) -> PageLabelFormat {
        PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for PageLabelFormat {
    type Output = PageLabelFormat;
    fn sub(self, rhs: u32) -> PageLabelFormat {
        PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for PageLabelFormat {
    type Output = PageLabelFormat;
    fn mul(self, rhs: u32) -> PageLabelFormat {
        PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for PageLabelFormat {
    type Output = PageLabelFormat;
    fn div(self, rhs: u32) -> PageLabelFormat {
        PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for PageLabelFormat {
    type Output = PageLabelFormat;
    fn rem(self, rhs: u32) -> PageLabelFormat {
        PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum MarkWrappedLinesStyle {
    MWLS_NONE = 0,
    MWLS_PLUS = 1,
    MWLS_BOX = 2,
    MWLS_ARROW = 3,
}
impl MarkWrappedLinesStyle {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            MarkWrappedLinesStyle::MWLS_NONE => 0,
            MarkWrappedLinesStyle::MWLS_PLUS => 1,
            MarkWrappedLinesStyle::MWLS_BOX => 2,
            MarkWrappedLinesStyle::MWLS_ARROW => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> MarkWrappedLinesStyle {
        match value {
            0 => MarkWrappedLinesStyle::MWLS_NONE,
            1 => MarkWrappedLinesStyle::MWLS_PLUS,
            2 => MarkWrappedLinesStyle::MWLS_BOX,
            3 => MarkWrappedLinesStyle::MWLS_ARROW,
            _ => panic!("Invalid value for MarkWrappedLinesStyle: {}", value),
        }
    }
}
impl AddAssign<u32> for MarkWrappedLinesStyle {
    fn add_assign(&mut self, rhs: u32) {
        *self = MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for MarkWrappedLinesStyle {
    fn sub_assign(&mut self, rhs: u32) {
        *self = MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for MarkWrappedLinesStyle {
    fn mul_assign(&mut self, rhs: u32) {
        *self = MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for MarkWrappedLinesStyle {
    fn div_assign(&mut self, rhs: u32) {
        *self = MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for MarkWrappedLinesStyle {
    fn rem_assign(&mut self, rhs: u32) {
        *self = MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for MarkWrappedLinesStyle {
    type Output = MarkWrappedLinesStyle;
    fn add(self, rhs: u32) -> MarkWrappedLinesStyle {
        MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for MarkWrappedLinesStyle {
    type Output = MarkWrappedLinesStyle;
    fn sub(self, rhs: u32) -> MarkWrappedLinesStyle {
        MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for MarkWrappedLinesStyle {
    type Output = MarkWrappedLinesStyle;
    fn mul(self, rhs: u32) -> MarkWrappedLinesStyle {
        MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for MarkWrappedLinesStyle {
    type Output = MarkWrappedLinesStyle;
    fn div(self, rhs: u32) -> MarkWrappedLinesStyle {
        MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for MarkWrappedLinesStyle {
    type Output = MarkWrappedLinesStyle;
    fn rem(self, rhs: u32) -> MarkWrappedLinesStyle {
        MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum NonPrintableFormat {
    NPF_SPACE,
    NPF_QUESTIONMARK,
    NPF_CARET,
    NPF_OCTAL,
}
impl NonPrintableFormat {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            NonPrintableFormat::NPF_SPACE => 0,
            NonPrintableFormat::NPF_QUESTIONMARK => 1,
            NonPrintableFormat::NPF_CARET => 2,
            NonPrintableFormat::NPF_OCTAL => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> NonPrintableFormat {
        match value {
            0 => NonPrintableFormat::NPF_SPACE,
            1 => NonPrintableFormat::NPF_QUESTIONMARK,
            2 => NonPrintableFormat::NPF_CARET,
            3 => NonPrintableFormat::NPF_OCTAL,
            _ => panic!("Invalid value for NonPrintableFormat: {}", value),
        }
    }
}
impl AddAssign<u32> for NonPrintableFormat {
    fn add_assign(&mut self, rhs: u32) {
        *self = NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for NonPrintableFormat {
    fn sub_assign(&mut self, rhs: u32) {
        *self = NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for NonPrintableFormat {
    fn mul_assign(&mut self, rhs: u32) {
        *self = NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for NonPrintableFormat {
    fn div_assign(&mut self, rhs: u32) {
        *self = NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for NonPrintableFormat {
    fn rem_assign(&mut self, rhs: u32) {
        *self = NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for NonPrintableFormat {
    type Output = NonPrintableFormat;
    fn add(self, rhs: u32) -> NonPrintableFormat {
        NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for NonPrintableFormat {
    type Output = NonPrintableFormat;
    fn sub(self, rhs: u32) -> NonPrintableFormat {
        NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for NonPrintableFormat {
    type Output = NonPrintableFormat;
    fn mul(self, rhs: u32) -> NonPrintableFormat {
        NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for NonPrintableFormat {
    type Output = NonPrintableFormat;
    fn div(self, rhs: u32) -> NonPrintableFormat {
        NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for NonPrintableFormat {
    type Output = NonPrintableFormat;
    fn rem(self, rhs: u32) -> NonPrintableFormat {
        NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum FormFeedType {
    FORMFEED_COLUMN,
    FORMFEED_PAGE,
    FORMFEED_HCOLUMN,
}
impl FormFeedType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            FormFeedType::FORMFEED_COLUMN => 0,
            FormFeedType::FORMFEED_PAGE => 1,
            FormFeedType::FORMFEED_HCOLUMN => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> FormFeedType {
        match value {
            0 => FormFeedType::FORMFEED_COLUMN,
            1 => FormFeedType::FORMFEED_PAGE,
            2 => FormFeedType::FORMFEED_HCOLUMN,
            _ => panic!("Invalid value for FormFeedType: {}", value),
        }
    }
}
impl AddAssign<u32> for FormFeedType {
    fn add_assign(&mut self, rhs: u32) {
        *self = FormFeedType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for FormFeedType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = FormFeedType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for FormFeedType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = FormFeedType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for FormFeedType {
    fn div_assign(&mut self, rhs: u32) {
        *self = FormFeedType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for FormFeedType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = FormFeedType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for FormFeedType {
    type Output = FormFeedType;
    fn add(self, rhs: u32) -> FormFeedType {
        FormFeedType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for FormFeedType {
    type Output = FormFeedType;
    fn sub(self, rhs: u32) -> FormFeedType {
        FormFeedType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for FormFeedType {
    type Output = FormFeedType;
    fn mul(self, rhs: u32) -> FormFeedType {
        FormFeedType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for FormFeedType {
    type Output = FormFeedType;
    fn div(self, rhs: u32) -> FormFeedType {
        FormFeedType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for FormFeedType {
    type Output = FormFeedType;
    fn rem(self, rhs: u32) -> FormFeedType {
        FormFeedType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum LineEndType {
    LE_TRUNCATE,
    LE_CHAR_WRAP,
    LE_WORD_WRAP,
}
impl LineEndType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            LineEndType::LE_TRUNCATE => 0,
            LineEndType::LE_CHAR_WRAP => 1,
            LineEndType::LE_WORD_WRAP => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> LineEndType {
        match value {
            0 => LineEndType::LE_TRUNCATE,
            1 => LineEndType::LE_CHAR_WRAP,
            2 => LineEndType::LE_WORD_WRAP,
            _ => panic!("Invalid value for LineEndType: {}", value),
        }
    }
}
impl AddAssign<u32> for LineEndType {
    fn add_assign(&mut self, rhs: u32) {
        *self = LineEndType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for LineEndType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = LineEndType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for LineEndType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = LineEndType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for LineEndType {
    fn div_assign(&mut self, rhs: u32) {
        *self = LineEndType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for LineEndType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = LineEndType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for LineEndType {
    type Output = LineEndType;
    fn add(self, rhs: u32) -> LineEndType {
        LineEndType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for LineEndType {
    type Output = LineEndType;
    fn sub(self, rhs: u32) -> LineEndType {
        LineEndType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for LineEndType {
    type Output = LineEndType;
    fn mul(self, rhs: u32) -> LineEndType {
        LineEndType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for LineEndType {
    type Output = LineEndType;
    fn div(self, rhs: u32) -> LineEndType {
        LineEndType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for LineEndType {
    type Output = LineEndType;
    fn rem(self, rhs: u32) -> LineEndType {
        LineEndType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_lookup_ctx_st {
    pub name: [i8; 256],
    pub suffix: [i8; 256],
    pub fullname: [i8; 512],
}
pub type FileLookupCtx = file_lookup_ctx_st;
pub type PathWalkProc = Option<unsafe extern "C" fn(*mut i8, *mut libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_stream_st {
    pub is_pipe: i32,
    pub fp: *mut FILE,
    pub buf: [u8; 4096],
    pub data_in_buf: u32,
    pub bufpos: u32,
    pub nreads: u32,
    pub unget_ch: *mut u8,
    pub unget_pos: u32,
    pub unget_alloc: u32,
}
pub type InputStream = input_stream_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct page_range_st {
    pub next: *mut page_range_st,
    pub odd: i32,
    pub even: i32,
    pub start: u32,
    pub end: u32,
}
pub type PageRange = page_range_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_point_st {
    pub w: libc::c_double,
    pub h: libc::c_double,
}
pub type FontPoint = font_point_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_st {
    pub r: libc::c_float,
    pub g: libc::c_float,
    pub b: libc::c_float,
}
pub type Color = color_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum TokenType {
    tNONE,
    tEOF,
    tSTRING,
    tFORMFEED,
    tNEWLINE,
    tCARRIAGE_RETURN,
    tWRAPPED_NEWLINE,
    tEPSF,
    tSETFILENAME,
    tSETPAGENUMBER,
    tNEWPAGE,
    tFONT,
    tCOLOR,
    tPS,
}
impl TokenType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            TokenType::tNONE => 0,
            TokenType::tEOF => 1,
            TokenType::tSTRING => 2,
            TokenType::tFORMFEED => 3,
            TokenType::tNEWLINE => 4,
            TokenType::tCARRIAGE_RETURN => 5,
            TokenType::tWRAPPED_NEWLINE => 6,
            TokenType::tEPSF => 7,
            TokenType::tSETFILENAME => 8,
            TokenType::tSETPAGENUMBER => 9,
            TokenType::tNEWPAGE => 10,
            TokenType::tFONT => 11,
            TokenType::tCOLOR => 12,
            TokenType::tPS => 13,
        }
    }
    fn from_libc_c_uint(value: u32) -> TokenType {
        match value {
            0 => TokenType::tNONE,
            1 => TokenType::tEOF,
            2 => TokenType::tSTRING,
            3 => TokenType::tFORMFEED,
            4 => TokenType::tNEWLINE,
            5 => TokenType::tCARRIAGE_RETURN,
            6 => TokenType::tWRAPPED_NEWLINE,
            7 => TokenType::tEPSF,
            8 => TokenType::tSETFILENAME,
            9 => TokenType::tSETPAGENUMBER,
            10 => TokenType::tNEWPAGE,
            11 => TokenType::tFONT,
            12 => TokenType::tCOLOR,
            13 => TokenType::tPS,
            _ => panic!("Invalid value for TokenType: {}", value),
        }
    }
}
impl AddAssign<u32> for TokenType {
    fn add_assign(&mut self, rhs: u32) {
        *self = TokenType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for TokenType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = TokenType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for TokenType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = TokenType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for TokenType {
    fn div_assign(&mut self, rhs: u32) {
        *self = TokenType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for TokenType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = TokenType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for TokenType {
    type Output = TokenType;
    fn add(self, rhs: u32) -> TokenType {
        TokenType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for TokenType {
    type Output = TokenType;
    fn sub(self, rhs: u32) -> TokenType {
        TokenType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for TokenType {
    type Output = TokenType;
    fn mul(self, rhs: u32) -> TokenType {
        TokenType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for TokenType {
    type Output = TokenType;
    fn div(self, rhs: u32) -> TokenType {
        TokenType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for TokenType {
    type Output = TokenType;
    fn rem(self, rhs: u32) -> TokenType {
        TokenType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type Token = gs_token_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gs_token_st {
    pub type_0: TokenType,
    pub flags: u32,
    pub new_x: libc::c_double,
    pub new_y: libc::c_double,
    pub new_col: i32,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub i: i32,
    pub str_0: *mut i8,
    pub epsf: C2RustUnnamed_2,
    pub color: Color,
    pub font: C2RustUnnamed_1,
    pub filename: [i8; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub name: [i8; 512],
    pub size: FontPoint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub w: libc::c_double,
    pub h: libc::c_double,
    pub xscale: libc::c_double,
    pub yscale: libc::c_double,
    pub llx: i32,
    pub lly: i32,
    pub urx: i32,
    pub ury: i32,
    pub filename: [i8; 512],
    pub skipbuf: *mut i8,
    pub skipbuf_len: u32,
    pub skipbuf_pos: u32,
    pub fp: *mut FILE,
    pub pipe: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum SpecialEscape {
    ESC_COMMENT,
    ESC_EPSF,
    ESC_FONT,
    ESC_COLOR,
    ESC_NEWPAGE,
    ESC_SETFILENAME,
    ESC_SETPAGENUMBER,
    ESC_SHADE,
    ESC_BGGRAY,
    ESC_ESCAPE,
    ESC_PS,
}
impl SpecialEscape {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            SpecialEscape::ESC_COMMENT => 0,
            SpecialEscape::ESC_EPSF => 1,
            SpecialEscape::ESC_FONT => 2,
            SpecialEscape::ESC_COLOR => 3,
            SpecialEscape::ESC_NEWPAGE => 4,
            SpecialEscape::ESC_SETFILENAME => 5,
            SpecialEscape::ESC_SETPAGENUMBER => 6,
            SpecialEscape::ESC_SHADE => 7,
            SpecialEscape::ESC_BGGRAY => 8,
            SpecialEscape::ESC_ESCAPE => 9,
            SpecialEscape::ESC_PS => 10,
        }
    }
    fn from_libc_c_uint(value: u32) -> SpecialEscape {
        match value {
            0 => SpecialEscape::ESC_COMMENT,
            1 => SpecialEscape::ESC_EPSF,
            2 => SpecialEscape::ESC_FONT,
            3 => SpecialEscape::ESC_COLOR,
            4 => SpecialEscape::ESC_NEWPAGE,
            5 => SpecialEscape::ESC_SETFILENAME,
            6 => SpecialEscape::ESC_SETPAGENUMBER,
            7 => SpecialEscape::ESC_SHADE,
            8 => SpecialEscape::ESC_BGGRAY,
            9 => SpecialEscape::ESC_ESCAPE,
            10 => SpecialEscape::ESC_PS,
            _ => panic!("Invalid value for SpecialEscape: {}", value),
        }
    }
}
impl AddAssign<u32> for SpecialEscape {
    fn add_assign(&mut self, rhs: u32) {
        *self = SpecialEscape::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for SpecialEscape {
    fn sub_assign(&mut self, rhs: u32) {
        *self = SpecialEscape::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for SpecialEscape {
    fn mul_assign(&mut self, rhs: u32) {
        *self = SpecialEscape::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for SpecialEscape {
    fn div_assign(&mut self, rhs: u32) {
        *self = SpecialEscape::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for SpecialEscape {
    fn rem_assign(&mut self, rhs: u32) {
        *self = SpecialEscape::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for SpecialEscape {
    type Output = SpecialEscape;
    fn add(self, rhs: u32) -> SpecialEscape {
        SpecialEscape::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for SpecialEscape {
    type Output = SpecialEscape;
    fn sub(self, rhs: u32) -> SpecialEscape {
        SpecialEscape::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for SpecialEscape {
    type Output = SpecialEscape;
    fn mul(self, rhs: u32) -> SpecialEscape {
        SpecialEscape::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for SpecialEscape {
    type Output = SpecialEscape;
    fn div(self, rhs: u32) -> SpecialEscape {
        SpecialEscape::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for SpecialEscape {
    type Output = SpecialEscape;
    fn rem(self, rhs: u32) -> SpecialEscape {
        SpecialEscape::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub name: *mut i8,
    pub escape: SpecialEscape,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub magic: *mut i8,
    pub magiclen: u32,
    pub name: *mut i8,
    pub revert_delta: i32,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const i8) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut i8);
}
#[no_mangle]
pub static mut current_pagenum: u32 = 0;
#[no_mangle]
pub static mut total_pages_in_file: u32 = 0;
#[no_mangle]
pub static mut input_filenum: u32 = 0 as i32 as u32;
#[no_mangle]
pub static mut current_file_linenum: u32 = 0;
#[no_mangle]
pub static mut fname: [i8; 1024] = [0; 1024];
static mut ps_header_dumped: i32 = 0 as i32;
static mut divertfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut cofp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut do_print: i32 = 1 as i32;
static mut user_fontp: i32 = 0 as i32;
static mut user_font_name: [i8; 256] = [0; 256];
static mut user_font_pt: FontPoint = FontPoint { w: 0., h: 0. };
static mut user_colorp: i32 = 0 as i32;
static mut user_color: Color = Color { r: 0., g: 0., b: 0. };
static mut print_line_number_last: u32 = 0;
#[no_mangle]
pub unsafe extern "C" fn dump_ps_header() {
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut cp2: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut got: i32 = 0;
    if ps_header_dumped != 0 {
        return;
    }
    ps_header_dumped = 1 as i32;
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%s\n\0" as *const u8 as *const i8,
            output_first_line.as_mut_ptr(),
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%BoundingBox: %d %d %d %d\n\0" as *const u8 as *const i8,
            (*media).llx,
            (*media).lly,
            (*media).urx,
            (*media).ury,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%Title: %s\n\0" as *const u8 as *const i8, title);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%For: %s\n\0" as *const u8 as *const i8, (*passwd).pw_gecos);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%Creator: %s\n\0" as *const u8 as *const i8,
            version_string.as_mut_ptr(),
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%CreationDate: %s\n\0" as *const u8 as *const i8,
            date_string.as_mut_ptr(),
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%Orientation: %s\n\0" as *const u8 as *const i8,
            if nup > 1 as i32 as u32 && nup_landscape != 0
                || nup == 1 as i32 as u32 && landscape != 0
            {
                b"Landscape\0" as *const u8 as *const i8
            } else {
                b"Portrait\0" as *const u8 as *const i8
            },
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%Pages: (atend)\n\0" as *const u8 as *const i8);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%DocumentMedia: %s %d %d 0 () ()\n\0" as *const u8 as *const i8,
            (*media).name,
            (*media).w,
            (*media).h,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%DocumentNeededResources: (atend)\n\0" as *const u8 as *const i8,
        );
    }
    if count_key_value_set(pagedevice) > 0 as i32 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"%%%%LanguageLevel: 2\n\0" as *const u8 as *const i8);
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndComments\n\0" as *const u8 as *const i8);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%BeginProlog\n\0" as *const u8 as *const i8);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%BeginResource: procset Enscript-Prolog %s\n\0" as *const u8
                as *const i8,
            ps_version_string.as_mut_ptr(),
        );
    }
    if paste_file(
        b"enscript\0" as *const u8 as *const i8 as *mut i8,
        b".pro\0" as *const u8 as *const i8 as *mut i8,
    ) == 0
    {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"couldn't find prolog \"%s\": %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"enscript.pro\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fflush(stderr);
        exit(1 as i32);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndResource\n\0" as *const u8 as *const i8);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%BeginResource: procset Enscript-Encoding-%s %s\n\0" as *const u8
                as *const i8,
            encoding_name,
            ps_version_string.as_mut_ptr(),
        );
    }
    if paste_file(encoding_name, b".enc\0" as *const u8 as *const i8 as *mut i8) == 0 {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"couldn't find encoding file \"%s.enc\": %s\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            encoding_name,
            strerror(*__errno_location()),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fflush(stderr);
        exit(1 as i32);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndResource\n\0" as *const u8 as *const i8);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndProlog\n\0" as *const u8 as *const i8);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%BeginSetup\n\0" as *const u8 as *const i8);
    }
    got = strhash_get_first(
        download_fonts,
        &mut cp,
        &mut j,
        &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
    );
    while got != 0 {
        download_font(cp);
        got = strhash_get_next(
            download_fonts,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
        );
    }
    got = strhash_get_first(
        res_fonts,
        &mut cp,
        &mut j,
        &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
    );
    while got != 0 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%%%%IncludeResource: font %s\n\0" as *const u8 as *const i8,
                cp,
            );
        }
        got = strhash_get_next(
            res_fonts,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/HFpt_w %g def\n\0" as *const u8 as *const i8, HFpt.w);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/HFpt_h %g def\n\0" as *const u8 as *const i8, HFpt.h);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/%s /HF-gs-font MF\n\0" as *const u8 as *const i8, HFname);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/HF /HF-gs-font findfont [HFpt_w 0 0 HFpt_h 0 0] makefont def\n\0"
                as *const u8 as *const i8,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/%s /F-gs-font MF\n\0" as *const u8 as *const i8, Fname);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/F-gs-font %g %g SF\n\0" as *const u8 as *const i8,
            Fpt.w,
            Fpt.h,
        );
    }
    if !underlay.is_null() {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"/ul_str (%s) def\n\0" as *const u8 as *const i8, underlay);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/ul_w_ptsize %g def\n\0" as *const u8 as *const i8,
                ul_ptsize.w,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/ul_h_ptsize %g def\n\0" as *const u8 as *const i8,
                ul_ptsize.h,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"/ul_gray %g def\n\0" as *const u8 as *const i8, ul_gray);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"/ul_x %g def\n\0" as *const u8 as *const i8, ul_x);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"/ul_y %g def\n\0" as *const u8 as *const i8, ul_y);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"/ul_angle %g def\n\0" as *const u8 as *const i8, ul_angle);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"/ul_style %d def\n\0" as *const u8 as *const i8, ul_style);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"/%s /F-ul-font MF\n\0" as *const u8 as *const i8, ul_font);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/ul_font /F-ul-font findfont [ul_w_ptsize 0 0 ul_h_ptsize 0 0] makefont def\n\0"
                    as *const u8 as *const i8,
            );
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/#copies %d def\n\0" as *const u8 as *const i8, num_copies);
    }
    if page_prefeed != 0 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"true page_prefeed\n\0" as *const u8 as *const i8);
        }
    }
    if count_key_value_set(statusdict) > 0 as i32 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%% Statustdict definitions:\nstatusdict begin\n  \0" as *const u8
                    as *const i8,
            );
        }
        i = 2 as i32;
        got = strhash_get_first(
            statusdict,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
        );
        while got != 0 {
            j = (strlen(cp))
                .wrapping_add(1 as i32 as u64)
                .wrapping_add(strlen(cp2))
                .wrapping_add(1 as i32 as u64) as i32;
            if i + j > 75 as i32 {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(cofp, b"\n  \0" as *const u8 as *const i8);
                }
                i = 2 as i32;
            }
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"%s %s \0" as *const u8 as *const i8, cp2, cp);
            }
            i += j;
            got = strhash_get_next(
                statusdict,
                &mut cp,
                &mut j,
                &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"\nend\n\0" as *const u8 as *const i8);
        }
    }
    if pslevel >= 2 as i32 as u32
        && (count_key_value_set(pagedevice) > 0 as i32 || generate_PageSize != 0)
    {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"%% Pagedevice definitions:\n\0" as *const u8 as *const i8);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"gs_languagelevel 1 gt {\n  <<\n    \0" as *const u8 as *const i8,
            );
        }
        i = 4 as i32;
        got = strhash_get_first(
            pagedevice,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
        );
        while got != 0 {
            j = (strlen(cp2))
                .wrapping_add(1 as i32 as u64)
                .wrapping_add(strlen(cp))
                .wrapping_add(2 as i32 as u64) as i32;
            if i + j > 75 as i32 {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(cofp, b"\n    \0" as *const u8 as *const i8);
                }
                i = 4 as i32;
            }
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"/%s %s \0" as *const u8 as *const i8, cp, cp2);
            }
            i += j;
            got = strhash_get_next(
                pagedevice,
                &mut cp,
                &mut j,
                &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
            );
        }
        if generate_PageSize != 0 {
            if i + 21 as i32 > 75 as i32 {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(cofp, b"\n    \0" as *const u8 as *const i8);
                }
                i = 4 as i32;
            }
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"/PageSize [%d %d] \0" as *const u8 as *const i8,
                    (*media).w,
                    (*media).h,
                );
            }
            i += 21 as i32;
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"\n  >> setpagedevice\n} if\n\0" as *const u8 as *const i8);
        }
    }
    if header as u32 != HeaderType::HDR_NONE as i32 as u32 {
        let mut hdr: *mut i8 = 0 as *mut i8;
        if header as u32 == HeaderType::HDR_SIMPLE as i32 as u32 {
            hdr = b"simple\0" as *const u8 as *const i8 as *mut i8;
        } else {
            hdr = fancy_header_name;
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%%%%BeginResource: procset Enscript-Header-%s %s\n\0" as *const u8
                    as *const i8,
                hdr,
                ps_version_string.as_mut_ptr(),
            );
        }
        if paste_file(hdr, b".hdr\0" as *const u8 as *const i8 as *mut i8) == 0 {
            fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"couldn't find header definition file \"%s.hdr\": %s\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                hdr,
                strerror(*__errno_location()),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
            fflush(stderr);
            exit(1 as i32);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"%%%%EndResource\n\0" as *const u8 as *const i8);
        }
    }
    d_output_w = d_page_w;
    d_output_h = d_page_h - d_header_h - d_footer_h;
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/d_page_w %d def\n\0" as *const u8 as *const i8, d_page_w);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/d_page_h %d def\n\0" as *const u8 as *const i8, d_page_h);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/d_header_x %d def\n\0" as *const u8 as *const i8, 0 as i32);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_header_y %d def\n\0" as *const u8 as *const i8,
            d_output_h + d_footer_h,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/d_header_w %d def\n\0" as *const u8 as *const i8, d_header_w);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/d_header_h %d def\n\0" as *const u8 as *const i8, d_header_h);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/d_footer_x %d def\n\0" as *const u8 as *const i8, 0 as i32);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/d_footer_y %d def\n\0" as *const u8 as *const i8, 0 as i32);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/d_footer_w %d def\n\0" as *const u8 as *const i8, d_header_w);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/d_footer_h %d def\n\0" as *const u8 as *const i8, d_footer_h);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/d_output_w %d def\n\0" as *const u8 as *const i8, d_output_w);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/d_output_h %d def\n\0" as *const u8 as *const i8, d_output_h);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/cols %d def\n\0" as *const u8 as *const i8, num_columns);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndSetup\n\0" as *const u8 as *const i8);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dump_ps_trailer() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut got: i32 = 0;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nup_subpage: u32 = 0;
    if ps_header_dumped == 0 {
        return;
    }
    nup_subpage = ((total_pages - 1 as i32) as u32).wrapping_rem(nup);
    if nup > 1 as i32 as u32 && nup_subpage.wrapping_add(1 as i32 as u32) != nup {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"_R\nS\n\0" as *const u8 as *const i8);
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%Trailer\n\0" as *const u8 as *const i8);
    }
    if page_prefeed != 0 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"false page_prefeed\n\0" as *const u8 as *const i8);
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%Pages: %d\n\0" as *const u8 as *const i8, total_pages);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%DocumentNeededResources: font \0" as *const u8 as *const i8);
    }
    i = 32 as i32;
    got = strhash_get_first(res_fonts, &mut cp, &mut j, &mut value);
    while got != 0 {
        if (i as u64).wrapping_add(strlen(cp)).wrapping_add(1 as i32 as u64)
            > 75 as i32 as u64
        {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"\n%%%%+ font \0" as *const u8 as *const i8);
            }
            i = 9 as i32;
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"%s \0" as *const u8 as *const i8, cp);
        }
        i = (i as u64).wrapping_add((strlen(cp)).wrapping_add(1 as i32 as u64)) as i32
            as i32;
        got = strhash_get_next(res_fonts, &mut cp, &mut j, &mut value);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"\n%%%%EOF\n\0" as *const u8 as *const i8);
    }
}
#[no_mangle]
pub unsafe extern "C" fn process_file(mut fname_arg: *mut i8, mut is: *mut InputStream) {
    let mut col: i32 = 0;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut lx: libc::c_double = 0.;
    let mut ly: libc::c_double = 0.;
    let mut linewidth: libc::c_double = 0.;
    let mut lineend: libc::c_double = 0.;
    let mut done: i32 = 0 as i32;
    let mut page_clear: i32 = 1 as i32;
    let mut line_column: u32 = 0;
    let mut current_linenum: u32 = 0;
    let mut linenumber_space: libc::c_double = 0 as i32 as libc::c_double;
    let mut linenumber_margin: libc::c_double = 0 as i32 as libc::c_double;
    let mut token: Token = Token {
        type_0: TokenType::tNONE,
        flags: 0,
        new_x: 0.,
        new_y: 0.,
        new_col: 0,
        u: C2RustUnnamed_0 { i: 0 },
    };
    let mut reuse_last_token: i32 = 0 as i32;
    let mut current_slice: u32 = 1 as i32 as u32;
    let mut last_wrapped_line: i32 = -(1 as i32);
    let mut last_spaced_file_linenum: i32 = -(1 as i32);
    strcpy(fname.as_mut_ptr(), fname_arg);
    current_pagenum = 0 as i32 as u32;
    total_pages_in_file = 0 as i32 as u32;
    current_file_linenum = start_line_number;
    linenumber_space = *font_widths.as_mut_ptr().offset('0' as i32 as u8 as isize)
        * 5 as i32 as libc::c_double + 1.0f64;
    linenumber_margin = *font_widths.as_mut_ptr().offset(':' as i32 as u8 as isize)
        + *font_widths.as_mut_ptr().offset('m' as i32 as u8 as isize);
    input_filenum = input_filenum.wrapping_add(1);
    input_filenum;
    print_line_number_last = -(1 as i32) as u32;
    if pass_through != 0 || output_language_pass_through != 0 {
        if do_pass_through(fname.as_mut_ptr(), is) != 0 {
            return;
        }
    }
    dump_ps_header();
    while (total_pages as u32).wrapping_rem(file_align) != 0 as i32 as u32 {
        total_pages += 1;
        total_pages;
        dump_empty_page();
    }
    if quiet == 0 && verbose >= 1 as i32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"processing file \"%s\"...\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fname.as_mut_ptr(),
        );
    }
    linewidth = (d_output_w / num_columns - 2 as i32 * d_output_x_margin)
        as libc::c_double - line_indent;
    divert();
    while done == 0 {
        page_clear = 1 as i32;
        col = 0 as i32;
        's_120: while done == 0 && col < num_columns {
            x = ((col * d_output_w) as libc::c_float / num_columns as libc::c_float
                + d_output_x_margin as libc::c_float) as libc::c_double + line_indent;
            lx = x;
            lineend = lx + linewidth;
            y = (d_footer_h + d_output_h - d_output_y_margin) as libc::c_double
                - (Fpt.h + baselineskip);
            ly = y;
            current_linenum = 0 as i32 as u32;
            line_column = 0 as i32 as u32;
            loop {
                if line_numbers != 0 && line_column == 0 as i32 as u32
                    && current_file_linenum != last_spaced_file_linenum as u32
                {
                    x += linenumber_space + linenumber_margin;
                    last_spaced_file_linenum = current_file_linenum as i32;
                }
                if reuse_last_token == 0 {
                    get_next_token(is, lx, x, line_column, lineend, &mut token);
                }
                reuse_last_token = 0 as i32;
                if token.type_0 as u32 == TokenType::tEOF as i32 as u32 {
                    done = 1 as i32;
                    break 's_120;
                } else {
                    if page_clear != 0 {
                        let mut pr: *mut PageRange = 0 as *mut PageRange;
                        current_pagenum = current_pagenum.wrapping_add(1);
                        current_pagenum;
                        total_pages_in_file = total_pages_in_file.wrapping_add(1);
                        total_pages_in_file;
                        if page_ranges.is_null() {
                            do_print = 1 as i32;
                        } else {
                            do_print = 0 as i32;
                            pr = page_ranges;
                            while !pr.is_null() {
                                if (*pr).odd != 0 || (*pr).even != 0 {
                                    if (*pr).odd != 0
                                        && current_pagenum.wrapping_rem(2 as i32 as u32)
                                            == 1 as i32 as u32
                                        || (*pr).even != 0
                                            && current_pagenum.wrapping_rem(2 as i32 as u32)
                                                == 0 as i32 as u32
                                    {
                                        do_print = 1 as i32;
                                        break;
                                    }
                                } else if (*pr).start <= current_pagenum
                                    && current_pagenum <= (*pr).end
                                {
                                    do_print = 1 as i32;
                                    break;
                                }
                                pr = (*pr).next;
                            }
                        }
                        if do_print != 0 {
                            total_pages += 1;
                            total_pages;
                        }
                        dump_ps_page_header(fname.as_mut_ptr(), 0 as i32);
                        page_clear = 0 as i32;
                    }
                    if line_numbers != 0 && line_column == 0 as i32 as u32
                        && token.type_0 as u32 != TokenType::tFORMFEED as i32 as u32
                    {
                        print_line_number(
                            lx,
                            y,
                            linenumber_space,
                            linenumber_margin,
                            current_file_linenum,
                        );
                    }
                    if line_column == 0 as i32 as u32 && line_highlight_gray < 1.0f64 {
                        if cofp.is_null() {
                            cofp = ofp;
                        }
                        if do_print != 0 {
                            fprintf(
                                cofp,
                                b"%g %g %g %g %g line_highlight\n\0" as *const u8
                                    as *const i8,
                                lx,
                                y - baselineskip
                                    + font_bbox_lly * Fpt.h / 1000 as i32 as libc::c_double,
                                linewidth,
                                Fpt.h + baselineskip,
                                line_highlight_gray,
                            );
                        }
                    }
                    match token.type_0 as u32 {
                        3 => {
                            match formfeed_type as u32 {
                                0 => {
                                    break;
                                }
                                1 => {
                                    break 's_120;
                                }
                                2 => {}
                                _ => {
                                    continue;
                                }
                            }
                            let mut current_row: i32 = 0;
                            current_row = ((ly - y) / horizontal_column_height) as i32;
                            y = ly
                                - (current_row + 1 as i32) as libc::c_double
                                    * horizontal_column_height;
                            if y < (d_footer_h + d_output_y_margin) as libc::c_double {
                                break;
                            }
                        }
                        2 => {
                            if slicing == 0 as i32 || current_slice == slice {
                                if bggray < 1.0f64 {
                                    if cofp.is_null() {
                                        cofp = ofp;
                                    }
                                    if do_print != 0 {
                                        fprintf(
                                            cofp,
                                            b"%g %g %g %g %g (%s) bgs\n\0" as *const u8 as *const i8,
                                            x,
                                            y,
                                            Fpt.h + baselineskip,
                                            baselineskip
                                                - font_bbox_lly * Fpt.h / 1000 as i32 as libc::c_double,
                                            bggray,
                                            token.u.str_0,
                                        );
                                    }
                                } else {
                                    if cofp.is_null() {
                                        cofp = ofp;
                                    }
                                    if do_print != 0 {
                                        fprintf(
                                            cofp,
                                            b"%g %g M\n(%s) s\n\0" as *const u8 as *const i8,
                                            x,
                                            y,
                                            token.u.str_0,
                                        );
                                    }
                                }
                            }
                            x = token.new_x;
                            line_column = token.new_col as u32;
                        }
                        5 => {
                            x = ((col * d_output_w) as libc::c_float
                                / num_columns as libc::c_float
                                + d_output_x_margin as libc::c_float) as libc::c_double
                                + line_indent;
                            line_column = 0 as i32 as u32;
                        }
                        4 | 6 => {
                            if token.type_0 as u32 == TokenType::tNEWLINE as i32 as u32 {
                                current_file_linenum = current_file_linenum.wrapping_add(1);
                                current_file_linenum;
                                current_slice = 1 as i32 as u32;
                                y -= Fpt.h + baselineskip;
                            } else {
                                current_slice = current_slice.wrapping_add(1);
                                current_slice;
                                if slicing == 0 {
                                    match mark_wrapped_lines_style as u32 {
                                        0 => {}
                                        1 => {
                                            if cofp.is_null() {
                                                cofp = ofp;
                                            }
                                            if do_print != 0 {
                                                fprintf(
                                                    cofp,
                                                    b"%g %g M (+) s\n\0" as *const u8 as *const i8,
                                                    x,
                                                    y,
                                                );
                                            }
                                        }
                                        _ => {
                                            if cofp.is_null() {
                                                cofp = ofp;
                                            }
                                            if do_print != 0 {
                                                fprintf(
                                                    cofp,
                                                    b"%g %g %g %g %d wrapped_line_mark\n\0" as *const u8
                                                        as *const i8,
                                                    x,
                                                    y,
                                                    Fpt.w,
                                                    Fpt.h,
                                                    mark_wrapped_lines_style as u32,
                                                );
                                            }
                                        }
                                    }
                                    y -= Fpt.h + baselineskip;
                                }
                                if slicing == 0 || current_slice > slice {
                                    if current_file_linenum != last_wrapped_line as u32 {
                                        if do_print != 0 {
                                            num_truncated_lines += 1;
                                            num_truncated_lines;
                                        }
                                        last_wrapped_line = current_file_linenum as i32;
                                    }
                                }
                            }
                            current_linenum = current_linenum.wrapping_add(1);
                            current_linenum;
                            if current_linenum >= lines_per_page
                                || y < (d_footer_h + d_output_y_margin) as libc::c_double
                            {
                                break;
                            }
                            x = ((col * d_output_w) as libc::c_float
                                / num_columns as libc::c_float
                                + d_output_x_margin as libc::c_float) as libc::c_double
                                + line_indent;
                            line_column = 0 as i32 as u32;
                        }
                        7 => {
                            if token.flags & 0x20 as i32 as u32 != 0 {
                                token.new_y = ly;
                            } else {
                                token.new_y = y;
                            }
                            token.new_y += token.u.epsf.y - token.u.epsf.h;
                            if token.flags & 0x10 as i32 as u32 != 0 {
                                token.new_x = lx;
                            } else {
                                token.new_x = x;
                            }
                            token.new_x += token.u.epsf.x;
                            if token.flags & 0x1 as i32 as u32 != 0 {
                                token.new_x = lx
                                    + (linewidth - token.u.epsf.w) / 2 as i32 as libc::c_double;
                            }
                            if token.flags & 0x2 as i32 as u32 != 0 {
                                token.new_x = lx + (linewidth - token.u.epsf.w);
                            }
                            if token.flags & 0x8 as i32 as u32 == 0 as i32 as u32
                                && token.new_y
                                    < (d_footer_h + d_output_y_margin) as libc::c_double
                            {
                                if current_linenum == 0 as i32 as u32 {
                                    if quiet == 0 && verbose >= 0 as i32 {
                                        fprintf(
                                            stderr,
                                            dcgettext(
                                                0 as *const i8,
                                                b"EPS file \"%s\" is too large for page\n\0" as *const u8
                                                    as *const i8,
                                                5 as i32,
                                            ),
                                            (token.u.epsf.filename).as_mut_ptr(),
                                        );
                                    }
                                } else {
                                    reuse_last_token = 1 as i32;
                                    break;
                                }
                            }
                            if slicing == 0 as i32 || current_slice == slice {
                                paste_epsf(&mut token);
                            }
                            if token.flags & 0x8 as i32 as u32 == 0 {
                                y = token.new_y;
                            }
                            if token.flags & 0x4 as i32 as u32 == 0 {
                                x = token.new_x + token.u.epsf.w;
                            }
                            if y < (d_footer_h + d_output_y_margin) as libc::c_double {
                                break;
                            }
                        }
                        11 => {
                            if line_column == 0 as i32 as u32 {
                                let mut newh: libc::c_double = 0.;
                                if token.u.font.name[0 as i32 as usize] as i32
                                    == '\0' as i32
                                {
                                    newh = default_Fpt.h;
                                } else {
                                    newh = token.u.font.size.h;
                                }
                                if newh != Fpt.h {
                                    y -= newh - Fpt.h;
                                }
                            }
                            if quiet == 0 && verbose >= 2 as i32 {
                                fprintf(stderr, b"^@font=\0" as *const u8 as *const i8);
                            }
                            if token.u.font.name[0 as i32 as usize] as i32 == '\0' as i32
                            {
                                Fpt.w = default_Fpt.w;
                                Fpt.h = default_Fpt.h;
                                Fname = default_Fname;
                                if cofp.is_null() {
                                    cofp = ofp;
                                }
                                if do_print != 0 {
                                    fprintf(
                                        cofp,
                                        b"/F-gs-font %g %g SF\n\0" as *const u8 as *const i8,
                                        Fpt.w,
                                        Fpt.h,
                                    );
                                }
                                user_fontp = 0 as i32;
                            } else {
                                strhash_put(
                                    res_fonts,
                                    (token.u.font.name).as_mut_ptr(),
                                    (strlen((token.u.font.name).as_mut_ptr()))
                                        .wrapping_add(1 as i32 as u64) as i32,
                                    0 as *mut libc::c_void,
                                    0 as *mut *mut libc::c_void,
                                );
                                if cofp.is_null() {
                                    cofp = ofp;
                                }
                                if do_print != 0 {
                                    fprintf(
                                        cofp,
                                        b"/%s %g %g SUF\n\0" as *const u8 as *const i8,
                                        (token.u.font.name).as_mut_ptr(),
                                        token.u.font.size.w,
                                        token.u.font.size.h,
                                    );
                                }
                                strcpy(
                                    user_font_name.as_mut_ptr(),
                                    (token.u.font.name).as_mut_ptr(),
                                );
                                user_font_pt.w = token.u.font.size.w;
                                user_font_pt.h = token.u.font.size.h;
                                user_fontp = 1 as i32;
                                Fpt.w = user_font_pt.w;
                                Fpt.h = user_font_pt.h;
                                Fname = user_font_name.as_mut_ptr();
                            }
                            if quiet == 0 && verbose >= 2 as i32 {
                                fprintf(
                                    stderr,
                                    b"%s %g/%gpt\n\0" as *const u8 as *const i8,
                                    Fname,
                                    Fpt.w,
                                    Fpt.h,
                                );
                            }
                            read_font_info();
                            if y < (d_footer_h + d_output_y_margin) as libc::c_double {
                                break;
                            }
                        }
                        12 => {
                            if quiet == 0 && verbose >= 2 as i32 {
                                fprintf(
                                    stderr,
                                    b"^@color{%f %f %f}\n\0" as *const u8 as *const i8,
                                    token.u.color.r as libc::c_double,
                                    token.u.color.g as libc::c_double,
                                    token.u.color.b as libc::c_double,
                                );
                            }
                            if token.u.color.r == token.u.color.g
                                && token.u.color.g == token.u.color.b
                                && token.u.color.b as libc::c_double == 0.0f64
                            {
                                if cofp.is_null() {
                                    cofp = ofp;
                                }
                                if do_print != 0 {
                                    fprintf(cofp, b"0 setgray\n\0" as *const u8 as *const i8);
                                }
                                user_colorp = 0 as i32;
                            } else {
                                if cofp.is_null() {
                                    cofp = ofp;
                                }
                                if do_print != 0 {
                                    fprintf(
                                        cofp,
                                        b"%g %g %g setrgbcolor\n\0" as *const u8 as *const i8,
                                        token.u.color.r as libc::c_double,
                                        token.u.color.g as libc::c_double,
                                        token.u.color.b as libc::c_double,
                                    );
                                }
                                user_color.r = token.u.color.r;
                                user_color.g = token.u.color.g;
                                user_color.b = token.u.color.b;
                                user_colorp = 1 as i32;
                            }
                        }
                        8 => {
                            strcpy(fname.as_mut_ptr(), (token.u.filename).as_mut_ptr());
                        }
                        9 => {
                            current_pagenum = (token.u.i - 1 as i32) as u32;
                        }
                        10 => {
                            if current_linenum >= token.u.i as u32 {
                                break 's_120;
                            }
                        }
                        13 => {
                            if cofp.is_null() {
                                cofp = ofp;
                            }
                            if do_print != 0 {
                                fprintf(
                                    cofp,
                                    b"%g %g M\n%s\n\0" as *const u8 as *const i8,
                                    x,
                                    y,
                                    token.u.str_0,
                                );
                            }
                            xfree(token.u.str_0 as *mut libc::c_void);
                        }
                        0 | _ => {
                            fprintf(
                                stderr,
                                b"%s: \0" as *const u8 as *const i8,
                                program,
                            );
                            fprintf(
                                stderr,
                                b"process_file(): got illegal token %d\0" as *const u8
                                    as *const i8,
                                token.type_0 as u32,
                            );
                            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                            fflush(stderr);
                            exit(1 as i32);
                        }
                    }
                }
            }
            col += 1;
            col;
        }
        if page_clear == 0 {
            dump_ps_page_trailer();
        }
    }
    do_print = 1 as i32;
    undivert();
    if toc != 0 {
        let mut cp: *mut i8 = 0 as *mut i8;
        cp = format_user_string(
            b"TOC\0" as *const u8 as *const i8 as *mut i8,
            toc_fmt_string,
        );
        fprintf(toc_fp, b"%s\n\0" as *const u8 as *const i8, cp);
        xfree(cp as *mut libc::c_void);
    }
}
static mut escapes: [C2RustUnnamed_3; 12] = [
    {
        let mut init = C2RustUnnamed_3 {
            name: b"comment\0" as *const u8 as *const i8 as *mut i8,
            escape: SpecialEscape::ESC_COMMENT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"epsf\0" as *const u8 as *const i8 as *mut i8,
            escape: SpecialEscape::ESC_EPSF,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"font\0" as *const u8 as *const i8 as *mut i8,
            escape: SpecialEscape::ESC_FONT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"color\0" as *const u8 as *const i8 as *mut i8,
            escape: SpecialEscape::ESC_COLOR,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"newpage\0" as *const u8 as *const i8 as *mut i8,
            escape: SpecialEscape::ESC_NEWPAGE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"ps\0" as *const u8 as *const i8 as *mut i8,
            escape: SpecialEscape::ESC_PS,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"setfilename\0" as *const u8 as *const i8 as *mut i8,
            escape: SpecialEscape::ESC_SETFILENAME,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"setpagenumber\0" as *const u8 as *const i8 as *mut i8,
            escape: SpecialEscape::ESC_SETPAGENUMBER,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"shade\0" as *const u8 as *const i8 as *mut i8,
            escape: SpecialEscape::ESC_SHADE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"bggray\0" as *const u8 as *const i8 as *mut i8,
            escape: SpecialEscape::ESC_BGGRAY,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"escape\0" as *const u8 as *const i8 as *mut i8,
            escape: SpecialEscape::ESC_ESCAPE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: 0 as *const i8 as *mut i8,
            escape: SpecialEscape::ESC_COMMENT,
        };
        init
    },
];
unsafe extern "C" fn read_special_escape(
    mut is: *mut InputStream,
    mut token: *mut Token,
) {
    let mut escname: [i8; 256] = [0; 256];
    let mut buf: [i8; 4096] = [0; 4096];
    let mut i: i32 = 0;
    let mut e: i32 = 0;
    let mut ch: i32 = 0;
    i = 0 as i32;
    while (i as u64)
        < (::core::mem::size_of::<[i8; 256]>() as u64).wrapping_sub(1 as i32 as u64)
        && {
            ch = is_getc(is);
            ch != -(1 as i32)
        }
    {
        if *(*__ctype_b_loc()).offset(ch as isize) as i32
            & C2RustUnnamed::_ISalnum as i32 as libc::c_ushort as i32 == 0
        {
            is_ungetc(ch, is);
            break;
        } else {
            escname[i as usize] = ch as i8;
            i += 1;
            i;
        }
    }
    escname[i as usize] = '\0' as i32 as i8;
    e = 0 as i32;
    while !(escapes[e as usize].name).is_null() {
        if strcmp(escname.as_mut_ptr(), escapes[e as usize].name) == 0 as i32 {
            break;
        }
        e += 1;
        e;
    }
    if (escapes[e as usize].name).is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"unknown special escape: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            escname.as_mut_ptr(),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fflush(stderr);
        exit(1 as i32);
    }
    if escapes[e as usize].escape as u32 == SpecialEscape::ESC_EPSF as i32 as u32 {
        let mut i_0: i32 = 0;
        let mut pw: i32 = 0;
        let mut ph: i32 = 0;
        let mut scale: libc::c_double = 0.;
        (*token).flags = 0 as i32 as u32;
        (*token).u.epsf.x = 0.0f64;
        (*token).u.epsf.y = 0.0f64;
        (*token).u.epsf.h = 0.0f64;
        (*token).u.epsf.pipe = 0 as i32;
        ch = is_getc(is);
        if ch == '[' as i32 {
            loop {
                ch = is_getc(is);
                if !(ch != -(1 as i32) && ch != ']' as i32) {
                    break;
                }
                match ch {
                    99 => {
                        (*token).flags &= !(0x3 as i32) as u32;
                        (*token).flags |= 0x1 as i32 as u32;
                    }
                    110 => {
                        ch = is_getc(is);
                        match ch {
                            120 => {
                                (*token).flags |= 0x4 as i32 as u32;
                            }
                            121 => {
                                (*token).flags |= 0x8 as i32 as u32;
                            }
                            _ => {
                                is_ungetc(ch, is);
                                (*token).flags |= 0x4 as i32 as u32;
                                (*token).flags |= 0x8 as i32 as u32;
                            }
                        }
                    }
                    114 => {
                        (*token).flags &= !(0x3 as i32) as u32;
                        (*token).flags |= 0x2 as i32 as u32;
                    }
                    115 => {
                        ch = is_getc(is);
                        match ch {
                            120 => {
                                (*token).flags |= 0x40 as i32 as u32;
                                (*token).u.epsf.xscale = read_float(is, 0 as i32, 1 as i32);
                            }
                            121 => {
                                (*token).flags |= 0x80 as i32 as u32;
                                (*token).u.epsf.yscale = read_float(is, 0 as i32, 0 as i32);
                            }
                            _ => {
                                is_ungetc(ch, is);
                                (*token).flags |= 0x40 as i32 as u32;
                                (*token).flags |= 0x80 as i32 as u32;
                                (*token).u.epsf.yscale = read_float(is, 0 as i32, 1 as i32);
                                (*token).u.epsf.xscale = (*token).u.epsf.yscale;
                            }
                        }
                    }
                    120 => {
                        (*token).u.epsf.x = read_float(is, 1 as i32, 1 as i32);
                        ch = is_getc(is);
                        match ch {
                            97 => {
                                (*token).flags |= 0x10 as i32 as u32;
                            }
                            _ => {
                                is_ungetc(ch, is);
                            }
                        }
                    }
                    121 => {
                        (*token).u.epsf.y = -read_float(is, 1 as i32, 0 as i32);
                        ch = is_getc(is);
                        match ch {
                            97 => {
                                (*token).flags |= 0x20 as i32 as u32;
                            }
                            _ => {
                                is_ungetc(ch, is);
                            }
                        }
                    }
                    104 => {
                        (*token).u.epsf.h = read_float(is, 1 as i32, 0 as i32);
                    }
                    32 | 9 => {}
                    _ => {
                        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"illegal option %c for ^@epsf escape\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            ch,
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                        fflush(stderr);
                        exit(1 as i32);
                    }
                }
            }
            if ch != ']' as i32 {
                fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"malformed ^@epsf escape: no ']' after options\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                fflush(stderr);
                exit(1 as i32);
            }
            ch = is_getc(is);
        }
        if ch == '{' as i32 {
            i_0 = 0 as i32;
            loop {
                ch = is_getc(is);
                if !(ch != -(1 as i32) && ch != '}' as i32) {
                    break;
                }
                (*token).u.epsf.filename[i_0 as usize] = ch as i8;
                if (i_0 + 1 as i32) as u64 >= ::core::mem::size_of::<[i8; 512]>() as u64
                {
                    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"too long file name for ^@epsf escape:\n%.*s\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        i_0,
                        ((*token).u.epsf.filename).as_mut_ptr(),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                    fflush(stderr);
                    exit(1 as i32);
                }
                i_0 += 1;
                i_0;
            }
            if ch == -(1 as i32) {
                fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"unexpected EOF while scanning ^@epsf escape\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                fflush(stderr);
                exit(1 as i32);
            }
            (*token).u.epsf.filename[i_0 as usize] = '\0' as i32 as i8;
            (*token).type_0 = TokenType::tEPSF;
        } else {
            fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"malformed ^@epsf escape: no '{' found\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
            fflush(stderr);
            exit(1 as i32);
        }
        if recognize_eps_file(token) == 0 {
            (*token).type_0 = TokenType::tNONE;
        } else {
            (*token).u.epsf.y += Fpt.h + baselineskip - 1 as i32 as libc::c_double;
            if (*token).u.epsf.h != 0.0f64 {
                (*token).u.epsf.h -= 1.0f64;
            }
            pw = (*token).u.epsf.urx - (*token).u.epsf.llx;
            ph = (*token).u.epsf.ury - (*token).u.epsf.lly;
            if (*token).u.epsf.h == 0.0f64 {
                scale = 1.0f64;
            } else {
                scale = (*token).u.epsf.h / ph as libc::c_double;
            }
            if (*token).flags & 0x40 as i32 as u32 == 0 as i32 as u32 {
                (*token).u.epsf.xscale = scale;
            }
            if (*token).flags & 0x80 as i32 as u32 == 0 as i32 as u32 {
                (*token).u.epsf.yscale = scale;
            }
            pw = (pw as libc::c_double * (*token).u.epsf.xscale) as i32;
            ph = (ph as libc::c_double * (*token).u.epsf.yscale) as i32;
            (*token).u.epsf.w = pw as libc::c_double;
            (*token).u.epsf.h = ph as libc::c_double;
        }
    } else if escapes[e as usize].escape as u32
        == SpecialEscape::ESC_COMMENT as i32 as u32
    {
        loop {
            ch = is_getc(is);
            if !(ch != -(1 as i32) && ch != nl) {
                break;
            }
        }
        (*token).type_0 = TokenType::tNONE;
    } else {
        let mut cp: *mut i8 = 0 as *mut i8;
        let mut parenlevel: i32 = 0;
        ch = is_getc(is);
        if ch != '{' as i32 {
            fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"malformed %s escape: no '{' found\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                escapes[e as usize].name,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
            fflush(stderr);
            exit(1 as i32);
        }
        parenlevel = 0 as i32;
        i = 0 as i32;
        loop {
            ch = is_getc(is);
            if !(ch != -(1 as i32) && (parenlevel > 0 as i32 || ch != '}' as i32)) {
                break;
            }
            if ch == '{' as i32 {
                parenlevel += 1;
                parenlevel;
            } else if ch == '}' as i32 {
                parenlevel -= 1;
                parenlevel;
            }
            buf[i as usize] = ch as i8;
            if (i + 1 as i32) as u64 >= ::core::mem::size_of::<[i8; 4096]>() as u64 {
                fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"too long argument for %s escape:\n%.*s\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    escapes[i as usize].name,
                    i,
                    buf.as_mut_ptr(),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                fflush(stderr);
                exit(1 as i32);
            }
            i += 1;
            i;
        }
        buf[i as usize] = '\0' as i32 as i8;
        match escapes[e as usize].escape as u32 {
            2 => {
                strcpy(((*token).u.font.name).as_mut_ptr(), buf.as_mut_ptr());
                if strcmp(
                    ((*token).u.font.name).as_mut_ptr(),
                    b"default\0" as *const u8 as *const i8,
                ) == 0 as i32
                {
                    (*token).u.font.name[0 as i32 as usize] = '\0' as i32 as i8;
                } else {
                    if parse_font_spec(
                        ((*token).u.font.name).as_mut_ptr(),
                        &mut cp,
                        &mut (*token).u.font.size,
                    ) == 0
                    {
                        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"malformed font spec for ^@font escape: %s\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            ((*token).u.font.name).as_mut_ptr(),
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                        fflush(stderr);
                        exit(1 as i32);
                    }
                    strcpy(((*token).u.font.name).as_mut_ptr(), cp);
                    xfree(cp as *mut libc::c_void);
                }
                (*token).type_0 = TokenType::tFONT;
            }
            3 => {
                if strcmp(buf.as_mut_ptr(), b"default\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*token).u.color.r = 0 as i32 as libc::c_float;
                    (*token).u.color.g = 0 as i32 as libc::c_float;
                    (*token).u.color.b = 0 as i32 as libc::c_float;
                } else {
                    let mut got: i32 = 0;
                    got = sscanf(
                        buf.as_mut_ptr(),
                        b"%g %g %g\0" as *const u8 as *const i8,
                        &mut (*token).u.color.r as *mut libc::c_float,
                        &mut (*token).u.color.g as *mut libc::c_float,
                        &mut (*token).u.color.b as *mut libc::c_float,
                    );
                    match got {
                        0 | 2 => {
                            fprintf(
                                stderr,
                                b"%s: \0" as *const u8 as *const i8,
                                program,
                            );
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const i8,
                                    b"malformed color spec for ^@color escape: %s\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                buf.as_mut_ptr(),
                            );
                            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                            fflush(stderr);
                            exit(1 as i32);
                        }
                        1 => {
                            (*token).u.color.b = (*token).u.color.r;
                            (*token).u.color.g = (*token).u.color.b;
                        }
                        _ => {}
                    }
                }
                (*token).type_0 = TokenType::tCOLOR;
            }
            7 => {
                line_highlight_gray = atof(buf.as_mut_ptr());
                if line_highlight_gray < 0.0f64 || line_highlight_gray > 1.0f64 {
                    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"invalid value for ^@shade escape: %s\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        buf.as_mut_ptr(),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                    fflush(stderr);
                    exit(1 as i32);
                }
                (*token).type_0 = TokenType::tNONE;
            }
            8 => {
                bggray = atof(buf.as_mut_ptr());
                if bggray < 0.0f64 || bggray > 1.0f64 {
                    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"invalid value for ^@bggray escape: %s\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        buf.as_mut_ptr(),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                    fflush(stderr);
                    exit(1 as i32);
                }
                (*token).type_0 = TokenType::tNONE;
            }
            9 => {
                if strcmp(buf.as_mut_ptr(), b"default\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    escape_char = default_escape_char;
                } else {
                    escape_char = atoi(buf.as_mut_ptr());
                }
                (*token).type_0 = TokenType::tNONE;
            }
            5 => {
                strcpy(((*token).u.filename).as_mut_ptr(), buf.as_mut_ptr());
                (*token).type_0 = TokenType::tSETFILENAME;
            }
            6 => {
                (*token).u.i = atoi(buf.as_mut_ptr());
                (*token).type_0 = TokenType::tSETPAGENUMBER;
            }
            4 => {
                if i == 0 as i32 {
                    (*token).u.i = 1 as i32;
                } else {
                    (*token).u.i = atoi(buf.as_mut_ptr());
                }
                (*token).type_0 = TokenType::tNEWPAGE;
            }
            10 => {
                (*token).u.str_0 = xstrdup(buf.as_mut_ptr());
                (*token).type_0 = TokenType::tPS;
            }
            _ => {
                abort();
            }
        }
    };
}
unsafe extern "C" fn get_next_token(
    mut is: *mut InputStream,
    mut linestart: libc::c_double,
    mut linepos: libc::c_double,
    mut col: u32,
    mut linew: libc::c_double,
    mut token: *mut Token,
) {
    static mut buffer: *mut u8 = 0 as *const u8 as *mut u8;
    static mut buflen: u32 = 0 as i32 as u32;
    let mut bufpos: u32 = 0 as i32 as u32;
    let mut ch: i32 = 0 as i32;
    let mut done: i32 = 0 as i32;
    let mut i: i32 = 0;
    static mut pending_token: i32 = TokenType::tNONE as i32;
    let mut original_col: u32 = col;
    if pending_token != TokenType::tNONE as i32 {
        (*token).type_0 = TokenType::from_libc_c_uint(pending_token as u32);
        pending_token = TokenType::tNONE as i32;
        return;
    }
    while done == 0 {
        ch = is_getc(is);
        match ch {
            -1 => {
                if bufpos == 0 as i32 as u32 {
                    (*token).type_0 = TokenType::tEOF;
                    return;
                }
                done = 1 as i32;
            }
            13 | 10 => {
                if ch == nl {
                    if bufpos == 0 as i32 as u32 {
                        (*token).type_0 = TokenType::tNEWLINE;
                        return;
                    } else {
                        is_ungetc(ch, is);
                        done = 1 as i32;
                    }
                } else if bufpos == 0 as i32 as u32 {
                    (*token).type_0 = TokenType::tCARRIAGE_RETURN;
                    return;
                } else {
                    is_ungetc(ch, is);
                    done = 1 as i32;
                }
            }
            9 => {
                if font_is_fixed != 0 {
                    i = (tabsize as u32).wrapping_sub(col.wrapping_rem(tabsize as u32))
                        as i32;
                    while i > 0 as i32 {
                        if linepos
                            + *font_widths.as_mut_ptr().offset(' ' as i32 as u8 as isize)
                            < linew || col == 0 as i32 as u32
                        {
                            if bufpos >= buflen {
                                buflen = buflen.wrapping_add(4096 as i32 as u32);
                                buffer = xrealloc(
                                    buffer as *mut libc::c_void,
                                    buflen as size_t,
                                ) as *mut u8;
                            }
                            let fresh0 = bufpos;
                            bufpos = bufpos.wrapping_add(1);
                            *buffer.offset(fresh0 as isize) = ' ' as i32 as u8;
                            linepos
                                += *font_widths
                                    .as_mut_ptr()
                                    .offset(' ' as i32 as u8 as isize);
                            col = col.wrapping_add(1);
                            col;
                            i -= 1;
                            i;
                        } else {
                            done = 2 as i32;
                            break;
                        }
                    }
                } else {
                    let mut grid: libc::c_double = tabsize as libc::c_double
                        * *font_widths.as_mut_ptr().offset(' ' as i32 as u8 as isize);
                    col = col.wrapping_add(1);
                    col;
                    linepos = (((linepos - linestart) / grid) as i32 + 1 as i32)
                        as libc::c_double * grid + linestart;
                    if linepos >= linew {
                        done = 2 as i32;
                    } else {
                        done = 1 as i32;
                    }
                }
            }
            12 => {
                if bufpos == 0 as i32 as u32 {
                    if interpret_formfeed != 0 {
                        (*token).type_0 = TokenType::tFORMFEED;
                    } else {
                        (*token).type_0 = TokenType::tNEWLINE;
                    }
                    return;
                } else {
                    is_ungetc(ch, is);
                    done = 1 as i32;
                }
            }
            _ => {
                if special_escapes != 0 && ch == escape_char {
                    if bufpos == 0 as i32 as u32 {
                        read_special_escape(is, token);
                        if (*token).type_0 as u32 != TokenType::tNONE as i32 as u32 {
                            return;
                        }
                    } else {
                        is_ungetc(ch, is);
                        done = 1 as i32;
                    }
                } else if ch == bs {
                    if bufpos == 0 as i32 as u32
                        || !(*font_ctype
                            .as_mut_ptr()
                            .offset(
                                *buffer
                                    .offset(bufpos.wrapping_sub(1 as i32 as u32) as isize)
                                    as isize,
                            ) as i32 == '*' as i32)
                    {
                        linepos
                            -= *font_widths
                                .as_mut_ptr()
                                .offset('m' as i32 as u8 as isize);
                    } else {
                        linepos
                            -= *font_widths
                                .as_mut_ptr()
                                .offset(
                                    *buffer
                                        .offset(bufpos.wrapping_sub(1 as i32 as u32) as isize)
                                        as isize,
                                );
                    }
                    done = 1 as i32;
                } else if *font_ctype.as_mut_ptr().offset(ch as u8 as isize) as i32
                    == '*' as i32
                {
                    if linepos + *font_widths.as_mut_ptr().offset(ch as u8 as isize)
                        < linew || col == 0 as i32 as u32
                    {
                        if ch < 0o40 as i32 || clean_7bit != 0 && ch >= 0o200 as i32 {
                            let mut buf: [i8; 10] = [0; 10];
                            sprintf(
                                buf.as_mut_ptr(),
                                b"\\%03o\0" as *const u8 as *const i8,
                                ch,
                            );
                            i = 0 as i32;
                            while buf[i as usize] != 0 {
                                if bufpos >= buflen {
                                    buflen = buflen.wrapping_add(4096 as i32 as u32);
                                    buffer = xrealloc(
                                        buffer as *mut libc::c_void,
                                        buflen as size_t,
                                    ) as *mut u8;
                                }
                                let fresh1 = bufpos;
                                bufpos = bufpos.wrapping_add(1);
                                *buffer.offset(fresh1 as isize) = buf[i as usize] as u8;
                                i += 1;
                                i;
                            }
                            linepos
                                += *font_widths.as_mut_ptr().offset(ch as u8 as isize);
                            col = col.wrapping_add(1);
                            col;
                        } else if ch == '(' as i32 || ch == ')' as i32
                            || ch == '\\' as i32
                        {
                            if bufpos >= buflen {
                                buflen = buflen.wrapping_add(4096 as i32 as u32);
                                buffer = xrealloc(
                                    buffer as *mut libc::c_void,
                                    buflen as size_t,
                                ) as *mut u8;
                            }
                            let fresh2 = bufpos;
                            bufpos = bufpos.wrapping_add(1);
                            *buffer.offset(fresh2 as isize) = '\\' as i32 as u8;
                            if bufpos >= buflen {
                                buflen = buflen.wrapping_add(4096 as i32 as u32);
                                buffer = xrealloc(
                                    buffer as *mut libc::c_void,
                                    buflen as size_t,
                                ) as *mut u8;
                            }
                            let fresh3 = bufpos;
                            bufpos = bufpos.wrapping_add(1);
                            *buffer.offset(fresh3 as isize) = ch as u8;
                            linepos
                                += *font_widths.as_mut_ptr().offset(ch as u8 as isize);
                            col = col.wrapping_add(1);
                            col;
                        } else {
                            if bufpos >= buflen {
                                buflen = buflen.wrapping_add(4096 as i32 as u32);
                                buffer = xrealloc(
                                    buffer as *mut libc::c_void,
                                    buflen as size_t,
                                ) as *mut u8;
                            }
                            let fresh4 = bufpos;
                            bufpos = bufpos.wrapping_add(1);
                            *buffer.offset(fresh4 as isize) = ch as u8;
                            linepos
                                += *font_widths.as_mut_ptr().offset(ch as u8 as isize);
                            col = col.wrapping_add(1);
                            col;
                        }
                    } else {
                        is_ungetc(ch, is);
                        done = 2 as i32;
                    }
                } else if *font_ctype.as_mut_ptr().offset(ch as u8 as isize) as i32
                    != ' ' as i32
                {
                    if linepos
                        + *font_widths.as_mut_ptr().offset('?' as i32 as u8 as isize)
                        < linew || col == 0 as i32 as u32
                    {
                        if bufpos >= buflen {
                            buflen = buflen.wrapping_add(4096 as i32 as u32);
                            buffer = xrealloc(
                                buffer as *mut libc::c_void,
                                buflen as size_t,
                            ) as *mut u8;
                        }
                        let fresh5 = bufpos;
                        bufpos = bufpos.wrapping_add(1);
                        *buffer.offset(fresh5 as isize) = '?' as i32 as u8;
                        linepos
                            += *font_widths
                                .as_mut_ptr()
                                .offset('?' as i32 as u8 as isize);
                        col = col.wrapping_add(1);
                        col;
                        let ref mut fresh6 = *missing_chars
                            .as_mut_ptr()
                            .offset(ch as isize);
                        let fresh7 = *fresh6;
                        *fresh6 = *fresh6 + 1;
                        if fresh7 == 0 as i32 {
                            num_missing_chars += 1;
                            num_missing_chars;
                        }
                    } else {
                        is_ungetc(ch, is);
                        done = 2 as i32;
                    }
                } else {
                    let mut buf_0: [i8; 20] = [0; 20];
                    let mut len: libc::c_double = 0.0f64;
                    let ref mut fresh8 = *non_printable_chars
                        .as_mut_ptr()
                        .offset(ch as isize);
                    let fresh9 = *fresh8;
                    *fresh8 = *fresh8 + 1;
                    if fresh9 == 0 as i32 {
                        num_non_printable_chars += 1;
                        num_non_printable_chars;
                    }
                    let mut current_block_145: u64;
                    match non_printable_format as u32 {
                        0 => {
                            strcpy(buf_0.as_mut_ptr(), b" \0" as *const u8 as *const i8);
                            current_block_145 = 7761909515536616543;
                        }
                        1 => {
                            strcpy(buf_0.as_mut_ptr(), b"?\0" as *const u8 as *const i8);
                            current_block_145 = 7761909515536616543;
                        }
                        2 => {
                            if ch < 0x20 as i32 {
                                buf_0[0 as i32 as usize] = '^' as i32 as i8;
                                buf_0[1 as i32 as usize] = ('@' as i32 + ch) as i8;
                                buf_0[2 as i32 as usize] = '\0' as i32 as i8;
                                current_block_145 = 7761909515536616543;
                            } else {
                                current_block_145 = 9112997927631832806;
                            }
                        }
                        3 => {
                            current_block_145 = 9112997927631832806;
                        }
                        _ => {
                            current_block_145 = 7761909515536616543;
                        }
                    }
                    match current_block_145 {
                        9112997927631832806 => {
                            sprintf(
                                buf_0.as_mut_ptr(),
                                b"\\%03o\0" as *const u8 as *const i8,
                                ch,
                            );
                        }
                        _ => {}
                    }
                    i = 0 as i32;
                    while buf_0[i as usize] != 0 {
                        len
                            += *font_widths
                                .as_mut_ptr()
                                .offset(buf_0[i as usize] as u8 as isize);
                        i += 1;
                        i;
                    }
                    if linepos + len < linew || col == 0 as i32 as u32 {
                        i = 0 as i32;
                        while buf_0[i as usize] != 0 {
                            if buf_0[i as usize] as i32 == '\\' as i32 {
                                if bufpos >= buflen {
                                    buflen = buflen.wrapping_add(4096 as i32 as u32);
                                    buffer = xrealloc(
                                        buffer as *mut libc::c_void,
                                        buflen as size_t,
                                    ) as *mut u8;
                                }
                                let fresh10 = bufpos;
                                bufpos = bufpos.wrapping_add(1);
                                *buffer.offset(fresh10 as isize) = '\\' as i32 as u8;
                            }
                            if bufpos >= buflen {
                                buflen = buflen.wrapping_add(4096 as i32 as u32);
                                buffer = xrealloc(
                                    buffer as *mut libc::c_void,
                                    buflen as size_t,
                                ) as *mut u8;
                            }
                            let fresh11 = bufpos;
                            bufpos = bufpos.wrapping_add(1);
                            *buffer.offset(fresh11 as isize) = buf_0[i as usize] as u8;
                            linepos
                                += *font_widths
                                    .as_mut_ptr()
                                    .offset(buf_0[i as usize] as u8 as isize);
                            col = col.wrapping_add(1);
                            col;
                            i += 1;
                            i;
                        }
                    } else {
                        is_ungetc(ch, is);
                        done = 2 as i32;
                    }
                }
            }
        }
    }
    if done == 2 as i32 {
        ch = nl;
        if line_end as u32 == LineEndType::LE_TRUNCATE as i32 as u32 {
            loop {
                ch = is_getc(is);
                if !(ch != -(1 as i32) && ch != nl) {
                    break;
                }
            }
        } else if !(bufpos == 0 as i32 as u32)
            && line_end as u32 == LineEndType::LE_WORD_WRAP as i32 as u32
        {
            let mut w: i32 = 0;
            if *buffer.offset(bufpos.wrapping_sub(1 as i32 as u32) as isize) as i32
                == ' ' as i32
                || *buffer.offset(bufpos.wrapping_sub(1 as i32 as u32) as isize) as i32
                    == '\t' as i32
            {
                loop {
                    w = is_getc(is);
                    if !(w != -(1 as i32) && (w == ' ' as i32 || w == '\t' as i32)) {
                        break;
                    }
                }
                is_ungetc(w, is);
            } else {
                w = bufpos.wrapping_sub(1 as i32 as u32) as i32;
                while w >= 0 as i32
                    && !(*buffer.offset(w as isize) as i32 == ' ' as i32
                        || *buffer.offset(w as isize) as i32 == '\t' as i32)
                {
                    w -= 1;
                    w;
                }
                w += 1;
                w;
                if w > 0 as i32 || original_col > 0 as i32 as u32 {
                    loop {
                        bufpos = bufpos.wrapping_sub(1);
                        bufpos;
                        if bufpos > w as u32
                            && (*buffer.offset(bufpos as isize) as i32 == '(' as i32
                                || *buffer.offset(bufpos as isize) as i32 == ')' as i32
                                || *buffer.offset(bufpos as isize) as i32 == '\\' as i32)
                            && *buffer
                                .offset(bufpos.wrapping_sub(1 as i32 as u32) as isize)
                                as i32 == '\\' as i32
                        {
                            is_ungetc(*buffer.offset(bufpos as isize) as i32, is);
                            linepos
                                -= *font_widths
                                    .as_mut_ptr()
                                    .offset(*buffer.offset(bufpos as isize) as isize);
                            col = col.wrapping_sub(1);
                            col;
                            bufpos = bufpos.wrapping_sub(1);
                            bufpos;
                        } else {
                            let mut current_block_226: u64;
                            if bufpos.wrapping_sub(2 as i32 as u32) > w as u32
                                && ('0' as i32 <= *buffer.offset(bufpos as isize) as i32
                                    && *buffer.offset(bufpos as isize) as i32 <= '7' as i32)
                                && ('0' as i32
                                    <= *buffer
                                        .offset(bufpos.wrapping_sub(1 as i32 as u32) as isize)
                                        as i32
                                    && *buffer
                                        .offset(bufpos.wrapping_sub(1 as i32 as u32) as isize)
                                        as i32 <= '7' as i32)
                                && ('0' as i32
                                    <= *buffer
                                        .offset(bufpos.wrapping_sub(2 as i32 as u32) as isize)
                                        as i32
                                    && *buffer
                                        .offset(bufpos.wrapping_sub(2 as i32 as u32) as isize)
                                        as i32 <= '7' as i32)
                                && *buffer
                                    .offset(bufpos.wrapping_sub(3 as i32 as u32) as isize)
                                    as i32 == '\\' as i32
                            {
                                let mut ti: u32 = 0;
                                ti = w as u32;
                                while ti < bufpos.wrapping_sub(3 as i32 as u32) {
                                    if *buffer.offset(ti as isize) as i32 == '\\' as i32 {
                                        if '0' as i32
                                            <= *buffer.offset(ti.wrapping_add(1 as i32 as u32) as isize)
                                                as i32
                                            && *buffer.offset(ti.wrapping_add(1 as i32 as u32) as isize)
                                                as i32 <= '7' as i32
                                        {
                                            let mut tti: u32 = 0;
                                            tti = 0 as i32 as u32;
                                            while tti < 3 as i32 as u32
                                                && ('0' as i32
                                                    <= *buffer.offset(ti.wrapping_add(1 as i32 as u32) as isize)
                                                        as i32
                                                    && *buffer.offset(ti.wrapping_add(1 as i32 as u32) as isize)
                                                        as i32 <= '7' as i32)
                                            {
                                                tti = tti.wrapping_add(1);
                                                tti;
                                                ti = ti.wrapping_add(1);
                                                ti;
                                            }
                                        } else {
                                            ti = ti.wrapping_add(1);
                                            ti;
                                        }
                                    }
                                    ti = ti.wrapping_add(1);
                                    ti;
                                }
                                if ti == bufpos.wrapping_sub(3 as i32 as u32) {
                                    let mut tch: i32 = 0;
                                    tch = ((*buffer
                                        .offset(bufpos.wrapping_sub(2 as i32 as u32) as isize)
                                        as i32 - '0' as i32) << 6 as i32)
                                        + ((*buffer
                                            .offset(bufpos.wrapping_sub(1 as i32 as u32) as isize)
                                            as i32 - '0' as i32) << 3 as i32)
                                        + (*buffer.offset(bufpos as isize) as i32 - '0' as i32);
                                    is_ungetc(tch, is);
                                    linepos
                                        -= *font_widths.as_mut_ptr().offset(tch as u8 as isize);
                                    col = col.wrapping_sub(1);
                                    col;
                                    bufpos = bufpos.wrapping_sub(3 as i32 as u32);
                                    current_block_226 = 13164310931121142693;
                                } else {
                                    current_block_226 = 9327940613784395629;
                                }
                            } else {
                                current_block_226 = 9327940613784395629;
                            }
                            match current_block_226 {
                                9327940613784395629 => {
                                    is_ungetc(*buffer.offset(bufpos as isize) as i32, is);
                                    linepos
                                        -= *font_widths
                                            .as_mut_ptr()
                                            .offset(*buffer.offset(bufpos as isize) as isize);
                                    col = col.wrapping_sub(1);
                                    col;
                                }
                                _ => {}
                            }
                        }
                        if !(bufpos > w as u32) {
                            break;
                        }
                    }
                }
            }
        }
        if ch == nl {
            if line_end as u32 == LineEndType::LE_TRUNCATE as i32 as u32 {
                if do_print != 0 {
                    num_truncated_lines += 1;
                    num_truncated_lines;
                }
                pending_token = TokenType::tNEWLINE as i32;
            } else {
                pending_token = TokenType::tWRAPPED_NEWLINE as i32;
            }
        } else {
            pending_token = TokenType::tEOF as i32;
        }
    }
    if bufpos >= buflen {
        buflen = buflen.wrapping_add(4096 as i32 as u32);
        buffer = xrealloc(buffer as *mut libc::c_void, buflen as size_t) as *mut u8;
    }
    let fresh12 = bufpos;
    bufpos = bufpos.wrapping_add(1);
    *buffer.offset(fresh12 as isize) = '\0' as i32 as u8;
    (*token).type_0 = TokenType::tSTRING;
    (*token).u.str_0 = buffer as *mut i8;
    (*token).new_x = linepos;
    (*token).new_col = col as i32;
}
unsafe extern "C" fn dump_ps_page_header(mut fname_0: *mut i8, mut empty: i32) {
    let mut buf: [i8; 512] = [0; 512];
    let mut ftail: *mut i8 = 0 as *mut i8;
    let mut got: i32 = 0;
    let mut i: i32 = 0;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut cp2: *mut i8 = 0 as *mut i8;
    let mut cstr: *mut i8 = b"%%\0" as *const u8 as *const i8 as *mut i8;
    let mut nup_subpage: u32 = 0;
    nup_subpage = ((total_pages - 1 as i32) as u32).wrapping_rem(nup);
    ftail = strrchr(fname_0, '/' as i32);
    if ftail.is_null() {
        buf[0 as i32 as usize] = '\0' as i32 as i8;
        ftail = fname_0;
    } else {
        ftail = ftail.offset(1);
        ftail;
        strncpy(buf.as_mut_ptr(), fname_0, ftail.offset_from(fname_0) as i64 as u64);
        buf[ftail.offset_from(fname_0) as i64 as usize] = '\0' as i32 as i8;
    }
    if nup > 1 as i32 as u32 {
        cstr = b"%\0" as *const u8 as *const i8 as *mut i8;
        if nup_subpage == 0 as i32 as u32 {
            match page_label as u32 {
                0 => {
                    if cofp.is_null() {
                        cofp = ofp;
                    }
                    if do_print != 0 {
                        fprintf(
                            cofp,
                            b"%%%%Page: (%d-%d) %d\n\0" as *const u8 as *const i8,
                            current_pagenum,
                            current_pagenum
                                .wrapping_add(nup)
                                .wrapping_sub(1 as i32 as u32),
                            (total_pages as u32)
                                .wrapping_div(nup)
                                .wrapping_add(1 as i32 as u32),
                        );
                    }
                }
                1 => {
                    if cofp.is_null() {
                        cofp = ofp;
                    }
                    if do_print != 0 {
                        fprintf(
                            cofp,
                            b"%%%%Page: (%s:%3d-%3d) %d\n\0" as *const u8 as *const i8,
                            ftail,
                            current_pagenum,
                            current_pagenum
                                .wrapping_add(nup)
                                .wrapping_sub(1 as i32 as u32),
                            (total_pages as u32)
                                .wrapping_div(nup)
                                .wrapping_add(1 as i32 as u32),
                        );
                    }
                }
                _ => {}
            }
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"%%%%BeginPageSetup\n_S\n\0" as *const u8 as *const i8);
            }
            if (total_pages as u32)
                .wrapping_div(nup)
                .wrapping_add(1 as i32 as u32)
                .wrapping_rem(2 as i32 as u32) == 0 as i32 as u32
            {
                handle_two_side_options();
            }
            if landscape != 0 {
                if nup_landscape != 0 {
                    if cofp.is_null() {
                        cofp = ofp;
                    }
                    if do_print != 0 {
                        fprintf(
                            cofp,
                            b"90 rotate\n%d %d translate\n\0" as *const u8 as *const i8,
                            (*media).lly,
                            -(*media).urx,
                        );
                    }
                } else {
                    if cofp.is_null() {
                        cofp = ofp;
                    }
                    if do_print != 0 {
                        fprintf(
                            cofp,
                            b"%d %d translate\n\0" as *const u8 as *const i8,
                            (*media).llx,
                            (*media).lly,
                        );
                    }
                }
            } else if nup_landscape != 0 {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(
                        cofp,
                        b"90 rotate\n%d %d translate\n\0" as *const u8 as *const i8,
                        (*media).lly,
                        -(*media).llx,
                    );
                }
            } else {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(
                        cofp,
                        b"%d %d translate\n\0" as *const u8 as *const i8,
                        (*media).llx,
                        (*media).ury,
                    );
                }
            }
        }
    }
    match page_label as u32 {
        0 => {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%sPage: (%d) %d\n\0" as *const u8 as *const i8,
                    cstr,
                    current_pagenum,
                    total_pages,
                );
            }
        }
        1 => {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%sPage: (%s:%3d) %d\n\0" as *const u8 as *const i8,
                    cstr,
                    ftail,
                    current_pagenum,
                    total_pages,
                );
            }
        }
        _ => {}
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%sBeginPageSetup\n_S\n\0" as *const u8 as *const i8, cstr);
    }
    if nup > 1 as i32 as u32 {
        let mut xm: i32 = 0;
        let mut ym: i32 = 0;
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%% N-up sub-page %d/%d\n\0" as *const u8 as *const i8,
                nup_subpage.wrapping_add(1 as i32 as u32),
                nup,
            );
        }
        if landscape != 0 {
            xm = nup_subpage.wrapping_div(nup_rows) as i32;
            ym = nup_subpage.wrapping_rem(nup_rows) as i32;
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%d %d translate\n\0" as *const u8 as *const i8,
                    (xm as u32).wrapping_mul(nup_width.wrapping_add(nup_xpad)),
                    (ym as u32).wrapping_mul(nup_height.wrapping_add(nup_ypad)),
                );
            }
        } else {
            xm = nup_subpage.wrapping_rem(nup_columns) as i32;
            ym = nup_subpage.wrapping_div(nup_columns) as i32;
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%d %d translate\n\0" as *const u8 as *const i8,
                    (xm as u32).wrapping_mul(nup_width.wrapping_add(nup_xpad)),
                    (ym as u32)
                        .wrapping_mul(nup_height.wrapping_add(nup_ypad))
                        .wrapping_add(nup_height)
                        .wrapping_neg(),
                );
            }
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"%g dup scale\n\0" as *const u8 as *const i8, nup_scale);
        }
        if landscape != 0 {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"90 rotate\n%d %d translate\n\0" as *const u8 as *const i8,
                    0 as i32,
                    -d_page_h,
                );
            }
        }
    } else {
        if total_pages % 2 as i32 == 0 as i32 {
            handle_two_side_options();
        }
        if landscape != 0 {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"90 rotate\n%d %d translate\n\0" as *const u8 as *const i8,
                    (*media).lly,
                    -(*media).urx,
                );
            }
        } else {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%d %d translate\n\0" as *const u8 as *const i8,
                    (*media).llx,
                    (*media).lly,
                );
            }
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/pagenum %d def\n\0" as *const u8 as *const i8, current_pagenum);
    }
    cp = escape_string(fname_0);
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/fname (%s) def\n\0" as *const u8 as *const i8, cp);
    }
    xfree(cp as *mut libc::c_void);
    cp = escape_string(buf.as_mut_ptr());
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/fdir (%s) def\n\0" as *const u8 as *const i8, cp);
    }
    xfree(cp as *mut libc::c_void);
    cp = escape_string(ftail);
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/ftail (%s) def\n\0" as *const u8 as *const i8, cp);
    }
    xfree(cp as *mut libc::c_void);
    if user_fontp != 0 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/%s %g %g SUF\n\0" as *const u8 as *const i8,
                Fname,
                Fpt.w,
                Fpt.h,
            );
        }
    }
    if count_key_value_set(user_strings) > 0 as i32 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"%% User defined strings:\n\0" as *const u8 as *const i8);
        }
        got = strhash_get_first(
            user_strings,
            &mut cp,
            &mut i,
            &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
        );
        while got != 0 {
            cp2 = format_user_string(
                b"%Format\0" as *const u8 as *const i8 as *mut i8,
                cp2,
            );
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"/%s (%s) def\n\0" as *const u8 as *const i8, cp, cp2);
            }
            xfree(cp2 as *mut libc::c_void);
            got = strhash_get_next(
                user_strings,
                &mut cp,
                &mut i,
                &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
            );
        }
    }
    if !page_header.is_null() {
        let mut h_left: *mut i8 = 0 as *mut i8;
        let mut h_center: *mut i8 = 0 as *mut i8;
        let mut h_right: *mut i8 = 0 as *mut i8;
        h_left = format_user_string(
            b"page header\0" as *const u8 as *const i8 as *mut i8,
            page_header,
        );
        h_center = strchr(h_left, '|' as i32);
        if !h_center.is_null() {
            *h_center = '\0' as i32 as i8;
            h_center = h_center.offset(1);
            h_center;
            h_right = strchr(h_center, '|' as i32);
            if !h_right.is_null() {
                *h_right = '\0' as i32 as i8;
                h_right = h_right.offset(1);
                h_right;
            }
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"/user_header_p true def\n\0" as *const u8 as *const i8);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_header_left_str (%s) def\n\0" as *const u8 as *const i8,
                h_left,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_header_center_str (%s) def\n\0" as *const u8 as *const i8,
                if !h_center.is_null() {
                    h_center
                } else {
                    b"\0" as *const u8 as *const i8
                },
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_header_right_str (%s) def\n\0" as *const u8 as *const i8,
                if !h_right.is_null() {
                    h_right
                } else {
                    b"\0" as *const u8 as *const i8
                },
            );
        }
        xfree(h_left as *mut libc::c_void);
    } else {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"/user_header_p false def\n\0" as *const u8 as *const i8);
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndPageSetup\n\0" as *const u8 as *const i8);
    }
    if empty == 0 {
        if highlight_bars != 0 {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%d %f %d %f highlight_bars\n\0" as *const u8 as *const i8,
                    highlight_bars,
                    Fpt.h + baselineskip,
                    d_output_y_margin,
                    highlight_bar_gray,
                );
            }
        }
        if !underlay.is_null() {
            if ul_position_p != 0 || ul_angle_p != 0 {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(cofp, b"user_underlay\n\0" as *const u8 as *const i8);
                }
            } else {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(cofp, b"underlay\n\0" as *const u8 as *const i8);
                }
            }
        }
        if num_columns > 1 as i32
            && (header as u32 == HeaderType::HDR_FANCY as i32 as u32 || borders != 0)
        {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"column_lines\n\0" as *const u8 as *const i8);
            }
        }
        if borders != 0 {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"column_borders\n\0" as *const u8 as *const i8);
            }
        }
        match header as u32 {
            1 | 2 => {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(cofp, b"do_header\n\0" as *const u8 as *const i8);
                }
            }
            0 | _ => {}
        }
    }
    if user_colorp != 0 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%g %g %g setrgbcolor\n\0" as *const u8 as *const i8,
                user_color.r as libc::c_double,
                user_color.g as libc::c_double,
                user_color.b as libc::c_double,
            );
        }
    }
}
unsafe extern "C" fn dump_ps_page_trailer() {
    let mut nup_subpage: u32 = ((total_pages - 1 as i32) as u32).wrapping_rem(nup);
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"_R\n\0" as *const u8 as *const i8);
    }
    if nup > 1 as i32 as u32 {
        if nup_subpage.wrapping_add(1 as i32 as u32) == nup {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"_R\nS\n\0" as *const u8 as *const i8);
            }
        }
    } else {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"S\n\0" as *const u8 as *const i8);
        }
    };
}
unsafe extern "C" fn dump_empty_page() {
    if nup > 1 as i32 as u32 {
        let mut nup_subpage: u32 = ((total_pages - 1 as i32) as u32).wrapping_rem(nup);
        if nup_subpage == 0 as i32 as u32 {
            dump_ps_page_header(b"\0" as *const u8 as *const i8 as *mut i8, 1 as i32);
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"_R\n\0" as *const u8 as *const i8);
            }
        } else {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%%Page: (-) %d\n\0" as *const u8 as *const i8,
                    total_pages,
                );
            }
        }
        if nup_subpage.wrapping_add(1 as i32 as u32) == nup {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"_R\nS\n\0" as *const u8 as *const i8);
            }
        }
    } else {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%%%%Page: (-) %d\nS\n\0" as *const u8 as *const i8,
                total_pages,
            );
        }
    };
}
unsafe extern "C" fn recognize_eps_file(mut token: *mut Token) -> i32 {
    let mut i: i32 = 0;
    let mut filename: [i8; 512] = [0; 512];
    let mut buf: [i8; 4096] = [0; 4096];
    let mut line: i32 = 0;
    let mut valid_epsf: i32 = 0;
    let mut llx: libc::c_float = 0.;
    let mut lly: libc::c_float = 0.;
    let mut urx: libc::c_float = 0.;
    let mut ury: libc::c_float = 0.;
    if quiet == 0 && verbose >= 2 as i32 {
        fprintf(
            stderr,
            b"^@epsf=\"%s\"\n\0" as *const u8 as *const i8,
            ((*token).u.epsf.filename).as_mut_ptr(),
        );
    }
    i = strlen(((*token).u.epsf.filename).as_mut_ptr()) as i32;
    if i > 0 as i32
        && (*token).u.epsf.filename[(i - 1 as i32) as usize] as i32 == '|' as i32
    {
        (*token).u.epsf.pipe = 1 as i32;
        (*token).u.epsf.filename[(i - 1 as i32) as usize] = '\0' as i32 as i8;
        (*token).u.epsf.fp = popen(
            ((*token).u.epsf.filename).as_mut_ptr(),
            b"r\0" as *const u8 as *const i8,
        );
        if ((*token).u.epsf.fp).is_null() {
            if quiet == 0 && verbose >= 0 as i32 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"epsf: couldn't open pipe to command \"%s\": %s\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    ((*token).u.epsf.filename).as_mut_ptr(),
                    strerror(*__errno_location()),
                );
            }
            return 0 as i32;
        }
    } else {
        tilde_subst(((*token).u.epsf.filename).as_mut_ptr(), filename.as_mut_ptr());
        (*token).u.epsf.fp = fopen(
            filename.as_mut_ptr(),
            b"rb\0" as *const u8 as *const i8,
        );
        if ((*token).u.epsf.fp).is_null() {
            if (*token).u.epsf.filename[0 as i32 as usize] as i32 != '/' as i32 {
                let mut ctx: FileLookupCtx = FileLookupCtx {
                    name: [0; 256],
                    suffix: [0; 256],
                    fullname: [0; 512],
                };
                strcpy((ctx.name).as_mut_ptr(), ((*token).u.epsf.filename).as_mut_ptr());
                strcpy((ctx.suffix).as_mut_ptr(), b"\0" as *const u8 as *const i8);
                if pathwalk(
                    libpath.as_mut_ptr(),
                    Some(
                        file_lookup
                            as unsafe extern "C" fn(*mut i8, *mut libc::c_void) -> i32,
                    ),
                    &mut ctx as *mut FileLookupCtx as *mut libc::c_void,
                ) != 0
                {
                    (*token).u.epsf.fp = fopen(
                        (ctx.fullname).as_mut_ptr(),
                        b"rb\0" as *const u8 as *const i8,
                    );
                }
            }
            if ((*token).u.epsf.fp).is_null() {
                if quiet == 0 && verbose >= 0 as i32 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"couldn't open EPS file \"%s\": %s\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        ((*token).u.epsf.filename).as_mut_ptr(),
                        strerror(*__errno_location()),
                    );
                }
                return 0 as i32;
            }
        }
    }
    line = 0 as i32;
    valid_epsf = 0 as i32;
    (*token).u.epsf.skipbuf = 0 as *mut i8;
    (*token).u.epsf.skipbuf_len = 0 as i32 as u32;
    (*token).u.epsf.skipbuf_pos = 0 as i32 as u32;
    while !(fgets(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 4096]>() as u64 as i32,
        (*token).u.epsf.fp,
    ))
        .is_null()
    {
        line += 1;
        line;
        i = strlen(buf.as_mut_ptr()) as i32;
        if (i as u32).wrapping_add((*token).u.epsf.skipbuf_pos)
            >= (*token).u.epsf.skipbuf_len
        {
            (*token).u.epsf.skipbuf_len = ((*token).u.epsf.skipbuf_len)
                .wrapping_add(8192 as i32 as u32);
            (*token).u.epsf.skipbuf = xrealloc(
                (*token).u.epsf.skipbuf as *mut libc::c_void,
                (*token).u.epsf.skipbuf_len as size_t,
            ) as *mut i8;
        }
        memcpy(
            ((*token).u.epsf.skipbuf).offset((*token).u.epsf.skipbuf_pos as isize)
                as *mut libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            i as u64,
        );
        (*token).u.epsf.skipbuf_pos = ((*token).u.epsf.skipbuf_pos)
            .wrapping_add(i as u32);
        if line == 1 as i32 {
            if buf[0 as i32 as usize] as i32 != '%' as i32
                || buf[1 as i32 as usize] as i32 != '!' as i32
            {
                if quiet == 0 && verbose >= 0 as i32 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"EPS file \"%s\" does not start with \"%%!\" magic\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        ((*token).u.epsf.filename).as_mut_ptr(),
                    );
                }
                break;
            }
        }
        if !(strncmp(
            buf.as_mut_ptr(),
            b"%%BoundingBox:\0" as *const u8 as *const i8,
            strlen(b"%%BoundingBox:\0" as *const u8 as *const i8),
        ) == 0 as i32)
        {
            continue;
        }
        i = sscanf(
            buf
                .as_mut_ptr()
                .offset(strlen(b"%%BoundingBox:\0" as *const u8 as *const i8) as isize),
            b"%f %f %f %f\0" as *const u8 as *const i8,
            &mut llx as *mut libc::c_float,
            &mut lly as *mut libc::c_float,
            &mut urx as *mut libc::c_float,
            &mut ury as *mut libc::c_float,
        );
        if i != 4 as i32 {
            i = strlen(b"%%BoundingBox:\0" as *const u8 as *const i8) as i32;
            while buf[i as usize] as i32 != 0
                && (buf[i as usize] as i32 == ' ' as i32
                    || buf[i as usize] as i32 == '\t' as i32)
            {
                i += 1;
                i;
            }
            if !(strncmp(
                buf.as_mut_ptr().offset(i as isize),
                b"(atend)\0" as *const u8 as *const i8,
                strlen(b"(atend)\0" as *const u8 as *const i8),
            ) != 0 as i32)
            {
                continue;
            }
            if quiet == 0 && verbose >= 0 as i32 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"EPS file \"%s\" contains malformed %%%%BoundingBox row:\n\"%.*s\"\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    ((*token).u.epsf.filename).as_mut_ptr(),
                    (strlen(buf.as_mut_ptr())).wrapping_sub(1 as i32 as u64),
                    buf.as_mut_ptr(),
                );
            }
            break;
        } else {
            (*token).u.epsf.llx = llx as i32;
            (*token).u.epsf.lly = lly as i32;
            (*token).u.epsf.urx = urx as i32;
            (*token).u.epsf.ury = ury as i32;
            valid_epsf = 1 as i32;
            break;
        }
    }
    if valid_epsf == 0 {
        if quiet == 0 && verbose >= 0 as i32 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"EPS file \"%s\" is not a valid EPS file\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                ((*token).u.epsf.filename).as_mut_ptr(),
            );
        }
        if (*token).u.epsf.pipe != 0 {
            pclose((*token).u.epsf.fp);
        } else {
            fclose((*token).u.epsf.fp);
        }
        xfree((*token).u.epsf.skipbuf as *mut libc::c_void);
        return 0 as i32;
    }
    if quiet == 0 && verbose >= 2 as i32 {
        fprintf(
            stderr,
            b"BoundingBox: %d %d %d %d\n\0" as *const u8 as *const i8,
            (*token).u.epsf.llx,
            (*token).u.epsf.lly,
            (*token).u.epsf.urx,
            (*token).u.epsf.ury,
        );
    }
    return 1 as i32;
}
unsafe extern "C" fn paste_epsf(mut token: *mut Token) {
    let mut buf: [i8; 4096] = [0; 4096];
    let mut i: i32 = 0;
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"BeginEPSF\n\0" as *const u8 as *const i8);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%g %g translate\n\0" as *const u8 as *const i8,
            (*token).new_x,
            (*token).new_y,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%g %g scale\n\0" as *const u8 as *const i8,
            (*token).u.epsf.xscale,
            (*token).u.epsf.yscale,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%d %d translate\n\0" as *const u8 as *const i8,
            -(*token).u.epsf.llx,
            -(*token).u.epsf.lly,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%d %d %d %d Box clip newpath\n\0" as *const u8 as *const i8,
            (*token).u.epsf.llx - 1 as i32,
            (*token).u.epsf.lly - 1 as i32,
            (*token).u.epsf.urx - (*token).u.epsf.llx + 2 as i32,
            (*token).u.epsf.ury - (*token).u.epsf.lly + 2 as i32,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%BeginDocument: %s%s\n\0" as *const u8 as *const i8,
            ((*token).u.epsf.filename).as_mut_ptr(),
            if (*token).u.epsf.pipe != 0 {
                b"|\0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
        );
    }
    if do_print != 0 {
        fwrite(
            (*token).u.epsf.skipbuf as *const libc::c_void,
            1 as i32 as size_t,
            (*token).u.epsf.skipbuf_pos as size_t,
            cofp,
        );
        loop {
            i = fread(
                buf.as_mut_ptr() as *mut libc::c_void,
                1 as i32 as size_t,
                ::core::mem::size_of::<[i8; 4096]>() as u64,
                (*token).u.epsf.fp,
            ) as i32;
            if !(i != 0 as i32) {
                break;
            }
            fwrite(
                buf.as_mut_ptr() as *const libc::c_void,
                1 as i32 as size_t,
                i as size_t,
                cofp,
            );
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"\n\0" as *const u8 as *const i8);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndDocument\nEndEPSF\n\0" as *const u8 as *const i8);
    }
    if (*token).u.epsf.pipe != 0 {
        pclose((*token).u.epsf.fp);
    } else {
        fclose((*token).u.epsf.fp);
    }
    xfree((*token).u.epsf.skipbuf as *mut libc::c_void);
}
unsafe extern "C" fn read_float(
    mut is: *mut InputStream,
    mut units: i32,
    mut horizontal: i32,
) -> libc::c_double {
    let mut buf: [i8; 256] = [0; 256];
    let mut i: i32 = 0;
    let mut ch: i32 = 0;
    let mut val: libc::c_double = 0.;
    i = 0 as i32;
    while (i as u64)
        < (::core::mem::size_of::<[i8; 256]>() as u64).wrapping_sub(1 as i32 as u64)
        && {
            ch = is_getc(is);
            ch != -(1 as i32)
        }
        && ('0' as i32 <= ch && ch <= '9' as i32 || ch == '.' as i32 || ch == '-' as i32
            || ch == '+' as i32)
    {
        buf[i as usize] = ch as i8;
        i += 1;
        i;
    }
    buf[i as usize] = '\0' as i32 as i8;
    if ch != -(1 as i32) {
        is_ungetc(ch, is);
    }
    val = atof(buf.as_mut_ptr());
    if units != 0 {
        ch = is_getc(is);
        let mut current_block_13: u64;
        match ch {
            99 => {
                val *= 72 as i32 as libc::c_double / 2.54f64;
                current_block_13 = 8831408221741692167;
            }
            112 => {
                current_block_13 = 8831408221741692167;
            }
            105 => {
                val *= 72 as i32 as libc::c_double;
                current_block_13 = 8831408221741692167;
            }
            108 => {
                current_block_13 = 6942066315874235028;
            }
            _ => {
                is_ungetc(ch, is);
                current_block_13 = 6942066315874235028;
            }
        }
        match current_block_13 {
            6942066315874235028 => {
                if horizontal != 0 {
                    val *= *font_widths.as_mut_ptr().offset('m' as i32 as u8 as isize);
                } else {
                    val *= Fpt.h + baselineskip;
                }
            }
            _ => {}
        }
    }
    return val;
}
static mut pass_through_magics: [C2RustUnnamed_4; 5] = [
    {
        let mut init = C2RustUnnamed_4 {
            magic: b"%!\0" as *const u8 as *const i8 as *mut i8,
            magiclen: 2 as i32 as u32,
            name: b"PostScript\0" as *const u8 as *const i8 as *mut i8,
            revert_delta: -(2 as i32),
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            magic: b"\x04%!\0" as *const u8 as *const i8 as *mut i8,
            magiclen: 3 as i32 as u32,
            name: b"PostScript\0" as *const u8 as *const i8 as *mut i8,
            revert_delta: -(2 as i32),
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            magic: b"\x1BE\0" as *const u8 as *const i8 as *mut i8,
            magiclen: 2 as i32 as u32,
            name: b"PCL\0" as *const u8 as *const i8 as *mut i8,
            revert_delta: -(2 as i32),
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            magic: b"\x1B%\0" as *const u8 as *const i8 as *mut i8,
            magiclen: 2 as i32 as u32,
            name: b"PCL\0" as *const u8 as *const i8 as *mut i8,
            revert_delta: -(2 as i32),
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            magic: 0 as *const i8 as *mut i8,
            magiclen: 0 as i32 as u32,
            name: 0 as *const i8 as *mut i8,
            revert_delta: 0 as i32,
        };
        init
    },
];
unsafe extern "C" fn do_pass_through(
    mut fname_0: *mut i8,
    mut is: *mut InputStream,
) -> i32 {
    let mut ch: i32 = 0;
    let mut saved_pos: u64 = (*is).bufpos as u64;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if output_language_pass_through != 0 {
        if quiet == 0 && verbose >= 1 as i32 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"passing through all input files for output language `%s'\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                output_language,
            );
        }
    } else {
        i = 0 as i32;
        while !(pass_through_magics[i as usize].magic).is_null() {
            j = 0 as i32;
            while (j as u32) < pass_through_magics[i as usize].magiclen {
                ch = is_getc(is);
                if ch == -(1 as i32)
                    || ch
                        != *(pass_through_magics[i as usize].magic).offset(j as isize)
                            as u8 as i32
                {
                    break;
                }
                j += 1;
                j;
            }
            if j as u32 >= pass_through_magics[i as usize].magiclen {
                break;
            }
            (*is).bufpos = saved_pos as u32;
            i += 1;
            i;
        }
        if (pass_through_magics[i as usize].magic).is_null() {
            return 0 as i32;
        }
        (*is).bufpos = ((*is).bufpos)
            .wrapping_add(pass_through_magics[i as usize].revert_delta as u32);
        if ps_header_dumped != 0 {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%%%%Page: (%s) -1\n_S\n%%%%BeginDocument: %s\n\0" as *const u8
                        as *const i8,
                    fname_0,
                    fname_0,
                );
            }
        }
        if quiet == 0 && verbose >= 1 as i32 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"passing through %s file \"%s\"\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                pass_through_magics[i as usize].name,
                fname_0,
            );
        }
    }
    loop {
        fwrite(
            ((*is).buf).as_mut_ptr().offset((*is).bufpos as isize)
                as *const libc::c_void,
            1 as i32 as size_t,
            ((*is).data_in_buf).wrapping_sub((*is).bufpos) as size_t,
            ofp,
        );
        (*is).bufpos = (*is).data_in_buf;
        ch = is_getc(is);
        (*is).bufpos = 0 as i32 as u32;
        if !(ch != -(1 as i32)) {
            break;
        }
    }
    if output_language_pass_through == 0 {
        if ps_header_dumped != 0 {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"%%%%EndDocument\n_R\n\0" as *const u8 as *const i8);
            }
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn print_line_number(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut space: libc::c_double,
    mut margin: libc::c_double,
    mut linenum: u32,
) {
    let mut len: libc::c_double = 0.0f64;
    let mut buf: [i8; 20] = [0; 20];
    let mut i: i32 = 0;
    let mut saved_Fname: *mut i8 = b"\0" as *const u8 as *const i8 as *mut i8;
    let mut saved_Fpt: FontPoint = FontPoint { w: 0., h: 0. };
    saved_Fpt.w = 0.0f64;
    saved_Fpt.h = 0.0f64;
    if linenum == print_line_number_last {
        return;
    }
    print_line_number_last = linenum;
    if user_fontp != 0 {
        saved_Fname = Fname;
        saved_Fpt.w = Fpt.w;
        saved_Fpt.h = Fpt.h;
        Fname = default_Fname;
        Fpt.w = default_Fpt.w;
        Fpt.h = default_Fpt.h;
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/F-gs-font %g %g SF\n\0" as *const u8 as *const i8,
                Fpt.w,
                Fpt.h,
            );
        }
        read_font_info();
    }
    sprintf(buf.as_mut_ptr(), b"%d\0" as *const u8 as *const i8, linenum);
    i = 0 as i32;
    while buf[i as usize] != 0 {
        len += *font_widths.as_mut_ptr().offset(buf[i as usize] as u8 as isize);
        i += 1;
        i;
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%g %g M (%s:) s\n\0" as *const u8 as *const i8,
            x + space - len,
            y,
            buf.as_mut_ptr(),
        );
    }
    if user_fontp != 0 {
        Fname = saved_Fname;
        Fpt.w = saved_Fpt.w;
        Fpt.h = saved_Fpt.h;
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/%s %g %g SUF\n\0" as *const u8 as *const i8,
                Fname,
                Fpt.w,
                Fpt.h,
            );
        }
        read_font_info();
    }
}
static mut divertfname: [i8; 512] = [0; 512];
unsafe extern "C" fn divert() {
    let mut cp: *mut i8 = 0 as *mut i8;
    if divertfp.is_null() {} else {
        __assert_fail(
            b"divertfp == NULL\0" as *const u8 as *const i8,
            b"psgen.c\0" as *const u8 as *const i8,
            2623 as i32 as u32,
            (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"void divert()\0"))
                .as_ptr(),
        );
    }
    'c_18653: {
        if divertfp.is_null() {} else {
            __assert_fail(
                b"divertfp == NULL\0" as *const u8 as *const i8,
                b"psgen.c\0" as *const u8 as *const i8,
                2623 as i32 as u32,
                (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"void divert()\0"))
                    .as_ptr(),
            );
        }
    };
    cp = tempnam(0 as *const i8, b"ens\0" as *const u8 as *const i8);
    if cp.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"couldn't create divert file name: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            strerror(*__errno_location()),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fflush(stderr);
        exit(1 as i32);
    }
    strcpy(divertfname.as_mut_ptr(), cp);
    divertfp = fopen(divertfname.as_mut_ptr(), b"w+b\0" as *const u8 as *const i8);
    if divertfp.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"couldn't create divert file \"%s\": %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            divertfname.as_mut_ptr(),
            strerror(*__errno_location()),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fflush(stderr);
        exit(1 as i32);
    }
    if remove(divertfname.as_mut_ptr()) == 0 as i32 {
        divertfname[0 as i32 as usize] = '\0' as i32 as i8;
    }
    free(cp as *mut libc::c_void);
    cofp = divertfp;
}
unsafe extern "C" fn undivert() {
    let mut buf: [i8; 1024] = [0; 1024];
    let mut doc_level: i32 = 0 as i32;
    let mut cp: *mut i8 = 0 as *mut i8;
    if !divertfp.is_null() {} else {
        __assert_fail(
            b"divertfp != NULL\0" as *const u8 as *const i8,
            b"psgen.c\0" as *const u8 as *const i8,
            2657 as i32 as u32,
            (*::core::mem::transmute::<&[u8; 16], &[i8; 16]>(b"void undivert()\0"))
                .as_ptr(),
        );
    }
    'c_8716: {
        if !divertfp.is_null() {} else {
            __assert_fail(
                b"divertfp != NULL\0" as *const u8 as *const i8,
                b"psgen.c\0" as *const u8 as *const i8,
                2657 as i32 as u32,
                (*::core::mem::transmute::<&[u8; 16], &[i8; 16]>(b"void undivert()\0"))
                    .as_ptr(),
            );
        }
    };
    if fseek(divertfp, 0 as i32 as i64, 0 as i32) != 0 as i32 {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"couldn't rewind divert file: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            strerror(*__errno_location()),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fflush(stderr);
        exit(1 as i32);
    }
    while !(fgets(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 1024]>() as u64 as i32,
        divertfp,
    ))
        .is_null()
    {
        if strncmp(
            buf.as_mut_ptr(),
            b"%%BeginDocument\0" as *const u8 as *const i8,
            15 as i32 as u64,
        ) == 0 as i32
        {
            doc_level += 1;
            doc_level;
        } else if strncmp(
            buf.as_mut_ptr(),
            b"%%EndDocument\0" as *const u8 as *const i8,
            13 as i32 as u64,
        ) == 0 as i32
        {
            doc_level -= 1;
            doc_level;
        }
        if doc_level == 0 as i32 {
            if strncmp(
                buf.as_mut_ptr(),
                b"% User defined strings\0" as *const u8 as *const i8,
                22 as i32 as u64,
            ) == 0 as i32
            {
                fputs(buf.as_mut_ptr(), ofp);
                while !(fgets(
                    buf.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 1024]>() as u64 as i32,
                    divertfp,
                ))
                    .is_null()
                {
                    if strncmp(
                        buf.as_mut_ptr(),
                        b"%%EndPageSetup\0" as *const u8 as *const i8,
                        14 as i32 as u64,
                    ) == 0 as i32
                    {
                        break;
                    }
                    cp = strchr(buf.as_mut_ptr(), '\u{1}' as i32);
                    if !cp.is_null() {
                        *cp = '\0' as i32 as i8;
                        fputs(buf.as_mut_ptr(), ofp);
                        fprintf(
                            ofp,
                            b"%d\0" as *const u8 as *const i8,
                            total_pages_in_file,
                        );
                        fputs(cp.offset(1 as i32 as isize), ofp);
                    } else {
                        fputs(buf.as_mut_ptr(), ofp);
                    }
                }
            }
        }
        fputs(buf.as_mut_ptr(), ofp);
    }
    fclose(divertfp);
    divertfp = 0 as *mut FILE;
    if divertfname[0 as i32 as usize] != 0 {
        remove(divertfname.as_mut_ptr());
    }
    cofp = ofp;
}
unsafe extern "C" fn handle_two_side_options() {
    if rotate_even_pages != 0 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"180 rotate\n%d %d translate\n\0" as *const u8 as *const i8,
                -(*media).w,
                -(*media).h,
            );
        }
    }
}