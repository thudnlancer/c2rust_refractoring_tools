// Rust translation of the C code for creating a tar archive

use std::{
    collections::HashMap,
    ffi::CString,
    fs::{self, File, Metadata},
    io::{self, Read, Write},
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const BLOCKSIZE: usize = 512;
const NAME_FIELD_SIZE: usize = 100;
const PREFIX_FIELD_SIZE: usize = 155;
const TMAGIC: &[u8] = b"ustar";
const TMAGLEN: usize = 6;
const TVERSION: &[u8] = b"00";
const TVERSLEN: usize = 2;
const OLDGNU_MAGIC: &[u8] = b"ustar  ";
const GNUTYPE_LONGLINK: u8 = b'K';
const GNUTYPE_LONGNAME: u8 = b'L';
const GNUTYPE_DUMPDIR: u8 = b'D';
const DIRTYPE: u8 = b'5';
const LNKTYPE: u8 = b'1';
const SYMTYPE: u8 = b'2';
const CHRTYPE: u8 = b'3';
const BLKTYPE: u8 = b'4';
const FIFOTYPE: u8 = b'6';
const CONTTYPE: u8 = b'7';
const XHDTYPE: u8 = b'x';
const XGLTYPE: u8 = b'g';

struct TarHeader {
    name: [u8; NAME_FIELD_SIZE],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    size: [u8; 12],
    mtime: [u8; 12],
    chksum: [u8; 8],
    typeflag: u8,
    linkname: [u8; NAME_FIELD_SIZE],
    magic: [u8; TMAGLEN],
    version: [u8; TVERSLEN],
    uname: [u8; 32],
    gname: [u8; 32],
    devmajor: [u8; 8],
    devminor: [u8; 8],
    prefix: [u8; PREFIX_FIELD_SIZE],
    padding: [u8; 12],
}

impl Default for TarHeader {
    fn default() -> Self {
        TarHeader {
            name: [0; NAME_FIELD_SIZE],
            mode: [0; 8],
            uid: [0; 8],
            gid: [0; 8],
            size: [0; 12],
            mtime: [0; 12],
            chksum: [0; 8],
            typeflag: 0,
            linkname: [0; NAME_FIELD_SIZE],
            magic: [0; TMAGLEN],
            version: [0; TVERSLEN],
            uname: [0; 32],
            gname: [0; 32],
            devmajor: [0; 8],
            devminor: [0; 8],
            prefix: [0; PREFIX_FIELD_SIZE],
            padding: [0; 12],
        }
    }
}

struct TarStatInfo {
    file_name: String,
    orig_file_name: String,
    link_name: Option<String>,
    stat: Metadata,
    fd: Option<File>,
    parent: Option<Box<TarStatInfo>>,
}

impl TarStatInfo {
    fn new() -> Self {
        TarStatInfo {
            file_name: String::new(),
            orig_file_name: String::new(),
            link_name: None,
            stat: Metadata::from(std::fs::metadata(".").unwrap()), // Default metadata
            fd: None,
            parent: None,
        }
    }
}

struct TarArchive {
    file: File,
    format: ArchiveFormat,
    link_table: HashMap<(u64, u64), String>, // (dev, ino) -> name
}

enum ArchiveFormat {
    V7,
    Ustar,
    OldGnu,
    Gnu,
    Posix,
}

impl TarArchive {
    fn new(file: File) -> Self {
        TarArchive {
            file,
            format: ArchiveFormat::Ustar,
            link_table: HashMap::new(),
        }
    }

    fn write_header(&mut self, st: &TarStatInfo) -> io::Result<()> {
        let mut header = TarHeader::default();
        
        // Fill in header fields
        self.copy_str(&mut header.name, &st.file_name);
        self.to_octal(st.stat.mode() as u64, &mut header.mode);
        self.to_octal(st.stat.uid() as u64, &mut header.uid);
        self.to_octal(st.stat.gid() as u64, &mut header.gid);
        self.to_octal(st.stat.size() as u64, &mut header.size);
        
        let mtime = st.stat.modified()
            .unwrap_or(UNIX_EPOCH)
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.to_octal(mtime, &mut header.mtime);
        
        header.typeflag = if st.stat.is_dir() {
            DIRTYPE
        } else if st.stat.is_file() {
            b'0'
        } else if st.stat.is_symlink() {
            SYMTYPE
        } else {
            0
        };
        
        if let Some(link) = &st.link_name {
            self.copy_str(&mut header.linkname, link);
        }
        
        match self.format {
            ArchiveFormat::V7 => {}
            ArchiveFormat::Ustar | ArchiveFormat::Posix => {
                header.magic.copy_from_slice(TMAGIC);
                header.version.copy_from_slice(TVERSION);
            }
            ArchiveFormat::OldGnu | ArchiveFormat::Gnu => {
                header.magic.copy_from_slice(OLDGNU_MAGIC);
            }
        }
        
        // Calculate checksum
        let checksum = self.calculate_checksum(&header);
        self.to_octal(checksum as u64, &mut header.chksum);
        
        // Write header
        self.file.write_all(unsafe {
            std::slice::from_raw_parts(
                &header as *const _ as *const u8,
                std::mem::size_of::<TarHeader>()
            )
        })
    }
    
    fn copy_str(&self, dest: &mut [u8], src: &str) {
        let bytes = src.as_bytes();
        let len = std::cmp::min(dest.len(), bytes.len());
        dest[..len].copy_from_slice(&bytes[..len]);
        if len < dest.len() {
            dest[len] = 0;
        }
    }
    
    fn to_octal(&self, mut value: u64, field: &mut [u8]) {
        let len = field.len();
        for i in (0..len).rev() {
            field[i] = b'0' + ((value & 0o7) as u8);
            value >>= 3;
        }
    }
    
    fn calculate_checksum(&self, header: &TarHeader) -> u32 {
        let mut sum = 0;
        let bytes = unsafe {
            std::slice::from_raw_parts(
                header as *const _ as *const u8,
                std::mem::size_of::<TarHeader>()
            )
        };
        
        for &byte in bytes {
            sum += byte as u32;
        }
        
        // Subtract the sum of the chksum field which was initially all spaces
        for &byte in &header.chksum {
            sum -= byte as u32;
        }
        sum += b' ' as u32 * header.chksum.len() as u32;
        
        sum
    }
    
    fn write_file(&mut self, st: &TarStatInfo) -> io::Result<()> {
        if let Some(mut fd) = st.fd.take() {
            let mut remaining = st.stat.size();
            let mut buffer = vec![0; BLOCKSIZE];
            
            while remaining > 0 {
                let to_read = std::cmp::min(buffer.len(), remaining as usize);
                let read = fd.read(&mut buffer[..to_read])?;
                
                if read == 0 {
                    break;
                }
                
                self.file.write_all(&buffer[..read])?;
                remaining -= read as u64;
                
                // Pad last block if needed
                if remaining == 0 && read % BLOCKSIZE != 0 {
                    let padding = BLOCKSIZE - (read % BLOCKSIZE);
                    self.file.write_all(&vec![0; padding])?;
                }
            }
            
            Ok(())
        } else {
            Ok(())
        }
    }
    
    fn write_eot(&mut self) -> io::Result<()> {
        // Write two empty blocks
        self.file.write_all(&[0; BLOCKSIZE * 2])
    }
}

fn create_archive(output_path: &str, paths: &[&str]) -> io::Result<()> {
    let file = File::create(output_path)?;
    let mut archive = TarArchive::new(file);
    
    for path in paths {
        let st = TarStatInfo::new();
        // TODO: Properly populate TarStatInfo with file metadata
        archive.write_header(&st)?;
        archive.write_file(&st)?;
    }
    
    archive.write_eot()?;
    Ok(())
}

// Note: This is a simplified translation. The full implementation would need to:
// 1. Handle all the various file types (symlinks, devices, etc.)
// 2. Implement long name/link handling
// 3. Handle extended attributes and other metadata
// 4. Implement proper error handling
// 5. Handle all the various archive formats
// 6. Implement incremental backup support
// 7. Handle hard links properly
// 8. Implement all the various options from the original code