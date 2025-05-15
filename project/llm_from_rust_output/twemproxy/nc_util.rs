use std::ffi::{CStr, CString};
use std::io::{Error, ErrorKind};
use std::mem;
use std::net::{SocketAddr, ToSocketAddrs};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::RawFd;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

const SOCK_STREAM: i32 = 1;
const IPPROTO_TCP: i32 = 6;

pub fn nc_set_blocking(sd: RawFd) -> Result<(), Error> {
    let flags = libc::fcntl(sd, libc::F_GETFL);
    if flags == -1 {
        return Err(Error::last_os_error());
    }
    let res = libc::fcntl(sd, libc::F_SETFL, flags & !libc::O_NONBLOCK);
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn nc_set_nonblocking(sd: RawFd) -> Result<(), Error> {
    let flags = libc::fcntl(sd, libc::F_GETFL);
    if flags == -1 {
        return Err(Error::last_os_error());
    }
    let res = libc::fcntl(sd, libc::F_SETFL, flags | libc::O_NONBLOCK);
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn nc_set_reuseaddr(sd: RawFd) -> Result<(), Error> {
    let reuse = 1;
    let res = unsafe {
        libc::setsockopt(
            sd,
            libc::SOL_SOCKET,
            libc::SO_REUSEADDR,
            &reuse as *const _ as *const libc::c_void,
            mem::size_of_val(&reuse) as libc::socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn nc_set_reuseport(sd: RawFd) -> Result<(), Error> {
    let reuse = 1;
    let res = unsafe {
        libc::setsockopt(
            sd,
            libc::SOL_SOCKET,
            libc::SO_REUSEPORT,
            &reuse as *const _ as *const libc::c_void,
            mem::size_of_val(&reuse) as libc::socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn nc_set_tcpnodelay(sd: RawFd) -> Result<(), Error> {
    let nodelay = 1;
    let res = unsafe {
        libc::setsockopt(
            sd,
            IPPROTO_TCP,
            libc::TCP_NODELAY,
            &nodelay as *const _ as *const libc::c_void,
            mem::size_of_val(&nodelay) as libc::socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn nc_set_linger(sd: RawFd, timeout: i32) -> Result<(), Error> {
    let linger = libc::linger {
        l_onoff: 1,
        l_linger: timeout,
    };
    let res = unsafe {
        libc::setsockopt(
            sd,
            libc::SOL_SOCKET,
            libc::SO_LINGER,
            &linger as *const _ as *const libc::c_void,
            mem::size_of_val(&linger) as libc::socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn nc_set_tcpkeepalive(sd: RawFd) -> Result<(), Error> {
    let val = 1;
    let res = unsafe {
        libc::setsockopt(
            sd,
            libc::SOL_SOCKET,
            libc::SO_KEEPALIVE,
            &val as *const _ as *const libc::c_void,
            mem::size_of_val(&val) as libc::socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn nc_set_sndbuf(sd: RawFd, size: i32) -> Result<(), Error> {
    let res = unsafe {
        libc::setsockopt(
            sd,
            libc::SOL_SOCKET,
            libc::SO_SNDBUF,
            &size as *const _ as *const libc::c_void,
            mem::size_of_val(&size) as libc::socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn nc_set_rcvbuf(sd: RawFd, size: i32) -> Result<(), Error> {
    let res = unsafe {
        libc::setsockopt(
            sd,
            libc::SOL_SOCKET,
            libc::SO_RCVBUF,
            &size as *const _ as *const libc::c_void,
            mem::size_of_val(&size) as libc::socklen_t,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn nc_get_soerror(sd: RawFd) -> Result<i32, Error> {
    let mut err = 0;
    let mut len = mem::size_of_val(&err) as libc::socklen_t;
    let res = unsafe {
        libc::getsockopt(
            sd,
            libc::SOL_SOCKET,
            libc::SO_ERROR,
            &mut err as *mut _ as *mut libc::c_void,
            &mut len,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(err)
    }
}

pub fn nc_get_sndbuf(sd: RawFd) -> Result<i32, Error> {
    let mut size = 0;
    let mut len = mem::size_of_val(&size) as libc::socklen_t;
    let res = unsafe {
        libc::getsockopt(
            sd,
            libc::SOL_SOCKET,
            libc::SO_SNDBUF,
            &mut size as *mut _ as *mut libc::c_void,
            &mut len,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(size)
    }
}

pub fn nc_get_rcvbuf(sd: RawFd) -> Result<i32, Error> {
    let mut size = 0;
    let mut len = mem::size_of_val(&size) as libc::socklen_t;
    let res = unsafe {
        libc::getsockopt(
            sd,
            libc::SOL_SOCKET,
            libc::SO_RCVBUF,
            &mut size as *mut _ as *mut libc::c_void,
            &mut len,
        )
    };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(size)
    }
}

pub fn nc_valid_port(n: i32) -> bool {
    n > 0 && n <= 65535
}

pub fn nc_usec_now() -> Result<i64, Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::new(ErrorKind::Other, "SystemTime before UNIX EPOCH"))?;
    Ok(now.as_secs() as i64 * 1_000_000 + i64::from(now.subsec_micros()))
}

pub fn nc_msec_now() -> Result<i64, Error> {
    Ok(nc_usec_now()? / 1000)
}

pub fn nc_resolve(addr: &str, port: u16) -> Result<SocketAddr, Error> {
    let addr_str = format!("{}:{}", addr, port);
    addr_str.to_socket_addrs()?
        .next()
        .ok_or_else(|| Error::new(ErrorKind::Other, "could not resolve address"))
}

pub fn nc_unresolve_addr(addr: &SocketAddr) -> String {
    addr.to_string()
}

pub fn nc_unresolve_peer_desc(sd: RawFd) -> Result<String, Error> {
    let mut addr: libc::sockaddr_storage = unsafe { mem::zeroed() };
    let mut len = mem::size_of_val(&addr) as libc::socklen_t;
    
    let res = unsafe {
        libc::getpeername(
            sd,
            &mut addr as *mut _ as *mut libc::sockaddr,
            &mut len,
        )
    };
    
    if res == -1 {
        return Err(Error::last_os_error());
    }
    
    let addr = unsafe {
        match addr.ss_family as i32 {
            libc::AF_INET => {
                let addr: &libc::sockaddr_in = &*(&addr as *const _ as *const _);
                SocketAddr::from((addr.sin_addr, addr.sin_port))
            }
            libc::AF_INET6 => {
                let addr: &libc::sockaddr_in6 = &*(&addr as *const _ as *const _);
                SocketAddr::from((addr.sin6_addr, addr.sin6_port))
            }
            _ => return Err(Error::new(ErrorKind::Other, "unsupported address family")),
        }
    };
    
    Ok(addr.to_string())
}

pub fn nc_unresolve_desc(sd: RawFd) -> Result<String, Error> {
    let mut addr: libc::sockaddr_storage = unsafe { mem::zeroed() };
    let mut len = mem::size_of_val(&addr) as libc::socklen_t;
    
    let res = unsafe {
        libc::getsockname(
            sd,
            &mut addr as *mut _ as *mut libc::sockaddr,
            &mut len,
        )
    };
    
    if res == -1 {
        return Err(Error::last_os_error());
    }
    
    let addr = unsafe {
        match addr.ss_family as i32 {
            libc::AF_INET => {
                let addr: &libc::sockaddr_in = &*(&addr as *const _ as *const _);
                SocketAddr::from((addr.sin_addr, addr.sin_port))
            }
            libc::AF_INET6 => {
                let addr: &libc::sockaddr_in6 = &*(&addr as *const _ as *const _);
                SocketAddr::from((addr.sin6_addr, addr.sin6_port))
            }
            _ => return Err(Error::new(ErrorKind::Other, "unsupported address family")),
        }
    };
    
    Ok(addr.to_string())
}