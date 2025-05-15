use std::ffi::{CStr, CString};
use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::slice;
use libc::{size_t, wchar_t, iconv_t, mbstate_t, nl_item, iconv, iconv_open, iconv_close, mbrtowc, wcsnlen};
use libc::{calloc, free, memset, memcmp, strcpy, strcat, strlen, strerror, sprintf, fprintf, stderr, exit, __errno_location};
use libc::nl_langinfo;

const CODESET: nl_item = 14;

static mut wcharCp: *const c_char = ptr::null();
static wcharTries: [&[u8]; 13] = [
    b"WCHAR_T\0",
    b"UTF-32BE\0",
    b"UTF-32LE\0", 
    b"UTF-16BE\0",
    b"UTF-16LE\0",
    b"UTF-32\0",
    b"UTF-16\0",
    b"UCS-4BE\0",
    b"UCS-4LE\0",
    b"UCS-2BE\0",
    b"UCS-2LE\0",
    b"UCS-4\0",
    b"UCS-2\0",
];

static asciiTries: [&[u8]; 3] = [
    b"ASCII\0",
    b"ASCII-GR\0",
    b"ISO8859-1\0",
];

static testString: [wchar_t; 3] = [b'a' as wchar_t, b'b' as wchar_t, 0];

struct DosCp {
    from: iconv_t,
    to: iconv_t,
}

unsafe fn try_cp(testCp: *const c_char) -> c_int {
    let mut inbuf = testString.as_ptr() as *mut c_char;
    let mut inbuf_len = 2 * mem::size_of::<wchar_t>();
    let mut outbuf = [0u8; 3];
    let mut outbuf_ptr = outbuf.as_mut_ptr();
    let mut outbuf_len = 2;
    
    for &try_cp in &asciiTries {
        let test = iconv_open(CStr::from_bytes_with_nul(try_cp).unwrap().as_ptr(), testCp);
        if test != -1 as iconv_t {
            let res = iconv(
                test,
                &mut inbuf,
                &mut inbuf_len,
                &mut outbuf_ptr,
                &mut outbuf_len,
            );
            iconv_close(test);
            
            if res != -1 as size_t && outbuf_len == 0 && inbuf_len == 0 {
                if memcmp(outbuf.as_ptr() as *const c_void, b"ab\0".as_ptr() as *const c_void, 2) == 0 {
                    return 1;
                }
            }
        }
    }
    0
}

unsafe fn get_wchar_cp() -> *const c_char {
    if !wcharCp.is_null() {
        return wcharCp;
    }
    
    for &try_cp in &wcharTries {
        if try_cp(CStr::from_bytes_with_nul(try_cp).unwrap().as_ptr()) != 0 {
            wcharCp = try_cp.as_ptr() as *const c_char;
            return wcharCp;
        }
    }
    
    fprintf(stderr, b"No codepage found for wchar_t\n\0".as_ptr() as *const c_char);
    ptr::null()
}

pub unsafe fn cp_open(codepage: c_uint) -> *mut DosCp {
    let mut dos_cp = [0u8; 17];
    let mut ret: *mut DosCp = ptr::null_mut();
    
    if codepage == 0 {
        codepage = mtools_default_codepage;
    }
    
    if codepage > 9999 {
        fprintf(stderr, b"Bad codepage %d\n\0".as_ptr() as *const c_char, codepage);
        return ptr::null_mut();
    }
    
    if get_wchar_cp().is_null() {
        return ptr::null_mut();
    }
    
    sprintf(dos_cp.as_mut_ptr(), b"CP%d\0".as_ptr() as *const c_char, codepage);
    let from = iconv_open(wcharCp, dos_cp.as_ptr());
    
    if from == -1 as iconv_t {
        fprintf(
            stderr,
            b"Error converting to codepage %d %s\n\0".as_ptr() as *const c_char,
            codepage,
            strerror(*__errno_location()),
        );
        return ptr::null_mut();
    }
    
    sprintf(dos_cp.as_mut_ptr(), b"CP%d//TRANSLIT\0".as_ptr() as *const c_char, codepage);
    let mut to = iconv_open(dos_cp.as_ptr(), wcharCp);
    
    if to == -1 as iconv_t {
        sprintf(dos_cp.as_mut_ptr(), b"CP%d\0".as_ptr() as *const c_char, codepage);
        to = iconv_open(dos_cp.as_ptr(), wcharCp);
    }
    
    if to == -1 as iconv_t {
        iconv_close(from);
        fprintf(
            stderr,
            b"Error converting to codepage %d %s\n\0".as_ptr() as *const c_char,
            codepage,
            strerror(*__errno_location()),
        );
        return ptr::null_mut();
    }
    
    ret = calloc(1, mem::size_of::<DosCp>()) as *mut DosCp;
    if ret.is_null() {
        return ret;
    }
    
    (*ret).from = from;
    (*ret).to = to;
    ret
}

pub unsafe fn cp_close(cp: *mut DosCp) {
    iconv_close((*cp).to);
    iconv_close((*cp).from);
    free(cp as *mut c_void);
}

pub unsafe fn dos_to_wchar(
    cp: *mut DosCp,
    dos: *const c_char,
    wchar: *mut wchar_t,
    len: size_t,
) -> size_t {
    let mut in_len = len;
    let mut out_len = len * mem::size_of::<wchar_t>();
    let mut dptr = wchar;
    let mut dos2 = dos as *mut c_char;
    
    let r = iconv(
        (*cp).from,
        &mut dos2,
        &mut in_len,
        &mut (dptr as *mut c_char),
        &mut out_len,
    );
    
    if r == -1 as size_t {
        return r;
    }
    
    *dptr = 0;
    dptr.offset_from(wchar) as size_t
}

unsafe fn safe_iconv(
    conv: iconv_t,
    wchar: *const wchar_t,
    dest: *mut c_char,
    in_len: size_t,
    out_len: size_t,
    mangled: *mut c_int,
) -> size_t {
    let mut in_len = in_len * mem::size_of::<wchar_t>();
    let mut dptr = dest;
    let mut wptr = wchar;
    
    while in_len > 0 && out_len > 0 {
        let r = iconv(
            conv,
            &mut (wptr as *mut c_char),
            &mut in_len,
            &mut dptr,
            &mut out_len,
        );
        
        if r == -1 as size_t && *__errno_location() != 84 {
            break;
        }
        
        if r == -1 as size_t {
            *mangled |= 1;
            if out_len == 0 {
                break;
            }
            *dptr = b'_' as c_char;
            dptr = dptr.add(1);
            in_len -= mem::size_of::<wchar_t>();
            wptr = wptr.add(1);
            out_len -= 1;
        }
    }
    
    let len = dptr.offset_from(dest) as size_t;
    
    for i in 0..len {
        if *dest.add(i) == b'?' as c_char {
            *dest.add(i) = b'_' as c_char;
            *mangled |= 1;
        }
    }
    
    len
}

pub unsafe fn wchar_to_dos(
    cp: *mut DosCp,
    wchar: *mut wchar_t,
    dos: *mut c_char,
    len: size_t,
    mangled: *mut c_int,
) {
    safe_iconv((*cp).to, wchar, dos, len, len, mangled);
}

static mut to_native: iconv_t = ptr::null_mut();

unsafe fn initialize_to_native() {
    if !to_native.is_null() {
        return;
    }
    
    let li = nl_langinfo(CODESET);
    let len = strlen(li) + 11;
    
    if get_wchar_cp().is_null() {
        exit(1);
    }
    
    let mut cp = CString::new(vec![0u8; len as usize]).unwrap();
    strcpy(cp.as_ptr(), li);
    strcat(cp.as_ptr(), b"//TRANSLIT\0".as_ptr() as *const c_char);
    
    to_native = iconv_open(cp.as_ptr(), wcharCp);
    if to_native == -1 as iconv_t {
        to_native = iconv_open(li, wcharCp);
    }
    
    if to_native == -1 as iconv_t {
        fprintf(
            stderr,
            b"Could not allocate iconv for %s\n\0".as_ptr() as *const c_char,
            cp.as_ptr(),
        );
        exit(1);
    }
}

pub unsafe fn wchar_to_native(
    wchar: *const wchar_t,
    native: *mut c_char,
    len: size_t,
    out_len: size_t,
) -> size_t {
    let mut mangled = 0;
    initialize_to_native();
    let len = wcsnlen(wchar, len);
    let r = safe_iconv(to_native, wchar, native, len, out_len, &mut mangled);
    *native.add(r) = 0;
    r
}

pub unsafe fn native_to_wchar(
    native: *const c_char,
    wchar: *mut wchar_t,
    len: size_t,
    end: *const c_char,
    mangled: *mut c_int,
) -> size_t {
    let mut ps = mbstate_t {
        __count: 0,
        __value: std::mem::zeroed(),
    };
    
    memset(&mut ps as *mut _ as *mut c_void, 0, mem::size_of::<mbstate_t>());
    
    let mut i = 0;
    let mut nptr = native;
    
    while i < len && (nptr < end || end.is_null()) {
        let mut wc = 0;
        let r = mbrtowc(&mut wc, nptr, len - i, &mut ps);
        
        if r == -1 as size_t {
            let c = *nptr;
            if c >= -96 && c < -1 {
                *wchar.add(i) = c as c_int & 0xff;
            } else {
                *wchar.add(i) = b'_' as c_int;
            }
            memset(&mut ps as *mut _ as *mut c_void, 0, mem::size_of::<mbstate_t>());
            nptr = nptr.add(1);
            i += 1;
            continue;
        }
        
        if r == 0 {
            break;
        }
        
        *wchar.add(i) = wc;
        nptr = nptr.add(r);
        i += 1;
    }
    
    if !mangled.is_null() && ((!end.is_null() && nptr < end) || (end.is_null() && *nptr != 0 && i == len)) {
        *mangled |= 3;
    }
    
    *wchar.add(i) = 0;
    i
}