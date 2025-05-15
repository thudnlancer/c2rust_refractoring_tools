use std::ffi::{CString, CStr};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::os::unix::io::{FromRawFd, RawFd};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::time::Duration;

const DEFAULT_PORT: u16 = 5703;
const PROTOCOL_VERSION: u32 = 11;
const FALLBACK_PROTOCOL_VERSION: u32 = 10;

#[derive(Debug, PartialEq)]
enum AuthError {
    Success,
    PacketOversize,
    AuthFailed,
    WrongVersion,
    DeviceLocked,
    IoError,
}

impl AuthError {
    fn from_code(code: u32) -> Self {
        match code {
            0 => AuthError::Success,
            1 => AuthError::PacketOversize,
            2 => AuthError::AuthFailed,
            3 => AuthError::WrongVersion,
            4 => AuthError::DeviceLocked,
            _ => AuthError::IoError,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            AuthError::Success => "Auth success!",
            AuthError::PacketOversize => "Auth failed: Packet oversized!",
            AuthError::AuthFailed => "Auth failed: X-Cookie doesn't match!",
            AuthError::WrongVersion => "Auth failed: Wrong transmission protocol version!",
            AuthError::DeviceLocked => "Auth failed: Device locked!",
            AuthError::IoError => "Auth failed: IO error!",
        }
    }
}

#[derive(Debug)]
struct ConnectionInfo {
    hostname: String,
    display: String,
    port: u16,
}

fn parse_connection_string(conn_str: &str) -> Option<ConnectionInfo> {
    let mut parts = conn_str.split('/');
    let host_part = parts.next()?;
    let port_part = parts.next().unwrap_or("");

    let mut host_port = host_part.split(':');
    let hostname = host_port.next()?.to_string();
    let port_offset = host_port.next().and_then(|s| s.parse::<u16>().ok()).unwrap_or(0);

    let port = if port_part.is_empty() {
        DEFAULT_PORT
    } else {
        port_part.parse().unwrap_or(DEFAULT_PORT)
    } + port_offset;

    let display = if hostname == "unix" || hostname.is_empty() {
        "localhost".to_string()
    } else {
        hostname.clone()
    };

    Some(ConnectionInfo {
        hostname: if hostname.is_empty() || hostname == "unix" {
            "localhost".to_string()
        } else {
            hostname
        },
        display,
        port,
    })
}

fn get_xauth_cookie(display: &str) -> Option<Vec<u8>> {
    let output = Command::new("xauth")
        .args(&["extract", "-", display])
        .stdout(Stdio::piped())
        .output()
        .ok()?;

    if output.status.success() {
        Some(output.stdout)
    } else {
        None
    }
}

fn authenticate(
    stream: &mut TcpStream,
    full_auth: bool,
    display: &str,
    protocol_version: u32,
) -> Result<(u32, u32), AuthError> {
    let mut buf = [0u8; 16];
    
    // Write auth header
    buf[0..4].copy_from_slice(&4u32.to_be_bytes());
    buf[4..8].copy_from_slice(&protocol_version.to_be_bytes());
    stream.write_all(&buf[..8]).map_err(|_| AuthError::IoError)?;

    // Read server response
    let mut response = [0u8; 12];
    let bytes_read = stream.read(&mut response).map_err(|_| AuthError::IoError)?;

    if bytes_read != 4 && bytes_read != 12 {
        return Err(AuthError::WrongVersion);
    }

    let error_code = u32::from_be_bytes([response[0], response[1], response[2], response[3]]);
    if error_code != 0 {
        return Err(AuthError::from_code(error_code));
    }

    let (server_version, capabilities) = if bytes_read == 12 {
        (
            u32::from_be_bytes([response[4], response[5], response[6], response[7]]),
            u32::from_be_bytes([response[8], response[9], response[10], response[11]]),
        )
    } else {
        (FALLBACK_PROTOCOL_VERSION, 0)
    };

    if full_auth {
        if let Some(cookie) = get_xauth_cookie(display) {
            let len = cookie.len() as u32;
            let mut auth_packet = Vec::with_capacity(4 + cookie.len());
            auth_packet.extend_from_slice(&len.to_be_bytes());
            auth_packet.extend_from_slice(&cookie);
            stream.write_all(&auth_packet).map_err(|_| AuthError::IoError)?;

            let mut response = [0u8; 8];
            stream.read_exact(&mut response).map_err(|_| AuthError::IoError)?;
            let error_code = u32::from_be_bytes([response[0], response[1], response[2], response[3]]);
            if error_code != 0 {
                return Err(AuthError::from_code(error_code));
            }
        } else {
            return Err(AuthError::AuthFailed);
        }
    }

    Ok((server_version, capabilities))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: floppyd_installtest [-f] Connect-String\n-f\tDo full X-Cookie-Authentication");
        std::process::exit(1);
    }

    let (full_auth, conn_str) = if args[1] == "-f" {
        if args.len() < 3 {
            eprintln!("Missing connection string after -f");
            std::process::exit(1);
        }
        (true, args[2].as_str())
    } else {
        (false, args[1].as_str())
    };

    let conn_info = parse_connection_string(conn_str).ok_or("Invalid connection string")?;

    let addr = format!("{}:{}", conn_info.hostname, conn_info.port)
        .to_socket_addrs()?
        .next()
        .ok_or("Could not resolve address")?;

    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(5))?;
    stream.set_nodelay(true)?;

    let mut protocol_version = PROTOCOL_VERSION;
    let auth_result = loop {
        match authenticate(&mut stream, full_auth, &conn_info.display, protocol_version) {
            Err(AuthError::WrongVersion) if protocol_version == PROTOCOL_VERSION => {
                protocol_version = FALLBACK_PROTOCOL_VERSION;
                continue;
            }
            res => break res,
        }
    };

    match auth_result {
        Ok((ver, caps)) => {
            println!("Protocol Version={}", ver);
            if ver >= 11 {
                println!(
                    "Capabilities:{}{}",
                    if caps & 1 != 0 { " ExplicitOpen" } else { "" },
                    if caps & 2 != 0 { " LargeFiles" } else { "" }
                );
            }
        }
        Err(e) => {
            eprintln!("Connection to floppyd failed:\n{}", e.message());
            std::process::exit(1);
        }
    }

    // Send close command
    stream.write_all(&1u32.to_be_bytes())?;
    stream.write_all(&4u8.to_be_bytes())?;

    Ok(())
}