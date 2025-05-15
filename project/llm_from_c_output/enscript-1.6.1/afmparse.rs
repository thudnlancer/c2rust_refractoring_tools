use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, Read};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AFMError {
    FileIo,
    NotAfmFile,
    UnsupportedFormat,
    Syntax,
    Memory,
    // Add other error variants as needed
}

#[derive(Debug, Clone, Copy)]
pub enum AFMKey {
    Ascender,
    Axes,
    AxisLabel,
    AxisType,
    B,
    BlendAxisTypes,
    BlendDesignMap,
    BlendDesignPositions,
    C,
    CC,
    CH,
    CapHeight,
    CharWidth,
    CharacterSet,
    Characters,
    Comment,
    Descendents,
    Descender,
    EncodingScheme,
    EndAxis,
    EndCharMetrics,
    EndCompFontMetrics,
    EndComposites,
    EndDescendent,
    EndDirection,
    EndFontMetrics,
    EndKernData,
    EndKernPairs,
    EndMaster,
    EndMasterFontMetrics,
    EndTrackKern,
    EscChar,
    FamilyName,
    FontBBox,
    FontName,
    FullName,
    IsBaseFont,
    IsFixedPitch,
    IsFixedV,
    ItalicAngle,
    KP,
    KPH,
    KPX,
    KPY,
    L,
    MappingScheme,
    Masters,
    MetricsSets,
    N,
    Notice,
    PCC,
    StartAxis,
    StartCharMetrics,
    StartCompFontMetrics,
    StartComposites,
    StartDescendent,
    StartDirection,
    StartFontMetrics,
    StartKernData,
    StartKernPairs,
    StartMaster,
    StartMasterFontMetrics,
    StartTrackKern,
    TrackKern,
    UnderlinePosition,
    UnderlineThickness,
    VV,
    VVector,
    Version,
    W,
    W0,
    W0X,
    W0Y,
    W1,
    W1X,
    W1Y,
    WX,
    WY,
    Weight,
    WeightVector,
    XHeight,
}

#[derive(Debug, Clone)]
pub enum AFMNode {
    String(String),
    Name(String),
    Number(f64),
    Integer(i32),
    Boolean(bool),
    // Array(Vec<AFMNode>), // Not implemented yet
}

#[derive(Debug, Clone)]
pub struct AFMFont {
    pub version: f64,
    pub global_info: AFMGlobalInfo,
    pub writing_direction_metrics: [AFMWritingDirectionMetrics; 2],
    pub num_character_metrics: usize,
    pub character_metrics: Vec<AFMIndividualCharacterMetrics>,
    pub num_kern_pairs: usize,
    pub kern_pairs: Vec<AFMPairWiseKerning>,
    pub num_track_kerns: usize,
    pub track_kerns: Vec<AFMTrackKern>,
    pub num_composites: usize,
    pub composites: Vec<AFMComposite>,
    pub encoding: [Option<AFMIndividualCharacterMetrics>; 256],
    pub private: AFMFontPrivate,
}

#[derive(Debug, Clone)]
pub struct AFMGlobalInfo {
    pub font_name: Option<String>,
    pub full_name: Option<String>,
    pub family_name: Option<String>,
    pub weight: Option<String>,
    pub font_bbox: (f64, f64, f64, f64),
    pub version: Option<String>,
    pub notice: Option<String>,
    pub encoding_scheme: Option<String>,
    pub mapping_scheme: Option<i32>,
    pub esc_char: Option<i32>,
    pub character_set: Option<String>,
    pub characters: Option<i32>,
    pub is_base_font: bool,
    pub v_vector: (f64, f64),
    pub is_fixed_v: bool,
    pub cap_height: Option<f64>,
    pub x_height: Option<f64>,
    pub ascender: Option<f64>,
    pub descender: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct AFMWritingDirectionMetrics {
    pub is_valid: bool,
    pub underline_position: Option<f64>,
    pub underline_thickness: Option<f64>,
    pub italic_angle: Option<f64>,
    pub char_width: (f64, f64),
    pub is_fixed_pitch: bool,
}

#[derive(Debug, Clone)]
pub struct AFMIndividualCharacterMetrics {
    pub character_code: i32,
    pub name: Option<String>,
    pub w0x: f64,
    pub w0y: f64,
    pub w1x: f64,
    pub w1y: f64,
    pub vv_x: f64,
    pub vv_y: f64,
    pub llx: f64,
    pub lly: f64,
    pub urx: f64,
    pub ury: f64,
}

#[derive(Debug, Clone)]
pub struct AFMPairWiseKerning {
    pub name1: String,
    pub name2: String,
    pub kx: f64,
    pub ky: f64,
}

#[derive(Debug, Clone)]
pub struct AFMTrackKern {
    pub degree: i32,
    pub min_ptsize: f64,
    pub min_kern: f64,
    pub max_ptsize: f64,
    pub max_kern: f64,
}

#[derive(Debug, Clone)]
pub struct AFMComposite {
    pub name: String,
    pub num_components: usize,
    pub components: Vec<AFMCompositeComponent>,
}

#[derive(Debug, Clone)]
pub struct AFMCompositeComponent {
    pub name: String,
    pub deltax: f64,
    pub deltay: f64,
}

#[derive(Debug, Clone)]
pub struct AFMFontPrivate {
    pub fontnames: HashMap<String, AFMIndividualCharacterMetrics>,
    pub compositenames: HashMap<String, AFMComposite>,
    pub undef: Option<AFMIndividualCharacterMetrics>,
}

struct ParseContext {
    reader: BufReader<File>,
    token: String,
}

impl ParseContext {
    fn new(file: File) -> Self {
        ParseContext {
            reader: BufReader::new(file),
            token: String::new(),
        }
    }

    fn is_space(ch: char) -> bool {
        ch == ' ' || ch == '\n' || ch == '\r' || ch == '\t' || ch == ';'
    }

    fn get_token(&mut self) -> Result<bool, AFMError> {
        self.token.clear();
        let mut ch = [0u8];
        
        // Skip whitespace
        loop {
            if self.reader.read(&mut ch).map_err(|_| AFMError::FileIo)? == 0 {
                return Ok(false);
            }
            let c = ch[0] as char;
            if !Self::is_space(c) {
                break;
            }
        }
        
        // Read token
        loop {
            let c = ch[0] as char;
            if Self::is_space(c) {
                break;
            }
            self.token.push(c);
            if self.reader.read(&mut ch).map_err(|_| AFMError::FileIo)? == 0 {
                break;
            }
        }
        
        Ok(true)
    }

    fn get_line_token(&mut self) -> Result<bool, AFMError> {
        self.token.clear();
        let mut line = String::new();
        if self.reader.read_line(&mut line).map_err(|_| AFMError::FileIo)? == 0 {
            return Ok(false);
        }
        
        // Trim whitespace
        self.token = line.trim().to_string();
        Ok(true)
    }
}

pub fn afm_parse_file(filename: &Path) -> Result<AFMFont, AFMError> {
    let file = File::open(filename).map_err(|_| AFMError::FileIo)?;
    let mut ctx = ParseContext::new(file);
    let mut font = AFMFont {
        version: 0.0,
        global_info: AFMGlobalInfo::default(),
        writing_direction_metrics: [AFMWritingDirectionMetrics::default(); 2],
        num_character_metrics: 0,
        character_metrics: Vec::new(),
        num_kern_pairs: 0,
        kern_pairs: Vec::new(),
        num_track_kerns: 0,
        track_kerns: Vec::new(),
        num_composites: 0,
        composites: Vec::new(),
        encoding: [None; 256],
        private: AFMFontPrivate {
            fontnames: HashMap::new(),
            compositenames: HashMap::new(),
            undef: None,
        },
    };

    // Check that file is really an AFM file
    let key = get_key(&mut ctx)?;
    if key != AFMKey::StartFontMetrics {
        return Err(AFMError::NotAfmFile);
    }
    let node = get_type(&mut ctx, AFMType::Number)?;
    font.version = if let AFMNode::Number(n) = node { n } else { 0.0 };

    // Main parsing loop
    let mut done = false;
    let mut wd = 0; // Writing direction
    while !done {
        let key = get_key(&mut ctx)?;
        match key {
            AFMKey::Comment => {
                ctx.get_line_token()?;
                continue;
            }
            AFMKey::StartFontMetrics => {
                let node = get_type(&mut ctx, AFMType::Number)?;
                font.version = if let AFMNode::Number(n) = node { n } else { 0.0 };
            }
            AFMKey::EndFontMetrics => {
                done = true;
            }
            // Handle other keys...
            _ => {}
        }
    }

    // Post-processing
    if !font.writing_direction_metrics[0].is_valid && !font.writing_direction_metrics[1].is_valid {
        font.writing_direction_metrics[0].is_valid = true;
    }

    Ok(font)
}

#[derive(Debug)]
enum AFMType {
    String,
    Name,
    Number,
    Integer,
    Boolean,
    // Array,
}

fn get_type(ctx: &mut ParseContext, type_: AFMType) -> Result<AFMNode, AFMError> {
    match type_ {
        AFMType::String => {
            ctx.get_line_token()?;
            Ok(AFMNode::String(ctx.token.clone()))
        }
        AFMType::Name => {
            ctx.get_token()?;
            Ok(AFMNode::Name(ctx.token.clone()))
        }
        AFMType::Number => {
            ctx.get_token()?;
            let num = f64::from_str(&ctx.token).map_err(|_| AFMError::Syntax)?;
            Ok(AFMNode::Number(num))
        }
        AFMType::Integer => {
            ctx.get_token()?;
            let num = i32::from_str(&ctx.token).map_err(|_| AFMError::Syntax)?;
            Ok(AFMNode::Integer(num))
        }
        AFMType::Boolean => {
            ctx.get_token()?;
            let val = match ctx.token.as_str() {
                "true" => true,
                "false" => false,
                _ => return Err(AFMError::Syntax),
            };
            Ok(AFMNode::Boolean(val))
        }
    }
}

fn get_key(ctx: &mut ParseContext) -> Result<AFMKey, AFMError> {
    loop {
        if !ctx.get_token()? {
            return Err(AFMError::Syntax);
        }
        if let Some(key) = match_key(&ctx.token) {
            return Ok(key);
        }
        // Skip unknown key
        ctx.get_line_token()?;
    }
}

fn match_key(key: &str) -> Option<AFMKey> {
    match key {
        "Ascender" => Some(AFMKey::Ascender),
        "Axes" => Some(AFMKey::Axes),
        // Add all other key mappings...
        _ => None,
    }
}

// Implement Default for all structs
impl Default for AFMGlobalInfo {
    fn default() -> Self {
        Self {
            font_name: None,
            full_name: None,
            family_name: None,
            weight: None,
            font_bbox: (0.0, 0.0, 0.0, 0.0),
            version: None,
            notice: None,
            encoding_scheme: None,
            mapping_scheme: None,
            esc_char: None,
            character_set: None,
            characters: None,
            is_base_font: false,
            v_vector: (0.0, 0.0),
            is_fixed_v: false,
            cap_height: None,
            x_height: None,
            ascender: None,
            descender: None,
        }
    }
}

impl Default for AFMWritingDirectionMetrics {
    fn default() -> Self {
        Self {
            is_valid: false,
            underline_position: None,
            underline_thickness: None,
            italic_angle: None,
            char_width: (0.0, 0.0),
            is_fixed_pitch: false,
        }
    }
}

impl Default for AFMIndividualCharacterMetrics {
    fn default() -> Self {
        Self {
            character_code: 0,
            name: None,
            w0x: 0.0,
            w0y: 0.0,
            w1x: 0.0,
            w1y: 0.0,
            vv_x: 0.0,
            vv_y: 0.0,
            llx: 0.0,
            lly: 0.0,
            urx: 0.0,
            ury: 0.0,
        }
    }
}