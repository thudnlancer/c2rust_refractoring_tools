use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type stringhash_st;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn afm_close_font(font: AFMFont) -> AFMError;
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut i8,
        keylen: i32,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> i32;
    fn strhash_init() -> StringHashPtr;
}
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
pub type AFMEncodingTable = encoding_table_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encoding_table_st {
    pub code: i32,
    pub character: *mut i8,
}
static mut builtin_courier: [AFMEncodingTable; 261] = [
    {
        let mut init = encoding_table_st {
            code: 32 as i32,
            character: b"space\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 33 as i32,
            character: b"exclam\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 34 as i32,
            character: b"quotedbl\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 35 as i32,
            character: b"numbersign\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 36 as i32,
            character: b"dollar\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 37 as i32,
            character: b"percent\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 38 as i32,
            character: b"ampersand\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 39 as i32,
            character: b"quoteright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 40 as i32,
            character: b"parenleft\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 41 as i32,
            character: b"parenright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 42 as i32,
            character: b"asterisk\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 43 as i32,
            character: b"plus\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 44 as i32,
            character: b"comma\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 45 as i32,
            character: b"hyphen\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 46 as i32,
            character: b"period\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 47 as i32,
            character: b"slash\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 48 as i32,
            character: b"zero\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 49 as i32,
            character: b"one\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 50 as i32,
            character: b"two\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 51 as i32,
            character: b"three\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 52 as i32,
            character: b"four\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 53 as i32,
            character: b"five\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 54 as i32,
            character: b"six\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 55 as i32,
            character: b"seven\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 56 as i32,
            character: b"eight\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 57 as i32,
            character: b"nine\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 58 as i32,
            character: b"colon\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 59 as i32,
            character: b"semicolon\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 60 as i32,
            character: b"less\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 61 as i32,
            character: b"equal\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 62 as i32,
            character: b"greater\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 63 as i32,
            character: b"question\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 64 as i32,
            character: b"at\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 65 as i32,
            character: b"A\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 66 as i32,
            character: b"B\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 67 as i32,
            character: b"C\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 68 as i32,
            character: b"D\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 69 as i32,
            character: b"E\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 70 as i32,
            character: b"F\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 71 as i32,
            character: b"G\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 72 as i32,
            character: b"H\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 73 as i32,
            character: b"I\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 74 as i32,
            character: b"J\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 75 as i32,
            character: b"K\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 76 as i32,
            character: b"L\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 77 as i32,
            character: b"M\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 78 as i32,
            character: b"N\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 79 as i32,
            character: b"O\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 80 as i32,
            character: b"P\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 81 as i32,
            character: b"Q\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 82 as i32,
            character: b"R\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 83 as i32,
            character: b"S\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 84 as i32,
            character: b"T\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 85 as i32,
            character: b"U\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 86 as i32,
            character: b"V\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 87 as i32,
            character: b"W\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 88 as i32,
            character: b"X\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 89 as i32,
            character: b"Y\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 90 as i32,
            character: b"Z\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 91 as i32,
            character: b"bracketleft\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 92 as i32,
            character: b"backslash\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 93 as i32,
            character: b"bracketright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 94 as i32,
            character: b"asciicircum\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 95 as i32,
            character: b"underscore\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 96 as i32,
            character: b"quoteleft\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 97 as i32,
            character: b"a\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 98 as i32,
            character: b"b\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 99 as i32,
            character: b"c\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 100 as i32,
            character: b"d\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 101 as i32,
            character: b"e\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 102 as i32,
            character: b"f\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 103 as i32,
            character: b"g\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 104 as i32,
            character: b"h\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 105 as i32,
            character: b"i\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 106 as i32,
            character: b"j\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 107 as i32,
            character: b"k\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 108 as i32,
            character: b"l\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 109 as i32,
            character: b"m\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 110 as i32,
            character: b"n\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 111 as i32,
            character: b"o\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 112 as i32,
            character: b"p\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 113 as i32,
            character: b"q\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 114 as i32,
            character: b"r\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 115 as i32,
            character: b"s\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 116 as i32,
            character: b"t\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 117 as i32,
            character: b"u\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 118 as i32,
            character: b"v\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 119 as i32,
            character: b"w\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 120 as i32,
            character: b"x\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 121 as i32,
            character: b"y\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 122 as i32,
            character: b"z\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 123 as i32,
            character: b"braceleft\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 124 as i32,
            character: b"bar\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 125 as i32,
            character: b"braceright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 126 as i32,
            character: b"asciitilde\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 161 as i32,
            character: b"exclamdown\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 162 as i32,
            character: b"cent\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 163 as i32,
            character: b"sterling\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 164 as i32,
            character: b"fraction\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 165 as i32,
            character: b"yen\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 166 as i32,
            character: b"florin\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 167 as i32,
            character: b"section\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 168 as i32,
            character: b"currency\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 169 as i32,
            character: b"quotesingle\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 170 as i32,
            character: b"quotedblleft\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 171 as i32,
            character: b"guillemotleft\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 172 as i32,
            character: b"guilsinglleft\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 173 as i32,
            character: b"guilsinglright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 174 as i32,
            character: b"fi\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 175 as i32,
            character: b"fl\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 177 as i32,
            character: b"endash\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 178 as i32,
            character: b"dagger\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 179 as i32,
            character: b"daggerdbl\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 180 as i32,
            character: b"periodcentered\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 182 as i32,
            character: b"paragraph\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 183 as i32,
            character: b"bullet\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 184 as i32,
            character: b"quotesinglbase\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 185 as i32,
            character: b"quotedblbase\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 186 as i32,
            character: b"quotedblright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 187 as i32,
            character: b"guillemotright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 188 as i32,
            character: b"ellipsis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 189 as i32,
            character: b"perthousand\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 191 as i32,
            character: b"questiondown\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 193 as i32,
            character: b"grave\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 194 as i32,
            character: b"acute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 195 as i32,
            character: b"circumflex\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 196 as i32,
            character: b"tilde\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 197 as i32,
            character: b"macron\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 198 as i32,
            character: b"breve\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 199 as i32,
            character: b"dotaccent\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 200 as i32,
            character: b"dieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 202 as i32,
            character: b"ring\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 203 as i32,
            character: b"cedilla\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 205 as i32,
            character: b"hungarumlaut\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 206 as i32,
            character: b"ogonek\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 207 as i32,
            character: b"caron\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 208 as i32,
            character: b"emdash\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 225 as i32,
            character: b"AE\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 227 as i32,
            character: b"ordfeminine\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 232 as i32,
            character: b"Lslash\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 233 as i32,
            character: b"Oslash\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 234 as i32,
            character: b"OE\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 235 as i32,
            character: b"ordmasculine\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 241 as i32,
            character: b"ae\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 245 as i32,
            character: b"dotlessi\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 248 as i32,
            character: b"lslash\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 249 as i32,
            character: b"oslash\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 250 as i32,
            character: b"oe\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 251 as i32,
            character: b"germandbls\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Aacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Acircumflex\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Adieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Agrave\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Aring\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Atilde\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Ccedilla\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Eacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Ecircumflex\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Edieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Egrave\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Eth\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Gcaron\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"IJ\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Iacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Icircumflex\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Idieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Idot\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Igrave\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"LL\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Ntilde\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Oacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Ocircumflex\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Odieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Ograve\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Otilde\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Scaron\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Scedilla\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Thorn\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Uacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Ucircumflex\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Udieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Ugrave\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Yacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Ydieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"Zcaron\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"aacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"acircumflex\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"adieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"agrave\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"aring\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"arrowboth\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"arrowdown\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"arrowleft\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"arrowright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"arrowup\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"atilde\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"brokenbar\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"ccedilla\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"center\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"copyright\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"dectab\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"degree\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"divide\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"down\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"eacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"ecircumflex\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"edieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"egrave\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"eth\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"format\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"gcaron\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"graybox\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"iacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"icircumflex\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"idieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"igrave\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"ij\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"indent\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"largebullet\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"left\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"lira\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"ll\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"logicalnot\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"merge\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"minus\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"mu\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"multiply\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"notegraphic\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"ntilde\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"oacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"ocircumflex\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"odieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"ograve\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"onehalf\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"onequarter\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"onesuperior\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"otilde\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"overscore\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"plusminus\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"prescription\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"registered\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"return\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"scaron\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"scedilla\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"square\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"stop\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"tab\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"thorn\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"threequarters\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"threesuperior\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"trademark\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"twosuperior\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"uacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"ucircumflex\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"udieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"ugrave\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"up\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"yacute\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"ydieresis\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as i32),
            character: b"zcaron\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0 as i32,
            character: 0 as *const i8 as *mut i8,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn afm_open_default_font(
    mut handle: AFMHandle,
    mut font_return: *mut AFMFont,
) -> AFMError {
    let mut current_block: u64;
    let mut font: AFMFont = 0 as *mut afm_font_st;
    let mut cm: *mut AFMIndividualCharacterMetrics = 0
        as *mut AFMIndividualCharacterMetrics;
    let mut i: i32 = 0;
    font = calloc(1 as i32 as u64, ::core::mem::size_of::<afm_font_st>() as u64)
        as AFMFont;
    if !font.is_null() {
        (*font).private = calloc(
            1 as i32 as u64,
            ::core::mem::size_of::<afm_font_private_data_st>() as u64,
        ) as *mut afm_font_private_data_st;
        if !((*font).private).is_null() {
            (*(*font).private).fontnames = strhash_init();
            if !((*(*font).private).fontnames).is_null() {
                (*font).version = 4.0f64;
                (*font).global_info.FontName = malloc(
                    (strlen(b"Courier\0" as *const u8 as *const i8))
                        .wrapping_add(1 as i32 as u64),
                ) as *mut i8;
                if !((*font).global_info.FontName).is_null() {
                    strcpy(
                        (*font).global_info.FontName,
                        b"Courier\0" as *const u8 as *const i8,
                    );
                    (*font).global_info.FontBBox_llx = -40.0f64;
                    (*font).global_info.FontBBox_lly = -290.0f64;
                    (*font).global_info.FontBBox_urx = 640.0f64;
                    (*font).global_info.FontBBox_ury = 795.0f64;
                    (*font).writing_direction_metrics[0 as i32 as usize].is_valid = AFMBoolean::AFMTrue;
                    (*font).writing_direction_metrics[0 as i32 as usize].IsFixedPitch = AFMBoolean::AFMTrue;
                    (*font).writing_direction_metrics[0 as i32 as usize].CharWidth_x = 600.0f64;
                    (*font).writing_direction_metrics[0 as i32 as usize].CharWidth_y = 0.0f64;
                    (*font).num_character_metrics = (::core::mem::size_of::<
                        [AFMEncodingTable; 261],
                    >() as u64)
                        .wrapping_div(::core::mem::size_of::<AFMEncodingTable>() as u64)
                        .wrapping_sub(1 as i32 as u64) as AFMInteger;
                    (*font).character_metrics = calloc(
                        (::core::mem::size_of::<[AFMEncodingTable; 261]>() as u64)
                            .wrapping_div(
                                ::core::mem::size_of::<AFMEncodingTable>() as u64,
                            )
                            .wrapping_sub(1 as i32 as u64),
                        ::core::mem::size_of::<AFMIndividualCharacterMetrics>() as u64,
                    ) as *mut AFMIndividualCharacterMetrics;
                    if !((*font).character_metrics).is_null() {
                        i = 0 as i32;
                        loop {
                            if (builtin_courier[i as usize].character).is_null() {
                                current_block = 10652014663920648156;
                                break;
                            }
                            cm = &mut *((*font).character_metrics).offset(i as isize)
                                as *mut AFMIndividualCharacterMetrics;
                            (*cm).name = malloc(
                                (strlen(builtin_courier[i as usize].character))
                                    .wrapping_add(1 as i32 as u64),
                            ) as *mut i8;
                            if ((*cm).name).is_null() {
                                current_block = 7794662285553927151;
                                break;
                            }
                            strcpy((*cm).name, builtin_courier[i as usize].character);
                            if strhash_put(
                                (*(*font).private).fontnames,
                                (*cm).name,
                                strlen((*cm).name as *const i8) as i32,
                                cm as *mut libc::c_void,
                                0 as *mut *mut libc::c_void,
                            ) == 0
                            {
                                current_block = 7794662285553927151;
                                break;
                            }
                            (*cm).character_code = builtin_courier[i as usize].code
                                as AFMInteger;
                            (*cm).w0x = 600.0f64;
                            (*cm).w0y = 0.0f64;
                            i += 1;
                            i;
                        }
                        match current_block {
                            7794662285553927151 => {}
                            _ => {
                                *font_return = font;
                                return 0 as i32 as AFMError;
                            }
                        }
                    }
                }
            }
        }
    }
    afm_close_font(font);
    return 2 as i32 as AFMError;
}