use std::ffi::CString;
use std::ptr;
use std::mem;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}

pub type __jmp_buf = [libc::c_long; 8];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}

pub type jmp_buf = [__jmp_buf_tag; 1];
pub type AFMString = *mut libc::c_char;
pub type AFMName = *mut libc::c_char;
pub type AFMNumber = libc::c_double;
pub type AFMInteger = libc::c_long;
pub type AFMBoolean = libc::c_uint;

pub const AFMTrue: AFMBoolean = 1;
pub const AFMFalse: AFMBoolean = 0;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct afm_array_st {
    pub num_items: AFMNumber,
    pub items: *mut AFMNode,
}

pub type AFMNode = afm_node_st;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct afm_node_st {
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub union C2RustUnnamed {
    pub string: AFMString,
    pub name: AFMName,
    pub number: AFMNumber,
    pub integer: AFMInteger,
    pub array: AFMArray,
    pub boolean: AFMBoolean,
}

pub type AFMArray = *mut afm_array_st;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ligature_st {
    pub successor: AFMName,
    pub ligature: AFMName,
}

pub type AFMLigature = ligature_st;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct track_kern_st {
    pub degree: AFMInteger,
    pub min_ptsize: AFMNumber,
    pub min_kern: AFMNumber,
    pub max_ptsize: AFMNumber,
    pub max_kern: AFMNumber,
}

pub type AFMTrackKern = track_kern_st;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct pair_wise_kerning_st {
    pub name1: AFMName,
    pub name2: AFMName,
    pub kx: AFMNumber,
    pub ky: AFMNumber,
}

pub type AFMPairWiseKerning = pair_wise_kerning_st;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct composite_component_st {
    pub name: AFMName,
    pub deltax: AFMNumber,
    pub deltay: AFMNumber,
}

pub type AFMCompositeComponent = composite_component_st;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct composite_st {
    pub name: AFMName,
    pub num_components: AFMInteger,
    pub components: *mut AFMCompositeComponent,
}

pub type AFMComposite = composite_st;

pub type AFMError = libc::c_uint;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct afm_handle_st {
    pub verbose: libc::c_uint,
    pub font_map: StringHashPtr,
    pub jmpbuf: jmp_buf,
    pub parse_error: AFMError,
}

pub type StringHashPtr = *mut stringhash_st;
pub type AFMHandle = *mut afm_handle_st;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct afm_font_st {
    pub private: *mut afm_font_private_data_st,
    pub version: AFMNumber,
    pub info_level: libc::c_uint,
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct afm_font_private_data_st {
    pub undef: *mut AFMIndividualCharacterMetrics,
    pub fontnames: StringHashPtr,
    pub compositenames: StringHashPtr,
}

pub type AFMFont = *mut afm_font_st;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct encoding_table_st {
    pub code: libc::c_int,
    pub character: *mut libc::c_char,
}

static BUILTIN_COURIER: [encoding_table_st; 261] = [
    encoding_table_st {
        code: 32,
        character: b"space\0".as_ptr() as *mut libc::c_char,
    },
    // ... (rest of the encoding table entries)
    encoding_table_st {
        code: 0,
        character: ptr::null_mut(),
    },
];

#[no_mangle]
pub extern "C" fn afm_open_default_font(
    handle: AFMHandle,
    font_return: *mut AFMFont,
) -> AFMError {
    unsafe {
        let font = libc::calloc(1, mem::size_of::<afm_font_st>()) as AFMFont;
        if font.is_null() {
            return 2;
        }

        (*font).private = libc::calloc(1, mem::size_of::<afm_font_private_data_st>()) 
            as *mut afm_font_private_data_st;
        if (*font).private.is_null() {
            libc::free(font as *mut libc::c_void);
            return 2;
        }

        (*(*font).private).fontnames = strhash_init();
        if (*(*font).private).fontnames.is_null() {
            libc::free((*font).private as *mut libc::c_void);
            libc::free(font as *mut libc::c_void);
            return 2;
        }

        (*font).version = 4.0;
        let font_name = CString::new("Courier").unwrap();
        (*font).global_info.FontName = libc::strdup(font_name.as_ptr());
        if (*font).global_info.FontName.is_null() {
            afm_close_font(font);
            return 2;
        }

        (*font).global_info.FontBBox_llx = -40.0;
        (*font).global_info.FontBBox_lly = -290.0;
        (*font).global_info.FontBBox_urx = 640.0;
        (*font).global_info.FontBBox_ury = 795.0;

        (*font).writing_direction_metrics[0].is_valid = AFMTrue;
        (*font).writing_direction_metrics[0].IsFixedPitch = AFMTrue;
        (*font).writing_direction_metrics[0].CharWidth_x = 600.0;
        (*font).writing_direction_metrics[0].CharWidth_y = 0.0;

        (*font).num_character_metrics = (BUILTIN_COURIER.len() - 1) as AFMInteger;
        (*font).character_metrics = libc::calloc(
            BUILTIN_COURIER.len() - 1,
            mem::size_of::<AFMIndividualCharacterMetrics>(),
        ) as *mut AFMIndividualCharacterMetrics;

        if (*font).character_metrics.is_null() {
            afm_close_font(font);
            return 2;
        }

        for (i, entry) in BUILTIN_COURIER.iter().enumerate() {
            if entry.character.is_null() {
                break;
            }

            let cm = &mut *(*font).character_metrics.add(i) as *mut AFMIndividualCharacterMetrics;
            (*cm).name = libc::strdup(entry.character);
            if (*cm).name.is_null() {
                afm_close_font(font);
                return 2;
            }

            if strhash_put(
                (*(*font).private).fontnames,
                (*cm).name,
                libc::strlen((*cm).name) as libc::c_int,
                cm as *mut libc::c_void,
                ptr::null_mut(),
            ) == 0
            {
                afm_close_font(font);
                return 2;
            }

            (*cm).character_code = entry.code as AFMInteger;
            (*cm).w0x = 600.0;
            (*cm).w0y = 0.0;
        }

        *font_return = font;
        0
    }
}

// Placeholder for strhash_put and strhash_init functions
extern "C" {
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut libc::c_char,
        keylen: libc::c_int,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn strhash_init() -> StringHashPtr;
    fn afm_close_font(font: AFMFont) -> AFMError;
}