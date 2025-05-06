#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm, extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    pub type DBusMessage;
    pub type DBusWatch;
    pub type DBusTimeout;
    pub type DBusConnection;
    pub type __GLXcontextRec;
    pub type __GLXFBConfigRec;
    pub type _c2_lptr;
    fn dbus_connection_pop_message(connection: *mut DBusConnection) -> *mut DBusMessage;
    fn dbus_connection_send(
        connection: *mut DBusConnection,
        message: *mut DBusMessage,
        client_serial: *mut dbus_uint32_t,
    ) -> dbus_bool_t;
    fn dbus_connection_set_watch_functions(
        connection: *mut DBusConnection,
        add_function: DBusAddWatchFunction,
        remove_function: DBusRemoveWatchFunction,
        toggled_function: DBusWatchToggledFunction,
        data: *mut libc::c_void,
        free_data_function: DBusFreeFunction,
    ) -> dbus_bool_t;
    fn dbus_connection_set_timeout_functions(
        connection: *mut DBusConnection,
        add_function: DBusAddTimeoutFunction,
        remove_function: DBusRemoveTimeoutFunction,
        toggled_function: DBusTimeoutToggledFunction,
        data: *mut libc::c_void,
        free_data_function: DBusFreeFunction,
    ) -> dbus_bool_t;
    fn dbus_watch_get_unix_fd(watch: *mut DBusWatch) -> i32;
    fn dbus_watch_get_flags(watch: *mut DBusWatch) -> u32;
    fn dbus_watch_get_enabled(watch: *mut DBusWatch) -> dbus_bool_t;
    fn dbus_timeout_get_interval(timeout: *mut DBusTimeout) -> i32;
    fn dbus_timeout_get_data(timeout: *mut DBusTimeout) -> *mut libc::c_void;
    fn dbus_timeout_set_data(
        timeout: *mut DBusTimeout,
        data: *mut libc::c_void,
        free_data_function: DBusFreeFunction,
    );
    fn dbus_timeout_handle(timeout: *mut DBusTimeout) -> dbus_bool_t;
    fn dbus_timeout_get_enabled(timeout: *mut DBusTimeout) -> dbus_bool_t;
    fn dbus_bus_get_private(
        type_0: DBusBusType,
        error: *mut DBusError,
    ) -> *mut DBusConnection;
    fn dbus_bus_request_name(
        connection: *mut DBusConnection,
        name: *const i8,
        flags: u32,
        error: *mut DBusError,
    ) -> i32;
    fn dbus_bus_release_name(
        connection: *mut DBusConnection,
        name: *const i8,
        error: *mut DBusError,
    ) -> i32;
    fn dbus_bus_add_match(
        connection: *mut DBusConnection,
        rule: *const i8,
        error: *mut DBusError,
    );
    fn dbus_get_local_machine_id() -> *mut i8;
    fn strcasecmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn exit(_: i32) -> !;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn dbus_connection_read_write(
        connection: *mut DBusConnection,
        timeout_milliseconds: i32,
    ) -> dbus_bool_t;
    fn dbus_connection_flush(connection: *mut DBusConnection);
    fn dbus_connection_set_exit_on_disconnect(
        connection: *mut DBusConnection,
        exit_on_disconnect: dbus_bool_t,
    );
    fn dbus_connection_close(connection: *mut DBusConnection);
    fn dbus_connection_unref(connection: *mut DBusConnection);
    fn dbus_message_type_to_string(type_0: i32) -> *const i8;
    fn dbus_message_iter_get_basic(iter: *mut DBusMessageIter, value: *mut libc::c_void);
    fn dbus_message_iter_get_arg_type(iter: *mut DBusMessageIter) -> i32;
    fn dbus_message_iter_next(iter: *mut DBusMessageIter) -> dbus_bool_t;
    fn dbus_message_iter_init(
        message: *mut DBusMessage,
        iter: *mut DBusMessageIter,
    ) -> dbus_bool_t;
    fn dbus_message_get_args(
        message: *mut DBusMessage,
        error: *mut DBusError,
        first_arg_type: i32,
        _: ...
    ) -> dbus_bool_t;
    fn dbus_message_append_args(
        message: *mut DBusMessage,
        first_arg_type: i32,
        _: ...
    ) -> dbus_bool_t;
    fn dbus_message_is_signal(
        message: *mut DBusMessage,
        iface: *const i8,
        signal_name: *const i8,
    ) -> dbus_bool_t;
    fn dbus_message_is_method_call(
        message: *mut DBusMessage,
        iface: *const i8,
        method: *const i8,
    ) -> dbus_bool_t;
    fn dbus_message_get_no_reply(message: *mut DBusMessage) -> dbus_bool_t;
    fn dbus_message_get_error_name(message: *mut DBusMessage) -> *const i8;
    fn dbus_message_get_member(message: *mut DBusMessage) -> *const i8;
    fn dbus_message_get_interface(message: *mut DBusMessage) -> *const i8;
    fn dbus_message_get_path(message: *mut DBusMessage) -> *const i8;
    fn dbus_message_get_type(message: *mut DBusMessage) -> i32;
    fn dbus_message_unref(message: *mut DBusMessage);
    fn dbus_message_new_error_printf(
        reply_to: *mut DBusMessage,
        error_name: *const i8,
        error_format: *const i8,
        _: ...
    ) -> *mut DBusMessage;
    fn dbus_message_new_signal(
        path: *const i8,
        iface: *const i8,
        name: *const i8,
    ) -> *mut DBusMessage;
    fn dbus_message_new_method_return(method_call: *mut DBusMessage) -> *mut DBusMessage;
    fn dbus_free(memory: *mut libc::c_void);
    fn dbus_error_is_set(error: *const DBusError) -> dbus_bool_t;
    fn dbus_error_free(error: *mut DBusError);
    fn dbus_error_init(error: *mut DBusError);
    static VSYNC_STRS: [*const i8; 7];
    static BACKEND_STRS: [*const i8; 4];
    fn timeout_insert(
        ps: *mut session_t,
        interval: time_ms_t,
        callback: Option<unsafe extern "C" fn(*mut session_t, *mut timeout_t) -> bool>,
        data: *mut libc::c_void,
    ) -> *mut timeout_t;
    fn timeout_drop(ps: *mut session_t, prm: *mut timeout_t) -> bool;
    fn force_repaint(ps: *mut session_t);
    fn vsync_init(ps: *mut session_t) -> bool;
    fn vsync_deinit(ps: *mut session_t);
    fn opts_init_track_focus(ps: *mut session_t);
    fn opts_set_no_fading_openclose(ps: *mut session_t, newval: bool);
    fn win_set_invert_color_force(ps: *mut session_t, w: *mut win, val: switch_t);
    fn win_set_focused_force(ps: *mut session_t, w: *mut win, val: switch_t);
    fn win_set_fade_force(ps: *mut session_t, w: *mut win, val: switch_t);
    fn win_set_shadow_force(ps: *mut session_t, w: *mut win, val: switch_t);
    fn getpid() -> __pid_t;
}
pub type size_t = u64;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
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
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type int_fast16_t = i64;
pub type XID = u64;
pub type Atom = u64;
pub type VisualID = u64;
pub type Window = XID;
pub type Drawable = XID;
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
pub struct XRectangle {
    pub x: libc::c_short,
    pub y: libc::c_short,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
pub type _XPrivDisplay = *mut C2RustUnnamed;
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
    pub type_0: i32,
    pub depth: i32,
    pub direct: XRenderDirectFormat,
    pub colormap: Colormap,
}
pub type XFixed = i32;
pub type XdbeBackBuffer = Drawable;
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
pub type dbus_uint32_t = u32;
pub type dbus_bool_t = dbus_uint32_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct DBusError {
    pub name: *const i8,
    pub message: *const i8,
    #[bitfield(name = "dummy1", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "dummy2", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "dummy3", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "dummy4", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "dummy5", ty = "libc::c_uint", bits = "4..=4")]
    pub dummy1_dummy2_dummy3_dummy4_dummy5: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub padding1: *mut libc::c_void,
}
pub type DBusFreeFunction = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DBusMessageIter {
    pub dummy1: *mut libc::c_void,
    pub dummy2: *mut libc::c_void,
    pub dummy3: dbus_uint32_t,
    pub dummy4: i32,
    pub dummy5: i32,
    pub dummy6: i32,
    pub dummy7: i32,
    pub dummy8: i32,
    pub dummy9: i32,
    pub dummy10: i32,
    pub dummy11: i32,
    pub pad1: i32,
    pub pad2: *mut libc::c_void,
    pub pad3: *mut libc::c_void,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum DBusBusType {
    DBUS_BUS_STARTER = 2,
    DBUS_BUS_SYSTEM = 1,
    DBUS_BUS_SESSION = 0,
}
impl DBusBusType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            DBusBusType::DBUS_BUS_STARTER => 2,
            DBusBusType::DBUS_BUS_SYSTEM => 1,
            DBusBusType::DBUS_BUS_SESSION => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> DBusBusType {
        match value {
            2 => DBusBusType::DBUS_BUS_STARTER,
            1 => DBusBusType::DBUS_BUS_SYSTEM,
            0 => DBusBusType::DBUS_BUS_SESSION,
            _ => panic!("Invalid value for DBusBusType: {}", value),
        }
    }
}
impl AddAssign<u32> for DBusBusType {
    fn add_assign(&mut self, rhs: u32) {
        *self = DBusBusType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for DBusBusType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = DBusBusType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for DBusBusType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = DBusBusType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for DBusBusType {
    fn div_assign(&mut self, rhs: u32) {
        *self = DBusBusType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for DBusBusType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = DBusBusType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for DBusBusType {
    type Output = DBusBusType;
    fn add(self, rhs: u32) -> DBusBusType {
        DBusBusType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for DBusBusType {
    type Output = DBusBusType;
    fn sub(self, rhs: u32) -> DBusBusType {
        DBusBusType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for DBusBusType {
    type Output = DBusBusType;
    fn mul(self, rhs: u32) -> DBusBusType {
        DBusBusType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for DBusBusType {
    type Output = DBusBusType;
    fn div(self, rhs: u32) -> DBusBusType {
        DBusBusType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for DBusBusType {
    type Output = DBusBusType;
    fn rem(self, rhs: u32) -> DBusBusType {
        DBusBusType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    DBUS_WATCH_HANGUP = 8,
    DBUS_WATCH_ERROR = 4,
    DBUS_WATCH_WRITABLE = 2,
    DBUS_WATCH_READABLE = 1,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::DBUS_WATCH_HANGUP => 8,
            C2RustUnnamed_0::DBUS_WATCH_ERROR => 4,
            C2RustUnnamed_0::DBUS_WATCH_WRITABLE => 2,
            C2RustUnnamed_0::DBUS_WATCH_READABLE => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            8 => C2RustUnnamed_0::DBUS_WATCH_HANGUP,
            4 => C2RustUnnamed_0::DBUS_WATCH_ERROR,
            2 => C2RustUnnamed_0::DBUS_WATCH_WRITABLE,
            1 => C2RustUnnamed_0::DBUS_WATCH_READABLE,
            _ => panic!("Invalid value for C2RustUnnamed_0: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_0 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_0 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_0 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_0 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_0 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn add(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn sub(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn mul(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn div(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn rem(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type DBusAddWatchFunction = Option<
    unsafe extern "C" fn(*mut DBusWatch, *mut libc::c_void) -> dbus_bool_t,
>;
pub type DBusWatchToggledFunction = Option<
    unsafe extern "C" fn(*mut DBusWatch, *mut libc::c_void) -> (),
>;
pub type DBusRemoveWatchFunction = Option<
    unsafe extern "C" fn(*mut DBusWatch, *mut libc::c_void) -> (),
>;
pub type DBusAddTimeoutFunction = Option<
    unsafe extern "C" fn(*mut DBusTimeout, *mut libc::c_void) -> dbus_bool_t,
>;
pub type DBusTimeoutToggledFunction = Option<
    unsafe extern "C" fn(*mut DBusTimeout, *mut libc::c_void) -> (),
>;
pub type DBusRemoveTimeoutFunction = Option<
    unsafe extern "C" fn(*mut DBusTimeout, *mut libc::c_void) -> (),
>;
pub type GLenum = u32;
pub type GLint = i32;
pub type GLuint = u32;
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
pub type timeout_t = _timeout_t;
pub type cdbus_enum_t = uint16_t;
pub type cdbus_window_t = uint32_t;
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
unsafe extern "C" fn max_i(mut a: i32, mut b: i32) -> i32 {
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
unsafe extern "C" fn fds_drop(
    mut ps: *mut session_t,
    mut fd: i32,
    mut events: libc::c_short,
) {
    if 0x1 as i32 & events as i32 != 0 && !((*ps).pfds_read).is_null() {
        (*(*ps).pfds_read)
            .fds_bits[(fd
            / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32)) as usize]
            &= !(((1 as u64)
                << fd % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask);
    }
    if 0x4 as i32 & events as i32 != 0 && !((*ps).pfds_write).is_null() {
        (*(*ps).pfds_write)
            .fds_bits[(fd
            / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32)) as usize]
            &= !(((1 as u64)
                << fd % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask);
    }
    if 0x2 as i32 & events as i32 != 0 && !((*ps).pfds_except).is_null() {
        (*(*ps).pfds_except)
            .fds_bits[(fd
            / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32)) as usize]
            &= !(((1 as u64)
                << fd % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask);
    }
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
unsafe extern "C" fn win_is_focused_real(
    mut ps: *mut session_t,
    mut w: *const win,
) -> bool {
    return 2 as i32 == (*w).a.map_state && (*ps).active_win == w as *mut _win;
}
#[inline]
unsafe extern "C" fn find_focused(mut ps: *mut session_t) -> *mut win {
    if !(*ps).o.track_focus {
        return 0 as *mut win;
    }
    if !((*ps).active_win).is_null()
        && win_is_focused_real(ps, (*ps).active_win) as i32 != 0
    {
        return (*ps).active_win;
    }
    return 0 as *mut win;
}
#[inline]
unsafe extern "C" fn cdbus_get_watch_cond(mut watch: *mut DBusWatch) -> libc::c_short {
    let flags: u32 = dbus_watch_get_flags(watch);
    let mut condition: libc::c_short = (0x8 as i32 | 0x10 as i32) as libc::c_short;
    if flags & C2RustUnnamed_0::DBUS_WATCH_READABLE as i32 as u32 != 0 {
        condition = (condition as i32 | 0x1 as i32) as libc::c_short;
    }
    if flags & C2RustUnnamed_0::DBUS_WATCH_WRITABLE as i32 as u32 != 0 {
        condition = (condition as i32 | 0x4 as i32) as libc::c_short;
    }
    return condition;
}
#[inline]
unsafe extern "C" fn cdbus_repr_msgtype(mut msg: *mut DBusMessage) -> *const i8 {
    return dbus_message_type_to_string(dbus_message_get_type(msg));
}
#[inline]
unsafe extern "C" fn cdbus_reply_string(
    mut ps: *mut session_t,
    mut srcmsg: *mut DBusMessage,
    mut str: *const i8,
) -> bool {
    return cdbus_reply(
        ps,
        srcmsg,
        Some(
            cdbus_apdarg_string
                as unsafe extern "C" fn(
                    *mut session_t,
                    *mut DBusMessage,
                    *const libc::c_void,
                ) -> bool,
        ),
        str as *const libc::c_void,
    );
}
#[inline]
unsafe extern "C" fn cdbus_reply_bool(
    mut ps: *mut session_t,
    mut srcmsg: *mut DBusMessage,
    mut bval: bool,
) -> bool {
    return cdbus_reply(
        ps,
        srcmsg,
        Some(
            cdbus_apdarg_bool
                as unsafe extern "C" fn(
                    *mut session_t,
                    *mut DBusMessage,
                    *const libc::c_void,
                ) -> bool,
        ),
        &mut bval as *mut bool as *const libc::c_void,
    );
}
#[inline]
unsafe extern "C" fn cdbus_reply_int32(
    mut ps: *mut session_t,
    mut srcmsg: *mut DBusMessage,
    mut val: int32_t,
) -> bool {
    return cdbus_reply(
        ps,
        srcmsg,
        Some(
            cdbus_apdarg_int32
                as unsafe extern "C" fn(
                    *mut session_t,
                    *mut DBusMessage,
                    *const libc::c_void,
                ) -> bool,
        ),
        &mut val as *mut int32_t as *const libc::c_void,
    );
}
#[inline]
unsafe extern "C" fn cdbus_reply_double(
    mut ps: *mut session_t,
    mut srcmsg: *mut DBusMessage,
    mut val: libc::c_double,
) -> bool {
    return cdbus_reply(
        ps,
        srcmsg,
        Some(
            cdbus_apdarg_double
                as unsafe extern "C" fn(
                    *mut session_t,
                    *mut DBusMessage,
                    *const libc::c_void,
                ) -> bool,
        ),
        &mut val as *mut libc::c_double as *const libc::c_void,
    );
}
#[inline]
unsafe extern "C" fn cdbus_reply_enum(
    mut ps: *mut session_t,
    mut srcmsg: *mut DBusMessage,
    mut eval: cdbus_enum_t,
) -> bool {
    return cdbus_reply(
        ps,
        srcmsg,
        Some(
            cdbus_apdarg_enum
                as unsafe extern "C" fn(
                    *mut session_t,
                    *mut DBusMessage,
                    *const libc::c_void,
                ) -> bool,
        ),
        &mut eval as *mut cdbus_enum_t as *const libc::c_void,
    );
}
#[inline]
unsafe extern "C" fn cdbus_reply_uint32(
    mut ps: *mut session_t,
    mut srcmsg: *mut DBusMessage,
    mut val: uint32_t,
) -> bool {
    return cdbus_reply(
        ps,
        srcmsg,
        Some(
            cdbus_apdarg_uint32
                as unsafe extern "C" fn(
                    *mut session_t,
                    *mut DBusMessage,
                    *const libc::c_void,
                ) -> bool,
        ),
        &mut val as *mut uint32_t as *const libc::c_void,
    );
}
#[inline]
unsafe extern "C" fn cdbus_reply_wid(
    mut ps: *mut session_t,
    mut srcmsg: *mut DBusMessage,
    mut wid: Window,
) -> bool {
    return cdbus_reply(
        ps,
        srcmsg,
        Some(
            cdbus_apdarg_wid
                as unsafe extern "C" fn(
                    *mut session_t,
                    *mut DBusMessage,
                    *const libc::c_void,
                ) -> bool,
        ),
        &mut wid as *mut Window as *const libc::c_void,
    );
}
#[inline]
unsafe extern "C" fn cdbus_signal_wid(
    mut ps: *mut session_t,
    mut name: *const i8,
    mut wid: Window,
) -> bool {
    return cdbus_signal(
        ps,
        name,
        Some(
            cdbus_apdarg_wid
                as unsafe extern "C" fn(
                    *mut session_t,
                    *mut DBusMessage,
                    *const libc::c_void,
                ) -> bool,
        ),
        &mut wid as *mut Window as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cdbus_init(mut ps: *mut session_t) -> bool {
    let mut err: DBusError = {
        let mut init = DBusError {
            dummy1_dummy2_dummy3_dummy4_dummy5: [0; 1],
            c2rust_padding: [0; 7],
            name: 0 as *const i8,
            message: 0 as *const i8,
            padding1: 0 as *mut libc::c_void,
        };
        init.set_dummy1(0);
        init.set_dummy2(0);
        init.set_dummy3(0);
        init.set_dummy4(0);
        init.set_dummy5(0);
        init
    };
    dbus_error_init(&mut err);
    (*ps).dbus_conn = dbus_bus_get_private(DBusBusType::DBUS_BUS_SESSION, &mut err);
    if dbus_error_is_set(&mut err) != 0 {
        fprintf(
            stderr,
            b"%s(): D-Bus connection failed (%s).\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"cdbus_init\0")).as_ptr(),
            err.message,
        );
        dbus_error_free(&mut err);
        return 0 as i32 != 0;
    }
    if ((*ps).dbus_conn).is_null() {
        fprintf(
            stderr,
            b"%s(): D-Bus connection failed for unknown reason.\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"cdbus_init\0")).as_ptr(),
        );
        return 0 as i32 != 0;
    }
    dbus_connection_set_exit_on_disconnect((*ps).dbus_conn, 0 as i32 as dbus_bool_t);
    let mut service: *mut i8 = mstrjoin3(
        b"com.github.chjj.compton\0" as *const u8 as *const i8,
        b".\0" as *const u8 as *const i8,
        (*ps).o.display_repr,
    );
    (*ps).dbus_service = service;
    let mut ret: i32 = dbus_bus_request_name(
        (*ps).dbus_conn,
        service,
        0x4 as i32 as u32,
        &mut err,
    );
    if dbus_error_is_set(&mut err) != 0 {
        fprintf(
            stderr,
            b"%s(): Failed to obtain D-Bus name (%s).\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"cdbus_init\0")).as_ptr(),
            err.message,
        );
        dbus_error_free(&mut err);
    }
    if 1 as i32 != ret && 4 as i32 != ret {
        fprintf(
            stderr,
            b"%s(): Failed to become the primary owner of requested D-Bus name (%d).\n\0"
                as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"cdbus_init\0")).as_ptr(),
            ret,
        );
    }
    if dbus_connection_set_watch_functions(
        (*ps).dbus_conn,
        Some(
            cdbus_callback_add_watch
                as unsafe extern "C" fn(*mut DBusWatch, *mut libc::c_void) -> dbus_bool_t,
        ),
        Some(
            cdbus_callback_remove_watch
                as unsafe extern "C" fn(*mut DBusWatch, *mut libc::c_void) -> (),
        ),
        Some(
            cdbus_callback_watch_toggled
                as unsafe extern "C" fn(*mut DBusWatch, *mut libc::c_void) -> (),
        ),
        ps as *mut libc::c_void,
        None,
    ) == 0
    {
        fprintf(
            stderr,
            b"%s(): Failed to add D-Bus watch functions.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"cdbus_init\0")).as_ptr(),
        );
        return 0 as i32 != 0;
    }
    if dbus_connection_set_timeout_functions(
        (*ps).dbus_conn,
        Some(
            cdbus_callback_add_timeout
                as unsafe extern "C" fn(
                    *mut DBusTimeout,
                    *mut libc::c_void,
                ) -> dbus_bool_t,
        ),
        Some(
            cdbus_callback_remove_timeout
                as unsafe extern "C" fn(*mut DBusTimeout, *mut libc::c_void) -> (),
        ),
        Some(
            cdbus_callback_timeout_toggled
                as unsafe extern "C" fn(*mut DBusTimeout, *mut libc::c_void) -> (),
        ),
        ps as *mut libc::c_void,
        None,
    ) == 0
    {
        fprintf(
            stderr,
            b"%s(): Failed to add D-Bus timeout functions.\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"cdbus_init\0")).as_ptr(),
        );
        return 0 as i32 != 0;
    }
    dbus_bus_add_match(
        (*ps).dbus_conn,
        b"type='method_call',interface='com.github.chjj.compton'\0" as *const u8
            as *const i8,
        &mut err,
    );
    if dbus_error_is_set(&mut err) != 0 {
        fprintf(
            stderr,
            b"%s(): Failed to add D-Bus match.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"cdbus_init\0")).as_ptr(),
        );
        dbus_error_free(&mut err);
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn cdbus_destroy(mut ps: *mut session_t) {
    if !((*ps).dbus_conn).is_null() {
        if !((*ps).dbus_service).is_null() {
            let mut err: DBusError = {
                let mut init = DBusError {
                    dummy1_dummy2_dummy3_dummy4_dummy5: [0; 1],
                    c2rust_padding: [0; 7],
                    name: 0 as *const i8,
                    message: 0 as *const i8,
                    padding1: 0 as *mut libc::c_void,
                };
                init.set_dummy1(0);
                init.set_dummy2(0);
                init.set_dummy3(0);
                init.set_dummy4(0);
                init.set_dummy5(0);
                init
            };
            dbus_error_init(&mut err);
            dbus_bus_release_name((*ps).dbus_conn, (*ps).dbus_service, &mut err);
            if dbus_error_is_set(&mut err) != 0 {
                fprintf(
                    stderr,
                    b"%s(): Failed to release DBus name (%s).\n\0" as *const u8
                        as *const i8,
                    (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"cdbus_destroy\0"))
                        .as_ptr(),
                    err.message,
                );
                dbus_error_free(&mut err);
            }
        }
        dbus_connection_close((*ps).dbus_conn);
        dbus_connection_unref((*ps).dbus_conn);
    }
}
unsafe extern "C" fn cdbus_callback_add_timeout(
    mut timeout: *mut DBusTimeout,
    mut data: *mut libc::c_void,
) -> dbus_bool_t {
    let mut ps: *mut session_t = data as *mut session_t;
    let mut ptmout: *mut timeout_t = timeout_insert(
        ps,
        dbus_timeout_get_interval(timeout) as time_ms_t,
        Some(
            cdbus_callback_handle_timeout
                as unsafe extern "C" fn(*mut session_t, *mut timeout_t) -> bool,
        ),
        timeout as *mut libc::c_void,
    );
    if !ptmout.is_null() {
        dbus_timeout_set_data(timeout, ptmout as *mut libc::c_void, None);
    }
    return !ptmout.is_null() as dbus_bool_t;
}
unsafe extern "C" fn cdbus_callback_remove_timeout(
    mut timeout: *mut DBusTimeout,
    mut data: *mut libc::c_void,
) {
    let mut ps: *mut session_t = data as *mut session_t;
    let mut ptmout: *mut timeout_t = dbus_timeout_get_data(timeout) as *mut timeout_t;
    if !ptmout.is_null() {
        timeout_drop(ps, ptmout);
    }
}
unsafe extern "C" fn cdbus_callback_timeout_toggled(
    mut timeout: *mut DBusTimeout,
    mut data: *mut libc::c_void,
) {
    let mut ptmout: *mut timeout_t = dbus_timeout_get_data(timeout) as *mut timeout_t;
    if !ptmout.is_null() {
        (*ptmout).enabled = dbus_timeout_get_enabled(timeout) != 0;
        (*ptmout).interval = dbus_timeout_get_interval(timeout) as time_ms_t;
    }
}
unsafe extern "C" fn cdbus_callback_handle_timeout(
    mut ps: *mut session_t,
    mut ptmout: *mut timeout_t,
) -> bool {
    if !ptmout.is_null() && !((*ptmout).data).is_null() {
        return dbus_timeout_handle((*ptmout).data as *mut DBusTimeout) != 0;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn cdbus_callback_add_watch(
    mut watch: *mut DBusWatch,
    mut data: *mut libc::c_void,
) -> dbus_bool_t {
    if dbus_watch_get_enabled(watch) == 0 {
        return 1 as i32 as dbus_bool_t;
    }
    let mut ps: *mut session_t = data as *mut session_t;
    fds_insert(ps, dbus_watch_get_unix_fd(watch), cdbus_get_watch_cond(watch));
    return 1 as i32 as dbus_bool_t;
}
unsafe extern "C" fn cdbus_callback_remove_watch(
    mut watch: *mut DBusWatch,
    mut data: *mut libc::c_void,
) {
    let mut ps: *mut session_t = data as *mut session_t;
    fds_drop(ps, dbus_watch_get_unix_fd(watch), cdbus_get_watch_cond(watch));
}
unsafe extern "C" fn cdbus_callback_watch_toggled(
    mut watch: *mut DBusWatch,
    mut data: *mut libc::c_void,
) {
    if dbus_watch_get_enabled(watch) != 0 {
        cdbus_callback_add_watch(watch, data);
    } else {
        cdbus_callback_remove_watch(watch, data);
    };
}
unsafe extern "C" fn cdbus_apdarg_bool(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
    mut data: *const libc::c_void,
) -> bool {
    let mut val: dbus_bool_t = *(data as *const bool) as dbus_bool_t;
    if dbus_message_append_args(
        msg,
        'b' as i32,
        &mut val as *mut dbus_bool_t,
        '\0' as i32,
    ) == 0
    {
        fprintf(
            stderr,
            b"%s(): Failed to append argument.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 18], &[i8; 18]>(b"cdbus_apdarg_bool\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_apdarg_int32(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
    mut data: *const libc::c_void,
) -> bool {
    if dbus_message_append_args(msg, 'i' as i32, data, '\0' as i32) == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to append argument.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"cdbus_apdarg_int32\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_apdarg_uint32(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
    mut data: *const libc::c_void,
) -> bool {
    if dbus_message_append_args(msg, 'u' as i32, data, '\0' as i32) == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to append argument.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 20], &[i8; 20]>(b"cdbus_apdarg_uint32\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_apdarg_double(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
    mut data: *const libc::c_void,
) -> bool {
    if dbus_message_append_args(msg, 'd' as i32, data, '\0' as i32) == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to append argument.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 20], &[i8; 20]>(b"cdbus_apdarg_double\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_apdarg_wid(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
    mut data: *const libc::c_void,
) -> bool {
    let mut val: cdbus_window_t = *(data as *const Window) as cdbus_window_t;
    if dbus_message_append_args(
        msg,
        'u' as i32,
        &mut val as *mut cdbus_window_t,
        '\0' as i32,
    ) == 0
    {
        fprintf(
            stderr,
            b"%s(): Failed to append argument.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 17], &[i8; 17]>(b"cdbus_apdarg_wid\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_apdarg_enum(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
    mut data: *const libc::c_void,
) -> bool {
    if dbus_message_append_args(msg, 'q' as i32, data, '\0' as i32) == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to append argument.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 18], &[i8; 18]>(b"cdbus_apdarg_enum\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_apdarg_string(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
    mut data: *const libc::c_void,
) -> bool {
    let mut str: *const i8 = data as *const i8;
    if str.is_null() {
        str = b"\0" as *const u8 as *const i8;
    }
    if dbus_message_append_args(msg, 's' as i32, &mut str as *mut *const i8, '\0' as i32)
        == 0
    {
        fprintf(
            stderr,
            b"%s(): Failed to append argument.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 20], &[i8; 20]>(b"cdbus_apdarg_string\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_apdarg_wids(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
    mut data: *const libc::c_void,
) -> bool {
    let mut count: u32 = 0 as i32 as u32;
    let mut w: *mut win = (*ps).list;
    while !w.is_null() {
        if !(*w).destroyed {
            count = count.wrapping_add(1);
            count;
        }
        w = (*w).next;
    }
    let mut arr: *mut cdbus_window_t = malloc(
        (::core::mem::size_of::<cdbus_window_t>() as u64).wrapping_mul(count as u64),
    ) as *mut cdbus_window_t;
    if arr.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed to allocate memory for window ID array.\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 18], &[i8; 18]>(b"cdbus_apdarg_wids\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    let mut pcur: *mut cdbus_window_t = arr;
    let mut w_0: *mut win = (*ps).list;
    while !w_0.is_null() {
        if !(*w_0).destroyed {
            *pcur = (*w_0).id as cdbus_window_t;
            pcur = pcur.offset(1);
            pcur;
        }
        w_0 = (*w_0).next;
    }
    if dbus_message_append_args(
        msg,
        'a' as i32,
        'u' as i32,
        &mut arr as *mut *mut cdbus_window_t,
        count,
        '\0' as i32,
    ) == 0
    {
        fprintf(
            stderr,
            b"%s(): Failed to append argument.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 18], &[i8; 18]>(b"cdbus_apdarg_wids\0"))
                .as_ptr(),
        );
        free(arr as *mut libc::c_void);
        return 0 as i32 != 0;
    }
    free(arr as *mut libc::c_void);
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_signal(
    mut ps: *mut session_t,
    mut name: *const i8,
    mut func: Option<
        unsafe extern "C" fn(
            *mut session_t,
            *mut DBusMessage,
            *const libc::c_void,
        ) -> bool,
    >,
    mut data: *const libc::c_void,
) -> bool {
    let mut msg: *mut DBusMessage = 0 as *mut DBusMessage;
    msg = dbus_message_new_signal(
        b"/com/github/chjj/compton\0" as *const u8 as *const i8,
        b"com.github.chjj.compton\0" as *const u8 as *const i8,
        name,
    );
    if msg.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed to create D-Bus signal.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 13], &[i8; 13]>(b"cdbus_signal\0")).as_ptr(),
        );
        return 0 as i32 != 0;
    }
    if func.is_some() && !func.expect("non-null function pointer")(ps, msg, data) {
        dbus_message_unref(msg);
        return 0 as i32 != 0;
    }
    if dbus_connection_send((*ps).dbus_conn, msg, 0 as *mut dbus_uint32_t) == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to send D-Bus signal.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 13], &[i8; 13]>(b"cdbus_signal\0")).as_ptr(),
        );
        dbus_message_unref(msg);
        return 0 as i32 != 0;
    }
    dbus_connection_flush((*ps).dbus_conn);
    dbus_message_unref(msg);
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_reply(
    mut ps: *mut session_t,
    mut srcmsg: *mut DBusMessage,
    mut func: Option<
        unsafe extern "C" fn(
            *mut session_t,
            *mut DBusMessage,
            *const libc::c_void,
        ) -> bool,
    >,
    mut data: *const libc::c_void,
) -> bool {
    let mut msg: *mut DBusMessage = 0 as *mut DBusMessage;
    msg = dbus_message_new_method_return(srcmsg);
    if msg.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed to create D-Bus reply.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 12], &[i8; 12]>(b"cdbus_reply\0")).as_ptr(),
        );
        return 0 as i32 != 0;
    }
    if func.is_some() && !func.expect("non-null function pointer")(ps, msg, data) {
        dbus_message_unref(msg);
        return 0 as i32 != 0;
    }
    if dbus_connection_send((*ps).dbus_conn, msg, 0 as *mut dbus_uint32_t) == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to send D-Bus reply.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 12], &[i8; 12]>(b"cdbus_reply\0")).as_ptr(),
        );
        dbus_message_unref(msg);
        return 0 as i32 != 0;
    }
    dbus_connection_flush((*ps).dbus_conn);
    dbus_message_unref(msg);
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_reply_errm(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
) -> bool {
    if msg.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed to create D-Bus reply.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 17], &[i8; 17]>(b"cdbus_reply_errm\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    if dbus_connection_send((*ps).dbus_conn, msg, 0 as *mut dbus_uint32_t) == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to send D-Bus reply.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 17], &[i8; 17]>(b"cdbus_reply_errm\0"))
                .as_ptr(),
        );
        dbus_message_unref(msg);
        return 0 as i32 != 0;
    }
    dbus_connection_flush((*ps).dbus_conn);
    dbus_message_unref(msg);
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_msg_get_arg(
    mut msg: *mut DBusMessage,
    mut count: i32,
    type_0: i32,
    mut pdest: *mut libc::c_void,
) -> bool {
    let mut iter: DBusMessageIter = {
        let mut init = DBusMessageIter {
            dummy1: 0 as *mut libc::c_void,
            dummy2: 0 as *mut libc::c_void,
            dummy3: 0,
            dummy4: 0,
            dummy5: 0,
            dummy6: 0,
            dummy7: 0,
            dummy8: 0,
            dummy9: 0,
            dummy10: 0,
            dummy11: 0,
            pad1: 0,
            pad2: 0 as *mut libc::c_void,
            pad3: 0 as *mut libc::c_void,
        };
        init
    };
    if dbus_message_iter_init(msg, &mut iter) == 0 {
        fprintf(
            stderr,
            b"%s(): Message has no argument.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 18], &[i8; 18]>(b"cdbus_msg_get_arg\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    let oldcount: i32 = count;
    while count != 0 {
        if dbus_message_iter_next(&mut iter) == 0 {
            fprintf(
                stderr,
                b"%s(): Failed to find argument %d.\n\0" as *const u8 as *const i8,
                (*::core::mem::transmute::<&[u8; 18], &[i8; 18]>(b"cdbus_msg_get_arg\0"))
                    .as_ptr(),
                oldcount,
            );
            return 0 as i32 != 0;
        }
        count -= 1;
        count;
    }
    if type_0 != dbus_message_iter_get_arg_type(&mut iter) {
        fprintf(
            stderr,
            b"%s(): Argument has incorrect type.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 18], &[i8; 18]>(b"cdbus_msg_get_arg\0"))
                .as_ptr(),
        );
        return 0 as i32 != 0;
    }
    dbus_message_iter_get_basic(&mut iter, pdest);
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn cdbus_loop(mut ps: *mut session_t) {
    dbus_connection_read_write((*ps).dbus_conn, 0 as i32);
    let mut msg: *mut DBusMessage = 0 as *mut DBusMessage;
    loop {
        msg = dbus_connection_pop_message((*ps).dbus_conn);
        if msg.is_null() {
            break;
        }
        cdbus_process(ps, msg);
    };
}
unsafe extern "C" fn cdbus_process(mut ps: *mut session_t, mut msg: *mut DBusMessage) {
    let mut success: bool = 0 as i32 != 0;
    if dbus_message_is_method_call(
        msg,
        b"com.github.chjj.compton\0" as *const u8 as *const i8,
        b"reset\0" as *const u8 as *const i8,
    ) != 0
    {
        (*ps).reset = 1 as i32 != 0;
        if dbus_message_get_no_reply(msg) == 0 {
            cdbus_reply_bool(ps, msg, 1 as i32 != 0);
        }
        success = 1 as i32 != 0;
    } else if dbus_message_is_method_call(
        msg,
        b"com.github.chjj.compton\0" as *const u8 as *const i8,
        b"repaint\0" as *const u8 as *const i8,
    ) != 0
    {
        force_repaint(ps);
        if dbus_message_get_no_reply(msg) == 0 {
            cdbus_reply_bool(ps, msg, 1 as i32 != 0);
        }
        success = 1 as i32 != 0;
    } else if dbus_message_is_method_call(
        msg,
        b"com.github.chjj.compton\0" as *const u8 as *const i8,
        b"list_win\0" as *const u8 as *const i8,
    ) != 0
    {
        success = cdbus_process_list_win(ps, msg);
    } else if dbus_message_is_method_call(
        msg,
        b"com.github.chjj.compton\0" as *const u8 as *const i8,
        b"win_get\0" as *const u8 as *const i8,
    ) != 0
    {
        success = cdbus_process_win_get(ps, msg);
    } else if dbus_message_is_method_call(
        msg,
        b"com.github.chjj.compton\0" as *const u8 as *const i8,
        b"win_set\0" as *const u8 as *const i8,
    ) != 0
    {
        success = cdbus_process_win_set(ps, msg);
    } else if dbus_message_is_method_call(
        msg,
        b"com.github.chjj.compton\0" as *const u8 as *const i8,
        b"find_win\0" as *const u8 as *const i8,
    ) != 0
    {
        success = cdbus_process_find_win(ps, msg);
    } else if dbus_message_is_method_call(
        msg,
        b"com.github.chjj.compton\0" as *const u8 as *const i8,
        b"opts_get\0" as *const u8 as *const i8,
    ) != 0
    {
        success = cdbus_process_opts_get(ps, msg);
    } else if dbus_message_is_method_call(
        msg,
        b"com.github.chjj.compton\0" as *const u8 as *const i8,
        b"opts_set\0" as *const u8 as *const i8,
    ) != 0
    {
        success = cdbus_process_opts_set(ps, msg);
    } else if dbus_message_is_method_call(
        msg,
        b"org.freedesktop.DBus.Introspectable\0" as *const u8 as *const i8,
        b"Introspect\0" as *const u8 as *const i8,
    ) != 0
    {
        success = cdbus_process_introspect(ps, msg);
    } else if dbus_message_is_method_call(
        msg,
        b"org.freedesktop.DBus.Peer\0" as *const u8 as *const i8,
        b"Ping\0" as *const u8 as *const i8,
    ) != 0
    {
        cdbus_reply(ps, msg, None, 0 as *const libc::c_void);
        success = 1 as i32 != 0;
    } else if dbus_message_is_method_call(
        msg,
        b"org.freedesktop.DBus.Peer\0" as *const u8 as *const i8,
        b"GetMachineId\0" as *const u8 as *const i8,
    ) != 0
    {
        let mut uuid: *mut i8 = dbus_get_local_machine_id();
        if !uuid.is_null() {
            cdbus_reply_string(ps, msg, uuid);
            dbus_free(uuid as *mut libc::c_void);
            success = 1 as i32 != 0;
        }
    } else if dbus_message_is_signal(
        msg,
        b"org.freedesktop.DBus\0" as *const u8 as *const i8,
        b"NameAcquired\0" as *const u8 as *const i8,
    ) != 0
        || dbus_message_is_signal(
            msg,
            b"org.freedesktop.DBus\0" as *const u8 as *const i8,
            b"NameLost\0" as *const u8 as *const i8,
        ) != 0
    {
        success = 1 as i32 != 0;
    } else {
        if 3 as i32 == dbus_message_get_type(msg) {
            fprintf(
                stderr,
                b"%s(): Error message of path \"%s\" interface \"%s\", member \"%s\", error \"%s\"\n\0"
                    as *const u8 as *const i8,
                (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"cdbus_process\0"))
                    .as_ptr(),
                dbus_message_get_path(msg),
                dbus_message_get_interface(msg),
                dbus_message_get_member(msg),
                dbus_message_get_error_name(msg),
            );
        } else {
            fprintf(
                stderr,
                b"%s(): Illegal message of type \"%s\", path \"%s\" interface \"%s\", member \"%s\"\n\0"
                    as *const u8 as *const i8,
                (*::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"cdbus_process\0"))
                    .as_ptr(),
                cdbus_repr_msgtype(msg),
                dbus_message_get_path(msg),
                dbus_message_get_interface(msg),
                dbus_message_get_member(msg),
            );
        }
        if 1 as i32 == dbus_message_get_type(msg) && dbus_message_get_no_reply(msg) == 0
        {
            cdbus_reply_errm(
                ps,
                dbus_message_new_error_printf(
                    msg,
                    b"com.github.chjj.compton.error.bad_message\0" as *const u8
                        as *const i8,
                    b"Unrecognized command. Beware compton cannot make you a sandwich.\0"
                        as *const u8 as *const i8,
                ),
            );
        }
        success = 1 as i32 != 0;
    }
    if !success && 1 as i32 == dbus_message_get_type(msg)
        && dbus_message_get_no_reply(msg) == 0
    {
        cdbus_reply_errm(
            ps,
            dbus_message_new_error_printf(
                msg,
                b"com.github.chjj.compton.error.unknown\0" as *const u8 as *const i8,
                b"Well, I don't know what happened. Do you?\0" as *const u8 as *const i8,
            ),
        );
    }
    dbus_message_unref(msg);
}
unsafe extern "C" fn cdbus_process_list_win(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
) -> bool {
    cdbus_reply(
        ps,
        msg,
        Some(
            cdbus_apdarg_wids
                as unsafe extern "C" fn(
                    *mut session_t,
                    *mut DBusMessage,
                    *const libc::c_void,
                ) -> bool,
        ),
        0 as *const libc::c_void,
    );
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_process_win_get(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
) -> bool {
    let mut wid: cdbus_window_t = 0 as i64 as cdbus_window_t;
    let mut target: *const i8 = 0 as *const i8;
    let mut err: DBusError = {
        let mut init = DBusError {
            dummy1_dummy2_dummy3_dummy4_dummy5: [0; 1],
            c2rust_padding: [0; 7],
            name: 0 as *const i8,
            message: 0 as *const i8,
            padding1: 0 as *mut libc::c_void,
        };
        init.set_dummy1(0);
        init.set_dummy2(0);
        init.set_dummy3(0);
        init.set_dummy4(0);
        init.set_dummy5(0);
        init
    };
    if dbus_message_get_args(
        msg,
        &mut err as *mut DBusError,
        'u' as i32,
        &mut wid as *mut cdbus_window_t,
        's' as i32,
        &mut target as *mut *const i8,
        '\0' as i32,
    ) == 0
    {
        fprintf(
            stderr,
            b"%s(): Failed to parse argument of \"win_get\" (%s).\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"cdbus_process_win_get\0"))
                .as_ptr(),
            err.message,
        );
        dbus_error_free(&mut err);
        return 0 as i32 != 0;
    }
    let mut w: *mut win = find_win(ps, wid as Window);
    if w.is_null() {
        fprintf(
            stderr,
            b"%s(): Window %#010x not found.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"cdbus_process_win_get\0"))
                .as_ptr(),
            wid,
        );
        cdbus_reply_errm(
            ps,
            dbus_message_new_error_printf(
                msg,
                b"com.github.chjj.compton.error.bad_window\0" as *const u8 as *const i8,
                b"Requested window %#010lx not found.\0" as *const u8 as *const i8,
                wid,
            ),
        );
        return 1 as i32 != 0;
    }
    if strcmp(b"id\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_wid(ps, msg, (*w).id);
        return 1 as i32 != 0;
    }
    if strcmp(b"next\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_wid(
            ps,
            msg,
            if !((*w).next).is_null() { (*(*w).next).id } else { 0 as i32 as u64 },
        );
        return 1 as i32 != 0;
    }
    if strcmp(b"map_state\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*w).a.map_state != 0);
        return 1 as i32 != 0;
    }
    if strcmp(b"mode\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_enum(ps, msg, (*w).mode as cdbus_enum_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"client_win\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_wid(ps, msg, (*w).client_win);
        return 1 as i32 != 0;
    }
    if strcmp(b"damaged\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*w).damaged);
        return 1 as i32 != 0;
    }
    if strcmp(b"destroyed\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*w).destroyed);
        return 1 as i32 != 0;
    }
    if strcmp(b"window_type\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_enum(ps, msg, (*w).window_type as cdbus_enum_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"wmwin\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*w).wmwin);
        return 1 as i32 != 0;
    }
    if strcmp(b"leader\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_wid(ps, msg, (*w).leader);
        return 1 as i32 != 0;
    }
    if strcmp(b"focused_real\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, win_is_focused_real(ps, w));
        return 1 as i32 != 0;
    }
    if strcmp(b"fade_force\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_enum(ps, msg, (*w).fade_force as cdbus_enum_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"shadow_force\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_enum(ps, msg, (*w).shadow_force as cdbus_enum_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"focused_force\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_enum(ps, msg, (*w).focused_force as cdbus_enum_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"invert_color_force\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_enum(ps, msg, (*w).invert_color_force as cdbus_enum_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"name\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_string(ps, msg, (*w).name);
        return 1 as i32 != 0;
    }
    if strcmp(b"class_instance\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_string(ps, msg, (*w).class_instance);
        return 1 as i32 != 0;
    }
    if strcmp(b"class_general\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_string(ps, msg, (*w).class_general);
        return 1 as i32 != 0;
    }
    if strcmp(b"role\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_string(ps, msg, (*w).role);
        return 1 as i32 != 0;
    }
    if strcmp(b"opacity\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_uint32(ps, msg, (*w).opacity);
        return 1 as i32 != 0;
    }
    if strcmp(b"opacity_tgt\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_uint32(ps, msg, (*w).opacity_tgt);
        return 1 as i32 != 0;
    }
    if strcmp(b"opacity_prop\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_uint32(ps, msg, (*w).opacity_prop);
        return 1 as i32 != 0;
    }
    if strcmp(b"opacity_prop_client\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_uint32(ps, msg, (*w).opacity_prop_client);
        return 1 as i32 != 0;
    }
    if strcmp(b"opacity_set\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_uint32(ps, msg, (*w).opacity_set);
        return 1 as i32 != 0;
    }
    if strcmp(b"frame_opacity\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_double(ps, msg, (*w).frame_opacity);
        return 1 as i32 != 0;
    }
    if strcmp(b"left_width\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_uint32(ps, msg, (*w).frame_extents.left as uint32_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"right_width\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_uint32(ps, msg, (*w).frame_extents.right as uint32_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"top_width\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_uint32(ps, msg, (*w).frame_extents.top as uint32_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"bottom_width\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_uint32(ps, msg, (*w).frame_extents.bottom as uint32_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"shadow\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*w).shadow);
        return 1 as i32 != 0;
    }
    if strcmp(b"fade\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*w).fade);
        return 1 as i32 != 0;
    }
    if strcmp(b"invert_color\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*w).invert_color);
        return 1 as i32 != 0;
    }
    if strcmp(b"blur_background\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*w).blur_background);
        return 1 as i32 != 0;
    }
    fprintf(
        stderr,
        b"%s(): Target \"%s\" not found.\n\0" as *const u8 as *const i8,
        (*::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"cdbus_process_win_get\0"))
            .as_ptr(),
        target,
    );
    cdbus_reply_errm(
        ps,
        dbus_message_new_error_printf(
            msg,
            b"com.github.chjj.compton.error.bad_target\0" as *const u8 as *const i8,
            b"Target \"%s\" not found.\0" as *const u8 as *const i8,
            target,
        ),
    );
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_process_win_set(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
) -> bool {
    let mut wid: cdbus_window_t = 0 as i64 as cdbus_window_t;
    let mut target: *const i8 = 0 as *const i8;
    let mut err: DBusError = {
        let mut init = DBusError {
            dummy1_dummy2_dummy3_dummy4_dummy5: [0; 1],
            c2rust_padding: [0; 7],
            name: 0 as *const i8,
            message: 0 as *const i8,
            padding1: 0 as *mut libc::c_void,
        };
        init.set_dummy1(0);
        init.set_dummy2(0);
        init.set_dummy3(0);
        init.set_dummy4(0);
        init.set_dummy5(0);
        init
    };
    if dbus_message_get_args(
        msg,
        &mut err as *mut DBusError,
        'u' as i32,
        &mut wid as *mut cdbus_window_t,
        's' as i32,
        &mut target as *mut *const i8,
        '\0' as i32,
    ) == 0
    {
        fprintf(
            stderr,
            b"%s(): Failed to parse argument of \"win_set\" (%s).\n\0" as *const u8
                as *const i8,
            (*::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"cdbus_process_win_set\0"))
                .as_ptr(),
            err.message,
        );
        dbus_error_free(&mut err);
        return 0 as i32 != 0;
    }
    let mut w: *mut win = find_win(ps, wid as Window);
    if w.is_null() {
        fprintf(
            stderr,
            b"%s(): Window %#010x not found.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"cdbus_process_win_set\0"))
                .as_ptr(),
            wid,
        );
        cdbus_reply_errm(
            ps,
            dbus_message_new_error_printf(
                msg,
                b"com.github.chjj.compton.error.bad_window\0" as *const u8 as *const i8,
                b"Requested window %#010lx not found.\0" as *const u8 as *const i8,
                wid,
            ),
        );
        return 1 as i32 != 0;
    }
    if strcmp(b"shadow_force\0" as *const u8 as *const i8, target) == 0 {
        let mut val: cdbus_enum_t = switch_t::UNSET as i32 as cdbus_enum_t;
        if !cdbus_msg_get_arg(
            msg,
            2 as i32,
            'q' as i32,
            &mut val as *mut cdbus_enum_t as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        win_set_shadow_force(ps, w, switch_t::from_libc_c_uint(val as u32));
    } else if strcmp(b"fade_force\0" as *const u8 as *const i8, target) == 0 {
        let mut val_0: cdbus_enum_t = switch_t::UNSET as i32 as cdbus_enum_t;
        if !cdbus_msg_get_arg(
            msg,
            2 as i32,
            'q' as i32,
            &mut val_0 as *mut cdbus_enum_t as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        win_set_fade_force(ps, w, switch_t::from_libc_c_uint(val_0 as u32));
    } else if strcmp(b"focused_force\0" as *const u8 as *const i8, target) == 0 {
        let mut val_1: cdbus_enum_t = switch_t::UNSET as i32 as cdbus_enum_t;
        if !cdbus_msg_get_arg(
            msg,
            2 as i32,
            'q' as i32,
            &mut val_1 as *mut cdbus_enum_t as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        win_set_focused_force(ps, w, switch_t::from_libc_c_uint(val_1 as u32));
    } else if strcmp(b"invert_color_force\0" as *const u8 as *const i8, target) == 0 {
        let mut val_2: cdbus_enum_t = switch_t::UNSET as i32 as cdbus_enum_t;
        if !cdbus_msg_get_arg(
            msg,
            2 as i32,
            'q' as i32,
            &mut val_2 as *mut cdbus_enum_t as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        win_set_invert_color_force(ps, w, switch_t::from_libc_c_uint(val_2 as u32));
    } else {
        fprintf(
            stderr,
            b"%s(): Target \"%s\" not found.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"cdbus_process_win_set\0"))
                .as_ptr(),
            target,
        );
        cdbus_reply_errm(
            ps,
            dbus_message_new_error_printf(
                msg,
                b"com.github.chjj.compton.error.bad_target\0" as *const u8 as *const i8,
                b"Target \"%s\" not found.\0" as *const u8 as *const i8,
                target,
            ),
        );
        return 1 as i32 != 0;
    }
    if dbus_message_get_no_reply(msg) == 0 {
        cdbus_reply_bool(ps, msg, 1 as i32 != 0);
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_process_find_win(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
) -> bool {
    let mut target: *const i8 = 0 as *const i8;
    if !cdbus_msg_get_arg(
        msg,
        0 as i32,
        's' as i32,
        &mut target as *mut *const i8 as *mut libc::c_void,
    ) {
        return 0 as i32 != 0;
    }
    let mut wid: Window = 0 as i64 as Window;
    if strcmp(b"client\0" as *const u8 as *const i8, target) == 0 {
        let mut client: cdbus_window_t = 0 as i64 as cdbus_window_t;
        if !cdbus_msg_get_arg(
            msg,
            1 as i32,
            'u' as i32,
            &mut client as *mut cdbus_window_t as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        let mut w: *mut win = find_toplevel(ps, client as Window);
        if !w.is_null() {
            wid = (*w).id;
        }
    } else if strcmp(b"focused\0" as *const u8 as *const i8, target) == 0 {
        let mut w_0: *mut win = find_focused(ps);
        if !w_0.is_null() {
            wid = (*w_0).id;
        }
    } else {
        fprintf(
            stderr,
            b"%s(): Target \"%s\" not found.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[i8; 23],
            >(b"cdbus_process_find_win\0"))
                .as_ptr(),
            target,
        );
        cdbus_reply_errm(
            ps,
            dbus_message_new_error_printf(
                msg,
                b"com.github.chjj.compton.error.bad_target\0" as *const u8 as *const i8,
                b"Target \"%s\" not found.\0" as *const u8 as *const i8,
                target,
            ),
        );
        return 1 as i32 != 0;
    }
    cdbus_reply_wid(ps, msg, wid);
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_process_opts_get(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
) -> bool {
    let mut target: *const i8 = 0 as *const i8;
    if !cdbus_msg_get_arg(
        msg,
        0 as i32,
        's' as i32,
        &mut target as *mut *const i8 as *mut libc::c_void,
    ) {
        return 0 as i32 != 0;
    }
    if strcmp(b"version\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_string(
            ps,
            msg,
            b"git-5c9b648b-dirty-2025-03-16\0" as *const u8 as *const i8,
        );
        return 1 as i32 != 0;
    }
    if strcmp(b"pid\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_int32(ps, msg, getpid());
        return 1 as i32 != 0;
    }
    if strcmp(b"display\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_string(ps, msg, (*((*ps).dpy as _XPrivDisplay)).display_name);
        return 1 as i32 != 0;
    }
    if strcmp(b"config_file\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_string(ps, msg, (*ps).o.config_file);
        return 1 as i32 != 0;
    }
    if strcmp(b"display_repr\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_string(ps, msg, (*ps).o.display_repr);
        return 1 as i32 != 0;
    }
    if strcmp(b"write_pid_path\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_string(ps, msg, (*ps).o.write_pid_path);
        return 1 as i32 != 0;
    }
    if strcmp(b"mark_wmwin_focused\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.mark_wmwin_focused);
        return 1 as i32 != 0;
    }
    if strcmp(b"mark_ovredir_focused\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.mark_ovredir_focused);
        return 1 as i32 != 0;
    }
    if strcmp(b"fork_after_register\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.fork_after_register);
        return 1 as i32 != 0;
    }
    if strcmp(b"detect_rounded_corners\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.detect_rounded_corners);
        return 1 as i32 != 0;
    }
    if strcmp(b"paint_on_overlay\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.paint_on_overlay);
        return 1 as i32 != 0;
    }
    if strcmp(b"paint_on_overlay_id\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_uint32(ps, msg, (*ps).overlay as uint32_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"unredir_if_possible\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.unredir_if_possible);
        return 1 as i32 != 0;
    }
    if strcmp(b"unredir_if_possible_delay\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_int32(ps, msg, (*ps).o.unredir_if_possible_delay as int32_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"redirected_force\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_enum(ps, msg, (*ps).o.redirected_force as cdbus_enum_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"stoppaint_force\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_enum(ps, msg, (*ps).o.stoppaint_force as cdbus_enum_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"logpath\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_string(ps, msg, (*ps).o.logpath);
        return 1 as i32 != 0;
    }
    if strcmp(b"synchronize\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.synchronize);
        return 1 as i32 != 0;
    }
    if strcmp(b"refresh_rate\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_int32(ps, msg, (*ps).o.refresh_rate);
        return 1 as i32 != 0;
    }
    if strcmp(b"sw_opti\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.sw_opti);
        return 1 as i32 != 0;
    }
    if strcmp(b"vsync\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_string(ps, msg, VSYNC_STRS[(*ps).o.vsync as usize]);
        return 1 as i32 != 0;
    }
    if strcmp(b"backend\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_string(ps, msg, BACKEND_STRS[(*ps).o.backend as usize]);
        return 1 as i32 != 0;
    }
    if strcmp(b"dbe\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.dbe);
        return 1 as i32 != 0;
    }
    if strcmp(b"vsync_aggressive\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.vsync_aggressive);
        return 1 as i32 != 0;
    }
    if strcmp(b"shadow_red\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_double(ps, msg, (*ps).o.shadow_red);
        return 1 as i32 != 0;
    }
    if strcmp(b"shadow_green\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_double(ps, msg, (*ps).o.shadow_green);
        return 1 as i32 != 0;
    }
    if strcmp(b"shadow_blue\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_double(ps, msg, (*ps).o.shadow_blue);
        return 1 as i32 != 0;
    }
    if strcmp(b"shadow_radius\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_int32(ps, msg, (*ps).o.shadow_radius);
        return 1 as i32 != 0;
    }
    if strcmp(b"shadow_offset_x\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_int32(ps, msg, (*ps).o.shadow_offset_x);
        return 1 as i32 != 0;
    }
    if strcmp(b"shadow_offset_y\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_int32(ps, msg, (*ps).o.shadow_offset_y);
        return 1 as i32 != 0;
    }
    if strcmp(b"shadow_opacity\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_double(ps, msg, (*ps).o.shadow_opacity);
        return 1 as i32 != 0;
    }
    if strcmp(b"clear_shadow\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.clear_shadow);
        return 1 as i32 != 0;
    }
    if strcmp(b"xinerama_shadow_crop\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.xinerama_shadow_crop);
        return 1 as i32 != 0;
    }
    if strcmp(b"fade_delta\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_int32(ps, msg, (*ps).o.fade_delta as int32_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"fade_in_step\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_int32(ps, msg, (*ps).o.fade_in_step as int32_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"fade_out_step\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_int32(ps, msg, (*ps).o.fade_out_step as int32_t);
        return 1 as i32 != 0;
    }
    if strcmp(b"no_fading_openclose\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.no_fading_openclose);
        return 1 as i32 != 0;
    }
    if strcmp(b"blur_background\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.blur_background);
        return 1 as i32 != 0;
    }
    if strcmp(b"blur_background_frame\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.blur_background_frame);
        return 1 as i32 != 0;
    }
    if strcmp(b"blur_background_fixed\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.blur_background_fixed);
        return 1 as i32 != 0;
    }
    if strcmp(b"inactive_dim\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_double(ps, msg, (*ps).o.inactive_dim);
        return 1 as i32 != 0;
    }
    if strcmp(b"inactive_dim_fixed\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.inactive_dim_fixed);
        return 1 as i32 != 0;
    }
    if strcmp(b"use_ewmh_active_win\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.use_ewmh_active_win);
        return 1 as i32 != 0;
    }
    if strcmp(b"detect_transient\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.detect_transient);
        return 1 as i32 != 0;
    }
    if strcmp(b"detect_client_leader\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.detect_client_leader);
        return 1 as i32 != 0;
    }
    if strcmp(b"glx_no_stencil\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.glx_no_stencil);
        return 1 as i32 != 0;
    }
    if strcmp(b"glx_copy_from_front\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.glx_copy_from_front);
        return 1 as i32 != 0;
    }
    if strcmp(b"glx_use_copysubbuffermesa\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.glx_use_copysubbuffermesa);
        return 1 as i32 != 0;
    }
    if strcmp(b"glx_no_rebind_pixmap\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.glx_no_rebind_pixmap);
        return 1 as i32 != 0;
    }
    if strcmp(b"glx_swap_method\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_int32(ps, msg, (*ps).o.glx_swap_method);
        return 1 as i32 != 0;
    }
    if strcmp(b"track_focus\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.track_focus);
        return 1 as i32 != 0;
    }
    if strcmp(b"track_wdata\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.track_wdata);
        return 1 as i32 != 0;
    }
    if strcmp(b"track_leader\0" as *const u8 as *const i8, target) == 0 {
        cdbus_reply_bool(ps, msg, (*ps).o.track_leader);
        return 1 as i32 != 0;
    }
    fprintf(
        stderr,
        b"%s(): Target \"%s\" not found.\n\0" as *const u8 as *const i8,
        (*::core::mem::transmute::<&[u8; 23], &[i8; 23]>(b"cdbus_process_opts_get\0"))
            .as_ptr(),
        target,
    );
    cdbus_reply_errm(
        ps,
        dbus_message_new_error_printf(
            msg,
            b"com.github.chjj.compton.error.bad_target\0" as *const u8 as *const i8,
            b"Target \"%s\" not found.\0" as *const u8 as *const i8,
            target,
        ),
    );
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_process_opts_set(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
) -> bool {
    let mut current_block: u64;
    let mut target: *const i8 = 0 as *const i8;
    if !cdbus_msg_get_arg(
        msg,
        0 as i32,
        's' as i32,
        &mut target as *mut *const i8 as *mut libc::c_void,
    ) {
        return 0 as i32 != 0;
    }
    if strcmp(b"fade_delta\0" as *const u8 as *const i8, target) == 0 {
        let mut val: int32_t = 0.0f64 as int32_t;
        if !cdbus_msg_get_arg(
            msg,
            1 as i32,
            'i' as i32,
            &mut val as *mut int32_t as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        (*ps).o.fade_delta = max_i(val, 1 as i32) as time_ms_t;
    } else if strcmp(b"fade_in_step\0" as *const u8 as *const i8, target) == 0 {
        let mut val_0: libc::c_double = 0.0f64;
        if !cdbus_msg_get_arg(
            msg,
            1 as i32,
            'd' as i32,
            &mut val_0 as *mut libc::c_double as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        (*ps).o.fade_in_step = (normalize_d(val_0) * 0xffffffff as u32 as libc::c_double)
            as opacity_t;
    } else if strcmp(b"fade_out_step\0" as *const u8 as *const i8, target) == 0 {
        let mut val_1: libc::c_double = 0.0f64;
        if !cdbus_msg_get_arg(
            msg,
            1 as i32,
            'd' as i32,
            &mut val_1 as *mut libc::c_double as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        (*ps).o.fade_out_step = (normalize_d(val_1)
            * 0xffffffff as u32 as libc::c_double) as opacity_t;
    } else if strcmp(b"no_fading_openclose\0" as *const u8 as *const i8, target) == 0 {
        let mut val_2: dbus_bool_t = 0 as i32 as dbus_bool_t;
        if !cdbus_msg_get_arg(
            msg,
            1 as i32,
            'b' as i32,
            &mut val_2 as *mut dbus_bool_t as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        opts_set_no_fading_openclose(ps, val_2 != 0);
    } else if strcmp(b"unredir_if_possible\0" as *const u8 as *const i8, target) == 0 {
        let mut val_3: dbus_bool_t = 0 as i32 as dbus_bool_t;
        if !cdbus_msg_get_arg(
            msg,
            1 as i32,
            'b' as i32,
            &mut val_3 as *mut dbus_bool_t as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        if (*ps).o.unredir_if_possible as u32 != val_3 {
            (*ps).o.unredir_if_possible = val_3 != 0;
            (*ps).ev_received = 1 as i32 != 0;
        }
    } else if strcmp(b"clear_shadow\0" as *const u8 as *const i8, target) == 0 {
        let mut val_4: dbus_bool_t = 0 as i32 as dbus_bool_t;
        if !cdbus_msg_get_arg(
            msg,
            1 as i32,
            'b' as i32,
            &mut val_4 as *mut dbus_bool_t as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        if (*ps).o.clear_shadow as u32 != val_4 {
            (*ps).o.clear_shadow = val_4 != 0;
            force_repaint(ps);
        }
    } else if strcmp(b"track_focus\0" as *const u8 as *const i8, target) == 0 {
        let mut val_5: dbus_bool_t = 0 as i32 as dbus_bool_t;
        if !cdbus_msg_get_arg(
            msg,
            1 as i32,
            'b' as i32,
            &mut val_5 as *mut dbus_bool_t as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        if val_5 != 0 {
            opts_init_track_focus(ps);
        }
    } else if strcmp(b"vsync\0" as *const u8 as *const i8, target) == 0 {
        let mut val_6: *const i8 = 0 as *const i8;
        if !cdbus_msg_get_arg(
            msg,
            1 as i32,
            's' as i32,
            &mut val_6 as *mut *const i8 as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        vsync_deinit(ps);
        if !parse_vsync(ps, val_6) {
            fprintf(
                stderr,
                b"%s(): Failed to parse argument %d: %s\n\0" as *const u8 as *const i8,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[i8; 23],
                >(b"cdbus_process_opts_set\0"))
                    .as_ptr(),
                1 as i32,
                b"Value invalid.\0" as *const u8 as *const i8,
            );
            cdbus_reply_errm(
                ps,
                dbus_message_new_error_printf(
                    msg,
                    b"com.github.chjj.compton.error.bad_argument\0" as *const u8
                        as *const i8,
                    b"Failed to parse argument %d: %s\0" as *const u8 as *const i8,
                    1 as i32,
                    b"Value invalid.\0" as *const u8 as *const i8,
                ),
            );
            current_block = 1847472278776910194;
        } else if !vsync_init(ps) {
            fprintf(
                stderr,
                b"%s(): %s\n\0" as *const u8 as *const i8,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[i8; 23],
                >(b"cdbus_process_opts_set\0"))
                    .as_ptr(),
                b"Failed to initialize specified VSync method.\0" as *const u8
                    as *const i8,
            );
            cdbus_reply_errm(
                ps,
                dbus_message_new_error_printf(
                    msg,
                    b"com.github.chjj.compton.error.custom\0" as *const u8 as *const i8,
                    b"%s\0" as *const u8 as *const i8,
                    b"Failed to initialize specified VSync method.\0" as *const u8
                        as *const i8,
                ),
            );
            current_block = 1847472278776910194;
        } else {
            current_block = 4781280134633896646;
        }
        match current_block {
            4781280134633896646 => {}
            _ => return 1 as i32 != 0,
        }
    } else if strcmp(b"redirected_force\0" as *const u8 as *const i8, target) == 0 {
        let mut val_7: cdbus_enum_t = switch_t::UNSET as i32 as cdbus_enum_t;
        if !cdbus_msg_get_arg(
            msg,
            1 as i32,
            'q' as i32,
            &mut val_7 as *mut cdbus_enum_t as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        (*ps).o.redirected_force = switch_t::from_libc_c_uint(val_7 as u32);
        force_repaint(ps);
    } else if strcmp(b"stoppaint_force\0" as *const u8 as *const i8, target) == 0 {
        let mut val_8: cdbus_enum_t = 0;
        if !cdbus_msg_get_arg(
            msg,
            1 as i32,
            'q' as i32,
            &mut val_8 as *mut cdbus_enum_t as *mut libc::c_void,
        ) {
            return 0 as i32 != 0;
        }
        (*ps).o.stoppaint_force = switch_t::from_libc_c_uint(val_8 as u32);
    } else {
        fprintf(
            stderr,
            b"%s(): Target \"%s\" not found.\n\0" as *const u8 as *const i8,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[i8; 23],
            >(b"cdbus_process_opts_set\0"))
                .as_ptr(),
            target,
        );
        cdbus_reply_errm(
            ps,
            dbus_message_new_error_printf(
                msg,
                b"com.github.chjj.compton.error.bad_target\0" as *const u8 as *const i8,
                b"Target \"%s\" not found.\0" as *const u8 as *const i8,
                target,
            ),
        );
        return 1 as i32 != 0;
    }
    if dbus_message_get_no_reply(msg) == 0 {
        cdbus_reply_bool(ps, msg, 1 as i32 != 0);
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cdbus_process_introspect(
    mut ps: *mut session_t,
    mut msg: *mut DBusMessage,
) -> bool {
    static mut str_introspect: *const i8 = b"<!DOCTYPE node PUBLIC \"-//freedesktop//DTD D-BUS Object Introspection 1.0//EN\"\n \"http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd\">\n<node name='/com/github/chjj/compton'>\n  <interface name='org.freedesktop.DBus.Introspectable'>\n    <method name='Introspect'>\n      <arg name='data' direction='out' type='s' />\n    </method>\n  </interface>\n  <interface name='org.freedesktop.DBus.Peer'>\n    <method name='Ping' />\n    <method name='GetMachineId'>\n      <arg name='machine_uuid' direction='out' type='s' />\n    </method>\n  </interface>\n  <interface name='com.github.chjj.compton'>\n    <signal name='win_added'>\n      <arg name='wid' type='u'/>\n    </signal>\n    <signal name='win_destroyed'>\n      <arg name='wid' type='u'/>\n    </signal>\n    <signal name='win_mapped'>\n      <arg name='wid' type='u'/>\n    </signal>\n    <signal name='win_unmapped'>\n      <arg name='wid' type='u'/>\n    </signal>\n    <signal name='win_focusin'>\n      <arg name='wid' type='u'/>\n    </signal>\n    <signal name='win_focusout'>\n      <arg name='wid' type='u'/>\n    </signal>\n    <method name='reset' />\n    <method name='repaint' />\n  </interface>\n</node>\n\0"
        as *const u8 as *const i8;
    cdbus_reply_string(ps, msg, str_introspect);
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn cdbus_ev_win_added(mut ps: *mut session_t, mut w: *mut win) {
    if !((*ps).dbus_conn).is_null() {
        cdbus_signal_wid(ps, b"win_added\0" as *const u8 as *const i8, (*w).id);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cdbus_ev_win_destroyed(
    mut ps: *mut session_t,
    mut w: *mut win,
) {
    if !((*ps).dbus_conn).is_null() {
        cdbus_signal_wid(ps, b"win_destroyed\0" as *const u8 as *const i8, (*w).id);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cdbus_ev_win_mapped(mut ps: *mut session_t, mut w: *mut win) {
    if !((*ps).dbus_conn).is_null() {
        cdbus_signal_wid(ps, b"win_mapped\0" as *const u8 as *const i8, (*w).id);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cdbus_ev_win_unmapped(mut ps: *mut session_t, mut w: *mut win) {
    if !((*ps).dbus_conn).is_null() {
        cdbus_signal_wid(ps, b"win_unmapped\0" as *const u8 as *const i8, (*w).id);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cdbus_ev_win_focusout(mut ps: *mut session_t, mut w: *mut win) {
    if !((*ps).dbus_conn).is_null() {
        cdbus_signal_wid(ps, b"win_focusout\0" as *const u8 as *const i8, (*w).id);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cdbus_ev_win_focusin(mut ps: *mut session_t, mut w: *mut win) {
    if !((*ps).dbus_conn).is_null() {
        cdbus_signal_wid(ps, b"win_focusin\0" as *const u8 as *const i8, (*w).id);
    }
}