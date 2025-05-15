use std::io::{self, Write, Read, Seek, SeekFrom};
use std::ptr;
use std::mem;
use std::time::SystemTime;
use std::convert::TryInto;

const ERROR: Position = Position::Error;
const INSIDE: Position = Position::Inside;
const APPEND: Position = Position::Append;
const OUTSIDE: Position = Position::Outside;

#[derive(Debug, PartialEq)]
enum Position {
    Outside,
    Append,
    Inside,
    Error,
}

struct Buffer {
    data: Vec<u8>,
    dirty: bool,
    ever_dirty: bool,
    dirty_pos: usize,
    dirty_end: usize,
    current: i64,
    cur_size: usize,
    sector_size: usize,
    cylinder_size: usize,
    next: Box<dyn Stream>,
}

trait Stream: Send + Sync {
    fn pread(&mut self, buf: &mut [u8], offset: i64) -> io::Result<usize>;
    fn pwrite(&mut self, buf: &[u8], offset: i64) -> io::Result<usize>;
    fn flush(&mut self) -> io::Result<()>;
    fn get_data(&self) -> io::Result<(SystemTime, u64, u32, u32)>;
}

impl Buffer {
    fn new(next: Box<dyn Stream>, size: usize, cylinder_size: usize, sector_size: usize) -> io::Result<Self> {
        if size == 0 || cylinder_size == 0 || sector_size == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "size parameters cannot be zero"));
        }
        if size % cylinder_size != 0 || cylinder_size % sector_size != 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "size must be multiple of cylinder size which must be multiple of sector size"));
        }

        Ok(Buffer {
            data: vec![0; size],
            dirty: false,
            ever_dirty: false,
            dirty_pos: 0,
            dirty_end: 0,
            current: 0,
            cur_size: 0,
            sector_size,
            cylinder_size,
            next,
        })
    }

    fn abs_pos(&self, rel: usize) -> i64 {
        self.current + rel as i64
    }

    fn cur_end(&self) -> i64 {
        self.abs_pos(self.cur_size)
    }

    fn pos_to_next_full_cyl(&self, pos: i64) -> usize {
        (self.cylinder_size - (pos % self.cylinder_size as i64) as usize)
    }

    fn flush_buf(&mut self) -> io::Result<()> {
        if !self.dirty {
            return Ok(());
        }

        let write_len = self.dirty_end - self.dirty_pos;
        let written = self.next.pwrite(
            &self.data[self.dirty_pos..self.dirty_end],
            self.current + self.dirty_pos as i64,
        )?;

        if written != write_len {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "buffer_flush: short write",
            ));
        }

        self.dirty = false;
        self.dirty_pos = 0;
        self.dirty_end = 0;
        Ok(())
    }

    fn invalidate_buffer(&mut self, start: i64) -> io::Result<()> {
        self.flush_buf()?;
        self.current = start - start % self.sector_size as i64;
        self.cur_size = 0;
        Ok(())
    }

    fn is_in_buffer(&mut self, start: i64, len: &mut usize) -> io::Result<Position> {
        if start >= self.current && start < self.cur_end() {
            let available = self.cur_size - (start - self.current) as usize;
            if *len > available {
                *len = available;
            }
            Ok(INSIDE)
        } else if start == self.cur_end() && self.cur_size < self.data.len() && *len >= self.sector_size {
            let available = self.data.len() - self.cur_size;
            if *len > available {
                *len = available;
            }
            *len -= *len % self.sector_size;
            Ok(APPEND)
        } else {
            self.invalidate_buffer(start)?;
            let cyl_remaining = self.cylinder_size - (start - self.current) as usize % self.cylinder_size;
            if *len > cyl_remaining {
                *len = cyl_remaining;
            }
            Ok(OUTSIDE)
        }
    }

    fn pread(&mut self, buf: &mut [u8], offset: i64) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let mut len = buf.len();
        match self.is_in_buffer(offset, &mut len)? {
            OUTSIDE | APPEND => {
                let fill_len = self.pos_to_next_full_cyl(self.cur_end())
                    .min(self.data.len() - self.cur_size);

                let read = self.next.pread(
                    &mut self.data[self.cur_size..self.cur_size + fill_len],
                    self.current + self.cur_size as i64,
                )?;

                self.cur_size += read;
                if (self.current + self.cur_size as i64) < offset {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Short buffer fill"));
                }
            }
            ERROR => return Err(io::Error::last_os_error()),
            INSIDE => {}
        }

        let offset_in_buf = (offset - self.current) as usize;
        let copy_len = len.min(self.cur_size - offset_in_buf);
        buf[..copy_len].copy_from_slice(&self.data[offset_in_buf..offset_in_buf + copy_len]);
        Ok(copy_len)
    }

    fn pwrite(&mut self, buf: &[u8], offset: i64) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        self.ever_dirty = true;
        let mut len = buf.len();
        let mut offset_in_buf = 0;

        match self.is_in_buffer(offset, &mut len)? {
            OUTSIDE => {
                if offset % self.cylinder_size as i64 != 0 || len < self.sector_size {
                    let read_size = self.cylinder_size - (self.current % self.cylinder_size as i64) as usize;
                    let read = self.next.pread(&mut self.data[..read_size], self.current)?;

                    if read % self.sector_size != 0 {
                        eprintln!("Weird: read size ({}) not a multiple of sector size ({})", 
                                 read, self.sector_size);
                        let aligned_read = read - read % self.sector_size;
                        if aligned_read == 0 {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "Nothing left after alignment"));
                        }
                        self.cur_size = aligned_read;
                    } else {
                        self.cur_size = read;
                    }

                    if self.cur_size == 0 {
                        self.data[..read_size].fill(0);
                        self.cur_size = read_size;
                    }
                    offset_in_buf = (offset - self.current) as usize;
                } else {
                    len -= len % self.sector_size;
                    offset_in_buf = (offset - self.current) as usize;
                    if len > self.data.len() - offset_in_buf {
                        len = self.data.len() - offset_in_buf;
                    }
                    self.cur_size += len;
                }
            }
            APPEND => {
                len -= len % self.sector_size;
                offset_in_buf = (offset - self.current) as usize;
                if len > self.data.len() - offset_in_buf {
                    len = self.data.len() - offset_in_buf;
                }
                self.cur_size += len;
            }
            INSIDE => {
                offset_in_buf = (offset - self.current) as usize;
                if len > self.cur_size - offset_in_buf {
                    len = self.cur_size - offset_in_buf;
                }
            }
            ERROR => return Err(io::Error::last_os_error()),
        }

        let end = offset_in_buf + len;
        if end > self.cur_size {
            len -= (end - self.cur_size) % self.sector_size;
            self.cur_size = len + offset_in_buf;
        }

        self.data[offset_in_buf..offset_in_buf + len].copy_from_slice(&buf[..len]);

        if !self.dirty || offset_in_buf < self.dirty_pos {
            self.dirty_pos = offset_in_buf - offset_in_buf % self.sector_size;
        }

        let new_dirty_end = offset_in_buf + len + self.sector_size - 1;
        let new_dirty_end = new_dirty_end - new_dirty_end % self.sector_size;
        if !self.dirty || offset_in_buf + len > self.dirty_end {
            self.dirty_end = new_dirty_end;
        }

        if self.dirty_end > self.cur_size {
            eprintln!("Internal error, dirty end too big dirty_end={} cur_size={} len={} offset={} sector_size={}",
                     self.dirty_end, self.cur_size, len, offset_in_buf, self.sector_size);
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Dirty end exceeds current size"));
        }

        self.dirty = true;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        if !self.ever_dirty {
            return Ok(());
        }

        self.flush_buf()?;
        self.ever_dirty = false;
        Ok(())
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}