use std::ffi::CString;
use std::ptr;
use std::time::SystemTime;

type MtOffT = i64;
type SSizeT = i64;
type SizeT = usize;
type UInt8T = u8;
type UInt16T = u16;
type UInt32T = u32;

#[derive(Clone)]
struct Device {
    name: Option<CString>,
    drive: char,
    fat_bits: i32,
    mode: i32,
    tracks: u32,
    heads: UInt16T,
    sectors: UInt16T,
    hidden: u32,
    offset: MtOffT,
    partition: u32,
    misc_flags: u32,
    ssize: UInt8T,
    use_2m: u32,
    precmd: Option<CString>,
    file_nr: i32,
    blocksize: u32,
    codepage: u32,
    data_map: Option<CString>,
    tot_sectors: UInt32T,
    sector_size: UInt16T,
    postcmd: Option<CString>,
    cfg_filename: Option<CString>,
}

trait StreamOperations {
    fn read(&mut self, buf: &mut [u8]) -> Result<SSizeT, i32>;
    fn write(&mut self, buf: &[u8]) -> Result<SSizeT, i32>;
    fn pread(&mut self, buf: &mut [u8], start: MtOffT) -> Result<SSizeT, i32>;
    fn pwrite(&mut self, buf: &[u8], start: MtOffT) -> Result<SSizeT, i32>;
    fn flush(&mut self) -> Result<(), i32>;
    fn free(&mut self) -> Result<(), i32>;
    fn set_geom(&mut self, dev: &Device, orig_dev: &Device) -> Result<(), i32>;
    fn get_data(&mut self) -> Result<(SystemTime, MtOffT, i32, UInt32T), i32>;
    fn pre_allocate(&mut self, size: MtOffT) -> Result<(), i32>;
    fn get_dos_convert(&mut self) -> Result<(), i32>;
    fn discard(&mut self) -> Result<(), i32>;
}

struct Stream {
    operations: Box<dyn StreamOperations>,
    refs: i32,
    next: Option<Box<Stream>>,
}

impl Stream {
    fn new(operations: Box<dyn StreamOperations>, next: Option<Box<Stream>>) -> Self {
        Stream {
            operations,
            refs: 1,
            next,
        }
    }

    fn copy(&mut self) {
        self.refs += 1;
    }

    fn flush(&mut self) -> Result<(), i32> {
        if batchmode() == 0 {
            self.operations.flush()?;
            if let Some(next) = &mut self.next {
                next.flush()?;
            }
        }
        Ok(())
    }

    fn free(&mut self) -> Result<(), i32> {
        self.refs -= 1;
        if self.refs == 0 {
            self.operations.flush()?;
            self.operations.free()?;
            if let Some(next) = &mut self.next {
                next.free()?;
            }
        }
        Ok(())
    }
}

fn batchmode() -> i32 {
    0 // Placeholder for actual implementation
}

fn limit_size_to_off_t(len: &mut SizeT, max_len: MtOffT) {
    if *len > max_len as SizeT {
        *len = max_len as SizeT;
    }
}

fn adjust_tot_sectors(dev: &mut Device, offset: MtOffT) -> Result<(), String> {
    if dev.tot_sectors == 0 {
        return Ok(());
    }

    let sector_size = if dev.sector_size != 0 {
        dev.sector_size as i64
    } else {
        512
    };
    let offs_sectors = offset / sector_size;

    if offs_sectors > 0 && (dev.tot_sectors as i64) < offs_sectors {
        return Err("init: Offset bigger than base image".to_string());
    }

    dev.tot_sectors = dev.tot_sectors.saturating_sub(offs_sectors as UInt32T);
    Ok(())
}