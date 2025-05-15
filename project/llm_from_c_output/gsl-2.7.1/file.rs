use std::io::{Read, Write, Error, ErrorKind};
use std::fmt;

pub struct Combination {
    k: usize,
    data: Vec<usize>,
}

impl Combination {
    pub fn new(k: usize) -> Self {
        Combination {
            k,
            data: vec![0; k],
        }
    }

    pub fn fread<R: Read>(&mut self, stream: &mut R) -> Result<(), Error> {
        let mut buf = vec![0u8; self.k * std::mem::size_of::<usize>()];
        stream.read_exact(&mut buf)?;
        
        unsafe {
            let data_ptr = self.data.as_mut_ptr() as *mut u8;
            std::ptr::copy_nonoverlapping(
                buf.as_ptr(),
                data_ptr,
                buf.len()
            );
        }
        
        Ok(())
    }

    pub fn fwrite<W: Write>(&self, stream: &mut W) -> Result<(), Error> {
        let buf = unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr() as *const u8,
                self.k * std::mem::size_of::<usize>()
            )
        };
        stream.write_all(buf)?;
        Ok(())
    }

    pub fn fprintf<W: Write>(&self, stream: &mut W, format: &str) -> Result<(), Error> {
        for &item in &self.data {
            write!(stream, format, item)?;
        }
        Ok(())
    }

    pub fn fscanf<R: Read>(&mut self, stream: &mut R) -> Result<(), Error> {
        let mut input = String::new();
        stream.read_to_string(&mut input)?;
        
        let mut numbers = input.split_whitespace();
        for i in 0..self.k {
            let num = numbers.next()
                .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Not enough numbers"))?
                .parse::<usize>()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            self.data[i] = num;
        }
        
        if numbers.next().is_some() {
            return Err(Error::new(ErrorKind::InvalidData, "Too many numbers"));
        }
        
        Ok(())
    }
}

impl fmt::Display for Combination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, &item) in self.data.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}