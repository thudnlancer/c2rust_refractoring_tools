use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::cmp;
use std::fmt;
use std::str;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UrlScheme {
    Http,
    Https,
    Ftp,
    Ftps,
    Invalid,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UrlAuthMode {
    Show,
    HidePasswd,
    Hide,
}

#[derive(Debug)]
pub struct Url {
    pub url: String,
    pub scheme: UrlScheme,
    pub host: String,
    pub port: i32,
    pub path: String,
    pub params: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
    pub dir: String,
    pub file: String,
    pub user: Option<String>,
    pub passwd: Option<String>,
}

impl Url {
    pub fn new() -> Self {
        Url {
            url: String::new(),
            scheme: UrlScheme::Invalid,
            host: String::new(),
            port: 0,
            path: String::new(),
            params: None,
            query: None,
            fragment: None,
            dir: String::new(),
            file: String::new(),
            user: None,
            passwd: None,
        }
    }

    pub fn parse(url_str: &str) -> Result<Self, String> {
        // Simplified URL parsing logic
        let mut url = Url::new();
        url.url = url_str.to_string();
        
        // Extract scheme
        if url_str.starts_with("http://") {
            url.scheme = UrlScheme::Http;
            url.port = 80;
        } else if url_str.starts_with("https://") {
            url.scheme = UrlScheme::Https;
            url.port = 443;
        } else if url_str.starts_with("ftp://") {
            url.scheme = UrlScheme::Ftp;
            url.port = 21;
        } else if url_str.starts_with("ftps://") {
            url.scheme = UrlScheme::Ftps;
            url.port = 990;
        } else {
            return Err("Unsupported scheme".to_string());
        }

        // Extract host and path
        let rest = if url.scheme == UrlScheme::Http {
            &url_str[7..]
        } else if url.scheme == UrlScheme::Https {
            &url_str[8..]
        } else if url.scheme == UrlScheme::Ftp {
            &url_str[6..]
        } else {
            &url_str[7..]
        };

        let path_start = rest.find('/').unwrap_or(rest.len());
        url.host = rest[..path_start].to_string();
        
        if path_start < rest.len() {
            url.path = rest[path_start..].to_string();
        } else {
            url.path = "/".to_string();
        }

        // Split path into dir/file
        if let Some(last_slash) = url.path.rfind('/') {
            url.dir = url.path[..last_slash + 1].to_string();
            url.file = url.path[last_slash + 1..].to_string();
        } else {
            url.dir = "".to_string();
            url.file = url.path.clone();
        }

        Ok(url)
    }

    pub fn full_path(&self) -> String {
        let mut path = self.path.clone();
        if let Some(ref params) = self.params {
            path.push(';');
            path.push_str(params);
        }
        if let Some(ref query) = self.query {
            path.push('?');
            path.push_str(query);
        }
        path
    }

    pub fn to_string(&self, auth_mode: UrlAuthMode) -> String {
        let mut s = String::new();
        
        match self.scheme {
            UrlScheme::Http => s.push_str("http://"),
            UrlScheme::Https => s.push_str("https://"),
            UrlScheme::Ftp => s.push_str("ftp://"),
            UrlScheme::Ftps => s.push_str("ftps://"),
            UrlScheme::Invalid => {}
        }

        if let Some(ref user) = self.user {
            s.push_str(user);
            if let UrlAuthMode::Show = auth_mode {
                if let Some(ref passwd) = self.passwd {
                    s.push(':');
                    s.push_str(passwd);
                }
            } else if let UrlAuthMode::HidePasswd = auth_mode {
                s.push_str(":*password*");
            }
            s.push('@');
        }

        s.push_str(&self.host);

        let default_port = match self.scheme {
            UrlScheme::Http => 80,
            UrlScheme::Https => 443,
            UrlScheme::Ftp => 21,
            UrlScheme::Ftps => 990,
            UrlScheme::Invalid => 0,
        };

        if self.port != default_port {
            s.push(':');
            s.push_str(&self.port.to_string());
        }

        s.push_str(&self.full_path());

        s
    }
}

pub fn url_escape(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_ascii_alphanumeric() || "-_.~".contains(c) {
            result.push(c);
        } else {
            result.push_str(&format!("%{:02X}", c as u8));
        }
    }
    result
}

pub fn url_unescape(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().collect::<Vec<_>>();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '%' && i + 2 < chars.len() {
            if let Ok(byte) = u8::from_str_radix(&chars[i+1..i+3].iter().collect::<String>(), 16) {
                result.push(byte as char);
                i += 3;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

pub fn uri_merge(base: &str, link: &str) -> String {
    if link.starts_with("http://") || link.starts_with("https://") ||
       link.starts_with("ftp://") || link.starts_with("ftps://") {
        return link.to_string();
    }

    let base_url = match Url::parse(base) {
        Ok(u) => u,
        Err(_) => return link.to_string(),
    };

    if link.is_empty() {
        return base.to_string();
    }

    if link.starts_with('?') {
        return format!("{}{}", base_url.path, link);
    }

    if link.starts_with('#') {
        if let Some(pos) = base.find('#') {
            return format!("{}{}", &base[..pos], link);
        } else {
            return format!("{}{}", base, link);
        }
    }

    if link.starts_with("//") {
        let scheme = match base_url.scheme {
            UrlScheme::Http => "http:",
            UrlScheme::Https => "https:",
            UrlScheme::Ftp => "ftp:",
            UrlScheme::Ftps => "ftps:",
            UrlScheme::Invalid => "",
        };
        return format!("{}{}", scheme, link);
    }

    if link.starts_with('/') {
        return format!("{}://{}{}", 
            match base_url.scheme {
                UrlScheme::Http => "http",
                UrlScheme::Https => "https",
                UrlScheme::Ftp => "ftp",
                UrlScheme::Ftps => "ftps",
                UrlScheme::Invalid => "",
            },
            base_url.host,
            link
        );
    }

    let mut path = base_url.path.clone();
    if !path.ends_with('/') {
        if let Some(pos) = path.rfind('/') {
            path = path[..pos+1].to_string();
        } else {
            path = "/".to_string();
        }
    }
    format!("{}://{}{}{}", 
        match base_url.scheme {
            UrlScheme::Http => "http",
            UrlScheme::Https => "https",
            UrlScheme::Ftp => "ftp",
            UrlScheme::Ftps => "ftps",
            UrlScheme::Invalid => "",
        },
        base_url.host,
        path,
        link
    )
}

pub fn schemes_are_similar(a: UrlScheme, b: UrlScheme) -> bool {
    a == b || 
    (a == UrlScheme::Http && b == UrlScheme::Https) ||
    (a == UrlScheme::Https && b == UrlScheme::Http)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_parse() {
        let url = Url::parse("http://example.com/path/to/file").unwrap();
        assert_eq!(url.scheme, UrlScheme::Http);
        assert_eq!(url.host, "example.com");
        assert_eq!(url.path, "/path/to/file");
        assert_eq!(url.dir, "/path/to/");
        assert_eq!(url.file, "file");
    }

    #[test]
    fn test_url_to_string() {
        let mut url = Url::new();
        url.scheme = UrlScheme::Http;
        url.host = "example.com".to_string();
        url.path = "/path".to_string();
        assert_eq!(url.to_string(UrlAuthMode::Show), "http://example.com/path");
    }

    #[test]
    fn test_url_escape() {
        assert_eq!(url_escape("test string"), "test%20string");
        assert_eq!(url_escape("abc123"), "abc123");
    }

    #[test]
    fn test_url_unescape() {
        assert_eq!(url_unescape("test%20string"), "test string");
        assert_eq!(url_unescape("abc123"), "abc123");
    }

    #[test]
    fn test_uri_merge() {
        assert_eq!(
            uri_merge("http://example.com/dir/", "file"), 
            "http://example.com/dir/file"
        );
        assert_eq!(
            uri_merge("http://example.com/dir/", "../file"), 
            "http://example.com/file"
        );
    }
}