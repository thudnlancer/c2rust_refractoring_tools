use std::io::{self, Write, Read};
use std::fmt;

#[derive(Debug)]
pub enum HistogramError {
    IoError(io::Error),
    FormatError,
}

impl From<io::Error> for HistogramError {
    fn from(err: io::Error) -> Self {
        HistogramError::IoError(err)
    }
}

pub struct Histogram2D {
    pub xrange: Vec<f64>,
    pub yrange: Vec<f64>,
    pub bin: Vec<f64>,
    pub nx: usize,
    pub ny: usize,
}

impl Histogram2D {
    pub fn fread<R: Read>(stream: &mut R, h: &mut Histogram2D) -> Result<(), HistogramError> {
        Self::read_raw(stream, &mut h.xrange, h.nx + 1)?;
        Self::read_raw(stream, &mut h.yrange, h.ny + 1)?;
        Self::read_raw(stream, &mut h.bin, h.nx * h.ny)?;
        Ok(())
    }

    pub fn fwrite<W: Write>(&self, stream: &mut W) -> Result<(), HistogramError> {
        Self::write_raw(stream, &self.xrange)?;
        Self::write_raw(stream, &self.yrange)?;
        Self::write_raw(stream, &self.bin)?;
        Ok(())
    }

    pub fn fprintf<W: Write>(
        &self,
        stream: &mut W,
        range_format: &str,
        bin_format: &str,
    ) -> Result<(), HistogramError> {
        for i in 0..self.nx {
            for j in 0..self.ny {
                write!(stream, "{} ", format_args!(range_format, self.xrange[i]))?;
                write!(stream, "{} ", format_args!(range_format, self.xrange[i + 1]))?;
                write!(stream, "{} ", format_args!(range_format, self.yrange[j]))?;
                write!(stream, "{} ", format_args!(range_format, self.yrange[j + 1]))?;
                writeln!(stream, "{}", format_args!(bin_format, self.bin[i * self.ny + j]))?;
            }
            writeln!(stream)?;
        }
        Ok(())
    }

    pub fn fscanf<R: Read>(stream: &mut R, h: &mut Histogram2D) -> Result<(), HistogramError> {
        let mut xupper = 0.0;
        let mut yupper = 0.0;

        for i in 0..h.nx {
            for j in 0..h.ny {
                let mut line = String::new();
                stream.read_to_string(&mut line)?;
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() != 5 {
                    return Err(HistogramError::FormatError);
                }

                h.xrange[i] = parts[0].parse().map_err(|_| HistogramError::FormatError)?;
                xupper = parts[1].parse().map_err(|_| HistogramError::FormatError)?;
                h.yrange[j] = parts[2].parse().map_err(|_| HistogramError::FormatError)?;
                yupper = parts[3].parse().map_err(|_| HistogramError::FormatError)?;
                h.bin[i * h.ny + j] = parts[4].parse().map_err(|_| HistogramError::FormatError)?;
            }
            h.yrange[h.ny] = yupper;
        }
        h.xrange[h.nx] = xupper;

        Ok(())
    }

    fn read_raw<R: Read>(stream: &mut R, buffer: &mut [f64], count: usize) -> Result<(), HistogramError> {
        let mut bytes = vec![0u8; count * 8];
        stream.read_exact(&mut bytes)?;
        for i in 0..count {
            let mut arr = [0u8; 8];
            arr.copy_from_slice(&bytes[i*8..(i+1)*8]);
            buffer[i] = f64::from_le_bytes(arr);
        }
        Ok(())
    }

    fn write_raw<W: Write>(&self, stream: &mut W, data: &[f64]) -> Result<(), HistogramError> {
        for &value in data {
            stream.write_all(&value.to_le_bytes())?;
        }
        Ok(())
    }
}