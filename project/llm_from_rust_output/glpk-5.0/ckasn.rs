use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::mem;

#[derive(Debug)]
pub enum GlpError {
    InvalidOffset,
    InconsistentAssignment,
    InvalidAssignmentValue,
    BothInAndOut,
}

#[derive(Debug)]
pub struct GlpGraph {
    pool: *mut c_void,
    name: *mut c_char,
    nv_max: c_int,
    nv: c_int,
    na: c_int,
    v: *mut *mut GlpVertex,
    index: *mut c_void,
    v_size: c_int,
    a_size: c_int,
}

#[derive(Debug)]
pub struct GlpVertex {
    i: c_int,
    name: *mut c_char,
    entry: *mut c_void,
    data: *mut c_void,
    temp: *mut c_void,
    in_0: *mut GlpArc,
    out: *mut GlpArc,
}

#[derive(Debug)]
pub struct GlpArc {
    tail: *mut GlpVertex,
    head: *mut GlpVertex,
    data: *mut c_void,
    temp: *mut c_void,
    t_prev: *mut GlpArc,
    t_next: *mut GlpArc,
    h_prev: *mut GlpArc,
    h_next: *mut GlpArc,
}

pub fn glp_check_asnprob(graph: &GlpGraph, v_set: c_int) -> Result<(), GlpError> {
    if v_set >= 0 && v_set > graph.v_size - mem::size_of::<c_int>() as c_int {
        return Err(GlpError::InvalidOffset);
    }

    for i in 1..=graph.nv {
        let v = unsafe { *graph.v.offset(i as isize) };
        if v_set >= 0 {
            let k = unsafe {
                let mut k = 0;
                ptr::copy_nonoverlapping(
                    v.data.cast::<c_char>().offset(v_set as isize).cast(),
                    &mut k as *mut c_int,
                    1,
                );
                k
            };

            match k {
                0 if !v.in_0.is_null() => return Err(GlpError::InconsistentAssignment),
                1 if !v.out.is_null() => return Err(GlpError::InconsistentAssignment),
                0 | 1 => (),
                _ => return Err(GlpError::InvalidAssignmentValue),
            }
        } else if !v.in_0.is_null() && !v.out.is_null() {
            return Err(GlpError::BothInAndOut);
        }
    }

    Ok(())
}