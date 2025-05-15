use std::ffi::{CStr, CString};
use std::io::{Read, Write};
use std::mem;
use std::net::{TcpStream, ToSocketAddrs};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::ptr;
use std::time::Duration;

use libc::{
    c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void, close, connect, free, gethostbyname,
    hostent, in_addr, in_addr_t, inet_addr, setsockopt, shutdown, sockaddr, sockaddr_in, socklen_t,
    socket, strerror, AF_INET, IPPROTO_TCP, SOCK_STREAM, TCP_CORK,
};

const AUTH_SUCCESS: u32 = 0;
const AUTH_AUTHFAILED: u32 = 2;
const AUTH_WRONGVERSION: u32 = 3;
const AUTH_IO_ERROR: u32 = 6;

const OP_READ: u8 = 0;
const OP_WRITE: u8 = 1;
const OP_SEEK: u8 = 2;
const OP_FLUSH: u8 = 3;
const OP_CLOSE: u8 = 4;
const OP_OPRO: u8 = 6;
const OP_OPRW: u8 = 7;
const OP_SEEK64: u8 = 8;

type Dword = u32;
type Byte = u8;
type Qword = u64;
type MtOffT = i64;

struct RemoteFile {
    fd: RawFd,
    offset: MtOffT,
    lastwhere: MtOffT,
    size: MtOffT,
    version: u32,
    capabilities: u32,
    drive: c_int,
}

impl RemoteFile {
    fn new() -> Self {
        RemoteFile {
            fd: -1,
            offset: 0,
            lastwhere: 0,
            size: 0,
            version: 11,
            capabilities: 0,
            drive: 0,
        }
    }

    fn authenticate(&mut self, sock: RawFd, display: &str) -> Result<(), String> {
        // Authentication logic here
        Ok(())
    }

    fn connect(&mut self, name: &str) -> Result<(), String> {
        // Connection logic here
        Ok(())
    }

    fn read(&mut self, buf: &mut [u8], len: usize) -> Result<usize, String> {
        // Read logic here
        Ok(0)
    }

    fn write(&mut self, buf: &[u8], len: usize) -> Result<usize, String> {
        // Write logic here
        Ok(0)
    }

    fn seek(&mut self, offset: MtOffT, whence: c_int) -> Result<MtOffT, String> {
        // Seek logic here
        Ok(0)
    }

    fn flush(&mut self) -> Result<(), String> {
        // Flush logic here
        Ok(())
    }

    fn close(&mut self) -> Result<(), String> {
        // Close logic here
        Ok(())
    }
}

impl Drop for RemoteFile {
    fn drop(&mut self) {
        if self.fd != -1 {
            unsafe { close(self.fd) };
        }
    }
}

fn floppyd_open(
    dev: *mut c_void,
    name: *const c_char,
    mode: c_int,
    errmsg: *mut c_char,
    max_size: *mut MtOffT,
) -> *mut c_void {
    unsafe {
        let name_str = CStr::from_ptr(name).to_string_lossy();
        let mut file = RemoteFile::new();

        match file.connect(&name_str) {
            Ok(_) => {
                if !max_size.is_null() {
                    *max_size = if file.capabilities & 2 != 0 {
                        i64::MAX
                    } else {
                        i32::MAX as i64
                    };
                }
                Box::into_raw(Box::new(file)) as *mut c_void
            }
            Err(e) => {
                let err_str = CString::new(e).unwrap();
                libc::strcpy(errmsg, err_str.as_ptr());
                ptr::null_mut()
            }
        }
    }
}

// Helper functions
fn byte2dword(val: &[Byte]) -> Dword {
    ((val[0] as Dword) << 24)
        | ((val[1] as Dword) << 16)
        | ((val[2] as Dword) << 8)
        | (val[3] as Dword)
}

fn dword2byte(parm: Dword) -> [Byte; 4] {
    [
        ((parm >> 24) & 0xff) as Byte,
        ((parm >> 16) & 0xff) as Byte,
        ((parm >> 8) & 0xff) as Byte,
        (parm & 0xff) as Byte,
    ]
}

fn connect_to_server(ip: in_addr_t, port: u16) -> Result<RawFd, String> {
    unsafe {
        let sock = socket(AF_INET, SOCK_STREAM, 0);
        if sock < 0 {
            return Err("socket failed".to_string());
        }

        let mut addr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: port.to_be(),
            sin_addr: in_addr { s_addr: ip },
            sin_zero: [0; 8],
        };

        if connect(
            sock,
            &addr as *const sockaddr_in as *const sockaddr,
            mem::size_of::<sockaddr_in>() as socklen_t,
        ) < 0
        {
            close(sock);
            return Err("connect failed".to_string());
        }

        let on: c_int = 1;
        setsockopt(
            sock,
            IPPROTO_TCP,
            TCP_CORK,
            &on as *const c_int as *const c_void,
            mem::size_of::<c_int>() as socklen_t,
        );

        Ok(sock)
    }
}