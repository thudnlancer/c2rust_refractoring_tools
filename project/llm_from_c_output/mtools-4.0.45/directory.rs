use std::io::{self, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::mem;

const MDIR_SIZE: usize = 32;
const ENDMARK: u8 = 0;

#[derive(Debug, Clone)]
pub struct DosName {
    pub base: [u8; 8],
    pub ext: [u8; 3],
}

#[derive(Debug)]
pub struct Directory {
    pub attr: u8,
    pub ctime_ms: u8,
    pub ctime: [u8; 2],
    pub time: [u8; 2],
    pub adate: [u8; 2],
    pub cdate: [u8; 2],
    pub date: [u8; 2],
    pub start: [u8; 2],
    pub start_hi: [u8; 2],
    pub size: [u8; 4],
    pub case_flags: u8,
}

#[derive(Debug)]
pub struct DirEntry<'a> {
    pub dir_stream: &'a mut dyn ReadWriteSeek,
    pub entry: usize,
    pub dir: Directory,
}

pub trait ReadWriteSeek: Read + Write + Seek {}
impl<T: Read + Write + Seek> ReadWriteSeek for T {}

pub fn dir_read(entry: &mut DirEntry) -> io::Result<&Directory> {
    let offset = entry.entry as u64 * MDIR_SIZE as u64;
    let buf = unsafe {
        std::slice::from_raw_parts_mut(
            &mut entry.dir as *mut Directory as *mut u8,
            MDIR_SIZE
        )
    };
    
    entry.dir_stream.seek(io::SeekFrom::Start(offset))?;
    entry.dir_stream.read_exact(buf)?;
    
    Ok(&entry.dir)
}

pub fn dir_grow(dir: &mut dyn ReadWriteSeek, size: usize, cluster_bytes: usize) -> io::Result<()> {
    let mut buffer = vec![0u8; cluster_bytes];
    let offset = size as u64 * MDIR_SIZE as u64;
    
    dir.seek(io::SeekFrom::Start(offset))?;
    dir.write_all(&buffer)?;
    
    Ok(())
}

pub fn low_level_dir_write(entry: &DirEntry) -> io::Result<()> {
    let offset = entry.entry as u64 * MDIR_SIZE as u64;
    let buf = unsafe {
        std::slice::from_raw_parts(
            &entry.dir as *const Directory as *const u8,
            MDIR_SIZE
        )
    };
    
    entry.dir_stream.seek(io::SeekFrom::Start(offset))?;
    entry.dir_stream.write_all(buf)?;
    
    Ok(())
}

pub fn low_level_dir_write_end(dir: &mut dyn ReadWriteSeek, entry: usize) -> io::Result<()> {
    let offset = entry as u64 * MDIR_SIZE as u64;
    dir.seek(io::SeekFrom::Start(offset))?;
    dir.write_all(&[ENDMARK])?;
    Ok(())
}

pub fn mk_entry(
    dn: &DosName,
    attr: u8,
    fat: u32,
    size: u32,
    date: SystemTime,
    ndir: &mut Directory,
) -> &mut Directory {
    let duration = date.duration_since(UNIX_EPOCH).unwrap();
    let time = duration.as_secs();
    
    let tm = time_to_tm(time);
    
    dosname_to_direntry(dn, ndir);
    ndir.attr = attr;
    ndir.ctime_ms = 0;
    
    let hour = (tm.tm_hour << 3) as u8;
    let min_hi = (tm.tm_min >> 3) as u8;
    let min_low = (tm.tm_min << 5) as u8;
    let sec = (tm.tm_sec / 2) as u8;
    
    ndir.ctime[1] = hour + min_hi;
    ndir.ctime[0] = min_low + sec;
    ndir.time[1] = ndir.ctime[1];
    ndir.time[0] = ndir.ctime[0];
    
    let year = ((tm.tm_year - 80) << 1) as u8;
    let month_hi = ((tm.tm_mon + 1) >> 3) as u8;
    let month_low = ((tm.tm_mon + 1) << 5) as u8;
    let day = tm.tm_mday as u8;
    
    ndir.adate[1] = year + month_hi;
    ndir.cdate[1] = ndir.adate[1];
    ndir.date[1] = ndir.adate[1];
    
    ndir.adate[0] = month_low + day;
    ndir.cdate[0] = ndir.adate[0];
    ndir.date[0] = ndir.adate[0];
    
    set_word(&mut ndir.start, (fat & 0xFFFF) as u16);
    set_word(&mut ndir.start_hi, (fat >> 16) as u16);
    set_dword(&mut ndir.size, size);
    
    ndir
}

pub fn mk_entry_from_base(
    base: &str,
    attr: u8,
    fat: u32,
    size: u32,
    date: SystemTime,
    ndir: &mut Directory,
) -> &mut Directory {
    let mut dn = DosName {
        base: [b' '; 8],
        ext: [b' '; 3],
    };
    
    let base_bytes = base.as_bytes();
    let len = base_bytes.len().min(8);
    dn.base[..len].copy_from_slice(&base_bytes[..len]);
    
    let entry = mk_entry(&dn, attr, fat, size, date, ndir);
    entry.case_flags = 0;
    entry
}

// Helper functions
fn set_word(dest: &mut [u8; 2], value: u16) {
    dest[0] = value as u8;
    dest[1] = (value >> 8) as u8;
}

fn set_dword(dest: &mut [u8; 4], value: u32) {
    dest[0] = value as u8;
    dest[1] = (value >> 8) as u8;
    dest[2] = (value >> 16) as u8;
    dest[3] = (value >> 24) as u8;
}

fn time_to_tm(secs: u64) -> Tm {
    // Simplified time conversion - in real code use chrono or similar
    unimplemented!()
}

fn dosname_to_direntry(dn: &DosName, dir: &mut Directory) {
    // Implement DOS name to directory entry conversion
    unimplemented!()
}

struct Tm {
    tm_sec: i32,
    tm_min: i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon: i32,
    tm_year: i32,
    // Other fields omitted for brevity
}