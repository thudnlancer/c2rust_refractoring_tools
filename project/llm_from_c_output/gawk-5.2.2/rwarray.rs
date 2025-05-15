/*
 * rwarray.rs - Rust implementation of binary read/write array functions
 *
 * Translated from C to Rust with safety and idiomatic practices
 * Original C code by Arnold Robbins
 */

use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use std::mem;
use std::ptr;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};
use std::convert::TryInto;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::collections::HashMap;
use std::path::Path;
use std::error::Error;
use std::fmt;

#[cfg(feature = "mpfr")]
use gmp::mpz::Mpz;
#[cfg(feature = "mpfr")]
use mpfr::Mpfr;

const MAGIC: &str = "awkrulz\n";
const MAJOR: u32 = 4;
const MINOR: u32 = 1;

#[derive(Debug)]
enum ValueType {
    String = 1,
    Number = 2,
    Gmp = 3,
    Mpfr = 4,
    Array = 5,
    Regex = 6,
    StrNum = 7,
    Bool = 8,
    Undefined = 20,
}

impl From<u32> for ValueType {
    fn from(code: u32) -> Self {
        match code {
            1 => ValueType::String,
            2 => ValueType::Number,
            3 => ValueType::Gmp,
            4 => ValueType::Mpfr,
            5 => ValueType::Array,
            6 => ValueType::Regex,
            7 => ValueType::StrNum,
            8 => ValueType::Bool,
            _ => ValueType::Undefined,
        }
    }
}

#[derive(Debug)]
struct AwkValue {
    val_type: ValueType,
    str_value: Option<String>,
    num_value: Option<f64>,
    array_value: Option<HashMap<String, AwkValue>>,
    bool_value: Option<bool>,
    #[cfg(feature = "mpfr")]
    mpz_value: Option<Mpz>,
    #[cfg(feature = "mpfr")]
    mpfr_value: Option<Mpfr>,
}

impl AwkValue {
    fn new() -> Self {
        AwkValue {
            val_type: ValueType::Undefined,
            str_value: None,
            num_value: None,
            array_value: None,
            bool_value: None,
            #[cfg(feature = "mpfr")]
            mpz_value: None,
            #[cfg(feature = "mpfr")]
            mpfr_value: None,
        }
    }

    fn make_string(s: &str) -> Self {
        AwkValue {
            val_type: ValueType::String,
            str_value: Some(s.to_string()),
            ..AwkValue::new()
        }
    }

    fn make_number(n: f64) -> Self {
        AwkValue {
            val_type: ValueType::Number,
            num_value: Some(n),
            ..AwkValue::new()
        }
    }

    fn make_bool(b: bool) -> Self {
        AwkValue {
            val_type: ValueType::Bool,
            bool_value: Some(b),
            ..AwkValue::new()
        }
    }

    fn make_array() -> Self {
        AwkValue {
            val_type: ValueType::Array,
            array_value: Some(HashMap::new()),
            ..AwkValue::new()
        }
    }
}

struct ArrayReader {
    file: File,
}

impl ArrayReader {
    fn new(filename: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(filename)?;
        Ok(ArrayReader { file })
    }

    fn read_magic(&mut self) -> Result<(), Box<dyn Error>> {
        let mut magic = [0u8; 8];
        self.file.read_exact(&mut magic)?;
        if &magic != MAGIC.as_bytes() {
            return Err("Invalid magic number".into());
        }
        Ok(())
    }

    fn read_version(&mut self) -> Result<(u32, u32), Box<dyn Error>> {
        let mut major = [0u8; 4];
        let mut minor = [0u8; 4];
        self.file.read_exact(&mut major)?;
        self.file.read_exact(&mut minor)?;
        
        let major = u32::from_be_bytes(major);
        let minor = u32::from_be_bytes(minor);
        
        if major != MAJOR || minor != MINOR {
            return Err("Version mismatch".into());
        }
        
        Ok((major, minor))
    }

    fn read_array(&mut self) -> Result<AwkValue, Box<dyn Error>> {
        let mut count_bytes = [0u8; 4];
        self.file.read_exact(&mut count_bytes)?;
        let count = u32::from_be_bytes(count_bytes);
        
        let mut array = AwkValue::make_array();
        for _ in 0..count {
            let elem = self.read_element()?;
            if let Some(key) = &elem.0.str_value {
                array.array_value.as_mut().unwrap().insert(key.clone(), elem.1);
            }
        }
        Ok(array)
    }

    fn read_element(&mut self) -> Result<(AwkValue, AwkValue), Box<dyn Error>> {
        let mut len_bytes = [0u8; 4];
        self.file.read_exact(&mut len_bytes)?;
        let len = u32::from_be_bytes(len_bytes) as usize;
        
        let mut key = vec![0u8; len];
        if len > 0 {
            self.file.read_exact(&mut key)?;
        }
        
        let key_str = String::from_utf8(key)?;
        let key_val = AwkValue::make_string(&key_str);
        let value = self.read_value()?;
        
        Ok((key_val, value))
    }

    fn read_value(&mut self) -> Result<AwkValue, Box<dyn Error>> {
        let mut type_bytes = [0u8; 4];
        self.file.read_exact(&mut type_bytes)?;
        let val_type = ValueType::from(u32::from_be_bytes(type_bytes));
        
        match val_type {
            ValueType::Array => self.read_array(),
            ValueType::Number => {
                let mut len_bytes = [0u8; 4];
                self.file.read_exact(&mut len_bytes)?;
                let len = u32::from_be_bytes(len_bytes) as usize;
                
                let mut buf = vec![0u8; len];
                self.file.read_exact(&mut buf)?;
                
                let num_str = String::from_utf8(buf)?;
                let num = num_str.parse::<f64>()?;
                Ok(AwkValue::make_number(num))
            }
            ValueType::String | ValueType::Regex | ValueType::StrNum | ValueType::Undefined => {
                let mut len_bytes = [0u8; 4];
                self.file.read_exact(&mut len_bytes)?;
                let len = u32::from_be_bytes(len_bytes) as usize;
                
                let mut buf = vec![0u8; len];
                self.file.read_exact(&mut buf)?;
                
                let s = String::from_utf8(buf)?;
                Ok(AwkValue::make_string(&s))
            }
            ValueType::Bool => {
                let mut len_bytes = [0u8; 4];
                self.file.read_exact(&mut len_bytes)?;
                let len = u32::from_be_bytes(len_bytes) as usize;
                
                let mut buf = vec![0u8; len];
                self.file.read_exact(&mut buf)?;
                
                let s = String::from_utf8(buf)?;
                let b = s == "TRUE";
                Ok(AwkValue::make_bool(b))
            }
            #[cfg(feature = "mpfr")]
            ValueType::Gmp => {
                // MPZ reading implementation
                unimplemented!()
            }
            #[cfg(feature = "mpfr")]
            ValueType::Mpfr => {
                // MPFR reading implementation
                unimplemented!()
            }
            _ => Err("Unknown value type".into()),
        }
    }
}

struct ArrayWriter {
    file: File,
}

impl ArrayWriter {
    fn new(filename: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::create(filename)?;
        Ok(ArrayWriter { file })
    }

    fn write_magic(&mut self) -> Result<(), Box<dyn Error>> {
        self.file.write_all(MAGIC.as_bytes())?;
        Ok(())
    }

    fn write_version(&mut self) -> Result<(), Box<dyn Error>> {
        self.file.write_all(&MAJOR.to_be_bytes())?;
        self.file.write_all(&MINOR.to_be_bytes())?;
        Ok(())
    }

    fn write_array(&mut self, array: &HashMap<String, AwkValue>) -> Result<(), Box<dyn Error>> {
        let count = (array.len() as u32).to_be_bytes();
        self.file.write_all(&count)?;
        
        for (key, value) in array {
            self.write_element(key, value)?;
        }
        Ok(())
    }

    fn write_element(&mut self, key: &str, value: &AwkValue) -> Result<(), Box<dyn Error>> {
        let len = (key.len() as u32).to_be_bytes();
        self.file.write_all(&len)?;
        self.file.write_all(key.as_bytes())?;
        self.write_value(value)
    }

    fn write_value(&mut self, value: &AwkValue) -> Result<(), Box<dyn Error>> {
        match value.val_type {
            ValueType::Array => {
                let code = (ValueType::Array as u32).to_be_bytes();
                self.file.write_all(&code)?;
                if let Some(arr) = &value.array_value {
                    self.write_array(arr)?;
                }
            }
            ValueType::Number => {
                let code = (ValueType::Number as u32).to_be_bytes();
                self.file.write_all(&code)?;
                if let Some(num) = value.num_value {
                    let num_str = format!("{:.17}", num);
                    let len = (num_str.len() as u32 + 1).to_be_bytes();
                    self.file.write_all(&len)?;
                    self.file.write_all(num_str.as_bytes())?;
                    self.file.write_all(&[0])?; // null terminator
                }
            }
            ValueType::String | ValueType::Regex | ValueType::StrNum | ValueType::Undefined => {
                let code = (value.val_type as u32).to_be_bytes();
                self.file.write_all(&code)?;
                if let Some(s) = &value.str_value {
                    let len = (s.len() as u32).to_be_bytes();
                    self.file.write_all(&len)?;
                    self.file.write_all(s.as_bytes())?;
                }
            }
            ValueType::Bool => {
                let code = (ValueType::Bool as u32).to_be_bytes();
                self.file.write_all(&code)?;
                if let Some(b) = value.bool_value {
                    let s = if b { "TRUE" } else { "FALSE" };
                    let len = (s.len() as u32).to_be_bytes();
                    self.file.write_all(&len)?;
                    self.file.write_all(s.as_bytes())?;
                }
            }
            #[cfg(feature = "mpfr")]
            ValueType::Gmp => {
                // MPZ writing implementation
                unimplemented!()
            }
            #[cfg(feature = "mpfr")]
            ValueType::Mpfr => {
                // MPFR writing implementation
                unimplemented!()
            }
            _ => return Err("Unknown value type".into()),
        }
        Ok(())
    }
}

pub fn write_array(filename: &str, array: &HashMap<String, AwkValue>) -> Result<(), Box<dyn Error>> {
    let mut writer = ArrayWriter::new(filename)?;
    writer.write_magic()?;
    writer.write_version()?;
    writer.write_array(array)?;
    Ok(())
}

pub fn read_array(filename: &str) -> Result<HashMap<String, AwkValue>, Box<dyn Error>> {
    let mut reader = ArrayReader::new(filename)?;
    reader.read_magic()?;
    reader.read_version()?;
    if let AwkValue { array_value: Some(arr), .. } = reader.read_array()? {
        Ok(arr)
    } else {
        Err("Invalid array data".into())
    }
}

// Additional helper functions and error handling would be implemented here
// following Rust's best practices for error handling and memory safety