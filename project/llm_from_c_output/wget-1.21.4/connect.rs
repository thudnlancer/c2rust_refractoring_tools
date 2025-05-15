use std::net::{SocketAddr, TcpStream, TcpListener};
use std::time::Duration;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::os::unix::io::{AsRawFd, RawFd};

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {
    Local,
    Peer,
}

#[derive(Debug)]
pub enum ConnectError {
    HostUnresolvable,
    Timeout,
    ConnectionRefused,
    Other(io::Error),
}

pub struct TransportImplementation {
    reader: Option<Box<dyn Fn(RawFd, &mut [u8], Duration) -> io::Result<usize> + Send + Sync>>,
    writer: Option<Box<dyn Fn(RawFd, &[u8]) -> io::Result<usize> + Send + Sync>>,
    poller: Option<Box<dyn Fn(RawFd, Duration, i32) -> io::Result<bool> + Send + Sync>>,
    peeker: Option<Box<dyn Fn(RawFd, &mut [u8], Duration) -> io::Result<usize> + Send + Sync>>,
    errstr: Option<Box<dyn Fn(RawFd) -> String + Send + Sync>>,
    closer: Option<Box<dyn Fn(RawFd) -> io::Result<()> + Send + Sync>>,
}

pub struct SocketManager {
    transports: Arc<Mutex<HashMap<RawFd, (TransportImplementation, Box<dyn Any + Send + Sync>)>>>,
}

impl SocketManager {
    pub fn new() -> Self {
        SocketManager {
            transports: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_transport(
        &self,
        fd: RawFd,
        imp: TransportImplementation,
        ctx: Box<dyn Any + Send + Sync>,
    ) {
        self.transports.lock().unwrap().insert(fd, (imp, ctx));
    }

    pub fn connect_to_host(host: &str, port: u16) -> Result<TcpStream, ConnectError> {
        let addrs = match host.parse::<SocketAddr>() {
            Ok(addr) => vec![addr],
            Err(_) => match (host, port).to_socket_addrs() {
                Ok(addrs) => addrs.collect(),
                Err(_) => return Err(ConnectError::HostUnresolvable),
            },
        };

        for addr in addrs {
            match TcpStream::connect_timeout(&addr, Duration::from_secs(10)) {
                Ok(stream) => return Ok(stream),
                Err(e) if e.kind() == io::ErrorKind::ConnectionRefused => {
                    return Err(ConnectError::ConnectionRefused)
                }
                Err(e) if e.kind() == io::ErrorKind::TimedOut => {
                    return Err(ConnectError::Timeout)
                }
                _ => continue,
            }
        }

        Err(ConnectError::Other(io::Error::new(
            io::ErrorKind::Other,
            "Failed to connect to any address",
        )))
    }

    pub fn bind_local(addr: &str, port: u16) -> Result<(TcpListener, u16), io::Error> {
        let socket_addr = SocketAddr::new(addr.parse().unwrap(), port);
        let listener = TcpListener::bind(socket_addr)?;
        let local_port = listener.local_addr()?.port();
        Ok((listener, local_port))
    }

    pub fn accept_connection(listener: &TcpListener, timeout: Option<Duration>) -> Result<TcpStream, io::Error> {
        if let Some(timeout) = timeout {
            listener.set_nonblocking(true)?;
            let start = std::time::Instant::now();
            loop {
                match listener.accept() {
                    Ok((stream, _)) => {
                        stream.set_nonblocking(false)?;
                        return Ok(stream);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if start.elapsed() >= timeout {
                            return Err(io::Error::new(io::ErrorKind::TimedOut, "Connection timeout"));
                        }
                        std::thread::sleep(Duration::from_millis(100));
                    }
                    Err(e) => return Err(e),
                }
            }
        } else {
            listener.accept().map(|(stream, _)| stream)
        }
    }

    pub fn socket_ip_address(stream: &TcpStream, endpoint: Endpoint) -> Result<SocketAddr, io::Error> {
        match endpoint {
            Endpoint::Local => stream.local_addr(),
            Endpoint::Peer => stream.peer_addr(),
        }
    }

    pub fn test_socket_open(stream: &TcpStream) -> bool {
        let mut buf = [0; 1];
        match stream.peek(&mut buf) {
            Ok(0) => false,
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn fd_read(&self, fd: RawFd, buf: &mut [u8], timeout: Option<Duration>) -> io::Result<usize> {
        let transports = self.transports.lock().unwrap();
        if let Some((imp, _)) = transports.get(&fd) {
            if let Some(reader) = &imp.reader {
                return reader(fd, buf, timeout.unwrap_or(Duration::from_secs(10)));
            }
        }
        
        let mut stream = unsafe { TcpStream::from_raw_fd(fd) };
        let result = stream.read(buf);
        std::mem::forget(stream);
        result
    }

    pub fn fd_write(&self, fd: RawFd, buf: &[u8], timeout: Option<Duration>) -> io::Result<usize> {
        let transports = self.transports.lock().unwrap();
        if let Some((imp, _)) = transports.get(&fd) {
            if let Some(writer) = &imp.writer {
                return writer(fd, buf);
            }
        }
        
        let mut stream = unsafe { TcpStream::from_raw_fd(fd) };
        let result = stream.write(buf);
        std::mem::forget(stream);
        result
    }

    pub fn fd_close(&self, fd: RawFd) -> io::Result<()> {
        let mut transports = self.transports.lock().unwrap();
        if let Some((imp, _)) = transports.remove(&fd) {
            if let Some(closer) = imp.closer {
                closer(fd)?;
            }
        }
        
        let stream = unsafe { TcpStream::from_raw_fd(fd) };
        drop(stream);
        Ok(())
    }
}

impl Drop for SocketManager {
    fn drop(&mut self) {
        let mut transports = self.transports.lock().unwrap();
        for (fd, (imp, _)) in transports.drain() {
            if let Some(closer) = imp.closer {
                let _ = closer(fd);
            }
            let _ = unsafe { TcpStream::from_raw_fd(fd) };
        }
    }
}