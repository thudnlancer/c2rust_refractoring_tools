/*
 * Compton - a compositor for X11
 *
 * Based on `xcompmgr` - Copyright (c) 2003, Keith Packard
 *
 * Copyright (c) 2011-2013, Christopher Jeffrey
 * See LICENSE for more information.
 *
 */

use std::ffi::{CStr, CString};
use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::sync::Arc;
use std::collections::HashMap;

use libc::{self, isspace, strstr, strlen};
use x11::xlib;
use gl::types::*;
use glx::types::*;

use crate::common::*;
use crate::opengl::*;

#[cfg(debug_assertions)]
const DEBUG_GLX_ERR: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG_GLX_ERR: bool = false;

#[cfg(debug_assertions)]
const DEBUG_GLX_DEBUG_CONTEXT: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG_GLX_DEBUG_CONTEXT: bool = false;

const OPENGL_MAX_DEPTH: i32 = 32;
const CGLX_MAX_BUFFER_AGE: i32 = 4;
const MAX_BLUR_PASS: usize = 2;

#[derive(Debug, Clone, Copy)]
struct GLXFBConfig {
    cfg: GLXFBConfig,
    texture_fmt: i32,
    texture_tgts: i32,
    y_inverted: bool,
}

#[derive(Debug, Clone)]
struct GLXTexture {
    texture: GLuint,
    glpixmap: GLXPixmap,
    pixmap: xlib::Pixmap,
    target: GLenum,
    width: u32,
    height: u32,
    depth: u32,
    y_inverted: bool,
}

#[derive(Debug, Clone)]
struct GLXBlurPass {
    frag_shader: GLuint,
    prog: GLuint,
    unifm_factor_center: i32,
    unifm_offset_x: i32,
    unifm_offset_y: i32,
}

#[derive(Debug, Clone)]
struct GLXBlurCache {
    textures: [GLuint; 2],
    fbo: GLuint,
    width: i32,
    height: i32,
}

#[derive(Debug, Clone)]
struct GLXProgMain {
    prog: GLuint,
    unifm_opacity: i32,
    unifm_invert_color: i32,
    unifm_tex: i32,
}

#[derive(Debug, Clone)]
struct GLXSession {
    context: GLXContext,
    fbconfigs: [Option<GLXFBConfig>; OPENGL_MAX_DEPTH as usize + 1],
    has_texture_non_power_of_two: bool,
    blur_passes: [GLXBlurPass; MAX_BLUR_PASS],
    z: f32,
    glx_bind_tex_image_proc: Option<unsafe extern "C" fn(*mut xlib::Display, GLXPixmap, i32, *const i32)>,
    glx_release_tex_image_proc: Option<unsafe extern "C" fn(*mut xlib::Display, GLXPixmap, i32)>,
    glx_copy_sub_buffer_proc: Option<unsafe extern "C" fn(*mut xlib::Display, xlib::Drawable, i32, i32, i32, i32)>,
    gl_fence_sync_proc: Option<unsafe extern "C" fn(u32, u32) -> GLsync>,
    gl_is_sync_proc: Option<unsafe extern "C" fn(GLsync) -> u8>,
    gl_delete_sync_proc: Option<unsafe extern "C" fn(GLsync)>,
    gl_client_wait_sync_proc: Option<unsafe extern "C" fn(GLsync, u32, u64) -> u32>,
    gl_wait_sync_proc: Option<unsafe extern "C" fn(GLsync, u32, u64)>,
    gl_import_sync_ext: Option<unsafe extern "C" fn(u32, xlib::XSyncFence, u32) -> GLsync>,
    gl_string_marker_gremedy: Option<unsafe extern "C" fn(i32, *const c_void)>,
    gl_frame_terminator_gremedy: Option<unsafe extern "C" fn()>,
}

impl Default for GLXSession {
    fn default() -> Self {
        GLXSession {
            context: ptr::null_mut(),
            fbconfigs: [None; OPENGL_MAX_DEPTH as usize + 1],
            has_texture_non_power_of_two: false,
            blur_passes: [
                GLXBlurPass {
                    frag_shader: 0,
                    prog: 0,
                    unifm_factor_center: -1,
                    unifm_offset_x: -1,
                    unifm_offset_y: -1,
                },
                GLXBlurPass {
                    frag_shader: 0,
                    prog: 0,
                    unifm_factor_center: -1,
                    unifm_offset_x: -1,
                    unifm_offset_y: -1,
                },
            ],
            z: 0.0,
            glx_bind_tex_image_proc: None,
            glx_release_tex_image_proc: None,
            glx_copy_sub_buffer_proc: None,
            gl_fence_sync_proc: None,
            gl_is_sync_proc: None,
            gl_delete_sync_proc: None,
            gl_client_wait_sync_proc: None,
            gl_wait_sync_proc: None,
            gl_import_sync_ext: None,
            gl_string_marker_gremedy: None,
            gl_frame_terminator_gremedy: None,
        }
    }
}

#[derive(Debug, Clone)]
struct Session {
    dpy: *mut xlib::Display,
    scr: i32,
    vis: *mut xlib::Visual,
    root_width: i32,
    root_height: i32,
    depth: i32,
    glx_exists: bool,
    glx_event: i32,
    glx_error: i32,
    psglx: Option<Box<GLXSession>>,
    o: Options,
    list: Option<Box<Window>>,
    screen_reg: xlib::XserverRegion,
    all_damage_last: [xlib::XserverRegion; CGLX_MAX_BUFFER_AGE as usize],
}

#[derive(Debug, Clone)]
struct Options {
    glx_no_stencil: bool,
    glx_use_copysubbuffermesa: bool,
    glx_use_gpushader4: bool,
    glx_copy_from_front: bool,
    glx_swap_method: i32,
    blur_kerns: [Option<Vec<XFixed>>; MAX_BLUR_PASS],
    resize_damage: i32,
    glx_prog_win: GLXProgMain,
}

#[derive(Debug, Clone)]
struct Window {
    next: Option<Box<Window>>,
    // Other window fields...
}

type XFixed = i32;

#[cfg(debug_assertions)]
fn glx_dump_err_str(err: GLenum) -> Option<&'static str> {
    match err {
        gl::NO_ERROR => Some("GL_NO_ERROR"),
        gl::INVALID_ENUM => Some("GL_INVALID_ENUM"),
        gl::INVALID_VALUE => Some("GL_INVALID_VALUE"),
        gl::INVALID_OPERATION => Some("GL_INVALID_OPERATION"),
        gl::INVALID_FRAMEBUFFER_OPERATION => Some("GL_INVALID_FRAMEBUFFER_OPERATION"),
        gl::OUT_OF_MEMORY => Some("GL_OUT_OF_MEMORY"),
        gl::STACK_UNDERFLOW => Some("GL_STACK_UNDERFLOW"),
        gl::STACK_OVERFLOW => Some("GL_STACK_OVERFLOW"),
        _ => None,
    }
}

#[cfg(debug_assertions)]
fn glx_check_err(ps: &Session, func: &str, line: i32) {
    if ps.psglx.is_none() || ps.psglx.as_ref().unwrap().context.is_null() {
        return;
    }

    let mut err = gl::NO_ERROR;
    while gl::NO_ERROR != {
        err = unsafe { gl::GetError() };
        err
    } {
        print_timestamp(ps);
        print!("{}():{}: GLX error ", func, line);
        if let Some(errtext) = glx_dump_err_str(err) {
            println_dbg!("{}", errtext);
        } else {
            println_dbg!("{}", err);
        }
    }
}

#[cfg(not(debug_assertions))]
fn glx_check_err(_ps: &Session) {}

fn wd_is_in_str(haystack: Option<&str>, needle: &str) -> bool {
    if haystack.is_none() {
        return false;
    }
    let haystack = haystack.unwrap();
    assert!(!needle.is_empty());

    let mut pos = haystack.as_ptr() as usize - 1;
    while let Some(p) = unsafe {
        strstr(
            (pos + 1) as *const c_char,
            needle.as_ptr() as *const c_char,
        )
    } {
        pos = p as usize;
        let offset = pos - haystack.as_ptr() as usize;
        if (offset != 0 && unsafe { !isspace(*(p as *const c_char).offset(-1)) })
            || (unsafe { strlen(p as *const c_char) } > needle.len() as usize
                && unsafe { !isspace(*(p as *const c_char).offset(needle.len() as isize)) })
        {
            continue;
        }
        return true;
    }

    false
}

fn glx_hasglxext(ps: &Session, ext: &str) -> bool {
    unsafe {
        let glx_exts = glx::QueryExtensionsString(ps.dpy, ps.scr);
        if glx_exts.is_null() {
            println_errf("(): Failed get GLX extension list.");
            return false;
        }

        let found = wd_is_in_str(Some(CStr::from_ptr(glx_exts).to_str().ok(), ext);
        if !found {
            println_errf("(): Missing GLX extension {}.", ext);
        }

        found
    }
}

fn glx_hasglext(ps: &Session, ext: &str) -> bool {
    unsafe {
        let gl_exts = gl::GetString(gl::EXTENSIONS) as *const c_char;
        if gl_exts.is_null() {
            println_errf("(): Failed get GL extension list.");
            return false;
        }

        let found = wd_is_in_str(CStr::from_ptr(gl_exts).to_str().ok(), ext);
        if !found {
            println_errf("(): Missing GL extension {}.", ext);
        }

        found
    }
}

fn get_visualinfo_from_visual(ps: &Session, visual: *mut xlib::Visual) -> Option<xlib::XVisualInfo> {
    unsafe {
        let mut vreq = mem::zeroed::<xlib::XVisualInfo>();
        vreq.visualid = xlib::XVisualIDFromVisual(visual);
        let mut nitems = 0;
        let vis_info = xlib::XGetVisualInfo(ps.dpy, xlib::VisualIDMask, &vreq, &mut nitems);
        if vis_info.is_null() || nitems == 0 {
            None
        } else {
            let info = *vis_info;
            xlib::XFree(vis_info as *mut c_void);
            Some(info)
        }
    }
}

fn get_fbconfig_from_visualinfo(ps: &Session, visualinfo: &xlib::XVisualInfo) -> Option<GLXFBConfig> {
    unsafe {
        let mut nelements = 0;
        let fbconfigs = glx::GetFBConfigs(ps.dpy, visualinfo.screen, &mut nelements);
        if fbconfigs.is_null() {
            return None;
        }

        let mut result = None;
        for i in 0..nelements {
            let mut visual_id = 0;
            if glx::GetFBConfigAttrib(
                ps.dpy,
                *fbconfigs.offset(i as isize),
                glx::VISUAL_ID,
                &mut visual_id,
            ) == xlib::Success
                && visual_id == visualinfo.visualid
            {
                result = Some(GLXFBConfig {
                    cfg: *fbconfigs.offset(i as isize),
                    texture_fmt: 0,
                    texture_tgts: 0,
                    y_inverted: false,
                });
                break;
            }
        }
        xlib::XFree(fbconfigs as *mut c_void);
        result
    }
}

#[cfg(debug_assertions)]
extern "C" fn glx_debug_msg_callback(
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _userParam: *mut GLvoid,
) {
    unsafe {
        println_dbgf!(
            "(): source 0x{:04X}, type 0x{:04X}, id {}, severity 0x{:0X}, \"{}\"\n",
            source,
            type_,
            id,
            severity,
            CStr::from_ptr(message).to_string_lossy()
        );
    }
}

fn glx_init(ps: &mut Session, need_render: bool) -> bool {
    let mut success = false;
    let mut pvis = None;

    // Check for GLX extension
    if !ps.glx_exists {
        let mut event = 0;
        let mut error = 0;
        if unsafe { glx::QueryExtension(ps.dpy, &mut event, &mut error) } != 0 {
            ps.glx_exists = true;
            ps.glx_event = event;
            ps.glx_error = error;
        } else {
            println_errf("(): No GLX extension.");
            return false;
        }
    }

    // Get XVisualInfo
    pvis = get_visualinfo_from_visual(ps, ps.vis);
    if pvis.is_none() {
        println_errf("(): Failed to acquire XVisualInfo for current visual.");
        return false;
    }
    let pvis = pvis.unwrap();

    // Ensure the visual is double-buffered
    if need_render {
        let mut value = 0;
        if unsafe { glx::GetConfig(ps.dpy, &pvis, glx::USE_GL, &mut value) } != xlib::Success || value == 0 {
            println_errf("(): Root visual is not a GL visual.");
            return false;
        }

        if unsafe { glx::GetConfig(ps.dpy, &pvis, glx::DOUBLEBUFFER, &mut value) } != xlib::Success || value == 0 {
            println_errf("(): Root visual is not a double buffered GL visual.");
            return false;
        }
    }

    // Ensure GLX_EXT_texture_from_pixmap exists
    if need_render && !glx_hasglxext(ps, "GLX_EXT_texture_from_pixmap") {
        return false;
    }

    // Initialize GLX data structure
    if ps.psglx.is_none() {
        ps.psglx = Some(Box::new(GLXSession::default()));
    }

    let psglx = ps.psglx.as_mut().unwrap();

    if psglx.context.is_null() {
        // Get GLX context
        #[cfg(not(debug_assertions))]
        {
            psglx.context = unsafe { glx::CreateContext(ps.dpy, &pvis, ptr::null_mut(), gl::TRUE as i32) };
        }
        #[cfg(debug_assertions)]
        {
            let fbconfig = get_fbconfig_from_visualinfo(ps, &pvis);
            if fbconfig.is_none() {
                println_errf!(
                    "(): Failed to get GLXFBConfig for root visual {:#x}.",
                    pvis.visualid
                );
                return false;
            }
            let fbconfig = fbconfig.unwrap();

            let p_glx_create_context_attribs_arb = unsafe {
                mem::transmute::<*const u8, unsafe extern "C" fn(
                    *mut xlib::Display,
                    GLXFBConfig,
                    GLXContext,
                    i32,
                    *const i32,
                ) -> GLXContext>(
                    glx::GetProcAddress(b"glXCreateContextAttribsARB\0".as_ptr() as *const _),
                )
            };

            if p_glx_create_context_attribs_arb.is_null() {
                println_errf("(): Failed to get glXCreateContextAttribsARB().");
                return false;
            }

            let attrib_list = [glx::CONTEXT_FLAGS_ARB, glx::CONTEXT_DEBUG_BIT_ARB, 0];
            psglx.context = unsafe {
                p_glx_create_context_attribs_arb(
                    ps.dpy,
                    fbconfig.cfg,
                    ptr::null_mut(),
                    gl::TRUE as i32,
                    attrib_list.as_ptr(),
                )
            };
        }

        if psglx.context.is_null() {
            println_errf("(): Failed to get GLX context.");
            return false;
        }

        // Attach GLX context
        if unsafe { glx::MakeCurrent(ps.dpy, get_tgt_window(ps), psglx.context) } == 0 {
            println_errf("(): Failed to attach GLX context.");
            return false;
        }

        #[cfg(debug_assertions)]
        {
            let p_debug_message_callback = unsafe {
                mem::transmute::<*const u8, unsafe extern "C" fn(
                    Option<unsafe extern "C" fn(
                        GLenum,
                        GLenum,
                        GLuint,
                        GLenum,
                        GLsizei,
                        *const GLchar,
                        *mut GLvoid,
                    )>,
                    *mut c_void,
               