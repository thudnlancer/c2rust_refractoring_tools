use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::str;
use std::fmt;
use std::error::Error;
use base64::encode as base64_encode;
use md5::{Md5, Digest};
use url::Url;
use httparse::{Request, Response, Status};

#[derive(Debug)]
pub enum HttpError {
    ConnectionFailed,
    InvalidResponse,
    IoError(io::Error),
    // 其他错误类型...
}

impl From<io::Error> for HttpError {
    fn from(err: io::Error) -> Self {
        HttpError::IoError(err)
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpError::ConnectionFailed => write!(f, "Connection failed"),
            HttpError::InvalidResponse => write!(f, "Invalid HTTP response"),
            HttpError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for HttpError {}

pub struct HttpClient {
    user_agent: String,
    timeout: u64,
    persistent: bool,
    cookies: HashMap<String, String>,
    auth: Option<(String, String)>,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            user_agent: "rust-http-client/1.0".to_string(),
            timeout: 30,
            persistent: true,
            cookies: HashMap::new(),
            auth: None,
        }
    }

    pub fn set_user_agent(&mut self, ua: &str) {
        self.user_agent = ua.to_string();
    }

    pub fn set_auth(&mut self, username: &str, password: &str) {
        self.auth = Some((username.to_string(), password.to_string()));
    }

    pub fn get(&self, url: &str) -> Result<HttpResponse, HttpError> {
        let url = Url::parse(url).map_err(|_| HttpError::InvalidResponse)?;
        let host = url.host_str().ok_or(HttpError::InvalidResponse)?;
        let port = url.port_or_known_default().ok_or(HttpError::InvalidResponse)?;
        
        let addr = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(addr)?;
        
        let path = if url.path().is_empty() { "/" } else { url.path() };
        let query = url.query().unwrap_or("");
        
        let mut request = format!(
            "GET {}?{} HTTP/1.1\r\n\
             Host: {}\r\n\
             User-Agent: {}\r\n",
            path, query, host, self.user_agent
        );
        
        if let Some((user, pass)) = &self.auth {
            let auth = format!("{}:{}", user, pass);
            request.push_str(&format!("Authorization: Basic {}\r\n", base64_encode(auth)));
        }
        
        request.push_str("\r\n");
        stream.write_all(request.as_bytes())?;
        
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer)?;
        
        let response = parse_http_response(&buffer)?;
        Ok(response)
    }
}

pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

fn parse_http_response(data: &[u8]) -> Result<HttpResponse, HttpError> {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut resp = httparse::Response::new(&mut headers);
    
    let status = resp.parse(data).map_err(|_| HttpError::InvalidResponse)?;
    
    if status.is_partial() {
        return Err(HttpError::InvalidResponse);
    }
    
    let status_code = resp.code.ok_or(HttpError::InvalidResponse)?;
    let mut header_map = HashMap::new();
    
    for header in resp.headers {
        header_map.insert(
            header.name.to_string(),
            str::from_utf8(header.value).unwrap_or("").to_string()
        );
    }
    
    let body_start = status.unwrap();
    let body = data[body_start..].to_vec();
    
    Ok(HttpResponse {
        status_code,
        headers: header_map,
        body,
    })
}

pub fn basic_auth_encode(username: &str, password: &str) -> String {
    let creds = format!("{}:{}", username, password);
    format!("Basic {}", base64_encode(creds))
}

pub fn md5_hash(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

// 其他HTTP相关功能...