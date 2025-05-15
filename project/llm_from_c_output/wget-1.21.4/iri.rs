use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::c_char;
use std::str;
use std::fmt;
use std::error::Error;
use encoding_rs::{Encoding, UTF_8};
use idna;
use libc::{iconv_t, iconv, iconv_open, iconv_close};
use std::mem;

#[derive(Debug, Clone)]
pub struct Iri {
    uri_encoding: Option<String>,
    content_encoding: Option<String>,
    orig_url: Option<String>,
    utf8_encode: bool,
}

#[derive(Debug)]
pub enum IriError {
    ConversionFailed,
    InvalidEncoding,
    IdnaError(idna::Errors),
    IconvError(String),
}

impl fmt::Display for IriError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IriError::ConversionFailed => write!(f, "Conversion failed"),
            IriError::InvalidEncoding => write!(f, "Invalid encoding"),
            IriError::IdnaError(e) => write!(f, "IDNA error: {:?}", e),
            IriError::IconvError(e) => write!(f, "iconv error: {}", e),
        }
    }
}

impl Error for IriError {}

impl Iri {
    pub fn new() -> Self {
        Iri {
            uri_encoding: None,
            content_encoding: None,
            orig_url: None,
            utf8_encode: true, // Default to true as per opt.enable_iri
        }
    }

    pub fn dup(&self) -> Self {
        Iri {
            uri_encoding: self.uri_encoding.clone(),
            content_encoding: self.content_encoding.clone(),
            orig_url: self.orig_url.clone(),
            utf8_encode: self.utf8_encode,
        }
    }

    pub fn set_uri_encoding(&mut self, charset: Option<&str>, force: bool) {
        if !force && self.uri_encoding.is_some() {
            return;
        }
        self.uri_encoding = charset.map(|s| s.to_string());
    }

    pub fn set_content_encoding(&mut self, charset: Option<&str>) {
        if self.content_encoding.is_some() {
            return;
        }
        self.content_encoding = charset.map(|s| s.to_string());
    }
}

pub fn parse_charset(s: &str) -> Option<String> {
    let s = s.to_lowercase();
    if let Some(pos) = s.find("charset=") {
        let start = pos + 8;
        let end = s[start..].find(|c: char| c.is_whitespace()).unwrap_or(s.len() - start);
        let charset = &s[start..start + end];
        if check_encoding_name(charset) {
            return Some(charset.to_string());
        }
    }
    None
}

pub fn find_locale() -> String {
    unsafe {
        let codeset = libc::nl_langinfo(libc::CODESET);
        if codeset.is_null() {
            return "ASCII".to_string();
        }
        let c_str = CStr::from_ptr(codeset);
        c_str.to_string_lossy().into_owned()
    }
}

pub fn check_encoding_name(encoding: &str) -> bool {
    encoding.chars().all(|c| c.is_ascii() && !c.is_whitespace())
}

#[cfg(feature = "iconv")]
fn do_conversion(
    tocode: &str,
    fromcode: &str,
    input: &str,
) -> Result<String, IriError> {
    unsafe {
        let cd = iconv_open(
            CString::new(tocode).unwrap().as_ptr(),
            CString::new(fromcode).unwrap().as_ptr(),
        );
        if cd == (iconv_t)(-1) {
            return Err(IriError::IconvError("iconv_open failed".to_string()));
        }

        let in_bytes = input.as_bytes();
        let mut in_ptr = in_bytes.as_ptr();
        let mut in_len = in_bytes.len();

        let mut out_len = in_len * 4; // Generous estimate
        let mut out_buf = Vec::with_capacity(out_len);
        let mut out_ptr = out_buf.as_mut_ptr() as *mut c_char;
        let mut out_remaining = out_len;

        loop {
            let result = iconv(
                cd,
                &mut in_ptr,
                &mut in_len,
                &mut out_ptr,
                &mut out_remaining,
            );

            if result != (size_t)(-1) {
                break;
            }

            match errno() {
                libc::E2BIG => {
                    let used = out_len - out_remaining;
                    out_len *= 2;
                    out_buf.reserve(out_len);
                    out_ptr = out_buf.as_mut_ptr().add(used) as *mut c_char;
                    out_remaining = out_len - used;
                }
                libc::EILSEQ | libc::EINVAL => {
                    iconv_close(cd);
                    return Err(IriError::IconvError("Invalid sequence".to_string()));
                }
                _ => {
                    iconv_close(cd);
                    return Err(IriError::IconvError("Unknown error".to_string()));
                }
            }
        }

        iconv_close(cd);
        let used = out_len - out_remaining;
        out_buf.set_len(used);
        Ok(String::from_utf8(out_buf).map_err(|_| IriError::ConversionFailed)?)
    }
}

#[cfg(not(feature = "iconv"))]
fn do_conversion(
    _tocode: &str,
    _fromcode: &str,
    _input: &str,
) -> Result<String, IriError> {
    Err(IriError::IconvError("iconv not available".to_string()))
}

pub fn locale_to_utf8(s: &str, locale: Option<&str>) -> Result<String, IriError> {
    let locale = locale.unwrap_or("UTF-8");
    if locale.eq_ignore_ascii_case("UTF-8") {
        return Ok(s.to_string());
    }
    do_conversion("UTF-8", locale, s)
}

pub fn idn_encode(iri: &Iri, host: &str) -> Result<String, IriError> {
    let src = if iri.utf8_encode {
        host
    } else {
        match remote_to_utf8(iri, host)? {
            Some(s) => &s,
            None => return Ok(host.to_string()),
        }
    };

    let result = idna::Config::default()
        .non_transitional_processing(true)
        .to_ascii(src)
        .map_err(IriError::IdnaError)?;
    Ok(result.0)
}

pub fn idn_decode(host: &str) -> String {
    host.to_string()
}

pub fn remote_to_utf8(iri: &Iri, s: &str) -> Result<Option<String>, IriError> {
    let encoding = match &iri.uri_encoding {
        Some(e) if e.eq_ignore_ascii_case("UTF-8") => {
            if s.chars().any(|c| c > '\x7F') {
                return Ok(Some(s.to_string()));
            }
            return Ok(None);
        }
        Some(e) => e,
        None => return Ok(None),
    };

    do_conversion("UTF-8", encoding, s).map(Some)
}