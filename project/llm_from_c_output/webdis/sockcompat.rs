/*
 * Copyright (c) 2019, Marcus Geelnard <m at bitsnbites dot eu>
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *     this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *   * Neither the name of Redis nor the names of its contributors may be used
 *     to endorse or promote products derived from this software without
 *     specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

#[cfg(not(windows))]
mod posix {
    use std::os::unix::io::RawFd;
    use std::net::{TcpStream, TcpListener, UdpSocket};
    use std::time::Duration;
    use std::io::{self, Error, ErrorKind};

    pub type Socket = RawFd;

    pub fn socket(domain: i32, type_: i32, protocol: i32) -> io::Result<Socket> {
        unsafe {
            let fd = libc::socket(domain, type_, protocol);
            if fd == -1 {
                Err(Error::last_os_error())
            } else {
                Ok(fd)
            }
        }
    }

    pub fn connect(sockfd: Socket, addr: &libc::sockaddr, addrlen: libc::socklen_t) -> io::Result<()> {
        unsafe {
            if libc::connect(sockfd, addr, addrlen) == -1 {
                Err(Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }

    // Other POSIX socket functions would be implemented similarly
}

#[cfg(windows)]
mod win32 {
    use winapi::um::winsock2::{
        WSAGetLastError, WSAStartup, WSACleanup, WSAPoll, WSAPOLLFD,
        SOCKET, INVALID_SOCKET, SOCKET_ERROR,
        closesocket, connect, bind, getsockopt, setsockopt,
        recv, send, ioctlsocket, poll as win_poll,
    };
    use winapi::um::ws2tcpip::{
        getaddrinfo, freeaddrinfo, gai_strerror,
        addrinfo, socklen_t, ADDRINFOA,
        WSATRY_AGAIN, WSAEINVAL, WSAEAFNOSUPPORT,
        WSA_NOT_ENOUGH_MEMORY, WSAHOST_NOT_FOUND,
        WSATYPE_NOT_FOUND, WSAESOCKTNOSUPPORT,
        WSANO_RECOVERY,
    };
    use std::ptr;
    use std::io::{self, Error, ErrorKind};
    use std::time::Duration;
    use std::mem;
    use std::ffi::CStr;

    pub type Socket = SOCKET;

    static mut WSA_INITIALIZED: bool = false;

    fn init_winsock() -> io::Result<()> {
        unsafe {
            if !WSA_INITIALIZED {
                let mut wsadata = mem::zeroed();
                let err = WSAStartup(0x202, &mut wsadata);
                if err != 0 {
                    return Err(Error::from_raw_os_error(err));
                }
                WSA_INITIALIZED = true;
            }
            Ok(())
        }
    }

    fn wsa_error_to_io(err: i32) -> Error {
        Error::from_raw_os_error(wsa_error_to_errno(err))
    }

    fn wsa_error_to_errno(err: i32) -> i32 {
        match err {
            winapi::um::winsock2::WSAEWOULDBLOCK => libc::EWOULDBLOCK,
            winapi::um::winsock2::WSAEINPROGRESS => libc::EINPROGRESS,
            // ... other error mappings ...
            _ => libc::EIO,
        }
    }

    pub fn socket(domain: i32, type_: i32, protocol: i32) -> io::Result<Socket> {
        init_winsock()?;
        unsafe {
            let s = winsock2::socket(domain, type_, protocol);
            if s == INVALID_SOCKET {
                Err(wsa_error_to_io(WSAGetLastError()))
            } else {
                Ok(s)
            }
        }
    }

    pub fn connect(sockfd: Socket, addr: *const libc::sockaddr, addrlen: socklen_t) -> io::Result<()> {
        unsafe {
            if winsock2::connect(sockfd, addr, addrlen) == SOCKET_ERROR {
                let err = WSAGetLastError();
                if err == winapi::um::winsock2::WSAEWOULDBLOCK {
                    Err(Error::from_raw_os_error(libc::EINPROGRESS))
                } else {
                    Err(wsa_error_to_io(err))
                }
            } else {
                Ok(())
            }
        }
    }

    // Other Windows socket functions would be implemented similarly
}

#[cfg(windows)]
pub use win32::*;
#[cfg(not(windows))]
pub use posix::*;