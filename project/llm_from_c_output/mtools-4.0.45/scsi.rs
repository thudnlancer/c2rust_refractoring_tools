//! SCSI interface for mtools
//! 
//! Copyright 1997-1999,2001,2002,2009 Alain Knaff.
//! This file is part of mtools.
//!
//! Mtools is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! Mtools is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with Mtools.  If not, see <http://www.gnu.org/licenses/>.

#[cfg(feature = "scsi")]
mod scsi {
    use std::fs::File;
    use std::io::{Error, ErrorKind, Result};
    use std::os::unix::io::AsRawFd;
    use std::path::Path;

    pub const SCSI_READ: u8 = 0x8;
    pub const SCSI_WRITE: u8 = 0xA;
    pub const SCSI_IOMEGA: u8 = 0xC;
    pub const SCSI_INQUIRY: u8 = 0x12;
    pub const SCSI_MODE_SENSE: u8 = 0x1A;
    pub const SCSI_START_STOP: u8 = 0x1B;
    pub const SCSI_ALLOW_MEDIUM_REMOVAL: u8 = 0x1E;
    pub const SCSI_GROUP1: u8 = 0x20;
    pub const SCSI_READ_CAPACITY: u8 = 0x25;

    #[derive(Debug, Clone, Copy)]
    pub enum ScsiIoMode {
        Read,
        Write,
    }

    pub fn scsi_max_length() -> usize {
        #[cfg(target_os = "linux")]
        { 8 }
        #[cfg(not(target_os = "linux"))]
        { 255 }
    }

    pub fn scsi_open(name: &Path) -> Result<File> {
        File::open(name)
    }

    #[cfg(target_os = "linux")]
    pub fn scsi_cmd(
        file: &File,
        cdb: &[u8; 6],
        cmdlen: u8,
        mode: ScsiIoMode,
        data: Option<&mut [u8]>,
    ) -> Result<()> {
        use std::mem::MaybeUninit;
        use libc::{sg_io_hdr, SG_DXFER_FROM_DEV, SG_DXFER_TO_DEV, SG_IO};

        let mut my_scsi_cmd = MaybeUninit::<sg_io_hdr>::zeroed();
        let mut my_scsi_cmd = unsafe { my_scsi_cmd.assume_init() };

        my_scsi_cmd.interface_id = b'S' as i32;
        my_scsi_cmd.dxfer_direction = match mode {
            ScsiIoMode::Read => SG_DXFER_FROM_DEV,
            ScsiIoMode::Write => SG_DXFER_TO_DEV,
        };
        my_scsi_cmd.cmd_len = cmdlen as u8;
        my_scsi_cmd.mx_sb_len = 0;
        my_scsi_cmd.dxfer_len = data.as_ref().map(|d| d.len()).unwrap_or(0) as u32;
        my_scsi_cmd.dxferp = data.map(|d| d.as_mut_ptr()).unwrap_or(std::ptr::null_mut()) as *mut _;
        my_scsi_cmd.cmdp = cdb.as_ptr() as *mut u8;
        my_scsi_cmd.timeout = !0u32;

        let res = unsafe {
            libc::ioctl(
                file.as_raw_fd(),
                SG_IO,
                &mut my_scsi_cmd as *mut sg_io_hdr,
            )
        };

        if res < 0 {
            return Err(Error::last_os_error());
        }

        if (my_scsi_cmd.status & 0x7E) != 0 {
            return Err(Error::new(
                ErrorKind::Other,
                format!("SCSI command failed with status {:x}", my_scsi_cmd.status),
            ));
        }

        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    pub fn scsi_cmd(
        file: &File,
        cdb: &[u8; 6],
        cmdlen: u8,
        mode: ScsiIoMode,
        data: Option<&mut [u8]>,
    ) -> Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "SCSI commands not implemented for this platform",
        ))
    }
}

#[cfg(not(feature = "scsi"))]
mod scsi {
    pub const SCSI_READ: u8 = 0;
    pub const SCSI_WRITE: u8 = 0;
    pub const SCSI_IOMEGA: u8 = 0;
    pub const SCSI_INQUIRY: u8 = 0;
    pub const SCSI_MODE_SENSE: u8 = 0;
    pub const SCSI_START_STOP: u8 = 0;
    pub const SCSI_ALLOW_MEDIUM_REMOVAL: u8 = 0;
    pub const SCSI_GROUP1: u8 = 0;
    pub const SCSI_READ_CAPACITY: u8 = 0;

    #[derive(Debug, Clone, Copy)]
    pub enum ScsiIoMode {
        Read,
        Write,
    }

    pub fn scsi_max_length() -> usize {
        0
    }

    pub fn scsi_open(_name: &std::path::Path) -> std::io::Result<std::fs::File> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "SCSI support not compiled in",
        ))
    }

    pub fn scsi_cmd(
        _file: &std::fs::File,
        _cdb: &[u8; 6],
        _cmdlen: u8,
        _mode: ScsiIoMode,
        _data: Option<&mut [u8]>,
    ) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "SCSI support not compiled in",
        ))
    }
}

pub use scsi::*;