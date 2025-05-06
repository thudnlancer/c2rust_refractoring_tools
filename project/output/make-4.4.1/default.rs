#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn strlen(_: *const i8) -> u64;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const i8) -> *mut i8;
    static mut no_builtin_variables_flag: i32;
    fn strcache_add(str: *const i8) -> *const i8;
    static mut no_builtin_rules_flag: i32;
    fn enter_file(name: *const i8) -> *mut file;
    fn enter_prereqs(prereqs: *mut dep, stem: *const i8) -> *mut dep;
    static mut current_variable_set_list: *mut variable_set_list;
    fn define_variable_in_set(
        name: *const i8,
        length: size_t,
        value: *const i8,
        origin: variable_origin,
        recursive: i32,
        set: *mut variable_set,
        flocp: *const floc,
    ) -> *mut variable;
    fn undefine_variable_in_set(
        name: *const i8,
        length: size_t,
        origin: variable_origin,
        set: *mut variable_set,
    );
    static mut suffix_file: *mut file;
    fn install_pattern_rule(p: *mut pspec, terminal: i32);
    fn parse_file_seq(
        stringp: *mut *mut i8,
        size: size_t,
        stopmap: i32,
        prefix: *const i8,
        flags: i32,
    ) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __uintmax_t = u64;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const i8,
    pub hname: *const i8,
    pub vpath: *const i8,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const i8,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: u32,
    pub command_flags: i32,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "low_resolution_time", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "tried_implicit", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "libc::c_uint", bits = "24..=24")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum cmd_state {
    cs_finished = 3,
    cs_running = 2,
    cs_deps_running = 1,
    cs_not_started = 0,
}
impl cmd_state {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            cmd_state::cs_finished => 3,
            cmd_state::cs_running => 2,
            cmd_state::cs_deps_running => 1,
            cmd_state::cs_not_started => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> cmd_state {
        match value {
            3 => cmd_state::cs_finished,
            2 => cmd_state::cs_running,
            1 => cmd_state::cs_deps_running,
            0 => cmd_state::cs_not_started,
            _ => panic!("Invalid value for cmd_state: {}", value),
        }
    }
}
impl AddAssign<u32> for cmd_state {
    fn add_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for cmd_state {
    fn sub_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for cmd_state {
    fn mul_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for cmd_state {
    fn div_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for cmd_state {
    fn rem_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for cmd_state {
    type Output = cmd_state;
    fn add(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for cmd_state {
    type Output = cmd_state;
    fn sub(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for cmd_state {
    type Output = cmd_state;
    fn mul(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for cmd_state {
    type Output = cmd_state;
    fn div(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for cmd_state {
    type Output = cmd_state;
    fn rem(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum update_status {
    us_failed = 3,
    us_question = 2,
    us_none = 1,
    us_success = 0,
}
impl update_status {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            update_status::us_failed => 3,
            update_status::us_question => 2,
            update_status::us_none => 1,
            update_status::us_success => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> update_status {
        match value {
            3 => update_status::us_failed,
            2 => update_status::us_question,
            1 => update_status::us_none,
            0 => update_status::us_success,
            _ => panic!("Invalid value for update_status: {}", value),
        }
    }
}
impl AddAssign<u32> for update_status {
    fn add_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for update_status {
    fn sub_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for update_status {
    fn mul_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for update_status {
    fn div_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for update_status {
    fn rem_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for update_status {
    type Output = update_status;
    fn add(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for update_status {
    type Output = update_status;
    fn sub(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for update_status {
    type Output = update_status;
    fn mul(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for update_status {
    type Output = update_status;
    fn div(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for update_status {
    type Output = update_status;
    fn rem(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set_list {
    pub next: *mut variable_set_list,
    pub set: *mut variable_set,
    pub next_is_parent: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set {
    pub table: hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut libc::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: u64,
    pub ht_capacity: u64,
    pub ht_fill: u64,
    pub ht_empty_slots: u64,
    pub ht_collisions: u64,
    pub ht_lookups: u64,
    pub ht_rehashes: u32,
}
pub type hash_cmp_func_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type hash_func_t = Option<unsafe extern "C" fn(*const libc::c_void) -> u64>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct dep {
    pub next: *mut dep,
    pub name: *const i8,
    pub file: *mut file,
    pub shuf: *mut dep,
    pub stem: *const i8,
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "need_2nd_expansion", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "ignore_automatic_vars", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "libc::c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct commands {
    pub fileinfo: floc,
    pub commands: *mut i8,
    pub command_lines: *mut *mut i8,
    pub lines_flags: *mut u8,
    pub ncommand_lines: libc::c_ushort,
    pub recipe_prefix: i8,
    #[bitfield(name = "any_recurse", ty = "libc::c_uint", bits = "0..=0")]
    pub any_recurse: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const i8,
    pub lineno: u64,
    pub offset: u64,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum variable_origin {
    o_default,
    o_env,
    o_file,
    o_env_override,
    o_command,
    o_override,
    o_automatic,
    o_invalid,
}
impl variable_origin {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            variable_origin::o_default => 0,
            variable_origin::o_env => 1,
            variable_origin::o_file => 2,
            variable_origin::o_env_override => 3,
            variable_origin::o_command => 4,
            variable_origin::o_override => 5,
            variable_origin::o_automatic => 6,
            variable_origin::o_invalid => 7,
        }
    }
    fn from_libc_c_uint(value: u32) -> variable_origin {
        match value {
            0 => variable_origin::o_default,
            1 => variable_origin::o_env,
            2 => variable_origin::o_file,
            3 => variable_origin::o_env_override,
            4 => variable_origin::o_command,
            5 => variable_origin::o_override,
            6 => variable_origin::o_automatic,
            7 => variable_origin::o_invalid,
            _ => panic!("Invalid value for variable_origin: {}", value),
        }
    }
}
impl AddAssign<u32> for variable_origin {
    fn add_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for variable_origin {
    fn sub_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for variable_origin {
    fn mul_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for variable_origin {
    fn div_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for variable_origin {
    fn rem_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for variable_origin {
    type Output = variable_origin;
    fn add(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for variable_origin {
    type Output = variable_origin;
    fn sub(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for variable_origin {
    type Output = variable_origin;
    fn mul(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for variable_origin {
    type Output = variable_origin;
    fn div(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for variable_origin {
    type Output = variable_origin;
    fn rem(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct variable {
    pub name: *mut i8,
    pub value: *mut i8,
    pub fileinfo: floc,
    pub length: u32,
    #[bitfield(name = "recursive", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "append", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "conditional", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "per_target", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "special", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "exportable", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "expanding", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "private_var", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "exp_count", ty = "libc::c_uint", bits = "8..=22")]
    #[bitfield(name = "flavor", ty = "variable_flavor", bits = "23..=25")]
    #[bitfield(name = "origin", ty = "variable_origin", bits = "26..=28")]
    #[bitfield(name = "export", ty = "variable_export", bits = "29..=30")]
    pub recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [u8; 4],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum variable_export {
    v_default = 0,
    v_export,
    v_noexport,
    v_ifset,
}
impl variable_export {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            variable_export::v_default => 0,
            variable_export::v_export => 1,
            variable_export::v_noexport => 2,
            variable_export::v_ifset => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> variable_export {
        match value {
            0 => variable_export::v_default,
            1 => variable_export::v_export,
            2 => variable_export::v_noexport,
            3 => variable_export::v_ifset,
            _ => panic!("Invalid value for variable_export: {}", value),
        }
    }
}
impl AddAssign<u32> for variable_export {
    fn add_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for variable_export {
    fn sub_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for variable_export {
    fn mul_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for variable_export {
    fn div_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for variable_export {
    fn rem_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for variable_export {
    type Output = variable_export;
    fn add(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for variable_export {
    type Output = variable_export;
    fn sub(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for variable_export {
    type Output = variable_export;
    fn mul(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for variable_export {
    type Output = variable_export;
    fn div(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for variable_export {
    type Output = variable_export;
    fn rem(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum variable_flavor {
    f_bogus,
    f_simple,
    f_recursive,
    f_expand,
    f_append,
    f_conditional,
    f_shell,
    f_append_value,
}
impl variable_flavor {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            variable_flavor::f_bogus => 0,
            variable_flavor::f_simple => 1,
            variable_flavor::f_recursive => 2,
            variable_flavor::f_expand => 3,
            variable_flavor::f_append => 4,
            variable_flavor::f_conditional => 5,
            variable_flavor::f_shell => 6,
            variable_flavor::f_append_value => 7,
        }
    }
    fn from_libc_c_uint(value: u32) -> variable_flavor {
        match value {
            0 => variable_flavor::f_bogus,
            1 => variable_flavor::f_simple,
            2 => variable_flavor::f_recursive,
            3 => variable_flavor::f_expand,
            4 => variable_flavor::f_append,
            5 => variable_flavor::f_conditional,
            6 => variable_flavor::f_shell,
            7 => variable_flavor::f_append_value,
            _ => panic!("Invalid value for variable_flavor: {}", value),
        }
    }
}
impl AddAssign<u32> for variable_flavor {
    fn add_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for variable_flavor {
    fn sub_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for variable_flavor {
    fn mul_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for variable_flavor {
    fn div_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for variable_flavor {
    fn rem_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for variable_flavor {
    type Output = variable_flavor;
    fn add(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for variable_flavor {
    type Output = variable_flavor;
    fn sub(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for variable_flavor {
    type Output = variable_flavor;
    fn mul(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for variable_flavor {
    type Output = variable_flavor;
    fn div(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for variable_flavor {
    type Output = variable_flavor;
    fn rem(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pspec {
    pub target: *const i8,
    pub dep: *const i8,
    pub commands: *const i8,
}
static mut default_suffixes: [i8; 147] = unsafe {
    *::core::mem::transmute::<
        &[u8; 147],
        &mut [i8; 147],
    >(
        b".out .a .ln .o .c .cc .C .cpp .p .f .F .m .r .y .l .ym .yl .s .S .mod .sym .def .h .info .dvi .tex .texinfo .texi .txinfo .w .ch .web .sh .elc .el\0",
    )
};
static mut default_pattern_rules: [pspec; 5] = [
    {
        let mut init = pspec {
            target: b"(%)\0" as *const u8 as *const i8,
            dep: b"%\0" as *const u8 as *const i8,
            commands: b"$(AR) $(ARFLAGS) $@ $<\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%.out\0" as *const u8 as *const i8,
            dep: b"%\0" as *const u8 as *const i8,
            commands: b"@rm -f $@ \n cp $< $@\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%.c\0" as *const u8 as *const i8,
            dep: b"%.w %.ch\0" as *const u8 as *const i8,
            commands: b"$(CTANGLE) $^ $@\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%.tex\0" as *const u8 as *const i8,
            dep: b"%.w %.ch\0" as *const u8 as *const i8,
            commands: b"$(CWEAVE) $^ $@\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = pspec {
            target: 0 as *const i8,
            dep: 0 as *const i8,
            commands: 0 as *const i8,
        };
        init
    },
];
static mut default_terminal_rules: [pspec; 6] = [
    {
        let mut init = pspec {
            target: b"%\0" as *const u8 as *const i8,
            dep: b"%,v\0" as *const u8 as *const i8,
            commands: b"$(CHECKOUT,v)\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%\0" as *const u8 as *const i8,
            dep: b"RCS/%,v\0" as *const u8 as *const i8,
            commands: b"$(CHECKOUT,v)\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%\0" as *const u8 as *const i8,
            dep: b"RCS/%\0" as *const u8 as *const i8,
            commands: b"$(CHECKOUT,v)\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%\0" as *const u8 as *const i8,
            dep: b"s.%\0" as *const u8 as *const i8,
            commands: b"$(GET) $(GFLAGS) $(SCCS_OUTPUT_OPTION) $<\0" as *const u8
                as *const i8,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%\0" as *const u8 as *const i8,
            dep: b"SCCS/s.%\0" as *const u8 as *const i8,
            commands: b"$(GET) $(GFLAGS) $(SCCS_OUTPUT_OPTION) $<\0" as *const u8
                as *const i8,
        };
        init
    },
    {
        let mut init = pspec {
            target: 0 as *const i8,
            dep: 0 as *const i8,
            commands: 0 as *const i8,
        };
        init
    },
];
static mut default_suffix_rules: [*const i8; 100] = [
    b".o\0" as *const u8 as *const i8,
    b"$(LINK.o) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const i8,
    b".s\0" as *const u8 as *const i8,
    b"$(LINK.s) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const i8,
    b".S\0" as *const u8 as *const i8,
    b"$(LINK.S) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const i8,
    b".c\0" as *const u8 as *const i8,
    b"$(LINK.c) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const i8,
    b".cc\0" as *const u8 as *const i8,
    b"$(LINK.cc) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const i8,
    b".C\0" as *const u8 as *const i8,
    b"$(LINK.C) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const i8,
    b".cpp\0" as *const u8 as *const i8,
    b"$(LINK.cpp) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const i8,
    b".f\0" as *const u8 as *const i8,
    b"$(LINK.f) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const i8,
    b".m\0" as *const u8 as *const i8,
    b"$(LINK.m) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const i8,
    b".p\0" as *const u8 as *const i8,
    b"$(LINK.p) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const i8,
    b".F\0" as *const u8 as *const i8,
    b"$(LINK.F) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const i8,
    b".r\0" as *const u8 as *const i8,
    b"$(LINK.r) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const i8,
    b".mod\0" as *const u8 as *const i8,
    b"$(COMPILE.mod) -o $@ -e $@ $^\0" as *const u8 as *const i8,
    b".def.sym\0" as *const u8 as *const i8,
    b"$(COMPILE.def) -o $@ $<\0" as *const u8 as *const i8,
    b".sh\0" as *const u8 as *const i8,
    b"cat $< >$@ \n chmod a+x $@\0" as *const u8 as *const i8,
    b".s.o\0" as *const u8 as *const i8,
    b"$(COMPILE.s) -o $@ $<\0" as *const u8 as *const i8,
    b".S.o\0" as *const u8 as *const i8,
    b"$(COMPILE.S) -o $@ $<\0" as *const u8 as *const i8,
    b".c.o\0" as *const u8 as *const i8,
    b"$(COMPILE.c) $(OUTPUT_OPTION) $<\0" as *const u8 as *const i8,
    b".cc.o\0" as *const u8 as *const i8,
    b"$(COMPILE.cc) $(OUTPUT_OPTION) $<\0" as *const u8 as *const i8,
    b".C.o\0" as *const u8 as *const i8,
    b"$(COMPILE.C) $(OUTPUT_OPTION) $<\0" as *const u8 as *const i8,
    b".cpp.o\0" as *const u8 as *const i8,
    b"$(COMPILE.cpp) $(OUTPUT_OPTION) $<\0" as *const u8 as *const i8,
    b".f.o\0" as *const u8 as *const i8,
    b"$(COMPILE.f) $(OUTPUT_OPTION) $<\0" as *const u8 as *const i8,
    b".m.o\0" as *const u8 as *const i8,
    b"$(COMPILE.m) $(OUTPUT_OPTION) $<\0" as *const u8 as *const i8,
    b".p.o\0" as *const u8 as *const i8,
    b"$(COMPILE.p) $(OUTPUT_OPTION) $<\0" as *const u8 as *const i8,
    b".F.o\0" as *const u8 as *const i8,
    b"$(COMPILE.F) $(OUTPUT_OPTION) $<\0" as *const u8 as *const i8,
    b".r.o\0" as *const u8 as *const i8,
    b"$(COMPILE.r) $(OUTPUT_OPTION) $<\0" as *const u8 as *const i8,
    b".mod.o\0" as *const u8 as *const i8,
    b"$(COMPILE.mod) -o $@ $<\0" as *const u8 as *const i8,
    b".c.ln\0" as *const u8 as *const i8,
    b"$(LINT.c) -C$* $<\0" as *const u8 as *const i8,
    b".y.ln\0" as *const u8 as *const i8,
    b"$(YACC.y) $< \n $(LINT.c) -C$* y.tab.c \n $(RM) y.tab.c\0" as *const u8
        as *const i8,
    b".l.ln\0" as *const u8 as *const i8,
    b"@$(RM) $*.c\n $(LEX.l) $< > $*.c\n$(LINT.c) -i $*.c -o $@\n $(RM) $*.c\0"
        as *const u8 as *const i8,
    b".y.c\0" as *const u8 as *const i8,
    b"$(YACC.y) $< \n mv -f y.tab.c $@\0" as *const u8 as *const i8,
    b".l.c\0" as *const u8 as *const i8,
    b"@$(RM) $@ \n $(LEX.l) $< > $@\0" as *const u8 as *const i8,
    b".ym.m\0" as *const u8 as *const i8,
    b"$(YACC.m) $< \n mv -f y.tab.c $@\0" as *const u8 as *const i8,
    b".lm.m\0" as *const u8 as *const i8,
    b"@$(RM) $@ \n $(LEX.m) $< > $@\0" as *const u8 as *const i8,
    b".F.f\0" as *const u8 as *const i8,
    b"$(PREPROCESS.F) $(OUTPUT_OPTION) $<\0" as *const u8 as *const i8,
    b".r.f\0" as *const u8 as *const i8,
    b"$(PREPROCESS.r) $(OUTPUT_OPTION) $<\0" as *const u8 as *const i8,
    b".l.r\0" as *const u8 as *const i8,
    b"$(LEX.l) $< > $@ \n mv -f lex.yy.r $@\0" as *const u8 as *const i8,
    b".S.s\0" as *const u8 as *const i8,
    b"$(PREPROCESS.S) $< > $@\0" as *const u8 as *const i8,
    b".texinfo.info\0" as *const u8 as *const i8,
    b"$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@\0" as *const u8 as *const i8,
    b".texi.info\0" as *const u8 as *const i8,
    b"$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@\0" as *const u8 as *const i8,
    b".txinfo.info\0" as *const u8 as *const i8,
    b"$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@\0" as *const u8 as *const i8,
    b".tex.dvi\0" as *const u8 as *const i8,
    b"$(TEX) $<\0" as *const u8 as *const i8,
    b".texinfo.dvi\0" as *const u8 as *const i8,
    b"$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<\0" as *const u8 as *const i8,
    b".texi.dvi\0" as *const u8 as *const i8,
    b"$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<\0" as *const u8 as *const i8,
    b".txinfo.dvi\0" as *const u8 as *const i8,
    b"$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<\0" as *const u8 as *const i8,
    b".w.c\0" as *const u8 as *const i8,
    b"$(CTANGLE) $< - $@\0" as *const u8 as *const i8,
    b".web.p\0" as *const u8 as *const i8,
    b"$(TANGLE) $<\0" as *const u8 as *const i8,
    b".w.tex\0" as *const u8 as *const i8,
    b"$(CWEAVE) $< - $@\0" as *const u8 as *const i8,
    b".web.tex\0" as *const u8 as *const i8,
    b"$(WEAVE) $<\0" as *const u8 as *const i8,
    0 as *const i8,
    0 as *const i8,
];
static mut default_variables: [*const i8; 130] = [
    b"AR\0" as *const u8 as *const i8,
    b"ar\0" as *const u8 as *const i8,
    b"ARFLAGS\0" as *const u8 as *const i8,
    b"-rv\0" as *const u8 as *const i8,
    b"AS\0" as *const u8 as *const i8,
    b"as\0" as *const u8 as *const i8,
    b"CC\0" as *const u8 as *const i8,
    b"cc\0" as *const u8 as *const i8,
    b"OBJC\0" as *const u8 as *const i8,
    b"cc\0" as *const u8 as *const i8,
    b"CXX\0" as *const u8 as *const i8,
    b"g++\0" as *const u8 as *const i8,
    b"CHECKOUT,v\0" as *const u8 as *const i8,
    b"+$(if $(wildcard $@),,$(CO) $(COFLAGS) $< $@)\0" as *const u8 as *const i8,
    b"CO\0" as *const u8 as *const i8,
    b"co\0" as *const u8 as *const i8,
    b"COFLAGS\0" as *const u8 as *const i8,
    b"\0" as *const u8 as *const i8,
    b"CPP\0" as *const u8 as *const i8,
    b"$(CC) -E\0" as *const u8 as *const i8,
    b"FC\0" as *const u8 as *const i8,
    b"f77\0" as *const u8 as *const i8,
    b"F77\0" as *const u8 as *const i8,
    b"$(FC)\0" as *const u8 as *const i8,
    b"F77FLAGS\0" as *const u8 as *const i8,
    b"$(FFLAGS)\0" as *const u8 as *const i8,
    b"GET\0" as *const u8 as *const i8,
    b"get\0" as *const u8 as *const i8,
    b"LD\0" as *const u8 as *const i8,
    b"ld\0" as *const u8 as *const i8,
    b"LEX\0" as *const u8 as *const i8,
    b"lex\0" as *const u8 as *const i8,
    b"LINT\0" as *const u8 as *const i8,
    b"lint\0" as *const u8 as *const i8,
    b"M2C\0" as *const u8 as *const i8,
    b"m2c\0" as *const u8 as *const i8,
    b"PC\0" as *const u8 as *const i8,
    b"pc\0" as *const u8 as *const i8,
    b"YACC\0" as *const u8 as *const i8,
    b"yacc\0" as *const u8 as *const i8,
    b"MAKEINFO\0" as *const u8 as *const i8,
    b"makeinfo\0" as *const u8 as *const i8,
    b"TEX\0" as *const u8 as *const i8,
    b"tex\0" as *const u8 as *const i8,
    b"TEXI2DVI\0" as *const u8 as *const i8,
    b"texi2dvi\0" as *const u8 as *const i8,
    b"WEAVE\0" as *const u8 as *const i8,
    b"weave\0" as *const u8 as *const i8,
    b"CWEAVE\0" as *const u8 as *const i8,
    b"cweave\0" as *const u8 as *const i8,
    b"TANGLE\0" as *const u8 as *const i8,
    b"tangle\0" as *const u8 as *const i8,
    b"CTANGLE\0" as *const u8 as *const i8,
    b"ctangle\0" as *const u8 as *const i8,
    b"RM\0" as *const u8 as *const i8,
    b"rm -f\0" as *const u8 as *const i8,
    b"LINK.o\0" as *const u8 as *const i8,
    b"$(CC) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const i8,
    b"COMPILE.c\0" as *const u8 as *const i8,
    b"$(CC) $(CFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const i8,
    b"LINK.c\0" as *const u8 as *const i8,
    b"$(CC) $(CFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const i8,
    b"COMPILE.m\0" as *const u8 as *const i8,
    b"$(OBJC) $(OBJCFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const i8,
    b"LINK.m\0" as *const u8 as *const i8,
    b"$(OBJC) $(OBJCFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const i8,
    b"COMPILE.cc\0" as *const u8 as *const i8,
    b"$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const i8,
    b"COMPILE.C\0" as *const u8 as *const i8,
    b"$(COMPILE.cc)\0" as *const u8 as *const i8,
    b"COMPILE.cpp\0" as *const u8 as *const i8,
    b"$(COMPILE.cc)\0" as *const u8 as *const i8,
    b"LINK.cc\0" as *const u8 as *const i8,
    b"$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const i8,
    b"LINK.C\0" as *const u8 as *const i8,
    b"$(LINK.cc)\0" as *const u8 as *const i8,
    b"LINK.cpp\0" as *const u8 as *const i8,
    b"$(LINK.cc)\0" as *const u8 as *const i8,
    b"YACC.y\0" as *const u8 as *const i8,
    b"$(YACC) $(YFLAGS)\0" as *const u8 as *const i8,
    b"LEX.l\0" as *const u8 as *const i8,
    b"$(LEX) $(LFLAGS) -t\0" as *const u8 as *const i8,
    b"YACC.m\0" as *const u8 as *const i8,
    b"$(YACC) $(YFLAGS)\0" as *const u8 as *const i8,
    b"LEX.m\0" as *const u8 as *const i8,
    b"$(LEX) $(LFLAGS) -t\0" as *const u8 as *const i8,
    b"COMPILE.f\0" as *const u8 as *const i8,
    b"$(FC) $(FFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const i8,
    b"LINK.f\0" as *const u8 as *const i8,
    b"$(FC) $(FFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const i8,
    b"COMPILE.F\0" as *const u8 as *const i8,
    b"$(FC) $(FFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const i8,
    b"LINK.F\0" as *const u8 as *const i8,
    b"$(FC) $(FFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const i8,
    b"COMPILE.r\0" as *const u8 as *const i8,
    b"$(FC) $(FFLAGS) $(RFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const i8,
    b"LINK.r\0" as *const u8 as *const i8,
    b"$(FC) $(FFLAGS) $(RFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const i8,
    b"COMPILE.def\0" as *const u8 as *const i8,
    b"$(M2C) $(M2FLAGS) $(DEFFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const i8,
    b"COMPILE.mod\0" as *const u8 as *const i8,
    b"$(M2C) $(M2FLAGS) $(MODFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const i8,
    b"COMPILE.p\0" as *const u8 as *const i8,
    b"$(PC) $(PFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const i8,
    b"LINK.p\0" as *const u8 as *const i8,
    b"$(PC) $(PFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const i8,
    b"LINK.s\0" as *const u8 as *const i8,
    b"$(CC) $(ASFLAGS) $(LDFLAGS) $(TARGET_MACH)\0" as *const u8 as *const i8,
    b"COMPILE.s\0" as *const u8 as *const i8,
    b"$(AS) $(ASFLAGS) $(TARGET_MACH)\0" as *const u8 as *const i8,
    b"LINK.S\0" as *const u8 as *const i8,
    b"$(CC) $(ASFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_MACH)\0" as *const u8
        as *const i8,
    b"COMPILE.S\0" as *const u8 as *const i8,
    b"$(CC) $(ASFLAGS) $(CPPFLAGS) $(TARGET_MACH) -c\0" as *const u8 as *const i8,
    b"PREPROCESS.S\0" as *const u8 as *const i8,
    b"$(CPP) $(CPPFLAGS)\0" as *const u8 as *const i8,
    b"PREPROCESS.F\0" as *const u8 as *const i8,
    b"$(FC) $(FFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -F\0" as *const u8 as *const i8,
    b"PREPROCESS.r\0" as *const u8 as *const i8,
    b"$(FC) $(FFLAGS) $(RFLAGS) $(TARGET_ARCH) -F\0" as *const u8 as *const i8,
    b"LINT.c\0" as *const u8 as *const i8,
    b"$(LINT) $(LINTFLAGS) $(CPPFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const i8,
    b"OUTPUT_OPTION\0" as *const u8 as *const i8,
    b"-o $@\0" as *const u8 as *const i8,
    b".LIBPATTERNS\0" as *const u8 as *const i8,
    b"lib%.so lib%.a\0" as *const u8 as *const i8,
    b"GNUMAKEFLAGS\0" as *const u8 as *const i8,
    b"\0" as *const u8 as *const i8,
    0 as *const i8,
    0 as *const i8,
];
#[no_mangle]
pub unsafe extern "C" fn set_default_suffixes() {
    suffix_file = enter_file(strcache_add(b".SUFFIXES\0" as *const u8 as *const i8));
    (*suffix_file).set_builtin(1 as i32 as u32);
    if no_builtin_rules_flag != 0 {
        define_variable_in_set(
            b"SUFFIXES\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64),
            b"\0" as *const u8 as *const i8,
            variable_origin::o_default,
            0 as i32,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
    } else {
        let mut d: *mut dep = 0 as *mut dep;
        let mut p: *const i8 = default_suffixes.as_mut_ptr();
        (*suffix_file).deps = enter_prereqs(
            parse_file_seq(
                &mut p as *mut *const i8 as *mut *mut i8,
                ::core::mem::size_of::<dep>() as u64,
                0x1 as i32,
                0 as *const i8,
                0 as i32,
            ) as *mut dep,
            0 as *const i8,
        );
        d = (*suffix_file).deps;
        while !d.is_null() {
            (*(*d).file).set_builtin(1 as i32 as u32);
            d = (*d).next;
        }
        define_variable_in_set(
            b"SUFFIXES\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64),
            default_suffixes.as_mut_ptr(),
            variable_origin::o_default,
            0 as i32,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn install_default_suffix_rules() {
    let mut s: *mut *const i8 = 0 as *mut *const i8;
    if no_builtin_rules_flag != 0 {
        return;
    }
    s = default_suffix_rules.as_mut_ptr();
    while !(*s).is_null() {
        let mut f: *mut file = enter_file(strcache_add(*s.offset(0 as i32 as isize)));
        if ((*f).cmds).is_null() {
            (*f).cmds = xmalloc(::core::mem::size_of::<commands>() as u64)
                as *mut commands;
            (*(*f).cmds).fileinfo.filenm = 0 as *const i8;
            (*(*f).cmds).commands = xstrdup(*s.offset(1 as i32 as isize));
            (*(*f).cmds).command_lines = 0 as *mut *mut i8;
            (*(*f).cmds).recipe_prefix = '\t' as i32 as i8;
            (*f).set_builtin(1 as i32 as u32);
        }
        s = s.offset(2 as i32 as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn install_default_implicit_rules() {
    let mut p: *mut pspec = 0 as *mut pspec;
    if no_builtin_rules_flag != 0 {
        return;
    }
    p = default_pattern_rules.as_mut_ptr();
    while !((*p).target).is_null() {
        install_pattern_rule(p, 0 as i32);
        p = p.offset(1);
        p;
    }
    p = default_terminal_rules.as_mut_ptr();
    while !((*p).target).is_null() {
        install_pattern_rule(p, 1 as i32);
        p = p.offset(1);
        p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn define_default_variables() {
    let mut s: *mut *const i8 = 0 as *mut *const i8;
    if no_builtin_variables_flag != 0 {
        return;
    }
    s = default_variables.as_mut_ptr();
    while !(*s).is_null() {
        define_variable_in_set(
            *s.offset(0 as i32 as isize),
            strlen(*s.offset(0 as i32 as isize)),
            *s.offset(1 as i32 as isize),
            variable_origin::o_default,
            1 as i32,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
        s = s.offset(2 as i32 as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn undefine_default_variables() {
    let mut s: *mut *const i8 = 0 as *mut *const i8;
    s = default_variables.as_mut_ptr();
    while !(*s).is_null() {
        undefine_variable_in_set(
            *s.offset(0 as i32 as isize),
            strlen(*s.offset(0 as i32 as isize)),
            variable_origin::o_default,
            0 as *mut variable_set,
        );
        s = s.offset(2 as i32 as isize);
    }
}