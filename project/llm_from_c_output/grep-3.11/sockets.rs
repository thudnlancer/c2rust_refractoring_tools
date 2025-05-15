//! sockets.rs - wrappers for Windows socket functions
//!
//! This file is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Lesser General Public License as
//! published by the Free Software Foundation; either version 2.1 of the
//! License, or (at your option) any later version.
//!
//! This file is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU Lesser General Public License for more details.
//!
//! You should have received a copy of the GNU Lesser General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io;
use std::os::windows::io::{AsRawSocket, FromRawSocket, RawSocket};
use winapi::um::winsock2::{WSACleanup, WSAStartup, WSADATA};

pub const SOCKETS_1_0: u16 = 0x0001;
pub const SOCKETS_1_1: u16 = 0x0101;
pub const SOCKETS_2_0: u16 = 0x0002;
pub const SOCKETS_2_1: u16 = 0x0102;
pub const SOCKETS_2_2: u16 = 0x0202;

#[cfg(windows)]
pub fn gl_sockets_startup(version: u16) -> io::Result<()> {
    unsafe {
        let mut data: WSADATA = std::mem::zeroed();
        let err = WSAStartup(version, &mut data);
        if err != 0 {
            return Err(io::Error::last_os_error());
        }

        if data.wVersion != version {
            WSACleanup();
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Requested socket version not available",
            ));
        }

        Ok(())
    }
}

#[cfg(not(windows))]
pub fn gl_sockets_startup(_version: u16) -> io::Result<()> {
    Ok(())
}

#[cfg(windows)]
pub fn gl_sockets_cleanup() -> io::Result<()> {
    unsafe {
        let err = WSACleanup();
        if err != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

#[cfg(not(windows))]
pub fn gl_sockets_cleanup() -> io::Result<()> {
    Ok(())
}

pub trait SocketExt {
    fn as_raw_socket(&self) -> RawSocket;
    fn from_raw_socket(sock: RawSocket) -> Self;
}

impl<T: AsRawSocket> SocketExt for T {
    fn as_raw_socket(&self) -> RawSocket {
        self.as_raw_socket()
    }
    
    fn from_raw_socket(sock: RawSocket) -> Self {
        unsafe { Self::from_raw_socket(sock) }
    }
}

#[cfg(windows)]
pub fn gl_fd_to_handle(fd: RawSocket) -> RawSocket {
    fd
}

#[cfg(not(windows))]
pub fn gl_fd_to_handle(fd: i32) -> i32 {
    fd
}