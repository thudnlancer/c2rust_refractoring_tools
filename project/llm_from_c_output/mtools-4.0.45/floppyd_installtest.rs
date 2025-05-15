use std::{
    ffi::CString,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    process::Command,
    str,
};

const FLOPPYD_DEFAULT_PORT: u16 = 5703;
const FLOPPYD_PROTOCOL_VERSION: u32 = 2;
const FLOPPYD_PROTOCOL_VERSION_OLD: u32 = 1;
const FLOPPYD_CAP_EXPLICIT_OPEN: u32 = 1;
const FLOPPYD_CAP_LARGE_SEEK: u32 = 2;

const AUTH_SUCCESS: u32 = 0;
const AUTH_AUTHFAILED: u32 = 1;
const AUTH_PACKETOVERSIZE: u32 = 2;
const AUTH_WRONGVERSION: u32 = 3;
const AUTH_IO_ERROR: u32 = 4;

const OP_CLOSE: u8 = 0;

static AUTH_ERRORS: [&str; 5] = [
    "Auth success!",
    "Auth failed: Packet oversized!",
    "Auth failed: X-Cookie doesn't match!",
    "Auth failed: Wrong transmission protocol version!",
    "Auth failed: Device locked!",
];

fn dword2byte(value: u32, buf: &mut [u8; 4]) {
    buf[0] = (value >> 24) as u8;
    buf[1] = (value >> 16) as u8;
    buf[2] = (value >> 8) as u8;
    buf[3] = value as u8;
}

fn read_dword(stream: &mut TcpStream) -> Result<u32, std::io::Error> {
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

fn write_dword(stream: &mut TcpStream, value: u32) -> Result<(), std::io::Error> {
    let mut buf = [0u8; 4];
    dword2byte(value, &mut buf);
    stream.write_all(&buf)
}

fn authenticate_to_floppyd(
    fullauth: bool,
    sock: &mut TcpStream,
    display: &str,
    protoversion: u32,
) -> Result<u32, std::io::Error> {
    let mut buf = [0u8; 16];
    let mut xcookie = Vec::new();

    if fullauth {
        let output = Command::new("xauth")
            .args(&["extract", "-", display])
            .output()?;

        if output.stdout.is_empty() {
            return Ok(AUTH_AUTHFAILED);
        }

        xcookie = output.stdout;
    }

    dword2byte(4, &mut buf[0..4]);
    dword2byte(protoversion, &mut buf[4..8]);
    sock.write_all(&buf[0..8])?;

    let bytes_read = read_dword(sock)?;

    if bytes_read != 4 && bytes_read != 12 {
        return Ok(AUTH_WRONGVERSION);
    }

    let errcode = read_dword(sock)?;

    if errcode != AUTH_SUCCESS {
        return Ok(errcode);
    }

    let mut protoversion = FLOPPYD_PROTOCOL_VERSION_OLD;
    let mut cap = 0;
    if bytes_read >= 12 {
        protoversion = read_dword(sock)?;
        cap = read_dword(sock)?;
    }

    eprintln!("Protocol Version={}", protoversion);
    if protoversion >= FLOPPYD_PROTOCOL_VERSION {
        eprintln!(
            "Capabilities:{}{}",
            if (cap & FLOPPYD_CAP_EXPLICIT_OPEN) != 0 {
                " ExplicitOpen"
            } else {
                ""
            },
            if (cap & FLOPPYD_CAP_LARGE_SEEK) != 0 {
                " LargeFiles"
            } else {
                ""
            }
        );
    }

    if fullauth {
        let mut len_buf = [0u8; 4];
        dword2byte(xcookie.len() as u32, &mut len_buf);
        sock.write_all(&len_buf)?;
        sock.write_all(&xcookie)?;

        if read_dword(sock)? != 4 {
            return Ok(AUTH_PACKETOVERSIZE);
        }

        let errcode = read_dword(sock)?;
        return Ok(errcode);
    }

    Ok(errcode)
}

fn get_host_and_port(name: &str) -> Result<(String, String, u16), Box<dyn std::error::Error>> {
    let mut parts = name.split('/');
    let mut display = parts.next().unwrap_or("").to_string();
    let port_str = parts.next().unwrap_or("");

    let mut port = if port_str.is_empty() {
        FLOPPYD_DEFAULT_PORT
    } else {
        port_str.parse::<u16>()?
    };

    let colon_pos = display.find(':');
    let hostname = if let Some(pos) = colon_pos {
        let display_num = display[pos + 1..].parse::<u16>()?;
        port += display_num;
        display.truncate(pos);
        if display.is_empty() || display == "unix" {
            "localhost".to_string()
        } else {
            display
        }
    } else {
        if display.is_empty() || display == "unix" {
            "localhost".to_string()
        } else {
            display.clone()
        }
    };

    Ok((hostname, display, port))
}

fn connect_to_server(host: &str, port: u16) -> Result<TcpStream, std::io::Error> {
    let addr = format!("{}:{}", host, port);
    let addrs = addr.to_socket_addrs()?.next().ok_or(std::io::Error::new(
        std::io::ErrorKind::Other,
        "No address found",
    ))?;
    TcpStream::connect(addrs)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!(
            "Usage: floppyd_installtest [-f] Connect-String\n\
             -f\tDo full X-Cookie-Authentication"
        );
        return Ok(());
    }

    let mut fullauth = false;
    let mut name_index = 1;

    if args[1] == "-f" {
        fullauth = true;
        if args.len() < 3 {
            println!("Missing connection string after -f");
            return Ok(());
        }
        name_index = 2;
    }

    let name = &args[name_index];
    let (hostname, display, port) = get_host_and_port(name)?;

    let mut sock = match connect_to_server(&hostname, port) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "Can't connect to floppyd server on {}, port {}!\n{}",
                hostname, port, e
            );
            return Ok(());
        }
    };

    let mut protoversion = FLOPPYD_PROTOCOL_VERSION;
    let reply = loop {
        let reply = authenticate_to_floppyd(fullauth, &mut sock, &display, protoversion)?;
        if protoversion == FLOPPYD_PROTOCOL_VERSION_OLD {
            break reply;
        }
        if reply == AUTH_WRONGVERSION {
            protoversion = FLOPPYD_PROTOCOL_VERSION_OLD;
            continue;
        }
        break reply;
    };

    if reply != AUTH_SUCCESS {
        eprintln!(
            "Connection to floppyd failed:\n{}",
            AUTH_ERRORS[reply as usize]
        );
        return Ok(());
    }

    write_dword(&mut sock, 1)?;
    sock.write_all(&[OP_CLOSE])?;

    Ok(())
}