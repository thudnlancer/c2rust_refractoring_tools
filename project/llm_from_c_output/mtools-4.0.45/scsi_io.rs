use std::os::unix::io::{RawFd, AsRawFd};
use std::ffi::{CStr, CString};
use std::ptr;
use std::mem;
use std::io::{self, Error, ErrorKind};
use std::time::SystemTime;
use libc::{c_char, c_int, c_void, size_t, ssize_t};

#[cfg(feature = "scsi")]
mod scsi {
    use super::*;
    
    #[repr(C)]
    pub struct Device {
        // Device fields from C code
    }
    
    #[repr(C)]
    pub struct Stream {
        // Stream fields from C code
    }
    
    pub type Stream_t = Stream;
    
    #[derive(Debug)]
    pub struct ScsiDevice {
        head: Stream_t,
        fd: RawFd,
        privileged: bool,
        scsi_sector_size: u32,
        device_size: i64,
        tot_sectors: u32,
        extra_data: *mut c_void,
        postcmd: Option<CString>,
    }
    
    impl ScsiDevice {
        fn init(&mut self) -> io::Result<()> {
            let mut cdb = [0u8; 10];
            let mut buf = [0u8; 8];
            
            cdb[0] = SCSI_READ_CAPACITY;
            
            unsafe {
                if scsi_cmd(
                    self.fd,
                    cdb.as_ptr(),
                    cdb.len(),
                    SCSI_IO_READ,
                    buf.as_mut_ptr(),
                    buf.len(),
                    self.extra_data,
                ) == 0
                {
                    self.tot_sectors = u32::from(buf[0]) << 24
                        | u32::from(buf[1]) << 16
                        | u32::from(buf[2]) << 8
                        | u32::from(buf[3]);
                    
                    if self.tot_sectors < u32::MAX {
                        self.tot_sectors += 1;
                    }
                    
                    self.scsi_sector_size = u32::from(buf[5]) << 16
                        | u32::from(buf[6]) << 8
                        | u32::from(buf[7]);
                    
                    if self.scsi_sector_size != 512 {
                        eprintln!("  (scsi_sector_size={})", self.scsi_sector_size);
                    }
                    Ok(())
                } else {
                    Err(Error::last_os_error())
                }
            }
        }
        
        fn bytes_to_sectors(bytes: usize, sector_size: u32) -> u32 {
            let mut sectors = bytes / sector_size as usize;
            if bytes % sector_size as usize != 0 {
                sectors += 1;
            }
            if sectors > u32::MAX as usize {
                u32::MAX
            } else {
                sectors as u32
            }
        }
        
        fn io(
            &mut self,
            buf: &mut [u8],
            where_: i64,
            len: usize,
            rwcmd: ScsiIoMode,
        ) -> io::Result<usize> {
            let firstblock = (where_ / self.scsi_sector_size as i64) as u32;
            let offset = (where_ % self.scsi_sector_size as i64) as u32;
            let mut nsect = Self::bytes_to_sectors((offset as usize) + len, self.scsi_sector_size);
            
            #[cfg(all(target_os = "sun", target_arch = "x86"))]
            if self.scsi_sector_size > 512 {
                firstblock *= self.scsi_sector_size / 512;
            }
            
            if len > 512 {
                while nsect as usize * self.scsi_sector_size as usize > len {
                    nsect -= 1;
                }
                if nsect == 0 {
                    return Err(Error::new(ErrorKind::Other, "Scsi buffer too small"));
                }
                if rwcmd == ScsiIoMode::Write && offset != 0 {
                    return Err(Error::new(ErrorKind::Other, "Unaligned write"));
                }
            }
            
            let max = scsi_max_length();
            if nsect > max {
                nsect = max;
            }
            
            let mut cdb = [0u8; 10];
            let clen = match rwcmd {
                ScsiIoMode::Read => {
                    cdb[0] = SCSI_READ;
                    if firstblock > 0x1FFFFF || nsect > 0xFF {
                        cdb[0] |= SCSI_GROUP1;
                        cdb[1] = 0;
                        cdb[2] = (firstblock >> 24) as u8;
                        cdb[3] = (firstblock >> 16) as u8;
                        cdb[4] = (firstblock >> 8) as u8;
                        cdb[5] = firstblock as u8;
                        cdb[6] = 0;
                        cdb[7] = (nsect >> 8) as u8;
                        cdb[8] = nsect as u8;
                        cdb[9] = 0;
                        10
                    } else {
                        cdb[1] |= ((firstblock >> 16) & 0x1F) as u8;
                        cdb[2] = (firstblock >> 8) as u8;
                        cdb[3] = firstblock as u8;
                        cdb[4] = nsect as u8;
                        cdb[5] = 0;
                        6
                    }
                }
                ScsiIoMode::Write => {
                    cdb[0] = SCSI_WRITE;
                    if firstblock > 0x1FFFFF || nsect > 0xFF {
                        cdb[0] |= SCSI_GROUP1;
                        cdb[1] = 0;
                        cdb[2] = (firstblock >> 24) as u8;
                        cdb[3] = (firstblock >> 16) as u8;
                        cdb[4] = (firstblock >> 8) as u8;
                        cdb[5] = firstblock as u8;
                        cdb[6] = 0;
                        cdb[7] = (nsect >> 8) as u8;
                        cdb[8] = nsect as u8;
                        cdb[9] = 0;
                        10
                    } else {
                        cdb[1] |= ((firstblock >> 16) & 0x1F) as u8;
                        cdb[2] = (firstblock >> 8) as u8;
                        cdb[3] = firstblock as u8;
                        cdb[4] = nsect as u8;
                        cdb[5] = 0;
                        6
                    }
                }
            };
            
            if self.privileged {
                reclaim_privs();
            }
            
            let res = unsafe {
                scsi_cmd(
                    self.fd,
                    cdb.as_ptr(),
                    clen,
                    rwcmd,
                    buf.as_mut_ptr(),
                    (nsect as usize) * (self.scsi_sector_size as usize),
                    self.extra_data,
                )
            };
            
            if self.privileged {
                drop_privs();
            }
            
            if res != 0 {
                return Err(Error::new(
                    ErrorKind::Other,
                    match rwcmd {
                        ScsiIoMode::Read => "SCMD_READ failed",
                        ScsiIoMode::Write => "SCMD_WRITE failed",
                    },
                ));
            }
            
            if offset > 0 {
                unsafe {
                    let src = buf.as_ptr().add(offset as usize);
                    let dst = buf.as_mut_ptr();
                    let count = (nsect as usize * self.scsi_sector_size as usize) - offset as usize;
                    ptr::copy(src, dst, count);
                }
            }
            
            match len {
                256 => Ok(256),
                512 => Ok(512),
                _ => Ok((nsect as usize * self.scsi_sector_size as usize) - offset as usize),
            }
        }
    }
    
    impl Drop for ScsiDevice {
        fn drop(&mut self) {
            if self.fd > 2 {
                unsafe {
                    libc::close(self.fd);
                }
                if let Some(cmd) = &self.postcmd {
                    postcmd(cmd.as_ptr());
                }
            }
        }
    }
    
    #[derive(Debug, Clone, Copy)]
    pub enum ScsiIoMode {
        Read,
        Write,
    }
    
    pub const SCSI_IO_READ: ScsiIoMode = ScsiIoMode::Read;
    pub const SCSI_IO_WRITE: ScsiIoMode = ScsiIoMode::Write;
    
    pub fn open_scsi(
        dev: &Device,
        name: &CStr,
        mode: c_int,
        errmsg: &mut [u8],
        mode2: c_int,
        locked: c_int,
        lock_mode: c_int,
        max_size: Option<&mut i64>,
    ) -> Option<Box<ScsiDevice>> {
        if !is_scsi(dev) {
            return None;
        }
        
        let mut device = Box::new(ScsiDevice {
            head: unsafe { mem::zeroed() },
            fd: -1,
            privileged: false,
            scsi_sector_size: 512,
            device_size: 0,
            tot_sectors: 0,
            extra_data: ptr::null_mut(),
            postcmd: None,
        });
        
        if !dev.is_null() {
            if mode2 & NO_PRIV == 0 {
                device.privileged = is_privileged(dev);
            }
            // mode |= dev.mode;
        }
        
        precmd(dev);
        if !dev.is_null() {
            unsafe {
                device.postcmd = Some(CStr::from_ptr((*dev).postcmd).to_owned());
            }
        }
        
        if is_privileged(dev) && mode2 & NO_PRIV == 0 {
            reclaim_privs();
        }
        
        device.fd = unsafe {
            scsi_open(
                name.as_ptr(),
                mode,
                if is_nolock(dev) { 0o444 } else { 0o666 },
                &mut device.extra_data,
            )
        };
        
        if is_privileged(dev) && mode2 & NO_PRIV == 0 {
            drop_privs();
        }
        
        if device.fd < 0 {
            if !errmsg.is_empty() {
                let msg = format!("Can't open {}: {}", name.to_string_lossy(), Error::last_os_error());
                let bytes = msg.as_bytes();
                let len = std::cmp::min(errmsg.len() - 1, bytes.len());
                errmsg[..len].copy_from_slice(&bytes[..len]);
                errmsg[len] = 0;
            }
            return None;
        }
        
        if is_privileged(dev) && mode2 & NO_PRIV == 0 {
            close_exec(device.fd);
        }
        
        if lock_device(device.fd, dev, locked, lock_mode, errmsg) < 0 {
            return None;
        }
        
        if let Some(max) = max_size {
            *max = MAX_OFF_T_B(31 + log_2(device.scsi_sector_size));
        }
        
        if device.privileged {
            reclaim_privs();
        }
        
        let ret = device.init();
        
        if device.privileged {
            drop_privs();
        }
        
        if ret.is_err() {
            return None;
        }
        
        unsafe {
            (*dev).tot_sectors = device.tot_sectors;
        }
        
        Some(device)
    }
}

// Constants and external functions from C code
const SCSI_READ_CAPACITY: u8 = 0x25;
const SCSI_READ: u8 = 0x08;
const SCSI_WRITE: u8 = 0x0A;
const SCSI_GROUP1: u8 = 0x20;
const NO_PRIV: c_int = 0x01;

extern "C" {
    fn scsi_cmd(
        fd: RawFd,
        cdb: *const u8,
        cdb_len: usize,
        direction: ScsiIoMode,
        buf: *mut u8,
        buf_len: usize,
        extra_data: *mut c_void,
    ) -> c_int;
    
    fn scsi_open(
        name: *const c_char,
        mode: c_int,
        permissions: c_int,
        extra_data: *mut *mut c_void,
    ) -> RawFd;
    
    fn scsi_max_length() -> u32;
    fn is_scsi(dev: *const Device) -> bool;
    fn is_privileged(dev: *const Device) -> bool;
    fn is_nolock(dev: *const Device) -> bool;
    fn reclaim_privs();
    fn drop_privs();
    fn precmd(dev: *const Device);
    fn postcmd(cmd: *const c_char);
    fn close_exec(fd: RawFd);
    fn lock_device(
        fd: RawFd,
        dev: *const Device,
        locked: c_int,
        lock_mode: c_int,
        errmsg: *mut c_char,
    ) -> c_int;
    fn MAX_OFF_T_B(bits: c_int) -> i64;
    fn log_2(value: u32) -> c_int;
}