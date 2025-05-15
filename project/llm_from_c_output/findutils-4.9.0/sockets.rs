/*!
 * Rust implementation of Windows socket wrappers
 *
 * Original C code Copyright (C) 2008-2022 Free Software Foundation, Inc.
 * Rust translation maintains same license: GNU Lesser General Public License
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketVersion {
    V1_0 = 0x0001,
    V1_1 = 0x0101,
    V2_0 = 0x0002,
    V2_1 = 0x0102,
    V2_2 = 0x0202,
}

impl SocketVersion {
    pub fn from_raw(version: u16) -> Option<Self> {
        match version {
            0x0001 => Some(Self::V1_0),
            0x0101 => Some(Self::V1_1),
            0x0002 => Some(Self::V2_0),
            0x0102 => Some(Self::V2_1),
            0x0202 => Some(Self::V2_2),
            _ => None,
        }
    }
}

#[cfg(windows)]
mod windows_impl {
    use super::SocketVersion;
    use std::io;
    use std::os::windows::io::{AsRawSocket, FromRawSocket, RawSocket};
    use winapi::um::winsock2::{WSADATA, WSAStartup, WSACleanup, WSAEnumNetworkEvents, WSANETWORKEVENTS};
    use winapi::um::winsock2::{closesocket, ioctlsocket};

    static mut INITIALIZED_VERSION: Option<SocketVersion> = None;

    pub fn sockets_startup(version: SocketVersion) -> io::Result<()> {
        unsafe {
            if INITIALIZED_VERSION.map_or(true, |v| version as u16 > v as u16) {
                let mut data: WSADATA = std::mem::zeroed();
                let err = WSAStartup(version as u16, &mut data);
                
                if err != 0 {
                    return Err(io::Error::from_raw_os_error(err));
                }

                if data.wVersion != version as u16 {
                    WSACleanup();
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Requested socket version not available",
                    ));
                }

                if INITIALIZED_VERSION.is_none() {
                    // TODO: Register fd hooks equivalent
                }

                INITIALIZED_VERSION = Some(version);
            }
            Ok(())
        }
    }

    pub fn sockets_cleanup() -> io::Result<()> {
        unsafe {
            INITIALIZED_VERSION = None;
            // TODO: Unregister fd hooks equivalent
            
            let err = WSACleanup();
            if err != 0 {
                return Err(io::Error::from_raw_os_error(err));
            }
            Ok(())
        }
    }

    pub fn fd_to_handle(fd: RawSocket) -> RawSocket {
        fd
    }

    fn is_socket(fd: RawSocket) -> bool {
        unsafe {
            let mut ev: WSANETWORKEVENTS = std::mem::zeroed();
            ev.lNetworkEvents = 0xDEADBEEF;
            WSAEnumNetworkEvents(fd, std::ptr::null_mut(), &mut ev);
            ev.lNetworkEvents != 0xDEADBEEF
        }
    }

    pub fn close_socket(fd: RawSocket) -> io::Result<()> {
        unsafe {
            if is_socket(fd) {
                if closesocket(fd) != 0 {
                    return Err(io::Error::last_os_error());
                }
            }
            Ok(())
        }
    }

    pub fn ioctl_socket(fd: RawSocket, request: i32, arg: *mut u32) -> io::Result<()> {
        unsafe {
            if is_socket(fd) {
                if ioctlsocket(fd, request, arg) != 0 {
                    return Err(io::Error::last_os_error());
                }
            }
            Ok(())
        }
    }
}

#[cfg(not(windows))]
mod unix_impl {
    use super::SocketVersion;
    use std::os::unix::io::RawFd;

    pub fn sockets_startup(_version: SocketVersion) -> std::io::Result<()> {
        Ok(())
    }

    pub fn sockets_cleanup() -> std::io::Result<()> {
        Ok(())
    }

    pub fn fd_to_handle(fd: RawFd) -> RawFd {
        fd
    }
}

#[cfg(windows)]
pub use windows_impl::*;

#[cfg(not(windows))]
pub use unix_impl::*;