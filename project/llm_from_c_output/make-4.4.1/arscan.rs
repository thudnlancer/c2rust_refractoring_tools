use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::os::unix::fs::MetadataExt;
use std::ffi::CStr;
use std::os::raw::c_char;
use libc::{open, close, lseek, O_RDONLY, O_RDWR, SEEK_SET};
use std::ptr;
use std::mem;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::ffi::OsStr;

type ArMemberFunc = fn(i32, &str, bool, i64, i64, i64, i64, i32, i32, u32, &()) -> i64;

const ARMAG: &[u8] = b"!<arch>\n";
const SARMAG: usize = 8;
const ARFMAG: &[u8] = b"`\n";

#[repr(C)]
struct ArHdr {
    ar_name: [c_char; 16],
    ar_date: [c_char; 12],
    ar_uid: [c_char; 6],
    ar_gid: [c_char; 6],
    ar_mode: [c_char; 8],
    ar_size: [c_char; 10],
    ar_fmag: [c_char; 2],
}

const AR_HDR_SIZE: usize = mem::size_of::<ArHdr>();

fn parse_int(ptr: &[u8], base: u32, max: u64, type_name: &str, archive: &str, name: &str) -> Result<u64, String> {
    let mut val = 0;
    let mut ptr = ptr;
    
    while !ptr.is_empty() && ptr[0] == b' ' {
        ptr = &ptr[1..];
    }
    
    for &c in ptr {
        if c == b' ' {
            break;
        }
        
        if c < b'0' || c > (b'0' + base as u8 - 1) {
            return Err(format!("Invalid {} for archive {} member {}", type_name, archive, name));
        }
        
        let nv = val * u64::from(base) + u64::from(c - b'0');
        if nv < val || nv > max {
            return Err(format!("Invalid {} for archive {} member {}", type_name, archive, name));
        }
        val = nv;
    }
    
    Ok(val)
}

fn ar_scan(archive: &str, function: ArMemberFunc, arg: &()) -> i64 {
    let archive_c = CString::new(archive).unwrap();
    let desc = unsafe { open(archive_c.as_ptr(), O_RDONLY, 0) };
    if desc < 0 {
        return -1;
    }

    let mut buf = [0u8; SARMAG];
    let nread = unsafe {
        libc::read(desc, buf.as_mut_ptr() as *mut libc::c_void, SARMAG)
    };
    
    if nread != SARMAG as isize || &buf != ARMAG {
        unsafe { close(desc); }
        return -2;
    }

    let mut member_offset = SARMAG as i64;
    let mut namemap: Option<Vec<u8>> = None;
    let mut namemap_size = 0;

    loop {
        let o = unsafe { lseek(desc, member_offset, SEEK_SET) };
        if o < 0 {
            unsafe { close(desc); }
            return -2;
        }

        let mut member_header: ArHdr = unsafe { mem::zeroed() };
        let nread = unsafe {
            libc::read(desc, &mut member_header as *mut _ as *mut libc::c_void, AR_HDR_SIZE)
        };

        if nread == 0 {
            break;
        }

        if nread != AR_HDR_SIZE as isize || unsafe {
            CStr::from_ptr(member_header.ar_fmag.as_ptr()).to_bytes() != ARFMAG
        } {
            unsafe { close(desc); }
            return -2;
        }

        let mut name_buf = [0u8; 16];
        for (i, &c) in member_header.ar_name.iter().enumerate() {
            if c == 0 || c == b' ' as c_char {
                break;
            }
            name_buf[i] = c as u8;
        }
        let name = unsafe { CStr::from_ptr(name_buf.as_ptr() as *const c_char) };
        let name = name.to_str().unwrap_or("");

        let is_namemap = name == "//" || name == "ARFILENAMES/";

        let eltmode = match parse_int(
            unsafe { CStr::from_ptr(member_header.ar_mode.as_ptr()).to_bytes() },
            8,
            u32::MAX as u64,
            "mode",
            archive,
            name
        ) {
            Ok(v) => v as u32,
            Err(_) => {
                unsafe { close(desc); }
                return -2;
            }
        };

        let eltsize = match parse_int(
            unsafe { CStr::from_ptr(member_header.ar_size.as_ptr()).to_bytes() },
            10,
            i64::MAX as u64,
            "size",
            archive,
            name
        ) {
            Ok(v) => v as i64,
            Err(_) => {
                unsafe { close(desc); }
                return -2;
            }
        };

        let eltdate = match parse_int(
            unsafe { CStr::from_ptr(member_header.ar_date.as_ptr()).to_bytes() },
            10,
            i64::MAX as u64,
            "date",
            archive,
            name
        ) {
            Ok(v) => v as i64,
            Err(_) => {
                unsafe { close(desc); }
                return -2;
            }
        };

        let eltuid = match parse_int(
            unsafe { CStr::from_ptr(member_header.ar_uid.as_ptr()).to_bytes() },
            10,
            i32::MAX as u64,
            "uid",
            archive,
            name
        ) {
            Ok(v) => v as i32,
            Err(_) => {
                unsafe { close(desc); }
                return -2;
            }
        };

        let eltgid = match parse_int(
            unsafe { CStr::from_ptr(member_header.ar_gid.as_ptr()).to_bytes() },
            10,
            i32::MAX as u64,
            "gid",
            archive,
            name
        ) {
            Ok(v) => v as i32,
            Err(_) => {
                unsafe { close(desc); }
                return -2;
            }
        };

        let fnval = function(
            desc,
            name,
            false,
            member_offset,
            member_offset + AR_HDR_SIZE as i64,
            eltsize,
            eltdate,
            eltuid,
            eltgid,
            eltmode,
            arg
        );

        if fnval != 0 {
            unsafe { close(desc); }
            return fnval;
        }

        if is_namemap {
            let mut map = vec![0u8; eltsize as usize];
            let nread = unsafe {
                libc::read(desc, map.as_mut_ptr() as *mut libc::c_void, eltsize as usize)
            };
            if nread != eltsize as isize {
                unsafe { close(desc); }
                return -2;
            }
            namemap = Some(map);
            namemap_size = eltsize as usize;
        }

        member_offset += AR_HDR_SIZE as i64 + eltsize;
        if member_offset % 2 != 0 {
            member_offset += 1;
        }
    }

    unsafe { close(desc); }
    0
}

fn ar_name_equal(name: &str, mem: &str, truncated: bool) -> bool {
    let name = Path::new(name).file_name().unwrap_or(OsStr::new(name)).to_str().unwrap_or(name);
    
    if truncated {
        let len = mem.len().min(15);
        name.len() >= len && name[..len] == mem[..len]
    } else {
        name == mem
    }
}

fn ar_member_pos(
    _desc: i32,
    mem: &str,
    truncated: bool,
    hdrpos: i64,
    _datapos: i64,
    _size: i64,
    _date: i64,
    _uid: i32,
    _gid: i32,
    _mode: u32,
    name: &(),
) -> i64 {
    if ar_name_equal(unsafe { mem::transmute(name) }, mem, truncated) {
        hdrpos
    } else {
        0
    }
}

fn ar_member_touch(arname: &str, memname: &str) -> i32 {
    let pos = ar_scan(arname, ar_member_pos, unsafe { mem::transmute(memname) });
    if pos < 0 {
        return pos as i32;
    }
    if pos == 0 {
        return 1;
    }

    let fd = unsafe { open(CString::new(arname).unwrap().as_ptr(), O_RDWR, 0o666) };
    if fd < 0 {
        return -3;
    }

    if unsafe { lseek(fd, pos, SEEK_SET) } < 0 {
        unsafe { close(fd); }
        return -3;
    }

    let mut ar_hdr: ArHdr = unsafe { mem::zeroed() };
    if unsafe { libc::read(fd, &mut ar_hdr as *mut _ as *mut libc::c_void, AR_HDR_SIZE) } != AR_HDR_SIZE as isize {
        unsafe { close(fd); }
        return -3;
    }

    let mut statbuf: libc::stat = unsafe { mem::zeroed() };
    if unsafe { libc::fstat(fd, &mut statbuf) } < 0 {
        unsafe { close(fd); }
        return -3;
    }

    let date_str = format!("{}", statbuf.st_mtime);
    let date_bytes = date_str.as_bytes();
    let len = date_bytes.len().min(11);
    unsafe {
        ptr::copy_nonoverlapping(
            date_bytes.as_ptr(),
            ar_hdr.ar_date.as_mut_ptr() as *mut u8,
            len
        );
        for i in len..11 {
            ar_hdr.ar_date[i] = b' ' as c_char;
        }
    }

    if unsafe { lseek(fd, pos, SEEK_SET) } < 0 {
        unsafe { close(fd); }
        return -3;
    }

    if unsafe { libc::write(fd, &ar_hdr as *const _ as *const libc::c_void, AR_HDR_SIZE) } != AR_HDR_SIZE as isize {
        unsafe { close(fd); }
        return -3;
    }

    unsafe { close(fd); }
    0
}