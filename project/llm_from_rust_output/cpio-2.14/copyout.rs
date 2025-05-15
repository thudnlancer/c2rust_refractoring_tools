use std::fs::{File, Metadata, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::os::unix::fs::{MetadataExt, FileTypeExt};
use std::path::{Path, PathBuf};
use std::ffi::{CString, CStr};
use std::os::unix::ffi::OsStrExt;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{self, mode_t, dev_t, ino_t, uid_t, gid_t, off_t, time_t, size_t, ssize_t};
use nix::sys::stat::{self, SFlag};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, readlink, lseek, Whence};
use std::ptr;
use std::mem;
use std::collections::HashMap;

const CPIO_NEWC_MAGIC: &[u8; 6] = b"070701";
const CPIO_CRC_MAGIC: &[u8; 6] = b"070702";
const TRAILER_NAME: &str = "TRAILER!!!";

#[derive(Debug, Clone)]
struct CpioFileStat {
    c_magic: u16,
    c_ino: ino_t,
    c_mode: mode_t,
    c_uid: uid_t,
    c_gid: gid_t,
    c_nlink: size_t,
    c_mtime: time_t,
    c_filesize: off_t,
    c_dev_maj: u32,
    c_dev_min: u32,
    c_rdev_maj: u32,
    c_rdev_min: u32,
    c_namesize: size_t,
    c_chksum: u32,
    c_name: CString,
    c_tar_linkname: Option<CString>,
}

impl CpioFileStat {
    fn new() -> Self {
        Self {
            c_magic: 0o70707,
            c_ino: 0,
            c_mode: 0,
            c_uid: 0,
            c_gid: 0,
            c_nlink: 0,
            c_mtime: 0,
            c_filesize: 0,
            c_dev_maj: 0,
            c_dev_min: 0,
            c_rdev_maj: 0,
            c_rdev_min: 0,
            c_namesize: 0,
            c_chksum: 0,
            c_name: CString::new("").unwrap(),
            c_tar_linkname: None,
        }
    }

    fn set_name(&mut self, name: &str) {
        let cname = CString::new(name).unwrap();
        self.c_namesize = cname.as_bytes_with_nul().len() as size_t;
        self.c_name = cname;
    }
}

struct ArchiveWriter {
    output: File,
    archive_format: ArchiveFormat,
    io_block_size: usize,
    output_bytes: u64,
    deferouts: Vec<CpioFileStat>,
    inode_map: HashMap<(ino_t, u64, u64), CString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArchiveFormat {
    Binary,
    OldAscii,
    NewAscii,
    CrcAscii,
    Tar,
    Ustar,
    HpOldAscii,
    HpBinary,
}

impl ArchiveWriter {
    fn new(output: File, format: ArchiveFormat) -> Self {
        Self {
            output,
            archive_format: format,
            io_block_size: 512,
            output_bytes: 0,
            deferouts: Vec::new(),
            inode_map: HashMap::new(),
        }
    }

    fn write_header(&mut self, file_stat: &CpioFileStat) -> io::Result<()> {
        match self.archive_format {
            ArchiveFormat::NewAscii => self.write_new_ascii_header(file_stat, CPIO_NEWC_MAGIC),
            ArchiveFormat::CrcAscii => self.write_new_ascii_header(file_stat, CPIO_CRC_MAGIC),
            ArchiveFormat::OldAscii => self.write_old_ascii_header(file_stat),
            ArchiveFormat::Binary => self.write_binary_header(file_stat),
            ArchiveFormat::Tar | ArchiveFormat::Ustar => self.write_tar_header(file_stat),
            _ => unimplemented!(),
        }
    }

    fn write_new_ascii_header(&mut self, file_stat: &CpioFileStat, magic: &[u8; 6]) -> io::Result<()> {
        let mut header = [0u8; 110];
        header[..6].copy_from_slice(magic);
        
        // Write fields in the header
        // ... (implementation of each field writing)
        
        self.output.write_all(&header)?;
        self.output.write_all(file_stat.c_name.as_bytes_with_nul())?;
        self.pad_output(file_stat.c_namesize as usize + 110)
    }

    fn write_old_ascii_header(&mut self, file_stat: &CpioFileStat) -> io::Result<()> {
        // Implementation for old ASCII format
        unimplemented!()
    }

    fn write_binary_header(&mut self, file_stat: &CpioFileStat) -> io::Result<()> {
        // Implementation for binary format
        unimplemented!()
    }

    fn write_tar_header(&mut self, file_stat: &CpioFileStat) -> io::Result<()> {
        // Implementation for tar format
        unimplemented!()
    }

    fn pad_output(&mut self, offset: usize) -> io::Result<()> {
        let pad = match self.archive_format {
            ArchiveFormat::NewAscii | ArchiveFormat::CrcAscii => (4 - offset % 4) % 4,
            ArchiveFormat::Tar | ArchiveFormat::Ustar => (512 - offset % 512) % 512,
            _ => (2 - offset % 2) % 2,
        };
        
        if pad > 0 {
            self.write_nuls(pad as u64)?;
        }
        Ok(())
    }

    fn write_nuls(&mut self, count: u64) -> io::Result<()> {
        let buf = vec![0u8; count as usize];
        self.output.write_all(&buf)
    }

    fn copy_file_data(&mut self, mut input: File, size: u64) -> io::Result<()> {
        let mut buf = vec![0u8; 8192];
        let mut remaining = size;
        
        while remaining > 0 {
            let to_read = std::cmp::min(remaining, buf.len() as u64);
            let n = input.read(&mut buf[..to_read as usize])?;
            if n == 0 {
                break;
            }
            self.output.write_all(&buf[..n])?;
            remaining -= n as u64;
        }
        
        Ok(())
    }

    fn process_file(&mut self, path: &Path) -> io::Result<()> {
        let metadata = path.metadata()?;
        let mut file_stat = CpioFileStat::new();
        
        // Populate file_stat from metadata
        file_stat.c_ino = metadata.ino();
        file_stat.c_mode = metadata.mode();
        file_stat.c_uid = metadata.uid();
        file_stat.c_gid = metadata.gid();
        file_stat.c_nlink = metadata.nlink() as size_t;
        file_stat.c_mtime = metadata.mtime() as time_t;
        file_stat.c_filesize = metadata.size() as off_t;
        file_stat.c_dev_maj = unsafe { libc::major(metadata.dev()) as u32 };
        file_stat.c_dev_min = unsafe { libc::minor(metadata.dev()) as u32 };
        
        let file_type = metadata.file_type();
        if file_type.is_file() {
            self.process_regular_file(path, &mut file_stat)?;
        } else if file_type.is_dir() {
            self.process_directory(&mut file_stat)?;
        } else if file_type.is_symlink() {
            self.process_symlink(path, &mut file_stat)?;
        } else {
            self.process_special_file(path, &mut file_stat)?;
        }
        
        Ok(())
    }

    fn process_regular_file(&mut self, path: &Path, file_stat: &mut CpioFileStat) -> io::Result<()> {
        let input = File::open(path)?;
        self.write_header(file_stat)?;
        self.copy_file_data(input, file_stat.c_filesize as u64)?;
        self.pad_output(file_stat.c_filesize as usize)
    }

    fn process_directory(&mut self, file_stat: &mut CpioFileStat) -> io::Result<()> {
        file_stat.c_filesize = 0;
        self.write_header(file_stat)
    }

    fn process_symlink(&mut self, path: &Path, file_stat: &mut CpioFileStat) -> io::Result<()> {
        let link_target = readlink(path)?;
        let target_str = link_target.to_string_lossy();
        file_stat.c_filesize = target_str.len() as off_t;
        
        if self.archive_format == ArchiveFormat::Tar || self.archive_format == ArchiveFormat::Ustar {
            if target_str.len() + 1 > 100 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "symbolic link too long"));
            }
            file_stat.c_tar_linkname = Some(CString::new(target_str.as_bytes()).unwrap());
            self.write_header(file_stat)
        } else {
            self.write_header(file_stat)?;
            self.output.write_all(target_str.as_bytes())?;
            self.pad_output(target_str.len())
        }
    }

    fn process_special_file(&mut self, _path: &Path, file_stat: &mut CpioFileStat) -> io::Result<()> {
        file_stat.c_filesize = 0;
        self.write_header(file_stat)
    }

    fn write_trailer(&mut self) -> io::Result<()> {
        let mut trailer = CpioFileStat::new();
        trailer.set_name(TRAILER_NAME);
        
        if self.archive_format != ArchiveFormat::Tar && self.archive_format != ArchiveFormat::Ustar {
            self.write_header(&trailer)?;
        } else {
            self.write_nuls(1024)?;
        }
        
        self.write_nuls((self.io_block_size - (self.output_bytes as usize % self.io_block_size)) as u64)
    }
}

fn process_copy_out(input_files: Vec<PathBuf>, output: File, format: ArchiveFormat) -> io::Result<()> {
    let mut writer = ArchiveWriter::new(output, format);
    
    for file_path in input_files {
        writer.process_file(&file_path)?;
    }
    
    writer.write_trailer()?;
    Ok(())
}