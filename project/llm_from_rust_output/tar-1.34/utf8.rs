use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::{c_char, c_void};
use libc::{size_t, c_ulong, c_int};
use encoding_rs::Encoding;

type IconvT = *mut c_void;

struct CharsetConverter {
    to_utf8: Option<IconvT>,
    from_utf8: Option<IconvT>,
}

impl CharsetConverter {
    fn new() -> Self {
        CharsetConverter {
            to_utf8: None,
            from_utf8: None,
        }
    }

    fn init(&mut self, to_utf: bool) -> Option<IconvT> {
        unsafe {
            let desc = if to_utf { &mut self.to_utf8 } else { &mut self.from_utf8 };
            if desc.is_none() {
                let from = if to_utf {
                    CStr::from_ptr(locale_charset())
                } else {
                    CStr::from_bytes_with_nul(b"UTF-8\0").unwrap()
                };
                let to = if to_utf {
                    CStr::from_bytes_with_nul(b"UTF-8\0").unwrap()
                } else {
                    CStr::from_ptr(locale_charset())
                };
                let cd = iconv_open(to.as_ptr(), from.as_ptr());
                if cd == (-1 as c_int) as IconvT {
                    None
                } else {
                    *desc = Some(cd);
                    Some(cd)
                }
            } else {
                *desc
            }
        }
    }

    fn convert(&mut self, to_utf: bool, input: &CStr) -> Option<CString> {
        let cd = self.init(to_utf)?;
        
        let input_len = input.to_bytes().len() + 1;
        let mut out_len = input_len * 16 + 1;
        let mut output = vec![0u8; out_len];

        unsafe {
            let mut in_ptr = input.as_ptr() as *mut c_char;
            let mut out_ptr = output.as_mut_ptr() as *mut c_char;
            let mut in_bytes = input_len as size_t;
            let mut out_bytes = out_len as size_t;

            if iconv(cd, &mut in_ptr, &mut in_bytes, &mut out_ptr, &mut out_bytes) == 0 {
                output.truncate(out_len - out_bytes as usize);
                CString::new(output).ok()
            } else {
                None
            }
        }
    }
}

impl Drop for CharsetConverter {
    fn drop(&mut self) {
        unsafe {
            if let Some(cd) = self.to_utf8 {
                libc::free(cd as *mut c_void);
            }
            if let Some(cd) = self.from_utf8 {
                libc::free(cd as *mut c_void);
            }
        }
    }
}

fn utf8_convert(to_utf: bool, input: &CStr) -> Option<CString> {
    let mut converter = CharsetConverter::new();
    converter.convert(to_utf, input)
}

fn string_ascii_p(input: &CStr) -> bool {
    input.to_bytes().iter().all(|&c| c & 0x80 == 0)
}

extern "C" {
    fn locale_charset() -> *const c_char;
    fn iconv_open(tocode: *const c_char, fromcode: *const c_char) -> IconvT;
    fn iconv(
        cd: IconvT,
        inbuf: *mut *mut c_char,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut c_char,
        outbytesleft: *mut size_t,
    ) -> size_t;
}