use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _XGC;
    pub type _XDisplay;
    pub type DBusConnection;
    pub type __GLXcontextRec;
    pub type __GLXFBConfigRec;
    pub type _c2_lptr;
    fn glDrawBuffers(n: GLsizei, bufs: *const GLenum);
    fn glAttachShader(program: GLuint, shader: GLuint);
    fn glCompileShader(shader: GLuint);
    fn glCreateProgram() -> GLuint;
    fn glCreateShader(type_0: GLenum) -> GLuint;
    fn glDeleteProgram(program: GLuint);
    fn glDeleteShader(shader: GLuint);
    fn glDetachShader(program: GLuint, shader: GLuint);
    fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);
    fn glGetProgramInfoLog(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    );
    fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
    fn glGetShaderInfoLog(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    );
    fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
    fn glLinkProgram(program: GLuint);
    fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    );
    fn glUseProgram(program: GLuint);
    fn glUniform1f(location: GLint, v0: GLfloat);
    fn glUniform1i(location: GLint, v0: GLint);
    fn glBindFramebuffer(target: GLenum, framebuffer: GLuint);
    fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint);
    fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint);
    fn glCheckFramebufferStatus(target: GLenum) -> GLenum;
    fn glFramebufferTexture2D(
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    );
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    fn glClear(mask: GLbitfield);
    fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
    fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    fn glLogicOp(opcode: GLenum);
    fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn glReadBuffer(mode: GLenum);
    fn glEnable(cap: GLenum);
    fn glDisable(cap: GLenum);
    fn glIsEnabled(cap: GLenum) -> GLboolean;
    fn glGetFloatv(pname: GLenum, params: *mut GLfloat);
    fn glGetIntegerv(pname: GLenum, params: *mut GLint);
    fn glGetString(name: GLenum) -> *const GLubyte;
    fn glDepthMask(flag: GLboolean);
    fn glMatrixMode(mode: GLenum);
    fn glOrtho(
        left: GLdouble,
        right: GLdouble,
        bottom: GLdouble,
        top: GLdouble,
        near_val: GLdouble,
        far_val: GLdouble,
    );
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn glLoadIdentity();
    fn glBegin(mode: GLenum);
    fn glEnd();
    fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat);
    fn glVertex3i(x: GLint, y: GLint, z: GLint);
    fn glColor4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    fn glTexCoord2f(s: GLfloat, t: GLfloat);
    fn glRasterPos2f(x: GLfloat, y: GLfloat);
    fn glRasterPos4fv(v: *const GLfloat);
    fn glPixelStorei(pname: GLenum, param: GLint);
    fn glBitmap(
        width: GLsizei,
        height: GLsizei,
        xorig: GLfloat,
        yorig: GLfloat,
        xmove: GLfloat,
        ymove: GLfloat,
        bitmap: *const GLubyte,
    );
    fn glReadPixels(
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_0: GLenum,
        pixels: *mut libc::c_void,
    );
    fn glCopyPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, type_0: GLenum);
    fn glStencilFunc(func: GLenum, ref_0: GLint, mask: GLuint);
    fn glStencilMask(mask: GLuint);
    fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum);
    fn glTexEnvf(target: GLenum, pname: GLenum, param: GLfloat);
    fn glTexEnvi(target: GLenum, pname: GLenum, param: GLint);
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_0: GLenum,
        pixels: *const libc::c_void,
    );
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    fn glDeleteTextures(n: GLsizei, textures: *const GLuint);
    fn glBindTexture(target: GLenum, texture: GLuint);
    fn glCopyTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );
    fn glActiveTexture(texture: GLenum);
    fn glMultiTexCoord2f(target: GLenum, s: GLfloat, t: GLfloat);
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn vsync_deinit(ps: *mut session_t);
    fn vsync_init(ps: *mut session_t) -> bool;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn XFixesFetchRegion(
        dpy: *mut Display,
        region: XserverRegion,
        nrectanglesRet: *mut libc::c_int,
    ) -> *mut XRectangle;
    fn XFixesSubtractRegion(
        dpy: *mut Display,
        dst: XserverRegion,
        src1: XserverRegion,
        src2: XserverRegion,
    );
    fn XFixesIntersectRegion(
        dpy: *mut Display,
        dst: XserverRegion,
        src1: XserverRegion,
        src2: XserverRegion,
    );
    fn XFixesUnionRegion(
        dpy: *mut Display,
        dst: XserverRegion,
        src1: XserverRegion,
        src2: XserverRegion,
    );
    fn XFixesCopyRegion(dpy: *mut Display, dst: XserverRegion, src: XserverRegion);
    fn XFixesDestroyRegion(dpy: *mut Display, region: XserverRegion);
    fn XFixesCreateRegion(
        dpy: *mut Display,
        rectangles: *mut XRectangle,
        nrectangles: libc::c_int,
    ) -> XserverRegion;
    fn XGetVisualInfo(
        _: *mut Display,
        _: libc::c_long,
        _: *mut XVisualInfo,
        _: *mut libc::c_int,
    ) -> *mut XVisualInfo;
    fn glXCreateContext(
        dpy: *mut Display,
        vis: *mut XVisualInfo,
        shareList: GLXContext,
        direct: libc::c_int,
    ) -> GLXContext;
    fn glXDestroyContext(dpy: *mut Display, ctx: GLXContext);
    fn glXMakeCurrent(
        dpy: *mut Display,
        drawable: GLXDrawable,
        ctx: GLXContext,
    ) -> libc::c_int;
    fn glXSwapBuffers(dpy: *mut Display, drawable: GLXDrawable);
    fn glXQueryExtension(
        dpy: *mut Display,
        errorb: *mut libc::c_int,
        event: *mut libc::c_int,
    ) -> libc::c_int;
    fn glXGetConfig(
        dpy: *mut Display,
        visual: *mut XVisualInfo,
        attrib: libc::c_int,
        value: *mut libc::c_int,
    ) -> libc::c_int;
    fn glXQueryExtensionsString(
        dpy: *mut Display,
        screen: libc::c_int,
    ) -> *const libc::c_char;
    fn glXGetFBConfigAttrib(
        dpy: *mut Display,
        config: GLXFBConfig,
        attribute: libc::c_int,
        value: *mut libc::c_int,
    ) -> libc::c_int;
    fn glXGetFBConfigs(
        dpy: *mut Display,
        screen: libc::c_int,
        nelements: *mut libc::c_int,
    ) -> *mut GLXFBConfig;
    fn glXGetVisualFromFBConfig(
        dpy: *mut Display,
        config: GLXFBConfig,
    ) -> *mut XVisualInfo;
    fn glXCreatePixmap(
        dpy: *mut Display,
        config: GLXFBConfig,
        pixmap: Pixmap,
        attribList: *const libc::c_int,
    ) -> GLXPixmap;
    fn glXDestroyPixmap(dpy: *mut Display, pixmap: GLXPixmap);
    fn glXQueryDrawable(
        dpy: *mut Display,
        draw: GLXDrawable,
        attribute: libc::c_int,
        value: *mut libc::c_uint,
    );
    fn glXGetProcAddress(
        procname: *const GLubyte,
    ) -> Option::<unsafe extern "C" fn() -> ()>;
    fn XVisualIDFromVisual(_: *mut Visual) -> VisualID;
    fn XFree(_: *mut libc::c_void) -> libc::c_int;
    fn XGetGeometry(
        _: *mut Display,
        _: Drawable,
        _: *mut Window,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_uint,
        _: *mut libc::c_uint,
        _: *mut libc::c_uint,
        _: *mut libc::c_uint,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type uint32_t = __uint32_t;
pub type int_fast16_t = libc::c_long;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
}

pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Pixmap = XID;
pub type Colormap = XID;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option::<unsafe extern "C" fn(*mut _XExtData) -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWindowAttributes {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub depth: libc::c_int,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: libc::c_int,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub colormap: Colormap,
    pub map_installed: libc::c_int,
    pub map_state: libc::c_int,
    pub all_event_masks: libc::c_long,
    pub your_event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub screen: *mut Screen,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRectangle {
    pub x: libc::c_short,
    pub y: libc::c_short,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisualInfo {
    pub visual: *mut Visual,
    pub visualid: VisualID,
    pub screen: libc::c_int,
    pub depth: libc::c_int,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub colormap_size: libc::c_int,
    pub bits_per_rgb: libc::c_int,
}
pub type XserverRegion = XID;
pub type Damage = XID;
pub type Picture = XID;
pub type PictFormat = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderDirectFormat {
    pub red: libc::c_short,
    pub redMask: libc::c_short,
    pub green: libc::c_short,
    pub greenMask: libc::c_short,
    pub blue: libc::c_short,
    pub blueMask: libc::c_short,
    pub alpha: libc::c_short,
    pub alphaMask: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderPictFormat {
    pub id: PictFormat,
    pub type_0: libc::c_int,
    pub depth: libc::c_int,
    pub direct: XRenderDirectFormat,
    pub colormap: Colormap,
}
pub type XDouble = libc::c_double;
pub type XFixed = libc::c_int;
pub type XdbeBackBuffer = Drawable;
pub type XSyncFence = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XineramaScreenInfo {
    pub screen_number: libc::c_int,
    pub x_org: libc::c_short,
    pub y_org: libc::c_short,
    pub width: libc::c_short,
    pub height: libc::c_short,
}
pub type GLenum = libc::c_uint;
pub type GLboolean = libc::c_uchar;
pub type GLbitfield = libc::c_uint;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLubyte = libc::c_uchar;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GLdouble = libc::c_double;
pub type GLchar = libc::c_char;
pub type GLXContext = *mut __GLXcontextRec;
pub type GLXPixmap = XID;
pub type GLXDrawable = XID;
pub type GLXFBConfig = *mut __GLXFBConfigRec;
pub type opacity_t = uint32_t;
pub type time_ms_t = libc::c_long;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum wintype_t {
    WINTYPE_UNKNOWN,
    WINTYPE_DESKTOP,
    WINTYPE_DOCK,
    WINTYPE_TOOLBAR,
    WINTYPE_MENU,
    WINTYPE_UTILITY,
    WINTYPE_SPLASH,
    WINTYPE_DIALOG,
    WINTYPE_NORMAL,
    WINTYPE_DROPDOWN_MENU,
    WINTYPE_POPUP_MENU,
    WINTYPE_TOOLTIP,
    WINTYPE_NOTIFY,
    WINTYPE_COMBO,
    WINTYPE_DND,
    NUM_WINTYPES,
impl wintype_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            wintype_t::WINTYPE_UNKNOWN => 0,
            wintype_t::WINTYPE_DESKTOP => 1,
            wintype_t::WINTYPE_DOCK => 2,
            wintype_t::WINTYPE_TOOLBAR => 3,
            wintype_t::WINTYPE_MENU => 4,
            wintype_t::WINTYPE_UTILITY => 5,
            wintype_t::WINTYPE_SPLASH => 6,
            wintype_t::WINTYPE_DIALOG => 7,
            wintype_t::WINTYPE_NORMAL => 8,
            wintype_t::WINTYPE_DROPDOWN_MENU => 9,
            wintype_t::WINTYPE_POPUP_MENU => 10,
            wintype_t::WINTYPE_TOOLTIP => 11,
            wintype_t::WINTYPE_NOTIFY => 12,
            wintype_t::WINTYPE_COMBO => 13,
            wintype_t::WINTYPE_DND => 14,
            wintype_t::NUM_WINTYPES => 15,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum switch_t {
    OFF,
    ON,
    UNSET,
impl switch_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            switch_t::OFF => 0,
            switch_t::ON => 1,
            switch_t::UNSET => 2,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct geometry_t {
    pub wid: libc::c_int,
    pub hei: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct margin_t {
    pub top: libc::c_int,
    pub left: libc::c_int,
    pub bottom: libc::c_int,
    pub right: libc::c_int,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum winmode_t {
    WMODE_TRANS,
    WMODE_SOLID,
    WMODE_ARGB,
impl winmode_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            winmode_t::WMODE_TRANS => 0,
            winmode_t::WMODE_SOLID => 1,
            winmode_t::WMODE_ARGB => 2,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ignore {
    pub next: *mut _ignore,
    pub sequence: libc::c_ulong,
}
pub type ignore_t = _ignore;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum vsync_t {
    VSYNC_NONE,
    VSYNC_DRM,
    VSYNC_OPENGL,
    VSYNC_OPENGL_OML,
    VSYNC_OPENGL_SWC,
    VSYNC_OPENGL_MSWC,
    NUM_VSYNC,
impl vsync_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            vsync_t::VSYNC_NONE => 0,
            vsync_t::VSYNC_DRM => 1,
            vsync_t::VSYNC_OPENGL => 2,
            vsync_t::VSYNC_OPENGL_OML => 3,
            vsync_t::VSYNC_OPENGL_SWC => 4,
            vsync_t::VSYNC_OPENGL_MSWC => 5,
            vsync_t::NUM_VSYNC => 6,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum backend {
    BKEND_XRENDER,
    BKEND_GLX,
    BKEND_XR_GLX_HYBRID,
    NUM_BKEND,
impl backend {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            backend::BKEND_XRENDER => 0,
            backend::BKEND_GLX => 1,
            backend::BKEND_XR_GLX_HYBRID => 2,
            backend::NUM_BKEND => 3,
        }
    }
}

pub type C2RustUnnamed_0 = libc::c_int;
pub const SWAPM_EXCHANGE: C2RustUnnamed_0 = 2;
pub const SWAPM_COPY: C2RustUnnamed_0 = 1;
pub const SWAPM_UNDEFINED: C2RustUnnamed_0 = 0;
pub const SWAPM_BUFFER_AGE: C2RustUnnamed_0 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _glx_texture {
    pub texture: GLuint,
    pub glpixmap: GLXPixmap,
    pub pixmap: Pixmap,
    pub target: GLenum,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub depth: libc::c_uint,
    pub y_inverted: bool,
}
pub type glx_texture_t = _glx_texture;
pub type f_WaitVideoSync = Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_uint) -> libc::c_int,
>;
pub type f_GetVideoSync = Option::<
    unsafe extern "C" fn(*mut libc::c_uint) -> libc::c_int,
>;
pub type f_GetSyncValuesOML = Option::<
    unsafe extern "C" fn(
        *mut Display,
        GLXDrawable,
        *mut int64_t,
        *mut int64_t,
        *mut int64_t,
    ) -> libc::c_int,
>;
pub type f_WaitForMscOML = Option::<
    unsafe extern "C" fn(
        *mut Display,
        GLXDrawable,
        int64_t,
        int64_t,
        int64_t,
        *mut int64_t,
        *mut int64_t,
        *mut int64_t,
    ) -> libc::c_int,
>;
pub type f_SwapIntervalSGI = Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>;
pub type f_SwapIntervalMESA = Option::<
    unsafe extern "C" fn(libc::c_uint) -> libc::c_int,
>;
pub type f_BindTexImageEXT = Option::<
    unsafe extern "C" fn(
        *mut Display,
        GLXDrawable,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
>;
pub type f_ReleaseTexImageEXT = Option::<
    unsafe extern "C" fn(*mut Display, GLXDrawable, libc::c_int) -> (),
>;
pub type f_CopySubBuffer = Option::<
    unsafe extern "C" fn(
        *mut Display,
        GLXDrawable,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glx_fbconfig_t {
    pub cfg: GLXFBConfig,
    pub texture_fmt: GLint,
    pub texture_tgts: GLint,
    pub y_inverted: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glx_blur_pass_t {
    pub frag_shader: GLuint,
    pub prog: GLuint,
    pub unifm_offset_x: GLint,
    pub unifm_offset_y: GLint,
    pub unifm_factor_center: GLint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glx_blur_cache_t {
    pub fbo: GLuint,
    pub textures: [GLuint; 2],
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glx_prog_main_t {
    pub prog: GLuint,
    pub unifm_opacity: GLint,
    pub unifm_invert_color: GLint,
    pub unifm_tex: GLint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paint_t {
    pub pixmap: Pixmap,
    pub pict: Picture,
    pub ptex: *mut glx_texture_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conv {
    pub size: libc::c_int,
    pub data: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _latom {
    pub atom: Atom,
    pub next: *mut _latom,
}
pub type latom_t = _latom;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reg_data_t {
    pub rects: *mut XRectangle,
    pub nrects: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _timeout_t {
    pub enabled: bool,
    pub data: *mut libc::c_void,
    pub callback: Option::<
        unsafe extern "C" fn(*mut session_t, *mut _timeout_t) -> bool,
    >,
    pub interval: time_ms_t,
    pub firstrun: time_ms_t,
    pub lastrun: time_ms_t,
    pub next: *mut _timeout_t,
}
pub type session_t = _session_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _session_t {
    pub dpy: *mut Display,
    pub scr: libc::c_int,
    pub vis: *mut Visual,
    pub depth: libc::c_int,
    pub root: Window,
    pub root_height: libc::c_int,
    pub root_width: libc::c_int,
    pub overlay: Window,
    pub root_tile_fill: bool,
    pub root_tile_paint: paint_t,
    pub screen_reg: XserverRegion,
    pub root_picture: Picture,
    pub tgt_picture: Picture,
    pub tgt_buffer: paint_t,
    pub tgt_buffer_fence: XSyncFence,
    pub root_dbe: XdbeBackBuffer,
    pub reg_win: Window,
    pub psglx: *mut glx_session_t,
    pub o: options_t,
    pub pfds_read: *mut fd_set,
    pub pfds_write: *mut fd_set,
    pub pfds_except: *mut fd_set,
    pub nfds_max: libc::c_int,
    pub tmout_lst: *mut _timeout_t,
    pub tmout_unredir: *mut _timeout_t,
    pub tmout_unredir_hit: bool,
    pub ev_received: bool,
    pub idling: bool,
    pub time_start: timeval,
    pub all_damage: XserverRegion,
    pub all_damage_last: [XserverRegion; 5],
    pub redirected: bool,
    pub alpha_picts: *mut Picture,
    pub reg_ignore_expire: bool,
    pub fade_time: time_ms_t,
    pub ignore_head: *mut ignore_t,
    pub ignore_tail: *mut *mut ignore_t,
    pub blur_kerns_cache: [*mut XFixed; 5],
    pub reset: bool,
    pub expose_rects: *mut XRectangle,
    pub size_expose: libc::c_int,
    pub n_expose: libc::c_int,
    pub list: *mut _win,
    pub active_win: *mut _win,
    pub active_leader: Window,
    pub black_picture: Picture,
    pub cshadow_picture: Picture,
    pub white_picture: Picture,
    pub gaussian_map: *mut conv,
    pub cgsize: libc::c_int,
    pub shadow_corner: *mut libc::c_uchar,
    pub shadow_top: *mut libc::c_uchar,
    pub shadow_exclude_reg: XserverRegion,
    pub refresh_rate: libc::c_short,
    pub refresh_intv: libc::c_long,
    pub paint_tm_offset: libc::c_long,
    pub drm_fd: libc::c_int,
    pub xfixes_event: libc::c_int,
    pub xfixes_error: libc::c_int,
    pub damage_event: libc::c_int,
    pub damage_error: libc::c_int,
    pub render_event: libc::c_int,
    pub render_error: libc::c_int,
    pub composite_event: libc::c_int,
    pub composite_error: libc::c_int,
    pub composite_opcode: libc::c_int,
    pub has_name_pixmap: bool,
    pub shape_exists: bool,
    pub shape_event: libc::c_int,
    pub shape_error: libc::c_int,
    pub randr_exists: bool,
    pub randr_event: libc::c_int,
    pub randr_error: libc::c_int,
    pub glx_exists: bool,
    pub glx_event: libc::c_int,
    pub glx_error: libc::c_int,
    pub dbe_exists: bool,
    pub xinerama_exists: bool,
    pub xinerama_scrs: *mut XineramaScreenInfo,
    pub xinerama_scr_regs: *mut XserverRegion,
    pub xinerama_nscrs: libc::c_int,
    pub xsync_exists: bool,
    pub xsync_event: libc::c_int,
    pub xsync_error: libc::c_int,
    pub xrfilter_convolution_exists: bool,
    pub atom_opacity: Atom,
    pub atom_frame_extents: Atom,
    pub atom_client: Atom,
    pub atom_name: Atom,
    pub atom_name_ewmh: Atom,
    pub atom_class: Atom,
    pub atom_role: Atom,
    pub atom_transient: Atom,
    pub atom_client_leader: Atom,
    pub atom_ewmh_active_win: Atom,
    pub atom_compton_shadow: Atom,
    pub atom_win_type: Atom,
    pub atoms_wintypes: [Atom; 15],
    pub track_atom_lst: *mut latom_t,
    pub dbus_conn: *mut DBusConnection,
    pub dbus_service: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win {
    pub next: *mut _win,
    pub prev_trans: *mut _win,
    pub id: Window,
    pub a: XWindowAttributes,
    pub xinerama_scr: libc::c_int,
    pub pictfmt: *mut XRenderPictFormat,
    pub mode: winmode_t,
    pub damaged: bool,
    pub fence: XSyncFence,
    pub pixmap_damaged: bool,
    pub damage: Damage,
    pub paint: paint_t,
    pub border_size: XserverRegion,
    pub extents: XserverRegion,
    pub flags: int_fast16_t,
    pub need_configure: bool,
    pub queue_configure: XConfigureEvent,
    pub reg_ignore: XserverRegion,
    pub widthb: libc::c_int,
    pub heightb: libc::c_int,
    pub destroyed: bool,
    pub bounding_shaped: bool,
    pub rounded_corners: bool,
    pub to_paint: bool,
    pub paint_excluded: bool,
    pub unredir_if_possible_excluded: bool,
    pub in_openclose: bool,
    pub client_win: Window,
    pub window_type: wintype_t,
    pub wmwin: bool,
    pub leader: Window,
    pub cache_leader: Window,
    pub focused: bool,
    pub focused_force: switch_t,
    pub name: *mut libc::c_char,
    pub class_instance: *mut libc::c_char,
    pub class_general: *mut libc::c_char,
    pub role: *mut libc::c_char,
    pub cache_sblst: *const c2_lptr_t,
    pub cache_fblst: *const c2_lptr_t,
    pub cache_fcblst: *const c2_lptr_t,
    pub cache_ivclst: *const c2_lptr_t,
    pub cache_bbblst: *const c2_lptr_t,
    pub cache_oparule: *const c2_lptr_t,
    pub cache_pblst: *const c2_lptr_t,
    pub cache_uipblst: *const c2_lptr_t,
    pub opacity: opacity_t,
    pub opacity_tgt: opacity_t,
    pub opacity_prop: opacity_t,
    pub opacity_prop_client: opacity_t,
    pub opacity_set: opacity_t,
    pub fade: bool,
    pub fade_last: bool,
    pub fade_force: switch_t,
    pub fade_callback: Option::<unsafe extern "C" fn(*mut session_t, *mut _win) -> ()>,
    pub frame_opacity: libc::c_double,
    pub frame_extents: margin_t,
    pub shadow: bool,
    pub shadow_last: bool,
    pub shadow_force: switch_t,
    pub shadow_opacity: libc::c_double,
    pub shadow_dx: libc::c_int,
    pub shadow_dy: libc::c_int,
    pub shadow_width: libc::c_int,
    pub shadow_height: libc::c_int,
    pub shadow_paint: paint_t,
    pub prop_shadow: libc::c_long,
    pub dim: bool,
    pub invert_color: bool,
    pub invert_color_last: bool,
    pub invert_color_force: switch_t,
    pub blur_background: bool,
    pub blur_background_last: bool,
    pub glx_blur_cache: glx_blur_cache_t,
}
pub type c2_lptr_t = _c2_lptr;
pub type options_t = _options_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _options_t {
    pub config_file: *mut libc::c_char,
    pub write_pid_path: *mut libc::c_char,
    pub display: *mut libc::c_char,
    pub display_repr: *mut libc::c_char,
    pub backend: backend,
    pub xrender_sync: bool,
    pub xrender_sync_fence: bool,
    pub glx_no_stencil: bool,
    pub glx_copy_from_front: bool,
    pub glx_use_copysubbuffermesa: bool,
    pub glx_no_rebind_pixmap: bool,
    pub glx_swap_method: libc::c_int,
    pub glx_use_gpushader4: bool,
    pub glx_fshader_win_str: *mut libc::c_char,
    pub glx_prog_win: glx_prog_main_t,
    pub fork_after_register: bool,
    pub detect_rounded_corners: bool,
    pub paint_on_overlay: bool,
    pub force_win_blend: bool,
    pub resize_damage: libc::c_int,
    pub unredir_if_possible: bool,
    pub unredir_if_possible_blacklist: *mut c2_lptr_t,
    pub unredir_if_possible_delay: time_ms_t,
    pub redirected_force: switch_t,
    pub stoppaint_force: switch_t,
    pub reredir_on_root_change: bool,
    pub glx_reinit_on_root_change: bool,
    pub dbus: bool,
    pub logpath: *mut libc::c_char,
    pub benchmark: libc::c_int,
    pub benchmark_wid: Window,
    pub paint_blacklist: *mut c2_lptr_t,
    pub no_name_pixmap: bool,
    pub synchronize: bool,
    pub show_all_xerrors: bool,
    pub no_x_selection: bool,
    pub refresh_rate: libc::c_int,
    pub sw_opti: bool,
    pub vsync: vsync_t,
    pub dbe: bool,
    pub vsync_aggressive: bool,
    pub vsync_use_glfinish: bool,
    pub wintype_shadow: [bool; 15],
    pub shadow_red: libc::c_double,
    pub shadow_green: libc::c_double,
    pub shadow_blue: libc::c_double,
    pub shadow_radius: libc::c_int,
    pub shadow_offset_x: libc::c_int,
    pub shadow_offset_y: libc::c_int,
    pub shadow_opacity: libc::c_double,
    pub clear_shadow: bool,
    pub shadow_exclude_reg_geom: geometry_t,
    pub shadow_blacklist: *mut c2_lptr_t,
    pub shadow_ignore_shaped: bool,
    pub respect_prop_shadow: bool,
    pub xinerama_shadow_crop: bool,
    pub wintype_fade: [bool; 15],
    pub fade_in_step: opacity_t,
    pub fade_out_step: opacity_t,
    pub fade_delta: time_ms_t,
    pub no_fading_openclose: bool,
    pub no_fading_destroyed_argb: bool,
    pub fade_blacklist: *mut c2_lptr_t,
    pub wintype_opacity: [libc::c_double; 15],
    pub inactive_opacity: opacity_t,
    pub active_opacity: opacity_t,
    pub inactive_opacity_override: bool,
    pub frame_opacity: libc::c_double,
    pub detect_client_opacity: bool,
    pub alpha_step: libc::c_double,
    pub blur_background: bool,
    pub blur_background_frame: bool,
    pub blur_background_fixed: bool,
    pub blur_background_blacklist: *mut c2_lptr_t,
    pub blur_kerns: [*mut XFixed; 5],
    pub inactive_dim: libc::c_double,
    pub inactive_dim_fixed: bool,
    pub invert_color_list: *mut c2_lptr_t,
    pub opacity_rules: *mut c2_lptr_t,
    pub wintype_focus: [bool; 15],
    pub mark_wmwin_focused: bool,
    pub mark_ovredir_focused: bool,
    pub use_ewmh_active_win: bool,
    pub focus_blacklist: *mut c2_lptr_t,
    pub detect_transient: bool,
    pub detect_client_leader: bool,
    pub track_focus: bool,
    pub track_wdata: bool,
    pub track_leader: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glx_session_t {
    pub context: GLXContext,
    pub has_texture_non_power_of_two: bool,
    pub glXGetVideoSyncSGI: f_GetVideoSync,
    pub glXWaitVideoSyncSGI: f_WaitVideoSync,
    pub glXGetSyncValuesOML: f_GetSyncValuesOML,
    pub glXWaitForMscOML: f_WaitForMscOML,
    pub glXSwapIntervalProc: f_SwapIntervalSGI,
    pub glXSwapIntervalMESAProc: f_SwapIntervalMESA,
    pub glXBindTexImageProc: f_BindTexImageEXT,
    pub glXReleaseTexImageProc: f_ReleaseTexImageEXT,
    pub glXCopySubBufferProc: f_CopySubBuffer,
    pub z: libc::c_int,
    pub fbconfigs: [*mut glx_fbconfig_t; 33],
    pub blur_passes: [glx_blur_pass_t; 5],
}
pub type win = _win;
#[inline]
unsafe extern "C" fn get_visualinfo_from_visual(
    mut ps: *mut session_t,
    mut visual: *mut Visual,
) -> *mut XVisualInfo {
    let mut vreq: XVisualInfo = {
        let mut init = XVisualInfo {
            visual: 0 as *mut Visual,
            visualid: XVisualIDFromVisual(visual),
            screen: 0,
            depth: 0,
            class: 0,
            red_mask: 0,
            green_mask: 0,
            blue_mask: 0,
            colormap_size: 0,
            bits_per_rgb: 0,
        };
        init
    };
    let mut nitems: libc::c_int = 0 as libc::c_int;
    return XGetVisualInfo(
        (*ps).dpy,
        0x1 as libc::c_int as libc::c_long,
        &mut vreq,
        &mut nitems,
    );
}
#[inline]
unsafe extern "C" fn glx_hasglxext(
    mut ps: *mut session_t,
    mut ext: *const libc::c_char,
) -> bool {
    let mut glx_exts: *const libc::c_char = glXQueryExtensionsString(
        (*ps).dpy,
        (*ps).scr,
    );
    if glx_exts.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed get GLX extension list.\n\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"glx_hasglxext\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    let mut found: bool = wd_is_in_str(glx_exts, ext);
    if !found {
        fprintf(
            stderr,
            b"%s(): Missing GLX extension %s.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"glx_hasglxext\0"))
                .as_ptr(),
            ext,
        );
    }
    return found;
}
#[inline]
unsafe extern "C" fn wd_is_in_str(
    mut haystick: *const libc::c_char,
    mut needle: *const libc::c_char,
) -> bool {
    if haystick.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut pos: *const libc::c_char = haystick.offset(-(1 as libc::c_int as isize));
    loop {
        pos = strstr(pos.offset(1 as libc::c_int as isize), needle);
        if pos.is_null() {
            break;
        }
        if pos.offset_from(haystick) as libc::c_long != 0
            && *(*__ctype_b_loc())
                .offset(
                    *pos.offset(-(1 as libc::c_int as isize)) as libc::c_int as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            || strlen(pos) > strlen(needle)
                && *(*__ctype_b_loc())
                    .offset(*pos.offset(strlen(needle) as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            continue;
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn glx_hasglext(
    mut ps: *mut session_t,
    mut ext: *const libc::c_char,
) -> bool {
    let mut gl_exts: *const libc::c_char = glGetString(0x1f03 as libc::c_int as GLenum)
        as *const libc::c_char;
    if gl_exts.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed get GL extension list.\n\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"glx_hasglext\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    let mut found: bool = wd_is_in_str(gl_exts, ext);
    if !found {
        fprintf(
            stderr,
            b"%s(): Missing GL extension %s.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"glx_hasglext\0"))
                .as_ptr(),
            ext,
        );
    }
    return found;
}
#[inline]
unsafe extern "C" fn free_texture(
    mut ps: *mut session_t,
    mut pptex: *mut *mut glx_texture_t,
) {
    let mut ptex: *mut glx_texture_t = *pptex;
    if ptex.is_null() {
        return;
    }
    glx_release_pixmap(ps, ptex);
    free_texture_r(ps, &mut (*ptex).texture);
    free(ptex as *mut libc::c_void);
    *pptex = 0 as *mut glx_texture_t;
}
#[inline]
unsafe extern "C" fn free_paint_glx(mut ps: *mut session_t, mut ppaint: *mut paint_t) {
    free_texture(ps, &mut (*ppaint).ptex);
}
#[inline]
unsafe extern "C" fn free_glx_fbo(mut ps: *mut session_t, mut pfbo: *mut GLuint) {
    if *pfbo != 0 {
        glDeleteFramebuffers(1 as libc::c_int, pfbo);
        *pfbo = 0 as libc::c_int as GLuint;
    }
}
#[inline]
unsafe extern "C" fn free_texture_r(mut ps: *mut session_t, mut ptexture: *mut GLuint) {
    if *ptexture != 0 {
        glDeleteTextures(1 as libc::c_int, ptexture);
        *ptexture = 0 as libc::c_int as GLuint;
    }
}
#[inline]
unsafe extern "C" fn free_glx_bc_resize(
    mut ps: *mut session_t,
    mut pbc: *mut glx_blur_cache_t,
) {
    free_texture_r(
        ps,
        &mut *((*pbc).textures).as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    free_texture_r(
        ps,
        &mut *((*pbc).textures).as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    (*pbc).width = 0 as libc::c_int;
    (*pbc).height = 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn free_glx_bc(
    mut ps: *mut session_t,
    mut pbc: *mut glx_blur_cache_t,
) {
    free_glx_fbo(ps, &mut (*pbc).fbo);
    free_glx_bc_resize(ps, pbc);
}
#[inline]
unsafe extern "C" fn free_win_res_glx(mut ps: *mut session_t, mut w: *mut win) {
    free_paint_glx(ps, &mut (*w).paint);
    free_paint_glx(ps, &mut (*w).shadow_paint);
    free_glx_bc(ps, &mut (*w).glx_blur_cache);
}
#[inline]
unsafe extern "C" fn rect_is_fullscreen(
    mut ps: *mut session_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut wid: libc::c_uint,
    mut hei: libc::c_uint,
) -> bool {
    return x <= 0 as libc::c_int && y <= 0 as libc::c_int
        && (x as libc::c_uint).wrapping_add(wid) >= (*ps).root_width as libc::c_uint
        && (y as libc::c_uint).wrapping_add(hei) >= (*ps).root_height as libc::c_uint;
}
#[inline]
unsafe extern "C" fn rect_crop(
    mut pdst: *mut XRectangle,
    mut psrc: *const XRectangle,
    mut pbound: *const XRectangle,
) {
    (*pdst)
        .x = max_i((*psrc).x as libc::c_int, (*pbound).x as libc::c_int)
        as libc::c_short;
    (*pdst)
        .y = max_i((*psrc).y as libc::c_int, (*pbound).y as libc::c_int)
        as libc::c_short;
    (*pdst)
        .width = max_i(
        0 as libc::c_int,
        min_i(
            (*psrc).x as libc::c_int + (*psrc).width as libc::c_int,
            (*pbound).x as libc::c_int + (*pbound).width as libc::c_int,
        ) - (*pdst).x as libc::c_int,
    ) as libc::c_ushort;
    (*pdst)
        .height = max_i(
        0 as libc::c_int,
        min_i(
            (*psrc).y as libc::c_int + (*psrc).height as libc::c_int,
            (*pbound).y as libc::c_int + (*pbound).height as libc::c_int,
        ) - (*pdst).y as libc::c_int,
    ) as libc::c_ushort;
}
#[inline]
unsafe extern "C" fn free_region(mut ps: *mut session_t, mut p: *mut XserverRegion) {
    if *p != 0 {
        XFixesDestroyRegion((*ps).dpy, *p);
        *p = 0 as libc::c_long as XserverRegion;
    }
}
#[inline]
unsafe extern "C" fn copy_region(
    mut ps: *const session_t,
    mut oldregion: XserverRegion,
) -> XserverRegion {
    if oldregion == 0 {
        return 0 as libc::c_long as XserverRegion;
    }
    let mut region: XserverRegion = XFixesCreateRegion(
        (*ps).dpy,
        0 as *mut XRectangle,
        0 as libc::c_int,
    );
    XFixesCopyRegion((*ps).dpy, region, oldregion);
    return region;
}
#[inline]
unsafe extern "C" fn get_tgt_window(mut ps: *mut session_t) -> Window {
    return if (*ps).o.paint_on_overlay as libc::c_int != 0 {
        (*ps).overlay
    } else {
        (*ps).root
    };
}
#[inline]
unsafe extern "C" fn cxfree(mut data: *mut libc::c_void) {
    if !data.is_null() {
        XFree(data);
    }
}
#[inline]
unsafe extern "C" fn min_i(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { b } else { a };
}
#[inline]
unsafe extern "C" fn max_i(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn mstrextend(
    mut psrc1: *mut *mut libc::c_char,
    mut src2: *const libc::c_char,
) {
    *psrc1 = allocchk_(
        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mstrextend\0"))
            .as_ptr(),
        realloc(
            *psrc1 as *mut libc::c_void,
            (if !(*psrc1).is_null() {
                strlen(*psrc1)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_add(strlen(src2))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ),
    ) as *mut libc::c_char;
    strcat(*psrc1, src2);
}
#[inline]
unsafe extern "C" fn mstrcpy(mut src: *const libc::c_char) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = allocchk_(
        (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"mstrcpy\0")).as_ptr(),
        malloc(
            (strlen(src))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ),
    ) as *mut libc::c_char;
    strcpy(str, src);
    return str;
}
#[inline]
unsafe extern "C" fn allocchk_(
    mut func_name: *const libc::c_char,
    mut ptr: *mut libc::c_void,
) -> *mut libc::c_void {
    if ptr.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed to allocate memory.\n\0" as *const u8 as *const libc::c_char,
            func_name,
        );
        exit(1 as libc::c_int);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn glx_init(
    mut ps: *mut session_t,
    mut need_render: bool,
) -> bool {
    let mut psglx: *mut glx_session_t = 0 as *mut glx_session_t;
    let mut current_block: u64;
    let mut success: bool = 0 as libc::c_int != 0;
    let mut pvis: *mut XVisualInfo = 0 as *mut XVisualInfo;
    if !(*ps).glx_exists {
        if glXQueryExtension((*ps).dpy, &mut (*ps).glx_event, &mut (*ps).glx_error) != 0
        {
            (*ps).glx_exists = 1 as libc::c_int != 0;
            current_block = 7502529970979898288;
        } else {
            fprintf(
                stderr,
                b"%s(): No GLX extension.\n\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"glx_init\0"))
                    .as_ptr(),
            );
            current_block = 4395476253999582535;
        }
    } else {
        current_block = 7502529970979898288;
    }
    match current_block {
        7502529970979898288 => {
            pvis = get_visualinfo_from_visual(ps, (*ps).vis);
            if pvis.is_null() {
                fprintf(
                    stderr,
                    b"%s(): Failed to acquire XVisualInfo for current visual.\n\0"
                        as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 9],
                        &[libc::c_char; 9],
                    >(b"glx_init\0"))
                        .as_ptr(),
                );
            } else {
                if need_render {
                    let mut value: libc::c_int = 0 as libc::c_int;
                    if 0 as libc::c_int
                        != glXGetConfig((*ps).dpy, pvis, 1 as libc::c_int, &mut value)
                        || value == 0
                    {
                        fprintf(
                            stderr,
                            b"%s(): Root visual is not a GL visual.\n\0" as *const u8
                                as *const libc::c_char,
                            (*::core::mem::transmute::<
                                &[u8; 9],
                                &[libc::c_char; 9],
                            >(b"glx_init\0"))
                                .as_ptr(),
                        );
                        current_block = 4395476253999582535;
                    } else if 0 as libc::c_int
                        != glXGetConfig((*ps).dpy, pvis, 5 as libc::c_int, &mut value)
                        || value == 0
                    {
                        fprintf(
                            stderr,
                            b"%s(): Root visual is not a double buffered GL visual.\n\0"
                                as *const u8 as *const libc::c_char,
                            (*::core::mem::transmute::<
                                &[u8; 9],
                                &[libc::c_char; 9],
                            >(b"glx_init\0"))
                                .as_ptr(),
                        );
                        current_block = 4395476253999582535;
                    } else {
                        current_block = 13586036798005543211;
                    }
                } else {
                    current_block = 13586036798005543211;
                }
                match current_block {
                    4395476253999582535 => {}
                    _ => {
                        if !(need_render as libc::c_int != 0
                            && !glx_hasglxext(
                                ps,
                                b"GLX_EXT_texture_from_pixmap\0" as *const u8
                                    as *const libc::c_char,
                            ))
                        {
                            if ((*ps).psglx).is_null() {
                                static mut CGLX_SESSION_DEF: glx_session_t = {
                                    let mut init = glx_session_t {
                                        context: 0 as *const __GLXcontextRec as GLXContext,
                                        has_texture_non_power_of_two: false,
                                        glXGetVideoSyncSGI: None,
                                        glXWaitVideoSyncSGI: None,
                                        glXGetSyncValuesOML: None,
                                        glXWaitForMscOML: None,
                                        glXSwapIntervalProc: None,
                                        glXSwapIntervalMESAProc: None,
                                        glXBindTexImageProc: None,
                                        glXReleaseTexImageProc: None,
                                        glXCopySubBufferProc: None,
                                        z: 0,
                                        fbconfigs: [0 as *const glx_fbconfig_t
                                            as *mut glx_fbconfig_t; 33],
                                        blur_passes: [glx_blur_pass_t {
                                            frag_shader: 0,
                                            prog: 0,
                                            unifm_offset_x: 0,
                                            unifm_offset_y: 0,
                                            unifm_factor_center: 0,
                                        }; 5],
                                    };
                                    init
                                };
                                (*ps)
                                    .psglx = allocchk_(
                                    (*::core::mem::transmute::<
                                        &[u8; 9],
                                        &[libc::c_char; 9],
                                    >(b"glx_init\0"))
                                        .as_ptr(),
                                    malloc(
                                        (1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<glx_session_t>() as libc::c_ulong,
                                            ),
                                    ),
                                ) as *mut glx_session_t;
                                memcpy(
                                    (*ps).psglx as *mut libc::c_void,
                                    &CGLX_SESSION_DEF as *const glx_session_t
                                        as *const libc::c_void,
                                    ::core::mem::size_of::<glx_session_t>() as libc::c_ulong,
                                );
                                let mut i: libc::c_int = 0 as libc::c_int;
                                while i < 5 as libc::c_int {
                                    let mut ppass: *mut glx_blur_pass_t = &mut *((*(*ps).psglx)
                                        .blur_passes)
                                        .as_mut_ptr()
                                        .offset(i as isize) as *mut glx_blur_pass_t;
                                    (*ppass).unifm_factor_center = -(1 as libc::c_int);
                                    (*ppass).unifm_offset_x = -(1 as libc::c_int);
                                    (*ppass).unifm_offset_y = -(1 as libc::c_int);
                                    i += 1;
                                    i;
                                }
                            }
                            psglx = (*ps).psglx;
                            if ((*psglx).context).is_null() {
                                (*psglx)
                                    .context = glXCreateContext(
                                    (*ps).dpy,
                                    pvis,
                                    0 as GLXContext,
                                    1 as libc::c_int,
                                );
                                if ((*psglx).context).is_null() {
                                    fprintf(
                                        stderr,
                                        b"%s(): Failed to get GLX context.\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*::core::mem::transmute::<
                                            &[u8; 9],
                                            &[libc::c_char; 9],
                                        >(b"glx_init\0"))
                                            .as_ptr(),
                                    );
                                    current_block = 4395476253999582535;
                                } else if glXMakeCurrent(
                                    (*ps).dpy,
                                    get_tgt_window(ps),
                                    (*psglx).context,
                                ) == 0
                                {
                                    fprintf(
                                        stderr,
                                        b"%s(): Failed to attach GLX context.\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*::core::mem::transmute::<
                                            &[u8; 9],
                                            &[libc::c_char; 9],
                                        >(b"glx_init\0"))
                                            .as_ptr(),
                                    );
                                    current_block = 4395476253999582535;
                                } else {
                                    current_block = 16924917904204750491;
                                }
                            } else {
                                current_block = 16924917904204750491;
                            }
                            match current_block {
                                4395476253999582535 => {}
                                _ => {
                                    if need_render as libc::c_int != 0
                                        && !(*ps).o.glx_no_stencil
                                    {
                                        let mut val: GLint = 0 as libc::c_int;
                                        glGetIntegerv(0xd57 as libc::c_int as GLenum, &mut val);
                                        if val == 0 {
                                            fprintf(
                                                stderr,
                                                b"%s(): Target window doesn't have stencil buffer.\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                (*::core::mem::transmute::<
                                                    &[u8; 9],
                                                    &[libc::c_char; 9],
                                                >(b"glx_init\0"))
                                                    .as_ptr(),
                                            );
                                            current_block = 4395476253999582535;
                                        } else {
                                            current_block = 8704759739624374314;
                                        }
                                    } else {
                                        current_block = 8704759739624374314;
                                    }
                                    match current_block {
                                        4395476253999582535 => {}
                                        _ => {
                                            if need_render {
                                                (*psglx)
                                                    .has_texture_non_power_of_two = glx_hasglext(
                                                    ps,
                                                    b"GL_ARB_texture_non_power_of_two\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                            if need_render {
                                                (*psglx)
                                                    .glXBindTexImageProc = ::core::mem::transmute::<
                                                    Option::<unsafe extern "C" fn() -> ()>,
                                                    f_BindTexImageEXT,
                                                >(
                                                    glXGetProcAddress(
                                                        b"glXBindTexImageEXT\0" as *const u8 as *const libc::c_char
                                                            as *const GLubyte,
                                                    ),
                                                );
                                                (*psglx)
                                                    .glXReleaseTexImageProc = ::core::mem::transmute::<
                                                    Option::<unsafe extern "C" fn() -> ()>,
                                                    f_ReleaseTexImageEXT,
                                                >(
                                                    glXGetProcAddress(
                                                        b"glXReleaseTexImageEXT\0" as *const u8
                                                            as *const libc::c_char as *const GLubyte,
                                                    ),
                                                );
                                                if ((*psglx).glXBindTexImageProc).is_none()
                                                    || ((*psglx).glXReleaseTexImageProc).is_none()
                                                {
                                                    fprintf(
                                                        stderr,
                                                        b"%s(): Failed to acquire glXBindTexImageEXT() / glXReleaseTexImageEXT().\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*::core::mem::transmute::<
                                                            &[u8; 9],
                                                            &[libc::c_char; 9],
                                                        >(b"glx_init\0"))
                                                            .as_ptr(),
                                                    );
                                                    current_block = 4395476253999582535;
                                                } else if (*ps).o.glx_use_copysubbuffermesa {
                                                    (*psglx)
                                                        .glXCopySubBufferProc = ::core::mem::transmute::<
                                                        Option::<unsafe extern "C" fn() -> ()>,
                                                        f_CopySubBuffer,
                                                    >(
                                                        glXGetProcAddress(
                                                            b"glXCopySubBufferMESA\0" as *const u8
                                                                as *const libc::c_char as *const GLubyte,
                                                        ),
                                                    );
                                                    if ((*psglx).glXCopySubBufferProc).is_none() {
                                                        fprintf(
                                                            stderr,
                                                            b"%s(): Failed to acquire glXCopySubBufferMESA().\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            (*::core::mem::transmute::<
                                                                &[u8; 9],
                                                                &[libc::c_char; 9],
                                                            >(b"glx_init\0"))
                                                                .as_ptr(),
                                                        );
                                                        current_block = 4395476253999582535;
                                                    } else {
                                                        current_block = 790185930182612747;
                                                    }
                                                } else {
                                                    current_block = 790185930182612747;
                                                }
                                            } else {
                                                current_block = 790185930182612747;
                                            }
                                            match current_block {
                                                4395476253999582535 => {}
                                                _ => {
                                                    if !(need_render as libc::c_int != 0
                                                        && !glx_update_fbconfig(ps))
                                                    {
                                                        if need_render {
                                                            glx_on_root_change(ps);
                                                            glDisable(0xb71 as libc::c_int as GLenum);
                                                            glDepthMask(0 as libc::c_int as GLboolean);
                                                            glTexEnvi(
                                                                0x2300 as libc::c_int as GLenum,
                                                                0x2200 as libc::c_int as GLenum,
                                                                0x1e01 as libc::c_int,
                                                            );
                                                            glDisable(0xbe2 as libc::c_int as GLenum);
                                                            if !(*ps).o.glx_no_stencil {
                                                                glClear(0x400 as libc::c_int as GLbitfield);
                                                                glDisable(0xb90 as libc::c_int as GLenum);
                                                                glStencilMask(0x1 as libc::c_int as GLuint);
                                                                glStencilFunc(
                                                                    0x202 as libc::c_int as GLenum,
                                                                    0x1 as libc::c_int,
                                                                    0x1 as libc::c_int as GLuint,
                                                                );
                                                            }
                                                            glClearColor(0.0f32, 0.0f32, 0.0f32, 1.0f32);
                                                        }
                                                        success = 1 as libc::c_int != 0;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    cxfree(pvis as *mut libc::c_void);
    if !success {
        glx_destroy(ps);
    }
    return success;
}
unsafe extern "C" fn glx_free_prog_main(
    mut ps: *mut session_t,
    mut pprogram: *mut glx_prog_main_t,
) {
    if pprogram.is_null() {
        return;
    }
    if (*pprogram).prog != 0 {
        glDeleteProgram((*pprogram).prog);
        (*pprogram).prog = 0 as libc::c_int as GLuint;
    }
    (*pprogram).unifm_opacity = -(1 as libc::c_int);
    (*pprogram).unifm_invert_color = -(1 as libc::c_int);
    (*pprogram).unifm_tex = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn glx_destroy(mut ps: *mut session_t) {
    if ((*ps).psglx).is_null() {
        return;
    }
    let mut w: *mut win = (*ps).list;
    while !w.is_null() {
        free_win_res_glx(ps, w);
        w = (*w).next;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        let mut ppass: *mut glx_blur_pass_t = &mut *((*(*ps).psglx).blur_passes)
            .as_mut_ptr()
            .offset(i as isize) as *mut glx_blur_pass_t;
        if (*ppass).frag_shader != 0 {
            glDeleteShader((*ppass).frag_shader);
        }
        if (*ppass).prog != 0 {
            glDeleteProgram((*ppass).prog);
        }
        i += 1;
        i;
    }
    glx_free_prog_main(ps, &mut (*ps).o.glx_prog_win);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 <= 32 as libc::c_int {
        free((*(*ps).psglx).fbconfigs[i_0 as usize] as *mut libc::c_void);
        (*(*ps).psglx).fbconfigs[i_0 as usize] = 0 as *mut glx_fbconfig_t;
        i_0 += 1;
        i_0;
    }
    if !((*(*ps).psglx).context).is_null() {
        glXDestroyContext((*ps).dpy, (*(*ps).psglx).context);
        (*(*ps).psglx).context = 0 as GLXContext;
    }
    free((*ps).psglx as *mut libc::c_void);
    (*ps).psglx = 0 as *mut glx_session_t;
}
#[no_mangle]
pub unsafe extern "C" fn glx_reinit(
    mut ps: *mut session_t,
    mut need_render: bool,
) -> bool {
    vsync_deinit(ps);
    glx_destroy(ps);
    if !glx_init(ps, need_render) {
        fprintf(
            stderr,
            b"%s(): Failed to initialize GLX.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"glx_reinit\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    if !vsync_init(ps) {
        fprintf(
            stderr,
            b"%s(): Failed to initialize VSync.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"glx_reinit\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn glx_on_root_change(mut ps: *mut session_t) {
    glViewport(0 as libc::c_int, 0 as libc::c_int, (*ps).root_width, (*ps).root_height);
    glMatrixMode(0x1701 as libc::c_int as GLenum);
    glLoadIdentity();
    glOrtho(
        0 as libc::c_int as GLdouble,
        (*ps).root_width as GLdouble,
        0 as libc::c_int as GLdouble,
        (*ps).root_height as GLdouble,
        -1000.0f64,
        1000.0f64,
    );
    glMatrixMode(0x1700 as libc::c_int as GLenum);
    glLoadIdentity();
}
#[no_mangle]
pub unsafe extern "C" fn glx_init_blur(mut ps: *mut session_t) -> bool {
    if !((*ps).o.blur_kerns[1 as libc::c_int as usize]).is_null() {
        let mut fbo: GLuint = 0 as libc::c_int as GLuint;
        glGenFramebuffers(1 as libc::c_int, &mut fbo);
        if fbo == 0 {
            fprintf(
                stderr,
                b"%s(): Failed to generate Framebuffer. Cannot do multi-pass blur with GLX backend.\n\0"
                    as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"glx_init_blur\0"))
                    .as_ptr(),
            );
            return 0 as libc::c_int != 0;
        }
        glDeleteFramebuffers(1 as libc::c_int, &mut fbo);
    }
    let mut lc_numeric_old: *mut libc::c_char = mstrcpy(
        setlocale(1 as libc::c_int, 0 as *const libc::c_char),
    );
    setlocale(1 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
    static mut FRAG_SHADER_BLUR_PREFIX: *const libc::c_char = b"#version 110\n%suniform float offset_x;\nuniform float offset_y;\nuniform float factor_center;\nuniform %s tex_scr;\n\nvoid main() {\n  vec4 sum = vec4(0.0, 0.0, 0.0, 0.0);\n\0"
        as *const u8 as *const libc::c_char;
    static mut FRAG_SHADER_BLUR_ADD: *const libc::c_char = b"  sum += float(%.7g) * %s(tex_scr, vec2(gl_TexCoord[0].x + offset_x * float(%d), gl_TexCoord[0].y + offset_y * float(%d)));\n\0"
        as *const u8 as *const libc::c_char;
    static mut FRAG_SHADER_BLUR_ADD_GPUSHADER4: *const libc::c_char = b"  sum += float(%.7g) * %sOffset(tex_scr, vec2(gl_TexCoord[0].x, gl_TexCoord[0].y), ivec2(%d, %d));\n\0"
        as *const u8 as *const libc::c_char;
    static mut FRAG_SHADER_BLUR_SUFFIX: *const libc::c_char = b"  sum += %s(tex_scr, vec2(gl_TexCoord[0].x, gl_TexCoord[0].y)) * factor_center;\n  gl_FragColor = sum / (factor_center + float(%.7g));\n}\n\0"
        as *const u8 as *const libc::c_char;
    let use_texture_rect: bool = !(*(*ps).psglx).has_texture_non_power_of_two;
    let mut sampler_type: *const libc::c_char = if use_texture_rect as libc::c_int != 0 {
        b"sampler2DRect\0" as *const u8 as *const libc::c_char
    } else {
        b"sampler2D\0" as *const u8 as *const libc::c_char
    };
    let mut texture_func: *const libc::c_char = if use_texture_rect as libc::c_int != 0 {
        b"texture2DRect\0" as *const u8 as *const libc::c_char
    } else {
        b"texture2D\0" as *const u8 as *const libc::c_char
    };
    let mut shader_add: *const libc::c_char = FRAG_SHADER_BLUR_ADD;
    let mut extension: *mut libc::c_char = mstrcpy(
        b"\0" as *const u8 as *const libc::c_char,
    );
    if use_texture_rect {
        mstrextend(
            &mut extension,
            b"#extension GL_ARB_texture_rectangle : require\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*ps).o.glx_use_gpushader4 {
        mstrextend(
            &mut extension,
            b"#extension GL_EXT_gpu_shader4 : require\n\0" as *const u8
                as *const libc::c_char,
        );
        shader_add = FRAG_SHADER_BLUR_ADD_GPUSHADER4;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 5 as libc::c_int && !((*ps).o.blur_kerns[i as usize]).is_null() {
        let mut kern: *mut XFixed = (*ps).o.blur_kerns[i as usize];
        if kern.is_null() {
            break;
        }
        let mut ppass: *mut glx_blur_pass_t = &mut *((*(*ps).psglx).blur_passes)
            .as_mut_ptr()
            .offset(i as isize) as *mut glx_blur_pass_t;
        let mut wid: libc::c_int = (*kern.offset(0 as libc::c_int as isize) as XDouble
            / 65536 as libc::c_int as libc::c_double) as libc::c_int;
        let mut hei: libc::c_int = (*kern.offset(1 as libc::c_int as isize) as XDouble
            / 65536 as libc::c_int as libc::c_double) as libc::c_int;
        let mut nele: libc::c_int = wid * hei - 1 as libc::c_int;
        let mut len: libc::c_int = (strlen(FRAG_SHADER_BLUR_PREFIX))
            .wrapping_add(strlen(sampler_type))
            .wrapping_add(strlen(extension))
            .wrapping_add(
                (strlen(shader_add))
                    .wrapping_add(strlen(texture_func))
                    .wrapping_add(42 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(nele as libc::c_ulong),
            )
            .wrapping_add(strlen(FRAG_SHADER_BLUR_SUFFIX))
            .wrapping_add(strlen(texture_func))
            .wrapping_add(12 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        let mut shader_str: *mut libc::c_char = calloc(
            len as libc::c_ulong,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
        if shader_str.is_null() {
            fprintf(
                stderr,
                b"%s(): Failed to allocate %d bytes for shader string.\n\0" as *const u8
                    as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"glx_init_blur\0"))
                    .as_ptr(),
                len,
            );
            return 0 as libc::c_int != 0;
        }
        let mut pc: *mut libc::c_char = shader_str;
        sprintf(pc, FRAG_SHADER_BLUR_PREFIX, extension, sampler_type);
        pc = pc.offset(strlen(pc) as isize);
        let mut sum: libc::c_double = 0.0f64;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < hei {
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < wid {
                if !(hei / 2 as libc::c_int == j && wid / 2 as libc::c_int == k) {
                    let mut val: libc::c_double = *kern
                        .offset((2 as libc::c_int + j * wid + k) as isize) as XDouble
                        / 65536 as libc::c_int as libc::c_double;
                    if !(0.0f64 == val) {
                        sum += val;
                        sprintf(
                            pc,
                            shader_add,
                            val,
                            texture_func,
                            k - wid / 2 as libc::c_int,
                            j - hei / 2 as libc::c_int,
                        );
                        pc = pc.offset(strlen(pc) as isize);
                    }
                }
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        sprintf(pc, FRAG_SHADER_BLUR_SUFFIX, texture_func, sum);
        (*ppass)
            .frag_shader = glx_create_shader(
            0x8b30 as libc::c_int as GLenum,
            shader_str,
        );
        free(shader_str as *mut libc::c_void);
        if (*ppass).frag_shader == 0 {
            fprintf(
                stderr,
                b"%s(): Failed to create fragment shader %d.\n\0" as *const u8
                    as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"glx_init_blur\0"))
                    .as_ptr(),
                i,
            );
            return 0 as libc::c_int != 0;
        }
        (*ppass).prog = glx_create_program(&mut (*ppass).frag_shader, 1 as libc::c_int);
        if (*ppass).prog == 0 {
            fprintf(
                stderr,
                b"%s(): Failed to create GLSL program.\n\0" as *const u8
                    as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"glx_init_blur\0"))
                    .as_ptr(),
            );
            return 0 as libc::c_int != 0;
        }
        (*ppass)
            .unifm_factor_center = glGetUniformLocation(
            (*ppass).prog,
            b"factor_center\0" as *const u8 as *const libc::c_char,
        );
        if (*ppass).unifm_factor_center < 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s(): Failed to get location of %d-th uniform 'factor_center'. Might be troublesome.\n\0"
                    as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"glx_init_blur\0"))
                    .as_ptr(),
                i,
            );
        }
        if !(*ps).o.glx_use_gpushader4 {
            (*ppass)
                .unifm_offset_x = glGetUniformLocation(
                (*ppass).prog,
                b"offset_x\0" as *const u8 as *const libc::c_char,
            );
            if (*ppass).unifm_offset_x < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s(): Failed to get location of %d-th uniform 'offset_x'. Might be troublesome.\n\0"
                        as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 14],
                        &[libc::c_char; 14],
                    >(b"glx_init_blur\0"))
                        .as_ptr(),
                    i,
                );
            }
            (*ppass)
                .unifm_offset_y = glGetUniformLocation(
                (*ppass).prog,
                b"offset_y\0" as *const u8 as *const libc::c_char,
            );
            if (*ppass).unifm_offset_y < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s(): Failed to get location of %d-th uniform 'offset_y'. Might be troublesome.\n\0"
                        as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 14],
                        &[libc::c_char; 14],
                    >(b"glx_init_blur\0"))
                        .as_ptr(),
                    i,
                );
            }
        }
        i += 1;
        i;
    }
    free(extension as *mut libc::c_void);
    setlocale(1 as libc::c_int, lc_numeric_old);
    free(lc_numeric_old as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn glx_load_prog_main(
    mut ps: *mut session_t,
    mut vshader_str: *const libc::c_char,
    mut fshader_str: *const libc::c_char,
    mut pprogram: *mut glx_prog_main_t,
) -> bool {
    (*pprogram).prog = glx_create_program_from_str(vshader_str, fshader_str);
    if (*pprogram).prog == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to create GLSL program.\n\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"glx_load_prog_main\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    (*pprogram)
        .unifm_opacity = glGetUniformLocation(
        (*pprogram).prog,
        b"opacity\0" as *const u8 as *const libc::c_char,
    );
    if (*pprogram).unifm_opacity < 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s(): Failed to get location of uniform 'opacity'. Might be troublesome.\n\0"
                as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"glx_load_prog_main\0"))
                .as_ptr(),
        );
    }
    (*pprogram)
        .unifm_invert_color = glGetUniformLocation(
        (*pprogram).prog,
        b"invert_color\0" as *const u8 as *const libc::c_char,
    );
    if (*pprogram).unifm_invert_color < 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s(): Failed to get location of uniform 'invert_color'. Might be troublesome.\n\0"
                as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"glx_load_prog_main\0"))
                .as_ptr(),
        );
    }
    (*pprogram)
        .unifm_tex = glGetUniformLocation(
        (*pprogram).prog,
        b"tex\0" as *const u8 as *const libc::c_char,
    );
    if (*pprogram).unifm_tex < 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s(): Failed to get location of uniform 'tex'. Might be troublesome.\n\0"
                as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"glx_load_prog_main\0"))
                .as_ptr(),
        );
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn glx_update_fbconfig_bydepth(
    mut ps: *mut session_t,
    mut depth: libc::c_int,
    mut pfbcfg: *mut glx_fbconfig_t,
) {
    if depth < 0 as libc::c_int || depth > 32 as libc::c_int {
        return;
    }
    if glx_cmp_fbconfig(ps, (*(*ps).psglx).fbconfigs[depth as usize], pfbcfg)
        < 0 as libc::c_int
    {
        if ((*(*ps).psglx).fbconfigs[depth as usize]).is_null() {
            (*(*ps).psglx)
                .fbconfigs[depth
                as usize] = malloc(
                ::core::mem::size_of::<glx_fbconfig_t>() as libc::c_ulong,
            ) as *mut glx_fbconfig_t;
            allocchk_(
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"glx_update_fbconfig_bydepth\0"))
                    .as_ptr(),
                (*(*ps).psglx).fbconfigs[depth as usize] as *mut libc::c_void,
            );
        }
        *(*(*ps).psglx).fbconfigs[depth as usize] = *pfbcfg;
    }
}
unsafe extern "C" fn glx_update_fbconfig(mut ps: *mut session_t) -> bool {
    let mut nele: libc::c_int = 0 as libc::c_int;
    let mut pfbcfgs: *mut GLXFBConfig = glXGetFBConfigs((*ps).dpy, (*ps).scr, &mut nele);
    let mut pcur: *mut GLXFBConfig = pfbcfgs;
    while pcur < pfbcfgs.offset(nele as isize) {
        let mut fbinfo: glx_fbconfig_t = {
            let mut init = glx_fbconfig_t {
                cfg: *pcur,
                texture_fmt: 0 as libc::c_int,
                texture_tgts: 0 as libc::c_int,
                y_inverted: 0 as libc::c_int != 0,
            };
            init
        };
        let mut id: libc::c_int = pcur.offset_from(pfbcfgs) as libc::c_long
            as libc::c_int;
        let mut depth: libc::c_int = 0 as libc::c_int;
        let mut depth_alpha: libc::c_int = 0 as libc::c_int;
        let mut val: libc::c_int = 0 as libc::c_int;
        if !(0 as libc::c_int
            == glXGetFBConfigAttrib((*ps).dpy, *pcur, 0x186a1 as libc::c_int, &mut val)
            && val > 1 as libc::c_int)
        {
            if 0 as libc::c_int
                != glXGetFBConfigAttrib((*ps).dpy, *pcur, 2 as libc::c_int, &mut depth)
                || 0 as libc::c_int
                    != glXGetFBConfigAttrib(
                        (*ps).dpy,
                        *pcur,
                        11 as libc::c_int,
                        &mut depth_alpha,
                    )
            {
                fprintf(
                    stderr,
                    b"%s(): Failed to retrieve buffer size and alpha size of FBConfig %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 20],
                        &[libc::c_char; 20],
                    >(b"glx_update_fbconfig\0"))
                        .as_ptr(),
                    id,
                );
            } else if 0 as libc::c_int
                != glXGetFBConfigAttrib(
                    (*ps).dpy,
                    *pcur,
                    0x20d3 as libc::c_int,
                    &mut fbinfo.texture_tgts,
                )
            {
                fprintf(
                    stderr,
                    b"%s(): Failed to retrieve BIND_TO_TEXTURE_TARGETS_EXT of FBConfig %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 20],
                        &[libc::c_char; 20],
                    >(b"glx_update_fbconfig\0"))
                        .as_ptr(),
                    id,
                );
            } else {
                let mut visualdepth: libc::c_int = 0 as libc::c_int;
                let mut pvi: *mut XVisualInfo = glXGetVisualFromFBConfig(
                    (*ps).dpy,
                    *pcur,
                );
                if !pvi.is_null() {
                    visualdepth = (*pvi).depth;
                    cxfree(pvi as *mut libc::c_void);
                    let mut rgb: bool = 0 as libc::c_int != 0;
                    let mut rgba: bool = 0 as libc::c_int != 0;
                    if depth >= 32 as libc::c_int && depth_alpha != 0
                        && 0 as libc::c_int
                            == glXGetFBConfigAttrib(
                                (*ps).dpy,
                                *pcur,
                                0x20d1 as libc::c_int,
                                &mut val,
                            ) && val != 0
                    {
                        rgba = 1 as libc::c_int != 0;
                    }
                    if 0 as libc::c_int
                        == glXGetFBConfigAttrib(
                            (*ps).dpy,
                            *pcur,
                            0x20d0 as libc::c_int,
                            &mut val,
                        ) && val != 0
                    {
                        rgb = 1 as libc::c_int != 0;
                    }
                    if 0 as libc::c_int
                        == glXGetFBConfigAttrib(
                            (*ps).dpy,
                            *pcur,
                            0x20d4 as libc::c_int,
                            &mut val,
                        )
                    {
                        fbinfo.y_inverted = val != 0;
                    }
                    let mut tgtdpt: libc::c_int = depth - depth_alpha;
                    if tgtdpt == visualdepth && tgtdpt < 32 as libc::c_int
                        && rgb as libc::c_int != 0
                    {
                        fbinfo.texture_fmt = 0x20d9 as libc::c_int;
                        glx_update_fbconfig_bydepth(ps, tgtdpt, &mut fbinfo);
                    }
                    if depth == visualdepth && rgba as libc::c_int != 0 {
                        fbinfo.texture_fmt = 0x20da as libc::c_int;
                        glx_update_fbconfig_bydepth(ps, depth, &mut fbinfo);
                    }
                }
            }
        }
        pcur = pcur.offset(1);
        pcur;
    }
    cxfree(pfbcfgs as *mut libc::c_void);
    if ((*(*ps).psglx).fbconfigs[(*ps).depth as usize]).is_null() {
        fprintf(
            stderr,
            b"%s(): No FBConfig found for default depth %d.\n\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"glx_update_fbconfig\0"))
                .as_ptr(),
            (*ps).depth,
        );
        return 0 as libc::c_int != 0;
    }
    if ((*(*ps).psglx).fbconfigs[32 as libc::c_int as usize]).is_null() {
        fprintf(
            stderr,
            b"%s(): No FBConfig found for depth 32. Expect crazy things.\n\0"
                as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"glx_update_fbconfig\0"))
                .as_ptr(),
        );
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn glx_cmp_fbconfig_cmpattr(
    mut ps: *mut session_t,
    mut pfbc_a: *const glx_fbconfig_t,
    mut pfbc_b: *const glx_fbconfig_t,
    mut attr: libc::c_int,
) -> libc::c_int {
    let mut attr_a: libc::c_int = 0 as libc::c_int;
    let mut attr_b: libc::c_int = 0 as libc::c_int;
    glXGetFBConfigAttrib((*ps).dpy, (*pfbc_a).cfg, attr, &mut attr_a);
    glXGetFBConfigAttrib((*ps).dpy, (*pfbc_b).cfg, attr, &mut attr_b);
    return attr_a - attr_b;
}
unsafe extern "C" fn glx_cmp_fbconfig(
    mut ps: *mut session_t,
    mut pfbc_a: *const glx_fbconfig_t,
    mut pfbc_b: *const glx_fbconfig_t,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    if pfbc_a.is_null() {
        return -(1 as libc::c_int);
    }
    if pfbc_b.is_null() {
        return 1 as libc::c_int;
    }
    result = glx_cmp_fbconfig_cmpattr(ps, pfbc_a, pfbc_b, 0x20d1 as libc::c_int);
    if result != 0 {
        return -result;
    }
    result = glx_cmp_fbconfig_cmpattr(ps, pfbc_a, pfbc_b, 5 as libc::c_int);
    if result != 0 {
        return -result;
    }
    result = glx_cmp_fbconfig_cmpattr(ps, pfbc_a, pfbc_b, 13 as libc::c_int);
    if result != 0 {
        return -result;
    }
    result = glx_cmp_fbconfig_cmpattr(ps, pfbc_a, pfbc_b, 12 as libc::c_int);
    if result != 0 {
        return -result;
    }
    result = glx_cmp_fbconfig_cmpattr(ps, pfbc_a, pfbc_b, 0x20d2 as libc::c_int);
    if result != 0 {
        return result;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn glx_bind_pixmap(
    mut ps: *mut session_t,
    mut pptex: *mut *mut glx_texture_t,
    mut pixmap: Pixmap,
    mut width: libc::c_uint,
    mut height: libc::c_uint,
    mut depth: libc::c_uint,
) -> bool {
    if pixmap == 0 {
        fprintf(
            stderr,
            b"%s(%#010lx): Binding to an empty pixmap. This can't work.\n\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"glx_bind_pixmap\0"))
                .as_ptr(),
            pixmap,
        );
        return 0 as libc::c_int != 0;
    }
    let mut ptex: *mut glx_texture_t = *pptex;
    let mut need_release: bool = 1 as libc::c_int != 0;
    if ptex.is_null() {
        static mut GLX_TEX_DEF: glx_texture_t = {
            let mut init = _glx_texture {
                texture: 0 as libc::c_int as GLuint,
                glpixmap: 0 as libc::c_int as GLXPixmap,
                pixmap: 0 as libc::c_int as Pixmap,
                target: 0 as libc::c_int as GLenum,
                width: 0 as libc::c_int as libc::c_uint,
                height: 0 as libc::c_int as libc::c_uint,
                depth: 0 as libc::c_int as libc::c_uint,
                y_inverted: 0 as libc::c_int != 0,
            };
            init
        };
        ptex = malloc(::core::mem::size_of::<glx_texture_t>() as libc::c_ulong)
            as *mut glx_texture_t;
        allocchk_(
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"glx_bind_pixmap\0"))
                .as_ptr(),
            ptex as *mut libc::c_void,
        );
        memcpy(
            ptex as *mut libc::c_void,
            &GLX_TEX_DEF as *const glx_texture_t as *const libc::c_void,
            ::core::mem::size_of::<glx_texture_t>() as libc::c_ulong,
        );
        *pptex = ptex;
    }
    if (*ptex).texture != 0 && (*ptex).pixmap != pixmap {
        glx_release_pixmap(ps, ptex);
    }
    if (*ptex).glpixmap == 0 {
        need_release = 0 as libc::c_int != 0;
        if !(width != 0 && height != 0 && depth != 0) {
            let mut rroot: Window = 0 as libc::c_long as Window;
            let mut rx: libc::c_int = 0 as libc::c_int;
            let mut ry: libc::c_int = 0 as libc::c_int;
            let mut rbdwid: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            if XGetGeometry(
                (*ps).dpy,
                pixmap,
                &mut rroot,
                &mut rx,
                &mut ry,
                &mut width,
                &mut height,
                &mut rbdwid,
                &mut depth,
            ) == 0
            {
                fprintf(
                    stderr,
                    b"%s(%#010lx): Failed to query Pixmap info.\n\0" as *const u8
                        as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 16],
                        &[libc::c_char; 16],
                    >(b"glx_bind_pixmap\0"))
                        .as_ptr(),
                    pixmap,
                );
                return 0 as libc::c_int != 0;
            }
            if depth > 32 as libc::c_int as libc::c_uint {
                fprintf(
                    stderr,
                    b"%s(%d): Requested depth higher than %d.\n\0" as *const u8
                        as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 16],
                        &[libc::c_char; 16],
                    >(b"glx_bind_pixmap\0"))
                        .as_ptr(),
                    depth,
                    32 as libc::c_int,
                );
                return 0 as libc::c_int != 0;
            }
        }
        let mut pcfg: *const glx_fbconfig_t = (*(*ps).psglx).fbconfigs[depth as usize];
        if pcfg.is_null() {
            fprintf(
                stderr,
                b"%s(%d): Couldn't find FBConfig with requested depth.\n\0" as *const u8
                    as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"glx_bind_pixmap\0"))
                    .as_ptr(),
                depth,
            );
            return 0 as libc::c_int != 0;
        }
        let mut tex_tgt: GLenum = 0 as libc::c_int as GLenum;
        if 0x2 as libc::c_int & (*pcfg).texture_tgts != 0
            && (*(*ps).psglx).has_texture_non_power_of_two as libc::c_int != 0
        {
            tex_tgt = 0x20dc as libc::c_int as GLenum;
        } else if 0x4 as libc::c_int & (*pcfg).texture_tgts != 0 {
            tex_tgt = 0x20dd as libc::c_int as GLenum;
        } else if 0x2 as libc::c_int & (*pcfg).texture_tgts == 0 {
            tex_tgt = 0x20dd as libc::c_int as GLenum;
        } else {
            tex_tgt = 0x20dc as libc::c_int as GLenum;
        }
        let mut attrs: [GLint; 5] = [
            0x20d5 as libc::c_int,
            (*pcfg).texture_fmt,
            0x20d6 as libc::c_int,
            tex_tgt as GLint,
            0 as libc::c_int,
        ];
        (*ptex)
            .glpixmap = glXCreatePixmap(
            (*ps).dpy,
            (*pcfg).cfg,
            pixmap,
            attrs.as_mut_ptr(),
        );
        (*ptex).pixmap = pixmap;
        (*ptex)
            .target = (if 0x20dc as libc::c_int as libc::c_uint == tex_tgt {
            0xde1 as libc::c_int
        } else {
            0x84f5 as libc::c_int
        }) as GLenum;
        (*ptex).width = width;
        (*ptex).height = height;
        (*ptex).depth = depth;
        (*ptex).y_inverted = (*pcfg).y_inverted;
    }
    if (*ptex).glpixmap == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to allocate GLX pixmap.\n\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"glx_bind_pixmap\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    glEnable((*ptex).target);
    if (*ptex).texture == 0 {
        need_release = 0 as libc::c_int != 0;
        let mut texture: GLuint = 0 as libc::c_int as GLuint;
        glGenTextures(1 as libc::c_int, &mut texture);
        glBindTexture((*ptex).target, texture);
        glTexParameteri(
            (*ptex).target,
            0x2801 as libc::c_int as GLenum,
            0x2600 as libc::c_int,
        );
        glTexParameteri(
            (*ptex).target,
            0x2800 as libc::c_int as GLenum,
            0x2600 as libc::c_int,
        );
        glTexParameteri(
            (*ptex).target,
            0x2802 as libc::c_int as GLenum,
            0x812f as libc::c_int,
        );
        glTexParameteri(
            (*ptex).target,
            0x2803 as libc::c_int as GLenum,
            0x812f as libc::c_int,
        );
        glBindTexture((*ptex).target, 0 as libc::c_int as GLuint);
        (*ptex).texture = texture;
    }
    if (*ptex).texture == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to allocate texture.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"glx_bind_pixmap\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    glBindTexture((*ptex).target, (*ptex).texture);
    if need_release {
        ((*(*ps).psglx).glXReleaseTexImageProc)
            .expect(
                "non-null function pointer",
            )((*ps).dpy, (*ptex).glpixmap, 0x20de as libc::c_int);
    }
    ((*(*ps).psglx).glXBindTexImageProc)
        .expect(
            "non-null function pointer",
        )((*ps).dpy, (*ptex).glpixmap, 0x20de as libc::c_int, 0 as *const libc::c_int);
    glBindTexture((*ptex).target, 0 as libc::c_int as GLuint);
    glDisable((*ptex).target);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn glx_release_pixmap(
    mut ps: *mut session_t,
    mut ptex: *mut glx_texture_t,
) {
    if (*ptex).glpixmap != 0 && (*ptex).texture != 0 {
        glBindTexture((*ptex).target, (*ptex).texture);
        ((*(*ps).psglx).glXReleaseTexImageProc)
            .expect(
                "non-null function pointer",
            )((*ps).dpy, (*ptex).glpixmap, 0x20de as libc::c_int);
        glBindTexture((*ptex).target, 0 as libc::c_int as GLuint);
    }
    if (*ptex).glpixmap != 0 {
        glXDestroyPixmap((*ps).dpy, (*ptex).glpixmap);
        (*ptex).glpixmap = 0 as libc::c_int as GLXPixmap;
    }
}
#[no_mangle]
pub unsafe extern "C" fn glx_paint_pre(
    mut ps: *mut session_t,
    mut preg: *mut XserverRegion,
) {
    (*(*ps).psglx).z = 0.0f64 as libc::c_int;
    let mut trace_damage: bool = (*ps).o.glx_swap_method < 0 as libc::c_int
        || (*ps).o.glx_swap_method > 1 as libc::c_int;
    let mut newdamage: XserverRegion = 0 as libc::c_long as XserverRegion;
    if trace_damage as libc::c_int != 0 && *preg != 0 {
        newdamage = copy_region(ps, *preg);
    }
    if !((*ps).o.glx_use_copysubbuffermesa as libc::c_int != 0 || *preg == 0) {
        let mut buffer_age: libc::c_int = (*ps).o.glx_swap_method;
        if SWAPM_BUFFER_AGE as libc::c_int == buffer_age {
            let mut val: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            glXQueryDrawable(
                (*ps).dpy,
                get_tgt_window(ps),
                0x20f4 as libc::c_int,
                &mut val,
            );
            buffer_age = val as libc::c_int;
        }
        if buffer_age > 5 as libc::c_int + 1 as libc::c_int {
            buffer_age = 0 as libc::c_int;
        }
        buffer_age = max_i(buffer_age, 0 as libc::c_int);
        if buffer_age > 1 as libc::c_int {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < buffer_age - 1 as libc::c_int {
                if (*ps).all_damage_last[i as usize] == 0 {
                    buffer_age = 0 as libc::c_int;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
        if 1 as libc::c_int != buffer_age {
            if (*ps).o.glx_copy_from_front {
                let mut reg_copy: XserverRegion = XFixesCreateRegion(
                    (*ps).dpy,
                    0 as *mut XRectangle,
                    0 as libc::c_int,
                );
                if buffer_age == 0 {
                    XFixesSubtractRegion((*ps).dpy, reg_copy, (*ps).screen_reg, *preg);
                } else {
                    let mut i_0: libc::c_int = 0 as libc::c_int;
                    while i_0 < buffer_age - 1 as libc::c_int {
                        XFixesUnionRegion(
                            (*ps).dpy,
                            reg_copy,
                            reg_copy,
                            (*ps).all_damage_last[i_0 as usize],
                        );
                        i_0 += 1;
                        i_0;
                    }
                    XFixesSubtractRegion((*ps).dpy, reg_copy, reg_copy, *preg);
                }
                let mut raster_pos: [GLfloat; 4] = [0.; 4];
                let mut curx: GLfloat = 0.0f32;
                let mut cury: GLfloat = 0.0f32;
                glGetFloatv(0xb07 as libc::c_int as GLenum, raster_pos.as_mut_ptr());
                glReadBuffer(0x404 as libc::c_int as GLenum);
                glRasterPos2f(0.0f64 as GLfloat, 0.0f64 as GLfloat);
                let mut nrects: libc::c_int = 0 as libc::c_int;
                let mut rects: *mut XRectangle = XFixesFetchRegion(
                    (*ps).dpy,
                    reg_copy,
                    &mut nrects,
                );
                let mut i_1: libc::c_int = 0 as libc::c_int;
                while i_1 < nrects {
                    let x: libc::c_int = (*rects.offset(i_1 as isize)).x as libc::c_int;
                    let y: libc::c_int = (*ps).root_height
                        - (*rects.offset(i_1 as isize)).y as libc::c_int
                        - (*rects.offset(i_1 as isize)).height as libc::c_int;
                    glBitmap(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int as GLfloat,
                        0 as libc::c_int as GLfloat,
                        x as libc::c_float - curx,
                        y as libc::c_float - cury,
                        0 as *const GLubyte,
                    );
                    curx = x as GLfloat;
                    cury = y as GLfloat;
                    glCopyPixels(
                        x,
                        y,
                        (*rects.offset(i_1 as isize)).width as GLsizei,
                        (*rects.offset(i_1 as isize)).height as GLsizei,
                        0x1800 as libc::c_int as GLenum,
                    );
                    i_1 += 1;
                    i_1;
                }
                cxfree(rects as *mut libc::c_void);
                glReadBuffer(0x405 as libc::c_int as GLenum);
                glRasterPos4fv(raster_pos.as_mut_ptr());
                free_region(ps, &mut reg_copy);
            }
            if !(*ps).o.glx_copy_from_front {
                if buffer_age != 0 {
                    let mut i_2: libc::c_int = 0 as libc::c_int;
                    while i_2 < buffer_age - 1 as libc::c_int {
                        XFixesUnionRegion(
                            (*ps).dpy,
                            *preg,
                            *preg,
                            (*ps).all_damage_last[i_2 as usize],
                        );
                        i_2 += 1;
                        i_2;
                    }
                } else {
                    free_region(ps, preg);
                }
            }
        }
    }
    if trace_damage {
        free_region(
            ps,
            &mut *((*ps).all_damage_last)
                .as_mut_ptr()
                .offset((5 as libc::c_int - 1 as libc::c_int) as isize),
        );
        memmove(
            ((*ps).all_damage_last).as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            ((*ps).all_damage_last).as_mut_ptr() as *const libc::c_void,
            ((5 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<XserverRegion>() as libc::c_ulong),
        );
        (*ps).all_damage_last[0 as libc::c_int as usize] = newdamage;
    }
    glx_set_clip(ps, *preg, 0 as *const reg_data_t);
}
#[no_mangle]
pub unsafe extern "C" fn glx_set_clip(
    mut ps: *mut session_t,
    mut reg: XserverRegion,
    mut pcache_reg: *const reg_data_t,
) {
    if (*ps).o.glx_no_stencil {
        return;
    }
    static mut rect_blank: XRectangle = {
        let mut init = XRectangle {
            x: 0 as libc::c_int as libc::c_short,
            y: 0 as libc::c_int as libc::c_short,
            width: 0 as libc::c_int as libc::c_ushort,
            height: 0 as libc::c_int as libc::c_ushort,
        };
        init
    };
    glDisable(0xb90 as libc::c_int as GLenum);
    glDisable(0xc11 as libc::c_int as GLenum);
    if reg == 0 {
        return;
    }
    let mut nrects: libc::c_int = 0 as libc::c_int;
    let mut rects_free: *mut XRectangle = 0 as *mut XRectangle;
    let mut rects: *const XRectangle = 0 as *const XRectangle;
    if !pcache_reg.is_null() {
        rects = (*pcache_reg).rects;
        nrects = (*pcache_reg).nrects;
    }
    if rects.is_null() {
        nrects = 0 as libc::c_int;
        rects_free = XFixesFetchRegion((*ps).dpy, reg, &mut nrects);
        rects = rects_free;
    }
    if nrects == 0 {
        cxfree(rects_free as *mut libc::c_void);
        rects_free = 0 as *mut XRectangle;
        nrects = 1 as libc::c_int;
        rects = &mut rect_blank;
    }
    if 1 as libc::c_int == nrects {
        glEnable(0xc11 as libc::c_int as GLenum);
        glScissor(
            (*rects.offset(0 as libc::c_int as isize)).x as GLint,
            (*ps).root_height
                - (*rects.offset(0 as libc::c_int as isize)).y as libc::c_int
                - (*rects.offset(0 as libc::c_int as isize)).height as libc::c_int,
            (*rects.offset(0 as libc::c_int as isize)).width as GLsizei,
            (*rects.offset(0 as libc::c_int as isize)).height as GLsizei,
        );
    } else {
        glEnable(0xb90 as libc::c_int as GLenum);
        glClear(0x400 as libc::c_int as GLbitfield);
        glColorMask(
            0 as libc::c_int as GLboolean,
            0 as libc::c_int as GLboolean,
            0 as libc::c_int as GLboolean,
            0 as libc::c_int as GLboolean,
        );
        glDepthMask(0 as libc::c_int as GLboolean);
        glStencilOp(
            0x1e01 as libc::c_int as GLenum,
            0x1e00 as libc::c_int as GLenum,
            0x1e00 as libc::c_int as GLenum,
        );
        glBegin(0x7 as libc::c_int as GLenum);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < nrects {
            let mut rx: GLint = (*rects.offset(i as isize)).x as GLint;
            let mut ry: GLint = (*ps).root_height
                - (*rects.offset(i as isize)).y as libc::c_int;
            let mut rxe: GLint = rx + (*rects.offset(i as isize)).width as libc::c_int;
            let mut rye: GLint = ry - (*rects.offset(i as isize)).height as libc::c_int;
            let mut z: GLint = 0 as libc::c_int;
            glVertex3i(rx, ry, z);
            glVertex3i(rxe, ry, z);
            glVertex3i(rxe, rye, z);
            glVertex3i(rx, rye, z);
            i += 1;
            i;
        }
        glEnd();
        glStencilOp(
            0x1e00 as libc::c_int as GLenum,
            0x1e00 as libc::c_int as GLenum,
            0x1e00 as libc::c_int as GLenum,
        );
        glColorMask(
            1 as libc::c_int as GLboolean,
            1 as libc::c_int as GLboolean,
            1 as libc::c_int as GLboolean,
            1 as libc::c_int as GLboolean,
        );
    }
    cxfree(rects_free as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn glx_gen_texture(
    mut ps: *mut session_t,
    mut tex_tgt: GLenum,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> GLuint {
    let mut tex: GLuint = 0 as libc::c_int as GLuint;
    glGenTextures(1 as libc::c_int, &mut tex);
    if tex == 0 {
        return 0 as libc::c_int as GLuint;
    }
    glEnable(tex_tgt);
    glBindTexture(tex_tgt, tex);
    glTexParameteri(tex_tgt, 0x2801 as libc::c_int as GLenum, 0x2601 as libc::c_int);
    glTexParameteri(tex_tgt, 0x2800 as libc::c_int as GLenum, 0x2601 as libc::c_int);
    glTexParameteri(tex_tgt, 0x2802 as libc::c_int as GLenum, 0x812f as libc::c_int);
    glTexParameteri(tex_tgt, 0x2803 as libc::c_int as GLenum, 0x812f as libc::c_int);
    glTexImage2D(
        tex_tgt,
        0 as libc::c_int,
        0x1907 as libc::c_int,
        width,
        height,
        0 as libc::c_int,
        0x1907 as libc::c_int as GLenum,
        0x1401 as libc::c_int as GLenum,
        0 as *const libc::c_void,
    );
    glBindTexture(tex_tgt, 0 as libc::c_int as GLuint);
    return tex;
}
#[inline]
unsafe extern "C" fn glx_copy_region_to_tex(
    mut ps: *mut session_t,
    mut tex_tgt: GLenum,
    mut basex: libc::c_int,
    mut basey: libc::c_int,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    if width > 0 as libc::c_int && height > 0 as libc::c_int {
        glCopyTexSubImage2D(
            tex_tgt,
            0 as libc::c_int,
            dx - basex,
            dy - basey,
            dx,
            (*ps).root_height - dy - height,
            width,
            height,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn glx_blur_dst(
    mut ps: *mut session_t,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut z: libc::c_float,
    mut factor_center: GLfloat,
    mut reg_tgt: XserverRegion,
    mut pcache_reg: *const reg_data_t,
    mut pbc: *mut glx_blur_cache_t,
) -> bool {
    let mut texfac_x: GLfloat = 0.;
    let mut texfac_y: GLfloat = 0.;
    let mut last_pass: bool = false;
    let mut current_block: u64;
    let more_passes: bool = (*(*ps).psglx).blur_passes[1 as libc::c_int as usize].prog
        != 0;
    let have_scissors: bool = glIsEnabled(0xc11 as libc::c_int as GLenum) != 0;
    let have_stencil: bool = glIsEnabled(0xb90 as libc::c_int as GLenum) != 0;
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut ibc: glx_blur_cache_t = {
        let mut init = glx_blur_cache_t {
            fbo: 0,
            textures: [0; 2],
            width: 0 as libc::c_int,
            height: 0 as libc::c_int,
        };
        init
    };
    if pbc.is_null() {
        pbc = &mut ibc;
    }
    let mut mdx: libc::c_int = dx;
    let mut mdy: libc::c_int = dy;
    let mut mwidth: libc::c_int = width;
    let mut mheight: libc::c_int = height;
    let mut tex_tgt: GLenum = 0x84f5 as libc::c_int as GLenum;
    if (*(*ps).psglx).has_texture_non_power_of_two {
        tex_tgt = 0xde1 as libc::c_int as GLenum;
    }
    if mwidth != (*pbc).width || mheight != (*pbc).height {
        free_glx_bc_resize(ps, pbc);
    }
    if (*pbc).textures[0 as libc::c_int as usize] == 0 {
        (*pbc)
            .textures[0 as libc::c_int
            as usize] = glx_gen_texture(ps, tex_tgt, mwidth, mheight);
    }
    let mut tex_scr: GLuint = (*pbc).textures[0 as libc::c_int as usize];
    if more_passes as libc::c_int != 0 && (*pbc).textures[1 as libc::c_int as usize] == 0
    {
        (*pbc)
            .textures[1 as libc::c_int
            as usize] = glx_gen_texture(ps, tex_tgt, mwidth, mheight);
    }
    (*pbc).width = mwidth;
    (*pbc).height = mheight;
    let mut tex_scr2: GLuint = (*pbc).textures[1 as libc::c_int as usize];
    if more_passes as libc::c_int != 0 && (*pbc).fbo == 0 {
        glGenFramebuffers(1 as libc::c_int, &mut (*pbc).fbo);
    }
    let fbo: GLuint = (*pbc).fbo;
    if tex_scr == 0 || more_passes as libc::c_int != 0 && tex_scr2 == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to allocate texture.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"glx_blur_dst\0"))
                .as_ptr(),
        );
    } else if more_passes as libc::c_int != 0 && fbo == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to allocate framebuffer.\n\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"glx_blur_dst\0"))
                .as_ptr(),
        );
    } else {
        glEnable(tex_tgt);
        glBindTexture(tex_tgt, tex_scr);
        glx_copy_region_to_tex(ps, tex_tgt, mdx, mdy, mdx, mdy, mwidth, mheight);
        texfac_x = 1.0f32;
        texfac_y = 1.0f32;
        if 0xde1 as libc::c_int as libc::c_uint == tex_tgt {
            texfac_x /= mwidth as libc::c_float;
            texfac_y /= mheight as libc::c_float;
        }
        if more_passes {
            glDisable(0xb90 as libc::c_int as GLenum);
            glDisable(0xc11 as libc::c_int as GLenum);
        }
        last_pass = 0 as libc::c_int != 0;
        let mut i: libc::c_int = 0 as libc::c_int;
        loop {
            if last_pass {
                current_block = 17688141731389699982;
                break;
            }
            last_pass = (*(*ps).psglx).blur_passes[(i + 1 as libc::c_int) as usize].prog
                == 0;
            let mut ppass: *const glx_blur_pass_t = &mut *((*(*ps).psglx).blur_passes)
                .as_mut_ptr()
                .offset(i as isize) as *mut glx_blur_pass_t;
            glBindTexture(tex_tgt, tex_scr);
            if !last_pass {
                static mut DRAWBUFS: [GLenum; 2] = [0x8ce0 as libc::c_int as GLenum, 0];
                glBindFramebuffer(0x8d40 as libc::c_int as GLenum, fbo);
                glFramebufferTexture2D(
                    0x8d40 as libc::c_int as GLenum,
                    0x8ce0 as libc::c_int as GLenum,
                    0xde1 as libc::c_int as GLenum,
                    tex_scr2,
                    0 as libc::c_int,
                );
                glDrawBuffers(1 as libc::c_int, DRAWBUFS.as_ptr());
                if glCheckFramebufferStatus(0x8d40 as libc::c_int as GLenum)
                    != 0x8cd5 as libc::c_int as libc::c_uint
                {
                    fprintf(
                        stderr,
                        b"%s(): Framebuffer attachment failed.\n\0" as *const u8
                            as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 13],
                            &[libc::c_char; 13],
                        >(b"glx_blur_dst\0"))
                            .as_ptr(),
                    );
                    current_block = 16103973629397080661;
                    break;
                }
            } else {
                static mut DRAWBUFS_0: [GLenum; 2] = [0x405 as libc::c_int as GLenum, 0];
                glBindFramebuffer(
                    0x8d40 as libc::c_int as GLenum,
                    0 as libc::c_int as GLuint,
                );
                glDrawBuffers(1 as libc::c_int, DRAWBUFS_0.as_ptr());
                if have_scissors {
                    glEnable(0xc11 as libc::c_int as GLenum);
                }
                if have_stencil {
                    glEnable(0xb90 as libc::c_int as GLenum);
                }
            }
            glTexEnvi(
                0x2300 as libc::c_int as GLenum,
                0x2200 as libc::c_int as GLenum,
                0x1e01 as libc::c_int,
            );
            glUseProgram((*ppass).prog);
            if (*ppass).unifm_offset_x >= 0 as libc::c_int {
                glUniform1f((*ppass).unifm_offset_x, texfac_x);
            }
            if (*ppass).unifm_offset_y >= 0 as libc::c_int {
                glUniform1f((*ppass).unifm_offset_y, texfac_y);
            }
            if (*ppass).unifm_factor_center >= 0 as libc::c_int {
                glUniform1f((*ppass).unifm_factor_center, factor_center);
            }
            let mut reg_new: XserverRegion = 0 as libc::c_long as XserverRegion;
            let mut rec_all: XRectangle = {
                let mut init = XRectangle {
                    x: dx as libc::c_short,
                    y: dy as libc::c_short,
                    width: width as libc::c_ushort,
                    height: height as libc::c_ushort,
                };
                init
            };
            let mut rects: *mut XRectangle = &mut rec_all;
            let mut nrects: libc::c_int = 1 as libc::c_int;
            if (*ps).o.glx_no_stencil as libc::c_int != 0 && reg_tgt != 0 {
                if !pcache_reg.is_null() {
                    rects = (*pcache_reg).rects;
                    nrects = (*pcache_reg).nrects;
                } else {
                    reg_new = XFixesCreateRegion(
                        (*ps).dpy,
                        &mut rec_all,
                        1 as libc::c_int,
                    );
                    XFixesIntersectRegion((*ps).dpy, reg_new, reg_new, reg_tgt);
                    nrects = 0 as libc::c_int;
                    rects = XFixesFetchRegion((*ps).dpy, reg_new, &mut nrects);
                }
            }
            glBegin(0x7 as libc::c_int as GLenum);
            let mut ri: libc::c_int = 0 as libc::c_int;
            while ri < nrects {
                let mut crect: XRectangle = XRectangle {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                };
                rect_crop(&mut crect, &mut *rects.offset(ri as isize), &mut rec_all);
                if !(crect.width == 0 || crect.height == 0) {
                    let rx: GLfloat = (crect.x as libc::c_int - mdx) as libc::c_float
                        * texfac_x;
                    let ry: GLfloat = (mheight - (crect.y as libc::c_int - mdy))
                        as libc::c_float * texfac_y;
                    let rxe: GLfloat = rx
                        + crect.width as libc::c_int as libc::c_float * texfac_x;
                    let rye: GLfloat = ry
                        - crect.height as libc::c_int as libc::c_float * texfac_y;
                    let mut rdx: GLfloat = (crect.x as libc::c_int - mdx) as GLfloat;
                    let mut rdy: GLfloat = (mheight - crect.y as libc::c_int + mdy)
                        as GLfloat;
                    let mut rdxe: GLfloat = rdx
                        + crect.width as libc::c_int as libc::c_float;
                    let mut rdye: GLfloat = rdy
                        - crect.height as libc::c_int as libc::c_float;
                    if last_pass {
                        rdx = crect.x as GLfloat;
                        rdy = ((*ps).root_height - crect.y as libc::c_int) as GLfloat;
                        rdxe = rdx + crect.width as libc::c_int as libc::c_float;
                        rdye = rdy - crect.height as libc::c_int as libc::c_float;
                    }
                    glTexCoord2f(rx, ry);
                    glVertex3f(rdx, rdy, z);
                    glTexCoord2f(rxe, ry);
                    glVertex3f(rdxe, rdy, z);
                    glTexCoord2f(rxe, rye);
                    glVertex3f(rdxe, rdye, z);
                    glTexCoord2f(rx, rye);
                    glVertex3f(rdx, rdye, z);
                }
                ri += 1;
                ri;
            }
            glEnd();
            if !rects.is_null() && rects != &mut rec_all as *mut XRectangle
                && !(!pcache_reg.is_null() && (*pcache_reg).rects == rects)
            {
                cxfree(rects as *mut libc::c_void);
            }
            free_region(ps, &mut reg_new);
            glUseProgram(0 as libc::c_int as GLuint);
            let mut tmp: GLuint = tex_scr2;
            tex_scr2 = tex_scr;
            tex_scr = tmp;
            i += 1;
            i;
        }
        match current_block {
            16103973629397080661 => {}
            _ => {
                ret = 1 as libc::c_int != 0;
            }
        }
    }
    glBindFramebuffer(0x8d40 as libc::c_int as GLenum, 0 as libc::c_int as GLuint);
    glBindTexture(tex_tgt, 0 as libc::c_int as GLuint);
    glDisable(tex_tgt);
    if have_scissors {
        glEnable(0xc11 as libc::c_int as GLenum);
    }
    if have_stencil {
        glEnable(0xb90 as libc::c_int as GLenum);
    }
    if &mut ibc as *mut glx_blur_cache_t == pbc {
        free_glx_bc(ps, pbc);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glx_dim_dst(
    mut ps: *mut session_t,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut z: libc::c_float,
    mut factor: GLfloat,
    mut reg_tgt: XserverRegion,
    mut pcache_reg: *const reg_data_t,
) -> bool {
    glEnable(0xbe2 as libc::c_int as GLenum);
    glBlendFunc(1 as libc::c_int as GLenum, 0x303 as libc::c_int as GLenum);
    glColor4f(0.0f32, 0.0f32, 0.0f32, factor);
    let mut reg_new: XserverRegion = 0 as libc::c_long as XserverRegion;
    let mut rec_all: XRectangle = {
        let mut init = XRectangle {
            x: dx as libc::c_short,
            y: dy as libc::c_short,
            width: width as libc::c_ushort,
            height: height as libc::c_ushort,
        };
        init
    };
    let mut rects: *mut XRectangle = &mut rec_all;
    let mut nrects: libc::c_int = 1 as libc::c_int;
    if (*ps).o.glx_no_stencil as libc::c_int != 0 && reg_tgt != 0 {
        if !pcache_reg.is_null() {
            rects = (*pcache_reg).rects;
            nrects = (*pcache_reg).nrects;
        } else {
            reg_new = XFixesCreateRegion((*ps).dpy, &mut rec_all, 1 as libc::c_int);
            XFixesIntersectRegion((*ps).dpy, reg_new, reg_new, reg_tgt);
            nrects = 0 as libc::c_int;
            rects = XFixesFetchRegion((*ps).dpy, reg_new, &mut nrects);
        }
    }
    glBegin(0x7 as libc::c_int as GLenum);
    let mut ri: libc::c_int = 0 as libc::c_int;
    while ri < nrects {
        let mut crect: XRectangle = XRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        rect_crop(&mut crect, &mut *rects.offset(ri as isize), &mut rec_all);
        if !(crect.width == 0 || crect.height == 0) {
            let mut rdx: GLint = crect.x as GLint;
            let mut rdy: GLint = (*ps).root_height - crect.y as libc::c_int;
            let mut rdxe: GLint = rdx + crect.width as libc::c_int;
            let mut rdye: GLint = rdy - crect.height as libc::c_int;
            glVertex3i(rdx, rdy, z as GLint);
            glVertex3i(rdxe, rdy, z as GLint);
            glVertex3i(rdxe, rdye, z as GLint);
            glVertex3i(rdx, rdye, z as GLint);
        }
        ri += 1;
        ri;
    }
    glEnd();
    if !rects.is_null() && rects != &mut rec_all as *mut XRectangle
        && !(!pcache_reg.is_null() && (*pcache_reg).rects == rects)
    {
        cxfree(rects as *mut libc::c_void);
    }
    free_region(ps, &mut reg_new);
    glEnd();
    glColor4f(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    glDisable(0xbe2 as libc::c_int as GLenum);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn glx_render_(
    mut ps: *mut session_t,
    mut ptex: *const glx_texture_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut z: libc::c_int,
    mut opacity: libc::c_double,
    mut argb: bool,
    mut neg: bool,
    mut reg_tgt: XserverRegion,
    mut pcache_reg: *const reg_data_t,
    mut pprogram: *const glx_prog_main_t,
) -> bool {
    if ptex.is_null() || (*ptex).texture == 0 {
        fprintf(
            stderr,
            b"%s(): Missing texture.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"glx_render_\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    argb = argb as libc::c_int != 0
        || 0x20da as libc::c_int
            == (*(*(*ps).psglx).fbconfigs[(*ptex).depth as usize]).texture_fmt;
    let has_prog: bool = !pprogram.is_null() && (*pprogram).prog != 0;
    let mut dual_texture: bool = 0 as libc::c_int != 0;
    glEnable((*ptex).target);
    if opacity < 1.0f64 || argb as libc::c_int != 0 {
        glEnable(0xbe2 as libc::c_int as GLenum);
        glTexEnvi(
            0x2300 as libc::c_int as GLenum,
            0x2200 as libc::c_int as GLenum,
            0x2100 as libc::c_int,
        );
        glBlendFunc(1 as libc::c_int as GLenum, 0x303 as libc::c_int as GLenum);
        glColor4f(
            opacity as GLfloat,
            opacity as GLfloat,
            opacity as GLfloat,
            opacity as GLfloat,
        );
    }
    if !has_prog {
        if neg {
            if glIsEnabled(0xbe2 as libc::c_int as GLenum) == 0 {
                glEnable(0xbf2 as libc::c_int as GLenum);
                glLogicOp(0x150c as libc::c_int as GLenum);
            } else if argb {
                dual_texture = 1 as libc::c_int != 0;
                glActiveTexture(0x84c0 as libc::c_int as GLenum);
                glTexEnvf(
                    0x2300 as libc::c_int as GLenum,
                    0x2200 as libc::c_int as GLenum,
                    0x8570 as libc::c_int as GLfloat,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8571 as libc::c_int as GLenum,
                    0x84e7 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8580 as libc::c_int as GLenum,
                    0x1702 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8590 as libc::c_int as GLenum,
                    0x302 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8581 as libc::c_int as GLenum,
                    0x1702 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8591 as libc::c_int as GLenum,
                    0x300 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8572 as libc::c_int as GLenum,
                    0x1e01 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8588 as libc::c_int as GLenum,
                    0x1702 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8598 as libc::c_int as GLenum,
                    0x302 as libc::c_int,
                );
                glActiveTexture(0x84c1 as libc::c_int as GLenum);
                glEnable((*ptex).target);
                glBindTexture((*ptex).target, (*ptex).texture);
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x2200 as libc::c_int as GLenum,
                    0x8570 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8571 as libc::c_int as GLenum,
                    0x2100 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8580 as libc::c_int as GLenum,
                    0x8578 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8590 as libc::c_int as GLenum,
                    0x300 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8581 as libc::c_int as GLenum,
                    0x8577 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8591 as libc::c_int as GLenum,
                    0x302 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8572 as libc::c_int as GLenum,
                    0x2100 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8588 as libc::c_int as GLenum,
                    0x8578 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8598 as libc::c_int as GLenum,
                    0x302 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8589 as libc::c_int as GLenum,
                    0x8577 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8599 as libc::c_int as GLenum,
                    0x302 as libc::c_int,
                );
                glActiveTexture(0x84c0 as libc::c_int as GLenum);
            } else {
                glTexEnvf(
                    0x2300 as libc::c_int as GLenum,
                    0x2200 as libc::c_int as GLenum,
                    0x8570 as libc::c_int as GLfloat,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8571 as libc::c_int as GLenum,
                    0x2100 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8580 as libc::c_int as GLenum,
                    0x1702 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8590 as libc::c_int as GLenum,
                    0x301 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8581 as libc::c_int as GLenum,
                    0x8577 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8591 as libc::c_int as GLenum,
                    0x300 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8572 as libc::c_int as GLenum,
                    0x2100 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8588 as libc::c_int as GLenum,
                    0x1702 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8598 as libc::c_int as GLenum,
                    0x302 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8589 as libc::c_int as GLenum,
                    0x8577 as libc::c_int,
                );
                glTexEnvi(
                    0x2300 as libc::c_int as GLenum,
                    0x8599 as libc::c_int as GLenum,
                    0x302 as libc::c_int,
                );
            }
        }
    } else {
        glUseProgram((*pprogram).prog);
        if (*pprogram).unifm_opacity >= 0 as libc::c_int {
            glUniform1f((*pprogram).unifm_opacity, opacity as GLfloat);
        }
        if (*pprogram).unifm_invert_color >= 0 as libc::c_int {
            glUniform1i((*pprogram).unifm_invert_color, neg as GLint);
        }
        if (*pprogram).unifm_tex >= 0 as libc::c_int {
            glUniform1i((*pprogram).unifm_tex, 0 as libc::c_int);
        }
    }
    glBindTexture((*ptex).target, (*ptex).texture);
    if dual_texture {
        glActiveTexture(0x84c1 as libc::c_int as GLenum);
        glBindTexture((*ptex).target, (*ptex).texture);
        glActiveTexture(0x84c0 as libc::c_int as GLenum);
    }
    let mut reg_new: XserverRegion = 0 as libc::c_long as XserverRegion;
    let mut rec_all: XRectangle = {
        let mut init = XRectangle {
            x: dx as libc::c_short,
            y: dy as libc::c_short,
            width: width as libc::c_ushort,
            height: height as libc::c_ushort,
        };
        init
    };
    let mut rects: *mut XRectangle = &mut rec_all;
    let mut nrects: libc::c_int = 1 as libc::c_int;
    if (*ps).o.glx_no_stencil as libc::c_int != 0 && reg_tgt != 0 {
        if !pcache_reg.is_null() {
            rects = (*pcache_reg).rects;
            nrects = (*pcache_reg).nrects;
        } else {
            reg_new = XFixesCreateRegion((*ps).dpy, &mut rec_all, 1 as libc::c_int);
            XFixesIntersectRegion((*ps).dpy, reg_new, reg_new, reg_tgt);
            nrects = 0 as libc::c_int;
            rects = XFixesFetchRegion((*ps).dpy, reg_new, &mut nrects);
        }
    }
    glBegin(0x7 as libc::c_int as GLenum);
    let mut ri: libc::c_int = 0 as libc::c_int;
    while ri < nrects {
        let mut crect: XRectangle = XRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        rect_crop(&mut crect, &mut *rects.offset(ri as isize), &mut rec_all);
        if !(crect.width == 0 || crect.height == 0) {
            let mut rx: GLfloat = (crect.x as libc::c_int - dx + x) as libc::c_double
                as GLfloat;
            let mut ry: GLfloat = (crect.y as libc::c_int - dy + y) as libc::c_double
                as GLfloat;
            let mut rxe: GLfloat = (rx as libc::c_double + crect.width as libc::c_double)
                as GLfloat;
            let mut rye: GLfloat = (ry as libc::c_double
                + crect.height as libc::c_double) as GLfloat;
            if 0xde1 as libc::c_int as libc::c_uint == (*ptex).target {
                rx = rx / (*ptex).width as libc::c_float;
                ry = ry / (*ptex).height as libc::c_float;
                rxe = rxe / (*ptex).width as libc::c_float;
                rye = rye / (*ptex).height as libc::c_float;
            }
            let mut rdx: GLint = crect.x as GLint;
            let mut rdy: GLint = (*ps).root_height - crect.y as libc::c_int;
            let mut rdxe: GLint = rdx + crect.width as libc::c_int;
            let mut rdye: GLint = rdy - crect.height as libc::c_int;
            if !(*ptex).y_inverted {
                ry = (1.0f64 - ry as libc::c_double) as GLfloat;
                rye = (1.0f64 - rye as libc::c_double) as GLfloat;
            }
            if dual_texture {
                glMultiTexCoord2f(0x84c0 as libc::c_int as GLenum, rx, ry);
                glMultiTexCoord2f(0x84c1 as libc::c_int as GLenum, rx, ry);
            } else {
                glTexCoord2f(rx, ry);
            }
            glVertex3i(rdx, rdy, z);
            if dual_texture {
                glMultiTexCoord2f(0x84c0 as libc::c_int as GLenum, rxe, ry);
                glMultiTexCoord2f(0x84c1 as libc::c_int as GLenum, rxe, ry);
            } else {
                glTexCoord2f(rxe, ry);
            }
            glVertex3i(rdxe, rdy, z);
            if dual_texture {
                glMultiTexCoord2f(0x84c0 as libc::c_int as GLenum, rxe, rye);
                glMultiTexCoord2f(0x84c1 as libc::c_int as GLenum, rxe, rye);
            } else {
                glTexCoord2f(rxe, rye);
            }
            glVertex3i(rdxe, rdye, z);
            if dual_texture {
                glMultiTexCoord2f(0x84c0 as libc::c_int as GLenum, rx, rye);
                glMultiTexCoord2f(0x84c1 as libc::c_int as GLenum, rx, rye);
            } else {
                glTexCoord2f(rx, rye);
            }
            glVertex3i(rdx, rdye, z);
        }
        ri += 1;
        ri;
    }
    glEnd();
    if !rects.is_null() && rects != &mut rec_all as *mut XRectangle
        && !(!pcache_reg.is_null() && (*pcache_reg).rects == rects)
    {
        cxfree(rects as *mut libc::c_void);
    }
    free_region(ps, &mut reg_new);
    glBindTexture((*ptex).target, 0 as libc::c_int as GLuint);
    glColor4f(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    glTexEnvi(
        0x2300 as libc::c_int as GLenum,
        0x2200 as libc::c_int as GLenum,
        0x1e01 as libc::c_int,
    );
    glDisable(0xbe2 as libc::c_int as GLenum);
    glDisable(0xbf2 as libc::c_int as GLenum);
    glDisable((*ptex).target);
    if dual_texture {
        glActiveTexture(0x84c1 as libc::c_int as GLenum);
        glBindTexture((*ptex).target, 0 as libc::c_int as GLuint);
        glDisable((*ptex).target);
        glActiveTexture(0x84c0 as libc::c_int as GLenum);
    }
    if has_prog {
        glUseProgram(0 as libc::c_int as GLuint);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn glx_swap_copysubbuffermesa(
    mut ps: *mut session_t,
    mut reg: XserverRegion,
) {
    let mut nrects: libc::c_int = 0 as libc::c_int;
    let mut rects: *mut XRectangle = XFixesFetchRegion((*ps).dpy, reg, &mut nrects);
    if 1 as libc::c_int == nrects
        && rect_is_fullscreen(
            ps,
            (*rects.offset(0 as libc::c_int as isize)).x as libc::c_int,
            (*rects.offset(0 as libc::c_int as isize)).y as libc::c_int,
            (*rects.offset(0 as libc::c_int as isize)).width as libc::c_uint,
            (*rects.offset(0 as libc::c_int as isize)).height as libc::c_uint,
        ) as libc::c_int != 0
    {
        glXSwapBuffers((*ps).dpy, get_tgt_window(ps));
    } else {
        glx_set_clip(ps, 0 as libc::c_long as XserverRegion, 0 as *const reg_data_t);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < nrects {
            let x: libc::c_int = (*rects.offset(i as isize)).x as libc::c_int;
            let y: libc::c_int = (*ps).root_height
                - (*rects.offset(i as isize)).y as libc::c_int
                - (*rects.offset(i as isize)).height as libc::c_int;
            let wid: libc::c_int = (*rects.offset(i as isize)).width as libc::c_int;
            let hei: libc::c_int = (*rects.offset(i as isize)).height as libc::c_int;
            ((*(*ps).psglx).glXCopySubBufferProc)
                .expect(
                    "non-null function pointer",
                )((*ps).dpy, get_tgt_window(ps), x, y, wid, hei);
            i += 1;
            i;
        }
    }
    cxfree(rects as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn glx_take_screenshot(
    mut ps: *mut session_t,
    mut out_length: *mut libc::c_int,
) -> *mut libc::c_uchar {
    let mut length: libc::c_int = 3 as libc::c_int * (*ps).root_width
        * (*ps).root_height;
    let mut unpack_align_old: GLint = 0 as libc::c_int;
    glGetIntegerv(0xcf5 as libc::c_int as GLenum, &mut unpack_align_old);
    glPixelStorei(0xcf5 as libc::c_int as GLenum, 1 as libc::c_int);
    let mut buf: *mut libc::c_uchar = allocchk_(
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"glx_take_screenshot\0"))
            .as_ptr(),
        malloc(
            (length as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        ),
    ) as *mut libc::c_uchar;
    glReadBuffer(0x404 as libc::c_int as GLenum);
    glReadPixels(
        0 as libc::c_int,
        0 as libc::c_int,
        (*ps).root_width,
        (*ps).root_height,
        0x1907 as libc::c_int as GLenum,
        0x1401 as libc::c_int as GLenum,
        buf as *mut libc::c_void,
    );
    glReadBuffer(0x405 as libc::c_int as GLenum);
    glPixelStorei(0xcf5 as libc::c_int as GLenum, unpack_align_old);
    if !out_length.is_null() {
        *out_length = (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul(length as libc::c_ulong) as libc::c_int;
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn glx_create_shader(
    mut shader_type: GLenum,
    mut shader_str: *const libc::c_char,
) -> GLuint {
    let mut success: bool = 0 as libc::c_int != 0;
    let mut shader: GLuint = glCreateShader(shader_type);
    if shader == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to create shader with type %#x.\n\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"glx_create_shader\0"))
                .as_ptr(),
            shader_type,
        );
    } else {
        glShaderSource(
            shader,
            1 as libc::c_int,
            &mut shader_str as *mut *const libc::c_char as *const *const GLchar,
            0 as *const GLint,
        );
        glCompileShader(shader);
        let mut status: GLint = 0 as libc::c_int;
        glGetShaderiv(shader, 0x8b81 as libc::c_int as GLenum, &mut status);
        if 0 as libc::c_int == status {
            let mut log_len: GLint = 0 as libc::c_int;
            glGetShaderiv(shader, 0x8b84 as libc::c_int as GLenum, &mut log_len);
            if log_len != 0 {
                let vla = (log_len + 1 as libc::c_int) as usize;
                let mut log: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
                glGetShaderInfoLog(shader, log_len, 0 as *mut GLsizei, log.as_mut_ptr());
                fprintf(
                    stderr,
                    b"%s(): Failed to compile shader with type %d: %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 18],
                        &[libc::c_char; 18],
                    >(b"glx_create_shader\0"))
                        .as_ptr(),
                    shader_type,
                    log.as_mut_ptr(),
                );
            }
        } else {
            success = 1 as libc::c_int != 0;
        }
    }
    if shader != 0 && !success {
        glDeleteShader(shader);
        shader = 0 as libc::c_int as GLuint;
    }
    return shader;
}
#[no_mangle]
pub unsafe extern "C" fn glx_create_program(
    shaders: *const GLuint,
    mut nshaders: libc::c_int,
) -> GLuint {
    let mut success: bool = 0 as libc::c_int != 0;
    let mut program: GLuint = glCreateProgram();
    if program == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to create program.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"glx_create_program\0"))
                .as_ptr(),
        );
    } else {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < nshaders {
            glAttachShader(program, *shaders.offset(i as isize));
            i += 1;
            i;
        }
        glLinkProgram(program);
        let mut status: GLint = 0 as libc::c_int;
        glGetProgramiv(program, 0x8b82 as libc::c_int as GLenum, &mut status);
        if 0 as libc::c_int == status {
            let mut log_len: GLint = 0 as libc::c_int;
            glGetProgramiv(program, 0x8b84 as libc::c_int as GLenum, &mut log_len);
            if log_len != 0 {
                let vla = (log_len + 1 as libc::c_int) as usize;
                let mut log: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
                glGetProgramInfoLog(
                    program,
                    log_len,
                    0 as *mut GLsizei,
                    log.as_mut_ptr(),
                );
                fprintf(
                    stderr,
                    b"%s(): Failed to link program: %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 19],
                        &[libc::c_char; 19],
                    >(b"glx_create_program\0"))
                        .as_ptr(),
                    log.as_mut_ptr(),
                );
            }
        } else {
            success = 1 as libc::c_int != 0;
        }
    }
    if program != 0 {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < nshaders {
            glDetachShader(program, *shaders.offset(i_0 as isize));
            i_0 += 1;
            i_0;
        }
    }
    if program != 0 && !success {
        glDeleteProgram(program);
        program = 0 as libc::c_int as GLuint;
    }
    return program;
}
#[no_mangle]
pub unsafe extern "C" fn glx_create_program_from_str(
    mut vert_shader_str: *const libc::c_char,
    mut frag_shader_str: *const libc::c_char,
) -> GLuint {
    let mut vert_shader: GLuint = 0 as libc::c_int as GLuint;
    let mut frag_shader: GLuint = 0 as libc::c_int as GLuint;
    let mut prog: GLuint = 0 as libc::c_int as GLuint;
    if !vert_shader_str.is_null() {
        vert_shader = glx_create_shader(
            0x8b31 as libc::c_int as GLenum,
            vert_shader_str,
        );
    }
    if !frag_shader_str.is_null() {
        frag_shader = glx_create_shader(
            0x8b30 as libc::c_int as GLenum,
            frag_shader_str,
        );
    }
    let mut shaders: [GLuint; 2] = [0; 2];
    let mut count: libc::c_int = 0 as libc::c_int;
    if vert_shader != 0 {
        let fresh0 = count;
        count = count + 1;
        shaders[fresh0 as usize] = vert_shader;
    }
    if frag_shader != 0 {
        let fresh1 = count;
        count = count + 1;
        shaders[fresh1 as usize] = frag_shader;
    }
    if count != 0 {
        prog = glx_create_program(shaders.as_mut_ptr(), count);
    }
    if vert_shader != 0 {
        glDeleteShader(vert_shader);
    }
    if frag_shader != 0 {
        glDeleteShader(frag_shader);
    }
    return prog;
}
