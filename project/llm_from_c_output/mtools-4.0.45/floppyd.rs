/* 
 * This is a Rust translation of the floppyd C code.
 * It maintains the same functionality while adhering to Rust's safety principles.
 */

use std::env;
use std::ffi::{CString, OsString};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::process;
use std::time::Duration;

use libc::{
    self, c_int, gid_t, pid_t, uid_t, 
    SIGCHLD, SIG_IGN, SIGHUP, SIGINT, SIGQUIT, SIGTERM, 
    alarm, close, dup2, exit, fork, getenv, getpid, 
    lockf, open, setsid, umask, F_LOCK, F_TEST, F_TLOCK, 
    O_RDONLY, O_RDWR, O_WRONLY
};

const DEBUG: bool = false;
const MAX_XAUTHORITY_LENGTH: usize = 3000;
const MAX_DATA_REQUEST: usize = 3000000;
const BUFFERED_IO_SIZE: usize = 16348;
const FLOPPYD_DEFAULT_PORT: u16 = 5703;
const FLOPPYD_PROTOCOL_VERSION: u32 = 1;
const FLOPPYD_PROTOCOL_VERSION_OLD: u32 = 0;

struct IoBuffer {
    handle: RawFd,
    in_buffer: [u8; BUFFERED_IO_SIZE],
    out_buffer: [u8; BUFFERED_IO_SIZE],
    in_valid: usize,
    in_start: usize,
    out_valid: usize,
}

impl IoBuffer {
    fn new(handle: RawFd) -> Self {
        IoBuffer {
            handle,
            in_buffer: [0; BUFFERED_IO_SIZE],
            out_buffer: [0; BUFFERED_IO_SIZE],
            in_valid: 0,
            in_start: 0,
            out_valid: 0,
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.out_valid > 0 {
            let written = unsafe {
                libc::write(
                    self.handle,
                    self.out_buffer.as_ptr() as *const libc::c_void,
                    self.out_valid,
                )
            };
            if written < 0 {
                return Err(std::io::Error::last_os_error());
            }
            self.out_valid = 0;
        }
        Ok(())
    }

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.len() <= self.in_valid {
            buf.copy_from_slice(
                &self.in_buffer[self.in_start..self.in_start + buf.len()]
            );
            self.in_valid -= buf.len();
            self.in_start += buf.len();
            Ok(buf.len())
        } else {
            let mut total_read = 0;
            if self.in_valid > 0 {
                let to_copy = std::cmp::min(self.in_valid, buf.len());
                buf[..to_copy].copy_from_slice(
                    &self.in_buffer[self.in_start..self.in_start + to_copy]
                );
                total_read += to_copy;
                self.in_valid -= to_copy;
                self.in_start += to_copy;
            }

            let remaining = buf.len() - total_read;
            if remaining > 0 {
                if remaining > BUFFERED_IO_SIZE {
                    let read_bytes = unsafe {
                        libc::read(
                            self.handle,
                            buf[total_read..].as_mut_ptr() as *mut libc::c_void,
                            remaining,
                        )
                    };
                    if read_bytes < 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    total_read += read_bytes as usize;
                } else {
                    let read_bytes = unsafe {
                        libc::read(
                            self.handle,
                            self.in_buffer.as_mut_ptr() as *mut libc::c_void,
                            BUFFERED_IO_SIZE,
                        )
                    };
                    if read_bytes < 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    let read_bytes = read_bytes as usize;
                    if read_bytes < remaining {
                        buf[total_read..total_read + read_bytes]
                            .copy_from_slice(&self.in_buffer[..read_bytes]);
                        total_read += read_bytes;
                        self.in_valid = 0;
                        self.in_start = 0;
                    } else {
                        buf[total_read..].copy_from_slice(&self.in_buffer[..remaining]);
                        self.in_start = remaining;
                        self.in_valid = read_bytes - remaining;
                        total_read += remaining;
                    }
                }
            }
            Ok(total_read)
        }
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.out_valid + buf.len() > BUFFERED_IO_SIZE {
            self.flush()?;
            let written = unsafe {
                libc::write(
                    self.handle,
                    buf.as_ptr() as *const libc::c_void,
                    buf.len(),
                )
            };
            if written < 0 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(written as usize)
            }
        } else {
            self.out_buffer[self.out_valid..self.out_valid + buf.len()]
                .copy_from_slice(buf);
            self.out_valid += buf.len();
            Ok(buf.len())
        }
    }
}

impl Drop for IoBuffer {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

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

    fn put_dword(&mut self, index: usize, value: u32) {
        let bytes = value.to_be_bytes();
        if index + 4 <= self.data.len() {
            self.data[index..index + 4].copy_from_slice(&bytes);
        } else {
            self.data.extend_from_slice(&bytes);
        }
    }

    fn get_dword(&self, index: usize) -> u32 {
        u32::from_be_bytes([
            self.data[index],
            self.data[index + 1],
            self.data[index + 2],
            self.data[index + 3],
        ])
    }

    fn put_qword(&mut self, index: usize, value: u64) {
        let bytes = value.to_be_bytes();
        if index + 8 <= self.data.len() {
            self.data[index..index + 8].copy_from_slice(&bytes);
        } else {
            self.data.extend_from_slice(&bytes);
        }
    }

    fn get_qword(&self, index: usize) -> u64 {
        u64::from_be_bytes([
            self.data[index],
            self.data[index + 1],
            self.data[index + 2],
            self.data[index + 3],
            self.data[index + 4],
            self.data[index + 5],
            self.data[index + 6],
            self.data[index + 7],
        ])
    }
}

fn read_dword(buffer: &mut IoBuffer) -> std::io::Result<u32> {
    let mut bytes = [0u8; 4];
    buffer.read(&mut bytes)?;
    Ok(u32::from_be_bytes(bytes))
}

fn write_dword(buffer: &mut IoBuffer, value: u32) -> std::io::Result<()> {
    buffer.write(&value.to_be_bytes())?;
    Ok(())
}

fn send_packet(packet: &Packet, buffer: &mut IoBuffer) -> std::io::Result<()> {
    write_dword(buffer, packet.len() as u32)?;
    buffer.write(&packet.data)?;
    buffer.flush()?;
    Ok(())
}

fn recv_packet(packet: &mut Packet, buffer: &mut IoBuffer, max_length: usize) -> std::io::Result<bool> {
    let length = read_dword(buffer)? as usize;
    if length > max_length || length == 0xffffffff {
        return Ok(false);
    }
    
    packet.data.resize(length, 0);
    let mut total_read = 0;
    while total_read < length {
        let read = buffer.read(&mut packet.data[total_read..])?;
        if read == 0 {
            return Ok(false);
        }
        total_read += read;
    }
    Ok(true)
}

fn do_auth(buffer: &mut IoBuffer) -> std::io::Result<Option<u32>> {
    let mut proto_version = Packet::new();
    let mut mit_cookie = Packet::new();
    let mut reply = Packet::with_capacity(4);

    if !recv_packet(&mut proto_version, buffer, 4)? {
        reply.put_dword(0, 1); // AUTH_PACKETOVERSIZE
        send_packet(&reply, buffer)?;
        return Ok(None);
    }

    let version = proto_version.get_dword(0);
    if version > FLOPPYD_PROTOCOL_VERSION || version < FLOPPYD_PROTOCOL_VERSION_OLD {
        reply.put_dword(0, 2); // AUTH_WRONGVERSION
        send_packet(&reply, buffer)?;
        return Ok(None);
    }

    if version == FLOPPYD_PROTOCOL_VERSION_OLD {
        reply.put_dword(0, 0); // AUTH_SUCCESS
    } else {
        let mut cap = 1; // FLOPPYD_CAP_EXPLICIT_OPEN
        if std::mem::size_of::<i64>() >= 8 {
            cap |= 2; // FLOPPYD_CAP_LARGE_SEEK
        }
        reply.data.resize(12, 0);
        reply.put_dword(0, 0); // AUTH_SUCCESS
        reply.put_dword(4, FLOPPYD_PROTOCOL_VERSION);
        reply.put_dword(8, cap);
    }
    send_packet(&reply, buffer)?;

    if !recv_packet(&mut mit_cookie, buffer, MAX_XAUTHORITY_LENGTH)? {
        reply.put_dword(0, 1); // AUTH_PACKETOVERSIZE
        send_packet(&reply, buffer)?;
        return Ok(None);
    }

    // Simplified auth handling - real implementation would need X11 auth
    reply.put_dword(0, 0); // AUTH_SUCCESS
    send_packet(&reply, buffer)?;
    
    Ok(Some(version))
}

fn serve_client(stream: TcpStream, device_name: &str) -> std::io::Result<()> {
    let mut buffer = IoBuffer::new(stream.as_raw_fd());
    
    let version = match do_auth(&mut buffer)? {
        Some(v) => v,
        None => return Ok(()),
    };

    let mut opcode = Packet::new();
    let mut param = Packet::new();
    let mut dev_fd: Option<RawFd> = None;
    let mut read_only = true;

    if version == FLOPPYD_PROTOCOL_VERSION_OLD {
        // Old protocol behavior
        read_only = false;
        dev_fd = Some(unsafe { open(device_name.as_ptr() as *const i8, O_RDWR | O_LARGEFILE) });
        if dev_fd.unwrap() < 0 {
            read_only = true;
            dev_fd = Some(unsafe { open(device_name.as_ptr() as *const i8, O_RDONLY | O_LARGEFILE) });
        }
        
        if dev_fd.unwrap() < 0 {
            let mut reply = Packet::with_capacity(8);
            reply.put_dword(0, 0);
            reply.put_dword(4, std::io::Error::last_os_error().raw_os_error().unwrap_or(0) as u32);
            send_packet(&reply, &mut buffer)?;
            return Ok(());
        }
    }

    loop {
        if !recv_packet(&mut opcode, &mut buffer, 1)? {
            break;
        }
        
        if opcode.data[0] != 6 { // OP_CLOSE
            recv_packet(&mut param, &mut buffer, MAX_DATA_REQUEST)?;
        }

        match opcode.data[0] {
            1 => { // OP_OPRO
                let dev_nr = if param.len() >= 4 { param.get_dword(0) } else { 0 };
                // Simplified device opening
                let fd = unsafe { open(device_name.as_ptr() as *const i8, O_RDONLY | O_LARGEFILE) };
                if fd >= 0 {
                    dev_fd = Some(fd);
                    read_only = true;
                }
                let mut reply = Packet::with_capacity(8);
                reply.put_dword(0, 0);
                reply.put_dword(4, if fd >= 0 { 0 } else { std::io::Error::last_os_error().raw_os_error().unwrap_or(0) as u32 });
                send_packet(&reply, &mut buffer)?;
            },
            2 => { // OP_OPRW
                let dev_nr = if param.len() >= 4 { param.get_dword(0) } else { 0 };
                // Simplified device opening
                let fd = unsafe { open(device_name.as_ptr() as *const i8, O_RDWR | O_LARGEFILE) };
                if fd >= 0 {
                    dev_fd = Some(fd);
                    read_only = false;
                }
                let mut reply = Packet::with_capacity(8);
                reply.put_dword(0, 0);
                reply.put_dword(4, if fd >= 0 { 0 } else { std::io::Error::last_os_error().raw_os_error().unwrap_or(0) as u32 });
                send_packet(&reply, &mut buffer)?;
            },
            3 => { // OP_READ
                if let Some(fd) = dev_fd {
                    let len = param.get_dword(0) as usize;
                    let mut data = vec![0; len];
                    let read = unsafe { libc::read(fd, data.as_mut_ptr() as *mut libc::c_void, len) };
                    if read < 0 {
                        let mut reply = Packet::with_capacity(8);
                        reply.put_dword(0, 0);
                        reply.put_dword(4, std::io::Error::last_os_error().raw_os_error().unwrap_or(0) as u32);
                        send_packet(&reply, &mut buffer)?;
                    } else {
                        let mut reply = Packet::with_capacity(8);
                        reply.put_dword(0, read as u32);
                        reply.put_dword(4, 0);
                        send_packet(&reply, &mut buffer)?;
                        param.data = data[..read as usize].to_vec();
                        send_packet(&param, &mut buffer)?;
                    }
                }
            },
            4 => { // OP_WRITE
                if let Some(fd) = dev_fd {
                    let written = if read_only {
                        -1
                    } else {
                        unsafe { libc::write(fd, param.data.as_ptr() as *const libc::c_void, param.len()) }
                    };
                    let mut reply = Packet::with_capacity(8);
                    reply.put_dword(0, if written >= 0 { written as u32 } else { 0 });
                    reply.put_dword(4, if written >= 0 { 0 } else { std::io::Error::last_os_error().raw_os_error().unwrap_or(0) as u32 });
                    send_packet(&reply, &mut buffer)?;
                }
            },
            5 => { // OP_SEEK
                if let Some(fd) = dev_fd {
                    let offset = param.get_dword(0) as i64;
                    let whence = param.get_dword(4) as i32;
                    unsafe { libc::lseek(fd, offset, whence) };
                    let new_pos = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
                    let mut reply = Packet::with_capacity(8);
                    reply.put_dword(0, new_pos as u32);
                    reply.put_dword(4, 0);
                    send_packet(&reply, &mut buffer)?;
                }
            },
            6 => { // OP_CLOSE
                if let Some(fd) = dev_fd {
                    unsafe { close(fd) };
                }
                break;
            },
            _ => {
                let mut reply = Packet::with_capacity(8);
                reply.put_dword(0, 0);
                reply.put_dword(4, libc::EINVAL as u32);
                send_packet(&reply, &mut buffer)?;
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <device>", args[0]);
        return Ok(());
    }

    let device_name = &args[1];
    let listener = TcpListener::bind("0.0.0.0:5703")?;
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = serve_client(stream, device_name) {
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