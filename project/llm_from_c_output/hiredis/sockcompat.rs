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

#[cfg(windows)]
mod win32 {
    use std::io;
    use std::os::windows::io::{AsRawSocket, RawSocket};
    use winapi::um::winsock2::{
        WSAGetLastError, WSAStartup, WSADATA, MAKEWORD, SOCKET, INVALID_SOCKET, SOCKET_ERROR,
        WSAPoll, closesocket, ioctlsocket, bind, connect, getsockopt, setsockopt, recv, send,
        WSAIoctl, SIO_KEEPALIVE_VALS, tcp_keepalive, DWORD, SOL_SOCKET, SO_RCVTIMEO, SO_SNDTIMEO,
        SO_ERROR, WSATRY_AGAIN, WSAEINVAL, WSAEAFNOSUPPORT, WSA_NOT_ENOUGH_MEMORY,
        WSAHOST_NOT_FOUND, WSATYPE_NOT_FOUND, WSAESOCKTNOSUPPORT, WSANO_RECOVERY,
    };
    use winapi::um::ws2tcpip::{
        getaddrinfo, freeaddrinfo, gai_strerror, addrinfo, socklen_t,
    };
    use libc::{EWOULDBLOCK, EINPROGRESS, EALREADY, ENOTSOCK, EDESTADDRREQ, EMSGSIZE, EPROTOTYPE,
               ENOPROTOOPT, EPROTONOSUPPORT, EOPNOTSUPP, EAFNOSUPPORT, EADDRINUSE, EADDRNOTAVAIL,
               ENETDOWN, ENETUNREACH, ENETRESET, ECONNABORTED, ECONNRESET, ENOBUFS, EISCONN,
               ENOTCONN, ETIMEDOUT, ECONNREFUSED, ELOOP, ENAMETOOLONG, EHOSTUNREACH, ENOTEMPTY,
               EIO, EAI_AGAIN, EAI_BADFLAGS, EAI_FAMILY, EAI_MEMORY, EAI_NONAME, EAI_SERVICE,
               EAI_SOCKTYPE, EAI_FAIL, timeval};

    fn wsa_error_to_errno(err: i32) -> i32 {
        match err {
            WSAEWOULDBLOCK => EWOULDBLOCK,
            WSAEINPROGRESS => EINPROGRESS,
            WSAEALREADY => EALREADY,
            WSAENOTSOCK => ENOTSOCK,
            WSAEDESTADDRREQ => EDESTADDRREQ,
            WSAEMSGSIZE => EMSGSIZE,
            WSAEPROTOTYPE => EPROTOTYPE,
            WSAENOPROTOOPT => ENOPROTOOPT,
            WSAEPROTONOSUPPORT => EPROTONOSUPPORT,
            WSAEOPNOTSUPP => EOPNOTSUPP,
            WSAEAFNOSUPPORT => EAFNOSUPPORT,
            WSAEADDRINUSE => EADDRINUSE,
            WSAEADDRNOTAVAIL => EADDRNOTAVAIL,
            WSAENETDOWN => ENETDOWN,
            WSAENETUNREACH => ENETUNREACH,
            WSAENETRESET => ENETRESET,
            WSAECONNABORTED => ECONNABORTED,
            WSAECONNRESET => ECONNRESET,
            WSAENOBUFS => ENOBUFS,
            WSAEISCONN => EISCONN,
            WSAENOTCONN => ENOTCONN,
            WSAETIMEDOUT => ETIMEDOUT,
            WSAECONNREFUSED => ECONNREFUSED,
            WSAELOOP => ELOOP,
            WSAENAMETOOLONG => ENAMETOOLONG,
            WSAEHOSTUNREACH => EHOSTUNREACH,
            WSAENOTEMPTY => ENOTEMPTY,
            _ => EIO,
        }
    }

    fn update_errno(success: bool) {
        unsafe {
            if success {
                io::Error::last_os_error().raw_os_error().unwrap_or(0);
            } else {
                io::Error::from_raw_os_error(wsa_error_to_errno(WSAGetLastError() as i32));
            }
        }
    }

    fn init_winsock() -> io::Result<()> {
        static mut INITIALIZED: bool = false;
        unsafe {
            if !INITIALIZED {
                let mut wsadata = WSADATA::default();
                let err = WSAStartup(MAKEWORD(2, 2), &mut wsadata);
                if err != 0 {
                    return Err(io::Error::from_raw_os_error(wsa_error_to_errno(err)));
                }
                INITIALIZED = true;
            }
        }
        Ok(())
    }

    pub fn win32_getaddrinfo(
        node: *const libc::c_char,
        service: *const libc::c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> i32 {
        if init_winsock().is_err() {
            return EAI_FAIL;
        }

        unsafe {
            match getaddrinfo(node, service, hints, res) {
                0 => 0,
                WSATRY_AGAIN => EAI_AGAIN,
                WSAEINVAL => EAI_BADFLAGS,
                WSAEAFNOSUPPORT => EAI_FAMILY,
                WSA_NOT_ENOUGH_MEMORY => EAI_MEMORY,
                WSAHOST_NOT_FOUND => EAI_NONAME,
                WSATYPE_NOT_FOUND => EAI_SERVICE,
                WSAESOCKTNOSUPPORT => EAI_SOCKTYPE,
                _ => EAI_FAIL,
            }
        }
    }

    pub fn win32_gai_strerror(errcode: i32) -> *const libc::c_char {
        let win_errcode = match errcode {
            0 => 0,
            EAI_AGAIN => WSATRY_AGAIN,
            EAI_BADFLAGS => WSAEINVAL,
            EAI_FAMILY => WSAEAFNOSUPPORT,
            EAI_MEMORY => WSA_NOT_ENOUGH_MEMORY,
            EAI_NONAME => WSAHOST_NOT_FOUND,
            EAI_SERVICE => WSATYPE_NOT_FOUND,
            EAI_SOCKTYPE => WSAESOCKTNOSUPPORT,
            _ => WSANO_RECOVERY,
        };
        unsafe { gai_strerror(win_errcode) }
    }

    pub fn win32_freeaddrinfo(res: *mut addrinfo) {
        unsafe { freeaddrinfo(res) }
    }

    pub fn win32_socket(domain: i32, type_: i32, protocol: i32) -> SOCKET {
        if init_winsock().is_err() {
            return INVALID_SOCKET;
        }

        let s = unsafe { socket(domain, type_, protocol) };
        update_errno(s != INVALID_SOCKET);
        s
    }

    pub fn win32_ioctl(fd: SOCKET, request: u32, argp: *mut u32) -> i32 {
        let ret = unsafe { ioctlsocket(fd, request as i32, argp) };
        update_errno(ret != SOCKET_ERROR);
        if ret == SOCKET_ERROR { -1 } else { ret }
    }

    pub fn win32_bind(sockfd: SOCKET, addr: *const libc::sockaddr, addrlen: socklen_t) -> i32 {
        let ret = unsafe { bind(sockfd, addr, addrlen) };
        update_errno(ret != SOCKET_ERROR);
        if ret == SOCKET_ERROR { -1 } else { ret }
    }

    pub fn win32_connect(sockfd: SOCKET, addr: *const libc::sockaddr, addrlen: socklen_t) -> i32 {
        let ret = unsafe { connect(sockfd, addr, addrlen) };
        update_errno(ret != SOCKET_ERROR);

        let err = io::Error::last_os_error().raw_os_error().unwrap_or(0);
        if err == EWOULDBLOCK {
            io::Error::last_os_error().raw_os_error().unwrap_or(EINPROGRESS);
        } else if err == EIO {
            io::Error::last_os_error().raw_os_error().unwrap_or(EALREADY);
        }

        if ret == SOCKET_ERROR { -1 } else { ret }
    }

    pub fn win32_getsockopt(
        sockfd: SOCKET,
        level: i32,
        optname: i32,
        optval: *mut libc::c_void,
        optlen: *mut socklen_t,
    ) -> i32 {
        let ret = if level == SOL_SOCKET && (optname == SO_RCVTIMEO || optname == SO_SNDTIMEO) {
            unsafe {
                if *optlen >= std::mem::size_of::<timeval>() as u32 {
                    let tv = optval as *mut timeval;
                    let mut timeout: DWORD = 0;
                    let mut dwlen: socklen_t = 0;
                    let ret = getsockopt(sockfd, level, optname, &mut timeout as *mut _ as *mut _, &mut dwlen);
                    (*tv).tv_sec = (timeout / 1000) as i32;
                    (*tv).tv_usec = ((timeout * 1000) % 1000000) as i32;
                    ret
                } else {
                    SOCKET_ERROR
                }
            }
        } else {
            unsafe { getsockopt(sockfd, level, optname, optval as *mut _, optlen) }
        };

        if ret != SOCKET_ERROR && level == SOL_SOCKET && optname == SO_ERROR {
            unsafe {
                let err = *(optval as *mut i32);
                if err != 0 {
                    *(optval as *mut i32) = wsa_error_to_errno(err);
                }
            }
        }

        update_errno(ret != SOCKET_ERROR);
        if ret == SOCKET_ERROR { -1 } else { ret }
    }

    pub fn win32_setsockopt(
        sockfd: SOCKET,
        level: i32,
        optname: i32,
        optval: *const libc::c_void,
        optlen: socklen_t,
    ) -> i32 {
        let ret = if level == SOL_SOCKET && (optname == SO_RCVTIMEO || optname == SO_SNDTIMEO) {
            unsafe {
                let tv = optval as *const timeval;
                let timeout = (*tv).tv_sec * 1000 + (*tv).tv_usec / 1000;
                setsockopt(sockfd, level, optname, &timeout as *const _ as *const _, std::mem::size_of::<DWORD>() as socklen_t)
            }
        } else {
            unsafe { setsockopt(sockfd, level, optname, optval as *const _, optlen) }
        };

        update_errno(ret != SOCKET_ERROR);
        if ret == SOCKET_ERROR { -1 } else { ret }
    }

    pub fn win32_close(fd: SOCKET) -> i32 {
        let ret = unsafe { closesocket(fd) };
        update_errno(ret != SOCKET_ERROR);
        if ret == SOCKET_ERROR { -1 } else { ret }
    }

    pub fn win32_recv(sockfd: SOCKET, buf: *mut libc::c_void, len: usize, flags: i32) -> isize {
        let ret = unsafe { recv(sockfd, buf as *mut _, len as i32, flags) };
        update_errno(ret != SOCKET_ERROR);
        if ret == SOCKET_ERROR { -1 } else { ret as isize }
    }

    pub fn win32_send(sockfd: SOCKET, buf: *const libc::c_void, len: usize, flags: i32) -> isize {
        let ret = unsafe { send(sockfd, buf as *const _, len as i32, flags) };
        update_errno(ret != SOCKET_ERROR);
        if ret == SOCKET_ERROR { -1 } else { ret as isize }
    }

    pub fn win32_poll(fds: *mut libc::pollfd, nfds: libc::nfds_t, timeout: i32) -> i32 {
        let ret = unsafe { WSAPoll(fds, nfds as u32, timeout) };
        update_errno(ret != SOCKET_ERROR);
        if ret == SOCKET_ERROR { -1 } else { ret }
    }

    pub fn win32_redis_keep_alive(sockfd: SOCKET, interval_ms: i32) -> i32 {
        let mut cfg = tcp_keepalive {
            onoff: 1,
            keepalivetime: interval_ms as u32,
            keepaliveinterval: interval_ms as u32,
        };

        let mut bytes_in: DWORD = 0;
        let res = unsafe {
            WSAIoctl(
                sockfd,
                SIO_KEEPALIVE_VALS,
                &mut cfg as *mut _ as *mut _,
                std::mem::size_of::<tcp_keepalive>() as DWORD,
                std::ptr::null_mut(),
                0,
                &mut bytes_in,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };

        if res == 0 { 0 } else { wsa_error_to_errno(res as i32) }
    }
}