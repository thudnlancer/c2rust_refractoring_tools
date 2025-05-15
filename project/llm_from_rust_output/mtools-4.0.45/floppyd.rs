use std::{
    ffi::{CStr, CString},
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    mem,
    net::{TcpListener, TcpStream},
    os::unix::{
        fs::OpenOptionsExt,
        io::{AsRawFd, FromRawFd, RawFd},
    },
    path::Path,
    ptr,
    time::Duration,
};

use libc::{
    self, c_char, c_int, c_uchar, c_uint, c_ulong, c_void, gid_t, pid_t, size_t, ssize_t, uid_t,
};

const OP_READ: u32 = 0;
const OP_WRITE: u32 = 1;
const OP_SEEK: u32 = 2;
const OP_FLUSH: u32 = 3;
const OP_CLOSE: u32 = 4;
const OP_IOCTL: u32 = 5;
const OP_OPRO: u32 = 6;
const OP_OPRW: u32 = 7;
const OP_SEEK64: u32 = 8;

const AUTH_SUCCESS: u32 = 0;
const AUTH_PACKETOVERSIZE: u32 = 1;
const AUTH_AUTHFAILED: u32 = 2;
const AUTH_WRONGVERSION: u32 = 3;
const AUTH_DEVLOCKED: u32 = 4;
const AUTH_BADPACKET: u32 = 5;
const AUTH_IO_ERROR: u32 = 6;

struct Packet {
    data: Vec<u8>,
}

impl Packet {
    fn new() -> Self {
        Packet { data: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        Packet {
            data: Vec::with_capacity(capacity),
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn put_dword(&mut self, offset: usize, value: u32) {
        self.data[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    }

    fn put_qword(&mut self, offset: usize, value: u64) {
        self.data[offset..offset + 8].copy_from_slice(&value.to_be_bytes());
    }

    fn get_dword(&self, offset: usize) -> u32 {
        u32::from_be_bytes([
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ])
    }

    fn get_qword(&self, offset: usize) -> u64 {
        u64::from_be_bytes([
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
            self.data[offset + 4],
            self.data[offset + 5],
            self.data[offset + 6],
            self.data[offset + 7],
        ])
    }
}

struct IoBuffer {
    handle: RawFd,
    in_buffer: Vec<u8>,
    in_start: usize,
    in_valid: usize,
    out_buffer: Vec<u8>,
    out_valid: usize,
}

impl IoBuffer {
    fn new(handle: RawFd) -> Self {
        IoBuffer {
            handle,
            in_buffer: vec![0; 16348],
            in_start: 0,
            in_valid: 0,
            out_buffer: vec![0; 16348],
            out_valid: 0,
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.out_valid > 0 {
            let written = unsafe {
                libc::write(
                    self.handle,
                    self.out_buffer.as_ptr() as *const c_void,
                    self.out_valid,
                )
            };
            if written < 0 {
                return Err(io::Error::last_os_error());
            }
            self.out_valid = 0;
        }
        Ok(())
    }

    fn buf_read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let nbytes = buffer.len();
        if nbytes <= self.in_valid {
            buffer.copy_from_slice(&self.in_buffer[self.in_start..self.in_start + nbytes]);
            self.in_valid -= nbytes;
            self.in_start += nbytes;
            return Ok(nbytes);
        }

        let mut read = 0;
        if self.in_valid > 0 {
            buffer[..self.in_valid].copy_from_slice(&self.in_buffer[self.in_start..]);
            read = self.in_valid;
            self.in_start = 0;
            self.in_valid = 0;
        }

        if nbytes - read > 16348 {
            let rval = unsafe {
                libc::read(
                    self.handle,
                    buffer[read..].as_mut_ptr() as *mut c_void,
                    nbytes - read,
                )
            };
            if rval < 0 {
                return Err(io::Error::last_os_error());
            }
            return Ok(read + rval as usize);
        }

        let rval = unsafe {
            libc::read(
                self.handle,
                self.in_buffer.as_mut_ptr() as *mut c_void,
                16348,
            )
        };
        if rval < 0 {
            return Err(io::Error::last_os_error());
        }

        let rval = rval as usize;
        if rval < nbytes - read {
            buffer[read..read + rval].copy_from_slice(&self.in_buffer[..rval]);
            Ok(read + rval)
        } else {
            buffer[read..].copy_from_slice(&self.in_buffer[..nbytes - read]);
            self.in_start = nbytes - read;
            self.in_valid = rval - (nbytes - read);
            Ok(nbytes)
        }
    }

    fn buf_write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if self.out_valid + buffer.len() > 16348 {
            self.flush()?;
            let written = unsafe {
                libc::write(
                    self.handle,
                    buffer.as_ptr() as *const c_void,
                    buffer.len(),
                )
            };
            if written < 0 {
                return Err(io::Error::last_os_error());
            }
            return Ok(written as usize);
        }

        self.out_buffer[self.out_valid..self.out_valid + buffer.len()].copy_from_slice(buffer);
        self.out_valid += buffer.len();
        Ok(buffer.len())
    }
}

fn send_packet(packet: &Packet, io_buffer: &mut IoBuffer) -> io::Result<()> {
    if !packet.data.is_empty() {
        let len_bytes = (packet.len() as u32).to_be_bytes();
        io_buffer.buf_write(&len_bytes)?;
        io_buffer.buf_write(&packet.data)?;
        io_buffer.flush()?;
    }
    Ok(())
}

fn recv_packet(
    packet: &mut Packet,
    io_buffer: &mut IoBuffer,
    max_length: u32,
) -> io::Result<bool> {
    let mut len_bytes = [0u8; 4];
    io_buffer.buf_read(&mut len_bytes)?;
    let length = u32::from_be_bytes(len_bytes);

    if length > max_length || length == 0xffffffff {
        return Ok(false);
    }

    packet.data.resize(length as usize, 0);
    let mut start = 0;
    while start < length as usize {
        let read = io_buffer.buf_read(&mut packet.data[start..])?;
        if read == 0 {
            return Ok(false);
        }
        start += read;
    }

    Ok(!packet.data.is_empty())
}

fn serve_client(
    sockhandle: RawFd,
    device_names: &[&str],
    close_stderr: bool,
) -> io::Result<()> {
    let mut sock = IoBuffer::new(sockhandle);

    // Authentication and version negotiation would go here
    // For simplicity, we'll assume authentication succeeds

    let mut opcode = Packet::new();
    let mut parm = Packet::new();
    let mut dev_fd: Option<File> = None;
    let mut read_only = true;

    loop {
        if !recv_packet(&mut opcode, &mut sock, 1)? {
            break;
        }

        recv_packet(&mut parm, &mut sock, 3000000)?;

        match opcode.data[0] {
            OP_READ => {
                if let Some(file) = &mut dev_fd {
                    let len = parm.get_dword(0) as usize;
                    parm.data.resize(len, 0);
                    let read = file.read(&mut parm.data)?;
                    parm.data.truncate(read);
                    send_packet(&parm, &mut sock)?;
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::NotConnected,
                        "No device opened",
                    ));
                }
            }
            OP_WRITE => {
                if read_only {
                    return Err(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        "Device is read-only",
                    ));
                }
                if let Some(file) = &mut dev_fd {
                    file.write_all(&parm.data)?;
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::NotConnected,
                        "No device opened",
                    ));
                }
            }
            OP_SEEK => {
                if let Some(file) = &mut dev_fd {
                    let offset = parm.get_dword(0) as i64;
                    let whence = parm.get_dword(4) as i32;
                    file.seek(SeekFrom::Start(offset as u64))?;
                }
            }
            OP_CLOSE => {
                dev_fd = None;
                break;
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unknown opcode",
                ));
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let device_names = if args.len() > 1 {
        &args[1..]
    } else {
        &["/dev/fd0"]
    };

    let listener = TcpListener::bind("0.0.0.0:5703")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = serve_client(stream.as_raw_fd(), device_names, true) {
                    eprintln!("Error serving client: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}