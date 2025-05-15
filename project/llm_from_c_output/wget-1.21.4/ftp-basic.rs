use std::io::{self, Read, Write};
use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::{self, FromStr};
use std::fmt;
use std::error::Error;
use std::mem;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UErr {
    FtpOk,
    FtpRerr,
    FtpSrverr,
    FtpLogrefused,
    FtpLoginc,
    FtpNopasv,
    FtpInvpasv,
    FtpNoauth,
    FtpNopbsz,
    FtpNoprot,
    FtpPorterr,
    FtpNsfod,
    FtpRestfail,
    FtpUnknownType,
    WriteFailed,
    FtpSyserr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SType {
    StUnix,
    StVms,
    StWinnt,
    StMacos,
    StOs400,
    StOther,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum USType {
    UstTypeL8,
    UstMultinet,
    UstOther,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtLevel {
    ProtClear,
    ProtSafe,
    ProtConfidential,
    ProtPrivate,
}

pub struct FtpConnection {
    stream: TcpStream,
    buffer: Vec<u8>,
}

impl FtpConnection {
    pub fn new(stream: TcpStream) -> Self {
        FtpConnection {
            stream,
            buffer: vec![0; 4096],
        }
    }

    pub fn read_response(&mut self) -> Result<String, UErr> {
        let mut response = String::new();
        loop {
            let n = self.stream.read(&mut self.buffer).map_err(|_| UErr::FtpRerr)?;
            if n == 0 {
                return Err(UErr::FtpRerr);
            }

            let line = str::from_utf8(&self.buffer[..n]).map_err(|_| UErr::FtpRerr)?;
            response.push_str(line);

            if line.len() >= 4 
                && line.chars().nth(0).unwrap().is_digit(10)
                && line.chars().nth(1).unwrap().is_digit(10)
                && line.chars().nth(2).unwrap().is_digit(10)
                && line.chars().nth(3) == Some(' ') 
            {
                break;
            }
        }

        // Strip CRLF
        let response = response.trim_end().to_string();
        Ok(response)
    }

    pub fn send_command(&mut self, command: &str, arg: Option<&str>) -> Result<String, UErr> {
        let mut request = String::new();
        request.push_str(command);
        if let Some(arg) = arg {
            // Check for newlines in argument
            if arg.contains('\r') || arg.contains('\n') {
                let sanitized = arg.replace(|c| c == '\r' || c == '\n', " ");
                request.push(' ');
                request.push_str(&sanitized);
            } else {
                request.push(' ');
                request.push_str(arg);
            }
        }
        request.push_str("\r\n");

        self.stream.write_all(request.as_bytes()).map_err(|_| UErr::WriteFailed)?;
        self.read_response()
    }

    pub fn login(&mut self, user: &str, pass: &str) -> Result<(), UErr> {
        let response = self.send_command("USER", Some(user))?;
        if !response.starts_with('3') && !response.starts_with('2') {
            return Err(UErr::FtpLogrefused);
        }

        if response.starts_with('2') {
            return Ok(());
        }

        let response = self.send_command("PASS", Some(pass))?;
        if !response.starts_with('2') {
            Err(UErr::FtpLoginc)
        } else {
            Ok(())
        }
    }

    pub fn pasv(&mut self) -> Result<(IpAddr, u16), UErr> {
        let response = self.send_command("PASV", None)?;
        if !response.starts_with('2') {
            return Err(UErr::FtpNopasv);
        }

        // Parse PASV response (h1,h2,h3,h4,p1,p2)
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() < 6 {
            return Err(UErr::FtpInvpasv);
        }

        let ip = Ipv4Addr::new(
            parts[0].parse().map_err(|_| UErr::FtpInvpasv)?,
            parts[1].parse().map_err(|_| UErr::FtpInvpasv)?,
            parts[2].parse().map_err(|_| UErr::FtpInvpasv)?,
            parts[3].parse().map_err(|_| UErr::FtpInvpasv)?,
        );

        let port = (parts[4].parse::<u16>().map_err(|_| UErr::FtpInvpasv)? << 8)
            + parts[5].parse::<u16>().map_err(|_| UErr::FtpInvpasv)?;

        Ok((IpAddr::V4(ip), port))
    }

    pub fn epsv(&mut self) -> Result<u16, UErr> {
        let response = self.send_command("EPSV", None)?;
        if !response.starts_with('2') {
            return Err(UErr::FtpNopasv);
        }

        // Parse EPSV response (|||port|)
        let start = response.find('(').ok_or(UErr::FtpInvpasv)?;
        let end = response.find(')').ok_or(UErr::FtpInvpasv)?;
        let parts: Vec<&str> = response[start+1..end].split('|').collect();

        if parts.len() < 4 {
            return Err(UErr::FtpInvpasv);
        }

        let port = parts[3].parse().map_err(|_| UErr::FtpInvpasv)?;
        Ok(port)
    }

    pub fn retr(&mut self, file: &str) -> Result<(), UErr> {
        let response = self.send_command("RETR", Some(file))?;
        if response.starts_with('5') {
            Err(UErr::FtpNsfod)
        } else if !response.starts_with('1') {
            Err(UErr::FtpRerr)
        } else {
            Ok(())
        }
    }

    pub fn list(&mut self, path: Option<&str>) -> Result<(), UErr> {
        let response = self.send_command("LIST", path)?;
        if response.starts_with('5') {
            Err(UErr::FtpNsfod)
        } else if !response.starts_with('1') {
            Err(UErr::FtpRerr)
        } else {
            Ok(())
        }
    }

    pub fn syst(&mut self) -> Result<(SType, USType), UErr> {
        let response = self.send_command("SYST", None)?;
        if response.starts_with('5') {
            return Err(UErr::FtpSrverr);
        }

        let parts: Vec<&str> = response.split_whitespace().collect();
        if parts.len() < 2 {
            return Ok((SType::StOther, USType::UstOther));
        }

        let systype = match parts[1].to_uppercase().as_str() {
            "UNIX" => {
                let unix_type = if response.contains("UNIX Type: L8") {
                    USType::UstTypeL8
                } else if response.contains("UNIX MultiNet") {
                    USType::UstMultinet
                } else {
                    USType::UstOther
                };
                (SType::StUnix, unix_type)
            }
            "VMS" => (SType::StVms, USType::UstOther),
            "WINDOWS_NT" | "WINDOWS2000" => (SType::StWinnt, USType::UstOther),
            "MACOS" => (SType::StMacos, USType::UstOther),
            "OS/400" => (SType::StOs400, USType::UstOther),
            _ => (SType::StOther, USType::UstOther),
        };

        Ok(systype)
    }

    pub fn pwd(&mut self) -> Result<String, UErr> {
        let response = self.send_command("PWD", None)?;
        if response.starts_with('5') {
            return Err(UErr::FtpSrverr);
        }

        let start = response.find('"').ok_or(UErr::FtpSrverr)?;
        let end = response.rfind('"').ok_or(UErr::FtpSrverr)?;
        Ok(response[start+1..end].to_string())
    }

    pub fn size(&mut self, file: &str) -> Result<u64, UErr> {
        let response = self.send_command("SIZE", Some(file))?;
        if response.starts_with('5') {
            return Ok(0);
        }

        let parts: Vec<&str> = response.split_whitespace().collect();
        if parts.len() < 2 {
            return Ok(0);
        }

        parts[1].parse().map_err(|_| UErr::FtpRerr)
    }

    pub fn cwd(&mut self, dir: &str) -> Result<(), UErr> {
        let response = self.send_command("CWD", Some(dir))?;
        if response.starts_with('5') {
            Err(UErr::FtpNsfod)
        } else if !response.starts_with('2') {
            Err(UErr::FtpRerr)
        } else {
            Ok(())
        }
    }

    pub fn rest(&mut self, offset: u64) -> Result<(), UErr> {
        let response = self.send_command("REST", Some(&offset.to_string()))?;
        if !response.starts_with('3') {
            Err(UErr::FtpRestfail)
        } else {
            Ok(())
        }
    }

    pub fn type_(&mut self, type_char: char) -> Result<(), UErr> {
        let response = self.send_command("TYPE", Some(&type_char.to_string()))?;
        if !response.starts_with('2') {
            Err(UErr::FtpUnknownType)
        } else {
            Ok(())
        }
    }

    pub fn auth(&mut self, scheme: &str) -> Result<(), UErr> {
        let response = self.send_command("AUTH", Some(scheme))?;
        if !response.starts_with('2') {
            Err(UErr::FtpNoauth)
        } else {
            Ok(())
        }
    }

    pub fn prot(&mut self, level: ProtLevel) -> Result<(), UErr> {
        let level_str = match level {
            ProtLevel::ProtClear => "C",
            ProtLevel::ProtSafe => "S",
            ProtLevel::ProtConfidential => "E",
            ProtLevel::ProtPrivate => "P",
        };
        let response = self.send_command("PROT", Some(level_str))?;
        if !response.starts_with('2') {
            Err(UErr::FtpNoprot)
        } else {
            Ok(())
        }
    }

    pub fn pbsz(&mut self, size: u32) -> Result<(), UErr> {
        let response = self.send_command("PBSZ", Some(&size.to_string()))?;
        if !response.starts_with('2') {
            Err(UErr::FtpNopbsz)
        } else {
            Ok(())
        }
    }
}

pub fn ftp_process_type(params: &str) -> char {
    if params.len() >= 5 && params[..5].eq_ignore_ascii_case("type=") {
        params[5..].chars().next().unwrap_or('I').to_ascii_uppercase()
    } else {
        'I'
    }
}