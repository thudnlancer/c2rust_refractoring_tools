use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::ptr;
use libc::{c_char, c_int, c_void, size_t, off_t};

struct Device {
    name: Option<CString>,
    drive: char,
    fat_bits: i32,
    mode: i32,
    tracks: u32,
    heads: u16,
    sectors: u16,
    hidden: u32,
    offset: off_t,
    partition: u32,
    misc_flags: u32,
    ssize: u8,
    use_2m: u32,
    precmd: Option<CString>,
    file_nr: i32,
    blocksize: u32,
    codepage: u32,
    data_map: Option<CString>,
    tot_sectors: u32,
    sector_size: u16,
    postcmd: Option<CString>,
    cfg_filename: Option<CString>,
}

struct Stream {
    file: File,
    device: Device,
    mode: i32,
}

impl Stream {
    fn new(device: Device, mode: i32) -> Result<Self, String> {
        let path = device.name.as_ref().ok_or("Device name missing")?;
        let file = OpenOptions::new()
            .read(mode & 0o1 == 0)
            .write(mode & 0o1 != 0)
            .open(Path::new(path.to_str().ok_or("Invalid device path")?))
            .map_err(|e| e.to_string())?;
        
        Ok(Self { file, device, mode })
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        self.file.read(buf).map_err(|e| e.to_string())
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, String> {
        self.file.write(buf).map_err(|e| e.to_string())
    }

    fn pread(&mut self, buf: &mut [u8], offset: off_t) -> Result<usize, String> {
        self.file.seek(SeekFrom::Start(offset as u64)).map_err(|e| e.to_string())?;
        self.read(buf)
    }

    fn pwrite(&mut self, buf: &[u8], offset: off_t) -> Result<usize, String> {
        self.file.seek(SeekFrom::Start(offset as u64)).map_err(|e| e.to_string())?;
        self.write(buf)
    }
}

fn usage() -> ! {
    eprintln!("Mtools version {}, dated {}", "version", "date");
    eprintln!("Usage: mcat [-V] [-w] device");
    eprintln!("       -w write on device else read");
    std::process::exit(1);
}

fn buf_len(blocksize: usize, total_size: off_t, address: off_t) -> usize {
    if total_size == 0 {
        blocksize
    } else if blocksize as off_t > total_size - address {
        (total_size - address) as usize
    } else {
        blocksize
    }
}

fn mcat(args: Vec<String>) {
    let mut mode = 0;
    let mut drive = '\0';
    let mut devices: Vec<Device> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-w" => mode = 0o1,
            "-i" => {
                i += 1;
                if i >= args.len() {
                    usage();
                }
                // set_cmd_line_image would be implemented here
            }
            _ => {
                if args[i].len() >= 2 && args[i].chars().nth(1) == Some(':') {
                    drive = args[i].chars().next().unwrap().to_ascii_uppercase();
                } else {
                    usage();
                }
            }
        }
        i += 1;
    }

    if drive == '\0' {
        drive = 'A'; // get_default_drive() would return this
    }

    let mut stream = None;
    for dev in devices.iter_mut() {
        if dev.drive == drive {
            match Stream::new(dev.clone(), mode) {
                Ok(s) => {
                    stream = Some(s);
                    break;
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    if let Some(mut stream) = stream {
        let mut buf = [0u8; 16000];
        let mut address = 0;

        if mode == 0o1 {
            // Write mode
            let size = 512 * stream.device.tot_sectors as off_t;
            loop {
                let len = buf_len(buf.len(), size, address);
                let read = std::io::stdin().read(&mut buf[..len]).unwrap();
                if read == 0 {
                    break;
                }
                match stream.pwrite(&buf[..read], address) {
                    Ok(written) => {
                        eprintln!("Wrote to {}", address);
                        address += written as off_t;
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        break;
                    }
                }
            }
        } else {
            // Read mode
            loop {
                match stream.pread(&mut buf, address) {
                    Ok(read) if read > 0 => {
                        std::io::stdout().write_all(&buf[..read]).unwrap();
                        address += read as off_t;
                    }
                    _ => break,
                }
            }
        }
        std::process::exit(0);
    } else {
        eprintln!("Drive '{}:' not supported", drive);
        std::process::exit(1);
    }
}

fn main() {
    mcat(std::env::args().collect());
}