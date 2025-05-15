use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type stringhash_st;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn _IO_getc(__fp: *mut _IO_FILE) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn ungetc(__c: i32, __stream: *mut FILE) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __errno_location() -> *mut i32;
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut i8,
        keylen: i32,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> i32;
    fn strhash_get(
        hash: StringHashPtr,
        key: *const i8,
        keylen: i32,
        data_return: *mut *mut libc::c_void,
    ) -> i32;
    fn afm_error(handle: AFMHandle, message: *mut i8);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum AFMKey {
    kComment,
    kStartFontMetrics,
    kEndFontMetrics,
    kStartCompFontMetrics,
    kEndCompFontMetrics,
    kStartDescendent,
    kEndDescendent,
    kStartMasterFontMetrics,
    kEndMasterFontMetrics,
    kMetricsSets,
    kDescendents,
    kMasters,
    kAxes,
    kFontName,
    kFullName,
    kFamilyName,
    kWeight,
    kFontBBox,
    kVersion,
    kNotice,
    kEncodingScheme,
    kMappingScheme,
    kEscChar,
    kCharacterSet,
    kCharacters,
    kIsBaseFont,
    kVVector,
    kIsFixedV,
    kCapHeight,
    kXHeight,
    kAscender,
    kDescender,
    kWeightVector,
    kBlendDesignPositions,
    kBlendDesignMap,
    kBlendAxisTypes,
    kStartDirection,
    kEndDirection,
    kUnderlinePosition,
    kUnderlineThickness,
    kItalicAngle,
    kCharWidth,
    kIsFixedPitch,
    kStartCharMetrics,
    kEndCharMetrics,
    kC,
    kCH,
    kWX,
    kW0X,
    kW1X,
    kWY,
    kW0Y,
    kW1Y,
    kW,
    kW0,
    kW1,
    kVV,
    kN,
    kB,
    kL,
    kStartKernData,
    kEndKernData,
    kStartTrackKern,
    kEndTrackKern,
    kTrackKern,
    kStartKernPairs,
    kEndKernPairs,
    kKP,
    kKPH,
    kKPX,
    kKPY,
    kStartComposites,
    kEndComposites,
    kCC,
    kPCC,
    kStartAxis,
    kEndAxis,
    kAxisType,
    kAxisLabel,
    kStartMaster,
    kEndMaster,
}
impl AFMKey {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            AFMKey::kComment => 0,
            AFMKey::kStartFontMetrics => 1,
            AFMKey::kEndFontMetrics => 2,
            AFMKey::kStartCompFontMetrics => 3,
            AFMKey::kEndCompFontMetrics => 4,
            AFMKey::kStartDescendent => 5,
            AFMKey::kEndDescendent => 6,
            AFMKey::kStartMasterFontMetrics => 7,
            AFMKey::kEndMasterFontMetrics => 8,
            AFMKey::kMetricsSets => 9,
            AFMKey::kDescendents => 10,
            AFMKey::kMasters => 11,
            AFMKey::kAxes => 12,
            AFMKey::kFontName => 13,
            AFMKey::kFullName => 14,
            AFMKey::kFamilyName => 15,
            AFMKey::kWeight => 16,
            AFMKey::kFontBBox => 17,
            AFMKey::kVersion => 18,
            AFMKey::kNotice => 19,
            AFMKey::kEncodingScheme => 20,
            AFMKey::kMappingScheme => 21,
            AFMKey::kEscChar => 22,
            AFMKey::kCharacterSet => 23,
            AFMKey::kCharacters => 24,
            AFMKey::kIsBaseFont => 25,
            AFMKey::kVVector => 26,
            AFMKey::kIsFixedV => 27,
            AFMKey::kCapHeight => 28,
            AFMKey::kXHeight => 29,
            AFMKey::kAscender => 30,
            AFMKey::kDescender => 31,
            AFMKey::kWeightVector => 32,
            AFMKey::kBlendDesignPositions => 33,
            AFMKey::kBlendDesignMap => 34,
            AFMKey::kBlendAxisTypes => 35,
            AFMKey::kStartDirection => 36,
            AFMKey::kEndDirection => 37,
            AFMKey::kUnderlinePosition => 38,
            AFMKey::kUnderlineThickness => 39,
            AFMKey::kItalicAngle => 40,
            AFMKey::kCharWidth => 41,
            AFMKey::kIsFixedPitch => 42,
            AFMKey::kStartCharMetrics => 43,
            AFMKey::kEndCharMetrics => 44,
            AFMKey::kC => 45,
            AFMKey::kCH => 46,
            AFMKey::kWX => 47,
            AFMKey::kW0X => 48,
            AFMKey::kW1X => 49,
            AFMKey::kWY => 50,
            AFMKey::kW0Y => 51,
            AFMKey::kW1Y => 52,
            AFMKey::kW => 53,
            AFMKey::kW0 => 54,
            AFMKey::kW1 => 55,
            AFMKey::kVV => 56,
            AFMKey::kN => 57,
            AFMKey::kB => 58,
            AFMKey::kL => 59,
            AFMKey::kStartKernData => 60,
            AFMKey::kEndKernData => 61,
            AFMKey::kStartTrackKern => 62,
            AFMKey::kEndTrackKern => 63,
            AFMKey::kTrackKern => 64,
            AFMKey::kStartKernPairs => 65,
            AFMKey::kEndKernPairs => 66,
            AFMKey::kKP => 67,
            AFMKey::kKPH => 68,
            AFMKey::kKPX => 69,
            AFMKey::kKPY => 70,
            AFMKey::kStartComposites => 71,
            AFMKey::kEndComposites => 72,
            AFMKey::kCC => 73,
            AFMKey::kPCC => 74,
            AFMKey::kStartAxis => 75,
            AFMKey::kEndAxis => 76,
            AFMKey::kAxisType => 77,
            AFMKey::kAxisLabel => 78,
            AFMKey::kStartMaster => 79,
            AFMKey::kEndMaster => 80,
        }
    }
    fn from_libc_c_uint(value: u32) -> AFMKey {
        match value {
            0 => AFMKey::kComment,
            1 => AFMKey::kStartFontMetrics,
            2 => AFMKey::kEndFontMetrics,
            3 => AFMKey::kStartCompFontMetrics,
            4 => AFMKey::kEndCompFontMetrics,
            5 => AFMKey::kStartDescendent,
            6 => AFMKey::kEndDescendent,
            7 => AFMKey::kStartMasterFontMetrics,
            8 => AFMKey::kEndMasterFontMetrics,
            9 => AFMKey::kMetricsSets,
            10 => AFMKey::kDescendents,
            11 => AFMKey::kMasters,
            12 => AFMKey::kAxes,
            13 => AFMKey::kFontName,
            14 => AFMKey::kFullName,
            15 => AFMKey::kFamilyName,
            16 => AFMKey::kWeight,
            17 => AFMKey::kFontBBox,
            18 => AFMKey::kVersion,
            19 => AFMKey::kNotice,
            20 => AFMKey::kEncodingScheme,
            21 => AFMKey::kMappingScheme,
            22 => AFMKey::kEscChar,
            23 => AFMKey::kCharacterSet,
            24 => AFMKey::kCharacters,
            25 => AFMKey::kIsBaseFont,
            26 => AFMKey::kVVector,
            27 => AFMKey::kIsFixedV,
            28 => AFMKey::kCapHeight,
            29 => AFMKey::kXHeight,
            30 => AFMKey::kAscender,
            31 => AFMKey::kDescender,
            32 => AFMKey::kWeightVector,
            33 => AFMKey::kBlendDesignPositions,
            34 => AFMKey::kBlendDesignMap,
            35 => AFMKey::kBlendAxisTypes,
            36 => AFMKey::kStartDirection,
            37 => AFMKey::kEndDirection,
            38 => AFMKey::kUnderlinePosition,
            39 => AFMKey::kUnderlineThickness,
            40 => AFMKey::kItalicAngle,
            41 => AFMKey::kCharWidth,
            42 => AFMKey::kIsFixedPitch,
            43 => AFMKey::kStartCharMetrics,
            44 => AFMKey::kEndCharMetrics,
            45 => AFMKey::kC,
            46 => AFMKey::kCH,
            47 => AFMKey::kWX,
            48 => AFMKey::kW0X,
            49 => AFMKey::kW1X,
            50 => AFMKey::kWY,
            51 => AFMKey::kW0Y,
            52 => AFMKey::kW1Y,
            53 => AFMKey::kW,
            54 => AFMKey::kW0,
            55 => AFMKey::kW1,
            56 => AFMKey::kVV,
            57 => AFMKey::kN,
            58 => AFMKey::kB,
            59 => AFMKey::kL,
            60 => AFMKey::kStartKernData,
            61 => AFMKey::kEndKernData,
            62 => AFMKey::kStartTrackKern,
            63 => AFMKey::kEndTrackKern,
            64 => AFMKey::kTrackKern,
            65 => AFMKey::kStartKernPairs,
            66 => AFMKey::kEndKernPairs,
            67 => AFMKey::kKP,
            68 => AFMKey::kKPH,
            69 => AFMKey::kKPX,
            70 => AFMKey::kKPY,
            71 => AFMKey::kStartComposites,
            72 => AFMKey::kEndComposites,
            73 => AFMKey::kCC,
            74 => AFMKey::kPCC,
            75 => AFMKey::kStartAxis,
            76 => AFMKey::kEndAxis,
            77 => AFMKey::kAxisType,
            78 => AFMKey::kAxisLabel,
            79 => AFMKey::kStartMaster,
            80 => AFMKey::kEndMaster,
            _ => panic!("Invalid value for AFMKey: {}", value),
        }
    }
}
impl AddAssign<u32> for AFMKey {
    fn add_assign(&mut self, rhs: u32) {
        *self = AFMKey::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for AFMKey {
    fn sub_assign(&mut self, rhs: u32) {
        *self = AFMKey::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for AFMKey {
    fn mul_assign(&mut self, rhs: u32) {
        *self = AFMKey::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for AFMKey {
    fn div_assign(&mut self, rhs: u32) {
        *self = AFMKey::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for AFMKey {
    fn rem_assign(&mut self, rhs: u32) {
        *self = AFMKey::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for AFMKey {
    type Output = AFMKey;
    fn add(self, rhs: u32) -> AFMKey {
        AFMKey::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for AFMKey {
    type Output = AFMKey;
    fn sub(self, rhs: u32) -> AFMKey {
        AFMKey::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for AFMKey {
    type Output = AFMKey;
    fn mul(self, rhs: u32) -> AFMKey {
        AFMKey::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for AFMKey {
    type Output = AFMKey;
    fn div(self, rhs: u32) -> AFMKey {
        AFMKey::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for AFMKey {
    type Output = AFMKey;
    fn rem(self, rhs: u32) -> AFMKey {
        AFMKey::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type ParseCtx = parse_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parse_ctx_st {
    pub fp: *mut FILE,
    pub token: [i8; 1024],
    pub tokenlen: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keyname_st {
    pub name: *mut i8,
    pub key: AFMKey,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const i8) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut i8);
}
static mut keynames: [keyname_st; 82] = [
    {
        let mut init = keyname_st {
            name: b"Ascender\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kAscender,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Axes\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kAxes,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"AxisLabel\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kAxisLabel,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"AxisType\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kAxisType,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"B\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kB,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"BlendAxisTypes\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kBlendAxisTypes,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"BlendDesignMap\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kBlendDesignMap,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"BlendDesignPositions\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kBlendDesignPositions,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"C\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kC,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"CC\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kCC,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"CH\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kCH,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"CapHeight\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kCapHeight,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"CharWidth\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kCharWidth,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"CharacterSet\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kCharacterSet,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Characters\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kCharacters,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Comment\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kComment,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Descendents\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kDescendents,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Descender\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kDescender,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EncodingScheme\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEncodingScheme,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndAxis\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEndAxis,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndCharMetrics\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEndCharMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndCompFontMetrics\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEndCompFontMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndComposites\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEndComposites,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndDescendent\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEndDescendent,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndDirection\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEndDirection,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndFontMetrics\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEndFontMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndKernData\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEndKernData,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndKernPairs\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEndKernPairs,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndMaster\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEndMaster,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndMasterFontMetrics\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEndMasterFontMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndTrackKern\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEndTrackKern,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EscChar\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kEscChar,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"FamilyName\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kFamilyName,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"FontBBox\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kFontBBox,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"FontName\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kFontName,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"FullName\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kFullName,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"IsBaseFont\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kIsBaseFont,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"IsFixedPitch\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kIsFixedPitch,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"IsFixedV\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kIsFixedV,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"ItalicAngle\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kItalicAngle,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"KP\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kKP,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"KPH\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kKPH,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"KPX\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kKPX,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"KPY\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kKPY,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"L\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kL,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"MappingScheme\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kMappingScheme,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Masters\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kMasters,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"MetricsSets\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kMetricsSets,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"N\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kN,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Notice\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kNotice,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"PCC\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kPCC,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartAxis\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kStartAxis,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartCharMetrics\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kStartCharMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartCompFontMetrics\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kStartCompFontMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartComposites\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kStartComposites,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartDescendent\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kStartDescendent,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartDirection\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kStartDirection,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartFontMetrics\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kStartFontMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartKernData\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kStartKernData,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartKernPairs\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kStartKernPairs,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartMaster\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kStartMaster,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartMasterFontMetrics\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kStartMasterFontMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartTrackKern\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kStartTrackKern,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"TrackKern\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kTrackKern,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"UnderlinePosition\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kUnderlinePosition,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"UnderlineThickness\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kUnderlineThickness,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"VV\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kVV,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"VVector\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kVVector,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Version\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kVersion,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kW,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W0\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kW0,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W0X\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kW0X,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W0Y\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kW0Y,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W1\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kW1,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W1X\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kW1X,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W1Y\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kW1Y,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"WX\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kWX,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"WY\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kWY,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Weight\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kWeight,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"WeightVector\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kWeightVector,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"XHeight\0" as *const u8 as *const i8 as *mut i8,
            key: AFMKey::kXHeight,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: 0 as *const i8 as *mut i8,
            key: AFMKey::kComment,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn afm_parse_file(
    mut handle: AFMHandle,
    mut filename: *const i8,
    mut font: AFMFont,
) {
    let mut key: AFMKey = AFMKey::kComment;
    let mut node: AFMNode = AFMNode {
        type_0: 0,
        u: C2RustUnnamed {
            string: 0 as *mut i8,
        },
    };
    let mut context: ParseCtx = ParseCtx {
        fp: 0 as *mut FILE,
        token: [0; 1024],
        tokenlen: 0,
    };
    let mut ctx: *mut ParseCtx = &mut context;
    let mut wd: i32 = 0 as i32;
    let mut done: i32 = 0 as i32;
    (*ctx).fp = fopen(filename, b"r\0" as *const u8 as *const i8);
    if ((*ctx).fp).is_null() {
        parse_error(handle, (*__errno_location() << 16 as i32 | 7 as i32) as AFMError);
    }
    get_key(handle, ctx, &mut key);
    if key as u32 != AFMKey::kStartFontMetrics as i32 as u32 {
        parse_error(handle, 8 as i32 as AFMError);
    }
    get_type(handle, ctx, 3 as i32, &mut node);
    (*font).version = node.u.number;
    while done == 0 {
        get_key(handle, ctx, &mut key);
        match key as u32 {
            0 => {
                get_line_token(handle, ctx);
            }
            1 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).version = node.u.number;
            }
            2 => {
                done = 1 as i32;
            }
            3 | 4 | 7 | 8 => {
                parse_error(handle, 6 as i32 as AFMError);
            }
            13 => {
                get_type(handle, ctx, 1 as i32, &mut node);
                (*font).global_info.FontName = node.u.string;
            }
            14 => {
                get_type(handle, ctx, 1 as i32, &mut node);
                (*font).global_info.FullName = node.u.string;
            }
            15 => {
                get_type(handle, ctx, 1 as i32, &mut node);
                (*font).global_info.FamilyName = node.u.string;
            }
            16 => {
                get_type(handle, ctx, 1 as i32, &mut node);
                (*font).global_info.Weight = node.u.string;
            }
            17 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).global_info.FontBBox_llx = node.u.number;
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).global_info.FontBBox_lly = node.u.number;
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).global_info.FontBBox_urx = node.u.number;
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).global_info.FontBBox_ury = node.u.number;
            }
            18 => {
                get_type(handle, ctx, 1 as i32, &mut node);
                (*font).global_info.Version = node.u.string;
            }
            19 => {
                get_type(handle, ctx, 1 as i32, &mut node);
                (*font).global_info.Notice = node.u.string;
            }
            20 => {
                get_type(handle, ctx, 1 as i32, &mut node);
                (*font).global_info.EncodingScheme = node.u.string;
            }
            21 => {
                get_type(handle, ctx, 4 as i32, &mut node);
                (*font).global_info.MappingScheme = node.u.integer;
            }
            22 => {
                get_type(handle, ctx, 4 as i32, &mut node);
                (*font).global_info.EscChar = node.u.integer;
            }
            23 => {
                get_type(handle, ctx, 1 as i32, &mut node);
                (*font).global_info.CharacterSet = node.u.string;
            }
            24 => {
                get_type(handle, ctx, 4 as i32, &mut node);
                (*font).global_info.Characters = node.u.integer;
            }
            25 => {
                get_type(handle, ctx, 6 as i32, &mut node);
                (*font).global_info.IsBaseFont = node.u.boolean;
            }
            26 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).global_info.VVector_0 = node.u.number;
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).global_info.VVector_1 = node.u.number;
            }
            27 => {
                get_type(handle, ctx, 6 as i32, &mut node);
                (*font).global_info.IsFixedV = node.u.boolean;
            }
            28 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).global_info.CapHeight = node.u.number;
            }
            29 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).global_info.XHeight = node.u.number;
            }
            30 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).global_info.Ascender = node.u.number;
            }
            31 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).global_info.Descender = node.u.number;
            }
            36 => {
                get_type(handle, ctx, 4 as i32, &mut node);
                wd = node.u.integer as i32;
                (*font).writing_direction_metrics[wd as usize].is_valid = AFMBoolean::AFMTrue;
            }
            38 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).writing_direction_metrics[wd as usize].UnderlinePosition = node
                    .u
                    .number;
            }
            39 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).writing_direction_metrics[wd as usize].UnderlineThickness = node
                    .u
                    .number;
            }
            40 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).writing_direction_metrics[wd as usize].ItalicAngle = node
                    .u
                    .number;
            }
            41 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).writing_direction_metrics[wd as usize].CharWidth_x = node
                    .u
                    .number;
                get_type(handle, ctx, 3 as i32, &mut node);
                (*font).writing_direction_metrics[wd as usize].CharWidth_y = node
                    .u
                    .number;
            }
            42 => {
                get_type(handle, ctx, 6 as i32, &mut node);
                (*font).writing_direction_metrics[wd as usize].IsFixedPitch = node
                    .u
                    .boolean;
            }
            43 => {
                get_type(handle, ctx, 4 as i32, &mut node);
                (*font).num_character_metrics = node.u.integer;
                (*font).character_metrics = calloc(
                    ((*font).num_character_metrics + 1 as i32 as i64) as u64,
                    ::core::mem::size_of::<AFMIndividualCharacterMetrics>() as u64,
                ) as *mut AFMIndividualCharacterMetrics;
                if ((*font).character_metrics).is_null() {
                    parse_error(handle, 2 as i32 as AFMError);
                }
                read_character_metrics(handle, ctx, font);
            }
            65 => {
                if (*font).info_level & 0x2 as i32 as u32 != 0 {
                    get_type(handle, ctx, 4 as i32, &mut node);
                    (*font).num_kern_pairs = node.u.integer;
                    (*font).kern_pairs = calloc(
                        ((*font).num_kern_pairs + 1 as i32 as i64) as u64,
                        ::core::mem::size_of::<AFMPairWiseKerning>() as u64,
                    ) as *mut AFMPairWiseKerning;
                    if ((*font).kern_pairs).is_null() {
                        parse_error(handle, 2 as i32 as AFMError);
                    }
                    read_kern_pairs(handle, ctx, font);
                } else {
                    loop {
                        get_line_token(handle, ctx);
                        get_key(handle, ctx, &mut key);
                        if !(key as u32 != AFMKey::kEndKernPairs as i32 as u32) {
                            break;
                        }
                    }
                }
            }
            62 => {
                if (*font).info_level & 0x4 as i32 as u32 != 0 {
                    get_type(handle, ctx, 4 as i32, &mut node);
                    (*font).num_track_kerns = node.u.integer;
                    (*font).track_kerns = calloc(
                        ((*font).num_track_kerns + 1 as i32 as i64) as u64,
                        ::core::mem::size_of::<AFMTrackKern>() as u64,
                    ) as *mut AFMTrackKern;
                    if ((*font).track_kerns).is_null() {
                        parse_error(handle, 2 as i32 as AFMError);
                    }
                    read_track_kerns(handle, ctx, font);
                } else {
                    loop {
                        get_line_token(handle, ctx);
                        get_key(handle, ctx, &mut key);
                        if !(key as u32 != AFMKey::kEndTrackKern as i32 as u32) {
                            break;
                        }
                    }
                }
            }
            37 | 60 | 61 => {}
            71 => {
                if (*font).info_level & 0x1 as i32 as u32 != 0 {
                    get_type(handle, ctx, 4 as i32, &mut node);
                    (*font).num_composites = node.u.integer;
                    (*font).composites = calloc(
                        ((*font).num_composites + 1 as i32 as i64) as u64,
                        ::core::mem::size_of::<AFMComposite>() as u64,
                    ) as *mut AFMComposite;
                    if ((*font).composites).is_null() {
                        parse_error(handle, 2 as i32 as AFMError);
                    }
                    read_composites(handle, ctx, font);
                } else {
                    loop {
                        get_line_token(handle, ctx);
                        get_key(handle, ctx, &mut key);
                        if !(key as u32 != AFMKey::kEndComposites as i32 as u32) {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    fclose((*ctx).fp);
    if (*font).writing_direction_metrics[0 as i32 as usize].is_valid as u64 == 0
        && (*font).writing_direction_metrics[1 as i32 as usize].is_valid as u64 == 0
    {
        (*font).writing_direction_metrics[0 as i32 as usize].is_valid = AFMBoolean::AFMTrue;
    }
    if strhash_get(
        (*(*font).private).fontnames,
        b"space\0" as *const u8 as *const i8,
        5 as i32,
        (*(*font).private).undef as *mut libc::c_void as *mut *mut libc::c_void,
    ) == 0
    {
        if (*font).num_character_metrics > 0 as i32 as i64 {} else {
            __assert_fail(
                b"font->num_character_metrics > 0\0" as *const u8 as *const i8,
                b"afmparse.c\0" as *const u8 as *const i8,
                481 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[i8; 54],
                >(b"void afm_parse_file(AFMHandle, const char *, AFMFont)\0"))
                    .as_ptr(),
            );
        }
        'c_3699: {
            if (*font).num_character_metrics > 0 as i32 as i64 {} else {
                __assert_fail(
                    b"font->num_character_metrics > 0\0" as *const u8 as *const i8,
                    b"afmparse.c\0" as *const u8 as *const i8,
                    481 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 54],
                        &[i8; 54],
                    >(b"void afm_parse_file(AFMHandle, const char *, AFMFont)\0"))
                        .as_ptr(),
                );
            }
        };
        (*(*font).private).undef = &mut *((*font).character_metrics)
            .offset(0 as i32 as isize) as *mut AFMIndividualCharacterMetrics;
    }
    if (*font).writing_direction_metrics[0 as i32 as usize].is_valid as u32 != 0
        && (*font).writing_direction_metrics[0 as i32 as usize].IsFixedPitch as u32 != 0
    {
        (*font).writing_direction_metrics[0 as i32 as usize].CharWidth_x = (*((*font)
            .character_metrics)
            .offset(0 as i32 as isize))
            .w0x;
        (*font).writing_direction_metrics[0 as i32 as usize].CharWidth_y = (*((*font)
            .character_metrics)
            .offset(0 as i32 as isize))
            .w0y;
    }
    if (*font).writing_direction_metrics[1 as i32 as usize].is_valid as u32 != 0
        && (*font).writing_direction_metrics[1 as i32 as usize].IsFixedPitch as u32 != 0
    {
        (*font).writing_direction_metrics[1 as i32 as usize].CharWidth_x = (*((*font)
            .character_metrics)
            .offset(1 as i32 as isize))
            .w1x;
        (*font).writing_direction_metrics[1 as i32 as usize].CharWidth_y = (*((*font)
            .character_metrics)
            .offset(1 as i32 as isize))
            .w1y;
    }
}
unsafe extern "C" fn parse_error(mut handle: AFMHandle, mut error: AFMError) {
    (*handle).parse_error = error;
    longjmp(((*handle).jmpbuf).as_mut_ptr(), 1 as i32);
}
unsafe extern "C" fn get_token(mut handle: AFMHandle, mut ctx: *mut ParseCtx) -> i32 {
    let mut ch: i32 = 0;
    let mut i: i32 = 0;
    loop {
        ch = _IO_getc((*ctx).fp);
        if !(ch != -(1 as i32)) {
            break;
        }
        if !(ch == ' ' as i32 || ch == '\n' as i32 || ch == '\r' as i32
            || ch == '\t' as i32 || ch == ';' as i32)
        {
            break;
        }
    }
    if ch == -(1 as i32) {
        return 0 as i32;
    }
    ungetc(ch, (*ctx).fp);
    i = 0 as i32;
    ch = _IO_getc((*ctx).fp);
    while (i as u64) < ::core::mem::size_of::<[i8; 1024]>() as u64 && ch != -(1 as i32)
        && !(ch == ' ' as i32 || ch == '\n' as i32 || ch == '\r' as i32
            || ch == '\t' as i32 || ch == ';' as i32)
    {
        (*ctx).token[i as usize] = ch as i8;
        i += 1;
        i;
        ch = _IO_getc((*ctx).fp);
    }
    if i as u64 >= ::core::mem::size_of::<[i8; 1024]>() as u64 {
        parse_error(handle, 5 as i32 as AFMError);
    }
    (*ctx).token[i as usize] = '\0' as i32 as i8;
    (*ctx).tokenlen = i as u32;
    return 1 as i32;
}
unsafe extern "C" fn get_line_token(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
) -> i32 {
    let mut i: i32 = 0;
    let mut ch: i32 = 0;
    loop {
        ch = _IO_getc((*ctx).fp);
        if !(ch != -(1 as i32)) {
            break;
        }
        if !(ch == ' ' as i32 || ch == '\n' as i32 || ch == '\r' as i32
            || ch == '\t' as i32 || ch == ';' as i32)
        {
            break;
        }
    }
    if ch == -(1 as i32) {
        return 0 as i32;
    }
    ungetc(ch, (*ctx).fp);
    i = 0 as i32;
    ch = _IO_getc((*ctx).fp);
    while (i as u64) < ::core::mem::size_of::<[i8; 1024]>() as u64 && ch != -(1 as i32)
        && ch != '\n' as i32
    {
        (*ctx).token[i as usize] = ch as i8;
        i += 1;
        i;
        ch = _IO_getc((*ctx).fp);
    }
    if i as u64 >= ::core::mem::size_of::<[i8; 1024]>() as u64 {
        parse_error(handle, 5 as i32 as AFMError);
    }
    i -= 1;
    i;
    while i >= 0 as i32
        && ((*ctx).token[i as usize] as i32 == ' ' as i32
            || (*ctx).token[i as usize] as i32 == '\n' as i32
            || (*ctx).token[i as usize] as i32 == '\r' as i32
            || (*ctx).token[i as usize] as i32 == '\t' as i32
            || (*ctx).token[i as usize] as i32 == ';' as i32)
    {
        i -= 1;
        i;
    }
    i += 1;
    i;
    (*ctx).token[i as usize] = '\0' as i32 as i8;
    (*ctx).tokenlen = i as u32;
    return 1 as i32;
}
unsafe extern "C" fn match_key(mut key: *mut i8) -> i32 {
    let mut lower: i32 = 0 as i32;
    let mut upper: i32 = (::core::mem::size_of::<[keyname_st; 82]>() as u64)
        .wrapping_div(::core::mem::size_of::<keyname_st>() as u64)
        .wrapping_sub(1 as i32 as u64) as i32;
    let mut midpoint: i32 = 0;
    let mut cmpvalue: i32 = 0;
    let mut found: AFMBoolean = AFMBoolean::AFMFalse;
    while upper >= lower && found as u64 == 0 {
        midpoint = (lower + upper) / 2 as i32;
        if (keynames[midpoint as usize].name).is_null() {
            break;
        }
        cmpvalue = strcmp(key, keynames[midpoint as usize].name);
        if cmpvalue == 0 as i32 {
            found = AFMBoolean::AFMTrue;
        } else if cmpvalue < 0 as i32 {
            upper = midpoint - 1 as i32;
        } else {
            lower = midpoint + 1 as i32;
        }
    }
    if found as u64 != 0 {
        return keynames[midpoint as usize].key as i32;
    }
    return -(1 as i32);
}
unsafe extern "C" fn get_key(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
    mut key_return: *mut AFMKey,
) {
    let mut key: i32 = 0;
    let mut msg: [i8; 256] = [0; 256];
    loop {
        if get_token(handle, ctx) == 0 {
            parse_error(handle, 5 as i32 as AFMError);
        }
        key = match_key(((*ctx).token).as_mut_ptr());
        if key >= 0 as i32 {
            *key_return = AFMKey::from_libc_c_uint(key as u32);
            return;
        }
        sprintf(
            msg.as_mut_ptr(),
            b"skipping key \"%s\"\0" as *const u8 as *const i8,
            ((*ctx).token).as_mut_ptr(),
        );
        afm_error(handle, msg.as_mut_ptr());
        get_line_token(handle, ctx);
    };
}
unsafe extern "C" fn get_type(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
    mut type_0: i32,
    mut type_return: *mut AFMNode,
) {
    let mut buf: [i8; 256] = [0; 256];
    match type_0 {
        1 => {
            if get_line_token(handle, ctx) == 0 {
                parse_error(handle, 5 as i32 as AFMError);
            }
            (*type_return).u.string = calloc(
                1 as i32 as u64,
                ((*ctx).tokenlen).wrapping_add(1 as i32 as u32) as u64,
            ) as AFMString;
            if ((*type_return).u.string).is_null() {
                parse_error(handle, 2 as i32 as AFMError);
            }
            memcpy(
                (*type_return).u.string as *mut libc::c_void,
                ((*ctx).token).as_mut_ptr() as *const libc::c_void,
                (*ctx).tokenlen as u64,
            );
        }
        2 => {
            if get_token(handle, ctx) == 0 {
                parse_error(handle, 5 as i32 as AFMError);
            }
            (*type_return).u.name = calloc(
                1 as i32 as u64,
                ((*ctx).tokenlen).wrapping_add(1 as i32 as u32) as u64,
            ) as AFMName;
            if ((*type_return).u.string).is_null() {
                parse_error(handle, 2 as i32 as AFMError);
            }
            memcpy(
                (*type_return).u.name as *mut libc::c_void,
                ((*ctx).token).as_mut_ptr() as *const libc::c_void,
                (*ctx).tokenlen as u64,
            );
        }
        3 => {
            if get_token(handle, ctx) == 0 {
                parse_error(handle, 5 as i32 as AFMError);
            }
            memcpy(
                buf.as_mut_ptr() as *mut libc::c_void,
                ((*ctx).token).as_mut_ptr() as *const libc::c_void,
                (*ctx).tokenlen as u64,
            );
            buf[(*ctx).tokenlen as usize] = '\0' as i32 as i8;
            (*type_return).u.number = atof(buf.as_mut_ptr());
        }
        4 => {
            if get_token(handle, ctx) == 0 {
                parse_error(handle, 5 as i32 as AFMError);
            }
            memcpy(
                buf.as_mut_ptr() as *mut libc::c_void,
                ((*ctx).token).as_mut_ptr() as *const libc::c_void,
                (*ctx).tokenlen as u64,
            );
            buf[(*ctx).tokenlen as usize] = '\0' as i32 as i8;
            (*type_return).u.integer = atoi(buf.as_mut_ptr()) as AFMInteger;
        }
        5 => {
            fprintf(
                stderr,
                b"Array types not implemented yet.\n\0" as *const u8 as *const i8,
            );
            abort();
        }
        6 => {
            if get_token(handle, ctx) == 0 {
                parse_error(handle, 5 as i32 as AFMError);
            }
            memcpy(
                buf.as_mut_ptr() as *mut libc::c_void,
                ((*ctx).token).as_mut_ptr() as *const libc::c_void,
                (*ctx).tokenlen as u64,
            );
            buf[(*ctx).tokenlen as usize] = '\0' as i32 as i8;
            if strcmp(buf.as_mut_ptr(), b"true\0" as *const u8 as *const i8) == 0 as i32
            {
                (*type_return).u.boolean = AFMBoolean::AFMTrue;
            } else if strcmp(buf.as_mut_ptr(), b"false\0" as *const u8 as *const i8)
                == 0 as i32
            {
                (*type_return).u.boolean = AFMBoolean::AFMFalse;
            } else {
                parse_error(handle, 5 as i32 as AFMError);
            }
        }
        _ => {
            fprintf(
                stderr,
                b"get_type(): illegal type %d\n\0" as *const u8 as *const i8,
                (*type_return).type_0,
            );
            abort();
        }
    };
}
unsafe extern "C" fn read_character_metrics(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
    mut font: AFMFont,
) {
    let mut i: i32 = 0 as i32;
    let mut node: AFMNode = AFMNode {
        type_0: 0,
        u: C2RustUnnamed {
            string: 0 as *mut i8,
        },
    };
    let mut cm: *mut AFMIndividualCharacterMetrics = 0
        as *mut AFMIndividualCharacterMetrics;
    let mut key: AFMKey = AFMKey::kComment;
    let mut done: i32 = 0 as i32;
    let mut first: i32 = 1 as i32;
    while done == 0 {
        get_key(handle, ctx, &mut key);
        match key as u32 {
            45 => {
                if first != 0 {
                    first = 0 as i32;
                } else {
                    i += 1;
                    i;
                }
                if i as i64 >= (*font).num_character_metrics {
                    parse_error(handle, 5 as i32 as AFMError);
                }
                cm = &mut *((*font).character_metrics).offset(i as isize)
                    as *mut AFMIndividualCharacterMetrics;
                get_type(handle, ctx, 4 as i32, &mut node);
                (*cm).character_code = node.u.integer;
                if (*cm).character_code >= 0 as i32 as i64
                    && (*cm).character_code <= 255 as i32 as i64
                {
                    (*font).encoding[(*cm).character_code as usize] = cm;
                }
            }
            46 => {
                printf(b"* CH\n\0" as *const u8 as *const i8);
            }
            47 | 48 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).w0x = node.u.number;
                (*cm).w0y = 0.0f64;
            }
            49 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).w1x = node.u.number;
                (*cm).w1y = 0.0f64;
            }
            50 | 51 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).w0y = node.u.number;
                (*cm).w0x = 0.0f64;
            }
            52 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).w1y = node.u.number;
                (*cm).w1x = 0.0f64;
            }
            53 | 54 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).w0x = node.u.number;
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).w0y = node.u.number;
            }
            55 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).w1x = node.u.number;
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).w1y = node.u.number;
            }
            56 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).vv_x = node.u.number;
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).vv_y = node.u.number;
            }
            57 => {
                get_type(handle, ctx, 2 as i32, &mut node);
                (*cm).name = node.u.name;
                if strhash_put(
                    (*(*font).private).fontnames,
                    (*cm).name,
                    strlen((*cm).name as *const i8) as i32,
                    cm as *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                ) == 0
                {
                    parse_error(handle, 2 as i32 as AFMError);
                }
            }
            58 => {
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).llx = node.u.number;
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).lly = node.u.number;
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).urx = node.u.number;
                get_type(handle, ctx, 3 as i32, &mut node);
                (*cm).ury = node.u.number;
            }
            59 => {
                get_line_token(handle, ctx);
            }
            44 => {
                if i as i64 != (*font).num_character_metrics - 1 as i32 as i64 {
                    (*font).num_character_metrics = (i + 1 as i32) as AFMInteger;
                }
                done = 1 as i32;
            }
            _ => {
                parse_error(handle, 5 as i32 as AFMError);
            }
        }
    }
}
unsafe extern "C" fn read_kern_pairs(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
    mut font: AFMFont,
) {
    let mut i: i32 = 0;
    let mut node: AFMNode = AFMNode {
        type_0: 0,
        u: C2RustUnnamed {
            string: 0 as *mut i8,
        },
    };
    let mut kp: *mut AFMPairWiseKerning = 0 as *mut AFMPairWiseKerning;
    let mut key: AFMKey = AFMKey::kComment;
    i = 0 as i32;
    while (i as i64) < (*font).num_kern_pairs {
        kp = &mut *((*font).kern_pairs).offset(i as isize) as *mut AFMPairWiseKerning;
        get_key(handle, ctx, &mut key);
        match key as u32 {
            67 | 69 | 70 => {
                get_type(handle, ctx, 2 as i32, &mut node);
                (*kp).name1 = node.u.name;
                get_type(handle, ctx, 2 as i32, &mut node);
                (*kp).name2 = node.u.name;
                get_type(handle, ctx, 3 as i32, &mut node);
                match key as u32 {
                    67 => {
                        (*kp).kx = node.u.number;
                        get_type(handle, ctx, 3 as i32, &mut node);
                        (*kp).ky = node.u.number;
                    }
                    69 => {
                        (*kp).kx = node.u.number;
                        (*kp).ky = 0.0f64;
                    }
                    70 => {
                        (*kp).ky = node.u.number;
                        (*kp).kx = 0.0f64;
                    }
                    _ => {
                        fprintf(
                            stderr,
                            b"AFM: fatal corruption\n\0" as *const u8 as *const i8,
                        );
                        abort();
                    }
                }
            }
            68 => {}
            _ => {
                parse_error(handle, 5 as i32 as AFMError);
            }
        }
        i += 1;
        i;
    }
    get_key(handle, ctx, &mut key);
    if key as u32 != AFMKey::kEndKernPairs as i32 as u32 {
        parse_error(handle, 5 as i32 as AFMError);
    }
}
unsafe extern "C" fn read_track_kerns(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
    mut font: AFMFont,
) {
    let mut i: i32 = 0;
    let mut node: AFMNode = AFMNode {
        type_0: 0,
        u: C2RustUnnamed {
            string: 0 as *mut i8,
        },
    };
    let mut tk: *mut AFMTrackKern = 0 as *mut AFMTrackKern;
    let mut key: AFMKey = AFMKey::kComment;
    i = 0 as i32;
    while (i as i64) < (*font).num_kern_pairs {
        tk = &mut *((*font).track_kerns).offset(i as isize) as *mut AFMTrackKern;
        get_key(handle, ctx, &mut key);
        if key as u32 != AFMKey::kTrackKern as i32 as u32 {
            parse_error(handle, 5 as i32 as AFMError);
        }
        get_type(handle, ctx, 4 as i32, &mut node);
        (*tk).degree = node.u.integer;
        get_type(handle, ctx, 3 as i32, &mut node);
        (*tk).min_ptsize = node.u.number;
        get_type(handle, ctx, 3 as i32, &mut node);
        (*tk).min_kern = node.u.number;
        get_type(handle, ctx, 3 as i32, &mut node);
        (*tk).max_ptsize = node.u.number;
        get_type(handle, ctx, 3 as i32, &mut node);
        (*tk).max_kern = node.u.number;
        i += 1;
        i;
    }
    get_key(handle, ctx, &mut key);
    if key as u32 != AFMKey::kEndTrackKern as i32 as u32 {
        parse_error(handle, 5 as i32 as AFMError);
    }
}
unsafe extern "C" fn read_composites(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
    mut font: AFMFont,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut node: AFMNode = AFMNode {
        type_0: 0,
        u: C2RustUnnamed {
            string: 0 as *mut i8,
        },
    };
    let mut cm: *mut AFMComposite = 0 as *mut AFMComposite;
    let mut key: AFMKey = AFMKey::kComment;
    i = 0 as i32;
    while (i as i64) < (*font).num_composites {
        cm = &mut *((*font).composites).offset(i as isize) as *mut AFMComposite;
        get_key(handle, ctx, &mut key);
        if key as u32 != AFMKey::kCC as i32 as u32 {
            parse_error(handle, 5 as i32 as AFMError);
        }
        get_type(handle, ctx, 2 as i32, &mut node);
        (*cm).name = node.u.name;
        if strhash_put(
            (*(*font).private).compositenames,
            (*cm).name,
            strlen((*cm).name as *const i8) as i32,
            cm as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        ) == 0
        {
            parse_error(handle, 2 as i32 as AFMError);
        }
        get_type(handle, ctx, 4 as i32, &mut node);
        (*cm).num_components = node.u.integer;
        (*cm).components = calloc(
            ((*cm).num_components + 1 as i32 as i64) as u64,
            ::core::mem::size_of::<AFMCompositeComponent>() as u64,
        ) as *mut AFMCompositeComponent;
        j = 0 as i32;
        while (j as i64) < (*cm).num_components {
            get_key(handle, ctx, &mut key);
            if key as u32 != AFMKey::kPCC as i32 as u32 {
                parse_error(handle, 5 as i32 as AFMError);
            }
            get_type(handle, ctx, 2 as i32, &mut node);
            let ref mut fresh0 = (*((*cm).components).offset(j as isize)).name;
            *fresh0 = node.u.name;
            get_type(handle, ctx, 3 as i32, &mut node);
            (*((*cm).components).offset(j as isize)).deltax = node.u.number;
            get_type(handle, ctx, 3 as i32, &mut node);
            (*((*cm).components).offset(j as isize)).deltay = node.u.number;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    get_key(handle, ctx, &mut key);
    if key as u32 != AFMKey::kEndComposites as i32 as u32 {
        parse_error(handle, 5 as i32 as AFMError);
    }
}