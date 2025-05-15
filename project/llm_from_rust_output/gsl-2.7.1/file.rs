use std::fs::File;
use std::io::{Read, Write, Error, ErrorKind};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMemory = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for GslError {}

#[derive(Debug, Clone)]
pub struct Multiset {
    n: usize,
    k: usize,
    data: Vec<usize>,
}

impl Multiset {
    pub fn new(n: usize, k: usize) -> Self {
        Self {
            n,
            k,
            data: vec![0; k],
        }
    }

    pub fn read(&mut self, file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = vec![0u8; self.k * std::mem::size_of::<usize>()];
        file.read_exact(&mut buf)?;
        
        for (i, chunk) in buf.chunks_exact(std::mem::size_of::<usize>()).enumerate() {
            self.data[i] = usize::from_ne_bytes(chunk.try_into().unwrap());
        }
        
        Ok(())
    }

    pub fn write(&self, file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = Vec::with_capacity(self.k * std::mem::size_of::<usize>());
        
        for &value in &self.data {
            buf.extend_from_slice(&value.to_ne_bytes());
        }
        
        file.write_all(&buf)?;
        Ok(())
    }

    pub fn write_formatted(
        &self,
        file: &mut File,
        format: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for &value in &self.data {
            writeln!(file, format, value)?;
        }
        Ok(())
    }

    pub fn read_formatted(&mut self, file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
        for i in 0..self.k {
            let mut line = String::new();
            file.read_to_string(&mut line)?;
            self.data[i] = line.trim().parse()?;
        }
        Ok(())
    }
}