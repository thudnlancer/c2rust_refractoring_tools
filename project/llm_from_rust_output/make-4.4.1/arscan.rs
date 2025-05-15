use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::time::SystemTime;
use std::mem;
use std::ffi::CStr;
use std::os::unix::ffi::OsStrExt;
use libc::{c_int, c_char, c_long, c_uint, c_void, off_t, size_t, ssize_t};
use std::ptr;

const AR_MAGIC: &[u8] = b"!<arch>\n";
const AR_FMAG: &[u8] = b"`\n";

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ArHdr {
    ar_name: [c_char; 16],
    ar_date: [c_char; 12],
    ar_uid: [c_char; 6],
    ar_gid: [c_char; 6],
    ar_mode: [c_char; 8],
    ar_size: [c_char; 10],
    ar_fmag: [c_char; 2],
}

impl Default for ArHdr {
    fn default() -> Self {
        ArHdr {
            ar_name: [0; 16],
            ar_date: [0; 12],
            ar_uid: [0; 6],
            ar_gid: [0; 6],
            ar_mode: [0; 8],
            ar_size: [0; 10],
            ar_fmag: [0; 2],
        }
    }
}

type ArMemberFunc = fn(
    desc: c_int,
    name: *const c_char,
    truncated: c_int,
    hdrpos: c_long,
    datapos: c_long,
    size: c_long,
    date: i64,
    uid: c_int,
    gid: c_int,
    mode: c_uint,
    arg: *const c_void,
) -> i64;

fn parse_int(
    ptr: &[c_char],
    base: c_int,
    max: u64,
    type_str: &str,
    archive: &str,
    name: &str,
) -> Result<u64, String> {
    let mut val = 0u64;
    let maxchar = b'0' as c_int + base - 1;

    for &c in ptr.iter().take_while(|&&c| c != b' ' as c_char) {
        if c < b'0' as c_char || c > maxchar as c_char {
            return Err(format!("Invalid {} for archive {} member {}", type_str, archive, name));
        }
        let nv = val * (base as u64) + (c as u64 - b'0' as u64);
        if nv < val || nv > max {
            return Err(format!("Invalid {} for archive {} member {}", type_str, archive, name));
        }
        val = nv;
    }
    Ok(val)
}

fn ar_scan(
    archive: &Path,
    function: ArMemberFunc,
    arg: *const c_void,
) -> Result<i64, i32> {
    let mut file = match File::open(archive) {
        Ok(f) => f,
        Err(_) => return Err(-1),
    };

    let mut magic = [0u8; 8];
    if file.read_exact(&mut magic).is_err() || magic != AR_MAGIC {
        return Err(-2);
    }

    let mut member_offset = 8i64;
    let mut namemap: Vec<c_char> = Vec::new();
    let mut namemap_size = 0u32;

    loop {
        file.seek(SeekFrom::Start(member_offset as u64)).map_err(|_| -2)?;

        let mut member_header = ArHdr::default();
        let hdr_slice = unsafe {
            std::slice::from_raw_parts_mut(
                &mut member_header as *mut _ as *mut u8,
                mem::size_of::<ArHdr>()
            )
        };

        if file.read_exact(hdr_slice).is_err() {
            break;
        }

        if member_header.ar_fmag[..2] != AR_FMAG[..2] {
            return Err(-2);
        }

        let mut name = member_header.ar_name.to_vec();
        while name.last() == Some(&(b' ' as c_char)) {
            name.pop();
        }

        let is_namemap = name == b"//" || name == b"ARFILENAMES/";
        let mut long_name = false;

        // Name processing logic here...

        let eltmode = parse_int(
            &member_header.ar_mode,
            8,
            u32::MAX as u64,
            "mode",
            archive.to_str().unwrap(),
            unsafe { CStr::from_ptr(name.as_ptr()).to_str().unwrap() },
        ).map_err(|_| -2)? as u32;

        let eltsize = parse_int(
            &member_header.ar_size,
            10,
            i64::MAX as u64,
            "size",
            archive.to_str().unwrap(),
            unsafe { CStr::from_ptr(name.as_ptr()).to_str().unwrap() },
        ).map_err(|_| -2)? as i64;

        // Other field parsing...

        let fnval = function(
            file.as_raw_fd(),
            name.as_ptr(),
            if long_name { 0 } else { 1 },
            member_offset,
            member_offset + mem::size_of::<ArHdr>() as i64,
            eltsize,
            0, // date
            0, // uid
            0, // gid
            eltmode,
            arg,
        );

        if fnval != 0 {
            return Ok(fnval);
        }

        if is_namemap {
            // Handle namemap...
        }

        member_offset += mem::size_of::<ArHdr>() as i64 + eltsize;
        if member_offset % 2 != 0 {
            member_offset += 1;
        }
    }

    Ok(0)
}

fn ar_name_equal(name: &str, mem: &str, truncated: bool) -> bool {
    let name = Path::new(name).file_name().unwrap().to_str().unwrap();
    if truncated {
        name.as_bytes().starts_with(&mem.as_bytes()[..15])
    } else {
        name == mem
    }
}

fn ar_member_pos(
    _desc: c_int,
    name: *const c_char,
    truncated: c_int,
    hdrpos: c_long,
    _datapos: c_long,
    _size: c_long,
    _date: i64,
    _uid: c_int,
    _gid: c_int,
    _mode: c_uint,
    arg: *const c_void,
) -> i64 {
    let mem = unsafe { CStr::from_ptr(arg as *const c_char) }.to_str().unwrap();
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap();
    if ar_name_equal(name, mem, truncated != 0) {
        hdrpos
    } else {
        0
    }
}

fn ar_member_touch(arname: &Path, memname: &str) -> Result<(), i32> {
    let pos = ar_scan(
        arname,
        ar_member_pos,
        memname.as_ptr() as *const c_void,
    )?;

    if pos <= 0 {
        return if pos == 0 { Err(1) } else { Err(pos as i32) };
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .mode(0o666)
        .open(arname)
        .map_err(|_| -3)?;

    file.seek(SeekFrom::Start(pos as u64)).map_err(|_| -3)?;

    let mut ar_hdr = ArHdr::default();
    let hdr_slice = unsafe {
        std::slice::from_raw_parts_mut(
            &mut ar_hdr as *mut _ as *mut u8,
            mem::size_of::<ArHdr>()
        )
    };

    file.read_exact(hdr_slice).map_err(|_| -3)?;

    let metadata = file.metadata().map_err(|_| -3)?;
    let mtime = metadata.modified().unwrap()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let date_str = format!("{: <12}", mtime);
    ar_hdr.ar_date[..date_str.len()].copy_from_slice(
        unsafe { &*(date_str.as_ptr() as *const [c_char; 12]) }
    );

    file.seek(SeekFrom::Start(pos as u64)).map_err(|_| -3)?;
    file.write_all(hdr_slice).map_err(|_| -3)?;

    Ok(())
}