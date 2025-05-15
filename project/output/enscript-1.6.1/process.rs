use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type stringhash_st;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn exit(_: i32) -> !;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn re_search(
        buffer: *mut re_pattern_buffer,
        string: *const i8,
        length: i32,
        start: i32,
        range: i32,
        regs: *mut re_registers,
    ) -> i32;
    fn strhash_get(
        hash: StringHashPtr,
        key: *const i8,
        keylen: i32,
        data_return: *mut *mut libc::c_void,
    ) -> i32;
    static mut program: *mut i8;
    static mut ofp: *mut FILE;
    static mut ns_vars: StringHashPtr;
    static mut ns_states: StringHashPtr;
    static mut start_stmts: *mut List;
    static mut nvoid: *mut Node;
    static mut ifp: *mut FILE;
    static mut inbuf: *mut i8;
    static mut data_in_buffer: u32;
    static mut bufpos: u32;
    static mut eof_seen: i32;
    static mut current_fname: *mut i8;
    static mut current_linenum: u32;
    static mut current_match: *mut re_registers;
    static mut current_match_buf: *mut i8;
    static mut start_state_arg: *mut i8;
    static mut start_state: *mut i8;
    fn node_free(node: *mut Node);
    fn enter_system_variable(name: *mut i8, value: *mut i8);
    fn compile_regexp(regexp: *mut Node);
    fn eval_statement_list(
        lst: *mut List,
        env: *mut Environment,
        return_seen: *mut i32,
    ) -> *mut Node;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type reg_syntax_t = u64;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut u8,
    pub allocated: u64,
    pub used: u64,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut i8,
    pub translate: *mut i8,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: u32,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
pub type StringHashPtr = *mut stringhash_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_item_st {
    pub next: *mut list_item_st,
    pub data: *mut libc::c_void,
}
pub type ListItem = list_item_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_st {
    pub head: *mut ListItem,
    pub tail: *mut ListItem,
}
pub type List = list_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum NodeType {
    nVOID,
    nSTRING,
    nREGEXP,
    nINTEGER,
    nREAL,
    nSYMBOL,
    nARRAY,
}
impl NodeType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            NodeType::nVOID => 0,
            NodeType::nSTRING => 1,
            NodeType::nREGEXP => 2,
            NodeType::nINTEGER => 3,
            NodeType::nREAL => 4,
            NodeType::nSYMBOL => 5,
            NodeType::nARRAY => 6,
        }
    }
    fn from_libc_c_uint(value: u32) -> NodeType {
        match value {
            0 => NodeType::nVOID,
            1 => NodeType::nSTRING,
            2 => NodeType::nREGEXP,
            3 => NodeType::nINTEGER,
            4 => NodeType::nREAL,
            5 => NodeType::nSYMBOL,
            6 => NodeType::nARRAY,
            _ => panic!("Invalid value for NodeType: {}", value),
        }
    }
}
impl AddAssign<u32> for NodeType {
    fn add_assign(&mut self, rhs: u32) {
        *self = NodeType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for NodeType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = NodeType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for NodeType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = NodeType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for NodeType {
    fn div_assign(&mut self, rhs: u32) {
        *self = NodeType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for NodeType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = NodeType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for NodeType {
    type Output = NodeType;
    fn add(self, rhs: u32) -> NodeType {
        NodeType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for NodeType {
    type Output = NodeType;
    fn sub(self, rhs: u32) -> NodeType {
        NodeType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for NodeType {
    type Output = NodeType;
    fn mul(self, rhs: u32) -> NodeType {
        NodeType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for NodeType {
    type Output = NodeType;
    fn div(self, rhs: u32) -> NodeType {
        NodeType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for NodeType {
    type Output = NodeType;
    fn rem(self, rhs: u32) -> NodeType {
        NodeType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_st {
    pub type_0: NodeType,
    pub refcount: u32,
    pub linenum: u32,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub str_0: C2RustUnnamed_2,
    pub re: C2RustUnnamed_1,
    pub integer: i32,
    pub real: libc::c_double,
    pub sym: *mut i8,
    pub array: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub array: *mut *mut node_st,
    pub len: u32,
    pub allocated: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub data: *mut i8,
    pub len: u32,
    pub flags: u32,
    pub compiled: regex_t,
    pub matches: re_registers,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub data: *mut i8,
    pub len: u32,
}
pub type Node = node_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cons_st {
    pub car: *mut libc::c_void,
    pub cdr: *mut libc::c_void,
}
pub type Cons = cons_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environment_st {
    pub next: *mut environment_st,
    pub name: *mut i8,
    pub val: *mut Node,
}
pub type Environment = environment_st;
#[no_mangle]
pub unsafe extern "C" fn process_file(mut fname: *mut i8) {
    let mut result: *mut Node = 0 as *mut Node;
    let mut return_seen: i32 = 0 as i32;
    start_state = 0 as *mut i8;
    current_fname = fname;
    current_linenum = 1 as i32 as u32;
    data_in_buffer = 0 as i32 as u32;
    bufpos = 0 as i32 as u32;
    eof_seen = 0 as i32;
    enter_system_variable(b"filename\0" as *const u8 as *const i8 as *mut i8, fname);
    data_in_buffer = fread(
        inbuf as *mut libc::c_void,
        1 as i32 as size_t,
        (20 as i32 * 1024 as i32) as size_t,
        ifp,
    ) as u32;
    if data_in_buffer < (20 as i32 * 1024 as i32) as u32 {
        eof_seen = 1 as i32;
    }
    if !start_state_arg.is_null() {
        start_state = start_state_arg;
    }
    result = eval_statement_list(start_stmts, 0 as *mut Environment, &mut return_seen);
    node_free(result);
    if start_state.is_null() {
        while data_in_buffer != 0 {
            fwrite(
                inbuf as *const libc::c_void,
                1 as i32 as size_t,
                data_in_buffer as size_t,
                ofp,
            );
            data_in_buffer = fread(
                inbuf as *mut libc::c_void,
                1 as i32 as size_t,
                (20 as i32 * 1024 as i32) as size_t,
                ifp,
            ) as u32;
        }
    } else {
        result = execute_state(start_state);
        node_free(result);
    };
}
#[no_mangle]
pub unsafe extern "C" fn execute_state(mut name: *mut i8) -> *mut Node {
    let mut current_block: u64;
    let mut state: *mut List = 0 as *mut List;
    let mut to_read: i32 = 0;
    let mut got: i32 = 0;
    let mut rule: *mut ListItem = 0 as *mut ListItem;
    let mut first_rule: *mut ListItem = 0 as *mut ListItem;
    let mut first_idx: u32 = 0;
    let mut match_len: u32 = 0;
    let mut result: *mut Node = nvoid;
    let mut r: *mut Cons = 0 as *mut Cons;
    let mut exp: *mut Node = 0 as *mut Node;
    let mut return_seen: i32 = 0 as i32;
    let mut idx: i32 = 0;
    if strhash_get(
        ns_states,
        name,
        strlen(name) as i32,
        &mut state as *mut *mut List as *mut *mut libc::c_void,
    ) == 0
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: undefined state `%s'\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            program,
            name,
        );
        exit(1 as i32);
    }
    rule = (*state).head;
    loop {
        if rule.is_null() {
            current_block = 3512920355445576850;
            break;
        }
        r = (*rule).data as *mut Cons;
        if ((*r).car).is_null() {
            node_free(result);
            result = eval_statement_list(
                (*r).cdr as *mut List,
                0 as *mut Environment,
                &mut return_seen,
            );
            if return_seen != 0 {
                current_block = 16824052130944227025;
                break;
            } else {
                current_block = 3512920355445576850;
                break;
            }
        } else {
            rule = (*rule).next;
        }
    }
    loop {
        match current_block {
            16824052130944227025 => {
                rule = (*state).head;
                while !rule.is_null() {
                    r = (*rule).data as *mut Cons;
                    if (*r).car == 1 as i32 as *mut libc::c_void {
                        node_free(result);
                        result = eval_statement_list(
                            (*r).cdr as *mut List,
                            0 as *mut Environment,
                            &mut return_seen,
                        );
                    }
                    rule = (*rule).next;
                }
                break;
            }
            _ => {
                let mut eol: i32 = 0;
                if bufpos >= data_in_buffer {
                    if eof_seen != 0 {
                        current_block = 16824052130944227025;
                        continue;
                    }
                    data_in_buffer = fread(
                        inbuf as *mut libc::c_void,
                        1 as i32 as size_t,
                        (20 as i32 * 1024 as i32) as size_t,
                        ifp,
                    ) as u32;
                    if data_in_buffer < (20 as i32 * 1024 as i32) as u32 {
                        eof_seen = 1 as i32;
                    }
                    bufpos = 0 as i32 as u32;
                    current_block = 3512920355445576850;
                } else {
                    if bufpos > 0 as i32 as u32
                        && *inbuf.offset(bufpos.wrapping_sub(1 as i32 as u32) as isize)
                            as i32 == '\n' as i32
                    {
                        current_linenum = current_linenum.wrapping_add(1);
                        current_linenum;
                    }
                    eol = bufpos as i32;
                    while (eol as u32) < data_in_buffer
                        && *inbuf.offset(eol as isize) as i32 != '\n' as i32
                    {
                        eol += 1;
                        eol;
                    }
                    if *inbuf.offset(eol as isize) as i32 == '\n' as i32 {
                        eol += 1;
                        eol;
                    }
                    if eol as u32 >= data_in_buffer && eof_seen == 0
                        && bufpos > 0 as i32 as u32
                    {
                        memmove(
                            inbuf as *mut libc::c_void,
                            inbuf.offset(bufpos as isize) as *const libc::c_void,
                            (eol as u32).wrapping_sub(bufpos) as u64,
                        );
                        data_in_buffer = (eol as u32).wrapping_sub(bufpos);
                        bufpos = 0 as i32 as u32;
                        to_read = ((20 as i32 * 1024 as i32) as u32)
                            .wrapping_sub(data_in_buffer) as i32;
                        got = fread(
                            inbuf.offset(data_in_buffer as isize) as *mut libc::c_void,
                            1 as i32 as size_t,
                            to_read as size_t,
                            ifp,
                        ) as i32;
                        if got < to_read {
                            eof_seen = 1 as i32;
                        }
                        data_in_buffer = data_in_buffer.wrapping_add(got as u32);
                        current_block = 3512920355445576850;
                    } else {
                        first_idx = eol as u32;
                        match_len = 0 as i32 as u32;
                        first_rule = 0 as *mut ListItem;
                        current_match = 0 as *mut re_registers;
                        let mut current_block_47: u64;
                        rule = (*state).head;
                        while !rule.is_null() {
                            let mut err: i32 = 0;
                            r = (*rule).data as *mut Cons;
                            exp = (*r).car as *mut Node;
                            if !(exp.is_null()
                                || exp == 1 as i32 as *mut libc::c_void as *mut Node)
                            {
                                if (*exp).type_0 as u32 == NodeType::nSYMBOL as i32 as u32 {
                                    let mut n: *mut Node = 0 as *mut Node;
                                    if strhash_get(
                                        ns_vars,
                                        (*exp).u.sym,
                                        strlen((*exp).u.sym) as i32,
                                        &mut n as *mut *mut Node as *mut *mut libc::c_void,
                                    ) == 0
                                    {
                                        fprintf(
                                            stderr,
                                            dcgettext(
                                                0 as *const i8,
                                                b"%s: error: undefined variable `%s'\n\0" as *const u8
                                                    as *const i8,
                                                5 as i32,
                                            ),
                                            program,
                                            (*exp).u.sym,
                                        );
                                        exit(1 as i32);
                                    }
                                    if (*n).type_0 as u32 != NodeType::nREGEXP as i32 as u32 {
                                        current_block_47 = 7245201122033322888;
                                    } else {
                                        exp = n;
                                        current_block_47 = 9441801433784995173;
                                    }
                                } else {
                                    current_block_47 = 9441801433784995173;
                                }
                                match current_block_47 {
                                    7245201122033322888 => {}
                                    _ => {
                                        err = re_search(
                                            if ((*exp).u.re.compiled).fastmap_accurate() as i32 != 0 {
                                                &mut (*exp).u.re.compiled
                                            } else {
                                                compile_regexp(exp);
                                                &mut (*exp).u.re.compiled
                                            },
                                            inbuf,
                                            eol,
                                            bufpos as i32,
                                            (eol as u32).wrapping_sub(bufpos) as i32,
                                            &mut (*exp).u.re.matches,
                                        );
                                        if !(err < 0 as i32) {
                                            idx = *((*exp).u.re.matches.start)
                                                .offset(0 as i32 as isize);
                                            if idx >= 0 as i32
                                                && ((idx as u32) < first_idx
                                                    || idx as u32 == first_idx
                                                        && (*((*exp).u.re.matches.end).offset(0 as i32 as isize)
                                                            - *((*exp).u.re.matches.start).offset(0 as i32 as isize))
                                                            as u32 > match_len)
                                            {
                                                first_idx = idx as u32;
                                                first_rule = rule;
                                                match_len = (*((*exp).u.re.matches.end)
                                                    .offset(0 as i32 as isize)
                                                    - *((*exp).u.re.matches.start).offset(0 as i32 as isize))
                                                    as u32;
                                                current_match = &mut (*exp).u.re.matches;
                                                current_match_buf = inbuf;
                                            }
                                        }
                                    }
                                }
                            }
                            rule = (*rule).next;
                        }
                        fwrite(
                            inbuf.offset(bufpos as isize) as *const libc::c_void,
                            1 as i32 as size_t,
                            first_idx.wrapping_sub(bufpos) as size_t,
                            ofp,
                        );
                        if !first_rule.is_null() {
                            bufpos = *((*current_match).end).offset(0 as i32 as isize)
                                as u32;
                            node_free(result);
                            result = eval_statement_list(
                                (*((*first_rule).data as *mut Cons)).cdr as *mut List,
                                0 as *mut Environment,
                                &mut return_seen,
                            );
                            if return_seen != 0 {
                                current_block = 16824052130944227025;
                            } else {
                                current_block = 3512920355445576850;
                            }
                        } else {
                            bufpos = first_idx;
                            current_block = 3512920355445576850;
                        }
                    }
                }
            }
        }
    }
    return result;
}