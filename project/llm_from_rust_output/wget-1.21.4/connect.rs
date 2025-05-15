use std::ffi::{CStr, CString};
use std::mem;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use std::time::{Duration, Instant};
use std::ptr;
use libc::{c_int, c_void, c_char, c_double, c_ulong, socklen_t, sa_family_t, in_addr, in6_addr};
use libc::{socket, connect, bind, getsockname, getpeername, recv, setsockopt, listen, accept};
use libc::{close, read, write, select, fd_set, timeval, IPPROTO_IPV6, IPPROTO_TCP, SO_REUSEADDR};
use libc::{SO_RCVBUF, SOCK_STREAM, AF_INET, AF_INET6, EINTR, EAGAIN, EWOULDBLOCK, ETIMEDOUT};
use libc::{MSG_PEEK, SOL_SOCKET};

const WAIT_FOR_READ: c_int = 1;
const WAIT_FOR_WRITE: c_int = 2;

struct TransportImplementation {
    reader: Option<fn(RawFd, *mut c_char, c_int, *mut c_void, c_double) -> c_int>,
    writer: Option<fn(RawFd, *mut c_char, c_int, *mut c_void) -> c_int>,
    poller: Option<fn(RawFd, c_double, c_int, *mut c_void) -> c_int>,
    peeker: Option<fn(RawFd, *mut c_char, c_int, *mut c_void, c_double) -> c_int>,
    errstr: Option<fn(RawFd, *mut c_void) -> *const c_char>,
    closer: Option<fn(RawFd, *mut c_void)>,
}

struct TransportInfo {
    imp: *mut TransportImplementation,
    ctx: *mut c_void,
}

struct Options {
    ipv6_only: bool,
    bind_address: Option<String>,
    connect_timeout: c_double,
    read_timeout: c_double,
    limit_rate: i64,
    debug: bool,
}

static mut OPT: Options = Options {
    ipv6_only: false,
    bind_address: None,
    connect_timeout: 0.0,
    read_timeout: 0.0,
    limit_rate: 0,
    debug: false,
};

fn socket_ip_address(fd: RawFd, endpoint: c_int) -> Option<SocketAddr> {
    unsafe {
        let mut storage: libc::sockaddr_storage = mem::zeroed();
        let mut addrlen = mem::size_of_val(&storage) as socklen_t;
        let sockaddr = &mut storage as *mut _ as *mut libc::sockaddr;

        let ret = match endpoint {
            0 => getsockname(fd, sockaddr, &mut addrlen),
            1 => getpeername(fd, sockaddr, &mut addrlen),
            _ => return None,
        };

        if ret < 0 {
            return None;
        }

        match storage.ss_family as c_int {
            AF_INET => {
                let sa = &storage as *const _ as *const libc::sockaddr_in;
                let ip = Ipv4Addr::from(u32::from_be(unsafe { (*sa).sin_addr.s_addr }));
                let port = unsafe { (*sa).sin_port }.to_be();
                Some(SocketAddr::new(IpAddr::V4(ip), port))
            }
            AF_INET6 => {
                let sa = &storage as *const _ as *const libc::sockaddr_in6;
                let ip = Ipv6Addr::from(unsafe { (*sa).sin6_addr.s6_addr });
                let port = unsafe { (*sa).sin6_port }.to_be();
                let scope_id = unsafe { (*sa).sin6_scope_id };
                Some(SocketAddr::new(IpAddr::V6(ip), port))
            }
            _ => None,
        }
    }
}

fn connect_to_host(host: &str, port: u16) -> Result<TcpStream, String> {
    let addr = format!("{}:{}", host, port);
    match TcpStream::connect(&addr) {
        Ok(stream) => Ok(stream),
        Err(e) => Err(format!("Failed to connect to {}: {}", addr, e)),
    }
}

fn bind_local(addr: IpAddr, port: u16) -> Result<TcpStream, String> {
    let socket_addr = SocketAddr::new(addr, port);
    match TcpStream::bind(&socket_addr) {
        Ok(stream) => Ok(stream),
        Err(e) => Err(format!("Failed to bind to {}: {}", socket_addr, e)),
    }
}

fn select_fd(fd: RawFd, timeout: Duration, wait_for: c_int) -> bool {
    unsafe {
        let mut read_fds = mem::zeroed::<fd_set>();
        let mut write_fds = mem::zeroed::<fd_set>();
        let mut except_fds = mem::zeroed::<fd_set>();
        let mut timeout = timeval {
            tv_sec: timeout.as_secs() as _,
            tv_usec: timeout.subsec_micros() as _,
        };

        libc::FD_SET(fd, &mut read_fds);
        libc::FD_SET(fd, &mut write_fds);

        let nfds = fd + 1;
        let read_fds_ptr = if wait_for & WAIT_FOR_READ != 0 {
            &mut read_fds
        } else {
            ptr::null_mut()
        };
        let write_fds_ptr = if wait_for & WAIT_FOR_WRITE != 0 {
            &mut write_fds
        } else {
            ptr::null_mut()
        };

        let ret = select(
            nfds,
            read_fds_ptr,
            write_fds_ptr,
            ptr::null_mut(),
            &mut timeout,
        );

        ret > 0
    }
}

fn fd_read(fd: RawFd, buf: &mut [u8], timeout: Duration) -> Result<usize, String> {
    if !select_fd(fd, timeout, WAIT_FOR_READ) {
        return Err("Timeout waiting for read".to_string());
    }

    unsafe {
        let ret = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len() as c_ulong);
        if ret < 0 {
            Err(format!("Read error: {}", std::io::Error::last_os_error()))
        } else {
            Ok(ret as usize)
        }
    }
}

fn fd_write(fd: RawFd, buf: &[u8], timeout: Duration) -> Result<usize, String> {
    if !select_fd(fd, timeout, WAIT_FOR_WRITE) {
        return Err("Timeout waiting for write".to_string());
    }

    unsafe {
        let ret = write(fd, buf.as_ptr() as *const c_void, buf.len() as c_ulong);
        if ret < 0 {
            Err(format!("Write error: {}", std::io::Error::last_os_error()))
        } else {
            Ok(ret as usize)
        }
    }
}

fn fd_close(fd: RawFd) {
    unsafe {
        close(fd);
    }
}