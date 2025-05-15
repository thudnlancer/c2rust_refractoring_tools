/* SSL support via GnuTLS library.
   Copyright (C) 2005-2012, 2015, 2018-2023 Free Software Foundation,
   Inc.

This file is part of GNU Wget.

GNU Wget is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 3 of the License, or
(at your option) any later version.

GNU Wget is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Wget.  If not, see <http://www.gnu.org/licenses/>.

Additional permission under GNU GPL version 3 section 7

If you modify this program, or any covered work, by linking or
combining it with the OpenSSL project's OpenSSL library (or a
modified version of that library), containing parts covered by the
terms of the OpenSSL or SSLeay licenses, the Free Software Foundation
grants you additional permission to convey the resulting work.
Corresponding Source for a non-source form of such a combination
shall include the source code for the parts of OpenSSL used as well
as that of the covered work.  */

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;
use std::fs;
use std::io::{Error, ErrorKind};
use libc::{time_t, close};
use gnutls_sys::*;
use std::collections::HashMap;

struct ReadTimer {
    timeout: f64,
    next_timeout: f64,
    timer: *mut ptimer,
    timed_out: bool,
}

struct WgnutlsTransportContext {
    session: gnutls_session_t,
    session_data: Option<Box<gnutls_datum_t>>,
    last_error: c_int,
    peekbuf: [u8; 512],
    peeklen: usize,
}

impl Drop for WgnutlsTransportContext {
    fn drop(&mut self) {
        unsafe {
            if let Some(data) = self.session_data.take() {
                if !data.data.is_null() {
                    gnutls_free(data.data as *mut _);
                }
            }
            gnutls_deinit(self.session);
        }
    }
}

static mut SSL_INITIALIZED: bool = false;
static mut CREDENTIALS: Option<gnutls_certificate_credentials_t> = None;

fn ssl_init() -> Result<(), String> {
    unsafe {
        if SSL_INITIALIZED {
            return Ok(());
        }

        let mut credentials: gnutls_certificate_credentials_t = ptr::null_mut();
        let rc = gnutls_certificate_allocate_credentials(&mut credentials);
        if rc != GNUTLS_E_SUCCESS {
            return Err(format!("Failed to allocate credentials: {}", gnutls_strerror(rc)));
        }

        gnutls_certificate_set_verify_flags(credentials, GNUTLS_VERIFY_ALLOW_X509_V1_CA_CRT);

        let mut ncerts = -1;
        #[cfg(gnutls_v3)]
        {
            if opt.ca_directory.is_none() {
                ncerts = gnutls_certificate_set_x509_system_trust(credentials);
            }
        }

        if ncerts <= 0 {
            ncerts = 0;
            let ca_directory = opt.ca_directory.as_ref().map(|s| s.as_str()).unwrap_or("/etc/ssl/certs");

            if let Ok(dir) = fs::read_dir(ca_directory) {
                let mut inode_map = HashMap::new();
                for entry in dir {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if let Ok(metadata) = fs::metadata(&path) {
                            if !metadata.is_file() {
                                continue;
                            }

                            let inode = metadata.ino();
                            if inode_map.contains_key(&inode) {
                                continue;
                            }

                            inode_map.insert(inode, ());
                            let path_str = path.to_str().unwrap_or("");
                            let rc = gnutls_certificate_set_x509_trust_file(
                                credentials,
                                path_str.as_ptr() as *const c_char,
                                GNUTLS_X509_FMT_PEM,
                            );
                            if rc <= 0 {
                                log_debug(format!("WARNING: Failed to open cert {}: ({})", path_str, rc));
                            } else {
                                ncerts += rc;
                            }
                        }
                    }
                }
            }
        }

        if let Some(ca_cert) = &opt.ca_cert {
            if ncerts < 0 {
                ncerts = 0;
            }
            let rc = gnutls_certificate_set_x509_trust_file(
                credentials,
                ca_cert.as_ptr() as *const c_char,
                GNUTLS_X509_FMT_PEM,
            );
            if rc <= 0 {
                log_error(format!("ERROR: Failed to open cert {}: ({})", ca_cert, rc));
            } else {
                ncerts += rc;
                log_verbose(format!("Loaded CA certificate '{}'", ca_cert));
            }
        }

        if let Some(crl_file) = &opt.crl_file {
            let rc = gnutls_certificate_set_x509_crl_file(
                credentials,
                crl_file.as_ptr() as *const c_char,
                GNUTLS_X509_FMT_PEM,
            );
            if rc <= 0 {
                return Err(format!("ERROR: Failed to load CRL file '{}': ({})", crl_file, rc));
            }
            log_verbose(format!("Loaded CRL file '{}'", crl_file));
        }

        log_debug(format!("Certificates loaded: {}", ncerts));

        if let (Some(cert_file), None) = (&opt.cert_file, &opt.private_key) {
            opt.private_key = Some(cert_file.clone());
            opt.private_key_type = opt.cert_type;
        }

        if let (None, Some(private_key)) = (&opt.cert_file, &opt.private_key) {
            opt.cert_file = Some(private_key.clone());
            opt.cert_type = opt.private_key_type;
        }

        if let (Some(cert_file), Some(private_key)) = (&opt.cert_file, &opt.private_key) {
            if opt.private_key_type != opt.cert_type {
                return Err("ERROR: GnuTLS requires the key and the cert to be of the same type.".to_string());
            }

            let type_ = key_type_to_gnutls_type(opt.private_key_type);
            let rc = gnutls_certificate_set_x509_key_file(
                credentials,
                cert_file.as_ptr() as *const c_char,
                private_key.as_ptr() as *const c_char,
                type_,
            );
            if rc != GNUTLS_E_SUCCESS {
                return Err(format!("Failed to set key file: {}", gnutls_strerror(rc)));
            }
        }

        CREDENTIALS = Some(credentials);
        SSL_INITIALIZED = true;
        Ok(())
    }
}

fn ssl_cleanup() {
    unsafe {
        if !SSL_INITIALIZED {
            return;
        }

        if let Some(credentials) = CREDENTIALS.take() {
            gnutls_certificate_free_credentials(credentials);
        }

        gnutls_global_deinit();
        SSL_INITIALIZED = false;
    }
}

fn key_type_to_gnutls_type(type_: keyfile_type) -> c_int {
    match type_ {
        keyfile_type::PEM => GNUTLS_X509_FMT_PEM,
        keyfile_type::ASN1 => GNUTLS_X509_FMT_DER,
        _ => panic!("Invalid keyfile type"),
    }
}

fn wgnutls_read_timeout(
    fd: c_int,
    buf: &mut [u8],
    ctx: &mut WgnutlsTransportContext,
    timeout: f64,
) -> Result<usize, Error> {
    unsafe {
        let pending = gnutls_record_check_pending(ctx.session);
        if pending > 0 {
            let to_read = std::cmp::min(pending as usize, buf.len());
            let rc = gnutls_record_recv(ctx.session, buf.as_mut_ptr() as *mut _, to_read as size_t);
            if rc < 0 {
                return Err(Error::new(ErrorKind::Other, gnutls_strerror(rc)));
            }
            return Ok(rc as usize);
        }

        let mut read_timer = ReadTimer {
            timeout: if timeout == -1.0 { opt.read_timeout } else { timeout },
            next_timeout: 0.0,
            timer: ptr::null_mut(),
            timed_out: false,
        };

        if read_timer.timeout > 0.0 {
            let flags = libc::fcntl(fd, libc::F_GETFL);
            if flags < 0 {
                return Err(Error::last_os_error());
            }
            if libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) < 0 {
                return Err(Error::last_os_error());
            }

            read_timer.timer = ptimer_new();
            if read_timer.timer.is_null() {
                return Err(Error::new(ErrorKind::Other, "Failed to create timer"));
            }
            read_timer.next_timeout = read_timer.timeout;
        }

        let mut ret = ctx.last_error;
        loop {
            if ret == GNUTLS_E_REHANDSHAKE {
                let err = do_handshake(ctx.session, fd, Some(&mut read_timer))?;
                if err != 0 {
                    ret = err;
                    break;
                }
            }
            #[cfg(gnutls_v3_6_4)]
            if ret == GNUTLS_E_REAUTH_REQUEST {
                let err = do_reauth(ctx.session, fd, Some(&mut read_timer))?;
                if err != 0 {
                    ret = err;
                    break;
                }
            }

            loop {
                ret = gnutls_record_recv(ctx.session, buf.as_mut_ptr() as *mut _, buf.len() as size_t);
                if ret == GNUTLS_E_AGAIN && !read_timer.timer.is_null() {
                    let direction = if gnutls_record_get_direction(ctx.session) != 0 {
                        WAIT_FOR_WRITE
                    } else {
                        WAIT_FOR_READ
                    };
                    let err = select_fd_nb(fd, read_timer.next_timeout, direction);
                    if err <= 0 {
                        if err == 0 {
                            read_timer.timed_out = true;
                        }
                        break;
                    }
                    read_timer.next_timeout = read_timer.timeout - ptimer_measure(read_timer.timer);
                    if read_timer.next_timeout <= 0.0 {
                        read_timer.timed_out = true;
                        break;
                    }
                } else if ret != GNUTLS_E_AGAIN && ret != GNUTLS_E_INTERRUPTED {
                    break;
                }
            }

            if ret != GNUTLS_E_REHANDSHAKE && 
               #[cfg(gnutls_v3_6_4)]
               ret != GNUTLS_E_REAUTH_REQUEST {
                break;
            }
        }

        if !read_timer.timer.is_null() {
            ptimer_destroy(read_timer.timer);
            let flags = libc::fcntl(fd, libc::F_GETFL);
            if flags < 0 {
                return Err(Error::last_os_error());
            }
            if libc::fcntl(fd, libc::F_SETFL, flags & !libc::O_NONBLOCK) < 0 {
                return Err(Error::last_os_error());
            }
            if read_timer.timed_out {
                return Err(Error::new(ErrorKind::TimedOut, "Operation timed out"));
            }
        }

        if ret < 0 {
            Err(Error::new(ErrorKind::Other, gnutls_strerror(ret)))
        } else {
            Ok(ret as usize)
        }
    }
}

// ... (rest of the implementation would follow the same pattern)

fn ssl_connect_wget(fd: c_int, hostname: &str, continue_session: Option<&mut c_int>) -> Result<bool, String> {
    unsafe {
        let mut session: gnutls_session_t = ptr::null_mut();
        #[cfg(gnutls_v3_6_4)]
        let rc = gnutls_init(&mut session, GNUTLS_CLIENT | GNUTLS_POST_HANDSHAKE_AUTH);
        #[cfg(not(gnutls_v3_6_4))]
        let rc = gnutls_init(&mut session, GNUTLS_CLIENT);
        
        if rc != GNUTLS_E_SUCCESS {
            return Err(format!("Failed to initialize session: {}", gnutls_strerror(rc)));
        }

        if !is_valid_ip_address(hostname) {
            let sni_hostname = _sni_hostname(hostname);
            let rc = gnutls_server_name_set(
                session,
                GNUTLS_NAME_DNS,
                sni_hostname.as_ptr() as *const c_char,
                sni_hostname.len(),
            );
            if rc != GNUTLS_E_SUCCESS {
                gnutls_deinit(session);
                return Err(format!("Failed to set server name: {}", gnutls_strerror(rc)));
            }
        }

        if let Some(credentials) = CREDENTIALS {
            let rc = gnutls_credentials_set(session, GNUTLS_CRD_CERTIFICATE, credentials);
            if rc != GNUTLS_E_SUCCESS {
                gnutls_deinit(session);
                return Err(format!("Failed to set credentials: {}", gnutls_strerror(rc)));
            }
        }

        gnutls_transport_set_ptr(session, fd as gnutls_transport_ptr_t);

        let rc = if opt.tls_ciphers_string.is_none() {
            set_prio_default(session)
        } else {
            #[cfg(have_gnutls_priority_set_direct)]
            {
                let ciphers = CString::new(opt.tls_ciphers_string.unwrap()).unwrap();
                gnutls_priority_set_direct(session, ciphers.as_ptr(), ptr::null_mut())
            }
            #[cfg(not(have_gnutls_priority_set_direct))]
            {
                log_warning("GnuTLS: Cannot set prio string directly. Falling back to default priority.");
                gnutls_set_default_priority(session)
            }
        };

        if rc != GNUTLS_E_SUCCESS {
            gnutls_deinit(session);
            return Err(format!("Failed to set priority: {}", gnutls_strerror(rc)));
        }

        if let Some(continue_session) = continue_session {
            let ctx = fd_transport_context(*continue_session);
            if gnutls_session_is_resumed(session) == 0 {
                if ctx.is_none() || ctx.unwrap().session_data.is_none() ||
                    gnutls_session_set_data(
                        session,
                        ctx.unwrap().session_data.as_ref().unwrap().data,
                        ctx.unwrap().session_data.as_ref().unwrap().size,
                    ) != GNUTLS_E_SUCCESS
                {
                    if let Some(ctx) = ctx {
                        if let Some(data) = ctx.session_data.take() {
                            if !data.data.is_null() {
                                gnutls_free(data.data as *mut _);
                            }
                        }
                    }
                    gnutls_deinit(session);
                    return Ok(false);
                }
            } else {
                log_info("SSL session has already been resumed. Continuing.");
            }
        }

        let rc = do_handshake(session, fd, None)?;
        if rc != 0 {
            gnutls_deinit(session);
            return Ok(false);
        }

        let mut ctx = Box::new(WgnutlsTransportContext {
            session,
            session_data: Some(Box::new(gnutls_datum_t {
                data: ptr::null_mut(),
                size: 0,
            })),
            last_error: 0,
            peekbuf: [0; 512],
            peeklen: 0,
        });

        if gnutls_session_get_data2(session, ctx.session_data.as_mut().unwrap()) != GNUTLS_E_SUCCESS {
            log_warning(format!("WARNING: Could not save SSL session data for socket {}", fd));
            ctx.session_data = None;
        }

        fd_register_transport(fd, &WGNUTLS_TRANSPORT, ctx);
        Ok(true)
    }
}

// ... (remaining functions would follow similar patterns)