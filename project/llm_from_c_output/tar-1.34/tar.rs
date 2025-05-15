use std::{
    ffi::{CStr, CString},
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    mem,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const BLOCKSIZE: usize = 512;

#[derive(Debug, Clone)]
struct PosixHeader {
    name: [u8; 100],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    size: [u8; 12],
    mtime: [u8; 12],
    chksum: [u8; 8],
    typeflag: u8,
    linkname: [u8; 100],
    magic: [u8; 6],
    version: [u8; 2],
    uname: [u8; 32],
    gname: [u8; 32],
    devmajor: [u8; 8],
    devminor: [u8; 8],
    prefix: [u8; 155],
}

impl Default for PosixHeader {
    fn default() -> Self {
        PosixHeader {
            name: [0; 100],
            mode: [0; 8],
            uid: [0; 8],
            gid: [0; 8],
            size: [0; 12],
            mtime: [0; 12],
            chksum: [0; 8],
            typeflag: 0,
            linkname: [0; 100],
            magic: [0; 6],
            version: [0; 2],
            uname: [0; 32],
            gname: [0; 32],
            devmajor: [0; 8],
            devminor: [0; 8],
            prefix: [0; 155],
        }
    }
}

#[derive(Debug)]
struct TarStatInfo {
    orig_file_name: Option<String>,
    file_name: Option<String>,
    had_trailing_slash: bool,
    link_name: Option<String>,
    uname: Option<String>,
    gname: Option<String>,
    cntx_name: Option<String>,
    acls_a_ptr: Option<Vec<u8>>,
    acls_a_len: usize,
    acls_d_ptr: Option<Vec<u8>>,
    acls_d_len: usize,
    stat: libc::stat,
    atime: libc::timespec,
    mtime: libc::timespec,
    ctime: libc::timespec,
    archive_file_size: i64,
    is_sparse: bool,
    sparse_major: u32,
    sparse_minor: u32,
    sparse_map_avail: usize,
    sparse_map_size: usize,
    sparse_map: Option<Vec<libc::off_t>>,
    real_size: i64,
    real_size_set: bool,
    sparse_name_done: bool,
    xattr_map_size: usize,
    xattr_map: Option<Vec<(String, Vec<u8>)>>,
    is_dumpdir: bool,
    skipped: bool,
    dumpdir: Option<String>,
    parent: Option<Box<TarStatInfo>>,
    dirstream: Option<*mut libc::DIR>,
    fd: i32,
    exclude_list: Option<Vec<String>>,
}

impl Default for TarStatInfo {
    fn default() -> Self {
        TarStatInfo {
            orig_file_name: None,
            file_name: None,
            had_trailing_slash: false,
            link_name: None,
            uname: None,
            gname: None,
            cntx_name: None,
            acls_a_ptr: None,
            acls_a_len: 0,
            acls_d_ptr: None,
            acls_d_len: 0,
            stat: unsafe { mem::zeroed() },
            atime: unsafe { mem::zeroed() },
            mtime: unsafe { mem::zeroed() },
            ctime: unsafe { mem::zeroed() },
            archive_file_size: 0,
            is_sparse: false,
            sparse_major: 0,
            sparse_minor: 0,
            sparse_map_avail: 0,
            sparse_map_size: 0,
            sparse_map: None,
            real_size: 0,
            real_size_set: false,
            sparse_name_done: false,
            xattr_map_size: 0,
            xattr_map: None,
            is_dumpdir: false,
            skipped: false,
            dumpdir: None,
            parent: None,
            dirstream: None,
            fd: -1,
            exclude_list: None,
        }
    }
}

impl TarStatInfo {
    fn new() -> Self {
        Self::default()
    }

    fn close(&mut self) -> io::Result<()> {
        if let Some(dir) = self.dirstream.take() {
            unsafe {
                if libc::closedir(dir) != 0 {
                    return Err(io::Error::last_os_error());
                }
            }
        }

        if self.fd > 0 {
            unsafe {
                if libc::close(self.fd) != 0 {
                    return Err(io::Error::last_os_error());
                }
            }
            self.fd = -1;
        }

        Ok(())
    }

    fn destroy(&mut self) {
        let _ = self.close();
        self.orig_file_name = None;
        self.file_name = None;
        self.link_name = None;
        self.uname = None;
        self.gname = None;
        self.cntx_name = None;
        self.acls_a_ptr = None;
        self.acls_d_ptr = None;
        self.sparse_map = None;
        self.xattr_map = None;
        self.dumpdir = None;
        self.parent = None;
        self.exclude_list = None;
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [options] [files]", args[0]);
        return Ok(());
    }

    let mut archive_names = Vec::new();
    let mut subcommand = None;
    let mut verbose = false;
    let mut preserve_permissions = false;
    let mut same_owner = false;
    let mut same_permissions = false;
    let mut extract_to_stdout = false;
    let mut create_archive = false;
    let mut list_archive = false;
    let mut extract_archive = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-c" | "--create" => {
                subcommand = Some("create");
                create_archive = true;
            }
            "-x" | "--extract" | "--get" => {
                subcommand = Some("extract");
                extract_archive = true;
            }
            "-t" | "--list" => {
                subcommand = Some("list");
                list_archive = true;
            }
            "-v" | "--verbose" => verbose = true,
            "-p" | "--preserve-permissions" | "--same-permissions" => {
                preserve_permissions = true;
            }
            "--same-owner" => same_owner = true,
            "--no-same-permissions" => same_permissions = true,
            "-O" | "--to-stdout" => extract_to_stdout = true,
            "-f" | "--file" => {
                i += 1;
                if i < args.len() {
                    archive_names.push(args[i].clone());
                } else {
                    eprintln!("{}: option requires an argument -- 'f'", args[0]);
                    return Ok(());
                }
            }
            _ => {
                if args[i].starts_with('-') {
                    eprintln!("{}: unrecognized option '{}'", args[0], args[i]);
                    return Ok(());
                }
                // Treat as file name
                archive_names.push(args[i].clone());
            }
        }
        i += 1;
    }

    if archive_names.is_empty() {
        archive_names.push("-".to_string());
    }

    match subcommand {
        Some("create") => create_archive(&archive_names)?,
        Some("extract") => extract_archive(&archive_names, extract_to_stdout)?,
        Some("list") => list_archive(&archive_names)?,
        _ => {
            eprintln!("{}: you must specify one of the -c, -t, -x options", args[0]);
            return Ok(());
        }
    }

    Ok(())
}

fn create_archive(archive_names: &[String]) -> io::Result<()> {
    let archive_name = if archive_names[0] == "-" {
        "/dev/stdout"
    } else {
        &archive_names[0]
    };

    let mut archive = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(archive_name)?;

    // In a real implementation, we would recursively add files here
    let header = PosixHeader::default();
    archive.write_all(unsafe {
        std::slice::from_raw_parts(
            &header as *const _ as *const u8,
            mem::size_of::<PosixHeader>(),
        )
    })?;

    Ok(())
}

fn extract_archive(archive_names: &[String], to_stdout: bool) -> io::Result<()> {
    let archive_name = if archive_names[0] == "-" {
        "/dev/stdin"
    } else {
        &archive_names[0]
    };

    let mut archive = File::open(archive_name)?;
    let mut header = PosixHeader::default();
    archive.read_exact(unsafe {
        std::slice::from_raw_parts_mut(
            &mut header as *mut _ as *mut u8,
            mem::size_of::<PosixHeader>(),
        )
    })?;

    // In a real implementation, we would process the header and extract files here
    if to_stdout {
        let mut buffer = vec![0; BLOCKSIZE];
        loop {
            let bytes_read = archive.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            io::stdout().write_all(&buffer[..bytes_read])?;
        }
    }

    Ok(())
}

fn list_archive(archive_names: &[String]) -> io::Result<()> {
    let archive_name = if archive_names[0] == "-" {
        "/dev/stdin"
    } else {
        &archive_names[0]
    };

    let mut archive = File::open(archive_name)?;
    let mut header = PosixHeader::default();
    archive.read_exact(unsafe {
        std::slice::from_raw_parts_mut(
            &mut header as *mut _ as *mut u8,
            mem::size_of::<PosixHeader>(),
        )
    })?;

    // In a real implementation, we would process the header and list files here
    let name = unsafe {
        CStr::from_ptr(header.name.as_ptr() as *const libc::c_char)
            .to_string_lossy()
            .into_owned()
    };
    println!("{}", name);

    Ok(())
}