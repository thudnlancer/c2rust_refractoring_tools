use std::io::{self, Error, ErrorKind};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;
use std::mem;
use libc::{self, c_int, sockaddr, socklen_t};
use socket2::{Domain, Protocol, Socket, Type};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid timeout specified")]
    InvalidTimeout,
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Server closed the connection")]
    ConnectionClosed,
    #[error("Timeout occurred")]
    Timeout,
    #[error("Other error: {0}")]
    Other(String),
}

pub struct RedisContext {
    fd: Option<Socket>,
    flags: i32,
    connection_type: ConnectionType,
    connect_timeout: Option<Duration>,
    command_timeout: Option<Duration>,
    tcp: TcpInfo,
    unix_sock: UnixInfo,
    saddr: Option<Box<sockaddr>>,
    addrlen: socklen_t,
}

pub enum ConnectionType {
    Tcp,
    Unix,
}

pub struct TcpInfo {
    host: String,
    port: i32,
    source_addr: Option<String>,
}

pub struct UnixInfo {
    path: String,
}

impl RedisContext {
    pub fn new() -> Self {
        RedisContext {
            fd: None,
            flags: 0,
            connection_type: ConnectionType::Tcp,
            connect_timeout: None,
            command_timeout: None,
            tcp: TcpInfo {
                host: String::new(),
                port: 0,
                source_addr: None,
            },
            unix_sock: UnixInfo {
                path: String::new(),
            },
            saddr: None,
            addrlen: 0,
        }
    }

    pub fn net_close(&mut self) {
        self.fd = None;
    }

    pub fn net_read(&mut self, buf: &mut [u8]) -> Result<usize, RedisError> {
        match &self.fd {
            Some(socket) => {
                match socket.read(buf) {
                    Ok(nread) => {
                        if nread == 0 {
                            Err(RedisError::ConnectionClosed)
                        } else {
                            Ok(nread)
                        }
                    }
                    Err(e) => {
                        if e.kind() == ErrorKind::WouldBlock && (self.flags & REDIS_BLOCK) == 0 ||
                           e.kind() == ErrorKind::Interrupted {
                            Ok(0)
                        } else if e.kind() == ErrorKind::TimedOut && (self.flags & REDIS_BLOCK) != 0 {
                            Err(RedisError::Timeout)
                        } else {
                            Err(RedisError::Io(e))
                        }
                    }
                }
            }
            None => Err(RedisError::Other("Socket not initialized".to_string())),
        }
    }

    pub fn net_write(&mut self, buf: &[u8]) -> Result<usize, RedisError> {
        match &self.fd {
            Some(socket) => {
                match socket.write(buf) {
                    Ok(nwritten) => Ok(nwritten),
                    Err(e) => {
                        if (e.kind() == ErrorKind::WouldBlock && (self.flags & REDIS_BLOCK) == 0) ||
                           e.kind() == ErrorKind::Interrupted {
                            Ok(0)
                        } else {
                            Err(RedisError::Io(e))
                        }
                    }
                }
            }
            None => Err(RedisError::Other("Socket not initialized".to_string())),
        }
    }

    pub fn set_timeout(&mut self, timeout: Duration) -> Result<(), RedisError> {
        self.command_timeout = Some(timeout);
        if let Some(socket) = &self.fd {
            socket.set_read_timeout(Some(timeout))?;
            socket.set_write_timeout(Some(timeout))?;
        }
        Ok(())
    }

    pub fn connect_tcp(
        &mut self,
        addr: &str,
        port: i32,
        timeout: Option<Duration>,
        source_addr: Option<&str>,
    ) -> Result<(), RedisError> {
        self.connection_type = ConnectionType::Tcp;
        self.tcp.host = addr.to_string();
        self.tcp.port = port;
        self.connect_timeout = timeout;

        if let Some(addr) = source_addr {
            self.tcp.source_addr = Some(addr.to_string());
        }

        let socket = Socket::new(Domain::ipv4(), Type::stream(), Some(Protocol::tcp()))?;
        self.fd = Some(socket);

        if let Err(e) = self.set_blocking(false) {
            self.net_close();
            return Err(e);
        }

        let addrs = format!("{}:{}", addr, port);
        let mut socket_addrs = addrs.to_socket_addrs()?;

        if let Some(socket_addr) = socket_addrs.next() {
            if let Some(source) = &self.tcp.source_addr {
                let source_socket = source.to_socket_addrs()?.next().ok_or_else(|| {
                    RedisError::Other("Can't resolve source address".to_string())
                })?;
                self.fd.as_ref().unwrap().bind(&source_socket.into())?;
            }

            match self.fd.as_ref().unwrap().connect_timeout(&socket_addr.into(), timeout.unwrap_or(Duration::from_secs(0))) {
                Ok(()) => {
                    if (self.flags & REDIS_BLOCK) != 0 {
                        self.set_blocking(true)?;
                    }
                    self.flags |= REDIS_CONNECTED;
                    Ok(())
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => {
                    if self.wait_ready(timeout)? {
                        self.flags |= REDIS_CONNECTED;
                        Ok(())
                    } else {
                        Err(RedisError::Timeout)
                    }
                }
                Err(e) => Err(RedisError::Io(e)),
            }
        } else {
            Err(RedisError::Other("Can't resolve address".to_string()))
        }
    }

    pub fn connect_unix(&mut self, path: &str, timeout: Option<Duration>) -> Result<(), RedisError> {
        #[cfg(unix)]
        {
            self.connection_type = ConnectionType::Unix;
            self.unix_sock.path = path.to_string();
            self.connect_timeout = timeout;

            let socket = Socket::new(Domain::unix(), Type::stream(), None)?;
            self.fd = Some(socket);

            if let Err(e) = self.set_blocking(false) {
                self.net_close();
                return Err(e);
            }

            let addr = socket2::SockAddr::unix(Path::new(path))?;
            match self.fd.as_ref().unwrap().connect(&addr) {
                Ok(()) => {
                    if (self.flags & REDIS_BLOCK) != 0 {
                        self.set_blocking(true)?;
                    }
                    self.flags |= REDIS_CONNECTED;
                    Ok(())
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    if self.wait_ready(timeout)? {
                        self.flags |= REDIS_CONNECTED;
                        Ok(())
                    } else {
                        Err(RedisError::Timeout)
                    }
                }
                Err(e) => Err(RedisError::Io(e)),
            }
        }
        #[cfg(not(unix))]
        {
            Err(RedisError::Other("Unix sockets not supported on this platform".to_string()))
        }
    }

    fn set_blocking(&self, blocking: bool) -> Result<(), RedisError> {
        if let Some(socket) = &self.fd {
            socket.set_nonblocking(!blocking)?;
            Ok(())
        } else {
            Err(RedisError::Other("Socket not initialized".to_string()))
        }
    }

    fn wait_ready(&self, timeout: Option<Duration>) -> Result<bool, RedisError> {
        let mut pollfd = libc::pollfd {
            fd: self.fd.as_ref().unwrap().as_raw_fd(),
            events: libc::POLLOUT,
            revents: 0,
        };

        let timeout_ms = timeout.map_or(-1, |t| t.as_millis() as i32);
        let start = std::time::Instant::now();

        loop {
            let res = unsafe { libc::poll(&mut pollfd, 1, timeout_ms) };
            if res > 0 {
                return Ok(true);
            } else if res == 0 {
                return Ok(false);
            } else if Error::last_os_error().kind() != ErrorKind::Interrupted {
                return Err(RedisError::Io(Error::last_os_error()));
            }

            if let Some(t) = timeout {
                if start.elapsed() >= t {
                    return Ok(false);
                }
            }
        }
    }

    pub fn set_tcp_nodelay(&self, enable: bool) -> Result<(), RedisError> {
        if let Some(socket) = &self.fd {
            socket.set_nodelay(enable)?;
            Ok(())
        } else {
            Err(RedisError::Other("Socket not initialized".to_string()))
        }
    }

    pub fn set_keepalive(&self, interval: Duration) -> Result<(), RedisError> {
        if let Some(socket) = &self.fd {
            socket.set_keepalive(Some(interval))?;
            Ok(())
        } else {
            Err(RedisError::Other("Socket not initialized".to_string()))
        }
    }
}

const REDIS_BLOCK: i32 = 1 << 0;
const REDIS_CONNECTED: i32 = 1 << 1;
const REDIS_REUSEADDR: i32 = 1 << 2;
const REDIS_PREFER_IPV4: i32 = 1 << 3;
const REDIS_PREFER_IPV6: i32 = 1 << 4;