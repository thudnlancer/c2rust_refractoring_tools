use std::io::{self, Read, Write, Seek, SeekFrom};
use std::cmp::{min, max};
use std::ptr;
use std::mem;
use std::os::raw::c_void;

struct Stream {
    // Placeholder for Stream implementation
}

struct Buffer {
    head: Stream,
    size: usize,
    dirty: bool,
    sector_size: usize,
    cylinder_size: usize,
    ever_dirty: bool,
    dirty_pos: usize,
    dirty_end: usize,
    current: u64,
    cur_size: usize,
    buf: Vec<u8>,
}

impl Buffer {
    fn abs_pos(&self, rel: usize) -> u64 {
        self.current + rel as u64
    }

    fn cur_end(&self) -> u64 {
        self.abs_pos(self.cur_size)
    }

    fn pos_to_next_full_cyl(&self, pos: u64) -> usize {
        (self.cylinder_size - (pos % self.cylinder_size as u64) as usize)
    }

    fn mt_buf_flush(&mut self) -> io::Result<()> {
        if !self.dirty {
            return Ok(());
        }

        let next = unsafe { &mut *self.head.next };
        let data = &self.buf[self.dirty_pos..self.dirty_end];
        let pos = self.current + self.dirty_pos as u64;
        
        next.seek(SeekFrom::Start(pos))?;
        next.write_all(data)?;

        self.dirty = false;
        self.dirty_end = 0;
        self.dirty_pos = 0;
        Ok(())
    }

    fn invalidate_buffer(&mut self, start: u64) -> io::Result<()> {
        self.mt_buf_flush()?;
        self.current = (start / self.sector_size as u64) * self.sector_size as u64;
        self.cur_size = 0;
        Ok(())
    }

    fn is_in_buffer(&mut self, start: u64, len: &mut usize) -> io::Result<Position> {
        if start >= self.current && start < self.cur_end() {
            *len = min(*len, self.cur_size - (start - self.current) as usize);
            Ok(Position::Inside)
        } else if start == self.cur_end() && 
                  self.cur_size < self.size && 
                  *len >= self.sector_size {
            *len = min(*len, self.size - self.cur_size);
            *len = (*len / self.sector_size) * self.sector_size;
            Ok(Position::Append)
        } else {
            self.invalidate_buffer(start)?;
            *len = min(*len, self.cylinder_size - (start - self.current) as usize);
            *len = min(*len, self.pos_to_next_full_cyl(self.current));
            Ok(Position::Outside)
        }
    }

    fn pread(&mut self, buf: &mut [u8], start: u64) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let mut len = buf.len();
        match self.is_in_buffer(start, &mut len)? {
            Position::Outside | Position::Append => {
                let length = self.pos_to_next_full_cyl(self.cur_end());
                let length = min(length, self.size - self.cur_size);

                let next = unsafe { &mut *self.head.next };
                next.seek(SeekFrom::Start(self.current + self.cur_size as u64))?;
                let read_len = next.read(&mut self.buf[self.cur_size..self.cur_size + length])?;
                
                self.cur_size += read_len;
                if self.current + self.cur_size as u64 < start {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Short buffer fill"));
                }
            }
            Position::Inside => {}
            Position::Error => return Err(io::Error::new(io::ErrorKind::Other, "Buffer error")),
        }

        let offset = (start - self.current) as usize;
        let disk_ptr = &self.buf[offset..offset + len];
        buf[..len].copy_from_slice(disk_ptr);
        Ok(len)
    }

    fn pwrite(&mut self, buf: &[u8], start: u64) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        self.ever_dirty = true;
        let mut len = buf.len();
        let offset = match self.is_in_buffer(start, &mut len)? {
            Position::Outside => {
                if start % self.cylinder_size as u64 != 0 || len < self.sector_size {
                    let read_size = self.cylinder_size - 
                        (self.current % self.cylinder_size as u64) as usize;

                    let next = unsafe { &mut *self.head.next };
                    next.seek(SeekFrom::Start(self.current))?;
                    let bytes_read = next.read(&mut self.buf[..read_size])?;

                    if bytes_read % self.sector_size != 0 {
                        let adjusted = (bytes_read / self.sector_size) * self.sector_size;
                        if adjusted == 0 {
                            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Nothing left"));
                        }
                        self.cur_size = adjusted;
                    } else {
                        self.cur_size = bytes_read;
                    }

                    if self.cur_size == 0 {
                        self.buf[..read_size].fill(0);
                        self.cur_size = read_size;
                    }
                    (start - self.current) as usize
                } else {
                    len = (len / self.sector_size) * self.sector_size;
                    let offset = (start - self.current) as usize;
                    len = min(len, self.size - offset);
                    self.cur_size += len;
                    if let Some(pre_allocate) = self.head.next.as_ref().unwrap().pre_allocate {
                        pre_allocate(self.head.next.as_mut().unwrap(), self.cur_end())?;
                    }
                    offset
                }
            }
            Position::Append => {
                len = (len / self.sector_size) * self.sector_size;
                let offset = (start - self.current) as usize;
                len = min(len, self.size - offset);
                self.cur_size += len;
                if let Some(pre_allocate) = self.head.next.as_ref().unwrap().pre_allocate {
                    pre_allocate(self.head.next.as_mut().unwrap(), self.cur_end())?;
                }
                offset
            }
            Position::Inside => {
                let offset = (start - self.current) as usize;
                len = min(len, self.cur_size - offset);
                offset
            }
            Position::Error => return Err(io::Error::new(io::ErrorKind::Other, "Buffer error")),
        };

        let disk_ptr = &mut self.buf[offset..offset + len];
        disk_ptr.copy_from_slice(&buf[..len]);

        if !self.dirty || offset < self.dirty_pos {
            self.dirty_pos = (offset / self.sector_size) * self.sector_size;
        }
        if !self.dirty || offset + len > self.dirty_end {
            self.dirty_end = ((offset + len + self.sector_size - 1) / self.sector_size) * self.sector_size;
        }

        if self.dirty_end > self.cur_size {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Dirty end too big"));
        }

        self.dirty = true;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        if !self.ever_dirty {
            return Ok(());
        }
        let ret = self.mt_buf_flush();
        if ret.is_ok() {
            self.ever_dirty = false;
        }
        ret
    }
}

enum Position {
    Outside,
    Append,
    Inside,
    Error,
}

fn buf_init(next: Box<Stream>, size: usize, cylinder_size: usize, sector_size: usize) -> io::Result<Box<Buffer>> {
    if size == 0 || cylinder_size == 0 || sector_size == 0 || next.is_null() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid parameters"));
    }

    if size % cylinder_size != 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Size not multiple of cylinder size"));
    }
    if cylinder_size % sector_size != 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cylinder size not multiple of sector size"));
    }

    let mut buffer = Box::new(Buffer {
        head: Stream::new(next),
        size,
        dirty: false,
        sector_size,
        cylinder_size,
        ever_dirty: false,
        dirty_pos: 0,
        dirty_end: 0,
        current: 0,
        cur_size: 0,
        buf: vec![0; size],
    });

    Ok(buffer)
}