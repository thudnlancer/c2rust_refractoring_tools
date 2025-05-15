use std::fmt;
use std::io::{self, Write};
use std::slice;

#[derive(Debug)]
pub enum GslError {
    Failed,
    Eof,
    // ... other error variants
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GslError::Failed => write!(f, "operation failed"),
            GslError::Eof => write!(f, "end of file reached"),
            // ... other error variants
        }
    }
}

impl std::error::Error for GslError {}

#[derive(Debug)]
pub struct Histogram2D {
    nx: usize,
    ny: usize,
    xrange: Vec<f64>,
    yrange: Vec<f64>,
    bin: Vec<f64>,
}

impl Histogram2D {
    pub fn new(nx: usize, ny: usize) -> Self {
        Histogram2D {
            nx,
            ny,
            xrange: vec![0.0; nx + 1],
            yrange: vec![0.0; ny + 1],
            bin: vec![0.0; nx * ny],
        }
    }

    pub fn fread<R: io::Read>(&mut self, mut stream: R) -> Result<(), GslError> {
        let xrange_slice = unsafe {
            slice::from_raw_parts_mut(self.xrange.as_mut_ptr(), self.nx + 1)
        };
        let yrange_slice = unsafe {
            slice::from_raw_parts_mut(self.yrange.as_mut_ptr(), self.ny + 1)
        };
        let bin_slice = unsafe {
            slice::from_raw_parts_mut(self.bin.as_mut_ptr(), self.nx * self.ny)
        };

        unsafe {
            if gsl_block_raw_fread(&mut stream, xrange_slice).is_err() {
                return Err(GslError::Failed);
            }
            if gsl_block_raw_fread(&mut stream, yrange_slice).is_err() {
                return Err(GslError::Failed);
            }
            if gsl_block_raw_fread(&mut stream, bin_slice).is_err() {
                return Err(GslError::Failed);
            }
        }

        Ok(())
    }

    pub fn fwrite<W: io::Write>(&self, mut stream: W) -> Result<(), GslError> {
        let xrange_slice = unsafe {
            slice::from_raw_parts(self.xrange.as_ptr(), self.nx + 1)
        };
        let yrange_slice = unsafe {
            slice::from_raw_parts(self.yrange.as_ptr(), self.ny + 1)
        };
        let bin_slice = unsafe {
            slice::from_raw_parts(self.bin.as_ptr(), self.nx * self.ny)
        };

        unsafe {
            if gsl_block_raw_fwrite(&mut stream, xrange_slice).is_err() {
                return Err(GslError::Failed);
            }
            if gsl_block_raw_fwrite(&mut stream, yrange_slice).is_err() {
                return Err(GslError::Failed);
            }
            if gsl_block_raw_fwrite(&mut stream, bin_slice).is_err() {
                return Err(GslError::Failed);
            }
        }

        Ok(())
    }

    pub fn fprintf<W: Write>(
        &self,
        mut stream: W,
        range_format: &str,
        bin_format: &str,
    ) -> Result<(), GslError> {
        for i in 0..self.nx {
            for j in 0..self.ny {
                write!(stream, "{} ", format!(range_format, self.xrange[i]))
                    .map_err(|_| GslError::Failed)?;
                write!(stream, " ").map_err(|_| GslError::Failed)?;
                write!(stream, "{} ", format!(range_format, self.xrange[i + 1]))
                    .map_err(|_| GslError::Failed)?;
                write!(stream, " ").map_err(|_| GslError::Failed)?;
                write!(stream, "{} ", format!(range_format, self.yrange[j]))
                    .map_err(|_| GslError::Failed)?;
                write!(stream, " ").map_err(|_| GslError::Failed)?;
                write!(stream, "{} ", format!(range_format, self.yrange[j + 1]))
                    .map_err(|_| GslError::Failed)?;
                write!(stream, " ").map_err(|_| GslError::Failed)?;
                write!(
                    stream,
                    "{}\n",
                    format!(bin_format, self.bin[i * self.ny + j])
                )
                .map_err(|_| GslError::Failed)?;
            }
            write!(stream, "\n").map_err(|_| GslError::Failed)?;
        }
        Ok(())
    }

    pub fn fscanf<R: io::Read>(&mut self, mut stream: R) -> Result<(), GslError> {
        let mut xupper = 0.0;
        let mut yupper = 0.0;

        for i in 0..self.nx {
            for j in 0..self.ny {
                let mut line = String::new();
                stream.read_to_string(&mut line).map_err(|_| GslError::Failed)?;
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.len() != 5 {
                    return Err(GslError::Failed);
                }

                self.xrange[i] = parts[0].parse().map_err(|_| GslError::Failed)?;
                xupper = parts[1].parse().map_err(|_| GslError::Failed)?;
                self.yrange[j] = parts[2].parse().map_err(|_| GslError::Failed)?;
                yupper = parts[3].parse().map_err(|_| GslError::Failed)?;
                self.bin[i * self.ny + j] = parts[4].parse().map_err(|_| GslError::Failed)?;
            }
            self.yrange[self.ny] = yupper;
        }
        self.xrange[self.nx] = xupper;

        Ok(())
    }
}

// Safe wrappers around unsafe operations
fn gsl_block_raw_fread<R: io::Read>(
    stream: &mut R,
    buffer: &mut [f64],
) -> Result<(), GslError> {
    unsafe {
        let ptr = buffer.as_mut_ptr();
        let size = buffer.len();
        let mut temp_buffer = vec![0u8; size * std::mem::size_of::<f64>()];
        stream.read_exact(&mut temp_buffer).map_err(|_| GslError::Failed)?;
        std::ptr::copy_nonoverlapping(
            temp_buffer.as_ptr() as *const f64,
            ptr,
            size,
        );
    }
    Ok(())
}

fn gsl_block_raw_fwrite<W: io::Write>(
    stream: &mut W,
    buffer: &[f64],
) -> Result<(), GslError> {
    unsafe {
        let ptr = buffer.as_ptr();
        let size = buffer.len();
        let temp_buffer = slice::from_raw_parts(ptr as *const u8, size * std::mem::size_of::<f64>());
        stream.write_all(temp_buffer).map_err(|_| GslError::Failed)?;
    }
    Ok(())
}