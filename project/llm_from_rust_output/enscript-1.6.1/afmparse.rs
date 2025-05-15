use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::{Read, BufRead, BufReader, Seek, SeekFrom};
use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_double, c_void};
use std::ptr;
use std::str::FromStr;
use std::collections::HashMap;

type AFMBoolean = c_uint;
const AFMTrue: AFMBoolean = 1;
const AFMFalse: AFMBoolean = 0;

#[derive(Debug, Clone)]
struct AFMNode {
    type_: i32,
    u: AFMNodeUnion,
}

#[derive(Debug, Clone)]
enum AFMNodeUnion {
    String(String),
    Name(String),
    Number(f64),
    Integer(i64),
    Boolean(AFMBoolean),
    Array(Vec<AFMNode>),
}

#[derive(Debug)]
struct AFMGlobalFontInformation {
    font_name: String,
    full_name: String,
    family_name: String,
    weight: String,
    font_bbox: (f64, f64, f64, f64),
    version: String,
    notice: String,
    encoding_scheme: String,
    mapping_scheme: i64,
    esc_char: i64,
    character_set: String,
    characters: i64,
    is_base_font: AFMBoolean,
    v_vector: (f64, f64),
    is_fixed_v: AFMBoolean,
    cap_height: f64,
    x_height: f64,
    ascender: f64,
    descender: f64,
}

#[derive(Debug)]
struct AFMWritingDirectionMetrics {
    is_valid: AFMBoolean,
    underline_position: f64,
    underline_thickness: f64,
    italic_angle: f64,
    char_width: (f64, f64),
    is_fixed_pitch: AFMBoolean,
}

#[derive(Debug)]
struct AFMIndividualCharacterMetrics {
    character_code: i64,
    width: (f64, f64),
    name: String,
    v_vector: (f64, f64),
    bbox: (f64, f64, f64, f64),
}

#[derive(Debug)]
struct AFMPairWiseKerning {
    name1: String,
    name2: String,
    kern: (f64, f64),
}

#[derive(Debug)]
struct AFMTrackKern {
    degree: i64,
    min_ptsize: f64,
    min_kern: f64,
    max_ptsize: f64,
    max_kern: f64,
}

#[derive(Debug)]
struct AFMComposite {
    name: String,
    components: Vec<AFMCompositeComponent>,
}

#[derive(Debug)]
struct AFMCompositeComponent {
    name: String,
    delta: (f64, f64),
}

#[derive(Debug)]
struct AFMFont {
    version: f64,
    info_level: u32,
    encoding: [Option<AFMIndividualCharacterMetrics>; 256],
    global_info: AFMGlobalFontInformation,
    writing_direction_metrics: [AFMWritingDirectionMetrics; 2],
    character_metrics: Vec<AFMIndividualCharacterMetrics>,
    composites: Vec<AFMComposite>,
    kern_pairs: Vec<AFMPairWiseKerning>,
    track_kerns: Vec<AFMTrackKern>,
}

struct ParseContext {
    reader: BufReader<File>,
    token: String,
}

impl ParseContext {
    fn new(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        Ok(Self {
            reader,
            token: String::new(),
        })
    }

    fn get_token(&mut self) -> Result<(), std::io::Error> {
        self.token.clear();
        let mut ch = [0u8];
        loop {
            if self.reader.read(&mut ch)? == 0 {
                break;
            }
            let c = ch[0] as char;
            if !c.is_whitespace() && c != ';' {
                self.token.push(c);
                break;
            }
        }
        while let Ok(n) = self.reader.read(&mut ch) {
            if n == 0 {
                break;
            }
            let c = ch[0] as char;
            if c.is_whitespace() || c == ';' {
                break;
            }
            self.token.push(c);
        }
        Ok(())
    }

    fn get_line_token(&mut self) -> Result<(), std::io::Error> {
        self.token.clear();
        self.reader.read_line(&mut self.token)?;
        self.token = self.token.trim_end().to_string();
        Ok(())
    }
}

fn parse_afm_file(filename: &str) -> Result<AFMFont, String> {
    let mut ctx = ParseContext::new(filename).map_err(|e| e.to_string())?;
    let mut font = AFMFont {
        version: 0.0,
        info_level: 0,
        encoding: [None; 256],
        global_info: AFMGlobalFontInformation {
            font_name: String::new(),
            full_name: String::new(),
            family_name: String::new(),
            weight: String::new(),
            font_bbox: (0.0, 0.0, 0.0, 0.0),
            version: String::new(),
            notice: String::new(),
            encoding_scheme: String::new(),
            mapping_scheme: 0,
            esc_char: 0,
            character_set: String::new(),
            characters: 0,
            is_base_font: AFMFalse,
            v_vector: (0.0, 0.0),
            is_fixed_v: AFMFalse,
            cap_height: 0.0,
            x_height: 0.0,
            ascender: 0.0,
            descender: 0.0,
        },
        writing_direction_metrics: [
            AFMWritingDirectionMetrics {
                is_valid: AFMFalse,
                underline_position: 0.0,
                underline_thickness: 0.0,
                italic_angle: 0.0,
                char_width: (0.0, 0.0),
                is_fixed_pitch: AFMFalse,
            },
            AFMWritingDirectionMetrics {
                is_valid: AFMFalse,
                underline_position: 0.0,
                underline_thickness: 0.0,
                italic_angle: 0.0,
                char_width: (0.0, 0.0),
                is_fixed_pitch: AFMFalse,
            },
        ],
        character_metrics: Vec::new(),
        composites: Vec::new(),
        kern_pairs: Vec::new(),
        track_kerns: Vec::new(),
    };

    // Main parsing logic would go here
    // This is just a skeleton - actual parsing would need to be implemented
    // following the C code's logic but using Rust's safety features

    Ok(font)
}

// Additional helper functions would be needed to implement the full parsing logic
// while maintaining Rust's safety guarantees