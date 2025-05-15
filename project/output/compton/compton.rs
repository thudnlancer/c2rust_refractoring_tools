use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    pub type _XRRScreenConfiguration;
    pub type DBusConnection;
    pub type __GLXcontextRec;
    pub type __GLXFBConfigRec;
    pub type _c2_lptr;
    fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint);
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strcasecmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strtok(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn getenv(__name: *const i8) -> *mut i8;
    fn exit(_: i32) -> !;
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn select(
        __nfds: i32,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> i32;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn freopen(
        __filename: *const i8,
        __modes: *const i8,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    static mut stderr: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stdin: *mut _IO_FILE;
    fn XRenderQueryExtension(
        dpy: *mut Display,
        event_basep: *mut i32,
        error_basep: *mut i32,
    ) -> i32;
    fn XRenderFindVisualFormat(
        dpy: *mut Display,
        visual: *const Visual,
    ) -> *mut XRenderPictFormat;
    fn XRenderFindStandardFormat(
        dpy: *mut Display,
        format: i32,
    ) -> *mut XRenderPictFormat;
    fn XRenderCreatePicture(
        dpy: *mut Display,
        drawable: Drawable,
        format: *const XRenderPictFormat,
        valuemask: u64,
        attributes: *const XRenderPictureAttributes,
    ) -> Picture;
    fn XRenderFreePicture(dpy: *mut Display, picture: Picture);
    fn XRenderComposite(
        dpy: *mut Display,
        op: i32,
        src: Picture,
        mask: Picture,
        dst: Picture,
        src_x: i32,
        src_y: i32,
        mask_x: i32,
        mask_y: i32,
        dst_x: i32,
        dst_y: i32,
        width: u32,
        height: u32,
    );
    fn XRenderFillRectangle(
        dpy: *mut Display,
        op: i32,
        dst: Picture,
        color: *const XRenderColor,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    );
    fn XRenderFillRectangles(
        dpy: *mut Display,
        op: i32,
        dst: Picture,
        color: *const XRenderColor,
        rectangles: *const XRectangle,
        n_rects: i32,
    );
    fn XFixesFetchRegion(
        dpy: *mut Display,
        region: XserverRegion,
        nrectanglesRet: *mut i32,
    ) -> *mut XRectangle;
    fn XRenderQueryFilters(dpy: *mut Display, drawable: Drawable) -> *mut XFilters;
    fn XRenderSetPictureFilter(
        dpy: *mut Display,
        picture: Picture,
        filter: *const i8,
        params: *mut XFixed,
        nparams: i32,
    );
    fn XShapeQueryExtension(_: *mut Display, _: *mut i32, _: *mut i32) -> i32;
    fn XShapeQueryExtents(
        _: *mut Display,
        _: Window,
        _: *mut i32,
        _: *mut i32,
        _: *mut i32,
        _: *mut u32,
        _: *mut u32,
        _: *mut i32,
        _: *mut i32,
        _: *mut i32,
        _: *mut u32,
        _: *mut u32,
    ) -> i32;
    fn XShapeSelectInput(_: *mut Display, _: Window, _: u64);
    fn XRRQueryExtension(
        dpy: *mut Display,
        event_base_return: *mut i32,
        error_base_return: *mut i32,
    ) -> i32;
    fn XRRGetScreenInfo(
        dpy: *mut Display,
        window: Window,
    ) -> *mut XRRScreenConfiguration;
    fn XRRFreeScreenConfigInfo(config: *mut XRRScreenConfiguration);
    fn XRRConfigCurrentRate(config: *mut XRRScreenConfiguration) -> libc::c_short;
    fn XRRSelectInput(dpy: *mut Display, window: Window, mask: i32);
    fn config_setting_lookup_float(
        setting: *const config_setting_t,
        name: *const i8,
        value: *mut libc::c_double,
    ) -> i32;
    fn XdbeQueryExtension(_: *mut Display, _: *mut i32, _: *mut i32) -> i32;
    fn XdbeAllocateBackBufferName(
        _: *mut Display,
        _: Window,
        _: XdbeSwapAction,
    ) -> XdbeBackBuffer;
    fn XdbeDeallocateBackBufferName(_: *mut Display, _: XdbeBackBuffer) -> i32;
    fn XdbeSwapBuffers(_: *mut Display, _: *mut XdbeSwapInfo, _: i32) -> i32;
    fn XSyncQueryExtension(_: *mut Display, _: *mut i32, _: *mut i32) -> i32;
    fn XSyncInitialize(_: *mut Display, _: *mut i32, _: *mut i32) -> i32;
    fn XSyncCreateFence(_: *mut Display, _: Drawable, _: i32) -> XSyncFence;
    fn XSyncTriggerFence(_: *mut Display, _: XSyncFence) -> i32;
    fn XSyncResetFence(_: *mut Display, _: XSyncFence) -> i32;
    fn XSyncDestroyFence(_: *mut Display, _: XSyncFence) -> i32;
    fn XSyncAwaitFence(_: *mut Display, _: *const XSyncFence, _: i32) -> i32;
    fn XineramaQueryExtension(
        dpy: *mut Display,
        event_base: *mut i32,
        error_base: *mut i32,
    ) -> i32;
    fn XineramaIsActive(dpy: *mut Display) -> i32;
    fn XineramaQueryScreens(
        dpy: *mut Display,
        number: *mut i32,
    ) -> *mut XineramaScreenInfo;
    fn dirname(__path: *mut i8) -> *mut i8;
    fn config_read(config: *mut config_t, stream: *mut FILE) -> i32;
    fn config_set_auto_convert(config: *mut config_t, flag: i32);
    fn config_set_include_dir(config: *mut config_t, include_dir: *const i8);
    fn config_init(config: *mut config_t);
    fn config_destroy(config: *mut config_t);
    fn config_setting_get_string(setting: *const config_setting_t) -> *const i8;
    fn config_setting_lookup_bool(
        setting: *const config_setting_t,
        name: *const i8,
        value: *mut i32,
    ) -> i32;
    fn config_setting_get_string_elem(
        setting: *const config_setting_t,
        idx: i32,
    ) -> *const i8;
    fn config_setting_length(setting: *const config_setting_t) -> i32;
    fn config_lookup(config: *const config_t, path: *const i8) -> *mut config_setting_t;
    fn config_lookup_int(
        config: *const config_t,
        path: *const i8,
        value: *mut i32,
    ) -> i32;
    fn config_lookup_float(
        config: *const config_t,
        path: *const i8,
        value: *mut libc::c_double,
    ) -> i32;
    fn config_lookup_bool(
        config: *const config_t,
        path: *const i8,
        value: *mut i32,
    ) -> i32;
    fn config_lookup_string(
        config: *const config_t,
        path: *const i8,
        value: *mut *const i8,
    ) -> i32;
    fn glFinish();
    fn glFlush();
    fn glDeleteTextures(n: GLsizei, textures: *const GLuint);
    fn XCreateImage(
        _: *mut Display,
        _: *mut Visual,
        _: u32,
        _: i32,
        _: i32,
        _: *mut i8,
        _: u32,
        _: u32,
        _: i32,
        _: i32,
    ) -> *mut XImage;
    fn XOpenDisplay(_: *const i8) -> *mut Display;
    fn XSynchronize(
        _: *mut Display,
        _: i32,
    ) -> Option<unsafe extern "C" fn(*mut Display) -> i32>;
    fn XInternAtom(_: *mut Display, _: *const i8, _: i32) -> Atom;
    fn XCreateGC(_: *mut Display, _: Drawable, _: u64, _: *mut XGCValues) -> GC;
    fn XCreatePixmap(_: *mut Display, _: Drawable, _: u32, _: u32, _: u32) -> Pixmap;
    fn XCreateSimpleWindow(
        _: *mut Display,
        _: Window,
        _: i32,
        _: i32,
        _: u32,
        _: u32,
        _: u32,
        _: u64,
        _: u64,
    ) -> Window;
    fn XSetErrorHandler(_: XErrorHandler) -> XErrorHandler;
    fn XFreeStringList(_: *mut *mut i8);
    fn XChangeProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: Atom,
        _: i32,
        _: i32,
        _: *const u8,
        _: i32,
    ) -> i32;
    fn XClearArea(
        _: *mut Display,
        _: Window,
        _: i32,
        _: i32,
        _: u32,
        _: u32,
        _: i32,
    ) -> i32;
    fn XDeleteProperty(_: *mut Display, _: Window, _: Atom) -> i32;
    fn XDestroyWindow(_: *mut Display, _: Window) -> i32;
    fn XEventsQueued(_: *mut Display, _: i32) -> i32;
    fn XFlush(_: *mut Display) -> i32;
    fn XFree(_: *mut libc::c_void) -> i32;
    fn XFreeGC(_: *mut Display, _: GC) -> i32;
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> i32;
    fn XGetErrorText(_: *mut Display, _: i32, _: *mut i8, _: i32) -> i32;
    fn XGetGeometry(
        _: *mut Display,
        _: Drawable,
        _: *mut Window,
        _: *mut i32,
        _: *mut i32,
        _: *mut u32,
        _: *mut u32,
        _: *mut u32,
        _: *mut u32,
    ) -> i32;
    fn XGetInputFocus(_: *mut Display, _: *mut Window, _: *mut i32) -> i32;
    fn XGetWindowProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: i64,
        _: i64,
        _: i32,
        _: Atom,
        _: *mut Atom,
        _: *mut i32,
        _: *mut u64,
        _: *mut u64,
        _: *mut *mut u8,
    ) -> i32;
    fn XGetWindowAttributes(
        _: *mut Display,
        _: Window,
        _: *mut XWindowAttributes,
    ) -> i32;
    fn XGrabServer(_: *mut Display) -> i32;
    fn XMapWindow(_: *mut Display, _: Window) -> i32;
    fn XNextEvent(_: *mut Display, _: *mut XEvent) -> i32;
    fn XPutImage(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XImage,
        _: i32,
        _: i32,
        _: i32,
        _: i32,
        _: u32,
        _: u32,
    ) -> i32;
    fn XQueryExtension(
        _: *mut Display,
        _: *const i8,
        _: *mut i32,
        _: *mut i32,
        _: *mut i32,
    ) -> i32;
    fn XQueryTree(
        _: *mut Display,
        _: Window,
        _: *mut Window,
        _: *mut Window,
        _: *mut *mut Window,
        _: *mut u32,
    ) -> i32;
    fn XSelectInput(_: *mut Display, _: Window, _: i64) -> i32;
    fn XSetSelectionOwner(_: *mut Display, _: Atom, _: Window, _: Time) -> i32;
    fn XSync(_: *mut Display, _: i32) -> i32;
    fn XUngrabServer(_: *mut Display) -> i32;
    fn XUnmapWindow(_: *mut Display, _: Window) -> i32;
    fn XAllocClassHint() -> *mut XClassHint;
    fn XGetTextProperty(
        _: *mut Display,
        _: Window,
        _: *mut XTextProperty,
        _: Atom,
    ) -> i32;
    fn XGetWMName(_: *mut Display, _: Window, _: *mut XTextProperty) -> i32;
    fn XSetTextProperty(_: *mut Display, _: Window, _: *mut XTextProperty, _: Atom);
    fn Xutf8SetWMProperties(
        _: *mut Display,
        _: Window,
        _: *const i8,
        _: *const i8,
        _: *mut *mut i8,
        _: i32,
        _: *mut XSizeHints,
        _: *mut XWMHints,
        _: *mut XClassHint,
    );
    fn XmbTextListToTextProperty(
        display: *mut Display,
        list: *mut *mut i8,
        count: i32,
        style: XICCEncodingStyle,
        text_prop_return: *mut XTextProperty,
    ) -> i32;
    fn XmbTextPropertyToTextList(
        display: *mut Display,
        text_prop: *const XTextProperty,
        list_return: *mut *mut *mut i8,
        count_return: *mut i32,
    ) -> i32;
    fn XFixesQueryExtension(
        dpy: *mut Display,
        event_base_return: *mut i32,
        error_base_return: *mut i32,
    ) -> i32;
    fn XFixesCreateRegion(
        dpy: *mut Display,
        rectangles: *mut XRectangle,
        nrectangles: i32,
    ) -> XserverRegion;
    fn XFixesCreateRegionFromWindow(
        dpy: *mut Display,
        window: Window,
        kind: i32,
    ) -> XserverRegion;
    fn XFixesDestroyRegion(dpy: *mut Display, region: XserverRegion);
    fn XFixesSetRegion(
        dpy: *mut Display,
        region: XserverRegion,
        rectangles: *mut XRectangle,
        nrectangles: i32,
    );
    fn XFixesCopyRegion(dpy: *mut Display, dst: XserverRegion, src: XserverRegion);
    fn XFixesUnionRegion(
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
    fn XFixesSubtractRegion(
        dpy: *mut Display,
        dst: XserverRegion,
        src1: XserverRegion,
        src2: XserverRegion,
    );
    fn XFixesTranslateRegion(dpy: *mut Display, region: XserverRegion, dx: i32, dy: i32);
    fn XFixesSetWindowShapeRegion(
        dpy: *mut Display,
        win: Window,
        shape_kind: i32,
        x_off: i32,
        y_off: i32,
        region: XserverRegion,
    );
    fn XFixesSetPictureClipRegion(
        dpy: *mut Display,
        picture: XID,
        clip_x_origin: i32,
        clip_y_origin: i32,
        region: XserverRegion,
    );
    fn XCompositeQueryVersion(
        dpy: *mut Display,
        major_version_return: *mut i32,
        minor_version_return: *mut i32,
    ) -> i32;
    fn XCompositeRedirectSubwindows(dpy: *mut Display, window: Window, update: i32);
    fn XCompositeUnredirectWindow(dpy: *mut Display, window: Window, update: i32);
    fn XCompositeUnredirectSubwindows(dpy: *mut Display, window: Window, update: i32);
    fn XCompositeNameWindowPixmap(dpy: *mut Display, window: Window) -> Pixmap;
    fn XCompositeGetOverlayWindow(dpy: *mut Display, window: Window) -> Window;
    fn XCompositeReleaseOverlayWindow(dpy: *mut Display, window: Window);
    fn XDamageQueryExtension(
        dpy: *mut Display,
        event_base_return: *mut i32,
        error_base_return: *mut i32,
    ) -> i32;
    fn XDamageCreate(dpy: *mut Display, drawable: Drawable, level: i32) -> Damage;
    fn XDamageDestroy(dpy: *mut Display, damage: Damage);
    fn XDamageSubtract(
        dpy: *mut Display,
        damage: Damage,
        repair: XserverRegion,
        parts: XserverRegion,
    );
    fn glXMakeCurrent(dpy: *mut Display, drawable: GLXDrawable, ctx: GLXContext) -> i32;
    fn glXSwapBuffers(dpy: *mut Display, drawable: GLXDrawable);
    fn glXWaitX();
    fn glXGetProcAddress(
        procname: *const GLubyte,
    ) -> Option<unsafe extern "C" fn() -> ()>;
    fn glx_init(ps: *mut session_t, need_render: bool) -> bool;
    fn __errno_location() -> *mut i32;
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn glx_destroy(ps: *mut session_t);
    fn glx_reinit(ps: *mut session_t, need_render: bool) -> bool;
    fn glx_on_root_change(ps: *mut session_t);
    fn glx_init_blur(ps: *mut session_t) -> bool;
    fn glx_load_prog_main(
        ps: *mut session_t,
        vshader_str: *const i8,
        fshader_str: *const i8,
        pprogram: *mut glx_prog_main_t,
    ) -> bool;
    fn glx_bind_pixmap(
        ps: *mut session_t,
        pptex: *mut *mut glx_texture_t,
        pixmap: Pixmap,
        width: u32,
        height: u32,
        depth: u32,
    ) -> bool;
    fn glx_release_pixmap(ps: *mut session_t, ptex: *mut glx_texture_t);
    fn glx_paint_pre(ps: *mut session_t, preg: *mut XserverRegion);
    fn glx_set_clip(
        ps: *mut session_t,
        reg: XserverRegion,
        pcache_reg: *const reg_data_t,
    );
    fn glx_blur_dst(
        ps: *mut session_t,
        dx: i32,
        dy: i32,
        width: i32,
        height: i32,
        z: libc::c_float,
        factor_center: GLfloat,
        reg_tgt: XserverRegion,
        pcache_reg: *const reg_data_t,
        pbc: *mut glx_blur_cache_t,
    ) -> bool;
    fn glx_dim_dst(
        ps: *mut session_t,
        dx: i32,
        dy: i32,
        width: i32,
        height: i32,
        z: libc::c_float,
        factor: GLfloat,
        reg_tgt: XserverRegion,
        pcache_reg: *const reg_data_t,
    ) -> bool;
    fn glx_render_(
        ps: *mut session_t,
        ptex: *const glx_texture_t,
        x: i32,
        y: i32,
        dx: i32,
        dy: i32,
        width: i32,
        height: i32,
        z: i32,
        opacity: libc::c_double,
        argb: bool,
        neg: bool,
        reg_tgt: XserverRegion,
        pcache_reg: *const reg_data_t,
        pprogram: *const glx_prog_main_t,
    ) -> bool;
    fn glx_swap_copysubbuffermesa(ps: *mut session_t, reg: XserverRegion);
    fn cdbus_init(ps: *mut session_t) -> bool;
    fn cdbus_destroy(ps: *mut session_t);
    fn cdbus_loop(ps: *mut session_t);
    fn cdbus_ev_win_added(ps: *mut session_t, w: *mut win);
    fn cdbus_ev_win_destroyed(ps: *mut session_t, w: *mut win);
    fn cdbus_ev_win_mapped(ps: *mut session_t, w: *mut win);
    fn cdbus_ev_win_unmapped(ps: *mut session_t, w: *mut win);
    fn cdbus_ev_win_focusout(ps: *mut session_t, w: *mut win);
    fn cdbus_ev_win_focusin(ps: *mut session_t, w: *mut win);
    fn c2_matchd(
        ps: *mut session_t,
        w: *mut win,
        condlst: *const c2_lptr_t,
        cache: *mut *const c2_lptr_t,
        pdata: *mut *mut libc::c_void,
    ) -> bool;
    fn c2_parsed(
        ps: *mut session_t,
        pcondlst: *mut *mut c2_lptr_t,
        pattern: *const i8,
        data: *mut libc::c_void,
    ) -> *mut c2_lptr_t;
    fn c2_free_lptr(lp: *mut c2_lptr_t) -> *mut c2_lptr_t;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaction(__sig: i32, __act: *const sigaction, __oact: *mut sigaction) -> i32;
    fn close(__fd: i32) -> i32;
    fn _exit(__status: i32) -> !;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
}
pub type size_t = u64;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uid_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __clock_t = i64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type uint32_t = __uint32_t;
pub type int_fast16_t = i64;
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut libc::c_void,
}
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
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
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
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            8192 => C2RustUnnamed::_ISspace,
            4096 => C2RustUnnamed::_ISxdigit,
            2048 => C2RustUnnamed::_ISdigit,
            1024 => C2RustUnnamed::_ISalpha,
            512 => C2RustUnnamed::_ISlower,
            256 => C2RustUnnamed::_ISupper,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
pub type __timezone_ptr_t = *mut timezone;
pub type XID = u64;
pub type Atom = u64;
pub type VisualID = u64;
pub type Time = u64;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Colormap = XID;
pub type XPointer = *mut i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: i32,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(*mut _XExtData) -> i32>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: i32,
    pub plane_mask: u64,
    pub foreground: u64,
    pub background: u64,
    pub line_width: i32,
    pub line_style: i32,
    pub cap_style: i32,
    pub join_style: i32,
    pub fill_style: i32,
    pub fill_rule: i32,
    pub arc_mode: i32,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: i32,
    pub ts_y_origin: i32,
    pub font: Font,
    pub subwindow_mode: i32,
    pub graphics_exposures: i32,
    pub clip_x_origin: i32,
    pub clip_y_origin: i32,
    pub clip_mask: Pixmap,
    pub dash_offset: i32,
    pub dashes: i8,
}
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: i32,
    pub red_mask: u64,
    pub green_mask: u64,
    pub blue_mask: u64,
    pub bits_per_rgb: i32,
    pub map_entries: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: i32,
    pub nvisuals: i32,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: i32,
    pub height: i32,
    pub mwidth: i32,
    pub mheight: i32,
    pub ndepths: i32,
    pub depths: *mut Depth,
    pub root_depth: i32,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: u64,
    pub black_pixel: u64,
    pub max_maps: i32,
    pub min_maps: i32,
    pub backing_store: i32,
    pub save_unders: i32,
    pub root_input_mask: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: i32,
    pub bits_per_pixel: i32,
    pub scanline_pad: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWindowAttributes {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub depth: i32,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: i32,
    pub bit_gravity: i32,
    pub win_gravity: i32,
    pub backing_store: i32,
    pub backing_planes: u64,
    pub backing_pixel: u64,
    pub save_under: i32,
    pub colormap: Colormap,
    pub map_installed: i32,
    pub map_state: i32,
    pub all_event_masks: i64,
    pub your_event_mask: i64,
    pub do_not_propagate_mask: i64,
    pub override_redirect: i32,
    pub screen: *mut Screen,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XImage {
    pub width: i32,
    pub height: i32,
    pub xoffset: i32,
    pub format: i32,
    pub data: *mut i8,
    pub byte_order: i32,
    pub bitmap_unit: i32,
    pub bitmap_bit_order: i32,
    pub bitmap_pad: i32,
    pub depth: i32,
    pub bytes_per_line: i32,
    pub bits_per_pixel: i32,
    pub red_mask: u64,
    pub green_mask: u64,
    pub blue_mask: u64,
    pub obdata: XPointer,
    pub f: funcs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcs {
    pub create_image: Option<
        unsafe extern "C" fn(
            *mut _XDisplay,
            *mut Visual,
            u32,
            i32,
            i32,
            *mut i8,
            u32,
            u32,
            i32,
            i32,
        ) -> *mut _XImage,
    >,
    pub destroy_image: Option<unsafe extern "C" fn(*mut _XImage) -> i32>,
    pub get_pixel: Option<unsafe extern "C" fn(*mut _XImage, i32, i32) -> u64>,
    pub put_pixel: Option<unsafe extern "C" fn(*mut _XImage, i32, i32, u64) -> i32>,
    pub sub_image: Option<
        unsafe extern "C" fn(*mut _XImage, i32, i32, u32, u32) -> *mut _XImage,
    >,
    pub add_pixel: Option<unsafe extern "C" fn(*mut _XImage, i64) -> i32>,
}
pub type XImage = _XImage;
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
pub struct C2RustUnnamed_0 {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: i32,
    pub private2: i32,
    pub proto_major_version: i32,
    pub proto_minor_version: i32,
    pub vendor: *mut i8,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: i32,
    pub resource_alloc: Option<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: i32,
    pub bitmap_unit: i32,
    pub bitmap_pad: i32,
    pub bitmap_bit_order: i32,
    pub nformats: i32,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: i32,
    pub release: i32,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: i32,
    pub last_request_read: u64,
    pub request: u64,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: u32,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(*mut _XDisplay) -> i32>,
    pub display_name: *mut i8,
    pub default_screen: i32,
    pub nscreens: i32,
    pub screens: *mut Screen,
    pub motion_buffer: u64,
    pub private16: u64,
    pub min_keycode: i32,
    pub max_keycode: i32,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: i32,
    pub xdefaults: *mut i8,
}
pub type _XPrivDisplay = *mut C2RustUnnamed_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub keycode: u32,
    pub same_screen: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub button: u32,
    pub same_screen: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub is_hint: i8,
    pub same_screen: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub mode: i32,
    pub detail: i32,
    pub same_screen: i32,
    pub focus: i32,
    pub state: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub mode: i32,
    pub detail: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [i8; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub count: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub count: i32,
    pub major_code: i32,
    pub minor_code: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: i32,
    pub minor_code: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub state: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub override_redirect: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: i32,
    pub y: i32,
    pub override_redirect: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub above: Window,
    pub override_redirect: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub width: i32,
    pub height: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub above: Window,
    pub detail: i32,
    pub value_mask: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: i32,
    pub state: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: i32,
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub b: [i8; 20],
    pub s: [libc::c_short; 10],
    pub l: [i64; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub request: i32,
    pub first_keycode: i32,
    pub count: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: i32,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: u64,
    pub error_code: u8,
    pub request_code: u8,
    pub minor_code: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub extension: i32,
    pub evtype: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub extension: i32,
    pub evtype: i32,
    pub cookie: u32,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: i32,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [i64; 24],
}
pub type XEvent = _XEvent;
pub type XErrorHandler = Option<
    unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSizeHints {
    pub flags: i64,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub min_width: i32,
    pub min_height: i32,
    pub max_width: i32,
    pub max_height: i32,
    pub width_inc: i32,
    pub height_inc: i32,
    pub min_aspect: C2RustUnnamed_2,
    pub max_aspect: C2RustUnnamed_2,
    pub base_width: i32,
    pub base_height: i32,
    pub win_gravity: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub x: i32,
    pub y: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWMHints {
    pub flags: i64,
    pub input: i32,
    pub initial_state: i32,
    pub icon_pixmap: Pixmap,
    pub icon_window: Window,
    pub icon_x: i32,
    pub icon_y: i32,
    pub icon_mask: Pixmap,
    pub window_group: XID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XTextProperty {
    pub value: *mut u8,
    pub encoding: Atom,
    pub format: i32,
    pub nitems: u64,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum XICCEncodingStyle {
    XUTF8StringStyle = 4,
    XStdICCTextStyle = 3,
    XTextStyle = 2,
    XCompoundTextStyle = 1,
    XStringStyle = 0,
}
impl XICCEncodingStyle {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            XICCEncodingStyle::XUTF8StringStyle => 4,
            XICCEncodingStyle::XStdICCTextStyle => 3,
            XICCEncodingStyle::XTextStyle => 2,
            XICCEncodingStyle::XCompoundTextStyle => 1,
            XICCEncodingStyle::XStringStyle => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> XICCEncodingStyle {
        match value {
            4 => XICCEncodingStyle::XUTF8StringStyle,
            3 => XICCEncodingStyle::XStdICCTextStyle,
            2 => XICCEncodingStyle::XTextStyle,
            1 => XICCEncodingStyle::XCompoundTextStyle,
            0 => XICCEncodingStyle::XStringStyle,
            _ => panic!("Invalid value for XICCEncodingStyle: {}", value),
        }
    }
}
impl AddAssign<u32> for XICCEncodingStyle {
    fn add_assign(&mut self, rhs: u32) {
        *self = XICCEncodingStyle::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for XICCEncodingStyle {
    fn sub_assign(&mut self, rhs: u32) {
        *self = XICCEncodingStyle::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for XICCEncodingStyle {
    fn mul_assign(&mut self, rhs: u32) {
        *self = XICCEncodingStyle::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for XICCEncodingStyle {
    fn div_assign(&mut self, rhs: u32) {
        *self = XICCEncodingStyle::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for XICCEncodingStyle {
    fn rem_assign(&mut self, rhs: u32) {
        *self = XICCEncodingStyle::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for XICCEncodingStyle {
    type Output = XICCEncodingStyle;
    fn add(self, rhs: u32) -> XICCEncodingStyle {
        XICCEncodingStyle::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for XICCEncodingStyle {
    type Output = XICCEncodingStyle;
    fn sub(self, rhs: u32) -> XICCEncodingStyle {
        XICCEncodingStyle::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for XICCEncodingStyle {
    type Output = XICCEncodingStyle;
    fn mul(self, rhs: u32) -> XICCEncodingStyle {
        XICCEncodingStyle::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for XICCEncodingStyle {
    type Output = XICCEncodingStyle;
    fn div(self, rhs: u32) -> XICCEncodingStyle {
        XICCEncodingStyle::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for XICCEncodingStyle {
    type Output = XICCEncodingStyle;
    fn rem(self, rhs: u32) -> XICCEncodingStyle {
        XICCEncodingStyle::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClassHint {
    pub res_name: *mut i8,
    pub res_class: *mut i8,
}
pub type XserverRegion = XID;
pub type Damage = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDamageNotifyEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub damage: Damage,
    pub level: i32,
    pub more: i32,
    pub timestamp: Time,
    pub area: XRectangle,
    pub geometry: XRectangle,
}
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
    pub type_0: i32,
    pub depth: i32,
    pub direct: XRenderDirectFormat,
    pub colormap: Colormap,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XRenderPictureAttributes {
    pub repeat: i32,
    pub alpha_map: Picture,
    pub alpha_x_origin: i32,
    pub alpha_y_origin: i32,
    pub clip_x_origin: i32,
    pub clip_y_origin: i32,
    pub clip_mask: Pixmap,
    pub graphics_exposures: i32,
    pub subwindow_mode: i32,
    pub poly_edge: i32,
    pub poly_mode: i32,
    pub dither: Atom,
    pub component_alpha: i32,
}
pub type XRenderPictureAttributes = _XRenderPictureAttributes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderColor {
    pub red: libc::c_ushort,
    pub green: libc::c_ushort,
    pub blue: libc::c_ushort,
    pub alpha: libc::c_ushort,
}
pub type XDouble = libc::c_double;
pub type XFixed = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XFilters {
    pub nfilter: i32,
    pub filter: *mut *mut i8,
    pub nalias: i32,
    pub alias: *mut libc::c_short,
}
pub type XFilters = _XFilters;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XShapeEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub kind: i32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub time: Time,
    pub shaped: i32,
}
pub type Rotation = libc::c_ushort;
pub type SizeID = libc::c_ushort;
pub type SubpixelOrder = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRRScreenChangeNotifyEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub timestamp: Time,
    pub config_timestamp: Time,
    pub size_index: SizeID,
    pub subpixel_order: SubpixelOrder,
    pub rotation: Rotation,
    pub width: i32,
    pub height: i32,
    pub mwidth: i32,
    pub mheight: i32,
}
pub type XRRScreenConfiguration = _XRRScreenConfiguration;
pub type XdbeBackBuffer = Drawable;
pub type XdbeSwapAction = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XdbeSwapInfo {
    pub swap_window: Window,
    pub swap_action: XdbeSwapAction,
}
pub type XSyncFence = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XineramaScreenInfo {
    pub screen_number: i32,
    pub x_org: libc::c_short,
    pub y_org: libc::c_short,
    pub width: libc::c_short,
    pub height: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union config_value_t {
    pub ival: i32,
    pub llval: libc::c_longlong,
    pub fval: libc::c_double,
    pub sval: *mut i8,
    pub list: *mut config_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_list_t {
    pub length: u32,
    pub elements: *mut *mut config_setting_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_setting_t {
    pub name: *mut i8,
    pub type_0: libc::c_short,
    pub format: libc::c_short,
    pub value: config_value_t,
    pub parent: *mut config_setting_t,
    pub config: *mut config_t,
    pub hook: *mut libc::c_void,
    pub line: u32,
    pub file: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_t {
    pub root: *mut config_setting_t,
    pub destructor: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub options: i32,
    pub tab_width: libc::c_ushort,
    pub default_format: libc::c_short,
    pub include_dir: *const i8,
    pub error_text: *const i8,
    pub error_file: *const i8,
    pub error_line: i32,
    pub error_type: config_error_t,
    pub filenames: *mut *const i8,
    pub num_filenames: u32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum config_error_t {
    CONFIG_ERR_PARSE = 2,
    CONFIG_ERR_FILE_IO = 1,
    CONFIG_ERR_NONE = 0,
}
impl config_error_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            config_error_t::CONFIG_ERR_PARSE => 2,
            config_error_t::CONFIG_ERR_FILE_IO => 1,
            config_error_t::CONFIG_ERR_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> config_error_t {
        match value {
            2 => config_error_t::CONFIG_ERR_PARSE,
            1 => config_error_t::CONFIG_ERR_FILE_IO,
            0 => config_error_t::CONFIG_ERR_NONE,
            _ => panic!("Invalid value for config_error_t: {}", value),
        }
    }
}
impl AddAssign<u32> for config_error_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = config_error_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for config_error_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = config_error_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for config_error_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = config_error_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for config_error_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = config_error_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for config_error_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = config_error_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for config_error_t {
    type Output = config_error_t;
    fn add(self, rhs: u32) -> config_error_t {
        config_error_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for config_error_t {
    type Output = config_error_t;
    fn sub(self, rhs: u32) -> config_error_t {
        config_error_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for config_error_t {
    type Output = config_error_t;
    fn mul(self, rhs: u32) -> config_error_t {
        config_error_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for config_error_t {
    type Output = config_error_t;
    fn div(self, rhs: u32) -> config_error_t {
        config_error_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for config_error_t {
    type Output = config_error_t;
    fn rem(self, rhs: u32) -> config_error_t {
        config_error_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type GLenum = u32;
pub type GLint = i32;
pub type GLubyte = u8;
pub type GLuint = u32;
pub type GLsizei = i32;
pub type GLfloat = libc::c_float;
pub type GLXContext = *mut __GLXcontextRec;
pub type GLXPixmap = XID;
pub type GLXDrawable = XID;
pub type GLXFBConfig = *mut __GLXFBConfigRec;
pub type opacity_t = uint32_t;
pub type time_ms_t = i64;
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
}
impl wintype_t {
    fn to_libc_c_uint(self) -> u32 {
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
    fn from_libc_c_uint(value: u32) -> wintype_t {
        match value {
            0 => wintype_t::WINTYPE_UNKNOWN,
            1 => wintype_t::WINTYPE_DESKTOP,
            2 => wintype_t::WINTYPE_DOCK,
            3 => wintype_t::WINTYPE_TOOLBAR,
            4 => wintype_t::WINTYPE_MENU,
            5 => wintype_t::WINTYPE_UTILITY,
            6 => wintype_t::WINTYPE_SPLASH,
            7 => wintype_t::WINTYPE_DIALOG,
            8 => wintype_t::WINTYPE_NORMAL,
            9 => wintype_t::WINTYPE_DROPDOWN_MENU,
            10 => wintype_t::WINTYPE_POPUP_MENU,
            11 => wintype_t::WINTYPE_TOOLTIP,
            12 => wintype_t::WINTYPE_NOTIFY,
            13 => wintype_t::WINTYPE_COMBO,
            14 => wintype_t::WINTYPE_DND,
            15 => wintype_t::NUM_WINTYPES,
            _ => panic!("Invalid value for wintype_t: {}", value),
        }
    }
}
impl AddAssign<u32> for wintype_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = wintype_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for wintype_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = wintype_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for wintype_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = wintype_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for wintype_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = wintype_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for wintype_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = wintype_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for wintype_t {
    type Output = wintype_t;
    fn add(self, rhs: u32) -> wintype_t {
        wintype_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for wintype_t {
    type Output = wintype_t;
    fn sub(self, rhs: u32) -> wintype_t {
        wintype_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for wintype_t {
    type Output = wintype_t;
    fn mul(self, rhs: u32) -> wintype_t {
        wintype_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for wintype_t {
    type Output = wintype_t;
    fn div(self, rhs: u32) -> wintype_t {
        wintype_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for wintype_t {
    type Output = wintype_t;
    fn rem(self, rhs: u32) -> wintype_t {
        wintype_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum switch_t {
    OFF,
    ON,
    UNSET,
}
impl switch_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            switch_t::OFF => 0,
            switch_t::ON => 1,
            switch_t::UNSET => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> switch_t {
        match value {
            0 => switch_t::OFF,
            1 => switch_t::ON,
            2 => switch_t::UNSET,
            _ => panic!("Invalid value for switch_t: {}", value),
        }
    }
}
impl AddAssign<u32> for switch_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = switch_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for switch_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = switch_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for switch_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = switch_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for switch_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = switch_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for switch_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = switch_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for switch_t {
    type Output = switch_t;
    fn add(self, rhs: u32) -> switch_t {
        switch_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for switch_t {
    type Output = switch_t;
    fn sub(self, rhs: u32) -> switch_t {
        switch_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for switch_t {
    type Output = switch_t;
    fn mul(self, rhs: u32) -> switch_t {
        switch_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for switch_t {
    type Output = switch_t;
    fn div(self, rhs: u32) -> switch_t {
        switch_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for switch_t {
    type Output = switch_t;
    fn rem(self, rhs: u32) -> switch_t {
        switch_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct geometry_t {
    pub wid: i32,
    pub hei: i32,
    pub x: i32,
    pub y: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct margin_t {
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum winmode_t {
    WMODE_TRANS,
    WMODE_SOLID,
    WMODE_ARGB,
}
impl winmode_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            winmode_t::WMODE_TRANS => 0,
            winmode_t::WMODE_SOLID => 1,
            winmode_t::WMODE_ARGB => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> winmode_t {
        match value {
            0 => winmode_t::WMODE_TRANS,
            1 => winmode_t::WMODE_SOLID,
            2 => winmode_t::WMODE_ARGB,
            _ => panic!("Invalid value for winmode_t: {}", value),
        }
    }
}
impl AddAssign<u32> for winmode_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = winmode_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for winmode_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = winmode_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for winmode_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = winmode_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for winmode_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = winmode_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for winmode_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = winmode_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for winmode_t {
    type Output = winmode_t;
    fn add(self, rhs: u32) -> winmode_t {
        winmode_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for winmode_t {
    type Output = winmode_t;
    fn sub(self, rhs: u32) -> winmode_t {
        winmode_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for winmode_t {
    type Output = winmode_t;
    fn mul(self, rhs: u32) -> winmode_t {
        winmode_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for winmode_t {
    type Output = winmode_t;
    fn div(self, rhs: u32) -> winmode_t {
        winmode_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for winmode_t {
    type Output = winmode_t;
    fn rem(self, rhs: u32) -> winmode_t {
        winmode_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winprop_t {
    pub data: C2RustUnnamed_3,
    pub nitems: u64,
    pub type_0: Atom,
    pub format: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub p8: *mut u8,
    pub p16: *mut libc::c_short,
    pub p32: *mut i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ignore {
    pub next: *mut _ignore,
    pub sequence: u64,
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
}
impl vsync_t {
    fn to_libc_c_uint(self) -> u32 {
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
    fn from_libc_c_uint(value: u32) -> vsync_t {
        match value {
            0 => vsync_t::VSYNC_NONE,
            1 => vsync_t::VSYNC_DRM,
            2 => vsync_t::VSYNC_OPENGL,
            3 => vsync_t::VSYNC_OPENGL_OML,
            4 => vsync_t::VSYNC_OPENGL_SWC,
            5 => vsync_t::VSYNC_OPENGL_MSWC,
            6 => vsync_t::NUM_VSYNC,
            _ => panic!("Invalid value for vsync_t: {}", value),
        }
    }
}
impl AddAssign<u32> for vsync_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = vsync_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for vsync_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = vsync_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for vsync_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = vsync_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for vsync_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = vsync_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for vsync_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = vsync_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for vsync_t {
    type Output = vsync_t;
    fn add(self, rhs: u32) -> vsync_t {
        vsync_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for vsync_t {
    type Output = vsync_t;
    fn sub(self, rhs: u32) -> vsync_t {
        vsync_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for vsync_t {
    type Output = vsync_t;
    fn mul(self, rhs: u32) -> vsync_t {
        vsync_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for vsync_t {
    type Output = vsync_t;
    fn div(self, rhs: u32) -> vsync_t {
        vsync_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for vsync_t {
    type Output = vsync_t;
    fn rem(self, rhs: u32) -> vsync_t {
        vsync_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum backend {
    BKEND_XRENDER,
    BKEND_GLX,
    BKEND_XR_GLX_HYBRID,
    NUM_BKEND,
}
impl backend {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            backend::BKEND_XRENDER => 0,
            backend::BKEND_GLX => 1,
            backend::BKEND_XR_GLX_HYBRID => 2,
            backend::NUM_BKEND => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> backend {
        match value {
            0 => backend::BKEND_XRENDER,
            1 => backend::BKEND_GLX,
            2 => backend::BKEND_XR_GLX_HYBRID,
            3 => backend::NUM_BKEND,
            _ => panic!("Invalid value for backend: {}", value),
        }
    }
}
impl AddAssign<u32> for backend {
    fn add_assign(&mut self, rhs: u32) {
        *self = backend::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for backend {
    fn sub_assign(&mut self, rhs: u32) {
        *self = backend::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for backend {
    fn mul_assign(&mut self, rhs: u32) {
        *self = backend::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for backend {
    fn div_assign(&mut self, rhs: u32) {
        *self = backend::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for backend {
    fn rem_assign(&mut self, rhs: u32) {
        *self = backend::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for backend {
    type Output = backend;
    fn add(self, rhs: u32) -> backend {
        backend::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for backend {
    type Output = backend;
    fn sub(self, rhs: u32) -> backend {
        backend::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for backend {
    type Output = backend;
    fn mul(self, rhs: u32) -> backend {
        backend::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for backend {
    type Output = backend;
    fn div(self, rhs: u32) -> backend {
        backend::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for backend {
    type Output = backend;
    fn rem(self, rhs: u32) -> backend {
        backend::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _glx_texture {
    pub texture: GLuint,
    pub glpixmap: GLXPixmap,
    pub pixmap: Pixmap,
    pub target: GLenum,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub y_inverted: bool,
}
pub type glx_texture_t = _glx_texture;
pub type f_WaitVideoSync = Option<unsafe extern "C" fn(i32, i32, *mut u32) -> i32>;
pub type f_GetVideoSync = Option<unsafe extern "C" fn(*mut u32) -> i32>;
pub type f_GetSyncValuesOML = Option<
    unsafe extern "C" fn(
        *mut Display,
        GLXDrawable,
        *mut int64_t,
        *mut int64_t,
        *mut int64_t,
    ) -> i32,
>;
pub type f_WaitForMscOML = Option<
    unsafe extern "C" fn(
        *mut Display,
        GLXDrawable,
        int64_t,
        int64_t,
        int64_t,
        *mut int64_t,
        *mut int64_t,
        *mut int64_t,
    ) -> i32,
>;
pub type f_SwapIntervalSGI = Option<unsafe extern "C" fn(i32) -> i32>;
pub type f_SwapIntervalMESA = Option<unsafe extern "C" fn(u32) -> i32>;
pub type f_BindTexImageEXT = Option<
    unsafe extern "C" fn(*mut Display, GLXDrawable, i32, *const i32) -> (),
>;
pub type f_ReleaseTexImageEXT = Option<
    unsafe extern "C" fn(*mut Display, GLXDrawable, i32) -> (),
>;
pub type f_CopySubBuffer = Option<
    unsafe extern "C" fn(*mut Display, GLXDrawable, i32, i32, i32, i32) -> (),
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
    pub width: i32,
    pub height: i32,
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
    pub size: i32,
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
    pub nrects: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _timeout_t {
    pub enabled: bool,
    pub data: *mut libc::c_void,
    pub callback: Option<unsafe extern "C" fn(*mut session_t, *mut _timeout_t) -> bool>,
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
    pub scr: i32,
    pub vis: *mut Visual,
    pub depth: i32,
    pub root: Window,
    pub root_height: i32,
    pub root_width: i32,
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
    pub nfds_max: i32,
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
    pub size_expose: i32,
    pub n_expose: i32,
    pub list: *mut _win,
    pub active_win: *mut _win,
    pub active_leader: Window,
    pub black_picture: Picture,
    pub cshadow_picture: Picture,
    pub white_picture: Picture,
    pub gaussian_map: *mut conv,
    pub cgsize: i32,
    pub shadow_corner: *mut u8,
    pub shadow_top: *mut u8,
    pub shadow_exclude_reg: XserverRegion,
    pub refresh_rate: libc::c_short,
    pub refresh_intv: i64,
    pub paint_tm_offset: i64,
    pub drm_fd: i32,
    pub xfixes_event: i32,
    pub xfixes_error: i32,
    pub damage_event: i32,
    pub damage_error: i32,
    pub render_event: i32,
    pub render_error: i32,
    pub composite_event: i32,
    pub composite_error: i32,
    pub composite_opcode: i32,
    pub has_name_pixmap: bool,
    pub shape_exists: bool,
    pub shape_event: i32,
    pub shape_error: i32,
    pub randr_exists: bool,
    pub randr_event: i32,
    pub randr_error: i32,
    pub glx_exists: bool,
    pub glx_event: i32,
    pub glx_error: i32,
    pub dbe_exists: bool,
    pub xinerama_exists: bool,
    pub xinerama_scrs: *mut XineramaScreenInfo,
    pub xinerama_scr_regs: *mut XserverRegion,
    pub xinerama_nscrs: i32,
    pub xsync_exists: bool,
    pub xsync_event: i32,
    pub xsync_error: i32,
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
    pub dbus_service: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win {
    pub next: *mut _win,
    pub prev_trans: *mut _win,
    pub id: Window,
    pub a: XWindowAttributes,
    pub xinerama_scr: i32,
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
    pub widthb: i32,
    pub heightb: i32,
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
    pub name: *mut i8,
    pub class_instance: *mut i8,
    pub class_general: *mut i8,
    pub role: *mut i8,
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
    pub fade_callback: Option<unsafe extern "C" fn(*mut session_t, *mut _win) -> ()>,
    pub frame_opacity: libc::c_double,
    pub frame_extents: margin_t,
    pub shadow: bool,
    pub shadow_last: bool,
    pub shadow_force: switch_t,
    pub shadow_opacity: libc::c_double,
    pub shadow_dx: i32,
    pub shadow_dy: i32,
    pub shadow_width: i32,
    pub shadow_height: i32,
    pub shadow_paint: paint_t,
    pub prop_shadow: i64,
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
    pub config_file: *mut i8,
    pub write_pid_path: *mut i8,
    pub display: *mut i8,
    pub display_repr: *mut i8,
    pub backend: backend,
    pub xrender_sync: bool,
    pub xrender_sync_fence: bool,
    pub glx_no_stencil: bool,
    pub glx_copy_from_front: bool,
    pub glx_use_copysubbuffermesa: bool,
    pub glx_no_rebind_pixmap: bool,
    pub glx_swap_method: i32,
    pub glx_use_gpushader4: bool,
    pub glx_fshader_win_str: *mut i8,
    pub glx_prog_win: glx_prog_main_t,
    pub fork_after_register: bool,
    pub detect_rounded_corners: bool,
    pub paint_on_overlay: bool,
    pub force_win_blend: bool,
    pub resize_damage: i32,
    pub unredir_if_possible: bool,
    pub unredir_if_possible_blacklist: *mut c2_lptr_t,
    pub unredir_if_possible_delay: time_ms_t,
    pub redirected_force: switch_t,
    pub stoppaint_force: switch_t,
    pub reredir_on_root_change: bool,
    pub glx_reinit_on_root_change: bool,
    pub dbus: bool,
    pub logpath: *mut i8,
    pub benchmark: i32,
    pub benchmark_wid: Window,
    pub paint_blacklist: *mut c2_lptr_t,
    pub no_name_pixmap: bool,
    pub synchronize: bool,
    pub show_all_xerrors: bool,
    pub no_x_selection: bool,
    pub refresh_rate: i32,
    pub sw_opti: bool,
    pub vsync: vsync_t,
    pub dbe: bool,
    pub vsync_aggressive: bool,
    pub vsync_use_glfinish: bool,
    pub wintype_shadow: [bool; 15],
    pub shadow_red: libc::c_double,
    pub shadow_green: libc::c_double,
    pub shadow_blue: libc::c_double,
    pub shadow_radius: i32,
    pub shadow_offset_x: i32,
    pub shadow_offset_y: i32,
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
    pub z: i32,
    pub fbconfigs: [*mut glx_fbconfig_t; 33],
    pub blur_passes: [glx_blur_pass_t; 5],
}
pub type win = _win;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_tmp {
    pub no_dock_shadow: bool,
    pub no_dnd_shadow: bool,
    pub menu_opacity: libc::c_double,
}
pub type timeout_t = _timeout_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum win_evmode_t {
    WIN_EVMODE_UNKNOWN,
    WIN_EVMODE_FRAME,
    WIN_EVMODE_CLIENT,
}
impl win_evmode_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            win_evmode_t::WIN_EVMODE_UNKNOWN => 0,
            win_evmode_t::WIN_EVMODE_FRAME => 1,
            win_evmode_t::WIN_EVMODE_CLIENT => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> win_evmode_t {
        match value {
            0 => win_evmode_t::WIN_EVMODE_UNKNOWN,
            1 => win_evmode_t::WIN_EVMODE_FRAME,
            2 => win_evmode_t::WIN_EVMODE_CLIENT,
            _ => panic!("Invalid value for win_evmode_t: {}", value),
        }
    }
}
impl AddAssign<u32> for win_evmode_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = win_evmode_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for win_evmode_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = win_evmode_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for win_evmode_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = win_evmode_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for win_evmode_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = win_evmode_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for win_evmode_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = win_evmode_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for win_evmode_t {
    type Output = win_evmode_t;
    fn add(self, rhs: u32) -> win_evmode_t {
        win_evmode_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for win_evmode_t {
    type Output = win_evmode_t;
    fn sub(self, rhs: u32) -> win_evmode_t {
        win_evmode_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for win_evmode_t {
    type Output = win_evmode_t;
    fn mul(self, rhs: u32) -> win_evmode_t {
        win_evmode_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for win_evmode_t {
    type Output = win_evmode_t;
    fn div(self, rhs: u32) -> win_evmode_t {
        win_evmode_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for win_evmode_t {
    type Output = win_evmode_t;
    fn rem(self, rhs: u32) -> win_evmode_t {
        win_evmode_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum drm_vblank_seq_type {
    _DRM_VBLANK_SIGNAL = 1073741824,
    _DRM_VBLANK_SECONDARY = 536870912,
    _DRM_VBLANK_NEXTONMISS = 268435456,
    _DRM_VBLANK_FLIP = 134217728,
    _DRM_VBLANK_EVENT = 67108864,
    _DRM_VBLANK_HIGH_CRTC_MASK = 62,
    _DRM_VBLANK_RELATIVE = 1,
    _DRM_VBLANK_ABSOLUTE = 0,
}
impl drm_vblank_seq_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            drm_vblank_seq_type::_DRM_VBLANK_SIGNAL => 1073741824,
            drm_vblank_seq_type::_DRM_VBLANK_SECONDARY => 536870912,
            drm_vblank_seq_type::_DRM_VBLANK_NEXTONMISS => 268435456,
            drm_vblank_seq_type::_DRM_VBLANK_FLIP => 134217728,
            drm_vblank_seq_type::_DRM_VBLANK_EVENT => 67108864,
            drm_vblank_seq_type::_DRM_VBLANK_HIGH_CRTC_MASK => 62,
            drm_vblank_seq_type::_DRM_VBLANK_RELATIVE => 1,
            drm_vblank_seq_type::_DRM_VBLANK_ABSOLUTE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> drm_vblank_seq_type {
        match value {
            1073741824 => drm_vblank_seq_type::_DRM_VBLANK_SIGNAL,
            536870912 => drm_vblank_seq_type::_DRM_VBLANK_SECONDARY,
            268435456 => drm_vblank_seq_type::_DRM_VBLANK_NEXTONMISS,
            134217728 => drm_vblank_seq_type::_DRM_VBLANK_FLIP,
            67108864 => drm_vblank_seq_type::_DRM_VBLANK_EVENT,
            62 => drm_vblank_seq_type::_DRM_VBLANK_HIGH_CRTC_MASK,
            1 => drm_vblank_seq_type::_DRM_VBLANK_RELATIVE,
            0 => drm_vblank_seq_type::_DRM_VBLANK_ABSOLUTE,
            _ => panic!("Invalid value for drm_vblank_seq_type: {}", value),
        }
    }
}
impl AddAssign<u32> for drm_vblank_seq_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = drm_vblank_seq_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for drm_vblank_seq_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = drm_vblank_seq_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for drm_vblank_seq_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = drm_vblank_seq_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for drm_vblank_seq_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = drm_vblank_seq_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for drm_vblank_seq_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = drm_vblank_seq_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for drm_vblank_seq_type {
    type Output = drm_vblank_seq_type;
    fn add(self, rhs: u32) -> drm_vblank_seq_type {
        drm_vblank_seq_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for drm_vblank_seq_type {
    type Output = drm_vblank_seq_type;
    fn sub(self, rhs: u32) -> drm_vblank_seq_type {
        drm_vblank_seq_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for drm_vblank_seq_type {
    type Output = drm_vblank_seq_type;
    fn mul(self, rhs: u32) -> drm_vblank_seq_type {
        drm_vblank_seq_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for drm_vblank_seq_type {
    type Output = drm_vblank_seq_type;
    fn div(self, rhs: u32) -> drm_vblank_seq_type {
        drm_vblank_seq_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for drm_vblank_seq_type {
    type Output = drm_vblank_seq_type;
    fn rem(self, rhs: u32) -> drm_vblank_seq_type {
        drm_vblank_seq_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drm_wait_vblank_request {
    pub type_0: drm_vblank_seq_type,
    pub sequence: u32,
    pub signal: u64,
}
pub type drm_wait_vblank_t = drm_wait_vblank;
#[derive(Copy, Clone)]
#[repr(C)]
pub union drm_wait_vblank {
    pub request: drm_wait_vblank_request,
    pub reply: drm_wait_vblank_reply,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drm_wait_vblank_reply {
    pub type_0: drm_vblank_seq_type,
    pub sequence: u32,
    pub tval_sec: i64,
    pub tval_usec: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _pad: [i32; 28],
    pub _kill: C2RustUnnamed_13,
    pub _timer: C2RustUnnamed_12,
    pub _rt: C2RustUnnamed_11,
    pub _sigchld: C2RustUnnamed_10,
    pub _sigfault: C2RustUnnamed_7,
    pub _sigpoll: C2RustUnnamed_6,
    pub _sigsys: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: i32,
    pub _arch: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_band: i64,
    pub si_fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub _addr_bnd: C2RustUnnamed_9,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: i32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_14,
    pub sa_mask: __sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub name: *const i8,
    pub kern_str: *const i8,
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const i8) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut i8);
}
#[inline]
unsafe extern "C" fn print_timestamp(mut ps: *mut session_t) {
    let mut tm: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut diff: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if gettimeofday(&mut tm, 0 as *mut timezone) != 0 {
        return;
    }
    timeval_subtract(&mut diff, &mut tm, &mut (*ps).time_start);
    printf(
        b"[ %5ld.%02ld ] \0" as *const u8 as *const i8,
        diff.tv_sec,
        diff.tv_usec / 10000 as i32 as i64,
    );
}
#[inline]
unsafe extern "C" fn timeval_subtract(
    mut result: *mut timeval,
    mut x: *mut timeval,
    mut y: *mut timeval,
) -> i32 {
    if (*x).tv_usec < (*y).tv_usec {
        let mut nsec: i64 = ((*y).tv_usec - (*x).tv_usec) / 1000000 as i32 as i64
            + 1 as i32 as i64;
        (*y).tv_usec -= 1000000 as i32 as i64 * nsec;
        (*y).tv_sec += nsec;
    }
    if (*x).tv_usec - (*y).tv_usec > 1000000 as i32 as i64 {
        let mut nsec_0: i64 = ((*x).tv_usec - (*y).tv_usec) / 1000000 as i32 as i64;
        (*y).tv_usec += 1000000 as i32 as i64 * nsec_0;
        (*y).tv_sec -= nsec_0;
    }
    (*result).tv_sec = (*x).tv_sec - (*y).tv_sec;
    (*result).tv_usec = (*x).tv_usec - (*y).tv_usec;
    return ((*x).tv_sec < (*y).tv_sec) as i32;
}
#[inline]
unsafe extern "C" fn allocchk_(
    mut func_name: *const i8,
    mut ptr: *mut libc::c_void,
) -> *mut libc::c_void {
    if ptr.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed to allocate memory.\n\0" as *const u8 as *const i8,
            func_name,
        );
        exit(1 as i32);
    }
    return ptr;
}
#[inline]
unsafe extern "C" fn timeval_isempty(mut ptv: *mut timeval) -> bool {
    if ptv.is_null() {
        return 0 as i32 != 0;
    }
    return (*ptv).tv_sec <= 0 as i32 as i64 && (*ptv).tv_usec <= 0 as i32 as i64;
}
#[inline]
unsafe extern "C" fn timeval_ms_cmp(mut ptv: *mut timeval, mut ms: time_ms_t) -> i32 {
    let mut sec: i64 = ms / 1000 as i32 as i64;
    if (*ptv).tv_sec > sec {
        return 1 as i32;
    }
    if (*ptv).tv_sec < sec {
        return -(1 as i32);
    }
    let mut usec: i64 = ms % 1000 as i32 as i64 * (1000000 as i64 / 1000 as i32 as i64);
    if (*ptv).tv_usec > usec {
        return 1 as i32;
    }
    if (*ptv).tv_usec < usec {
        return -(1 as i32);
    }
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn get_time_timeval() -> timeval {
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0 as i32 as __time_t,
            tv_usec: 0 as i32 as __suseconds_t,
        };
        init
    };
    gettimeofday(&mut tv, 0 as *mut timezone);
    return tv;
}
#[inline]
unsafe extern "C" fn mstrcpy(mut src: *const i8) -> *mut i8 {
    let mut str: *mut i8 = allocchk_(
        (*::core::mem::transmute::<&[u8; 8], &[i8; 8]>(b"mstrcpy\0")).as_ptr(),
        malloc(
            (strlen(src))
                .wrapping_add(1 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
        ),
    ) as *mut i8;
    strcpy(str, src);
    return str;
}
#[inline]
unsafe extern "C" fn mstrjoin(mut src1: *const i8, mut src2: *const i8) -> *mut i8 {
    let mut str: *mut i8 = allocchk_(
        (*::core::mem::transmute::<&[u8; 9], &[i8; 9]>(b"mstrjoin\0")).as_ptr(),
        malloc(
            (strlen(src1))
                .wrapping_add(strlen(src2))
                .wrapping_add(1 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
        ),
    ) as *mut i8;
    strcpy(str, src1);
    strcat(str, src2);
    return str;
}
#[inline]
unsafe extern "C" fn mstrjoin3(
    mut src1: *const i8,
    mut src2: *const i8,
    mut src3: *const i8,
) -> *mut i8 {
    let mut str: *mut i8 = allocchk_(
        (*::core::mem::transmute::<&[u8; 10], &[i8; 10]>(b"mstrjoin3\0")).as_ptr(),
        malloc(
            (strlen(src1))
                .wrapping_add(strlen(src2))
                .wrapping_add(strlen(src3))
                .wrapping_add(1 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
        ),
    ) as *mut i8;
    strcpy(str, src1);
    strcat(str, src2);
    strcat(str, src3);
    return str;
}
#[inline]
unsafe extern "C" fn normalize_i_range(mut i: i32, mut min: i32, mut max: i32) -> i32 {
    if i > max {
        return max;
    }
    if i < min {
        return min;
    }
    return i;
}
#[inline]
unsafe extern "C" fn max_i(mut a: i32, mut b: i32) -> i32 {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn min_i(mut a: i32, mut b: i32) -> i32 {
    return if a > b { b } else { a };
}
#[inline]
unsafe extern "C" fn max_l(mut a: i64, mut b: i64) -> i64 {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn normalize_d_range(
    mut d: libc::c_double,
    mut min: libc::c_double,
    mut max: libc::c_double,
) -> libc::c_double {
    if d > max {
        return max;
    }
    if d < min {
        return min;
    }
    return d;
}
#[inline]
unsafe extern "C" fn normalize_d(mut d: libc::c_double) -> libc::c_double {
    return normalize_d_range(d, 0.0f64, 1.0f64);
}
#[inline]
unsafe extern "C" fn parse_vsync(mut ps: *mut session_t, mut str: *const i8) -> bool {
    let mut i: vsync_t = vsync_t::VSYNC_NONE;
    while !(VSYNC_STRS[i as usize]).is_null() {
        if strcasecmp(str, VSYNC_STRS[i as usize]) == 0 {
            (*ps).o.vsync = i;
            return 1 as i32 != 0;
        }
        i += 1;
        i;
    }
    fprintf(
        stderr,
        b"%s(\"%s\"): Invalid vsync argument.\n\0" as *const u8 as *const i8,
        (*::core::mem::transmute::<&[u8; 12], &[i8; 12]>(b"parse_vsync\0")).as_ptr(),
        str,
    );
    return 0 as i32 != 0;
}
#[inline]
unsafe extern "C" fn parse_backend(mut ps: *mut session_t, mut str: *const i8) -> bool {
    let mut i: backend = backend::BKEND_XRENDER;
    while !(BACKEND_STRS[i as usize]).is_null() {
        if strcasecmp(str, BACKEND_STRS[i as usize]) == 0 {
            (*ps).o.backend = i;
            return 1 as i32 != 0;
        }
        i += 1;
        i;
    }
    if strcasecmp(str, b"xr_glx_hybird\0" as *const u8 as *const i8) == 0 {
        (*ps).o.backend = backend::BKEND_XR_GLX_HYBRID;
        return 1 as i32 != 0;
    }
    if strcasecmp(str, b"xr-glx-hybrid\0" as *const u8 as *const i8) == 0 {
        (*ps).o.backend = backend::BKEND_XR_GLX_HYBRID;
        return 1 as i32 != 0;
    }
    fprintf(
        stderr,
        b"%s(\"%s\"): Invalid backend argument.\n\0" as *const u8 as *const i8,
        (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"parse_backend\0")).as_ptr(),
        str,
    );
    return 0 as i32 != 0;
}
#[inline]
unsafe extern "C" fn parse_glx_swap_method(
    mut ps: *mut session_t,
    mut str: *const i8,
) -> bool {
    if strcmp(b"undefined\0" as *const u8 as *const i8, str) == 0 {
        (*ps).o.glx_swap_method = 0 as i32;
        return 1 as i32 != 0;
    }
    if strcmp(b"copy\0" as *const u8 as *const i8, str) == 0 {
        (*ps).o.glx_swap_method = 1 as i32;
        return 1 as i32 != 0;
    }
    if strcmp(b"exchange\0" as *const u8 as *const i8, str) == 0 {
        (*ps).o.glx_swap_method = 2 as i32;
        return 1 as i32 != 0;
    }
    if strcmp(b"buffer-age\0" as *const u8 as *const i8, str) == 0 {
        (*ps).o.glx_swap_method = -(1 as i32);
        return 1 as i32 != 0;
    }
    let mut pc: *mut i8 = 0 as *mut i8;
    let mut age: i32 = strtol(str, &mut pc, 0 as i32) as i32;
    if pc.is_null() || str == pc {
        fprintf(
            stderr,
            b"%s(\"%s\"): Invalid number.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"parse_glx_swap_method\0"))
                .as_ptr(),
            str,
        );
        return 0 as i32 != 0;
    }
    while *pc != 0 {
        if *(*__ctype_b_loc()).offset(*pc as i32 as isize) as i32
            & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 == 0
        {
            fprintf(
                stderr,
                b"%s(\"%s\"): Trailing characters.\n\0" as *const u8 as *const i8,
                (*::core::mem::transmute::<
                    &[u8; 22],
                    &[i8; 22],
                >(b"parse_glx_swap_method\0"))
                    .as_ptr(),
                str,
            );
            return 0 as i32 != 0;
        }
        pc = pc.offset(1);
        pc;
    }
    if age > 5 as i32 + 1 as i32 || age < -(1 as i32) {
        fprintf(
            stderr,
            b"%s(\"%s\"): Number too large / too small.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"parse_glx_swap_method\0"))
                .as_ptr(),
            str,
        );
        return 0 as i32 != 0;
    }
    (*ps).o.glx_swap_method = age;
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn get_time_ms() -> time_ms_t {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut timezone);
    return tv.tv_sec % (15 as i64 * 24 as i64 * 60 as i64 * 60 as i64)
        * 1000 as i32 as i64 + tv.tv_usec / 1000 as i32 as i64;
}
#[inline]
unsafe extern "C" fn fds_insert_select(
    mut ppfds: *mut *mut fd_set,
    mut fd: i32,
) -> bool {
    if (*ppfds).is_null() {
        *ppfds = malloc(::core::mem::size_of::<fd_set>() as u64) as *mut fd_set;
        if !(*ppfds).is_null() {
            let mut __d0: i32 = 0;
            let mut __d1: i32 = 0;
            let fresh0 = &mut __d0;
            let fresh1;
            let fresh2 = (::core::mem::size_of::<fd_set>() as u64)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
            let fresh3 = &mut __d1;
            let fresh4;
            let fresh5 = &mut *((**ppfds).fds_bits)
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) =>
                fresh4, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        } else {
            fprintf(
                stderr,
                b"Failed to allocate memory for select() fdset.\n\0" as *const u8
                    as *const i8,
            );
            exit(1 as i32);
        }
    }
    (**ppfds)
        .fds_bits[(fd / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
        as usize]
        |= ((1 as u64)
            << fd % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
            as __fd_mask;
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn fds_insert(
    mut ps: *mut session_t,
    mut fd: i32,
    mut events: libc::c_short,
) -> bool {
    let mut result: bool = 1 as i32 != 0;
    (*ps).nfds_max = max_i(fd + 1 as i32, (*ps).nfds_max);
    if 0x1 as i32 & events as i32 != 0 {
        result = fds_insert_select(&mut (*ps).pfds_read, fd) as i32 != 0
            && result as i32 != 0;
    }
    if 0x4 as i32 & events as i32 != 0 {
        result = fds_insert_select(&mut (*ps).pfds_write, fd) as i32 != 0
            && result as i32 != 0;
    }
    if 0x2 as i32 & events as i32 != 0 {
        result = fds_insert_select(&mut (*ps).pfds_except, fd) as i32 != 0
            && result as i32 != 0;
    }
    return result;
}
#[inline]
unsafe extern "C" fn fds_poll(mut ps: *mut session_t, mut ptv: *mut timeval) -> i32 {
    let mut pfds_read: *mut fd_set = 0 as *mut fd_set;
    if !((*ps).pfds_read).is_null() {
        pfds_read = malloc(::core::mem::size_of::<fd_set>() as u64) as *mut fd_set;
        memcpy(
            pfds_read as *mut libc::c_void,
            (*ps).pfds_read as *const libc::c_void,
            ::core::mem::size_of::<fd_set>() as u64,
        );
        if pfds_read.is_null() {
            fprintf(
                stderr,
                b"Failed to allocate memory for copying select() fdset.\n\0" as *const u8
                    as *const i8,
            );
            exit(1 as i32);
        }
    }
    let mut pfds_write: *mut fd_set = 0 as *mut fd_set;
    if !((*ps).pfds_write).is_null() {
        pfds_write = malloc(::core::mem::size_of::<fd_set>() as u64) as *mut fd_set;
        memcpy(
            pfds_write as *mut libc::c_void,
            (*ps).pfds_write as *const libc::c_void,
            ::core::mem::size_of::<fd_set>() as u64,
        );
        if pfds_write.is_null() {
            fprintf(
                stderr,
                b"Failed to allocate memory for copying select() fdset.\n\0" as *const u8
                    as *const i8,
            );
            exit(1 as i32);
        }
    }
    let mut pfds_except: *mut fd_set = 0 as *mut fd_set;
    if !((*ps).pfds_except).is_null() {
        pfds_except = malloc(::core::mem::size_of::<fd_set>() as u64) as *mut fd_set;
        memcpy(
            pfds_except as *mut libc::c_void,
            (*ps).pfds_except as *const libc::c_void,
            ::core::mem::size_of::<fd_set>() as u64,
        );
        if pfds_except.is_null() {
            fprintf(
                stderr,
                b"Failed to allocate memory for copying select() fdset.\n\0" as *const u8
                    as *const i8,
            );
            exit(1 as i32);
        }
    }
    let mut ret: i32 = select((*ps).nfds_max, pfds_read, pfds_write, pfds_except, ptv);
    free(pfds_read as *mut libc::c_void);
    free(pfds_write as *mut libc::c_void);
    free(pfds_except as *mut libc::c_void);
    return ret;
}
#[inline]
unsafe extern "C" fn cxfree(mut data: *mut libc::c_void) {
    if !data.is_null() {
        XFree(data);
    }
}
#[inline]
unsafe extern "C" fn get_atom(mut ps: *mut session_t, mut atom_name: *const i8) -> Atom {
    return XInternAtom((*ps).dpy, atom_name, 0 as i32);
}
#[inline]
unsafe extern "C" fn get_tgt_window(mut ps: *mut session_t) -> Window {
    return if (*ps).o.paint_on_overlay as i32 != 0 { (*ps).overlay } else { (*ps).root };
}
#[inline]
unsafe extern "C" fn find_win(mut ps: *mut session_t, mut id: Window) -> *mut win {
    if id == 0 {
        return 0 as *mut win;
    }
    let mut w: *mut win = 0 as *mut win;
    w = (*ps).list;
    while !w.is_null() {
        if (*w).id == id && !(*w).destroyed {
            return w;
        }
        w = (*w).next;
    }
    return 0 as *mut win;
}
#[inline]
unsafe extern "C" fn find_toplevel(mut ps: *mut session_t, mut id: Window) -> *mut win {
    if id == 0 {
        return 0 as *mut win;
    }
    let mut w: *mut win = (*ps).list;
    while !w.is_null() {
        if (*w).client_win == id && !(*w).destroyed {
            return w;
        }
        w = (*w).next;
    }
    return 0 as *mut win;
}
#[inline]
unsafe extern "C" fn bkend_use_xrender(mut ps: *mut session_t) -> bool {
    return backend::BKEND_XRENDER as i32 as u32 == (*ps).o.backend as u32
        || backend::BKEND_XR_GLX_HYBRID as i32 as u32 == (*ps).o.backend as u32;
}
#[inline]
unsafe extern "C" fn bkend_use_glx(mut ps: *mut session_t) -> bool {
    return backend::BKEND_GLX as i32 as u32 == (*ps).o.backend as u32
        || backend::BKEND_XR_GLX_HYBRID as i32 as u32 == (*ps).o.backend as u32;
}
#[inline]
unsafe extern "C" fn glx_has_context(mut ps: *mut session_t) -> bool {
    return !((*ps).psglx).is_null() && !((*(*ps).psglx).context).is_null();
}
#[inline]
unsafe extern "C" fn win_is_focused_real(
    mut ps: *mut session_t,
    mut w: *const win,
) -> bool {
    return 2 as i32 == (*w).a.map_state && (*ps).active_win == w as *mut _win;
}
#[inline]
unsafe extern "C" fn copy_region(
    mut ps: *const session_t,
    mut oldregion: XserverRegion,
) -> XserverRegion {
    if oldregion == 0 {
        return 0 as i64 as XserverRegion;
    }
    let mut region: XserverRegion = XFixesCreateRegion(
        (*ps).dpy,
        0 as *mut XRectangle,
        0 as i32,
    );
    XFixesCopyRegion((*ps).dpy, region, oldregion);
    return region;
}
#[inline]
unsafe extern "C" fn free_region(mut ps: *mut session_t, mut p: *mut XserverRegion) {
    if *p != 0 {
        XFixesDestroyRegion((*ps).dpy, *p);
        *p = 0 as i64 as XserverRegion;
    }
}
#[inline]
unsafe extern "C" fn free_all_damage_last(mut ps: *mut session_t) {
    let mut i: i32 = 0 as i32;
    while i < 5 as i32 {
        free_region(ps, &mut *((*ps).all_damage_last).as_mut_ptr().offset(i as isize));
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn free_fence(mut ps: *mut session_t, mut pfence: *mut XSyncFence) {
    if *pfence != 0 {
        XSyncDestroyFence((*ps).dpy, *pfence);
    }
    *pfence = 0 as i64 as XSyncFence;
}
#[inline]
unsafe extern "C" fn rect_crop(
    mut pdst: *mut XRectangle,
    mut psrc: *const XRectangle,
    mut pbound: *const XRectangle,
) {
    (*pdst).x = max_i((*psrc).x as i32, (*pbound).x as i32) as libc::c_short;
    (*pdst).y = max_i((*psrc).y as i32, (*pbound).y as i32) as libc::c_short;
    (*pdst).width = max_i(
        0 as i32,
        min_i(
            (*psrc).x as i32 + (*psrc).width as i32,
            (*pbound).x as i32 + (*pbound).width as i32,
        ) - (*pdst).x as i32,
    ) as libc::c_ushort;
    (*pdst).height = max_i(
        0 as i32,
        min_i(
            (*psrc).y as i32 + (*psrc).height as i32,
            (*pbound).y as i32 + (*pbound).height as i32,
        ) - (*pdst).y as i32,
    ) as libc::c_ushort;
}
#[inline]
unsafe extern "C" fn rect_is_fullscreen(
    mut ps: *mut session_t,
    mut x: i32,
    mut y: i32,
    mut wid: u32,
    mut hei: u32,
) -> bool {
    return x <= 0 as i32 && y <= 0 as i32
        && (x as u32).wrapping_add(wid) >= (*ps).root_width as u32
        && (y as u32).wrapping_add(hei) >= (*ps).root_height as u32;
}
#[inline]
unsafe extern "C" fn win_is_fullscreen(
    mut ps: *mut session_t,
    mut w: *const win,
) -> bool {
    return rect_is_fullscreen(
        ps,
        (*w).a.x,
        (*w).a.y,
        (*w).widthb as u32,
        (*w).heightb as u32,
    ) as i32 != 0 && (!(*w).bounding_shaped || (*w).rounded_corners as i32 != 0);
}
#[inline]
unsafe extern "C" fn win_is_solid(mut ps: *mut session_t, mut w: *const win) -> bool {
    return winmode_t::WMODE_SOLID as i32 as u32 == (*w).mode as u32
        && !(*ps).o.force_win_blend;
}
#[inline]
unsafe extern "C" fn wid_has_prop(
    mut ps: *const session_t,
    mut w: Window,
    mut atom: Atom,
) -> bool {
    let mut type_0: Atom = 0 as i64 as Atom;
    let mut format: i32 = 0;
    let mut nitems: u64 = 0;
    let mut after: u64 = 0;
    let mut data: *mut u8 = 0 as *mut u8;
    if 0 as i32
        == XGetWindowProperty(
            (*ps).dpy,
            w,
            atom,
            0 as i32 as i64,
            0 as i32 as i64,
            0 as i32,
            0 as i64 as Atom,
            &mut type_0,
            &mut format,
            &mut nitems,
            &mut after,
            &mut data,
        )
    {
        cxfree(data as *mut libc::c_void);
        if type_0 != 0 {
            return 1 as i32 != 0;
        }
    }
    return 0 as i32 != 0;
}
#[inline]
unsafe extern "C" fn wid_get_prop(
    mut ps: *const session_t,
    mut wid: Window,
    mut atom: Atom,
    mut length: i64,
    mut rtype: Atom,
    mut rformat: i32,
) -> winprop_t {
    return wid_get_prop_adv(ps, wid, atom, 0 as i64, length, rtype, rformat);
}
#[inline]
unsafe extern "C" fn free_winprop(mut pprop: *mut winprop_t) {
    if !((*pprop).data.p8).is_null() {
        cxfree((*pprop).data.p8 as *mut libc::c_void);
        (*pprop).data.p8 = 0 as *mut u8;
    }
    (*pprop).nitems = 0 as i32 as u64;
}
#[inline]
unsafe extern "C" fn ensure_glx_context(mut ps: *mut session_t) -> bool {
    if !glx_has_context(ps) {
        glx_init(ps, 0 as i32 != 0);
    }
    return !((*(*ps).psglx).context).is_null();
}
#[inline]
unsafe extern "C" fn glx_tex_binded(
    mut ptex: *const glx_texture_t,
    mut pixmap: Pixmap,
) -> bool {
    return !ptex.is_null() && (*ptex).glpixmap != 0 && (*ptex).texture != 0
        && (pixmap == 0 || pixmap == (*ptex).pixmap);
}
#[inline]
unsafe extern "C" fn free_texture_r(mut ps: *mut session_t, mut ptexture: *mut GLuint) {
    if *ptexture != 0 {
        glDeleteTextures(1 as i32, ptexture);
        *ptexture = 0 as i32 as GLuint;
    }
}
#[inline]
unsafe extern "C" fn free_glx_fbo(mut ps: *mut session_t, mut pfbo: *mut GLuint) {
    if *pfbo != 0 {
        glDeleteFramebuffers(1 as i32, pfbo);
        *pfbo = 0 as i32 as GLuint;
    }
}
#[inline]
unsafe extern "C" fn free_glx_bc_resize(
    mut ps: *mut session_t,
    mut pbc: *mut glx_blur_cache_t,
) {
    free_texture_r(ps, &mut *((*pbc).textures).as_mut_ptr().offset(0 as i32 as isize));
    free_texture_r(ps, &mut *((*pbc).textures).as_mut_ptr().offset(1 as i32 as isize));
    (*pbc).width = 0 as i32;
    (*pbc).height = 0 as i32;
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
unsafe extern "C" fn free_win_res_glx(mut ps: *mut session_t, mut w: *mut win) {
    free_paint_glx(ps, &mut (*w).paint);
    free_paint_glx(ps, &mut (*w).shadow_paint);
    free_glx_bc(ps, &mut (*w).glx_blur_cache);
}
#[inline]
unsafe extern "C" fn glx_mark_(
    mut ps: *mut session_t,
    mut func: *const i8,
    mut xid: XID,
    mut start: bool,
) {}
#[inline]
unsafe extern "C" fn glx_mark_frame(mut ps: *mut session_t) {}
#[inline]
unsafe extern "C" fn xr_sync_(
    mut ps: *mut session_t,
    mut d: Drawable,
    mut pfence: *mut XSyncFence,
) {
    if !(*ps).o.xrender_sync {
        return;
    }
    XSync((*ps).dpy, 0 as i32);
    if (*ps).o.xrender_sync_fence as i32 != 0 && (*ps).xsync_exists as i32 != 0 {
        let mut tmp_fence: XSyncFence = 0 as i64 as XSyncFence;
        if pfence.is_null() {
            pfence = &mut tmp_fence;
        }
        if *pfence == 0 {
            *pfence = XSyncCreateFence((*ps).dpy, d, 0 as i32);
        }
        if *pfence != 0 {
            let mut triggered: i32 = 0 as i32;
            XSyncTriggerFence((*ps).dpy, *pfence);
            XSyncAwaitFence((*ps).dpy, pfence, 1 as i32);
        } else {
            fprintf(
                stderr,
                b"%s(%#010lx): Failed to create X Sync fence.\n\0" as *const u8
                    as *const i8,
                (*::core::mem::transmute::<&[u8; 9], &[i8; 9]>(b"xr_sync_\0")).as_ptr(),
                d,
            );
        }
        free_fence(ps, &mut tmp_fence);
        if *pfence != 0 {
            XSyncResetFence((*ps).dpy, *pfence);
        }
    }
}
#[inline]
unsafe extern "C" fn add_damage_win(mut ps: *mut session_t, mut w: *mut win) {
    if (*w).extents != 0 {
        add_damage(ps, copy_region(ps, (*w).extents));
    }
}
#[inline]
unsafe extern "C" fn win_match(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut condlst: *mut c2_lptr_t,
    mut cache: *mut *const c2_lptr_t,
) -> bool {
    return c2_matchd(ps, w, condlst, cache, 0 as *mut *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn win_get_leader(mut ps: *mut session_t, mut w: *mut win) -> Window {
    return win_get_leader_raw(ps, w, 0 as i32);
}
#[inline]
unsafe extern "C" fn find_win_all(mut ps: *mut session_t, wid: Window) -> *mut win {
    if wid == 0 || 1 as i64 as u64 == wid || wid == (*ps).root || wid == (*ps).overlay {
        return 0 as *mut win;
    }
    let mut w: *mut win = find_win(ps, wid);
    if w.is_null() {
        w = find_toplevel(ps, wid);
    }
    if w.is_null() {
        w = find_toplevel2(ps, wid);
    }
    return w;
}
#[inline]
unsafe extern "C" fn wid_rm_opacity_prop(mut ps: *mut session_t, mut wid: Window) {
    XDeleteProperty((*ps).dpy, wid, (*ps).atom_opacity);
}
#[inline]
unsafe extern "C" fn wid_set_opacity_prop(
    mut ps: *mut session_t,
    mut wid: Window,
    mut val: opacity_t,
) {
    let v: u64 = val as u64;
    XChangeProperty(
        (*ps).dpy,
        wid,
        (*ps).atom_opacity,
        6 as i32 as Atom,
        32 as i32,
        0 as i32,
        &v as *const u64 as *mut u8,
        1 as i32,
    );
}
#[inline]
unsafe extern "C" fn group_update_focused(mut ps: *mut session_t, mut leader: Window) {
    if leader == 0 {
        return;
    }
    let mut w: *mut win = (*ps).list;
    while !w.is_null() {
        if win_get_leader(ps, w) == leader && !(*w).destroyed {
            win_update_focused(ps, w);
        }
        w = (*w).next;
    }
}
#[inline]
unsafe extern "C" fn group_is_focused(
    mut ps: *mut session_t,
    mut leader: Window,
) -> bool {
    if leader == 0 {
        return 0 as i32 != 0;
    }
    let mut w: *mut win = (*ps).list;
    while !w.is_null() {
        if win_get_leader(ps, w) == leader && !(*w).destroyed
            && win_is_focused_real(ps, w) as i32 != 0
        {
            return 1 as i32 != 0;
        }
        w = (*w).next;
    }
    return 0 as i32 != 0;
}
#[inline]
unsafe extern "C" fn timeout_get_newrun(mut ptmout: *const timeout_t) -> time_ms_t {
    return (*ptmout).firstrun
        + (max_l(
            ((*ptmout).lastrun
                + ((*ptmout).interval as libc::c_double * 0.05f64) as time_ms_t
                - (*ptmout).firstrun) / (*ptmout).interval,
            ((*ptmout).lastrun
                + ((*ptmout).interval as libc::c_double
                    * (1 as i32 as libc::c_double - 0.05f64)) as time_ms_t
                - (*ptmout).firstrun) / (*ptmout).interval,
        ) + 1 as i32 as i64) * (*ptmout).interval;
}
#[inline]
unsafe extern "C" fn set_ignore_next(mut ps: *mut session_t) {
    set_ignore(
        ps,
        ((*((*ps).dpy as _XPrivDisplay)).request).wrapping_add(1 as i32 as u64),
    );
}
#[inline]
unsafe extern "C" fn xrfilter_reset(mut ps: *mut session_t, mut p: Picture) {
    XRenderSetPictureFilter(
        (*ps).dpy,
        p,
        b"Nearest\0" as *const u8 as *const i8,
        0 as *mut XFixed,
        0 as i32,
    );
}
#[inline]
unsafe extern "C" fn wintype_arr_enable(mut arr: *mut bool) {
    let mut i: wintype_t = wintype_t::WINTYPE_UNKNOWN;
    i = wintype_t::WINTYPE_UNKNOWN;
    while (i as u32) < wintype_t::NUM_WINTYPES as i32 as u32 {
        *arr.offset(i as isize) = 1 as i32 != 0;
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn geom_to_rect(
    mut ps: *mut session_t,
    mut src: *const geometry_t,
    mut def: *const XRectangle,
) -> XRectangle {
    let mut rect_def: XRectangle = {
        let mut init = XRectangle {
            x: 0 as i32 as libc::c_short,
            y: 0 as i32 as libc::c_short,
            width: (*ps).root_width as libc::c_ushort,
            height: (*ps).root_height as libc::c_ushort,
        };
        init
    };
    if def.is_null() {
        def = &mut rect_def;
    }
    let mut rect: XRectangle = {
        let mut init = XRectangle {
            x: (*src).x as libc::c_short,
            y: (*src).y as libc::c_short,
            width: (*src).wid as libc::c_ushort,
            height: (*src).hei as libc::c_ushort,
        };
        init
    };
    if (*src).wid < 0 as i32 {
        rect.width = (*def).width;
    }
    if (*src).hei < 0 as i32 {
        rect.height = (*def).height;
    }
    if -(1 as i32) == (*src).x {
        rect.x = (*def).x;
    } else if (*src).x < 0 as i32 {
        rect.x = ((*ps).root_width + rect.x as i32 + 2 as i32 - rect.width as i32)
            as libc::c_short;
    }
    if -(1 as i32) == (*src).y {
        rect.y = (*def).y;
    } else if (*src).y < 0 as i32 {
        rect.y = ((*ps).root_height + rect.y as i32 + 2 as i32 - rect.height as i32)
            as libc::c_short;
    }
    return rect;
}
#[inline]
unsafe extern "C" fn rect_to_reg(
    mut ps: *mut session_t,
    mut src: *const XRectangle,
) -> XserverRegion {
    if src.is_null() {
        return 0 as i64 as XserverRegion;
    }
    let mut bound: XRectangle = {
        let mut init = XRectangle {
            x: 0 as i32 as libc::c_short,
            y: 0 as i32 as libc::c_short,
            width: (*ps).root_width as libc::c_ushort,
            height: (*ps).root_height as libc::c_ushort,
        };
        init
    };
    let mut res: XRectangle = {
        let mut init = XRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        init
    };
    rect_crop(&mut res, src, &mut bound);
    if res.width as i32 != 0 && res.height as i32 != 0 {
        return XFixesCreateRegion((*ps).dpy, &mut res, 1 as i32);
    }
    return 0 as i64 as XserverRegion;
}
#[inline]
unsafe extern "C" fn free_picture(mut ps: *mut session_t, mut p: *mut Picture) {
    if *p != 0 {
        XRenderFreePicture((*ps).dpy, *p);
        *p = 0 as i64 as Picture;
    }
}
#[inline]
unsafe extern "C" fn free_pixmap(mut ps: *mut session_t, mut p: *mut Pixmap) {
    if *p != 0 {
        XFreePixmap((*ps).dpy, *p);
        *p = 0 as i64 as Pixmap;
    }
}
#[inline]
unsafe extern "C" fn free_damage(mut ps: *mut session_t, mut p: *mut Damage) {
    if *p != 0 {
        set_ignore_next(ps);
        XDamageDestroy((*ps).dpy, *p);
        *p = 0 as i64 as Damage;
    }
}
#[inline]
unsafe extern "C" fn free_wincondlst(mut pcondlst: *mut *mut c2_lptr_t) {
    loop {
        *pcondlst = c2_free_lptr(*pcondlst);
        if (*pcondlst).is_null() {
            break;
        }
    };
}
#[inline]
unsafe extern "C" fn free_xinerama_info(mut ps: *mut session_t) {
    if !((*ps).xinerama_scr_regs).is_null() {
        let mut i: i32 = 0 as i32;
        while i < (*ps).xinerama_nscrs {
            free_region(ps, &mut *((*ps).xinerama_scr_regs).offset(i as isize));
            i += 1;
            i;
        }
        free((*ps).xinerama_scr_regs as *mut libc::c_void);
    }
    cxfree((*ps).xinerama_scrs as *mut libc::c_void);
    (*ps).xinerama_scrs = 0 as *mut XineramaScreenInfo;
    (*ps).xinerama_nscrs = 0 as i32;
}
#[inline]
unsafe extern "C" fn paint_isvalid(
    mut ps: *mut session_t,
    mut ppaint: *const paint_t,
) -> bool {
    if ppaint.is_null() {
        return 0 as i32 != 0;
    }
    if bkend_use_xrender(ps) as i32 != 0 && (*ppaint).pict == 0 {
        return 0 as i32 != 0;
    }
    if backend::BKEND_GLX as i32 as u32 == (*ps).o.backend as u32
        && !glx_tex_binded((*ppaint).ptex, 0 as i64 as Pixmap)
    {
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn paint_bind_tex_real(
    mut ps: *mut session_t,
    mut ppaint: *mut paint_t,
    mut wid: u32,
    mut hei: u32,
    mut depth: u32,
    mut force: bool,
) -> bool {
    if (*ppaint).pixmap == 0 {
        return 0 as i32 != 0;
    }
    if force as i32 != 0 || !glx_tex_binded((*ppaint).ptex, (*ppaint).pixmap) {
        return glx_bind_pixmap(
            ps,
            &mut (*ppaint).ptex,
            (*ppaint).pixmap,
            wid,
            hei,
            depth,
        );
    }
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn paint_bind_tex(
    mut ps: *mut session_t,
    mut ppaint: *mut paint_t,
    mut wid: u32,
    mut hei: u32,
    mut depth: u32,
    mut force: bool,
) -> bool {
    if backend::BKEND_GLX as i32 as u32 == (*ps).o.backend as u32 {
        return paint_bind_tex_real(ps, ppaint, wid, hei, depth, force);
    }
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn free_reg_data(mut pregd: *mut reg_data_t) {
    cxfree((*pregd).rects as *mut libc::c_void);
    (*pregd).rects = 0 as *mut XRectangle;
    (*pregd).nrects = 0 as i32;
}
#[inline]
unsafe extern "C" fn free_paint(mut ps: *mut session_t, mut ppaint: *mut paint_t) {
    free_paint_glx(ps, ppaint);
    free_picture(ps, &mut (*ppaint).pict);
    free_pixmap(ps, &mut (*ppaint).pixmap);
}
#[inline]
unsafe extern "C" fn free_wpaint(mut ps: *mut session_t, mut w: *mut win) {
    free_paint(ps, &mut (*w).paint);
    free_fence(ps, &mut (*w).fence);
}
#[inline]
unsafe extern "C" fn free_win_res(mut ps: *mut session_t, mut w: *mut win) {
    free_win_res_glx(ps, w);
    free_region(ps, &mut (*w).extents);
    free_paint(ps, &mut (*w).paint);
    free_region(ps, &mut (*w).border_size);
    free_paint(ps, &mut (*w).shadow_paint);
    free_damage(ps, &mut (*w).damage);
    free_region(ps, &mut (*w).reg_ignore);
    free((*w).name as *mut libc::c_void);
    free((*w).class_instance as *mut libc::c_void);
    free((*w).class_general as *mut libc::c_void);
    free((*w).role as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn free_root_tile(mut ps: *mut session_t) {
    free_picture(ps, &mut (*ps).root_tile_paint.pict);
    free_texture(ps, &mut (*ps).root_tile_paint.ptex);
    if (*ps).root_tile_fill {
        free_pixmap(ps, &mut (*ps).root_tile_paint.pixmap);
    }
    (*ps).root_tile_paint.pixmap = 0 as i64 as Pixmap;
    (*ps).root_tile_fill = 0 as i32 != 0;
}
#[inline]
unsafe extern "C" fn ms_to_tv(mut timeout: i32) -> timeval {
    return {
        let mut init = timeval {
            tv_sec: (timeout / 1000 as i32) as __time_t,
            tv_usec: (timeout % 1000 as i32) as i64
                * (1000000 as i64 / 1000 as i32 as i64),
        };
        init
    };
}
#[inline]
unsafe extern "C" fn isdamagenotify(
    mut ps: *mut session_t,
    mut ev: *const XEvent,
) -> bool {
    return (*ps).damage_event + 0 as i32 == (*ev).type_0;
}
#[inline]
unsafe extern "C" fn make_text_prop(
    mut ps: *mut session_t,
    mut str: *mut i8,
) -> *mut XTextProperty {
    let mut pprop: *mut XTextProperty = allocchk_(
        (*::core::mem::transmute::<&[u8; 15], &[i8; 15]>(b"make_text_prop\0")).as_ptr(),
        calloc(1 as i32 as u64, ::core::mem::size_of::<XTextProperty>() as u64),
    ) as *mut XTextProperty;
    if XmbTextListToTextProperty(
        (*ps).dpy,
        &mut str,
        1 as i32,
        XICCEncodingStyle::XStringStyle,
        pprop,
    ) != 0
    {
        cxfree((*pprop).value as *mut libc::c_void);
        free(pprop as *mut libc::c_void);
        pprop = 0 as *mut XTextProperty;
    }
    return pprop;
}
#[inline]
unsafe extern "C" fn wid_set_text_prop(
    mut ps: *mut session_t,
    mut wid: Window,
    mut prop_atom: Atom,
    mut str: *mut i8,
) -> bool {
    let mut pprop: *mut XTextProperty = make_text_prop(ps, str);
    if pprop.is_null() {
        fprintf(
            stderr,
            b"%s(\"%s\"): Failed to make text property.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 18], &[i8; 18]>(b"wid_set_text_prop\0"))
                .as_ptr(),
            str,
        );
        return 0 as i32 != 0;
    }
    XSetTextProperty((*ps).dpy, wid, pprop, prop_atom);
    cxfree((*pprop).value as *mut libc::c_void);
    cxfree(pprop as *mut libc::c_void);
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn check_fade_fin(mut ps: *mut session_t, mut w: *mut win) {
    if ((*w).fade_callback).is_some() && (*w).opacity == (*w).opacity_tgt {
        set_fade_callback(ps, w, None, 1 as i32 != 0);
    }
}
#[inline]
unsafe extern "C" fn win_ev_stop(mut ps: *mut session_t, mut w: *mut win) {
    set_ignore_next(ps);
    XSelectInput((*ps).dpy, (*w).id, 0 as i32 as i64);
    if (*w).client_win != 0 {
        set_ignore_next(ps);
        XSelectInput((*ps).dpy, (*w).client_win, 0 as i32 as i64);
    }
    if (*ps).shape_exists {
        set_ignore_next(ps);
        XShapeSelectInput((*ps).dpy, (*w).id, 0 as i32 as u64);
    }
}
#[inline]
unsafe extern "C" fn wid_get_children(
    mut ps: *mut session_t,
    mut w: Window,
    mut children: *mut *mut Window,
    mut nchildren: *mut u32,
) -> bool {
    let mut troot: Window = 0;
    let mut tparent: Window = 0;
    if XQueryTree((*ps).dpy, w, &mut troot, &mut tparent, children, nchildren) == 0 {
        *nchildren = 0 as i32 as u32;
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn wid_bounding_shaped(
    mut ps: *const session_t,
    mut wid: Window,
) -> bool {
    if (*ps).shape_exists {
        let mut bounding_shaped: i32 = 0 as i32;
        let mut clip_shaped: i32 = 0 as i32;
        let mut x_bounding: i32 = 0;
        let mut y_bounding: i32 = 0;
        let mut x_clip: i32 = 0;
        let mut y_clip: i32 = 0;
        let mut w_bounding: u32 = 0;
        let mut h_bounding: u32 = 0;
        let mut w_clip: u32 = 0;
        let mut h_clip: u32 = 0;
        XShapeQueryExtents(
            (*ps).dpy,
            wid,
            &mut bounding_shaped,
            &mut x_bounding,
            &mut y_bounding,
            &mut w_bounding,
            &mut h_bounding,
            &mut clip_shaped,
            &mut x_clip,
            &mut y_clip,
            &mut w_clip,
            &mut h_clip,
        );
        return bounding_shaped != 0;
    }
    return 0 as i32 != 0;
}
#[inline]
unsafe extern "C" fn update_reg_ignore_expire(
    mut ps: *mut session_t,
    mut w: *const win,
) {
    if (*w).to_paint as i32 != 0
        && winmode_t::WMODE_SOLID as i32 as u32 == (*w).mode as u32
    {
        (*ps).reg_ignore_expire = 1 as i32 != 0;
    }
}
#[inline]
unsafe extern "C" fn win_has_frame(mut w: *const win) -> bool {
    return (*w).a.border_width != 0 || (*w).frame_extents.top != 0
        || (*w).frame_extents.left != 0 || (*w).frame_extents.right != 0
        || (*w).frame_extents.bottom != 0;
}
#[inline]
unsafe extern "C" fn win_calc_frame_extents(
    mut ps: *mut session_t,
    mut w: *const win,
) -> margin_t {
    let mut result: margin_t = (*w).frame_extents;
    result.top = max_i(result.top, (*w).a.border_width);
    result.left = max_i(result.left, (*w).a.border_width);
    result.bottom = max_i(result.bottom, (*w).a.border_width);
    result.right = max_i(result.right, (*w).a.border_width);
    return result;
}
#[inline]
unsafe extern "C" fn validate_pixmap(mut ps: *mut session_t, mut pxmap: Pixmap) -> bool {
    if pxmap == 0 {
        return 0 as i32 != 0;
    }
    let mut rroot: Window = 0 as i64 as Window;
    let mut rx: i32 = 0 as i32;
    let mut ry: i32 = 0 as i32;
    let mut rwid: u32 = 0 as i32 as u32;
    let mut rhei: u32 = 0 as i32 as u32;
    let mut rborder: u32 = 0 as i32 as u32;
    let mut rdepth: u32 = 0 as i32 as u32;
    return XGetGeometry(
        (*ps).dpy,
        pxmap,
        &mut rroot,
        &mut rx,
        &mut ry,
        &mut rwid,
        &mut rhei,
        &mut rborder,
        &mut rdepth,
    ) != 0 && rwid != 0 && rhei != 0;
}
#[inline]
unsafe extern "C" fn win_validate_pixmap(mut ps: *mut session_t, mut w: *mut win) {
    if !validate_pixmap(ps, (*w).paint.pixmap) {
        free_paint(ps, &mut (*w).paint);
    }
}
unsafe extern "C" fn clear_cache_win_leaders(mut ps: *mut session_t) {
    let mut w: *mut win = (*ps).list;
    while !w.is_null() {
        (*w).cache_leader = 0 as i64 as Window;
        w = (*w).next;
    }
}
#[inline]
unsafe extern "C" fn lcfg_lookup_int(
    mut config: *const config_t,
    mut path: *const i8,
    mut value: *mut i32,
) -> i32 {
    return config_lookup_int(config, path, value);
}
#[inline]
unsafe extern "C" fn lcfg_lookup_bool(
    mut config: *const config_t,
    mut path: *const i8,
    mut value: *mut bool,
) {
    let mut ival: i32 = 0;
    if config_lookup_bool(config, path, &mut ival) != 0 {
        *value = ival != 0;
    }
}
#[inline]
unsafe extern "C" fn win_render(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut x: i32,
    mut y: i32,
    mut wid: i32,
    mut hei: i32,
    mut opacity: libc::c_double,
    mut reg_paint: XserverRegion,
    mut pcache_reg: *const reg_data_t,
    mut pict: Picture,
) {
    let dx: i32 = (if !w.is_null() { (*w).a.x } else { 0 as i32 }) + x;
    let dy: i32 = (if !w.is_null() { (*w).a.y } else { 0 as i32 }) + y;
    let argb: bool = !w.is_null()
        && (winmode_t::WMODE_ARGB as i32 as u32 == (*w).mode as u32
            || (*ps).o.force_win_blend as i32 != 0);
    let neg: bool = !w.is_null() && (*w).invert_color as i32 != 0;
    render_(
        ps,
        x,
        y,
        dx,
        dy,
        wid,
        hei,
        opacity,
        argb,
        neg,
        pict,
        if !w.is_null() { (*w).paint.ptex } else { (*ps).root_tile_paint.ptex },
        reg_paint,
        pcache_reg,
        if !w.is_null() { &mut (*ps).o.glx_prog_win } else { 0 as *mut glx_prog_main_t },
    );
}
#[inline]
unsafe extern "C" fn set_tgt_clip(
    mut ps: *mut session_t,
    mut reg: XserverRegion,
    mut pcache_reg: *const reg_data_t,
) {
    match (*ps).o.backend as u32 {
        0 | 2 => {
            XFixesSetPictureClipRegion(
                (*ps).dpy,
                (*ps).tgt_buffer.pict,
                0 as i32,
                0 as i32,
                reg,
            );
        }
        1 => {
            glx_set_clip(ps, reg, pcache_reg);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn resize_region(
    mut ps: *mut session_t,
    mut region: XserverRegion,
    mut mod_0: libc::c_short,
) {
    if mod_0 == 0 || region == 0 {
        return;
    }
    let mut nrects: i32 = 0 as i32;
    let mut nnewrects: i32 = 0 as i32;
    let mut newrects: *mut XRectangle = 0 as *mut XRectangle;
    let mut rects: *mut XRectangle = XFixesFetchRegion((*ps).dpy, region, &mut nrects);
    if !(rects.is_null() || nrects == 0) {
        newrects = calloc(nrects as u64, ::core::mem::size_of::<XRectangle>() as u64)
            as *mut XRectangle;
        if newrects.is_null() {
            fprintf(
                stderr,
                b"%s(): Failed to allocate memory.\n\0" as *const u8 as *const i8,
                (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"resize_region\0"))
                    .as_ptr(),
            );
            exit(1 as i32);
        }
        let mut i: i32 = 0 as i32;
        while i < nrects {
            let mut x1: i32 = max_i(
                (*rects.offset(i as isize)).x as i32 - mod_0 as i32,
                0 as i32,
            );
            let mut y1: i32 = max_i(
                (*rects.offset(i as isize)).y as i32 - mod_0 as i32,
                0 as i32,
            );
            let mut x2: i32 = min_i(
                (*rects.offset(i as isize)).x as i32
                    + (*rects.offset(i as isize)).width as i32 + mod_0 as i32,
                (*ps).root_width,
            );
            let mut y2: i32 = min_i(
                (*rects.offset(i as isize)).y as i32
                    + (*rects.offset(i as isize)).height as i32 + mod_0 as i32,
                (*ps).root_height,
            );
            let mut wid: i32 = x2 - x1;
            let mut hei: i32 = y2 - y1;
            if !(wid <= 0 as i32 || hei <= 0 as i32) {
                (*newrects.offset(nnewrects as isize)).x = x1 as libc::c_short;
                (*newrects.offset(nnewrects as isize)).y = y1 as libc::c_short;
                (*newrects.offset(nnewrects as isize)).width = wid as libc::c_ushort;
                (*newrects.offset(nnewrects as isize)).height = hei as libc::c_ushort;
                nnewrects += 1;
                nnewrects;
            }
            i += 1;
            i;
        }
        XFixesSetRegion((*ps).dpy, region, newrects, nnewrects);
    }
    cxfree(rects as *mut libc::c_void);
    free(newrects as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn normalize_conv_kern(
    mut wid: i32,
    mut hei: i32,
    mut kern: *mut XFixed,
) {
    let mut sum: libc::c_double = 0.0f64;
    let mut i: i32 = 0 as i32;
    while i < wid * hei {
        sum += *kern.offset(i as isize) as XDouble / 65536 as i32 as libc::c_double;
        i += 1;
        i;
    }
    let mut factor: libc::c_double = 1.0f64 / sum;
    let mut i_0: i32 = 0 as i32;
    while i_0 < wid * hei {
        *kern.offset(i_0 as isize) = (*kern.offset(i_0 as isize) as XDouble
            / 65536 as i32 as libc::c_double * factor * 65536 as i32 as libc::c_double)
            as XFixed;
        i_0 += 1;
        i_0;
    }
}
#[inline]
unsafe extern "C" fn is_region_empty(
    mut ps: *const session_t,
    mut region: XserverRegion,
    mut pcache_reg: *mut reg_data_t,
) -> bool {
    let mut nrects: i32 = 0 as i32;
    let mut rects: *mut XRectangle = XFixesFetchRegion((*ps).dpy, region, &mut nrects);
    if !pcache_reg.is_null() {
        (*pcache_reg).rects = rects;
        (*pcache_reg).nrects = nrects;
    } else {
        cxfree(rects as *mut libc::c_void);
    }
    return nrects == 0;
}
#[inline]
unsafe extern "C" fn get_screen_region(mut ps: *mut session_t) -> XserverRegion {
    let mut r: XRectangle = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    r.x = 0 as i32 as libc::c_short;
    r.y = 0 as i32 as libc::c_short;
    r.width = (*ps).root_width as libc::c_ushort;
    r.height = (*ps).root_height as libc::c_ushort;
    return XFixesCreateRegion((*ps).dpy, &mut r, 1 as i32);
}
#[inline]
unsafe extern "C" fn win_get_name(mut ps: *mut session_t, mut w: *mut win) -> i32 {
    let mut ret: i32 = win_get_prop_str(
        ps,
        w,
        &mut (*w).name,
        Some(
            wid_get_name
                as unsafe extern "C" fn(*mut session_t, Window, *mut *mut i8) -> bool,
        ),
    );
    return ret;
}
#[inline]
unsafe extern "C" fn cxinerama_win_upd_scr(mut ps: *mut session_t, mut w: *mut win) {
    (*w).xinerama_scr = -(1 as i32);
    let mut s: *mut XineramaScreenInfo = (*ps).xinerama_scrs;
    while s < ((*ps).xinerama_scrs).offset((*ps).xinerama_nscrs as isize) {
        if (*s).x_org as i32 <= (*w).a.x && (*s).y_org as i32 <= (*w).a.y
            && (*s).x_org as i32 + (*s).width as i32 >= (*w).a.x + (*w).widthb
            && (*s).y_org as i32 + (*s).height as i32 >= (*w).a.y + (*w).heightb
        {
            (*w).xinerama_scr = s.offset_from((*ps).xinerama_scrs) as i64 as i32;
            return;
        }
        s = s.offset(1);
        s;
    }
}
#[inline]
unsafe extern "C" fn win_get_role(mut ps: *mut session_t, mut w: *mut win) -> i32 {
    let mut ret: i32 = win_get_prop_str(
        ps,
        w,
        &mut (*w).role,
        Some(
            wid_get_role
                as unsafe extern "C" fn(*mut session_t, Window, *mut *mut i8) -> bool,
        ),
    );
    return ret;
}
#[inline]
unsafe extern "C" fn win_update_opacity_prop(mut ps: *mut session_t, mut w: *mut win) {
    (*w).opacity_prop = wid_get_opacity_prop(ps, (*w).id, 0xffffffff as u32);
    if !(*ps).o.detect_client_opacity || (*w).client_win == 0
        || (*w).id == (*w).client_win
    {
        (*w).opacity_prop_client = 0xffffffff as u32;
    } else {
        (*w).opacity_prop_client = wid_get_opacity_prop(
            ps,
            (*w).client_win,
            0xffffffff as u32,
        );
    };
}
#[no_mangle]
pub static mut WINTYPES: [*const i8; 15] = [
    b"unknown\0" as *const u8 as *const i8,
    b"desktop\0" as *const u8 as *const i8,
    b"dock\0" as *const u8 as *const i8,
    b"toolbar\0" as *const u8 as *const i8,
    b"menu\0" as *const u8 as *const i8,
    b"utility\0" as *const u8 as *const i8,
    b"splash\0" as *const u8 as *const i8,
    b"dialog\0" as *const u8 as *const i8,
    b"normal\0" as *const u8 as *const i8,
    b"dropdown_menu\0" as *const u8 as *const i8,
    b"popup_menu\0" as *const u8 as *const i8,
    b"tooltip\0" as *const u8 as *const i8,
    b"notify\0" as *const u8 as *const i8,
    b"combo\0" as *const u8 as *const i8,
    b"dnd\0" as *const u8 as *const i8,
];
#[no_mangle]
pub static mut VSYNC_STRS: [*const i8; 7] = [
    b"none\0" as *const u8 as *const i8,
    b"drm\0" as *const u8 as *const i8,
    b"opengl\0" as *const u8 as *const i8,
    b"opengl-oml\0" as *const u8 as *const i8,
    b"opengl-swc\0" as *const u8 as *const i8,
    b"opengl-mswc\0" as *const u8 as *const i8,
    0 as *const i8,
];
#[no_mangle]
pub static mut BACKEND_STRS: [*const i8; 4] = [
    b"xrender\0" as *const u8 as *const i8,
    b"glx\0" as *const u8 as *const i8,
    b"xr_glx_hybrid\0" as *const u8 as *const i8,
    0 as *const i8,
];
static mut VSYNC_FUNCS_INIT: [Option<unsafe extern "C" fn(*mut session_t) -> bool>; 6] = unsafe {
    [
        None,
        Some(vsync_drm_init as unsafe extern "C" fn(*mut session_t) -> bool),
        Some(vsync_opengl_init as unsafe extern "C" fn(*mut session_t) -> bool),
        Some(vsync_opengl_oml_init as unsafe extern "C" fn(*mut session_t) -> bool),
        Some(vsync_opengl_swc_init as unsafe extern "C" fn(*mut session_t) -> bool),
        Some(vsync_opengl_mswc_init as unsafe extern "C" fn(*mut session_t) -> bool),
    ]
};
static mut VSYNC_FUNCS_WAIT: [Option<unsafe extern "C" fn(*mut session_t) -> i32>; 6] = unsafe {
    [
        None,
        Some(vsync_drm_wait as unsafe extern "C" fn(*mut session_t) -> i32),
        Some(vsync_opengl_wait as unsafe extern "C" fn(*mut session_t) -> i32),
        Some(vsync_opengl_oml_wait as unsafe extern "C" fn(*mut session_t) -> i32),
        None,
        None,
    ]
};
static mut VSYNC_FUNCS_DEINIT: [Option<unsafe extern "C" fn(*mut session_t) -> ()>; 6] = unsafe {
    [
        None,
        None,
        None,
        None,
        Some(vsync_opengl_swc_deinit as unsafe extern "C" fn(*mut session_t) -> ()),
        Some(vsync_opengl_mswc_deinit as unsafe extern "C" fn(*mut session_t) -> ()),
    ]
};
static mut background_props_str: [*const i8; 3] = [
    b"_XROOTPMAP_ID\0" as *const u8 as *const i8,
    b"_XSETROOT_ID\0" as *const u8 as *const i8,
    0 as *const i8,
];
#[no_mangle]
pub static mut ps_g: *mut session_t = 0 as *const session_t as *mut session_t;
unsafe extern "C" fn fade_timeout(mut ps: *mut session_t) -> i32 {
    let mut diff: i32 = ((*ps).o.fade_delta - get_time_ms() + (*ps).fade_time) as i32;
    diff = normalize_i_range(
        diff,
        0 as i32,
        ((*ps).o.fade_delta * 2 as i32 as i64) as i32,
    );
    return diff;
}
unsafe extern "C" fn run_fade(mut ps: *mut session_t, mut w: *mut win, mut steps: u32) {
    if (*w).opacity == (*w).opacity_tgt {
        return;
    }
    if !(*w).fade {
        (*w).opacity = (*w).opacity_tgt;
    } else if steps != 0 {
        if (*w).opacity < (*w).opacity_tgt {
            (*w).opacity = normalize_d_range(
                (*w).opacity as libc::c_double
                    + (*ps).o.fade_in_step as libc::c_double * steps as libc::c_double,
                0.0f64,
                (*w).opacity_tgt as libc::c_double,
            ) as opacity_t;
        } else {
            (*w).opacity = normalize_d_range(
                (*w).opacity as libc::c_double
                    - (*ps).o.fade_out_step as libc::c_double * steps as libc::c_double,
                (*w).opacity_tgt as libc::c_double,
                0xffffffff as u32 as libc::c_double,
            ) as opacity_t;
        }
    }
    if (*w).opacity != (*w).opacity_tgt {
        (*ps).idling = 0 as i32 != 0;
    }
}
unsafe extern "C" fn set_fade_callback(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut callback: Option<unsafe extern "C" fn(*mut session_t, *mut win) -> ()>,
    mut exec_callback: bool,
) {
    let mut old_callback: Option<unsafe extern "C" fn(*mut session_t, *mut win) -> ()> = (*w)
        .fade_callback;
    (*w).fade_callback = callback;
    if exec_callback as i32 != 0 && old_callback.is_some() {
        old_callback.expect("non-null function pointer")(ps, w);
        (*ps).idling = 0 as i32 != 0;
    }
}
unsafe extern "C" fn gaussian(
    mut r: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    return 1 as i32 as libc::c_double
        / sqrt(2 as i32 as libc::c_double * 3.14159265358979323846f64 * r)
        * exp(-(x * x + y * y) / (2 as i32 as libc::c_double * r * r));
}
unsafe extern "C" fn make_gaussian_map(mut r: libc::c_double) -> *mut conv {
    let mut c: *mut conv = 0 as *mut conv;
    let mut size: i32 = ceil(r * 3 as i32 as libc::c_double) as i32 + 1 as i32
        & !(1 as i32);
    let mut center: i32 = size / 2 as i32;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut t: libc::c_double = 0.;
    let mut g: libc::c_double = 0.;
    c = malloc(
        (::core::mem::size_of::<conv>() as u64)
            .wrapping_add(
                ((size * size) as u64)
                    .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
            ),
    ) as *mut conv;
    (*c).size = size;
    (*c).data = c.offset(1 as i32 as isize) as *mut libc::c_double;
    t = 0.0f64;
    y = 0 as i32;
    while y < size {
        x = 0 as i32;
        while x < size {
            g = gaussian(
                r,
                (x - center) as libc::c_double,
                (y - center) as libc::c_double,
            );
            t += g;
            *((*c).data).offset((y * size + x) as isize) = g;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    y = 0 as i32;
    while y < size {
        x = 0 as i32;
        while x < size {
            *((*c).data).offset((y * size + x) as isize) /= t;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    return c;
}
unsafe extern "C" fn sum_gaussian(
    mut map: *mut conv,
    mut opacity: libc::c_double,
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut height: i32,
) -> u8 {
    let mut fx: i32 = 0;
    let mut fy: i32 = 0;
    let mut g_data: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut g_line: *mut libc::c_double = (*map).data;
    let mut g_size: i32 = (*map).size;
    let mut center: i32 = g_size / 2 as i32;
    let mut fx_start: i32 = 0;
    let mut fx_end: i32 = 0;
    let mut fy_start: i32 = 0;
    let mut fy_end: i32 = 0;
    let mut v: libc::c_double = 0.;
    fx_start = center - x;
    if fx_start < 0 as i32 {
        fx_start = 0 as i32;
    }
    fx_end = width + center - x;
    if fx_end > g_size {
        fx_end = g_size;
    }
    fy_start = center - y;
    if fy_start < 0 as i32 {
        fy_start = 0 as i32;
    }
    fy_end = height + center - y;
    if fy_end > g_size {
        fy_end = g_size;
    }
    g_line = g_line.offset((fy_start * g_size) as isize).offset(fx_start as isize);
    v = 0 as i32 as libc::c_double;
    fy = fy_start;
    while fy < fy_end {
        g_data = g_line;
        g_line = g_line.offset(g_size as isize);
        fx = fx_start;
        while fx < fx_end {
            let fresh6 = g_data;
            g_data = g_data.offset(1);
            v += *fresh6;
            fx += 1;
            fx;
        }
        fy += 1;
        fy;
    }
    if v > 1 as i32 as libc::c_double {
        v = 1 as i32 as libc::c_double;
    }
    return (v * opacity * 255.0f64) as u8;
}
unsafe extern "C" fn presum_gaussian(mut ps: *mut session_t, mut map: *mut conv) {
    let mut center: i32 = (*map).size / 2 as i32;
    let mut opacity: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    (*ps).cgsize = (*map).size;
    if !((*ps).shadow_corner).is_null() {
        free((*ps).shadow_corner as *mut libc::c_void);
    }
    if !((*ps).shadow_top).is_null() {
        free((*ps).shadow_top as *mut libc::c_void);
    }
    (*ps).shadow_corner = malloc(
        (((*ps).cgsize + 1 as i32) * ((*ps).cgsize + 1 as i32) * 26 as i32) as u64,
    ) as *mut u8;
    (*ps).shadow_top = malloc((((*ps).cgsize + 1 as i32) * 26 as i32) as u64) as *mut u8;
    x = 0 as i32;
    while x <= (*ps).cgsize {
        *((*ps).shadow_top)
            .offset((25 as i32 * ((*ps).cgsize + 1 as i32) + x) as isize) = sum_gaussian(
            map,
            1 as i32 as libc::c_double,
            x - center,
            center,
            (*ps).cgsize * 2 as i32,
            (*ps).cgsize * 2 as i32,
        );
        opacity = 0 as i32;
        while opacity < 25 as i32 {
            *((*ps).shadow_top)
                .offset((opacity * ((*ps).cgsize + 1 as i32) + x) as isize) = (*((*ps)
                .shadow_top)
                .offset((25 as i32 * ((*ps).cgsize + 1 as i32) + x) as isize) as i32
                * opacity / 25 as i32) as u8;
            opacity += 1;
            opacity;
        }
        y = 0 as i32;
        while y <= x {
            *((*ps).shadow_corner)
                .offset(
                    (25 as i32 * ((*ps).cgsize + 1 as i32) * ((*ps).cgsize + 1 as i32)
                        + y * ((*ps).cgsize + 1 as i32) + x) as isize,
                ) = sum_gaussian(
                map,
                1 as i32 as libc::c_double,
                x - center,
                y - center,
                (*ps).cgsize * 2 as i32,
                (*ps).cgsize * 2 as i32,
            );
            *((*ps).shadow_corner)
                .offset(
                    (25 as i32 * ((*ps).cgsize + 1 as i32) * ((*ps).cgsize + 1 as i32)
                        + x * ((*ps).cgsize + 1 as i32) + y) as isize,
                ) = *((*ps).shadow_corner)
                .offset(
                    (25 as i32 * ((*ps).cgsize + 1 as i32) * ((*ps).cgsize + 1 as i32)
                        + y * ((*ps).cgsize + 1 as i32) + x) as isize,
                );
            opacity = 0 as i32;
            while opacity < 25 as i32 {
                let ref mut fresh7 = *((*ps).shadow_corner)
                    .offset(
                        (opacity * ((*ps).cgsize + 1 as i32) * ((*ps).cgsize + 1 as i32)
                            + x * ((*ps).cgsize + 1 as i32) + y) as isize,
                    );
                *fresh7 = (*((*ps).shadow_corner)
                    .offset(
                        (25 as i32 * ((*ps).cgsize + 1 as i32)
                            * ((*ps).cgsize + 1 as i32) + y * ((*ps).cgsize + 1 as i32)
                            + x) as isize,
                    ) as i32 * opacity / 25 as i32) as u8;
                *((*ps).shadow_corner)
                    .offset(
                        (opacity * ((*ps).cgsize + 1 as i32) * ((*ps).cgsize + 1 as i32)
                            + y * ((*ps).cgsize + 1 as i32) + x) as isize,
                    ) = *fresh7;
                opacity += 1;
                opacity;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
}
unsafe extern "C" fn make_shadow(
    mut ps: *mut session_t,
    mut opacity: libc::c_double,
    mut width: i32,
    mut height: i32,
) -> *mut XImage {
    let mut ximage: *mut XImage = 0 as *mut XImage;
    let mut data: *mut u8 = 0 as *mut u8;
    let mut ylimit: i32 = 0;
    let mut xlimit: i32 = 0;
    let mut swidth: i32 = width + (*ps).cgsize;
    let mut sheight: i32 = height + (*ps).cgsize;
    let mut center: i32 = (*ps).cgsize / 2 as i32;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut d: u8 = 0;
    let mut x_diff: i32 = 0;
    let mut opacity_int: i32 = (opacity * 25 as i32 as libc::c_double) as i32;
    data = malloc(
        ((swidth * sheight) as u64).wrapping_mul(::core::mem::size_of::<u8>() as u64),
    ) as *mut u8;
    if data.is_null() {
        return 0 as *mut XImage;
    }
    ximage = XCreateImage(
        (*ps).dpy,
        (*ps).vis,
        8 as i32 as u32,
        2 as i32,
        0 as i32,
        data as *mut i8,
        swidth as u32,
        sheight as u32,
        8 as i32,
        (swidth as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32,
    );
    if ximage.is_null() {
        free(data as *mut libc::c_void);
        return 0 as *mut XImage;
    }
    if (*ps).cgsize > 0 as i32 {
        d = *((*ps).shadow_top)
            .offset((opacity_int * ((*ps).cgsize + 1 as i32) + (*ps).cgsize) as isize);
    } else {
        d = sum_gaussian((*ps).gaussian_map, opacity, center, center, width, height);
    }
    memset(data as *mut libc::c_void, d as i32, (sheight * swidth) as u64);
    ylimit = (*ps).cgsize;
    if ylimit > sheight / 2 as i32 {
        ylimit = (sheight + 1 as i32) / 2 as i32;
    }
    xlimit = (*ps).cgsize;
    if xlimit > swidth / 2 as i32 {
        xlimit = (swidth + 1 as i32) / 2 as i32;
    }
    y = 0 as i32;
    while y < ylimit {
        x = 0 as i32;
        while x < xlimit {
            if xlimit == (*ps).cgsize && ylimit == (*ps).cgsize {
                d = *((*ps).shadow_corner)
                    .offset(
                        (opacity_int * ((*ps).cgsize + 1 as i32)
                            * ((*ps).cgsize + 1 as i32) + y * ((*ps).cgsize + 1 as i32)
                            + x) as isize,
                    );
            } else {
                d = sum_gaussian(
                    (*ps).gaussian_map,
                    opacity,
                    x - center,
                    y - center,
                    width,
                    height,
                );
            }
            *data.offset((y * swidth + x) as isize) = d;
            *data.offset(((sheight - y - 1 as i32) * swidth + x) as isize) = d;
            *data
                .offset(
                    ((sheight - y - 1 as i32) * swidth + (swidth - x - 1 as i32))
                        as isize,
                ) = d;
            *data.offset((y * swidth + (swidth - x - 1 as i32)) as isize) = d;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    x_diff = swidth - (*ps).cgsize * 2 as i32;
    if x_diff > 0 as i32 && ylimit > 0 as i32 {
        y = 0 as i32;
        while y < ylimit {
            if ylimit == (*ps).cgsize {
                d = *((*ps).shadow_top)
                    .offset((opacity_int * ((*ps).cgsize + 1 as i32) + y) as isize);
            } else {
                d = sum_gaussian(
                    (*ps).gaussian_map,
                    opacity,
                    center,
                    y - center,
                    width,
                    height,
                );
            }
            memset(
                &mut *data.offset((y * swidth + (*ps).cgsize) as isize) as *mut u8
                    as *mut libc::c_void,
                d as i32,
                x_diff as u64,
            );
            memset(
                &mut *data
                    .offset(((sheight - y - 1 as i32) * swidth + (*ps).cgsize) as isize)
                    as *mut u8 as *mut libc::c_void,
                d as i32,
                x_diff as u64,
            );
            y += 1;
            y;
        }
    }
    x = 0 as i32;
    while x < xlimit {
        if xlimit == (*ps).cgsize {
            d = *((*ps).shadow_top)
                .offset((opacity_int * ((*ps).cgsize + 1 as i32) + x) as isize);
        } else {
            d = sum_gaussian(
                (*ps).gaussian_map,
                opacity,
                x - center,
                center,
                width,
                height,
            );
        }
        y = (*ps).cgsize;
        while y < sheight - (*ps).cgsize {
            *data.offset((y * swidth + x) as isize) = d;
            *data.offset((y * swidth + (swidth - x - 1 as i32)) as isize) = d;
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    return ximage;
}
unsafe extern "C" fn win_build_shadow(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut opacity: libc::c_double,
) -> bool {
    let width: i32 = (*w).widthb;
    let height: i32 = (*w).heightb;
    let mut shadow_image: *mut XImage = 0 as *mut XImage;
    let mut shadow_pixmap: Pixmap = 0 as i64 as Pixmap;
    let mut shadow_pixmap_argb: Pixmap = 0 as i64 as Pixmap;
    let mut shadow_picture: Picture = 0 as i64 as Picture;
    let mut shadow_picture_argb: Picture = 0 as i64 as Picture;
    let mut gc: GC = 0 as GC;
    shadow_image = make_shadow(ps, opacity, width, height);
    if shadow_image.is_null() {
        return 0 as i64 != 0;
    }
    shadow_pixmap = XCreatePixmap(
        (*ps).dpy,
        (*ps).root,
        (*shadow_image).width as u32,
        (*shadow_image).height as u32,
        8 as i32 as u32,
    );
    shadow_pixmap_argb = XCreatePixmap(
        (*ps).dpy,
        (*ps).root,
        (*shadow_image).width as u32,
        (*shadow_image).height as u32,
        32 as i32 as u32,
    );
    if !(shadow_pixmap == 0 || shadow_pixmap_argb == 0) {
        shadow_picture = XRenderCreatePicture(
            (*ps).dpy,
            shadow_pixmap,
            XRenderFindStandardFormat((*ps).dpy, 2 as i32),
            0 as i32 as u64,
            0 as *const XRenderPictureAttributes,
        );
        shadow_picture_argb = XRenderCreatePicture(
            (*ps).dpy,
            shadow_pixmap_argb,
            XRenderFindStandardFormat((*ps).dpy, 0 as i32),
            0 as i32 as u64,
            0 as *const XRenderPictureAttributes,
        );
        if !(shadow_picture == 0 || shadow_picture_argb == 0) {
            gc = XCreateGC(
                (*ps).dpy,
                shadow_pixmap,
                0 as i32 as u64,
                0 as *mut XGCValues,
            );
            if !gc.is_null() {
                XPutImage(
                    (*ps).dpy,
                    shadow_pixmap,
                    gc,
                    shadow_image,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    (*shadow_image).width as u32,
                    (*shadow_image).height as u32,
                );
                XRenderComposite(
                    (*ps).dpy,
                    1 as i32,
                    (*ps).cshadow_picture,
                    shadow_picture,
                    shadow_picture_argb,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    (*shadow_image).width as u32,
                    (*shadow_image).height as u32,
                );
                (*w).shadow_paint.pixmap = shadow_pixmap_argb;
                (*w).shadow_paint.pict = shadow_picture_argb;
                xr_sync_(ps, (*w).shadow_paint.pixmap, 0 as *mut XSyncFence);
                XFreeGC((*ps).dpy, gc);
                (Some(
                    ((*shadow_image).f.destroy_image).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(shadow_image);
                XFreePixmap((*ps).dpy, shadow_pixmap);
                XRenderFreePicture((*ps).dpy, shadow_picture);
                return 1 as i32 != 0;
            }
        }
    }
    if !shadow_image.is_null() {
        (Some(((*shadow_image).f.destroy_image).expect("non-null function pointer")))
            .expect("non-null function pointer")(shadow_image);
    }
    if shadow_pixmap != 0 {
        XFreePixmap((*ps).dpy, shadow_pixmap);
    }
    if shadow_pixmap_argb != 0 {
        XFreePixmap((*ps).dpy, shadow_pixmap_argb);
    }
    if shadow_picture != 0 {
        XRenderFreePicture((*ps).dpy, shadow_picture);
    }
    if shadow_picture_argb != 0 {
        XRenderFreePicture((*ps).dpy, shadow_picture_argb);
    }
    if !gc.is_null() {
        XFreeGC((*ps).dpy, gc);
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn solid_picture(
    mut ps: *mut session_t,
    mut argb: bool,
    mut a: libc::c_double,
    mut r: libc::c_double,
    mut g: libc::c_double,
    mut b: libc::c_double,
) -> Picture {
    let mut pixmap: Pixmap = 0;
    let mut picture: Picture = 0;
    let mut pa: XRenderPictureAttributes = XRenderPictureAttributes {
        repeat: 0,
        alpha_map: 0,
        alpha_x_origin: 0,
        alpha_y_origin: 0,
        clip_x_origin: 0,
        clip_y_origin: 0,
        clip_mask: 0,
        graphics_exposures: 0,
        subwindow_mode: 0,
        poly_edge: 0,
        poly_mode: 0,
        dither: 0,
        component_alpha: 0,
    };
    let mut c: XRenderColor = XRenderColor {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };
    pixmap = XCreatePixmap(
        (*ps).dpy,
        (*ps).root,
        1 as i32 as u32,
        1 as i32 as u32,
        (if argb as i32 != 0 { 32 as i32 } else { 8 as i32 }) as u32,
    );
    if pixmap == 0 {
        return 0 as i64 as Picture;
    }
    pa.repeat = 1 as i32;
    picture = XRenderCreatePicture(
        (*ps).dpy,
        pixmap,
        XRenderFindStandardFormat(
            (*ps).dpy,
            if argb as i32 != 0 { 0 as i32 } else { 2 as i32 },
        ),
        ((1 as i32) << 0 as i32) as u64,
        &mut pa,
    );
    if picture == 0 {
        XFreePixmap((*ps).dpy, pixmap);
        return 0 as i64 as Picture;
    }
    c.alpha = (a * 0xffff as i32 as libc::c_double) as libc::c_ushort;
    c.red = (r * 0xffff as i32 as libc::c_double) as libc::c_ushort;
    c.green = (g * 0xffff as i32 as libc::c_double) as libc::c_ushort;
    c.blue = (b * 0xffff as i32 as libc::c_double) as libc::c_ushort;
    XRenderFillRectangle(
        (*ps).dpy,
        1 as i32,
        picture,
        &mut c,
        0 as i32,
        0 as i32,
        1 as i32 as u32,
        1 as i32 as u32,
    );
    XFreePixmap((*ps).dpy, pixmap);
    return picture;
}
unsafe extern "C" fn discard_ignore(mut ps: *mut session_t, mut sequence: u64) {
    while !((*ps).ignore_head).is_null() {
        if !(sequence.wrapping_sub((*(*ps).ignore_head).sequence) as i64
            > 0 as i32 as i64)
        {
            break;
        }
        let mut next: *mut ignore_t = (*(*ps).ignore_head).next;
        free((*ps).ignore_head as *mut libc::c_void);
        (*ps).ignore_head = next;
        if ((*ps).ignore_head).is_null() {
            (*ps).ignore_tail = &mut (*ps).ignore_head;
        }
    }
}
unsafe extern "C" fn set_ignore(mut ps: *mut session_t, mut sequence: u64) {
    if (*ps).o.show_all_xerrors {
        return;
    }
    let mut i: *mut ignore_t = malloc(::core::mem::size_of::<ignore_t>() as u64)
        as *mut ignore_t;
    if i.is_null() {
        return;
    }
    (*i).sequence = sequence;
    (*i).next = 0 as *mut _ignore;
    *(*ps).ignore_tail = i;
    (*ps).ignore_tail = &mut (*i).next;
}
unsafe extern "C" fn should_ignore(mut ps: *mut session_t, mut sequence: u64) -> i32 {
    discard_ignore(ps, sequence);
    return (!((*ps).ignore_head).is_null() && (*(*ps).ignore_head).sequence == sequence)
        as i32;
}
#[no_mangle]
pub unsafe extern "C" fn wid_get_prop_adv(
    mut ps: *const session_t,
    mut w: Window,
    mut atom: Atom,
    mut offset: i64,
    mut length: i64,
    mut rtype: Atom,
    mut rformat: i32,
) -> winprop_t {
    let mut type_0: Atom = 0 as i64 as Atom;
    let mut format: i32 = 0 as i32;
    let mut nitems: u64 = 0 as i32 as u64;
    let mut after: u64 = 0 as i32 as u64;
    let mut data: *mut u8 = 0 as *mut u8;
    if 0 as i32
        == XGetWindowProperty(
            (*ps).dpy,
            w,
            atom,
            offset,
            length,
            0 as i32,
            rtype,
            &mut type_0,
            &mut format,
            &mut nitems,
            &mut after,
            &mut data,
        ) && nitems != 0 && (0 as i64 as u64 == type_0 || type_0 == rtype)
        && (rformat == 0 || format == rformat)
        && (8 as i32 == format || 16 as i32 == format || 32 as i32 == format)
    {
        return {
            let mut init = winprop_t {
                data: C2RustUnnamed_3 { p8: data },
                nitems: nitems,
                type_0: type_0,
                format: format,
            };
            init
        };
    }
    cxfree(data as *mut libc::c_void);
    return {
        let mut init = winprop_t {
            data: C2RustUnnamed_3 {
                p8: 0 as *mut u8,
            },
            nitems: 0 as i32 as u64,
            type_0: 0 as i64 as Atom,
            format: 0 as i32,
        };
        init
    };
}
unsafe extern "C" fn win_rounded_corners(mut ps: *mut session_t, mut w: *mut win) {
    (*w).rounded_corners = 0 as i32 != 0;
    if !(*w).bounding_shaped {
        return;
    }
    if (*w).border_size == 0 {
        (*w).border_size = border_size(ps, w, 1 as i32 != 0);
    }
    if (*w).border_size == 0 {
        return;
    }
    let mut minwidth: libc::c_ushort = max_i(
        ((*w).widthb as libc::c_double * (1 as i32 as libc::c_double - 0.05f64)) as i32,
        (*w).widthb - 10 as i32,
    ) as libc::c_ushort;
    let mut minheight: libc::c_ushort = max_i(
        ((*w).heightb as libc::c_double * (1 as i32 as libc::c_double - 0.05f64)) as i32,
        (*w).heightb - 10 as i32,
    ) as libc::c_ushort;
    let mut nrects: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut rects: *mut XRectangle = XFixesFetchRegion(
        (*ps).dpy,
        (*w).border_size,
        &mut nrects,
    );
    if rects.is_null() {
        return;
    }
    i = 0 as i32;
    while i < nrects {
        if (*rects.offset(i as isize)).width as i32 >= minwidth as i32
            && (*rects.offset(i as isize)).height as i32 >= minheight as i32
        {
            (*w).rounded_corners = 1 as i32 != 0;
            break;
        } else {
            i += 1;
            i;
        }
    }
    cxfree(rects as *mut libc::c_void);
}
unsafe extern "C" fn condlst_add(
    mut ps: *mut session_t,
    mut pcondlst: *mut *mut c2_lptr_t,
    mut pattern: *const i8,
) -> bool {
    if pattern.is_null() {
        return 0 as i32 != 0;
    }
    if (c2_parsed(ps, pcondlst, pattern, 0 as *mut libc::c_void)).is_null() {
        exit(1 as i32);
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn determine_evmask(
    mut ps: *mut session_t,
    mut wid: Window,
    mut mode: win_evmode_t,
) -> i64 {
    let mut evmask: i64 = 0 as i64;
    let mut w: *mut win = 0 as *mut win;
    if win_evmode_t::WIN_EVMODE_FRAME as i32 as u32 == mode as u32
        || {
            w = find_win(ps, wid);
            !w.is_null() && 2 as i32 == (*w).a.map_state
        }
    {
        evmask |= (1 as i64) << 22 as i32;
        if (*ps).o.track_focus as i32 != 0 && !(*ps).o.use_ewmh_active_win {
            evmask |= (1 as i64) << 21 as i32;
        }
    }
    if win_evmode_t::WIN_EVMODE_CLIENT as i32 as u32 == mode as u32
        || {
            w = find_toplevel(ps, wid);
            !w.is_null() && 2 as i32 == (*w).a.map_state
        }
    {
        if (*ps).o.frame_opacity != 0. || (*ps).o.track_wdata as i32 != 0
            || !((*ps).track_atom_lst).is_null()
            || (*ps).o.detect_client_opacity as i32 != 0
        {
            evmask |= (1 as i64) << 22 as i32;
        }
    }
    return evmask;
}
unsafe extern "C" fn find_toplevel2(
    mut ps: *mut session_t,
    mut wid: Window,
) -> *mut win {
    let mut w: *mut win = 0 as *mut win;
    while wid != 0 && wid != (*ps).root
        && {
            w = find_win(ps, wid);
            w.is_null()
        }
    {
        let mut troot: Window = 0;
        let mut parent: Window = 0;
        let mut tchildren: *mut Window = 0 as *mut Window;
        let mut tnchildren: u32 = 0;
        if XQueryTree(
            (*ps).dpy,
            wid,
            &mut troot,
            &mut parent,
            &mut tchildren,
            &mut tnchildren,
        ) == 0
        {
            parent = 0 as i32 as Window;
            break;
        } else {
            cxfree(tchildren as *mut libc::c_void);
            wid = parent;
        }
    }
    return w;
}
unsafe extern "C" fn recheck_focus(mut ps: *mut session_t) -> *mut win {
    if (*ps).o.use_ewmh_active_win {
        update_ewmh_active_win(ps);
        return (*ps).active_win;
    }
    let mut wid: Window = 0 as i32 as Window;
    let mut revert_to: i32 = 0;
    XGetInputFocus((*ps).dpy, &mut wid, &mut revert_to);
    let mut w: *mut win = find_win_all(ps, wid);
    if !w.is_null() {
        win_set_focused(ps, w, 1 as i32 != 0);
        return w;
    }
    return 0 as *mut win;
}
unsafe extern "C" fn get_root_tile(mut ps: *mut session_t) -> bool {
    (*ps).root_tile_fill = 0 as i32 != 0;
    let mut fill: bool = 0 as i32 != 0;
    let mut pixmap: Pixmap = 0 as i64 as Pixmap;
    let mut p: i32 = 0 as i32;
    while !(background_props_str[p as usize]).is_null() {
        let mut prop: winprop_t = wid_get_prop(
            ps,
            (*ps).root,
            get_atom(ps, background_props_str[p as usize]),
            1 as i64,
            20 as i32 as Atom,
            32 as i32,
        );
        if prop.nitems != 0 {
            pixmap = *prop.data.p32 as Pixmap;
            fill = 0 as i32 != 0;
            free_winprop(&mut prop);
            break;
        } else {
            free_winprop(&mut prop);
            p += 1;
            p;
        }
    }
    if pixmap != 0 && !validate_pixmap(ps, pixmap) {
        pixmap = 0 as i64 as Pixmap;
    }
    if pixmap == 0 {
        pixmap = XCreatePixmap(
            (*ps).dpy,
            (*ps).root,
            1 as i32 as u32,
            1 as i32 as u32,
            (*ps).depth as u32,
        );
        fill = 1 as i32 != 0;
    }
    let mut pa: XRenderPictureAttributes = {
        let mut init = _XRenderPictureAttributes {
            repeat: 1 as i32,
            alpha_map: 0,
            alpha_x_origin: 0,
            alpha_y_origin: 0,
            clip_x_origin: 0,
            clip_y_origin: 0,
            clip_mask: 0,
            graphics_exposures: 0,
            subwindow_mode: 0,
            poly_edge: 0,
            poly_mode: 0,
            dither: 0,
            component_alpha: 0,
        };
        init
    };
    (*ps).root_tile_paint.pict = XRenderCreatePicture(
        (*ps).dpy,
        pixmap,
        XRenderFindVisualFormat((*ps).dpy, (*ps).vis),
        ((1 as i32) << 0 as i32) as u64,
        &mut pa,
    );
    if fill {
        let mut c: XRenderColor = XRenderColor {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        };
        c.blue = 0x8080 as i32 as libc::c_ushort;
        c.green = c.blue;
        c.red = c.green;
        c.alpha = 0xffff as i32 as libc::c_ushort;
        XRenderFillRectangle(
            (*ps).dpy,
            1 as i32,
            (*ps).root_tile_paint.pict,
            &mut c,
            0 as i32,
            0 as i32,
            1 as i32 as u32,
            1 as i32 as u32,
        );
    }
    (*ps).root_tile_fill = fill;
    (*ps).root_tile_paint.pixmap = pixmap;
    if backend::BKEND_GLX as i32 as u32 == (*ps).o.backend as u32 {
        return glx_bind_pixmap(
            ps,
            &mut (*ps).root_tile_paint.ptex,
            (*ps).root_tile_paint.pixmap,
            0 as i32 as u32,
            0 as i32 as u32,
            0 as i32 as u32,
        );
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn paint_root(mut ps: *mut session_t, mut reg_paint: XserverRegion) {
    if (*ps).root_tile_paint.pixmap == 0 {
        get_root_tile(ps);
    }
    win_render(
        ps,
        0 as *mut win,
        0 as i32,
        0 as i32,
        (*ps).root_width,
        (*ps).root_height,
        1.0f64,
        reg_paint,
        0 as *const reg_data_t,
        (*ps).root_tile_paint.pict,
    );
}
unsafe extern "C" fn win_get_region(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut use_offset: bool,
) -> XserverRegion {
    let mut r: XRectangle = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    r.x = (if use_offset as i32 != 0 { (*w).a.x } else { 0 as i32 }) as libc::c_short;
    r.y = (if use_offset as i32 != 0 { (*w).a.y } else { 0 as i32 }) as libc::c_short;
    r.width = (*w).widthb as libc::c_ushort;
    r.height = (*w).heightb as libc::c_ushort;
    return XFixesCreateRegion((*ps).dpy, &mut r, 1 as i32);
}
unsafe extern "C" fn win_get_region_noframe(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut use_offset: bool,
) -> XserverRegion {
    let extents: margin_t = win_calc_frame_extents(ps, w);
    let mut r: XRectangle = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    r.x = ((if use_offset as i32 != 0 { (*w).a.x } else { 0 as i32 }) + extents.left)
        as libc::c_short;
    r.y = ((if use_offset as i32 != 0 { (*w).a.y } else { 0 as i32 }) + extents.top)
        as libc::c_short;
    r.width = max_i((*w).a.width - extents.left - extents.right, 0 as i32)
        as libc::c_ushort;
    r.height = max_i((*w).a.height - extents.top - extents.bottom, 0 as i32)
        as libc::c_ushort;
    if r.width as i32 > 0 as i32 && r.height as i32 > 0 as i32 {
        return XFixesCreateRegion((*ps).dpy, &mut r, 1 as i32)
    } else {
        return XFixesCreateRegion((*ps).dpy, 0 as *mut XRectangle, 0 as i32)
    };
}
unsafe extern "C" fn win_extents(
    mut ps: *mut session_t,
    mut w: *mut win,
) -> XserverRegion {
    let mut r: XRectangle = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    r.x = (*w).a.x as libc::c_short;
    r.y = (*w).a.y as libc::c_short;
    r.width = (*w).widthb as libc::c_ushort;
    r.height = (*w).heightb as libc::c_ushort;
    if (*w).shadow {
        let mut sr: XRectangle = XRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        sr.x = ((*w).a.x + (*w).shadow_dx) as libc::c_short;
        sr.y = ((*w).a.y + (*w).shadow_dy) as libc::c_short;
        sr.width = (*w).shadow_width as libc::c_ushort;
        sr.height = (*w).shadow_height as libc::c_ushort;
        if (sr.x as i32) < r.x as i32 {
            r.width = (r.x as i32 + r.width as i32 - sr.x as i32) as libc::c_ushort;
            r.x = sr.x;
        }
        if (sr.y as i32) < r.y as i32 {
            r.height = (r.y as i32 + r.height as i32 - sr.y as i32) as libc::c_ushort;
            r.y = sr.y;
        }
        if sr.x as i32 + sr.width as i32 > r.x as i32 + r.width as i32 {
            r.width = (sr.x as i32 + sr.width as i32 - r.x as i32) as libc::c_ushort;
        }
        if sr.y as i32 + sr.height as i32 > r.y as i32 + r.height as i32 {
            r.height = (sr.y as i32 + sr.height as i32 - r.y as i32) as libc::c_ushort;
        }
    }
    return XFixesCreateRegion((*ps).dpy, &mut r, 1 as i32);
}
unsafe extern "C" fn border_size(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut use_offset: bool,
) -> XserverRegion {
    let mut fin: XserverRegion = win_get_region(ps, w, use_offset);
    if (*w).bounding_shaped {
        let mut border: XserverRegion = XFixesCreateRegionFromWindow(
            (*ps).dpy,
            (*w).id,
            0 as i32,
        );
        if border == 0 {
            return fin;
        }
        if use_offset {
            XFixesTranslateRegion(
                (*ps).dpy,
                border,
                (*w).a.x + (*w).a.border_width,
                (*w).a.y + (*w).a.border_width,
            );
        }
        XFixesIntersectRegion((*ps).dpy, fin, fin, border);
        XFixesDestroyRegion((*ps).dpy, border);
    }
    return fin;
}
unsafe extern "C" fn find_client_win(mut ps: *mut session_t, mut w: Window) -> Window {
    if wid_has_prop(ps, w, (*ps).atom_client) {
        return w;
    }
    let mut children: *mut Window = 0 as *mut Window;
    let mut nchildren: u32 = 0;
    let mut i: u32 = 0;
    let mut ret: Window = 0 as i32 as Window;
    if !wid_get_children(ps, w, &mut children, &mut nchildren) {
        return 0 as i32 as Window;
    }
    i = 0 as i32 as u32;
    while i < nchildren {
        ret = find_client_win(ps, *children.offset(i as isize));
        if ret != 0 {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    cxfree(children as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn get_frame_extents(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut client: Window,
) {
    memset(
        &mut (*w).frame_extents as *mut margin_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<margin_t>() as u64,
    );
    let mut prop: winprop_t = wid_get_prop(
        ps,
        client,
        (*ps).atom_frame_extents,
        4 as i64,
        6 as i32 as Atom,
        32 as i32,
    );
    if 4 as i32 as u64 == prop.nitems {
        let extents: *const i64 = prop.data.p32;
        (*w).frame_extents.left = *extents.offset(0 as i32 as isize) as i32;
        (*w).frame_extents.right = *extents.offset(1 as i32 as isize) as i32;
        (*w).frame_extents.top = *extents.offset(2 as i32 as isize) as i32;
        (*w).frame_extents.bottom = *extents.offset(3 as i32 as isize) as i32;
        if (*ps).o.frame_opacity != 0. {
            update_reg_ignore_expire(ps, w);
        }
    }
    free_winprop(&mut prop);
}
#[inline]
unsafe extern "C" fn get_alpha_pict_d(
    mut ps: *mut session_t,
    mut o: libc::c_double,
) -> Picture {
    return *((*ps).alpha_picts)
        .offset(round(normalize_d(o) / (*ps).o.alpha_step) as i32 as isize);
}
#[inline]
unsafe extern "C" fn get_alpha_pict_o(
    mut ps: *mut session_t,
    mut o: opacity_t,
) -> Picture {
    return get_alpha_pict_d(
        ps,
        o as libc::c_double / 0xffffffff as u32 as libc::c_double,
    );
}
unsafe extern "C" fn paint_preprocess(
    mut ps: *mut session_t,
    mut list: *mut win,
) -> *mut win {
    let mut t: *mut win = 0 as *mut win;
    let mut next: *mut win = 0 as *mut win;
    let mut steps: time_ms_t = 0 as i64;
    if (*ps).fade_time != 0 {
        steps = (((get_time_ms() - (*ps).fade_time) as libc::c_double
            + 0.2f64 * (*ps).o.fade_delta as libc::c_double)
            / (*ps).o.fade_delta as libc::c_double) as time_ms_t;
    }
    if (*ps).fade_time == 0 || steps < 0 as i64 {
        (*ps).fade_time = get_time_ms();
        steps = 0 as i64;
    }
    (*ps).fade_time += steps * (*ps).o.fade_delta;
    let mut last_reg_ignore: XserverRegion = 0 as i64 as XserverRegion;
    let mut unredir_possible: bool = 0 as i32 != 0;
    let mut is_highest: bool = 1 as i32 != 0;
    let mut w: *mut win = list;
    while !w.is_null() {
        let mut to_paint: bool = 1 as i32 != 0;
        let mode_old: winmode_t = (*w).mode;
        next = (*w).next;
        let mut opacity_old: opacity_t = (*w).opacity;
        if (*w).flags & 0x1 as i32 as i64 != 0 {
            free_paint(ps, &mut (*w).shadow_paint);
        }
        if (*ps).reg_ignore_expire {
            free_region(ps, &mut (*w).reg_ignore);
        }
        if 0 as i32 == (*w).a.map_state {
            win_set_shadow(ps, w, (*w).shadow_last);
            (*w).fade = (*w).fade_last;
            win_set_invert_color(ps, w, (*w).invert_color_last);
            win_set_blur_background(ps, w, (*w).blur_background_last);
        }
        if 0x4 as i32 as i64 & (*w).flags != 0 {
            calc_opacity(ps, w);
            calc_dim(ps, w);
        }
        run_fade(ps, w, steps as u32);
        if !(*w).damaged || (*w).a.x + (*w).a.width < 1 as i32
            || (*w).a.y + (*w).a.height < 1 as i32 || (*w).a.x >= (*ps).root_width
            || (*w).a.y >= (*ps).root_height
            || (0 as i32 == (*w).a.map_state || (*w).destroyed as i32 != 0)
                && (*w).paint.pixmap == 0
            || get_alpha_pict_o(ps, (*w).opacity)
                == *((*ps).alpha_picts).offset(0 as i32 as isize)
            || (*w).paint_excluded as i32 != 0
        {
            to_paint = 0 as i32 != 0;
        }
        if to_paint as i32 != 0 && (!(*w).to_paint || (*w).opacity != opacity_old) {
            win_determine_mode(ps, w);
        }
        if to_paint {
            if (*w).border_size == 0 {
                (*w).border_size = border_size(ps, w, 1 as i32 != 0);
            }
            if (*w).extents == 0 {
                (*w).extents = win_extents(ps, w);
            }
            let mut frame_opacity_old: libc::c_double = (*w).frame_opacity;
            if (*ps).o.frame_opacity != 0. && 1.0f64 != (*ps).o.frame_opacity
                && win_has_frame(w) as i32 != 0
            {
                (*w).frame_opacity = get_opacity_percent(w) * (*ps).o.frame_opacity;
            } else {
                (*w).frame_opacity = 0.0f64;
            }
            if (*w).to_paint as i32 != 0
                && winmode_t::WMODE_SOLID as i32 as u32 == mode_old as u32
                && (0.0f64 == frame_opacity_old) as i32
                    != (0.0f64 == (*w).frame_opacity) as i32
            {
                (*ps).reg_ignore_expire = 1 as i32 != 0;
            }
            if (*w).frame_opacity != 0. {
                (*w).shadow_opacity = (*ps).o.shadow_opacity * (*w).frame_opacity;
            } else {
                (*w).shadow_opacity = (*ps).o.shadow_opacity * get_opacity_percent(w);
            }
        }
        if to_paint as i32 != (*w).to_paint as i32 || (*w).opacity != opacity_old {
            add_damage_win(ps, w);
        }
        if (to_paint as i32 != 0
            && winmode_t::WMODE_SOLID as i32 as u32 == (*w).mode as u32) as i32
            != ((*w).to_paint as i32 != 0
                && winmode_t::WMODE_SOLID as i32 as u32 == mode_old as u32) as i32
        {
            (*ps).reg_ignore_expire = 1 as i32 != 0;
        }
        if to_paint {
            if (*ps).reg_ignore_expire as i32 != 0 || !(*w).to_paint {
                free_region(ps, &mut (*w).reg_ignore);
                if win_is_solid(ps, w) {
                    if (*w).frame_opacity == 0. {
                        if (*w).border_size != 0 {
                            (*w).reg_ignore = copy_region(ps, (*w).border_size);
                        } else {
                            (*w).reg_ignore = win_get_region(ps, w, 1 as i32 != 0);
                        }
                    } else {
                        (*w).reg_ignore = win_get_region_noframe(ps, w, 1 as i32 != 0);
                        if (*w).border_size != 0 {
                            XFixesIntersectRegion(
                                (*ps).dpy,
                                (*w).reg_ignore,
                                (*w).reg_ignore,
                                (*w).border_size,
                            );
                        }
                    }
                    if last_reg_ignore != 0 {
                        XFixesUnionRegion(
                            (*ps).dpy,
                            (*w).reg_ignore,
                            (*w).reg_ignore,
                            last_reg_ignore,
                        );
                    }
                } else if last_reg_ignore != 0 {
                    (*w).reg_ignore = copy_region(ps, last_reg_ignore);
                } else {
                    (*w).reg_ignore = 0 as i64 as XserverRegion;
                }
            }
            last_reg_ignore = (*w).reg_ignore;
            if (*ps).o.unredir_if_possible as i32 != 0 && is_highest as i32 != 0
                && to_paint as i32 != 0
            {
                is_highest = 0 as i32 != 0;
                if win_is_solid(ps, w) as i32 != 0
                    && ((*w).frame_opacity == 0. || !win_has_frame(w))
                    && win_is_fullscreen(ps, w) as i32 != 0
                    && !(*w).unredir_if_possible_excluded
                {
                    unredir_possible = 1 as i32 != 0;
                }
            }
            (*w).flags = 0 as i32 as int_fast16_t;
        }
        let mut destroyed: bool = (*w).opacity_tgt == (*w).opacity
            && (*w).destroyed as i32 != 0;
        if to_paint {
            (*w).prev_trans = t;
            t = w;
        } else {
            check_fade_fin(ps, w);
        }
        if !destroyed {
            (*w).to_paint = to_paint;
            if (*w).to_paint {
                (*w).shadow_last = (*w).shadow;
                (*w).fade_last = (*w).fade;
                (*w).invert_color_last = (*w).invert_color;
                (*w).blur_background_last = (*w).blur_background;
            }
        }
        w = next;
    }
    if switch_t::UNSET as i32 as u32 != (*ps).o.redirected_force as u32 {
        unredir_possible = (*ps).o.redirected_force as u64 == 0;
    }
    if (*ps).o.unredir_if_possible as i32 != 0 && is_highest as i32 != 0
        && !(*ps).redirected
    {
        unredir_possible = 1 as i32 != 0;
    }
    if unredir_possible {
        if (*ps).redirected {
            if (*ps).o.unredir_if_possible_delay == 0
                || (*ps).tmout_unredir_hit as i32 != 0
            {
                redir_stop(ps);
            } else if !(*(*ps).tmout_unredir).enabled {
                timeout_reset(ps, (*ps).tmout_unredir);
                (*(*ps).tmout_unredir).enabled = 1 as i32 != 0;
            }
        }
    } else {
        (*(*ps).tmout_unredir).enabled = 0 as i32 != 0;
        redir_start(ps);
    }
    return t;
}
#[inline]
unsafe extern "C" fn win_paint_shadow(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut reg_paint: XserverRegion,
    mut pcache_reg: *const reg_data_t,
) {
    paint_bind_tex(
        ps,
        &mut (*w).shadow_paint,
        0 as i32 as u32,
        0 as i32 as u32,
        32 as i32 as u32,
        0 as i32 != 0,
    );
    if !paint_isvalid(ps, &mut (*w).shadow_paint) {
        fprintf(
            stderr,
            b"%s(%#010lx): Missing painting data. This is a bad sign.\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 17], &[i8; 17]>(b"win_paint_shadow\0"))
                .as_ptr(),
            (*w).id,
        );
        return;
    }
    render_(
        ps,
        0 as i32,
        0 as i32,
        (*w).a.x + (*w).shadow_dx,
        (*w).a.y + (*w).shadow_dy,
        (*w).shadow_width,
        (*w).shadow_height,
        (*w).shadow_opacity,
        1 as i32 != 0,
        0 as i32 != 0,
        (*w).shadow_paint.pict,
        (*w).shadow_paint.ptex,
        reg_paint,
        pcache_reg,
        0 as *const glx_prog_main_t,
    );
}
#[inline]
unsafe extern "C" fn xr_build_picture(
    mut ps: *mut session_t,
    mut wid: i32,
    mut hei: i32,
    mut pictfmt: *mut XRenderPictFormat,
) -> Picture {
    if pictfmt.is_null() {
        pictfmt = XRenderFindVisualFormat((*ps).dpy, (*ps).vis);
    }
    let mut depth: i32 = (*pictfmt).depth;
    let mut tmp_pixmap: Pixmap = XCreatePixmap(
        (*ps).dpy,
        (*ps).root,
        wid as u32,
        hei as u32,
        depth as u32,
    );
    if tmp_pixmap == 0 {
        return 0 as i64 as Picture;
    }
    let mut tmp_picture: Picture = XRenderCreatePicture(
        (*ps).dpy,
        tmp_pixmap,
        pictfmt,
        0 as i32 as u64,
        0 as *const XRenderPictureAttributes,
    );
    free_pixmap(ps, &mut tmp_pixmap);
    return tmp_picture;
}
unsafe extern "C" fn xr_blur_dst(
    mut ps: *mut session_t,
    mut tgt_buffer: Picture,
    mut x: i32,
    mut y: i32,
    mut wid: i32,
    mut hei: i32,
    mut blur_kerns: *mut *mut XFixed,
    mut reg_clip: XserverRegion,
) -> bool {
    let mut tmp_picture: Picture = xr_build_picture(
        ps,
        wid,
        hei,
        0 as *mut XRenderPictFormat,
    );
    if tmp_picture == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to build intermediate Picture.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 12], &[i8; 12]>(b"xr_blur_dst\0")).as_ptr(),
        );
        return 0 as i32 != 0;
    }
    if reg_clip != 0 && tmp_picture != 0 {
        XFixesSetPictureClipRegion(
            (*ps).dpy,
            tmp_picture,
            reg_clip as i32,
            0 as i32,
            0 as i32 as XserverRegion,
        );
    }
    let mut src_pict: Picture = tgt_buffer;
    let mut dst_pict: Picture = tmp_picture;
    let mut i: i32 = 0 as i32;
    while !(*blur_kerns.offset(i as isize)).is_null() {
        let mut convolution_blur: *mut XFixed = *blur_kerns.offset(i as isize);
        let mut kwid: i32 = (*convolution_blur.offset(0 as i32 as isize) as XDouble
            / 65536 as i32 as libc::c_double) as i32;
        let mut khei: i32 = (*convolution_blur.offset(1 as i32 as isize) as XDouble
            / 65536 as i32 as libc::c_double) as i32;
        let mut rd_from_tgt: bool = tgt_buffer == src_pict;
        XRenderSetPictureFilter(
            (*ps).dpy,
            src_pict,
            b"convolution\0" as *const u8 as *const i8,
            convolution_blur,
            kwid * khei + 2 as i32,
        );
        XRenderComposite(
            (*ps).dpy,
            1 as i32,
            src_pict,
            0 as i64 as Picture,
            dst_pict,
            if rd_from_tgt as i32 != 0 { x } else { 0 as i32 },
            if rd_from_tgt as i32 != 0 { y } else { 0 as i32 },
            0 as i32,
            0 as i32,
            if rd_from_tgt as i32 != 0 { 0 as i32 } else { x },
            if rd_from_tgt as i32 != 0 { 0 as i32 } else { y },
            wid as u32,
            hei as u32,
        );
        xrfilter_reset(ps, src_pict);
        let mut tmp: XserverRegion = src_pict;
        src_pict = dst_pict;
        dst_pict = tmp;
        i += 1;
        i;
    }
    if src_pict != tgt_buffer {
        XRenderComposite(
            (*ps).dpy,
            1 as i32,
            src_pict,
            0 as i64 as Picture,
            tgt_buffer,
            0 as i32,
            0 as i32,
            0 as i32,
            0 as i32,
            x,
            y,
            wid as u32,
            hei as u32,
        );
    }
    free_picture(ps, &mut tmp_picture);
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn win_blur_background(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut tgt_buffer: Picture,
    mut reg_paint: XserverRegion,
    mut pcache_reg: *const reg_data_t,
) {
    let x: i32 = (*w).a.x;
    let y: i32 = (*w).a.y;
    let wid: i32 = (*w).widthb;
    let hei: i32 = (*w).heightb;
    let mut factor_center: libc::c_double = 1.0f64;
    if !(*ps).o.blur_background_fixed {
        let mut pct: libc::c_double = 1.0f64
            - get_opacity_percent(w) * (1.0f64 - 1.0f64 / 9.0f64);
        factor_center = pct * 8.0f64 / (1.1f64 - pct);
    }
    match (*ps).o.backend as u32 {
        0 | 2 => {
            let mut i: i32 = 0 as i32;
            while i < 5 as i32 {
                let mut kern_src: *mut XFixed = (*ps).o.blur_kerns[i as usize];
                let mut kern_dst: *mut XFixed = (*ps).blur_kerns_cache[i as usize];
                if kern_src.is_null() {
                    break;
                }
                if !((*ps).o.blur_background_fixed as i32 != 0 && !kern_dst.is_null()) {
                    let mut kwid: i32 = (*kern_src.offset(0 as i32 as isize) as XDouble
                        / 65536 as i32 as libc::c_double) as i32;
                    let mut khei: i32 = (*kern_src.offset(1 as i32 as isize) as XDouble
                        / 65536 as i32 as libc::c_double) as i32;
                    if kern_dst.is_null() {
                        kern_dst = malloc(
                            ((kwid * khei + 2 as i32) as u64)
                                .wrapping_mul(::core::mem::size_of::<XFixed>() as u64),
                        ) as *mut XFixed;
                        if kern_dst.is_null() {
                            fprintf(
                                stderr,
                                b"%s(): Failed to allocate memory for blur kernel.\n\0"
                                    as *const u8 as *const i8,
                                (*::core::mem::transmute::<
                                    &[u8; 20],
                                    &[i8; 20],
                                >(b"win_blur_background\0"))
                                    .as_ptr(),
                            );
                            return;
                        }
                        (*ps).blur_kerns_cache[i as usize] = kern_dst;
                    }
                    *kern_src
                        .offset(
                            (2 as i32 + khei / 2 as i32 * kwid + kwid / 2 as i32)
                                as isize,
                        ) = (factor_center * 65536 as i32 as libc::c_double) as XFixed;
                    memcpy(
                        kern_dst as *mut libc::c_void,
                        kern_src as *const libc::c_void,
                        ((kwid * khei + 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<XFixed>() as u64),
                    );
                    normalize_conv_kern(kwid, khei, kern_dst.offset(2 as i32 as isize));
                }
                i += 1;
                i;
            }
            let mut reg_noframe: XserverRegion = 0 as i64 as XserverRegion;
            if win_is_solid(ps, w) {
                let mut reg_all: XserverRegion = border_size(ps, w, 0 as i32 != 0);
                reg_noframe = win_get_region_noframe(ps, w, 0 as i32 != 0);
                XFixesSubtractRegion((*ps).dpy, reg_noframe, reg_all, reg_noframe);
                free_region(ps, &mut reg_all);
            }
            xr_blur_dst(
                ps,
                tgt_buffer,
                x,
                y,
                wid,
                hei,
                ((*ps).blur_kerns_cache).as_mut_ptr(),
                reg_noframe,
            );
            free_region(ps, &mut reg_noframe);
        }
        1 => {
            glx_blur_dst(
                ps,
                x,
                y,
                wid,
                hei,
                ((*(*ps).psglx).z as libc::c_double - 0.5f64) as libc::c_float,
                factor_center as GLfloat,
                reg_paint,
                pcache_reg,
                &mut (*w).glx_blur_cache,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn render_(
    mut ps: *mut session_t,
    mut x: i32,
    mut y: i32,
    mut dx: i32,
    mut dy: i32,
    mut wid: i32,
    mut hei: i32,
    mut opacity: libc::c_double,
    mut argb: bool,
    mut neg: bool,
    mut pict: Picture,
    mut ptex: *mut glx_texture_t,
    mut reg_paint: XserverRegion,
    mut pcache_reg: *const reg_data_t,
    mut pprogram: *const glx_prog_main_t,
) {
    match (*ps).o.backend as u32 {
        0 | 2 => {
            let mut alpha_pict: Picture = get_alpha_pict_d(ps, opacity);
            if alpha_pict != *((*ps).alpha_picts).offset(0 as i32 as isize) {
                let mut op: i32 = if !argb && alpha_pict == 0 {
                    1 as i32
                } else {
                    3 as i32
                };
                XRenderComposite(
                    (*ps).dpy,
                    op,
                    pict,
                    alpha_pict,
                    (*ps).tgt_buffer.pict,
                    x,
                    y,
                    0 as i32,
                    0 as i32,
                    dx,
                    dy,
                    wid as u32,
                    hei as u32,
                );
            }
        }
        1 => {
            glx_render_(
                ps,
                ptex,
                x,
                y,
                dx,
                dy,
                wid,
                hei,
                (*(*ps).psglx).z,
                opacity,
                argb,
                neg,
                reg_paint,
                pcache_reg,
                pprogram,
            );
            (*(*ps).psglx).z += 1 as i32;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn win_paint_win(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut reg_paint: XserverRegion,
    mut pcache_reg: *const reg_data_t,
) {
    glx_mark_(
        ps,
        (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"win_paint_win\0")).as_ptr(),
        (*w).id,
        1 as i32 != 0,
    );
    if (*w).paint.pixmap == 0 && (*ps).has_name_pixmap as i32 != 0 {
        set_ignore_next(ps);
        (*w).paint.pixmap = XCompositeNameWindowPixmap((*ps).dpy, (*w).id);
        if (*w).paint.pixmap != 0 {
            free_fence(ps, &mut (*w).fence);
        }
    }
    let mut draw: Drawable = (*w).paint.pixmap;
    if draw == 0 {
        draw = (*w).id;
    }
    if bkend_use_xrender(ps) as i32 != 0 && (*w).paint.pict == 0 {
        let mut pa: XRenderPictureAttributes = {
            let mut init = _XRenderPictureAttributes {
                repeat: 0,
                alpha_map: 0,
                alpha_x_origin: 0,
                alpha_y_origin: 0,
                clip_x_origin: 0,
                clip_y_origin: 0,
                clip_mask: 0,
                graphics_exposures: 0,
                subwindow_mode: 1 as i32,
                poly_edge: 0,
                poly_mode: 0,
                dither: 0,
                component_alpha: 0,
            };
            init
        };
        (*w).paint.pict = XRenderCreatePicture(
            (*ps).dpy,
            draw,
            (*w).pictfmt,
            ((1 as i32) << 8 as i32) as u64,
            &mut pa,
        );
    }
    if 2 as i32 == (*w).a.map_state {
        xr_sync_(ps, draw, &mut (*w).fence);
    }
    if !paint_bind_tex(
        ps,
        &mut (*w).paint,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        !(*ps).o.glx_no_rebind_pixmap && (*w).pixmap_damaged as i32 != 0,
    ) {
        fprintf(
            stderr,
            b"%s(%#010lx): Failed to bind texture. Expect troubles.\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"win_paint_win\0"))
                .as_ptr(),
            (*w).id,
        );
    }
    (*w).pixmap_damaged = 0 as i32 != 0;
    if !paint_isvalid(ps, &mut (*w).paint) {
        fprintf(
            stderr,
            b"%s(%#010lx): Missing painting data. This is a bad sign.\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"win_paint_win\0"))
                .as_ptr(),
            (*w).id,
        );
        return;
    }
    let x: i32 = (*w).a.x;
    let y: i32 = (*w).a.y;
    let wid: i32 = (*w).widthb;
    let hei: i32 = (*w).heightb;
    let mut pict: Picture = (*w).paint.pict;
    if bkend_use_xrender(ps) as i32 != 0 && (*w).invert_color as i32 != 0 {
        let mut newpict: Picture = xr_build_picture(ps, wid, hei, (*w).pictfmt);
        if newpict != 0 {
            if reg_paint != 0 {
                let mut reg: XserverRegion = copy_region(ps, reg_paint);
                XFixesTranslateRegion((*ps).dpy, reg, -x, -y);
                XFixesSetPictureClipRegion((*ps).dpy, newpict, 0 as i32, 0 as i32, reg);
                free_region(ps, &mut reg);
            }
            XRenderComposite(
                (*ps).dpy,
                1 as i32,
                pict,
                0 as i64 as Picture,
                newpict,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                wid as u32,
                hei as u32,
            );
            XRenderComposite(
                (*ps).dpy,
                0x39 as i32,
                (*ps).white_picture,
                0 as i64 as Picture,
                newpict,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                wid as u32,
                hei as u32,
            );
            if winmode_t::WMODE_ARGB as i32 as u32 == (*w).mode as u32 {
                XRenderComposite(
                    (*ps).dpy,
                    6 as i32,
                    pict,
                    0 as i64 as Picture,
                    newpict,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    wid as u32,
                    hei as u32,
                );
            }
            pict = newpict;
        }
    }
    let dopacity: libc::c_double = get_opacity_percent(w);
    if (*w).frame_opacity == 0. {
        win_render(
            ps,
            w,
            0 as i32,
            0 as i32,
            wid,
            hei,
            dopacity,
            reg_paint,
            pcache_reg,
            pict,
        );
    } else {
        let extents: margin_t = win_calc_frame_extents(ps, w);
        let t: i32 = extents.top;
        let l: i32 = extents.left;
        let b: i32 = extents.bottom;
        let r: i32 = extents.right;
        let mut phei: i32 = min_i(t, hei);
        if phei > 0 as i32 {
            win_render(
                ps,
                w,
                0 as i32,
                0 as i32,
                wid,
                phei,
                (*w).frame_opacity,
                reg_paint,
                pcache_reg,
                pict,
            );
        }
        if hei > t {
            phei = min_i(hei - t, b);
            if phei > 0 as i32 {
                win_render(
                    ps,
                    w,
                    0 as i32,
                    hei - phei,
                    wid,
                    phei,
                    (*w).frame_opacity,
                    reg_paint,
                    pcache_reg,
                    pict,
                );
            }
            phei = hei - t - phei;
            if phei > 0 as i32 {
                let mut pwid: i32 = min_i(l, wid);
                if pwid > 0 as i32 {
                    win_render(
                        ps,
                        w,
                        0 as i32,
                        t,
                        pwid,
                        phei,
                        (*w).frame_opacity,
                        reg_paint,
                        pcache_reg,
                        pict,
                    );
                }
                if wid > l {
                    pwid = min_i(wid - l, r);
                    if pwid > 0 as i32 {
                        win_render(
                            ps,
                            w,
                            wid - pwid,
                            t,
                            pwid,
                            phei,
                            (*w).frame_opacity,
                            reg_paint,
                            pcache_reg,
                            pict,
                        );
                    }
                    pwid = wid - l - pwid;
                    if pwid > 0 as i32 {
                        win_render(
                            ps,
                            w,
                            l,
                            t,
                            pwid,
                            phei,
                            dopacity,
                            reg_paint,
                            pcache_reg,
                            pict,
                        );
                    }
                }
            }
        }
    }
    if pict != (*w).paint.pict {
        free_picture(ps, &mut pict);
    }
    if (*w).dim {
        let mut dim_opacity: libc::c_double = (*ps).o.inactive_dim;
        if !(*ps).o.inactive_dim_fixed {
            dim_opacity *= get_opacity_percent(w);
        }
        match (*ps).o.backend as u32 {
            0 | 2 => {
                let mut cval: libc::c_ushort = (0xffff as i32 as libc::c_double
                    * dim_opacity) as libc::c_ushort;
                let mut color: XRenderColor = {
                    let mut init = XRenderColor {
                        red: 0 as i32 as libc::c_ushort,
                        green: 0 as i32 as libc::c_ushort,
                        blue: 0 as i32 as libc::c_ushort,
                        alpha: cval,
                    };
                    init
                };
                let mut rect: XRectangle = {
                    let mut init = XRectangle {
                        x: x as libc::c_short,
                        y: y as libc::c_short,
                        width: wid as libc::c_ushort,
                        height: hei as libc::c_ushort,
                    };
                    init
                };
                XRenderFillRectangles(
                    (*ps).dpy,
                    3 as i32,
                    (*ps).tgt_buffer.pict,
                    &mut color,
                    &mut rect,
                    1 as i32,
                );
            }
            1 => {
                glx_dim_dst(
                    ps,
                    x,
                    y,
                    wid,
                    hei,
                    ((*(*ps).psglx).z as libc::c_double - 0.7f64) as libc::c_float,
                    dim_opacity as GLfloat,
                    reg_paint,
                    pcache_reg,
                );
            }
            _ => {}
        }
    }
    glx_mark_(
        ps,
        (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"win_paint_win\0")).as_ptr(),
        (*w).id,
        0 as i32 != 0,
    );
}
unsafe extern "C" fn rebuild_screen_reg(mut ps: *mut session_t) {
    if (*ps).screen_reg != 0 {
        XFixesDestroyRegion((*ps).dpy, (*ps).screen_reg);
    }
    (*ps).screen_reg = get_screen_region(ps);
}
unsafe extern "C" fn rebuild_shadow_exclude_reg(mut ps: *mut session_t) {
    free_region(ps, &mut (*ps).shadow_exclude_reg);
    let mut rect: XRectangle = geom_to_rect(
        ps,
        &mut (*ps).o.shadow_exclude_reg_geom,
        0 as *const XRectangle,
    );
    (*ps).shadow_exclude_reg = rect_to_reg(ps, &mut rect);
}
unsafe extern "C" fn paint_all(
    mut ps: *mut session_t,
    mut region: XserverRegion,
    mut region_real: XserverRegion,
    mut t: *mut win,
) {
    if region_real == 0 {
        region_real = region;
    }
    let mut reg_paint: XserverRegion = 0 as i64 as XserverRegion;
    let mut reg_tmp: XserverRegion = 0 as i64 as XserverRegion;
    let mut reg_tmp2: XserverRegion = 0 as i64 as XserverRegion;
    if bkend_use_glx(ps) {
        glx_paint_pre(ps, &mut region);
    }
    if region == 0 {
        region = get_screen_region(ps);
        region_real = region;
    } else {
        XFixesIntersectRegion((*ps).dpy, region, region, (*ps).screen_reg);
    }
    if !paint_isvalid(ps, &mut (*ps).tgt_buffer) {
        if backend::BKEND_XRENDER as i32 as u32 == (*ps).o.backend as u32
            && (*ps).o.dbe as i32 != 0
        {
            (*ps).tgt_buffer.pict = XRenderCreatePicture(
                (*ps).dpy,
                (*ps).root_dbe,
                XRenderFindVisualFormat((*ps).dpy, (*ps).vis),
                0 as i32 as u64,
                0 as *const XRenderPictureAttributes,
            );
        } else {
            if (*ps).tgt_buffer.pixmap == 0 {
                free_paint(ps, &mut (*ps).tgt_buffer);
                (*ps).tgt_buffer.pixmap = XCreatePixmap(
                    (*ps).dpy,
                    (*ps).root,
                    (*ps).root_width as u32,
                    (*ps).root_height as u32,
                    (*ps).depth as u32,
                );
            }
            if backend::BKEND_GLX as i32 as u32 != (*ps).o.backend as u32 {
                (*ps).tgt_buffer.pict = XRenderCreatePicture(
                    (*ps).dpy,
                    (*ps).tgt_buffer.pixmap,
                    XRenderFindVisualFormat((*ps).dpy, (*ps).vis),
                    0 as i32 as u64,
                    0 as *const XRenderPictureAttributes,
                );
            }
        }
    }
    if backend::BKEND_XRENDER as i32 as u32 == (*ps).o.backend as u32 {
        XFixesSetPictureClipRegion(
            (*ps).dpy,
            (*ps).tgt_picture,
            0 as i32,
            0 as i32,
            region_real,
        );
    }
    if !t.is_null() && (*t).reg_ignore != 0 {
        reg_tmp = XFixesCreateRegion((*ps).dpy, 0 as *mut XRectangle, 0 as i32);
        reg_paint = reg_tmp;
        XFixesSubtractRegion((*ps).dpy, reg_paint, region, (*t).reg_ignore);
    } else {
        reg_paint = region;
    }
    set_tgt_clip(ps, reg_paint, 0 as *const reg_data_t);
    paint_root(ps, reg_paint);
    if reg_tmp == 0 {
        reg_tmp = XFixesCreateRegion((*ps).dpy, 0 as *mut XRectangle, 0 as i32);
    }
    reg_tmp2 = XFixesCreateRegion((*ps).dpy, 0 as *mut XRectangle, 0 as i32);
    let mut w: *mut win = t;
    while !w.is_null() {
        if (*w).shadow {
            if (*w).shadow_paint.pixmap == 0 {
                win_build_shadow(ps, w, 1 as i32 as libc::c_double);
            }
            if (*w).reg_ignore != 0 {
                if w == t {
                    reg_paint = reg_tmp;
                    XFixesSubtractRegion((*ps).dpy, reg_paint, region, (*w).reg_ignore);
                } else {
                    reg_paint = reg_tmp2;
                }
                XFixesIntersectRegion((*ps).dpy, reg_paint, reg_paint, (*w).extents);
            } else {
                reg_paint = reg_tmp;
                XFixesIntersectRegion((*ps).dpy, reg_paint, region, (*w).extents);
            }
            if (*ps).shadow_exclude_reg != 0 {
                XFixesSubtractRegion(
                    (*ps).dpy,
                    reg_paint,
                    reg_paint,
                    (*ps).shadow_exclude_reg,
                );
            }
            let mut rec_shadow_border: XRectangle = {
                let mut init = XRectangle {
                    x: ((*w).a.x + (*w).shadow_dx) as libc::c_short,
                    y: ((*w).a.y + (*w).shadow_dy) as libc::c_short,
                    width: (*w).shadow_width as libc::c_ushort,
                    height: (*w).shadow_height as libc::c_ushort,
                };
                init
            };
            let mut reg_shadow: XserverRegion = XFixesCreateRegion(
                (*ps).dpy,
                &mut rec_shadow_border,
                1 as i32,
            );
            XFixesIntersectRegion((*ps).dpy, reg_paint, reg_paint, reg_shadow);
            free_region(ps, &mut reg_shadow);
            if (*ps).o.clear_shadow as i32 != 0 && (*w).border_size != 0 {
                XFixesSubtractRegion((*ps).dpy, reg_paint, reg_paint, (*w).border_size);
            }
            if (*ps).o.xinerama_shadow_crop as i32 != 0 && (*w).xinerama_scr >= 0 as i32
            {
                XFixesIntersectRegion(
                    (*ps).dpy,
                    reg_paint,
                    reg_paint,
                    *((*ps).xinerama_scr_regs).offset((*w).xinerama_scr as isize),
                );
            }
            let mut cache_reg: reg_data_t = {
                let mut init = reg_data_t {
                    rects: 0 as *mut XRectangle,
                    nrects: 0 as i32,
                };
                init
            };
            if region == reg_paint || !is_region_empty(ps, reg_paint, &mut cache_reg) {
                set_tgt_clip(ps, reg_paint, &mut cache_reg);
                win_paint_shadow(ps, w, reg_paint, &mut cache_reg);
            }
            free_reg_data(&mut cache_reg);
        }
        reg_paint = reg_tmp;
        if !((*w).prev_trans).is_null() && (*(*w).prev_trans).reg_ignore != 0 {
            XFixesSubtractRegion(
                (*ps).dpy,
                reg_paint,
                region,
                (*(*w).prev_trans).reg_ignore,
            );
            XFixesCopyRegion((*ps).dpy, reg_tmp2, reg_paint);
            if (*w).border_size != 0 {
                XFixesIntersectRegion((*ps).dpy, reg_paint, reg_paint, (*w).border_size);
            }
        } else if (*w).border_size != 0 {
            XFixesIntersectRegion((*ps).dpy, reg_paint, region, (*w).border_size);
        } else {
            reg_paint = region;
        }
        let mut cache_reg_0: reg_data_t = {
            let mut init = reg_data_t {
                rects: 0 as *mut XRectangle,
                nrects: 0 as i32,
            };
            init
        };
        if !is_region_empty(ps, reg_paint, &mut cache_reg_0) {
            set_tgt_clip(ps, reg_paint, &mut cache_reg_0);
            if (*w).blur_background as i32 != 0
                && (!win_is_solid(ps, w)
                    || (*ps).o.blur_background_frame as i32 != 0
                        && (*w).frame_opacity != 0.)
            {
                win_blur_background(
                    ps,
                    w,
                    (*ps).tgt_buffer.pict,
                    reg_paint,
                    &mut cache_reg_0,
                );
            }
            win_paint_win(ps, w, reg_paint, &mut cache_reg_0);
        }
        free_reg_data(&mut cache_reg_0);
        w = (*w).prev_trans;
    }
    XFixesDestroyRegion((*ps).dpy, reg_tmp);
    XFixesDestroyRegion((*ps).dpy, reg_tmp2);
    if !(*ps).o.dbe {
        set_tgt_clip(ps, 0 as i64 as XserverRegion, 0 as *const reg_data_t);
    }
    if (*ps).o.vsync as u64 != 0 {
        XSync((*ps).dpy, 0 as i32);
        if glx_has_context(ps) {
            if (*ps).o.vsync_use_glfinish {
                glFinish();
            } else {
                glFlush();
            }
            glXWaitX();
        }
    }
    if !(*ps).o.vsync_aggressive {
        vsync_wait(ps);
    }
    let mut current_block_126: u64;
    match (*ps).o.backend as u32 {
        0 => {
            if (*ps).o.dbe {
                let mut swap_info: XdbeSwapInfo = {
                    let mut init = XdbeSwapInfo {
                        swap_window: get_tgt_window(ps),
                        swap_action: 3 as i32 as XdbeSwapAction,
                    };
                    init
                };
                XdbeSwapBuffers((*ps).dpy, &mut swap_info, 1 as i32);
            } else if (*ps).tgt_buffer.pict != (*ps).tgt_picture {
                XRenderComposite(
                    (*ps).dpy,
                    1 as i32,
                    (*ps).tgt_buffer.pict,
                    0 as i64 as Picture,
                    (*ps).tgt_picture,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    (*ps).root_width as u32,
                    (*ps).root_height as u32,
                );
            }
            current_block_126 = 6950488995570599823;
        }
        2 => {
            XSync((*ps).dpy, 0 as i32);
            if (*ps).o.vsync_use_glfinish {
                glFinish();
            } else {
                glFlush();
            }
            glXWaitX();
            xr_sync_(ps, (*ps).tgt_buffer.pixmap, &mut (*ps).tgt_buffer_fence);
            paint_bind_tex_real(
                ps,
                &mut (*ps).tgt_buffer,
                (*ps).root_width as u32,
                (*ps).root_height as u32,
                (*ps).depth as u32,
                !(*ps).o.glx_no_rebind_pixmap,
            );
            xr_sync_(ps, (*ps).tgt_buffer.pixmap, &mut (*ps).tgt_buffer_fence);
            if (*ps).o.vsync_use_glfinish {
                glFinish();
            } else {
                glFlush();
            }
            glXWaitX();
            glx_render_(
                ps,
                (*ps).tgt_buffer.ptex,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                (*ps).root_width,
                (*ps).root_height,
                0 as i32,
                1.0f64,
                0 as i32 != 0,
                0 as i32 != 0,
                region_real,
                0 as *const reg_data_t,
                0 as *const glx_prog_main_t,
            );
            current_block_126 = 14404468346626157646;
        }
        1 => {
            current_block_126 = 14404468346626157646;
        }
        _ => {
            current_block_126 = 6950488995570599823;
        }
    }
    match current_block_126 {
        14404468346626157646 => {
            if (*ps).o.glx_use_copysubbuffermesa {
                glx_swap_copysubbuffermesa(ps, region_real);
            } else {
                glXSwapBuffers((*ps).dpy, get_tgt_window(ps));
            }
        }
        _ => {}
    }
    glx_mark_frame(ps);
    if (*ps).o.vsync_aggressive {
        vsync_wait(ps);
    }
    XFlush((*ps).dpy);
    if glx_has_context(ps) {
        glFlush();
        glXWaitX();
    }
    XFixesDestroyRegion((*ps).dpy, region);
    let mut pprev: *mut win = 0 as *mut win;
    let mut w_0: *mut win = t;
    while !w_0.is_null() {
        pprev = (*w_0).prev_trans;
        check_fade_fin(ps, w_0);
        w_0 = pprev;
    }
}
unsafe extern "C" fn add_damage(mut ps: *mut session_t, mut damage: XserverRegion) {
    if !(*ps).redirected {
        free_region(ps, &mut damage);
    }
    if damage == 0 {
        return;
    }
    if (*ps).all_damage != 0 {
        XFixesUnionRegion((*ps).dpy, (*ps).all_damage, (*ps).all_damage, damage);
        XFixesDestroyRegion((*ps).dpy, damage);
    } else {
        (*ps).all_damage = damage;
    };
}
unsafe extern "C" fn repair_win(mut ps: *mut session_t, mut w: *mut win) {
    if 2 as i32 != (*w).a.map_state {
        return;
    }
    let mut parts: XserverRegion = 0;
    if !(*w).damaged {
        parts = win_extents(ps, w);
        set_ignore_next(ps);
        XDamageSubtract(
            (*ps).dpy,
            (*w).damage,
            0 as i64 as XserverRegion,
            0 as i64 as XserverRegion,
        );
    } else {
        parts = XFixesCreateRegion((*ps).dpy, 0 as *mut XRectangle, 0 as i32);
        set_ignore_next(ps);
        XDamageSubtract((*ps).dpy, (*w).damage, 0 as i64 as XserverRegion, parts);
        XFixesTranslateRegion(
            (*ps).dpy,
            parts,
            (*w).a.x + (*w).a.border_width,
            (*w).a.y + (*w).a.border_width,
        );
    }
    (*w).damaged = 1 as i32 != 0;
    (*w).pixmap_damaged = 1 as i32 != 0;
    if !(*ps).redirected {
        free_region(ps, &mut parts);
        return;
    }
    if !(*ps).reg_ignore_expire && !((*w).prev_trans).is_null()
        && (*(*w).prev_trans).reg_ignore != 0
    {
        XFixesSubtractRegion((*ps).dpy, parts, parts, (*(*w).prev_trans).reg_ignore);
    }
    add_damage(ps, parts);
}
unsafe extern "C" fn wid_get_prop_wintype(
    mut ps: *mut session_t,
    mut wid: Window,
) -> wintype_t {
    set_ignore_next(ps);
    let mut prop: winprop_t = wid_get_prop(
        ps,
        wid,
        (*ps).atom_win_type,
        32 as i64,
        4 as i32 as Atom,
        32 as i32,
    );
    let mut i: u32 = 0 as i32 as u32;
    while (i as u64) < prop.nitems {
        let mut j: wintype_t = wintype_t::WINTYPE_DESKTOP;
        while (j as u32) < wintype_t::NUM_WINTYPES as i32 as u32 {
            if (*ps).atoms_wintypes[j as usize]
                == *(prop.data.p32).offset(i as isize) as Atom
            {
                free_winprop(&mut prop);
                return j;
            }
            j += 1;
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    free_winprop(&mut prop);
    return wintype_t::WINTYPE_UNKNOWN;
}
unsafe extern "C" fn map_win(mut ps: *mut session_t, mut id: Window) {
    if (*ps).overlay != 0 && id == (*ps).overlay && !(*ps).redirected {
        XUnmapWindow((*ps).dpy, (*ps).overlay);
        XFlush((*ps).dpy);
    }
    let mut w: *mut win = find_win(ps, id);
    if w.is_null() || 2 as i32 == (*w).a.class || 2 as i32 == (*w).a.map_state {
        return;
    }
    (*w).a.map_state = 2 as i32;
    cxinerama_win_upd_scr(ps, w);
    XSelectInput(
        (*ps).dpy,
        id,
        determine_evmask(ps, id, win_evmode_t::WIN_EVMODE_FRAME),
    );
    if (*ps).shape_exists {
        XShapeSelectInput((*ps).dpy, id, ((1 as i64) << 0 as i32) as u64);
    }
    XFlush((*ps).dpy);
    win_determine_mode(ps, w);
    if (*w).client_win == 0 {
        win_recheck_client(ps, w);
    } else {
        win_mark_client(ps, w, (*w).client_win);
    }
    win_update_shape_raw(ps, w);
    if (*ps).o.track_focus {
        recheck_focus(ps);
    }
    win_update_focused(ps, w);
    win_update_opacity_prop(ps, w);
    (*w).flags |= 0x4 as i32 as i64;
    if (*ps).o.respect_prop_shadow {
        win_update_prop_shadow_raw(ps, w);
    }
    win_determine_shadow(ps, w);
    (*w).in_openclose = 1 as i32 != 0;
    set_fade_callback(
        ps,
        w,
        Some(finish_map_win as unsafe extern "C" fn(*mut session_t, *mut win) -> ()),
        1 as i32 != 0,
    );
    win_determine_fade(ps, w);
    win_determine_blur_background(ps, w);
    (*w).damaged = 0 as i32 != 0;
    if (*w).need_configure {
        configure_win(ps, &mut (*w).queue_configure);
    }
    if (*ps).o.dbus {
        cdbus_ev_win_mapped(ps, w);
    }
}
unsafe extern "C" fn finish_map_win(mut ps: *mut session_t, mut w: *mut win) {
    (*w).in_openclose = 0 as i32 != 0;
    if (*ps).o.no_fading_openclose {
        win_determine_fade(ps, w);
    }
}
unsafe extern "C" fn finish_unmap_win(mut ps: *mut session_t, mut w: *mut win) {
    (*w).damaged = 0 as i32 != 0;
    (*w).in_openclose = 0 as i32 != 0;
    update_reg_ignore_expire(ps, w);
    if (*w).extents != 0 as i64 as u64 {
        add_damage(ps, (*w).extents);
        (*w).extents = 0 as i64 as XserverRegion;
    }
    free_wpaint(ps, w);
    free_region(ps, &mut (*w).border_size);
    free_paint(ps, &mut (*w).shadow_paint);
}
unsafe extern "C" fn unmap_callback(mut ps: *mut session_t, mut w: *mut win) {
    finish_unmap_win(ps, w);
}
unsafe extern "C" fn unmap_win(mut ps: *mut session_t, mut w: *mut win) {
    if w.is_null() || 0 as i32 == (*w).a.map_state {
        return;
    }
    if (*w).paint.pixmap != 0 {
        xr_sync_(ps, (*w).paint.pixmap, &mut (*w).fence);
    }
    free_fence(ps, &mut (*w).fence);
    win_set_focused(ps, w, 0 as i32 != 0);
    (*w).a.map_state = 0 as i32;
    (*w).flags |= 0x4 as i32 as i64;
    set_fade_callback(
        ps,
        w,
        Some(unmap_callback as unsafe extern "C" fn(*mut session_t, *mut win) -> ()),
        0 as i32 != 0,
    );
    (*w).in_openclose = 1 as i32 != 0;
    win_determine_fade(ps, w);
    if (*w).fade {
        win_validate_pixmap(ps, w);
    }
    win_ev_stop(ps, w);
    if (*ps).o.dbus {
        cdbus_ev_win_unmapped(ps, w);
    }
}
unsafe extern "C" fn wid_get_opacity_prop(
    mut ps: *mut session_t,
    mut wid: Window,
    mut def: opacity_t,
) -> opacity_t {
    let mut val: opacity_t = def;
    let mut prop: winprop_t = wid_get_prop(
        ps,
        wid,
        (*ps).atom_opacity,
        1 as i64,
        6 as i32 as Atom,
        32 as i32,
    );
    if prop.nitems != 0 {
        val = *prop.data.p32 as opacity_t;
    }
    free_winprop(&mut prop);
    return val;
}
unsafe extern "C" fn get_opacity_percent(mut w: *mut win) -> libc::c_double {
    return (*w).opacity as libc::c_double / 0xffffffff as u32 as libc::c_double;
}
unsafe extern "C" fn win_determine_mode(mut ps: *mut session_t, mut w: *mut win) {
    let mut mode: winmode_t = winmode_t::WMODE_SOLID;
    if !((*w).pictfmt).is_null() && (*(*w).pictfmt).type_0 == 1 as i32
        && (*(*w).pictfmt).direct.alphaMask as i32 != 0
    {
        mode = winmode_t::WMODE_ARGB;
    } else if (*w).opacity != 0xffffffff as u32 {
        mode = winmode_t::WMODE_TRANS;
    } else {
        mode = winmode_t::WMODE_SOLID;
    }
    (*w).mode = mode;
}
unsafe extern "C" fn calc_opacity(mut ps: *mut session_t, mut w: *mut win) {
    let mut opacity: opacity_t = 0xffffffff as u32;
    if (*w).destroyed as i32 != 0 || 2 as i32 != (*w).a.map_state {
        opacity = 0 as i32 as opacity_t;
    } else {
        opacity = (*w).opacity_prop;
        if 0xffffffff as u32 == opacity
            && {
                opacity = (*w).opacity_prop_client;
                0xffffffff as u32 == opacity
            }
        {
            opacity = ((*ps).o.wintype_opacity[(*w).window_type as usize]
                * 0xffffffff as u32 as libc::c_double) as opacity_t;
        }
        if (*ps).o.inactive_opacity != 0 && 0 as i32 == (*w).focused as i32
            && (0xffffffff as u32 == opacity
                || (*ps).o.inactive_opacity_override as i32 != 0)
        {
            opacity = (*ps).o.inactive_opacity;
        }
        if 0xffffffff as u32 == opacity && (*ps).o.active_opacity != 0
            && win_is_focused_real(ps, w) as i32 != 0
        {
            opacity = (*ps).o.active_opacity;
        }
    }
    (*w).opacity_tgt = opacity;
}
unsafe extern "C" fn calc_dim(mut ps: *mut session_t, mut w: *mut win) {
    let mut dim: bool = false;
    if (*w).destroyed as i32 != 0 || 2 as i32 != (*w).a.map_state {
        return;
    }
    if (*ps).o.inactive_dim != 0. && !(*w).focused {
        dim = 1 as i32 != 0;
    } else {
        dim = 0 as i32 != 0;
    }
    if dim as i32 != (*w).dim as i32 {
        (*w).dim = dim;
        add_damage_win(ps, w);
    }
}
unsafe extern "C" fn win_determine_fade(mut ps: *mut session_t, mut w: *mut win) {
    if switch_t::UNSET as i32 as u32 != (*w).fade_force as u32 {
        (*w).fade = (*w).fade_force as u64 != 0;
        (*w).fade_last = (*w).fade;
    } else if (*ps).o.no_fading_openclose as i32 != 0 && (*w).in_openclose as i32 != 0 {
        (*w).fade = 0 as i32 != 0;
        (*w).fade_last = (*w).fade;
    } else if (*ps).o.no_fading_destroyed_argb as i32 != 0 && (*w).destroyed as i32 != 0
        && winmode_t::WMODE_ARGB as i32 as u32 == (*w).mode as u32
        && (*w).client_win != 0 && (*w).client_win != (*w).id
    {
        (*w).fade = 0 as i32 != 0;
        (*w).fade_last = (*w).fade;
    } else if !(2 as i32 != (*w).a.map_state) {
        if win_match(ps, w, (*ps).o.fade_blacklist, &mut (*w).cache_fblst) {
            (*w).fade = 0 as i32 != 0;
        } else {
            (*w).fade = (*ps).o.wintype_fade[(*w).window_type as usize];
        }
    }
}
unsafe extern "C" fn win_update_shape_raw(mut ps: *mut session_t, mut w: *mut win) {
    if (*ps).shape_exists {
        (*w).bounding_shaped = wid_bounding_shaped(ps, (*w).id);
        if (*w).bounding_shaped as i32 != 0 && (*ps).o.detect_rounded_corners as i32 != 0
        {
            win_rounded_corners(ps, w);
        }
    }
}
unsafe extern "C" fn win_update_shape(mut ps: *mut session_t, mut w: *mut win) {
    if (*ps).shape_exists {
        win_update_shape_raw(ps, w);
        win_on_factor_change(ps, w);
    }
}
unsafe extern "C" fn win_update_prop_shadow_raw(
    mut ps: *mut session_t,
    mut w: *mut win,
) {
    let mut prop: winprop_t = wid_get_prop(
        ps,
        (*w).id,
        (*ps).atom_compton_shadow,
        1 as i32 as i64,
        6 as i32 as Atom,
        32 as i32,
    );
    if prop.nitems == 0 {
        (*w).prop_shadow = -(1 as i32) as i64;
    } else {
        (*w).prop_shadow = *prop.data.p32;
    }
    free_winprop(&mut prop);
}
unsafe extern "C" fn win_update_prop_shadow(mut ps: *mut session_t, mut w: *mut win) {
    let mut attr_shadow_old: i64 = (*w).prop_shadow;
    win_update_prop_shadow_raw(ps, w);
    if (*w).prop_shadow != attr_shadow_old {
        win_determine_shadow(ps, w);
    }
}
unsafe extern "C" fn win_set_shadow(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut shadow_new: bool,
) {
    if (*w).shadow as i32 == shadow_new as i32 {
        return;
    }
    (*w).shadow = shadow_new;
    if (*w).extents != 0 {
        if !(*w).shadow {
            add_damage(ps, (*w).extents);
        } else {
            free_region(ps, &mut (*w).extents);
        }
        (*w).extents = win_extents(ps, w);
        if (*w).shadow {
            add_damage_win(ps, w);
        }
    }
}
unsafe extern "C" fn win_determine_shadow(mut ps: *mut session_t, mut w: *mut win) {
    let mut shadow_new: bool = (*w).shadow;
    if switch_t::UNSET as i32 as u32 != (*w).shadow_force as u32 {
        shadow_new = (*w).shadow_force as u64 != 0;
    } else if 2 as i32 == (*w).a.map_state {
        shadow_new = (*ps).o.wintype_shadow[(*w).window_type as usize] as i32 != 0
            && !win_match(ps, w, (*ps).o.shadow_blacklist, &mut (*w).cache_sblst)
            && !((*ps).o.shadow_ignore_shaped as i32 != 0
                && (*w).bounding_shaped as i32 != 0 && !(*w).rounded_corners)
            && !((*ps).o.respect_prop_shadow as i32 != 0
                && 0 as i32 as i64 == (*w).prop_shadow);
    }
    win_set_shadow(ps, w, shadow_new);
}
unsafe extern "C" fn win_set_invert_color(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut invert_color_new: bool,
) {
    if (*w).invert_color as i32 == invert_color_new as i32 {
        return;
    }
    (*w).invert_color = invert_color_new;
    add_damage_win(ps, w);
}
unsafe extern "C" fn win_determine_invert_color(
    mut ps: *mut session_t,
    mut w: *mut win,
) {
    let mut invert_color_new: bool = (*w).invert_color;
    if switch_t::UNSET as i32 as u32 != (*w).invert_color_force as u32 {
        invert_color_new = (*w).invert_color_force as u64 != 0;
    } else if 2 as i32 == (*w).a.map_state {
        invert_color_new = win_match(
            ps,
            w,
            (*ps).o.invert_color_list,
            &mut (*w).cache_ivclst,
        );
    }
    win_set_invert_color(ps, w, invert_color_new);
}
unsafe extern "C" fn win_set_blur_background(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut blur_background_new: bool,
) {
    if (*w).blur_background as i32 == blur_background_new as i32 {
        return;
    }
    (*w).blur_background = blur_background_new;
    if !win_is_solid(ps, w)
        || (*ps).o.blur_background_frame as i32 != 0 && (*w).frame_opacity != 0.
    {
        add_damage_win(ps, w);
    }
}
unsafe extern "C" fn win_determine_blur_background(
    mut ps: *mut session_t,
    mut w: *mut win,
) {
    if 2 as i32 != (*w).a.map_state {
        return;
    }
    let mut blur_background_new: bool = (*ps).o.blur_background as i32 != 0
        && !win_match(ps, w, (*ps).o.blur_background_blacklist, &mut (*w).cache_bbblst);
    win_set_blur_background(ps, w, blur_background_new);
}
unsafe extern "C" fn win_update_opacity_rule(mut ps: *mut session_t, mut w: *mut win) {
    if 2 as i32 != (*w).a.map_state {
        return;
    }
    let mut opacity: opacity_t = 0xffffffff as u32;
    let mut val: *mut libc::c_void = 0 as *mut libc::c_void;
    if c2_matchd(ps, w, (*ps).o.opacity_rules, &mut (*w).cache_oparule, &mut val) {
        opacity = (val as i64 as libc::c_double / 100.0f64
            * 0xffffffff as u32 as libc::c_double) as opacity_t;
    }
    if opacity == (*w).opacity_set {
        return;
    }
    if 0xffffffff as u32 != opacity {
        wid_set_opacity_prop(ps, (*w).id, opacity);
    } else if 0xffffffff as u32 != (*w).opacity_set {
        wid_rm_opacity_prop(ps, (*w).id);
    }
    (*w).opacity_set = opacity;
}
unsafe extern "C" fn win_on_wtype_change(mut ps: *mut session_t, mut w: *mut win) {
    win_determine_shadow(ps, w);
    win_determine_fade(ps, w);
    win_update_focused(ps, w);
    if !((*ps).o.invert_color_list).is_null() {
        win_determine_invert_color(ps, w);
    }
    if !((*ps).o.opacity_rules).is_null() {
        win_update_opacity_rule(ps, w);
    }
}
unsafe extern "C" fn win_on_factor_change(mut ps: *mut session_t, mut w: *mut win) {
    if !((*ps).o.shadow_blacklist).is_null() {
        win_determine_shadow(ps, w);
    }
    if !((*ps).o.fade_blacklist).is_null() {
        win_determine_fade(ps, w);
    }
    if !((*ps).o.invert_color_list).is_null() {
        win_determine_invert_color(ps, w);
    }
    if !((*ps).o.focus_blacklist).is_null() {
        win_update_focused(ps, w);
    }
    if !((*ps).o.blur_background_blacklist).is_null() {
        win_determine_blur_background(ps, w);
    }
    if !((*ps).o.opacity_rules).is_null() {
        win_update_opacity_rule(ps, w);
    }
    if 2 as i32 == (*w).a.map_state && !((*ps).o.paint_blacklist).is_null() {
        (*w).paint_excluded = win_match(
            ps,
            w,
            (*ps).o.paint_blacklist,
            &mut (*w).cache_pblst,
        );
    }
    if 2 as i32 == (*w).a.map_state && !((*ps).o.unredir_if_possible_blacklist).is_null()
    {
        (*w).unredir_if_possible_excluded = win_match(
            ps,
            w,
            (*ps).o.unredir_if_possible_blacklist,
            &mut (*w).cache_uipblst,
        );
    }
}
unsafe extern "C" fn calc_win_size(mut ps: *mut session_t, mut w: *mut win) {
    (*w).widthb = (*w).a.width + (*w).a.border_width * 2 as i32;
    (*w).heightb = (*w).a.height + (*w).a.border_width * 2 as i32;
    calc_shadow_geometry(ps, w);
    (*w).flags |= 0x1 as i32 as i64;
}
unsafe extern "C" fn calc_shadow_geometry(mut ps: *mut session_t, mut w: *mut win) {
    (*w).shadow_dx = (*ps).o.shadow_offset_x;
    (*w).shadow_dy = (*ps).o.shadow_offset_y;
    (*w).shadow_width = (*w).widthb + (*(*ps).gaussian_map).size;
    (*w).shadow_height = (*w).heightb + (*(*ps).gaussian_map).size;
}
unsafe extern "C" fn win_upd_wintype(mut ps: *mut session_t, mut w: *mut win) {
    let wtype_old: wintype_t = (*w).window_type;
    (*w).window_type = wid_get_prop_wintype(ps, (*w).client_win);
    if wintype_t::WINTYPE_UNKNOWN as i32 as u32 == (*w).window_type as u32 {
        if (*w).a.override_redirect != 0
            || !wid_has_prop(ps, (*w).client_win, (*ps).atom_transient)
        {
            (*w).window_type = wintype_t::WINTYPE_NORMAL;
        } else {
            (*w).window_type = wintype_t::WINTYPE_DIALOG;
        }
    }
    if (*w).window_type as u32 != wtype_old as u32 {
        win_on_wtype_change(ps, w);
    }
}
unsafe extern "C" fn win_mark_client(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut client: Window,
) {
    (*w).client_win = client;
    if 2 as i32 != (*w).a.map_state {
        return;
    }
    XSelectInput(
        (*ps).dpy,
        client,
        determine_evmask(ps, client, win_evmode_t::WIN_EVMODE_CLIENT),
    );
    XFlush((*ps).dpy);
    win_upd_wintype(ps, w);
    if (*ps).o.frame_opacity != 0. {
        get_frame_extents(ps, w, client);
    }
    if (*ps).o.track_leader {
        win_update_leader(ps, w);
    }
    if (*ps).o.track_wdata {
        win_get_name(ps, w);
        win_get_class(ps, w);
        win_get_role(ps, w);
    }
    win_on_factor_change(ps, w);
    win_update_focused(ps, w);
}
unsafe extern "C" fn win_unmark_client(mut ps: *mut session_t, mut w: *mut win) {
    let mut client: Window = (*w).client_win;
    (*w).client_win = 0 as i64 as Window;
    XSelectInput(
        (*ps).dpy,
        client,
        determine_evmask(ps, client, win_evmode_t::WIN_EVMODE_UNKNOWN),
    );
}
unsafe extern "C" fn win_recheck_client(mut ps: *mut session_t, mut w: *mut win) {
    (*w).wmwin = 0 as i32 != 0;
    let mut cw: Window = find_client_win(ps, (*w).id);
    if cw == 0 {
        cw = (*w).id;
        (*w).wmwin = (*w).a.override_redirect == 0;
    }
    if (*w).client_win != 0 && (*w).client_win != cw {
        win_unmark_client(ps, w);
    }
    win_mark_client(ps, w, cw);
}
unsafe extern "C" fn add_win(
    mut ps: *mut session_t,
    mut id: Window,
    mut prev: Window,
) -> bool {
    static mut win_def: win = {
        let mut init = _win {
            next: 0 as *const _win as *mut _win,
            prev_trans: 0 as *const _win as *mut _win,
            id: 0 as i64 as Window,
            a: {
                let mut init = XWindowAttributes {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                    border_width: 0,
                    depth: 0,
                    visual: 0 as *const Visual as *mut Visual,
                    root: 0,
                    class: 0,
                    bit_gravity: 0,
                    win_gravity: 0,
                    backing_store: 0,
                    backing_planes: 0,
                    backing_pixel: 0,
                    save_under: 0,
                    colormap: 0,
                    map_installed: 0,
                    map_state: 0,
                    all_event_masks: 0,
                    your_event_mask: 0,
                    do_not_propagate_mask: 0,
                    override_redirect: 0,
                    screen: 0 as *const Screen as *mut Screen,
                };
                init
            },
            xinerama_scr: -(1 as i32),
            pictfmt: 0 as *const XRenderPictFormat as *mut XRenderPictFormat,
            mode: winmode_t::WMODE_TRANS,
            damaged: 0 as i32 != 0,
            fence: 0,
            pixmap_damaged: 0 as i32 != 0,
            damage: 0 as i64 as Damage,
            paint: {
                let mut init = paint_t {
                    pixmap: 0 as i64 as Pixmap,
                    pict: 0 as i64 as Picture,
                    ptex: 0 as *const glx_texture_t as *mut glx_texture_t,
                };
                init
            },
            border_size: 0 as i64 as XserverRegion,
            extents: 0 as i64 as XserverRegion,
            flags: 0 as i32 as int_fast16_t,
            need_configure: 0 as i32 != 0,
            queue_configure: {
                let mut init = XConfigureEvent {
                    type_0: 0,
                    serial: 0,
                    send_event: 0,
                    display: 0 as *const Display as *mut Display,
                    event: 0,
                    window: 0,
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                    border_width: 0,
                    above: 0,
                    override_redirect: 0,
                };
                init
            },
            reg_ignore: 0 as i64 as XserverRegion,
            widthb: 0 as i32,
            heightb: 0 as i32,
            destroyed: 0 as i32 != 0,
            bounding_shaped: 0 as i32 != 0,
            rounded_corners: 0 as i32 != 0,
            to_paint: 0 as i32 != 0,
            paint_excluded: false,
            unredir_if_possible_excluded: false,
            in_openclose: 0 as i32 != 0,
            client_win: 0 as i64 as Window,
            window_type: wintype_t::WINTYPE_UNKNOWN,
            wmwin: 0 as i32 != 0,
            leader: 0 as i64 as Window,
            cache_leader: 0 as i64 as Window,
            focused: 0 as i32 != 0,
            focused_force: switch_t::UNSET,
            name: 0 as *const i8 as *mut i8,
            class_instance: 0 as *const i8 as *mut i8,
            class_general: 0 as *const i8 as *mut i8,
            role: 0 as *const i8 as *mut i8,
            cache_sblst: 0 as *const c2_lptr_t,
            cache_fblst: 0 as *const c2_lptr_t,
            cache_fcblst: 0 as *const c2_lptr_t,
            cache_ivclst: 0 as *const c2_lptr_t,
            cache_bbblst: 0 as *const c2_lptr_t,
            cache_oparule: 0 as *const c2_lptr_t,
            cache_pblst: 0 as *const c2_lptr_t,
            cache_uipblst: 0 as *const c2_lptr_t,
            opacity: 0 as i32 as opacity_t,
            opacity_tgt: 0 as i32 as opacity_t,
            opacity_prop: 0xffffffff as u32,
            opacity_prop_client: 0xffffffff as u32,
            opacity_set: 0xffffffff as u32,
            fade: 0 as i32 != 0,
            fade_last: false,
            fade_force: switch_t::UNSET,
            fade_callback: None,
            frame_opacity: 0.0f64,
            frame_extents: {
                let mut init = margin_t {
                    top: 0 as i32,
                    left: 0 as i32,
                    bottom: 0 as i32,
                    right: 0 as i32,
                };
                init
            },
            shadow: 0 as i32 != 0,
            shadow_last: false,
            shadow_force: switch_t::UNSET,
            shadow_opacity: 0.0f64,
            shadow_dx: 0 as i32,
            shadow_dy: 0 as i32,
            shadow_width: 0 as i32,
            shadow_height: 0 as i32,
            shadow_paint: {
                let mut init = paint_t {
                    pixmap: 0 as i64 as Pixmap,
                    pict: 0 as i64 as Picture,
                    ptex: 0 as *const glx_texture_t as *mut glx_texture_t,
                };
                init
            },
            prop_shadow: -(1 as i32) as i64,
            dim: 0 as i32 != 0,
            invert_color: 0 as i32 != 0,
            invert_color_last: false,
            invert_color_force: switch_t::UNSET,
            blur_background: 0 as i32 != 0,
            blur_background_last: false,
            glx_blur_cache: glx_blur_cache_t {
                fbo: 0,
                textures: [0; 2],
                width: 0,
                height: 0,
            },
        };
        init
    };
    if id == (*ps).overlay || !(find_win(ps, id)).is_null() {
        return 0 as i32 != 0;
    }
    let mut new: *mut win = malloc(::core::mem::size_of::<win>() as u64) as *mut win;
    if new.is_null() {
        fprintf(
            stderr,
            b"%s(%#010lx): Failed to allocate memory for the new window.\n\0"
                as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 8], &[i8; 8]>(b"add_win\0")).as_ptr(),
            id,
        );
        return 0 as i32 != 0;
    }
    memcpy(
        new as *mut libc::c_void,
        &win_def as *const win as *const libc::c_void,
        ::core::mem::size_of::<win>() as u64,
    );
    let mut p: *mut *mut win = 0 as *mut *mut win;
    if prev != 0 {
        p = &mut (*ps).list;
        while !(*p).is_null() {
            if (**p).id == prev && !(**p).destroyed {
                break;
            }
            p = &mut (**p).next;
        }
    } else {
        p = &mut (*ps).list;
    }
    (*new).id = id;
    set_ignore_next(ps);
    if XGetWindowAttributes((*ps).dpy, id, &mut (*new).a) == 0
        || 1 as i32 == (*new).a.map_state
    {
        free(new as *mut libc::c_void);
        return 0 as i32 != 0;
    }
    let mut map_state: i32 = (*new).a.map_state;
    (*new).a.map_state = 0 as i32;
    if 1 as i32 == (*new).a.class {
        (*new).pictfmt = XRenderFindVisualFormat((*ps).dpy, (*new).a.visual);
        set_ignore_next(ps);
        (*new).damage = XDamageCreate((*ps).dpy, id, 3 as i32);
    }
    calc_win_size(ps, new);
    (*new).next = *p;
    *p = new;
    if (*ps).o.dbus {
        cdbus_ev_win_added(ps, new);
    }
    if 2 as i32 == map_state {
        map_win(ps, id);
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn restack_win(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut new_above: Window,
) {
    let mut old_above: Window = 0;
    update_reg_ignore_expire(ps, w);
    if !((*w).next).is_null() {
        old_above = (*(*w).next).id;
    } else {
        old_above = 0 as i64 as Window;
    }
    if old_above != new_above {
        let mut prev: *mut *mut win = 0 as *mut *mut win;
        let mut prev_old: *mut *mut win = 0 as *mut *mut win;
        prev = &mut (*ps).list;
        while !(*prev).is_null() {
            if *prev == w {
                break;
            }
            prev = &mut (**prev).next;
        }
        prev_old = prev;
        let mut found: bool = 0 as i32 != 0;
        prev = &mut (*ps).list;
        while !(*prev).is_null() {
            if (**prev).id == new_above && !(**prev).destroyed {
                found = 1 as i32 != 0;
                break;
            } else {
                prev = &mut (**prev).next;
            }
        }
        if new_above != 0 && !found {
            fprintf(
                stderr,
                b"%s(%#010lx, %#010lx): Failed to found new above window.\n\0"
                    as *const u8 as *const i8,
                (*::core::mem::transmute::<&[u8; 12], &[i8; 12]>(b"restack_win\0"))
                    .as_ptr(),
                (*w).id,
                new_above,
            );
            return;
        }
        *prev_old = (*w).next;
        (*w).next = *prev;
        *prev = w;
    }
}
unsafe extern "C" fn configure_win(
    mut ps: *mut session_t,
    mut ce: *mut XConfigureEvent,
) {
    if (*ce).window == (*ps).root {
        free_paint(ps, &mut (*ps).tgt_buffer);
        (*ps).root_width = (*ce).width;
        (*ps).root_height = (*ce).height;
        rebuild_screen_reg(ps);
        rebuild_shadow_exclude_reg(ps);
        free_all_damage_last(ps);
        if (*ps).o.reredir_on_root_change as i32 != 0 && (*ps).redirected as i32 != 0 {
            redir_stop(ps);
            redir_start(ps);
        }
        if (*ps).o.glx_reinit_on_root_change as i32 != 0 && !((*ps).psglx).is_null() {
            if !glx_reinit(ps, bkend_use_glx(ps)) {
                fprintf(
                    stderr,
                    b"%s(): Failed to reinitialize GLX, troubles ahead.\n\0" as *const u8
                        as *const i8,
                    (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"configure_win\0"))
                        .as_ptr(),
                );
            }
            if backend::BKEND_GLX as i32 as u32 == (*ps).o.backend as u32
                && !init_filters(ps)
            {
                fprintf(
                    stderr,
                    b"%s(): Failed to initialize filters.\n\0" as *const u8 as *const i8,
                    (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"configure_win\0"))
                        .as_ptr(),
                );
            }
        }
        if backend::BKEND_GLX as i32 as u32 == (*ps).o.backend as u32 {
            glx_on_root_change(ps);
        }
        force_repaint(ps);
        return;
    }
    let mut w: *mut win = find_win(ps, (*ce).window);
    let mut damage: XserverRegion = 0 as i64 as XserverRegion;
    if w.is_null() {
        return;
    }
    if (*w).a.map_state == 0 as i32 {
        (*w).need_configure = 1 as i32 != 0;
        (*w).queue_configure = *ce;
        restack_win(ps, w, (*ce).above);
    } else {
        if !(*w).need_configure {
            restack_win(ps, w, (*ce).above);
        }
        let mut factor_change: bool = 0 as i32 != 0;
        (*ps).reg_ignore_expire = 1 as i32 != 0;
        (*w).need_configure = 0 as i32 != 0;
        damage = XFixesCreateRegion((*ps).dpy, 0 as *mut XRectangle, 0 as i32);
        if (*w).extents != 0 as i64 as u64 {
            XFixesCopyRegion((*ps).dpy, damage, (*w).extents);
        }
        if (*w).a.x != (*ce).x || (*w).a.y != (*ce).y || (*w).a.width != (*ce).width
            || (*w).a.height != (*ce).height || (*w).a.border_width != (*ce).border_width
        {
            factor_change = 1 as i32 != 0;
            free_region(ps, &mut (*w).extents);
            free_region(ps, &mut (*w).border_size);
        }
        (*w).a.x = (*ce).x;
        (*w).a.y = (*ce).y;
        if (*w).a.width != (*ce).width || (*w).a.height != (*ce).height
            || (*w).a.border_width != (*ce).border_width
        {
            free_wpaint(ps, w);
        }
        if (*w).a.width != (*ce).width || (*w).a.height != (*ce).height
            || (*w).a.border_width != (*ce).border_width
        {
            (*w).a.width = (*ce).width;
            (*w).a.height = (*ce).height;
            (*w).a.border_width = (*ce).border_width;
            calc_win_size(ps, w);
            if (*ps).shape_exists as i32 != 0 && (*ps).o.shadow_ignore_shaped as i32 != 0
                && (*ps).o.detect_rounded_corners as i32 != 0
                && (*w).bounding_shaped as i32 != 0
            {
                win_update_shape(ps, w);
            }
        }
        if damage != 0 {
            let mut extents: XserverRegion = win_extents(ps, w);
            XFixesUnionRegion((*ps).dpy, damage, damage, extents);
            XFixesDestroyRegion((*ps).dpy, extents);
            add_damage(ps, damage);
        }
        if factor_change {
            cxinerama_win_upd_scr(ps, w);
            win_on_factor_change(ps, w);
        }
    }
    (*w).a.override_redirect = (*ce).override_redirect;
}
unsafe extern "C" fn circulate_win(
    mut ps: *mut session_t,
    mut ce: *mut XCirculateEvent,
) {
    let mut w: *mut win = find_win(ps, (*ce).window);
    let mut new_above: Window = 0;
    if w.is_null() {
        return;
    }
    if (*ce).place == 0 as i32 {
        new_above = (*(*ps).list).id;
    } else {
        new_above = 0 as i64 as Window;
    }
    restack_win(ps, w, new_above);
}
unsafe extern "C" fn finish_destroy_win(mut ps: *mut session_t, mut id: Window) {
    let mut prev: *mut *mut win = 0 as *mut *mut win;
    let mut w: *mut win = 0 as *mut win;
    prev = &mut (*ps).list;
    loop {
        w = *prev;
        if w.is_null() {
            break;
        }
        if (*w).id == id && (*w).destroyed as i32 != 0 {
            finish_unmap_win(ps, w);
            *prev = (*w).next;
            if w == (*ps).active_win {
                (*ps).active_win = 0 as *mut _win;
            }
            free_win_res(ps, w);
            let mut w2: *mut win = (*ps).list;
            while !w2.is_null() {
                if w == (*w2).prev_trans {
                    (*w2).prev_trans = 0 as *mut _win;
                }
                w2 = (*w2).next;
            }
            free(w as *mut libc::c_void);
            break;
        } else {
            prev = &mut (*w).next;
        }
    };
}
unsafe extern "C" fn destroy_callback(mut ps: *mut session_t, mut w: *mut win) {
    finish_destroy_win(ps, (*w).id);
}
unsafe extern "C" fn destroy_win(mut ps: *mut session_t, mut id: Window) {
    let mut w: *mut win = find_win(ps, id);
    if !w.is_null() {
        unmap_win(ps, w);
        (*w).destroyed = 1 as i32 != 0;
        if (*ps).o.no_fading_destroyed_argb {
            win_determine_fade(ps, w);
        }
        set_fade_callback(
            ps,
            w,
            Some(
                destroy_callback as unsafe extern "C" fn(*mut session_t, *mut win) -> (),
            ),
            0 as i32 != 0,
        );
        if (*ps).o.dbus {
            cdbus_ev_win_destroyed(ps, w);
        }
    }
}
#[inline]
unsafe extern "C" fn root_damaged(mut ps: *mut session_t) {
    if (*ps).root_tile_paint.pixmap != 0 {
        XClearArea(
            (*ps).dpy,
            (*ps).root,
            0 as i32,
            0 as i32,
            0 as i32 as u32,
            0 as i32 as u32,
            1 as i32,
        );
        free_root_tile(ps);
    }
    force_repaint(ps);
}
unsafe extern "C" fn damage_win(
    mut ps: *mut session_t,
    mut de: *mut XDamageNotifyEvent,
) {
    let mut w: *mut win = find_win(ps, (*de).drawable);
    if w.is_null() {
        return;
    }
    repair_win(ps, w);
}
unsafe extern "C" fn xerror(mut dpy: *mut Display, mut ev: *mut XErrorEvent) -> i32 {
    let ps: *mut session_t = ps_g;
    let mut o: i32 = 0 as i32;
    let mut name: *const i8 = b"Unknown\0" as *const u8 as *const i8;
    if should_ignore(ps, (*ev).serial) != 0 {
        return 0 as i32;
    }
    if (*ev).request_code as i32 == (*ps).composite_opcode
        && (*ev).minor_code as i32 == 2 as i32
    {
        fprintf(
            stderr,
            b"Another composite manager is already running\n\0" as *const u8 as *const i8,
        );
        exit(1 as i32);
    }
    o = (*ev).error_code as i32 - (*ps).xfixes_error;
    match o {
        0 => {
            name = b"BadRegion\0" as *const u8 as *const i8;
        }
        _ => {}
    }
    o = (*ev).error_code as i32 - (*ps).damage_error;
    match o {
        0 => {
            name = b"BadDamage\0" as *const u8 as *const i8;
        }
        _ => {}
    }
    o = (*ev).error_code as i32 - (*ps).render_error;
    match o {
        0 => {
            name = b"BadPictFormat\0" as *const u8 as *const i8;
        }
        1 => {
            name = b"BadPicture\0" as *const u8 as *const i8;
        }
        2 => {
            name = b"BadPictOp\0" as *const u8 as *const i8;
        }
        3 => {
            name = b"BadGlyphSet\0" as *const u8 as *const i8;
        }
        4 => {
            name = b"BadGlyph\0" as *const u8 as *const i8;
        }
        _ => {}
    }
    if (*ps).glx_exists {
        o = (*ev).error_code as i32 - (*ps).glx_error;
        match o {
            1 => {
                name = b"GLX_BAD_SCREEN\0" as *const u8 as *const i8;
            }
            2 => {
                name = b"GLX_BAD_ATTRIBUTE\0" as *const u8 as *const i8;
            }
            3 => {
                name = b"GLX_NO_EXTENSION\0" as *const u8 as *const i8;
            }
            4 => {
                name = b"GLX_BAD_VISUAL\0" as *const u8 as *const i8;
            }
            5 => {
                name = b"GLX_BAD_CONTEXT\0" as *const u8 as *const i8;
            }
            6 => {
                name = b"GLX_BAD_VALUE\0" as *const u8 as *const i8;
            }
            7 => {
                name = b"GLX_BAD_ENUM\0" as *const u8 as *const i8;
            }
            _ => {}
        }
    }
    if (*ps).xsync_exists {
        o = (*ev).error_code as i32 - (*ps).xsync_error;
        match o {
            0 => {
                name = b"XSyncBadCounter\0" as *const u8 as *const i8;
            }
            1 => {
                name = b"XSyncBadAlarm\0" as *const u8 as *const i8;
            }
            2 => {
                name = b"XSyncBadFence\0" as *const u8 as *const i8;
            }
            _ => {}
        }
    }
    match (*ev).error_code as i32 {
        10 => {
            name = b"BadAccess\0" as *const u8 as *const i8;
        }
        11 => {
            name = b"BadAlloc\0" as *const u8 as *const i8;
        }
        5 => {
            name = b"BadAtom\0" as *const u8 as *const i8;
        }
        12 => {
            name = b"BadColor\0" as *const u8 as *const i8;
        }
        6 => {
            name = b"BadCursor\0" as *const u8 as *const i8;
        }
        9 => {
            name = b"BadDrawable\0" as *const u8 as *const i8;
        }
        7 => {
            name = b"BadFont\0" as *const u8 as *const i8;
        }
        13 => {
            name = b"BadGC\0" as *const u8 as *const i8;
        }
        14 => {
            name = b"BadIDChoice\0" as *const u8 as *const i8;
        }
        17 => {
            name = b"BadImplementation\0" as *const u8 as *const i8;
        }
        16 => {
            name = b"BadLength\0" as *const u8 as *const i8;
        }
        8 => {
            name = b"BadMatch\0" as *const u8 as *const i8;
        }
        15 => {
            name = b"BadName\0" as *const u8 as *const i8;
        }
        4 => {
            name = b"BadPixmap\0" as *const u8 as *const i8;
        }
        1 => {
            name = b"BadRequest\0" as *const u8 as *const i8;
        }
        2 => {
            name = b"BadValue\0" as *const u8 as *const i8;
        }
        3 => {
            name = b"BadWindow\0" as *const u8 as *const i8;
        }
        _ => {}
    }
    print_timestamp(ps);
    let mut buf: [i8; 80] = *::core::mem::transmute::<
        &[u8; 80],
        &mut [i8; 80],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    XGetErrorText((*ps).dpy, (*ev).error_code as i32, buf.as_mut_ptr(), 80 as i32);
    printf(
        b"error %4d %-12s request %4d minor %4d serial %6lu: \"%s\"\n\0" as *const u8
            as *const i8,
        (*ev).error_code as i32,
        name,
        (*ev).request_code as i32,
        (*ev).minor_code as i32,
        (*ev).serial,
        buf.as_mut_ptr(),
    );
    return 0 as i32;
}
unsafe extern "C" fn expose_root(
    mut ps: *mut session_t,
    mut rects: *mut XRectangle,
    mut nrects: i32,
) {
    free_all_damage_last(ps);
    let mut region: XserverRegion = XFixesCreateRegion((*ps).dpy, rects, nrects);
    add_damage(ps, region);
}
unsafe extern "C" fn wid_get_prop_window(
    mut ps: *mut session_t,
    mut wid: Window,
    mut aprop: Atom,
) -> Window {
    let mut p: Window = 0 as i64 as Window;
    let mut prop: winprop_t = wid_get_prop(
        ps,
        wid,
        aprop,
        1 as i64,
        33 as i32 as Atom,
        32 as i32,
    );
    if prop.nitems != 0 {
        p = *prop.data.p32 as Window;
    }
    free_winprop(&mut prop);
    return p;
}
unsafe extern "C" fn win_update_focused(mut ps: *mut session_t, mut w: *mut win) {
    let mut focused_old: bool = (*w).focused;
    if switch_t::UNSET as i32 as u32 != (*w).focused_force as u32 {
        (*w).focused = (*w).focused_force as u64 != 0;
    } else {
        (*w).focused = win_is_focused_real(ps, w);
        if (*ps).o.wintype_focus[(*w).window_type as usize] as i32 != 0
            || (*ps).o.mark_wmwin_focused as i32 != 0 && (*w).wmwin as i32 != 0
            || (*ps).o.mark_ovredir_focused as i32 != 0 && (*w).id == (*w).client_win
                && !(*w).wmwin
            || 2 as i32 == (*w).a.map_state
                && win_match(ps, w, (*ps).o.focus_blacklist, &mut (*w).cache_fcblst)
                    as i32 != 0
        {
            (*w).focused = 1 as i32 != 0;
        }
        if (*ps).o.track_leader as i32 != 0 && (*ps).active_leader != 0
            && win_get_leader(ps, w) == (*ps).active_leader
        {
            (*w).focused = 1 as i32 != 0;
        }
    }
    (*w).flags |= 0x4 as i32 as i64;
}
unsafe extern "C" fn win_set_focused(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut focused: bool,
) {
    if 0 as i32 == (*w).a.map_state {
        return;
    }
    if win_is_focused_real(ps, w) as i32 == focused as i32 {
        return;
    }
    if focused {
        if !((*ps).active_win).is_null() {
            win_set_focused(ps, (*ps).active_win, 0 as i32 != 0);
        }
        (*ps).active_win = w;
    } else if w == (*ps).active_win {
        (*ps).active_win = 0 as *mut _win;
    }
    win_on_focus_change(ps, w);
}
unsafe extern "C" fn win_on_focus_change(mut ps: *mut session_t, mut w: *mut win) {
    if (*ps).o.track_leader {
        let mut leader: Window = win_get_leader(ps, w);
        if win_is_focused_real(ps, w) as i32 != 0 && leader != (*ps).active_leader {
            let mut active_leader_old: Window = (*ps).active_leader;
            (*ps).active_leader = leader;
            group_update_focused(ps, active_leader_old);
            group_update_focused(ps, leader);
        } else if !win_is_focused_real(ps, w) && leader != 0
            && leader == (*ps).active_leader && !group_is_focused(ps, leader)
        {
            (*ps).active_leader = 0 as i64 as Window;
            group_update_focused(ps, leader);
        }
        win_update_focused(ps, w);
    } else {
        win_update_focused(ps, w);
    }
    win_on_factor_change(ps, w);
    if (*ps).o.dbus {
        if win_is_focused_real(ps, w) {
            cdbus_ev_win_focusin(ps, w);
        } else {
            cdbus_ev_win_focusout(ps, w);
        }
    }
}
unsafe extern "C" fn win_update_leader(mut ps: *mut session_t, mut w: *mut win) {
    let mut leader: Window = 0 as i64 as Window;
    if (*ps).o.detect_transient as i32 != 0 && leader == 0 {
        leader = wid_get_prop_window(ps, (*w).client_win, (*ps).atom_transient);
    }
    if (*ps).o.detect_client_leader as i32 != 0 && leader == 0 {
        leader = wid_get_prop_window(ps, (*w).client_win, (*ps).atom_client_leader);
    }
    win_set_leader(ps, w, leader);
}
unsafe extern "C" fn win_set_leader(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut nleader: Window,
) {
    if (*w).leader != nleader {
        let mut cache_leader_old: Window = win_get_leader(ps, w);
        (*w).leader = nleader;
        clear_cache_win_leaders(ps);
        let mut cache_leader: Window = win_get_leader(ps, w);
        if win_is_focused_real(ps, w) as i32 != 0 && cache_leader_old != cache_leader {
            (*ps).active_leader = cache_leader;
            group_update_focused(ps, cache_leader_old);
            group_update_focused(ps, cache_leader);
        } else {
            win_update_focused(ps, w);
        }
        win_on_factor_change(ps, w);
    }
}
unsafe extern "C" fn win_get_leader_raw(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut recursions: i32,
) -> Window {
    if (*w).cache_leader == 0 && ((*w).client_win != 0 || (*w).leader != 0) {
        (*w).cache_leader = (*w).leader;
        if (*w).cache_leader == 0 {
            (*w).cache_leader = (*w).client_win;
        }
        if (*w).cache_leader != 0 && (*w).cache_leader != (*w).client_win {
            let mut wp: *mut win = find_toplevel(ps, (*w).cache_leader);
            if !wp.is_null() {
                if recursions > 20 as i32 {
                    return 0 as i64 as Window;
                }
                (*w).cache_leader = win_get_leader_raw(ps, wp, recursions + 1 as i32);
            }
        }
    }
    return (*w).cache_leader;
}
#[no_mangle]
pub unsafe extern "C" fn wid_get_text_prop(
    mut ps: *mut session_t,
    mut wid: Window,
    mut prop: Atom,
    mut pstrlst: *mut *mut *mut i8,
    mut pnstr: *mut i32,
) -> bool {
    let mut text_prop: XTextProperty = {
        let mut init = XTextProperty {
            value: 0 as *mut u8,
            encoding: 0 as i64 as Atom,
            format: 0 as i32,
            nitems: 0 as i32 as u64,
        };
        init
    };
    if !(XGetTextProperty((*ps).dpy, wid, &mut text_prop, prop) != 0
        && !(text_prop.value).is_null())
    {
        return 0 as i32 != 0;
    }
    if 0 as i32 != XmbTextPropertyToTextList((*ps).dpy, &mut text_prop, pstrlst, pnstr)
        || *pnstr == 0
    {
        *pnstr = 0 as i32;
        if !(*pstrlst).is_null() {
            XFreeStringList(*pstrlst);
        }
        cxfree(text_prop.value as *mut libc::c_void);
        return 0 as i32 != 0;
    }
    cxfree(text_prop.value as *mut libc::c_void);
    return 1 as i32 != 0;
}
unsafe extern "C" fn wid_get_name(
    mut ps: *mut session_t,
    mut wid: Window,
    mut name: *mut *mut i8,
) -> bool {
    let mut text_prop: XTextProperty = {
        let mut init = XTextProperty {
            value: 0 as *mut u8,
            encoding: 0 as i64 as Atom,
            format: 0 as i32,
            nitems: 0 as i32 as u64,
        };
        init
    };
    let mut strlst: *mut *mut i8 = 0 as *mut *mut i8;
    let mut nstr: i32 = 0 as i32;
    if !wid_get_text_prop(ps, wid, (*ps).atom_name_ewmh, &mut strlst, &mut nstr) {
        if !(XGetWMName((*ps).dpy, wid, &mut text_prop) != 0
            && !(text_prop.value).is_null())
        {
            return 0 as i32 != 0;
        }
        if 0 as i32
            != XmbTextPropertyToTextList(
                (*ps).dpy,
                &mut text_prop,
                &mut strlst,
                &mut nstr,
            ) || nstr == 0 || strlst.is_null()
        {
            if !strlst.is_null() {
                XFreeStringList(strlst);
            }
            cxfree(text_prop.value as *mut libc::c_void);
            return 0 as i32 != 0;
        }
        cxfree(text_prop.value as *mut libc::c_void);
    }
    *name = mstrcpy(*strlst.offset(0 as i32 as isize));
    XFreeStringList(strlst);
    return 1 as i32 != 0;
}
unsafe extern "C" fn wid_get_role(
    mut ps: *mut session_t,
    mut wid: Window,
    mut role: *mut *mut i8,
) -> bool {
    let mut strlst: *mut *mut i8 = 0 as *mut *mut i8;
    let mut nstr: i32 = 0 as i32;
    if !wid_get_text_prop(ps, wid, (*ps).atom_role, &mut strlst, &mut nstr) {
        return 0 as i32 != 0;
    }
    *role = mstrcpy(*strlst.offset(0 as i32 as isize));
    XFreeStringList(strlst);
    return 1 as i32 != 0;
}
unsafe extern "C" fn win_get_prop_str(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut tgt: *mut *mut i8,
    mut func_wid_get_prop_str: Option<
        unsafe extern "C" fn(*mut session_t, Window, *mut *mut i8) -> bool,
    >,
) -> i32 {
    let mut ret: i32 = -(1 as i32);
    let mut prop_old: *mut i8 = *tgt;
    if (*w).client_win == 0 {
        return 0 as i32;
    }
    ret = func_wid_get_prop_str
        .expect("non-null function pointer")(ps, (*w).client_win, tgt) as i32;
    if ret == 0 {
        ret = -(1 as i32);
    } else if !prop_old.is_null() && strcmp(*tgt, prop_old) == 0 {
        ret = 0 as i32;
    } else {
        ret = 1 as i32;
    }
    if *tgt != prop_old {
        free(prop_old as *mut libc::c_void);
    }
    return ret;
}
unsafe extern "C" fn win_get_class(mut ps: *mut session_t, mut w: *mut win) -> bool {
    let mut strlst: *mut *mut i8 = 0 as *mut *mut i8;
    let mut nstr: i32 = 0 as i32;
    if (*w).client_win == 0 {
        return 0 as i32 != 0;
    }
    free((*w).class_instance as *mut libc::c_void);
    free((*w).class_general as *mut libc::c_void);
    (*w).class_instance = 0 as *mut i8;
    (*w).class_general = 0 as *mut i8;
    if !wid_get_text_prop(
        ps,
        (*w).client_win,
        (*ps).atom_class,
        &mut strlst,
        &mut nstr,
    ) {
        return 0 as i32 != 0;
    }
    (*w).class_instance = mstrcpy(*strlst.offset(0 as i32 as isize));
    if nstr > 1 as i32 {
        (*w).class_general = mstrcpy(*strlst.offset(1 as i32 as isize));
    }
    XFreeStringList(strlst);
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn force_repaint(mut ps: *mut session_t) {
    let mut reg: XserverRegion = 0 as i64 as XserverRegion;
    if (*ps).screen_reg != 0
        && {
            reg = copy_region(ps, (*ps).screen_reg);
            reg != 0
        }
    {
        (*ps).ev_received = 1 as i32 != 0;
        add_damage(ps, reg);
    }
}
#[no_mangle]
pub unsafe extern "C" fn win_set_shadow_force(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut val: switch_t,
) {
    if val as u32 != (*w).shadow_force as u32 {
        (*w).shadow_force = val;
        win_determine_shadow(ps, w);
        (*ps).ev_received = 1 as i32 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn win_set_fade_force(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut val: switch_t,
) {
    if val as u32 != (*w).fade_force as u32 {
        (*w).fade_force = val;
        win_determine_fade(ps, w);
        (*ps).ev_received = 1 as i32 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn win_set_focused_force(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut val: switch_t,
) {
    if val as u32 != (*w).focused_force as u32 {
        (*w).focused_force = val;
        win_update_focused(ps, w);
        (*ps).ev_received = 1 as i32 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn win_set_invert_color_force(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut val: switch_t,
) {
    if val as u32 != (*w).invert_color_force as u32 {
        (*w).invert_color_force = val;
        win_determine_invert_color(ps, w);
        (*ps).ev_received = 1 as i32 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn opts_init_track_focus(mut ps: *mut session_t) {
    if (*ps).o.track_focus {
        return;
    }
    (*ps).o.track_focus = 1 as i32 != 0;
    if !(*ps).o.use_ewmh_active_win {
        let mut w: *mut win = (*ps).list;
        while !w.is_null() {
            if 2 as i32 == (*w).a.map_state {
                XSelectInput(
                    (*ps).dpy,
                    (*w).id,
                    determine_evmask(ps, (*w).id, win_evmode_t::WIN_EVMODE_FRAME),
                );
            }
            w = (*w).next;
        }
    }
    recheck_focus(ps);
}
#[no_mangle]
pub unsafe extern "C" fn opts_set_no_fading_openclose(
    mut ps: *mut session_t,
    mut newval: bool,
) {
    if newval as i32 != (*ps).o.no_fading_openclose as i32 {
        (*ps).o.no_fading_openclose = newval;
        let mut w: *mut win = (*ps).list;
        while !w.is_null() {
            win_determine_fade(ps, w);
            w = (*w).next;
        }
        (*ps).ev_received = 1 as i32 != 0;
    }
}
#[inline]
unsafe extern "C" fn ev_focus_in(
    mut ps: *mut session_t,
    mut ev: *mut XFocusChangeEvent,
) {
    recheck_focus(ps);
}
#[inline]
unsafe extern "C" fn ev_focus_out(
    mut ps: *mut session_t,
    mut ev: *mut XFocusChangeEvent,
) {
    recheck_focus(ps);
}
#[inline]
unsafe extern "C" fn ev_create_notify(
    mut ps: *mut session_t,
    mut ev: *mut XCreateWindowEvent,
) {
    add_win(ps, (*ev).window, 0 as i32 as Window);
}
#[inline]
unsafe extern "C" fn ev_configure_notify(
    mut ps: *mut session_t,
    mut ev: *mut XConfigureEvent,
) {
    configure_win(ps, ev);
}
#[inline]
unsafe extern "C" fn ev_destroy_notify(
    mut ps: *mut session_t,
    mut ev: *mut XDestroyWindowEvent,
) {
    destroy_win(ps, (*ev).window);
}
#[inline]
unsafe extern "C" fn ev_map_notify(mut ps: *mut session_t, mut ev: *mut XMapEvent) {
    map_win(ps, (*ev).window);
}
#[inline]
unsafe extern "C" fn ev_unmap_notify(mut ps: *mut session_t, mut ev: *mut XUnmapEvent) {
    let mut w: *mut win = find_win(ps, (*ev).window);
    if !w.is_null() {
        unmap_win(ps, w);
    }
}
#[inline]
unsafe extern "C" fn ev_reparent_notify(
    mut ps: *mut session_t,
    mut ev: *mut XReparentEvent,
) {
    if (*ev).parent == (*ps).root {
        add_win(ps, (*ev).window, 0 as i32 as Window);
    } else {
        destroy_win(ps, (*ev).window);
        XSelectInput(
            (*ps).dpy,
            (*ev).window,
            determine_evmask(ps, (*ev).window, win_evmode_t::WIN_EVMODE_UNKNOWN),
        );
        if (find_toplevel(ps, (*ev).window)).is_null() {
            let mut w_top: *mut win = find_toplevel2(ps, (*ev).parent);
            if !w_top.is_null()
                && ((*w_top).client_win == 0 || (*w_top).client_win == (*w_top).id)
            {
                if wid_has_prop(ps, (*ev).window, (*ps).atom_client) {
                    (*w_top).wmwin = 0 as i32 != 0;
                    win_unmark_client(ps, w_top);
                    win_mark_client(ps, w_top, (*ev).window);
                } else {
                    XSelectInput(
                        (*ps).dpy,
                        (*ev).window,
                        determine_evmask(
                            ps,
                            (*ev).window,
                            win_evmode_t::WIN_EVMODE_UNKNOWN,
                        ) | (1 as i64) << 22 as i32,
                    );
                }
            }
        }
    };
}
#[inline]
unsafe extern "C" fn ev_circulate_notify(
    mut ps: *mut session_t,
    mut ev: *mut XCirculateEvent,
) {
    circulate_win(ps, ev);
}
#[inline]
unsafe extern "C" fn ev_expose(mut ps: *mut session_t, mut ev: *mut XExposeEvent) {
    if (*ev).window == (*ps).root || (*ps).overlay != 0 && (*ev).window == (*ps).overlay
    {
        let mut more: i32 = (*ev).count + 1 as i32;
        if (*ps).n_expose == (*ps).size_expose {
            if !((*ps).expose_rects).is_null() {
                (*ps).expose_rects = realloc(
                    (*ps).expose_rects as *mut libc::c_void,
                    (((*ps).size_expose + more) as u64)
                        .wrapping_mul(::core::mem::size_of::<XRectangle>() as u64),
                ) as *mut XRectangle;
                (*ps).size_expose += more;
            } else {
                (*ps).expose_rects = malloc(
                    (more as u64)
                        .wrapping_mul(::core::mem::size_of::<XRectangle>() as u64),
                ) as *mut XRectangle;
                (*ps).size_expose = more;
            }
        }
        (*((*ps).expose_rects).offset((*ps).n_expose as isize)).x = (*ev).x
            as libc::c_short;
        (*((*ps).expose_rects).offset((*ps).n_expose as isize)).y = (*ev).y
            as libc::c_short;
        (*((*ps).expose_rects).offset((*ps).n_expose as isize)).width = (*ev).width
            as libc::c_ushort;
        (*((*ps).expose_rects).offset((*ps).n_expose as isize)).height = (*ev).height
            as libc::c_ushort;
        (*ps).n_expose += 1;
        (*ps).n_expose;
        if (*ev).count == 0 as i32 {
            expose_root(ps, (*ps).expose_rects, (*ps).n_expose);
            (*ps).n_expose = 0 as i32;
        }
    }
}
unsafe extern "C" fn update_ewmh_active_win(mut ps: *mut session_t) {
    let mut wid: Window = wid_get_prop_window(
        ps,
        (*ps).root,
        (*ps).atom_ewmh_active_win,
    );
    let mut w: *mut win = find_win_all(ps, wid);
    if !w.is_null() {
        win_set_focused(ps, w, 1 as i32 != 0);
    }
}
#[inline]
unsafe extern "C" fn ev_property_notify(
    mut ps: *mut session_t,
    mut ev: *mut XPropertyEvent,
) {
    if (*ps).root == (*ev).window {
        if (*ps).o.track_focus as i32 != 0 && (*ps).o.use_ewmh_active_win as i32 != 0
            && (*ps).atom_ewmh_active_win == (*ev).atom
        {
            update_ewmh_active_win(ps);
        } else {
            let mut p: i32 = 0 as i32;
            while !(background_props_str[p as usize]).is_null() {
                if (*ev).atom == get_atom(ps, background_props_str[p as usize]) {
                    root_damaged(ps);
                    break;
                } else {
                    p += 1;
                    p;
                }
            }
        }
        return;
    }
    if (*ev).atom == (*ps).atom_client {
        if (find_toplevel(ps, (*ev).window)).is_null() {
            XSelectInput(
                (*ps).dpy,
                (*ev).window,
                determine_evmask(ps, (*ev).window, win_evmode_t::WIN_EVMODE_UNKNOWN),
            );
            let mut w_top: *mut win = find_toplevel2(ps, (*ev).window);
            if !w_top.is_null()
                && ((*w_top).client_win == 0 || (*w_top).client_win == (*w_top).id)
                && wid_has_prop(ps, (*ev).window, (*ps).atom_client) as i32 != 0
            {
                (*w_top).wmwin = 0 as i32 != 0;
                win_unmark_client(ps, w_top);
                win_mark_client(ps, w_top, (*ev).window);
            }
        }
    }
    if (*ev).atom == (*ps).atom_win_type {
        let mut w: *mut win = 0 as *mut win;
        w = find_toplevel(ps, (*ev).window);
        if !w.is_null() {
            win_upd_wintype(ps, w);
        }
    }
    if (*ev).atom == (*ps).atom_opacity {
        let mut w_0: *mut win = 0 as *mut win;
        w_0 = find_win(ps, (*ev).window);
        if !w_0.is_null() {
            (*w_0).opacity_prop = wid_get_opacity_prop(ps, (*w_0).id, 0xffffffff as u32);
        } else if (*ps).o.detect_client_opacity as i32 != 0
            && {
                w_0 = find_toplevel(ps, (*ev).window);
                !w_0.is_null()
            }
        {
            (*w_0).opacity_prop_client = wid_get_opacity_prop(
                ps,
                (*w_0).client_win,
                0xffffffff as u32,
            );
        }
        if !w_0.is_null() {
            (*w_0).flags |= 0x4 as i32 as i64;
        }
    }
    if (*ps).o.frame_opacity != 0. && (*ev).atom == (*ps).atom_frame_extents {
        let mut w_1: *mut win = find_toplevel(ps, (*ev).window);
        if !w_1.is_null() {
            get_frame_extents(ps, w_1, (*ev).window);
            add_damage_win(ps, w_1);
        }
    }
    if (*ps).o.track_wdata as i32 != 0
        && ((*ps).atom_name == (*ev).atom || (*ps).atom_name_ewmh == (*ev).atom)
    {
        let mut w_2: *mut win = find_toplevel(ps, (*ev).window);
        if !w_2.is_null() && 1 as i32 == win_get_name(ps, w_2) {
            win_on_factor_change(ps, w_2);
        }
    }
    if (*ps).o.track_wdata as i32 != 0 && (*ps).atom_class == (*ev).atom {
        let mut w_3: *mut win = find_toplevel(ps, (*ev).window);
        if !w_3.is_null() {
            win_get_class(ps, w_3);
            win_on_factor_change(ps, w_3);
        }
    }
    if (*ps).o.track_wdata as i32 != 0 && (*ps).atom_role == (*ev).atom {
        let mut w_4: *mut win = find_toplevel(ps, (*ev).window);
        if !w_4.is_null() && 1 as i32 == win_get_role(ps, w_4) {
            win_on_factor_change(ps, w_4);
        }
    }
    if (*ps).o.respect_prop_shadow as i32 != 0 && (*ps).atom_compton_shadow == (*ev).atom
    {
        let mut w_5: *mut win = find_win(ps, (*ev).window);
        if !w_5.is_null() {
            win_update_prop_shadow(ps, w_5);
        }
    }
    if (*ps).o.detect_transient as i32 != 0 && (*ps).atom_transient == (*ev).atom
        || (*ps).o.detect_client_leader as i32 != 0
            && (*ps).atom_client_leader == (*ev).atom
    {
        let mut w_6: *mut win = find_toplevel(ps, (*ev).window);
        if !w_6.is_null() {
            win_update_leader(ps, w_6);
        }
    }
    let mut platom: *mut latom_t = (*ps).track_atom_lst;
    while !platom.is_null() {
        if (*platom).atom == (*ev).atom {
            let mut w_7: *mut win = find_win(ps, (*ev).window);
            if w_7.is_null() {
                w_7 = find_toplevel(ps, (*ev).window);
            }
            if !w_7.is_null() {
                win_on_factor_change(ps, w_7);
            }
            break;
        } else {
            platom = (*platom).next;
        }
    }
}
#[inline]
unsafe extern "C" fn ev_damage_notify(
    mut ps: *mut session_t,
    mut ev: *mut XDamageNotifyEvent,
) {
    damage_win(ps, ev);
}
#[inline]
unsafe extern "C" fn ev_shape_notify(mut ps: *mut session_t, mut ev: *mut XShapeEvent) {
    let mut w: *mut win = find_win(ps, (*ev).window);
    if w.is_null() || 0 as i32 == (*w).a.map_state {
        return;
    }
    if (*w).border_size != 0 {
        add_damage(ps, (*w).border_size);
        (*w).border_size = border_size(ps, w, 1 as i32 != 0);
        add_damage(ps, copy_region(ps, (*w).border_size));
    }
    win_update_shape(ps, w);
    update_reg_ignore_expire(ps, w);
}
unsafe extern "C" fn ev_screen_change_notify(
    mut ps: *mut session_t,
    mut ev: *mut XRRScreenChangeNotifyEvent,
) {
    if (*ps).o.xinerama_shadow_crop {
        cxinerama_upd_scrs(ps);
    }
    if (*ps).o.sw_opti as i32 != 0 && (*ps).o.refresh_rate == 0 {
        update_refresh_rate(ps);
        if (*ps).refresh_rate == 0 {
            fprintf(
                stderr,
                b"ev_screen_change_notify(): Refresh rate detection failed, --sw-opti disabled.\0"
                    as *const u8 as *const i8,
            );
            (*ps).o.sw_opti = 0 as i32 != 0;
        }
    }
}
unsafe extern "C" fn ev_handle(mut ps: *mut session_t, mut ev: *mut XEvent) {
    if (*ev).type_0 & 0x7f as i32 != 11 as i32 {
        discard_ignore(ps, (*ev).xany.serial);
    }
    match (*ev).type_0 {
        9 => {
            ev_focus_in(ps, ev as *mut XFocusChangeEvent);
        }
        10 => {
            ev_focus_out(ps, ev as *mut XFocusChangeEvent);
        }
        16 => {
            ev_create_notify(ps, ev as *mut XCreateWindowEvent);
        }
        22 => {
            ev_configure_notify(ps, ev as *mut XConfigureEvent);
        }
        17 => {
            ev_destroy_notify(ps, ev as *mut XDestroyWindowEvent);
        }
        19 => {
            ev_map_notify(ps, ev as *mut XMapEvent);
        }
        18 => {
            ev_unmap_notify(ps, ev as *mut XUnmapEvent);
        }
        21 => {
            ev_reparent_notify(ps, ev as *mut XReparentEvent);
        }
        26 => {
            ev_circulate_notify(ps, ev as *mut XCirculateEvent);
        }
        12 => {
            ev_expose(ps, ev as *mut XExposeEvent);
        }
        28 => {
            ev_property_notify(ps, ev as *mut XPropertyEvent);
        }
        _ => {
            if (*ps).shape_exists as i32 != 0 && (*ev).type_0 == (*ps).shape_event {
                ev_shape_notify(ps, ev as *mut XShapeEvent);
            } else if (*ps).randr_exists as i32 != 0
                && (*ev).type_0 == (*ps).randr_event + 0 as i32
            {
                ev_screen_change_notify(ps, ev as *mut XRRScreenChangeNotifyEvent);
            } else if isdamagenotify(ps, ev) {
                ev_damage_notify(ps, ev as *mut XDamageNotifyEvent);
            }
        }
    };
}
unsafe extern "C" fn usage(mut ret: i32) -> ! {
    static mut usage_text: *const i8 = b"compton (git-62f8c2b8-dirty-2025-05-05)\nusage: compton [options]\nOptions:\n\n-d display\n  Which display should be managed.\n\n-r radius\n  The blur radius for shadows. (default 12)\n\n-o opacity\n  The translucency for shadows. (default .75)\n\n-l left-offset\n  The left offset for shadows. (default -15)\n\n-t top-offset\n  The top offset for shadows. (default -15)\n\n-I fade-in-step\n  Opacity change between steps while fading in. (default 0.028)\n\n-O fade-out-step\n  Opacity change between steps while fading out. (default 0.03)\n\n-D fade-delta-time\n  The time between steps in a fade in milliseconds. (default 10)\n\n-m opacity\n  The opacity for menus. (default 1.0)\n\n-c\n  Enabled client-side shadows on windows.\n\n-C\n  Avoid drawing shadows on dock/panel windows.\n\n-z\n  Zero the part of the shadow's mask behind the window.\n\n-f\n  Fade windows in/out when opening/closing and when opacity\n  changes, unless --no-fading-openclose is used.\n\n-F\n  Equals to -f. Deprecated.\n\n-i opacity\n  Opacity of inactive windows. (0.1 - 1.0)\n\n-e opacity\n  Opacity of window titlebars and borders. (0.1 - 1.0)\n\n-G\n  Don't draw shadows on DND windows\n\n-b\n  Daemonize process.\n\n-S\n  Enable synchronous operation (for debugging).\n\n--show-all-xerrors\n  Show all X errors (for debugging).\n\n--config path\n  Look for configuration file at the path. Use /dev/null to avoid\n  loading configuration file.\n\n--write-pid-path path\n  Write process ID to a file.\n\n--shadow-red value\n  Red color value of shadow (0.0 - 1.0, defaults to 0).\n\n--shadow-green value\n  Green color value of shadow (0.0 - 1.0, defaults to 0).\n\n--shadow-blue value\n  Blue color value of shadow (0.0 - 1.0, defaults to 0).\n\n--inactive-opacity-override\n  Inactive opacity set by -i overrides value of _NET_WM_OPACITY.\n\n--inactive-dim value\n  Dim inactive windows. (0.0 - 1.0, defaults to 0)\n\n--active-opacity opacity\n  Default opacity for active windows. (0.0 - 1.0)\n\n--mark-wmwin-focused\n  Try to detect WM windows and mark them as active.\n\n--shadow-exclude condition\n  Exclude conditions for shadows.\n\n--fade-exclude condition\n  Exclude conditions for fading.\n\n--mark-ovredir-focused\n  Mark windows that have no WM frame as active.\n\n--no-fading-openclose\n  Do not fade on window open/close.\n\n--no-fading-destroyed-argb\n  Do not fade destroyed ARGB windows with WM frame. Workaround of bugs\n  in Openbox, Fluxbox, etc.\n\n--shadow-ignore-shaped\n  Do not paint shadows on shaped windows. (Deprecated, use\n  --shadow-exclude 'bounding_shaped' or\n  --shadow-exclude 'bounding_shaped && !rounded_corners' instead.)\n\n--detect-rounded-corners\n  Try to detect windows with rounded corners and don't consider\n  them shaped windows. Affects --shadow-ignore-shaped,\n  --unredir-if-possible, and possibly others. You need to turn this\n  on manually if you want to match against rounded_corners in\n  conditions.\n\n--detect-client-opacity\n  Detect _NET_WM_OPACITY on client windows, useful for window\n  managers not passing _NET_WM_OPACITY of client windows to frame\n  windows.\n\n--refresh-rate val\n  Specify refresh rate of the screen. If not specified or 0, compton\n  will try detecting this with X RandR extension.\n\n--vsync vsync-method\n  Set VSync method. There are (up to) 5 VSync methods currently\n  available:\n    none = No VSync\n    drm = VSync with DRM_IOCTL_WAIT_VBLANK. May only work on some\n      (DRI-based) drivers.\n    opengl = Try to VSync with SGI_video_sync OpenGL extension. Only\n      work on some drivers.\n    opengl-oml = Try to VSync with OML_sync_control OpenGL extension.\n      Only work on some drivers.\n    opengl-swc = Try to VSync with SGI_swap_control OpenGL extension.\n      Only work on some drivers. Works only with GLX backend.\n    opengl-mswc = Try to VSync with MESA_swap_control OpenGL\n      extension. Basically the same as opengl-swc above, except the\n      extension we use.\n\n--vsync-aggressive\n  Attempt to send painting request before VBlank and do XFlush()\n  during VBlank. This switch may be lifted out at any moment.\n\n--alpha-step val\n  X Render backend: Step for pregenerating alpha pictures. \n  0.01 - 1.0. Defaults to 0.03.\n\n--dbe\n  Enable DBE painting mode, intended to use with VSync to\n  (hopefully) eliminate tearing.\n\n--paint-on-overlay\n  Painting on X Composite overlay window.\n\n--sw-opti\n  Limit compton to repaint at most once every 1 / refresh_rate\n  second to boost performance.\n\n--use-ewmh-active-win\n  Use _NET_WM_ACTIVE_WINDOW on the root window to determine which\n  window is focused instead of using FocusIn/Out events.\n\n--respect-prop-shadow\n  Respect _COMPTON_SHADOW. This a prototype-level feature, which\n  you must not rely on.\n\n--unredir-if-possible\n  Unredirect all windows if a full-screen opaque window is\n  detected, to maximize performance for full-screen windows.\n\n--unredir-if-possible-delay ms\n  Delay before unredirecting the window, in milliseconds.\n  Defaults to 0.\n\n--unredir-if-possible-exclude condition\n  Conditions of windows that shouldn't be considered full-screen\n  for unredirecting screen.\n\n--focus-exclude condition\n  Specify a list of conditions of windows that should always be\n  considered focused.\n\n--inactive-dim-fixed\n  Use fixed inactive dim value.\n\n--detect-transient\n  Use WM_TRANSIENT_FOR to group windows, and consider windows in\n  the same group focused at the same time.\n\n--detect-client-leader\n  Use WM_CLIENT_LEADER to group windows, and consider windows in\n  the same group focused at the same time. WM_TRANSIENT_FOR has\n  higher priority if --detect-transient is enabled, too.\n\n--blur-background\n  Blur background of semi-transparent / ARGB windows. Bad in\n  performance. The switch name may change without prior\n  notifications.\n\n--blur-background-frame\n  Blur background of windows when the window frame is not opaque.\n  Implies --blur-background. Bad in performance. The switch name\n  may change.\n\n--blur-background-fixed\n  Use fixed blur strength instead of adjusting according to window\n  opacity.\n\n--blur-kern matrix\n  Specify the blur convolution kernel, with the following format:\n    WIDTH,HEIGHT,ELE1,ELE2,ELE3,ELE4,ELE5...\n  The element in the center must not be included, it will be forever\n  1.0 or changing based on opacity, depending on whether you have\n  --blur-background-fixed.\n  A 7x7 Gaussian blur kernel looks like:\n    --blur-kern '7,7,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,0.000102,0.003494,0.029143,0.059106,0.029143,0.003494,0.000102,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.001723,0.059106,0.493069,0.493069,0.059106,0.001723,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.000102,0.003494,0.029143,0.059106,0.029143,0.003494,0.000102,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003'\n  Up to 4 blur kernels may be specified, separated with semicolon, for\n  multi-pass blur.\n  May also be one the predefined kernels: 3x3box (default), 5x5box,\n  7x7box, 3x3gaussian, 5x5gaussian, 7x7gaussian, 9x9gaussian,\n  11x11gaussian.\n\n--blur-background-exclude condition\n  Exclude conditions for background blur.\n\n--resize-damage integer\n  Resize damaged region by a specific number of pixels. A positive\n  value enlarges it while a negative one shrinks it. Useful for\n  fixing the line corruption issues of blur. May or may not\n  work with --glx-no-stencil. Shrinking doesn't function correctly.\n\n--invert-color-include condition\n  Specify a list of conditions of windows that should be painted with\n  inverted color. Resource-hogging, and is not well tested.\n\n--opacity-rule opacity:condition\n  Specify a list of opacity rules, in the format \"PERCENT:PATTERN\",\n  like '50:name *= \"Firefox\"'. compton-trans is recommended over\n  this. Note we do not distinguish 100% and unset, and we don't make\n  any guarantee about possible conflicts with other programs that set\n  _NET_WM_WINDOW_OPACITY on frame or client windows.\n\n--shadow-exclude-reg geometry\n  Specify a X geometry that describes the region in which shadow\n  should not be painted in, such as a dock window region.\n  Use --shadow-exclude-reg 'x10+0-0', for example, if the 10 pixels\n  on the bottom of the screen should not have shadows painted on.\n\n--xinerama-shadow-crop\n  Crop shadow of a window fully on a particular Xinerama screen to the\n  screen.\n\n--backend backend\n  Choose backend. Possible choices are xrender, glx, and\n  xr_glx_hybrid.\n\n--glx-no-stencil\n  GLX backend: Avoid using stencil buffer. Might cause issues\n  when rendering transparent content. My tests show a 15% performance\n  boost.\n\n--glx-copy-from-front\n  GLX backend: Copy unmodified regions from front buffer instead of\n  redrawing them all. My tests with nvidia-drivers show a 5% decrease\n  in performance when the whole screen is modified, but a 30% increase\n  when only 1/4 is. My tests on nouveau show terrible slowdown. Could\n  work with --glx-swap-method but not --glx-use-copysubbuffermesa.\n\n--glx-use-copysubbuffermesa\n  GLX backend: Use MESA_copy_sub_buffer to do partial screen update.\n  My tests on nouveau shows a 200% performance boost when only 1/4 of\n  the screen is updated. May break VSync and is not available on some\n  drivers. Overrides --glx-copy-from-front.\n\n--glx-no-rebind-pixmap\n  GLX backend: Avoid rebinding pixmap on window damage. Probably\n  could improve performance on rapid window content changes, but is\n  known to break things on some drivers (LLVMpipe, xf86-video-intel,\n  etc.).\n\n--glx-swap-method undefined/copy/exchange/3/4/5/6/buffer-age\n  GLX backend: GLX buffer swap method we assume. Could be\n  undefined (0), copy (1), exchange (2), 3-6, or buffer-age (-1).\n  \"undefined\" is the slowest and the safest, and the default value.\n  1 is fastest, but may fail on some drivers, 2-6 are gradually slower\n  but safer (6 is still faster than 0). -1 means auto-detect using\n  GLX_EXT_buffer_age, supported by some drivers. Useless with\n  --glx-use-copysubbuffermesa.\n\n--glx-use-gpushader4\n  GLX backend: Use GL_EXT_gpu_shader4 for some optimization on blur\n  GLSL code. My tests on GTX 670 show no noticeable effect.\n\n--xrender-sync\n  Attempt to synchronize client applications' draw calls with XSync(),\n  used on GLX backend to ensure up-to-date window content is painted.\n\n--xrender-sync-fence\n  Additionally use X Sync fence to sync clients' draw calls. Needed\n  on nvidia-drivers with GLX backend for some users.\n\n--glx-fshader-win shader\n  GLX backend: Use specified GLSL fragment shader for rendering window\n  contents.\n\n--force-win-blend\n  Force all windows to be painted with blending. Useful if you have a\n  --glx-fshader-win that could turn opaque pixels transparent.\n\n--dbus\n  Enable remote control via D-Bus. See the D-BUS API section in the\n  man page for more details.\n\n--benchmark cycles\n  Benchmark mode. Repeatedly paint until reaching the specified cycles.\n\n--benchmark-wid window-id\n  Specify window ID to repaint in benchmark mode. If omitted or is 0,\n  the whole screen is repainted.\n\0"
        as *const u8 as *const i8;
    let mut f: *mut FILE = if ret != 0 { stderr } else { stdout };
    fputs(usage_text, f);
    exit(ret);
}
unsafe extern "C" fn register_cm(mut ps: *mut session_t) -> bool {
    (*ps).reg_win = XCreateSimpleWindow(
        (*ps).dpy,
        (*ps).root,
        0 as i32,
        0 as i32,
        1 as i32 as u32,
        1 as i32 as u32,
        0 as i32 as u32,
        0 as i64 as u64,
        0 as i64 as u64,
    );
    if (*ps).reg_win == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to create window.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 12], &[i8; 12]>(b"register_cm\0")).as_ptr(),
        );
        return 0 as i32 != 0;
    }
    if (*ps).redirected {
        XCompositeUnredirectWindow((*ps).dpy, (*ps).reg_win, 1 as i32);
    }
    let mut h: *mut XClassHint = XAllocClassHint();
    if !h.is_null() {
        (*h).res_name = b"compton\0" as *const u8 as *const i8 as *mut i8;
        (*h).res_class = b"xcompmgr\0" as *const u8 as *const i8 as *mut i8;
    }
    Xutf8SetWMProperties(
        (*ps).dpy,
        (*ps).reg_win,
        b"xcompmgr\0" as *const u8 as *const i8,
        b"xcompmgr\0" as *const u8 as *const i8,
        0 as *mut *mut i8,
        0 as i32,
        0 as *mut XSizeHints,
        0 as *mut XWMHints,
        h,
    );
    cxfree(h as *mut libc::c_void);
    let mut pid: i64 = getpid() as i64;
    if XChangeProperty(
        (*ps).dpy,
        (*ps).reg_win,
        get_atom(ps, b"_NET_WM_PID\0" as *const u8 as *const i8),
        6 as i32 as Atom,
        32 as i32,
        0 as i32,
        &mut pid as *mut i64 as *mut u8,
        1 as i32,
    ) == 0
    {
        fprintf(
            stderr,
            b"%s(): Failed to set _NET_WM_PID.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 12], &[i8; 12]>(b"register_cm\0")).as_ptr(),
        );
    }
    if !wid_set_text_prop(
        ps,
        (*ps).reg_win,
        get_atom(ps, b"COMPTON_VERSION\0" as *const u8 as *const i8),
        b"git-62f8c2b8-dirty-2025-05-05\0" as *const u8 as *const i8 as *mut i8,
    ) {
        fprintf(
            stderr,
            b"%s(): Failed to set COMPTON_VERSION.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 12], &[i8; 12]>(b"register_cm\0")).as_ptr(),
        );
    }
    if !(*ps).o.no_x_selection {
        let mut len: u32 = (strlen(b"_NET_WM_CM_S\0" as *const u8 as *const i8))
            .wrapping_add(2 as i32 as u64) as u32;
        let mut s: i32 = (*ps).scr;
        while s >= 10 as i32 {
            len = len.wrapping_add(1);
            len;
            s /= 10 as i32;
        }
        let mut buf: *mut i8 = malloc(len as u64) as *mut i8;
        snprintf(
            buf,
            len as u64,
            b"_NET_WM_CM_S%d\0" as *const u8 as *const i8,
            (*ps).scr,
        );
        *buf.offset(len.wrapping_sub(1 as i32 as u32) as isize) = '\0' as i32 as i8;
        XSetSelectionOwner(
            (*ps).dpy,
            get_atom(ps, buf),
            (*ps).reg_win,
            0 as i32 as Time,
        );
        free(buf as *mut libc::c_void);
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn ostream_reopen(
    mut ps: *mut session_t,
    mut path: *const i8,
) -> bool {
    if path.is_null() {
        path = (*ps).o.logpath;
    }
    if path.is_null() {
        path = b"/dev/null\0" as *const u8 as *const i8;
    }
    let mut success: bool = !(freopen(path, b"a\0" as *const u8 as *const i8, stdout))
        .is_null();
    success = !(freopen(path, b"a\0" as *const u8 as *const i8, stderr)).is_null()
        && success as i32 != 0;
    if !success {
        fprintf(
            stderr,
            b"%s(%s): freopen() failed.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 15], &[i8; 15]>(b"ostream_reopen\0"))
                .as_ptr(),
            path,
        );
        exit(1 as i32);
    }
    return success;
}
#[inline]
unsafe extern "C" fn fork_after(mut ps: *mut session_t) -> bool {
    if getppid() == 1 as i32 {
        return 1 as i32 != 0;
    }
    if glx_has_context(ps) as i32 != 0
        && glXMakeCurrent((*ps).dpy, 0 as i64 as GLXDrawable, 0 as GLXContext) == 0
    {
        fprintf(
            stderr,
            b"%s(): Failed to detach GLx context.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"fork_after\0")).as_ptr(),
        );
        return 0 as i32 != 0;
    }
    let mut pid: i32 = fork();
    if -(1 as i32) == pid {
        fprintf(
            stderr,
            b"%s(): fork() failed.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"fork_after\0")).as_ptr(),
        );
        return 0 as i32 != 0;
    }
    if pid > 0 as i32 {
        _exit(0 as i32);
    }
    setsid();
    if glx_has_context(ps) as i32 != 0
        && glXMakeCurrent((*ps).dpy, get_tgt_window(ps), (*(*ps).psglx).context) == 0
    {
        fprintf(
            stderr,
            b"%s(): Failed to make GLX context current.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"fork_after\0")).as_ptr(),
        );
        return 0 as i32 != 0;
    }
    let mut success: bool = !(freopen(
        b"/dev/null\0" as *const u8 as *const i8,
        b"r\0" as *const u8 as *const i8,
        stdin,
    ))
        .is_null();
    if !success {
        fprintf(
            stderr,
            b"%s(): freopen() failed.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"fork_after\0")).as_ptr(),
        );
        return 0 as i32 != 0;
    }
    return success;
}
#[inline]
unsafe extern "C" fn write_pid(mut ps: *mut session_t) -> bool {
    if ((*ps).o.write_pid_path).is_null() {
        return 1 as i32 != 0;
    }
    let mut f: *mut FILE = fopen(
        (*ps).o.write_pid_path,
        b"w\0" as *const u8 as *const i8,
    );
    if f.is_null() as i32 as i64 != 0 {
        fprintf(
            stderr,
            b"%s(): Failed to write PID to \"%s\".\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 10], &[i8; 10]>(b"write_pid\0")).as_ptr(),
            (*ps).o.write_pid_path,
        );
        return 0 as i32 != 0;
    }
    fprintf(f, b"%ld\n\0" as *const u8 as *const i8, getpid() as i64);
    fclose(f);
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn parse_long(mut s: *const i8, mut dest: *mut i64) -> bool {
    let mut endptr: *const i8 = 0 as *const i8;
    let mut val: i64 = strtol(
        s,
        &mut endptr as *mut *const i8 as *mut *mut i8,
        0 as i32,
    );
    if endptr.is_null() || endptr == s {
        fprintf(
            stderr,
            b"%s(\"%s\"): Invalid number.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"parse_long\0")).as_ptr(),
            s,
        );
        return 0 as i32 != 0;
    }
    while *(*__ctype_b_loc()).offset(*endptr as i32 as isize) as i32
        & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
    {
        endptr = endptr.offset(1);
        endptr;
    }
    if *endptr != 0 {
        fprintf(
            stderr,
            b"%s(\"%s\"): Trailing characters.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"parse_long\0")).as_ptr(),
            s,
        );
        return 0 as i32 != 0;
    }
    *dest = val;
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn parse_matrix_readnum(
    mut src: *const i8,
    mut dest: *mut libc::c_double,
) -> *const i8 {
    let mut pc: *mut i8 = 0 as *mut i8;
    let mut val: libc::c_double = strtod(src, &mut pc);
    if pc.is_null() || pc == src as *mut i8 {
        fprintf(
            stderr,
            b"%s(\"%s\"): No number found.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 21], &[i8; 21]>(b"parse_matrix_readnum\0"))
                .as_ptr(),
            src,
        );
        return src;
    }
    while *pc as i32 != 0
        && (*(*__ctype_b_loc()).offset(*pc as i32 as isize) as i32
            & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
            || ',' as i32 == *pc as i32)
    {
        pc = pc.offset(1);
        pc;
    }
    *dest = val;
    return pc;
}
#[inline]
unsafe extern "C" fn parse_matrix(
    mut ps: *mut session_t,
    mut src: *const i8,
    mut endptr: *mut *const i8,
) -> *mut XFixed {
    let mut current_block: u64;
    let mut wid: i32 = 0 as i32;
    let mut hei: i32 = 0 as i32;
    let mut pc: *const i8 = 0 as *const i8;
    let mut matrix: *mut XFixed = 0 as *mut XFixed;
    let mut val: libc::c_double = 0.0f64;
    pc = parse_matrix_readnum(src, &mut val);
    if !(src == pc) {
        src = pc;
        wid = val as i32;
        pc = parse_matrix_readnum(src, &mut val);
        if !(src == pc) {
            src = pc;
            hei = val as i32;
            if wid <= 0 as i32 || hei <= 0 as i32 {
                fprintf(
                    stderr,
                    b"%s(): Invalid matrix width/height.\n\0" as *const u8 as *const i8,
                    (*::core::mem::transmute::<&[u8; 13], &[i8; 13]>(b"parse_matrix\0"))
                        .as_ptr(),
                );
            } else if !(wid % 2 as i32 != 0 && hei % 2 as i32 != 0) {
                fprintf(
                    stderr,
                    b"%s(): Width/height not odd.\n\0" as *const u8 as *const i8,
                    (*::core::mem::transmute::<&[u8; 13], &[i8; 13]>(b"parse_matrix\0"))
                        .as_ptr(),
                );
            } else if wid > 16 as i32 || hei > 16 as i32 {
                fprintf(
                    stderr,
                    b"%s(): Matrix width/height too large.\n\0" as *const u8
                        as *const i8,
                    (*::core::mem::transmute::<&[u8; 13], &[i8; 13]>(b"parse_matrix\0"))
                        .as_ptr(),
                );
            } else {
                matrix = calloc(
                    (wid * hei + 2 as i32) as u64,
                    ::core::mem::size_of::<XFixed>() as u64,
                ) as *mut XFixed;
                if matrix.is_null() {
                    fprintf(
                        stderr,
                        b"%s(): Failed to allocate memory for matrix.\n\0" as *const u8
                            as *const i8,
                        (*::core::mem::transmute::<
                            &[u8; 13],
                            &[i8; 13],
                        >(b"parse_matrix\0"))
                            .as_ptr(),
                    );
                } else {
                    let mut skip: i32 = hei / 2 as i32 * wid + wid / 2 as i32;
                    let mut hasneg: bool = 0 as i32 != 0;
                    let mut i: i32 = 0 as i32;
                    loop {
                        if !(i < wid * hei) {
                            current_block = 17478428563724192186;
                            break;
                        }
                        if i == skip {
                            *matrix.offset((2 as i32 + i) as isize) = 0 as i32
                                * 65536 as i32;
                        } else {
                            let mut val_0: libc::c_double = 0 as i32 as libc::c_double;
                            pc = parse_matrix_readnum(src, &mut val_0);
                            if src == pc {
                                current_block = 2731446459762748680;
                                break;
                            }
                            src = pc;
                            if val_0 < 0 as i32 as libc::c_double {
                                hasneg = 1 as i32 != 0;
                            }
                            *matrix.offset((2 as i32 + i) as isize) = (val_0
                                * 65536 as i32 as libc::c_double) as XFixed;
                        }
                        i += 1;
                        i;
                    }
                    match current_block {
                        2731446459762748680 => {}
                        _ => {
                            if backend::BKEND_XRENDER as i32 as u32
                                == (*ps).o.backend as u32 && hasneg as i32 != 0
                            {
                                fprintf(
                                    stderr,
                                    b"%s(): A convolution kernel with negative values may not work properly under X Render backend.\n\0"
                                        as *const u8 as *const i8,
                                    (*::core::mem::transmute::<
                                        &[u8; 13],
                                        &[i8; 13],
                                    >(b"parse_matrix\0"))
                                        .as_ptr(),
                                );
                            }
                            loop {
                                if !(*pc as i32 != 0 && ';' as i32 != *pc as i32) {
                                    current_block = 7205609094909031804;
                                    break;
                                }
                                if *(*__ctype_b_loc()).offset(*pc as i32 as isize) as i32
                                    & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32
                                    == 0 && ',' as i32 != *pc as i32
                                {
                                    fprintf(
                                        stderr,
                                        b"%s(): Trailing characters in matrix string.\n\0"
                                            as *const u8 as *const i8,
                                        (*::core::mem::transmute::<
                                            &[u8; 13],
                                            &[i8; 13],
                                        >(b"parse_matrix\0"))
                                            .as_ptr(),
                                    );
                                    current_block = 2731446459762748680;
                                    break;
                                } else {
                                    pc = pc.offset(1);
                                    pc;
                                }
                            }
                            match current_block {
                                2731446459762748680 => {}
                                _ => {
                                    if ';' as i32 == *pc as i32 {
                                        pc = pc.offset(1);
                                        pc;
                                        while *pc as i32 != 0
                                            && *(*__ctype_b_loc()).offset(*pc as i32 as isize) as i32
                                                & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32
                                                != 0
                                        {
                                            pc = pc.offset(1);
                                            pc;
                                        }
                                    }
                                    if !endptr.is_null() {
                                        *endptr = pc;
                                        current_block = 14832935472441733737;
                                    } else if *pc != 0 {
                                        fprintf(
                                            stderr,
                                            b"%s(): Only one matrix expected.\n\0" as *const u8
                                                as *const i8,
                                            (*::core::mem::transmute::<
                                                &[u8; 13],
                                                &[i8; 13],
                                            >(b"parse_matrix\0"))
                                                .as_ptr(),
                                        );
                                        current_block = 2731446459762748680;
                                    } else {
                                        current_block = 14832935472441733737;
                                    }
                                    match current_block {
                                        2731446459762748680 => {}
                                        _ => {
                                            *matrix.offset(0 as i32 as isize) = wid * 65536 as i32;
                                            *matrix.offset(1 as i32 as isize) = hei * 65536 as i32;
                                            return matrix;
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
    free(matrix as *mut libc::c_void);
    return 0 as *mut XFixed;
}
#[inline]
unsafe extern "C" fn parse_conv_kern(
    mut ps: *mut session_t,
    mut src: *const i8,
    mut endptr: *mut *const i8,
) -> *mut XFixed {
    return parse_matrix(ps, src, endptr);
}
unsafe extern "C" fn parse_conv_kern_lst(
    mut ps: *mut session_t,
    mut src: *const i8,
    mut dest: *mut *mut XFixed,
    mut max: i32,
) -> bool {
    static mut CONV_KERN_PREDEF: [C2RustUnnamed_15; 8] = [
        {
            let mut init = C2RustUnnamed_15 {
                name: b"3x3box\0" as *const u8 as *const i8,
                kern_str: b"3,3,1,1,1,1,1,1,1,1,\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"5x5box\0" as *const u8 as *const i8,
                kern_str: b"5,5,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"7x7box\0" as *const u8 as *const i8,
                kern_str: b"7,7,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"3x3gaussian\0" as *const u8 as *const i8,
                kern_str: b"3,3,0.243117,0.493069,0.243117,0.493069,0.493069,0.243117,0.493069,0.243117,\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"5x5gaussian\0" as *const u8 as *const i8,
                kern_str: b"5,5,0.003493,0.029143,0.059106,0.029143,0.003493,0.029143,0.243117,0.493069,0.243117,0.029143,0.059106,0.493069,0.493069,0.059106,0.029143,0.243117,0.493069,0.243117,0.029143,0.003493,0.029143,0.059106,0.029143,0.003493,\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"7x7gaussian\0" as *const u8 as *const i8,
                kern_str: b"7,7,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,0.000102,0.003493,0.029143,0.059106,0.029143,0.003493,0.000102,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.001723,0.059106,0.493069,0.493069,0.059106,0.001723,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.000102,0.003493,0.029143,0.059106,0.029143,0.003493,0.000102,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"9x9gaussian\0" as *const u8 as *const i8,
                kern_str: b"9,9,0.000000,0.000000,0.000001,0.000006,0.000012,0.000006,0.000001,0.000000,0.000000,0.000000,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,0.000000,0.000001,0.000102,0.003493,0.029143,0.059106,0.029143,0.003493,0.000102,0.000001,0.000006,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.000006,0.000012,0.001723,0.059106,0.493069,0.493069,0.059106,0.001723,0.000012,0.000006,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.000006,0.000001,0.000102,0.003493,0.029143,0.059106,0.029143,0.003493,0.000102,0.000001,0.000000,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,0.000000,0.000000,0.000000,0.000001,0.000006,0.000012,0.000006,0.000001,0.000000,0.000000,\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"11x11gaussian\0" as *const u8 as *const i8,
                kern_str: b"11,11,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000001,0.000006,0.000012,0.000006,0.000001,0.000000,0.000000,0.000000,0.000000,0.000000,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,0.000000,0.000000,0.000000,0.000001,0.000102,0.003493,0.029143,0.059106,0.029143,0.003493,0.000102,0.000001,0.000000,0.000000,0.000006,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.000006,0.000000,0.000000,0.000012,0.001723,0.059106,0.493069,0.493069,0.059106,0.001723,0.000012,0.000000,0.000000,0.000006,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.000006,0.000000,0.000000,0.000001,0.000102,0.003493,0.029143,0.059106,0.029143,0.003493,0.000102,0.000001,0.000000,0.000000,0.000000,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,0.000000,0.000000,0.000000,0.000000,0.000000,0.000001,0.000006,0.000012,0.000006,0.000001,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,\0"
                    as *const u8 as *const i8,
            };
            init
        },
    ];
    let mut i: i32 = 0 as i32;
    while (i as u64)
        < (::core::mem::size_of::<[C2RustUnnamed_15; 8]>() as u64)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_15>() as u64)
    {
        if strcmp(CONV_KERN_PREDEF[i as usize].name, src) == 0 {
            return parse_conv_kern_lst(
                ps,
                CONV_KERN_PREDEF[i as usize].kern_str,
                dest,
                max,
            );
        }
        i += 1;
        i;
    }
    let mut i_0: i32 = 0 as i32;
    let mut pc: *const i8 = src;
    i_0 = 0 as i32;
    while i_0 < max {
        free(*dest.offset(i_0 as isize) as *mut libc::c_void);
        let ref mut fresh8 = *dest.offset(i_0 as isize);
        *fresh8 = 0 as *mut XFixed;
        i_0 += 1;
        i_0;
    }
    i_0 = 0 as i32;
    while !pc.is_null() && *pc as i32 != 0 && i_0 < max - 1 as i32 {
        let fresh9 = i_0;
        i_0 = i_0 + 1;
        let ref mut fresh10 = *dest.offset(fresh9 as isize);
        *fresh10 = parse_conv_kern(ps, pc, &mut pc);
        if (*fresh10).is_null() {
            return 0 as i32 != 0;
        }
    }
    if *pc != 0 {
        fprintf(
            stderr,
            b"%s(): Too many blur kernels!\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 20], &[i8; 20]>(b"parse_conv_kern_lst\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn parse_geometry(
    mut ps: *mut session_t,
    mut src: *const i8,
    mut dest: *mut geometry_t,
) -> bool {
    let mut current_block: u64;
    let mut geom: geometry_t = {
        let mut init = geometry_t {
            wid: -(1 as i32),
            hei: -(1 as i32),
            x: -(1 as i32),
            y: -(1 as i32),
        };
        init
    };
    let mut val: i64 = 0 as i64;
    let mut endptr: *mut i8 = 0 as *mut i8;
    while *src as i32 != 0
        && *(*__ctype_b_loc()).offset(*src as i32 as isize) as i32
            & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
    {
        src = src.offset(1);
        src;
    }
    if !(*src == 0) {
        if !('+' as i32 == *src as i32 || '-' as i32 == *src as i32) {
            val = strtol(src, &mut endptr, 10 as i32);
            if !endptr.is_null() && src != endptr {
                geom.wid = val as i32;
                src = endptr;
            }
            while *src as i32 != 0
                && *(*__ctype_b_loc()).offset(*src as i32 as isize) as i32
                    & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
            {
                src = src.offset(1);
                src;
            }
            if *src == 0 {
                current_block = 5231873688044786582;
            } else {
                current_block = 9606288038608642794;
            }
        } else {
            current_block = 9606288038608642794;
        }
        match current_block {
            5231873688044786582 => {}
            _ => {
                if 'x' as i32 == *src as i32 {
                    src = src.offset(1);
                    src;
                    val = strtol(src, &mut endptr, 10 as i32);
                    if !endptr.is_null() && src != endptr {
                        geom.hei = val as i32;
                        if geom.hei < 0 as i32 {
                            fprintf(
                                stderr,
                                b"%s(\"%s\"): Invalid height.\n\0" as *const u8
                                    as *const i8,
                                (*::core::mem::transmute::<
                                    &[u8; 15],
                                    &[i8; 15],
                                >(b"parse_geometry\0"))
                                    .as_ptr(),
                                src,
                            );
                            return 0 as i32 != 0;
                        }
                        src = endptr;
                    }
                    while *src as i32 != 0
                        && *(*__ctype_b_loc()).offset(*src as i32 as isize) as i32
                            & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32
                            != 0
                    {
                        src = src.offset(1);
                        src;
                    }
                    if *src == 0 {
                        current_block = 5231873688044786582;
                    } else {
                        current_block = 15345278821338558188;
                    }
                } else {
                    current_block = 15345278821338558188;
                }
                match current_block {
                    5231873688044786582 => {}
                    _ => {
                        if '+' as i32 == *src as i32 || '-' as i32 == *src as i32 {
                            val = strtol(src, &mut endptr, 10 as i32);
                            if !endptr.is_null() && src != endptr {
                                geom.x = val as i32;
                                if '-' as i32 == *src as i32 && geom.x <= 0 as i32 {
                                    geom.x -= 2 as i32;
                                }
                                src = endptr;
                            }
                            while *src as i32 != 0
                                && *(*__ctype_b_loc()).offset(*src as i32 as isize) as i32
                                    & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32
                                    != 0
                            {
                                src = src.offset(1);
                                src;
                            }
                            if *src == 0 {
                                current_block = 5231873688044786582;
                            } else {
                                current_block = 14072441030219150333;
                            }
                        } else {
                            current_block = 14072441030219150333;
                        }
                        match current_block {
                            5231873688044786582 => {}
                            _ => {
                                if '+' as i32 == *src as i32 || '-' as i32 == *src as i32 {
                                    val = strtol(src, &mut endptr, 10 as i32);
                                    if !endptr.is_null() && src != endptr {
                                        geom.y = val as i32;
                                        if '-' as i32 == *src as i32 && geom.y <= 0 as i32 {
                                            geom.y -= 2 as i32;
                                        }
                                        src = endptr;
                                    }
                                    while *src as i32 != 0
                                        && *(*__ctype_b_loc()).offset(*src as i32 as isize) as i32
                                            & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32
                                            != 0
                                    {
                                        src = src.offset(1);
                                        src;
                                    }
                                    if *src == 0 {
                                        current_block = 5231873688044786582;
                                    } else {
                                        current_block = 13678349939556791712;
                                    }
                                } else {
                                    current_block = 13678349939556791712;
                                }
                                match current_block {
                                    5231873688044786582 => {}
                                    _ => {
                                        if *src != 0 {
                                            fprintf(
                                                stderr,
                                                b"%s(\"%s\"): Trailing characters.\n\0" as *const u8
                                                    as *const i8,
                                                (*::core::mem::transmute::<
                                                    &[u8; 15],
                                                    &[i8; 15],
                                                >(b"parse_geometry\0"))
                                                    .as_ptr(),
                                                src,
                                            );
                                            return 0 as i32 != 0;
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
    *dest = geom;
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn parse_rule_opacity(
    mut ps: *mut session_t,
    mut src: *const i8,
) -> bool {
    let mut endptr: *mut i8 = 0 as *mut i8;
    let mut val: i64 = strtol(src, &mut endptr, 0 as i32);
    if endptr.is_null() || endptr == src as *mut i8 {
        fprintf(
            stderr,
            b"%s(\"%s\"): No opacity specified?\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"parse_rule_opacity\0"))
                .as_ptr(),
            src,
        );
        return 0 as i32 != 0;
    }
    if val > 100 as i32 as i64 || val < 0 as i32 as i64 {
        fprintf(
            stderr,
            b"%s(\"%s\"): Opacity %ld invalid.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"parse_rule_opacity\0"))
                .as_ptr(),
            src,
            val,
        );
        return 0 as i32 != 0;
    }
    while *endptr as i32 != 0
        && *(*__ctype_b_loc()).offset(*endptr as i32 as isize) as i32
            & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
    {
        endptr = endptr.offset(1);
        endptr;
    }
    if ':' as i32 != *endptr as i32 {
        fprintf(
            stderr,
            b"%s(\"%s\"): Opacity terminator not found.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"parse_rule_opacity\0"))
                .as_ptr(),
            src,
        );
        return 0 as i32 != 0;
    }
    endptr = endptr.offset(1);
    endptr;
    return !(c2_parsed(ps, &mut (*ps).o.opacity_rules, endptr, val as *mut libc::c_void))
        .is_null();
}
unsafe extern "C" fn open_config_file(
    mut cpath: *mut i8,
    mut ppath: *mut *mut i8,
) -> *mut FILE {
    static mut config_filename: *const i8 = b"/compton.conf\0" as *const u8 as *const i8;
    static mut config_filename_legacy: *const i8 = b"/.compton.conf\0" as *const u8
        as *const i8;
    static mut config_home_suffix: *const i8 = b"/.config\0" as *const u8 as *const i8;
    static mut config_system_dir: *const i8 = b"/etc/xdg\0" as *const u8 as *const i8;
    let mut dir: *mut i8 = 0 as *mut i8;
    let mut home: *mut i8 = 0 as *mut i8;
    let mut path: *mut i8 = cpath;
    let mut f: *mut FILE = 0 as *mut FILE;
    if !path.is_null() {
        f = fopen(path, b"r\0" as *const u8 as *const i8);
        if !f.is_null() && !ppath.is_null() {
            *ppath = path;
        }
        return f;
    }
    dir = getenv(b"XDG_CONFIG_HOME\0" as *const u8 as *const i8);
    if !(!dir.is_null() && strlen(dir) != 0) {
        home = getenv(b"HOME\0" as *const u8 as *const i8);
        if !(!home.is_null() && strlen(home) != 0) {
            return 0 as *mut FILE;
        }
        path = mstrjoin3(home, config_home_suffix, config_filename);
    } else {
        path = mstrjoin(dir, config_filename);
    }
    f = fopen(path, b"r\0" as *const u8 as *const i8);
    if !f.is_null() && !ppath.is_null() {
        *ppath = path;
    } else {
        free(path as *mut libc::c_void);
    }
    if !f.is_null() {
        return f;
    }
    home = getenv(b"HOME\0" as *const u8 as *const i8);
    if !home.is_null() && strlen(home) != 0 {
        path = mstrjoin(home, config_filename_legacy);
        f = fopen(path, b"r\0" as *const u8 as *const i8);
        if !f.is_null() && !ppath.is_null() {
            *ppath = path;
        } else {
            free(path as *mut libc::c_void);
        }
        if !f.is_null() {
            return f;
        }
    }
    dir = getenv(b"XDG_CONFIG_DIRS\0" as *const u8 as *const i8);
    if !dir.is_null() && strlen(dir) != 0 {
        let mut part: *mut i8 = strtok(dir, b":\0" as *const u8 as *const i8);
        while !part.is_null() {
            path = mstrjoin(part, config_filename);
            f = fopen(path, b"r\0" as *const u8 as *const i8);
            if !f.is_null() && !ppath.is_null() {
                *ppath = path;
            } else {
                free(path as *mut libc::c_void);
            }
            if !f.is_null() {
                return f;
            }
            part = strtok(0 as *mut i8, b":\0" as *const u8 as *const i8);
        }
    } else {
        path = mstrjoin(config_system_dir, config_filename);
        f = fopen(path, b"r\0" as *const u8 as *const i8);
        if !f.is_null() && !ppath.is_null() {
            *ppath = path;
        } else {
            free(path as *mut libc::c_void);
        }
        if !f.is_null() {
            return f;
        }
    }
    return 0 as *mut FILE;
}
#[inline]
unsafe extern "C" fn parse_cfg_condlst(
    mut ps: *mut session_t,
    mut pcfg: *const config_t,
    mut pcondlst: *mut *mut c2_lptr_t,
    mut name: *const i8,
) {
    let mut setting: *mut config_setting_t = config_lookup(pcfg, name);
    if !setting.is_null() {
        if (*setting).type_0 as i32 == 7 as i32 {
            let mut i: i32 = config_setting_length(setting);
            loop {
                let fresh11 = i;
                i = i - 1;
                if !(fresh11 != 0) {
                    break;
                }
                condlst_add(ps, pcondlst, config_setting_get_string_elem(setting, i));
            }
        } else if 5 as i32 == (*setting).type_0 as i32 {
            condlst_add(ps, pcondlst, config_setting_get_string(setting));
        }
    }
}
#[inline]
unsafe extern "C" fn parse_cfg_condlst_opct(
    mut ps: *mut session_t,
    mut pcfg: *const config_t,
    mut name: *const i8,
) {
    let mut setting: *mut config_setting_t = config_lookup(pcfg, name);
    if !setting.is_null() {
        if (*setting).type_0 as i32 == 7 as i32 {
            let mut i: i32 = config_setting_length(setting);
            loop {
                let fresh12 = i;
                i = i - 1;
                if !(fresh12 != 0) {
                    break;
                }
                if !parse_rule_opacity(ps, config_setting_get_string_elem(setting, i)) {
                    exit(1 as i32);
                }
            }
        } else if 5 as i32 == (*setting).type_0 as i32 {
            parse_rule_opacity(ps, config_setting_get_string(setting));
        }
    }
}
unsafe extern "C" fn parse_config(
    mut ps: *mut session_t,
    mut pcfgtmp: *mut options_tmp,
) {
    let mut path: *mut i8 = 0 as *mut i8;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut cfg: config_t = config_t {
        root: 0 as *mut config_setting_t,
        destructor: None,
        options: 0,
        tab_width: 0,
        default_format: 0,
        include_dir: 0 as *const i8,
        error_text: 0 as *const i8,
        error_file: 0 as *const i8,
        error_line: 0,
        error_type: config_error_t::CONFIG_ERR_NONE,
        filenames: 0 as *mut *const i8,
        num_filenames: 0,
    };
    let mut ival: i32 = 0 as i32;
    let mut dval: libc::c_double = 0.0f64;
    let mut sval: *const i8 = 0 as *const i8;
    f = open_config_file((*ps).o.config_file, &mut path);
    if f.is_null() {
        if !((*ps).o.config_file).is_null() {
            fprintf(
                stderr,
                b"%s(): Failed to read configuration file \"%s\".\n\0" as *const u8
                    as *const i8,
                (*::core::mem::transmute::<&[u8; 13], &[i8; 13]>(b"parse_config\0"))
                    .as_ptr(),
                (*ps).o.config_file,
            );
            exit(1 as i32);
        }
        return;
    }
    config_init(&mut cfg);
    let mut path2: *mut i8 = mstrcpy(path);
    let mut parent: *mut i8 = dirname(path2);
    if !parent.is_null() {
        config_set_include_dir(&mut cfg, parent);
    }
    free(path2 as *mut libc::c_void);
    let mut read_result: i32 = config_read(&mut cfg, f);
    fclose(f);
    f = 0 as *mut FILE;
    if 0 as i32 == read_result {
        printf(
            b"Error when reading configuration file \"%s\", line %d: %s\n\0" as *const u8
                as *const i8,
            path,
            cfg.error_line,
            cfg.error_text,
        );
        config_destroy(&mut cfg);
        free(path as *mut libc::c_void);
        return;
    }
    config_set_auto_convert(&mut cfg, 1 as i32);
    if path != (*ps).o.config_file {
        free((*ps).o.config_file as *mut libc::c_void);
        (*ps).o.config_file = path;
    }
    if lcfg_lookup_int(&mut cfg, b"fade-delta\0" as *const u8 as *const i8, &mut ival)
        != 0
    {
        (*ps).o.fade_delta = ival as time_ms_t;
    }
    if config_lookup_float(
        &mut cfg,
        b"fade-in-step\0" as *const u8 as *const i8,
        &mut dval,
    ) != 0
    {
        (*ps).o.fade_in_step = (normalize_d(dval) * 0xffffffff as u32 as libc::c_double)
            as opacity_t;
    }
    if config_lookup_float(
        &mut cfg,
        b"fade-out-step\0" as *const u8 as *const i8,
        &mut dval,
    ) != 0
    {
        (*ps).o.fade_out_step = (normalize_d(dval) * 0xffffffff as u32 as libc::c_double)
            as opacity_t;
    }
    lcfg_lookup_int(
        &mut cfg,
        b"shadow-radius\0" as *const u8 as *const i8,
        &mut (*ps).o.shadow_radius,
    );
    config_lookup_float(
        &mut cfg,
        b"shadow-opacity\0" as *const u8 as *const i8,
        &mut (*ps).o.shadow_opacity,
    );
    lcfg_lookup_int(
        &mut cfg,
        b"shadow-offset-x\0" as *const u8 as *const i8,
        &mut (*ps).o.shadow_offset_x,
    );
    lcfg_lookup_int(
        &mut cfg,
        b"shadow-offset-y\0" as *const u8 as *const i8,
        &mut (*ps).o.shadow_offset_y,
    );
    if config_lookup_float(
        &mut cfg,
        b"inactive-opacity\0" as *const u8 as *const i8,
        &mut dval,
    ) != 0
    {
        (*ps).o.inactive_opacity = (normalize_d(dval)
            * 0xffffffff as u32 as libc::c_double) as opacity_t;
    }
    if config_lookup_float(
        &mut cfg,
        b"active-opacity\0" as *const u8 as *const i8,
        &mut dval,
    ) != 0
    {
        (*ps).o.active_opacity = (normalize_d(dval)
            * 0xffffffff as u32 as libc::c_double) as opacity_t;
    }
    config_lookup_float(
        &mut cfg,
        b"frame-opacity\0" as *const u8 as *const i8,
        &mut (*ps).o.frame_opacity,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"clear-shadow\0" as *const u8 as *const i8,
        &mut (*ps).o.clear_shadow,
    );
    if config_lookup_bool(&mut cfg, b"shadow\0" as *const u8 as *const i8, &mut ival)
        != 0 && ival != 0
    {
        wintype_arr_enable(((*ps).o.wintype_shadow).as_mut_ptr());
    }
    lcfg_lookup_bool(
        &mut cfg,
        b"no-dock-shadow\0" as *const u8 as *const i8,
        &mut (*pcfgtmp).no_dock_shadow,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"no-dnd-shadow\0" as *const u8 as *const i8,
        &mut (*pcfgtmp).no_dnd_shadow,
    );
    config_lookup_float(
        &mut cfg,
        b"menu-opacity\0" as *const u8 as *const i8,
        &mut (*pcfgtmp).menu_opacity,
    );
    if config_lookup_bool(&mut cfg, b"fading\0" as *const u8 as *const i8, &mut ival)
        != 0 && ival != 0
    {
        wintype_arr_enable(((*ps).o.wintype_fade).as_mut_ptr());
    }
    lcfg_lookup_bool(
        &mut cfg,
        b"no-fading-openclose\0" as *const u8 as *const i8,
        &mut (*ps).o.no_fading_openclose,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"no-fading-destroyed-argb\0" as *const u8 as *const i8,
        &mut (*ps).o.no_fading_destroyed_argb,
    );
    config_lookup_float(
        &mut cfg,
        b"shadow-red\0" as *const u8 as *const i8,
        &mut (*ps).o.shadow_red,
    );
    config_lookup_float(
        &mut cfg,
        b"shadow-green\0" as *const u8 as *const i8,
        &mut (*ps).o.shadow_green,
    );
    config_lookup_float(
        &mut cfg,
        b"shadow-blue\0" as *const u8 as *const i8,
        &mut (*ps).o.shadow_blue,
    );
    if config_lookup_string(
        &mut cfg,
        b"shadow-exclude-reg\0" as *const u8 as *const i8,
        &mut sval,
    ) != 0 && !parse_geometry(ps, sval, &mut (*ps).o.shadow_exclude_reg_geom)
    {
        exit(1 as i32);
    }
    lcfg_lookup_bool(
        &mut cfg,
        b"inactive-opacity-override\0" as *const u8 as *const i8,
        &mut (*ps).o.inactive_opacity_override,
    );
    config_lookup_float(
        &mut cfg,
        b"inactive-dim\0" as *const u8 as *const i8,
        &mut (*ps).o.inactive_dim,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"mark-wmwin-focused\0" as *const u8 as *const i8,
        &mut (*ps).o.mark_wmwin_focused,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"mark-ovredir-focused\0" as *const u8 as *const i8,
        &mut (*ps).o.mark_ovredir_focused,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"shadow-ignore-shaped\0" as *const u8 as *const i8,
        &mut (*ps).o.shadow_ignore_shaped,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"detect-rounded-corners\0" as *const u8 as *const i8,
        &mut (*ps).o.detect_rounded_corners,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"xinerama-shadow-crop\0" as *const u8 as *const i8,
        &mut (*ps).o.xinerama_shadow_crop,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"detect-client-opacity\0" as *const u8 as *const i8,
        &mut (*ps).o.detect_client_opacity,
    );
    lcfg_lookup_int(
        &mut cfg,
        b"refresh-rate\0" as *const u8 as *const i8,
        &mut (*ps).o.refresh_rate,
    );
    if config_lookup_string(&mut cfg, b"vsync\0" as *const u8 as *const i8, &mut sval)
        != 0 && !parse_vsync(ps, sval)
    {
        exit(1 as i32);
    }
    if config_lookup_string(&mut cfg, b"backend\0" as *const u8 as *const i8, &mut sval)
        != 0 && !parse_backend(ps, sval)
    {
        exit(1 as i32);
    }
    config_lookup_float(
        &mut cfg,
        b"alpha-step\0" as *const u8 as *const i8,
        &mut (*ps).o.alpha_step,
    );
    lcfg_lookup_bool(&mut cfg, b"dbe\0" as *const u8 as *const i8, &mut (*ps).o.dbe);
    lcfg_lookup_bool(
        &mut cfg,
        b"paint-on-overlay\0" as *const u8 as *const i8,
        &mut (*ps).o.paint_on_overlay,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"sw-opti\0" as *const u8 as *const i8,
        &mut (*ps).o.sw_opti,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"use-ewmh-active-win\0" as *const u8 as *const i8,
        &mut (*ps).o.use_ewmh_active_win,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"unredir-if-possible\0" as *const u8 as *const i8,
        &mut (*ps).o.unredir_if_possible,
    );
    if lcfg_lookup_int(
        &mut cfg,
        b"unredir-if-possible-delay\0" as *const u8 as *const i8,
        &mut ival,
    ) != 0
    {
        (*ps).o.unredir_if_possible_delay = ival as time_ms_t;
    }
    lcfg_lookup_bool(
        &mut cfg,
        b"inactive-dim-fixed\0" as *const u8 as *const i8,
        &mut (*ps).o.inactive_dim_fixed,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"detect-transient\0" as *const u8 as *const i8,
        &mut (*ps).o.detect_transient,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"detect-client-leader\0" as *const u8 as *const i8,
        &mut (*ps).o.detect_client_leader,
    );
    parse_cfg_condlst(
        ps,
        &mut cfg,
        &mut (*ps).o.shadow_blacklist,
        b"shadow-exclude\0" as *const u8 as *const i8,
    );
    parse_cfg_condlst(
        ps,
        &mut cfg,
        &mut (*ps).o.fade_blacklist,
        b"fade-exclude\0" as *const u8 as *const i8,
    );
    parse_cfg_condlst(
        ps,
        &mut cfg,
        &mut (*ps).o.focus_blacklist,
        b"focus-exclude\0" as *const u8 as *const i8,
    );
    parse_cfg_condlst(
        ps,
        &mut cfg,
        &mut (*ps).o.invert_color_list,
        b"invert-color-include\0" as *const u8 as *const i8,
    );
    parse_cfg_condlst(
        ps,
        &mut cfg,
        &mut (*ps).o.blur_background_blacklist,
        b"blur-background-exclude\0" as *const u8 as *const i8,
    );
    parse_cfg_condlst_opct(ps, &mut cfg, b"opacity-rule\0" as *const u8 as *const i8);
    parse_cfg_condlst(
        ps,
        &mut cfg,
        &mut (*ps).o.unredir_if_possible_blacklist,
        b"unredir-if-possible-exclude\0" as *const u8 as *const i8,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"blur-background\0" as *const u8 as *const i8,
        &mut (*ps).o.blur_background,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"blur-background-frame\0" as *const u8 as *const i8,
        &mut (*ps).o.blur_background_frame,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"blur-background-fixed\0" as *const u8 as *const i8,
        &mut (*ps).o.blur_background_fixed,
    );
    if config_lookup_string(
        &mut cfg,
        b"blur-kern\0" as *const u8 as *const i8,
        &mut sval,
    ) != 0 && !parse_conv_kern_lst(ps, sval, ((*ps).o.blur_kerns).as_mut_ptr(), 5 as i32)
    {
        exit(1 as i32);
    }
    lcfg_lookup_int(
        &mut cfg,
        b"resize-damage\0" as *const u8 as *const i8,
        &mut (*ps).o.resize_damage,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"glx-no-stencil\0" as *const u8 as *const i8,
        &mut (*ps).o.glx_no_stencil,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"glx-copy-from-front\0" as *const u8 as *const i8,
        &mut (*ps).o.glx_copy_from_front,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"glx-use-copysubbuffermesa\0" as *const u8 as *const i8,
        &mut (*ps).o.glx_use_copysubbuffermesa,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"glx-no-rebind-pixmap\0" as *const u8 as *const i8,
        &mut (*ps).o.glx_no_rebind_pixmap,
    );
    if config_lookup_string(
        &mut cfg,
        b"glx-swap-method\0" as *const u8 as *const i8,
        &mut sval,
    ) != 0 && !parse_glx_swap_method(ps, sval)
    {
        exit(1 as i32);
    }
    lcfg_lookup_bool(
        &mut cfg,
        b"glx-use-gpushader4\0" as *const u8 as *const i8,
        &mut (*ps).o.glx_use_gpushader4,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"xrender-sync\0" as *const u8 as *const i8,
        &mut (*ps).o.xrender_sync,
    );
    lcfg_lookup_bool(
        &mut cfg,
        b"xrender-sync-fence\0" as *const u8 as *const i8,
        &mut (*ps).o.xrender_sync_fence,
    );
    let mut i: wintype_t = wintype_t::WINTYPE_UNKNOWN;
    i = wintype_t::WINTYPE_UNKNOWN;
    while (i as u32) < wintype_t::NUM_WINTYPES as i32 as u32 {
        let mut str: *mut i8 = mstrjoin(
            b"wintypes.\0" as *const u8 as *const i8,
            WINTYPES[i as usize],
        );
        let mut setting: *mut config_setting_t = config_lookup(&mut cfg, str);
        free(str as *mut libc::c_void);
        if !setting.is_null() {
            if config_setting_lookup_bool(
                setting,
                b"shadow\0" as *const u8 as *const i8,
                &mut ival,
            ) != 0
            {
                (*ps).o.wintype_shadow[i as usize] = ival != 0;
            }
            if config_setting_lookup_bool(
                setting,
                b"fade\0" as *const u8 as *const i8,
                &mut ival,
            ) != 0
            {
                (*ps).o.wintype_fade[i as usize] = ival != 0;
            }
            if config_setting_lookup_bool(
                setting,
                b"focus\0" as *const u8 as *const i8,
                &mut ival,
            ) != 0
            {
                (*ps).o.wintype_focus[i as usize] = ival != 0;
            }
            config_setting_lookup_float(
                setting,
                b"opacity\0" as *const u8 as *const i8,
                &mut *((*ps).o.wintype_opacity).as_mut_ptr().offset(i as isize),
            );
        }
        i += 1;
        i;
    }
    config_destroy(&mut cfg);
}
unsafe extern "C" fn get_cfg(
    mut ps: *mut session_t,
    mut argc: i32,
    mut argv: *const *mut i8,
    mut first_pass: bool,
) {
    static mut shortopts: *const i8 = b"D:I:O:d:r:o:m:l:t:i:e:hscnfFCaSzGb\0"
        as *const u8 as *const i8;
    static mut longopts: [option; 85] = [
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"config\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 256 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-radius\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-opacity\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-offset-x\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-offset-y\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fade-in-step\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'I' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fade-out-step\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'O' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fade-delta\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'D' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"menu-opacity\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-dock-shadow\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'C' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"clear-shadow\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fading\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"inactive-opacity\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"frame-opacity\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"daemon\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-dnd-shadow\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'G' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-red\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 257 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-green\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 258 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-blue\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 259 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"inactive-opacity-override\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 260 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"inactive-dim\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 261 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"mark-wmwin-focused\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 262 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-exclude\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 263 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"mark-ovredir-focused\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 264 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-fading-openclose\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 265 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-ignore-shaped\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 266 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"detect-rounded-corners\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 267 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"detect-client-opacity\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 268 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"refresh-rate\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 269 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"vsync\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 270 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"alpha-step\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 271 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"dbe\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 272 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"paint-on-overlay\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 273 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"sw-opti\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 274 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"vsync-aggressive\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 275 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"use-ewmh-active-win\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 276 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"respect-prop-shadow\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 277 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"unredir-if-possible\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 278 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"focus-exclude\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 279 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"inactive-dim-fixed\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 280 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"detect-transient\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 281 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"detect-client-leader\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 282 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"blur-background\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 283 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"blur-background-frame\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 284 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"blur-background-fixed\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 285 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"dbus\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 286 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"logpath\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 287 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"invert-color-include\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 288 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"opengl\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 289 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"backend\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 290 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-no-stencil\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 291 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-copy-from-front\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 292 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"benchmark\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 293 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"benchmark-wid\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 294 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-use-copysubbuffermesa\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 295 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"blur-background-exclude\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 296 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"active-opacity\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 297 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-no-rebind-pixmap\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 298 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-swap-method\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 299 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fade-exclude\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 300 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"blur-kern\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 301 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"resize-damage\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 302 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-use-gpushader4\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 303 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"opacity-rule\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 304 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-exclude-reg\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 305 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"paint-exclude\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 306 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"xinerama-shadow-crop\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 307 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"unredir-if-possible-exclude\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 308 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"unredir-if-possible-delay\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 309 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"write-pid-path\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 310 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"vsync-use-glfinish\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 311 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"xrender-sync\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 312 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"xrender-sync-fence\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 313 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"show-all-xerrors\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 314 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-fading-destroyed-argb\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 315 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"force-win-blend\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 316 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-fshader-win\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 317 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 318 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-x-selection\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 319 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-name-pixmap\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 320 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"reredir-on-root-change\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 731 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-reinit-on-root-change\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 732 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 0 as i32,
            };
            init
        },
    ];
    let mut o: i32 = 0 as i32;
    let mut longopt_idx: i32 = -(1 as i32);
    let mut i: i32 = 0 as i32;
    if first_pass {
        optind = 1 as i32;
        loop {
            o = getopt_long(argc, argv, shortopts, longopts.as_ptr(), &mut longopt_idx);
            if !(-(1 as i32) != o) {
                break;
            }
            if 256 as i32 == o {
                (*ps).o.config_file = mstrcpy(optarg);
            } else if 'd' as i32 == o {
                (*ps).o.display = mstrcpy(optarg);
            } else if 'S' as i32 == o {
                (*ps).o.synchronize = 1 as i32 != 0;
            } else if 314 as i32 == o {
                (*ps).o.show_all_xerrors = 1 as i32 != 0;
            } else if 318 as i32 == o {
                printf(
                    b"%s\n\0" as *const u8 as *const i8,
                    b"git-62f8c2b8-dirty-2025-05-05\0" as *const u8 as *const i8,
                );
                exit(0 as i32);
            } else if 320 as i32 == o {
                (*ps).o.no_name_pixmap = 1 as i32 != 0;
            } else if '?' as i32 == o || ':' as i32 == o {
                usage(1 as i32);
            }
        }
        if optind < argc {
            fprintf(
                stderr,
                b"%s(): compton doesn't accept positional arguments.\n\0" as *const u8
                    as *const i8,
                (*::core::mem::transmute::<&[u8; 8], &[i8; 8]>(b"get_cfg\0")).as_ptr(),
            );
            exit(1 as i32);
        }
        return;
    }
    let mut cfgtmp: options_tmp = {
        let mut init = options_tmp {
            no_dock_shadow: 0 as i32 != 0,
            no_dnd_shadow: 0 as i32 != 0,
            menu_opacity: 1.0f64,
        };
        init
    };
    let mut shadow_enable: bool = 0 as i32 != 0;
    let mut fading_enable: bool = 0 as i32 != 0;
    let mut lc_numeric_old: *mut i8 = mstrcpy(setlocale(1 as i32, 0 as *const i8));
    i = 0 as i32;
    while i < wintype_t::NUM_WINTYPES as i32 {
        (*ps).o.wintype_fade[i as usize] = 0 as i32 != 0;
        (*ps).o.wintype_shadow[i as usize] = 0 as i32 != 0;
        (*ps).o.wintype_opacity[i as usize] = 1.0f64;
        i += 1;
        i;
    }
    setlocale(1 as i32, b"C\0" as *const u8 as *const i8);
    parse_config(ps, &mut cfgtmp);
    optind = 1 as i32;
    loop {
        o = getopt_long(argc, argv, shortopts, longopts.as_ptr(), &mut longopt_idx);
        if !(-(1 as i32) != o) {
            break;
        }
        let mut val: i64 = 0 as i32 as i64;
        match o {
            104 => {
                usage(0 as i32);
            }
            68 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as i32);
                }
                (*ps).o.fade_delta = val;
            }
            73 => {
                (*ps).o.fade_in_step = (normalize_d(atof(optarg))
                    * 0xffffffff as u32 as libc::c_double) as opacity_t;
            }
            79 => {
                (*ps).o.fade_out_step = (normalize_d(atof(optarg))
                    * 0xffffffff as u32 as libc::c_double) as opacity_t;
            }
            99 => {
                shadow_enable = 1 as i32 != 0;
            }
            67 => {
                cfgtmp.no_dock_shadow = 1 as i32 != 0;
            }
            71 => {
                cfgtmp.no_dnd_shadow = 1 as i32 != 0;
            }
            109 => {
                cfgtmp.menu_opacity = atof(optarg);
            }
            102 | 70 => {
                fading_enable = 1 as i32 != 0;
            }
            114 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as i32);
                }
                (*ps).o.shadow_radius = val as i32;
            }
            111 => {
                (*ps).o.shadow_opacity = atof(optarg);
            }
            108 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as i32);
                }
                (*ps).o.shadow_offset_x = val as i32;
            }
            116 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as i32);
                }
                (*ps).o.shadow_offset_y = val as i32;
            }
            105 => {
                (*ps).o.inactive_opacity = (normalize_d(atof(optarg))
                    * 0xffffffff as u32 as libc::c_double) as opacity_t;
            }
            101 => {
                (*ps).o.frame_opacity = atof(optarg);
            }
            122 => {
                (*ps).o.clear_shadow = 1 as i32 != 0;
            }
            110 | 97 | 115 => {
                fprintf(
                    stderr,
                    b"%s(): -n, -a, and -s have been removed.\n\0" as *const u8
                        as *const i8,
                    (*::core::mem::transmute::<&[u8; 8], &[i8; 8]>(b"get_cfg\0"))
                        .as_ptr(),
                );
                exit(1 as i32);
            }
            98 => {
                (*ps).o.fork_after_register = 1 as i32 != 0;
            }
            100 | 83 | 314 | 318 | 320 | 256 => {}
            257 => {
                (*ps).o.shadow_red = atof(optarg);
            }
            258 => {
                (*ps).o.shadow_green = atof(optarg);
            }
            259 => {
                (*ps).o.shadow_blue = atof(optarg);
            }
            260 => {
                (*ps).o.inactive_opacity_override = 1 as i32 != 0;
            }
            261 => {
                (*ps).o.inactive_dim = atof(optarg);
            }
            262 => {
                (*ps).o.mark_wmwin_focused = 1 as i32 != 0;
            }
            263 => {
                condlst_add(ps, &mut (*ps).o.shadow_blacklist, optarg);
            }
            264 => {
                (*ps).o.mark_ovredir_focused = 1 as i32 != 0;
            }
            265 => {
                (*ps).o.no_fading_openclose = 1 as i32 != 0;
            }
            266 => {
                (*ps).o.shadow_ignore_shaped = 1 as i32 != 0;
            }
            267 => {
                (*ps).o.detect_rounded_corners = 1 as i32 != 0;
            }
            268 => {
                (*ps).o.detect_client_opacity = 1 as i32 != 0;
            }
            269 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as i32);
                }
                (*ps).o.refresh_rate = val as i32;
            }
            270 => {
                if !parse_vsync(ps, optarg) {
                    exit(1 as i32);
                }
            }
            271 => {
                (*ps).o.alpha_step = atof(optarg);
            }
            272 => {
                (*ps).o.dbe = 1 as i32 != 0;
            }
            273 => {
                (*ps).o.paint_on_overlay = 1 as i32 != 0;
            }
            274 => {
                (*ps).o.sw_opti = 1 as i32 != 0;
            }
            275 => {
                (*ps).o.vsync_aggressive = 1 as i32 != 0;
            }
            276 => {
                (*ps).o.use_ewmh_active_win = 1 as i32 != 0;
            }
            277 => {
                (*ps).o.respect_prop_shadow = 1 as i32 != 0;
            }
            278 => {
                (*ps).o.unredir_if_possible = 1 as i32 != 0;
            }
            279 => {
                condlst_add(ps, &mut (*ps).o.focus_blacklist, optarg);
            }
            280 => {
                (*ps).o.inactive_dim_fixed = 1 as i32 != 0;
            }
            281 => {
                (*ps).o.detect_transient = 1 as i32 != 0;
            }
            282 => {
                (*ps).o.detect_client_leader = 1 as i32 != 0;
            }
            283 => {
                (*ps).o.blur_background = 1 as i32 != 0;
            }
            284 => {
                (*ps).o.blur_background_frame = 1 as i32 != 0;
            }
            285 => {
                (*ps).o.blur_background_fixed = 1 as i32 != 0;
            }
            286 => {
                (*ps).o.dbus = 1 as i32 != 0;
            }
            287 => {
                (*ps).o.logpath = mstrcpy(optarg);
            }
            288 => {
                condlst_add(ps, &mut (*ps).o.invert_color_list, optarg);
            }
            289 => {
                (*ps).o.backend = backend::BKEND_GLX;
            }
            290 => {
                if !parse_backend(ps, optarg) {
                    exit(1 as i32);
                }
            }
            291 => {
                (*ps).o.glx_no_stencil = 1 as i32 != 0;
            }
            292 => {
                (*ps).o.glx_copy_from_front = 1 as i32 != 0;
            }
            293 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as i32);
                }
                (*ps).o.benchmark = val as i32;
            }
            294 => {
                (*ps).o.benchmark_wid = strtol(optarg, 0 as *mut *mut i8, 0 as i32)
                    as Window;
            }
            295 => {
                (*ps).o.glx_use_copysubbuffermesa = 1 as i32 != 0;
            }
            296 => {
                condlst_add(ps, &mut (*ps).o.blur_background_blacklist, optarg);
            }
            297 => {
                (*ps).o.active_opacity = (normalize_d(atof(optarg))
                    * 0xffffffff as u32 as libc::c_double) as opacity_t;
            }
            298 => {
                (*ps).o.glx_no_rebind_pixmap = 1 as i32 != 0;
            }
            299 => {
                if !parse_glx_swap_method(ps, optarg) {
                    exit(1 as i32);
                }
            }
            300 => {
                condlst_add(ps, &mut (*ps).o.fade_blacklist, optarg);
            }
            301 => {
                if !parse_conv_kern_lst(
                    ps,
                    optarg,
                    ((*ps).o.blur_kerns).as_mut_ptr(),
                    5 as i32,
                ) {
                    exit(1 as i32);
                }
            }
            302 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as i32);
                }
                (*ps).o.resize_damage = val as i32;
            }
            303 => {
                (*ps).o.glx_use_gpushader4 = 1 as i32 != 0;
            }
            304 => {
                if !parse_rule_opacity(ps, optarg) {
                    exit(1 as i32);
                }
            }
            305 => {
                if !parse_geometry(ps, optarg, &mut (*ps).o.shadow_exclude_reg_geom) {
                    exit(1 as i32);
                }
            }
            306 => {
                condlst_add(ps, &mut (*ps).o.paint_blacklist, optarg);
            }
            307 => {
                (*ps).o.xinerama_shadow_crop = 1 as i32 != 0;
            }
            308 => {
                condlst_add(ps, &mut (*ps).o.unredir_if_possible_blacklist, optarg);
            }
            309 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as i32);
                }
                (*ps).o.unredir_if_possible_delay = val;
            }
            310 => {
                (*ps).o.write_pid_path = mstrcpy(optarg);
            }
            311 => {
                (*ps).o.vsync_use_glfinish = 1 as i32 != 0;
            }
            312 => {
                (*ps).o.xrender_sync = 1 as i32 != 0;
            }
            313 => {
                (*ps).o.xrender_sync_fence = 1 as i32 != 0;
            }
            315 => {
                (*ps).o.no_fading_destroyed_argb = 1 as i32 != 0;
            }
            316 => {
                (*ps).o.force_win_blend = 1 as i32 != 0;
            }
            317 => {
                (*ps).o.glx_fshader_win_str = mstrcpy(optarg);
            }
            319 => {
                (*ps).o.no_x_selection = 1 as i32 != 0;
            }
            731 => {
                (*ps).o.reredir_on_root_change = 1 as i32 != 0;
            }
            732 => {
                (*ps).o.glx_reinit_on_root_change = 1 as i32 != 0;
            }
            _ => {
                usage(1 as i32);
            }
        }
    }
    setlocale(1 as i32, lc_numeric_old);
    free(lc_numeric_old as *mut libc::c_void);
    (*ps).o.fade_delta = max_i((*ps).o.fade_delta as i32, 1 as i32) as time_ms_t;
    (*ps).o.shadow_radius = max_i((*ps).o.shadow_radius, 1 as i32);
    (*ps).o.shadow_red = normalize_d((*ps).o.shadow_red);
    (*ps).o.shadow_green = normalize_d((*ps).o.shadow_green);
    (*ps).o.shadow_blue = normalize_d((*ps).o.shadow_blue);
    (*ps).o.inactive_dim = normalize_d((*ps).o.inactive_dim);
    (*ps).o.frame_opacity = normalize_d((*ps).o.frame_opacity);
    (*ps).o.shadow_opacity = normalize_d((*ps).o.shadow_opacity);
    cfgtmp.menu_opacity = normalize_d(cfgtmp.menu_opacity);
    (*ps).o.refresh_rate = normalize_i_range((*ps).o.refresh_rate, 0 as i32, 300 as i32);
    (*ps).o.alpha_step = normalize_d_range((*ps).o.alpha_step, 0.01f64, 1.0f64);
    if 0xffffffff as u32 == (*ps).o.inactive_opacity {
        (*ps).o.inactive_opacity = 0 as i32 as opacity_t;
    }
    if 0xffffffff as u32 == (*ps).o.active_opacity {
        (*ps).o.active_opacity = 0 as i32 as opacity_t;
    }
    if shadow_enable {
        wintype_arr_enable(((*ps).o.wintype_shadow).as_mut_ptr());
    }
    (*ps).o.wintype_shadow[wintype_t::WINTYPE_DESKTOP as i32 as usize] = 0 as i32 != 0;
    if cfgtmp.no_dock_shadow {
        (*ps).o.wintype_shadow[wintype_t::WINTYPE_DOCK as i32 as usize] = 0 as i32 != 0;
    }
    if cfgtmp.no_dnd_shadow {
        (*ps).o.wintype_shadow[wintype_t::WINTYPE_DND as i32 as usize] = 0 as i32 != 0;
    }
    if fading_enable {
        wintype_arr_enable(((*ps).o.wintype_fade).as_mut_ptr());
    }
    if 1.0f64 != cfgtmp.menu_opacity {
        (*ps).o.wintype_opacity[wintype_t::WINTYPE_DROPDOWN_MENU as i32 as usize] = cfgtmp
            .menu_opacity;
        (*ps).o.wintype_opacity[wintype_t::WINTYPE_POPUP_MENU as i32 as usize] = cfgtmp
            .menu_opacity;
    }
    if (*ps).o.blur_background_frame {
        (*ps).o.blur_background = 1 as i32 != 0;
    }
    if (*ps).o.xrender_sync_fence {
        (*ps).o.xrender_sync = 1 as i32 != 0;
    }
    if (*ps).o.inactive_opacity != 0 || (*ps).o.active_opacity != 0
        || (*ps).o.inactive_dim != 0.
    {
        (*ps).o.track_focus = 1 as i32 != 0;
    }
    if (*ps).o.detect_transient as i32 != 0 || (*ps).o.detect_client_leader as i32 != 0 {
        (*ps).o.track_leader = 1 as i32 != 0;
    }
    if (*ps).o.blur_background as i32 != 0
        && ((*ps).o.blur_kerns[0 as i32 as usize]).is_null()
    {
        static mut convolution_blur: [XFixed; 11] = [
            3 as i32 * 65536 as i32,
            3 as i32 * 65536 as i32,
            1 as i32 * 65536 as i32,
            1 as i32 * 65536 as i32,
            1 as i32 * 65536 as i32,
            1 as i32 * 65536 as i32,
            1 as i32 * 65536 as i32,
            1 as i32 * 65536 as i32,
            1 as i32 * 65536 as i32,
            1 as i32 * 65536 as i32,
            1 as i32 * 65536 as i32,
        ];
        (*ps).o.blur_kerns[0 as i32 as usize] = malloc(
            ::core::mem::size_of::<[XFixed; 11]>() as u64,
        ) as *mut XFixed;
        if ((*ps).o.blur_kerns[0 as i32 as usize]).is_null() {
            fprintf(
                stderr,
                b"%s(): Failed to allocate memory for convolution kernel.\n\0"
                    as *const u8 as *const i8,
                (*::core::mem::transmute::<&[u8; 8], &[i8; 8]>(b"get_cfg\0")).as_ptr(),
            );
            exit(1 as i32);
        }
        memcpy(
            (*ps).o.blur_kerns[0 as i32 as usize] as *mut libc::c_void,
            &convolution_blur as *const [XFixed; 11] as *const libc::c_void,
            ::core::mem::size_of::<[XFixed; 11]>() as u64,
        );
    }
    rebuild_shadow_exclude_reg(ps);
    if (*ps).o.resize_damage < 0 as i32 {
        fprintf(
            stderr,
            b"%s(): Negative --resize-damage does not work correctly.\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 8], &[i8; 8]>(b"get_cfg\0")).as_ptr(),
        );
    }
}
unsafe extern "C" fn init_atoms(mut ps: *mut session_t) {
    (*ps).atom_opacity = get_atom(
        ps,
        b"_NET_WM_WINDOW_OPACITY\0" as *const u8 as *const i8,
    );
    (*ps).atom_frame_extents = get_atom(
        ps,
        b"_NET_FRAME_EXTENTS\0" as *const u8 as *const i8,
    );
    (*ps).atom_client = get_atom(ps, b"WM_STATE\0" as *const u8 as *const i8);
    (*ps).atom_name = 39 as i32 as Atom;
    (*ps).atom_name_ewmh = get_atom(ps, b"_NET_WM_NAME\0" as *const u8 as *const i8);
    (*ps).atom_class = 67 as i32 as Atom;
    (*ps).atom_role = get_atom(ps, b"WM_WINDOW_ROLE\0" as *const u8 as *const i8);
    (*ps).atom_transient = 68 as i32 as Atom;
    (*ps).atom_client_leader = get_atom(
        ps,
        b"WM_CLIENT_LEADER\0" as *const u8 as *const i8,
    );
    (*ps).atom_ewmh_active_win = get_atom(
        ps,
        b"_NET_ACTIVE_WINDOW\0" as *const u8 as *const i8,
    );
    (*ps).atom_compton_shadow = get_atom(
        ps,
        b"_COMPTON_SHADOW\0" as *const u8 as *const i8,
    );
    (*ps).atom_win_type = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_UNKNOWN as i32 as usize] = 0 as i32 as Atom;
    (*ps).atoms_wintypes[wintype_t::WINTYPE_DESKTOP as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_DESKTOP\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_DOCK as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_DOCK\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_TOOLBAR as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_TOOLBAR\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_MENU as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_MENU\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_UTILITY as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_UTILITY\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_SPLASH as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_SPLASH\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_DIALOG as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_DIALOG\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_NORMAL as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_NORMAL\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_DROPDOWN_MENU as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_DROPDOWN_MENU\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_POPUP_MENU as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_POPUP_MENU\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_TOOLTIP as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_TOOLTIP\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_NOTIFY as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_NOTIFICATION\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_COMBO as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_COMBO\0" as *const u8 as *const i8,
    );
    (*ps).atoms_wintypes[wintype_t::WINTYPE_DND as i32 as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_DND\0" as *const u8 as *const i8,
    );
}
unsafe extern "C" fn update_refresh_rate(mut ps: *mut session_t) {
    let mut randr_info: *mut XRRScreenConfiguration = 0 as *mut XRRScreenConfiguration;
    randr_info = XRRGetScreenInfo((*ps).dpy, (*ps).root);
    if randr_info.is_null() {
        return;
    }
    (*ps).refresh_rate = XRRConfigCurrentRate(randr_info);
    XRRFreeScreenConfigInfo(randr_info);
    if (*ps).refresh_rate != 0 {
        (*ps).refresh_intv = 1000000 as i64 / (*ps).refresh_rate as i64;
    } else {
        (*ps).refresh_intv = 0 as i32 as i64;
    };
}
unsafe extern "C" fn swopti_init(mut ps: *mut session_t) -> bool {
    (*ps).refresh_rate = (*ps).o.refresh_rate as libc::c_short;
    if (*ps).refresh_rate != 0 {
        (*ps).refresh_intv = 1000000 as i64 / (*ps).refresh_rate as i64;
    }
    if (*ps).refresh_rate == 0 && (*ps).randr_exists as i32 != 0 {
        update_refresh_rate(ps);
    }
    if (*ps).refresh_rate == 0 {
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn swopti_handle_timeout(
    mut ps: *mut session_t,
    mut ptv: *mut timeval,
) {
    if ptv.is_null() {
        return;
    }
    let mut offset: i64 = ((*ptv).tv_usec + (get_time_timeval()).tv_usec
        - (*ps).paint_tm_offset) % (*ps).refresh_intv;
    if offset < 0 as i32 as i64 {
        offset += (*ps).refresh_intv;
    }
    if offset < 3000 as i32 as i64 || offset > (*ps).refresh_intv - 3000 as i32 as i64 {
        return;
    }
    (*ptv).tv_usec += (*ps).refresh_intv - offset;
    if (*ptv).tv_usec > 1000000 as i64 {
        (*ptv).tv_usec -= 1000000 as i64;
        (*ptv).tv_sec += 1;
        (*ptv).tv_sec;
    }
}
unsafe extern "C" fn vsync_drm_init(mut ps: *mut session_t) -> bool {
    if (*ps).drm_fd < 0 as i32
        && {
            (*ps).drm_fd = open(
                b"/dev/dri/card0\0" as *const u8 as *const i8,
                0o2 as i32,
            );
            (*ps).drm_fd < 0 as i32
        }
    {
        fprintf(
            stderr,
            b"%s(): Failed to open device.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 15], &[i8; 15]>(b"vsync_drm_init\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    if vsync_drm_wait(ps) != 0 {
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn vsync_drm_wait(mut ps: *mut session_t) -> i32 {
    let mut ret: i32 = -(1 as i32);
    let mut vbl: drm_wait_vblank_t = drm_wait_vblank {
        request: drm_wait_vblank_request {
            type_0: drm_vblank_seq_type::_DRM_VBLANK_ABSOLUTE,
            sequence: 0,
            signal: 0,
        },
    };
    vbl.request.type_0 = drm_vblank_seq_type::_DRM_VBLANK_RELATIVE;
    vbl.request.sequence = 1 as i32 as u32;
    loop {
        ret = ioctl(
            (*ps).drm_fd,
            ((2 as u32 | 1 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
                | (('d' as i32) << 0 as i32 + 8 as i32) as u32
                | ((0x3a as i32) << 0 as i32) as u32) as u64
                | (::core::mem::size_of::<drm_wait_vblank>() as u64)
                    << 0 as i32 + 8 as i32 + 8 as i32,
            &mut vbl as *mut drm_wait_vblank_t,
        );
        vbl.request.type_0 = ::core::mem::transmute::<
            u32,
            drm_vblank_seq_type,
        >(
            vbl.request.type_0 as u32
                & !(drm_vblank_seq_type::_DRM_VBLANK_RELATIVE as i32) as u32,
        );
        if !(ret != 0 && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if ret != 0 {
        fprintf(
            stderr,
            b"vsync_drm_wait(): VBlank ioctl did not work, unimplemented in this drmver?\n\0"
                as *const u8 as *const i8,
        );
    }
    return ret;
}
unsafe extern "C" fn vsync_opengl_init(mut ps: *mut session_t) -> bool {
    if !ensure_glx_context(ps) {
        return 0 as i32 != 0;
    }
    if ((*(*ps).psglx).glXGetVideoSyncSGI).is_none() {
        (*(*ps).psglx).glXGetVideoSyncSGI = ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            f_GetVideoSync,
        >(
            glXGetProcAddress(
                b"glXGetVideoSyncSGI\0" as *const u8 as *const i8 as *const GLubyte,
            ),
        );
    }
    if ((*(*ps).psglx).glXWaitVideoSyncSGI).is_none() {
        (*(*ps).psglx).glXWaitVideoSyncSGI = ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            f_WaitVideoSync,
        >(
            glXGetProcAddress(
                b"glXWaitVideoSyncSGI\0" as *const u8 as *const i8 as *const GLubyte,
            ),
        );
    }
    if ((*(*ps).psglx).glXWaitVideoSyncSGI).is_none()
        || ((*(*ps).psglx).glXGetVideoSyncSGI).is_none()
    {
        fprintf(
            stderr,
            b"%s(): Failed to get glXWait/GetVideoSyncSGI function.\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 18], &[i8; 18]>(b"vsync_opengl_init\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn vsync_opengl_oml_init(mut ps: *mut session_t) -> bool {
    if !ensure_glx_context(ps) {
        return 0 as i32 != 0;
    }
    if ((*(*ps).psglx).glXGetSyncValuesOML).is_none() {
        (*(*ps).psglx).glXGetSyncValuesOML = ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            f_GetSyncValuesOML,
        >(
            glXGetProcAddress(
                b"glXGetSyncValuesOML\0" as *const u8 as *const i8 as *const GLubyte,
            ),
        );
    }
    if ((*(*ps).psglx).glXWaitForMscOML).is_none() {
        (*(*ps).psglx).glXWaitForMscOML = ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            f_WaitForMscOML,
        >(
            glXGetProcAddress(
                b"glXWaitForMscOML\0" as *const u8 as *const i8 as *const GLubyte,
            ),
        );
    }
    if ((*(*ps).psglx).glXGetSyncValuesOML).is_none()
        || ((*(*ps).psglx).glXWaitForMscOML).is_none()
    {
        fprintf(
            stderr,
            b"%s(): Failed to get OML_sync_control functions.\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"vsync_opengl_oml_init\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn vsync_opengl_swc_init(mut ps: *mut session_t) -> bool {
    if !ensure_glx_context(ps) {
        return 0 as i32 != 0;
    }
    if !bkend_use_glx(ps) {
        fprintf(
            stderr,
            b"%s(): I'm afraid glXSwapIntervalSGI wouldn't help if you are not using GLX backend. You could try, nonetheless.\n\0"
                as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"vsync_opengl_swc_init\0"))
                .as_ptr(),
        );
    }
    if ((*(*ps).psglx).glXSwapIntervalProc).is_none() {
        (*(*ps).psglx).glXSwapIntervalProc = ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            f_SwapIntervalSGI,
        >(
            glXGetProcAddress(
                b"glXSwapIntervalSGI\0" as *const u8 as *const i8 as *const GLubyte,
            ),
        );
    }
    if ((*(*ps).psglx).glXSwapIntervalProc).is_none() {
        fprintf(
            stderr,
            b"%s(): Failed to get SGI_swap_control function.\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"vsync_opengl_swc_init\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    ((*(*ps).psglx).glXSwapIntervalProc).expect("non-null function pointer")(1 as i32);
    return 1 as i32 != 0;
}
unsafe extern "C" fn vsync_opengl_mswc_init(mut ps: *mut session_t) -> bool {
    if !ensure_glx_context(ps) {
        return 0 as i32 != 0;
    }
    if !bkend_use_glx(ps) {
        fprintf(
            stderr,
            b"%s(): I'm afraid glXSwapIntervalMESA wouldn't help if you are not using GLX backend. You could try, nonetheless.\n\0"
                as *const u8 as *const i8,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[i8; 23],
            >(b"vsync_opengl_mswc_init\0"))
                .as_ptr(),
        );
    }
    if ((*(*ps).psglx).glXSwapIntervalMESAProc).is_none() {
        (*(*ps).psglx).glXSwapIntervalMESAProc = ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            f_SwapIntervalMESA,
        >(
            glXGetProcAddress(
                b"glXSwapIntervalMESA\0" as *const u8 as *const i8 as *const GLubyte,
            ),
        );
    }
    if ((*(*ps).psglx).glXSwapIntervalMESAProc).is_none() {
        fprintf(
            stderr,
            b"%s(): Failed to get MESA_swap_control function.\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[i8; 23],
            >(b"vsync_opengl_mswc_init\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    ((*(*ps).psglx).glXSwapIntervalMESAProc)
        .expect("non-null function pointer")(1 as i32 as u32);
    return 1 as i32 != 0;
}
unsafe extern "C" fn vsync_opengl_wait(mut ps: *mut session_t) -> i32 {
    let mut vblank_count: u32 = 0 as i32 as u32;
    ((*(*ps).psglx).glXGetVideoSyncSGI)
        .expect("non-null function pointer")(&mut vblank_count);
    ((*(*ps).psglx).glXWaitVideoSyncSGI)
        .expect(
            "non-null function pointer",
        )(
        2 as i32,
        vblank_count.wrapping_add(1 as i32 as u32).wrapping_rem(2 as i32 as u32) as i32,
        &mut vblank_count,
    );
    return 0 as i32;
}
unsafe extern "C" fn vsync_opengl_oml_wait(mut ps: *mut session_t) -> i32 {
    let mut ust: int64_t = 0 as i32 as int64_t;
    let mut msc: int64_t = 0 as i32 as int64_t;
    let mut sbc: int64_t = 0 as i32 as int64_t;
    ((*(*ps).psglx).glXGetSyncValuesOML)
        .expect(
            "non-null function pointer",
        )((*ps).dpy, (*ps).reg_win, &mut ust, &mut msc, &mut sbc);
    ((*(*ps).psglx).glXWaitForMscOML)
        .expect(
            "non-null function pointer",
        )(
        (*ps).dpy,
        (*ps).reg_win,
        0 as i32 as int64_t,
        2 as i32 as int64_t,
        (msc + 1 as i32 as i64) % 2 as i32 as i64,
        &mut ust,
        &mut msc,
        &mut sbc,
    );
    return 0 as i32;
}
unsafe extern "C" fn vsync_opengl_swc_deinit(mut ps: *mut session_t) {
    if glx_has_context(ps) as i32 != 0 && ((*(*ps).psglx).glXSwapIntervalProc).is_some()
    {
        ((*(*ps).psglx).glXSwapIntervalProc)
            .expect("non-null function pointer")(0 as i32);
    }
}
unsafe extern "C" fn vsync_opengl_mswc_deinit(mut ps: *mut session_t) {
    if glx_has_context(ps) as i32 != 0
        && ((*(*ps).psglx).glXSwapIntervalMESAProc).is_some()
    {
        ((*(*ps).psglx).glXSwapIntervalMESAProc)
            .expect("non-null function pointer")(0 as i32 as u32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn vsync_init(mut ps: *mut session_t) -> bool {
    if (*ps).o.vsync as u32 != 0 && (VSYNC_FUNCS_INIT[(*ps).o.vsync as usize]).is_some()
        && !(VSYNC_FUNCS_INIT[(*ps).o.vsync as usize])
            .expect("non-null function pointer")(ps)
    {
        (*ps).o.vsync = vsync_t::VSYNC_NONE;
        return 0 as i32 != 0;
    } else {
        return 1 as i32 != 0
    };
}
unsafe extern "C" fn vsync_wait(mut ps: *mut session_t) {
    if (*ps).o.vsync as u64 == 0 {
        return;
    }
    if (VSYNC_FUNCS_WAIT[(*ps).o.vsync as usize]).is_some() {
        (VSYNC_FUNCS_WAIT[(*ps).o.vsync as usize])
            .expect("non-null function pointer")(ps);
    }
}
#[no_mangle]
pub unsafe extern "C" fn vsync_deinit(mut ps: *mut session_t) {
    if (*ps).o.vsync as u32 != 0
        && (VSYNC_FUNCS_DEINIT[(*ps).o.vsync as usize]).is_some()
    {
        (VSYNC_FUNCS_DEINIT[(*ps).o.vsync as usize])
            .expect("non-null function pointer")(ps);
    }
}
unsafe extern "C" fn init_alpha_picts(mut ps: *mut session_t) {
    let mut i: i32 = 0;
    let mut num: i32 = (round(1.0f64 / (*ps).o.alpha_step) + 1 as i32 as libc::c_double)
        as i32;
    (*ps).alpha_picts = malloc(
        (::core::mem::size_of::<Picture>() as u64).wrapping_mul(num as u64),
    ) as *mut Picture;
    i = 0 as i32;
    while i < num {
        let mut o: libc::c_double = i as libc::c_double * (*ps).o.alpha_step;
        if 1.0f64 - o > (*ps).o.alpha_step {
            *((*ps).alpha_picts).offset(i as isize) = solid_picture(
                ps,
                0 as i32 != 0,
                o,
                0 as i32 as libc::c_double,
                0 as i32 as libc::c_double,
                0 as i32 as libc::c_double,
            );
        } else {
            *((*ps).alpha_picts).offset(i as isize) = 0 as i64 as Picture;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn init_dbe(mut ps: *mut session_t) -> bool {
    (*ps).root_dbe = XdbeAllocateBackBufferName(
        (*ps).dpy,
        if (*ps).o.paint_on_overlay as i32 != 0 { (*ps).overlay } else { (*ps).root },
        3 as i32 as XdbeSwapAction,
    );
    if (*ps).root_dbe == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to create double buffer. Double buffering cannot work.\n\0"
                as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 9], &[i8; 9]>(b"init_dbe\0")).as_ptr(),
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn init_overlay(mut ps: *mut session_t) -> bool {
    (*ps).overlay = XCompositeGetOverlayWindow((*ps).dpy, (*ps).root);
    if (*ps).overlay != 0 {
        let mut region: XserverRegion = XFixesCreateRegion(
            (*ps).dpy,
            0 as *mut XRectangle,
            0 as i32,
        );
        XFixesSetWindowShapeRegion(
            (*ps).dpy,
            (*ps).overlay,
            0 as i32,
            0 as i32,
            0 as i32,
            0 as i32 as XserverRegion,
        );
        XFixesSetWindowShapeRegion(
            (*ps).dpy,
            (*ps).overlay,
            2 as i32,
            0 as i32,
            0 as i32,
            region,
        );
        XFixesDestroyRegion((*ps).dpy, region);
        XSelectInput((*ps).dpy, (*ps).overlay, (1 as i64) << 15 as i32);
    } else {
        fprintf(
            stderr,
            b"Cannot get X Composite overlay window. Falling back to painting on root window.\n\0"
                as *const u8 as *const i8,
        );
        (*ps).o.paint_on_overlay = 0 as i32 != 0;
    }
    return (*ps).overlay != 0;
}
unsafe extern "C" fn init_filters(mut ps: *mut session_t) -> bool {
    if (*ps).o.blur_background as i32 != 0 || (*ps).o.blur_background_frame as i32 != 0 {
        match (*ps).o.backend as u32 {
            0 | 2 => {
                let mut pf: *mut XFilters = XRenderQueryFilters(
                    (*ps).dpy,
                    get_tgt_window(ps),
                );
                if !pf.is_null() {
                    let mut i: i32 = 0 as i32;
                    while i < (*pf).nfilter {
                        if strcmp(
                            *((*pf).filter).offset(i as isize),
                            b"convolution\0" as *const u8 as *const i8,
                        ) == 0
                        {
                            (*ps).xrfilter_convolution_exists = 1 as i32 != 0;
                        }
                        i += 1;
                        i;
                    }
                }
                cxfree(pf as *mut libc::c_void);
                if !(*ps).xrfilter_convolution_exists {
                    fprintf(
                        stderr,
                        b"%s(): X Render convolution filter unsupported by your X server. Background blur is not possible.\n\0"
                            as *const u8 as *const i8,
                        (*::core::mem::transmute::<
                            &[u8; 13],
                            &[i8; 13],
                        >(b"init_filters\0"))
                            .as_ptr(),
                    );
                    return 0 as i32 != 0;
                }
            }
            1 => {
                if !glx_init_blur(ps) {
                    return 0 as i32 != 0;
                }
            }
            _ => {}
        }
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn redir_start(mut ps: *mut session_t) {
    if !(*ps).redirected {
        if (*ps).overlay != 0 {
            XMapWindow((*ps).dpy, (*ps).overlay);
        }
        XCompositeRedirectSubwindows((*ps).dpy, (*ps).root, 1 as i32);
        XSync((*ps).dpy, 0 as i32);
        (*ps).redirected = 1 as i32 != 0;
        force_repaint(ps);
    }
}
unsafe extern "C" fn timeout_get_poll_time(mut ps: *mut session_t) -> time_ms_t {
    let now: time_ms_t = get_time_ms();
    let mut wait: time_ms_t = 9223372036854775807 as i64;
    let mut ptmout: *mut timeout_t = (*ps).tmout_lst;
    while !ptmout.is_null() {
        if (*ptmout).enabled {
            let mut newrun: time_ms_t = timeout_get_newrun(ptmout);
            if newrun <= now {
                wait = 0 as i32 as time_ms_t;
                break;
            } else {
                let mut newwait: time_ms_t = newrun - now;
                if newwait < wait {
                    wait = newwait;
                }
            }
        }
        ptmout = (*ptmout).next;
    }
    return wait;
}
#[no_mangle]
pub unsafe extern "C" fn timeout_insert(
    mut ps: *mut session_t,
    mut interval: time_ms_t,
    mut callback: Option<unsafe extern "C" fn(*mut session_t, *mut timeout_t) -> bool>,
    mut data: *mut libc::c_void,
) -> *mut timeout_t {
    static mut tmout_def: timeout_t = {
        let mut init = _timeout_t {
            enabled: 1 as i32 != 0,
            data: 0 as *const libc::c_void as *mut libc::c_void,
            callback: None,
            interval: 0 as i64,
            firstrun: 0 as i64,
            lastrun: 0 as i64,
            next: 0 as *const _timeout_t as *mut _timeout_t,
        };
        init
    };
    let now: time_ms_t = get_time_ms();
    let mut ptmout: *mut timeout_t = malloc(::core::mem::size_of::<timeout_t>() as u64)
        as *mut timeout_t;
    if ptmout.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed to allocate memory for timeout.\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 15], &[i8; 15]>(b"timeout_insert\0"))
                .as_ptr(),
        );
        exit(1 as i32);
    }
    memcpy(
        ptmout as *mut libc::c_void,
        &tmout_def as *const timeout_t as *const libc::c_void,
        ::core::mem::size_of::<timeout_t>() as u64,
    );
    (*ptmout).interval = interval;
    (*ptmout).firstrun = now;
    (*ptmout).lastrun = now;
    (*ptmout).data = data;
    (*ptmout).callback = callback;
    (*ptmout).next = (*ps).tmout_lst;
    (*ps).tmout_lst = ptmout;
    return ptmout;
}
#[no_mangle]
pub unsafe extern "C" fn timeout_drop(
    mut ps: *mut session_t,
    mut prm: *mut timeout_t,
) -> bool {
    let mut pplast: *mut *mut timeout_t = &mut (*ps).tmout_lst;
    let mut ptmout: *mut timeout_t = (*ps).tmout_lst;
    while !ptmout.is_null() {
        if prm == ptmout {
            *pplast = (*ptmout).next;
            free(ptmout as *mut libc::c_void);
            return 1 as i32 != 0;
        }
        pplast = &mut (*ptmout).next;
        ptmout = (*ptmout).next;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn timeout_clear(mut ps: *mut session_t) {
    let mut ptmout: *mut timeout_t = (*ps).tmout_lst;
    let mut next: *mut timeout_t = 0 as *mut timeout_t;
    while !ptmout.is_null() {
        next = (*ptmout).next;
        free(ptmout as *mut libc::c_void);
        ptmout = next;
    }
}
unsafe extern "C" fn timeout_run(mut ps: *mut session_t) -> bool {
    let now: time_ms_t = get_time_ms();
    let mut ret: bool = 0 as i32 != 0;
    let mut pnext: *mut timeout_t = 0 as *mut timeout_t;
    let mut ptmout: *mut timeout_t = (*ps).tmout_lst;
    while !ptmout.is_null() {
        pnext = (*ptmout).next;
        if (*ptmout).enabled {
            let max: time_ms_t = now
                + ((*ptmout).interval as libc::c_double * 0.05f64) as time_ms_t;
            let mut newrun: time_ms_t = timeout_get_newrun(ptmout);
            if newrun <= max {
                ret = 1 as i32 != 0;
                timeout_invoke(ps, ptmout);
            }
        }
        ptmout = pnext;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn timeout_invoke(
    mut ps: *mut session_t,
    mut ptmout: *mut timeout_t,
) {
    let now: time_ms_t = get_time_ms();
    (*ptmout).lastrun = now;
    if ((*ptmout).callback).is_some() {
        ((*ptmout).callback).expect("non-null function pointer")(ps, ptmout);
    }
}
#[no_mangle]
pub unsafe extern "C" fn timeout_reset(
    mut ps: *mut session_t,
    mut ptmout: *mut timeout_t,
) {
    (*ptmout).lastrun = get_time_ms();
    (*ptmout).firstrun = (*ptmout).lastrun;
}
unsafe extern "C" fn redir_stop(mut ps: *mut session_t) {
    if (*ps).redirected {
        let mut w: *mut win = (*ps).list;
        while !w.is_null() {
            free_wpaint(ps, w);
            w = (*w).next;
        }
        XCompositeUnredirectSubwindows((*ps).dpy, (*ps).root, 1 as i32);
        if (*ps).overlay != 0 {
            XUnmapWindow((*ps).dpy, (*ps).overlay);
        }
        XSync((*ps).dpy, 0 as i32);
        (*ps).redirected = 0 as i32 != 0;
    }
}
unsafe extern "C" fn tmout_unredir_callback(
    mut ps: *mut session_t,
    mut tmout: *mut timeout_t,
) -> bool {
    (*ps).tmout_unredir_hit = 1 as i32 != 0;
    (*tmout).enabled = 0 as i32 != 0;
    return 1 as i32 != 0;
}
unsafe extern "C" fn mainloop(mut ps: *mut session_t) -> bool {
    timeout_run(ps);
    if XEventsQueued((*ps).dpy, 1 as i32) != 0 {
        let mut ev: XEvent = _XEvent { type_0: 0 };
        XNextEvent((*ps).dpy, &mut ev);
        ev_handle(ps, &mut ev);
        (*ps).ev_received = 1 as i32 != 0;
        return 1 as i32 != 0;
    }
    if (*ps).o.dbus {
        cdbus_loop(ps);
    }
    if (*ps).reset {
        return 0 as i32 != 0;
    }
    let mut ptv: *mut timeval = 0 as *mut timeval;
    if (*ps).ev_received as i32 != 0 || (*ps).o.benchmark != 0 {
        ptv = malloc(::core::mem::size_of::<timeval>() as u64) as *mut timeval;
        (*ptv).tv_sec = 0 as i64;
        (*ptv).tv_usec = 0 as i64;
    } else if !(*ps).idling {
        ptv = malloc(::core::mem::size_of::<timeval>() as u64) as *mut timeval;
        *ptv = ms_to_tv(fade_timeout(ps));
    }
    if !ptv.is_null() && (*ps).o.sw_opti as i32 != 0 {
        swopti_handle_timeout(ps, ptv);
    }
    if !ptv.is_null() && timeval_isempty(ptv) as i32 != 0 {
        free(ptv as *mut libc::c_void);
        return 0 as i32 != 0;
    }
    let mut tmout_ms: time_ms_t = timeout_get_poll_time(ps);
    if tmout_ms < 9223372036854775807 as i64 {
        if ptv.is_null() {
            ptv = malloc(::core::mem::size_of::<timeval>() as u64) as *mut timeval;
            *ptv = ms_to_tv(tmout_ms as i32);
        } else if timeval_ms_cmp(ptv, tmout_ms) > 0 as i32 {
            *ptv = ms_to_tv(tmout_ms as i32);
        }
    }
    if !ptv.is_null() && timeval_isempty(ptv) as i32 != 0 {
        free(ptv as *mut libc::c_void);
        return 0 as i32 != 0;
    }
    fds_poll(ps, ptv);
    free(ptv as *mut libc::c_void);
    ptv = 0 as *mut timeval;
    return 1 as i32 != 0;
}
unsafe extern "C" fn cxinerama_upd_scrs(mut ps: *mut session_t) {
    free_xinerama_info(ps);
    if !(*ps).o.xinerama_shadow_crop || !(*ps).xinerama_exists {
        return;
    }
    if XineramaIsActive((*ps).dpy) == 0 {
        return;
    }
    (*ps).xinerama_scrs = XineramaQueryScreens((*ps).dpy, &mut (*ps).xinerama_nscrs);
    if (*ps).xinerama_nscrs == 0 {
        cxfree((*ps).xinerama_scrs as *mut libc::c_void);
        (*ps).xinerama_scrs = 0 as *mut XineramaScreenInfo;
        return;
    }
    (*ps).xinerama_scr_regs = allocchk_(
        (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"cxinerama_upd_scrs\0"))
            .as_ptr(),
        malloc(
            (::core::mem::size_of::<*mut XserverRegion>() as u64)
                .wrapping_mul((*ps).xinerama_nscrs as u64),
        ),
    ) as *mut XserverRegion;
    let mut i: i32 = 0 as i32;
    while i < (*ps).xinerama_nscrs {
        let s: *const XineramaScreenInfo = &mut *((*ps).xinerama_scrs).offset(i as isize)
            as *mut XineramaScreenInfo;
        let mut r: XRectangle = {
            let mut init = XRectangle {
                x: (*s).x_org,
                y: (*s).y_org,
                width: (*s).width as libc::c_ushort,
                height: (*s).height as libc::c_ushort,
            };
            init
        };
        *((*ps).xinerama_scr_regs).offset(i as isize) = XFixesCreateRegion(
            (*ps).dpy,
            &mut r,
            1 as i32,
        );
        i += 1;
        i;
    }
}
unsafe extern "C" fn session_init(
    mut ps_old: *mut session_t,
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> *mut session_t {
    static mut s_def: session_t = {
        let mut init = _session_t {
            dpy: 0 as *const Display as *mut Display,
            scr: 0 as i32,
            vis: 0 as *const Visual as *mut Visual,
            depth: 0 as i32,
            root: 0 as i64 as Window,
            root_height: 0 as i32,
            root_width: 0 as i32,
            overlay: 0 as i64 as Window,
            root_tile_fill: 0 as i32 != 0,
            root_tile_paint: {
                let mut init = paint_t {
                    pixmap: 0 as i64 as Pixmap,
                    pict: 0 as i64 as Picture,
                    ptex: 0 as *const glx_texture_t as *mut glx_texture_t,
                };
                init
            },
            screen_reg: 0 as i64 as XserverRegion,
            root_picture: 0,
            tgt_picture: 0 as i64 as Picture,
            tgt_buffer: {
                let mut init = paint_t {
                    pixmap: 0 as i64 as Pixmap,
                    pict: 0 as i64 as Picture,
                    ptex: 0 as *const glx_texture_t as *mut glx_texture_t,
                };
                init
            },
            tgt_buffer_fence: 0,
            root_dbe: 0 as i64 as XdbeBackBuffer,
            reg_win: 0 as i64 as Window,
            psglx: 0 as *const glx_session_t as *mut glx_session_t,
            o: {
                let mut init = _options_t {
                    config_file: 0 as *const i8 as *mut i8,
                    write_pid_path: 0 as *const i8 as *mut i8,
                    display: 0 as *const i8 as *mut i8,
                    display_repr: 0 as *const i8 as *mut i8,
                    backend: backend::BKEND_XRENDER,
                    xrender_sync: false,
                    xrender_sync_fence: false,
                    glx_no_stencil: 0 as i32 != 0,
                    glx_copy_from_front: 0 as i32 != 0,
                    glx_use_copysubbuffermesa: false,
                    glx_no_rebind_pixmap: false,
                    glx_swap_method: 0,
                    glx_use_gpushader4: false,
                    glx_fshader_win_str: 0 as *const i8 as *mut i8,
                    glx_prog_win: {
                        let mut init = glx_prog_main_t {
                            prog: 0 as i32 as GLuint,
                            unifm_opacity: -(1 as i32),
                            unifm_invert_color: -(1 as i32),
                            unifm_tex: -(1 as i32),
                        };
                        init
                    },
                    fork_after_register: 0 as i32 != 0,
                    detect_rounded_corners: 0 as i32 != 0,
                    paint_on_overlay: 0 as i32 != 0,
                    force_win_blend: false,
                    resize_damage: 0 as i32,
                    unredir_if_possible: 0 as i32 != 0,
                    unredir_if_possible_blacklist: 0 as *const c2_lptr_t
                        as *mut c2_lptr_t,
                    unredir_if_possible_delay: 0 as i32 as time_ms_t,
                    redirected_force: switch_t::UNSET,
                    stoppaint_force: switch_t::UNSET,
                    reredir_on_root_change: false,
                    glx_reinit_on_root_change: false,
                    dbus: 0 as i32 != 0,
                    logpath: 0 as *const i8 as *mut i8,
                    benchmark: 0 as i32,
                    benchmark_wid: 0 as i64 as Window,
                    paint_blacklist: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    no_name_pixmap: false,
                    synchronize: 0 as i32 != 0,
                    show_all_xerrors: false,
                    no_x_selection: false,
                    refresh_rate: 0 as i32,
                    sw_opti: 0 as i32 != 0,
                    vsync: vsync_t::VSYNC_NONE,
                    dbe: 0 as i32 != 0,
                    vsync_aggressive: 0 as i32 != 0,
                    vsync_use_glfinish: false,
                    wintype_shadow: [
                        0 as i32 != 0,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                    ],
                    shadow_red: 0.0f64,
                    shadow_green: 0.0f64,
                    shadow_blue: 0.0f64,
                    shadow_radius: 12 as i32,
                    shadow_offset_x: -(15 as i32),
                    shadow_offset_y: -(15 as i32),
                    shadow_opacity: 0.75f64,
                    clear_shadow: 0 as i32 != 0,
                    shadow_exclude_reg_geom: geometry_t {
                        wid: 0,
                        hei: 0,
                        x: 0,
                        y: 0,
                    },
                    shadow_blacklist: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    shadow_ignore_shaped: 0 as i32 != 0,
                    respect_prop_shadow: 0 as i32 != 0,
                    xinerama_shadow_crop: 0 as i32 != 0,
                    wintype_fade: [
                        0 as i32 != 0,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                    ],
                    fade_in_step: (0.028f64 * 0xffffffff as u32 as libc::c_double)
                        as opacity_t,
                    fade_out_step: (0.03f64 * 0xffffffff as u32 as libc::c_double)
                        as opacity_t,
                    fade_delta: 10 as i32 as time_ms_t,
                    no_fading_openclose: 0 as i32 != 0,
                    no_fading_destroyed_argb: 0 as i32 != 0,
                    fade_blacklist: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    wintype_opacity: [
                        0.0f64,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                    ],
                    inactive_opacity: 0 as i32 as opacity_t,
                    active_opacity: 0 as i32 as opacity_t,
                    inactive_opacity_override: 0 as i32 != 0,
                    frame_opacity: 0.0f64,
                    detect_client_opacity: 0 as i32 != 0,
                    alpha_step: 0.03f64,
                    blur_background: 0 as i32 != 0,
                    blur_background_frame: 0 as i32 != 0,
                    blur_background_fixed: 0 as i32 != 0,
                    blur_background_blacklist: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    blur_kerns: [
                        0 as *const XFixed as *mut XFixed,
                        0 as *const XFixed as *mut XFixed,
                        0 as *const XFixed as *mut XFixed,
                        0 as *const XFixed as *mut XFixed,
                        0 as *const XFixed as *mut XFixed,
                    ],
                    inactive_dim: 0.0f64,
                    inactive_dim_fixed: 0 as i32 != 0,
                    invert_color_list: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    opacity_rules: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    wintype_focus: [
                        0 as i32 != 0,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                    ],
                    mark_wmwin_focused: 0 as i32 != 0,
                    mark_ovredir_focused: 0 as i32 != 0,
                    use_ewmh_active_win: 0 as i32 != 0,
                    focus_blacklist: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    detect_transient: 0 as i32 != 0,
                    detect_client_leader: 0 as i32 != 0,
                    track_focus: 0 as i32 != 0,
                    track_wdata: 0 as i32 != 0,
                    track_leader: 0 as i32 != 0,
                };
                init
            },
            pfds_read: 0 as *const fd_set as *mut fd_set,
            pfds_write: 0 as *const fd_set as *mut fd_set,
            pfds_except: 0 as *const fd_set as *mut fd_set,
            nfds_max: 0 as i32,
            tmout_lst: 0 as *const _timeout_t as *mut _timeout_t,
            tmout_unredir: 0 as *const _timeout_t as *mut _timeout_t,
            tmout_unredir_hit: false,
            ev_received: false,
            idling: 0 as i32 != 0,
            time_start: {
                let mut init = timeval {
                    tv_sec: 0 as i32 as __time_t,
                    tv_usec: 0 as i32 as __suseconds_t,
                };
                init
            },
            all_damage: 0 as i64 as XserverRegion,
            all_damage_last: [0 as i64 as XserverRegion, 0, 0, 0, 0],
            redirected: 0 as i32 != 0,
            alpha_picts: 0 as *const Picture as *mut Picture,
            reg_ignore_expire: 0 as i32 != 0,
            fade_time: 0 as i64,
            ignore_head: 0 as *const ignore_t as *mut ignore_t,
            ignore_tail: 0 as *const *mut ignore_t as *mut *mut ignore_t,
            blur_kerns_cache: [0 as *const XFixed as *mut XFixed; 5],
            reset: 0 as i32 != 0,
            expose_rects: 0 as *const XRectangle as *mut XRectangle,
            size_expose: 0 as i32,
            n_expose: 0 as i32,
            list: 0 as *const _win as *mut _win,
            active_win: 0 as *const _win as *mut _win,
            active_leader: 0 as i64 as Window,
            black_picture: 0 as i64 as Picture,
            cshadow_picture: 0 as i64 as Picture,
            white_picture: 0 as i64 as Picture,
            gaussian_map: 0 as *const conv as *mut conv,
            cgsize: 0 as i32,
            shadow_corner: 0 as *const u8 as *mut u8,
            shadow_top: 0 as *const u8 as *mut u8,
            shadow_exclude_reg: 0,
            refresh_rate: 0 as i32 as libc::c_short,
            refresh_intv: 0 as u64 as i64,
            paint_tm_offset: 0 as i64,
            drm_fd: -(1 as i32),
            xfixes_event: 0 as i32,
            xfixes_error: 0 as i32,
            damage_event: 0 as i32,
            damage_error: 0 as i32,
            render_event: 0 as i32,
            render_error: 0 as i32,
            composite_event: 0 as i32,
            composite_error: 0 as i32,
            composite_opcode: 0 as i32,
            has_name_pixmap: 0 as i32 != 0,
            shape_exists: 0 as i32 != 0,
            shape_event: 0 as i32,
            shape_error: 0 as i32,
            randr_exists: 0 as i32 != 0,
            randr_event: 0 as i32,
            randr_error: 0 as i32,
            glx_exists: 0 as i32 != 0,
            glx_event: 0 as i32,
            glx_error: 0 as i32,
            dbe_exists: 0 as i32 != 0,
            xinerama_exists: false,
            xinerama_scrs: 0 as *const XineramaScreenInfo as *mut XineramaScreenInfo,
            xinerama_scr_regs: 0 as *const XserverRegion as *mut XserverRegion,
            xinerama_nscrs: 0,
            xsync_exists: false,
            xsync_event: 0,
            xsync_error: 0,
            xrfilter_convolution_exists: 0 as i32 != 0,
            atom_opacity: 0 as i64 as Atom,
            atom_frame_extents: 0 as i64 as Atom,
            atom_client: 0 as i64 as Atom,
            atom_name: 0 as i64 as Atom,
            atom_name_ewmh: 0 as i64 as Atom,
            atom_class: 0 as i64 as Atom,
            atom_role: 0 as i64 as Atom,
            atom_transient: 0 as i64 as Atom,
            atom_client_leader: 0,
            atom_ewmh_active_win: 0 as i64 as Atom,
            atom_compton_shadow: 0 as i64 as Atom,
            atom_win_type: 0 as i64 as Atom,
            atoms_wintypes: [0 as i32 as Atom, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            track_atom_lst: 0 as *const latom_t as *mut latom_t,
            dbus_conn: 0 as *const DBusConnection as *mut DBusConnection,
            dbus_service: 0 as *const i8 as *mut i8,
        };
        init
    };
    let mut ps: *mut session_t = malloc(::core::mem::size_of::<session_t>() as u64)
        as *mut session_t;
    memcpy(
        ps as *mut libc::c_void,
        &s_def as *const session_t as *const libc::c_void,
        ::core::mem::size_of::<session_t>() as u64,
    );
    ps_g = ps;
    (*ps).ignore_tail = &mut (*ps).ignore_head;
    gettimeofday(&mut (*ps).time_start, 0 as *mut timezone);
    wintype_arr_enable(((*ps).o.wintype_focus).as_mut_ptr());
    (*ps).o.wintype_focus[wintype_t::WINTYPE_UNKNOWN as i32 as usize] = 0 as i32 != 0;
    (*ps).o.wintype_focus[wintype_t::WINTYPE_NORMAL as i32 as usize] = 0 as i32 != 0;
    (*ps).o.wintype_focus[wintype_t::WINTYPE_UTILITY as i32 as usize] = 0 as i32 != 0;
    get_cfg(ps, argc, argv, 1 as i32 != 0);
    if !ps_old.is_null() && !((*ps_old).dpy).is_null() {
        (*ps).dpy = (*ps_old).dpy;
    }
    if ((*ps).dpy).is_null() {
        (*ps).dpy = XOpenDisplay((*ps).o.display);
        if ((*ps).dpy).is_null() {
            fprintf(
                stderr,
                b"%s(): Can't open display.\n\0" as *const u8 as *const i8,
                (*::core::mem::transmute::<&[u8; 13], &[i8; 13]>(b"session_init\0"))
                    .as_ptr(),
            );
            exit(1 as i32);
        }
    }
    XSetErrorHandler(
        Some(xerror as unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> i32),
    );
    if (*ps).o.synchronize {
        XSynchronize((*ps).dpy, 1 as i32);
    }
    (*ps).scr = (*((*ps).dpy as _XPrivDisplay)).default_screen;
    (*ps).root = (*((*((*ps).dpy as _XPrivDisplay)).screens).offset((*ps).scr as isize))
        .root;
    (*ps).vis = (*((*((*ps).dpy as _XPrivDisplay)).screens).offset((*ps).scr as isize))
        .root_visual;
    (*ps).depth = (*((*((*ps).dpy as _XPrivDisplay)).screens).offset((*ps).scr as isize))
        .root_depth;
    XSelectInput(
        (*ps).dpy,
        (*ps).root,
        (1 as i64) << 19 as i32 | (1 as i64) << 15 as i32 | (1 as i64) << 17 as i32
            | (1 as i64) << 22 as i32,
    );
    XFlush((*ps).dpy);
    (*ps).root_width = (*((*((*ps).dpy as _XPrivDisplay)).screens)
        .offset((*ps).scr as isize))
        .width;
    (*ps).root_height = (*((*((*ps).dpy as _XPrivDisplay)).screens)
        .offset((*ps).scr as isize))
        .height;
    if XRenderQueryExtension((*ps).dpy, &mut (*ps).render_event, &mut (*ps).render_error)
        == 0
    {
        fprintf(stderr, b"No render extension\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    if XQueryExtension(
        (*ps).dpy,
        b"Composite\0" as *const u8 as *const i8,
        &mut (*ps).composite_opcode,
        &mut (*ps).composite_event,
        &mut (*ps).composite_error,
    ) == 0
    {
        fprintf(stderr, b"No composite extension\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    let mut composite_major: i32 = 0 as i32;
    let mut composite_minor: i32 = 0 as i32;
    XCompositeQueryVersion((*ps).dpy, &mut composite_major, &mut composite_minor);
    if !(*ps).o.no_name_pixmap
        && (composite_major > 0 as i32 || composite_minor >= 2 as i32)
    {
        (*ps).has_name_pixmap = 1 as i32 != 0;
    }
    if XDamageQueryExtension((*ps).dpy, &mut (*ps).damage_event, &mut (*ps).damage_error)
        == 0
    {
        fprintf(stderr, b"No damage extension\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    if XFixesQueryExtension((*ps).dpy, &mut (*ps).xfixes_event, &mut (*ps).xfixes_error)
        == 0
    {
        fprintf(stderr, b"No XFixes extension\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    let mut display_repr: *mut i8 = (*((*ps).dpy as _XPrivDisplay)).display_name;
    if display_repr.is_null() {
        display_repr = b"unknown\0" as *const u8 as *const i8 as *mut i8;
    }
    display_repr = mstrcpy(display_repr);
    let mut pdisp: *mut i8 = display_repr;
    while *pdisp != 0 {
        if *(*__ctype_b_loc()).offset(*pdisp as i32 as isize) as i32
            & C2RustUnnamed::_ISalnum as i32 as libc::c_ushort as i32 == 0
        {
            *pdisp = '_' as i32 as i8;
        }
        pdisp = pdisp.offset(1);
        pdisp;
    }
    (*ps).o.display_repr = display_repr;
    get_cfg(ps, argc, argv, 0 as i32 != 0);
    if XShapeQueryExtension((*ps).dpy, &mut (*ps).shape_event, &mut (*ps).shape_error)
        != 0
    {
        (*ps).shape_exists = 1 as i32 != 0;
    }
    if (*ps).o.xrender_sync_fence {
        if XSyncQueryExtension((*ps).dpy, &mut (*ps).xsync_event, &mut (*ps).xsync_error)
            != 0
        {
            let mut major_version_return: i32 = 0 as i32;
            let mut minor_version_return: i32 = 0 as i32;
            if XSyncInitialize(
                (*ps).dpy,
                &mut major_version_return,
                &mut minor_version_return,
            ) != 0
            {
                (*ps).xsync_exists = 1 as i32 != 0;
            }
        }
        if !(*ps).xsync_exists {
            fprintf(
                stderr,
                b"%s(): X Sync extension not found. No X Sync fence sync is possible.\n\0"
                    as *const u8 as *const i8,
                (*::core::mem::transmute::<&[u8; 13], &[i8; 13]>(b"session_init\0"))
                    .as_ptr(),
            );
            exit(1 as i32);
        }
    }
    if (*ps).o.sw_opti as i32 != 0 && (*ps).o.refresh_rate == 0
        || (*ps).o.xinerama_shadow_crop as i32 != 0
    {
        if XRRQueryExtension((*ps).dpy, &mut (*ps).randr_event, &mut (*ps).randr_error)
            != 0
        {
            (*ps).randr_exists = 1 as i32 != 0;
        } else {
            fprintf(
                stderr,
                b"%s(): No XRandR extension, automatic screen change detection impossible.\n\0"
                    as *const u8 as *const i8,
                (*::core::mem::transmute::<&[u8; 13], &[i8; 13]>(b"session_init\0"))
                    .as_ptr(),
            );
        }
    }
    if (*ps).o.dbe {
        let mut dbe_ver_major: i32 = 0 as i32;
        let mut dbe_ver_minor: i32 = 0 as i32;
        if XdbeQueryExtension((*ps).dpy, &mut dbe_ver_major, &mut dbe_ver_minor) != 0 {
            if dbe_ver_major >= 1 as i32 {
                (*ps).dbe_exists = 1 as i32 != 0;
            } else {
                fprintf(
                    stderr,
                    b"DBE extension version too low. Double buffering impossible.\n\0"
                        as *const u8 as *const i8,
                );
            }
        } else {
            fprintf(
                stderr,
                b"No DBE extension. Double buffering impossible.\n\0" as *const u8
                    as *const i8,
            );
        }
        if !(*ps).dbe_exists {
            (*ps).o.dbe = 0 as i32 != 0;
        }
    }
    if (*ps).o.xinerama_shadow_crop {
        let mut xinerama_event: i32 = 0 as i32;
        let mut xinerama_error: i32 = 0 as i32;
        if XineramaQueryExtension((*ps).dpy, &mut xinerama_event, &mut xinerama_error)
            != 0
        {
            (*ps).xinerama_exists = 1 as i32 != 0;
        }
    }
    rebuild_screen_reg(ps);
    if (*ps).o.paint_on_overlay {
        init_overlay(ps);
    }
    if (*ps).o.dbe as i32 != 0
        && backend::BKEND_XRENDER as i32 as u32 != (*ps).o.backend as u32
    {
        fprintf(
            stderr,
            b"%s(): DBE couldn't be used on GLX backend.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 13], &[i8; 13]>(b"session_init\0")).as_ptr(),
        );
        (*ps).o.dbe = 0 as i32 != 0;
    }
    if (*ps).o.dbe as i32 != 0 && !init_dbe(ps) {
        exit(1 as i32);
    }
    if bkend_use_glx(ps) {
        if !glx_init(ps, 1 as i32 != 0) {
            exit(1 as i32);
        }
    }
    if backend::BKEND_GLX as i32 as u32 == (*ps).o.backend as u32
        && !((*ps).o.glx_fshader_win_str).is_null()
    {
        if !glx_load_prog_main(
            ps,
            0 as *const i8,
            (*ps).o.glx_fshader_win_str,
            &mut (*ps).o.glx_prog_win,
        ) {
            exit(1 as i32);
        }
    }
    if (*ps).o.sw_opti {
        (*ps).o.sw_opti = swopti_init(ps);
    }
    if (*ps).randr_exists as i32 != 0
        && ((*ps).o.sw_opti as i32 != 0 && (*ps).o.refresh_rate == 0
            || (*ps).o.xinerama_shadow_crop as i32 != 0)
    {
        XRRSelectInput((*ps).dpy, (*ps).root, ((1 as i64) << 0 as i32) as i32);
    }
    if !vsync_init(ps) {
        exit(1 as i32);
    }
    cxinerama_upd_scrs(ps);
    if (*ps).reg_win == 0 && !register_cm(ps) {
        exit(1 as i32);
    }
    init_atoms(ps);
    init_alpha_picts(ps);
    (*ps).gaussian_map = make_gaussian_map((*ps).o.shadow_radius as libc::c_double);
    presum_gaussian(ps, (*ps).gaussian_map);
    let mut pa: XRenderPictureAttributes = XRenderPictureAttributes {
        repeat: 0,
        alpha_map: 0,
        alpha_x_origin: 0,
        alpha_y_origin: 0,
        clip_x_origin: 0,
        clip_y_origin: 0,
        clip_mask: 0,
        graphics_exposures: 0,
        subwindow_mode: 0,
        poly_edge: 0,
        poly_mode: 0,
        dither: 0,
        component_alpha: 0,
    };
    pa.subwindow_mode = 1 as i32;
    (*ps).root_picture = XRenderCreatePicture(
        (*ps).dpy,
        (*ps).root,
        XRenderFindVisualFormat((*ps).dpy, (*ps).vis),
        ((1 as i32) << 8 as i32) as u64,
        &mut pa,
    );
    if (*ps).o.paint_on_overlay {
        (*ps).tgt_picture = XRenderCreatePicture(
            (*ps).dpy,
            (*ps).overlay,
            XRenderFindVisualFormat((*ps).dpy, (*ps).vis),
            ((1 as i32) << 8 as i32) as u64,
            &mut pa,
        );
    } else {
        (*ps).tgt_picture = (*ps).root_picture;
    }
    if !init_filters(ps) {
        exit(1 as i32);
    }
    (*ps).black_picture = solid_picture(
        ps,
        1 as i32 != 0,
        1 as i32 as libc::c_double,
        0 as i32 as libc::c_double,
        0 as i32 as libc::c_double,
        0 as i32 as libc::c_double,
    );
    (*ps).white_picture = solid_picture(
        ps,
        1 as i32 != 0,
        1 as i32 as libc::c_double,
        1 as i32 as libc::c_double,
        1 as i32 as libc::c_double,
        1 as i32 as libc::c_double,
    );
    if (*ps).o.shadow_red == 0. && (*ps).o.shadow_green == 0.
        && (*ps).o.shadow_blue == 0.
    {
        (*ps).cshadow_picture = (*ps).black_picture;
    } else {
        (*ps).cshadow_picture = solid_picture(
            ps,
            1 as i32 != 0,
            1 as i32 as libc::c_double,
            (*ps).o.shadow_red,
            (*ps).o.shadow_green,
            (*ps).o.shadow_blue,
        );
    }
    fds_insert(ps, (*((*ps).dpy as _XPrivDisplay)).fd, 0x1 as i32 as libc::c_short);
    (*ps).tmout_unredir = timeout_insert(
        ps,
        (*ps).o.unredir_if_possible_delay,
        Some(
            tmout_unredir_callback
                as unsafe extern "C" fn(*mut session_t, *mut timeout_t) -> bool,
        ),
        0 as *mut libc::c_void,
    );
    (*(*ps).tmout_unredir).enabled = 0 as i32 != 0;
    XGrabServer((*ps).dpy);
    let mut root_return: Window = 0;
    let mut parent_return: Window = 0;
    let mut children: *mut Window = 0 as *mut Window;
    let mut nchildren: u32 = 0;
    XQueryTree(
        (*ps).dpy,
        (*ps).root,
        &mut root_return,
        &mut parent_return,
        &mut children,
        &mut nchildren,
    );
    let mut i: u32 = 0 as i32 as u32;
    while i < nchildren {
        add_win(
            ps,
            *children.offset(i as isize),
            if i != 0 {
                *children.offset(i.wrapping_sub(1 as i32 as u32) as isize)
            } else {
                0 as i64 as u64
            },
        );
        i = i.wrapping_add(1);
        i;
    }
    cxfree(children as *mut libc::c_void);
    if (*ps).o.track_focus {
        recheck_focus(ps);
    }
    XUngrabServer((*ps).dpy);
    XFlush((*ps).dpy);
    if (*ps).o.dbus {
        cdbus_init(ps);
        if ((*ps).dbus_conn).is_null() {
            cdbus_destroy(ps);
            (*ps).o.dbus = 0 as i32 != 0;
        }
    }
    if (*ps).o.fork_after_register {
        if !fork_after(ps) {
            session_destroy(ps);
            return 0 as *mut session_t;
        }
    }
    if (*ps).o.fork_after_register as i32 != 0 || !((*ps).o.logpath).is_null() {
        ostream_reopen(ps, 0 as *const i8);
    }
    write_pid(ps);
    if !ps_old.is_null() {
        free(ps_old as *mut libc::c_void);
    }
    return ps;
}
unsafe extern "C" fn session_destroy(mut ps: *mut session_t) {
    redir_stop(ps);
    XSelectInput((*ps).dpy, (*ps).root, 0 as i32 as i64);
    if (*ps).o.dbus {
        cdbus_destroy(ps);
    }
    free((*ps).dbus_service as *mut libc::c_void);
    let mut next: *mut win = 0 as *mut win;
    let mut w: *mut win = (*ps).list;
    while !w.is_null() {
        next = (*w).next;
        if 2 as i32 == (*w).a.map_state && !(*w).destroyed {
            win_ev_stop(ps, w);
        }
        free_win_res(ps, w);
        free(w as *mut libc::c_void);
        w = next;
    }
    (*ps).list = 0 as *mut _win;
    let max: i32 = (round(1.0f64 / (*ps).o.alpha_step) + 1 as i32 as libc::c_double)
        as i32;
    let mut i: i32 = 0 as i32;
    while i < max {
        free_picture(ps, &mut *((*ps).alpha_picts).offset(i as isize));
        i += 1;
        i;
    }
    free((*ps).alpha_picts as *mut libc::c_void);
    (*ps).alpha_picts = 0 as *mut Picture;
    free_wincondlst(&mut (*ps).o.shadow_blacklist);
    free_wincondlst(&mut (*ps).o.fade_blacklist);
    free_wincondlst(&mut (*ps).o.focus_blacklist);
    free_wincondlst(&mut (*ps).o.invert_color_list);
    free_wincondlst(&mut (*ps).o.blur_background_blacklist);
    free_wincondlst(&mut (*ps).o.opacity_rules);
    free_wincondlst(&mut (*ps).o.paint_blacklist);
    free_wincondlst(&mut (*ps).o.unredir_if_possible_blacklist);
    let mut next_0: *mut latom_t = 0 as *mut latom_t;
    let mut this: *mut latom_t = (*ps).track_atom_lst;
    while !this.is_null() {
        next_0 = (*this).next;
        free(this as *mut libc::c_void);
        this = next_0;
    }
    (*ps).track_atom_lst = 0 as *mut latom_t;
    let mut next_1: *mut ignore_t = 0 as *mut ignore_t;
    let mut ign: *mut ignore_t = (*ps).ignore_head;
    while !ign.is_null() {
        next_1 = (*ign).next;
        free(ign as *mut libc::c_void);
        ign = next_1;
    }
    (*ps).ignore_head = 0 as *mut ignore_t;
    (*ps).ignore_tail = &mut (*ps).ignore_head;
    if (*ps).cshadow_picture == (*ps).black_picture {
        (*ps).cshadow_picture = 0 as i64 as Picture;
    } else {
        free_picture(ps, &mut (*ps).cshadow_picture);
    }
    free_picture(ps, &mut (*ps).black_picture);
    free_picture(ps, &mut (*ps).white_picture);
    if (*ps).tgt_buffer.pict == (*ps).tgt_picture {
        (*ps).tgt_buffer.pict = 0 as i64 as Picture;
    }
    if (*ps).tgt_picture == (*ps).root_picture {
        (*ps).tgt_picture = 0 as i64 as Picture;
    } else {
        free_picture(ps, &mut (*ps).tgt_picture);
    }
    free_fence(ps, &mut (*ps).tgt_buffer_fence);
    free_picture(ps, &mut (*ps).root_picture);
    free_paint(ps, &mut (*ps).tgt_buffer);
    free_root_tile(ps);
    free_region(ps, &mut (*ps).screen_reg);
    free_region(ps, &mut (*ps).all_damage);
    let mut i_0: i32 = 0 as i32;
    while i_0 < 5 as i32 {
        free_region(ps, &mut *((*ps).all_damage_last).as_mut_ptr().offset(i_0 as isize));
        i_0 += 1;
        i_0;
    }
    free((*ps).expose_rects as *mut libc::c_void);
    free((*ps).shadow_corner as *mut libc::c_void);
    free((*ps).shadow_top as *mut libc::c_void);
    free((*ps).gaussian_map as *mut libc::c_void);
    free((*ps).o.config_file as *mut libc::c_void);
    free((*ps).o.write_pid_path as *mut libc::c_void);
    free((*ps).o.display as *mut libc::c_void);
    free((*ps).o.display_repr as *mut libc::c_void);
    free((*ps).o.logpath as *mut libc::c_void);
    let mut i_1: i32 = 0 as i32;
    while i_1 < 5 as i32 {
        free((*ps).o.blur_kerns[i_1 as usize] as *mut libc::c_void);
        free((*ps).blur_kerns_cache[i_1 as usize] as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free((*ps).pfds_read as *mut libc::c_void);
    free((*ps).pfds_write as *mut libc::c_void);
    free((*ps).pfds_except as *mut libc::c_void);
    free((*ps).o.glx_fshader_win_str as *mut libc::c_void);
    free_xinerama_info(ps);
    glx_destroy(ps);
    if (*ps).root_dbe != 0 {
        XdbeDeallocateBackBufferName((*ps).dpy, (*ps).root_dbe);
        (*ps).root_dbe = 0 as i64 as XdbeBackBuffer;
    }
    if (*ps).drm_fd >= 0 as i32 {
        close((*ps).drm_fd);
        (*ps).drm_fd = -(1 as i32);
    }
    if (*ps).overlay != 0 {
        XCompositeReleaseOverlayWindow((*ps).dpy, (*ps).overlay);
        (*ps).overlay = 0 as i64 as Window;
    }
    if (*ps).reg_win != 0 {
        XDestroyWindow((*ps).dpy, (*ps).reg_win);
        (*ps).reg_win = 0 as i64 as Window;
    }
    XSync((*ps).dpy, 1 as i32);
    (*ps).tmout_unredir = 0 as *mut _timeout_t;
    timeout_clear(ps);
    if ps == ps_g {
        ps_g = 0 as *mut session_t;
    }
}
unsafe extern "C" fn session_run(mut ps: *mut session_t) {
    let mut t: *mut win = 0 as *mut win;
    if (*ps).o.sw_opti {
        (*ps).paint_tm_offset = (get_time_timeval()).tv_usec;
    }
    (*ps).reg_ignore_expire = 1 as i32 != 0;
    t = paint_preprocess(ps, (*ps).list);
    if (*ps).redirected {
        paint_all(ps, 0 as i64 as XserverRegion, 0 as i64 as XserverRegion, t);
    }
    (*ps).idling = 0 as i32 != 0;
    while !(*ps).reset {
        (*ps).ev_received = 0 as i32 != 0;
        while mainloop(ps) {}
        if (*ps).o.benchmark != 0 {
            if (*ps).o.benchmark_wid != 0 {
                let mut w: *mut win = find_win(ps, (*ps).o.benchmark_wid);
                if w.is_null() {
                    fprintf(
                        stderr,
                        b"%s(): Couldn't find specified benchmark window.\n\0"
                            as *const u8 as *const i8,
                        (*::core::mem::transmute::<
                            &[u8; 12],
                            &[i8; 12],
                        >(b"session_run\0"))
                            .as_ptr(),
                    );
                    session_destroy(ps);
                    exit(1 as i32);
                }
                add_damage_win(ps, w);
            } else {
                force_repaint(ps);
            }
        }
        (*ps).idling = 1 as i32 != 0;
        t = paint_preprocess(ps, (*ps).list);
        (*ps).tmout_unredir_hit = 0 as i32 != 0;
        if !(*ps).redirected
            || switch_t::ON as i32 as u32 == (*ps).o.stoppaint_force as u32
        {
            free_region(ps, &mut (*ps).all_damage);
        }
        let mut all_damage_orig: XserverRegion = 0 as i64 as XserverRegion;
        if (*ps).o.resize_damage > 0 as i32 {
            all_damage_orig = copy_region(ps, (*ps).all_damage);
        }
        resize_region(ps, (*ps).all_damage, (*ps).o.resize_damage as libc::c_short);
        if (*ps).all_damage != 0
            && !is_region_empty(ps, (*ps).all_damage, 0 as *mut reg_data_t)
        {
            static mut paint: i32 = 0 as i32;
            paint_all(ps, (*ps).all_damage, all_damage_orig, t);
            (*ps).reg_ignore_expire = 0 as i32 != 0;
            paint += 1;
            paint;
            if (*ps).o.benchmark != 0 && paint >= (*ps).o.benchmark {
                exit(0 as i32);
            }
            XSync((*ps).dpy, 0 as i32);
            (*ps).all_damage = 0 as i64 as XserverRegion;
        }
        free_region(ps, &mut all_damage_orig);
        if (*ps).idling {
            (*ps).fade_time = 0 as i64;
        }
    }
}
unsafe extern "C" fn reset_enable(mut signum: i32) {
    let ps: *mut session_t = ps_g;
    (*ps).reset = 1 as i32 != 0;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    setlocale(6 as i32, b"\0" as *const u8 as *const i8);
    let mut block_mask: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut block_mask);
    let action: sigaction = {
        let mut init = sigaction {
            __sigaction_handler: C2RustUnnamed_14 {
                sa_handler: Some(reset_enable as unsafe extern "C" fn(i32) -> ()),
            },
            sa_mask: block_mask,
            sa_flags: 0 as i32,
            sa_restorer: None,
        };
        init
    };
    sigaction(10 as i32, &action, 0 as *mut sigaction);
    let mut ps_old: *mut session_t = ps_g;
    loop {
        ps_g = session_init(ps_old, argc, argv);
        if ps_g.is_null() {
            fprintf(
                stderr,
                b"%s(): Failed to create new session.\n\0" as *const u8 as *const i8,
                (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"main\0")).as_ptr(),
            );
            return 1 as i32;
        }
        session_run(ps_g);
        ps_old = ps_g;
        session_destroy(ps_g);
    };
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}