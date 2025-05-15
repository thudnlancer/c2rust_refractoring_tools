use std::{
    ffi::{CStr, CString},
    fs::File,
    io::{self, Read, Seek, SeekFrom, Write},
    mem,
    os::unix::io::{AsRawFd, FromRawFd, RawFd},
    ptr,
    time::SystemTime,
};

const SCSI_IO_READ: u32 = 0;
const SCSI_IO_WRITE: u32 = 1;

#[derive(Debug)]
struct ScsiDevice {
    file: File,
    privileged: bool,
    scsi_sector_size: u32,
    device_size: i64,
    tot_sectors: u32,
    postcmd: Option<CString>,
}

impl ScsiDevice {
    fn new(file: File, privileged: bool, postcmd: Option<CString>) -> Self {
        Self {
            file,
            privileged,
            scsi_sector_size: 512,
            device_size: 0,
            tot_sectors: 0,
            postcmd,
        }
    }

    fn init(&mut self) -> io::Result<()> {
        let mut cdb = [0u8; 10];
        let mut buf = [0u8; 8];

        cdb[0] = 0x25;

        if self.scsi_cmd(&cdb, SCSI_IO_READ, &mut buf)? == 0 {
            self.tot_sectors = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]);
            if self.tot_sectors < u32::MAX {
                self.tot_sectors += 1;
            }

            self.scsi_sector_size = u32::from_be_bytes([0, buf[5], buf[6], buf[7]]);
            if self.scsi_sector_size != 512 {
                eprintln!("  (scsi_sector_size={})", self.scsi_sector_size);
            }
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "SCSI command failed"))
        }
    }

    fn bytes_to_sectors(&self, bytes: usize) -> u32 {
        let mut sectors = bytes / self.scsi_sector_size as usize;
        if bytes % self.scsi_sector_size as usize != 0 {
            sectors += 1;
        }
        sectors.min(u32::MAX as usize) as u32
    }

    fn scsi_io(
        &mut self,
        buf: &mut [u8],
        offset: i64,
        len: usize,
        rwcmd: u32,
    ) -> io::Result<usize> {
        let firstblock = (offset / self.scsi_sector_size as i64) as u32;
        let offset_in_block = (offset % self.scsi_sector_size as i64) as u32;
        let mut nsect = self.bytes_to_sectors(offset_in_block as usize + len);

        if len > 512 {
            while nsect * self.scsi_sector_size > len as u32 {
                nsect -= 1;
            }
            if nsect == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Scsi buffer too small",
                ));
            }
            if rwcmd == SCSI_IO_WRITE && offset_in_block != 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unaligned write",
                ));
            }
        }

        let max = scsi_max_length();
        if nsect > max {
            nsect = max;
        }

        let mut cdb = [0u8; 10];
        match rwcmd {
            SCSI_IO_READ => cdb[0] = 0x08,
            SCSI_IO_WRITE => cdb[0] = 0x0A,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid SCSI command")),
        }

        cdb[1] = 0;

        let clen = if firstblock > 0x1FFFFF || nsect > 0xFF {
            cdb[0] |= 0x20;
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
            cdb[1] |= (firstblock >> 16 & 0x1F) as u8;
            cdb[2] = (firstblock >> 8) as u8;
            cdb[3] = firstblock as u8;
            cdb[4] = nsect as u8;
            cdb[5] = 0;
            6
        };

        let result = if self.privileged {
            reclaim_privs();
            let res = self.scsi_cmd(&cdb, rwcmd, buf);
            drop_privs();
            res
        } else {
            self.scsi_cmd(&cdb, rwcmd, buf)
        };

        if result.is_err() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                if rwcmd == SCSI_IO_READ {
                    "SCMD_READ failed"
                } else {
                    "SCMD_WRITE failed"
                },
            ));
        }

        if offset_in_block > 0 {
            buf.copy_within(
                offset_in_block as usize..nsect as usize * self.scsi_sector_size as usize,
                0,
            );
        }

        Ok(if len == 256 {
            256
        } else if len == 512 {
            512
        } else {
            nsect as usize * self.scsi_sector_size as usize - offset_in_block as usize
        })
    }

    fn scsi_cmd(&self, cdb: &[u8], mode: u32, data: &mut [u8]) -> io::Result<i32> {
        // Implementation depends on platform-specific SCSI interface
        // This is a placeholder for the actual implementation
        unimplemented!("scsi_cmd needs platform-specific implementation")
    }
}

impl Read for ScsiDevice {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let pos = self.file.seek(SeekFrom::Current(0))?;
        self.scsi_io(buf, pos, buf.len(), SCSI_IO_READ)
    }
}

impl Write for ScsiDevice {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let pos = self.file.seek(SeekFrom::Current(0))?;
        self.scsi_io(buf, pos, buf.len(), SCSI_IO_WRITE)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

impl Seek for ScsiDevice {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.file.seek(pos)
    }
}

impl Drop for ScsiDevice {
    fn drop(&mut self) {
        if let Some(cmd) = &self.postcmd {
            postcmd(cmd);
        }
    }
}

fn scsi_max_length() -> u32 {
    // Implementation depends on platform-specific SCSI interface
    // This is a placeholder for the actual implementation
    unimplemented!("scsi_max_length needs platform-specific implementation")
}

fn reclaim_privs() {
    // Implementation depends on privilege management
    unimplemented!("reclaim_privs needs platform-specific implementation")
}

fn drop_privs() {
    // Implementation depends on privilege management
    unimplemented!("drop_privs needs platform-specific implementation")
}

fn postcmd(cmd: &CStr) {
    // Implementation depends on command execution
    unimplemented!("postcmd needs platform-specific implementation")
}

fn open_scsi(
    dev: Option<&Device>,
    name: &CStr,
    mode: i32,
    errmsg: &mut [u8],
    mode2: i32,
    locked: bool,
    lock_mode: i32,
    max_size: &mut i64,
) -> io::Result<ScsiDevice> {
    if dev.is_none() || dev.unwrap().misc_flags & 0x1 == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid device"));
    }

    let privileged = dev.map_or(false, |d| d.misc_flags & 0x2 != 0 && mode2 & 1 == 0);
    let postcmd = dev.and_then(|d| d.postcmd.map(|s| CString::new(s).unwrap()));

    if privileged {
        reclaim_privs();
    }

    let file = unsafe {
        File::from_raw_fd(scsi_open(
            name.as_ptr(),
            mode,
            if dev.map_or(false, |d| d.misc_flags & 0x4 != 0) {
                0o444
            } else {
                0o666
            },
        ))
    };

    if privileged {
        drop_privs();
    }

    if file.as_raw_fd() < 0 {
        let err = io::Error::last_os_error();
        if !errmsg.is_empty() {
            let msg = format!("Can't open {}: {}", name.to_str().unwrap(), err);
            errmsg[..msg.len().min(errmsg.len() - 1)]
                .copy_from_slice(msg.as_bytes());
            errmsg[msg.len().min(errmsg.len() - 1)] = 0;
        }
        return Err(err);
    }

    if privileged {
        close_exec(file.as_raw_fd());
    }

    let mut device = ScsiDevice::new(file, privileged, postcmd);
    device.init()?;

    if let Some(d) = dev {
        d.tot_sectors = device.tot_sectors;
    }

    Ok(device)
}

struct Device {
    misc_flags: u32,
    tot_sectors: u32,
    postcmd: Option<*const i8>,
    // Other fields omitted for brevity
}

fn close_exec(_fd: RawFd) {
    // Implementation depends on platform-specific file descriptor handling
    unimplemented!("close_exec needs platform-specific implementation")
}

fn scsi_open(_name: *const i8, _flags: i32, _mode: i32) -> RawFd {
    // Implementation depends on platform-specific SCSI interface
    unimplemented!("scsi_open needs platform-specific implementation")
}