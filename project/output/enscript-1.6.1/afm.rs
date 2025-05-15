use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type stringhash_st;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    fn __errno_location() -> *mut i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    static mut afm_88592_encoding: [AFMEncodingTable; 0];
    static mut afm_88593_encoding: [AFMEncodingTable; 0];
    static mut afm_88594_encoding: [AFMEncodingTable; 0];
    static mut afm_88595_encoding: [AFMEncodingTable; 0];
    fn strhash_free(hash: StringHashPtr);
    fn strhash_get_first(
        hash: StringHashPtr,
        key_return: *mut *mut i8,
        keylen_return: *mut i32,
        data_return: *mut *mut libc::c_void,
    ) -> i32;
    static mut afm_88597_encoding: [AFMEncodingTable; 0];
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut i8,
        keylen: i32,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> i32;
    static mut afm_ibmpc_encoding: [AFMEncodingTable; 0];
    static mut afm_88591_encoding: [AFMEncodingTable; 0];
    fn strhash_get(
        hash: StringHashPtr,
        key: *const i8,
        keylen: i32,
        data_return: *mut *mut libc::c_void,
    ) -> i32;
    static mut afm_mac_encoding: [AFMEncodingTable; 0];
    fn strhash_init() -> StringHashPtr;
    static mut afm_vms_encoding: [AFMEncodingTable; 0];
    static mut afm_hp8_encoding: [AFMEncodingTable; 0];
    static mut afm_koi8_encoding: [AFMEncodingTable; 0];
    fn afm_parse_file(handle: AFMHandle, filename: *const i8, font: AFMFont);
}
pub type size_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
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
pub struct __sigset_t {
    pub __val: [u64; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type AFMString = *mut i8;
pub type AFMName = *mut i8;
pub type AFMNumber = libc::c_double;
pub type AFMInteger = i64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum AFMBoolean {
    AFMFalse = 0,
    AFMTrue = 1,
}
impl AFMBoolean {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            AFMBoolean::AFMFalse => 0,
            AFMBoolean::AFMTrue => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> AFMBoolean {
        match value {
            0 => AFMBoolean::AFMFalse,
            1 => AFMBoolean::AFMTrue,
            _ => panic!("Invalid value for AFMBoolean: {}", value),
        }
    }
}
impl AddAssign<u32> for AFMBoolean {
    fn add_assign(&mut self, rhs: u32) {
        *self = AFMBoolean::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for AFMBoolean {
    fn sub_assign(&mut self, rhs: u32) {
        *self = AFMBoolean::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for AFMBoolean {
    fn mul_assign(&mut self, rhs: u32) {
        *self = AFMBoolean::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for AFMBoolean {
    fn div_assign(&mut self, rhs: u32) {
        *self = AFMBoolean::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for AFMBoolean {
    fn rem_assign(&mut self, rhs: u32) {
        *self = AFMBoolean::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for AFMBoolean {
    type Output = AFMBoolean;
    fn add(self, rhs: u32) -> AFMBoolean {
        AFMBoolean::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for AFMBoolean {
    type Output = AFMBoolean;
    fn sub(self, rhs: u32) -> AFMBoolean {
        AFMBoolean::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for AFMBoolean {
    type Output = AFMBoolean;
    fn mul(self, rhs: u32) -> AFMBoolean {
        AFMBoolean::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for AFMBoolean {
    type Output = AFMBoolean;
    fn div(self, rhs: u32) -> AFMBoolean {
        AFMBoolean::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for AFMBoolean {
    type Output = AFMBoolean;
    fn rem(self, rhs: u32) -> AFMBoolean {
        AFMBoolean::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afm_handle_st {
    pub verbose: u32,
    pub font_map: StringHashPtr,
    pub jmpbuf: jmp_buf,
    pub parse_error: AFMError,
}
pub type StringHashPtr = *mut stringhash_st;
pub type AFMHandle = *mut afm_handle_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum AFMEncoding {
    AFM_ENCODING_DEFAULT,
    AFM_ENCODING_ISO_8859_1,
    AFM_ENCODING_ISO_8859_2,
    AFM_ENCODING_ISO_8859_3,
    AFM_ENCODING_ISO_8859_4,
    AFM_ENCODING_ISO_8859_5,
    AFM_ENCODING_ISO_8859_7,
    AFM_ENCODING_IBMPC,
    AFM_ENCODING_ASCII,
    AFM_ENCODING_MAC,
    AFM_ENCODING_VMS,
    AFM_ENCODING_HP8,
    AFM_ENCODING_KOI8,
}
impl AFMEncoding {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            AFMEncoding::AFM_ENCODING_DEFAULT => 0,
            AFMEncoding::AFM_ENCODING_ISO_8859_1 => 1,
            AFMEncoding::AFM_ENCODING_ISO_8859_2 => 2,
            AFMEncoding::AFM_ENCODING_ISO_8859_3 => 3,
            AFMEncoding::AFM_ENCODING_ISO_8859_4 => 4,
            AFMEncoding::AFM_ENCODING_ISO_8859_5 => 5,
            AFMEncoding::AFM_ENCODING_ISO_8859_7 => 6,
            AFMEncoding::AFM_ENCODING_IBMPC => 7,
            AFMEncoding::AFM_ENCODING_ASCII => 8,
            AFMEncoding::AFM_ENCODING_MAC => 9,
            AFMEncoding::AFM_ENCODING_VMS => 10,
            AFMEncoding::AFM_ENCODING_HP8 => 11,
            AFMEncoding::AFM_ENCODING_KOI8 => 12,
        }
    }
    fn from_libc_c_uint(value: u32) -> AFMEncoding {
        match value {
            0 => AFMEncoding::AFM_ENCODING_DEFAULT,
            1 => AFMEncoding::AFM_ENCODING_ISO_8859_1,
            2 => AFMEncoding::AFM_ENCODING_ISO_8859_2,
            3 => AFMEncoding::AFM_ENCODING_ISO_8859_3,
            4 => AFMEncoding::AFM_ENCODING_ISO_8859_4,
            5 => AFMEncoding::AFM_ENCODING_ISO_8859_5,
            6 => AFMEncoding::AFM_ENCODING_ISO_8859_7,
            7 => AFMEncoding::AFM_ENCODING_IBMPC,
            8 => AFMEncoding::AFM_ENCODING_ASCII,
            9 => AFMEncoding::AFM_ENCODING_MAC,
            10 => AFMEncoding::AFM_ENCODING_VMS,
            11 => AFMEncoding::AFM_ENCODING_HP8,
            12 => AFMEncoding::AFM_ENCODING_KOI8,
            _ => panic!("Invalid value for AFMEncoding: {}", value),
        }
    }
}
impl AddAssign<u32> for AFMEncoding {
    fn add_assign(&mut self, rhs: u32) {
        *self = AFMEncoding::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for AFMEncoding {
    fn sub_assign(&mut self, rhs: u32) {
        *self = AFMEncoding::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for AFMEncoding {
    fn mul_assign(&mut self, rhs: u32) {
        *self = AFMEncoding::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for AFMEncoding {
    fn div_assign(&mut self, rhs: u32) {
        *self = AFMEncoding::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for AFMEncoding {
    fn rem_assign(&mut self, rhs: u32) {
        *self = AFMEncoding::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for AFMEncoding {
    type Output = AFMEncoding;
    fn add(self, rhs: u32) -> AFMEncoding {
        AFMEncoding::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for AFMEncoding {
    type Output = AFMEncoding;
    fn sub(self, rhs: u32) -> AFMEncoding {
        AFMEncoding::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for AFMEncoding {
    type Output = AFMEncoding;
    fn mul(self, rhs: u32) -> AFMEncoding {
        AFMEncoding::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for AFMEncoding {
    type Output = AFMEncoding;
    fn div(self, rhs: u32) -> AFMEncoding {
        AFMEncoding::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for AFMEncoding {
    type Output = AFMEncoding;
    fn rem(self, rhs: u32) -> AFMEncoding {
        AFMEncoding::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afm_font_private_data_st {
    pub undef: *mut AFMIndividualCharacterMetrics,
    pub fontnames: StringHashPtr,
    pub compositenames: StringHashPtr,
}
pub type AFMFont = *mut afm_font_st;
pub type AFMEncodingTable = encoding_table_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encoding_table_st {
    pub code: i32,
    pub character: *mut i8,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
static mut default_path: *mut i8 = b"/usr/local/lib/ps:/usr/lib/ps\0" as *const u8
    as *const i8 as *mut i8;
static mut error_names: [*mut i8; 9] = [
    b"AFM Success\0" as *const u8 as *const i8 as *mut i8,
    b"AFM Error\0" as *const u8 as *const i8 as *mut i8,
    b"out of memory\0" as *const u8 as *const i8 as *mut i8,
    b"illegal argument\0" as *const u8 as *const i8 as *mut i8,
    b"unknown font\0" as *const u8 as *const i8 as *mut i8,
    b"syntax error\0" as *const u8 as *const i8 as *mut i8,
    b"unsupported format\0" as *const u8 as *const i8 as *mut i8,
    b"file IO failed\0" as *const u8 as *const i8 as *mut i8,
    b"file is not an AFM file\0" as *const u8 as *const i8 as *mut i8,
];
#[no_mangle]
pub unsafe extern "C" fn afm_error_to_string(mut error: AFMError, mut buf: *mut i8) {
    let mut syserr: *mut i8 = 0 as *mut i8;
    let mut code: i32 = 0;
    let mut syserrno: i32 = 0;
    code = (error & 0xffff as i32 as u32) as i32;
    syserrno = (error >> 16 as i32 & 0xffff as i32 as u32) as i32;
    if syserrno != 0 {
        syserr = strerror(syserrno);
    } else {
        syserr = 0 as *mut i8;
    }
    if code >= 9 as i32 {
        sprintf(
            buf,
            b"afm_error_to_string(): illegal error code: %d\n\0" as *const u8
                as *const i8,
            error,
        );
        return;
    }
    if code == 0 as i32 {
        sprintf(buf, b"AFM Success\0" as *const u8 as *const i8);
    } else if code == 1 as i32 {
        sprintf(
            buf,
            b"%s%s%s\0" as *const u8 as *const i8,
            b"AFM Error\0" as *const u8 as *const i8,
            if !syserr.is_null() {
                b":\0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            if !syserr.is_null() { syserr } else { b"\0" as *const u8 as *const i8 },
        );
    } else {
        sprintf(
            buf,
            b"AFM Error: %s%s%s\0" as *const u8 as *const i8,
            error_names[code as usize],
            if !syserr.is_null() {
                b": \0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            if !syserr.is_null() { syserr } else { b"\0" as *const u8 as *const i8 },
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn afm_create(
    mut path: *const i8,
    mut verbose_level: u32,
    mut handle_return: *mut AFMHandle,
) -> AFMError {
    let mut handle: AFMHandle = 0 as *mut afm_handle_st;
    let mut error: AFMError = 0 as i32 as AFMError;
    let mut cp: *const i8 = 0 as *const i8;
    let mut cp2: *const i8 = 0 as *const i8;
    let mut len: i32 = 0;
    let mut buf: [i8; 512] = [0; 512];
    let mut stat_st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    handle = calloc(1 as i32 as u64, ::core::mem::size_of::<afm_handle_st>() as u64)
        as AFMHandle;
    if handle.is_null() {
        error = 2 as i32 as AFMError;
    } else {
        (*handle).font_map = strhash_init();
        if ((*handle).font_map).is_null() {
            error = 2 as i32 as AFMError;
        } else {
            (*handle).verbose = verbose_level;
            if path.is_null() {
                path = default_path;
            }
            afm_message(
                handle,
                1 as i32,
                b"AFM: scanning path...\n\0" as *const u8 as *const i8 as *mut i8,
            );
            cp = path;
            while !cp.is_null() {
                if cp != path {
                    cp = cp.offset(1);
                    cp;
                }
                cp2 = strchr(cp, ':' as i32);
                if !cp2.is_null() {
                    len = cp2.offset_from(cp) as i64 as i32;
                } else {
                    len = strlen(cp) as i32;
                }
                memcpy(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    cp as *const libc::c_void,
                    len as u64,
                );
                buf[len as usize] = '\0' as i32 as i8;
                if len > 0 as i32 && buf[(len - 1 as i32) as usize] as i32 == '/' as i32
                {
                    buf[(len - 1 as i32) as usize] = '\0' as i32 as i8;
                }
                strcat(buf.as_mut_ptr(), b"/font.map\0" as *const u8 as *const i8);
                if stat(buf.as_mut_ptr(), &mut stat_st) == 0 as i32 {
                    read_font_map(handle, buf.as_mut_ptr());
                }
                cp = strchr(cp, ':' as i32);
            }
            *handle_return = handle;
            return 0 as i32 as AFMError;
        }
    }
    afm_destroy(handle);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn afm_destroy(mut handle: AFMHandle) -> AFMError {
    let mut key: *mut i8 = 0 as *mut i8;
    let mut keylen: i32 = 0;
    let mut cp: *mut i8 = 0 as *mut i8;
    if handle.is_null() {
        return 3 as i32 as AFMError;
    }
    while strhash_get_first(
        (*handle).font_map,
        &mut key,
        &mut keylen,
        &mut cp as *mut *mut i8 as *mut libc::c_void as *mut *mut libc::c_void,
    ) != 0
    {
        free(cp as *mut libc::c_void);
    }
    strhash_free((*handle).font_map);
    free(handle as *mut libc::c_void);
    return 0 as i32 as AFMError;
}
#[no_mangle]
pub unsafe extern "C" fn afm_set_verbose(
    mut handle: AFMHandle,
    mut level: u32,
) -> AFMError {
    if handle.is_null() {
        return 3 as i32 as AFMError;
    }
    (*handle).verbose = level;
    return 0 as i32 as AFMError;
}
#[no_mangle]
pub unsafe extern "C" fn afm_font_prefix(
    mut handle: AFMHandle,
    mut fontname: *const i8,
    mut prefix_return: *mut *const i8,
) -> AFMError {
    let mut filename: *mut i8 = 0 as *mut i8;
    if handle.is_null() || fontname.is_null() || prefix_return.is_null() {
        return 3 as i32 as AFMError;
    }
    if strhash_get(
        (*handle).font_map,
        fontname,
        strlen(fontname) as i32,
        &mut filename as *mut *mut i8 as *mut libc::c_void as *mut *mut libc::c_void,
    ) == 0
    {
        return 4 as i32 as AFMError;
    }
    *prefix_return = filename;
    return 0 as i32 as AFMError;
}
#[no_mangle]
pub unsafe extern "C" fn afm_open_font(
    mut handle: AFMHandle,
    mut info_level: u32,
    mut fontname: *const i8,
    mut font_return: *mut AFMFont,
) -> AFMError {
    let mut filename: *mut i8 = 0 as *mut i8;
    let mut fname: [i8; 512] = [0; 512];
    if handle.is_null() || fontname.is_null() {
        return 3 as i32 as AFMError;
    }
    if strhash_get(
        (*handle).font_map,
        fontname,
        strlen(fontname) as i32,
        &mut filename as *mut *mut i8 as *mut libc::c_void as *mut *mut libc::c_void,
    ) == 0
    {
        return 4 as i32 as AFMError;
    }
    sprintf(fname.as_mut_ptr(), b"%s.afm\0" as *const u8 as *const i8, filename);
    return afm_open_file(handle, info_level, fname.as_mut_ptr(), font_return);
}
#[no_mangle]
pub unsafe extern "C" fn afm_open_file(
    mut handle: AFMHandle,
    mut info_level: u32,
    mut filename: *const i8,
    mut font_return: *mut AFMFont,
) -> AFMError {
    let mut font: AFMFont = 0 as *mut afm_font_st;
    let mut error: AFMError = 0 as i32 as AFMError;
    if handle.is_null() || filename.is_null() {
        return 3 as i32 as AFMError;
    }
    font = calloc(1 as i32 as u64, ::core::mem::size_of::<afm_font_st>() as u64)
        as AFMFont;
    if font.is_null() {
        return 2 as i32 as AFMError;
    }
    (*font).private = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<afm_font_private_data_st>() as u64,
    ) as *mut afm_font_private_data_st;
    if ((*font).private).is_null() {
        error = 2 as i32 as AFMError;
    } else {
        (*(*font).private).fontnames = strhash_init();
        if ((*(*font).private).fontnames).is_null() {
            error = 2 as i32 as AFMError;
        } else {
            (*(*font).private).compositenames = strhash_init();
            if ((*(*font).private).compositenames).is_null() {
                error = 2 as i32 as AFMError;
            } else {
                (*font).info_level = info_level;
                if _setjmp(((*handle).jmpbuf).as_mut_ptr()) != 0 {
                    error = (*handle).parse_error;
                } else {
                    afm_parse_file(handle, filename, font);
                    *font_return = font;
                    return 0 as i32 as AFMError;
                }
            }
        }
    }
    afm_close_font(font);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn afm_close_font(mut font: AFMFont) -> AFMError {
    let mut i: i32 = 0;
    if font.is_null() {
        return 3 as i32 as AFMError;
    }
    if !((*font).global_info.FontName).is_null() {
        free((*font).global_info.FontName as *mut libc::c_void);
    }
    if !((*font).global_info.FullName).is_null() {
        free((*font).global_info.FullName as *mut libc::c_void);
    }
    if !((*font).global_info.FamilyName).is_null() {
        free((*font).global_info.FamilyName as *mut libc::c_void);
    }
    if !((*font).global_info.Weight).is_null() {
        free((*font).global_info.Weight as *mut libc::c_void);
    }
    if !((*font).global_info.Version).is_null() {
        free((*font).global_info.Version as *mut libc::c_void);
    }
    if !((*font).global_info.Notice).is_null() {
        free((*font).global_info.Notice as *mut libc::c_void);
    }
    if !((*font).global_info.EncodingScheme).is_null() {
        free((*font).global_info.EncodingScheme as *mut libc::c_void);
    }
    if !((*font).global_info.CharacterSet).is_null() {
        free((*font).global_info.CharacterSet as *mut libc::c_void);
    }
    i = 0 as i32;
    while (i as i64) < (*font).num_character_metrics {
        if !((*((*font).character_metrics).offset(i as isize)).name).is_null() {
            free(
                (*((*font).character_metrics).offset(i as isize)).name
                    as *mut libc::c_void,
            );
        }
        i += 1;
        i;
    }
    if !((*font).character_metrics).is_null() {
        free((*font).character_metrics as *mut libc::c_void);
    }
    i = 0 as i32;
    while (i as i64) < (*font).num_composites {
        if !((*((*font).composites).offset(i as isize)).name).is_null() {
            free((*((*font).composites).offset(i as isize)).name as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    if !((*font).composites).is_null() {
        free((*font).composites as *mut libc::c_void);
    }
    i = 0 as i32;
    while (i as i64) < (*font).num_kern_pairs {
        if !((*((*font).kern_pairs).offset(i as isize)).name1).is_null() {
            free((*((*font).kern_pairs).offset(i as isize)).name1 as *mut libc::c_void);
        }
        if !((*((*font).kern_pairs).offset(i as isize)).name2).is_null() {
            free((*((*font).kern_pairs).offset(i as isize)).name2 as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    if !((*font).kern_pairs).is_null() {
        free((*font).kern_pairs as *mut libc::c_void);
    }
    if !((*font).track_kerns).is_null() {
        free((*font).track_kerns as *mut libc::c_void);
    }
    strhash_free((*(*font).private).fontnames);
    strhash_free((*(*font).private).compositenames);
    free(font as *mut libc::c_void);
    return 0 as i32 as AFMError;
}
#[no_mangle]
pub unsafe extern "C" fn afm_font_dump(mut fp: *mut FILE, mut font: AFMFont) {
    let mut i: i32 = 0;
    fprintf(
        fp,
        b"AFM Format Specification version: %g\n\0" as *const u8 as *const i8,
        (*font).version,
    );
    fprintf(fp, b"Global Font Information\n\0" as *const u8 as *const i8);
    fprintf(
        fp,
        b"  FontName:\t%s\n\0" as *const u8 as *const i8,
        if !((*font).global_info.FontName).is_null() {
            (*font).global_info.FontName as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    fprintf(
        fp,
        b"  FullName:\t%s\n\0" as *const u8 as *const i8,
        if !((*font).global_info.FullName).is_null() {
            (*font).global_info.FullName as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    fprintf(
        fp,
        b"  FamilyName:\t%s\n\0" as *const u8 as *const i8,
        if !((*font).global_info.FamilyName).is_null() {
            (*font).global_info.FamilyName as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    fprintf(
        fp,
        b"  Weight:\t%s\n\0" as *const u8 as *const i8,
        if !((*font).global_info.Weight).is_null() {
            (*font).global_info.Weight as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    fprintf(
        fp,
        b"  FontBBox:\t%g %g %g %g\n\0" as *const u8 as *const i8,
        (*font).global_info.FontBBox_llx,
        (*font).global_info.FontBBox_lly,
        (*font).global_info.FontBBox_urx,
        (*font).global_info.FontBBox_ury,
    );
    fprintf(
        fp,
        b"  Version:\t%s\n\0" as *const u8 as *const i8,
        if !((*font).global_info.Version).is_null() {
            (*font).global_info.Version as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    fprintf(
        fp,
        b"  Notice:\t%s\n\0" as *const u8 as *const i8,
        if !((*font).global_info.Notice).is_null() {
            (*font).global_info.Notice as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    fprintf(
        fp,
        b"  EncodingScheme:\t%s\n\0" as *const u8 as *const i8,
        if !((*font).global_info.EncodingScheme).is_null() {
            (*font).global_info.EncodingScheme as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    fprintf(
        fp,
        b"  MappingScheme:\t%ld\n\0" as *const u8 as *const i8,
        (*font).global_info.MappingScheme,
    );
    fprintf(
        fp,
        b"  EscChar:\t%ld\n\0" as *const u8 as *const i8,
        (*font).global_info.EscChar,
    );
    fprintf(
        fp,
        b"  CharacterSet:\t%s\n\0" as *const u8 as *const i8,
        if !((*font).global_info.CharacterSet).is_null() {
            (*font).global_info.CharacterSet as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    fprintf(
        fp,
        b"  Characters:\t%ld\n\0" as *const u8 as *const i8,
        (*font).global_info.Characters,
    );
    fprintf(
        fp,
        b"  IsBaseFont:\t%s\n\0" as *const u8 as *const i8,
        if (*font).global_info.IsBaseFont as u32 != 0 {
            b"true\0" as *const u8 as *const i8
        } else {
            b"false\0" as *const u8 as *const i8
        },
    );
    fprintf(
        fp,
        b"  VVector:\t%g %g\n\0" as *const u8 as *const i8,
        (*font).global_info.VVector_0,
        (*font).global_info.VVector_1,
    );
    fprintf(
        fp,
        b"  IsFixedV:\t%s\n\0" as *const u8 as *const i8,
        if (*font).global_info.IsFixedV as u32 != 0 {
            b"true\0" as *const u8 as *const i8
        } else {
            b"false\0" as *const u8 as *const i8
        },
    );
    fprintf(
        fp,
        b"  CapHeight:\t%g\n\0" as *const u8 as *const i8,
        (*font).global_info.CapHeight,
    );
    fprintf(
        fp,
        b"  XHeight:\t%g\n\0" as *const u8 as *const i8,
        (*font).global_info.XHeight,
    );
    fprintf(
        fp,
        b"  Ascender:\t%g\n\0" as *const u8 as *const i8,
        (*font).global_info.Ascender,
    );
    fprintf(
        fp,
        b"  Descender:\t%g\n\0" as *const u8 as *const i8,
        (*font).global_info.Descender,
    );
    i = 0 as i32;
    while i < 2 as i32 {
        if (*font).writing_direction_metrics[i as usize].is_valid as u64 != 0 {
            fprintf(fp, b"Writing Direction %d\n\0" as *const u8 as *const i8, i);
            fprintf(
                fp,
                b"  UnderlinePosition: %g\n\0" as *const u8 as *const i8,
                (*font).writing_direction_metrics[i as usize].UnderlinePosition,
            );
            fprintf(
                fp,
                b"  UnderlineThickness: %g\n\0" as *const u8 as *const i8,
                (*font).writing_direction_metrics[i as usize].UnderlineThickness,
            );
            fprintf(
                fp,
                b"  ItalicAngle: %g\n\0" as *const u8 as *const i8,
                (*font).writing_direction_metrics[i as usize].ItalicAngle,
            );
            fprintf(
                fp,
                b"  CharWidth: %g %g\n\0" as *const u8 as *const i8,
                (*font).writing_direction_metrics[i as usize].CharWidth_x,
                (*font).writing_direction_metrics[i as usize].CharWidth_y,
            );
            fprintf(
                fp,
                b"  IsFixedPitch: %s\n\0" as *const u8 as *const i8,
                if (*font).writing_direction_metrics[i as usize].IsFixedPitch as u32 != 0
                {
                    b"true\0" as *const u8 as *const i8
                } else {
                    b"false\0" as *const u8 as *const i8
                },
            );
        }
        i += 1;
        i;
    }
    fprintf(
        fp,
        b"Individual Character Metrics %ld\n\0" as *const u8 as *const i8,
        (*font).num_character_metrics,
    );
    i = 0 as i32;
    while (i as i64) < (*font).num_character_metrics {
        let mut cm: *mut AFMIndividualCharacterMetrics = 0
            as *mut AFMIndividualCharacterMetrics;
        cm = &mut *((*font).character_metrics).offset(i as isize)
            as *mut AFMIndividualCharacterMetrics;
        fprintf(
            fp,
            b"  C %ld ; N %s ; B %g %g %g %g\n\0" as *const u8 as *const i8,
            (*cm).character_code,
            if !((*cm).name).is_null() {
                (*cm).name as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            (*cm).llx,
            (*cm).lly,
            (*cm).urx,
            (*cm).ury,
        );
        fprintf(
            fp,
            b"    W0X %g ; W0Y %g ; W1X %g ; W1Y %g ; VV %g %g\n\0" as *const u8
                as *const i8,
            (*cm).w0x,
            (*cm).w0y,
            (*cm).w1x,
            (*cm).w1y,
            (*cm).vv_x,
            (*cm).vv_y,
        );
        i += 1;
        i;
    }
    fprintf(
        fp,
        b"Composite Character Data %ld\n\0" as *const u8 as *const i8,
        (*font).num_composites,
    );
    i = 0 as i32;
    while (i as i64) < (*font).num_composites {
        let mut cm_0: *mut AFMComposite = 0 as *mut AFMComposite;
        let mut j: i32 = 0;
        cm_0 = &mut *((*font).composites).offset(i as isize) as *mut AFMComposite;
        fprintf(
            fp,
            b"  CC %s %ld\0" as *const u8 as *const i8,
            (*cm_0).name,
            (*cm_0).num_components,
        );
        j = 0 as i32;
        while (j as i64) < (*cm_0).num_components {
            fprintf(
                fp,
                b" ; PCC %s %g %g\0" as *const u8 as *const i8,
                (*((*cm_0).components).offset(j as isize)).name,
                (*((*cm_0).components).offset(j as isize)).deltax,
                (*((*cm_0).components).offset(j as isize)).deltay,
            );
            j += 1;
            j;
        }
        fprintf(fp, b"\n\0" as *const u8 as *const i8);
        i += 1;
        i;
    }
    fprintf(
        fp,
        b"Pair-Wise Kerning %ld\n\0" as *const u8 as *const i8,
        (*font).num_kern_pairs,
    );
    i = 0 as i32;
    while (i as i64) < (*font).num_kern_pairs {
        let mut kp: *mut AFMPairWiseKerning = 0 as *mut AFMPairWiseKerning;
        kp = &mut *((*font).kern_pairs).offset(i as isize) as *mut AFMPairWiseKerning;
        fprintf(
            fp,
            b"  KP %s %s %g %g\n\0" as *const u8 as *const i8,
            if !((*kp).name1).is_null() {
                (*kp).name1 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            if !((*kp).name2).is_null() {
                (*kp).name2 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            (*kp).kx,
            (*kp).ky,
        );
        i += 1;
        i;
    }
    fprintf(
        fp,
        b"Track Kerning %ld\n\0" as *const u8 as *const i8,
        (*font).num_track_kerns,
    );
    i = 0 as i32;
    while (i as i64) < (*font).num_track_kerns {
        let mut tk: *mut AFMTrackKern = 0 as *mut AFMTrackKern;
        tk = &mut *((*font).track_kerns).offset(i as isize) as *mut AFMTrackKern;
        fprintf(
            fp,
            b"  TrackKern %ld %g %g %g %g\n\0" as *const u8 as *const i8,
            (*tk).degree,
            (*tk).min_ptsize,
            (*tk).min_kern,
            (*tk).max_ptsize,
            (*tk).max_kern,
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn afm_font_stringwidth(
    mut font: AFMFont,
    mut ptsize: AFMNumber,
    mut string: *mut i8,
    mut stringlen: u32,
    mut w0x_return: *mut AFMNumber,
    mut w0y_return: *mut AFMNumber,
) -> AFMError {
    let mut i: u32 = 0;
    let mut x: AFMNumber = 0.0f64;
    let mut y: AFMNumber = 0.0f64;
    let mut cm: *mut AFMIndividualCharacterMetrics = 0
        as *mut AFMIndividualCharacterMetrics;
    if font.is_null() || string.is_null()
        || (*font).writing_direction_metrics[0 as i32 as usize].is_valid as u64 == 0
    {
        return 3 as i32 as AFMError;
    }
    if (*font).writing_direction_metrics[0 as i32 as usize].IsFixedPitch as u64 != 0 {
        x = stringlen as libc::c_double
            * (*font).writing_direction_metrics[0 as i32 as usize].CharWidth_x;
        y = stringlen as libc::c_double
            * (*font).writing_direction_metrics[0 as i32 as usize].CharWidth_y;
    } else {
        i = 0 as i32 as u32;
        while i < stringlen {
            cm = (*font).encoding[*string.offset(i as isize) as u8 as usize];
            if cm.is_null()
                || cm
                    == 1 as i32 as *mut libc::c_void
                        as *mut AFMIndividualCharacterMetrics
            {
                x += (*(*(*font).private).undef).w0x;
                y += (*(*(*font).private).undef).w0y;
            } else {
                x += (*cm).w0x;
                y += (*cm).w0y;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    *w0x_return = x / 1000 as i32 as libc::c_double * ptsize;
    *w0y_return = y / 1000 as i32 as libc::c_double * ptsize;
    return 0 as i32 as AFMError;
}
#[no_mangle]
pub unsafe extern "C" fn afm_font_charwidth(
    mut font: AFMFont,
    mut ptsize: AFMNumber,
    mut ch: i8,
    mut w0x_return: *mut AFMNumber,
    mut w0y_return: *mut AFMNumber,
) -> AFMError {
    let mut x: AFMNumber = 0.0f64;
    let mut y: AFMNumber = 0.0f64;
    let mut cm: *mut AFMIndividualCharacterMetrics = 0
        as *mut AFMIndividualCharacterMetrics;
    if font.is_null()
        || (*font).writing_direction_metrics[0 as i32 as usize].is_valid as u64 == 0
    {
        return 3 as i32 as AFMError;
    }
    if (*font).writing_direction_metrics[0 as i32 as usize].IsFixedPitch as u64 != 0 {
        x = (*font).writing_direction_metrics[0 as i32 as usize].CharWidth_x;
        y = (*font).writing_direction_metrics[0 as i32 as usize].CharWidth_y;
    } else {
        cm = (*font).encoding[ch as u8 as usize];
        if cm.is_null()
            || cm == 1 as i32 as *mut libc::c_void as *mut AFMIndividualCharacterMetrics
        {
            x = (*(*(*font).private).undef).w0x;
            y = (*(*(*font).private).undef).w0y;
        } else {
            x = (*cm).w0x;
            y = (*cm).w0y;
        }
    }
    *w0x_return = x / 1000 as i32 as libc::c_double * ptsize;
    *w0y_return = y / 1000 as i32 as libc::c_double * ptsize;
    return 0 as i32 as AFMError;
}
#[no_mangle]
pub unsafe extern "C" fn afm_font_encode(
    mut font: AFMFont,
    mut code: u8,
    mut name: *mut i8,
    mut flags: u32,
) -> AFMError {
    let mut cm: *mut AFMIndividualCharacterMetrics = 0
        as *mut AFMIndividualCharacterMetrics;
    let mut comp: *mut AFMComposite = 0 as *mut AFMComposite;
    if font.is_null() {
        return 3 as i32 as AFMError;
    }
    if !name.is_null() {
        if strhash_get(
            (*(*font).private).fontnames,
            name,
            strlen(name) as i32,
            &mut cm as *mut *mut AFMIndividualCharacterMetrics as *mut libc::c_void
                as *mut *mut libc::c_void,
        ) == 0
        {
            if flags & 0x1 as i32 as u32 == 0 as i32 as u32
                || strhash_get(
                    (*(*font).private).compositenames,
                    name,
                    strlen(name) as i32,
                    &mut comp as *mut *mut AFMComposite as *mut libc::c_void
                        as *mut *mut libc::c_void,
                ) == 0 as i32
            {
                cm = 1 as i32 as *mut libc::c_void as *mut AFMIndividualCharacterMetrics;
            } else if strhash_get(
                (*(*font).private).fontnames,
                (*((*comp).components).offset(0 as i32 as isize)).name as *const i8,
                strlen(
                    (*((*comp).components).offset(0 as i32 as isize)).name as *const i8,
                ) as i32,
                &mut cm as *mut *mut AFMIndividualCharacterMetrics as *mut libc::c_void
                    as *mut *mut libc::c_void,
            ) == 0
            {
                cm = 1 as i32 as *mut libc::c_void as *mut AFMIndividualCharacterMetrics;
            }
        }
    } else {
        cm = 0 as *mut AFMIndividualCharacterMetrics;
    }
    (*font).encoding[code as u32 as usize] = cm;
    return 0 as i32 as AFMError;
}
#[no_mangle]
pub unsafe extern "C" fn afm_font_encoding(
    mut font: AFMFont,
    mut enc: AFMEncoding,
    mut flags: u32,
) -> AFMError {
    let mut i: i32 = 0;
    let mut cm: *mut AFMIndividualCharacterMetrics = 0
        as *mut AFMIndividualCharacterMetrics;
    if font.is_null() {
        return 3 as i32 as AFMError;
    }
    match enc as u32 {
        0 => {
            i = 0 as i32;
            while i < 256 as i32 {
                (*font).encoding[i as usize] = 0 as *mut AFMIndividualCharacterMetrics;
                i += 1;
                i;
            }
            i = 0 as i32;
            while (i as i64) < (*font).num_character_metrics {
                cm = &mut *((*font).character_metrics).offset(i as isize)
                    as *mut AFMIndividualCharacterMetrics;
                (*font).encoding[(*cm).character_code as usize] = cm;
                i += 1;
                i;
            }
        }
        1 => {
            apply_encoding(font, afm_88591_encoding.as_mut_ptr(), flags);
        }
        2 => {
            apply_encoding(font, afm_88592_encoding.as_mut_ptr(), flags);
        }
        3 => {
            apply_encoding(font, afm_88593_encoding.as_mut_ptr(), flags);
        }
        4 => {
            apply_encoding(font, afm_88594_encoding.as_mut_ptr(), flags);
        }
        5 => {
            apply_encoding(font, afm_88595_encoding.as_mut_ptr(), flags);
        }
        6 => {
            apply_encoding(font, afm_88597_encoding.as_mut_ptr(), flags);
        }
        7 => {
            apply_encoding(font, afm_ibmpc_encoding.as_mut_ptr(), flags);
        }
        8 => {
            apply_encoding(font, afm_88591_encoding.as_mut_ptr(), flags);
            i = 128 as i32;
            while i < 256 as i32 {
                (*font).encoding[i as usize] = 0 as *mut AFMIndividualCharacterMetrics;
                i += 1;
                i;
            }
        }
        9 => {
            apply_encoding(font, afm_mac_encoding.as_mut_ptr(), flags);
        }
        10 => {
            apply_encoding(font, afm_vms_encoding.as_mut_ptr(), flags);
        }
        11 => {
            apply_encoding(font, afm_hp8_encoding.as_mut_ptr(), flags);
        }
        12 => {
            apply_encoding(font, afm_koi8_encoding.as_mut_ptr(), flags);
        }
        _ => {}
    }
    return 0 as i32 as AFMError;
}
#[no_mangle]
pub unsafe extern "C" fn afm_message(
    mut handle: AFMHandle,
    mut level: i32,
    mut message: *mut i8,
) {
    if (*handle).verbose < level as u32 {
        return;
    }
    fprintf(stderr, b"%s\0" as *const u8 as *const i8, message);
}
#[no_mangle]
pub unsafe extern "C" fn afm_error(mut handle: AFMHandle, mut message: *mut i8) {
    fprintf(stderr, b"AFM Error: %s\n\0" as *const u8 as *const i8, message);
}
unsafe extern "C" fn read_font_map(mut handle: AFMHandle, mut name: *mut i8) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [i8; 512] = [0; 512];
    let mut fullname: [i8; 512] = [0; 512];
    let mut dirlen: u32 = 0;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut cp2: *mut i8 = 0 as *mut i8;
    let mut msg: [i8; 256] = [0; 256];
    sprintf(
        msg.as_mut_ptr(),
        b"AFM: reading font map \"%s\"\n\0" as *const u8 as *const i8,
        name,
    );
    afm_message(handle, 1 as i32, msg.as_mut_ptr());
    fp = fopen(name, b"r\0" as *const u8 as *const i8);
    if fp.is_null() {
        sprintf(
            msg.as_mut_ptr(),
            b"AFM: couldn't open font map \"%s\": %s\n\0" as *const u8 as *const i8,
            name,
            strerror(*__errno_location()),
        );
        afm_message(handle, 1 as i32, msg.as_mut_ptr());
        return;
    }
    cp = strrchr(name, '/' as i32);
    if !cp.is_null() {
        dirlen = (cp.offset_from(name) as i64 + 1 as i32 as i64) as u32;
        memcpy(
            fullname.as_mut_ptr() as *mut libc::c_void,
            name as *const libc::c_void,
            dirlen as u64,
        );
    } else {
        dirlen = 2 as i32 as u32;
        memcpy(
            fullname.as_mut_ptr() as *mut libc::c_void,
            b"./\0" as *const u8 as *const i8 as *const libc::c_void,
            dirlen as u64,
        );
    }
    while !(fgets(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 512]>() as u64 as i32,
        fp,
    ))
        .is_null()
    {
        let mut font: [i8; 256] = [0; 256];
        let mut file: [i8; 256] = [0; 256];
        if sscanf(
            buf.as_mut_ptr(),
            b"%s %s\0" as *const u8 as *const i8,
            font.as_mut_ptr(),
            file.as_mut_ptr(),
        ) != 2 as i32
        {
            sprintf(
                msg.as_mut_ptr(),
                b"malformed line in font map \"%s\":\n%s\0" as *const u8 as *const i8,
                name,
                buf.as_mut_ptr(),
            );
            afm_error(handle, msg.as_mut_ptr());
        } else {
            if strhash_get(
                (*handle).font_map,
                font.as_mut_ptr(),
                strlen(font.as_mut_ptr()) as i32,
                &mut cp as *mut *mut i8 as *mut libc::c_void as *mut *mut libc::c_void,
            ) != 0
            {
                continue;
            }
            strcpy(fullname.as_mut_ptr().offset(dirlen as isize), file.as_mut_ptr());
            cp = malloc((strlen(fullname.as_mut_ptr())).wrapping_add(1 as i32 as u64))
                as *mut i8;
            if cp.is_null() {
                afm_error(
                    handle,
                    b"couldn't add font: out of memory\0" as *const u8 as *const i8
                        as *mut i8,
                );
                break;
            } else {
                strcpy(cp, fullname.as_mut_ptr());
                sprintf(
                    msg.as_mut_ptr(),
                    b"AFM: font mapping: %s -> %s\n\0" as *const u8 as *const i8,
                    font.as_mut_ptr(),
                    cp,
                );
                afm_message(handle, 2 as i32, msg.as_mut_ptr());
                strhash_put(
                    (*handle).font_map,
                    font.as_mut_ptr(),
                    strlen(font.as_mut_ptr()) as i32,
                    cp as *mut libc::c_void,
                    &mut cp2 as *mut *mut i8 as *mut libc::c_void
                        as *mut *mut libc::c_void,
                );
            }
        }
    }
    fclose(fp);
}
unsafe extern "C" fn apply_encoding(
    mut font: AFMFont,
    mut enc: *mut AFMEncodingTable,
    mut flags: u32,
) {
    let mut i: i32 = 0;
    let mut cm: *mut AFMIndividualCharacterMetrics = 0
        as *mut AFMIndividualCharacterMetrics;
    let mut comp: *mut AFMComposite = 0 as *mut AFMComposite;
    i = 0 as i32;
    while (*enc.offset(i as isize)).code >= 0 as i32 {
        if ((*enc.offset(i as isize)).character).is_null() {
            (*font).encoding[(*enc.offset(i as isize)).code as usize] = 0
                as *mut AFMIndividualCharacterMetrics;
        } else if (*enc.offset(i as isize)).character
            == 1 as i32 as *mut libc::c_void as *mut i8
        {
            (*font).encoding[(*enc.offset(i as isize)).code as usize] = 1 as i32
                as *mut libc::c_void as *mut AFMIndividualCharacterMetrics;
        } else if strhash_get(
            (*(*font).private).fontnames,
            (*enc.offset(i as isize)).character,
            strlen((*enc.offset(i as isize)).character) as i32,
            &mut cm as *mut *mut AFMIndividualCharacterMetrics as *mut libc::c_void
                as *mut *mut libc::c_void,
        ) != 0
        {
            (*font).encoding[(*enc.offset(i as isize)).code as usize] = cm;
        } else if flags & 0x1 as i32 as u32 == 0 as i32 as u32
            || strhash_get(
                (*(*font).private).compositenames,
                (*enc.offset(i as isize)).character,
                strlen((*enc.offset(i as isize)).character) as i32,
                &mut comp as *mut *mut AFMComposite as *mut libc::c_void
                    as *mut *mut libc::c_void,
            ) == 0 as i32
        {
            (*font).encoding[(*enc.offset(i as isize)).code as usize] = 1 as i32
                as *mut libc::c_void as *mut AFMIndividualCharacterMetrics;
        } else if strhash_get(
            (*(*font).private).fontnames,
            (*((*comp).components).offset(0 as i32 as isize)).name as *const i8,
            strlen((*((*comp).components).offset(0 as i32 as isize)).name as *const i8)
                as i32,
            &mut cm as *mut *mut AFMIndividualCharacterMetrics as *mut libc::c_void
                as *mut *mut libc::c_void,
        ) != 0
        {
            (*font).encoding[(*enc.offset(i as isize)).code as usize] = cm;
        } else {
            (*font).encoding[(*enc.offset(i as isize)).code as usize] = 1 as i32
                as *mut libc::c_void as *mut AFMIndividualCharacterMetrics;
        }
        i += 1;
        i;
    }
}