use libc::{c_int, c_ulong};
use std::io;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sigset {
    pub __val: [c_ulong; 16],
}

pub enum SignalMaskHow {
    Block,
    Unblock,
    SetMask,
}

impl SignalMaskHow {
    fn to_libc(&self) -> c_int {
        match self {
            SignalMaskHow::Block => libc::SIG_BLOCK,
            SignalMaskHow::Unblock => libc::SIG_UNBLOCK,
            SignalMaskHow::SetMask => libc::SIG_SETMASK,
        }
    }
}

pub fn pthread_sigmask(
    how: SignalMaskHow,
    new_mask: Option<&Sigset>,
    old_mask: Option<&mut Sigset>,
) -> io::Result<()> {
    let how = how.to_libc();
    let new_mask_ptr = new_mask.map_or(std::ptr::null(), |m| m as *const _);
    let old_mask_ptr = old_mask.map_or(std::ptr::null_mut(), |m| m as *mut _);

    let ret = unsafe { libc::sigprocmask(how, new_mask_ptr, old_mask_ptr) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}