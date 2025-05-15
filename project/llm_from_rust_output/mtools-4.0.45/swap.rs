use std::mem;
use std::ptr;
use std::slice;

pub type mt_off_t = i64;
pub type ssize_t = isize;
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;

pub trait StreamOperations {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()>;
    fn write(&mut self, buf: &[u8]) -> Result<usize, ()>;
    fn pread(&mut self, buf: &mut [u8], offset: mt_off_t) -> Result<usize, ()>;
    fn pwrite(&mut self, buf: &[u8], offset: mt_off_t) -> Result<usize, ()>;
    fn flush(&mut self) -> Result<(), ()>;
    fn set_geom(&mut self, dev: &mut Device, orig_dev: &mut Device) -> Result<(), ()>;
    fn get_dos_convert(&mut self) -> Option<&mut DosConvert>;
}

pub struct DosConvert;
pub struct Device {
    pub name: String,
    pub drive: char,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: u16,
    pub sectors: u16,
    pub hidden: u32,
    pub offset: mt_off_t,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: u8,
    pub use_2m: u32,
    pub precmd: Option<String>,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: Option<String>,
    pub tot_sectors: u32,
    pub sector_size: u16,
    pub postcmd: Option<String>,
    pub cfg_filename: Option<String>,
}

pub struct SwapStream<T: StreamOperations> {
    inner: T,
}

impl<T: StreamOperations> SwapStream<T> {
    pub fn new(inner: T) -> Self {
        SwapStream { inner }
    }

    fn swap_buffer(buf: &mut [u8]) {
        for i in (0..buf.len()).step_by(2) {
            if i + 1 < buf.len() {
                buf.swap(i, i + 1);
            }
        }
    }
}

impl<T: StreamOperations> StreamOperations for SwapStream<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        let result = self.inner.read(buf)?;
        Self::swap_buffer(&mut buf[..result]);
        Ok(result)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        let mut swapped = buf.to_vec();
        Self::swap_buffer(&mut swapped);
        self.inner.write(&swapped)
    }

    fn pread(&mut self, buf: &mut [u8], offset: mt_off_t) -> Result<usize, ()> {
        let result = self.inner.pread(buf, offset)?;
        Self::swap_buffer(&mut buf[..result]);
        Ok(result)
    }

    fn pwrite(&mut self, buf: &[u8], offset: mt_off_t) -> Result<usize, ()> {
        let mut swapped = buf.to_vec();
        Self::swap_buffer(&mut swapped);
        self.inner.pwrite(&swapped, offset)
    }

    fn flush(&mut self) -> Result<(), ()> {
        self.inner.flush()
    }

    fn set_geom(&mut self, dev: &mut Device, orig_dev: &mut Device) -> Result<(), ()> {
        self.inner.set_geom(dev, orig_dev)
    }

    fn get_dos_convert(&mut self) -> Option<&mut DosConvert> {
        self.inner.get_dos_convert()
    }
}

pub fn open_swap<T: StreamOperations>(inner: T) -> SwapStream<T> {
    SwapStream::new(inner)
}