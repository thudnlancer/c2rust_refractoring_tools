use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::os::unix::fs::{FileTypeExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use libc::{dev_t, mode_t, ino_t, off_t};
use nix::sys::stat::{stat, FileStat};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, lseek, read, write};
use bitflags::bitflags;
use thiserror::Error;

const BUFSIZ: usize = 8192;
const CPIO_TRAILER_NAME: &str = "TRAILER!!!";
const LG_8: u32 = 3;
const LG_16: u32 = 4;

bitflags! {
    pub struct CpioFileMode: u32 {
        const IFMT = 0o170000;
        const IFREG = 0o100000;
        const IFBLK = 0o060000;
        const IFDIR = 0o040000;
        const IFCHR = 0o020000;
        const IFLNK = 0o120000;
        const IFSOCK = 0o140000;
        const IFIFO = 0o010000;
    }
}

#[derive(Debug, Default)]
pub struct CpioFileStat {
    pub c_ino: ino_t,
    pub c_mode: mode_t,
    pub c_uid: u32,
    pub c_gid: u32,
    pub c_nlink: u32,
    pub c_mtime: u64,
    pub c_filesize: u64,
    pub c_dev_maj: u32,
    pub c_dev_min: u32,
    pub c_rdev_maj: u32,
    pub c_rdev_min: u32,
    pub c_namesize: u32,
    pub c_chksum: u32,
    pub c_name: String,
    pub c_tar_linkname: Option<String>,
}

#[derive(Debug)]
pub struct Deferment {
    pub header: CpioFileStat,
    pub next: Option<Box<Deferment>>,
}

pub struct CpioArchive {
    pub deferouts: Option<Box<Deferment>>,
    pub archive_format: ArchiveFormat,
    pub output_bytes: u64,
    pub io_block_size: usize,
    pub output_is_special: bool,
    pub output_is_seekable: bool,
    pub verbose_flag: bool,
    pub quiet_flag: bool,
    pub dot_flag: bool,
    pub reset_time_flag: bool,
    pub no_abs_paths_flag: bool,
    pub ignore_dirnlink_option: bool,
    pub append_flag: bool,
    pub warn_option: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArchiveFormat {
    NewAscii,
    CrcAscii,
    OldAscii,
    HPOldAscii,
    Tar,
    Ustar,
    Binary,
    HPBinary,
}

#[derive(Error, Debug)]
pub enum CpioError {
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("Nix error")]
    Nix(#[from] nix::Error),
    #[error("Invalid file type")]
    InvalidFileType,
    #[error("File name too long")]
    FileNameTooLong,
    #[error("Checksum error")]
    ChecksumError,
    #[error("Field width error")]
    FieldWidthError,
}

impl CpioArchive {
    pub fn new(format: ArchiveFormat) -> Self {
        CpioArchive {
            deferouts: None,
            archive_format: format,
            output_bytes: 0,
            io_block_size: 512,
            output_is_special: false,
            output_is_seekable: true,
            verbose_flag: false,
            quiet_flag: false,
            dot_flag: false,
            reset_time_flag: false,
            no_abs_paths_flag: false,
            ignore_dirnlink_option: false,
            append_flag: false,
            warn_option: 0,
        }
    }

    pub fn read_for_checksum(
        &mut self,
        in_file_des: i32,
        file_size: u64,
        file_name: &str,
    ) -> Result<u32, CpioError> {
        let mut crc = 0;
        let mut remaining = file_size;
        let mut buf = [0u8; BUFSIZ];

        while remaining > 0 {
            let to_read = std::cmp::min(remaining as usize, BUFSIZ);
            let bytes_read = read(in_file_des, &mut buf[..to_read])?;

            if bytes_read == 0 {
                break;
            }

            for &byte in &buf[..bytes_read] {
                crc = crc.wrapping_add(byte as u32);
            }

            remaining -= bytes_read as u64;
        }

        lseek(in_file_des, 0, SeekFrom::Start(0))?;
        Ok(crc)
    }

    pub fn tape_clear_rest_of_block(&mut self, out_file_des: i32) -> Result<(), CpioError> {
        let pad = self.io_block_size - (self.output_bytes as usize % self.io_block_size);
        if pad > 0 {
            self.write_nuls_to_file(pad, out_file_des)?;
        }
        Ok(())
    }

    pub fn tape_pad_output(&mut self, out_file_des: i32, offset: usize) -> Result<(), CpioError> {
        let pad = match self.archive_format {
            ArchiveFormat::NewAscii | ArchiveFormat::CrcAscii => (4 - (offset % 4)) % 4,
            ArchiveFormat::Tar | ArchiveFormat::Ustar => (512 - (offset % 512)) % 512,
            ArchiveFormat::OldAscii | ArchiveFormat::HPOldAscii => 0,
            _ => (2 - (offset % 2)) % 2,
        };

        if pad != 0 {
            self.write_nuls_to_file(pad, out_file_des)?;
        }
        Ok(())
    }

    fn write_nuls_to_file(&mut self, count: usize, out_file_des: i32) -> Result<(), CpioError> {
        let buf = vec![0u8; count];
        let written = write(out_file_des, &buf)?;
        self.output_bytes += written as u64;
        Ok(())
    }

    pub fn count_defered_links_to_dev_ino(&self, file_hdr: &CpioFileStat) -> usize {
        let mut count = 0;
        let mut current = &self.deferouts;

        while let Some(d) = current {
            if d.header.c_ino == file_hdr.c_ino
                && d.header.c_dev_maj == file_hdr.c_dev_maj
                && d.header.c_dev_min == file_hdr.c_dev_min
            {
                count += 1;
            }
            current = &d.next;
        }
        count
    }

    pub fn last_link(&self, file_hdr: &CpioFileStat) -> bool {
        file_hdr.c_nlink as usize == self.count_defered_links_to_dev_ino(file_hdr) + 1
    }

    pub fn add_link_defer(&mut self, file_hdr: &CpioFileStat) {
        let new_defer = Deferment {
            header: file_hdr.clone(),
            next: self.deferouts.take().map(Box::new),
        };
        self.deferouts = Some(Box::new(new_defer));
    }

    pub fn writeout_other_defers(
        &mut self,
        file_hdr: &CpioFileStat,
        out_des: i32,
    ) -> Result<(), CpioError> {
        let mut prev: Option<&mut Box<Deferment>> = None;
        let mut current = &mut self.deferouts;

        while let Some(d) = current {
            if d.header.c_ino == file_hdr.c_ino
                && d.header.c_dev_maj == file_hdr.c_dev_maj
                && d.header.c_dev_min == file_hdr.c_dev_min
            {
                d.header.c_filesize = 0;
                self.write_out_header(&d.header, out_des)?;

                if let Some(p) = prev {
                    p.next = d.next.take();
                } else {
                    self.deferouts = d.next.take();
                }
            } else {
                prev = Some(d);
            }
            current = &mut prev.as_mut().unwrap().next;
        }
        Ok(())
    }

    pub fn writeout_defered_file(
        &mut self,
        header: &CpioFileStat,
        out_file_des: i32,
    ) -> Result<(), CpioError> {
        let mut file_hdr = header.clone();

        let in_file_des = open(
            Path::new(&header.c_name),
            OFlag::O_RDONLY,
            Mode::empty(),
        )?;

        if self.archive_format == ArchiveFormat::CrcAscii {
            file_hdr.c_chksum = self.read_for_checksum(
                in_file_des,
                file_hdr.c_filesize,
                &header.c_name,
            )?;
        }

        self.write_out_header(&file_hdr, out_file_des)?;
        self.copy_files_disk_to_tape(in_file_des, out_file_des, file_hdr.c_filesize, &header.c_name)?;

        if self.archive_format == ArchiveFormat::Tar || self.archive_format == ArchiveFormat::Ustar {
            // TODO: Implement add_inode
        }

        self.tape_pad_output(out_file_des, file_hdr.c_filesize as usize)?;

        if self.reset_time_flag {
            // TODO: Implement set_file_times
        }

        close(in_file_des)?;
        Ok(())
    }

    pub fn writeout_final_defers(&mut self, out_des: i32) -> Result<(), CpioError> {
        while let Some(mut d) = self.deferouts.take() {
            let other_count = self.count_defered_links_to_dev_ino(&d.header);
            if other_count == 1 {
                self.writeout_defered_file(&d.header, out_des)?;
            } else {
                let mut file_hdr = d.header;
                file_hdr.c_filesize = 0;
                self.write_out_header(&file_hdr, out_des)?;
            }
            self.deferouts = d.next.take();
        }
        Ok(())
    }

    pub fn to_ascii(
        &self,
        where_: &mut [u8],
        v: u64,
        digits: usize,
        logbase: u32,
        nul: bool,
    ) -> bool {
        static CODETAB: &[u8] = b"0123456789ABCDEF";
        let mut v = v;
        let mut digits = digits;

        if nul {
            digits -= 1;
            where_[digits] = 0;
        }

        while digits > 0 {
            digits -= 1;
            where_[digits] = CODETAB[(v & ((1 << logbase) - 1)) as usize];
            v >>= logbase;
        }

        v != 0
    }

    pub fn write_out_new_ascii_header(
        &mut self,
        magic_string: &str,
        file_hdr: &CpioFileStat,
        out_des: i32,
    ) -> Result<(), CpioError> {
        let mut ascii_header = [0u8; 110];
        let mut p = 0;

        p += magic_string.as_bytes().len();
        ascii_header[..p].copy_from_slice(magic_string.as_bytes());

        let fields = [
            (file_hdr.c_ino, "inode number"),
            (file_hdr.c_mode, "file mode"),
            (file_hdr.c_uid, "uid"),
            (file_hdr.c_gid, "gid"),
            (file_hdr.c_nlink, "number of links"),
            (file_hdr.c_mtime, "modification time"),
        ];

        for (i, &(value, field)) in fields.iter().enumerate() {
            let start = p + i * 8;
            if self.to_ascii(&mut ascii_header[start..start + 8], value as u64, 8, LG_16, false) {
                // TODO: Handle field width warning
            }
        }

        let critical_fields = [
            (file_hdr.c_filesize, "file size"),
            (file_hdr.c_dev_maj as u64, "device major number"),
            (file_hdr.c_dev_min as u64, "device minor number"),
            (file_hdr.c_rdev_maj as u64, "rdev major"),
            (file_hdr.c_rdev_min as u64, "rdev minor"),
            (file_hdr.c_namesize as u64, "name size"),
        ];

        for (i, &(value, field)) in critical_fields.iter().enumerate() {
            let start = p + 48 + i * 8;
            if self.to_ascii(&mut ascii_header[start..start + 8], value, 8, LG_16, false) {
                return Err(CpioError::FieldWidthError);
            }
        }

        p += 96;
        self.to_ascii(&mut ascii_header[p..p + 8], file_hdr.c_chksum as u64, 8, LG_16, false);

        write(out_des, &ascii_header)?;
        self.output_bytes += ascii_header.len() as u64;

        write(out_des, file_hdr.c_name.as_bytes())?;
        self.output_bytes += file_hdr.c_name.len() as u64;

        self.tape_pad_output(out_des, file_hdr.c_name.len() + ascii_header.len())
    }

    pub fn write_out_header(&mut self, file_hdr: &CpioFileStat, out_des: i32) -> Result<(), CpioError> {
        match self.archive_format {
            ArchiveFormat::NewAscii => self.write_out_new_ascii_header("070701", file_hdr, out_des),
            ArchiveFormat::CrcAscii => self.write_out_new_ascii_header("070702", file_hdr, out_des),
            _ => unimplemented!("Other archive formats not yet implemented"),
        }
    }

    pub fn copy_files_disk_to_tape(
        &mut self,
        in_file_des: i32,
        out_file_des: i32,
        file_size: u64,
        file_name: &str,
    ) -> Result<(), CpioError> {
        let mut remaining = file_size;
        let mut buf = [0u8; BUFSIZ];

        while remaining > 0 {
            let to_read = std::cmp::min(remaining as usize, BUFSIZ);
            let bytes_read = read(in_file_des, &mut buf[..to_read])?;

            if bytes_read == 0 {
                break;
            }

            write(out_file_des, &buf[..bytes_read])?;
            self.output_bytes += bytes_read as u64;
            remaining -= bytes_read as u64;
        }

        Ok(())
    }

    pub fn process_copy_out(&mut self) -> Result<(), CpioError> {
        let mut input_name = String::new();
        let mut file_hdr = CpioFileStat::default();
        file_hdr.c_magic = 0o70707;

        // TODO: Implement the rest of the process_copy_out functionality
        // including reading input files, handling different file types,
        // writing trailers, etc.

        self.writeout_final_defers(1)?; // TODO: Use actual output descriptor

        // Write trailer
        let mut trailer = CpioFileStat::default();
        trailer.c_nlink = 1;
        trailer.c_name = CPIO_TRAILER_NAME.to_string();
        self.write_out_header(&trailer, 1)?; // TODO: Use actual output descriptor

        self.tape_clear_rest_of_block(1)?; // TODO: Use actual output descriptor

        if !self.quiet_flag {
            let blocks = (self.output_bytes + self.io_block_size as u64 - 1) / self.io_block_size as u64;
            eprintln!("{} blocks", blocks);
        }

        Ok(())
    }
}

fn main() -> Result<(), CpioError> {
    let mut cpio = CpioArchive::new(ArchiveFormat::NewAscii);
    cpio.process_copy_out()
}