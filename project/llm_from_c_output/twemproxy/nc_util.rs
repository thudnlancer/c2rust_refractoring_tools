//! Utilities for socket operations and memory management in Rust.

use std::ffi::{CStr, CString};
use std::io::{Error, ErrorKind, Result};
use std::mem;
use std::net::{SocketAddr, ToSocketAddrs};
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

use libc::{
    self, c_int, c_void, sockaddr, sockaddr_in, sockaddr_in6, sockaddr_un, socklen_t, AF_INET,
    AF_INET6, AF_UNIX, F_GETFL, F_SETFL, IPPROTO_TCP, NI_MAXHOST, NI_MAXSERV, O_NONBLOCK,
    SOL_SOCKET, SO_ERROR, SO_KEEPALIVE, SO_LINGER, SO_RCVBUF, SO_REUSEADDR, SO_REUSEPORT,
    SO_SNDBUF, TCP_NODELAY,
};

pub const LF: u8 = 10;
pub const CR: u8 = 13;
pub const CRLF: &str = "\x0d\x0a";
pub const CRLF_LEN: usize = 2;

pub const NC_INET4_ADDRSTRLEN: usize = 15;
pub const NC_INET6_ADDRSTRLEN: usize = 45;
pub const NC_INET_ADDRSTRLEN: usize = NC_INET6_ADDRSTRLEN;
pub const NC_UNIX_ADDRSTRLEN: usize = mem::size_of::<sockaddr_un>() - mem::size_of::<c_int>();

pub const NC_MAXHOSTNAMELEN: usize = 256;

pub const NC_UINT8_MAXLEN: usize = 4;
pub const NC_UINT16_MAXLEN: usize = 6;
pub const NC_UINT32_MAXLEN: usize = 11;
pub const NC_UINT64_MAXLEN: usize = 21;
pub const NC_UINTMAX_MAXLEN: usize = NC_UINT64_MAXLEN;

pub fn nelems<T>(arr: &[T]) -> usize {
    arr.len()
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

pub fn square(d: f64) -> f64 {
    d * d
}

pub fn variance(s: f64, s2: f64, n: usize) -> f64 {
    if n < 2 {
        0.0
    } else {
        (s2 - square(s) / (n as f64)) / ((n - 1) as f64)
    }
}

pub fn stddev(s: f64, s2: f64, n: usize) -> f64 {
    if n < 2 {
        0.0
    } else {
        variance(s, s2, n).sqrt()
    }
}

pub fn align(d: usize, n: usize) -> usize {
    (d + (n - 1)) & !(n - 1)
}

pub fn align_ptr<T>(p: *mut T, n: usize) -> *mut T {
    ((p as usize + (n - 1)) & !(n - 1)) as *mut T
}

pub fn set_blocking(sd: c_int) -> Result<()> {
    let flags = unsafe { libc::fcntl(sd, F_GETFL) };
    if flags == -1 {
        return Err(Error::last_os_error());
    }
    let res = unsafe { libc::fcntl(sd, F_SETFL, flags & !O_NONBLOCK) };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn set_nonblocking(sd: c_int) -> Result<()> {
    let flags = unsafe { libc::fcntl(sd, F_GETFL) };
    if flags == -1 {
        return Err(Error::last_os_error());
    }
    let res = unsafe { libc::fcntl(sd, F_SETFL, flags | O_NONBLOCK) };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn set_reuseaddr(sd: c_int) -> Result<()> {
    let reuse: c_int = 1;
    let res = unsafe {
        libc::setsockopt(
            sd,
            SOL_SOCKET,
            SO_REUSEADDR,
            &reuse as *const _ as *const c_void,
            mem::size_of_val(&reuse) as socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn set_reuseport(sd: c_int) -> Result<()> {
    let reuse: c_int = 1;
    let res = unsafe {
        libc::setsockopt(
            sd,
            SOL_SOCKET,
            SO_REUSEPORT,
            &reuse as *const _ as *const c_void,
            mem::size_of_val(&reuse) as socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn set_tcpnodelay(sd: c_int) -> Result<()> {
    let nodelay: c_int = 1;
    let res = unsafe {
        libc::setsockopt(
            sd,
            IPPROTO_TCP,
            TCP_NODELAY,
            &nodelay as *const _ as *const c_void,
            mem::size_of_val(&nodelay) as socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn set_linger(sd: c_int, timeout: c_int) -> Result<()> {
    let linger = libc::linger {
        l_onoff: 1,
        l_linger: timeout,
    };
    let res = unsafe {
        libc::setsockopt(
            sd,
            SOL_SOCKET,
            SO_LINGER,
            &linger as *const _ as *const c_void,
            mem::size_of_val(&linger) as socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn set_tcpkeepalive(sd: c_int) -> Result<()> {
    let val: c_int = 1;
    let res = unsafe {
        libc::setsockopt(
            sd,
            SOL_SOCKET,
            SO_KEEPALIVE,
            &val as *const _ as *const c_void,
            mem::size_of_val(&val) as socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn set_sndbuf(sd: c_int, size: c_int) -> Result<()> {
    let res = unsafe {
        libc::setsockopt(
            sd,
            SOL_SOCKET,
            SO_SNDBUF,
            &size as *const _ as *const c_void,
            mem::size_of_val(&size) as socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn set_rcvbuf(sd: c_int, size: c_int) -> Result<()> {
    let res = unsafe {
        libc::setsockopt(
            sd,
            SOL_SOCKET,
            SO_RCVBUF,
            &size as *const _ as *const c_void,
            mem::size_of_val(&size) as socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn get_soerror(sd: c_int) -> Result<c_int> {
    let mut err: c_int = 0;
    let mut len = mem::size_of_val(&err) as socklen_t;
    let res = unsafe {
        libc::getsockopt(
            sd,
            SOL_SOCKET,
            SO_ERROR,
            &mut err as *mut _ as *mut c_void,
            &mut len,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(err)
    }
}

pub fn get_sndbuf(sd: c_int) -> Result<c_int> {
    let mut size: c_int = 0;
    let mut len = mem::size_of_val(&size) as socklen_t;
    let res = unsafe {
        libc::getsockopt(
            sd,
            SOL_SOCKET,
            SO_SNDBUF,
            &mut size as *mut _ as *mut c_void,
            &mut len,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(size)
    }
}

pub fn get_rcvbuf(sd: c_int) -> Result<c_int> {
    let mut size: c_int = 0;
    let mut len = mem::size_of_val(&size) as socklen_t;
    let res = unsafe {
        libc::getsockopt(
            sd,
            SOL_SOCKET,
            SO_RCVBUF,
            &mut size as *mut _ as *mut c_void,
            &mut len,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(size)
    }
}

pub fn nc_atoi(line: &[u8]) -> Result<i32> {
    if line.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "empty input"));
    }

    let mut value = 0;
    for &c in line {
        if !c.is_ascii_digit() {
            return Err(Error::new(ErrorKind::InvalidInput, "non-digit character"));
        }
        value = value * 10 + (c - b'0') as i32;
    }

    if value < 0 {
        Err(Error::new(ErrorKind::InvalidInput, "negative value"))
    } else {
        Ok(value)
    }
}

pub fn valid_port(n: i32) -> bool {
    n >= 1 && n <= u16::MAX as i32
}

pub fn usec_now() -> Result<i64> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => Ok(d.as_secs() as i64 * 1_000_000 + d.subsec_micros() as i64),
        Err(e) => Err(Error::new(ErrorKind::Other, e)),
    }
}

pub fn msec_now() -> Result<i64> {
    usec_now().map(|us| us / 1000)
}

pub fn sendn(sd: c_int, buf: &[u8]) -> Result<usize> {
    let mut sent = 0;
    while sent < buf.len() {
        let res = unsafe {
            libc::send(
                sd,
                buf[sent..].as_ptr() as *const c_void,
                buf.len() - sent,
                0,
            )
        };
        if res == -1 {
            let err = Error::last_os_error();
            if err.kind() == ErrorKind::Interrupted {
                continue;
            }
            return Err(err);
        }
        sent += res as usize;
    }
    Ok(sent)
}

pub fn recvn(sd: c_int, buf: &mut [u8]) -> Result<usize> {
    let mut received = 0;
    while received < buf.len() {
        let res = unsafe {
            libc::recv(
                sd,
                buf[received..].as_mut_ptr() as *mut c_void,
                buf.len() - received,
                0,
            )
        };
        if res == -1 {
            let err = Error::last_os_error();
            if err.kind() == ErrorKind::Interrupted {
                continue;
            }
            return Err(err);
        } else if res == 0 {
            break;
        }
        received += res as usize;
    }
    Ok(received)
}

pub struct SockInfo {
    pub family: c_int,
    pub addrlen: socklen_t,
    pub addr: sockaddr_storage,
}

impl SockInfo {
    pub fn new() -> Self {
        Self {
            family: 0,
            addrlen: 0,
            addr: unsafe { mem::zeroed() },
        }
    }
}

pub fn resolve(name: Option<&str>, port: u16) -> Result<SockInfo> {
    if let Some(name) = name {
        if name.starts_with('/') {
            return resolve_unix(name);
        }
    }
    resolve_inet(name, port)
}

fn resolve_inet(name: Option<&str>, port: u16) -> Result<SockInfo> {
    let mut hints = unsafe { mem::zeroed::<libc::addrinfo>() };
    hints.ai_socktype = libc::SOCK_STREAM;
    hints.ai_protocol = libc::IPPROTO_TCP;

    let node = name.map(|s| CString::new(s).unwrap());
    let service = CString::new(port.to_string()).unwrap();

    let mut res = ptr::null_mut();
    let status = unsafe {
        libc::getaddrinfo(
            node.map(|n| n.as_ptr()).unwrap_or(ptr::null()),
            service.as_ptr(),
            &hints,
            &mut res,
        )
    };
    if status != 0 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("getaddrinfo failed: {}", status),
        ));
    }

    let mut si = SockInfo::new();
    unsafe {
        si.family = (*res).ai_family;
        si.addrlen = (*res).ai_addrlen;
        ptr::copy_nonoverlapping(
            (*res).ai_addr,
            &mut si.addr as *mut _ as *mut libc::sockaddr,
            (*res).ai_addrlen as usize,
        );
        libc::freeaddrinfo(res);
    }

    Ok(si)
}

fn resolve_unix(path: &str) -> Result<SockInfo> {
    if path.len() >= NC_UNIX_ADDRSTRLEN {
        return Err(Error::new(ErrorKind::InvalidInput, "path too long"));
    }

    let mut si = SockInfo::new();
    si.family = AF_UNIX;
    si.addrlen = mem::size_of::<sockaddr_un>() as socklen_t;

    unsafe {
        let un = &mut si.addr as *mut _ as *mut sockaddr_un;
        (*un).sun_family = AF_UNIX as _;
        ptr::copy_nonoverlapping(
            path.as_ptr(),
            (*un).sun_path.as_mut_ptr(),
            path.len(),
        );
    }

    Ok(si)
}

pub fn unresolve_addr(addr: &sockaddr_storage, addrlen: socklen_t) -> String {
    let mut host = [0; NI_MAXHOST];
    let mut service = [0; NI_MAXSERV];

    let status = unsafe {
        libc::getnameinfo(
            addr as *const _ as *const sockaddr,
            addrlen,
            host.as_mut_ptr(),
            host.len() as socklen_t,
            service.as_mut_ptr(),
            service.len() as socklen_t,
            libc::NI_NUMERICHOST | libc::NI_NUMERICSERV,
        )
    };

    if status != 0 {
        "unknown".to_string()
    } else {
        unsafe {
            format!(
                "{}:{}",
                CStr::from_ptr(host.as_ptr()).to_string_lossy(),
                CStr::from_ptr(service.as_ptr()).to_string_lossy()
            )
        }
    }
}

pub fn unresolve_peer_desc(sd: c_int) -> String {
    let mut si = SockInfo::new();
    let addrlen = mem::size_of_val(&si.addr) as socklen_t;

    let status = unsafe {
        libc::getpeername(
            sd,
            &mut si.addr as *mut _ as *mut sockaddr,
            &mut si.addrlen,
        )
    };

    if status != 0 {
        "unknown".to_string()
    } else {
        unresolve_addr(&si.addr, si.addrlen)
    }
}

pub fn unresolve_desc(sd: c_int) -> String {
    let mut si = SockInfo::new();
    let addrlen = mem::size_of_val(&si.addr) as socklen_t;

    let status = unsafe {
        libc::getsockname(
            sd,
            &mut si.addr as *mut _ as *mut sockaddr,
            &mut si.addrlen,
        )
    };

    if status != 0 {
        "unknown".to_string()
    } else {
        unresolve_addr(&si.addr, si.addrlen)
    }
}