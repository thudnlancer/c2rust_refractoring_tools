use libc::mode_t;

#[derive(Copy, Clone)]
pub struct PermissionContext {
    pub mode: mode_t,
}

impl PermissionContext {
    pub fn free(self) {}
}