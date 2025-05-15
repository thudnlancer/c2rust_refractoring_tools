use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::os::raw::{c_char, c_int, c_void};
use std::time::{SystemTime, UNIX_EPOCH};

const PTH_TCB_NAMELEN: usize = 32;
const PTH_PRIO_STD: i32 = 0;
const PTH_CANCEL_DEFAULT: u32 = 0;
const TRUE: i32 = 1;
const FALSE: i32 = 0;

enum PthAttrOp {
    Get,
    Set,
}

#[derive(Debug)]
struct PthAttr {
    a_tid: Option<Box<Pth>>,
    a_prio: i32,
    a_dispatches: i32,
    a_name: [c_char; PTH_TCB_NAMELEN],
    a_joinable: i32,
    a_cancelstate: u32,
    a_stacksize: u32,
    a_stackaddr: *mut c_char,
}

#[derive(Debug)]
struct Pth {
    prio: i32,
    name: [c_char; PTH_TCB_NAMELEN],
    dispatches: i32,
    joinable: i32,
    cancelstate: u32,
    stacksize: u32,
    stack: *mut c_char,
    spawned: SystemTime,
    lastran: SystemTime,
    running: SystemTime,
    start_func: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    start_arg: *mut c_void,
    state: i32,
    events: *mut c_void,
}

impl PthAttr {
    fn of(t: Option<Box<Pth>>) -> Result<Box<Self>, i32> {
        if t.is_none() {
            return Err(libc::EINVAL);
        }
        let mut attr = Box::new(PthAttr {
            a_tid: t,
            a_prio: PTH_PRIO_STD,
            a_dispatches: 0,
            a_name: [0; PTH_TCB_NAMELEN],
            a_joinable: TRUE,
            a_cancelstate: PTH_CANCEL_DEFAULT,
            a_stacksize: 64 * 1024,
            a_stackaddr: ptr::null_mut(),
        });
        attr.init()?;
        Ok(attr)
    }

    fn new() -> Result<Box<Self>, i32> {
        let mut attr = Box::new(PthAttr {
            a_tid: None,
            a_prio: PTH_PRIO_STD,
            a_dispatches: 0,
            a_name: [0; PTH_TCB_NAMELEN],
            a_joinable: TRUE,
            a_cancelstate: PTH_CANCEL_DEFAULT,
            a_stacksize: 64 * 1024,
            a_stackaddr: ptr::null_mut(),
        });
        attr.init()?;
        Ok(attr)
    }

    fn destroy(&mut self) -> Result<(), i32> {
        Ok(())
    }

    fn init(&mut self) -> Result<(), i32> {
        if self.a_tid.is_some() {
            return Err(libc::EPERM);
        }
        self.a_prio = PTH_PRIO_STD;
        let name = CString::new("unknown").unwrap();
        unsafe {
            libc::strncpy(
                self.a_name.as_mut_ptr(),
                name.as_ptr(),
                PTH_TCB_NAMELEN - 1,
            );
        }
        self.a_dispatches = 0;
        self.a_joinable = TRUE;
        self.a_cancelstate = PTH_CANCEL_DEFAULT;
        self.a_stacksize = 64 * 1024;
        self.a_stackaddr = ptr::null_mut();
        Ok(())
    }

    fn get(&self, op: i32, dst: &mut dyn std::any::Any) -> Result<(), i32> {
        self.ctrl(PthAttrOp::Get, op, dst)
    }

    fn set(&mut self, op: i32, src: &dyn std::any::Any) -> Result<(), i32> {
        self.ctrl(PthAttrOp::Set, op, src)
    }

    fn ctrl(&mut self, cmd: PthAttrOp, op: i32, arg: &dyn std::any::Any) -> Result<(), i32> {
        match op {
            // Priority
            0 => {
                let (src, dst) = match cmd {
                    PthAttrOp::Set => {
                        let val = arg.downcast_ref::<i32>().unwrap();
                        if let Some(ref tid) = self.a_tid {
                            (&val, &mut tid.prio)
                        } else {
                            (&val, &mut self.a_prio)
                        }
                    }
                    PthAttrOp::Get => {
                        let dst = arg.downcast_ref::<&mut i32>().unwrap();
                        if let Some(ref tid) = self.a_tid {
                            (&tid.prio, dst)
                        } else {
                            (&self.a_prio, dst)
                        }
                    }
                };
                *dst = *src;
            }
            // Name
            1 => match cmd {
                PthAttrOp::Set => {
                    let src = arg.downcast_ref::<*const c_char>().unwrap();
                    let dst = if let Some(ref tid) = self.a_tid {
                        tid.name.as_mut_ptr()
                    } else {
                        self.a_name.as_mut_ptr()
                    };
                    unsafe {
                        libc::strncpy(dst, *src, PTH_TCB_NAMELEN - 1);
                    }
                }
                PthAttrOp::Get => {
                    let dst = arg.downcast_ref::<*mut *const c_char>().unwrap();
                    let src = if let Some(ref tid) = self.a_tid {
                        tid.name.as_ptr()
                    } else {
                        self.a_name.as_ptr()
                    };
                    unsafe {
                        **dst = src;
                    }
                }
            },
            // Dispatches
            2 => {
                let (src, dst) = match cmd {
                    PthAttrOp::Set => {
                        let val = arg.downcast_ref::<i32>().unwrap();
                        if let Some(ref tid) = self.a_tid {
                            (&val, &mut tid.dispatches)
                        } else {
                            (&val, &mut self.a_dispatches)
                        }
                    }
                    PthAttrOp::Get => {
                        let dst = arg.downcast_ref::<&mut i32>().unwrap();
                        if let Some(ref tid) = self.a_tid {
                            (&tid.dispatches, dst)
                        } else {
                            (&self.a_dispatches, dst)
                        }
                    }
                };
                *dst = *src;
            }
            // Joinable
            3 => {
                let (src, dst) = match cmd {
                    PthAttrOp::Set => {
                        let val = arg.downcast_ref::<i32>().unwrap();
                        if let Some(ref tid) = self.a_tid {
                            (&val, &mut tid.joinable)
                        } else {
                            (&val, &mut self.a_joinable)
                        }
                    }
                    PthAttrOp::Get => {
                        let dst = arg.downcast_ref::<&mut i32>().unwrap();
                        if let Some(ref tid) = self.a_tid {
                            (&tid.joinable, dst)
                        } else {
                            (&self.a_joinable, dst)
                        }
                    }
                };
                *dst = *src;
            }
            // Cancel state
            4 => {
                let (src, dst) = match cmd {
                    PthAttrOp::Set => {
                        let val = arg.downcast_ref::<u32>().unwrap();
                        if let Some(ref tid) = self.a_tid {
                            (&val, &mut tid.cancelstate)
                        } else {
                            (&val, &mut self.a_cancelstate)
                        }
                    }
                    PthAttrOp::Get => {
                        let dst = arg.downcast_ref::<&mut u32>().unwrap();
                        if let Some(ref tid) = self.a_tid {
                            (&tid.cancelstate, dst)
                        } else {
                            (&self.a_cancelstate, dst)
                        }
                    }
                };
                *dst = *src;
            }
            // Stack size
            5 => {
                match cmd {
                    PthAttrOp::Set => {
                        if self.a_tid.is_some() {
                            return Err(libc::EPERM);
                        }
                        let val = arg.downcast_ref::<u32>().unwrap();
                        self.a_stacksize = *val;
                    }
                    PthAttrOp::Get => {
                        let dst = arg.downcast_ref::<&mut u32>().unwrap();
                        *dst = if let Some(ref tid) = self.a_tid {
                            tid.stacksize
                        } else {
                            self.a_stacksize
                        };
                    }
                }
            }
            // Stack address
            6 => {
                match cmd {
                    PthAttrOp::Set => {
                        if self.a_tid.is_some() {
                            return Err(libc::EPERM);
                        }
                        let val = arg.downcast_ref::<*mut c_char>().unwrap();
                        self.a_stackaddr = *val;
                    }
                    PthAttrOp::Get => {
                        let dst = arg.downcast_ref::<&mut *mut c_char>().unwrap();
                        *dst = if let Some(ref tid) = self.a_tid {
                            tid.stack
                        } else {
                            self.a_stackaddr
                        };
                    }
                }
            }
            // Time spawned
            7 => {
                if let PthAttrOp::Set = cmd {
                    return Err(libc::EPERM);
                }
                let dst = arg.downcast_ref::<&mut SystemTime>().unwrap();
                *dst = if let Some(ref tid) = self.a_tid {
                    tid.spawned
                } else {
                    UNIX_EPOCH
                };
            }
            // Time last ran
            8 => {
                if let PthAttrOp::Set = cmd {
                    return Err(libc::EPERM);
                }
                let dst = arg.downcast_ref::<&mut SystemTime>().unwrap();
                *dst = if let Some(ref tid) = self.a_tid {
                    tid.lastran
                } else {
                    UNIX_EPOCH
                };
            }
            // Time ran
            9 => {
                if let PthAttrOp::Set = cmd {
                    return Err(libc::EPERM);
                }
                let dst = arg.downcast_ref::<&mut SystemTime>().unwrap();
                *dst = if let Some(ref tid) = self.a_tid {
                    tid.running
                } else {
                    UNIX_EPOCH
                };
            }
            // Start func
            10 => {
                if let PthAttrOp::Set = cmd {
                    return Err(libc::EPERM);
                }
                if self.a_tid.is_none() {
                    return Err(libc::EACCES);
                }
                let dst = arg.downcast_ref::<&mut Option<extern "C" fn(*mut c_void) -> *mut c_void>>().unwrap();
                *dst = self.a_tid.as_ref().unwrap().start_func;
            }
            // Start arg
            11 => {
                if let PthAttrOp::Set = cmd {
                    return Err(libc::EPERM);
                }
                if self.a_tid.is_none() {
                    return Err(libc::EACCES);
                }
                let dst = arg.downcast_ref::<&mut *mut c_void>().unwrap();
                *dst = self.a_tid.as_ref().unwrap().start_arg;
            }
            // State
            12 => {
                if let PthAttrOp::Set = cmd {
                    return Err(libc::EPERM);
                }
                if self.a_tid.is_none() {
                    return Err(libc::EACCES);
                }
                let dst = arg.downcast_ref::<&mut i32>().unwrap();
                *dst = self.a_tid.as_ref().unwrap().state;
            }
            // Events
            13 => {
                if let PthAttrOp::Set = cmd {
                    return Err(libc::EPERM);
                }
                if self.a_tid.is_none() {
                    return Err(libc::EACCES);
                }
                let dst = arg.downcast_ref::<&mut *mut c_void>().unwrap();
                *dst = self.a_tid.as_ref().unwrap().events;
            }
            // Bound
            14 => {
                if let PthAttrOp::Set = cmd {
                    return Err(libc::EPERM);
                }
                let dst = arg.downcast_ref::<&mut i32>().unwrap();
                *dst = if self.a_tid.is_some() { TRUE } else { FALSE };
            }
            _ => return Err(libc::EINVAL),
        }
        Ok(())
    }
}