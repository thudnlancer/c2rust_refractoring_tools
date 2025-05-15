/* mpl6.rs */

use std::fs::File;
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::Path;
use std::str;
use std::fmt;
use std::error::Error;
use std::convert::TryFrom;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use libc::{c_int, c_void, size_t};
use mysql::{Conn, Opts};
use odbc::{Connection, Statement, DataType};

const CSV_FIELD_MAX: usize = 50;
const CSV_FDLEN_MAX: usize = 100;
const DBF_FIELD_MAX: usize = 50;
const DBF_FDLEN_MAX: usize = 100;

#[derive(Debug)]
enum CsvError {
    Io(io::Error),
    Parse(String),
    FieldTooLong,
    EmptyField,
    InvalidField,
    TooManyFields,
    MissingFields(usize),
    UnexpectedEof,
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CsvError::Io(e) => write!(f, "IO error: {}", e),
            CsvError::Parse(s) => write!(f, "Parse error: {}", s),
            CsvError::FieldTooLong => write!(f, "Field too long"),
            CsvError::EmptyField => write!(f, "Empty field not allowed"),
            CsvError::InvalidField => write!(f, "Invalid field"),
            CsvError::TooManyFields => write!(f, "Too many fields"),
            CsvError::MissingFields(n) => write!(f, "{} fields missing", n),
            CsvError::UnexpectedEof => write!(f, "Unexpected end of file"),
        }
    }
}

impl Error for CsvError {}

impl From<io::Error> for CsvError {
    fn from(err: io::Error) -> CsvError {
        CsvError::Io(err)
    }
}

struct CsvFile {
    mode: char,
    fname: String,
    file: File,
    count: usize,
    c: Option<u8>,
    what: u8,
    field: String,
    nf: usize,
    refs: Vec<usize>,
    nskip: usize,
}

impl CsvFile {
    fn new(dca: &TabDca, mode: char) -> Result<Self, CsvError> {
        if dca.args.len() < 2 {
            return Err(CsvError::Parse("File name not specified".to_string()));
        }

        let fname = dca.args[1].clone();
        let file = match mode {
            'R' => File::open(&fname)?,
            'W' => File::create(&fname)?,
            _ => return Err(CsvError::Parse("Invalid mode".to_string())),
        };

        let mut csv = CsvFile {
            mode,
            fname,
            file,
            count: 0,
            c: Some(b'\n'),
            what: 0,
            field: String::new(),
            nf: 0,
            refs: vec![0; CSV_FIELD_MAX + 1],
            nskip: 0,
        };

        if mode == 'R' {
            csv.read_field()?;
            if csv.what != 1 {
                return Err(CsvError::Parse("Invalid file format".to_string()));
            }

            while csv.what != 1 {
                csv.read_field()?;
                if csv.what != 2 && csv.what != 3 {
                    return Err(CsvError::Parse("Invalid field name".to_string()));
                }
                if csv.nf == CSV_FIELD_MAX {
                    return Err(CsvError::TooManyFields);
                }
                csv.nf += 1;

                let mut found = 0;
                for k in (1..=dca.num_fields).rev() {
                    if dca.get_name(k) == csv.field {
                        found = k;
                        break;
                    }
                }
                csv.refs[csv.nf] = found;
            }

            for k in (1..=dca.num_fields).rev() {
                if dca.get_name(k) == "RECNO" {
                    csv.refs[0] = k;
                    break;
                }
            }
        } else if mode == 'W' {
            let nf = dca.num_fields;
            for k in 1..=nf {
                write!(csv.file, "{}", dca.get_name(k))?;
                if k < nf {
                    write!(csv.file, ",")?;
                } else {
                    writeln!(csv.file)?;
                }
            }
            csv.count += 1;
        }

        Ok(csv)
    }

    fn read_char(&mut self) -> Result<(), CsvError> {
        if self.c == Some(b'\n') {
            self.count += 1;
        }

        let mut buf = [0u8; 1];
        loop {
            match self.file.read_exact(&mut buf) {
                Ok(_) => {
                    let c = buf[0];
                    if c == b'\r' {
                        continue;
                    } else if c == b'\n' {
                        self.c = Some(c);
                        break;
                    } else if c.is_ascii_control() {
                        return Err(CsvError::Parse(format!(
                            "Invalid control character 0x{:02X}",
                            c
                        )));
                    }
                    self.c = Some(c);
                    break;
                }
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    if self.c == Some(b'\n') {
                        self.count -= 1;
                        self.c = None;
                    } else {
                        return Err(CsvError::UnexpectedEof);
                    }
                    break;
                }
                Err(e) => return Err(CsvError::Io(e)),
            }
        }
        Ok(())
    }

    fn read_field(&mut self) -> Result<(), CsvError> {
        self.field.clear();

        if self.c.is_none() {
            self.what = 0;
            self.field = "EOF".to_string();
            return Ok(());
        }

        if self.c == Some(b'\n') {
            self.what = 1;
            self.field = "EOR".to_string();
            self.read_char()?;
            if self.c == Some(b',') {
                return Err(CsvError::EmptyField);
            }
            if self.c == Some(b'\n') {
                return Err(CsvError::Parse("Empty record not allowed".to_string()));
            }
            if self.c == Some(b'#') && self.count == 1 {
                while self.c == Some(b'#') {
                    while self.c != Some(b'\n') {
                        self.read_char()?;
                    }
                    self.read_char()?;
                    self.nskip += 1;
                }
            }
            return Ok(());
        }

        if self.c == Some(b',') {
            self.read_char()?;
        }

        if self.c == Some(b'\'') || self.c == Some(b'"') {
            let quote = self.c.unwrap();
            self.what = 3;
            self.read_char()?;

            loop {
                if self.c == Some(quote) {
                    self.read_char()?;
                    if self.c == Some(quote) {
                        // Double quote - keep one
                    } else if self.c == Some(b',') || self.c == Some(b'\n') {
                        break;
                    } else {
                        return Err(CsvError::InvalidField);
                    }
                }

                if self.field.len() == CSV_FDLEN_MAX {
                    return Err(CsvError::FieldTooLong);
                }

                if let Some(c) = self.c {
                    self.field.push(c as char);
                }
                self.read_char()?;
            }

            if self.field.is_empty() {
                return Err(CsvError::EmptyField);
            }
        } else {
            self.what = 2;
            while !(self.c == Some(b',') || self.c == Some(b'\n')) {
                if self.c == Some(b'\'') || self.c == Some(b'"') {
                    return Err(CsvError::Parse(
                        "Invalid use of quote within field".to_string(),
                    ));
                }

                if self.field.len() == CSV_FDLEN_MAX {
                    return Err(CsvError::FieldTooLong);
                }

                if let Some(c) = self.c {
                    self.field.push(c as char);
                }
                self.read_char()?;
            }

            if self.field.is_empty() {
                return Err(CsvError::EmptyField);
            }

            if self.field.parse::<f64>().is_ok() {
                self.what = 3;
            }
        }

        Ok(())
    }

    fn read_record(&mut self, dca: &mut TabDca) -> Result<i32, CsvError> {
        if self.refs[0] > 0 {
            dca.set_num(self.refs[0], (self.count - self.nskip - 1) as f64);
        }

        for k in 1..=self.nf {
            self.read_field()?;
            match self.what {
                0 => return Ok(-1),
                1 => {
                    let lack = self.nf - k + 1;
                    return Err(if lack == 1 {
                        CsvError::MissingFields(1)
                    } else {
                        CsvError::MissingFields(lack)
                    });
                }
                2 => {
                    if self.refs[k] > 0 {
                        let num = self.field.parse().map_err(|_| {
                            CsvError::Parse("Invalid number".to_string())
                        })?;
                        dca.set_num(self.refs[k], num);
                    }
                }
                3 => {
                    if self.refs[k] > 0 {
                        dca.set_str(self.refs[k], &self.field);
                    }
                }
                _ => unreachable!(),
            }
        }

        self.read_field()?;
        if self.what != 1 {
            return Err(CsvError::TooManyFields);
        }

        Ok(0)
    }

    fn write_record(&mut self, dca: &TabDca) -> Result<(), CsvError> {
        let nf = dca.num_fields;
        for k in 1..=nf {
            match dca.get_type(k) {
                'N' => write!(self.file, "{:.15}", dca.get_num(k))?,
                'S' => {
                    write!(self.file, "\"")?;
                    for c in dca.get_str(k).chars() {
                        if c == '"' {
                            write!(self.file, "\"\"")?;
                        } else {
                            write!(self.file, "{}", c)?;
                        }
                    }
                    write!(self.file, "\"")?;
                }
                _ => unreachable!(),
            }
            if k < nf {
                write!(self.file, ",")?;
            } else {
                writeln!(self.file)?;
            }
        }
        self.count += 1;
        Ok(())
    }
}

#[derive(Debug)]
struct DbfFile {
    mode: char,
    fname: String,
    file: File,
    offset: usize,
    count: usize,
    nf: usize,
    refs: Vec<usize>,
    types: Vec<char>,
    lengths: Vec<usize>,
    precisions: Vec<usize>,
}

impl DbfFile {
    fn new(dca: &TabDca, mode: char) -> Result<Self, DbfError> {
        if dca.args.len() < 2 {
            return Err(DbfError::MissingFileName);
        }

        let fname = dca.args[1].clone();
        let mut dbf = DbfFile {
            mode,
            fname: fname.clone(),
            file: match mode {
                'R' => File::open(&fname)?,
                'W' => File::create(&fname)?,
                _ => return Err(DbfError::InvalidMode),
            },
            offset: 0,
            count: 0,
            nf: 0,
            refs: vec![0; DBF_FIELD_MAX + 1],
            types: vec!['\0'; DBF_FIELD_MAX + 1],
            lengths: vec![0; DBF_FIELD_MAX + 1],
            precisions: vec![0; DBF_FIELD_MAX + 1],
        };

        if mode == 'R' {
            dbf.read_header(dca)?;
            for k in (1..=dca.num_fields).rev() {
                if dca.get_name(k) == "RECNO" {
                    dbf.refs[0] = k;
                    break;
                }
            }
        } else if mode == 'W' {
            if dca.args.len() < 3 {
                return Err(DbfError::MissingFormat);
            }
            dbf.parse_format(dca)?;
            dbf.write_header(dca)?;
        }

        Ok(dbf)
    }

    fn read_byte(&mut self) -> Result<u8, DbfError> {
        let mut buf = [0u8; 1];
        self.file.read_exact(&mut buf).map_err(|e| {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                DbfError::UnexpectedEof(self.offset)
            } else {
                DbfError::Io(e)
            }
        })?;
        self.offset += 1;
        Ok(buf[0])
    }

    fn read_header(&mut self, dca: &TabDca) -> Result<(), DbfError> {
        // Skip first 10 bytes
        for _ in 0..10 {
            self.read_byte()?;
        }

        // Read record length
        let recl = self.read_byte()? as usize;
        let recl = recl + (self.read_byte()? as usize) << 8;

        // Skip next 20 bytes
        for _ in 0..20 {
            self.read_byte()?;
        }

        // Read field descriptors
        loop {
            let b = self.read_byte()?;
            if b == 0x0D {
                break;
            }
            if self.nf == DBF_FIELD_MAX {
                return Err(DbfError::TooManyFields);
            }
            self.nf += 1;

            // Read field name
            let mut name = String::new();
            name.push(b as char);
            for _ in 1..10 {
                name.push(self.read_byte()? as char);
            }
            if self.read_byte()? != 0x00 {
                return Err(DbfError::InvalidFieldName);
            }
            name = name.trim_end_matches('\0').to_string();

            // Find corresponding field in table
            let mut found = 0;
            for k in (1..=dca.num_fields).rev() {
                if dca.get_name(k) == name {
                    found = k;
                    break;
                }
            }
            self.refs[self.nf] = found;

            // Read field type
            let typ = self.read_byte()? as char;
            if typ != 'C' && typ != 'N' {
                return Err(DbfError::InvalidFieldType);
            }
            self.types[self.nf] = typ;

            // Skip 4 bytes
            for _ in 0..4 {
                self.read_byte()?;
            }

            // Read field length
            let len = self.read_byte()? as usize;
            if len == 0 || len > DBF_FDLEN_MAX {
                return Err(DbfError::InvalidFieldLength);
            }
            self.lengths[self.nf] = len;

            // Read field precision
            self.precisions[self.nf] = self.read_byte()? as usize;

            // Skip 14 bytes
            for _ in 0..14 {
                self.read_byte()?;
            }
        }

        Ok(())
    }

    fn parse_format(&mut self, dca: &TabDca) -> Result<(), DbfError> {
        self.nf = dca.num_fields;
        let format = &dca.args[2];
        let mut pos = 0;
        let chars: Vec<char> = format.chars().collect();

        for k in 1..=self.nf {
            // Parse field type
            if pos >= chars.len() {
                return Err(DbfError::MissingFieldSpec(k));
            }
            let typ = chars[pos];
            if typ != 'C' && typ != 'N' {
                return Err(DbfError::InvalidFieldType);
            }
            self.types[k] = typ;
            pos += 1;

            // Check for '('
            if pos >= chars.len() || chars[pos] != '(' {
                return Err(DbfError::InvalidFieldFormat(k));
            }
            pos += 1;

            // Parse field length
            let mut len = 0;
            while pos < chars.len() && chars[pos].is_ascii_digit() {
                len = len * 10 + chars[pos].to_digit(10).unwrap() as usize;
                if len > DBF_FDLEN_MAX {
                    break;
                }
                pos += 1;
            }
            if len == 0 || len > DBF_FDLEN_MAX {
                return Err(DbfError::InvalidFieldLength);
            }
            self.lengths[k] = len;

            // Parse optional precision for numeric fields
            if typ == 'N' && pos < chars.len() && chars[pos] == ',' {
                pos += 1;
                let mut prec = 0;
                while pos < chars.len() && chars[pos].is_ascii_digit() {
                    prec = prec * 10 + chars[pos].to_digit(10).unwrap() as usize;
                    if prec > len {
                        break;
                    }
                    pos += 1;
                }
                if prec > len {
                    return Err(DbfError::InvalidFieldPrecision(k));
                }
                self.precisions[k] = prec;
            }

            // Check for ')'
            if pos >= chars.len() || chars[pos] != ')' {
                return Err(DbfError::InvalidFieldFormat(k));
            }
            pos += 1;
        }

        Ok(())
    }

    fn write_byte(&mut self, b: u8) -> Result<(), DbfError> {
        self.file.write_all(&[b])?;
        self.offset += 1;
        Ok(())
    }

    fn write_header(&mut self, dca: &TabDca) -> Result<(), DbfError> {
        // Version number