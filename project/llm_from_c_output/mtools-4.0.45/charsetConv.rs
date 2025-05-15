/*
 * Copyright 2008,2009 Alain Knaff.
 * This file is part of mtools.
 *
 * Mtools is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Mtools is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 *
 * Various character set conversions used by mtools
 */

use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_void};
use std::slice;
use std::str;
use std::io::{Error, ErrorKind};
use libc::{iconv_t, iconv, iconv_open, iconv_close, size_t};
use encoding_rs::*;

#[cfg(feature = "iconv")]
use libc::{EILSEQ, errno};

struct DosCp {
    from: iconv_t,
    to: iconv_t,
}

static WCHAR_CP: &str = "WCHAR_T";
static WCHAR_TRIES: &[&str] = &[
    "WCHAR_T",
    "UTF-32BE", "UTF-32LE",
    "UTF-16BE", "UTF-16LE",
    "UTF-32", "UTF-16",
    "UCS-4BE", "UCS-4LE",
    "UCS-2BE", "UCS-2LE",
    "UCS-4", "UCS-2"
];

static ASCII_TRIES: &[&str] = &[
    "ASCII", "ASCII-GR", "ISO8859-1"
];

static TEST_STRING: &str = "ab";

fn try_codepage(test_cp: &str) -> Result<(), Error> {
    let test_string_c = CString::new(TEST_STRING).unwrap();
    let mut inbuf = test_string_c.as_ptr();
    let mut inbuf_len = TEST_STRING.len() as size_t;
    let mut outbuf = [0u8; 3];
    let mut outbuf_ptr = outbuf.as_mut_ptr();
    let mut outbuf_len = outbuf.len() as size_t;
    
    let mut test = None;
    for cp in ASCII_TRIES {
        let to_cp = CString::new(*cp).unwrap();
        let from_cp = CString::new(test_cp).unwrap();
        let cd = unsafe { iconv_open(to_cp.as_ptr(), from_cp.as_ptr()) };
        if cd != -1 as iconv_t {
            test = Some(cd);
            break;
        }
    }
    
    let test = test.ok_or(Error::new(ErrorKind::Other, "No valid conversion"))?;
    
    let res = unsafe {
        iconv(
            test,
            &mut inbuf as *mut *const c_char,
            &mut inbuf_len,
            &mut outbuf_ptr as *mut *mut c_char,
            &mut outbuf_len
        )
    };
    
    unsafe { iconv_close(test) };
    
    if res == -1 as size_t || inbuf_len != 0 || outbuf_len != 0 {
        return Err(Error::new(ErrorKind::Other, "Conversion failed"));
    }
    
    if &outbuf[..2] != b"ab" {
        return Err(Error::new(ErrorKind::Other, "Unexpected output"));
    }
    
    Ok(())
}

fn get_wchar_cp() -> Result<&'static str, Error> {
    for cp in WCHAR_TRIES {
        if try_codepage(cp).is_ok() {
            return Ok(cp);
        }
    }
    Err(Error::new(ErrorKind::Other, "No codepage found for wchar_t"))
}

impl DosCp {
    pub fn new(codepage: u32) -> Result<Self, Error> {
        let codepage = if codepage == 0 {
            mtools_default_codepage()
        } else {
            codepage
        };
        
        if codepage > 9999 {
            return Err(Error::new(ErrorKind::InvalidInput, "Bad codepage"));
        }
        
        let wchar_cp = get_wchar_cp()?;
        let wchar_cp_c = CString::new(wchar_cp).unwrap();
        
        let dos_cp = format!("CP{}", codepage);
        let dos_cp_c = CString::new(dos_cp).unwrap();
        
        let from = unsafe { iconv_open(wchar_cp_c.as_ptr(), dos_cp_c.as_ptr()) };
        if from == -1 as iconv_t {
            return Err(Error::new(ErrorKind::Other, format!(
                "Error converting to codepage {}: {}",
                codepage,
                Error::last_os_error()
            )));
        }
        
        let dos_cp_translit = format!("CP{}//TRANSLIT", codepage);
        let dos_cp_translit_c = CString::new(dos_cp_translit).unwrap();
        
        let to = unsafe { iconv_open(dos_cp_translit_c.as_ptr(), wchar_cp_c.as_ptr()) };
        let to = if to == -1 as iconv_t {
            unsafe { iconv_open(dos_cp_c.as_ptr(), wchar_cp_c.as_ptr()) }
        } else {
            to
        };
        
        if to == -1 as iconv_t {
            unsafe { iconv_close(from) };
            return Err(Error::new(ErrorKind::Other, format!(
                "Error converting from codepage {}: {}",
                codepage,
                Error::last_os_error()
            )));
        }
        
        Ok(Self { from, to })
    }
    
    pub fn dos_to_wchar(&self, dos: &[u8], wchar: &mut [u16]) -> Result<usize, Error> {
        let mut inbuf = dos.as_ptr();
        let mut inbuf_len = dos.len() as size_t;
        let mut outbuf = wchar.as_mut_ptr() as *mut c_char;
        let mut outbuf_len = wchar.len() * mem::size_of::<u16>() as size_t;
        
        let res = unsafe {
            iconv(
                self.from,
                &mut inbuf as *mut *const c_char,
                &mut inbuf_len,
                &mut outbuf as *mut *mut c_char,
                &mut outbuf_len
            )
        };
        
        if res == -1 as size_t {
            return Err(Error::last_os_error());
        }
        
        let converted = (wchar.len() * mem::size_of::<u16>() - outbuf_len as usize) / mem::size_of::<u16>();
        if converted < wchar.len() {
            wchar[converted] = 0;
        }
        
        Ok(converted)
    }
    
    pub fn wchar_to_dos(&self, wchar: &[u16], dos: &mut [u8], mangled: &mut bool) -> Result<usize, Error> {
        self.safe_iconv(self.to, wchar, dos, mangled)
    }
    
    fn safe_iconv(&self, conv: iconv_t, wchar: &[u16], dest: &mut [u8], mangled: &mut bool) -> Result<usize, Error> {
        let mut inbuf = wchar.as_ptr();
        let mut inbuf_len = wchar.len() * mem::size_of::<u16>() as size_t;
        let mut outbuf = dest.as_mut_ptr();
        let mut outbuf_len = dest.len() as size_t;
        let mut total = 0;
        
        while inbuf_len > 0 && outbuf_len > 0 {
            let res = unsafe {
                iconv(
                    conv,
                    &mut inbuf as *mut *const c_char as *mut *mut c_char,
                    &mut inbuf_len,
                    &mut outbuf as *mut *mut c_char,
                    &mut outbuf_len
                )
            };
            
            if res != -1 as size_t {
                break;
            }
            
            let err = Error::last_os_error();
            if err.raw_os_error() != Some(EILSEQ as i32) {
                return Err(err);
            }
            
            *mangled = true;
            if outbuf_len == 0 {
                break;
            }
            
            unsafe {
                *outbuf = b'_';
                outbuf = outbuf.add(1);
                outbuf_len -= 1;
                total += 1;
            }
            
            inbuf_len -= mem::size_of::<u16>() as size_t;
            unsafe { inbuf = inbuf.add(1) };
        }
        
        let converted = dest.len() - outbuf_len as usize;
        
        // Replace question marks with underscores
        for i in 0..converted {
            if dest[i] == b'?' {
                dest[i] = b'_';
                *mangled = true;
            }
        }
        
        Ok(converted)
    }
}

impl Drop for DosCp {
    fn drop(&mut self) {
        unsafe {
            iconv_close(self.to);
            iconv_close(self.from);
        }
    }
}

fn mtools_default_codepage() -> u32 {
    850
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dos_to_wchar() {
        let cp = DosCp::new(850).unwrap();
        let dos = b"Hello";
        let mut wchar = [0u16; 10];
        let len = cp.dos_to_wchar(dos, &mut wchar).unwrap();
        assert_eq!(len, 5);
        assert_eq!(&wchar[..5], &[72u16, 101, 108, 108, 111]);
    }
}