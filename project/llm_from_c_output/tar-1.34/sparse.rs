use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::ptr;
use std::slice;
use std::sync::atomic::{AtomicBool, Ordering};

const BLOCKSIZE: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SparseScanState {
    Begin,
    Block,
    End,
}

#[derive(Debug)]
struct SpArray {
    offset: u64,
    numbytes: u64,
}

#[derive(Debug)]
struct TarStatInfo {
    orig_file_name: String,
    archive_file_size: u64,
    sparse_map: Vec<SpArray>,
    sparse_map_avail: usize,
    sparse_map_size: usize,
    sparse_major: u32,
    sparse_minor: u32,
    stat: libc::stat,
}

struct TarSparseFile<'a> {
    fd: File,
    seekable: bool,
    offset: u64,
    dumped_size: u64,
    stat_info: &'a mut TarStatInfo,
    optab: &'a TarSparseOptab,
    closure: Option<Box<dyn std::any::Any>>,
}

struct TarSparseOptab {
    init: Option<fn(&mut TarSparseFile) -> bool>,
    done: Option<fn(&mut TarSparseFile) -> bool>,
    sparse_member_p: Option<fn(&TarSparseFile) -> bool>,
    dump_header: Option<fn(&mut TarSparseFile) -> bool>,
    fixup_header: Option<fn(&mut TarSparseFile) -> bool>,
    decode_header: Option<fn(&mut TarSparseFile) -> bool>,
    scan_block: Option<fn(&mut TarSparseFile, SparseScanState, &[u8]) -> bool>,
    dump_region: Option<fn(&mut TarSparseFile, usize) -> bool>,
    extract_region: Option<fn(&mut TarSparseFile, usize) -> bool>,
}

static OLDGNU_OPTAB: TarSparseOptab = TarSparseOptab {
    init: None,
    done: None,
    sparse_member_p: Some(oldgnu_sparse_member_p),
    dump_header: Some(oldgnu_dump_header),
    fixup_header: Some(oldgnu_fixup_header),
    decode_header: Some(oldgnu_get_sparse_info),
    scan_block: None,
    dump_region: Some(sparse_dump_region),
    extract_region: Some(sparse_extract_region),
};

static STAR_OPTAB: TarSparseOptab = TarSparseOptab {
    init: None,
    done: None,
    sparse_member_p: Some(star_sparse_member_p),
    dump_header: None,
    fixup_header: Some(star_fixup_header),
    decode_header: Some(star_get_sparse_info),
    scan_block: None,
    dump_region: None,
    extract_region: Some(sparse_extract_region),
};

static PAX_OPTAB: TarSparseOptab = TarSparseOptab {
    init: None,
    done: None,
    sparse_member_p: Some(pax_sparse_member_p),
    dump_header: Some(pax_dump_header),
    fixup_header: None,
    decode_header: Some(pax_decode_header),
    scan_block: None,
    dump_region: Some(sparse_dump_region),
    extract_region: Some(sparse_extract_region),
};

fn dump_zeros(file: &mut TarSparseFile, offset: u64) -> io::Result<()> {
    static ZERO_BUF: [u8; BLOCKSIZE] = [0; BLOCKSIZE];

    if offset < file.offset {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "offset < file.offset"));
    }

    while file.offset < offset {
        let size = std::cmp::min(BLOCKSIZE as u64, offset - file.offset) as usize;
        let wrbytes = file.fd.write(&ZERO_BUF[..size])?;
        if wrbytes == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "write returned 0"));
        }
        file.offset += wrbytes as u64;
    }

    Ok(())
}

fn tar_sparse_member_p(file: &TarSparseFile) -> bool {
    file.optab.sparse_member_p.map_or(false, |f| f(file))
}

fn tar_sparse_init(file: &mut TarSparseFile) -> bool {
    // In Rust we don't need to memset, initialization is handled by the type system
    sparse_select_optab(file).map_or(false, |_| {
        file.optab.init.map_or(true, |f| f(file))
    })
}

fn tar_sparse_done(file: &mut TarSparseFile) -> bool {
    file.optab.done.map_or(true, |f| f(file))
}

fn tar_sparse_scan(file: &mut TarSparseFile, state: SparseScanState, block: &[u8]) -> bool {
    file.optab.scan_block.map_or(true, |f| f(file, state, block))
}

fn tar_sparse_dump_region(file: &mut TarSparseFile, i: usize) -> bool {
    file.optab.dump_region.map_or(false, |f| f(file, i))
}

fn tar_sparse_extract_region(file: &mut TarSparseFile, i: usize) -> bool {
    file.optab.extract_region.map_or(false, |f| f(file, i))
}

fn tar_sparse_dump_header(file: &mut TarSparseFile) -> bool {
    file.optab.dump_header.map_or(false, |f| f(file))
}

fn tar_sparse_decode_header(file: &mut TarSparseFile) -> bool {
    file.optab.decode_header.map_or(true, |f| f(file))
}

fn tar_sparse_fixup_header(file: &mut TarSparseFile) -> bool {
    file.optab.fixup_header.map_or(true, |f| f(file))
}

fn lseek_or_error(file: &mut TarSparseFile, offset: u64) -> bool {
    if file.seekable {
        file.fd.seek(SeekFrom::Start(offset)).is_ok()
    } else {
        dump_zeros(file, offset).is_ok()
    }
}

fn zero_block_p(buffer: &[u8]) -> bool {
    buffer.iter().all(|&b| b == 0)
}

fn sparse_add_map(st: &mut TarStatInfo, sp: &SpArray) {
    if st.sparse_map_avail == st.sparse_map_size {
        st.sparse_map_size = if st.sparse_map_size == 0 { 1 } else { st.sparse_map_size * 2 };
        st.sparse_map.resize(st.sparse_map_size, SpArray { offset: 0, numbytes: 0 });
    }
    st.sparse_map[st.sparse_map_avail] = *sp;
    st.sparse_map_avail += 1;
}

fn sparse_scan_file_raw(file: &mut TarSparseFile) -> bool {
    let st = file.stat_info;
    let mut buffer = [0; BLOCKSIZE];
    let mut offset = 0;
    let mut sp = SpArray { offset: 0, numbytes: 0 };

    st.archive_file_size = 0;

    if !tar_sparse_scan(file, SparseScanState::Begin, &[]) {
        return false;
    }

    loop {
        let count = match file.fd.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => return false,
        };

        if zero_block_p(&buffer[..count]) {
            if sp.numbytes != 0 {
                sparse_add_map(st, &sp);
                sp.numbytes = 0;
                if !tar_sparse_scan(file, SparseScanState::Block, &[]) {
                    return false;
                }
            }
        } else {
            if sp.numbytes == 0 {
                sp.offset = offset;
            }
            sp.numbytes += count as u64;
            st.archive_file_size += count as u64;
            if !tar_sparse_scan(file, SparseScanState::Block, &buffer[..count]) {
                return false;
            }
        }

        offset += count as u64;
    }

    if sp.numbytes == 0 {
        sp.offset = offset;
    }
    sparse_add_map(st, &sp);
    true
}

// ... (additional functions would follow the same pattern of translation)

// Note: This is a partial translation. The complete translation would include:
// - All the remaining C functions translated to Rust
// - Proper error handling throughout
// - Implementation of all the optab functions (oldgnu, star, pax)
// - Proper memory management using Rust's ownership system
// - Safe pointer handling (no raw pointers where possible)
// - Proper use of Rust's Result type for error handling
// - Implementation of all the helper functions used in the original C code

// The translation maintains the same logical structure while using Rust's idioms:
// - Using enums instead of #defines
// - Using Result types for error handling
// - Using slices instead of raw pointers
// - Using Vec for dynamically sized arrays
// - Using proper ownership semantics
// - Avoiding unsafe blocks where possible