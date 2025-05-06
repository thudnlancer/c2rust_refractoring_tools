#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn perror(__s: *const i8);
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type scsi_io_mode_t = u32;
pub const SCSI_IO_WRITE: scsi_io_mode_t = 1;
pub const SCSI_IO_READ: scsi_io_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_io_hdr {
    pub interface_id: i32,
    pub dxfer_direction: i32,
    pub cmd_len: u8,
    pub mx_sb_len: u8,
    pub iovec_count: libc::c_ushort,
    pub dxfer_len: u32,
    pub dxferp: *mut libc::c_void,
    pub cmdp: *mut u8,
    pub sbp: *mut u8,
    pub timeout: u32,
    pub flags: u32,
    pub pack_id: i32,
    pub usr_ptr: *mut libc::c_void,
    pub status: u8,
    pub masked_status: u8,
    pub msg_status: u8,
    pub sb_len_wr: u8,
    pub host_status: libc::c_ushort,
    pub driver_status: libc::c_ushort,
    pub resid: i32,
    pub duration: u32,
    pub info: u32,
}
#[no_mangle]
pub unsafe extern "C" fn scsi_max_length() -> u32 {
    return 8 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn scsi_open(
    mut name: *const i8,
    mut flag: i32,
    mut mode: i32,
    mut extra_data: *mut *mut libc::c_void,
) -> i32 {
    return open(name, 0 as i32 | 0 as i32 | 0 as i32 | 0o4000 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn scsi_cmd(
    mut fd: i32,
    mut cdb: *mut u8,
    mut cmdlen: uint8_t,
    mut mode: scsi_io_mode_t,
    mut data: *mut libc::c_void,
    mut len: uint32_t,
    mut extra_data: *mut libc::c_void,
) -> i32 {
    let mut my_scsi_cmd: sg_io_hdr = sg_io_hdr {
        interface_id: 0,
        dxfer_direction: 0,
        cmd_len: 0,
        mx_sb_len: 0,
        iovec_count: 0,
        dxfer_len: 0,
        dxferp: 0 as *mut libc::c_void,
        cmdp: 0 as *mut u8,
        sbp: 0 as *mut u8,
        timeout: 0,
        flags: 0,
        pack_id: 0,
        usr_ptr: 0 as *mut libc::c_void,
        status: 0,
        masked_status: 0,
        msg_status: 0,
        sb_len_wr: 0,
        host_status: 0,
        driver_status: 0,
        resid: 0,
        duration: 0,
        info: 0,
    };
    memset(
        &mut my_scsi_cmd as *mut sg_io_hdr as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<sg_io_hdr>() as u64,
    );
    my_scsi_cmd.interface_id = 'S' as i32;
    my_scsi_cmd.dxfer_direction = if mode as u32 == SCSI_IO_READ as i32 as u32 {
        -(3 as i32)
    } else {
        -(2 as i32)
    };
    my_scsi_cmd.cmd_len = cmdlen;
    my_scsi_cmd.mx_sb_len = 0 as i32 as u8;
    my_scsi_cmd.dxfer_len = len;
    my_scsi_cmd.dxferp = data;
    my_scsi_cmd.cmdp = cdb;
    my_scsi_cmd.timeout = !(0 as u32);
    if ioctl(fd, 0x2285 as i32 as u64, &mut my_scsi_cmd as *mut sg_io_hdr) < 0 as i32 {
        perror(b"scsi_io\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    return my_scsi_cmd.status as i32 & 0x3e as i32;
}