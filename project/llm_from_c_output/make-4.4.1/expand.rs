use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::mem;
use std::slice;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static VARIABLE_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(200));
    static VARIABLE_BUFFER_LENGTH: RefCell<usize> = RefCell::new(200);
    static EXPANDING_VAR: RefCell<*const floc> = RefCell::new(ptr::null());
}

const VARIABLE_BUFFER_ZONE: usize = 5;

#[derive(Debug, Clone, Copy)]
struct floc {
    filenm: *const c_char,
    lineno: usize,
}

struct Variable {
    name: CString,
    value: CString,
    recursive: bool,
    append: bool,
    expanding: bool,
    exp_count: usize,
    fileinfo: floc,
    private_var: bool,
}

struct VariableSetList {
    set: *const c_void,
    next: *const VariableSetList,
    next_is_parent: bool,
}

struct File {
    variables: *const VariableSetList,
    cmds: *const c_void,
}

fn variable_buffer_output(ptr: usize, string: &[u8]) -> usize {
    VARIABLE_BUFFER.with(|buf| {
        let mut buf = buf.borrow_mut();
        let newlen = ptr + string.len();
        
        if newlen + VARIABLE_BUFFER_ZONE > buf.capacity() {
            let new_cap = if newlen + 100 > 2 * buf.capacity() {
                newlen + 100
            } else {
                2 * buf.capacity()
            };
            buf.reserve(new_cap);
            VARIABLE_BUFFER_LENGTH.with(|len| *len.borrow_mut() = new_cap);
        }
        
        let start = ptr;
        let end = start + string.len();
        if end > buf.len() {
            buf.resize(end, 0);
        }
        buf[start..end].copy_from_slice(string);
        end
    })
}

fn initialize_variable_output() -> usize {
    VARIABLE_BUFFER.with(|buf| {
        let mut buf = buf.borrow_mut();
        if buf.is_empty() {
            buf.resize(200, 0);
            buf[0] = b'\0';
            VARIABLE_BUFFER_LENGTH.with(|len| *len.borrow_mut() = 200);
        }
        0
    })
}

fn recursively_expand_for_file(v: &Variable, file: Option<&File>) -> CString {
    let saved_varp = EXPANDING_VAR.with(|var| *var.borrow());
    let this_var = if !v.fileinfo.filenm.is_null() {
        &v.fileinfo
    } else {
        ptr::null()
    };
    
    if !this_var.is_null() {
        EXPANDING_VAR.with(|var| *var.borrow_mut() = this_var);
    }
    
    let mut set_reading = false;
    unsafe {
        if READING_FILE.is_null() && !v.fileinfo.filenm.is_null() {
            set_reading = true;
            READING_FILE = &v.fileinfo;
        }
    }
    
    if v.expanding {
        if v.exp_count == 0 {
            panic!("Recursive variable '{}' references itself", v.name.to_str().unwrap());
        }
    }
    
    let save = if let Some(f) = file {
        unsafe { CURRENT_VARIABLE_SET_LIST = f.variables };
        Some(unsafe { CURRENT_VARIABLE_SET_LIST })
    } else {
        None
    };
    
    let value = if v.append {
        allocated_variable_append(v)
    } else {
        allocated_variable_expand(&v.value)
    };
    
    if set_reading {
        unsafe { READING_FILE = ptr::null(); }
    }
    
    if let Some(s) = save {
        unsafe { CURRENT_VARIABLE_SET_LIST = s; }
    }
    
    EXPANDING_VAR.with(|var| *var.borrow_mut() = saved_varp);
    value
}

fn reference_variable(o: usize, name: &str) -> usize {
    let v = lookup_variable(name);
    
    if v.is_none() {
        warn_undefined(name);
        return o;
    }
    
    let v = v.unwrap();
    if v.value.is_empty() && !v.append {
        return o;
    }
    
    let value = if v.recursive {
        recursively_expand(&v)
    } else {
        v.value.clone()
    };
    
    let new_o = variable_buffer_output(o, value.to_bytes());
    
    if v.recursive {
        // value is dropped here
    }
    
    new_o
}

fn variable_expand_string(line: Option<usize>, string: &str, length: Option<usize>) -> CString {
    let mut o = line.unwrap_or_else(initialize_variable_output);
    let line_offset = o;
    
    if let Some(len) = length {
        if len == 0 {
            variable_buffer_output(o, b"");
            return unsafe { CStr::from_ptr(VARIABLE_BUFFER.with(|buf| buf.borrow().as_ptr() as *const c_char)).to_owned() };
        }
    }
    
    let save = if let Some(len) = length {
        if len == usize::MAX {
            CString::new(string).unwrap()
        } else {
            CString::new(&string[..len]).unwrap()
        }
    } else {
        CString::new(string).unwrap()
    };
    
    let mut p = save.to_bytes();
    
    loop {
        let p1 = memchr::memchr(b'$', p);
        
        let copy_len = if let Some(pos) = p1 {
            pos
        } else {
            p.len()
        };
        
        o = variable_buffer_output(o, &p[..copy_len]);
        
        if p1.is_none() {
            break;
        }
        
        p = &p[p1.unwrap() + 1..];
        
        match p.get(0) {
            Some(b'$') | None => {
                o = variable_buffer_output(o, b"$");
            },
            Some(b'(') | Some(b'{') => {
                // Handle complex variable references
                // ... (implementation omitted for brevity)
            },
            Some(_) => {
                if p.len() > 0 && !p[0].is_ascii_whitespace() {
                    o = reference_variable(o, std::str::from_utf8(&p[..1]).unwrap());
                }
            }
        }
        
        if p.is_empty() {
            break;
        }
        p = &p[1..];
    }
    
    variable_buffer_output(o, b"");
    unsafe { CStr::from_ptr(VARIABLE_BUFFER.with(|buf| buf.borrow().as_ptr().add(line_offset) as *const c_char).to_owned() }
}

fn variable_expand(line: &str) -> CString {
    variable_expand_string(None, line, None)
}

fn expand_argument(str: &str, end: Option<&str>) -> CString {
    if str.is_empty() || end.map(|e| e.is_empty()).unwrap_or(false) {
        return CString::new("").unwrap();
    }
    
    if end.is_none() {
        return allocated_variable_expand(str);
    }
    
    let tmp = if let Some(e) = end {
        if str.len() + e.len() + 1 > 1000 {
            let mut s = str.to_string();
            s.push_str(e);
            CString::new(s).unwrap()
        } else {
            let mut buf = [0u8; 1000];
            let len = str.len() + e.len();
            buf[..str.len()].copy_from_slice(str.as_bytes());
            buf[str.len()..len].copy_from_slice(e.as_bytes());
            CString::new(&buf[..len]).unwrap()
        }
    } else {
        CString::new(str).unwrap()
    };
    
    let r = allocated_variable_expand(tmp.to_str().unwrap());
    r
}

fn variable_expand_for_file(line: &str, file: Option<&File>) -> CString {
    let savev = unsafe { CURRENT_VARIABLE_SET_LIST };
    let savef = unsafe { READING_FILE };
    
    if let Some(f) = file {
        unsafe { CURRENT_VARIABLE_SET_LIST = f.variables };
        unsafe { READING_FILE = if !f.cmds.is_null() && !(*f.cmds).fileinfo.filenm.is_null() {
            &(*f.cmds).fileinfo
        } else {
            ptr::null()
        }};
    }
    
    let result = variable_expand(line);
    
    unsafe { CURRENT_VARIABLE_SET_LIST = savev };
    unsafe { READING_FILE = savef };
    
    result
}

fn variable_append(name: &str, length: usize, set: Option<&VariableSetList>, local: bool) -> CString {
    if set.is_none() {
        initialize_variable_output();
        return unsafe { CStr::from_ptr(VARIABLE_BUFFER.with(|buf| buf.borrow().as_ptr() as *const c_char)).to_owned() };
    }
    
    let set = set.unwrap();
    let nextlocal = local && !set.next_is_parent;
    
    let v = lookup_variable_in_set(name, length, set.set);
    
    if v.is_none() || (!local && v.as_ref().unwrap().private_var) {
        return variable_append(name, length, unsafe { set.next.as_ref() }, nextlocal);
    }
    
    let v = v.unwrap();
    let mut buf = if v.append {
        variable_append(name, length, unsafe { set.next.as_ref() }, nextlocal)
    } else {
        initialize_variable_output();
        unsafe { CStr::from_ptr(VARIABLE_BUFFER.with(|buf| buf.borrow().as_ptr() as *const c_char)).to_owned() }
    };
    
    if buf.to_bytes().len() > 0 {
        buf = CString::new([buf.to_bytes(), b" "].concat()).unwrap();
    }
    
    if !v.recursive {
        CString::new([buf.to_bytes(), v.value.to_bytes()].concat()).unwrap()
    } else {
        let expanded = variable_expand_string(Some(buf.to_bytes().len()), v.value.to_str().unwrap(), None);
        CString::new(expanded.to_bytes()).unwrap()
    }
}

fn allocated_variable_append(v: &Variable) -> CString {
    let obuf = VARIABLE_BUFFER.with(|buf| buf.borrow().clone());
    let olen = VARIABLE_BUFFER_LENGTH.with(|len| *len.borrow());
    
    VARIABLE_BUFFER.with(|buf| buf.borrow_mut().clear());
    
    let val = variable_append(v.name.to_str().unwrap(), v.name.to_bytes().len(), 
                            unsafe { CURRENT_VARIABLE_SET_LIST.as_ref() }, true);
    variable_buffer_output(val.to_bytes().len(), b"");
    let result = unsafe { CStr::from_ptr(VARIABLE_BUFFER.with(|buf| buf.borrow().as_ptr() as *const c_char).to_owned() };
    
    VARIABLE_BUFFER.with(|buf| *buf.borrow_mut() = obuf);
    VARIABLE_BUFFER_LENGTH.with(|len| *len.borrow_mut() = olen);
    
    result
}

fn allocated_variable_expand_for_file(line: &str, file: Option<&File>) -> CString {
    let obuf = VARIABLE_BUFFER.with(|buf| buf.borrow().clone());
    let olen = VARIABLE_BUFFER_LENGTH.with(|len| *len.borrow());
    
    VARIABLE_BUFFER.with(|buf| buf.borrow_mut().clear());
    
    let value = variable_expand_for_file(line, file);
    
    VARIABLE_BUFFER.with(|buf| *buf.borrow_mut() = obuf);
    VARIABLE_BUFFER_LENGTH.with(|len| *len.borrow_mut() = olen);
    
    value
}

fn install_variable_buffer() -> (Vec<u8>, usize) {
    let buf = VARIABLE_BUFFER.with(|b| b.borrow().clone());
    let len = VARIABLE_BUFFER_LENGTH.with(|l| *l.borrow());
    
    VARIABLE_BUFFER.with(|b| b.borrow_mut().clear());
    initialize_variable_output();
    
    (buf, len)
}

fn restore_variable_buffer(buf: Vec<u8>, len: usize) {
    VARIABLE_BUFFER.with(|b| *b.borrow_mut() = buf);
    VARIABLE_BUFFER_LENGTH.with(|l| *l.borrow_mut() = len);
}

static mut CURRENT_VARIABLE_SET_LIST: *const VariableSetList = ptr::null();
static mut READING_FILE: *const floc = ptr::null();

fn lookup_variable(name: &str) -> Option<Variable> { None }
fn warn_undefined(name: &str) {}
fn allocated_variable_expand(s: &str) -> CString { CString::new(s).unwrap() }
fn recursively_expand(v: &Variable) -> CString { v.value.clone() }
fn lookup_variable_in_set(name: &str, length: usize, set: *const c_void) -> Option<Variable> { None }