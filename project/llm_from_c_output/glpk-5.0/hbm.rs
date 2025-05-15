use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Read};
use std::path::Path;
use std::str::{FromStr, Utf8Error};
use std::string::FromUtf8Error;
use std::fmt;

#[derive(Debug)]
pub enum HbmError {
    Io(io::Error),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    Utf8(Utf8Error),
    FromUtf8(FromUtf8Error),
    Custom(String),
}

impl fmt::Display for HbmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HbmError::Io(e) => write!(f, "IO error: {}", e),
            HbmError::ParseInt(e) => write!(f, "Parse int error: {}", e),
            HbmError::ParseFloat(e) => write!(f, "Parse float error: {}", e),
            HbmError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            HbmError::FromUtf8(e) => write!(f, "From UTF-8 error: {}", e),
            HbmError::Custom(s) => write!(f, "Error: {}", s),
        }
    }
}

impl From<io::Error> for HbmError {
    fn from(err: io::Error) -> HbmError {
        HbmError::Io(err)
    }
}

impl From<std::num::ParseIntError> for HbmError {
    fn from(err: std::num::ParseIntError) -> HbmError {
        HbmError::ParseInt(err)
    }
}

impl From<std::num::ParseFloatError> for HbmError {
    fn from(err: std::num::ParseFloatError) -> HbmError {
        HbmError::ParseFloat(err)
    }
}

impl From<Utf8Error> for HbmError {
    fn from(err: Utf8Error) -> HbmError {
        HbmError::Utf8(err)
    }
}

impl From<FromUtf8Error> for HbmError {
    fn from(err: FromUtf8Error) -> HbmError {
        HbmError::FromUtf8(err)
    }
}

#[derive(Debug)]
pub struct Hbm {
    pub title: String,
    pub key: String,
    pub mxtype: String,
    pub rhstyp: String,
    pub ptrfmt: String,
    pub indfmt: String,
    pub valfmt: String,
    pub rhsfmt: String,
    pub totcrd: i32,
    pub ptrcrd: i32,
    pub indcrd: i32,
    pub valcrd: i32,
    pub rhscrd: i32,
    pub nrow: i32,
    pub ncol: i32,
    pub nnzero: i32,
    pub neltvl: i32,
    pub nrhs: i32,
    pub nrhsix: i32,
    pub nrhsvl: i32,
    pub nguess: i32,
    pub nexact: i32,
    pub colptr: Vec<i32>,
    pub rowind: Vec<i32>,
    pub rhsptr: Option<Vec<i32>>,
    pub rhsind: Option<Vec<i32>>,
    pub values: Option<Vec<f64>>,
    pub rhsval: Option<Vec<f64>>,
    pub sguess: Option<Vec<f64>>,
    pub xexact: Option<Vec<f64>>,
}

struct Dsa {
    fname: String,
    reader: BufReader<File>,
    seqn: i32,
    card: String,
    fmt_p: i32,
    fmt_k: i32,
    fmt_f: char,
    fmt_w: i32,
    fmt_d: i32,
}

impl Dsa {
    fn new(fname: &str) -> Result<Self, HbmError> {
        let file = File::open(fname)?;
        let reader = BufReader::new(file);
        Ok(Self {
            fname: fname.to_string(),
            reader,
            seqn: 0,
            card: String::with_capacity(81),
            fmt_p: 0,
            fmt_k: 1,
            fmt_f: 'I',
            fmt_w: 0,
            fmt_d: 0,
        })
    }

    fn read_card(&mut self) -> Result<(), HbmError> {
        self.seqn += 1;
        let mut buf = Vec::new();
        
        self.reader.read_until(b'\n', &mut buf)?;
        
        // Remove trailing newline and carriage return
        while buf.last() == Some(&b'\n') || buf.last() == Some(&b'\r') {
            buf.pop();
        }
        
        // Check for control characters
        for &c in &buf {
            if c.is_ascii_control() && c != b'\n' && c != b'\r' {
                return Err(HbmError::Custom(format!(
                    "{}:{}: invalid control character", 
                    self.fname, self.seqn
                )));
            }
        }
        
        let line = String::from_utf8(buf)?;
        
        // Remove trailing spaces
        let line = line.trim_end();
        
        // Check line length
        if line.len() > 80 {
            return Err(HbmError::Custom(format!(
                "{}:{}: card image too long", 
                self.fname, self.seqn
            )));
        }
        
        // Pad with spaces to 80 characters
        self.card = format!("{:80}", line);
        Ok(())
    }

    fn scan_int(&mut self, fld: &str, pos: usize, width: usize) -> Result<i32, HbmError> {
        let end = pos + width;
        if end > 80 {
            return Err(HbmError::Custom("Field position out of bounds".to_string()));
        }
        
        let s = self.card[pos..end].trim();
        s.parse::<i32>().map_err(|e| {
            HbmError::Custom(format!(
                "{}:{}: field '{}' contains invalid value '{}'", 
                self.fname, self.seqn, fld, s
            ))
        })
    }

    fn parse_fmt(&mut self, fmt: &str) -> Result<(), HbmError> {
        let mut chars = fmt.chars();
        
        if chars.next() != Some('(') {
            return Err(HbmError::Custom(format!(
                "hbm_read_mat: format '{}' not recognised", fmt
            )));
        }
        
        let mut k = 1;
        let fmt_bytes = fmt.as_bytes();
        
        // Optional scale factor
        self.fmt_p = 0;
        if fmt_bytes[k].is_ascii_digit() {
            let mut s = 0;
            let mut val_str = String::new();
            while k < fmt.len() && fmt_bytes[k].is_ascii_digit() {
                val_str.push(fmt_bytes[k] as char);
                k += 1;
                s += 1;
                if s > 80 {
                    return Err(HbmError::Custom("Format too long".to_string()));
                }
            }
            
            let val = val_str.parse::<i32>()?;
            if k >= fmt.len() || fmt_bytes[k].to_ascii_uppercase() != b'P' {
                self.fmt_k = val;
                if k < fmt.len() && fmt_bytes[k] == b',' {
                    k += 1;
                }
            } else {
                self.fmt_p = val;
                k += 1;
                if !(0..=255).contains(&self.fmt_p) {
                    return Err(HbmError::Custom("Invalid scale factor".to_string()));
                }
                if k < fmt.len() && fmt_bytes[k] == b',' {
                    k += 1;
                }
            }
        }
        
        // Optional iterator
        if k < fmt.len() && fmt_bytes[k].is_ascii_digit() {
            let mut s = 0;
            let mut val_str = String::new();
            while k < fmt.len() && fmt_bytes[k].is_ascii_digit() {
                val_str.push(fmt_bytes[k] as char);
                k += 1;
                s += 1;
                if s > 80 {
                    return Err(HbmError::Custom("Format too long".to_string()));
                }
            }
            
            self.fmt_k = val_str.parse::<i32>()?;
            if !(1..=255).contains(&self.fmt_k) {
                return Err(HbmError::Custom("Invalid iterator".to_string()));
            }
        }
        
        // Format code
        if k >= fmt.len() {
            return Err(HbmError::Custom("Missing format code".to_string()));
        }
        self.fmt_f = fmt_bytes[k].to_ascii_uppercase() as char;
        if !matches!(self.fmt_f, 'D' | 'E' | 'F' | 'G' | 'I') {
            return Err(HbmError::Custom("Invalid format code".to_string()));
        }
        k += 1;
        
        // Field width
        if k >= fmt.len() || !fmt_bytes[k].is_ascii_digit() {
            return Err(HbmError::Custom("Missing field width".to_string()));
        }
        
        let mut s = 0;
        let mut width_str = String::new();
        while k < fmt.len() && fmt_bytes[k].is_ascii_digit() {
            width_str.push(fmt_bytes[k] as char);
            k += 1;
            s += 1;
            if s > 80 {
                return Err(HbmError::Custom("Format too long".to_string()));
            }
        }
        
        self.fmt_w = width_str.parse::<i32>()?;
        if !(1..=255).contains(&self.fmt_w) {
            return Err(HbmError::Custom("Invalid field width".to_string()));
        }
        
        // Optional decimal places
        self.fmt_d = 0;
        if k < fmt.len() && fmt_bytes[k] == b'.' {
            k += 1;
            if k >= fmt.len() || !fmt_bytes[k].is_ascii_digit() {
                return Err(HbmError::Custom("Missing decimal places".to_string()));
            }
            
            let mut s = 0;
            let mut dec_str = String::new();
            while k < fmt.len() && fmt_bytes[k].is_ascii_digit() {
                dec_str.push(fmt_bytes[k] as char);
                k += 1;
                s += 1;
                if s > 80 {
                    return Err(HbmError::Custom("Format too long".to_string()));
                }
            }
            
            self.fmt_d = dec_str.parse::<i32>()?;
            if !(0..=255).contains(&self.fmt_d) {
                return Err(HbmError::Custom("Invalid decimal places".to_string()));
            }
        }
        
        // Closing parenthesis
        if k >= fmt.len() || fmt_bytes[k] != b')' || (k + 1 < fmt.len() && fmt_bytes[k + 1] != 0) {
            return Err(HbmError::Custom("Missing closing parenthesis".to_string()));
        }
        
        Ok(())
    }

    fn read_int_array(&mut self, name: &str, fmt: &str, n: i32) -> Result<Vec<i32>, HbmError> {
        self.parse_fmt(fmt)?;
        
        if self.fmt_f != 'I' || self.fmt_w > 80 || self.fmt_k * self.fmt_w > 80 {
            return Err(HbmError::Custom(format!(
                "{}:{}: can't read array '{}' - invalid format '{}'",
                self.fname, self.seqn, name, fmt
            )));
        }
        
        let mut result = Vec::with_capacity(n as usize);
        let mut pos = i32::MAX;
        
        for _ in 0..n {
            if pos >= self.fmt_k {
                self.read_card()?;
                pos = 0;
            }
            
            let start = (self.fmt_w * pos) as usize;
            let end = start + self.fmt_w as usize;
            let s = self.card[start..end].trim();
            
            let val = s.parse::<i32>().map_err(|_| {
                HbmError::Custom(format!(
                    "{}:{}: can't read array '{}' - invalid value '{}'",
                    self.fname, self.seqn, name, s
                ))
            })?;
            
            result.push(val);
            pos += 1;
        }
        
        Ok(result)
    }

    fn read_real_array(&mut self, name: &str, fmt: &str, n: i32) -> Result<Vec<f64>, HbmError> {
        self.parse_fmt(fmt)?;
        
        if self.fmt_f == 'I' || self.fmt_w > 80 || self.fmt_k * self.fmt_w > 80 {
            return Err(HbmError::Custom(format!(
                "{}:{}: can't read array '{}' - invalid format '{}'",
                self.fname, self.seqn, name, fmt
            )));
        }
        
        let mut result = Vec::with_capacity(n as usize);
        let mut pos = i32::MAX;
        
        for _ in 0..n {
            if pos >= self.fmt_k {
                self.read_card()?;
                pos = 0;
            }
            
            let start = (self.fmt_w * pos) as usize;
            let end = start + self.fmt_w as usize;
            let mut s = self.card[start..end].trim().to_string();
            
            // Check for decimal point
            if !s.contains('.') && s != "0" {
                return Err(HbmError::Custom(format!(
                    "{}:{}: can't read array '{}' - value '{}' has no decimal point",
                    self.fname, self.seqn, name, s
                )));
            }
            
            // Convert to uppercase
            s = s.to_uppercase();
            
            // Replace 'D' with 'E'
            if let Some(pos) = s.find('D') {
                s.replace_range(pos..pos+1, "E");
            }
            
            // Handle missing exponent letter
            if let Some(pos) = s[1..].find(|c| c == '+' || c == '-') {
                let pos = pos + 1;
                if pos > 0 && s.chars().nth(pos - 1) != Some('E') {
                    s.insert(pos, 'E');
                }
            }
            
            let val = s.parse::<f64>().map_err(|_| {
                HbmError::Custom(format!(
                    "{}:{}: can't read array '{}' - invalid value '{}'",
                    self.fname, self.seqn, name, s
                ))
            })?;
            
            result.push(val);
            pos += 1;
        }
        
        Ok(result)
    }
}

pub fn hbm_read_mat(fname: &str) -> Result<Hbm, HbmError> {
    let mut dsa = Dsa::new(fname)?;
    
    println!("hbm_read_mat: reading matrix from '{}'...", fname);
    
    // Read first heading card
    dsa.read_card()?;
    let title = dsa.card[..72].trim().to_string();
    println!("{}", title);
    
    let key = dsa.card[72..80].trim().to_string();
    println!("key = {}", key);
    
    // Read second heading card
    dsa.read_card()?;
    let totcrd = dsa.scan_int("totcrd", 0, 14)?;
    let ptrcrd = dsa.scan_int("ptrcrd", 14, 14)?;
    let indcrd = dsa.scan_int("indcrd", 28, 14)?;
    let valcrd = dsa.scan_int("valcrd", 42, 14)?;
    let rhscrd = dsa.scan_int("rhscrd", 56, 14)?;
    
    println!(
        "totcrd = {}; ptrcrd = {}; indcrd = {}; valcrd = {}; rhscrd = {}",
        totcrd, ptrcrd, indcrd, valcrd, rhscrd
    );
    
    // Read third heading card
    dsa.read_card()?;
    let mxtype = dsa.card[..3].to_string();
    
    if !mxtype.starts_with(|c| c == 'R' || c == 'C' || c == 'P') ||
       !mxtype.chars().nth(1).map_or(false, |c| "SUHZR".contains(c)) ||
       !mxtype.ends_with(|c| c == 'A' || c == 'E') {
        return Err(HbmError::Custom(format!(
            "{}:{}: matrix type '{}' not recognised",
            dsa.fname, dsa.seqn, mxtype
        )));
    }
    
    let nrow = dsa.scan_int("nrow", 14, 14)?;
    let ncol = dsa.scan_int("ncol", 28, 14)?;
    let nnzero = dsa.scan_int("nnzero", 42, 14)?;
    let neltvl = dsa.scan_int("neltvl", 56, 14)?;
    
    println!(
        "mxtype = {}; nrow = {}; ncol = {}; nnzero = {}; neltvl = {}",
        mxtype, nrow, ncol, nnzero, neltvl
    );
    
    // Read fourth heading card
    dsa.read_card()?;
    let ptrfmt = dsa.card[..16].trim().to_string();
    let indfmt = dsa.card[16..32].trim().to_string();
    let valfmt = dsa.card[32..52].trim().to_string();
    let rhsfmt = dsa.card[52..72].trim().to_string();
    
    println!(
        "ptrfmt = {}; indfmt = {}; valfmt = {}; rhsfmt