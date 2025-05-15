use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::Path;
use std::ptr;
use libc::{c_int, c_char, c_void, size_t};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

const BUFFER_SIZE: usize = 8192;
const TBUF_SIZE: usize = 4096;

#[derive(Debug)]
pub enum GlpFileMode {
    Read,
    Write,
    Append,
}

#[derive(Debug)]
pub enum GlpFileType {
    Null,
    Stdin,
    Stdout,
    Stderr,
    Regular(File),
    Gzip(Box<dyn Read>, Box<dyn Write>),
}

#[derive(Debug)]
pub struct GlpFile {
    buffer: Vec<u8>,
    position: usize,
    count: usize,
    mode: GlpFileMode,
    file_type: GlpFileType,
    eof: bool,
    error: bool,
}

impl GlpFile {
    pub fn open(name: &str, mode: &str) -> io::Result<Self> {
        let file_mode = match mode {
            "r" | "rb" => GlpFileMode::Read,
            "w" | "wb" => GlpFileMode::Write,
            "a" | "ab" => GlpFileMode::Append,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid mode string")),
        };

        let file_type = match name {
            "/dev/null" => GlpFileType::Null,
            "/dev/stdin" => GlpFileType::Stdin,
            "/dev/stdout" => GlpFileType::Stdout,
            "/dev/stderr" => GlpFileType::Stderr,
            _ => {
                if name.ends_with(".gz") {
                    let mode = match mode {
                        "r" => "rb",
                        "w" => "wb",
                        "a" => "ab",
                        _ => mode,
                    };
                    match file_mode {
                        GlpFileMode::Read => {
                            let file = File::open(name)?;
                            let decoder = GzDecoder::new(file);
                            GlpFileType::Gzip(Box::new(decoder), Box::new(io::sink()))
                        },
                        GlpFileMode::Write | GlpFileMode::Append => {
                            let file = File::create(name)?;
                            let encoder = GzEncoder::new(file, Compression::default());
                            GlpFileType::Gzip(Box::new(io::empty()), Box::new(encoder))
                        },
                    }
                } else {
                    match file_mode {
                        GlpFileMode::Read => {
                            let file = File::open(name)?;
                            GlpFileType::Regular(file)
                        },
                        GlpFileMode::Write => {
                            let file = File::create(name)?;
                            GlpFileType::Regular(file)
                        },
                        GlpFileMode::Append => {
                            let file = std::fs::OpenOptions::new()
                                .append(true)
                                .open(name)?;
                            GlpFileType::Regular(file)
                        },
                    }
                }
            }
        };

        Ok(Self {
            buffer: vec![0; BUFFER_SIZE],
            position: 0,
            count: 0,
            mode: file_mode,
            file_type,
            eof: false,
            error: false,
        })
    }

    pub fn eof(&self) -> bool {
        self.eof
    }

    pub fn error(&self) -> bool {
        self.error
    }

    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let GlpFileMode::Write | GlpFileMode::Append = self.mode {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Attempt to read from output stream"));
        }

        if buf.is_empty() {
            return Ok(0);
        }

        let mut total_read = 0;
        while total_read < buf.len() {
            if self.count == 0 {
                let bytes_read = match &mut self.file_type {
                    GlpFileType::Null => 0,
                    GlpFileType::Stdin => {
                        let mut stdin = io::stdin();
                        stdin.read(&mut self.buffer)?
                    },
                    GlpFileType::Regular(file) => file.read(&mut self.buffer)?,
                    GlpFileType::Gzip(reader, _) => reader.read(&mut self.buffer)?,
                    _ => return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Invalid file type for reading")),
                };

                if bytes_read == 0 {
                    self.eof = true;
                    break;
                }
                self.position = 0;
                self.count = bytes_read;
            }

            let to_copy = std::cmp::min(self.count, buf.len() - total_read);
            buf[total_read..total_read + to_copy].copy_from_slice(&self.buffer[self.position..self.position + to_copy]);
            self.position += to_copy;
            self.count -= to_copy;
            total_read += to_copy;
        }

        Ok(total_read)
    }

    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let GlpFileMode::Read = self.mode {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Attempt to write to input stream"));
        }

        if buf.is_empty() {
            return Ok(0);
        }

        let mut total_written = 0;
        while total_written < buf.len() {
            let available = self.buffer.len() - self.position;
            let to_copy = std::cmp::min(available, buf.len() - total_written);
            
            if to_copy == 0 {
                self.flush()?;
                continue;
            }

            self.buffer[self.position..self.position + to_copy].copy_from_slice(&buf[total_written..total_written + to_copy]);
            self.position += to_copy;
            total_written += to_copy;
        }

        Ok(total_written)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        if self.position > 0 {
            match &mut self.file_type {
                GlpFileType::Null => {},
                GlpFileType::Stdout => {
                    io::stdout().write_all(&self.buffer[..self.position])?;
                },
                GlpFileType::Stderr => {
                    io::stderr().write_all(&self.buffer[..self.position])?;
                },
                GlpFileType::Regular(file) => {
                    file.write_all(&self.buffer[..self.position])?;
                },
                GlpFileType::Gzip(_, writer) => {
                    writer.write_all(&self.buffer[..self.position])?;
                },
            }
            self.position = 0;
        }
        Ok(())
    }

    pub fn close(mut self) -> io::Result<()> {
        self.flush()?;
        Ok(())
    }
}

impl Drop for GlpFile {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}