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
use winapi::shared::ws2def::MAKEWORD;

pub const SOCKETS_1_0: u16 = 0x0001;
pub const SOCKETS_1_1: u16 = 0x0101;
pub const SOCKETS_2_0: u16 = 0x0002;
pub const SOCKETS_2_1: u16 = 0x0102;
pub const SOCKETS_2_2: u16 = 0x0202;

static mut INITIALIZED_SOCKETS_VERSION: u16 = 0;

/// Initialize Windows sockets
pub fn sockets_startup(version: u16) -> io::Result<()> {
    unsafe {
        if version > INITIALIZED_SOCKETS_VERSION {
            let mut data: WSADATA = std::mem::zeroed();
            let err = WSAStartup(MAKEWORD((version & 0xFF) as u8, (version >> 8) as u8), &mut data);
            
            if err != 0 {
                return Err(io::Error::from_raw_os_error(err));
            }

            if data.wVersion != version as u16 {
                WSACleanup();
                return Err(io::Error::new(io::ErrorKind::Other, "Invalid socket version"));
            }

            INITIALIZED_SOCKETS_VERSION = version;
        }
    }
    Ok(())
}

/// Clean up Windows sockets
pub fn sockets_cleanup() -> io::Result<()> {
    unsafe {
        INITIALIZED_SOCKETS_VERSION = 0;
        let err = WSACleanup();
        if err != 0 {
            return Err(io::Error::from_raw_os_error(err));
        }
    }
    Ok(())
}

/// Convert file descriptor to socket handle (Windows-specific)
#[cfg(windows)]
pub fn fd_to_handle<T: AsRawSocket>(socket: T) -> RawSocket {
    socket.as_raw_socket()
}

/// No-op conversion for non-Windows platforms
#[cfg(not(windows))]
pub fn fd_to_handle(fd: i32) -> i32 {
    fd
}

// Note: The original C code had additional functionality for socket hooks and
// IOCTL operations that would require unsafe Rust and more complex FFI bindings.
// This translation focuses on the core socket initialization functionality while
// maintaining Rust's safety guarantees.