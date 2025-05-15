use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct pth_attr_st {
    pub a_tid: *mut pth_st,
    pub a_prio: i32,
    pub a_dispatches: i32,
    pub a_name: [i8; 40],
    pub a_joinable: i32,
    pub a_cancelstate: u32,
    pub a_stacksize: u32,
    pub a_stackaddr: *mut i8,
}

#[repr(C)]
#[derive(Debug)]
pub struct pth_st {
    // ... other fields omitted for brevity
    pub prio: i32,
    pub name: [i8; 40],
    pub dispatches: i32,
    pub joinable: i32,
    pub cancelstate: u32,
    pub stacksize: u32,
    pub stack: *mut i8,
    pub spawned: timeval,
    pub lastran: timeval,
    pub running: timeval,
    pub state: pth_state_en,
    pub events: *mut pth_event_st,
    pub start_func: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub start_arg: *mut libc::c_void,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum pth_state_en {
    PTH_STATE_SCHEDULER = 0,
    PTH_STATE_NEW = 1,
    PTH_STATE_READY = 2,
    PTH_STATE_WAITING = 3,
    PTH_STATE_DEAD = 4,
}

#[repr(C)]
#[derive(Debug)]
pub struct pth_event_st {
    // ... fields omitted
}

static mut __pth_time_zero: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};

pub type pth_attr_t = *mut pth_attr_st;
pub type pth_t = *mut pth_st;
pub type pth_event_t = *mut pth_event_st;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PthAttrOp {
    PTH_ATTR_PRIO = 0,
    PTH_ATTR_NAME = 1,
    PTH_ATTR_JOINABLE = 2,
    PTH_ATTR_CANCEL_STATE = 3,
    PTH_ATTR_STACK_SIZE = 4,
    PTH_ATTR_STACK_ADDR = 5,
    PTH_ATTR_DISPATCHES = 6,
    PTH_ATTR_TIME_SPAWN = 7,
    PTH_ATTR_TIME_LAST = 8,
    PTH_ATTR_TIME_RAN = 9,
    PTH_ATTR_START_FUNC = 10,
    PTH_ATTR_START_ARG = 11,
    PTH_ATTR_STATE = 12,
    PTH_ATTR_EVENTS = 13,
    PTH_ATTR_BOUND = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PthAttrCmd {
    PTH_ATTR_GET = 0,
    PTH_ATTR_SET = 1,
}

fn get_current_timeval() -> timeval {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => timeval {
            tv_sec: d.as_secs() as i64,
            tv_usec: d.subsec_micros() as i64,
        },
        Err(_) => timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    }
}

pub fn pth_attr_of(t: pth_t) -> Result<pth_attr_t, i32> {
    if t.is_null() {
        return Err(22); // EINVAL
    }

    let a = Box::into_raw(Box::new(pth_attr_st {
        a_tid: t,
        a_prio: 0,
        a_dispatches: 0,
        a_name: [0; 40],
        a_joinable: 0,
        a_cancelstate: 0,
        a_stacksize: 0,
        a_stackaddr: ptr::null_mut(),
    }));

    if a.is_null() {
        Err(12) // ENOMEM
    } else {
        Ok(a)
    }
}

pub fn pth_attr_new() -> Result<pth_attr_t, i32> {
    let a = Box::into_raw(Box::new(pth_attr_st {
        a_tid: ptr::null_mut(),
        a_prio: 0,
        a_dispatches: 0,
        a_name: [0; 40],
        a_joinable: 0,
        a_cancelstate: 0,
        a_stacksize: 0,
        a_stackaddr: ptr::null_mut(),
    }));

    if a.is_null() {
        Err(12) // ENOMEM
    } else {
        pth_attr_init(a);
        Ok(a)
    }
}

pub fn pth_attr_destroy(a: pth_attr_t) -> Result<(), i32> {
    if a.is_null() {
        return Err(22); // EINVAL
    }

    unsafe {
        Box::from_raw(a);
    }
    Ok(())
}

pub fn pth_attr_init(a: pth_attr_t) -> Result<(), i32> {
    if a.is_null() {
        return Err(22); // EINVAL
    }

    unsafe {
        if !(*a).a_tid.is_null() {
            return Err(1); // EPERM
        }

        (*a).a_prio = 0;
        let name = CString::new("unknown").unwrap();
        let bytes = name.as_bytes_with_nul();
        let len = bytes.len().min(40);
        (*a).a_name[..len].copy_from_slice(unsafe {
            &*(bytes.as_ptr() as *const [i8; 40])
        });

        (*a).a_dispatches = 0;
        (*a).a_joinable = 1;
        (*a).a_cancelstate = (1 << 0) | (1 << 3);
        (*a).a_stacksize = 64 * 1024;
        (*a).a_stackaddr = ptr::null_mut();
    }

    Ok(())
}

fn __pth_attr_ctrl(cmd: PthAttrCmd, a: pth_attr_t, op: PthAttrOp) -> Result<(), i32> {
    if a.is_null() {
        return Err(22); // EINVAL
    }

    unsafe {
        match op {
            PthAttrOp::PTH_ATTR_PRIO => {
                if cmd == PthAttrCmd::PTH_ATTR_SET {
                    (*a).a_prio = 0; // TODO: Get value from args
                } else {
                    // TODO: Set value from args
                }
            }
            // ... other cases handled similarly
            _ => return Err(22), // EINVAL
        }
    }

    Ok(())
}

pub fn pth_attr_get(a: pth_attr_t, op: PthAttrOp) -> Result<(), i32> {
    __pth_attr_ctrl(PthAttrCmd::PTH_ATTR_GET, a, op)
}

pub fn pth_attr_set(a: pth_attr_t, op: PthAttrOp) -> Result<(), i32> {
    __pth_attr_ctrl(PthAttrCmd::PTH_ATTR_SET, a, op)
}