use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use std::ptr;
use std::mem;
use std::path::Path;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GslError {
    NoMem,
    Failed,
    Eof,
    Io(std::io::Error),
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GslError::NoMem => write!(f, "failed to allocate space"),
            GslError::Failed => write!(f, "operation failed"),
            GslError::Eof => write!(f, "end of file reached"),
            GslError::Io(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for GslError {}

impl From<std::io::Error> for GslError {
    fn from(err: std::io::Error) -> GslError {
        GslError::Io(err)
    }
}

type GslResult<T> = Result<T, GslError>;

pub struct GslNtuple<T> {
    file: File,
    ntuple_data: T,
    size: usize,
}

pub type GslNtupleValueFn<T> = dyn Fn(&T) -> f64;
pub type GslNtupleSelectFn<T> = dyn Fn(&T) -> bool;

pub struct GslHistogram;

impl GslHistogram {
    pub fn increment(&mut self, _value: f64) {
        // Implementation omitted for brevity
    }
}

impl<T> GslNtuple<T> {
    pub fn create(filename: &Path, ntuple_data: T, size: usize) -> GslResult<Self> {
        let file = File::create(filename).map_err(GslError::Io)?;
        Ok(GslNtuple {
            file,
            ntuple_data,
            size,
        })
    }

    pub fn open(filename: &Path, ntuple_data: T, size: usize) -> GslResult<Self> {
        let file = File::open(filename).map_err(GslError::Io)?;
        Ok(GslNtuple {
            file,
            ntuple_data,
            size,
        })
    }

    pub fn write(&mut self) -> GslResult<()> {
        let data_ptr = &self.ntuple_data as *const _ as *const u8;
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, self.size) };
        self.file.write_all(data_slice).map_err(GslError::Io)?;
        Ok(())
    }

    pub fn bookdata(&mut self) -> GslResult<()> {
        self.write()
    }

    pub fn read(&mut self) -> GslResult<()> {
        let data_ptr = &mut self.ntuple_data as *mut _ as *mut u8;
        let data_slice = unsafe { std::slice::from_raw_parts_mut(data_ptr, self.size) };
        
        match self.file.read_exact(data_slice) {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Err(GslError::Eof),
            Err(e) => Err(GslError::Io(e)),
        }
    }

    pub fn project(
        &mut self,
        h: &mut GslHistogram,
        value_func: &GslNtupleValueFn<T>,
        select_func: &GslNtupleSelectFn<T>,
    ) -> GslResult<()> {
        loop {
            match self.read() {
                Ok(_) => {
                    if select_func(&self.ntuple_data) {
                        h.increment(value_func(&self.ntuple_data));
                    }
                }
                Err(GslError::Eof) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub fn close(self) -> GslResult<()> {
        // File is automatically closed when dropped
        Ok(())
    }
}