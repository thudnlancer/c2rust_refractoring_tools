#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn iswalnum(__wc: wint_t) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn wctype(__property: *const libc::c_char) -> wctype_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn towupper(__wc: wint_t) -> wint_t;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn towlower(__wc: wint_t) -> wint_t;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn iswctype(__wc: wint_t, __desc: wctype_t) -> libc::c_int;
    fn gl_dynarray_resize(
        _: *mut dynarray_header,
        size: size_t,
        scratch: *mut libc::c_void,
        element_size: size_t,
    ) -> bool;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __btowc_alias(__c: libc::c_int) -> wint_t;
    fn wcrtomb(__s: *mut libc::c_char, __wc: wchar_t, __ps: *mut mbstate_t) -> size_t;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type __re_size_t = size_t;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = libc::c_ulong;
pub type reg_errcode_t = libc::c_int;
pub const _REG_ERPAREN: reg_errcode_t = 16;
pub const _REG_ESIZE: reg_errcode_t = 15;
pub const _REG_EEND: reg_errcode_t = 14;
pub const _REG_BADRPT: reg_errcode_t = 13;
pub const _REG_ESPACE: reg_errcode_t = 12;
pub const _REG_ERANGE: reg_errcode_t = 11;
pub const _REG_BADBR: reg_errcode_t = 10;
pub const _REG_EBRACE: reg_errcode_t = 9;
pub const _REG_EPAREN: reg_errcode_t = 8;
pub const _REG_EBRACK: reg_errcode_t = 7;
pub const _REG_ESUBREG: reg_errcode_t = 6;
pub const _REG_EESCAPE: reg_errcode_t = 5;
pub const _REG_ECTYPE: reg_errcode_t = 4;
pub const _REG_ECOLLATE: reg_errcode_t = 3;
pub const _REG_BADPAT: reg_errcode_t = 2;
pub const _REG_NOMATCH: reg_errcode_t = 1;
pub const _REG_NOERROR: reg_errcode_t = 0;
pub const _REG_ENOSYS: reg_errcode_t = -1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_dfa_t {
    pub nodes: *mut re_token_t,
    pub nodes_alloc: size_t,
    pub nodes_len: size_t,
    pub nexts: *mut Idx,
    pub org_indices: *mut Idx,
    pub edests: *mut re_node_set,
    pub eclosures: *mut re_node_set,
    pub inveclosures: *mut re_node_set,
    pub state_table: *mut re_state_table_entry,
    pub init_state: *mut re_dfastate_t,
    pub init_state_word: *mut re_dfastate_t,
    pub init_state_nl: *mut re_dfastate_t,
    pub init_state_begbuf: *mut re_dfastate_t,
    pub str_tree: *mut bin_tree_t,
    pub str_tree_storage: *mut bin_tree_storage_t,
    pub sb_char: re_bitset_ptr_t,
    pub str_tree_storage_idx: libc::c_int,
    pub state_hash_mask: re_hashval_t,
    pub init_node: Idx,
    pub nbackref: Idx,
    pub used_bkref_map: bitset_word_t,
    pub completed_bkref_map: bitset_word_t,
    #[bitfield(name = "has_plural_match", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "has_mb_node", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_utf8", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "map_notascii", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "word_ops_used", ty = "libc::c_uint", bits = "4..=4")]
    pub has_plural_match_has_mb_node_is_utf8_map_notascii_word_ops_used: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub mb_cur_max: libc::c_int,
    pub word_char: bitset_t,
    pub syntax: reg_syntax_t,
    pub subexp_map: *mut Idx,
}
pub type Idx = regoff_t;
pub type regoff_t = ssize_t;
pub type bitset_t = [bitset_word_t; 4];
pub type bitset_word_t = libc::c_ulong;
pub type re_hashval_t = __re_size_t;
pub type re_bitset_ptr_t = *mut bitset_word_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bin_tree_storage_t {
    pub next: *mut bin_tree_storage_t,
    pub data: [bin_tree_t; 15],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bin_tree_t {
    pub parent: *mut bin_tree_t,
    pub left: *mut bin_tree_t,
    pub right: *mut bin_tree_t,
    pub first: *mut bin_tree_t,
    pub next: *mut bin_tree_t,
    pub token: re_token_t,
    pub node_idx: Idx,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_token_t {
    pub opr: C2RustUnnamed,
    #[bitfield(name = "type_0", ty = "re_token_type_t", bits = "0..=7")]
    #[bitfield(name = "constraint", ty = "libc::c_uint", bits = "8..=17")]
    #[bitfield(name = "duplicated", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "opt_subexp", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "accept_mb", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "mb_partial", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "word_char", ty = "libc::c_uint", bits = "22..=22")]
    pub type_0_constraint_duplicated_opt_subexp_accept_mb_mb_partial_word_char: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum re_token_type_t {
    NON_TYPE = 0,
    CHARACTER = 1,
    END_OF_RE = 2,
    SIMPLE_BRACKET = 3,
    OP_BACK_REF = 4,
    OP_PERIOD = 5,
    COMPLEX_BRACKET = 6,
    OP_UTF8_PERIOD = 7,
    OP_OPEN_SUBEXP,
    OP_CLOSE_SUBEXP,
    OP_ALT,
    OP_DUP_ASTERISK,
    ANCHOR,
    CONCAT = 16,
    SUBEXP = 17,
    OP_DUP_PLUS = 18,
    OP_DUP_QUESTION,
    OP_OPEN_BRACKET,
    OP_CLOSE_BRACKET,
    OP_CHARSET_RANGE,
    OP_OPEN_DUP_NUM,
    OP_CLOSE_DUP_NUM,
    OP_NON_MATCH_LIST,
    OP_OPEN_COLL_ELEM,
    OP_CLOSE_COLL_ELEM,
    OP_OPEN_EQUIV_CLASS,
    OP_CLOSE_EQUIV_CLASS,
    OP_OPEN_CHAR_CLASS,
    OP_CLOSE_CHAR_CLASS,
    OP_WORD,
    OP_NOTWORD,
    OP_SPACE,
    OP_NOTSPACE,
    BACK_SLASH,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub c: libc::c_uchar,
    pub sbcset: re_bitset_ptr_t,
    pub mbcset: *mut re_charset_t,
    pub idx: Idx,
    pub ctx_type: re_context_type,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum re_context_type {
    INSIDE_WORD,
    WORD_FIRST,
    WORD_LAST,
    INSIDE_NOTWORD,
    LINE_FIRST,
    LINE_LAST,
    BUF_FIRST,
    BUF_LAST,
    WORD_DELIM,
    NOT_WORD_DELIM,
}  // end of enum

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_charset_t {
    pub mbchars: *mut wchar_t,
    pub range_starts: *mut wchar_t,
    pub range_ends: *mut wchar_t,
    pub char_classes: *mut wctype_t,
    #[bitfield(name = "non_match", ty = "libc::c_uint", bits = "0..=0")]
    pub non_match: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub nmbchars: Idx,
    pub ncoll_syms: Idx,
    pub nequiv_classes: Idx,
    pub nranges: Idx,
    pub nchar_classes: Idx,
}
pub type wctype_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_dfastate_t {
    pub hash: re_hashval_t,
    pub nodes: re_node_set,
    pub non_eps_nodes: re_node_set,
    pub inveclosure: re_node_set,
    pub entrance_nodes: *mut re_node_set,
    pub trtable: *mut *mut re_dfastate_t,
    pub word_trtable: *mut *mut re_dfastate_t,
    #[bitfield(name = "context", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "halt", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "accept_mb", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "has_backref", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "has_constraint", ty = "libc::c_uint", bits = "7..=7")]
    pub context_halt_accept_mb_has_backref_has_constraint: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_node_set {
    pub alloc: Idx,
    pub nelem: Idx,
    pub elems: *mut Idx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_state_table_entry {
    pub num: Idx,
    pub alloc: Idx,
    pub array: *mut *mut re_dfastate_t,
}
pub type regex_t = re_pattern_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_string_t {
    pub raw_mbs: *const libc::c_uchar,
    pub mbs: *mut libc::c_uchar,
    pub wcs: *mut wint_t,
    pub offsets: *mut Idx,
    pub cur_state: mbstate_t,
    pub raw_mbs_idx: Idx,
    pub valid_len: Idx,
    pub valid_raw_len: Idx,
    pub bufs_len: Idx,
    pub cur_idx: Idx,
    pub raw_len: Idx,
    pub len: Idx,
    pub raw_stop: Idx,
    pub stop: Idx,
    pub tip_context: libc::c_uint,
    pub trans: *mut libc::c_uchar,
    pub word_char: re_const_bitset_ptr_t,
    pub icase: libc::c_uchar,
    pub is_utf8: libc::c_uchar,
    pub map_notascii: libc::c_uchar,
    pub mbs_allocated: libc::c_uchar,
    pub offsets_needed: libc::c_uchar,
    pub newline_anchor: libc::c_uchar,
    pub word_ops_used: libc::c_uchar,
    pub mb_cur_max: libc::c_int,
}
pub type re_const_bitset_ptr_t = *const bitset_word_t;
pub type __mbstate_t = mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type wint_t = libc::c_uint;
pub type uintptr_t = libc::c_ulong;
pub const _ISalnum: C2RustUnnamed_4 = 8;
pub const _ISxdigit: C2RustUnnamed_4 = 4096;
pub const _ISpunct: C2RustUnnamed_4 = 4;
pub const _ISgraph: C2RustUnnamed_4 = 32768;
pub const _ISblank: C2RustUnnamed_4 = 1;
pub const _ISupper: C2RustUnnamed_4 = 256;
pub const _ISprint: C2RustUnnamed_4 = 16384;
pub const _ISdigit: C2RustUnnamed_4 = 2048;
pub const _ISalpha: C2RustUnnamed_4 = 1024;
pub const _ISspace: C2RustUnnamed_4 = 8192;
pub const _ISlower: C2RustUnnamed_4 = 512;
pub const _IScntrl: C2RustUnnamed_4 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ch: libc::c_uchar,
    pub name: *mut libc::c_uchar,
    pub wch: wchar_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bracket_elem_t {
    pub type_0: bracket_elem_type,
    pub opr: C2RustUnnamed_1,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum bracket_elem_type {
    SB_CHAR,
    MB_CHAR,
    EQUIV_CLASS,
    COLL_SYM,
    CHAR_CLASS,
}  // end of enum

pub type int_fast32_t = libc::c_long;
pub type uint_fast32_t = libc::c_ulong;
pub const CODESET: C2RustUnnamed_5 = 14;
pub type nl_item = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_match_context_t {
    pub input: re_string_t,
    pub dfa: *const re_dfa_t,
    pub eflags: libc::c_int,
    pub match_last: Idx,
    pub last_node: Idx,
    pub state_log: *mut *mut re_dfastate_t,
    pub state_log_top: Idx,
    pub nbkref_ents: Idx,
    pub abkref_ents: Idx,
    pub bkref_ents: *mut re_backref_cache_entry,
    pub max_mb_elem_len: libc::c_int,
    pub nsub_tops: Idx,
    pub asub_tops: Idx,
    pub sub_tops: *mut *mut re_sub_match_top_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_sub_match_top_t {
    pub str_idx: Idx,
    pub node: Idx,
    pub path: *mut state_array_t,
    pub alasts: Idx,
    pub nlasts: Idx,
    pub lasts: *mut *mut re_sub_match_last_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_sub_match_last_t {
    pub node: Idx,
    pub str_idx: Idx,
    pub path: state_array_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_array_t {
    pub next_idx: Idx,
    pub alloc: Idx,
    pub array: *mut *mut re_dfastate_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_backref_cache_entry {
    pub node: Idx,
    pub str_idx: Idx,
    pub subexp_from: Idx,
    pub subexp_to: Idx,
    pub eps_reachable_subexps_map: bitset_word_t,
    pub more: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_fail_stack_t {
    pub num: Idx,
    pub alloc: Idx,
    pub stack: *mut re_fail_stack_ent_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_fail_stack_ent_t {
    pub idx: Idx,
    pub node: Idx,
    pub regs: *mut regmatch_t,
    pub eps_via_nodes: re_node_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_list {
    pub u: C2RustUnnamed_2,
    pub scratch: [regmatch_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub dynarray_abstract: dynarray_header,
    pub dynarray_header: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut regmatch_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynarray_header {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_sift_context_t {
    pub sifted_states: *mut *mut re_dfastate_t,
    pub limited_states: *mut *mut re_dfastate_t,
    pub last_node: Idx,
    pub last_str_idx: Idx,
    pub limits: re_node_set,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
    _ISalnum = 8,
    _ISxdigit = 4096,
    _ISpunct = 4,
    _ISgraph = 32768,
    _ISblank = 1,
    _ISupper = 256,
    _ISprint = 16384,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISspace = 8192,
    _ISlower = 512,
    _IScntrl = 2,
}  // end of enum
er: C2RustUnnamed_4 = 512;
pub const _IScntrl: C2RustUnnamed_4 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ch: libc::c_uchar,
    pub name: *mut libc::c_uchar,
    pub wch: wchar_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bracket_elem_t {
    pub type_0: bracket_elem_type,
    pub opr: C2RustUnnamed_1,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum bracket_elem_type {
    SB_CHAR,
    MB_CHAR,
    EQUIV_CLASS,
    COLL_SYM,
    CHAR_CLASS,
}  // end of enum

pub type int_fast32_t = libc::c_long;
pub type uint_fast32_t = libc::c_ulong;
pub const CODESET: C2RustUnnamed_5 = 14;
pub type nl_item = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_match_context_t {
    pub input: re_string_t,
    pub dfa: *const re_dfa_t,
    pub eflags: libc::c_int,
    pub match_last: Idx,
    pub last_node: Idx,
    pub state_log: *mut *mut re_dfastate_t,
    pub state_log_top: Idx,
    pub nbkref_ents: Idx,
    pub abkref_ents: Idx,
    pub bkref_ents: *mut re_backref_cache_entry,
    pub max_mb_elem_len: libc::c_int,
    pub nsub_tops: Idx,
    pub asub_tops: Idx,
    pub sub_tops: *mut *mut re_sub_match_top_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_sub_match_top_t {
    pub str_idx: Idx,
    pub node: Idx,
    pub path: *mut state_array_t,
    pub alasts: Idx,
    pub nlasts: Idx,
    pub lasts: *mut *mut re_sub_match_last_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_sub_match_last_t {
    pub node: Idx,
    pub str_idx: Idx,
    pub path: state_array_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_array_t {
    pub next_idx: Idx,
    pub alloc: Idx,
    pub array: *mut *mut re_dfastate_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_backref_cache_entry {
    pub node: Idx,
    pub str_idx: Idx,
    pub subexp_from: Idx,
    pub subexp_to: Idx,
    pub eps_reachable_subexps_map: bitset_word_t,
    pub more: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_fail_stack_t {
    pub num: Idx,
    pub alloc: Idx,
    pub stack: *mut re_fail_stack_ent_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_fail_stack_ent_t {
    pub idx: Idx,
    pub node: Idx,
    pub regs: *mut regmatch_t,
    pub eps_via_nodes: re_node_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_list {
    pub u: C2RustUnnamed_2,
    pub scratch: [regmatch_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub dynarray_abstract: dynarray_header,
    pub dynarray_header: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut regmatch_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynarray_header {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_sift_context_t {
    pub sifted_states: *mut *mut re_dfastate_t,
    pub limited_states: *mut *mut re_dfastate_t,
    pub last_node: Idx,
    pub last_str_idx: Idx,
    pub limits: re_node_set,
}
pub type C2RustUnnamed_4 = libc::c_uint;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    CODESET = 14,
    _NL_NUM = 786449,
    _NL_NUM_LC_IDENTIFICATION = 786448,
    _NL_IDENTIFICATION_CODESET = 786447,
    _NL_IDENTIFICATION_CATEGORY = 786446,
    _NL_IDENTIFICATION_DATE = 786445,
    _NL_IDENTIFICATION_REVISION = 786444,
    _NL_IDENTIFICATION_ABBREVIATION = 786443,
    _NL_IDENTIFICATION_APPLICATION = 786442,
    _NL_IDENTIFICATION_AUDIENCE = 786441,
    _NL_IDENTIFICATION_TERRITORY = 786440,
    _NL_IDENTIFICATION_LANGUAGE = 786439,
    _NL_IDENTIFICATION_FAX = 786438,
    _NL_IDENTIFICATION_TEL = 786437,
    _NL_IDENTIFICATION_EMAIL = 786436,
    _NL_IDENTIFICATION_CONTACT = 786435,
    _NL_IDENTIFICATION_ADDRESS = 786434,
    _NL_IDENTIFICATION_SOURCE = 786433,
    _NL_IDENTIFICATION_TITLE = 786432,
    _NL_NUM_LC_MEASUREMENT = 720898,
    _NL_MEASUREMENT_CODESET = 720897,
    _NL_MEASUREMENT_MEASUREMENT = 720896,
    _NL_NUM_LC_TELEPHONE = 655365,
    _NL_TELEPHONE_CODESET = 655364,
    _NL_TELEPHONE_INT_PREFIX = 655363,
    _NL_TELEPHONE_INT_SELECT = 655362,
    _NL_TELEPHONE_TEL_DOM_FMT = 655361,
    _NL_TELEPHONE_TEL_INT_FMT = 655360,
    _NL_NUM_LC_ADDRESS = 589837,
    _NL_ADDRESS_CODESET = 589836,
    _NL_ADDRESS_LANG_LIB = 589835,
    _NL_ADDRESS_LANG_TERM = 589834,
    _NL_ADDRESS_LANG_AB = 589833,
    _NL_ADDRESS_LANG_NAME = 589832,
    _NL_ADDRESS_COUNTRY_ISBN = 589831,
    _NL_ADDRESS_COUNTRY_NUM = 589830,
    _NL_ADDRESS_COUNTRY_CAR = 589829,
    _NL_ADDRESS_COUNTRY_AB3 = 589828,
    _NL_ADDRESS_COUNTRY_AB2 = 589827,
    _NL_ADDRESS_COUNTRY_POST = 589826,
    _NL_ADDRESS_COUNTRY_NAME = 589825,
    _NL_ADDRESS_POSTAL_FMT = 589824,
    _NL_NUM_LC_NAME = 524295,
    _NL_NAME_CODESET = 524294,
    _NL_NAME_NAME_MS = 524293,
    _NL_NAME_NAME_MISS = 524292,
    _NL_NAME_NAME_MRS = 524291,
    _NL_NAME_NAME_MR = 524290,
    _NL_NAME_NAME_GEN = 524289,
    _NL_NAME_NAME_FMT = 524288,
    _NL_NUM_LC_PAPER = 458755,
    _NL_PAPER_CODESET = 458754,
    _NL_PAPER_WIDTH = 458753,
    _NL_PAPER_HEIGHT = 458752,
    _NL_NUM_LC_MESSAGES = 327685,
    _NL_MESSAGES_CODESET = 327684,
    __NOSTR = 327683,
    __YESSTR = 327682,
    __NOEXPR = 327681,
    __YESEXPR = 327680,
    _NL_NUM_LC_NUMERIC = 65542,
    _NL_NUMERIC_CODESET = 65541,
    _NL_NUMERIC_THOUSANDS_SEP_WC = 65540,
    _NL_NUMERIC_DECIMAL_POINT_WC = 65539,
    __GROUPING = 65538,
    THOUSEP = 65537,
    __THOUSANDS_SEP = 65537,
    RADIXCHAR = 65536,
    __DECIMAL_POINT = 65536,
    _NL_NUM_LC_MONETARY = 262190,
    _NL_MONETARY_CODESET = 262189,
    _NL_MONETARY_THOUSANDS_SEP_WC = 262188,
    _NL_MONETARY_DECIMAL_POINT_WC = 262187,
    _NL_MONETARY_CONVERSION_RATE = 262186,
    _NL_MONETARY_DUO_VALID_TO = 262185,
    _NL_MONETARY_DUO_VALID_FROM = 262184,
    _NL_MONETARY_UNO_VALID_TO = 262183,
    _NL_MONETARY_UNO_VALID_FROM = 262182,
    _NL_MONETARY_DUO_INT_N_SIGN_POSN = 262181,
    _NL_MONETARY_DUO_INT_P_SIGN_POSN = 262180,
    _NL_MONETARY_DUO_N_SIGN_POSN = 262179,
    _NL_MONETARY_DUO_P_SIGN_POSN = 262178,
    _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE = 262177,
    _NL_MONETARY_DUO_INT_N_CS_PRECEDES = 262176,
    _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE = 262175,
    _NL_MONETARY_DUO_INT_P_CS_PRECEDES = 262174,
    _NL_MONETARY_DUO_N_SEP_BY_SPACE = 262173,
    _NL_MONETARY_DUO_N_CS_PRECEDES = 262172,
    _NL_MONETARY_DUO_P_SEP_BY_SPACE = 262171,
    _NL_MONETARY_DUO_P_CS_PRECEDES = 262170,
    _NL_MONETARY_DUO_FRAC_DIGITS = 262169,
    _NL_MONETARY_DUO_INT_FRAC_DIGITS = 262168,
    _NL_MONETARY_DUO_CURRENCY_SYMBOL = 262167,
    _NL_MONETARY_DUO_INT_CURR_SYMBOL = 262166,
    __INT_N_SIGN_POSN = 262165,
    __INT_P_SIGN_POSN = 262164,
    __INT_N_SEP_BY_SPACE = 262163,
    __INT_N_CS_PRECEDES = 262162,
    __INT_P_SEP_BY_SPACE = 262161,
    __INT_P_CS_PRECEDES = 262160,
    _NL_MONETARY_CRNCYSTR = 262159,
    __N_SIGN_POSN = 262158,
    __P_SIGN_POSN = 262157,
    __N_SEP_BY_SPACE = 262156,
    __N_CS_PRECEDES = 262155,
    __P_SEP_BY_SPACE = 262154,
    __P_CS_PRECEDES = 262153,
    __FRAC_DIGITS = 262152,
    __INT_FRAC_DIGITS = 262151,
    __NEGATIVE_SIGN = 262150,
    __POSITIVE_SIGN = 262149,
    __MON_GROUPING = 262148,
    __MON_THOUSANDS_SEP = 262147,
    __MON_DECIMAL_POINT = 262146,
    __CURRENCY_SYMBOL = 262145,
    __INT_CURR_SYMBOL = 262144,
    _NL_NUM_LC_CTYPE = 86,
    _NL_CTYPE_EXTRA_MAP_14 = 85,
    _NL_CTYPE_EXTRA_MAP_13 = 84,
    _NL_CTYPE_EXTRA_MAP_12 = 83,
    _NL_CTYPE_EXTRA_MAP_11 = 82,
    _NL_CTYPE_EXTRA_MAP_10 = 81,
    _NL_CTYPE_EXTRA_MAP_9 = 80,
    _NL_CTYPE_EXTRA_MAP_8 = 79,
    _NL_CTYPE_EXTRA_MAP_7 = 78,
    _NL_CTYPE_EXTRA_MAP_6 = 77,
    _NL_CTYPE_EXTRA_MAP_5 = 76,
    _NL_CTYPE_EXTRA_MAP_4 = 75,
    _NL_CTYPE_EXTRA_MAP_3 = 74,
    _NL_CTYPE_EXTRA_MAP_2 = 73,
    _NL_CTYPE_EXTRA_MAP_1 = 72,
    _NL_CTYPE_NONASCII_CASE = 71,
    _NL_CTYPE_MAP_TO_NONASCII = 70,
    _NL_CTYPE_TRANSLIT_IGNORE = 69,
    _NL_CTYPE_TRANSLIT_IGNORE_LEN = 68,
    _NL_CTYPE_TRANSLIT_DEFAULT_MISSING = 67,
    _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN = 66,
    _NL_CTYPE_TRANSLIT_TO_TBL = 65,
    _NL_CTYPE_TRANSLIT_TO_IDX = 64,
    _NL_CTYPE_TRANSLIT_FROM_TBL = 63,
    _NL_CTYPE_TRANSLIT_FROM_IDX = 62,
    _NL_CTYPE_TRANSLIT_TAB_SIZE = 61,
    _NL_CTYPE_OUTDIGIT9_WC = 60,
    _NL_CTYPE_OUTDIGIT8_WC = 59,
    _NL_CTYPE_OUTDIGIT7_WC = 58,
    _NL_CTYPE_OUTDIGIT6_WC = 57,
    _NL_CTYPE_OUTDIGIT5_WC = 56,
    _NL_CTYPE_OUTDIGIT4_WC = 55,
    _NL_CTYPE_OUTDIGIT3_WC = 54,
    _NL_CTYPE_OUTDIGIT2_WC = 53,
    _NL_CTYPE_OUTDIGIT1_WC = 52,
    _NL_CTYPE_OUTDIGIT0_WC = 51,
    _NL_CTYPE_OUTDIGIT9_MB = 50,
    _NL_CTYPE_OUTDIGIT8_MB = 49,
    _NL_CTYPE_OUTDIGIT7_MB = 48,
    _NL_CTYPE_OUTDIGIT6_MB = 47,
    _NL_CTYPE_OUTDIGIT5_MB = 46,
    _NL_CTYPE_OUTDIGIT4_MB = 45,
    _NL_CTYPE_OUTDIGIT3_MB = 44,
    _NL_CTYPE_OUTDIGIT2_MB = 43,
    _NL_CTYPE_OUTDIGIT1_MB = 42,
    _NL_CTYPE_OUTDIGIT0_MB = 41,
    _NL_CTYPE_INDIGITS9_WC = 40,
    _NL_CTYPE_INDIGITS8_WC = 39,
    _NL_CTYPE_INDIGITS7_WC = 38,
    _NL_CTYPE_INDIGITS6_WC = 37,
    _NL_CTYPE_INDIGITS5_WC = 36,
    _NL_CTYPE_INDIGITS4_WC = 35,
    _NL_CTYPE_INDIGITS3_WC = 34,
    _NL_CTYPE_INDIGITS2_WC = 33,
    _NL_CTYPE_INDIGITS1_WC = 32,
    _NL_CTYPE_INDIGITS0_WC = 31,
    _NL_CTYPE_INDIGITS_WC_LEN = 30,
    _NL_CTYPE_INDIGITS9_MB = 29,
    _NL_CTYPE_INDIGITS8_MB = 28,
    _NL_CTYPE_INDIGITS7_MB = 27,
    _NL_CTYPE_INDIGITS6_MB = 26,
    _NL_CTYPE_INDIGITS5_MB = 25,
    _NL_CTYPE_INDIGITS4_MB = 24,
    _NL_CTYPE_INDIGITS3_MB = 23,
    _NL_CTYPE_INDIGITS2_MB = 22,
    _NL_CTYPE_INDIGITS1_MB = 21,
    _NL_CTYPE_INDIGITS0_MB = 20,
    _NL_CTYPE_INDIGITS_MB_LEN = 19,
    _NL_CTYPE_MAP_OFFSET = 18,
    _NL_CTYPE_CLASS_OFFSET = 17,
    _NL_CTYPE_TOLOWER32 = 16,
    _NL_CTYPE_TOUPPER32 = 15,
    _NL_CTYPE_CODESET_NAME = 14,
    _NL_CTYPE_MB_CUR_MAX = 13,
    _NL_CTYPE_WIDTH = 12,
    _NL_CTYPE_MAP_NAMES = 11,
    _NL_CTYPE_CLASS_NAMES = 10,
    _NL_CTYPE_GAP6 = 9,
    _NL_CTYPE_GAP5 = 8,
    _NL_CTYPE_GAP4 = 7,
    _NL_CTYPE_GAP3 = 6,
    _NL_CTYPE_CLASS32 = 5,
    _NL_CTYPE_GAP2 = 4,
    _NL_CTYPE_TOLOWER = 3,
    _NL_CTYPE_GAP1 = 2,
    _NL_CTYPE_TOUPPER = 1,
    _NL_CTYPE_CLASS = 0,
    _NL_NUM_LC_COLLATE = 196627,
    _NL_COLLATE_CODESET = 196626,
    _NL_COLLATE_COLLSEQWC = 196625,
    _NL_COLLATE_COLLSEQMB = 196624,
    _NL_COLLATE_SYMB_EXTRAMB = 196623,
    _NL_COLLATE_SYMB_TABLEMB = 196622,
    _NL_COLLATE_SYMB_HASH_SIZEMB = 196621,
    _NL_COLLATE_INDIRECTWC = 196620,
    _NL_COLLATE_EXTRAWC = 196619,
    _NL_COLLATE_WEIGHTWC = 196618,
    _NL_COLLATE_TABLEWC = 196617,
    _NL_COLLATE_GAP3 = 196616,
    _NL_COLLATE_GAP2 = 196615,
    _NL_COLLATE_GAP1 = 196614,
    _NL_COLLATE_INDIRECTMB = 196613,
    _NL_COLLATE_EXTRAMB = 196612,
    _NL_COLLATE_WEIGHTMB = 196611,
    _NL_COLLATE_TABLEMB = 196610,
    _NL_COLLATE_RULESETS = 196609,
    _NL_COLLATE_NRULES = 196608,
    _NL_NUM_LC_TIME = 131231,
    _NL_WABALTMON_12 = 131230,
    _NL_WABALTMON_11 = 131229,
    _NL_WABALTMON_10 = 131228,
    _NL_WABALTMON_9 = 131227,
    _NL_WABALTMON_8 = 131226,
    _NL_WABALTMON_7 = 131225,
    _NL_WABALTMON_6 = 131224,
    _NL_WABALTMON_5 = 131223,
    _NL_WABALTMON_4 = 131222,
    _NL_WABALTMON_3 = 131221,
    _NL_WABALTMON_2 = 131220,
    _NL_WABALTMON_1 = 131219,
    _NL_ABALTMON_12 = 131218,
    _NL_ABALTMON_11 = 131217,
    _NL_ABALTMON_10 = 131216,
    _NL_ABALTMON_9 = 131215,
    _NL_ABALTMON_8 = 131214,
    _NL_ABALTMON_7 = 131213,
    _NL_ABALTMON_6 = 131212,
    _NL_ABALTMON_5 = 131211,
    _NL_ABALTMON_4 = 131210,
    _NL_ABALTMON_3 = 131209,
    _NL_ABALTMON_2 = 131208,
    _NL_ABALTMON_1 = 131207,
    _NL_WALTMON_12 = 131206,
    _NL_WALTMON_11 = 131205,
    _NL_WALTMON_10 = 131204,
    _NL_WALTMON_9 = 131203,
    _NL_WALTMON_8 = 131202,
    _NL_WALTMON_7 = 131201,
    _NL_WALTMON_6 = 131200,
    _NL_WALTMON_5 = 131199,
    _NL_WALTMON_4 = 131198,
    _NL_WALTMON_3 = 131197,
    _NL_WALTMON_2 = 131196,
    _NL_WALTMON_1 = 131195,
    __ALTMON_12 = 131194,
    __ALTMON_11 = 131193,
    __ALTMON_10 = 131192,
    __ALTMON_9 = 131191,
    __ALTMON_8 = 131190,
    __ALTMON_7 = 131189,
    __ALTMON_6 = 131188,
    __ALTMON_5 = 131187,
    __ALTMON_4 = 131186,
    __ALTMON_3 = 131185,
    __ALTMON_2 = 131184,
    __ALTMON_1 = 131183,
    _NL_TIME_CODESET = 131182,
    _NL_W_DATE_FMT = 131181,
    _DATE_FMT = 131180,
    _NL_TIME_TIMEZONE = 131179,
    _NL_TIME_CAL_DIRECTION = 131178,
    _NL_TIME_FIRST_WORKDAY = 131177,
    _NL_TIME_FIRST_WEEKDAY = 131176,
    _NL_TIME_WEEK_1STWEEK = 131175,
    _NL_TIME_WEEK_1STDAY = 131174,
    _NL_TIME_WEEK_NDAYS = 131173,
    _NL_WERA_T_FMT = 131172,
    _NL_WERA_D_T_FMT = 131171,
    _NL_WALT_DIGITS = 131170,
    _NL_WERA_D_FMT = 131169,
    _NL_WERA_YEAR = 131168,
    _NL_WT_FMT_AMPM = 131167,
    _NL_WT_FMT = 131166,
    _NL_WD_FMT = 131165,
    _NL_WD_T_FMT = 131164,
    _NL_WPM_STR = 131163,
    _NL_WAM_STR = 131162,
    _NL_WMON_12 = 131161,
    _NL_WMON_11 = 131160,
    _NL_WMON_10 = 131159,
    _NL_WMON_9 = 131158,
    _NL_WMON_8 = 131157,
    _NL_WMON_7 = 131156,
    _NL_WMON_6 = 131155,
    _NL_WMON_5 = 131154,
    _NL_WMON_4 = 131153,
    _NL_WMON_3 = 131152,
    _NL_WMON_2 = 131151,
    _NL_WMON_1 = 131150,
    _NL_WABMON_12 = 131149,
    _NL_WABMON_11 = 131148,
    _NL_WABMON_10 = 131147,
    _NL_WABMON_9 = 131146,
    _NL_WABMON_8 = 131145,
    _NL_WABMON_7 = 131144,
    _NL_WABMON_6 = 131143,
    _NL_WABMON_5 = 131142,
    _NL_WABMON_4 = 131141,
    _NL_WABMON_3 = 131140,
    _NL_WABMON_2 = 131139,
    _NL_WABMON_1 = 131138,
    _NL_WDAY_7 = 131137,
    _NL_WDAY_6 = 131136,
    _NL_WDAY_5 = 131135,
    _NL_WDAY_4 = 131134,
    _NL_WDAY_3 = 131133,
    _NL_WDAY_2 = 131132,
    _NL_WDAY_1 = 131131,
    _NL_WABDAY_7 = 131130,
    _NL_WABDAY_6 = 131129,
    _NL_WABDAY_5 = 131128,
    _NL_WABDAY_4 = 131127,
    _NL_WABDAY_3 = 131126,
    _NL_WABDAY_2 = 131125,
    _NL_WABDAY_1 = 131124,
    _NL_TIME_ERA_ENTRIES = 131123,
    _NL_TIME_ERA_NUM_ENTRIES = 131122,
    ERA_T_FMT = 131121,
    ERA_D_T_FMT = 131120,
    ALT_DIGITS = 131119,
    ERA_D_FMT = 131118,
    __ERA_YEAR = 131117,
    ERA = 131116,
    T_FMT_AMPM = 131115,
    T_FMT = 131114,
    D_FMT = 131113,
    D_T_FMT = 131112,
    PM_STR = 131111,
    AM_STR = 131110,
    MON_12 = 131109,
    MON_11 = 131108,
    MON_10 = 131107,
    MON_9 = 131106,
    MON_8 = 131105,
    MON_7 = 131104,
    MON_6 = 131103,
    MON_5 = 131102,
    MON_4 = 131101,
    MON_3 = 131100,
    MON_2 = 131099,
    MON_1 = 131098,
    ABMON_12 = 131097,
    ABMON_11 = 131096,
    ABMON_10 = 131095,
    ABMON_9 = 131094,
    ABMON_8 = 131093,
    ABMON_7 = 131092,
    ABMON_6 = 131091,
    ABMON_5 = 131090,
    ABMON_4 = 131089,
    ABMON_3 = 131088,
    ABMON_2 = 131087,
    ABMON_1 = 131086,
    DAY_7 = 131085,
    DAY_6 = 131084,
    DAY_5 = 131083,
    DAY_4 = 131082,
    DAY_3 = 131081,
    DAY_2 = 131080,
    DAY_1 = 131079,
    ABDAY_7 = 131078,
    ABDAY_6 = 131077,
    ABDAY_5 = 131076,
    ABDAY_4 = 131075,
    ABDAY_3 = 131074,
    ABDAY_2 = 131073,
    ABDAY_1 = 131072,
}  // end of enum

unsafe extern "C" fn re_string_wchar_at(
    mut pstr: *const re_string_t,
    mut idx: Idx,
) -> wint_t {
    if (*pstr).mb_cur_max == 1 as libc::c_int {
        return *((*pstr).mbs).offset(idx as isize) as wint_t;
    }
    return *((*pstr).wcs).offset(idx as isize);
}
#[inline]
unsafe extern "C" fn bitset_mask(
    mut dest: *mut bitset_word_t,
    mut src: *const bitset_word_t,
) {
    let mut bitset_i: libc::c_int = 0;
    bitset_i = 0 as libc::c_int;
    while bitset_i
        < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 64 as libc::c_int - 1 as libc::c_int) / 64 as libc::c_int
    {
        let ref mut fresh0 = *dest.offset(bitset_i as isize);
        *fresh0 &= *src.offset(bitset_i as isize);
        bitset_i += 1;
        bitset_i;
    }
}
#[inline]
unsafe extern "C" fn bitset_not(mut set: *mut bitset_word_t) {
    let mut bitset_i: libc::c_int = 0;
    bitset_i = 0 as libc::c_int;
    while bitset_i
        < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
            / 64 as libc::c_int
    {
        *set.offset(bitset_i as isize) = !*set.offset(bitset_i as isize);
        bitset_i += 1;
        bitset_i;
    }
    if (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
        % 64 as libc::c_int != 0 as libc::c_int
    {
        *set
            .offset(
                ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                    / 64 as libc::c_int - 1 as libc::c_int) as isize,
            ) = ((1 as libc::c_int as bitset_word_t)
            << (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int) % 64 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !*set
                .offset(
                    ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                        + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                        / 64 as libc::c_int - 1 as libc::c_int) as isize,
                );
    }
}
#[inline]
unsafe extern "C" fn bitset_set(mut set: *mut bitset_word_t, mut i: Idx) {
    let ref mut fresh1 = *set.offset((i / 64 as libc::c_int as libc::c_long) as isize);
    *fresh1
        |= (1 as libc::c_int as bitset_word_t) << i % 64 as libc::c_int as libc::c_long;
}
unsafe extern "C" fn re_string_char_size_at(
    mut pstr: *const re_string_t,
    mut idx: Idx,
) -> libc::c_int {
    let mut byte_idx: libc::c_int = 0;
    if (*pstr).mb_cur_max == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    byte_idx = 1 as libc::c_int;
    while (idx + byte_idx as libc::c_long) < (*pstr).valid_len {
        if *((*pstr).wcs).offset((idx + byte_idx as libc::c_long) as isize)
            != 0xffffffff as libc::c_uint
        {
            break;
        }
        byte_idx += 1;
        byte_idx;
    }
    return byte_idx;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn bitset_contain(mut set: *const bitset_word_t, mut i: Idx) -> bool {
    return *set.offset((i / 64 as libc::c_int as libc::c_long) as isize)
        >> i % 64 as libc::c_int as libc::c_long & 1 as libc::c_int as libc::c_ulong
        != 0;
}
unsafe extern "C" fn re_string_elem_size_at(
    mut pstr: *const re_string_t,
    mut idx: Idx,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn __dynarray_error_marker() -> size_t {
    return -(1 as libc::c_int) as size_t;
}
#[inline]
unsafe extern "C" fn bitset_merge(
    mut dest: *mut bitset_word_t,
    mut src: *const bitset_word_t,
) {
    let mut bitset_i: libc::c_int = 0;
    bitset_i = 0 as libc::c_int;
    while bitset_i
        < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 64 as libc::c_int - 1 as libc::c_int) / 64 as libc::c_int
    {
        let ref mut fresh2 = *dest.offset(bitset_i as isize);
        *fresh2 |= *src.offset(bitset_i as isize);
        bitset_i += 1;
        bitset_i;
    }
}
#[inline]
unsafe extern "C" fn bitset_empty(mut set: *mut bitset_word_t) {
    memset(
        set as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<bitset_t>() as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn bitset_copy(
    mut dest: *mut bitset_word_t,
    mut src: *const bitset_word_t,
) {
    memcpy(
        dest as *mut libc::c_void,
        src as *const libc::c_void,
        ::core::mem::size_of::<bitset_t>() as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn bitset_clear(mut set: *mut bitset_word_t, mut i: Idx) {
    let ref mut fresh3 = *set.offset((i / 64 as libc::c_int as libc::c_long) as isize);
    *fresh3
        &= !((1 as libc::c_int as bitset_word_t)
            << i % 64 as libc::c_int as libc::c_long);
}
#[inline]
unsafe extern "C" fn bitset_set_all(mut set: *mut bitset_word_t) {
    memset(
        set as *mut libc::c_void,
        -(1 as libc::c_int),
        (::core::mem::size_of::<bitset_word_t>() as libc::c_ulong)
            .wrapping_mul(
                ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int) / 64 as libc::c_int) as libc::c_ulong,
            ),
    );
    if (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
        % 64 as libc::c_int != 0 as libc::c_int
    {
        *set
            .offset(
                ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                    / 64 as libc::c_int - 1 as libc::c_int) as isize,
            ) = ((1 as libc::c_int as bitset_word_t)
            << (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int) % 64 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    }
}
#[inline]
unsafe extern "C" fn btowc(mut __c: libc::c_int) -> wint_t {
    return if 0 != 0 && __c >= '\0' as i32 && __c <= '\u{7f}' as i32 {
        __c as wint_t
    } else {
        __btowc_alias(__c)
    };
}
unsafe extern "C" fn re_string_allocate(
    mut pstr: *mut re_string_t,
    mut str: *const libc::c_char,
    mut len: Idx,
    mut init_len: Idx,
    mut trans: *mut libc::c_uchar,
    mut icase: bool,
    mut dfa: *const re_dfa_t,
) -> reg_errcode_t {
    let mut ret: reg_errcode_t = _REG_NOERROR;
    let mut init_buf_len: Idx = 0;
    if init_len < (*dfa).mb_cur_max as libc::c_long {
        init_len = (*dfa).mb_cur_max as Idx;
    }
    init_buf_len = if (len + 1 as libc::c_int as libc::c_long) < init_len {
        len + 1 as libc::c_int as libc::c_long
    } else {
        init_len
    };
    re_string_construct_common(str, len, pstr, trans, icase, dfa);
    ret = re_string_realloc_buffers(pstr, init_buf_len);
    if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return ret;
    }
    (*pstr).word_char = ((*dfa).word_char).as_ptr();
    (*pstr).word_ops_used = (*dfa).word_ops_used() as libc::c_uchar;
    (*pstr)
        .mbs = if (*pstr).mbs_allocated as libc::c_int != 0 {
        (*pstr).mbs
    } else {
        str as *mut libc::c_uchar
    };
    (*pstr)
        .valid_len = if (*pstr).mbs_allocated as libc::c_int != 0
        || (*dfa).mb_cur_max > 1 as libc::c_int
    {
        0 as libc::c_int as libc::c_long
    } else {
        len
    };
    (*pstr).valid_raw_len = (*pstr).valid_len;
    return _REG_NOERROR;
}
unsafe extern "C" fn re_string_construct(
    mut pstr: *mut re_string_t,
    mut str: *const libc::c_char,
    mut len: Idx,
    mut trans: *mut libc::c_uchar,
    mut icase: bool,
    mut dfa: *const re_dfa_t,
) -> reg_errcode_t {
    let mut ret: reg_errcode_t = _REG_NOERROR;
    memset(
        pstr as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<re_string_t>() as libc::c_ulong,
    );
    re_string_construct_common(str, len, pstr, trans, icase, dfa);
    if len > 0 as libc::c_int as libc::c_long {
        ret = re_string_realloc_buffers(pstr, len + 1 as libc::c_int as libc::c_long);
        if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return ret;
        }
    }
    (*pstr)
        .mbs = if (*pstr).mbs_allocated as libc::c_int != 0 {
        (*pstr).mbs
    } else {
        str as *mut libc::c_uchar
    };
    if icase {
        if (*dfa).mb_cur_max > 1 as libc::c_int {
            loop {
                ret = build_wcs_upper_buffer(pstr);
                if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    return ret;
                }
                if (*pstr).valid_raw_len >= len {
                    break;
                }
                if (*pstr).bufs_len
                    > (*pstr).valid_len + (*dfa).mb_cur_max as libc::c_long
                {
                    break;
                }
                ret = re_string_realloc_buffers(
                    pstr,
                    (*pstr).bufs_len * 2 as libc::c_int as libc::c_long,
                );
                if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    return ret;
                }
            }
        } else {
            build_upper_buffer(pstr);
        }
    } else if (*dfa).mb_cur_max > 1 as libc::c_int {
        build_wcs_buffer(pstr);
    } else if !trans.is_null() {
        re_string_translate_buffer(pstr);
    } else {
        (*pstr).valid_len = (*pstr).bufs_len;
        (*pstr).valid_raw_len = (*pstr).bufs_len;
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn re_string_realloc_buffers(
    mut pstr: *mut re_string_t,
    mut new_buf_len: Idx,
) -> reg_errcode_t {
    if (*pstr).mb_cur_max > 1 as libc::c_int {
        let mut new_wcs: *mut wint_t = 0 as *mut wint_t;
        let max_object_size: size_t = if (::core::mem::size_of::<wint_t>()
            as libc::c_ulong) < ::core::mem::size_of::<Idx>() as libc::c_ulong
        {
            ::core::mem::size_of::<Idx>() as libc::c_ulong
        } else {
            ::core::mem::size_of::<wint_t>() as libc::c_ulong
        };
        if ((if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < (18446744073709551615 as libc::c_ulong).wrapping_div(max_object_size)
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            (18446744073709551615 as libc::c_ulong).wrapping_div(max_object_size)
        }) < new_buf_len as libc::c_ulong) as libc::c_int as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        new_wcs = realloc(
            (*pstr).wcs as *mut libc::c_void,
            (new_buf_len as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<wint_t>() as libc::c_ulong),
        ) as *mut wint_t;
        if (new_wcs == 0 as *mut libc::c_void as *mut wint_t) as libc::c_int
            as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        (*pstr).wcs = new_wcs;
        if !((*pstr).offsets).is_null() {
            let mut new_offsets: *mut Idx = realloc(
                (*pstr).offsets as *mut libc::c_void,
                (new_buf_len as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
            ) as *mut Idx;
            if (new_offsets == 0 as *mut libc::c_void as *mut Idx) as libc::c_int
                as libc::c_long != 0
            {
                return _REG_ESPACE;
            }
            (*pstr).offsets = new_offsets;
        }
    }
    if (*pstr).mbs_allocated != 0 {
        let mut new_mbs: *mut libc::c_uchar = realloc(
            (*pstr).mbs as *mut libc::c_void,
            (new_buf_len as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        ) as *mut libc::c_uchar;
        if (new_mbs == 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        (*pstr).mbs = new_mbs;
    }
    (*pstr).bufs_len = new_buf_len;
    return _REG_NOERROR;
}
unsafe extern "C" fn re_string_construct_common(
    mut str: *const libc::c_char,
    mut len: Idx,
    mut pstr: *mut re_string_t,
    mut trans: *mut libc::c_uchar,
    mut icase: bool,
    mut dfa: *const re_dfa_t,
) {
    (*pstr).raw_mbs = str as *const libc::c_uchar;
    (*pstr).len = len;
    (*pstr).raw_len = len;
    (*pstr).trans = trans;
    (*pstr).icase = icase as libc::c_uchar;
    (*pstr)
        .mbs_allocated = (!trans.is_null() || icase as libc::c_int != 0) as libc::c_int
        as libc::c_uchar;
    (*pstr).mb_cur_max = (*dfa).mb_cur_max;
    (*pstr).is_utf8 = (*dfa).is_utf8() as libc::c_uchar;
    (*pstr).map_notascii = (*dfa).map_notascii() as libc::c_uchar;
    (*pstr).stop = (*pstr).len;
    (*pstr).raw_stop = (*pstr).stop;
}
unsafe extern "C" fn build_wcs_buffer(mut pstr: *mut re_string_t) {
    let mut buf: [libc::c_uchar; 64] = [0; 64];
    let mut prev_st: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed_0 { __wch: 0 },
    };
    let mut byte_idx: Idx = 0;
    let mut end_idx: Idx = 0;
    let mut remain_len: Idx = 0;
    let mut mbclen: size_t = 0;
    end_idx = if (*pstr).bufs_len > (*pstr).len {
        (*pstr).len
    } else {
        (*pstr).bufs_len
    };
    byte_idx = (*pstr).valid_len;
    while byte_idx < end_idx {
        let mut wc: wchar_t = 0;
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        remain_len = end_idx - byte_idx;
        prev_st = (*pstr).cur_state;
        if ((*pstr).trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            let mut i: libc::c_int = 0;
            let mut ch: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*pstr).mb_cur_max && (i as libc::c_long) < remain_len {
                ch = *((*pstr).raw_mbs)
                    .offset(
                        ((*pstr).raw_mbs_idx + byte_idx + i as libc::c_long) as isize,
                    ) as libc::c_int;
                let ref mut fresh4 = *((*pstr).mbs)
                    .offset((byte_idx + i as libc::c_long) as isize);
                *fresh4 = *((*pstr).trans).offset(ch as isize);
                buf[i as usize] = *fresh4;
                i += 1;
                i;
            }
            p = buf.as_mut_ptr() as *const libc::c_char;
        } else {
            p = ((*pstr).raw_mbs as *const libc::c_char)
                .offset((*pstr).raw_mbs_idx as isize)
                .offset(byte_idx as isize);
        }
        mbclen = rpl_mbrtowc(&mut wc, p, remain_len as size_t, &mut (*pstr).cur_state);
        if (mbclen == -(1 as libc::c_int) as size_t
            || mbclen == 0 as libc::c_int as libc::c_ulong
            || mbclen == -(2 as libc::c_int) as size_t
                && (*pstr).bufs_len >= (*pstr).len) as libc::c_int as libc::c_long != 0
        {
            mbclen = 1 as libc::c_int as size_t;
            wc = *((*pstr).raw_mbs).offset(((*pstr).raw_mbs_idx + byte_idx) as isize)
                as wchar_t;
            if ((*pstr).trans != 0 as *mut libc::c_void as *mut libc::c_uchar)
                as libc::c_int as libc::c_long != 0
            {
                wc = *((*pstr).trans).offset(wc as isize) as wchar_t;
            }
            (*pstr).cur_state = prev_st;
        } else if (mbclen == -(2 as libc::c_int) as size_t) as libc::c_int
            as libc::c_long != 0
        {
            (*pstr).cur_state = prev_st;
            break;
        }
        let fresh5 = byte_idx;
        byte_idx = byte_idx + 1;
        *((*pstr).wcs).offset(fresh5 as isize) = wc as wint_t;
        remain_len = (byte_idx as libc::c_ulong)
            .wrapping_add(mbclen)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as Idx;
        while byte_idx < remain_len {
            let fresh6 = byte_idx;
            byte_idx = byte_idx + 1;
            *((*pstr).wcs).offset(fresh6 as isize) = 0xffffffff as libc::c_uint;
        }
    }
    (*pstr).valid_len = byte_idx;
    (*pstr).valid_raw_len = byte_idx;
}
unsafe extern "C" fn build_wcs_upper_buffer(
    mut pstr: *mut re_string_t,
) -> reg_errcode_t {
    let mut prev_st: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed_0 { __wch: 0 },
    };
    let mut src_idx: Idx = 0;
    let mut byte_idx: Idx = 0;
    let mut end_idx: Idx = 0;
    let mut remain_len: Idx = 0;
    let mut mbclen: size_t = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    byte_idx = (*pstr).valid_len;
    end_idx = if (*pstr).bufs_len > (*pstr).len {
        (*pstr).len
    } else {
        (*pstr).bufs_len
    };
    let mut wc_0: wchar_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_block_88: u64;
    if (*pstr).map_notascii == 0 && ((*pstr).trans).is_null()
        && (*pstr).offsets_needed == 0
    {
        loop {
            if !(byte_idx < end_idx) {
                current_block_88 = 5689316957504528238;
                break;
            }
            let mut wc: wchar_t = 0;
            let mut ch: libc::c_uchar = *((*pstr).raw_mbs)
                .offset(((*pstr).raw_mbs_idx + byte_idx) as isize);
            if ch as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
                && mbsinit(&mut (*pstr).cur_state) != 0
            {
                let mut wcu: wchar_t = towupper(ch as wint_t) as wchar_t;
                if wcu & !(0x7f as libc::c_int) == 0 as libc::c_int {
                    *((*pstr).mbs).offset(byte_idx as isize) = wcu as libc::c_uchar;
                    *((*pstr).wcs).offset(byte_idx as isize) = wcu as wint_t;
                    byte_idx += 1;
                    byte_idx;
                    continue;
                }
            }
            remain_len = end_idx - byte_idx;
            prev_st = (*pstr).cur_state;
            mbclen = rpl_mbrtowc(
                &mut wc,
                ((*pstr).raw_mbs as *const libc::c_char)
                    .offset((*pstr).raw_mbs_idx as isize)
                    .offset(byte_idx as isize),
                remain_len as size_t,
                &mut (*pstr).cur_state,
            );
            if ((0 as libc::c_int as libc::c_ulong) < mbclen
                && mbclen < -(2 as libc::c_int) as size_t) as libc::c_int as libc::c_long
                != 0
            {
                let mut wcu_0: wchar_t = towupper(wc as wint_t) as wchar_t;
                if wcu_0 != wc {
                    let mut mbcdlen: size_t = 0;
                    mbcdlen = wcrtomb(buf.as_mut_ptr(), wcu_0, &mut prev_st);
                    if (mbclen == mbcdlen) as libc::c_int as libc::c_long != 0 {
                        memcpy(
                            ((*pstr).mbs).offset(byte_idx as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            mbclen,
                        );
                    } else {
                        src_idx = byte_idx;
                        current_block_88 = 6981066898412724707;
                        break;
                    }
                } else {
                    memcpy(
                        ((*pstr).mbs).offset(byte_idx as isize) as *mut libc::c_void,
                        ((*pstr).raw_mbs)
                            .offset((*pstr).raw_mbs_idx as isize)
                            .offset(byte_idx as isize) as *const libc::c_void,
                        mbclen,
                    );
                }
                let fresh7 = byte_idx;
                byte_idx = byte_idx + 1;
                *((*pstr).wcs).offset(fresh7 as isize) = wcu_0 as wint_t;
                remain_len = (byte_idx as libc::c_ulong)
                    .wrapping_add(mbclen)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as Idx;
                while byte_idx < remain_len {
                    let fresh8 = byte_idx;
                    byte_idx = byte_idx + 1;
                    *((*pstr).wcs).offset(fresh8 as isize) = 0xffffffff as libc::c_uint;
                }
            } else if mbclen == -(1 as libc::c_int) as size_t
                || mbclen == 0 as libc::c_int as libc::c_ulong
                || mbclen == -(2 as libc::c_int) as size_t
                    && (*pstr).bufs_len >= (*pstr).len
            {
                *((*pstr).mbs).offset(byte_idx as isize) = ch;
                let fresh9 = byte_idx;
                byte_idx = byte_idx + 1;
                *((*pstr).wcs).offset(fresh9 as isize) = ch as wchar_t as wint_t;
                if (mbclen == -(1 as libc::c_int) as size_t) as libc::c_int
                    as libc::c_long != 0
                {
                    (*pstr).cur_state = prev_st;
                }
            } else {
                (*pstr).cur_state = prev_st;
                current_block_88 = 5689316957504528238;
                break;
            }
        }
        match current_block_88 {
            6981066898412724707 => {}
            _ => {
                (*pstr).valid_len = byte_idx;
                (*pstr).valid_raw_len = byte_idx;
                return _REG_NOERROR;
            }
        }
    } else {
        src_idx = (*pstr).valid_raw_len;
        current_block_88 = 2891135413264362348;
    }
    loop {
        match current_block_88 {
            2891135413264362348 => {
                if !(byte_idx < end_idx) {
                    break;
                }
                wc_0 = 0;
                p = 0 as *const libc::c_char;
                current_block_88 = 6981066898412724707;
            }
            _ => {
                remain_len = end_idx - byte_idx;
                prev_st = (*pstr).cur_state;
                if ((*pstr).trans != 0 as *mut libc::c_void as *mut libc::c_uchar)
                    as libc::c_int as libc::c_long != 0
                {
                    let mut i: libc::c_int = 0;
                    let mut ch_0: libc::c_int = 0;
                    i = 0 as libc::c_int;
                    while i < (*pstr).mb_cur_max && (i as libc::c_long) < remain_len {
                        ch_0 = *((*pstr).raw_mbs)
                            .offset(
                                ((*pstr).raw_mbs_idx + src_idx + i as libc::c_long) as isize,
                            ) as libc::c_int;
                        buf[i
                            as usize] = *((*pstr).trans).offset(ch_0 as isize)
                            as libc::c_char;
                        i += 1;
                        i;
                    }
                    p = buf.as_mut_ptr() as *const libc::c_char;
                } else {
                    p = ((*pstr).raw_mbs as *const libc::c_char)
                        .offset((*pstr).raw_mbs_idx as isize)
                        .offset(src_idx as isize);
                }
                mbclen = rpl_mbrtowc(
                    &mut wc_0,
                    p,
                    remain_len as size_t,
                    &mut (*pstr).cur_state,
                );
                if ((0 as libc::c_int as libc::c_ulong) < mbclen
                    && mbclen < -(2 as libc::c_int) as size_t) as libc::c_int
                    as libc::c_long != 0
                {
                    let mut wcu_1: wchar_t = towupper(wc_0 as wint_t) as wchar_t;
                    if wcu_1 != wc_0 {
                        let mut mbcdlen_0: size_t = 0;
                        mbcdlen_0 = wcrtomb(buf.as_mut_ptr(), wcu_1, &mut prev_st);
                        if (mbclen == mbcdlen_0) as libc::c_int as libc::c_long != 0 {
                            memcpy(
                                ((*pstr).mbs).offset(byte_idx as isize)
                                    as *mut libc::c_void,
                                buf.as_mut_ptr() as *const libc::c_void,
                                mbclen,
                            );
                        } else if mbcdlen_0 != -(1 as libc::c_int) as size_t {
                            let mut i_0: size_t = 0;
                            if (byte_idx as libc::c_ulong).wrapping_add(mbcdlen_0)
                                > (*pstr).bufs_len as libc::c_ulong
                            {
                                (*pstr).cur_state = prev_st;
                                break;
                            } else {
                                if ((*pstr).offsets).is_null() {
                                    (*pstr)
                                        .offsets = malloc(
                                        ((*pstr).bufs_len as libc::c_ulong)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<Idx>() as libc::c_ulong,
                                            ),
                                    ) as *mut Idx;
                                    if ((*pstr).offsets).is_null() {
                                        return _REG_ESPACE;
                                    }
                                }
                                if (*pstr).offsets_needed == 0 {
                                    i_0 = 0 as libc::c_int as size_t;
                                    while i_0 < byte_idx as size_t {
                                        *((*pstr).offsets).offset(i_0 as isize) = i_0 as Idx;
                                        i_0 = i_0.wrapping_add(1);
                                        i_0;
                                    }
                                    (*pstr).offsets_needed = 1 as libc::c_int as libc::c_uchar;
                                }
                                memcpy(
                                    ((*pstr).mbs).offset(byte_idx as isize)
                                        as *mut libc::c_void,
                                    buf.as_mut_ptr() as *const libc::c_void,
                                    mbcdlen_0,
                                );
                                *((*pstr).wcs).offset(byte_idx as isize) = wcu_1 as wint_t;
                                *((*pstr).offsets).offset(byte_idx as isize) = src_idx;
                                i_0 = 1 as libc::c_int as size_t;
                                while i_0 < mbcdlen_0 {
                                    *((*pstr).offsets)
                                        .offset(
                                            (byte_idx as libc::c_ulong).wrapping_add(i_0) as isize,
                                        ) = (src_idx as libc::c_ulong)
                                        .wrapping_add(
                                            (if i_0 < mbclen {
                                                i_0
                                            } else {
                                                mbclen.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            }),
                                        ) as Idx;
                                    *((*pstr).wcs)
                                        .offset(
                                            (byte_idx as libc::c_ulong).wrapping_add(i_0) as isize,
                                        ) = 0xffffffff as libc::c_uint;
                                    i_0 = i_0.wrapping_add(1);
                                    i_0;
                                }
                                (*pstr)
                                    .len = ((*pstr).len as libc::c_ulong)
                                    .wrapping_add(mbcdlen_0.wrapping_sub(mbclen)) as Idx as Idx;
                                if (*pstr).raw_stop > src_idx {
                                    (*pstr)
                                        .stop = ((*pstr).stop as libc::c_ulong)
                                        .wrapping_add(mbcdlen_0.wrapping_sub(mbclen)) as Idx as Idx;
                                }
                                end_idx = if (*pstr).bufs_len > (*pstr).len {
                                    (*pstr).len
                                } else {
                                    (*pstr).bufs_len
                                };
                                byte_idx = (byte_idx as libc::c_ulong)
                                    .wrapping_add(mbcdlen_0) as Idx as Idx;
                                src_idx = (src_idx as libc::c_ulong).wrapping_add(mbclen)
                                    as Idx as Idx;
                                current_block_88 = 2891135413264362348;
                                continue;
                            }
                        } else {
                            memcpy(
                                ((*pstr).mbs).offset(byte_idx as isize)
                                    as *mut libc::c_void,
                                p as *const libc::c_void,
                                mbclen,
                            );
                        }
                    } else {
                        memcpy(
                            ((*pstr).mbs).offset(byte_idx as isize) as *mut libc::c_void,
                            p as *const libc::c_void,
                            mbclen,
                        );
                    }
                    if ((*pstr).offsets_needed as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as libc::c_long != 0
                    {
                        let mut i_1: size_t = 0;
                        i_1 = 0 as libc::c_int as size_t;
                        while i_1 < mbclen {
                            *((*pstr).offsets)
                                .offset(
                                    (byte_idx as libc::c_ulong).wrapping_add(i_1) as isize,
                                ) = (src_idx as libc::c_ulong).wrapping_add(i_1) as Idx;
                            i_1 = i_1.wrapping_add(1);
                            i_1;
                        }
                    }
                    src_idx = (src_idx as libc::c_ulong).wrapping_add(mbclen) as Idx
                        as Idx;
                    let fresh10 = byte_idx;
                    byte_idx = byte_idx + 1;
                    *((*pstr).wcs).offset(fresh10 as isize) = wcu_1 as wint_t;
                    remain_len = (byte_idx as libc::c_ulong)
                        .wrapping_add(mbclen)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as Idx;
                    while byte_idx < remain_len {
                        let fresh11 = byte_idx;
                        byte_idx = byte_idx + 1;
                        *((*pstr).wcs)
                            .offset(fresh11 as isize) = 0xffffffff as libc::c_uint;
                    }
                    current_block_88 = 2891135413264362348;
                } else if mbclen == -(1 as libc::c_int) as size_t
                    || mbclen == 0 as libc::c_int as libc::c_ulong
                    || mbclen == -(2 as libc::c_int) as size_t
                        && (*pstr).bufs_len >= (*pstr).len
                {
                    let mut ch_1: libc::c_int = *((*pstr).raw_mbs)
                        .offset(((*pstr).raw_mbs_idx + src_idx) as isize) as libc::c_int;
                    if ((*pstr).trans != 0 as *mut libc::c_void as *mut libc::c_uchar)
                        as libc::c_int as libc::c_long != 0
                    {
                        ch_1 = *((*pstr).trans).offset(ch_1 as isize) as libc::c_int;
                    }
                    *((*pstr).mbs).offset(byte_idx as isize) = ch_1 as libc::c_uchar;
                    if ((*pstr).offsets_needed as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as libc::c_long != 0
                    {
                        *((*pstr).offsets).offset(byte_idx as isize) = src_idx;
                    }
                    src_idx += 1;
                    src_idx;
                    let fresh12 = byte_idx;
                    byte_idx = byte_idx + 1;
                    *((*pstr).wcs).offset(fresh12 as isize) = ch_1 as wint_t;
                    if (mbclen == -(1 as libc::c_int) as size_t) as libc::c_int
                        as libc::c_long != 0
                    {
                        (*pstr).cur_state = prev_st;
                    }
                    current_block_88 = 2891135413264362348;
                } else {
                    (*pstr).cur_state = prev_st;
                    break;
                }
            }
        }
    }
    (*pstr).valid_len = byte_idx;
    (*pstr).valid_raw_len = src_idx;
    return _REG_NOERROR;
}
unsafe extern "C" fn re_string_skip_chars(
    mut pstr: *mut re_string_t,
    mut new_raw_idx: Idx,
    mut last_wc: *mut wint_t,
) -> Idx {
    let mut prev_st: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed_0 { __wch: 0 },
    };
    let mut rawbuf_idx: Idx = 0;
    let mut mbclen: size_t = 0;
    let mut wc: wint_t = 0xffffffff as libc::c_uint;
    rawbuf_idx = (*pstr).raw_mbs_idx + (*pstr).valid_raw_len;
    while rawbuf_idx < new_raw_idx {
        let mut wc2: wchar_t = 0;
        let mut remain_len: Idx = (*pstr).raw_len - rawbuf_idx;
        prev_st = (*pstr).cur_state;
        mbclen = rpl_mbrtowc(
            &mut wc2,
            ((*pstr).raw_mbs as *const libc::c_char).offset(rawbuf_idx as isize),
            remain_len as size_t,
            &mut (*pstr).cur_state,
        );
        if (mbclen == -(2 as libc::c_int) as size_t
            || mbclen == -(1 as libc::c_int) as size_t
            || mbclen == 0 as libc::c_int as libc::c_ulong) as libc::c_int
            as libc::c_long != 0
        {
            if mbclen == 0 as libc::c_int as libc::c_ulong
                || remain_len == 0 as libc::c_int as libc::c_long
            {
                wc = '\0' as i32 as wint_t;
            } else {
                wc = *(((*pstr).raw_mbs).offset(rawbuf_idx as isize)
                    as *mut libc::c_uchar) as wint_t;
            }
            mbclen = 1 as libc::c_int as size_t;
            (*pstr).cur_state = prev_st;
        } else {
            wc = wc2 as wint_t;
        }
        rawbuf_idx = (rawbuf_idx as libc::c_ulong).wrapping_add(mbclen) as Idx as Idx;
    }
    *last_wc = wc;
    return rawbuf_idx;
}
unsafe extern "C" fn build_upper_buffer(mut pstr: *mut re_string_t) {
    let mut char_idx: Idx = 0;
    let mut end_idx: Idx = 0;
    end_idx = if (*pstr).bufs_len > (*pstr).len {
        (*pstr).len
    } else {
        (*pstr).bufs_len
    };
    char_idx = (*pstr).valid_len;
    while char_idx < end_idx {
        let mut ch: libc::c_int = *((*pstr).raw_mbs)
            .offset(((*pstr).raw_mbs_idx + char_idx) as isize) as libc::c_int;
        if ((*pstr).trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            ch = *((*pstr).trans).offset(ch as isize) as libc::c_int;
        }
        *((*pstr).mbs)
            .offset(
                char_idx as isize,
            ) = ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = ch;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(ch);
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(ch as isize);
            }
            __res
        }) as libc::c_uchar;
        char_idx += 1;
        char_idx;
    }
    (*pstr).valid_len = char_idx;
    (*pstr).valid_raw_len = char_idx;
}
unsafe extern "C" fn re_string_translate_buffer(mut pstr: *mut re_string_t) {
    let mut buf_idx: Idx = 0;
    let mut end_idx: Idx = 0;
    end_idx = if (*pstr).bufs_len > (*pstr).len {
        (*pstr).len
    } else {
        (*pstr).bufs_len
    };
    buf_idx = (*pstr).valid_len;
    while buf_idx < end_idx {
        let mut ch: libc::c_int = *((*pstr).raw_mbs)
            .offset(((*pstr).raw_mbs_idx + buf_idx) as isize) as libc::c_int;
        *((*pstr).mbs).offset(buf_idx as isize) = *((*pstr).trans).offset(ch as isize);
        buf_idx += 1;
        buf_idx;
    }
    (*pstr).valid_len = buf_idx;
    (*pstr).valid_raw_len = buf_idx;
}
unsafe extern "C" fn re_string_reconstruct(
    mut pstr: *mut re_string_t,
    mut idx: Idx,
    mut eflags: libc::c_int,
) -> reg_errcode_t {
    let mut offset: Idx = 0;
    if ((*pstr).raw_mbs_idx <= idx) as libc::c_int as libc::c_long != 0 {
        offset = idx - (*pstr).raw_mbs_idx;
    } else {
        if (*pstr).mb_cur_max > 1 as libc::c_int {
            memset(
                &mut (*pstr).cur_state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
        }
        (*pstr).len = (*pstr).raw_len;
        (*pstr).stop = (*pstr).raw_stop;
        (*pstr).valid_len = 0 as libc::c_int as Idx;
        (*pstr).raw_mbs_idx = 0 as libc::c_int as Idx;
        (*pstr).valid_raw_len = 0 as libc::c_int as Idx;
        (*pstr).offsets_needed = 0 as libc::c_int as libc::c_uchar;
        (*pstr)
            .tip_context = (if eflags & 1 as libc::c_int != 0 {
            ((1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
        } else {
            (1 as libc::c_int) << 1 as libc::c_int
                | ((1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
        }) as libc::c_uint;
        if (*pstr).mbs_allocated == 0 {
            (*pstr).mbs = (*pstr).raw_mbs as *mut libc::c_uchar;
        }
        offset = idx;
    }
    if (offset != 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0 {
        if (offset < (*pstr).valid_raw_len) as libc::c_int as libc::c_long != 0 {
            if (*pstr).offsets_needed as libc::c_long != 0 {
                let mut low: Idx = 0 as libc::c_int as Idx;
                let mut high: Idx = (*pstr).valid_len;
                let mut mid: Idx = 0;
                loop {
                    mid = (high + low) / 2 as libc::c_int as libc::c_long;
                    if *((*pstr).offsets).offset(mid as isize) > offset {
                        high = mid;
                    } else {
                        if !(*((*pstr).offsets).offset(mid as isize) < offset) {
                            break;
                        }
                        low = mid + 1 as libc::c_int as libc::c_long;
                    }
                    if !(low < high) {
                        break;
                    }
                }
                if *((*pstr).offsets).offset(mid as isize) < offset {
                    mid += 1;
                    mid;
                }
                (*pstr)
                    .tip_context = re_string_context_at(
                    pstr,
                    mid - 1 as libc::c_int as libc::c_long,
                    eflags,
                );
                if (*pstr).valid_len > offset && mid == offset
                    && *((*pstr).offsets).offset(mid as isize) == offset
                {
                    memmove(
                        (*pstr).wcs as *mut libc::c_void,
                        ((*pstr).wcs).offset(offset as isize) as *const libc::c_void,
                        (((*pstr).valid_len - offset) as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<wint_t>() as libc::c_ulong,
                            ),
                    );
                    memmove(
                        (*pstr).mbs as *mut libc::c_void,
                        ((*pstr).mbs).offset(offset as isize) as *const libc::c_void,
                        ((*pstr).valid_len - offset) as libc::c_ulong,
                    );
                    (*pstr).valid_len -= offset;
                    (*pstr).valid_raw_len -= offset;
                    low = 0 as libc::c_int as Idx;
                    while low < (*pstr).valid_len {
                        *((*pstr).offsets)
                            .offset(
                                low as isize,
                            ) = *((*pstr).offsets).offset((low + offset) as isize)
                            - offset;
                        low += 1;
                        low;
                    }
                } else {
                    (*pstr).len = (*pstr).raw_len - idx + offset;
                    (*pstr).stop = (*pstr).raw_stop - idx + offset;
                    (*pstr).offsets_needed = 0 as libc::c_int as libc::c_uchar;
                    while mid > 0 as libc::c_int as libc::c_long
                        && *((*pstr).offsets)
                            .offset((mid - 1 as libc::c_int as libc::c_long) as isize)
                            == offset
                    {
                        mid -= 1;
                        mid;
                    }
                    while mid < (*pstr).valid_len {
                        if *((*pstr).wcs).offset(mid as isize)
                            != 0xffffffff as libc::c_uint
                        {
                            break;
                        }
                        mid += 1;
                        mid;
                    }
                    if mid == (*pstr).valid_len {
                        (*pstr).valid_len = 0 as libc::c_int as Idx;
                    } else {
                        (*pstr)
                            .valid_len = *((*pstr).offsets).offset(mid as isize)
                            - offset;
                        if (*pstr).valid_len != 0 {
                            low = 0 as libc::c_int as Idx;
                            while low < (*pstr).valid_len {
                                *((*pstr).wcs)
                                    .offset(low as isize) = 0xffffffff as libc::c_uint;
                                low += 1;
                                low;
                            }
                            memset(
                                (*pstr).mbs as *mut libc::c_void,
                                255 as libc::c_int,
                                (*pstr).valid_len as libc::c_ulong,
                            );
                        }
                    }
                    (*pstr).valid_raw_len = (*pstr).valid_len;
                }
            } else {
                (*pstr)
                    .tip_context = re_string_context_at(
                    pstr,
                    offset - 1 as libc::c_int as libc::c_long,
                    eflags,
                );
                if (*pstr).mb_cur_max > 1 as libc::c_int {
                    memmove(
                        (*pstr).wcs as *mut libc::c_void,
                        ((*pstr).wcs).offset(offset as isize) as *const libc::c_void,
                        (((*pstr).valid_len - offset) as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<wint_t>() as libc::c_ulong,
                            ),
                    );
                }
                if (*pstr).mbs_allocated as libc::c_long != 0 {
                    memmove(
                        (*pstr).mbs as *mut libc::c_void,
                        ((*pstr).mbs).offset(offset as isize) as *const libc::c_void,
                        ((*pstr).valid_len - offset) as libc::c_ulong,
                    );
                }
                (*pstr).valid_len -= offset;
                (*pstr).valid_raw_len -= offset;
                if (*pstr).valid_len > 0 as libc::c_int as libc::c_long {} else {
                    unreachable!();
                };
            }
        } else {
            let mut prev_valid_len: Idx = (*pstr).valid_len;
            if (*pstr).offsets_needed as libc::c_long != 0 {
                (*pstr).len = (*pstr).raw_len - idx + offset;
                (*pstr).stop = (*pstr).raw_stop - idx + offset;
                (*pstr).offsets_needed = 0 as libc::c_int as libc::c_uchar;
            }
            (*pstr).valid_len = 0 as libc::c_int as Idx;
            if (*pstr).mb_cur_max > 1 as libc::c_int {
                let mut wcs_idx: Idx = 0;
                let mut wc: wint_t = 0xffffffff as libc::c_uint;
                if (*pstr).is_utf8 != 0 {
                    let mut raw: *const libc::c_uchar = 0 as *const libc::c_uchar;
                    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
                    let mut end: *const libc::c_uchar = 0 as *const libc::c_uchar;
                    raw = ((*pstr).raw_mbs).offset((*pstr).raw_mbs_idx as isize);
                    end = raw
                        .offset((offset - (*pstr).mb_cur_max as libc::c_long) as isize);
                    if end < (*pstr).raw_mbs {
                        end = (*pstr).raw_mbs;
                    }
                    p = raw.offset(offset as isize).offset(-(1 as libc::c_int as isize));
                    while p >= end {
                        if *p as libc::c_int & 0xc0 as libc::c_int != 0x80 as libc::c_int
                        {
                            let mut cur_state: mbstate_t = mbstate_t {
                                __count: 0,
                                __value: C2RustUnnamed_0 { __wch: 0 },
                            };
                            let mut wc2: wchar_t = 0;
                            let mut mlen: Idx = raw
                                .offset((*pstr).len as isize)
                                .offset_from(p) as libc::c_long;
                            let mut buf: [libc::c_uchar; 6] = [0; 6];
                            let mut mbclen: size_t = 0;
                            let mut pp: *const libc::c_uchar = p;
                            if ((*pstr).trans
                                != 0 as *mut libc::c_void as *mut libc::c_uchar)
                                as libc::c_int as libc::c_long != 0
                            {
                                let mut i: libc::c_int = (if mlen
                                    < 6 as libc::c_int as libc::c_long
                                {
                                    mlen
                                } else {
                                    6 as libc::c_int as libc::c_long
                                }) as libc::c_int;
                                loop {
                                    i -= 1;
                                    if !(i >= 0 as libc::c_int) {
                                        break;
                                    }
                                    buf[i
                                        as usize] = *((*pstr).trans)
                                        .offset(*p.offset(i as isize) as isize);
                                }
                                pp = buf.as_mut_ptr();
                            }
                            memset(
                                &mut cur_state as *mut mbstate_t as *mut libc::c_void,
                                0 as libc::c_int,
                                ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
                            );
                            mbclen = rpl_mbrtowc(
                                &mut wc2,
                                pp as *const libc::c_char,
                                mlen as size_t,
                                &mut cur_state,
                            );
                            if raw.offset(offset as isize).offset_from(p) as libc::c_long
                                as libc::c_ulong <= mbclen
                                && mbclen < -(2 as libc::c_int) as size_t
                            {
                                memset(
                                    &mut (*pstr).cur_state as *mut mbstate_t
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
                                );
                                (*pstr)
                                    .valid_len = mbclen
                                    .wrapping_sub(
                                        raw.offset(offset as isize).offset_from(p) as libc::c_long
                                            as libc::c_ulong,
                                    ) as Idx;
                                wc = wc2 as wint_t;
                            }
                            break;
                        } else {
                            p = p.offset(-1);
                            p;
                        }
                    }
                }
                if wc == 0xffffffff as libc::c_uint {
                    (*pstr).valid_len = re_string_skip_chars(pstr, idx, &mut wc) - idx;
                }
                if wc == 0xffffffff as libc::c_uint {
                    (*pstr)
                        .tip_context = re_string_context_at(
                        pstr,
                        prev_valid_len - 1 as libc::c_int as libc::c_long,
                        eflags,
                    );
                } else {
                    (*pstr)
                        .tip_context = (if ((*pstr).word_ops_used as libc::c_int
                        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                        && (iswalnum(wc) != 0 || wc == '_' as i32 as libc::c_uint)
                    {
                        1 as libc::c_int
                    } else if wc == '\n' as i32 as libc::c_uint
                        && (*pstr).newline_anchor as libc::c_int != 0
                    {
                        (1 as libc::c_int) << 1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_uint;
                }
                if (*pstr).valid_len != 0 {
                    wcs_idx = 0 as libc::c_int as Idx;
                    while wcs_idx < (*pstr).valid_len {
                        *((*pstr).wcs)
                            .offset(wcs_idx as isize) = 0xffffffff as libc::c_uint;
                        wcs_idx += 1;
                        wcs_idx;
                    }
                    if (*pstr).mbs_allocated != 0 {
                        memset(
                            (*pstr).mbs as *mut libc::c_void,
                            255 as libc::c_int,
                            (*pstr).valid_len as libc::c_ulong,
                        );
                    }
                }
                (*pstr).valid_raw_len = (*pstr).valid_len;
            } else {
                let mut c: libc::c_int = *((*pstr).raw_mbs)
                    .offset(
                        ((*pstr).raw_mbs_idx + offset - 1 as libc::c_int as libc::c_long)
                            as isize,
                    ) as libc::c_int;
                (*pstr).valid_raw_len = 0 as libc::c_int as Idx;
                if !((*pstr).trans).is_null() {
                    c = *((*pstr).trans).offset(c as isize) as libc::c_int;
                }
                (*pstr)
                    .tip_context = (if bitset_contain((*pstr).word_char, c as Idx)
                    as libc::c_int != 0
                {
                    1 as libc::c_int
                } else if c == '\n' as i32 && (*pstr).newline_anchor as libc::c_int != 0
                {
                    (1 as libc::c_int) << 1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_uint;
            }
        }
        if (*pstr).mbs_allocated as libc::c_long == 0 {
            (*pstr).mbs = ((*pstr).mbs).offset(offset as isize);
        }
    }
    (*pstr).raw_mbs_idx = idx;
    (*pstr).len -= offset;
    (*pstr).stop -= offset;
    if (*pstr).mb_cur_max > 1 as libc::c_int {
        if (*pstr).icase != 0 {
            let mut ret: reg_errcode_t = build_wcs_upper_buffer(pstr);
            if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return ret;
            }
        } else {
            build_wcs_buffer(pstr);
        }
    } else if (*pstr).mbs_allocated as libc::c_long != 0 {
        if (*pstr).icase != 0 {
            build_upper_buffer(pstr);
        } else if !((*pstr).trans).is_null() {
            re_string_translate_buffer(pstr);
        }
    } else {
        (*pstr).valid_len = (*pstr).len;
    }
    (*pstr).cur_idx = 0 as libc::c_int as Idx;
    return _REG_NOERROR;
}
unsafe extern "C" fn re_string_peek_byte_case(
    mut pstr: *const re_string_t,
    mut idx: Idx,
) -> libc::c_uchar {
    let mut ch: libc::c_int = 0;
    let mut off: Idx = 0;
    if ((*pstr).mbs_allocated == 0) as libc::c_int as libc::c_long != 0 {
        return *((*pstr).mbs).offset(((*pstr).cur_idx + idx) as isize);
    }
    if (*pstr).mb_cur_max > 1 as libc::c_int
        && !(*((*pstr).wcs).offset(((*pstr).cur_idx + idx) as isize)
            != 0xffffffff as libc::c_uint
            && ((*pstr).valid_len
                == (*pstr).cur_idx + idx + 1 as libc::c_int as libc::c_long
                || *((*pstr).wcs)
                    .offset(
                        ((*pstr).cur_idx + idx + 1 as libc::c_int as libc::c_long)
                            as isize,
                    ) != 0xffffffff as libc::c_uint))
    {
        return *((*pstr).mbs).offset(((*pstr).cur_idx + idx) as isize);
    }
    off = (*pstr).cur_idx + idx;
    if (*pstr).offsets_needed != 0 {
        off = *((*pstr).offsets).offset(off as isize);
    }
    ch = *((*pstr).raw_mbs).offset(((*pstr).raw_mbs_idx + off) as isize) as libc::c_int;
    if (*pstr).offsets_needed as libc::c_int != 0
        && !(ch & !(0x7f as libc::c_int) == 0 as libc::c_int)
    {
        return *((*pstr).mbs).offset(((*pstr).cur_idx + idx) as isize);
    }
    return ch as libc::c_uchar;
}
unsafe extern "C" fn re_string_fetch_byte_case(
    mut pstr: *mut re_string_t,
) -> libc::c_uchar {
    if ((*pstr).mbs_allocated == 0) as libc::c_int as libc::c_long != 0 {
        let fresh13 = (*pstr).cur_idx;
        (*pstr).cur_idx = (*pstr).cur_idx + 1;
        return *((*pstr).mbs).offset(fresh13 as isize);
    }
    if (*pstr).offsets_needed != 0 {
        let mut off: Idx = 0;
        let mut ch: libc::c_int = 0;
        if !((*pstr).cur_idx == (*pstr).valid_len
            || *((*pstr).wcs).offset((*pstr).cur_idx as isize)
                != 0xffffffff as libc::c_uint)
        {
            let fresh14 = (*pstr).cur_idx;
            (*pstr).cur_idx = (*pstr).cur_idx + 1;
            return *((*pstr).mbs).offset(fresh14 as isize);
        }
        off = *((*pstr).offsets).offset((*pstr).cur_idx as isize);
        ch = *((*pstr).raw_mbs).offset(((*pstr).raw_mbs_idx + off) as isize)
            as libc::c_int;
        if !(ch & !(0x7f as libc::c_int) == 0 as libc::c_int) {
            let fresh15 = (*pstr).cur_idx;
            (*pstr).cur_idx = (*pstr).cur_idx + 1;
            return *((*pstr).mbs).offset(fresh15 as isize);
        }
        (*pstr).cur_idx += re_string_char_size_at(pstr, (*pstr).cur_idx) as libc::c_long;
        return ch as libc::c_uchar;
    }
    let fresh16 = (*pstr).cur_idx;
    (*pstr).cur_idx = (*pstr).cur_idx + 1;
    return *((*pstr).raw_mbs).offset(((*pstr).raw_mbs_idx + fresh16) as isize);
}
unsafe extern "C" fn re_string_destruct(mut pstr: *mut re_string_t) {
    rpl_free((*pstr).wcs as *mut libc::c_void);
    rpl_free((*pstr).offsets as *mut libc::c_void);
    if (*pstr).mbs_allocated != 0 {
        rpl_free((*pstr).mbs as *mut libc::c_void);
    }
}
unsafe extern "C" fn re_string_context_at(
    mut input: *const re_string_t,
    mut idx: Idx,
    mut eflags: libc::c_int,
) -> libc::c_uint {
    let mut c: libc::c_int = 0;
    if (idx < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0 {
        return (*input).tip_context;
    }
    if (idx == (*input).len) as libc::c_int as libc::c_long != 0 {
        return (if eflags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            (((1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int
        } else {
            (1 as libc::c_int) << 1 as libc::c_int
                | (((1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int
        }) as libc::c_uint;
    }
    if (*input).mb_cur_max > 1 as libc::c_int {
        let mut wc: wint_t = 0;
        let mut wc_idx: Idx = idx;
        while *((*input).wcs).offset(wc_idx as isize) == 0xffffffff as libc::c_uint {
            if wc_idx >= 0 as libc::c_int as libc::c_long {} else {
                unreachable!();
            };
            wc_idx -= 1;
            wc_idx;
            if wc_idx < 0 as libc::c_int as libc::c_long {
                return (*input).tip_context;
            }
        }
        wc = *((*input).wcs).offset(wc_idx as isize);
        if ((*input).word_ops_used as libc::c_int != 0 as libc::c_int) as libc::c_int
            as libc::c_long != 0
            && (iswalnum(wc) != 0 || wc == '_' as i32 as libc::c_uint)
        {
            return 1 as libc::c_int as libc::c_uint;
        }
        return (if wc == '\n' as i32 as libc::c_uint
            && (*input).newline_anchor as libc::c_int != 0
        {
            (1 as libc::c_int) << 1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
    } else {
        c = *((*input).mbs).offset(idx as isize) as libc::c_int;
        if bitset_contain((*input).word_char, c as Idx) {
            return 1 as libc::c_int as libc::c_uint;
        }
        return (if c == '\n' as i32 && (*input).newline_anchor as libc::c_int != 0 {
            (1 as libc::c_int) << 1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
    };
}
unsafe extern "C" fn re_node_set_alloc(
    mut set: *mut re_node_set,
    mut size: Idx,
) -> reg_errcode_t {
    (*set).alloc = size;
    (*set).nelem = 0 as libc::c_int as Idx;
    (*set)
        .elems = malloc(
        (size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
    ) as *mut Idx;
    if ((*set).elems == 0 as *mut libc::c_void as *mut Idx) as libc::c_int
        as libc::c_long != 0
        && (1 as libc::c_int != 0 || size != 0 as libc::c_int as libc::c_long)
    {
        return _REG_ESPACE;
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn re_node_set_init_1(
    mut set: *mut re_node_set,
    mut elem: Idx,
) -> reg_errcode_t {
    (*set).alloc = 1 as libc::c_int as Idx;
    (*set).nelem = 1 as libc::c_int as Idx;
    (*set)
        .elems = malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
    ) as *mut Idx;
    if ((*set).elems == 0 as *mut libc::c_void as *mut Idx) as libc::c_int
        as libc::c_long != 0
    {
        (*set).nelem = 0 as libc::c_int as Idx;
        (*set).alloc = (*set).nelem;
        return _REG_ESPACE;
    }
    *((*set).elems).offset(0 as libc::c_int as isize) = elem;
    return _REG_NOERROR;
}
unsafe extern "C" fn re_node_set_init_2(
    mut set: *mut re_node_set,
    mut elem1: Idx,
    mut elem2: Idx,
) -> reg_errcode_t {
    (*set).alloc = 2 as libc::c_int as Idx;
    (*set)
        .elems = malloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
    ) as *mut Idx;
    if ((*set).elems == 0 as *mut libc::c_void as *mut Idx) as libc::c_int
        as libc::c_long != 0
    {
        return _REG_ESPACE;
    }
    if elem1 == elem2 {
        (*set).nelem = 1 as libc::c_int as Idx;
        *((*set).elems).offset(0 as libc::c_int as isize) = elem1;
    } else {
        (*set).nelem = 2 as libc::c_int as Idx;
        if elem1 < elem2 {
            *((*set).elems).offset(0 as libc::c_int as isize) = elem1;
            *((*set).elems).offset(1 as libc::c_int as isize) = elem2;
        } else {
            *((*set).elems).offset(0 as libc::c_int as isize) = elem2;
            *((*set).elems).offset(1 as libc::c_int as isize) = elem1;
        }
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn re_node_set_init_copy(
    mut dest: *mut re_node_set,
    mut src: *const re_node_set,
) -> reg_errcode_t {
    (*dest).nelem = (*src).nelem;
    if (*src).nelem > 0 as libc::c_int as libc::c_long {
        (*dest).alloc = (*dest).nelem;
        (*dest)
            .elems = malloc(
            ((*dest).alloc as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        ) as *mut Idx;
        if ((*dest).elems == 0 as *mut libc::c_void as *mut Idx) as libc::c_int
            as libc::c_long != 0
        {
            (*dest).nelem = 0 as libc::c_int as Idx;
            (*dest).alloc = (*dest).nelem;
            return _REG_ESPACE;
        }
        memcpy(
            (*dest).elems as *mut libc::c_void,
            (*src).elems as *const libc::c_void,
            ((*src).nelem as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        );
    } else {
        memset(
            dest as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<re_node_set>() as libc::c_ulong,
        );
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn re_node_set_add_intersect(
    mut dest: *mut re_node_set,
    mut src1: *const re_node_set,
    mut src2: *const re_node_set,
) -> reg_errcode_t {
    let mut i1: Idx = 0;
    let mut i2: Idx = 0;
    let mut is: Idx = 0;
    let mut id: Idx = 0;
    let mut delta: Idx = 0;
    let mut sbase: Idx = 0;
    if (*src1).nelem == 0 as libc::c_int as libc::c_long
        || (*src2).nelem == 0 as libc::c_int as libc::c_long
    {
        return _REG_NOERROR;
    }
    if (*src1).nelem + (*src2).nelem + (*dest).nelem > (*dest).alloc {
        let mut new_alloc: Idx = (*src1).nelem + (*src2).nelem + (*dest).alloc;
        let mut new_elems: *mut Idx = realloc(
            (*dest).elems as *mut libc::c_void,
            (new_alloc as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        ) as *mut Idx;
        if (new_elems == 0 as *mut libc::c_void as *mut Idx) as libc::c_int
            as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        (*dest).elems = new_elems;
        (*dest).alloc = new_alloc;
    }
    sbase = (*dest).nelem + (*src1).nelem + (*src2).nelem;
    i1 = (*src1).nelem - 1 as libc::c_int as libc::c_long;
    i2 = (*src2).nelem - 1 as libc::c_int as libc::c_long;
    id = (*dest).nelem - 1 as libc::c_int as libc::c_long;
    loop {
        if *((*src1).elems).offset(i1 as isize) == *((*src2).elems).offset(i2 as isize) {
            while id >= 0 as libc::c_int as libc::c_long
                && *((*dest).elems).offset(id as isize)
                    > *((*src1).elems).offset(i1 as isize)
            {
                id -= 1;
                id;
            }
            if id < 0 as libc::c_int as libc::c_long
                || *((*dest).elems).offset(id as isize)
                    != *((*src1).elems).offset(i1 as isize)
            {
                sbase -= 1;
                *((*dest).elems)
                    .offset(sbase as isize) = *((*src1).elems).offset(i1 as isize);
            }
            i1 -= 1;
            if i1 < 0 as libc::c_int as libc::c_long
                || {
                    i2 -= 1;
                    i2 < 0 as libc::c_int as libc::c_long
                }
            {
                break;
            }
        } else if *((*src1).elems).offset(i1 as isize)
            < *((*src2).elems).offset(i2 as isize)
        {
            i2 -= 1;
            if i2 < 0 as libc::c_int as libc::c_long {
                break;
            }
        } else {
            i1 -= 1;
            if i1 < 0 as libc::c_int as libc::c_long {
                break;
            }
        }
    }
    id = (*dest).nelem - 1 as libc::c_int as libc::c_long;
    is = (*dest).nelem + (*src1).nelem + (*src2).nelem
        - 1 as libc::c_int as libc::c_long;
    delta = is - sbase + 1 as libc::c_int as libc::c_long;
    (*dest).nelem += delta;
    if delta > 0 as libc::c_int as libc::c_long && id >= 0 as libc::c_int as libc::c_long
    {
        loop {
            if *((*dest).elems).offset(is as isize)
                > *((*dest).elems).offset(id as isize)
            {
                let fresh17 = is;
                is = is - 1;
                let fresh18 = delta;
                delta = delta - 1;
                *((*dest).elems)
                    .offset(
                        (id + fresh18) as isize,
                    ) = *((*dest).elems).offset(fresh17 as isize);
                if delta == 0 as libc::c_int as libc::c_long {
                    break;
                }
            } else {
                *((*dest).elems)
                    .offset(
                        (id + delta) as isize,
                    ) = *((*dest).elems).offset(id as isize);
                id -= 1;
                if id < 0 as libc::c_int as libc::c_long {
                    break;
                }
            }
        }
    }
    memcpy(
        (*dest).elems as *mut libc::c_void,
        ((*dest).elems).offset(sbase as isize) as *const libc::c_void,
        (delta as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
    );
    return _REG_NOERROR;
}
unsafe extern "C" fn re_node_set_init_union(
    mut dest: *mut re_node_set,
    mut src1: *const re_node_set,
    mut src2: *const re_node_set,
) -> reg_errcode_t {
    let mut i1: Idx = 0;
    let mut i2: Idx = 0;
    let mut id: Idx = 0;
    if !src1.is_null() && (*src1).nelem > 0 as libc::c_int as libc::c_long
        && !src2.is_null() && (*src2).nelem > 0 as libc::c_int as libc::c_long
    {
        (*dest).alloc = (*src1).nelem + (*src2).nelem;
        (*dest)
            .elems = malloc(
            ((*dest).alloc as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        ) as *mut Idx;
        if ((*dest).elems == 0 as *mut libc::c_void as *mut Idx) as libc::c_int
            as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
    } else {
        if !src1.is_null() && (*src1).nelem > 0 as libc::c_int as libc::c_long {
            return re_node_set_init_copy(dest, src1)
        } else if !src2.is_null() && (*src2).nelem > 0 as libc::c_int as libc::c_long {
            return re_node_set_init_copy(dest, src2)
        } else {
            memset(
                dest as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<re_node_set>() as libc::c_ulong,
            );
        }
        return _REG_NOERROR;
    }
    id = 0 as libc::c_int as Idx;
    i2 = id;
    i1 = i2;
    while i1 < (*src1).nelem && i2 < (*src2).nelem {
        if *((*src1).elems).offset(i1 as isize) > *((*src2).elems).offset(i2 as isize) {
            let fresh19 = i2;
            i2 = i2 + 1;
            let fresh20 = id;
            id = id + 1;
            *((*dest).elems)
                .offset(fresh20 as isize) = *((*src2).elems).offset(fresh19 as isize);
        } else {
            if *((*src1).elems).offset(i1 as isize)
                == *((*src2).elems).offset(i2 as isize)
            {
                i2 += 1;
                i2;
            }
            let fresh21 = i1;
            i1 = i1 + 1;
            let fresh22 = id;
            id = id + 1;
            *((*dest).elems)
                .offset(fresh22 as isize) = *((*src1).elems).offset(fresh21 as isize);
        }
    }
    if i1 < (*src1).nelem {
        memcpy(
            ((*dest).elems).offset(id as isize) as *mut libc::c_void,
            ((*src1).elems).offset(i1 as isize) as *const libc::c_void,
            (((*src1).nelem - i1) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        );
        id += (*src1).nelem - i1;
    } else if i2 < (*src2).nelem {
        memcpy(
            ((*dest).elems).offset(id as isize) as *mut libc::c_void,
            ((*src2).elems).offset(i2 as isize) as *const libc::c_void,
            (((*src2).nelem - i2) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        );
        id += (*src2).nelem - i2;
    }
    (*dest).nelem = id;
    return _REG_NOERROR;
}
unsafe extern "C" fn re_node_set_merge(
    mut dest: *mut re_node_set,
    mut src: *const re_node_set,
) -> reg_errcode_t {
    let mut is: Idx = 0;
    let mut id: Idx = 0;
    let mut sbase: Idx = 0;
    let mut delta: Idx = 0;
    if src.is_null() || (*src).nelem == 0 as libc::c_int as libc::c_long {
        return _REG_NOERROR;
    }
    if (*dest).alloc < 2 as libc::c_int as libc::c_long * (*src).nelem + (*dest).nelem {
        let mut new_alloc: Idx = 2 as libc::c_int as libc::c_long
            * ((*src).nelem + (*dest).alloc);
        let mut new_buffer: *mut Idx = realloc(
            (*dest).elems as *mut libc::c_void,
            (new_alloc as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        ) as *mut Idx;
        if (new_buffer == 0 as *mut libc::c_void as *mut Idx) as libc::c_int
            as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        (*dest).elems = new_buffer;
        (*dest).alloc = new_alloc;
    }
    if ((*dest).nelem == 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        if !((*dest).elems).is_null() {} else {
            unreachable!();
        };
        (*dest).nelem = (*src).nelem;
        memcpy(
            (*dest).elems as *mut libc::c_void,
            (*src).elems as *const libc::c_void,
            ((*src).nelem as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        );
        return _REG_NOERROR;
    }
    sbase = (*dest).nelem + 2 as libc::c_int as libc::c_long * (*src).nelem;
    is = (*src).nelem - 1 as libc::c_int as libc::c_long;
    id = (*dest).nelem - 1 as libc::c_int as libc::c_long;
    while is >= 0 as libc::c_int as libc::c_long
        && id >= 0 as libc::c_int as libc::c_long
    {
        if *((*dest).elems).offset(id as isize) == *((*src).elems).offset(is as isize) {
            is -= 1;
            is;
            id -= 1;
            id;
        } else if *((*dest).elems).offset(id as isize)
            < *((*src).elems).offset(is as isize)
        {
            let fresh23 = is;
            is = is - 1;
            sbase -= 1;
            *((*dest).elems)
                .offset(sbase as isize) = *((*src).elems).offset(fresh23 as isize);
        } else {
            id -= 1;
            id;
        }
    }
    if is >= 0 as libc::c_int as libc::c_long {
        sbase -= is + 1 as libc::c_int as libc::c_long;
        memcpy(
            ((*dest).elems).offset(sbase as isize) as *mut libc::c_void,
            (*src).elems as *const libc::c_void,
            ((is + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        );
    }
    id = (*dest).nelem - 1 as libc::c_int as libc::c_long;
    is = (*dest).nelem + 2 as libc::c_int as libc::c_long * (*src).nelem
        - 1 as libc::c_int as libc::c_long;
    delta = is - sbase + 1 as libc::c_int as libc::c_long;
    if delta == 0 as libc::c_int as libc::c_long {
        return _REG_NOERROR;
    }
    (*dest).nelem += delta;
    loop {
        if *((*dest).elems).offset(is as isize) > *((*dest).elems).offset(id as isize) {
            let fresh24 = is;
            is = is - 1;
            let fresh25 = delta;
            delta = delta - 1;
            *((*dest).elems)
                .offset(
                    (id + fresh25) as isize,
                ) = *((*dest).elems).offset(fresh24 as isize);
            if delta == 0 as libc::c_int as libc::c_long {
                break;
            }
        } else {
            *((*dest).elems)
                .offset((id + delta) as isize) = *((*dest).elems).offset(id as isize);
            id -= 1;
            if !(id < 0 as libc::c_int as libc::c_long) {
                continue;
            }
            memcpy(
                (*dest).elems as *mut libc::c_void,
                ((*dest).elems).offset(sbase as isize) as *const libc::c_void,
                (delta as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
            );
            break;
        }
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn re_node_set_insert(
    mut set: *mut re_node_set,
    mut elem: Idx,
) -> bool {
    let mut idx: Idx = 0;
    if (*set).alloc == 0 as libc::c_int as libc::c_long {
        return (re_node_set_init_1(set, elem) as libc::c_int
            == _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long != 0;
    }
    if (*set).nelem == 0 as libc::c_int as libc::c_long {
        if !((*set).elems).is_null() {} else {
            unreachable!();
        };
        *((*set).elems).offset(0 as libc::c_int as isize) = elem;
        (*set).nelem += 1;
        (*set).nelem;
        return 1 as libc::c_int != 0;
    }
    if (*set).alloc == (*set).nelem {
        let mut new_elems: *mut Idx = 0 as *mut Idx;
        (*set).alloc = (*set).alloc * 2 as libc::c_int as libc::c_long;
        new_elems = realloc(
            (*set).elems as *mut libc::c_void,
            ((*set).alloc as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        ) as *mut Idx;
        if (new_elems == 0 as *mut libc::c_void as *mut Idx) as libc::c_int
            as libc::c_long != 0
        {
            return 0 as libc::c_int != 0;
        }
        (*set).elems = new_elems;
    }
    if elem < *((*set).elems).offset(0 as libc::c_int as isize) {
        idx = (*set).nelem;
        while idx > 0 as libc::c_int as libc::c_long {
            *((*set).elems)
                .offset(
                    idx as isize,
                ) = *((*set).elems)
                .offset((idx - 1 as libc::c_int as libc::c_long) as isize);
            idx -= 1;
            idx;
        }
    } else {
        idx = (*set).nelem;
        while *((*set).elems).offset((idx - 1 as libc::c_int as libc::c_long) as isize)
            > elem
        {
            *((*set).elems)
                .offset(
                    idx as isize,
                ) = *((*set).elems)
                .offset((idx - 1 as libc::c_int as libc::c_long) as isize);
            idx -= 1;
            idx;
        }
        if *((*set).elems).offset((idx - 1 as libc::c_int as libc::c_long) as isize)
            < elem
        {} else {
            unreachable!();
        };
    }
    *((*set).elems).offset(idx as isize) = elem;
    (*set).nelem += 1;
    (*set).nelem;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn re_node_set_insert_last(
    mut set: *mut re_node_set,
    mut elem: Idx,
) -> bool {
    if (*set).alloc == (*set).nelem {
        let mut new_elems: *mut Idx = 0 as *mut Idx;
        (*set)
            .alloc = ((*set).alloc + 1 as libc::c_int as libc::c_long)
            * 2 as libc::c_int as libc::c_long;
        new_elems = realloc(
            (*set).elems as *mut libc::c_void,
            ((*set).alloc as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        ) as *mut Idx;
        if (new_elems == 0 as *mut libc::c_void as *mut Idx) as libc::c_int
            as libc::c_long != 0
        {
            return 0 as libc::c_int != 0;
        }
        (*set).elems = new_elems;
    }
    let fresh26 = (*set).nelem;
    (*set).nelem = (*set).nelem + 1;
    *((*set).elems).offset(fresh26 as isize) = elem;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn re_node_set_compare(
    mut set1: *const re_node_set,
    mut set2: *const re_node_set,
) -> bool {
    let mut i: Idx = 0;
    if set1.is_null() || set2.is_null() || (*set1).nelem != (*set2).nelem {
        return 0 as libc::c_int != 0;
    }
    i = (*set1).nelem;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        if *((*set1).elems).offset(i as isize) != *((*set2).elems).offset(i as isize) {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn re_node_set_contains(
    mut set: *const re_node_set,
    mut elem: Idx,
) -> Idx {
    let mut idx: __re_size_t = 0;
    let mut right: __re_size_t = 0;
    let mut mid: __re_size_t = 0;
    if (*set).nelem <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as Idx;
    }
    idx = 0 as libc::c_int as __re_size_t;
    right = ((*set).nelem - 1 as libc::c_int as libc::c_long) as __re_size_t;
    while idx < right {
        mid = idx.wrapping_add(right).wrapping_div(2 as libc::c_int as libc::c_ulong);
        if *((*set).elems).offset(mid as isize) < elem {
            idx = mid.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            right = mid;
        }
    }
    return (if *((*set).elems).offset(idx as isize) == elem {
        idx.wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as Idx;
}
unsafe extern "C" fn re_node_set_remove_at(mut set: *mut re_node_set, mut idx: Idx) {
    if idx < 0 as libc::c_int as libc::c_long || idx >= (*set).nelem {
        return;
    }
    (*set).nelem -= 1;
    (*set).nelem;
    while idx < (*set).nelem {
        *((*set).elems)
            .offset(
                idx as isize,
            ) = *((*set).elems)
            .offset((idx + 1 as libc::c_int as libc::c_long) as isize);
        idx += 1;
        idx;
    }
}
unsafe extern "C" fn re_dfa_add_node(
    mut dfa: *mut re_dfa_t,
    mut token: re_token_t,
) -> Idx {
    if ((*dfa).nodes_len >= (*dfa).nodes_alloc) as libc::c_int as libc::c_long != 0 {
        let mut new_nodes_alloc: size_t = ((*dfa).nodes_alloc)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong);
        let mut new_nexts: *mut Idx = 0 as *mut Idx;
        let mut new_indices: *mut Idx = 0 as *mut Idx;
        let mut new_edests: *mut re_node_set = 0 as *mut re_node_set;
        let mut new_eclosures: *mut re_node_set = 0 as *mut re_node_set;
        let mut new_nodes: *mut re_token_t = 0 as *mut re_token_t;
        let max_object_size: size_t = if (::core::mem::size_of::<re_token_t>()
            as libc::c_ulong)
            < (if (::core::mem::size_of::<re_node_set>() as libc::c_ulong)
                < ::core::mem::size_of::<Idx>() as libc::c_ulong
            {
                ::core::mem::size_of::<Idx>() as libc::c_ulong
            } else {
                ::core::mem::size_of::<re_node_set>() as libc::c_ulong
            })
        {
            if (::core::mem::size_of::<re_node_set>() as libc::c_ulong)
                < ::core::mem::size_of::<Idx>() as libc::c_ulong
            {
                ::core::mem::size_of::<Idx>() as libc::c_ulong
            } else {
                ::core::mem::size_of::<re_node_set>() as libc::c_ulong
            }
        } else {
            ::core::mem::size_of::<re_token_t>() as libc::c_ulong
        };
        if ((if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < (18446744073709551615 as libc::c_ulong).wrapping_div(max_object_size)
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            (18446744073709551615 as libc::c_ulong).wrapping_div(max_object_size)
        }) < new_nodes_alloc) as libc::c_int as libc::c_long != 0
        {
            return -(1 as libc::c_int) as Idx;
        }
        new_nodes = realloc(
            (*dfa).nodes as *mut libc::c_void,
            new_nodes_alloc
                .wrapping_mul(::core::mem::size_of::<re_token_t>() as libc::c_ulong),
        ) as *mut re_token_t;
        if (new_nodes == 0 as *mut libc::c_void as *mut re_token_t) as libc::c_int
            as libc::c_long != 0
        {
            return -(1 as libc::c_int) as Idx;
        }
        (*dfa).nodes = new_nodes;
        (*dfa).nodes_alloc = new_nodes_alloc;
        new_nexts = realloc(
            (*dfa).nexts as *mut libc::c_void,
            new_nodes_alloc.wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        ) as *mut Idx;
        if !new_nexts.is_null() {
            (*dfa).nexts = new_nexts;
        }
        new_indices = realloc(
            (*dfa).org_indices as *mut libc::c_void,
            new_nodes_alloc.wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
        ) as *mut Idx;
        if !new_indices.is_null() {
            (*dfa).org_indices = new_indices;
        }
        new_edests = realloc(
            (*dfa).edests as *mut libc::c_void,
            new_nodes_alloc
                .wrapping_mul(::core::mem::size_of::<re_node_set>() as libc::c_ulong),
        ) as *mut re_node_set;
        if !new_edests.is_null() {
            (*dfa).edests = new_edests;
        }
        new_eclosures = realloc(
            (*dfa).eclosures as *mut libc::c_void,
            new_nodes_alloc
                .wrapping_mul(::core::mem::size_of::<re_node_set>() as libc::c_ulong),
        ) as *mut re_node_set;
        if !new_eclosures.is_null() {
            (*dfa).eclosures = new_eclosures;
        }
        if (new_nexts.is_null() || new_indices.is_null() || new_edests.is_null()
            || new_eclosures.is_null()) as libc::c_int as libc::c_long != 0
        {
            return -(1 as libc::c_int) as Idx;
        }
    }
    *((*dfa).nodes).offset((*dfa).nodes_len as isize) = token;
    let ref mut fresh27 = *((*dfa).nodes).offset((*dfa).nodes_len as isize);
    (*fresh27).set_constraint(0 as libc::c_int as libc::c_uint);
    let ref mut fresh28 = *((*dfa).nodes).offset((*dfa).nodes_len as isize);
    (*fresh28)
        .set_accept_mb(
            (token.type_0() as libc::c_int == OP_PERIOD as libc::c_int
                && (*dfa).mb_cur_max > 1 as libc::c_int
                || token.type_0() as libc::c_int == COMPLEX_BRACKET as libc::c_int)
                as libc::c_int as libc::c_uint,
        );
    *((*dfa).nexts).offset((*dfa).nodes_len as isize) = -(1 as libc::c_int) as Idx;
    memset(
        ((*dfa).edests).offset((*dfa).nodes_len as isize) as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<re_node_set>() as libc::c_ulong,
    );
    memset(
        ((*dfa).eclosures).offset((*dfa).nodes_len as isize) as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<re_node_set>() as libc::c_ulong,
    );
    let fresh29 = (*dfa).nodes_len;
    (*dfa).nodes_len = ((*dfa).nodes_len).wrapping_add(1);
    return fresh29 as Idx;
}
unsafe extern "C" fn calc_state_hash(
    mut nodes: *const re_node_set,
    mut context: libc::c_uint,
) -> re_hashval_t {
    let mut hash: re_hashval_t = ((*nodes).nelem + context as libc::c_long)
        as re_hashval_t;
    let mut i: Idx = 0;
    i = 0 as libc::c_int as Idx;
    while i < (*nodes).nelem {
        hash = (hash as libc::c_ulong)
            .wrapping_add(*((*nodes).elems).offset(i as isize) as libc::c_ulong)
            as re_hashval_t as re_hashval_t;
        i += 1;
        i;
    }
    return hash;
}
unsafe extern "C" fn re_acquire_state(
    mut err: *mut reg_errcode_t,
    mut dfa: *const re_dfa_t,
    mut nodes: *const re_node_set,
) -> *mut re_dfastate_t {
    let mut hash: re_hashval_t = 0;
    let mut new_state: *mut re_dfastate_t = 0 as *mut re_dfastate_t;
    let mut spot: *mut re_state_table_entry = 0 as *mut re_state_table_entry;
    let mut i: Idx = 0;
    if ((*nodes).nelem == 0 as libc::c_int as libc::c_long) as libc::c_int
        as libc::c_long != 0
    {
        *err = _REG_NOERROR;
        return 0 as *mut re_dfastate_t;
    }
    hash = calc_state_hash(nodes, 0 as libc::c_int as libc::c_uint);
    spot = ((*dfa).state_table).offset((hash & (*dfa).state_hash_mask) as isize);
    i = 0 as libc::c_int as Idx;
    while i < (*spot).num {
        let mut state: *mut re_dfastate_t = *((*spot).array).offset(i as isize);
        if !(hash != (*state).hash) {
            if re_node_set_compare(&mut (*state).nodes, nodes) {
                return state;
            }
        }
        i += 1;
        i;
    }
    new_state = create_ci_newstate(dfa, nodes, hash);
    if (new_state == 0 as *mut libc::c_void as *mut re_dfastate_t) as libc::c_int
        as libc::c_long != 0
    {
        *err = _REG_ESPACE;
    }
    return new_state;
}
unsafe extern "C" fn re_acquire_state_context(
    mut err: *mut reg_errcode_t,
    mut dfa: *const re_dfa_t,
    mut nodes: *const re_node_set,
    mut context: libc::c_uint,
) -> *mut re_dfastate_t {
    let mut hash: re_hashval_t = 0;
    let mut new_state: *mut re_dfastate_t = 0 as *mut re_dfastate_t;
    let mut spot: *mut re_state_table_entry = 0 as *mut re_state_table_entry;
    let mut i: Idx = 0;
    if (*nodes).nelem == 0 as libc::c_int as libc::c_long {
        *err = _REG_NOERROR;
        return 0 as *mut re_dfastate_t;
    }
    hash = calc_state_hash(nodes, context);
    spot = ((*dfa).state_table).offset((hash & (*dfa).state_hash_mask) as isize);
    i = 0 as libc::c_int as Idx;
    while i < (*spot).num {
        let mut state: *mut re_dfastate_t = *((*spot).array).offset(i as isize);
        if (*state).hash == hash && (*state).context() == context
            && re_node_set_compare((*state).entrance_nodes, nodes) as libc::c_int != 0
        {
            return state;
        }
        i += 1;
        i;
    }
    new_state = create_cd_newstate(dfa, nodes, context, hash);
    if (new_state == 0 as *mut libc::c_void as *mut re_dfastate_t) as libc::c_int
        as libc::c_long != 0
    {
        *err = _REG_ESPACE;
    }
    return new_state;
}
unsafe extern "C" fn register_state(
    mut dfa: *const re_dfa_t,
    mut newstate: *mut re_dfastate_t,
    mut hash: re_hashval_t,
) -> reg_errcode_t {
    let mut spot: *mut re_state_table_entry = 0 as *mut re_state_table_entry;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut i: Idx = 0;
    (*newstate).hash = hash;
    err = re_node_set_alloc(&mut (*newstate).non_eps_nodes, (*newstate).nodes.nelem);
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return _REG_ESPACE;
    }
    i = 0 as libc::c_int as Idx;
    while i < (*newstate).nodes.nelem {
        let mut elem: Idx = *((*newstate).nodes.elems).offset(i as isize);
        if (*((*dfa).nodes).offset(elem as isize)).type_0() as libc::c_int
            & 8 as libc::c_int == 0
        {
            if !re_node_set_insert_last(&mut (*newstate).non_eps_nodes, elem) {
                return _REG_ESPACE;
            }
        }
        i += 1;
        i;
    }
    spot = ((*dfa).state_table).offset((hash & (*dfa).state_hash_mask) as isize);
    if ((*spot).alloc <= (*spot).num) as libc::c_int as libc::c_long != 0 {
        let mut new_alloc: Idx = 2 as libc::c_int as libc::c_long * (*spot).num
            + 2 as libc::c_int as libc::c_long;
        let mut new_array: *mut *mut re_dfastate_t = realloc(
            (*spot).array as *mut libc::c_void,
            (new_alloc as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong,
                ),
        ) as *mut *mut re_dfastate_t;
        if (new_array == 0 as *mut libc::c_void as *mut *mut re_dfastate_t)
            as libc::c_int as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        (*spot).array = new_array;
        (*spot).alloc = new_alloc;
    }
    let fresh30 = (*spot).num;
    (*spot).num = (*spot).num + 1;
    let ref mut fresh31 = *((*spot).array).offset(fresh30 as isize);
    *fresh31 = newstate;
    return _REG_NOERROR;
}
unsafe extern "C" fn free_state(mut state: *mut re_dfastate_t) {
    rpl_free((*state).non_eps_nodes.elems as *mut libc::c_void);
    rpl_free((*state).inveclosure.elems as *mut libc::c_void);
    if (*state).entrance_nodes != &mut (*state).nodes as *mut re_node_set {
        rpl_free((*(*state).entrance_nodes).elems as *mut libc::c_void);
        rpl_free((*state).entrance_nodes as *mut libc::c_void);
    }
    rpl_free((*state).nodes.elems as *mut libc::c_void);
    rpl_free((*state).word_trtable as *mut libc::c_void);
    rpl_free((*state).trtable as *mut libc::c_void);
    rpl_free(state as *mut libc::c_void);
}
unsafe extern "C" fn create_ci_newstate(
    mut dfa: *const re_dfa_t,
    mut nodes: *const re_node_set,
    mut hash: re_hashval_t,
) -> *mut re_dfastate_t {
    let mut i: Idx = 0;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut newstate: *mut re_dfastate_t = 0 as *mut re_dfastate_t;
    newstate = calloc(
        ::core::mem::size_of::<re_dfastate_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut re_dfastate_t;
    if (newstate == 0 as *mut libc::c_void as *mut re_dfastate_t) as libc::c_int
        as libc::c_long != 0
    {
        return 0 as *mut re_dfastate_t;
    }
    err = re_node_set_init_copy(&mut (*newstate).nodes, nodes);
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        rpl_free(newstate as *mut libc::c_void);
        return 0 as *mut re_dfastate_t;
    }
    (*newstate).entrance_nodes = &mut (*newstate).nodes;
    i = 0 as libc::c_int as Idx;
    while i < (*nodes).nelem {
        let mut node: *mut re_token_t = ((*dfa).nodes)
            .offset(*((*nodes).elems).offset(i as isize) as isize);
        let mut type_0: re_token_type_t = (*node).type_0();
        if !(type_0 as libc::c_uint == CHARACTER as libc::c_int as libc::c_uint
            && (*node).constraint() == 0)
        {
            (*newstate)
                .set_accept_mb(
                    (*newstate).accept_mb()
                        | (*node).accept_mb() as libc::c_int as libc::c_uint,
                );
            if type_0 as libc::c_uint == END_OF_RE as libc::c_int as libc::c_uint {
                (*newstate).set_halt(1 as libc::c_int as libc::c_uint);
            } else if type_0 as libc::c_uint
                == OP_BACK_REF as libc::c_int as libc::c_uint
            {
                (*newstate).set_has_backref(1 as libc::c_int as libc::c_uint);
            } else if type_0 as libc::c_uint == ANCHOR as libc::c_int as libc::c_uint
                || (*node).constraint() as libc::c_int != 0
            {
                (*newstate).set_has_constraint(1 as libc::c_int as libc::c_uint);
            }
        }
        i += 1;
        i;
    }
    err = register_state(dfa, newstate, hash);
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        free_state(newstate);
        newstate = 0 as *mut re_dfastate_t;
    }
    return newstate;
}
unsafe extern "C" fn create_cd_newstate(
    mut dfa: *const re_dfa_t,
    mut nodes: *const re_node_set,
    mut context: libc::c_uint,
    mut hash: re_hashval_t,
) -> *mut re_dfastate_t {
    let mut i: Idx = 0;
    let mut nctx_nodes: Idx = 0 as libc::c_int as Idx;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut newstate: *mut re_dfastate_t = 0 as *mut re_dfastate_t;
    newstate = calloc(
        ::core::mem::size_of::<re_dfastate_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut re_dfastate_t;
    if (newstate == 0 as *mut libc::c_void as *mut re_dfastate_t) as libc::c_int
        as libc::c_long != 0
    {
        return 0 as *mut re_dfastate_t;
    }
    err = re_node_set_init_copy(&mut (*newstate).nodes, nodes);
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        rpl_free(newstate as *mut libc::c_void);
        return 0 as *mut re_dfastate_t;
    }
    (*newstate).set_context(context);
    (*newstate).entrance_nodes = &mut (*newstate).nodes;
    i = 0 as libc::c_int as Idx;
    while i < (*nodes).nelem {
        let mut node: *mut re_token_t = ((*dfa).nodes)
            .offset(*((*nodes).elems).offset(i as isize) as isize);
        let mut type_0: re_token_type_t = (*node).type_0();
        let mut constraint: libc::c_uint = (*node).constraint();
        if !(type_0 as libc::c_uint == CHARACTER as libc::c_int as libc::c_uint
            && constraint == 0)
        {
            (*newstate)
                .set_accept_mb(
                    (*newstate).accept_mb()
                        | (*node).accept_mb() as libc::c_int as libc::c_uint,
                );
            if type_0 as libc::c_uint == END_OF_RE as libc::c_int as libc::c_uint {
                (*newstate).set_halt(1 as libc::c_int as libc::c_uint);
            } else if type_0 as libc::c_uint
                == OP_BACK_REF as libc::c_int as libc::c_uint
            {
                (*newstate).set_has_backref(1 as libc::c_int as libc::c_uint);
            }
            if constraint != 0 {
                if (*newstate).entrance_nodes
                    == &mut (*newstate).nodes as *mut re_node_set
                {
                    let mut entrance_nodes: *mut re_node_set = malloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<re_node_set>() as libc::c_ulong,
                            ),
                    ) as *mut re_node_set;
                    if (entrance_nodes == 0 as *mut libc::c_void as *mut re_node_set)
                        as libc::c_int as libc::c_long != 0
                    {
                        free_state(newstate);
                        return 0 as *mut re_dfastate_t;
                    }
                    (*newstate).entrance_nodes = entrance_nodes;
                    if re_node_set_init_copy((*newstate).entrance_nodes, nodes)
                        as libc::c_int != _REG_NOERROR as libc::c_int
                    {
                        free_state(newstate);
                        return 0 as *mut re_dfastate_t;
                    }
                    nctx_nodes = 0 as libc::c_int as Idx;
                    (*newstate).set_has_constraint(1 as libc::c_int as libc::c_uint);
                }
                if constraint & 0x1 as libc::c_int as libc::c_uint != 0
                    && context & 1 as libc::c_int as libc::c_uint == 0
                    || constraint & 0x2 as libc::c_int as libc::c_uint != 0
                        && context & 1 as libc::c_int as libc::c_uint != 0
                    || constraint & 0x10 as libc::c_int as libc::c_uint != 0
                        && context
                            & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                            == 0
                    || constraint & 0x40 as libc::c_int as libc::c_uint != 0
                        && context
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) as libc::c_uint == 0
                {
                    re_node_set_remove_at(&mut (*newstate).nodes, i - nctx_nodes);
                    nctx_nodes += 1;
                    nctx_nodes;
                }
            }
        }
        i += 1;
        i;
    }
    err = register_state(dfa, newstate, hash);
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        free_state(newstate);
        newstate = 0 as *mut re_dfastate_t;
    }
    return newstate;
}
static mut __re_error_msgid: [libc::c_char; 382] = unsafe {
    *::core::mem::transmute::<
        &[u8; 382],
        &[libc::c_char; 382],
    >(
        b"Success\0No match\0Invalid regular expression\0Invalid collation character\0Invalid character class name\0Trailing backslash\0Invalid back reference\0Unmatched [, [^, [:, [., or [=\0Unmatched ( or \\(\0Unmatched \\{\0Invalid content of \\{\\}\0Invalid range end\0Memory exhausted\0Invalid preceding regular expression\0Premature end of regular expression\0Regular expression too big\0Unmatched ) or \\)\0",
    )
};
static mut __re_error_msgid_idx: [size_t; 17] = [0; 17];
#[no_mangle]
pub unsafe extern "C" fn rpl_re_compile_pattern(
    mut pattern: *const libc::c_char,
    mut length: size_t,
    mut bufp: *mut re_pattern_buffer,
) -> *const libc::c_char {
    let mut ret: reg_errcode_t = _REG_NOERROR;
    (*bufp)
        .set_no_sub(
            (rpl_re_syntax_options
                & (((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int != 0) as libc::c_int as libc::c_uint,
        );
    (*bufp).set_newline_anchor(1 as libc::c_int as libc::c_uint);
    ret = re_compile_internal(bufp, pattern, length, rpl_re_syntax_options);
    if ret as u64 == 0 {
        return 0 as *const libc::c_char;
    }
    return dcgettext(
        0 as *const libc::c_char,
        __re_error_msgid
            .as_ptr()
            .offset(__re_error_msgid_idx[ret as libc::c_int as usize] as isize),
        5 as libc::c_int,
    );
}
#[no_mangle]
pub static mut rpl_re_syntax_options: reg_syntax_t = 0;
#[no_mangle]
pub unsafe extern "C" fn rpl_re_set_syntax(mut syntax: reg_syntax_t) -> reg_syntax_t {
    let mut ret: reg_syntax_t = rpl_re_syntax_options;
    rpl_re_syntax_options = syntax;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_re_compile_fastmap(
    mut bufp: *mut re_pattern_buffer,
) -> libc::c_int {
    let mut dfa: *mut re_dfa_t = (*bufp).buffer;
    let mut fastmap: *mut libc::c_char = (*bufp).fastmap;
    memset(
        fastmap as *mut libc::c_void,
        '\0' as i32,
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int) as libc::c_ulong,
            ),
    );
    re_compile_fastmap_iter(bufp, (*dfa).init_state, fastmap);
    if (*dfa).init_state != (*dfa).init_state_word {
        re_compile_fastmap_iter(bufp, (*dfa).init_state_word, fastmap);
    }
    if (*dfa).init_state != (*dfa).init_state_nl {
        re_compile_fastmap_iter(bufp, (*dfa).init_state_nl, fastmap);
    }
    if (*dfa).init_state != (*dfa).init_state_begbuf {
        re_compile_fastmap_iter(bufp, (*dfa).init_state_begbuf, fastmap);
    }
    (*bufp).set_fastmap_accurate(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn re_set_fastmap(
    mut fastmap: *mut libc::c_char,
    mut icase: bool,
    mut ch: libc::c_int,
) {
    *fastmap.offset(ch as isize) = 1 as libc::c_int as libc::c_char;
    if icase {
        *fastmap
            .offset(
                ({
                    let mut __res: libc::c_int = 0;
                    if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = ch;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(ch);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc()).offset(ch as isize);
                    }
                    __res
                }) as isize,
            ) = 1 as libc::c_int as libc::c_char;
    }
}
unsafe extern "C" fn re_compile_fastmap_iter(
    mut bufp: *mut regex_t,
    mut init_state: *const re_dfastate_t,
    mut fastmap: *mut libc::c_char,
) {
    let mut dfa: *mut re_dfa_t = (*bufp).buffer;
    let mut node_cnt: Idx = 0;
    let mut icase: bool = (*dfa).mb_cur_max == 1 as libc::c_int
        && (*bufp).syntax
            & ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int != 0;
    node_cnt = 0 as libc::c_int as Idx;
    while node_cnt < (*init_state).nodes.nelem {
        let mut node: Idx = *((*init_state).nodes.elems).offset(node_cnt as isize);
        let mut type_0: re_token_type_t = (*((*dfa).nodes).offset(node as isize))
            .type_0();
        if type_0 as libc::c_uint == CHARACTER as libc::c_int as libc::c_uint {
            re_set_fastmap(
                fastmap,
                icase,
                (*((*dfa).nodes).offset(node as isize)).opr.c as libc::c_int,
            );
            if (*bufp).syntax
                & ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int != 0 && (*dfa).mb_cur_max > 1 as libc::c_int
            {
                let mut buf: [libc::c_uchar; 16] = [0; 16];
                let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut wc: wchar_t = 0;
                let mut state: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: C2RustUnnamed_0 { __wch: 0 },
                };
                p = buf.as_mut_ptr();
                let fresh32 = p;
                p = p.offset(1);
                *fresh32 = (*((*dfa).nodes).offset(node as isize)).opr.c;
                loop {
                    node += 1;
                    if !((node as libc::c_ulong) < (*dfa).nodes_len
                        && (*((*dfa).nodes).offset(node as isize)).type_0()
                            as libc::c_int == CHARACTER as libc::c_int
                        && (*((*dfa).nodes).offset(node as isize)).mb_partial()
                            as libc::c_int != 0)
                    {
                        break;
                    }
                    let fresh33 = p;
                    p = p.offset(1);
                    *fresh33 = (*((*dfa).nodes).offset(node as isize)).opr.c;
                }
                memset(
                    &mut state as *mut mbstate_t as *mut libc::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
                );
                if rpl_mbrtowc(
                    &mut wc,
                    buf.as_mut_ptr() as *const libc::c_char,
                    p.offset_from(buf.as_mut_ptr()) as libc::c_long as size_t,
                    &mut state,
                ) == p.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_ulong
                    && wcrtomb(
                        buf.as_mut_ptr() as *mut libc::c_char,
                        towlower(wc as wint_t) as wchar_t,
                        &mut state,
                    ) != -(1 as libc::c_int) as size_t
                {
                    re_set_fastmap(
                        fastmap,
                        0 as libc::c_int != 0,
                        buf[0 as libc::c_int as usize] as libc::c_int,
                    );
                }
            }
        } else if type_0 as libc::c_uint == SIMPLE_BRACKET as libc::c_int as libc::c_uint
        {
            let mut i: libc::c_int = 0;
            let mut ch: libc::c_int = 0;
            i = 0 as libc::c_int;
            ch = 0 as libc::c_int;
            while i
                < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                    / 64 as libc::c_int
            {
                let mut j: libc::c_int = 0;
                let mut w: bitset_word_t = *((*((*dfa).nodes).offset(node as isize))
                    .opr
                    .sbcset)
                    .offset(i as isize);
                j = 0 as libc::c_int;
                while j < 64 as libc::c_int {
                    if w & (1 as libc::c_int as bitset_word_t) << j != 0 {
                        re_set_fastmap(fastmap, icase, ch);
                    }
                    j += 1;
                    j;
                    ch += 1;
                    ch;
                }
                i += 1;
                i;
            }
        } else if type_0 as libc::c_uint
            == COMPLEX_BRACKET as libc::c_int as libc::c_uint
        {
            let mut cset: *mut re_charset_t = (*((*dfa).nodes).offset(node as isize))
                .opr
                .mbcset;
            let mut i_0: Idx = 0;
            if (*dfa).mb_cur_max > 1 as libc::c_int
                && ((*cset).nchar_classes != 0 || (*cset).non_match() as libc::c_int != 0
                    || (*cset).nranges != 0)
            {
                let mut c: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
                loop {
                    let mut mbs: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: C2RustUnnamed_0 { __wch: 0 },
                    };
                    memset(
                        &mut mbs as *mut mbstate_t as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
                    );
                    if rpl_mbrtowc(
                        0 as *mut wchar_t,
                        &mut c as *mut libc::c_uchar as *mut libc::c_char,
                        1 as libc::c_int as size_t,
                        &mut mbs,
                    ) == -(2 as libc::c_int) as size_t
                    {
                        re_set_fastmap(fastmap, 0 as libc::c_int != 0, c as libc::c_int);
                    }
                    c = c.wrapping_add(1);
                    if !(c as libc::c_int != 0 as libc::c_int) {
                        break;
                    }
                }
            } else {
                i_0 = 0 as libc::c_int as Idx;
                while i_0 < (*cset).nmbchars {
                    let mut buf_0: [libc::c_char; 256] = [0; 256];
                    let mut state_0: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: C2RustUnnamed_0 { __wch: 0 },
                    };
                    memset(
                        &mut state_0 as *mut mbstate_t as *mut libc::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
                    );
                    if wcrtomb(
                        buf_0.as_mut_ptr(),
                        *((*cset).mbchars).offset(i_0 as isize),
                        &mut state_0,
                    ) != -(1 as libc::c_int) as size_t
                    {
                        re_set_fastmap(
                            fastmap,
                            icase,
                            *(buf_0.as_mut_ptr() as *mut libc::c_uchar) as libc::c_int,
                        );
                    }
                    if (*bufp).syntax
                        & ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int != 0
                        && (*dfa).mb_cur_max > 1 as libc::c_int
                    {
                        if wcrtomb(
                            buf_0.as_mut_ptr(),
                            towlower(*((*cset).mbchars).offset(i_0 as isize) as wint_t)
                                as wchar_t,
                            &mut state_0,
                        ) != -(1 as libc::c_int) as size_t
                        {
                            re_set_fastmap(
                                fastmap,
                                0 as libc::c_int != 0,
                                *(buf_0.as_mut_ptr() as *mut libc::c_uchar) as libc::c_int,
                            );
                        }
                    }
                    i_0 += 1;
                    i_0;
                }
            }
        } else if type_0 as libc::c_uint == OP_PERIOD as libc::c_int as libc::c_uint
            || type_0 as libc::c_uint == OP_UTF8_PERIOD as libc::c_int as libc::c_uint
            || type_0 as libc::c_uint == END_OF_RE as libc::c_int as libc::c_uint
        {
            memset(
                fastmap as *mut libc::c_void,
                '\u{1}' as i32,
                (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(
                        (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                            + 1 as libc::c_int) as libc::c_ulong,
                    ),
            );
            if type_0 as libc::c_uint == END_OF_RE as libc::c_int as libc::c_uint {
                (*bufp).set_can_be_null(1 as libc::c_int as libc::c_uint);
            }
            return;
        }
        node_cnt += 1;
        node_cnt;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rpl_regcomp(
    mut preg: *mut regex_t,
    mut pattern: *const libc::c_char,
    mut cflags: libc::c_int,
) -> libc::c_int {
    let mut ret: reg_errcode_t = _REG_NOERROR;
    let mut syntax: reg_syntax_t = if cflags & 1 as libc::c_int != 0 {
        ((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int
            | (((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int
    } else {
        ((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
            | ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
    };
    (*preg).buffer = 0 as *mut re_dfa_t;
    (*preg).allocated = 0 as libc::c_int as __re_long_size_t;
    (*preg).used = 0 as libc::c_int as __re_long_size_t;
    (*preg)
        .fastmap = malloc(
        ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
            as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if ((*preg).fastmap == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
        as libc::c_long != 0
    {
        return _REG_ESPACE as libc::c_int;
    }
    syntax
        |= if cflags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
        } else {
            0 as libc::c_int as libc::c_ulong
        };
    if cflags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        syntax
            &= !(((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int);
        syntax
            |= ((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int;
        (*preg).set_newline_anchor(1 as libc::c_int as libc::c_uint);
    } else {
        (*preg).set_newline_anchor(0 as libc::c_int as libc::c_uint);
    }
    (*preg)
        .set_no_sub(
            (cflags & (1 as libc::c_int) << 3 as libc::c_int != 0) as libc::c_int
                as libc::c_uint,
        );
    (*preg).translate = 0 as *mut libc::c_uchar;
    ret = re_compile_internal(preg, pattern, strlen(pattern), syntax);
    if ret as libc::c_int == _REG_ERPAREN as libc::c_int {
        ret = _REG_EPAREN;
    }
    if (ret as libc::c_int == _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        rpl_re_compile_fastmap(preg);
    } else {
        rpl_free((*preg).fastmap as *mut libc::c_void);
        (*preg).fastmap = 0 as *mut libc::c_char;
    }
    return ret as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_regerror(
    mut errcode: libc::c_int,
    mut preg: *const regex_t,
    mut errbuf: *mut libc::c_char,
    mut errbuf_size: size_t,
) -> size_t {
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut msg_size: size_t = 0;
    let mut nerrcodes: libc::c_int = (::core::mem::size_of::<[size_t; 17]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong) as libc::c_int;
    if (errcode < 0 as libc::c_int || errcode >= nerrcodes) as libc::c_int
        as libc::c_long != 0
    {
        abort();
    }
    msg = dcgettext(
        0 as *const libc::c_char,
        __re_error_msgid
            .as_ptr()
            .offset(__re_error_msgid_idx[errcode as usize] as isize),
        5 as libc::c_int,
    );
    msg_size = (strlen(msg)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if (errbuf_size != 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long
        != 0
    {
        let mut cpy_size: size_t = msg_size;
        if (msg_size > errbuf_size) as libc::c_int as libc::c_long != 0 {
            cpy_size = errbuf_size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            *errbuf.offset(cpy_size as isize) = '\0' as i32 as libc::c_char;
        }
        memcpy(errbuf as *mut libc::c_void, msg as *const libc::c_void, cpy_size);
    }
    return msg_size;
}
static mut utf8_sb_map: bitset_t = [0; 4];
unsafe extern "C" fn free_dfa_content(mut dfa: *mut re_dfa_t) {
    let mut i: Idx = 0;
    let mut j: Idx = 0;
    if !((*dfa).nodes).is_null() {
        i = 0 as libc::c_int as Idx;
        while (i as libc::c_ulong) < (*dfa).nodes_len {
            free_token(((*dfa).nodes).offset(i as isize));
            i += 1;
            i;
        }
    }
    rpl_free((*dfa).nexts as *mut libc::c_void);
    i = 0 as libc::c_int as Idx;
    while (i as libc::c_ulong) < (*dfa).nodes_len {
        if !((*dfa).eclosures).is_null() {
            rpl_free(
                (*((*dfa).eclosures).offset(i as isize)).elems as *mut libc::c_void,
            );
        }
        if !((*dfa).inveclosures).is_null() {
            rpl_free(
                (*((*dfa).inveclosures).offset(i as isize)).elems as *mut libc::c_void,
            );
        }
        if !((*dfa).edests).is_null() {
            rpl_free((*((*dfa).edests).offset(i as isize)).elems as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    rpl_free((*dfa).edests as *mut libc::c_void);
    rpl_free((*dfa).eclosures as *mut libc::c_void);
    rpl_free((*dfa).inveclosures as *mut libc::c_void);
    rpl_free((*dfa).nodes as *mut libc::c_void);
    if !((*dfa).state_table).is_null() {
        i = 0 as libc::c_int as Idx;
        while i as libc::c_ulong <= (*dfa).state_hash_mask {
            let mut entry: *mut re_state_table_entry = ((*dfa).state_table)
                .offset(i as isize);
            j = 0 as libc::c_int as Idx;
            while j < (*entry).num {
                let mut state: *mut re_dfastate_t = *((*entry).array).offset(j as isize);
                free_state(state);
                j += 1;
                j;
            }
            rpl_free((*entry).array as *mut libc::c_void);
            i += 1;
            i;
        }
    }
    rpl_free((*dfa).state_table as *mut libc::c_void);
    if (*dfa).sb_char != utf8_sb_map.as_ptr() as re_bitset_ptr_t {
        rpl_free((*dfa).sb_char as *mut libc::c_void);
    }
    rpl_free((*dfa).subexp_map as *mut libc::c_void);
    rpl_free(dfa as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn rpl_regfree(mut preg: *mut regex_t) {
    let mut dfa: *mut re_dfa_t = (*preg).buffer;
    if (dfa != 0 as *mut libc::c_void as *mut re_dfa_t) as libc::c_int as libc::c_long
        != 0
    {
        free_dfa_content(dfa);
    }
    (*preg).buffer = 0 as *mut re_dfa_t;
    (*preg).allocated = 0 as libc::c_int as __re_long_size_t;
    rpl_free((*preg).fastmap as *mut libc::c_void);
    (*preg).fastmap = 0 as *mut libc::c_char;
    rpl_free((*preg).translate as *mut libc::c_void);
    (*preg).translate = 0 as *mut libc::c_uchar;
}
unsafe extern "C" fn re_compile_internal(
    mut preg: *mut regex_t,
    mut pattern: *const libc::c_char,
    mut length: size_t,
    mut syntax: reg_syntax_t,
) -> reg_errcode_t {
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut dfa: *mut re_dfa_t = 0 as *mut re_dfa_t;
    let mut regexp: re_string_t = re_string_t {
        raw_mbs: 0 as *const libc::c_uchar,
        mbs: 0 as *mut libc::c_uchar,
        wcs: 0 as *mut wint_t,
        offsets: 0 as *mut Idx,
        cur_state: mbstate_t {
            __count: 0,
            __value: C2RustUnnamed_0 { __wch: 0 },
        },
        raw_mbs_idx: 0,
        valid_len: 0,
        valid_raw_len: 0,
        bufs_len: 0,
        cur_idx: 0,
        raw_len: 0,
        len: 0,
        raw_stop: 0,
        stop: 0,
        tip_context: 0,
        trans: 0 as *mut libc::c_uchar,
        word_char: 0 as *const bitset_word_t,
        icase: 0,
        is_utf8: 0,
        map_notascii: 0,
        mbs_allocated: 0,
        offsets_needed: 0,
        newline_anchor: 0,
        word_ops_used: 0,
        mb_cur_max: 0,
    };
    (*preg).set_fastmap_accurate(0 as libc::c_int as libc::c_uint);
    (*preg).syntax = syntax;
    (*preg).set_not_eol(0 as libc::c_int as libc::c_uint);
    (*preg).set_not_bol((*preg).not_eol());
    (*preg).used = 0 as libc::c_int as __re_long_size_t;
    (*preg).re_nsub = 0 as libc::c_int as size_t;
    (*preg).set_can_be_null(0 as libc::c_int as libc::c_uint);
    (*preg).set_regs_allocated(0 as libc::c_int as libc::c_uint);
    dfa = (*preg).buffer;
    if ((*preg).allocated < ::core::mem::size_of::<re_dfa_t>() as libc::c_ulong)
        as libc::c_int as libc::c_long != 0
    {
        dfa = realloc(
            (*preg).buffer as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<re_dfa_t>() as libc::c_ulong),
        ) as *mut re_dfa_t;
        if dfa.is_null() {
            return _REG_ESPACE;
        }
        (*preg).allocated = ::core::mem::size_of::<re_dfa_t>() as libc::c_ulong;
        (*preg).buffer = dfa;
    }
    (*preg).used = ::core::mem::size_of::<re_dfa_t>() as libc::c_ulong;
    err = init_dfa(dfa, length);
    if (err as libc::c_int == _REG_NOERROR as libc::c_int
        && 0 as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        err = _REG_ESPACE;
    }
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        free_dfa_content(dfa);
        (*preg).buffer = 0 as *mut re_dfa_t;
        (*preg).allocated = 0 as libc::c_int as __re_long_size_t;
        return err;
    }
    err = re_string_construct(
        &mut regexp,
        pattern,
        length as Idx,
        (*preg).translate,
        syntax
            & ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int != 0 as libc::c_int as libc::c_ulong,
        dfa,
    );
    if !((err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
        as libc::c_long != 0)
    {
        (*preg).re_nsub = 0 as libc::c_int as size_t;
        (*dfa).str_tree = parse(&mut regexp, preg, syntax, &mut err);
        if !(((*dfa).str_tree == 0 as *mut libc::c_void as *mut bin_tree_t)
            as libc::c_int as libc::c_long != 0)
        {
            err = analyze(preg);
            if !((err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0)
            {
                if (*dfa).is_utf8() as libc::c_int != 0
                    && syntax
                        & ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int == 0
                    && ((*preg).translate).is_null()
                {
                    optimize_utf8(dfa);
                }
                err = create_initial_state(dfa);
                free_workarea_compile(preg);
                re_string_destruct(&mut regexp);
                if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    free_dfa_content(dfa);
                    (*preg).buffer = 0 as *mut re_dfa_t;
                    (*preg).allocated = 0 as libc::c_int as __re_long_size_t;
                }
                return err;
            }
        }
    }
    free_workarea_compile(preg);
    re_string_destruct(&mut regexp);
    free_dfa_content(dfa);
    (*preg).buffer = 0 as *mut re_dfa_t;
    (*preg).allocated = 0 as libc::c_int as __re_long_size_t;
    return err;
}
unsafe extern "C" fn init_dfa(
    mut dfa: *mut re_dfa_t,
    mut pat_len: size_t,
) -> reg_errcode_t {
    let mut table_size: __re_size_t = 0;
    let mut codeset_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut max_i18n_object_size: size_t = if (::core::mem::size_of::<wchar_t>()
        as libc::c_ulong) < ::core::mem::size_of::<wctype_t>() as libc::c_ulong
    {
        ::core::mem::size_of::<wctype_t>() as libc::c_ulong
    } else {
        ::core::mem::size_of::<wchar_t>() as libc::c_ulong
    };
    let mut max_object_size: size_t = if (::core::mem::size_of::<re_state_table_entry>()
        as libc::c_ulong)
        < (if (::core::mem::size_of::<re_token_t>() as libc::c_ulong)
            < (if (::core::mem::size_of::<re_node_set>() as libc::c_ulong)
                < (if (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
                    < max_i18n_object_size
                {
                    max_i18n_object_size
                } else {
                    ::core::mem::size_of::<regmatch_t>() as libc::c_ulong
                })
            {
                (if (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
                    < max_i18n_object_size
                {
                    max_i18n_object_size
                } else {
                    ::core::mem::size_of::<regmatch_t>() as libc::c_ulong
                })
            } else {
                ::core::mem::size_of::<re_node_set>() as libc::c_ulong
            })
        {
            (if (::core::mem::size_of::<re_node_set>() as libc::c_ulong)
                < (if (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
                    < max_i18n_object_size
                {
                    max_i18n_object_size
                } else {
                    ::core::mem::size_of::<regmatch_t>() as libc::c_ulong
                })
            {
                (if (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
                    < max_i18n_object_size
                {
                    max_i18n_object_size
                } else {
                    ::core::mem::size_of::<regmatch_t>() as libc::c_ulong
                })
            } else {
                ::core::mem::size_of::<re_node_set>() as libc::c_ulong
            })
        } else {
            ::core::mem::size_of::<re_token_t>() as libc::c_ulong
        })
    {
        if (::core::mem::size_of::<re_token_t>() as libc::c_ulong)
            < (if (::core::mem::size_of::<re_node_set>() as libc::c_ulong)
                < (if (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
                    < max_i18n_object_size
                {
                    max_i18n_object_size
                } else {
                    ::core::mem::size_of::<regmatch_t>() as libc::c_ulong
                })
            {
                (if (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
                    < max_i18n_object_size
                {
                    max_i18n_object_size
                } else {
                    ::core::mem::size_of::<regmatch_t>() as libc::c_ulong
                })
            } else {
                ::core::mem::size_of::<re_node_set>() as libc::c_ulong
            })
        {
            if (::core::mem::size_of::<re_node_set>() as libc::c_ulong)
                < (if (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
                    < max_i18n_object_size
                {
                    max_i18n_object_size
                } else {
                    ::core::mem::size_of::<regmatch_t>() as libc::c_ulong
                })
            {
                if (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
                    < max_i18n_object_size
                {
                    max_i18n_object_size
                } else {
                    ::core::mem::size_of::<regmatch_t>() as libc::c_ulong
                }
            } else {
                ::core::mem::size_of::<re_node_set>() as libc::c_ulong
            }
        } else {
            ::core::mem::size_of::<re_token_t>() as libc::c_ulong
        }
    } else {
        ::core::mem::size_of::<re_state_table_entry>() as libc::c_ulong
    };
    memset(
        dfa as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<re_dfa_t>() as libc::c_ulong,
    );
    (*dfa)
        .str_tree_storage_idx = (1024 as libc::c_int as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<bin_tree_t>() as libc::c_ulong)
        as libc::c_int;
    if ((if (9223372036854775807 as libc::c_long as libc::c_ulong)
        < (18446744073709551615 as libc::c_ulong).wrapping_div(max_object_size)
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        (18446744073709551615 as libc::c_ulong).wrapping_div(max_object_size)
    })
        .wrapping_div(2 as libc::c_int as libc::c_ulong) <= pat_len) as libc::c_int
        as libc::c_long != 0
    {
        return _REG_ESPACE;
    }
    (*dfa).nodes_alloc = pat_len.wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*dfa)
        .nodes = malloc(
        ((*dfa).nodes_alloc)
            .wrapping_mul(::core::mem::size_of::<re_token_t>() as libc::c_ulong),
    ) as *mut re_token_t;
    table_size = 1 as libc::c_int as __re_size_t;
    while !(table_size > pat_len) {
        table_size <<= 1 as libc::c_int;
    }
    (*dfa)
        .state_table = calloc(
        ::core::mem::size_of::<re_state_table_entry>() as libc::c_ulong,
        table_size,
    ) as *mut re_state_table_entry;
    (*dfa).state_hash_mask = table_size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*dfa).mb_cur_max = __ctype_get_mb_cur_max() as libc::c_int;
    codeset_name = nl_langinfo(CODESET as libc::c_int);
    if (*codeset_name.offset(0 as libc::c_int as isize) as libc::c_int == 'U' as i32
        || *codeset_name.offset(0 as libc::c_int as isize) as libc::c_int == 'u' as i32)
        && (*codeset_name.offset(1 as libc::c_int as isize) as libc::c_int == 'T' as i32
            || *codeset_name.offset(1 as libc::c_int as isize) as libc::c_int
                == 't' as i32)
        && (*codeset_name.offset(2 as libc::c_int as isize) as libc::c_int == 'F' as i32
            || *codeset_name.offset(2 as libc::c_int as isize) as libc::c_int
                == 'f' as i32)
        && strcmp(
            codeset_name
                .offset(3 as libc::c_int as isize)
                .offset(
                    (*codeset_name.offset(3 as libc::c_int as isize) as libc::c_int
                        == '-' as i32) as libc::c_int as isize,
                ),
            b"8\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        (*dfa).set_is_utf8(1 as libc::c_int as libc::c_uint);
    }
    (*dfa).set_map_notascii(0 as libc::c_int as libc::c_uint);
    if (*dfa).mb_cur_max > 1 as libc::c_int {
        if (*dfa).is_utf8() != 0 {
            (*dfa).sb_char = utf8_sb_map.as_ptr() as re_bitset_ptr_t;
        } else {
            let mut i: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            let mut ch: libc::c_int = 0;
            (*dfa)
                .sb_char = calloc(
                ::core::mem::size_of::<bitset_t>() as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
            ) as re_bitset_ptr_t;
            if ((*dfa).sb_char == 0 as *mut libc::c_void as re_bitset_ptr_t)
                as libc::c_int as libc::c_long != 0
            {
                return _REG_ESPACE;
            }
            i = 0 as libc::c_int;
            ch = 0 as libc::c_int;
            while i
                < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                    / 64 as libc::c_int
            {
                j = 0 as libc::c_int;
                while j < 64 as libc::c_int {
                    let mut wch: wint_t = btowc(ch);
                    if wch != 0xffffffff as libc::c_uint {
                        let ref mut fresh34 = *((*dfa).sb_char).offset(i as isize);
                        *fresh34 |= (1 as libc::c_int as bitset_word_t) << j;
                    }
                    if ch & !(0x7f as libc::c_int) == 0 as libc::c_int
                        && wch != ch as libc::c_uint
                    {
                        (*dfa).set_map_notascii(1 as libc::c_int as libc::c_uint);
                    }
                    j += 1;
                    j;
                    ch += 1;
                    ch;
                }
                i += 1;
                i;
            }
        }
    }
    if (((*dfa).nodes).is_null() || ((*dfa).state_table).is_null()) as libc::c_int
        as libc::c_long != 0
    {
        return _REG_ESPACE;
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn init_word_char(mut dfa: *mut re_dfa_t) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut ch: libc::c_int = 0 as libc::c_int;
    (*dfa).set_word_ops_used(1 as libc::c_int as libc::c_uint);
    if ((*dfa).map_notascii() as libc::c_int == 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        let mut bits0: bitset_word_t = 0 as libc::c_int as bitset_word_t;
        let mut bits1: bitset_word_t = 0x3ff0000 as libc::c_int as bitset_word_t;
        let mut bits2: bitset_word_t = 0x87fffffe as libc::c_uint as bitset_word_t;
        let mut bits3: bitset_word_t = 0x7fffffe as libc::c_int as bitset_word_t;
        if 64 as libc::c_int == 64 as libc::c_int {
            (*dfa)
                .word_char[0 as libc::c_int
                as usize] = (bits1 << 31 as libc::c_int) << 1 as libc::c_int | bits0;
            (*dfa)
                .word_char[1 as libc::c_int
                as usize] = (bits3 << 31 as libc::c_int) << 1 as libc::c_int | bits2;
            i = 2 as libc::c_int;
            current_block = 1841672684692190573;
        } else if 64 as libc::c_int == 32 as libc::c_int {
            (*dfa).word_char[0 as libc::c_int as usize] = bits0;
            (*dfa).word_char[1 as libc::c_int as usize] = bits1;
            (*dfa).word_char[2 as libc::c_int as usize] = bits2;
            (*dfa).word_char[3 as libc::c_int as usize] = bits3;
            i = 4 as libc::c_int;
            current_block = 1841672684692190573;
        } else {
            current_block = 8812046683468980733;
        }
        match current_block {
            8812046683468980733 => {}
            _ => {
                ch = 128 as libc::c_int;
                if (*dfa).is_utf8() as libc::c_long != 0 {
                    memset(
                        &mut *((*dfa).word_char).as_mut_ptr().offset(i as isize)
                            as *mut bitset_word_t as *mut libc::c_void,
                        '\0' as i32,
                        ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                            + 1 as libc::c_int - ch) / 8 as libc::c_int) as libc::c_ulong,
                    );
                    return;
                }
            }
        }
    }
    while i
        < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 64 as libc::c_int - 1 as libc::c_int) / 64 as libc::c_int
    {
        j = 0 as libc::c_int;
        while j < 64 as libc::c_int {
            if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                || ch == '_' as i32
            {
                (*dfa).word_char[i as usize] |= (1 as libc::c_int as bitset_word_t) << j;
            }
            j += 1;
            j;
            ch += 1;
            ch;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn free_workarea_compile(mut preg: *mut regex_t) {
    let mut dfa: *mut re_dfa_t = (*preg).buffer;
    let mut storage: *mut bin_tree_storage_t = 0 as *mut bin_tree_storage_t;
    let mut next: *mut bin_tree_storage_t = 0 as *mut bin_tree_storage_t;
    storage = (*dfa).str_tree_storage;
    while !storage.is_null() {
        next = (*storage).next;
        rpl_free(storage as *mut libc::c_void);
        storage = next;
    }
    (*dfa).str_tree_storage = 0 as *mut bin_tree_storage_t;
    (*dfa)
        .str_tree_storage_idx = (1024 as libc::c_int as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<bin_tree_t>() as libc::c_ulong)
        as libc::c_int;
    (*dfa).str_tree = 0 as *mut bin_tree_t;
    rpl_free((*dfa).org_indices as *mut libc::c_void);
    (*dfa).org_indices = 0 as *mut Idx;
}
unsafe extern "C" fn create_initial_state(mut dfa: *mut re_dfa_t) -> reg_errcode_t {
    let mut first: Idx = 0;
    let mut i: Idx = 0;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut init_nodes: re_node_set = re_node_set {
        alloc: 0,
        nelem: 0,
        elems: 0 as *mut Idx,
    };
    first = (*(*(*dfa).str_tree).first).node_idx;
    (*dfa).init_node = first;
    err = re_node_set_init_copy(
        &mut init_nodes,
        ((*dfa).eclosures).offset(first as isize),
    );
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return err;
    }
    if (*dfa).nbackref > 0 as libc::c_int as libc::c_long {
        i = 0 as libc::c_int as Idx;
        while i < init_nodes.nelem {
            let mut node_idx: Idx = *(init_nodes.elems).offset(i as isize);
            let mut type_0: re_token_type_t = (*((*dfa).nodes).offset(node_idx as isize))
                .type_0();
            let mut clexp_idx: Idx = 0;
            if !(type_0 as libc::c_uint != OP_BACK_REF as libc::c_int as libc::c_uint) {
                clexp_idx = 0 as libc::c_int as Idx;
                while clexp_idx < init_nodes.nelem {
                    let mut clexp_node: *mut re_token_t = 0 as *mut re_token_t;
                    clexp_node = ((*dfa).nodes)
                        .offset(*(init_nodes.elems).offset(clexp_idx as isize) as isize);
                    if (*clexp_node).type_0() as libc::c_int
                        == OP_CLOSE_SUBEXP as libc::c_int
                        && (*clexp_node).opr.idx
                            == (*((*dfa).nodes).offset(node_idx as isize)).opr.idx
                    {
                        break;
                    }
                    clexp_idx += 1;
                    clexp_idx;
                }
                if !(clexp_idx == init_nodes.nelem) {
                    if type_0 as libc::c_uint
                        == OP_BACK_REF as libc::c_int as libc::c_uint
                    {
                        let mut dest_idx: Idx = *((*((*dfa).edests)
                            .offset(node_idx as isize))
                            .elems)
                            .offset(0 as libc::c_int as isize);
                        if re_node_set_contains(&mut init_nodes, dest_idx) == 0 {
                            let mut merge_err: reg_errcode_t = re_node_set_merge(
                                &mut init_nodes,
                                ((*dfa).eclosures).offset(dest_idx as isize),
                            );
                            if merge_err as libc::c_int != _REG_NOERROR as libc::c_int {
                                return merge_err;
                            }
                            i = 0 as libc::c_int as Idx;
                        }
                    }
                }
            }
            i += 1;
            i;
        }
    }
    (*dfa)
        .init_state = re_acquire_state_context(
        &mut err,
        dfa,
        &mut init_nodes,
        0 as libc::c_int as libc::c_uint,
    );
    if ((*dfa).init_state == 0 as *mut libc::c_void as *mut re_dfastate_t) as libc::c_int
        as libc::c_long != 0
    {
        return err;
    }
    if (*(*dfa).init_state).has_constraint() != 0 {
        (*dfa)
            .init_state_word = re_acquire_state_context(
            &mut err,
            dfa,
            &mut init_nodes,
            1 as libc::c_int as libc::c_uint,
        );
        (*dfa)
            .init_state_nl = re_acquire_state_context(
            &mut err,
            dfa,
            &mut init_nodes,
            ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
        );
        (*dfa)
            .init_state_begbuf = re_acquire_state_context(
            &mut err,
            dfa,
            &mut init_nodes,
            ((1 as libc::c_int) << 1 as libc::c_int
                | ((1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                as libc::c_uint,
        );
        if (((*dfa).init_state_word).is_null() || ((*dfa).init_state_nl).is_null()
            || ((*dfa).init_state_begbuf).is_null()) as libc::c_int as libc::c_long != 0
        {
            return err;
        }
    } else {
        (*dfa).init_state_begbuf = (*dfa).init_state;
        (*dfa).init_state_nl = (*dfa).init_state_begbuf;
        (*dfa).init_state_word = (*dfa).init_state_nl;
    }
    rpl_free(init_nodes.elems as *mut libc::c_void);
    return _REG_NOERROR;
}
unsafe extern "C" fn optimize_utf8(mut dfa: *mut re_dfa_t) {
    let mut node: Idx = 0;
    let mut i: libc::c_int = 0;
    let mut mb_chars: bool = 0 as libc::c_int != 0;
    let mut has_period: bool = 0 as libc::c_int != 0;
    node = 0 as libc::c_int as Idx;
    while (node as libc::c_ulong) < (*dfa).nodes_len {
        match (*((*dfa).nodes).offset(node as isize)).type_0() as libc::c_int {
            1 => {
                if (*((*dfa).nodes).offset(node as isize)).opr.c as libc::c_int
                    >= 0x80 as libc::c_int
                {
                    mb_chars = 1 as libc::c_int != 0;
                }
            }
            12 => {
                match (*((*dfa).nodes).offset(node as isize)).opr.ctx_type
                    as libc::c_uint
                {
                    16 | 32 | 64 | 128 => {}
                    _ => return,
                }
            }
            5 => {
                has_period = 1 as libc::c_int != 0;
            }
            4 | 10 | 2 | 11 | 8 | 9 => {}
            6 => return,
            3 => {
                let mut rshift: libc::c_int = if 0x80 as libc::c_int % 64 as libc::c_int
                    == 0 as libc::c_int
                {
                    0 as libc::c_int
                } else {
                    64 as libc::c_int - 0x80 as libc::c_int % 64 as libc::c_int
                };
                i = 0x80 as libc::c_int / 64 as libc::c_int;
                while i
                    < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                        + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                        / 64 as libc::c_int
                {
                    if *((*((*dfa).nodes).offset(node as isize)).opr.sbcset)
                        .offset(i as isize) >> rshift
                        != 0 as libc::c_int as libc::c_ulong
                    {
                        return;
                    }
                    rshift = 0 as libc::c_int;
                    i += 1;
                    i;
                }
            }
            _ => {
                abort();
            }
        }
        node += 1;
        node;
    }
    if mb_chars as libc::c_int != 0 || has_period as libc::c_int != 0 {
        node = 0 as libc::c_int as Idx;
        while (node as libc::c_ulong) < (*dfa).nodes_len {
            if (*((*dfa).nodes).offset(node as isize)).type_0() as libc::c_int
                == CHARACTER as libc::c_int
                && (*((*dfa).nodes).offset(node as isize)).opr.c as libc::c_int
                    >= 0x80 as libc::c_int
            {
                let ref mut fresh35 = *((*dfa).nodes).offset(node as isize);
                (*fresh35).set_mb_partial(0 as libc::c_int as libc::c_uint);
            } else if (*((*dfa).nodes).offset(node as isize)).type_0() as libc::c_int
                == OP_PERIOD as libc::c_int
            {
                let ref mut fresh36 = *((*dfa).nodes).offset(node as isize);
                (*fresh36).set_type_0(OP_UTF8_PERIOD);
            }
            node += 1;
            node;
        }
    }
    (*dfa).mb_cur_max = 1 as libc::c_int;
    (*dfa).set_is_utf8(0 as libc::c_int as libc::c_uint);
    (*dfa)
        .set_has_mb_node(
            ((*dfa).nbackref > 0 as libc::c_int as libc::c_long
                || has_period as libc::c_int != 0) as libc::c_int as libc::c_uint,
        );
}
unsafe extern "C" fn analyze(mut preg: *mut regex_t) -> reg_errcode_t {
    let mut dfa: *mut re_dfa_t = (*preg).buffer;
    let mut ret: reg_errcode_t = _REG_NOERROR;
    (*dfa)
        .nexts = malloc(
        ((*dfa).nodes_alloc).wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
    ) as *mut Idx;
    (*dfa)
        .org_indices = malloc(
        ((*dfa).nodes_alloc).wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
    ) as *mut Idx;
    (*dfa)
        .edests = malloc(
        ((*dfa).nodes_alloc)
            .wrapping_mul(::core::mem::size_of::<re_node_set>() as libc::c_ulong),
    ) as *mut re_node_set;
    (*dfa)
        .eclosures = malloc(
        ((*dfa).nodes_alloc)
            .wrapping_mul(::core::mem::size_of::<re_node_set>() as libc::c_ulong),
    ) as *mut re_node_set;
    if (((*dfa).nexts).is_null() || ((*dfa).org_indices).is_null()
        || ((*dfa).edests).is_null() || ((*dfa).eclosures).is_null()) as libc::c_int
        as libc::c_long != 0
    {
        return _REG_ESPACE;
    }
    (*dfa)
        .subexp_map = malloc(
        ((*preg).re_nsub).wrapping_mul(::core::mem::size_of::<Idx>() as libc::c_ulong),
    ) as *mut Idx;
    if !((*dfa).subexp_map).is_null() {
        let mut i: Idx = 0;
        i = 0 as libc::c_int as Idx;
        while (i as libc::c_ulong) < (*preg).re_nsub {
            *((*dfa).subexp_map).offset(i as isize) = i;
            i += 1;
            i;
        }
        preorder(
            (*dfa).str_tree,
            Some(
                optimize_subexps
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut bin_tree_t,
                    ) -> reg_errcode_t,
            ),
            dfa as *mut libc::c_void,
        );
        i = 0 as libc::c_int as Idx;
        while (i as libc::c_ulong) < (*preg).re_nsub {
            if *((*dfa).subexp_map).offset(i as isize) != i {
                break;
            }
            i += 1;
            i;
        }
        if i as libc::c_ulong == (*preg).re_nsub {
            rpl_free((*dfa).subexp_map as *mut libc::c_void);
            (*dfa).subexp_map = 0 as *mut Idx;
        }
    }
    ret = postorder(
        (*dfa).str_tree,
        Some(
            lower_subexps
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut bin_tree_t,
                ) -> reg_errcode_t,
        ),
        preg as *mut libc::c_void,
    );
    if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return ret;
    }
    ret = postorder(
        (*dfa).str_tree,
        Some(
            calc_first
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut bin_tree_t,
                ) -> reg_errcode_t,
        ),
        dfa as *mut libc::c_void,
    );
    if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return ret;
    }
    preorder(
        (*dfa).str_tree,
        Some(
            calc_next
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut bin_tree_t,
                ) -> reg_errcode_t,
        ),
        dfa as *mut libc::c_void,
    );
    ret = preorder(
        (*dfa).str_tree,
        Some(
            link_nfa_nodes
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut bin_tree_t,
                ) -> reg_errcode_t,
        ),
        dfa as *mut libc::c_void,
    );
    if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return ret;
    }
    ret = calc_eclosure(dfa);
    if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return ret;
    }
    if (*preg).no_sub() == 0 && (*preg).re_nsub > 0 as libc::c_int as libc::c_ulong
        && (*dfa).has_plural_match() as libc::c_int != 0 || (*dfa).nbackref != 0
    {
        (*dfa)
            .inveclosures = malloc(
            ((*dfa).nodes_len)
                .wrapping_mul(::core::mem::size_of::<re_node_set>() as libc::c_ulong),
        ) as *mut re_node_set;
        if ((*dfa).inveclosures == 0 as *mut libc::c_void as *mut re_node_set)
            as libc::c_int as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        ret = calc_inveclosure(dfa);
    }
    return ret;
}
unsafe extern "C" fn postorder(
    mut root: *mut bin_tree_t,
    mut fn_0: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut bin_tree_t) -> reg_errcode_t,
    >,
    mut extra: *mut libc::c_void,
) -> reg_errcode_t {
    let mut node: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut prev: *mut bin_tree_t = 0 as *mut bin_tree_t;
    node = root;
    loop {
        while !((*node).left).is_null() || !((*node).right).is_null() {
            if !((*node).left).is_null() {
                node = (*node).left;
            } else {
                node = (*node).right;
            }
        }
        loop {
            let mut err: reg_errcode_t = fn_0
                .expect("non-null function pointer")(extra, node);
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return err;
            }
            if ((*node).parent).is_null() {
                return _REG_NOERROR;
            }
            prev = node;
            node = (*node).parent;
            if !((*node).right == prev || ((*node).right).is_null()) {
                break;
            }
        }
        node = (*node).right;
    };
}
unsafe extern "C" fn preorder(
    mut root: *mut bin_tree_t,
    mut fn_0: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut bin_tree_t) -> reg_errcode_t,
    >,
    mut extra: *mut libc::c_void,
) -> reg_errcode_t {
    let mut node: *mut bin_tree_t = 0 as *mut bin_tree_t;
    node = root;
    loop {
        let mut err: reg_errcode_t = fn_0
            .expect("non-null function pointer")(extra, node);
        if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return err;
        }
        if !((*node).left).is_null() {
            node = (*node).left;
        } else {
            let mut prev: *mut bin_tree_t = 0 as *mut bin_tree_t;
            while (*node).right == prev || ((*node).right).is_null() {
                prev = node;
                node = (*node).parent;
                if node.is_null() {
                    return _REG_NOERROR;
                }
            }
            node = (*node).right;
        }
    };
}
unsafe extern "C" fn optimize_subexps(
    mut extra: *mut libc::c_void,
    mut node: *mut bin_tree_t,
) -> reg_errcode_t {
    let mut dfa: *mut re_dfa_t = extra as *mut re_dfa_t;
    if ((*node).token).type_0() as libc::c_int == OP_BACK_REF as libc::c_int
        && !((*dfa).subexp_map).is_null()
    {
        let mut idx: libc::c_int = (*node).token.opr.idx as libc::c_int;
        (*node).token.opr.idx = *((*dfa).subexp_map).offset(idx as isize);
        (*dfa).used_bkref_map
            |= ((1 as libc::c_int) << (*node).token.opr.idx) as libc::c_ulong;
    } else if ((*node).token).type_0() as libc::c_int == SUBEXP as libc::c_int
        && !((*node).left).is_null()
        && ((*(*node).left).token).type_0() as libc::c_int == SUBEXP as libc::c_int
    {
        let mut other_idx: Idx = (*(*node).left).token.opr.idx;
        (*node).left = (*(*node).left).left;
        if !((*node).left).is_null() {
            (*(*node).left).parent = node;
        }
        *((*dfa).subexp_map)
            .offset(
                other_idx as isize,
            ) = *((*dfa).subexp_map).offset((*node).token.opr.idx as isize);
        if other_idx < 64 as libc::c_int as libc::c_long {
            (*dfa).used_bkref_map &= !((1 as libc::c_int as bitset_word_t) << other_idx);
        }
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn lower_subexps(
    mut extra: *mut libc::c_void,
    mut node: *mut bin_tree_t,
) -> reg_errcode_t {
    let mut preg: *mut regex_t = extra as *mut regex_t;
    let mut err: reg_errcode_t = _REG_NOERROR;
    if !((*node).left).is_null()
        && ((*(*node).left).token).type_0() as libc::c_int == SUBEXP as libc::c_int
    {
        (*node).left = lower_subexp(&mut err, preg, (*node).left);
        if !((*node).left).is_null() {
            (*(*node).left).parent = node;
        }
    }
    if !((*node).right).is_null()
        && ((*(*node).right).token).type_0() as libc::c_int == SUBEXP as libc::c_int
    {
        (*node).right = lower_subexp(&mut err, preg, (*node).right);
        if !((*node).right).is_null() {
            (*(*node).right).parent = node;
        }
    }
    return err;
}
unsafe extern "C" fn lower_subexp(
    mut err: *mut reg_errcode_t,
    mut preg: *mut regex_t,
    mut node: *mut bin_tree_t,
) -> *mut bin_tree_t {
    let mut dfa: *mut re_dfa_t = (*preg).buffer;
    let mut body: *mut bin_tree_t = (*node).left;
    let mut op: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut cls: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut tree1: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
    if (*preg).no_sub() as libc::c_int != 0 && !((*node).left).is_null()
        && ((*node).token.opr.idx >= 64 as libc::c_int as libc::c_long
            || (*dfa).used_bkref_map
                & (1 as libc::c_int as bitset_word_t) << (*node).token.opr.idx == 0)
    {
        return (*node).left;
    }
    op = create_tree(dfa, 0 as *mut bin_tree_t, 0 as *mut bin_tree_t, OP_OPEN_SUBEXP);
    cls = create_tree(dfa, 0 as *mut bin_tree_t, 0 as *mut bin_tree_t, OP_CLOSE_SUBEXP);
    tree1 = if !body.is_null() { create_tree(dfa, body, cls, CONCAT) } else { cls };
    tree = create_tree(dfa, op, tree1, CONCAT);
    if (tree.is_null() || tree1.is_null() || op.is_null() || cls.is_null())
        as libc::c_int as libc::c_long != 0
    {
        *err = _REG_ESPACE;
        return 0 as *mut bin_tree_t;
    }
    (*cls).token.opr.idx = (*node).token.opr.idx;
    (*op).token.opr.idx = (*cls).token.opr.idx;
    ((*cls).token).set_opt_subexp(((*node).token).opt_subexp());
    ((*op).token).set_opt_subexp(((*cls).token).opt_subexp());
    return tree;
}
unsafe extern "C" fn calc_first(
    mut extra: *mut libc::c_void,
    mut node: *mut bin_tree_t,
) -> reg_errcode_t {
    let mut dfa: *mut re_dfa_t = extra as *mut re_dfa_t;
    if ((*node).token).type_0() as libc::c_int == CONCAT as libc::c_int {
        (*node).first = (*(*node).left).first;
        (*node).node_idx = (*(*node).left).node_idx;
    } else {
        (*node).first = node;
        (*node).node_idx = re_dfa_add_node(dfa, (*node).token);
        if ((*node).node_idx == -(1 as libc::c_int) as libc::c_long) as libc::c_int
            as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        if ((*node).token).type_0() as libc::c_int == ANCHOR as libc::c_int {
            let ref mut fresh37 = *((*dfa).nodes).offset((*node).node_idx as isize);
            (*fresh37).set_constraint((*node).token.opr.ctx_type as libc::c_uint);
        }
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn calc_next(
    mut extra: *mut libc::c_void,
    mut node: *mut bin_tree_t,
) -> reg_errcode_t {
    match ((*node).token).type_0() as libc::c_int {
        11 => {
            (*(*node).left).next = node;
        }
        16 => {
            (*(*node).left).next = (*(*node).right).first;
            (*(*node).right).next = (*node).next;
        }
        _ => {
            if !((*node).left).is_null() {
                (*(*node).left).next = (*node).next;
            }
            if !((*node).right).is_null() {
                (*(*node).right).next = (*node).next;
            }
        }
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn link_nfa_nodes(
    mut extra: *mut libc::c_void,
    mut node: *mut bin_tree_t,
) -> reg_errcode_t {
    let mut dfa: *mut re_dfa_t = extra as *mut re_dfa_t;
    let mut idx: Idx = (*node).node_idx;
    let mut err: reg_errcode_t = _REG_NOERROR;
    match ((*node).token).type_0() as libc::c_int {
        16 => {}
        2 => {
            if ((*node).next).is_null() {} else {
                unreachable!();
            };
        }
        11 | 10 => {
            let mut left: Idx = 0;
            let mut right: Idx = 0;
            (*dfa).set_has_plural_match(1 as libc::c_int as libc::c_uint);
            if !((*node).left).is_null() {
                left = (*(*(*node).left).first).node_idx;
            } else {
                left = (*(*node).next).node_idx;
            }
            if !((*node).right).is_null() {
                right = (*(*(*node).right).first).node_idx;
            } else {
                right = (*(*node).next).node_idx;
            }
            if left > -(1 as libc::c_int) as libc::c_long {} else {
                unreachable!();
            };
            if right > -(1 as libc::c_int) as libc::c_long {} else {
                unreachable!();
            };
            err = re_node_set_init_2(((*dfa).edests).offset(idx as isize), left, right);
        }
        12 | 8 | 9 => {
            err = re_node_set_init_1(
                ((*dfa).edests).offset(idx as isize),
                (*(*node).next).node_idx,
            );
        }
        4 => {
            *((*dfa).nexts).offset(idx as isize) = (*(*node).next).node_idx;
            if ((*node).token).type_0() as libc::c_int == OP_BACK_REF as libc::c_int {
                err = re_node_set_init_1(
                    ((*dfa).edests).offset(idx as isize),
                    *((*dfa).nexts).offset(idx as isize),
                );
            }
        }
        _ => {
            if ((*node).token).type_0() as libc::c_int & 8 as libc::c_int == 0 {} else {
                unreachable!();
            };
            *((*dfa).nexts).offset(idx as isize) = (*(*node).next).node_idx;
        }
    }
    return err;
}
unsafe extern "C" fn duplicate_node_closure(
    mut dfa: *mut re_dfa_t,
    mut top_org_node: Idx,
    mut top_clone_node: Idx,
    mut root_node: Idx,
    mut init_constraint: libc::c_uint,
) -> reg_errcode_t {
    let mut org_node: Idx = 0;
    let mut clone_node: Idx = 0;
    let mut ok: bool = false;
    let mut constraint: libc::c_uint = init_constraint;
    org_node = top_org_node;
    clone_node = top_clone_node;
    loop {
        let mut org_dest: Idx = 0;
        let mut clone_dest: Idx = 0;
        if (*((*dfa).nodes).offset(org_node as isize)).type_0() as libc::c_int
            == OP_BACK_REF as libc::c_int
        {
            org_dest = *((*dfa).nexts).offset(org_node as isize);
            (*((*dfa).edests).offset(clone_node as isize))
                .nelem = 0 as libc::c_int as Idx;
            clone_dest = duplicate_node(dfa, org_dest, constraint);
            if (clone_dest == -(1 as libc::c_int) as libc::c_long) as libc::c_int
                as libc::c_long != 0
            {
                return _REG_ESPACE;
            }
            *((*dfa).nexts)
                .offset(clone_node as isize) = *((*dfa).nexts).offset(org_node as isize);
            ok = re_node_set_insert(
                ((*dfa).edests).offset(clone_node as isize),
                clone_dest,
            );
            if !ok as libc::c_int as libc::c_long != 0 {
                return _REG_ESPACE;
            }
        } else if (*((*dfa).edests).offset(org_node as isize)).nelem
            == 0 as libc::c_int as libc::c_long
        {
            *((*dfa).nexts)
                .offset(clone_node as isize) = *((*dfa).nexts).offset(org_node as isize);
            break;
        } else if (*((*dfa).edests).offset(org_node as isize)).nelem
            == 1 as libc::c_int as libc::c_long
        {
            org_dest = *((*((*dfa).edests).offset(org_node as isize)).elems)
                .offset(0 as libc::c_int as isize);
            (*((*dfa).edests).offset(clone_node as isize))
                .nelem = 0 as libc::c_int as Idx;
            if org_node == root_node && clone_node != org_node {
                ok = re_node_set_insert(
                    ((*dfa).edests).offset(clone_node as isize),
                    org_dest,
                );
                if !ok as libc::c_int as libc::c_long != 0 {
                    return _REG_ESPACE;
                }
                break;
            } else {
                constraint |= (*((*dfa).nodes).offset(org_node as isize)).constraint();
                clone_dest = duplicate_node(dfa, org_dest, constraint);
                if (clone_dest == -(1 as libc::c_int) as libc::c_long) as libc::c_int
                    as libc::c_long != 0
                {
                    return _REG_ESPACE;
                }
                ok = re_node_set_insert(
                    ((*dfa).edests).offset(clone_node as isize),
                    clone_dest,
                );
                if !ok as libc::c_int as libc::c_long != 0 {
                    return _REG_ESPACE;
                }
            }
        } else {
            org_dest = *((*((*dfa).edests).offset(org_node as isize)).elems)
                .offset(0 as libc::c_int as isize);
            (*((*dfa).edests).offset(clone_node as isize))
                .nelem = 0 as libc::c_int as Idx;
            clone_dest = search_duplicated_node(dfa, org_dest, constraint);
            if clone_dest == -(1 as libc::c_int) as libc::c_long {
                let mut err: reg_errcode_t = _REG_NOERROR;
                clone_dest = duplicate_node(dfa, org_dest, constraint);
                if (clone_dest == -(1 as libc::c_int) as libc::c_long) as libc::c_int
                    as libc::c_long != 0
                {
                    return _REG_ESPACE;
                }
                ok = re_node_set_insert(
                    ((*dfa).edests).offset(clone_node as isize),
                    clone_dest,
                );
                if !ok as libc::c_int as libc::c_long != 0 {
                    return _REG_ESPACE;
                }
                err = duplicate_node_closure(
                    dfa,
                    org_dest,
                    clone_dest,
                    root_node,
                    constraint,
                );
                if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    return err;
                }
            } else {
                ok = re_node_set_insert(
                    ((*dfa).edests).offset(clone_node as isize),
                    clone_dest,
                );
                if !ok as libc::c_int as libc::c_long != 0 {
                    return _REG_ESPACE;
                }
            }
            org_dest = *((*((*dfa).edests).offset(org_node as isize)).elems)
                .offset(1 as libc::c_int as isize);
            clone_dest = duplicate_node(dfa, org_dest, constraint);
            if (clone_dest == -(1 as libc::c_int) as libc::c_long) as libc::c_int
                as libc::c_long != 0
            {
                return _REG_ESPACE;
            }
            ok = re_node_set_insert(
                ((*dfa).edests).offset(clone_node as isize),
                clone_dest,
            );
            if !ok as libc::c_int as libc::c_long != 0 {
                return _REG_ESPACE;
            }
        }
        org_node = org_dest;
        clone_node = clone_dest;
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn search_duplicated_node(
    mut dfa: *const re_dfa_t,
    mut org_node: Idx,
    mut constraint: libc::c_uint,
) -> Idx {
    let mut idx: Idx = 0;
    idx = ((*dfa).nodes_len).wrapping_sub(1 as libc::c_int as libc::c_ulong) as Idx;
    while (*((*dfa).nodes).offset(idx as isize)).duplicated() as libc::c_int != 0
        && idx > 0 as libc::c_int as libc::c_long
    {
        if org_node == *((*dfa).org_indices).offset(idx as isize)
            && constraint == (*((*dfa).nodes).offset(idx as isize)).constraint()
        {
            return idx;
        }
        idx -= 1;
        idx;
    }
    return -(1 as libc::c_int) as Idx;
}
unsafe extern "C" fn duplicate_node(
    mut dfa: *mut re_dfa_t,
    mut org_idx: Idx,
    mut constraint: libc::c_uint,
) -> Idx {
    let mut dup_idx: Idx = re_dfa_add_node(
        dfa,
        *((*dfa).nodes).offset(org_idx as isize),
    );
    if (dup_idx != -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        let ref mut fresh38 = *((*dfa).nodes).offset(dup_idx as isize);
        (*fresh38).set_constraint(constraint);
        let ref mut fresh39 = *((*dfa).nodes).offset(dup_idx as isize);
        (*fresh39)
            .set_constraint(
                (*fresh39).constraint()
                    | (*((*dfa).nodes).offset(org_idx as isize)).constraint()
                        as libc::c_int as libc::c_uint,
            );
        let ref mut fresh40 = *((*dfa).nodes).offset(dup_idx as isize);
        (*fresh40).set_duplicated(1 as libc::c_int as libc::c_uint);
        *((*dfa).org_indices).offset(dup_idx as isize) = org_idx;
    }
    return dup_idx;
}
unsafe extern "C" fn calc_inveclosure(mut dfa: *mut re_dfa_t) -> reg_errcode_t {
    let mut src: Idx = 0;
    let mut idx: Idx = 0;
    let mut ok: bool = false;
    idx = 0 as libc::c_int as Idx;
    while (idx as libc::c_ulong) < (*dfa).nodes_len {
        memset(
            ((*dfa).inveclosures).offset(idx as isize) as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<re_node_set>() as libc::c_ulong,
        );
        idx += 1;
        idx;
    }
    src = 0 as libc::c_int as Idx;
    while (src as libc::c_ulong) < (*dfa).nodes_len {
        let mut elems: *mut Idx = (*((*dfa).eclosures).offset(src as isize)).elems;
        idx = 0 as libc::c_int as Idx;
        while idx < (*((*dfa).eclosures).offset(src as isize)).nelem {
            ok = re_node_set_insert_last(
                ((*dfa).inveclosures).offset(*elems.offset(idx as isize) as isize),
                src,
            );
            if !ok as libc::c_int as libc::c_long != 0 {
                return _REG_ESPACE;
            }
            idx += 1;
            idx;
        }
        src += 1;
        src;
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn calc_eclosure(mut dfa: *mut re_dfa_t) -> reg_errcode_t {
    let mut node_idx: Idx = 0;
    let mut incomplete: bool = false;
    if (*dfa).nodes_len > 0 as libc::c_int as libc::c_ulong {} else {
        unreachable!();
    };
    incomplete = 0 as libc::c_int != 0;
    node_idx = 0 as libc::c_int as Idx;
    loop {
        let mut err: reg_errcode_t = _REG_NOERROR;
        let mut eclosure_elem: re_node_set = re_node_set {
            alloc: 0,
            nelem: 0,
            elems: 0 as *mut Idx,
        };
        if node_idx as libc::c_ulong == (*dfa).nodes_len {
            if !incomplete {
                break;
            }
            incomplete = 0 as libc::c_int != 0;
            node_idx = 0 as libc::c_int as Idx;
        }
        if (*((*dfa).eclosures).offset(node_idx as isize)).nelem
            != -(1 as libc::c_int) as libc::c_long
        {} else {
            unreachable!();
        };
        if !((*((*dfa).eclosures).offset(node_idx as isize)).nelem
            != 0 as libc::c_int as libc::c_long)
        {
            err = calc_eclosure_iter(
                &mut eclosure_elem,
                dfa,
                node_idx,
                1 as libc::c_int != 0,
            );
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return err;
            }
            if (*((*dfa).eclosures).offset(node_idx as isize)).nelem
                == 0 as libc::c_int as libc::c_long
            {
                incomplete = 1 as libc::c_int != 0;
                rpl_free(eclosure_elem.elems as *mut libc::c_void);
            }
        }
        node_idx += 1;
        node_idx;
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn calc_eclosure_iter(
    mut new_set: *mut re_node_set,
    mut dfa: *mut re_dfa_t,
    mut node: Idx,
    mut root: bool,
) -> reg_errcode_t {
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut i: Idx = 0;
    let mut eclosure: re_node_set = re_node_set {
        alloc: 0,
        nelem: 0,
        elems: 0 as *mut Idx,
    };
    let mut incomplete: bool = 0 as libc::c_int != 0;
    err = re_node_set_alloc(
        &mut eclosure,
        (*((*dfa).edests).offset(node as isize)).nelem + 1 as libc::c_int as libc::c_long,
    );
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return err;
    }
    let fresh41 = eclosure.nelem;
    eclosure.nelem = eclosure.nelem + 1;
    *(eclosure.elems).offset(fresh41 as isize) = node;
    (*((*dfa).eclosures).offset(node as isize)).nelem = -(1 as libc::c_int) as Idx;
    if (*((*dfa).nodes).offset(node as isize)).constraint() as libc::c_int != 0
        && (*((*dfa).edests).offset(node as isize)).nelem != 0
        && (*((*dfa).nodes)
            .offset(
                *((*((*dfa).edests).offset(node as isize)).elems)
                    .offset(0 as libc::c_int as isize) as isize,
            ))
            .duplicated() == 0
    {
        err = duplicate_node_closure(
            dfa,
            node,
            node,
            node,
            (*((*dfa).nodes).offset(node as isize)).constraint(),
        );
        if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return err;
        }
    }
    if (*((*dfa).nodes).offset(node as isize)).type_0() as libc::c_int & 8 as libc::c_int
        != 0
    {
        i = 0 as libc::c_int as Idx;
        while i < (*((*dfa).edests).offset(node as isize)).nelem {
            let mut eclosure_elem: re_node_set = re_node_set {
                alloc: 0,
                nelem: 0,
                elems: 0 as *mut Idx,
            };
            let mut edest: Idx = *((*((*dfa).edests).offset(node as isize)).elems)
                .offset(i as isize);
            if (*((*dfa).eclosures).offset(edest as isize)).nelem
                == -(1 as libc::c_int) as libc::c_long
            {
                incomplete = 1 as libc::c_int != 0;
            } else {
                if (*((*dfa).eclosures).offset(edest as isize)).nelem
                    == 0 as libc::c_int as libc::c_long
                {
                    err = calc_eclosure_iter(
                        &mut eclosure_elem,
                        dfa,
                        edest,
                        0 as libc::c_int != 0,
                    );
                    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        return err;
                    }
                } else {
                    eclosure_elem = *((*dfa).eclosures).offset(edest as isize);
                }
                err = re_node_set_merge(&mut eclosure, &mut eclosure_elem);
                if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    return err;
                }
                if (*((*dfa).eclosures).offset(edest as isize)).nelem
                    == 0 as libc::c_int as libc::c_long
                {
                    incomplete = 1 as libc::c_int != 0;
                    rpl_free(eclosure_elem.elems as *mut libc::c_void);
                }
            }
            i += 1;
            i;
        }
    }
    if incomplete as libc::c_int != 0 && !root {
        (*((*dfa).eclosures).offset(node as isize)).nelem = 0 as libc::c_int as Idx;
    } else {
        *((*dfa).eclosures).offset(node as isize) = eclosure;
    }
    *new_set = eclosure;
    return _REG_NOERROR;
}
unsafe extern "C" fn fetch_token(
    mut result: *mut re_token_t,
    mut input: *mut re_string_t,
    mut syntax: reg_syntax_t,
) {
    (*input).cur_idx += peek_token(result, input, syntax) as libc::c_long;
}
unsafe extern "C" fn peek_token(
    mut token: *mut re_token_t,
    mut input: *mut re_string_t,
    mut syntax: reg_syntax_t,
) -> libc::c_int {
    let mut c: libc::c_uchar = 0;
    if (*input).stop <= (*input).cur_idx {
        (*token).set_type_0(END_OF_RE);
        return 0 as libc::c_int;
    }
    c = *((*input).mbs)
        .offset(((*input).cur_idx + 0 as libc::c_int as libc::c_long) as isize);
    (*token).opr.c = c;
    (*token).set_word_char(0 as libc::c_int as libc::c_uint);
    (*token).set_mb_partial(0 as libc::c_int as libc::c_uint);
    if (*input).mb_cur_max > 1 as libc::c_int
        && !((*input).cur_idx == (*input).valid_len
            || *((*input).wcs).offset((*input).cur_idx as isize)
                != 0xffffffff as libc::c_uint)
    {
        (*token).set_type_0(CHARACTER);
        (*token).set_mb_partial(1 as libc::c_int as libc::c_uint);
        return 1 as libc::c_int;
    }
    if c as libc::c_int == '\\' as i32 {
        let mut c2: libc::c_uchar = 0;
        if (*input).cur_idx + 1 as libc::c_int as libc::c_long >= (*input).len {
            (*token).set_type_0(BACK_SLASH);
            return 1 as libc::c_int;
        }
        c2 = re_string_peek_byte_case(input, 1 as libc::c_int as Idx);
        (*token).opr.c = c2;
        (*token).set_type_0(CHARACTER);
        if (*input).mb_cur_max > 1 as libc::c_int {
            let mut wc: wint_t = re_string_wchar_at(
                input,
                (*input).cur_idx + 1 as libc::c_int as libc::c_long,
            );
            (*token)
                .set_word_char(
                    ((iswalnum(wc) != 0 || wc == '_' as i32 as libc::c_uint)
                        as libc::c_int != 0 as libc::c_int) as libc::c_int
                        as libc::c_uint,
                );
        } else {
            (*token)
                .set_word_char(
                    ((*(*__ctype_b_loc()).offset(c2 as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                        || c2 as libc::c_int == '_' as i32) as libc::c_int
                        != 0 as libc::c_int) as libc::c_int as libc::c_uint,
                );
        }
        match c2 as libc::c_int {
            124 => {
                if syntax
                    & ((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    == 0
                    && syntax
                        & (((((((((((((((1 as libc::c_int as libc::c_ulong)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                        == 0
                {
                    (*token).set_type_0(OP_ALT);
                }
            }
            49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                if syntax
                    & ((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(OP_BACK_REF);
                    (*token).opr.idx = (c2 as libc::c_int - '1' as i32) as Idx;
                }
            }
            60 => {
                if syntax
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(ANCHOR);
                    (*token).opr.ctx_type = WORD_FIRST;
                }
            }
            62 => {
                if syntax
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(ANCHOR);
                    (*token).opr.ctx_type = WORD_LAST;
                }
            }
            98 => {
                if syntax
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(ANCHOR);
                    (*token).opr.ctx_type = WORD_DELIM;
                }
            }
            66 => {
                if syntax
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(ANCHOR);
                    (*token).opr.ctx_type = NOT_WORD_DELIM;
                }
            }
            119 => {
                if syntax
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(OP_WORD);
                }
            }
            87 => {
                if syntax
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(OP_NOTWORD);
                }
            }
            115 => {
                if syntax
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(OP_SPACE);
                }
            }
            83 => {
                if syntax
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(OP_NOTSPACE);
                }
            }
            96 => {
                if syntax
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(ANCHOR);
                    (*token).opr.ctx_type = BUF_FIRST;
                }
            }
            39 => {
                if syntax
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(ANCHOR);
                    (*token).opr.ctx_type = BUF_LAST;
                }
            }
            40 => {
                if syntax
                    & (((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(OP_OPEN_SUBEXP);
                }
            }
            41 => {
                if syntax
                    & (((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(OP_CLOSE_SUBEXP);
                }
            }
            43 => {
                if syntax
                    & ((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    == 0
                    && syntax & (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
                        != 0
                {
                    (*token).set_type_0(OP_DUP_PLUS);
                }
            }
            63 => {
                if syntax
                    & ((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    == 0
                    && syntax & (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
                        != 0
                {
                    (*token).set_type_0(OP_DUP_QUESTION);
                }
            }
            123 => {
                if syntax
                    & (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int != 0
                    && syntax
                        & ((((((((((((1 as libc::c_int as libc::c_ulong)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(OP_OPEN_DUP_NUM);
                }
            }
            125 => {
                if syntax
                    & (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int != 0
                    && syntax
                        & ((((((((((((1 as libc::c_int as libc::c_ulong)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int == 0
                {
                    (*token).set_type_0(OP_CLOSE_DUP_NUM);
                }
            }
            _ => {}
        }
        return 2 as libc::c_int;
    }
    (*token).set_type_0(CHARACTER);
    if (*input).mb_cur_max > 1 as libc::c_int {
        let mut wc_0: wint_t = re_string_wchar_at(input, (*input).cur_idx);
        (*token)
            .set_word_char(
                ((iswalnum(wc_0) != 0 || wc_0 == '_' as i32 as libc::c_uint)
                    as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
    } else {
        (*token)
            .set_word_char(
                (*(*__ctype_b_loc()).offset((*token).opr.c as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || (*token).opr.c as libc::c_int == '_' as i32) as libc::c_int
                    as libc::c_uint,
            );
    }
    let mut current_block_108: u64;
    match c as libc::c_int {
        10 => {
            if syntax
                & (((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int != 0
            {
                (*token).set_type_0(OP_ALT);
            }
        }
        124 => {
            if syntax
                & ((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int == 0
                && syntax
                    & (((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    != 0
            {
                (*token).set_type_0(OP_ALT);
            }
        }
        42 => {
            (*token).set_type_0(OP_DUP_ASTERISK);
        }
        43 => {
            if syntax
                & ((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int == 0
                && syntax & (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int == 0
            {
                (*token).set_type_0(OP_DUP_PLUS);
            }
        }
        63 => {
            if syntax
                & ((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int == 0
                && syntax & (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int == 0
            {
                (*token).set_type_0(OP_DUP_QUESTION);
            }
        }
        123 => {
            if syntax
                & (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int != 0
                && syntax
                    & ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int != 0
            {
                (*token).set_type_0(OP_OPEN_DUP_NUM);
            }
        }
        125 => {
            if syntax
                & (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int != 0
                && syntax
                    & ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int != 0
            {
                (*token).set_type_0(OP_CLOSE_DUP_NUM);
            }
        }
        40 => {
            if syntax
                & (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int != 0
            {
                (*token).set_type_0(OP_OPEN_SUBEXP);
            }
        }
        41 => {
            if syntax
                & (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int != 0
            {
                (*token).set_type_0(OP_CLOSE_SUBEXP);
            }
        }
        91 => {
            (*token).set_type_0(OP_OPEN_BRACKET);
        }
        46 => {
            (*token).set_type_0(OP_PERIOD);
        }
        94 => {
            if syntax
                & ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) == 0
                && (*input).cur_idx != 0 as libc::c_int as libc::c_long
            {
                let mut prev: libc::c_char = *((*input).mbs)
                    .offset(
                        ((*input).cur_idx + -(1 as libc::c_int) as libc::c_long) as isize,
                    ) as libc::c_char;
                if syntax
                    & (((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0 || prev as libc::c_int != '\n' as i32
                {
                    current_block_108 = 18218798608644444571;
                } else {
                    current_block_108 = 4983594971376015098;
                }
            } else {
                current_block_108 = 4983594971376015098;
            }
            match current_block_108 {
                18218798608644444571 => {}
                _ => {
                    (*token).set_type_0(ANCHOR);
                    (*token).opr.ctx_type = LINE_FIRST;
                }
            }
        }
        36 => {
            if syntax
                & (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int == 0
                && (*input).cur_idx + 1 as libc::c_int as libc::c_long != (*input).len
            {
                let mut next: re_token_t = re_token_t {
                    opr: C2RustUnnamed { c: 0 },
                    type_0_constraint_duplicated_opt_subexp_accept_mb_mb_partial_word_char: [0; 3],
                    c2rust_padding: [0; 5],
                };
                (*input).cur_idx += 1 as libc::c_int as libc::c_long;
                peek_token(&mut next, input, syntax);
                (*input).cur_idx += -(1 as libc::c_int) as libc::c_long;
                if next.type_0() as libc::c_int != OP_ALT as libc::c_int
                    && next.type_0() as libc::c_int != OP_CLOSE_SUBEXP as libc::c_int
                {
                    current_block_108 = 18218798608644444571;
                } else {
                    current_block_108 = 8304106758420804164;
                }
            } else {
                current_block_108 = 8304106758420804164;
            }
            match current_block_108 {
                18218798608644444571 => {}
                _ => {
                    (*token).set_type_0(ANCHOR);
                    (*token).opr.ctx_type = LINE_LAST;
                }
            }
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn peek_token_bracket(
    mut token: *mut re_token_t,
    mut input: *mut re_string_t,
    mut syntax: reg_syntax_t,
) -> libc::c_int {
    let mut c: libc::c_uchar = 0;
    if (*input).stop <= (*input).cur_idx {
        (*token).set_type_0(END_OF_RE);
        return 0 as libc::c_int;
    }
    c = *((*input).mbs)
        .offset(((*input).cur_idx + 0 as libc::c_int as libc::c_long) as isize);
    (*token).opr.c = c;
    if (*input).mb_cur_max > 1 as libc::c_int
        && !((*input).cur_idx == (*input).valid_len
            || *((*input).wcs).offset((*input).cur_idx as isize)
                != 0xffffffff as libc::c_uint)
    {
        (*token).set_type_0(CHARACTER);
        return 1 as libc::c_int;
    }
    if c as libc::c_int == '\\' as i32 && syntax & 1 as libc::c_int as libc::c_ulong != 0
        && ((*input).cur_idx + 1 as libc::c_int as libc::c_long) < (*input).len
    {
        let mut c2: libc::c_uchar = 0;
        (*input).cur_idx += 1 as libc::c_int as libc::c_long;
        c2 = *((*input).mbs)
            .offset(((*input).cur_idx + 0 as libc::c_int as libc::c_long) as isize);
        (*token).opr.c = c2;
        (*token).set_type_0(CHARACTER);
        return 1 as libc::c_int;
    }
    if c as libc::c_int == '[' as i32 {
        let mut c2_0: libc::c_uchar = 0;
        let mut token_len: libc::c_int = 0;
        if ((*input).cur_idx + 1 as libc::c_int as libc::c_long) < (*input).len {
            c2_0 = *((*input).mbs)
                .offset(((*input).cur_idx + 1 as libc::c_int as libc::c_long) as isize);
        } else {
            c2_0 = 0 as libc::c_int as libc::c_uchar;
        }
        (*token).opr.c = c2_0;
        token_len = 2 as libc::c_int;
        let mut current_block_28: u64;
        match c2_0 as libc::c_int {
            46 => {
                (*token).set_type_0(OP_OPEN_COLL_ELEM);
                current_block_28 = 7172762164747879670;
            }
            61 => {
                (*token).set_type_0(OP_OPEN_EQUIV_CLASS);
                current_block_28 = 7172762164747879670;
            }
            58 => {
                if syntax
                    & ((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int != 0
                {
                    (*token).set_type_0(OP_OPEN_CHAR_CLASS);
                    current_block_28 = 7172762164747879670;
                } else {
                    current_block_28 = 14361539447548946753;
                }
            }
            _ => {
                current_block_28 = 14361539447548946753;
            }
        }
        match current_block_28 {
            14361539447548946753 => {
                (*token).set_type_0(CHARACTER);
                (*token).opr.c = c;
                token_len = 1 as libc::c_int;
            }
            _ => {}
        }
        return token_len;
    }
    let mut current_block_38: u64;
    match c as libc::c_int {
        93 => {
            (*token).set_type_0(OP_CLOSE_BRACKET);
            current_block_38 = 2604890879466389055;
        }
        94 => {
            (*token).set_type_0(OP_NON_MATCH_LIST);
            current_block_38 = 2604890879466389055;
        }
        45 => {
            if !(((*input).cur_idx + 2 as libc::c_int as libc::c_long) < (*input).len
                && *((*input).mbs)
                    .offset(
                        ((*input).cur_idx + 1 as libc::c_int as libc::c_long) as isize,
                    ) as libc::c_int == '-' as i32
                && *((*input).mbs)
                    .offset(
                        ((*input).cur_idx + 2 as libc::c_int as libc::c_long) as isize,
                    ) as libc::c_int == '-' as i32)
            {
                (*token).set_type_0(OP_CHARSET_RANGE);
                current_block_38 = 2604890879466389055;
            } else {
                (*input).cur_idx += 2 as libc::c_int as libc::c_long;
                current_block_38 = 12365250025022821251;
            }
        }
        _ => {
            current_block_38 = 12365250025022821251;
        }
    }
    match current_block_38 {
        12365250025022821251 => {
            (*token).set_type_0(CHARACTER);
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn parse(
    mut regexp: *mut re_string_t,
    mut preg: *mut regex_t,
    mut syntax: reg_syntax_t,
    mut err: *mut reg_errcode_t,
) -> *mut bin_tree_t {
    let mut dfa: *mut re_dfa_t = (*preg).buffer;
    let mut tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut eor: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut root: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut current_token: re_token_t = re_token_t {
        opr: C2RustUnnamed { c: 0 },
        type_0_constraint_duplicated_opt_subexp_accept_mb_mb_partial_word_char: [0; 3],
        c2rust_padding: [0; 5],
    };
    (*dfa).syntax = syntax;
    fetch_token(
        &mut current_token,
        regexp,
        syntax
            | (((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int,
    );
    tree = parse_reg_exp(
        regexp,
        preg,
        &mut current_token,
        syntax,
        0 as libc::c_int as Idx,
        err,
    );
    if (*err as libc::c_int != _REG_NOERROR as libc::c_int && tree.is_null())
        as libc::c_int as libc::c_long != 0
    {
        return 0 as *mut bin_tree_t;
    }
    eor = create_tree(dfa, 0 as *mut bin_tree_t, 0 as *mut bin_tree_t, END_OF_RE);
    if !tree.is_null() {
        root = create_tree(dfa, tree, eor, CONCAT);
    } else {
        root = eor;
    }
    if (eor.is_null() || root.is_null()) as libc::c_int as libc::c_long != 0 {
        *err = _REG_ESPACE;
        return 0 as *mut bin_tree_t;
    }
    return root;
}
unsafe extern "C" fn parse_reg_exp(
    mut regexp: *mut re_string_t,
    mut preg: *mut regex_t,
    mut token: *mut re_token_t,
    mut syntax: reg_syntax_t,
    mut nest: Idx,
    mut err: *mut reg_errcode_t,
) -> *mut bin_tree_t {
    let mut dfa: *mut re_dfa_t = (*preg).buffer;
    let mut tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut branch: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut initial_bkref_map: bitset_word_t = (*dfa).completed_bkref_map;
    tree = parse_branch(regexp, preg, token, syntax, nest, err);
    if (*err as libc::c_int != _REG_NOERROR as libc::c_int && tree.is_null())
        as libc::c_int as libc::c_long != 0
    {
        return 0 as *mut bin_tree_t;
    }
    while (*token).type_0() as libc::c_int == OP_ALT as libc::c_int {
        fetch_token(
            token,
            regexp,
            syntax
                | (((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int,
        );
        if (*token).type_0() as libc::c_int != OP_ALT as libc::c_int
            && (*token).type_0() as libc::c_int != END_OF_RE as libc::c_int
            && (nest == 0 as libc::c_int as libc::c_long
                || (*token).type_0() as libc::c_int != OP_CLOSE_SUBEXP as libc::c_int)
        {
            let mut accumulated_bkref_map: bitset_word_t = (*dfa).completed_bkref_map;
            (*dfa).completed_bkref_map = initial_bkref_map;
            branch = parse_branch(regexp, preg, token, syntax, nest, err);
            if (*err as libc::c_int != _REG_NOERROR as libc::c_int && branch.is_null())
                as libc::c_int as libc::c_long != 0
            {
                if !tree.is_null() {
                    postorder(
                        tree,
                        Some(
                            free_tree
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut bin_tree_t,
                                ) -> reg_errcode_t,
                        ),
                        0 as *mut libc::c_void,
                    );
                }
                return 0 as *mut bin_tree_t;
            }
            (*dfa).completed_bkref_map |= accumulated_bkref_map;
        } else {
            branch = 0 as *mut bin_tree_t;
        }
        tree = create_tree(dfa, tree, branch, OP_ALT);
        if (tree == 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int
            as libc::c_long != 0
        {
            *err = _REG_ESPACE;
            return 0 as *mut bin_tree_t;
        }
    }
    return tree;
}
unsafe extern "C" fn parse_branch(
    mut regexp: *mut re_string_t,
    mut preg: *mut regex_t,
    mut token: *mut re_token_t,
    mut syntax: reg_syntax_t,
    mut nest: Idx,
    mut err: *mut reg_errcode_t,
) -> *mut bin_tree_t {
    let mut tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut expr: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut dfa: *mut re_dfa_t = (*preg).buffer;
    tree = parse_expression(regexp, preg, token, syntax, nest, err);
    if (*err as libc::c_int != _REG_NOERROR as libc::c_int && tree.is_null())
        as libc::c_int as libc::c_long != 0
    {
        return 0 as *mut bin_tree_t;
    }
    while (*token).type_0() as libc::c_int != OP_ALT as libc::c_int
        && (*token).type_0() as libc::c_int != END_OF_RE as libc::c_int
        && (nest == 0 as libc::c_int as libc::c_long
            || (*token).type_0() as libc::c_int != OP_CLOSE_SUBEXP as libc::c_int)
    {
        expr = parse_expression(regexp, preg, token, syntax, nest, err);
        if (*err as libc::c_int != _REG_NOERROR as libc::c_int && expr.is_null())
            as libc::c_int as libc::c_long != 0
        {
            if !tree.is_null() {
                postorder(
                    tree,
                    Some(
                        free_tree
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut bin_tree_t,
                            ) -> reg_errcode_t,
                    ),
                    0 as *mut libc::c_void,
                );
            }
            return 0 as *mut bin_tree_t;
        }
        if !tree.is_null() && !expr.is_null() {
            let mut newtree: *mut bin_tree_t = create_tree(dfa, tree, expr, CONCAT);
            if newtree.is_null() {
                postorder(
                    expr,
                    Some(
                        free_tree
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut bin_tree_t,
                            ) -> reg_errcode_t,
                    ),
                    0 as *mut libc::c_void,
                );
                postorder(
                    tree,
                    Some(
                        free_tree
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut bin_tree_t,
                            ) -> reg_errcode_t,
                    ),
                    0 as *mut libc::c_void,
                );
                *err = _REG_ESPACE;
                return 0 as *mut bin_tree_t;
            }
            tree = newtree;
        } else if tree.is_null() {
            tree = expr;
        }
    }
    return tree;
}
unsafe extern "C" fn parse_expression(
    mut regexp: *mut re_string_t,
    mut preg: *mut regex_t,
    mut token: *mut re_token_t,
    mut syntax: reg_syntax_t,
    mut nest: Idx,
    mut err: *mut reg_errcode_t,
) -> *mut bin_tree_t {
    let mut dfa: *mut re_dfa_t = (*preg).buffer;
    let mut tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut current_block_103: u64;
    match (*token).type_0() as libc::c_int {
        1 => {
            tree = create_token_tree(
                dfa,
                0 as *mut bin_tree_t,
                0 as *mut bin_tree_t,
                token,
            );
            if (tree == 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int
                as libc::c_long != 0
            {
                *err = _REG_ESPACE;
                return 0 as *mut bin_tree_t;
            }
            if (*dfa).mb_cur_max > 1 as libc::c_int {
                while !((*regexp).stop <= (*regexp).cur_idx)
                    && !((*regexp).cur_idx == (*regexp).valid_len
                        || *((*regexp).wcs).offset((*regexp).cur_idx as isize)
                            != 0xffffffff as libc::c_uint)
                {
                    let mut mbc_remain: *mut bin_tree_t = 0 as *mut bin_tree_t;
                    fetch_token(token, regexp, syntax);
                    mbc_remain = create_token_tree(
                        dfa,
                        0 as *mut bin_tree_t,
                        0 as *mut bin_tree_t,
                        token,
                    );
                    tree = create_tree(dfa, tree, mbc_remain, CONCAT);
                    if (mbc_remain.is_null() || tree.is_null()) as libc::c_int
                        as libc::c_long != 0
                    {
                        *err = _REG_ESPACE;
                        return 0 as *mut bin_tree_t;
                    }
                }
            }
            current_block_103 = 3879520548144599102;
        }
        8 => {
            tree = parse_sub_exp(
                regexp,
                preg,
                token,
                syntax,
                nest + 1 as libc::c_int as libc::c_long,
                err,
            );
            if (*err as libc::c_int != _REG_NOERROR as libc::c_int && tree.is_null())
                as libc::c_int as libc::c_long != 0
            {
                return 0 as *mut bin_tree_t;
            }
            current_block_103 = 3879520548144599102;
        }
        20 => {
            tree = parse_bracket_exp(regexp, dfa, token, syntax, err);
            if (*err as libc::c_int != _REG_NOERROR as libc::c_int && tree.is_null())
                as libc::c_int as libc::c_long != 0
            {
                return 0 as *mut bin_tree_t;
            }
            current_block_103 = 3879520548144599102;
        }
        4 => {
            if ((*dfa).completed_bkref_map
                & ((1 as libc::c_int) << (*token).opr.idx) as libc::c_ulong)
                as libc::c_long == 0
            {
                *err = _REG_ESUBREG;
                return 0 as *mut bin_tree_t;
            }
            (*dfa).used_bkref_map
                |= ((1 as libc::c_int) << (*token).opr.idx) as libc::c_ulong;
            tree = create_token_tree(
                dfa,
                0 as *mut bin_tree_t,
                0 as *mut bin_tree_t,
                token,
            );
            if (tree == 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int
                as libc::c_long != 0
            {
                *err = _REG_ESPACE;
                return 0 as *mut bin_tree_t;
            }
            (*dfa).nbackref += 1;
            (*dfa).nbackref;
            (*dfa).set_has_mb_node(1 as libc::c_int as libc::c_uint);
            current_block_103 = 3879520548144599102;
        }
        23 => {
            if syntax
                & ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int != 0
            {
                *err = _REG_BADRPT;
                return 0 as *mut bin_tree_t;
            }
            current_block_103 = 11417783101803375067;
        }
        11 | 18 | 19 => {
            current_block_103 = 11417783101803375067;
        }
        9 => {
            current_block_103 = 8370857348744059481;
        }
        24 => {
            current_block_103 = 12675682207020406977;
        }
        12 => {
            if (*token).opr.ctx_type as libc::c_uint
                & (WORD_DELIM as libc::c_int | NOT_WORD_DELIM as libc::c_int
                    | WORD_FIRST as libc::c_int | WORD_LAST as libc::c_int)
                    as libc::c_uint != 0
                && (*dfa).word_ops_used() as libc::c_int == 0 as libc::c_int
            {
                init_word_char(dfa);
            }
            if (*token).opr.ctx_type as libc::c_uint
                == WORD_DELIM as libc::c_int as libc::c_uint
                || (*token).opr.ctx_type as libc::c_uint
                    == NOT_WORD_DELIM as libc::c_int as libc::c_uint
            {
                let mut tree_first: *mut bin_tree_t = 0 as *mut bin_tree_t;
                let mut tree_last: *mut bin_tree_t = 0 as *mut bin_tree_t;
                if (*token).opr.ctx_type as libc::c_uint
                    == WORD_DELIM as libc::c_int as libc::c_uint
                {
                    (*token).opr.ctx_type = WORD_FIRST;
                    tree_first = create_token_tree(
                        dfa,
                        0 as *mut bin_tree_t,
                        0 as *mut bin_tree_t,
                        token,
                    );
                    (*token).opr.ctx_type = WORD_LAST;
                } else {
                    (*token).opr.ctx_type = INSIDE_WORD;
                    tree_first = create_token_tree(
                        dfa,
                        0 as *mut bin_tree_t,
                        0 as *mut bin_tree_t,
                        token,
                    );
                    (*token).opr.ctx_type = INSIDE_NOTWORD;
                }
                tree_last = create_token_tree(
                    dfa,
                    0 as *mut bin_tree_t,
                    0 as *mut bin_tree_t,
                    token,
                );
                tree = create_tree(dfa, tree_first, tree_last, OP_ALT);
                if (tree_first.is_null() || tree_last.is_null() || tree.is_null())
                    as libc::c_int as libc::c_long != 0
                {
                    *err = _REG_ESPACE;
                    return 0 as *mut bin_tree_t;
                }
            } else {
                tree = create_token_tree(
                    dfa,
                    0 as *mut bin_tree_t,
                    0 as *mut bin_tree_t,
                    token,
                );
                if (tree == 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int
                    as libc::c_long != 0
                {
                    *err = _REG_ESPACE;
                    return 0 as *mut bin_tree_t;
                }
            }
            fetch_token(token, regexp, syntax);
            return tree;
        }
        5 => {
            tree = create_token_tree(
                dfa,
                0 as *mut bin_tree_t,
                0 as *mut bin_tree_t,
                token,
            );
            if (tree == 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int
                as libc::c_long != 0
            {
                *err = _REG_ESPACE;
                return 0 as *mut bin_tree_t;
            }
            if (*dfa).mb_cur_max > 1 as libc::c_int {
                (*dfa).set_has_mb_node(1 as libc::c_int as libc::c_uint);
            }
            current_block_103 = 3879520548144599102;
        }
        32 | 33 => {
            tree = build_charclass_op(
                dfa,
                (*regexp).trans,
                b"alnum\0" as *const u8 as *const libc::c_char,
                b"_\0" as *const u8 as *const libc::c_char,
                (*token).type_0() as libc::c_int == OP_NOTWORD as libc::c_int,
                err,
            );
            if (*err as libc::c_int != _REG_NOERROR as libc::c_int && tree.is_null())
                as libc::c_int as libc::c_long != 0
            {
                return 0 as *mut bin_tree_t;
            }
            current_block_103 = 3879520548144599102;
        }
        34 | 35 => {
            tree = build_charclass_op(
                dfa,
                (*regexp).trans,
                b"space\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
                (*token).type_0() as libc::c_int == OP_NOTSPACE as libc::c_int,
                err,
            );
            if (*err as libc::c_int != _REG_NOERROR as libc::c_int && tree.is_null())
                as libc::c_int as libc::c_long != 0
            {
                return 0 as *mut bin_tree_t;
            }
            current_block_103 = 3879520548144599102;
        }
        10 | 2 => return 0 as *mut bin_tree_t,
        36 => {
            *err = _REG_EESCAPE;
            return 0 as *mut bin_tree_t;
        }
        _ => {
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
            return 0 as *mut bin_tree_t;
        }
    }
    match current_block_103 {
        11417783101803375067 => {
            if syntax
                & (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int != 0
            {
                *err = _REG_BADRPT;
                return 0 as *mut bin_tree_t;
            } else if syntax
                & ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int != 0
            {
                fetch_token(token, regexp, syntax);
                return parse_expression(regexp, preg, token, syntax, nest, err);
            }
            current_block_103 = 8370857348744059481;
        }
        _ => {}
    }
    match current_block_103 {
        8370857348744059481 => {
            if (*token).type_0() as libc::c_int == OP_CLOSE_SUBEXP as libc::c_int
                && syntax
                    & (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int == 0
            {
                *err = _REG_ERPAREN;
                return 0 as *mut bin_tree_t;
            }
            current_block_103 = 12675682207020406977;
        }
        _ => {}
    }
    match current_block_103 {
        12675682207020406977 => {
            (*token).set_type_0(CHARACTER);
            tree = create_token_tree(
                dfa,
                0 as *mut bin_tree_t,
                0 as *mut bin_tree_t,
                token,
            );
            if (tree == 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int
                as libc::c_long != 0
            {
                *err = _REG_ESPACE;
                return 0 as *mut bin_tree_t;
            }
        }
        _ => {}
    }
    fetch_token(token, regexp, syntax);
    while (*token).type_0() as libc::c_int == OP_DUP_ASTERISK as libc::c_int
        || (*token).type_0() as libc::c_int == OP_DUP_PLUS as libc::c_int
        || (*token).type_0() as libc::c_int == OP_DUP_QUESTION as libc::c_int
        || (*token).type_0() as libc::c_int == OP_OPEN_DUP_NUM as libc::c_int
    {
        let mut dup_tree: *mut bin_tree_t = parse_dup_op(
            tree,
            regexp,
            dfa,
            token,
            syntax,
            err,
        );
        if (*err as libc::c_int != _REG_NOERROR as libc::c_int && dup_tree.is_null())
            as libc::c_int as libc::c_long != 0
        {
            if !tree.is_null() {
                postorder(
                    tree,
                    Some(
                        free_tree
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut bin_tree_t,
                            ) -> reg_errcode_t,
                    ),
                    0 as *mut libc::c_void,
                );
            }
            return 0 as *mut bin_tree_t;
        }
        tree = dup_tree;
        if syntax
            & ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int != 0
            && ((*token).type_0() as libc::c_int == OP_DUP_ASTERISK as libc::c_int
                || (*token).type_0() as libc::c_int == OP_OPEN_DUP_NUM as libc::c_int)
        {
            if !tree.is_null() {
                postorder(
                    tree,
                    Some(
                        free_tree
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut bin_tree_t,
                            ) -> reg_errcode_t,
                    ),
                    0 as *mut libc::c_void,
                );
            }
            *err = _REG_BADRPT;
            return 0 as *mut bin_tree_t;
        }
    }
    return tree;
}
unsafe extern "C" fn parse_sub_exp(
    mut regexp: *mut re_string_t,
    mut preg: *mut regex_t,
    mut token: *mut re_token_t,
    mut syntax: reg_syntax_t,
    mut nest: Idx,
    mut err: *mut reg_errcode_t,
) -> *mut bin_tree_t {
    let mut dfa: *mut re_dfa_t = (*preg).buffer;
    let mut tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut cur_nsub: size_t = 0;
    let fresh42 = (*preg).re_nsub;
    (*preg).re_nsub = ((*preg).re_nsub).wrapping_add(1);
    cur_nsub = fresh42;
    fetch_token(
        token,
        regexp,
        syntax
            | (((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int,
    );
    if (*token).type_0() as libc::c_int == OP_CLOSE_SUBEXP as libc::c_int {
        tree = 0 as *mut bin_tree_t;
    } else {
        tree = parse_reg_exp(regexp, preg, token, syntax, nest, err);
        if (*err as libc::c_int == _REG_NOERROR as libc::c_int
            && (*token).type_0() as libc::c_int != OP_CLOSE_SUBEXP as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            if !tree.is_null() {
                postorder(
                    tree,
                    Some(
                        free_tree
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut bin_tree_t,
                            ) -> reg_errcode_t,
                    ),
                    0 as *mut libc::c_void,
                );
            }
            *err = _REG_EPAREN;
        }
        if (*err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return 0 as *mut bin_tree_t;
        }
    }
    if cur_nsub <= ('9' as i32 - '1' as i32) as libc::c_ulong {
        (*dfa).completed_bkref_map |= ((1 as libc::c_int) << cur_nsub) as libc::c_ulong;
    }
    tree = create_tree(dfa, tree, 0 as *mut bin_tree_t, SUBEXP);
    if (tree == 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int as libc::c_long
        != 0
    {
        *err = _REG_ESPACE;
        return 0 as *mut bin_tree_t;
    }
    (*tree).token.opr.idx = cur_nsub as Idx;
    return tree;
}
unsafe extern "C" fn parse_dup_op(
    mut elem: *mut bin_tree_t,
    mut regexp: *mut re_string_t,
    mut dfa: *mut re_dfa_t,
    mut token: *mut re_token_t,
    mut syntax: reg_syntax_t,
    mut err: *mut reg_errcode_t,
) -> *mut bin_tree_t {
    let mut current_block: u64;
    let mut tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut old_tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut i: Idx = 0;
    let mut start: Idx = 0;
    let mut end: Idx = 0;
    let mut start_idx: Idx = (*regexp).cur_idx;
    let mut start_token: re_token_t = *token;
    if (*token).type_0() as libc::c_int == OP_OPEN_DUP_NUM as libc::c_int {
        end = 0 as libc::c_int as Idx;
        start = fetch_number(regexp, token, syntax);
        if start == -(1 as libc::c_int) as libc::c_long {
            if (*token).type_0() as libc::c_int == CHARACTER as libc::c_int
                && (*token).opr.c as libc::c_int == ',' as i32
            {
                start = 0 as libc::c_int as Idx;
            } else {
                *err = _REG_BADBR;
                return 0 as *mut bin_tree_t;
            }
        }
        if (start != -(2 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
            != 0
        {
            end = if (*token).type_0() as libc::c_int == OP_CLOSE_DUP_NUM as libc::c_int
            {
                start
            } else if (*token).type_0() as libc::c_int == CHARACTER as libc::c_int
                && (*token).opr.c as libc::c_int == ',' as i32
            {
                fetch_number(regexp, token, syntax)
            } else {
                -(2 as libc::c_int) as libc::c_long
            };
        }
        if (start == -(2 as libc::c_int) as libc::c_long
            || end == -(2 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
            != 0
        {
            if (syntax
                & (((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int == 0)
                as libc::c_int as libc::c_long != 0
            {
                if (*token).type_0() as libc::c_int == END_OF_RE as libc::c_int {
                    *err = _REG_EBRACE;
                } else {
                    *err = _REG_BADBR;
                }
                return 0 as *mut bin_tree_t;
            }
            (*regexp).cur_idx = start_idx;
            *token = start_token;
            (*token).set_type_0(CHARACTER);
            return elem;
        }
        if (end != -(1 as libc::c_int) as libc::c_long && start > end
            || (*token).type_0() as libc::c_int != OP_CLOSE_DUP_NUM as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            *err = _REG_BADBR;
            return 0 as *mut bin_tree_t;
        }
        if ((0x7fff as libc::c_int as libc::c_long)
            < (if end == -(1 as libc::c_int) as libc::c_long { start } else { end }))
            as libc::c_int as libc::c_long != 0
        {
            *err = _REG_ESIZE;
            return 0 as *mut bin_tree_t;
        }
    } else {
        start = (if (*token).type_0() as libc::c_int == OP_DUP_PLUS as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as Idx;
        end = (if (*token).type_0() as libc::c_int == OP_DUP_QUESTION as libc::c_int {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as Idx;
    }
    fetch_token(token, regexp, syntax);
    if (elem == 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int as libc::c_long
        != 0
    {
        return 0 as *mut bin_tree_t;
    }
    if (start == 0 as libc::c_int as libc::c_long
        && end == 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0
    {
        postorder(
            elem,
            Some(
                free_tree
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut bin_tree_t,
                    ) -> reg_errcode_t,
            ),
            0 as *mut libc::c_void,
        );
        return 0 as *mut bin_tree_t;
    }
    if (start > 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0 {
        tree = elem;
        i = 2 as libc::c_int as Idx;
        loop {
            if !(i <= start) {
                current_block = 4090602189656566074;
                break;
            }
            elem = duplicate_tree(elem, dfa);
            tree = create_tree(dfa, tree, elem, CONCAT);
            if (elem.is_null() || tree.is_null()) as libc::c_int as libc::c_long != 0 {
                current_block = 14134382327168489204;
                break;
            }
            i += 1;
            i;
        }
        match current_block {
            14134382327168489204 => {}
            _ => {
                if start == end {
                    return tree;
                }
                elem = duplicate_tree(elem, dfa);
                if (elem == 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int
                    as libc::c_long != 0
                {
                    current_block = 14134382327168489204;
                } else {
                    old_tree = tree;
                    current_block = 2480299350034459858;
                }
            }
        }
    } else {
        old_tree = 0 as *mut bin_tree_t;
        current_block = 2480299350034459858;
    }
    match current_block {
        2480299350034459858 => {
            if ((*elem).token).type_0() as libc::c_int == SUBEXP as libc::c_int {
                let mut subidx: uintptr_t = (*elem).token.opr.idx as uintptr_t;
                postorder(
                    elem,
                    Some(
                        mark_opt_subexp
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut bin_tree_t,
                            ) -> reg_errcode_t,
                    ),
                    subidx as *mut libc::c_void,
                );
            }
            tree = create_tree(
                dfa,
                elem,
                0 as *mut bin_tree_t,
                (if end == -(1 as libc::c_int) as libc::c_long {
                    OP_DUP_ASTERISK as libc::c_int
                } else {
                    OP_ALT as libc::c_int
                }) as re_token_type_t,
            );
            if !((tree == 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int
                as libc::c_long != 0)
            {
                if !((0 as libc::c_int as Idx) < -(1 as libc::c_int) as Idx)
                    || end != -(1 as libc::c_int) as libc::c_long
                {
                    i = start + 2 as libc::c_int as libc::c_long;
                    loop {
                        if !(i <= end) {
                            current_block = 1854459640724737493;
                            break;
                        }
                        elem = duplicate_tree(elem, dfa);
                        tree = create_tree(dfa, tree, elem, CONCAT);
                        if (elem.is_null() || tree.is_null()) as libc::c_int
                            as libc::c_long != 0
                        {
                            current_block = 14134382327168489204;
                            break;
                        }
                        tree = create_tree(dfa, tree, 0 as *mut bin_tree_t, OP_ALT);
                        if (tree == 0 as *mut libc::c_void as *mut bin_tree_t)
                            as libc::c_int as libc::c_long != 0
                        {
                            current_block = 14134382327168489204;
                            break;
                        }
                        i += 1;
                        i;
                    }
                } else {
                    current_block = 1854459640724737493;
                }
                match current_block {
                    14134382327168489204 => {}
                    _ => {
                        if !old_tree.is_null() {
                            tree = create_tree(dfa, old_tree, tree, CONCAT);
                        }
                        return tree;
                    }
                }
            }
        }
        _ => {}
    }
    *err = _REG_ESPACE;
    return 0 as *mut bin_tree_t;
}
unsafe extern "C" fn parse_byte(
    mut b: libc::c_uchar,
    mut dfa: *const re_dfa_t,
) -> wint_t {
    return if (*dfa).mb_cur_max > 1 as libc::c_int {
        btowc(b as libc::c_int)
    } else {
        b as libc::c_uint
    };
}
unsafe extern "C" fn build_range_exp(
    mut sbcset: *mut bitset_word_t,
    mut mbcset: *mut re_charset_t,
    mut range_alloc: *mut Idx,
    mut start_elem: *mut bracket_elem_t,
    mut end_elem: *mut bracket_elem_t,
    mut dfa: *mut re_dfa_t,
    mut syntax: reg_syntax_t,
    mut nrules: uint_fast32_t,
    mut collseqmb: *const libc::c_uchar,
    mut collseqwc: *const libc::c_char,
    mut table_size: int_fast32_t,
    mut symb_table: *const libc::c_void,
    mut extra: *const libc::c_uchar,
) -> reg_errcode_t {
    if ((*start_elem).type_0 as libc::c_uint
        == EQUIV_CLASS as libc::c_int as libc::c_uint
        || (*start_elem).type_0 as libc::c_uint
            == CHAR_CLASS as libc::c_int as libc::c_uint
        || (*end_elem).type_0 as libc::c_uint
            == EQUIV_CLASS as libc::c_int as libc::c_uint
        || (*end_elem).type_0 as libc::c_uint
            == CHAR_CLASS as libc::c_int as libc::c_uint) as libc::c_int as libc::c_long
        != 0
    {
        return _REG_ERANGE;
    }
    if ((*start_elem).type_0 as libc::c_uint == COLL_SYM as libc::c_int as libc::c_uint
        && strlen((*start_elem).opr.name as *mut libc::c_char)
            > 1 as libc::c_int as libc::c_ulong
        || (*end_elem).type_0 as libc::c_uint == COLL_SYM as libc::c_int as libc::c_uint
            && strlen((*end_elem).opr.name as *mut libc::c_char)
                > 1 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long != 0
    {
        return _REG_ECOLLATE;
    }
    let mut start_ch: libc::c_uint = (if (*start_elem).type_0 as libc::c_uint
        == SB_CHAR as libc::c_int as libc::c_uint
    {
        (*start_elem).opr.ch as libc::c_int
    } else if (*start_elem).type_0 as libc::c_uint
        == COLL_SYM as libc::c_int as libc::c_uint
    {
        *((*start_elem).opr.name).offset(0 as libc::c_int as isize) as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_uint;
    let mut end_ch: libc::c_uint = (if (*end_elem).type_0 as libc::c_uint
        == SB_CHAR as libc::c_int as libc::c_uint
    {
        (*end_elem).opr.ch as libc::c_int
    } else if (*end_elem).type_0 as libc::c_uint
        == COLL_SYM as libc::c_int as libc::c_uint
    {
        *((*end_elem).opr.name).offset(0 as libc::c_int as isize) as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_uint;
    let mut start_wc: wint_t = if (*start_elem).type_0 as libc::c_uint
        == SB_CHAR as libc::c_int as libc::c_uint
        || (*start_elem).type_0 as libc::c_uint
            == COLL_SYM as libc::c_int as libc::c_uint
    {
        parse_byte(start_ch as libc::c_uchar, dfa)
    } else {
        (*start_elem).opr.wch as libc::c_uint
    };
    let mut end_wc: wint_t = if (*end_elem).type_0 as libc::c_uint
        == SB_CHAR as libc::c_int as libc::c_uint
        || (*end_elem).type_0 as libc::c_uint == COLL_SYM as libc::c_int as libc::c_uint
    {
        parse_byte(end_ch as libc::c_uchar, dfa)
    } else {
        (*end_elem).opr.wch as libc::c_uint
    };
    if start_wc == 0xffffffff as libc::c_uint || end_wc == 0xffffffff as libc::c_uint {
        return _REG_ECOLLATE
    } else if (syntax
        & ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int != 0
        && start_wc > end_wc) as libc::c_int as libc::c_long != 0
    {
        return _REG_ERANGE
    }
    if (*dfa).mb_cur_max > 1 as libc::c_int {
        if (*range_alloc == (*mbcset).nranges) as libc::c_int as libc::c_long != 0 {
            let mut new_array_start: *mut wchar_t = 0 as *mut wchar_t;
            let mut new_array_end: *mut wchar_t = 0 as *mut wchar_t;
            let mut new_nranges: Idx = 0;
            new_nranges = 2 as libc::c_int as libc::c_long * (*mbcset).nranges
                + 1 as libc::c_int as libc::c_long;
            new_array_start = realloc(
                (*mbcset).range_starts as *mut libc::c_void,
                (new_nranges as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<wchar_t>() as libc::c_ulong),
            ) as *mut wchar_t;
            new_array_end = realloc(
                (*mbcset).range_ends as *mut libc::c_void,
                (new_nranges as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<wchar_t>() as libc::c_ulong),
            ) as *mut wchar_t;
            if (new_array_start.is_null() || new_array_end.is_null()) as libc::c_int
                as libc::c_long != 0
            {
                rpl_free(new_array_start as *mut libc::c_void);
                rpl_free(new_array_end as *mut libc::c_void);
                return _REG_ESPACE;
            }
            (*mbcset).range_starts = new_array_start;
            (*mbcset).range_ends = new_array_end;
            *range_alloc = new_nranges;
        }
        *((*mbcset).range_starts)
            .offset((*mbcset).nranges as isize) = start_wc as wchar_t;
        let fresh43 = (*mbcset).nranges;
        (*mbcset).nranges = (*mbcset).nranges + 1;
        *((*mbcset).range_ends).offset(fresh43 as isize) = end_wc as wchar_t;
    }
    let mut wc: wchar_t = 0 as libc::c_int;
    while wc
        < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
    {
        if start_wc <= wc as libc::c_uint && wc as libc::c_uint <= end_wc {
            bitset_set(sbcset, wc as Idx);
        }
        wc += 1;
        wc;
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn build_collating_symbol(
    mut sbcset: *mut bitset_word_t,
    mut mbcset: *mut re_charset_t,
    mut coll_sym_alloc: *mut Idx,
    mut name: *const libc::c_uchar,
    mut nrules: uint_fast32_t,
    mut table_size: int_fast32_t,
    mut symb_table: *const libc::c_void,
    mut extra: *const libc::c_uchar,
) -> reg_errcode_t {
    let mut name_len: size_t = strlen(name as *const libc::c_char);
    if (name_len != 1 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long
        != 0
    {
        return _REG_ECOLLATE
    } else {
        bitset_set(sbcset, *name.offset(0 as libc::c_int as isize) as Idx);
        return _REG_NOERROR;
    };
}
unsafe extern "C" fn parse_bracket_exp(
    mut regexp: *mut re_string_t,
    mut dfa: *mut re_dfa_t,
    mut token: *mut re_token_t,
    mut syntax: reg_syntax_t,
    mut err: *mut reg_errcode_t,
) -> *mut bin_tree_t {
    let mut current_block: u64;
    let mut collseqmb: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut collseqwc: *const libc::c_char = 0 as *const libc::c_char;
    let mut nrules: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    let mut table_size: int_fast32_t = 0 as libc::c_int as int_fast32_t;
    let mut symb_table: *const libc::c_void = 0 as *const libc::c_void;
    let mut extra: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut br_token: re_token_t = re_token_t {
        opr: C2RustUnnamed { c: 0 },
        type_0_constraint_duplicated_opt_subexp_accept_mb_mb_partial_word_char: [0; 3],
        c2rust_padding: [0; 5],
    };
    let mut sbcset: re_bitset_ptr_t = 0 as *mut bitset_word_t;
    let mut mbcset: *mut re_charset_t = 0 as *mut re_charset_t;
    let mut coll_sym_alloc: Idx = 0 as libc::c_int as Idx;
    let mut range_alloc: Idx = 0 as libc::c_int as Idx;
    let mut mbchar_alloc: Idx = 0 as libc::c_int as Idx;
    let mut equiv_class_alloc: Idx = 0 as libc::c_int as Idx;
    let mut char_class_alloc: Idx = 0 as libc::c_int as Idx;
    let mut non_match: bool = 0 as libc::c_int != 0;
    let mut work_tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut token_len: libc::c_int = 0;
    let mut first_round: bool = 1 as libc::c_int != 0;
    sbcset = calloc(
        ::core::mem::size_of::<bitset_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as re_bitset_ptr_t;
    mbcset = calloc(
        ::core::mem::size_of::<re_charset_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut re_charset_t;
    if (sbcset.is_null() || mbcset.is_null()) as libc::c_int as libc::c_long != 0 {
        rpl_free(sbcset as *mut libc::c_void);
        rpl_free(mbcset as *mut libc::c_void);
        *err = _REG_ESPACE;
        return 0 as *mut bin_tree_t;
    }
    token_len = peek_token_bracket(token, regexp, syntax);
    if ((*token).type_0() as libc::c_int == END_OF_RE as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        *err = _REG_BADPAT;
    } else {
        if (*token).type_0() as libc::c_int == OP_NON_MATCH_LIST as libc::c_int {
            (*mbcset).set_non_match(1 as libc::c_int as libc::c_uint);
            non_match = 1 as libc::c_int != 0;
            if syntax
                & ((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int != 0
            {
                bitset_set(sbcset, '\n' as i32 as Idx);
            }
            (*regexp).cur_idx += token_len as libc::c_long;
            token_len = peek_token_bracket(token, regexp, syntax);
            if ((*token).type_0() as libc::c_int == END_OF_RE as libc::c_int)
                as libc::c_int as libc::c_long != 0
            {
                *err = _REG_BADPAT;
                current_block = 856757234279063558;
            } else {
                current_block = 12147880666119273379;
            }
        } else {
            current_block = 12147880666119273379;
        }
        match current_block {
            856757234279063558 => {}
            _ => {
                if (*token).type_0() as libc::c_int == OP_CLOSE_BRACKET as libc::c_int {
                    (*token).set_type_0(CHARACTER);
                }
                loop {
                    let mut start_elem: bracket_elem_t = bracket_elem_t {
                        type_0: SB_CHAR,
                        opr: C2RustUnnamed_1 { ch: 0 },
                    };
                    let mut end_elem: bracket_elem_t = bracket_elem_t {
                        type_0: SB_CHAR,
                        opr: C2RustUnnamed_1 { ch: 0 },
                    };
                    let mut start_name_buf: [libc::c_uchar; 32] = [0; 32];
                    let mut end_name_buf: [libc::c_uchar; 32] = [0; 32];
                    let mut ret: reg_errcode_t = _REG_NOERROR;
                    let mut token_len2: libc::c_int = 0 as libc::c_int;
                    let mut is_range_exp: bool = 0 as libc::c_int != 0;
                    let mut token2: re_token_t = re_token_t {
                        opr: C2RustUnnamed { c: 0 },
                        type_0_constraint_duplicated_opt_subexp_accept_mb_mb_partial_word_char: [0; 3],
                        c2rust_padding: [0; 5],
                    };
                    start_elem.opr.name = start_name_buf.as_mut_ptr();
                    start_elem.type_0 = COLL_SYM;
                    ret = parse_bracket_element(
                        &mut start_elem,
                        regexp,
                        token,
                        token_len,
                        dfa,
                        syntax,
                        first_round,
                    );
                    if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        *err = ret;
                        current_block = 856757234279063558;
                        break;
                    } else {
                        first_round = 0 as libc::c_int != 0;
                        token_len = peek_token_bracket(token, regexp, syntax);
                        if start_elem.type_0 as libc::c_uint
                            != CHAR_CLASS as libc::c_int as libc::c_uint
                            && start_elem.type_0 as libc::c_uint
                                != EQUIV_CLASS as libc::c_int as libc::c_uint
                        {
                            if ((*token).type_0() as libc::c_int
                                == END_OF_RE as libc::c_int) as libc::c_int as libc::c_long
                                != 0
                            {
                                *err = _REG_EBRACK;
                                current_block = 856757234279063558;
                                break;
                            } else if (*token).type_0() as libc::c_int
                                == OP_CHARSET_RANGE as libc::c_int
                            {
                                (*regexp).cur_idx += token_len as libc::c_long;
                                token_len2 = peek_token_bracket(
                                    &mut token2,
                                    regexp,
                                    syntax,
                                );
                                if (token2.type_0() as libc::c_int
                                    == END_OF_RE as libc::c_int) as libc::c_int as libc::c_long
                                    != 0
                                {
                                    *err = _REG_EBRACK;
                                    current_block = 856757234279063558;
                                    break;
                                } else if token2.type_0() as libc::c_int
                                    == OP_CLOSE_BRACKET as libc::c_int
                                {
                                    (*regexp).cur_idx += -token_len as libc::c_long;
                                    (*token).set_type_0(CHARACTER);
                                } else {
                                    is_range_exp = 1 as libc::c_int != 0;
                                }
                            }
                        }
                        if is_range_exp as libc::c_int == 1 as libc::c_int {
                            end_elem.opr.name = end_name_buf.as_mut_ptr();
                            end_elem.type_0 = COLL_SYM;
                            ret = parse_bracket_element(
                                &mut end_elem,
                                regexp,
                                &mut token2,
                                token_len2,
                                dfa,
                                syntax,
                                1 as libc::c_int != 0,
                            );
                            if (ret as libc::c_int != _REG_NOERROR as libc::c_int)
                                as libc::c_int as libc::c_long != 0
                            {
                                *err = ret;
                                current_block = 856757234279063558;
                                break;
                            } else {
                                token_len = peek_token_bracket(token, regexp, syntax);
                                *err = build_range_exp(
                                    sbcset,
                                    mbcset,
                                    &mut range_alloc,
                                    &mut start_elem,
                                    &mut end_elem,
                                    dfa,
                                    syntax,
                                    nrules,
                                    collseqmb,
                                    collseqwc,
                                    table_size,
                                    symb_table,
                                    extra,
                                );
                                if (*err as libc::c_int != _REG_NOERROR as libc::c_int)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    current_block = 856757234279063558;
                                    break;
                                }
                            }
                        } else {
                            match start_elem.type_0 as libc::c_uint {
                                0 => {
                                    bitset_set(sbcset, start_elem.opr.ch as Idx);
                                }
                                1 => {
                                    if (mbchar_alloc == (*mbcset).nmbchars) as libc::c_int
                                        as libc::c_long != 0
                                    {
                                        let mut new_mbchars: *mut wchar_t = 0 as *mut wchar_t;
                                        mbchar_alloc = 2 as libc::c_int as libc::c_long
                                            * (*mbcset).nmbchars + 1 as libc::c_int as libc::c_long;
                                        new_mbchars = realloc(
                                            (*mbcset).mbchars as *mut libc::c_void,
                                            (mbchar_alloc as libc::c_ulong)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<wchar_t>() as libc::c_ulong,
                                                ),
                                        ) as *mut wchar_t;
                                        if (new_mbchars == 0 as *mut libc::c_void as *mut wchar_t)
                                            as libc::c_int as libc::c_long != 0
                                        {
                                            current_block = 5046467386909730888;
                                            break;
                                        }
                                        (*mbcset).mbchars = new_mbchars;
                                    }
                                    let fresh44 = (*mbcset).nmbchars;
                                    (*mbcset).nmbchars = (*mbcset).nmbchars + 1;
                                    *((*mbcset).mbchars)
                                        .offset(fresh44 as isize) = start_elem.opr.wch;
                                }
                                2 => {
                                    *err = build_equiv_class(
                                        sbcset,
                                        mbcset,
                                        &mut equiv_class_alloc,
                                        start_elem.opr.name,
                                    );
                                    if (*err as libc::c_int != _REG_NOERROR as libc::c_int)
                                        as libc::c_int as libc::c_long != 0
                                    {
                                        current_block = 856757234279063558;
                                        break;
                                    }
                                }
                                3 => {
                                    *err = build_collating_symbol(
                                        sbcset,
                                        mbcset,
                                        &mut coll_sym_alloc,
                                        start_elem.opr.name,
                                        nrules,
                                        table_size,
                                        symb_table,
                                        extra,
                                    );
                                    if (*err as libc::c_int != _REG_NOERROR as libc::c_int)
                                        as libc::c_int as libc::c_long != 0
                                    {
                                        current_block = 856757234279063558;
                                        break;
                                    }
                                }
                                4 => {
                                    *err = build_charclass(
                                        (*regexp).trans,
                                        sbcset,
                                        mbcset,
                                        &mut char_class_alloc,
                                        start_elem.opr.name as *const libc::c_char,
                                        syntax,
                                    );
                                    if (*err as libc::c_int != _REG_NOERROR as libc::c_int)
                                        as libc::c_int as libc::c_long != 0
                                    {
                                        current_block = 856757234279063558;
                                        break;
                                    }
                                }
                                _ => {
                                    if 0 as libc::c_int != 0 {} else {
                                        unreachable!();
                                    };
                                }
                            }
                        }
                        if ((*token).type_0() as libc::c_int == END_OF_RE as libc::c_int)
                            as libc::c_int as libc::c_long != 0
                        {
                            *err = _REG_EBRACK;
                            current_block = 856757234279063558;
                            break;
                        } else if (*token).type_0() as libc::c_int
                            == OP_CLOSE_BRACKET as libc::c_int
                        {
                            current_block = 7419121793134201633;
                            break;
                        }
                    }
                }
                match current_block {
                    856757234279063558 => {}
                    _ => {
                        match current_block {
                            7419121793134201633 => {
                                (*regexp).cur_idx += token_len as libc::c_long;
                                if non_match {
                                    bitset_not(sbcset);
                                }
                                if (*dfa).mb_cur_max > 1 as libc::c_int {
                                    bitset_mask(sbcset, (*dfa).sb_char as *const bitset_word_t);
                                }
                                if (*mbcset).nmbchars != 0 || (*mbcset).ncoll_syms != 0
                                    || (*mbcset).nequiv_classes != 0 || (*mbcset).nranges != 0
                                    || (*dfa).mb_cur_max > 1 as libc::c_int
                                        && ((*mbcset).nchar_classes != 0
                                            || (*mbcset).non_match() as libc::c_int != 0)
                                {
                                    let mut mbc_tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
                                    let mut sbc_idx: libc::c_int = 0;
                                    (*dfa).set_has_mb_node(1 as libc::c_int as libc::c_uint);
                                    br_token.set_type_0(COMPLEX_BRACKET);
                                    br_token.opr.mbcset = mbcset;
                                    mbc_tree = create_token_tree(
                                        dfa,
                                        0 as *mut bin_tree_t,
                                        0 as *mut bin_tree_t,
                                        &mut br_token,
                                    );
                                    if (mbc_tree == 0 as *mut libc::c_void as *mut bin_tree_t)
                                        as libc::c_int as libc::c_long != 0
                                    {
                                        current_block = 5046467386909730888;
                                    } else {
                                        sbc_idx = 0 as libc::c_int;
                                        while sbc_idx
                                            < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                                + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                                                / 64 as libc::c_int
                                        {
                                            if *sbcset.offset(sbc_idx as isize) != 0 {
                                                break;
                                            }
                                            sbc_idx += 1;
                                            sbc_idx;
                                        }
                                        if sbc_idx
                                            < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                                + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                                                / 64 as libc::c_int
                                        {
                                            br_token.set_type_0(SIMPLE_BRACKET);
                                            br_token.opr.sbcset = sbcset;
                                            work_tree = create_token_tree(
                                                dfa,
                                                0 as *mut bin_tree_t,
                                                0 as *mut bin_tree_t,
                                                &mut br_token,
                                            );
                                            if (work_tree == 0 as *mut libc::c_void as *mut bin_tree_t)
                                                as libc::c_int as libc::c_long != 0
                                            {
                                                current_block = 5046467386909730888;
                                            } else {
                                                work_tree = create_tree(dfa, work_tree, mbc_tree, OP_ALT);
                                                if (work_tree == 0 as *mut libc::c_void as *mut bin_tree_t)
                                                    as libc::c_int as libc::c_long != 0
                                                {
                                                    current_block = 5046467386909730888;
                                                } else {
                                                    current_block = 17958840340921835115;
                                                }
                                            }
                                        } else {
                                            rpl_free(sbcset as *mut libc::c_void);
                                            work_tree = mbc_tree;
                                            current_block = 17958840340921835115;
                                        }
                                    }
                                } else {
                                    free_charset(mbcset);
                                    br_token.set_type_0(SIMPLE_BRACKET);
                                    br_token.opr.sbcset = sbcset;
                                    work_tree = create_token_tree(
                                        dfa,
                                        0 as *mut bin_tree_t,
                                        0 as *mut bin_tree_t,
                                        &mut br_token,
                                    );
                                    if (work_tree == 0 as *mut libc::c_void as *mut bin_tree_t)
                                        as libc::c_int as libc::c_long != 0
                                    {
                                        current_block = 5046467386909730888;
                                    } else {
                                        current_block = 17958840340921835115;
                                    }
                                }
                                match current_block {
                                    5046467386909730888 => {}
                                    _ => return work_tree,
                                }
                            }
                            _ => {}
                        }
                        *err = _REG_ESPACE;
                    }
                }
            }
        }
    }
    rpl_free(sbcset as *mut libc::c_void);
    free_charset(mbcset);
    return 0 as *mut bin_tree_t;
}
unsafe extern "C" fn parse_bracket_element(
    mut elem: *mut bracket_elem_t,
    mut regexp: *mut re_string_t,
    mut token: *mut re_token_t,
    mut token_len: libc::c_int,
    mut dfa: *mut re_dfa_t,
    mut syntax: reg_syntax_t,
    mut accept_hyphen: bool,
) -> reg_errcode_t {
    let mut cur_char_size: libc::c_int = 0;
    cur_char_size = re_string_char_size_at(regexp, (*regexp).cur_idx);
    if cur_char_size > 1 as libc::c_int {
        (*elem).type_0 = MB_CHAR;
        (*elem).opr.wch = re_string_wchar_at(regexp, (*regexp).cur_idx) as wchar_t;
        (*regexp).cur_idx += cur_char_size as libc::c_long;
        return _REG_NOERROR;
    }
    (*regexp).cur_idx += token_len as libc::c_long;
    if (*token).type_0() as libc::c_int == OP_OPEN_COLL_ELEM as libc::c_int
        || (*token).type_0() as libc::c_int == OP_OPEN_CHAR_CLASS as libc::c_int
        || (*token).type_0() as libc::c_int == OP_OPEN_EQUIV_CLASS as libc::c_int
    {
        return parse_bracket_symbol(elem, regexp, token);
    }
    if ((*token).type_0() as libc::c_int == OP_CHARSET_RANGE as libc::c_int)
        as libc::c_int as libc::c_long != 0 && !accept_hyphen
    {
        let mut token2: re_token_t = re_token_t {
            opr: C2RustUnnamed { c: 0 },
            type_0_constraint_duplicated_opt_subexp_accept_mb_mb_partial_word_char: [0; 3],
            c2rust_padding: [0; 5],
        };
        peek_token_bracket(&mut token2, regexp, syntax);
        if token2.type_0() as libc::c_int != OP_CLOSE_BRACKET as libc::c_int {
            return _REG_ERANGE;
        }
    }
    (*elem).type_0 = SB_CHAR;
    (*elem).opr.ch = (*token).opr.c;
    return _REG_NOERROR;
}
unsafe extern "C" fn parse_bracket_symbol(
    mut elem: *mut bracket_elem_t,
    mut regexp: *mut re_string_t,
    mut token: *mut re_token_t,
) -> reg_errcode_t {
    let mut ch: libc::c_uchar = 0;
    let mut delim: libc::c_uchar = (*token).opr.c;
    let mut i: libc::c_int = 0 as libc::c_int;
    if (*regexp).stop <= (*regexp).cur_idx {
        return _REG_EBRACK;
    }
    loop {
        if i >= 32 as libc::c_int {
            return _REG_EBRACK;
        }
        if (*token).type_0() as libc::c_int == OP_OPEN_CHAR_CLASS as libc::c_int {
            ch = re_string_fetch_byte_case(regexp);
        } else {
            let fresh45 = (*regexp).cur_idx;
            (*regexp).cur_idx = (*regexp).cur_idx + 1;
            ch = *((*regexp).mbs).offset(fresh45 as isize);
        }
        if (*regexp).stop <= (*regexp).cur_idx {
            return _REG_EBRACK;
        }
        if ch as libc::c_int == delim as libc::c_int
            && *((*regexp).mbs)
                .offset(((*regexp).cur_idx + 0 as libc::c_int as libc::c_long) as isize)
                as libc::c_int == ']' as i32
        {
            break;
        }
        *((*elem).opr.name).offset(i as isize) = ch;
        i += 1;
        i;
    }
    (*regexp).cur_idx += 1 as libc::c_int as libc::c_long;
    *((*elem).opr.name).offset(i as isize) = '\0' as i32 as libc::c_uchar;
    match (*token).type_0() as libc::c_int {
        26 => {
            (*elem).type_0 = COLL_SYM;
        }
        28 => {
            (*elem).type_0 = EQUIV_CLASS;
        }
        30 => {
            (*elem).type_0 = CHAR_CLASS;
        }
        _ => {}
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn build_equiv_class(
    mut sbcset: *mut bitset_word_t,
    mut mbcset: *mut re_charset_t,
    mut equiv_class_alloc: *mut Idx,
    mut name: *const libc::c_uchar,
) -> reg_errcode_t {
    if (strlen(name as *const libc::c_char) != 1 as libc::c_int as libc::c_ulong)
        as libc::c_int as libc::c_long != 0
    {
        return _REG_ECOLLATE;
    }
    bitset_set(sbcset, *name as Idx);
    return _REG_NOERROR;
}
unsafe extern "C" fn build_charclass(
    mut trans: *mut libc::c_uchar,
    mut sbcset: *mut bitset_word_t,
    mut mbcset: *mut re_charset_t,
    mut char_class_alloc: *mut Idx,
    mut class_name: *const libc::c_char,
    mut syntax: reg_syntax_t,
) -> reg_errcode_t {
    let mut i: libc::c_int = 0;
    let mut name: *const libc::c_char = class_name;
    if syntax
        & ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int != 0
        && (strcmp(name, b"upper\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(name, b"lower\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int)
    {
        name = b"alpha\0" as *const u8 as *const libc::c_char;
    }
    if (*char_class_alloc == (*mbcset).nchar_classes) as libc::c_int as libc::c_long != 0
    {
        let mut new_char_class_alloc: Idx = 2 as libc::c_int as libc::c_long
            * (*mbcset).nchar_classes + 1 as libc::c_int as libc::c_long;
        let mut new_char_classes: *mut wctype_t = realloc(
            (*mbcset).char_classes as *mut libc::c_void,
            (new_char_class_alloc as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<wctype_t>() as libc::c_ulong),
        ) as *mut wctype_t;
        if (new_char_classes == 0 as *mut libc::c_void as *mut wctype_t) as libc::c_int
            as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        (*mbcset).char_classes = new_char_classes;
        *char_class_alloc = new_char_class_alloc;
    }
    let fresh46 = (*mbcset).nchar_classes;
    (*mbcset).nchar_classes = (*mbcset).nchar_classes + 1;
    *((*mbcset).char_classes).offset(fresh46 as isize) = wctype(name);
    if strcmp(name, b"alnum\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if (trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, *trans.offset(i as isize) as Idx);
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, i as Idx);
                }
                i += 1;
                i;
            }
        }
    } else if strcmp(name, b"cntrl\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if (trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, *trans.offset(i as isize) as Idx);
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, i as Idx);
                }
                i += 1;
                i;
            }
        }
    } else if strcmp(name, b"lower\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if (trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, *trans.offset(i as isize) as Idx);
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, i as Idx);
                }
                i += 1;
                i;
            }
        }
    } else if strcmp(name, b"space\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if (trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, *trans.offset(i as isize) as Idx);
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, i as Idx);
                }
                i += 1;
                i;
            }
        }
    } else if strcmp(name, b"alpha\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if (trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, *trans.offset(i as isize) as Idx);
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, i as Idx);
                }
                i += 1;
                i;
            }
        }
    } else if strcmp(name, b"digit\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if (trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, *trans.offset(i as isize) as Idx);
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, i as Idx);
                }
                i += 1;
                i;
            }
        }
    } else if strcmp(name, b"print\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if (trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, *trans.offset(i as isize) as Idx);
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, i as Idx);
                }
                i += 1;
                i;
            }
        }
    } else if strcmp(name, b"upper\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if (trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, *trans.offset(i as isize) as Idx);
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, i as Idx);
                }
                i += 1;
                i;
            }
        }
    } else if strcmp(name, b"blank\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if (trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, *trans.offset(i as isize) as Idx);
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, i as Idx);
                }
                i += 1;
                i;
            }
        }
    } else if strcmp(name, b"graph\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if (trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, *trans.offset(i as isize) as Idx);
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, i as Idx);
                }
                i += 1;
                i;
            }
        }
    } else if strcmp(name, b"punct\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if (trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, *trans.offset(i as isize) as Idx);
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, i as Idx);
                }
                i += 1;
                i;
            }
        }
    } else if strcmp(name, b"xdigit\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if (trans != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
            as libc::c_long != 0
        {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, *trans.offset(i as isize) as Idx);
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i
                < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int
            {
                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    bitset_set(sbcset, i as Idx);
                }
                i += 1;
                i;
            }
        }
    } else {
        return _REG_ECTYPE
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn build_charclass_op(
    mut dfa: *mut re_dfa_t,
    mut trans: *mut libc::c_uchar,
    mut class_name: *const libc::c_char,
    mut extra: *const libc::c_char,
    mut non_match: bool,
    mut err: *mut reg_errcode_t,
) -> *mut bin_tree_t {
    let mut sbcset: re_bitset_ptr_t = 0 as *mut bitset_word_t;
    let mut mbcset: *mut re_charset_t = 0 as *mut re_charset_t;
    let mut alloc: Idx = 0 as libc::c_int as Idx;
    let mut ret: reg_errcode_t = _REG_NOERROR;
    let mut tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
    sbcset = calloc(
        ::core::mem::size_of::<bitset_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as re_bitset_ptr_t;
    if (sbcset == 0 as *mut libc::c_void as re_bitset_ptr_t) as libc::c_int
        as libc::c_long != 0
    {
        *err = _REG_ESPACE;
        return 0 as *mut bin_tree_t;
    }
    mbcset = calloc(
        ::core::mem::size_of::<re_charset_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut re_charset_t;
    if (mbcset == 0 as *mut libc::c_void as *mut re_charset_t) as libc::c_int
        as libc::c_long != 0
    {
        rpl_free(sbcset as *mut libc::c_void);
        *err = _REG_ESPACE;
        return 0 as *mut bin_tree_t;
    }
    (*mbcset).set_non_match(non_match as libc::c_uint);
    ret = build_charclass(
        trans,
        sbcset,
        mbcset,
        &mut alloc,
        class_name,
        0 as libc::c_int as reg_syntax_t,
    );
    if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        rpl_free(sbcset as *mut libc::c_void);
        free_charset(mbcset);
        *err = ret;
        return 0 as *mut bin_tree_t;
    }
    while *extra != 0 {
        bitset_set(sbcset, *extra as Idx);
        extra = extra.offset(1);
        extra;
    }
    if non_match {
        bitset_not(sbcset);
    }
    if (*dfa).mb_cur_max > 1 as libc::c_int {
        bitset_mask(sbcset, (*dfa).sb_char as *const bitset_word_t);
    }
    let mut br_token: re_token_t = {
        let mut init = re_token_t {
            type_0_constraint_duplicated_opt_subexp_accept_mb_mb_partial_word_char: [0; 3],
            c2rust_padding: [0; 5],
            opr: C2RustUnnamed { sbcset: sbcset },
        };
        init.set_type_0(SIMPLE_BRACKET);
        init.set_constraint(0);
        init.set_duplicated(0);
        init.set_opt_subexp(0);
        init.set_accept_mb(0);
        init.set_mb_partial(0);
        init.set_word_char(0);
        init
    };
    tree = create_token_tree(
        dfa,
        0 as *mut bin_tree_t,
        0 as *mut bin_tree_t,
        &mut br_token,
    );
    if !((tree == 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int
        as libc::c_long != 0)
    {
        if (*dfa).mb_cur_max > 1 as libc::c_int {
            let mut mbc_tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
            br_token.set_type_0(COMPLEX_BRACKET);
            br_token.opr.mbcset = mbcset;
            (*dfa).set_has_mb_node(1 as libc::c_int as libc::c_uint);
            mbc_tree = create_token_tree(
                dfa,
                0 as *mut bin_tree_t,
                0 as *mut bin_tree_t,
                &mut br_token,
            );
            if !((mbc_tree == 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int
                as libc::c_long != 0)
            {
                tree = create_tree(dfa, tree, mbc_tree, OP_ALT);
                if (mbc_tree != 0 as *mut libc::c_void as *mut bin_tree_t) as libc::c_int
                    as libc::c_long != 0
                {
                    return tree;
                }
            }
        } else {
            free_charset(mbcset);
            return tree;
        }
    }
    rpl_free(sbcset as *mut libc::c_void);
    free_charset(mbcset);
    *err = _REG_ESPACE;
    return 0 as *mut bin_tree_t;
}
unsafe extern "C" fn fetch_number(
    mut input: *mut re_string_t,
    mut token: *mut re_token_t,
    mut syntax: reg_syntax_t,
) -> Idx {
    let mut num: Idx = -(1 as libc::c_int) as Idx;
    let mut c: libc::c_uchar = 0;
    loop {
        fetch_token(token, input, syntax);
        c = (*token).opr.c;
        if ((*token).type_0() as libc::c_int == END_OF_RE as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return -(2 as libc::c_int) as Idx;
        }
        if (*token).type_0() as libc::c_int == OP_CLOSE_DUP_NUM as libc::c_int
            || c as libc::c_int == ',' as i32
        {
            break;
        }
        num = if (*token).type_0() as libc::c_int != CHARACTER as libc::c_int
            || (c as libc::c_int) < '0' as i32 || ('9' as i32) < c as libc::c_int
            || num == -(2 as libc::c_int) as libc::c_long
        {
            -(2 as libc::c_int) as libc::c_long
        } else if num == -(1 as libc::c_int) as libc::c_long {
            (c as libc::c_int - '0' as i32) as libc::c_long
        } else if ((0x7fff as libc::c_int + 1 as libc::c_int) as libc::c_long)
            < num * 10 as libc::c_int as libc::c_long + c as libc::c_long
                - '0' as i32 as libc::c_long
        {
            (0x7fff as libc::c_int + 1 as libc::c_int) as libc::c_long
        } else {
            num * 10 as libc::c_int as libc::c_long + c as libc::c_long
                - '0' as i32 as libc::c_long
        };
    }
    return num;
}
unsafe extern "C" fn free_charset(mut cset: *mut re_charset_t) {
    rpl_free((*cset).mbchars as *mut libc::c_void);
    rpl_free((*cset).range_starts as *mut libc::c_void);
    rpl_free((*cset).range_ends as *mut libc::c_void);
    rpl_free((*cset).char_classes as *mut libc::c_void);
    rpl_free(cset as *mut libc::c_void);
}
unsafe extern "C" fn create_tree(
    mut dfa: *mut re_dfa_t,
    mut left: *mut bin_tree_t,
    mut right: *mut bin_tree_t,
    mut type_0: re_token_type_t,
) -> *mut bin_tree_t {
    let mut t: re_token_t = {
        let mut init = re_token_t {
            type_0_constraint_duplicated_opt_subexp_accept_mb_mb_partial_word_char: [0; 3],
            c2rust_padding: [0; 5],
            opr: C2RustUnnamed { c: 0 },
        };
        init.set_type_0(type_0);
        init.set_constraint(0);
        init.set_duplicated(0);
        init.set_opt_subexp(0);
        init.set_accept_mb(0);
        init.set_mb_partial(0);
        init.set_word_char(0);
        init
    };
    return create_token_tree(dfa, left, right, &mut t);
}
unsafe extern "C" fn create_token_tree(
    mut dfa: *mut re_dfa_t,
    mut left: *mut bin_tree_t,
    mut right: *mut bin_tree_t,
    mut token: *const re_token_t,
) -> *mut bin_tree_t {
    let mut tree: *mut bin_tree_t = 0 as *mut bin_tree_t;
    if ((*dfa).str_tree_storage_idx as libc::c_ulong
        == (1024 as libc::c_int as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<bin_tree_t>() as libc::c_ulong))
        as libc::c_int as libc::c_long != 0
    {
        let mut storage: *mut bin_tree_storage_t = malloc(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<bin_tree_storage_t>() as libc::c_ulong,
                ),
        ) as *mut bin_tree_storage_t;
        if storage.is_null() {
            return 0 as *mut bin_tree_t;
        }
        (*storage).next = (*dfa).str_tree_storage;
        (*dfa).str_tree_storage = storage;
        (*dfa).str_tree_storage_idx = 0 as libc::c_int;
    }
    let fresh47 = (*dfa).str_tree_storage_idx;
    (*dfa).str_tree_storage_idx = (*dfa).str_tree_storage_idx + 1;
    tree = &mut *((*(*dfa).str_tree_storage).data).as_mut_ptr().offset(fresh47 as isize)
        as *mut bin_tree_t;
    (*tree).parent = 0 as *mut bin_tree_t;
    (*tree).left = left;
    (*tree).right = right;
    (*tree).token = *token;
    ((*tree).token).set_duplicated(0 as libc::c_int as libc::c_uint);
    ((*tree).token).set_opt_subexp(0 as libc::c_int as libc::c_uint);
    (*tree).first = 0 as *mut bin_tree_t;
    (*tree).next = 0 as *mut bin_tree_t;
    (*tree).node_idx = -(1 as libc::c_int) as Idx;
    if !left.is_null() {
        (*left).parent = tree;
    }
    if !right.is_null() {
        (*right).parent = tree;
    }
    return tree;
}
unsafe extern "C" fn mark_opt_subexp(
    mut extra: *mut libc::c_void,
    mut node: *mut bin_tree_t,
) -> reg_errcode_t {
    let mut idx: Idx = extra as uintptr_t as Idx;
    if ((*node).token).type_0() as libc::c_int == SUBEXP as libc::c_int
        && (*node).token.opr.idx == idx
    {
        ((*node).token).set_opt_subexp(1 as libc::c_int as libc::c_uint);
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn free_token(mut node: *mut re_token_t) {
    if (*node).type_0() as libc::c_int == COMPLEX_BRACKET as libc::c_int
        && (*node).duplicated() as libc::c_int == 0 as libc::c_int
    {
        free_charset((*node).opr.mbcset);
    } else if (*node).type_0() as libc::c_int == SIMPLE_BRACKET as libc::c_int
        && (*node).duplicated() as libc::c_int == 0 as libc::c_int
    {
        rpl_free((*node).opr.sbcset as *mut libc::c_void);
    }
}
unsafe extern "C" fn free_tree(
    mut extra: *mut libc::c_void,
    mut node: *mut bin_tree_t,
) -> reg_errcode_t {
    free_token(&mut (*node).token);
    return _REG_NOERROR;
}
unsafe extern "C" fn duplicate_tree(
    mut root: *const bin_tree_t,
    mut dfa: *mut re_dfa_t,
) -> *mut bin_tree_t {
    let mut node: *const bin_tree_t = 0 as *const bin_tree_t;
    let mut dup_root: *mut bin_tree_t = 0 as *mut bin_tree_t;
    let mut p_new: *mut *mut bin_tree_t = &mut dup_root;
    let mut dup_node: *mut bin_tree_t = (*root).parent;
    node = root;
    loop {
        *p_new = create_token_tree(
            dfa,
            0 as *mut bin_tree_t,
            0 as *mut bin_tree_t,
            &(*node).token,
        );
        if (*p_new).is_null() {
            return 0 as *mut bin_tree_t;
        }
        (**p_new).parent = dup_node;
        ((**p_new).token).set_duplicated(1 as libc::c_int as libc::c_uint);
        dup_node = *p_new;
        if !((*node).left).is_null() {
            node = (*node).left;
            p_new = &mut (*dup_node).left;
        } else {
            let mut prev: *const bin_tree_t = 0 as *const bin_tree_t;
            while (*node).right == prev as *mut bin_tree_t || ((*node).right).is_null() {
                prev = node;
                node = (*node).parent;
                dup_node = (*dup_node).parent;
                if node.is_null() {
                    return dup_root;
                }
            }
            node = (*node).right;
            p_new = &mut (*dup_node).right;
        }
    };
}
#[inline]
unsafe extern "C" fn regmatch_list_begin(
    mut list: *mut regmatch_list,
) -> *mut regmatch_t {
    return (*list).u.dynarray_header.array;
}
unsafe extern "C" fn check_arrival_expand_ecl_sub(
    mut dfa: *const re_dfa_t,
    mut dst_nodes: *mut re_node_set,
    mut target: Idx,
    mut ex_subexp: Idx,
    mut type_0: libc::c_int,
) -> reg_errcode_t {
    let mut cur_node: Idx = 0;
    cur_node = target;
    while re_node_set_contains(dst_nodes, cur_node) == 0 {
        let mut ok: bool = false;
        if (*((*dfa).nodes).offset(cur_node as isize)).type_0() as libc::c_int == type_0
            && (*((*dfa).nodes).offset(cur_node as isize)).opr.idx == ex_subexp
        {
            if type_0 == OP_CLOSE_SUBEXP as libc::c_int {
                ok = re_node_set_insert(dst_nodes, cur_node);
                if !ok as libc::c_int as libc::c_long != 0 {
                    return _REG_ESPACE;
                }
            }
            break;
        } else {
            ok = re_node_set_insert(dst_nodes, cur_node);
            if !ok as libc::c_int as libc::c_long != 0 {
                return _REG_ESPACE;
            }
            if (*((*dfa).edests).offset(cur_node as isize)).nelem
                == 0 as libc::c_int as libc::c_long
            {
                break;
            }
            if (*((*dfa).edests).offset(cur_node as isize)).nelem
                == 2 as libc::c_int as libc::c_long
            {
                let mut err: reg_errcode_t = _REG_NOERROR;
                err = check_arrival_expand_ecl_sub(
                    dfa,
                    dst_nodes,
                    *((*((*dfa).edests).offset(cur_node as isize)).elems)
                        .offset(1 as libc::c_int as isize),
                    ex_subexp,
                    type_0,
                );
                if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    return err;
                }
            }
            cur_node = *((*((*dfa).edests).offset(cur_node as isize)).elems)
                .offset(0 as libc::c_int as isize);
        }
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn find_subexp_node(
    mut dfa: *const re_dfa_t,
    mut nodes: *const re_node_set,
    mut subexp_idx: Idx,
    mut type_0: libc::c_int,
) -> Idx {
    let mut cls_idx: Idx = 0;
    cls_idx = 0 as libc::c_int as Idx;
    while cls_idx < (*nodes).nelem {
        let mut cls_node: Idx = *((*nodes).elems).offset(cls_idx as isize);
        let mut node: *const re_token_t = ((*dfa).nodes).offset(cls_node as isize);
        if (*node).type_0() as libc::c_int == type_0 && (*node).opr.idx == subexp_idx {
            return cls_node;
        }
        cls_idx += 1;
        cls_idx;
    }
    return -(1 as libc::c_int) as Idx;
}
unsafe extern "C" fn check_arrival_add_next_nodes(
    mut mctx: *mut re_match_context_t,
    mut str_idx: Idx,
    mut cur_nodes: *mut re_node_set,
    mut next_nodes: *mut re_node_set,
) -> reg_errcode_t {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut ok: bool = false;
    let mut cur_idx: Idx = 0;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut union_set: re_node_set = re_node_set {
        alloc: 0,
        nelem: 0,
        elems: 0 as *mut Idx,
    };
    memset(
        &mut union_set as *mut re_node_set as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<re_node_set>() as libc::c_ulong,
    );
    cur_idx = 0 as libc::c_int as Idx;
    while cur_idx < (*cur_nodes).nelem {
        let mut naccepted: libc::c_int = 0 as libc::c_int;
        let mut cur_node: Idx = *((*cur_nodes).elems).offset(cur_idx as isize);
        if (*((*dfa).nodes).offset(cur_node as isize)).type_0() as libc::c_int
            & 8 as libc::c_int == 0
        {} else {
            unreachable!();
        };
        if (*((*dfa).nodes).offset(cur_node as isize)).accept_mb() != 0 {
            naccepted = check_node_accept_bytes(
                dfa,
                cur_node,
                &mut (*mctx).input,
                str_idx,
            );
            if naccepted > 1 as libc::c_int {
                let mut dest_state: *mut re_dfastate_t = 0 as *mut re_dfastate_t;
                let mut next_node: Idx = *((*dfa).nexts).offset(cur_node as isize);
                let mut next_idx: Idx = str_idx + naccepted as libc::c_long;
                dest_state = *((*mctx).state_log).offset(next_idx as isize);
                union_set.nelem = 0 as libc::c_int as Idx;
                if !dest_state.is_null() {
                    err = re_node_set_merge(&mut union_set, &mut (*dest_state).nodes);
                    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        rpl_free(union_set.elems as *mut libc::c_void);
                        return err;
                    }
                }
                ok = re_node_set_insert(&mut union_set, next_node);
                if !ok as libc::c_int as libc::c_long != 0 {
                    rpl_free(union_set.elems as *mut libc::c_void);
                    return _REG_ESPACE;
                }
                let ref mut fresh48 = *((*mctx).state_log).offset(next_idx as isize);
                *fresh48 = re_acquire_state(&mut err, dfa, &mut union_set);
                if ((*((*mctx).state_log).offset(next_idx as isize)).is_null()
                    && err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    rpl_free(union_set.elems as *mut libc::c_void);
                    return err;
                }
            }
        }
        if naccepted != 0
            || check_node_accept(mctx, ((*dfa).nodes).offset(cur_node as isize), str_idx)
                as libc::c_int != 0
        {
            ok = re_node_set_insert(
                next_nodes,
                *((*dfa).nexts).offset(cur_node as isize),
            );
            if !ok as libc::c_int as libc::c_long != 0 {
                rpl_free(union_set.elems as *mut libc::c_void);
                return _REG_ESPACE;
            }
        }
        cur_idx += 1;
        cur_idx;
    }
    rpl_free(union_set.elems as *mut libc::c_void);
    return _REG_NOERROR;
}
unsafe extern "C" fn match_ctx_add_sublast(
    mut subtop: *mut re_sub_match_top_t,
    mut node: Idx,
    mut str_idx: Idx,
) -> *mut re_sub_match_last_t {
    let mut new_entry: *mut re_sub_match_last_t = 0 as *mut re_sub_match_last_t;
    if ((*subtop).nlasts == (*subtop).alasts) as libc::c_int as libc::c_long != 0 {
        let mut new_alasts: Idx = 2 as libc::c_int as libc::c_long * (*subtop).alasts
            + 1 as libc::c_int as libc::c_long;
        let mut new_array: *mut *mut re_sub_match_last_t = realloc(
            (*subtop).lasts as *mut libc::c_void,
            (new_alasts as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut re_sub_match_last_t>() as libc::c_ulong,
                ),
        ) as *mut *mut re_sub_match_last_t;
        if (new_array == 0 as *mut libc::c_void as *mut *mut re_sub_match_last_t)
            as libc::c_int as libc::c_long != 0
        {
            return 0 as *mut re_sub_match_last_t;
        }
        (*subtop).lasts = new_array;
        (*subtop).alasts = new_alasts;
    }
    new_entry = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<re_sub_match_last_t>() as libc::c_ulong,
    ) as *mut re_sub_match_last_t;
    if (new_entry != 0 as *mut libc::c_void as *mut re_sub_match_last_t) as libc::c_int
        as libc::c_long != 0
    {
        let ref mut fresh49 = *((*subtop).lasts).offset((*subtop).nlasts as isize);
        *fresh49 = new_entry;
        (*new_entry).node = node;
        (*new_entry).str_idx = str_idx;
        (*subtop).nlasts += 1;
        (*subtop).nlasts;
    }
    return new_entry;
}
unsafe extern "C" fn transit_state(
    mut err: *mut reg_errcode_t,
    mut mctx: *mut re_match_context_t,
    mut state: *mut re_dfastate_t,
) -> *mut re_dfastate_t {
    let mut trtable: *mut *mut re_dfastate_t = 0 as *mut *mut re_dfastate_t;
    let mut ch: libc::c_uchar = 0;
    if (*state).accept_mb() as libc::c_long != 0 {
        *err = transit_state_mb(mctx, state);
        if (*err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return 0 as *mut re_dfastate_t;
        }
    }
    let fresh50 = (*mctx).input.cur_idx;
    (*mctx).input.cur_idx = (*mctx).input.cur_idx + 1;
    ch = *((*mctx).input.mbs).offset(fresh50 as isize);
    loop {
        trtable = (*state).trtable;
        if (trtable != 0 as *mut libc::c_void as *mut *mut re_dfastate_t) as libc::c_int
            as libc::c_long != 0
        {
            return *trtable.offset(ch as isize);
        }
        trtable = (*state).word_trtable;
        if (trtable != 0 as *mut libc::c_void as *mut *mut re_dfastate_t) as libc::c_int
            as libc::c_long != 0
        {
            let mut context: libc::c_uint = 0;
            context = re_string_context_at(
                &mut (*mctx).input,
                (*mctx).input.cur_idx - 1 as libc::c_int as libc::c_long,
                (*mctx).eflags,
            );
            if context & 1 as libc::c_int as libc::c_uint != 0 {
                return *trtable
                    .offset(
                        (ch as libc::c_int
                            + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                + 1 as libc::c_int)) as isize,
                    )
            } else {
                return *trtable.offset(ch as isize)
            }
        }
        if !build_trtable((*mctx).dfa, state) {
            *err = _REG_ESPACE;
            return 0 as *mut re_dfastate_t;
        }
    };
}
#[inline(never)]
unsafe extern "C" fn build_trtable(
    mut dfa: *const re_dfa_t,
    mut state: *mut re_dfastate_t,
) -> bool {
    let mut current_block: u64;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut i: Idx = 0;
    let mut j: Idx = 0;
    let mut ch: libc::c_int = 0;
    let mut need_word_trtable: bool = 0 as libc::c_int != 0;
    let mut elem: bitset_word_t = 0;
    let mut mask: bitset_word_t = 0;
    let mut ndests: Idx = 0;
    let mut trtable: *mut *mut re_dfastate_t = 0 as *mut *mut re_dfastate_t;
    let mut dest_states: [*mut re_dfastate_t; 256] = [0 as *mut re_dfastate_t; 256];
    let mut dest_states_word: [*mut re_dfastate_t; 256] = [0 as *mut re_dfastate_t; 256];
    let mut dest_states_nl: [*mut re_dfastate_t; 256] = [0 as *mut re_dfastate_t; 256];
    let mut follows: re_node_set = re_node_set {
        alloc: 0,
        nelem: 0,
        elems: 0 as *mut Idx,
    };
    let mut acceptable: bitset_t = [0; 4];
    let mut dests_node: [re_node_set; 256] = [re_node_set {
        alloc: 0,
        nelem: 0,
        elems: 0 as *mut Idx,
    }; 256];
    let mut dests_ch: [bitset_t; 256] = [[0; 4]; 256];
    (*state).trtable = 0 as *mut *mut re_dfastate_t;
    (*state).word_trtable = (*state).trtable;
    ndests = group_nodes_into_DFAstates(
        dfa,
        state,
        dests_node.as_mut_ptr(),
        dests_ch.as_mut_ptr(),
    );
    if (ndests <= 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0 {
        if ndests == 0 as libc::c_int as libc::c_long {
            (*state)
                .trtable = calloc(
                ::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong,
                (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int) as libc::c_ulong,
            ) as *mut *mut re_dfastate_t;
            if ((*state).trtable == 0 as *mut libc::c_void as *mut *mut re_dfastate_t)
                as libc::c_int as libc::c_long != 0
            {
                return 0 as libc::c_int != 0;
            }
            return 1 as libc::c_int != 0;
        }
        return 0 as libc::c_int != 0;
    }
    err = re_node_set_alloc(&mut follows, ndests + 1 as libc::c_int as libc::c_long);
    if !((err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
        as libc::c_long != 0)
    {
        bitset_empty(acceptable.as_mut_ptr());
        i = 0 as libc::c_int as Idx;
        's_99: loop {
            if !(i < ndests) {
                current_block = 9007357115414505193;
                break;
            }
            let mut next_node: Idx = 0;
            follows.nelem = 0 as libc::c_int as Idx;
            j = 0 as libc::c_int as Idx;
            while j < dests_node[i as usize].nelem {
                next_node = *((*dfa).nexts)
                    .offset(*(dests_node[i as usize].elems).offset(j as isize) as isize);
                if next_node != -(1 as libc::c_int) as libc::c_long {
                    err = re_node_set_merge(
                        &mut follows,
                        ((*dfa).eclosures).offset(next_node as isize),
                    );
                    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        current_block = 13266360174358878635;
                        break 's_99;
                    }
                }
                j += 1;
                j;
            }
            dest_states[i
                as usize] = re_acquire_state_context(
                &mut err,
                dfa,
                &mut follows,
                0 as libc::c_int as libc::c_uint,
            );
            if ((dest_states[i as usize]).is_null()
                && err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                current_block = 13266360174358878635;
                break;
            }
            if (*dest_states[i as usize]).has_constraint() != 0 {
                dest_states_word[i
                    as usize] = re_acquire_state_context(
                    &mut err,
                    dfa,
                    &mut follows,
                    1 as libc::c_int as libc::c_uint,
                );
                if ((dest_states_word[i as usize]).is_null()
                    && err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    current_block = 13266360174358878635;
                    break;
                }
                if dest_states[i as usize] != dest_states_word[i as usize]
                    && (*dfa).mb_cur_max > 1 as libc::c_int
                {
                    need_word_trtable = 1 as libc::c_int != 0;
                }
                dest_states_nl[i
                    as usize] = re_acquire_state_context(
                    &mut err,
                    dfa,
                    &mut follows,
                    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
                );
                if ((dest_states_nl[i as usize]).is_null()
                    && err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    current_block = 13266360174358878635;
                    break;
                }
            } else {
                dest_states_word[i as usize] = dest_states[i as usize];
                dest_states_nl[i as usize] = dest_states[i as usize];
            }
            bitset_merge(
                acceptable.as_mut_ptr(),
                (dests_ch[i as usize]).as_mut_ptr() as *const bitset_word_t,
            );
            i += 1;
            i;
        }
        match current_block {
            13266360174358878635 => {}
            _ => {
                if need_word_trtable as libc::c_long == 0 {
                    (*state)
                        .trtable = calloc(
                        ::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong,
                        (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                            + 1 as libc::c_int) as libc::c_ulong,
                    ) as *mut *mut re_dfastate_t;
                    trtable = (*state).trtable;
                    if (trtable == 0 as *mut libc::c_void as *mut *mut re_dfastate_t)
                        as libc::c_int as libc::c_long != 0
                    {
                        current_block = 13266360174358878635;
                    } else {
                        i = 0 as libc::c_int as Idx;
                        while i
                            < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                                / 64 as libc::c_int) as libc::c_long
                        {
                            ch = (i * 64 as libc::c_int as libc::c_long) as libc::c_int;
                            elem = acceptable[i as usize];
                            mask = 1 as libc::c_int as bitset_word_t;
                            while elem != 0 {
                                if (elem & 1 as libc::c_int as libc::c_ulong)
                                    as libc::c_long != 0
                                {
                                    j = 0 as libc::c_int as Idx;
                                    while dests_ch[j as usize][i as usize] & mask
                                        == 0 as libc::c_int as libc::c_ulong
                                    {
                                        j += 1;
                                        j;
                                    }
                                    if (*dfa).word_char[i as usize] & mask != 0 {
                                        let ref mut fresh51 = *trtable.offset(ch as isize);
                                        *fresh51 = dest_states_word[j as usize];
                                    } else {
                                        let ref mut fresh52 = *trtable.offset(ch as isize);
                                        *fresh52 = dest_states[j as usize];
                                    }
                                }
                                mask <<= 1 as libc::c_int;
                                elem >>= 1 as libc::c_int;
                                ch += 1;
                                ch;
                            }
                            i += 1;
                            i;
                        }
                        current_block = 1874315696050160458;
                    }
                } else {
                    (*state)
                        .word_trtable = calloc(
                        ::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong,
                        (2 as libc::c_int
                            * (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                + 1 as libc::c_int)) as libc::c_ulong,
                    ) as *mut *mut re_dfastate_t;
                    trtable = (*state).word_trtable;
                    if (trtable == 0 as *mut libc::c_void as *mut *mut re_dfastate_t)
                        as libc::c_int as libc::c_long != 0
                    {
                        current_block = 13266360174358878635;
                    } else {
                        i = 0 as libc::c_int as Idx;
                        while i
                            < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                                / 64 as libc::c_int) as libc::c_long
                        {
                            ch = (i * 64 as libc::c_int as libc::c_long) as libc::c_int;
                            elem = acceptable[i as usize];
                            mask = 1 as libc::c_int as bitset_word_t;
                            while elem != 0 {
                                if (elem & 1 as libc::c_int as libc::c_ulong)
                                    as libc::c_long != 0
                                {
                                    j = 0 as libc::c_int as Idx;
                                    while dests_ch[j as usize][i as usize] & mask
                                        == 0 as libc::c_int as libc::c_ulong
                                    {
                                        j += 1;
                                        j;
                                    }
                                    let ref mut fresh53 = *trtable.offset(ch as isize);
                                    *fresh53 = dest_states[j as usize];
                                    let ref mut fresh54 = *trtable
                                        .offset(
                                            (ch
                                                + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                                    + 1 as libc::c_int)) as isize,
                                        );
                                    *fresh54 = dest_states_word[j as usize];
                                }
                                mask <<= 1 as libc::c_int;
                                elem >>= 1 as libc::c_int;
                                ch += 1;
                                ch;
                            }
                            i += 1;
                            i;
                        }
                        current_block = 1874315696050160458;
                    }
                }
                match current_block {
                    13266360174358878635 => {}
                    _ => {
                        if bitset_contain(
                            acceptable.as_mut_ptr() as *const bitset_word_t,
                            '\n' as i32 as Idx,
                        ) {
                            j = 0 as libc::c_int as Idx;
                            while j < ndests {
                                if bitset_contain(
                                    (dests_ch[j as usize]).as_mut_ptr() as *const bitset_word_t,
                                    '\n' as i32 as Idx,
                                ) {
                                    let ref mut fresh55 = *trtable.offset('\n' as i32 as isize);
                                    *fresh55 = dest_states_nl[j as usize];
                                    if need_word_trtable {
                                        let ref mut fresh56 = *trtable
                                            .offset(
                                                ('\n' as i32
                                                    + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                                        + 1 as libc::c_int)) as isize,
                                            );
                                        *fresh56 = dest_states_nl[j as usize];
                                    }
                                    break;
                                } else {
                                    j += 1;
                                    j;
                                }
                            }
                        }
                        rpl_free(follows.elems as *mut libc::c_void);
                        i = 0 as libc::c_int as Idx;
                        while i < ndests {
                            rpl_free(
                                (*dests_node.as_mut_ptr().offset(i as isize)).elems
                                    as *mut libc::c_void,
                            );
                            i += 1;
                            i;
                        }
                        return 1 as libc::c_int != 0;
                    }
                }
            }
        }
    }
    rpl_free(follows.elems as *mut libc::c_void);
    i = 0 as libc::c_int as Idx;
    while i < ndests {
        rpl_free(
            (*dests_node.as_mut_ptr().offset(i as isize)).elems as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn re_search_internal(
    mut preg: *const regex_t,
    mut string: *const libc::c_char,
    mut length: Idx,
    mut start: Idx,
    mut last_start: Idx,
    mut stop: Idx,
    mut nmatch: size_t,
    mut pmatch: *mut regmatch_t,
    mut eflags: libc::c_int,
) -> reg_errcode_t {
    let mut current_block: u64;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut dfa: *const re_dfa_t = (*preg).buffer;
    let mut left_lim: Idx = 0;
    let mut right_lim: Idx = 0;
    let mut incr: libc::c_int = 0;
    let mut fl_longest_match: bool = false;
    let mut match_kind: libc::c_int = 0;
    let mut match_first: Idx = 0;
    let mut match_last: Idx = -(1 as libc::c_int) as Idx;
    let mut extra_nmatch: Idx = 0;
    let mut sb: bool = false;
    let mut ch: libc::c_int = 0;
    let mut mctx: re_match_context_t = {
        let mut init = re_match_context_t {
            input: re_string_t {
                raw_mbs: 0 as *const libc::c_uchar,
                mbs: 0 as *mut libc::c_uchar,
                wcs: 0 as *mut wint_t,
                offsets: 0 as *mut Idx,
                cur_state: mbstate_t {
                    __count: 0,
                    __value: C2RustUnnamed_0 { __wch: 0 },
                },
                raw_mbs_idx: 0,
                valid_len: 0,
                valid_raw_len: 0,
                bufs_len: 0,
                cur_idx: 0,
                raw_len: 0,
                len: 0,
                raw_stop: 0,
                stop: 0,
                tip_context: 0,
                trans: 0 as *mut libc::c_uchar,
                word_char: 0 as *const bitset_word_t,
                icase: 0,
                is_utf8: 0,
                map_notascii: 0,
                mbs_allocated: 0,
                offsets_needed: 0,
                newline_anchor: 0,
                word_ops_used: 0,
                mb_cur_max: 0,
            },
            dfa: dfa,
            eflags: 0,
            match_last: 0,
            last_node: 0,
            state_log: 0 as *mut *mut re_dfastate_t,
            state_log_top: 0,
            nbkref_ents: 0,
            abkref_ents: 0,
            bkref_ents: 0 as *mut re_backref_cache_entry,
            max_mb_elem_len: 0,
            nsub_tops: 0,
            asub_tops: 0,
            sub_tops: 0 as *mut *mut re_sub_match_top_t,
        };
        init
    };
    let mut fastmap: *mut libc::c_char = if !((*preg).fastmap).is_null()
        && (*preg).fastmap_accurate() as libc::c_int != 0 && start != last_start
        && (*preg).can_be_null() == 0
    {
        (*preg).fastmap
    } else {
        0 as *mut libc::c_char
    };
    let mut t: *mut libc::c_uchar = (*preg).translate;
    extra_nmatch = (if nmatch > (*preg).re_nsub {
        nmatch
            .wrapping_sub(
                ((*preg).re_nsub).wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as Idx;
    nmatch = (nmatch as libc::c_ulong).wrapping_sub(extra_nmatch as libc::c_ulong)
        as size_t as size_t;
    if ((*preg).used == 0 as libc::c_int as libc::c_ulong
        || ((*dfa).init_state).is_null() || ((*dfa).init_state_word).is_null()
        || ((*dfa).init_state_nl).is_null() || ((*dfa).init_state_begbuf).is_null())
        as libc::c_int as libc::c_long != 0
    {
        return _REG_NOMATCH;
    }
    if 0 as libc::c_int as libc::c_long <= last_start && last_start <= length {} else {
        unreachable!();
    };
    if (*(*dfa).init_state).nodes.nelem == 0 as libc::c_int as libc::c_long
        && (*(*dfa).init_state_word).nodes.nelem == 0 as libc::c_int as libc::c_long
        && ((*(*dfa).init_state_nl).nodes.nelem == 0 as libc::c_int as libc::c_long
            || (*preg).newline_anchor() == 0)
    {
        if start != 0 as libc::c_int as libc::c_long
            && last_start != 0 as libc::c_int as libc::c_long
        {
            return _REG_NOMATCH;
        }
        last_start = 0 as libc::c_int as Idx;
        start = last_start;
    }
    fl_longest_match = nmatch != 0 as libc::c_int as libc::c_ulong
        || (*dfa).nbackref != 0;
    err = re_string_allocate(
        &mut mctx.input,
        string,
        length,
        ((*dfa).nodes_len).wrapping_add(1 as libc::c_int as libc::c_ulong) as Idx,
        (*preg).translate,
        (*preg).syntax
            & ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int != 0 as libc::c_int as libc::c_ulong,
        dfa,
    );
    if !((err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
        as libc::c_long != 0)
    {
        mctx.input.stop = stop;
        mctx.input.raw_stop = stop;
        mctx.input.newline_anchor = (*preg).newline_anchor() as libc::c_uchar;
        err = match_ctx_init(
            &mut mctx,
            eflags,
            (*dfa).nbackref * 2 as libc::c_int as libc::c_long,
        );
        if !((err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0)
        {
            if nmatch > 1 as libc::c_int as libc::c_ulong
                || (*dfa).has_mb_node() as libc::c_int != 0
            {
                if ((if (9223372036854775807 as libc::c_long as libc::c_ulong)
                    < (18446744073709551615 as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong,
                        )
                {
                    9223372036854775807 as libc::c_long as libc::c_ulong
                } else {
                    (18446744073709551615 as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong,
                        )
                }) <= mctx.input.bufs_len as libc::c_ulong) as libc::c_int
                    as libc::c_long != 0
                {
                    err = _REG_ESPACE;
                    current_block = 7162871025868831372;
                } else {
                    mctx
                        .state_log = malloc(
                        ((mctx.input.bufs_len + 1 as libc::c_int as libc::c_long)
                            as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<*mut re_dfastate_t>()
                                    as libc::c_ulong,
                            ),
                    ) as *mut *mut re_dfastate_t;
                    if (mctx.state_log
                        == 0 as *mut libc::c_void as *mut *mut re_dfastate_t)
                        as libc::c_int as libc::c_long != 0
                    {
                        err = _REG_ESPACE;
                        current_block = 7162871025868831372;
                    } else {
                        current_block = 11194104282611034094;
                    }
                }
            } else {
                current_block = 11194104282611034094;
            }
            match current_block {
                7162871025868831372 => {}
                _ => {
                    match_first = start;
                    mctx
                        .input
                        .tip_context = (if eflags & 1 as libc::c_int != 0 {
                        ((1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    } else {
                        (1 as libc::c_int) << 1 as libc::c_int
                            | ((1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int
                    }) as libc::c_uint;
                    incr = if last_start < start {
                        -(1 as libc::c_int)
                    } else {
                        1 as libc::c_int
                    };
                    left_lim = if last_start < start { last_start } else { start };
                    right_lim = if last_start < start { start } else { last_start };
                    sb = (*dfa).mb_cur_max == 1 as libc::c_int;
                    match_kind = if !fastmap.is_null() {
                        (if sb as libc::c_int != 0
                            || !((*preg).syntax
                                & ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int != 0
                                || !t.is_null())
                        {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                            | (if start <= last_start {
                                2 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                            | (if !t.is_null() {
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                    } else {
                        8 as libc::c_int
                    };
                    's_148: loop {
                        err = _REG_NOMATCH;
                        if match_first < left_lim || right_lim < match_first {
                            current_block = 7162871025868831372;
                            break;
                        }
                        match match_kind {
                            8 => {
                                current_block = 6560072651652764009;
                            }
                            7 => {
                                while (match_first < right_lim) as libc::c_int
                                    as libc::c_long != 0
                                    && *fastmap
                                        .offset(
                                            *t
                                                .offset(
                                                    *string.offset(match_first as isize) as libc::c_uchar
                                                        as isize,
                                                ) as isize,
                                        ) == 0
                                {
                                    match_first += 1;
                                    match_first;
                                }
                                current_block = 3040362425723665191;
                            }
                            6 => {
                                while (match_first < right_lim) as libc::c_int
                                    as libc::c_long != 0
                                    && *fastmap
                                        .offset(
                                            *string.offset(match_first as isize) as libc::c_uchar
                                                as isize,
                                        ) == 0
                                {
                                    match_first += 1;
                                    match_first;
                                }
                                current_block = 3040362425723665191;
                            }
                            4 | 5 => {
                                while match_first >= left_lim {
                                    ch = if match_first >= length {
                                        0 as libc::c_int
                                    } else {
                                        *string.offset(match_first as isize) as libc::c_uchar
                                            as libc::c_int
                                    };
                                    if *fastmap
                                        .offset(
                                            (if !t.is_null() {
                                                *t.offset(ch as isize) as libc::c_int
                                            } else {
                                                ch
                                            }) as isize,
                                        ) != 0
                                    {
                                        break;
                                    }
                                    match_first -= 1;
                                    match_first;
                                }
                                if match_first < left_lim {
                                    current_block = 7162871025868831372;
                                    break;
                                }
                                current_block = 6560072651652764009;
                            }
                            _ => {
                                loop {
                                    let mut offset: __re_size_t = (match_first
                                        - mctx.input.raw_mbs_idx) as __re_size_t;
                                    if (offset >= mctx.input.valid_raw_len as __re_size_t)
                                        as libc::c_int as libc::c_long != 0
                                    {
                                        err = re_string_reconstruct(
                                            &mut mctx.input,
                                            match_first,
                                            eflags,
                                        );
                                        if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                            as libc::c_int as libc::c_long != 0
                                        {
                                            current_block = 7162871025868831372;
                                            break 's_148;
                                        }
                                        offset = (match_first - mctx.input.raw_mbs_idx)
                                            as __re_size_t;
                                    }
                                    ch = if offset < mctx.input.valid_len as libc::c_ulong {
                                        *(mctx.input.mbs).offset(offset as isize) as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    };
                                    if *fastmap.offset(ch as isize) != 0 {
                                        break;
                                    }
                                    match_first += incr as libc::c_long;
                                    if !(match_first < left_lim || match_first > right_lim) {
                                        continue;
                                    }
                                    err = _REG_NOMATCH;
                                    current_block = 7162871025868831372;
                                    break 's_148;
                                }
                                current_block = 6560072651652764009;
                            }
                        }
                        match current_block {
                            3040362425723665191 => {
                                if (match_first == right_lim) as libc::c_int as libc::c_long
                                    != 0
                                {
                                    ch = if match_first >= length {
                                        0 as libc::c_int
                                    } else {
                                        *string.offset(match_first as isize) as libc::c_uchar
                                            as libc::c_int
                                    };
                                    if *fastmap
                                        .offset(
                                            (if !t.is_null() {
                                                *t.offset(ch as isize) as libc::c_int
                                            } else {
                                                ch
                                            }) as isize,
                                        ) == 0
                                    {
                                        current_block = 7162871025868831372;
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                        err = re_string_reconstruct(
                            &mut mctx.input,
                            match_first,
                            eflags,
                        );
                        if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                            as libc::c_int as libc::c_long != 0
                        {
                            current_block = 7162871025868831372;
                            break;
                        }
                        if !(!sb
                            && !(0 as libc::c_int as libc::c_long == mctx.input.valid_len
                                || *(mctx.input.wcs).offset(0 as libc::c_int as isize)
                                    != 0xffffffff as libc::c_uint))
                        {
                            mctx.max_mb_elem_len = 0 as libc::c_int;
                            mctx.nbkref_ents = mctx.max_mb_elem_len as Idx;
                            mctx.state_log_top = mctx.nbkref_ents;
                            match_last = check_matching(
                                &mut mctx,
                                fl_longest_match,
                                if start <= last_start {
                                    &mut match_first
                                } else {
                                    0 as *mut Idx
                                },
                            );
                            if match_last != -(1 as libc::c_int) as libc::c_long {
                                if (match_last == -(2 as libc::c_int) as libc::c_long)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    err = _REG_ESPACE;
                                    current_block = 7162871025868831372;
                                    break;
                                } else {
                                    mctx.match_last = match_last;
                                    if (*preg).no_sub() == 0
                                        && nmatch > 1 as libc::c_int as libc::c_ulong
                                        || (*dfa).nbackref != 0
                                    {
                                        let mut pstate: *mut re_dfastate_t = *(mctx.state_log)
                                            .offset(match_last as isize);
                                        mctx
                                            .last_node = check_halt_state_context(
                                            &mut mctx,
                                            pstate,
                                            match_last,
                                        );
                                    }
                                    if !((*preg).no_sub() == 0
                                        && nmatch > 1 as libc::c_int as libc::c_ulong
                                        && (*dfa).has_plural_match() as libc::c_int != 0
                                        || (*dfa).nbackref != 0)
                                    {
                                        current_block = 11739054925370445424;
                                        break;
                                    }
                                    err = prune_impossible_nodes(&mut mctx);
                                    if err as libc::c_int == _REG_NOERROR as libc::c_int {
                                        current_block = 11739054925370445424;
                                        break;
                                    }
                                    if (err as libc::c_int != _REG_NOMATCH as libc::c_int)
                                        as libc::c_int as libc::c_long != 0
                                    {
                                        current_block = 7162871025868831372;
                                        break;
                                    }
                                    match_last = -(1 as libc::c_int) as Idx;
                                }
                            }
                            match_ctx_clean(&mut mctx);
                        }
                        match_first += incr as libc::c_long;
                    }
                    match current_block {
                        7162871025868831372 => {}
                        _ => {
                            if match_last != -(1 as libc::c_int) as libc::c_long
                            {} else {
                                unreachable!();
                            };
                            if err as libc::c_int == _REG_NOERROR as libc::c_int
                            {} else {
                                unreachable!();
                            };
                            if nmatch > 0 as libc::c_int as libc::c_ulong {
                                let mut reg_idx: Idx = 0;
                                reg_idx = 1 as libc::c_int as Idx;
                                while (reg_idx as libc::c_ulong) < nmatch {
                                    let ref mut fresh57 = (*pmatch.offset(reg_idx as isize))
                                        .rm_eo;
                                    *fresh57 = -(1 as libc::c_int) as regoff_t;
                                    (*pmatch.offset(reg_idx as isize)).rm_so = *fresh57;
                                    reg_idx += 1;
                                    reg_idx;
                                }
                                (*pmatch.offset(0 as libc::c_int as isize))
                                    .rm_so = 0 as libc::c_int as regoff_t;
                                (*pmatch.offset(0 as libc::c_int as isize))
                                    .rm_eo = mctx.match_last;
                                if (*preg).no_sub() == 0
                                    && nmatch > 1 as libc::c_int as libc::c_ulong
                                {
                                    err = set_regs(
                                        preg,
                                        &mut mctx,
                                        nmatch,
                                        pmatch,
                                        (*dfa).has_plural_match() as libc::c_int != 0
                                            && (*dfa).nbackref > 0 as libc::c_int as libc::c_long,
                                    );
                                    if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                        as libc::c_int as libc::c_long != 0
                                    {
                                        current_block = 7162871025868831372;
                                    } else {
                                        current_block = 5722677567366458307;
                                    }
                                } else {
                                    current_block = 5722677567366458307;
                                }
                                match current_block {
                                    7162871025868831372 => {}
                                    _ => {
                                        reg_idx = 0 as libc::c_int as Idx;
                                        while (reg_idx as libc::c_ulong) < nmatch {
                                            if (*pmatch.offset(reg_idx as isize)).rm_so
                                                != -(1 as libc::c_int) as libc::c_long
                                            {
                                                if (mctx.input.offsets_needed as libc::c_int
                                                    != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                                                {
                                                    (*pmatch.offset(reg_idx as isize))
                                                        .rm_so = if (*pmatch.offset(reg_idx as isize)).rm_so
                                                        == mctx.input.valid_len
                                                    {
                                                        mctx.input.valid_raw_len
                                                    } else {
                                                        *(mctx.input.offsets)
                                                            .offset((*pmatch.offset(reg_idx as isize)).rm_so as isize)
                                                    };
                                                    (*pmatch.offset(reg_idx as isize))
                                                        .rm_eo = if (*pmatch.offset(reg_idx as isize)).rm_eo
                                                        == mctx.input.valid_len
                                                    {
                                                        mctx.input.valid_raw_len
                                                    } else {
                                                        *(mctx.input.offsets)
                                                            .offset((*pmatch.offset(reg_idx as isize)).rm_eo as isize)
                                                    };
                                                }
                                                let ref mut fresh58 = (*pmatch.offset(reg_idx as isize))
                                                    .rm_so;
                                                *fresh58 += match_first;
                                                let ref mut fresh59 = (*pmatch.offset(reg_idx as isize))
                                                    .rm_eo;
                                                *fresh59 += match_first;
                                            }
                                            reg_idx += 1;
                                            reg_idx;
                                        }
                                        reg_idx = 0 as libc::c_int as Idx;
                                        while reg_idx < extra_nmatch {
                                            (*pmatch
                                                .offset(
                                                    nmatch.wrapping_add(reg_idx as libc::c_ulong) as isize,
                                                ))
                                                .rm_so = -(1 as libc::c_int) as regoff_t;
                                            (*pmatch
                                                .offset(
                                                    nmatch.wrapping_add(reg_idx as libc::c_ulong) as isize,
                                                ))
                                                .rm_eo = -(1 as libc::c_int) as regoff_t;
                                            reg_idx += 1;
                                            reg_idx;
                                        }
                                        if !((*dfa).subexp_map).is_null() {
                                            reg_idx = 0 as libc::c_int as Idx;
                                            while ((reg_idx + 1 as libc::c_int as libc::c_long)
                                                as libc::c_ulong) < nmatch
                                            {
                                                if *((*dfa).subexp_map).offset(reg_idx as isize) != reg_idx
                                                {
                                                    (*pmatch
                                                        .offset(
                                                            (reg_idx + 1 as libc::c_int as libc::c_long) as isize,
                                                        ))
                                                        .rm_so = (*pmatch
                                                        .offset(
                                                            (*((*dfa).subexp_map).offset(reg_idx as isize)
                                                                + 1 as libc::c_int as libc::c_long) as isize,
                                                        ))
                                                        .rm_so;
                                                    (*pmatch
                                                        .offset(
                                                            (reg_idx + 1 as libc::c_int as libc::c_long) as isize,
                                                        ))
                                                        .rm_eo = (*pmatch
                                                        .offset(
                                                            (*((*dfa).subexp_map).offset(reg_idx as isize)
                                                                + 1 as libc::c_int as libc::c_long) as isize,
                                                        ))
                                                        .rm_eo;
                                                }
                                                reg_idx += 1;
                                                reg_idx;
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
    rpl_free(mctx.state_log as *mut libc::c_void);
    if (*dfa).nbackref != 0 {
        match_ctx_free(&mut mctx);
    }
    re_string_destruct(&mut mctx.input);
    return err;
}
unsafe extern "C" fn expand_bkref_cache(
    mut mctx: *mut re_match_context_t,
    mut cur_nodes: *mut re_node_set,
    mut cur_str: Idx,
    mut subexp_num: Idx,
    mut type_0: libc::c_int,
) -> reg_errcode_t {
    let mut current_block: u64;
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut cache_idx_start: Idx = search_cur_bkref_entry(mctx, cur_str);
    let mut ent: *mut re_backref_cache_entry = 0 as *mut re_backref_cache_entry;
    if cache_idx_start == -(1 as libc::c_int) as libc::c_long {
        return _REG_NOERROR;
    }
    '_restart: loop {
        ent = ((*mctx).bkref_ents).offset(cache_idx_start as isize);
        loop {
            let mut to_idx: Idx = 0;
            let mut next_node: Idx = 0;
            if !(re_node_set_contains(cur_nodes, (*ent).node) == 0) {
                to_idx = cur_str + (*ent).subexp_to - (*ent).subexp_from;
                if to_idx == cur_str {
                    let mut new_dests: re_node_set = re_node_set {
                        alloc: 0,
                        nelem: 0,
                        elems: 0 as *mut Idx,
                    };
                    let mut err2: reg_errcode_t = _REG_NOERROR;
                    let mut err3: reg_errcode_t = _REG_NOERROR;
                    next_node = *((*((*dfa).edests).offset((*ent).node as isize)).elems)
                        .offset(0 as libc::c_int as isize);
                    if !(re_node_set_contains(cur_nodes, next_node) != 0) {
                        err = re_node_set_init_1(&mut new_dests, next_node);
                        err2 = check_arrival_expand_ecl(
                            dfa,
                            &mut new_dests,
                            subexp_num,
                            type_0,
                        );
                        err3 = re_node_set_merge(cur_nodes, &mut new_dests);
                        rpl_free(new_dests.elems as *mut libc::c_void);
                        if (err as libc::c_int != _REG_NOERROR as libc::c_int
                            || err2 as libc::c_int != _REG_NOERROR as libc::c_int
                            || err3 as libc::c_int != _REG_NOERROR as libc::c_int)
                            as libc::c_int as libc::c_long != 0
                        {
                            err = (if err as libc::c_int != _REG_NOERROR as libc::c_int {
                                err as libc::c_int
                            } else if err2 as libc::c_int != _REG_NOERROR as libc::c_int
                            {
                                err2 as libc::c_int
                            } else {
                                err3 as libc::c_int
                            }) as reg_errcode_t;
                            return err;
                        }
                        break;
                    }
                } else {
                    let mut union_set: re_node_set = re_node_set {
                        alloc: 0,
                        nelem: 0,
                        elems: 0 as *mut Idx,
                    };
                    next_node = *((*dfa).nexts).offset((*ent).node as isize);
                    if !(*((*mctx).state_log).offset(to_idx as isize)).is_null() {
                        let mut ok: bool = false;
                        if re_node_set_contains(
                            &mut (**((*mctx).state_log).offset(to_idx as isize)).nodes,
                            next_node,
                        ) != 0
                        {
                            current_block = 17179679302217393232;
                        } else {
                            err = re_node_set_init_copy(
                                &mut union_set,
                                &mut (**((*mctx).state_log).offset(to_idx as isize)).nodes,
                            );
                            ok = re_node_set_insert(&mut union_set, next_node);
                            if (err as libc::c_int != _REG_NOERROR as libc::c_int || !ok)
                                as libc::c_int as libc::c_long != 0
                            {
                                rpl_free(union_set.elems as *mut libc::c_void);
                                err = (if err as libc::c_int != _REG_NOERROR as libc::c_int
                                {
                                    err as libc::c_int
                                } else {
                                    _REG_ESPACE as libc::c_int
                                }) as reg_errcode_t;
                                return err;
                            }
                            current_block = 7056779235015430508;
                        }
                    } else {
                        err = re_node_set_init_1(&mut union_set, next_node);
                        if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                            as libc::c_int as libc::c_long != 0
                        {
                            return err;
                        }
                        current_block = 7056779235015430508;
                    }
                    match current_block {
                        17179679302217393232 => {}
                        _ => {
                            let ref mut fresh60 = *((*mctx).state_log)
                                .offset(to_idx as isize);
                            *fresh60 = re_acquire_state(&mut err, dfa, &mut union_set);
                            rpl_free(union_set.elems as *mut libc::c_void);
                            if ((*((*mctx).state_log).offset(to_idx as isize)).is_null()
                                && err as libc::c_int != _REG_NOERROR as libc::c_int)
                                as libc::c_int as libc::c_long != 0
                            {
                                return err;
                            }
                        }
                    }
                }
            }
            let fresh61 = ent;
            ent = ent.offset(1);
            if !((*fresh61).more != 0) {
                break '_restart;
            }
        }
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn group_nodes_into_DFAstates(
    mut dfa: *const re_dfa_t,
    mut state: *const re_dfastate_t,
    mut dests_node: *mut re_node_set,
    mut dests_ch: *mut bitset_t,
) -> Idx {
    let mut current_block: u64;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut ok: bool = false;
    let mut i: Idx = 0;
    let mut j: Idx = 0;
    let mut k: Idx = 0;
    let mut ndests: Idx = 0;
    let mut accepts: bitset_t = [0; 4];
    let mut cur_nodes: *const re_node_set = &(*state).nodes;
    bitset_empty(accepts.as_mut_ptr());
    ndests = 0 as libc::c_int as Idx;
    i = 0 as libc::c_int as Idx;
    's_20: loop {
        if !(i < (*cur_nodes).nelem) {
            current_block = 14648606000749551097;
            break;
        }
        let mut node: *mut re_token_t = &mut *((*dfa).nodes)
            .offset(*((*cur_nodes).elems).offset(i as isize) as isize)
            as *mut re_token_t;
        let mut type_0: re_token_type_t = (*node).type_0();
        let mut constraint: libc::c_uint = (*node).constraint();
        if type_0 as libc::c_uint == CHARACTER as libc::c_int as libc::c_uint {
            bitset_set(accepts.as_mut_ptr(), (*node).opr.c as Idx);
            current_block = 16203760046146113240;
        } else if type_0 as libc::c_uint == SIMPLE_BRACKET as libc::c_int as libc::c_uint
        {
            bitset_merge(
                accepts.as_mut_ptr(),
                (*node).opr.sbcset as *const bitset_word_t,
            );
            current_block = 16203760046146113240;
        } else if type_0 as libc::c_uint == OP_PERIOD as libc::c_int as libc::c_uint {
            if (*dfa).mb_cur_max > 1 as libc::c_int {
                bitset_merge(
                    accepts.as_mut_ptr(),
                    (*dfa).sb_char as *const bitset_word_t,
                );
            } else {
                bitset_set_all(accepts.as_mut_ptr());
            }
            if (*dfa).syntax
                & ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int == 0
            {
                bitset_clear(accepts.as_mut_ptr(), '\n' as i32 as Idx);
            }
            if (*dfa).syntax
                & (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int != 0
            {
                bitset_clear(accepts.as_mut_ptr(), '\0' as i32 as Idx);
            }
            current_block = 16203760046146113240;
        } else if type_0 as libc::c_uint == OP_UTF8_PERIOD as libc::c_int as libc::c_uint
        {
            if 0x80 as libc::c_int % 64 as libc::c_int == 0 as libc::c_int {
                memset(
                    accepts.as_mut_ptr() as *mut libc::c_void,
                    -(1 as libc::c_int),
                    (0x80 as libc::c_int / 8 as libc::c_int) as libc::c_ulong,
                );
            } else {
                bitset_merge(accepts.as_mut_ptr(), utf8_sb_map.as_ptr());
            }
            if (*dfa).syntax
                & ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int == 0
            {
                bitset_clear(accepts.as_mut_ptr(), '\n' as i32 as Idx);
            }
            if (*dfa).syntax
                & (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int != 0
            {
                bitset_clear(accepts.as_mut_ptr(), '\0' as i32 as Idx);
            }
            current_block = 16203760046146113240;
        } else {
            current_block = 17179679302217393232;
        }
        match current_block {
            16203760046146113240 => {
                if constraint != 0 {
                    if constraint & 0x20 as libc::c_int as libc::c_uint != 0 {
                        let mut accepts_newline: bool = bitset_contain(
                            accepts.as_mut_ptr() as *const bitset_word_t,
                            '\n' as i32 as Idx,
                        );
                        bitset_empty(accepts.as_mut_ptr());
                        if accepts_newline {
                            bitset_set(accepts.as_mut_ptr(), '\n' as i32 as Idx);
                            current_block = 11932355480408055363;
                        } else {
                            current_block = 17179679302217393232;
                        }
                    } else {
                        current_block = 11932355480408055363;
                    }
                    match current_block {
                        17179679302217393232 => {}
                        _ => {
                            if constraint & 0x80 as libc::c_int as libc::c_uint != 0 {
                                bitset_empty(accepts.as_mut_ptr());
                                current_block = 17179679302217393232;
                            } else {
                                if constraint & 0x4 as libc::c_int as libc::c_uint != 0 {
                                    let mut any_set: bitset_word_t = 0 as libc::c_int
                                        as bitset_word_t;
                                    if type_0 as libc::c_uint
                                        == CHARACTER as libc::c_int as libc::c_uint
                                        && (*node).word_char() == 0
                                    {
                                        bitset_empty(accepts.as_mut_ptr());
                                        current_block = 17179679302217393232;
                                    } else {
                                        if (*dfa).mb_cur_max > 1 as libc::c_int {
                                            j = 0 as libc::c_int as Idx;
                                            while j
                                                < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                                    + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                                                    / 64 as libc::c_int) as libc::c_long
                                            {
                                                accepts[j as usize]
                                                    &= (*dfa).word_char[j as usize]
                                                        | !*((*dfa).sb_char).offset(j as isize);
                                                any_set |= accepts[j as usize];
                                                j += 1;
                                                j;
                                            }
                                        } else {
                                            j = 0 as libc::c_int as Idx;
                                            while j
                                                < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                                    + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                                                    / 64 as libc::c_int) as libc::c_long
                                            {
                                                accepts[j as usize] &= (*dfa).word_char[j as usize];
                                                any_set |= accepts[j as usize];
                                                j += 1;
                                                j;
                                            }
                                        }
                                        if any_set == 0 {
                                            current_block = 17179679302217393232;
                                        } else {
                                            current_block = 721385680381463314;
                                        }
                                    }
                                } else {
                                    current_block = 721385680381463314;
                                }
                                match current_block {
                                    17179679302217393232 => {}
                                    _ => {
                                        if constraint & 0x8 as libc::c_int as libc::c_uint != 0 {
                                            let mut any_set_0: bitset_word_t = 0 as libc::c_int
                                                as bitset_word_t;
                                            if type_0 as libc::c_uint
                                                == CHARACTER as libc::c_int as libc::c_uint
                                                && (*node).word_char() as libc::c_int != 0
                                            {
                                                bitset_empty(accepts.as_mut_ptr());
                                                current_block = 17179679302217393232;
                                            } else {
                                                if (*dfa).mb_cur_max > 1 as libc::c_int {
                                                    j = 0 as libc::c_int as Idx;
                                                    while j
                                                        < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                                            + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                                                            / 64 as libc::c_int) as libc::c_long
                                                    {
                                                        accepts[j as usize]
                                                            &= !((*dfa).word_char[j as usize]
                                                                & *((*dfa).sb_char).offset(j as isize));
                                                        any_set_0 |= accepts[j as usize];
                                                        j += 1;
                                                        j;
                                                    }
                                                } else {
                                                    j = 0 as libc::c_int as Idx;
                                                    while j
                                                        < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                                            + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                                                            / 64 as libc::c_int) as libc::c_long
                                                    {
                                                        accepts[j as usize] &= !(*dfa).word_char[j as usize];
                                                        any_set_0 |= accepts[j as usize];
                                                        j += 1;
                                                        j;
                                                    }
                                                }
                                                if any_set_0 == 0 {
                                                    current_block = 17179679302217393232;
                                                } else {
                                                    current_block = 10380409671385728102;
                                                }
                                            }
                                        } else {
                                            current_block = 10380409671385728102;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    current_block = 10380409671385728102;
                }
                match current_block {
                    17179679302217393232 => {}
                    _ => {
                        j = 0 as libc::c_int as Idx;
                        while j < ndests {
                            let mut intersec: bitset_t = [0; 4];
                            let mut remains: bitset_t = [0; 4];
                            let mut has_intersec: bitset_word_t = 0;
                            let mut not_subset: bitset_word_t = 0;
                            let mut not_consumed: bitset_word_t = 0;
                            if !(type_0 as libc::c_uint
                                == CHARACTER as libc::c_int as libc::c_uint
                                && !bitset_contain(
                                    (*dests_ch.offset(j as isize)).as_mut_ptr()
                                        as *const bitset_word_t,
                                    (*node).opr.c as Idx,
                                ))
                            {
                                has_intersec = 0 as libc::c_int as bitset_word_t;
                                k = 0 as libc::c_int as Idx;
                                while k
                                    < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                        + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                                        / 64 as libc::c_int) as libc::c_long
                                {
                                    intersec[k
                                        as usize] = accepts[k as usize]
                                        & (*dests_ch.offset(j as isize))[k as usize];
                                    has_intersec |= intersec[k as usize];
                                    k += 1;
                                    k;
                                }
                                if !(has_intersec == 0) {
                                    not_consumed = 0 as libc::c_int as bitset_word_t;
                                    not_subset = not_consumed;
                                    k = 0 as libc::c_int as Idx;
                                    while k
                                        < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                            + 1 as libc::c_int + 64 as libc::c_int - 1 as libc::c_int)
                                            / 64 as libc::c_int) as libc::c_long
                                    {
                                        remains[k
                                            as usize] = !accepts[k as usize]
                                            & (*dests_ch.offset(j as isize))[k as usize];
                                        not_subset |= remains[k as usize];
                                        accepts[k
                                            as usize] = accepts[k as usize]
                                            & !(*dests_ch.offset(j as isize))[k as usize];
                                        not_consumed |= accepts[k as usize];
                                        k += 1;
                                        k;
                                    }
                                    if not_subset != 0 {
                                        bitset_copy(
                                            (*dests_ch.offset(ndests as isize)).as_mut_ptr(),
                                            remains.as_mut_ptr() as *const bitset_word_t,
                                        );
                                        bitset_copy(
                                            (*dests_ch.offset(j as isize)).as_mut_ptr(),
                                            intersec.as_mut_ptr() as *const bitset_word_t,
                                        );
                                        err = re_node_set_init_copy(
                                            dests_node.offset(ndests as isize),
                                            &mut *dests_node.offset(j as isize),
                                        );
                                        if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                            as libc::c_int as libc::c_long != 0
                                        {
                                            current_block = 1538840426499896341;
                                            break 's_20;
                                        }
                                        ndests += 1;
                                        ndests;
                                    }
                                    ok = re_node_set_insert(
                                        &mut *dests_node.offset(j as isize),
                                        *((*cur_nodes).elems).offset(i as isize),
                                    );
                                    if !ok as libc::c_int as libc::c_long != 0 {
                                        current_block = 1538840426499896341;
                                        break 's_20;
                                    }
                                    if not_consumed == 0 {
                                        break;
                                    }
                                }
                            }
                            j += 1;
                            j;
                        }
                        if j == ndests {
                            bitset_copy(
                                (*dests_ch.offset(ndests as isize)).as_mut_ptr(),
                                accepts.as_mut_ptr() as *const bitset_word_t,
                            );
                            err = re_node_set_init_1(
                                dests_node.offset(ndests as isize),
                                *((*cur_nodes).elems).offset(i as isize),
                            );
                            if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                as libc::c_int as libc::c_long != 0
                            {
                                current_block = 1538840426499896341;
                                break;
                            }
                            ndests += 1;
                            ndests;
                            bitset_empty(accepts.as_mut_ptr());
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    match current_block {
        1538840426499896341 => {
            j = 0 as libc::c_int as Idx;
            while j < ndests {
                rpl_free((*dests_node.offset(j as isize)).elems as *mut libc::c_void);
                j += 1;
                j;
            }
            return -(1 as libc::c_int) as Idx;
        }
        _ => {
            if ndests
                <= (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int) as libc::c_long
            {} else {
                unreachable!();
            };
            return ndests;
        }
    };
}
unsafe extern "C" fn check_arrival(
    mut mctx: *mut re_match_context_t,
    mut path: *mut state_array_t,
    mut top_node: Idx,
    mut top_str: Idx,
    mut last_node: Idx,
    mut last_str: Idx,
    mut type_0: libc::c_int,
) -> reg_errcode_t {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut subexp_num: Idx = 0;
    let mut backup_cur_idx: Idx = 0;
    let mut str_idx: Idx = 0;
    let mut null_cnt: Idx = 0;
    let mut cur_state: *mut re_dfastate_t = 0 as *mut re_dfastate_t;
    let mut cur_nodes: *mut re_node_set = 0 as *mut re_node_set;
    let mut next_nodes: re_node_set = re_node_set {
        alloc: 0,
        nelem: 0,
        elems: 0 as *mut Idx,
    };
    let mut backup_state_log: *mut *mut re_dfastate_t = 0 as *mut *mut re_dfastate_t;
    let mut context: libc::c_uint = 0;
    subexp_num = (*((*dfa).nodes).offset(top_node as isize)).opr.idx;
    if ((*path).alloc
        < last_str + (*mctx).max_mb_elem_len as libc::c_long
            + 1 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0
    {
        let mut new_array: *mut *mut re_dfastate_t = 0 as *mut *mut re_dfastate_t;
        let mut old_alloc: Idx = (*path).alloc;
        let mut incr_alloc: Idx = last_str + (*mctx).max_mb_elem_len as libc::c_long
            + 1 as libc::c_int as libc::c_long;
        let mut new_alloc: Idx = 0;
        if (9223372036854775807 as libc::c_long - old_alloc < incr_alloc) as libc::c_int
            as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        new_alloc = old_alloc + incr_alloc;
        if ((18446744073709551615 as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong)
            < new_alloc as libc::c_ulong) as libc::c_int as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        new_array = realloc(
            (*path).array as *mut libc::c_void,
            (new_alloc as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong,
                ),
        ) as *mut *mut re_dfastate_t;
        if (new_array == 0 as *mut libc::c_void as *mut *mut re_dfastate_t)
            as libc::c_int as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        (*path).array = new_array;
        (*path).alloc = new_alloc;
        memset(
            new_array.offset(old_alloc as isize) as *mut libc::c_void,
            '\0' as i32,
            (::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong)
                .wrapping_mul(((*path).alloc - old_alloc) as libc::c_ulong),
        );
    }
    str_idx = if (*path).next_idx != 0 { (*path).next_idx } else { top_str };
    backup_state_log = (*mctx).state_log;
    backup_cur_idx = (*mctx).input.cur_idx;
    (*mctx).state_log = (*path).array;
    (*mctx).input.cur_idx = str_idx;
    context = re_string_context_at(
        &mut (*mctx).input,
        str_idx - 1 as libc::c_int as libc::c_long,
        (*mctx).eflags,
    );
    if str_idx == top_str {
        err = re_node_set_init_1(&mut next_nodes, top_node);
        if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return err;
        }
        err = check_arrival_expand_ecl(dfa, &mut next_nodes, subexp_num, type_0);
        if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            rpl_free(next_nodes.elems as *mut libc::c_void);
            return err;
        }
    } else {
        cur_state = *((*mctx).state_log).offset(str_idx as isize);
        if !cur_state.is_null() && (*cur_state).has_backref() as libc::c_int != 0 {
            err = re_node_set_init_copy(&mut next_nodes, &mut (*cur_state).nodes);
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return err;
            }
        } else {
            memset(
                &mut next_nodes as *mut re_node_set as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<re_node_set>() as libc::c_ulong,
            );
        }
    }
    if str_idx == top_str
        || !cur_state.is_null() && (*cur_state).has_backref() as libc::c_int != 0
    {
        if next_nodes.nelem != 0 {
            err = expand_bkref_cache(mctx, &mut next_nodes, str_idx, subexp_num, type_0);
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                rpl_free(next_nodes.elems as *mut libc::c_void);
                return err;
            }
        }
        cur_state = re_acquire_state_context(&mut err, dfa, &mut next_nodes, context);
        if (cur_state.is_null() && err as libc::c_int != _REG_NOERROR as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            rpl_free(next_nodes.elems as *mut libc::c_void);
            return err;
        }
        let ref mut fresh62 = *((*mctx).state_log).offset(str_idx as isize);
        *fresh62 = cur_state;
    }
    null_cnt = 0 as libc::c_int as Idx;
    while str_idx < last_str && null_cnt <= (*mctx).max_mb_elem_len as libc::c_long {
        next_nodes.nelem = 0 as libc::c_int as Idx;
        if !(*((*mctx).state_log)
            .offset((str_idx + 1 as libc::c_int as libc::c_long) as isize))
            .is_null()
        {
            err = re_node_set_merge(
                &mut next_nodes,
                &mut (**((*mctx).state_log)
                    .offset((str_idx + 1 as libc::c_int as libc::c_long) as isize))
                    .nodes,
            );
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                rpl_free(next_nodes.elems as *mut libc::c_void);
                return err;
            }
        }
        if !cur_state.is_null() {
            err = check_arrival_add_next_nodes(
                mctx,
                str_idx,
                &mut (*cur_state).non_eps_nodes,
                &mut next_nodes,
            );
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                rpl_free(next_nodes.elems as *mut libc::c_void);
                return err;
            }
        }
        str_idx += 1;
        str_idx;
        if next_nodes.nelem != 0 {
            err = check_arrival_expand_ecl(dfa, &mut next_nodes, subexp_num, type_0);
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                rpl_free(next_nodes.elems as *mut libc::c_void);
                return err;
            }
            err = expand_bkref_cache(mctx, &mut next_nodes, str_idx, subexp_num, type_0);
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                rpl_free(next_nodes.elems as *mut libc::c_void);
                return err;
            }
        }
        context = re_string_context_at(
            &mut (*mctx).input,
            str_idx - 1 as libc::c_int as libc::c_long,
            (*mctx).eflags,
        );
        cur_state = re_acquire_state_context(&mut err, dfa, &mut next_nodes, context);
        if (cur_state.is_null() && err as libc::c_int != _REG_NOERROR as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            rpl_free(next_nodes.elems as *mut libc::c_void);
            return err;
        }
        let ref mut fresh63 = *((*mctx).state_log).offset(str_idx as isize);
        *fresh63 = cur_state;
        null_cnt = if cur_state.is_null() {
            null_cnt + 1 as libc::c_int as libc::c_long
        } else {
            0 as libc::c_int as libc::c_long
        };
    }
    rpl_free(next_nodes.elems as *mut libc::c_void);
    cur_nodes = if (*((*mctx).state_log).offset(last_str as isize)).is_null() {
        0 as *mut re_node_set
    } else {
        &mut (**((*mctx).state_log).offset(last_str as isize)).nodes
    };
    (*path).next_idx = str_idx;
    (*mctx).state_log = backup_state_log;
    (*mctx).input.cur_idx = backup_cur_idx;
    if !cur_nodes.is_null() && re_node_set_contains(cur_nodes, last_node) != 0 {
        return _REG_NOERROR;
    }
    return _REG_NOMATCH;
}
unsafe extern "C" fn match_ctx_add_entry(
    mut mctx: *mut re_match_context_t,
    mut node: Idx,
    mut str_idx: Idx,
    mut from: Idx,
    mut to: Idx,
) -> reg_errcode_t {
    if (*mctx).nbkref_ents >= (*mctx).abkref_ents {
        let mut new_entry: *mut re_backref_cache_entry = 0
            as *mut re_backref_cache_entry;
        new_entry = realloc(
            (*mctx).bkref_ents as *mut libc::c_void,
            (((*mctx).abkref_ents * 2 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<re_backref_cache_entry>() as libc::c_ulong,
                ),
        ) as *mut re_backref_cache_entry;
        if (new_entry == 0 as *mut libc::c_void as *mut re_backref_cache_entry)
            as libc::c_int as libc::c_long != 0
        {
            rpl_free((*mctx).bkref_ents as *mut libc::c_void);
            return _REG_ESPACE;
        }
        (*mctx).bkref_ents = new_entry;
        memset(
            ((*mctx).bkref_ents).offset((*mctx).nbkref_ents as isize)
                as *mut libc::c_void,
            '\0' as i32,
            (::core::mem::size_of::<re_backref_cache_entry>() as libc::c_ulong)
                .wrapping_mul((*mctx).abkref_ents as libc::c_ulong),
        );
        (*mctx).abkref_ents *= 2 as libc::c_int as libc::c_long;
    }
    if (*mctx).nbkref_ents > 0 as libc::c_int as libc::c_long
        && (*((*mctx).bkref_ents)
            .offset(((*mctx).nbkref_ents - 1 as libc::c_int as libc::c_long) as isize))
            .str_idx == str_idx
    {
        (*((*mctx).bkref_ents)
            .offset(((*mctx).nbkref_ents - 1 as libc::c_int as libc::c_long) as isize))
            .more = 1 as libc::c_int as libc::c_char;
    }
    (*((*mctx).bkref_ents).offset((*mctx).nbkref_ents as isize)).node = node;
    (*((*mctx).bkref_ents).offset((*mctx).nbkref_ents as isize)).str_idx = str_idx;
    (*((*mctx).bkref_ents).offset((*mctx).nbkref_ents as isize)).subexp_from = from;
    (*((*mctx).bkref_ents).offset((*mctx).nbkref_ents as isize)).subexp_to = to;
    (*((*mctx).bkref_ents).offset((*mctx).nbkref_ents as isize))
        .eps_reachable_subexps_map = (if from == to {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    }) as bitset_word_t;
    let fresh64 = (*mctx).nbkref_ents;
    (*mctx).nbkref_ents = (*mctx).nbkref_ents + 1;
    (*((*mctx).bkref_ents).offset(fresh64 as isize))
        .more = 0 as libc::c_int as libc::c_char;
    if ((*mctx).max_mb_elem_len as libc::c_long) < to - from {
        (*mctx).max_mb_elem_len = (to - from) as libc::c_int;
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn transit_state_mb(
    mut mctx: *mut re_match_context_t,
    mut pstate: *mut re_dfastate_t,
) -> reg_errcode_t {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut i: Idx = 0;
    let mut current_block_22: u64;
    i = 0 as libc::c_int as Idx;
    while i < (*pstate).nodes.nelem {
        let mut dest_nodes: re_node_set = re_node_set {
            alloc: 0,
            nelem: 0,
            elems: 0 as *mut Idx,
        };
        let mut new_nodes: *mut re_node_set = 0 as *mut re_node_set;
        let mut cur_node_idx: Idx = *((*pstate).nodes.elems).offset(i as isize);
        let mut naccepted: libc::c_int = 0;
        let mut dest_idx: Idx = 0;
        let mut context: libc::c_uint = 0;
        let mut dest_state: *mut re_dfastate_t = 0 as *mut re_dfastate_t;
        if !((*((*dfa).nodes).offset(cur_node_idx as isize)).accept_mb() == 0) {
            if (*((*dfa).nodes).offset(cur_node_idx as isize)).constraint() != 0 {
                context = re_string_context_at(
                    &mut (*mctx).input,
                    (*mctx).input.cur_idx,
                    (*mctx).eflags,
                );
                if (*((*dfa).nodes).offset(cur_node_idx as isize)).constraint()
                    as libc::c_int & 0x4 as libc::c_int != 0
                    && context & 1 as libc::c_int as libc::c_uint == 0
                    || (*((*dfa).nodes).offset(cur_node_idx as isize)).constraint()
                        as libc::c_int & 0x8 as libc::c_int != 0
                        && context & 1 as libc::c_int as libc::c_uint != 0
                    || (*((*dfa).nodes).offset(cur_node_idx as isize)).constraint()
                        as libc::c_int & 0x20 as libc::c_int != 0
                        && context
                            & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                            == 0
                    || (*((*dfa).nodes).offset(cur_node_idx as isize)).constraint()
                        as libc::c_int & 0x80 as libc::c_int != 0
                        && context
                            & ((((1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                            == 0
                {
                    current_block_22 = 6239978542346980191;
                } else {
                    current_block_22 = 3276175668257526147;
                }
            } else {
                current_block_22 = 3276175668257526147;
            }
            match current_block_22 {
                6239978542346980191 => {}
                _ => {
                    naccepted = check_node_accept_bytes(
                        dfa,
                        cur_node_idx,
                        &mut (*mctx).input,
                        (*mctx).input.cur_idx,
                    );
                    if !(naccepted == 0 as libc::c_int) {
                        dest_idx = (*mctx).input.cur_idx + naccepted as libc::c_long;
                        (*mctx)
                            .max_mb_elem_len = if (*mctx).max_mb_elem_len < naccepted {
                            naccepted
                        } else {
                            (*mctx).max_mb_elem_len
                        };
                        err = clean_state_log_if_needed(mctx, dest_idx);
                        if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                            as libc::c_int as libc::c_long != 0
                        {
                            return err;
                        }
                        if *((*dfa).nexts).offset(cur_node_idx as isize)
                            != -(1 as libc::c_int) as libc::c_long
                        {} else {
                            unreachable!();
                        };
                        new_nodes = ((*dfa).eclosures)
                            .offset(
                                *((*dfa).nexts).offset(cur_node_idx as isize) as isize,
                            );
                        dest_state = *((*mctx).state_log).offset(dest_idx as isize);
                        if dest_state.is_null() {
                            dest_nodes = *new_nodes;
                        } else {
                            err = re_node_set_init_union(
                                &mut dest_nodes,
                                (*dest_state).entrance_nodes,
                                new_nodes,
                            );
                            if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                as libc::c_int as libc::c_long != 0
                            {
                                return err;
                            }
                        }
                        context = re_string_context_at(
                            &mut (*mctx).input,
                            dest_idx - 1 as libc::c_int as libc::c_long,
                            (*mctx).eflags,
                        );
                        let ref mut fresh65 = *((*mctx).state_log)
                            .offset(dest_idx as isize);
                        *fresh65 = re_acquire_state_context(
                            &mut err,
                            dfa,
                            &mut dest_nodes,
                            context,
                        );
                        if !dest_state.is_null() {
                            rpl_free(dest_nodes.elems as *mut libc::c_void);
                        }
                        if ((*((*mctx).state_log).offset(dest_idx as isize)).is_null()
                            && err as libc::c_int != _REG_NOERROR as libc::c_int)
                            as libc::c_int as libc::c_long != 0
                        {
                            return err;
                        }
                    }
                }
            }
        }
        i += 1;
        i;
    }
    return _REG_NOERROR;
}
#[inline(always)]
unsafe extern "C" fn acquire_init_state_context(
    mut err: *mut reg_errcode_t,
    mut mctx: *const re_match_context_t,
    mut idx: Idx,
) -> *mut re_dfastate_t {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    if (*(*dfa).init_state).has_constraint() != 0 {
        let mut context: libc::c_uint = 0;
        context = re_string_context_at(
            &(*mctx).input,
            idx - 1 as libc::c_int as libc::c_long,
            (*mctx).eflags,
        );
        if context & 1 as libc::c_int as libc::c_uint != 0 {
            return (*dfa).init_state_word
        } else if context == 0 as libc::c_int as libc::c_uint {
            return (*dfa).init_state
        } else if context
            & (((1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                as libc::c_uint != 0
            && context & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0
        {
            return (*dfa).init_state_begbuf
        } else if context & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0
        {
            return (*dfa).init_state_nl
        } else if context
            & (((1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                as libc::c_uint != 0
        {
            return re_acquire_state_context(
                err,
                dfa,
                (*(*dfa).init_state).entrance_nodes,
                context,
            )
        } else {
            return (*dfa).init_state
        }
    } else {
        return (*dfa).init_state
    };
}
unsafe extern "C" fn extend_buffers(
    mut mctx: *mut re_match_context_t,
    mut min_len: libc::c_int,
) -> reg_errcode_t {
    let mut ret: reg_errcode_t = _REG_NOERROR;
    let mut pstr: *mut re_string_t = &mut (*mctx).input;
    if ((if (9223372036854775807 as libc::c_long as libc::c_ulong)
        < (18446744073709551615 as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong)
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        (18446744073709551615 as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong)
    })
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        <= (*pstr).bufs_len as libc::c_ulong) as libc::c_int as libc::c_long != 0
    {
        return _REG_ESPACE;
    }
    ret = re_string_realloc_buffers(
        pstr,
        if (min_len as libc::c_long)
            < (if (*pstr).len < (*pstr).bufs_len * 2 as libc::c_int as libc::c_long {
                (*pstr).len
            } else {
                (*pstr).bufs_len * 2 as libc::c_int as libc::c_long
            })
        {
            if (*pstr).len < (*pstr).bufs_len * 2 as libc::c_int as libc::c_long {
                (*pstr).len
            } else {
                (*pstr).bufs_len * 2 as libc::c_int as libc::c_long
            }
        } else {
            min_len as libc::c_long
        },
    );
    if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return ret;
    }
    if !((*mctx).state_log).is_null() {
        let mut new_array: *mut *mut re_dfastate_t = realloc(
            (*mctx).state_log as *mut libc::c_void,
            (((*pstr).bufs_len + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong,
                ),
        ) as *mut *mut re_dfastate_t;
        if (new_array == 0 as *mut libc::c_void as *mut *mut re_dfastate_t)
            as libc::c_int as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        (*mctx).state_log = new_array;
    }
    if (*pstr).icase != 0 {
        if (*pstr).mb_cur_max > 1 as libc::c_int {
            ret = build_wcs_upper_buffer(pstr);
            if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return ret;
            }
        } else {
            build_upper_buffer(pstr);
        }
    } else if (*pstr).mb_cur_max > 1 as libc::c_int {
        build_wcs_buffer(pstr);
    } else if !((*pstr).trans).is_null() {
        re_string_translate_buffer(pstr);
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn match_ctx_init(
    mut mctx: *mut re_match_context_t,
    mut eflags: libc::c_int,
    mut n: Idx,
) -> reg_errcode_t {
    (*mctx).eflags = eflags;
    (*mctx).match_last = -(1 as libc::c_int) as Idx;
    if n > 0 as libc::c_int as libc::c_long {
        let mut max_object_size: size_t = if (::core::mem::size_of::<
            re_backref_cache_entry,
        >() as libc::c_ulong)
            < ::core::mem::size_of::<*mut re_sub_match_top_t>() as libc::c_ulong
        {
            ::core::mem::size_of::<*mut re_sub_match_top_t>() as libc::c_ulong
        } else {
            ::core::mem::size_of::<re_backref_cache_entry>() as libc::c_ulong
        };
        if ((if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < (18446744073709551615 as libc::c_ulong).wrapping_div(max_object_size)
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            (18446744073709551615 as libc::c_ulong).wrapping_div(max_object_size)
        }) < n as libc::c_ulong) as libc::c_int as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        (*mctx)
            .bkref_ents = malloc(
            (n as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<re_backref_cache_entry>() as libc::c_ulong,
                ),
        ) as *mut re_backref_cache_entry;
        (*mctx)
            .sub_tops = malloc(
            (n as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut re_sub_match_top_t>() as libc::c_ulong,
                ),
        ) as *mut *mut re_sub_match_top_t;
        if (((*mctx).bkref_ents).is_null() || ((*mctx).sub_tops).is_null())
            as libc::c_int as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
    }
    (*mctx).abkref_ents = n;
    (*mctx).max_mb_elem_len = 1 as libc::c_int;
    (*mctx).asub_tops = n;
    return _REG_NOERROR;
}
unsafe extern "C" fn clean_state_log_if_needed(
    mut mctx: *mut re_match_context_t,
    mut next_state_log_idx: Idx,
) -> reg_errcode_t {
    let mut top: Idx = (*mctx).state_log_top;
    if next_state_log_idx >= (*mctx).input.bufs_len
        && (*mctx).input.bufs_len < (*mctx).input.len
        || next_state_log_idx >= (*mctx).input.valid_len
            && (*mctx).input.valid_len < (*mctx).input.len
    {
        let mut err: reg_errcode_t = _REG_NOERROR;
        err = extend_buffers(
            mctx,
            (next_state_log_idx + 1 as libc::c_int as libc::c_long) as libc::c_int,
        );
        if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return err;
        }
    }
    if top < next_state_log_idx {
        if !((*mctx).state_log).is_null() {} else {
            unreachable!();
        };
        memset(
            ((*mctx).state_log).offset(top as isize).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            '\0' as i32,
            (::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong)
                .wrapping_mul((next_state_log_idx - top) as libc::c_ulong),
        );
        (*mctx).state_log_top = next_state_log_idx;
    }
    return _REG_NOERROR;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_re_search_2(
    mut bufp: *mut re_pattern_buffer,
    mut string1: *const libc::c_char,
    mut length1: Idx,
    mut string2: *const libc::c_char,
    mut length2: Idx,
    mut start: Idx,
    mut range: regoff_t,
    mut regs: *mut re_registers,
    mut stop: Idx,
) -> regoff_t {
    return re_search_2_stub(
        bufp,
        string1,
        length1,
        string2,
        length2,
        start,
        range,
        regs,
        stop,
        0 as libc::c_int != 0,
    );
}
unsafe extern "C" fn re_search_2_stub(
    mut bufp: *mut re_pattern_buffer,
    mut string1: *const libc::c_char,
    mut length1: Idx,
    mut string2: *const libc::c_char,
    mut length2: Idx,
    mut start: Idx,
    mut range: regoff_t,
    mut regs: *mut re_registers,
    mut stop: Idx,
    mut ret_len: bool,
) -> regoff_t {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut rval: regoff_t = 0;
    let mut len: Idx = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if (length1 < 0 as libc::c_int as libc::c_long
        || length2 < 0 as libc::c_int as libc::c_long
        || stop < 0 as libc::c_int as libc::c_long
        || {
            let (fresh66, fresh67) = length1.overflowing_add(length2);
            *(&mut len as *mut Idx) = fresh66;
            fresh67 as libc::c_int != 0
        }) as libc::c_int as libc::c_long != 0
    {
        return -(2 as libc::c_int) as regoff_t;
    }
    if length2 > 0 as libc::c_int as libc::c_long {
        if length1 > 0 as libc::c_int as libc::c_long {
            s = malloc(
                (len as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
            if (s == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
                as libc::c_long != 0
            {
                return -(2 as libc::c_int) as regoff_t;
            }
            memcpy(
                s as *mut libc::c_void,
                string1 as *const libc::c_void,
                length1 as libc::c_ulong,
            );
            memcpy(
                s.offset(length1 as isize) as *mut libc::c_void,
                string2 as *const libc::c_void,
                length2 as libc::c_ulong,
            );
            str = s;
        } else {
            str = string2;
        }
    } else {
        str = string1;
    }
    rval = re_search_stub(bufp, str, len, start, range, stop, regs, ret_len);
    rpl_free(s as *mut libc::c_void);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_re_match(
    mut bufp: *mut re_pattern_buffer,
    mut string: *const libc::c_char,
    mut length: Idx,
    mut start: Idx,
    mut regs: *mut re_registers,
) -> regoff_t {
    return re_search_stub(
        bufp,
        string,
        length,
        start,
        0 as libc::c_int as regoff_t,
        length,
        regs,
        1 as libc::c_int != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rpl_re_match_2(
    mut bufp: *mut re_pattern_buffer,
    mut string1: *const libc::c_char,
    mut length1: Idx,
    mut string2: *const libc::c_char,
    mut length2: Idx,
    mut start: Idx,
    mut regs: *mut re_registers,
    mut stop: Idx,
) -> regoff_t {
    return re_search_2_stub(
        bufp,
        string1,
        length1,
        string2,
        length2,
        start,
        0 as libc::c_int as regoff_t,
        regs,
        stop,
        1 as libc::c_int != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rpl_re_set_registers(
    mut bufp: *mut re_pattern_buffer,
    mut regs: *mut re_registers,
    mut num_regs: __re_size_t,
    mut starts: *mut regoff_t,
    mut ends: *mut regoff_t,
) {
    if num_regs != 0 {
        (*bufp).set_regs_allocated(1 as libc::c_int as libc::c_uint);
        (*regs).num_regs = num_regs;
        (*regs).start = starts;
        (*regs).end = ends;
    } else {
        (*bufp).set_regs_allocated(0 as libc::c_int as libc::c_uint);
        (*regs).num_regs = 0 as libc::c_int as __re_size_t;
        (*regs).end = 0 as *mut regoff_t;
        (*regs).start = (*regs).end;
    };
}
unsafe extern "C" fn get_subexp_sub(
    mut mctx: *mut re_match_context_t,
    mut sub_top: *const re_sub_match_top_t,
    mut sub_last: *mut re_sub_match_last_t,
    mut bkref_node: Idx,
    mut bkref_str: Idx,
) -> reg_errcode_t {
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut to_idx: Idx = 0;
    err = check_arrival(
        mctx,
        &mut (*sub_last).path,
        (*sub_last).node,
        (*sub_last).str_idx,
        bkref_node,
        bkref_str,
        OP_OPEN_SUBEXP as libc::c_int,
    );
    if err as libc::c_int != _REG_NOERROR as libc::c_int {
        return err;
    }
    err = match_ctx_add_entry(
        mctx,
        bkref_node,
        bkref_str,
        (*sub_top).str_idx,
        (*sub_last).str_idx,
    );
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return err;
    }
    to_idx = bkref_str + (*sub_last).str_idx - (*sub_top).str_idx;
    return clean_state_log_if_needed(mctx, to_idx);
}
unsafe extern "C" fn get_subexp(
    mut mctx: *mut re_match_context_t,
    mut bkref_node: Idx,
    mut bkref_str_idx: Idx,
) -> reg_errcode_t {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut subexp_num: Idx = 0;
    let mut sub_top_idx: Idx = 0;
    let mut buf: *const libc::c_char = (*mctx).input.mbs as *const libc::c_char;
    let mut cache_idx: Idx = search_cur_bkref_entry(mctx, bkref_str_idx);
    if cache_idx != -(1 as libc::c_int) as libc::c_long {
        let mut entry: *const re_backref_cache_entry = ((*mctx).bkref_ents)
            .offset(cache_idx as isize);
        loop {
            if (*entry).node == bkref_node {
                return _REG_NOERROR;
            }
            let fresh68 = entry;
            entry = entry.offset(1);
            if !((*fresh68).more != 0) {
                break;
            }
        }
    }
    subexp_num = (*((*dfa).nodes).offset(bkref_node as isize)).opr.idx;
    sub_top_idx = 0 as libc::c_int as Idx;
    while sub_top_idx < (*mctx).nsub_tops {
        let mut err: reg_errcode_t = _REG_NOERROR;
        let mut sub_top: *mut re_sub_match_top_t = *((*mctx).sub_tops)
            .offset(sub_top_idx as isize);
        let mut sub_last: *mut re_sub_match_last_t = 0 as *mut re_sub_match_last_t;
        let mut sub_last_idx: Idx = 0;
        let mut sl_str: Idx = 0;
        let mut bkref_str_off: Idx = 0;
        if !((*((*dfa).nodes).offset((*sub_top).node as isize)).opr.idx != subexp_num) {
            sl_str = (*sub_top).str_idx;
            bkref_str_off = bkref_str_idx;
            sub_last_idx = 0 as libc::c_int as Idx;
            while sub_last_idx < (*sub_top).nlasts {
                let mut sl_str_diff: regoff_t = 0;
                sub_last = *((*sub_top).lasts).offset(sub_last_idx as isize);
                sl_str_diff = (*sub_last).str_idx - sl_str;
                if sl_str_diff > 0 as libc::c_int as libc::c_long {
                    if (bkref_str_off + sl_str_diff > (*mctx).input.valid_len)
                        as libc::c_int as libc::c_long != 0
                    {
                        if bkref_str_off + sl_str_diff > (*mctx).input.len {
                            break;
                        }
                        err = clean_state_log_if_needed(
                            mctx,
                            bkref_str_off + sl_str_diff,
                        );
                        if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                            as libc::c_int as libc::c_long != 0
                        {
                            return err;
                        }
                        buf = (*mctx).input.mbs as *const libc::c_char;
                    }
                    if memcmp(
                        buf.offset(bkref_str_off as isize) as *const libc::c_void,
                        buf.offset(sl_str as isize) as *const libc::c_void,
                        sl_str_diff as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        break;
                    }
                }
                bkref_str_off += sl_str_diff;
                sl_str += sl_str_diff;
                err = get_subexp_sub(mctx, sub_top, sub_last, bkref_node, bkref_str_idx);
                buf = (*mctx).input.mbs as *const libc::c_char;
                if !(err as libc::c_int == _REG_NOMATCH as libc::c_int) {
                    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        return err;
                    }
                }
                sub_last_idx += 1;
                sub_last_idx;
            }
            if !(sub_last_idx < (*sub_top).nlasts) {
                if sub_last_idx > 0 as libc::c_int as libc::c_long {
                    sl_str += 1;
                    sl_str;
                }
                while sl_str <= bkref_str_idx {
                    let mut cls_node: Idx = 0;
                    let mut sl_str_off: regoff_t = 0;
                    let mut nodes: *const re_node_set = 0 as *const re_node_set;
                    sl_str_off = sl_str - (*sub_top).str_idx;
                    if sl_str_off > 0 as libc::c_int as libc::c_long {
                        if (bkref_str_off >= (*mctx).input.valid_len) as libc::c_int
                            as libc::c_long != 0
                        {
                            if bkref_str_off >= (*mctx).input.len {
                                break;
                            }
                            err = extend_buffers(
                                mctx,
                                (bkref_str_off + 1 as libc::c_int as libc::c_long)
                                    as libc::c_int,
                            );
                            if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                as libc::c_int as libc::c_long != 0
                            {
                                return err;
                            }
                            buf = (*mctx).input.mbs as *const libc::c_char;
                        }
                        let fresh69 = bkref_str_off;
                        bkref_str_off = bkref_str_off + 1;
                        if *buf.offset(fresh69 as isize) as libc::c_int
                            != *buf
                                .offset(
                                    (sl_str - 1 as libc::c_int as libc::c_long) as isize,
                                ) as libc::c_int
                        {
                            break;
                        }
                    }
                    if !(*((*mctx).state_log).offset(sl_str as isize)).is_null() {
                        nodes = &mut (**((*mctx).state_log).offset(sl_str as isize))
                            .nodes;
                        cls_node = find_subexp_node(
                            dfa,
                            nodes,
                            subexp_num,
                            OP_CLOSE_SUBEXP as libc::c_int,
                        );
                        if !(cls_node == -(1 as libc::c_int) as libc::c_long) {
                            if ((*sub_top).path).is_null() {
                                (*sub_top)
                                    .path = calloc(
                                    ::core::mem::size_of::<state_array_t>() as libc::c_ulong,
                                    (sl_str - (*sub_top).str_idx
                                        + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                                ) as *mut state_array_t;
                                if ((*sub_top).path).is_null() {
                                    return _REG_ESPACE;
                                }
                            }
                            err = check_arrival(
                                mctx,
                                (*sub_top).path,
                                (*sub_top).node,
                                (*sub_top).str_idx,
                                cls_node,
                                sl_str,
                                OP_CLOSE_SUBEXP as libc::c_int,
                            );
                            if !(err as libc::c_int == _REG_NOMATCH as libc::c_int) {
                                if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    return err;
                                }
                                sub_last = match_ctx_add_sublast(sub_top, cls_node, sl_str);
                                if (sub_last
                                    == 0 as *mut libc::c_void as *mut re_sub_match_last_t)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    return _REG_ESPACE;
                                }
                                err = get_subexp_sub(
                                    mctx,
                                    sub_top,
                                    sub_last,
                                    bkref_node,
                                    bkref_str_idx,
                                );
                                buf = (*mctx).input.mbs as *const libc::c_char;
                                if !(err as libc::c_int == _REG_NOMATCH as libc::c_int) {
                                    if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                        as libc::c_int as libc::c_long != 0
                                    {
                                        return err;
                                    }
                                }
                            }
                        }
                    }
                    sl_str += 1;
                    sl_str;
                }
            }
        }
        sub_top_idx += 1;
        sub_top_idx;
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn match_ctx_add_subtop(
    mut mctx: *mut re_match_context_t,
    mut node: Idx,
    mut str_idx: Idx,
) -> reg_errcode_t {
    if !((*mctx).sub_tops).is_null() {} else {
        unreachable!();
    };
    if (*mctx).asub_tops > 0 as libc::c_int as libc::c_long {} else {
        unreachable!();
    };
    if ((*mctx).nsub_tops == (*mctx).asub_tops) as libc::c_int as libc::c_long != 0 {
        let mut new_asub_tops: Idx = (*mctx).asub_tops
            * 2 as libc::c_int as libc::c_long;
        let mut new_array: *mut *mut re_sub_match_top_t = realloc(
            (*mctx).sub_tops as *mut libc::c_void,
            (new_asub_tops as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut re_sub_match_top_t>() as libc::c_ulong,
                ),
        ) as *mut *mut re_sub_match_top_t;
        if (new_array == 0 as *mut libc::c_void as *mut *mut re_sub_match_top_t)
            as libc::c_int as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        (*mctx).sub_tops = new_array;
        (*mctx).asub_tops = new_asub_tops;
    }
    let ref mut fresh70 = *((*mctx).sub_tops).offset((*mctx).nsub_tops as isize);
    *fresh70 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<re_sub_match_top_t>() as libc::c_ulong,
    ) as *mut re_sub_match_top_t;
    if (*((*mctx).sub_tops).offset((*mctx).nsub_tops as isize)
        == 0 as *mut libc::c_void as *mut re_sub_match_top_t) as libc::c_int
        as libc::c_long != 0
    {
        return _REG_ESPACE;
    }
    (**((*mctx).sub_tops).offset((*mctx).nsub_tops as isize)).node = node;
    let fresh71 = (*mctx).nsub_tops;
    (*mctx).nsub_tops = (*mctx).nsub_tops + 1;
    (**((*mctx).sub_tops).offset(fresh71 as isize)).str_idx = str_idx;
    return _REG_NOERROR;
}
unsafe extern "C" fn transit_state_bkref(
    mut mctx: *mut re_match_context_t,
    mut nodes: *const re_node_set,
) -> reg_errcode_t {
    let mut current_block: u64;
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut i: Idx = 0;
    let mut cur_str_idx: Idx = (*mctx).input.cur_idx;
    i = 0 as libc::c_int as Idx;
    's_10: loop {
        if !(i < (*nodes).nelem) {
            current_block = 9520865839495247062;
            break;
        }
        let mut dest_str_idx: Idx = 0;
        let mut prev_nelem: Idx = 0;
        let mut bkc_idx: Idx = 0;
        let mut node_idx: Idx = *((*nodes).elems).offset(i as isize);
        let mut context: libc::c_uint = 0;
        let mut node: *const re_token_t = ((*dfa).nodes).offset(node_idx as isize);
        let mut new_dest_nodes: *mut re_node_set = 0 as *mut re_node_set;
        if !((*node).type_0() as libc::c_int != OP_BACK_REF as libc::c_int) {
            if (*node).constraint() != 0 {
                context = re_string_context_at(
                    &mut (*mctx).input,
                    cur_str_idx,
                    (*mctx).eflags,
                );
                if (*node).constraint() as libc::c_int & 0x4 as libc::c_int != 0
                    && context & 1 as libc::c_int as libc::c_uint == 0
                    || (*node).constraint() as libc::c_int & 0x8 as libc::c_int != 0
                        && context & 1 as libc::c_int as libc::c_uint != 0
                    || (*node).constraint() as libc::c_int & 0x20 as libc::c_int != 0
                        && context
                            & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                            == 0
                    || (*node).constraint() as libc::c_int & 0x80 as libc::c_int != 0
                        && context
                            & ((((1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                            == 0
                {
                    current_block = 8258075665625361029;
                } else {
                    current_block = 3276175668257526147;
                }
            } else {
                current_block = 3276175668257526147;
            }
            match current_block {
                8258075665625361029 => {}
                _ => {
                    bkc_idx = (*mctx).nbkref_ents;
                    err = get_subexp(mctx, node_idx, cur_str_idx);
                    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        current_block = 18250392879517264745;
                        break;
                    }
                    if *((*dfa).nexts).offset(node_idx as isize)
                        != -(1 as libc::c_int) as libc::c_long
                    {} else {
                        unreachable!();
                    };
                    while bkc_idx < (*mctx).nbkref_ents {
                        let mut subexp_len: Idx = 0;
                        let mut dest_state: *mut re_dfastate_t = 0 as *mut re_dfastate_t;
                        let mut bkref_ent: *mut re_backref_cache_entry = 0
                            as *mut re_backref_cache_entry;
                        bkref_ent = ((*mctx).bkref_ents).offset(bkc_idx as isize);
                        if !((*bkref_ent).node != node_idx
                            || (*bkref_ent).str_idx != cur_str_idx)
                        {
                            subexp_len = (*bkref_ent).subexp_to
                                - (*bkref_ent).subexp_from;
                            new_dest_nodes = if subexp_len
                                == 0 as libc::c_int as libc::c_long
                            {
                                ((*dfa).eclosures)
                                    .offset(
                                        *((*((*dfa).edests).offset(node_idx as isize)).elems)
                                            .offset(0 as libc::c_int as isize) as isize,
                                    )
                            } else {
                                ((*dfa).eclosures)
                                    .offset(*((*dfa).nexts).offset(node_idx as isize) as isize)
                            };
                            dest_str_idx = cur_str_idx + (*bkref_ent).subexp_to
                                - (*bkref_ent).subexp_from;
                            context = re_string_context_at(
                                &mut (*mctx).input,
                                dest_str_idx - 1 as libc::c_int as libc::c_long,
                                (*mctx).eflags,
                            );
                            dest_state = *((*mctx).state_log)
                                .offset(dest_str_idx as isize);
                            prev_nelem = if (*((*mctx).state_log)
                                .offset(cur_str_idx as isize))
                                .is_null()
                            {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (**((*mctx).state_log).offset(cur_str_idx as isize))
                                    .nodes
                                    .nelem
                            };
                            if dest_state.is_null() {
                                let ref mut fresh72 = *((*mctx).state_log)
                                    .offset(dest_str_idx as isize);
                                *fresh72 = re_acquire_state_context(
                                    &mut err,
                                    dfa,
                                    new_dest_nodes,
                                    context,
                                );
                                if ((*((*mctx).state_log).offset(dest_str_idx as isize))
                                    .is_null()
                                    && err as libc::c_int != _REG_NOERROR as libc::c_int)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    current_block = 18250392879517264745;
                                    break 's_10;
                                }
                            } else {
                                let mut dest_nodes: re_node_set = re_node_set {
                                    alloc: 0,
                                    nelem: 0,
                                    elems: 0 as *mut Idx,
                                };
                                err = re_node_set_init_union(
                                    &mut dest_nodes,
                                    (*dest_state).entrance_nodes,
                                    new_dest_nodes,
                                );
                                if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    rpl_free(dest_nodes.elems as *mut libc::c_void);
                                    current_block = 18250392879517264745;
                                    break 's_10;
                                } else {
                                    let ref mut fresh73 = *((*mctx).state_log)
                                        .offset(dest_str_idx as isize);
                                    *fresh73 = re_acquire_state_context(
                                        &mut err,
                                        dfa,
                                        &mut dest_nodes,
                                        context,
                                    );
                                    rpl_free(dest_nodes.elems as *mut libc::c_void);
                                    if ((*((*mctx).state_log).offset(dest_str_idx as isize))
                                        .is_null()
                                        && err as libc::c_int != _REG_NOERROR as libc::c_int)
                                        as libc::c_int as libc::c_long != 0
                                    {
                                        current_block = 18250392879517264745;
                                        break 's_10;
                                    }
                                }
                            }
                            if subexp_len == 0 as libc::c_int as libc::c_long
                                && (**((*mctx).state_log).offset(cur_str_idx as isize))
                                    .nodes
                                    .nelem > prev_nelem
                            {
                                err = check_subexp_matching_top(
                                    mctx,
                                    new_dest_nodes,
                                    cur_str_idx,
                                );
                                if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    current_block = 18250392879517264745;
                                    break 's_10;
                                }
                                err = transit_state_bkref(mctx, new_dest_nodes);
                                if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    current_block = 18250392879517264745;
                                    break 's_10;
                                }
                            }
                        }
                        bkc_idx += 1;
                        bkc_idx;
                    }
                }
            }
        }
        i += 1;
        i;
    }
    match current_block {
        9520865839495247062 => {
            err = _REG_NOERROR;
        }
        _ => {}
    }
    return err;
}
unsafe extern "C" fn merge_state_with_log(
    mut err: *mut reg_errcode_t,
    mut mctx: *mut re_match_context_t,
    mut next_state: *mut re_dfastate_t,
) -> *mut re_dfastate_t {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut cur_idx: Idx = (*mctx).input.cur_idx;
    if cur_idx > (*mctx).state_log_top {
        let ref mut fresh74 = *((*mctx).state_log).offset(cur_idx as isize);
        *fresh74 = next_state;
        (*mctx).state_log_top = cur_idx;
    } else if (*((*mctx).state_log).offset(cur_idx as isize)).is_null() {
        let ref mut fresh75 = *((*mctx).state_log).offset(cur_idx as isize);
        *fresh75 = next_state;
    } else {
        let mut pstate: *mut re_dfastate_t = 0 as *mut re_dfastate_t;
        let mut context: libc::c_uint = 0;
        let mut next_nodes: re_node_set = re_node_set {
            alloc: 0,
            nelem: 0,
            elems: 0 as *mut Idx,
        };
        let mut log_nodes: *mut re_node_set = 0 as *mut re_node_set;
        let mut table_nodes: *mut re_node_set = 0 as *mut re_node_set;
        pstate = *((*mctx).state_log).offset(cur_idx as isize);
        log_nodes = (*pstate).entrance_nodes;
        if !next_state.is_null() {
            table_nodes = (*next_state).entrance_nodes;
            *err = re_node_set_init_union(&mut next_nodes, table_nodes, log_nodes);
            if (*err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return 0 as *mut re_dfastate_t;
            }
        } else {
            next_nodes = *log_nodes;
        }
        context = re_string_context_at(
            &mut (*mctx).input,
            (*mctx).input.cur_idx - 1 as libc::c_int as libc::c_long,
            (*mctx).eflags,
        );
        let ref mut fresh76 = *((*mctx).state_log).offset(cur_idx as isize);
        *fresh76 = re_acquire_state_context(err, dfa, &mut next_nodes, context);
        next_state = *fresh76;
        if !table_nodes.is_null() {
            rpl_free(next_nodes.elems as *mut libc::c_void);
        }
    }
    if (*dfa).nbackref != 0 && !next_state.is_null() {
        *err = check_subexp_matching_top(mctx, &mut (*next_state).nodes, cur_idx);
        if (*err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return 0 as *mut re_dfastate_t;
        }
        if (*next_state).has_backref() != 0 {
            *err = transit_state_bkref(mctx, &mut (*next_state).nodes);
            if (*err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return 0 as *mut re_dfastate_t;
            }
            next_state = *((*mctx).state_log).offset(cur_idx as isize);
        }
    }
    return next_state;
}
unsafe extern "C" fn find_recover_state(
    mut err: *mut reg_errcode_t,
    mut mctx: *mut re_match_context_t,
) -> *mut re_dfastate_t {
    let mut cur_state: *mut re_dfastate_t = 0 as *mut re_dfastate_t;
    loop {
        let mut max: Idx = (*mctx).state_log_top;
        let mut cur_str_idx: Idx = (*mctx).input.cur_idx;
        loop {
            cur_str_idx += 1;
            if cur_str_idx > max {
                return 0 as *mut re_dfastate_t;
            }
            (*mctx).input.cur_idx += 1 as libc::c_int as libc::c_long;
            if !(*((*mctx).state_log).offset(cur_str_idx as isize)).is_null() {
                break;
            }
        }
        cur_state = merge_state_with_log(err, mctx, 0 as *mut re_dfastate_t);
        if !(*err as libc::c_int == _REG_NOERROR as libc::c_int && cur_state.is_null()) {
            break;
        }
    }
    return cur_state;
}
unsafe extern "C" fn check_matching(
    mut mctx: *mut re_match_context_t,
    mut fl_longest_match: bool,
    mut p_match_first: *mut Idx,
) -> Idx {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut match_0: Idx = 0 as libc::c_int as Idx;
    let mut match_last: Idx = -(1 as libc::c_int) as Idx;
    let mut cur_str_idx: Idx = (*mctx).input.cur_idx;
    let mut cur_state: *mut re_dfastate_t = 0 as *mut re_dfastate_t;
    let mut at_init_state: bool = !p_match_first.is_null();
    let mut next_start_idx: Idx = cur_str_idx;
    err = _REG_NOERROR;
    cur_state = acquire_init_state_context(&mut err, mctx, cur_str_idx);
    if (cur_state == 0 as *mut libc::c_void as *mut re_dfastate_t) as libc::c_int
        as libc::c_long != 0
    {
        if err as libc::c_int == _REG_ESPACE as libc::c_int {} else {
            unreachable!();
        };
        return -(2 as libc::c_int) as Idx;
    }
    if !((*mctx).state_log).is_null() {
        let ref mut fresh77 = *((*mctx).state_log).offset(cur_str_idx as isize);
        *fresh77 = cur_state;
        if (*dfa).nbackref != 0 {
            at_init_state = 0 as libc::c_int != 0;
            err = check_subexp_matching_top(
                mctx,
                &mut (*cur_state).nodes,
                0 as libc::c_int as Idx,
            );
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return err as Idx;
            }
            if (*cur_state).has_backref() != 0 {
                err = transit_state_bkref(mctx, &mut (*cur_state).nodes);
                if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    return err as Idx;
                }
            }
        }
    }
    if (*cur_state).halt() as libc::c_long != 0 {
        if (*cur_state).has_constraint() == 0
            || check_halt_state_context(mctx, cur_state, cur_str_idx) != 0
        {
            if !fl_longest_match {
                return cur_str_idx
            } else {
                match_last = cur_str_idx;
                match_0 = 1 as libc::c_int as Idx;
            }
        }
    }
    while !((*mctx).input.stop <= (*mctx).input.cur_idx) {
        let mut old_state: *mut re_dfastate_t = cur_state;
        let mut next_char_idx: Idx = (*mctx).input.cur_idx
            + 1 as libc::c_int as libc::c_long;
        if (next_char_idx >= (*mctx).input.bufs_len) as libc::c_int as libc::c_long != 0
            && (*mctx).input.bufs_len < (*mctx).input.len
            || (next_char_idx >= (*mctx).input.valid_len) as libc::c_int as libc::c_long
                != 0 && (*mctx).input.valid_len < (*mctx).input.len
        {
            err = extend_buffers(
                mctx,
                (next_char_idx + 1 as libc::c_int as libc::c_long) as libc::c_int,
            );
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                if err as libc::c_int == _REG_ESPACE as libc::c_int {} else {
                    unreachable!();
                };
                return -(2 as libc::c_int) as Idx;
            }
        }
        cur_state = transit_state(&mut err, mctx, cur_state);
        if !((*mctx).state_log).is_null() {
            cur_state = merge_state_with_log(&mut err, mctx, cur_state);
        }
        if cur_state.is_null() {
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return -(2 as libc::c_int) as Idx;
            }
            if ((*mctx).state_log).is_null() || match_0 != 0 && !fl_longest_match
                || {
                    cur_state = find_recover_state(&mut err, mctx);
                    cur_state.is_null()
                }
            {
                break;
            }
        }
        if at_init_state as libc::c_long != 0 {
            if old_state == cur_state {
                next_start_idx = next_char_idx;
            } else {
                at_init_state = 0 as libc::c_int != 0;
            }
        }
        if !((*cur_state).halt() != 0) {
            continue;
        }
        if !((*cur_state).has_constraint() == 0
            || check_halt_state_context(mctx, cur_state, (*mctx).input.cur_idx) != 0)
        {
            continue;
        }
        match_last = (*mctx).input.cur_idx;
        match_0 = 1 as libc::c_int as Idx;
        p_match_first = 0 as *mut Idx;
        if !fl_longest_match {
            break;
        }
    }
    if !p_match_first.is_null() {
        *p_match_first += next_start_idx;
    }
    return match_last;
}
unsafe extern "C" fn check_halt_node_context(
    mut dfa: *const re_dfa_t,
    mut node: Idx,
    mut context: libc::c_uint,
) -> bool {
    let mut type_0: re_token_type_t = (*((*dfa).nodes).offset(node as isize)).type_0();
    let mut constraint: libc::c_uint = (*((*dfa).nodes).offset(node as isize))
        .constraint();
    if type_0 as libc::c_uint != END_OF_RE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    if constraint == 0 {
        return 1 as libc::c_int != 0;
    }
    if constraint & 0x4 as libc::c_int as libc::c_uint != 0
        && context & 1 as libc::c_int as libc::c_uint == 0
        || constraint & 0x8 as libc::c_int as libc::c_uint != 0
            && context & 1 as libc::c_int as libc::c_uint != 0
        || constraint & 0x20 as libc::c_int as libc::c_uint != 0
            && context & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint == 0
        || constraint & 0x80 as libc::c_int as libc::c_uint != 0
            && context
                & ((((1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) as libc::c_uint == 0
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn check_halt_state_context(
    mut mctx: *const re_match_context_t,
    mut state: *const re_dfastate_t,
    mut idx: Idx,
) -> Idx {
    let mut i: Idx = 0;
    let mut context: libc::c_uint = 0;
    if (*state).halt() as libc::c_int != 0 {} else {
        unreachable!();
    };
    context = re_string_context_at(&(*mctx).input, idx, (*mctx).eflags);
    i = 0 as libc::c_int as Idx;
    while i < (*state).nodes.nelem {
        if check_halt_node_context(
            (*mctx).dfa,
            *((*state).nodes.elems).offset(i as isize),
            context,
        ) {
            return *((*state).nodes.elems).offset(i as isize);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int as Idx;
}
unsafe extern "C" fn sift_ctx_init(
    mut sctx: *mut re_sift_context_t,
    mut sifted_sts: *mut *mut re_dfastate_t,
    mut limited_sts: *mut *mut re_dfastate_t,
    mut last_node: Idx,
    mut last_str_idx: Idx,
) {
    (*sctx).sifted_states = sifted_sts;
    (*sctx).limited_states = limited_sts;
    (*sctx).last_node = last_node;
    (*sctx).last_str_idx = last_str_idx;
    memset(
        &mut (*sctx).limits as *mut re_node_set as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<re_node_set>() as libc::c_ulong,
    );
}
unsafe extern "C" fn sift_states_iter_mb(
    mut mctx: *const re_match_context_t,
    mut sctx: *mut re_sift_context_t,
    mut node_idx: Idx,
    mut str_idx: Idx,
    mut max_str_idx: Idx,
) -> libc::c_int {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut naccepted: libc::c_int = 0;
    naccepted = check_node_accept_bytes(dfa, node_idx, &(*mctx).input, str_idx);
    if naccepted > 0 as libc::c_int && str_idx + naccepted as libc::c_long <= max_str_idx
        && !(!(*((*sctx).sifted_states)
            .offset((str_idx + naccepted as libc::c_long) as isize))
            .is_null()
            && re_node_set_contains(
                &mut (**((*sctx).sifted_states)
                    .offset((str_idx + naccepted as libc::c_long) as isize))
                    .nodes,
                *((*dfa).nexts).offset(node_idx as isize),
            ) != 0)
    {
        naccepted = 0 as libc::c_int;
    }
    return naccepted;
}
unsafe extern "C" fn build_sifted_states(
    mut mctx: *const re_match_context_t,
    mut sctx: *mut re_sift_context_t,
    mut str_idx: Idx,
    mut cur_dest: *mut re_node_set,
) -> reg_errcode_t {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut cur_src: *const re_node_set = &mut (**((*mctx).state_log)
        .offset(str_idx as isize))
        .non_eps_nodes;
    let mut i: Idx = 0;
    let mut current_block_8: u64;
    i = 0 as libc::c_int as Idx;
    while i < (*cur_src).nelem {
        let mut prev_node: Idx = *((*cur_src).elems).offset(i as isize);
        let mut naccepted: libc::c_int = 0 as libc::c_int;
        let mut ok: bool = false;
        if (*((*dfa).nodes).offset(prev_node as isize)).type_0() as libc::c_int
            & 8 as libc::c_int == 0
        {} else {
            unreachable!();
        };
        if (*((*dfa).nodes).offset(prev_node as isize)).accept_mb() != 0 {
            naccepted = sift_states_iter_mb(
                mctx,
                sctx,
                prev_node,
                str_idx,
                (*sctx).last_str_idx,
            );
        }
        if naccepted == 0
            && check_node_accept(
                mctx,
                ((*dfa).nodes).offset(prev_node as isize),
                str_idx,
            ) as libc::c_int != 0
            && (!(*((*sctx).sifted_states)
                .offset((str_idx + 1 as libc::c_int as libc::c_long) as isize))
                .is_null()
                && re_node_set_contains(
                    &mut (**((*sctx).sifted_states)
                        .offset((str_idx + 1 as libc::c_int as libc::c_long) as isize))
                        .nodes,
                    *((*dfa).nexts).offset(prev_node as isize),
                ) != 0)
        {
            naccepted = 1 as libc::c_int;
        }
        if !(naccepted == 0 as libc::c_int) {
            if (*sctx).limits.nelem != 0 {
                let mut to_idx: Idx = str_idx + naccepted as libc::c_long;
                if check_dst_limits(
                    mctx,
                    &mut (*sctx).limits,
                    *((*dfa).nexts).offset(prev_node as isize),
                    to_idx,
                    prev_node,
                    str_idx,
                ) {
                    current_block_8 = 6239978542346980191;
                } else {
                    current_block_8 = 2968425633554183086;
                }
            } else {
                current_block_8 = 2968425633554183086;
            }
            match current_block_8 {
                6239978542346980191 => {}
                _ => {
                    ok = re_node_set_insert(cur_dest, prev_node);
                    if !ok as libc::c_int as libc::c_long != 0 {
                        return _REG_ESPACE;
                    }
                }
            }
        }
        i += 1;
        i;
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn add_epsilon_src_nodes(
    mut dfa: *const re_dfa_t,
    mut dest_nodes: *mut re_node_set,
    mut candidates: *const re_node_set,
) -> reg_errcode_t {
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut i: Idx = 0;
    let mut state: *mut re_dfastate_t = re_acquire_state(&mut err, dfa, dest_nodes);
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return err;
    }
    if (*state).inveclosure.alloc == 0 {
        err = re_node_set_alloc(&mut (*state).inveclosure, (*dest_nodes).nelem);
        if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return _REG_ESPACE;
        }
        i = 0 as libc::c_int as Idx;
        while i < (*dest_nodes).nelem {
            err = re_node_set_merge(
                &mut (*state).inveclosure,
                ((*dfa).inveclosures)
                    .offset(*((*dest_nodes).elems).offset(i as isize) as isize),
            );
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return _REG_ESPACE;
            }
            i += 1;
            i;
        }
    }
    return re_node_set_add_intersect(dest_nodes, candidates, &mut (*state).inveclosure);
}
unsafe extern "C" fn sub_epsilon_src_nodes(
    mut dfa: *const re_dfa_t,
    mut node: Idx,
    mut dest_nodes: *mut re_node_set,
    mut candidates: *const re_node_set,
) -> reg_errcode_t {
    let mut ecl_idx: Idx = 0;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut inv_eclosure: *mut re_node_set = ((*dfa).inveclosures).offset(node as isize);
    let mut except_nodes: re_node_set = re_node_set {
        alloc: 0,
        nelem: 0,
        elems: 0 as *mut Idx,
    };
    memset(
        &mut except_nodes as *mut re_node_set as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<re_node_set>() as libc::c_ulong,
    );
    ecl_idx = 0 as libc::c_int as Idx;
    while ecl_idx < (*inv_eclosure).nelem {
        let mut cur_node: Idx = *((*inv_eclosure).elems).offset(ecl_idx as isize);
        if !(cur_node == node) {
            if (*((*dfa).nodes).offset(cur_node as isize)).type_0() as libc::c_int
                & 8 as libc::c_int != 0
            {
                let mut edst1: Idx = *((*((*dfa).edests).offset(cur_node as isize))
                    .elems)
                    .offset(0 as libc::c_int as isize);
                let mut edst2: Idx = if (*((*dfa).edests).offset(cur_node as isize))
                    .nelem > 1 as libc::c_int as libc::c_long
                {
                    *((*((*dfa).edests).offset(cur_node as isize)).elems)
                        .offset(1 as libc::c_int as isize)
                } else {
                    -(1 as libc::c_int) as libc::c_long
                };
                if re_node_set_contains(inv_eclosure, edst1) == 0
                    && re_node_set_contains(dest_nodes, edst1) != 0
                    || edst2 > 0 as libc::c_int as libc::c_long
                        && re_node_set_contains(inv_eclosure, edst2) == 0
                        && re_node_set_contains(dest_nodes, edst2) != 0
                {
                    err = re_node_set_add_intersect(
                        &mut except_nodes,
                        candidates,
                        ((*dfa).inveclosures).offset(cur_node as isize),
                    );
                    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        rpl_free(except_nodes.elems as *mut libc::c_void);
                        return err;
                    }
                }
            }
        }
        ecl_idx += 1;
        ecl_idx;
    }
    ecl_idx = 0 as libc::c_int as Idx;
    while ecl_idx < (*inv_eclosure).nelem {
        let mut cur_node_0: Idx = *((*inv_eclosure).elems).offset(ecl_idx as isize);
        if re_node_set_contains(&mut except_nodes, cur_node_0) == 0 {
            let mut idx: Idx = re_node_set_contains(dest_nodes, cur_node_0)
                - 1 as libc::c_int as libc::c_long;
            re_node_set_remove_at(dest_nodes, idx);
        }
        ecl_idx += 1;
        ecl_idx;
    }
    rpl_free(except_nodes.elems as *mut libc::c_void);
    return _REG_NOERROR;
}
unsafe extern "C" fn check_subexp_limits(
    mut dfa: *const re_dfa_t,
    mut dest_nodes: *mut re_node_set,
    mut candidates: *const re_node_set,
    mut limits: *mut re_node_set,
    mut bkref_ents: *mut re_backref_cache_entry,
    mut str_idx: Idx,
) -> reg_errcode_t {
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut node_idx: Idx = 0;
    let mut lim_idx: Idx = 0;
    lim_idx = 0 as libc::c_int as Idx;
    while lim_idx < (*limits).nelem {
        let mut subexp_idx: Idx = 0;
        let mut ent: *mut re_backref_cache_entry = 0 as *mut re_backref_cache_entry;
        ent = bkref_ents.offset(*((*limits).elems).offset(lim_idx as isize) as isize);
        if !(str_idx <= (*ent).subexp_from || (*ent).str_idx < str_idx) {
            subexp_idx = (*((*dfa).nodes).offset((*ent).node as isize)).opr.idx;
            if (*ent).subexp_to == str_idx {
                let mut ops_node: Idx = -(1 as libc::c_int) as Idx;
                let mut cls_node: Idx = -(1 as libc::c_int) as Idx;
                node_idx = 0 as libc::c_int as Idx;
                while node_idx < (*dest_nodes).nelem {
                    let mut node: Idx = *((*dest_nodes).elems).offset(node_idx as isize);
                    let mut type_0: re_token_type_t = (*((*dfa).nodes)
                        .offset(node as isize))
                        .type_0();
                    if type_0 as libc::c_uint
                        == OP_OPEN_SUBEXP as libc::c_int as libc::c_uint
                        && subexp_idx == (*((*dfa).nodes).offset(node as isize)).opr.idx
                    {
                        ops_node = node;
                    } else if type_0 as libc::c_uint
                        == OP_CLOSE_SUBEXP as libc::c_int as libc::c_uint
                        && subexp_idx == (*((*dfa).nodes).offset(node as isize)).opr.idx
                    {
                        cls_node = node;
                    }
                    node_idx += 1;
                    node_idx;
                }
                if ops_node >= 0 as libc::c_int as libc::c_long {
                    err = sub_epsilon_src_nodes(dfa, ops_node, dest_nodes, candidates);
                    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        return err;
                    }
                }
                if cls_node >= 0 as libc::c_int as libc::c_long {
                    node_idx = 0 as libc::c_int as Idx;
                    while node_idx < (*dest_nodes).nelem {
                        let mut node_0: Idx = *((*dest_nodes).elems)
                            .offset(node_idx as isize);
                        if re_node_set_contains(
                            ((*dfa).inveclosures).offset(node_0 as isize),
                            cls_node,
                        ) == 0
                            && re_node_set_contains(
                                ((*dfa).eclosures).offset(node_0 as isize),
                                cls_node,
                            ) == 0
                        {
                            err = sub_epsilon_src_nodes(
                                dfa,
                                node_0,
                                dest_nodes,
                                candidates,
                            );
                            if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                as libc::c_int as libc::c_long != 0
                            {
                                return err;
                            }
                            node_idx -= 1;
                            node_idx;
                        }
                        node_idx += 1;
                        node_idx;
                    }
                }
            } else {
                node_idx = 0 as libc::c_int as Idx;
                while node_idx < (*dest_nodes).nelem {
                    let mut node_1: Idx = *((*dest_nodes).elems)
                        .offset(node_idx as isize);
                    let mut type_1: re_token_type_t = (*((*dfa).nodes)
                        .offset(node_1 as isize))
                        .type_0();
                    if type_1 as libc::c_uint
                        == OP_CLOSE_SUBEXP as libc::c_int as libc::c_uint
                        || type_1 as libc::c_uint
                            == OP_OPEN_SUBEXP as libc::c_int as libc::c_uint
                    {
                        if !(subexp_idx
                            != (*((*dfa).nodes).offset(node_1 as isize)).opr.idx)
                        {
                            err = sub_epsilon_src_nodes(
                                dfa,
                                node_1,
                                dest_nodes,
                                candidates,
                            );
                            if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                as libc::c_int as libc::c_long != 0
                            {
                                return err;
                            }
                        }
                    }
                    node_idx += 1;
                    node_idx;
                }
            }
        }
        lim_idx += 1;
        lim_idx;
    }
    return _REG_NOERROR;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_re_search(
    mut bufp: *mut re_pattern_buffer,
    mut string: *const libc::c_char,
    mut length: Idx,
    mut start: Idx,
    mut range: regoff_t,
    mut regs: *mut re_registers,
) -> regoff_t {
    return re_search_stub(
        bufp,
        string,
        length,
        start,
        range,
        length,
        regs,
        0 as libc::c_int != 0,
    );
}
unsafe extern "C" fn re_search_stub(
    mut bufp: *mut re_pattern_buffer,
    mut string: *const libc::c_char,
    mut length: Idx,
    mut start: Idx,
    mut range: regoff_t,
    mut stop: Idx,
    mut regs: *mut re_registers,
    mut ret_len: bool,
) -> regoff_t {
    let mut result: reg_errcode_t = _REG_NOERROR;
    let mut pmatch: *mut regmatch_t = 0 as *mut regmatch_t;
    let mut nregs: Idx = 0;
    let mut rval: regoff_t = 0;
    let mut eflags: libc::c_int = 0 as libc::c_int;
    let mut dfa: *mut re_dfa_t = (*bufp).buffer;
    let mut last_start: Idx = start + range;
    if (start < 0 as libc::c_int as libc::c_long || start > length) as libc::c_int
        as libc::c_long != 0
    {
        return -(1 as libc::c_int) as regoff_t;
    }
    if (length < last_start
        || 0 as libc::c_int as libc::c_long <= range && last_start < start)
        as libc::c_int as libc::c_long != 0
    {
        last_start = length;
    } else if (last_start < 0 as libc::c_int as libc::c_long
        || range < 0 as libc::c_int as libc::c_long && start <= last_start)
        as libc::c_int as libc::c_long != 0
    {
        last_start = 0 as libc::c_int as Idx;
    }
    eflags
        |= if (*bufp).not_bol() as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    eflags
        |= if (*bufp).not_eol() as libc::c_int != 0 {
            (1 as libc::c_int) << 1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    if start < last_start && !((*bufp).fastmap).is_null()
        && (*bufp).fastmap_accurate() == 0
    {
        rpl_re_compile_fastmap(bufp);
    }
    if (*bufp).no_sub() as libc::c_long != 0 {
        regs = 0 as *mut re_registers;
    }
    if regs.is_null() {
        nregs = 1 as libc::c_int as Idx;
    } else if ((*bufp).regs_allocated() as libc::c_int == 2 as libc::c_int
        && (*regs).num_regs <= (*bufp).re_nsub) as libc::c_int as libc::c_long != 0
    {
        nregs = (*regs).num_regs as Idx;
        if (nregs < 1 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0
        {
            regs = 0 as *mut re_registers;
            nregs = 1 as libc::c_int as Idx;
        }
    } else {
        nregs = ((*bufp).re_nsub).wrapping_add(1 as libc::c_int as libc::c_ulong) as Idx;
    }
    pmatch = malloc(
        (nregs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<regmatch_t>() as libc::c_ulong),
    ) as *mut regmatch_t;
    if (pmatch == 0 as *mut libc::c_void as *mut regmatch_t) as libc::c_int
        as libc::c_long != 0
    {
        rval = -(2 as libc::c_int) as regoff_t;
    } else {
        result = re_search_internal(
            bufp,
            string,
            length,
            start,
            last_start,
            stop,
            nregs as size_t,
            pmatch,
            eflags,
        );
        rval = 0 as libc::c_int as regoff_t;
        if result as libc::c_int != _REG_NOERROR as libc::c_int {
            rval = (if result as libc::c_int == _REG_NOMATCH as libc::c_int {
                -(1 as libc::c_int)
            } else {
                -(2 as libc::c_int)
            }) as regoff_t;
        } else if !regs.is_null() {
            (*bufp)
                .set_regs_allocated(
                    re_copy_regs(
                        regs,
                        pmatch,
                        nregs,
                        (*bufp).regs_allocated() as libc::c_int,
                    ),
                );
            if ((*bufp).regs_allocated() as libc::c_int == 0 as libc::c_int)
                as libc::c_int as libc::c_long != 0
            {
                rval = -(2 as libc::c_int) as regoff_t;
            }
        }
        if (rval == 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0
        {
            if ret_len {
                if (*pmatch.offset(0 as libc::c_int as isize)).rm_so == start {} else {
                    unreachable!();
                };
                rval = (*pmatch.offset(0 as libc::c_int as isize)).rm_eo - start;
            } else {
                rval = (*pmatch.offset(0 as libc::c_int as isize)).rm_so;
            }
        }
        rpl_free(pmatch as *mut libc::c_void);
    }
    return rval;
}
unsafe extern "C" fn re_copy_regs(
    mut regs: *mut re_registers,
    mut pmatch: *mut regmatch_t,
    mut nregs: Idx,
    mut regs_allocated: libc::c_int,
) -> libc::c_uint {
    let mut rval: libc::c_int = 1 as libc::c_int;
    let mut i: Idx = 0;
    let mut need_regs: Idx = nregs + 1 as libc::c_int as libc::c_long;
    if regs_allocated == 0 as libc::c_int {
        (*regs)
            .start = malloc(
            (need_regs as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<regoff_t>() as libc::c_ulong),
        ) as *mut regoff_t;
        if ((*regs).start == 0 as *mut libc::c_void as *mut regoff_t) as libc::c_int
            as libc::c_long != 0
        {
            return 0 as libc::c_int as libc::c_uint;
        }
        (*regs)
            .end = malloc(
            (need_regs as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<regoff_t>() as libc::c_ulong),
        ) as *mut regoff_t;
        if ((*regs).end == 0 as *mut libc::c_void as *mut regoff_t) as libc::c_int
            as libc::c_long != 0
        {
            rpl_free((*regs).start as *mut libc::c_void);
            return 0 as libc::c_int as libc::c_uint;
        }
        (*regs).num_regs = need_regs as __re_size_t;
    } else if regs_allocated == 1 as libc::c_int {
        if (need_regs as libc::c_ulong > (*regs).num_regs) as libc::c_int as libc::c_long
            != 0
        {
            let mut new_start: *mut regoff_t = realloc(
                (*regs).start as *mut libc::c_void,
                (need_regs as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<regoff_t>() as libc::c_ulong),
            ) as *mut regoff_t;
            let mut new_end: *mut regoff_t = 0 as *mut regoff_t;
            if (new_start == 0 as *mut libc::c_void as *mut regoff_t) as libc::c_int
                as libc::c_long != 0
            {
                return 0 as libc::c_int as libc::c_uint;
            }
            new_end = realloc(
                (*regs).end as *mut libc::c_void,
                (need_regs as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<regoff_t>() as libc::c_ulong),
            ) as *mut regoff_t;
            if (new_end == 0 as *mut libc::c_void as *mut regoff_t) as libc::c_int
                as libc::c_long != 0
            {
                rpl_free(new_start as *mut libc::c_void);
                return 0 as libc::c_int as libc::c_uint;
            }
            (*regs).start = new_start;
            (*regs).end = new_end;
            (*regs).num_regs = need_regs as __re_size_t;
        }
    } else {
        if regs_allocated == 2 as libc::c_int {} else {
            unreachable!();
        };
        if nregs as libc::c_ulong <= (*regs).num_regs {} else {
            unreachable!();
        };
        rval = 2 as libc::c_int;
    }
    i = 0 as libc::c_int as Idx;
    while i < nregs {
        *((*regs).start).offset(i as isize) = (*pmatch.offset(i as isize)).rm_so;
        *((*regs).end).offset(i as isize) = (*pmatch.offset(i as isize)).rm_eo;
        i += 1;
        i;
    }
    while (i as libc::c_ulong) < (*regs).num_regs {
        let ref mut fresh78 = *((*regs).end).offset(i as isize);
        *fresh78 = -(1 as libc::c_int) as regoff_t;
        *((*regs).start).offset(i as isize) = *fresh78;
        i += 1;
        i;
    }
    return rval as libc::c_uint;
}
unsafe extern "C" fn check_dst_limits_calc_pos_1(
    mut mctx: *const re_match_context_t,
    mut boundaries: libc::c_int,
    mut subexp_idx: Idx,
    mut from_node: Idx,
    mut bkref_idx: Idx,
) -> libc::c_int {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut eclosures: *const re_node_set = ((*dfa).eclosures)
        .offset(from_node as isize);
    let mut node_idx: Idx = 0;
    node_idx = 0 as libc::c_int as Idx;
    while node_idx < (*eclosures).nelem {
        let mut node: Idx = *((*eclosures).elems).offset(node_idx as isize);
        match (*((*dfa).nodes).offset(node as isize)).type_0() as libc::c_int {
            4 => {
                if bkref_idx != -(1 as libc::c_int) as libc::c_long {
                    let mut ent: *mut re_backref_cache_entry = ((*mctx).bkref_ents)
                        .offset(bkref_idx as isize);
                    loop {
                        let mut dst: Idx = 0;
                        let mut cpos: libc::c_int = 0;
                        if !((*ent).node != node) {
                            if !(subexp_idx < 64 as libc::c_int as libc::c_long
                                && (*ent).eps_reachable_subexps_map
                                    & (1 as libc::c_int as bitset_word_t) << subexp_idx == 0)
                            {
                                dst = *((*((*dfa).edests).offset(node as isize)).elems)
                                    .offset(0 as libc::c_int as isize);
                                if dst == from_node {
                                    if boundaries & 1 as libc::c_int != 0 {
                                        return -(1 as libc::c_int)
                                    } else {
                                        return 0 as libc::c_int
                                    }
                                }
                                cpos = check_dst_limits_calc_pos_1(
                                    mctx,
                                    boundaries,
                                    subexp_idx,
                                    dst,
                                    bkref_idx,
                                );
                                if cpos == -(1 as libc::c_int) {
                                    return -(1 as libc::c_int);
                                }
                                if cpos == 0 as libc::c_int
                                    && boundaries & 2 as libc::c_int != 0
                                {
                                    return 0 as libc::c_int;
                                }
                                if subexp_idx < 64 as libc::c_int as libc::c_long {
                                    (*ent).eps_reachable_subexps_map
                                        &= !((1 as libc::c_int as bitset_word_t) << subexp_idx);
                                }
                            }
                        }
                        let fresh79 = ent;
                        ent = ent.offset(1);
                        if !((*fresh79).more != 0) {
                            break;
                        }
                    }
                }
            }
            8 => {
                if boundaries & 1 as libc::c_int != 0
                    && subexp_idx == (*((*dfa).nodes).offset(node as isize)).opr.idx
                {
                    return -(1 as libc::c_int);
                }
            }
            9 => {
                if boundaries & 2 as libc::c_int != 0
                    && subexp_idx == (*((*dfa).nodes).offset(node as isize)).opr.idx
                {
                    return 0 as libc::c_int;
                }
            }
            _ => {}
        }
        node_idx += 1;
        node_idx;
    }
    return if boundaries & 2 as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn check_dst_limits_calc_pos(
    mut mctx: *const re_match_context_t,
    mut limit: Idx,
    mut subexp_idx: Idx,
    mut from_node: Idx,
    mut str_idx: Idx,
    mut bkref_idx: Idx,
) -> libc::c_int {
    let mut lim: *mut re_backref_cache_entry = ((*mctx).bkref_ents)
        .offset(limit as isize);
    let mut boundaries: libc::c_int = 0;
    if str_idx < (*lim).subexp_from {
        return -(1 as libc::c_int);
    }
    if (*lim).subexp_to < str_idx {
        return 1 as libc::c_int;
    }
    boundaries = (str_idx == (*lim).subexp_from) as libc::c_int;
    boundaries |= ((str_idx == (*lim).subexp_to) as libc::c_int) << 1 as libc::c_int;
    if boundaries == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return check_dst_limits_calc_pos_1(
        mctx,
        boundaries,
        subexp_idx,
        from_node,
        bkref_idx,
    );
}
unsafe extern "C" fn search_cur_bkref_entry(
    mut mctx: *const re_match_context_t,
    mut str_idx: Idx,
) -> Idx {
    let mut left: Idx = 0;
    let mut right: Idx = 0;
    let mut mid: Idx = 0;
    let mut last: Idx = 0;
    right = (*mctx).nbkref_ents;
    last = right;
    left = 0 as libc::c_int as Idx;
    while left < right {
        mid = (left + right) / 2 as libc::c_int as libc::c_long;
        if (*((*mctx).bkref_ents).offset(mid as isize)).str_idx < str_idx {
            left = mid + 1 as libc::c_int as libc::c_long;
        } else {
            right = mid;
        }
    }
    if left < last && (*((*mctx).bkref_ents).offset(left as isize)).str_idx == str_idx {
        return left
    } else {
        return -(1 as libc::c_int) as Idx
    };
}
unsafe extern "C" fn check_dst_limits(
    mut mctx: *const re_match_context_t,
    mut limits: *const re_node_set,
    mut dst_node: Idx,
    mut dst_idx: Idx,
    mut src_node: Idx,
    mut src_idx: Idx,
) -> bool {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut lim_idx: Idx = 0;
    let mut src_pos: Idx = 0;
    let mut dst_pos: Idx = 0;
    let mut dst_bkref_idx: Idx = search_cur_bkref_entry(mctx, dst_idx);
    let mut src_bkref_idx: Idx = search_cur_bkref_entry(mctx, src_idx);
    lim_idx = 0 as libc::c_int as Idx;
    while lim_idx < (*limits).nelem {
        let mut subexp_idx: Idx = 0;
        let mut ent: *mut re_backref_cache_entry = 0 as *mut re_backref_cache_entry;
        ent = ((*mctx).bkref_ents)
            .offset(*((*limits).elems).offset(lim_idx as isize) as isize);
        subexp_idx = (*((*dfa).nodes).offset((*ent).node as isize)).opr.idx;
        dst_pos = check_dst_limits_calc_pos(
            mctx,
            *((*limits).elems).offset(lim_idx as isize),
            subexp_idx,
            dst_node,
            dst_idx,
            dst_bkref_idx,
        ) as Idx;
        src_pos = check_dst_limits_calc_pos(
            mctx,
            *((*limits).elems).offset(lim_idx as isize),
            subexp_idx,
            src_node,
            src_idx,
            src_bkref_idx,
        ) as Idx;
        if src_pos == dst_pos {
            lim_idx += 1;
            lim_idx;
        } else {
            return 1 as libc::c_int != 0
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn merge_state_array(
    mut dfa: *const re_dfa_t,
    mut dst: *mut *mut re_dfastate_t,
    mut src: *mut *mut re_dfastate_t,
    mut num: Idx,
) -> reg_errcode_t {
    let mut st_idx: Idx = 0;
    let mut err: reg_errcode_t = _REG_NOERROR;
    st_idx = 0 as libc::c_int as Idx;
    while st_idx < num {
        if (*dst.offset(st_idx as isize)).is_null() {
            let ref mut fresh80 = *dst.offset(st_idx as isize);
            *fresh80 = *src.offset(st_idx as isize);
        } else if !(*src.offset(st_idx as isize)).is_null() {
            let mut merged_set: re_node_set = re_node_set {
                alloc: 0,
                nelem: 0,
                elems: 0 as *mut Idx,
            };
            err = re_node_set_init_union(
                &mut merged_set,
                &mut (**dst.offset(st_idx as isize)).nodes,
                &mut (**src.offset(st_idx as isize)).nodes,
            );
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return err;
            }
            let ref mut fresh81 = *dst.offset(st_idx as isize);
            *fresh81 = re_acquire_state(&mut err, dfa, &mut merged_set);
            rpl_free(merged_set.elems as *mut libc::c_void);
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return err;
            }
        }
        st_idx += 1;
        st_idx;
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn sift_states_bkref(
    mut mctx: *const re_match_context_t,
    mut sctx: *mut re_sift_context_t,
    mut str_idx: Idx,
    mut candidates: *const re_node_set,
) -> reg_errcode_t {
    let mut current_block: u64;
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut node_idx: Idx = 0;
    let mut node: Idx = 0;
    let mut local_sctx: re_sift_context_t = re_sift_context_t {
        sifted_states: 0 as *mut *mut re_dfastate_t,
        limited_states: 0 as *mut *mut re_dfastate_t,
        last_node: 0,
        last_str_idx: 0,
        limits: re_node_set {
            alloc: 0,
            nelem: 0,
            elems: 0 as *mut Idx,
        },
    };
    let mut first_idx: Idx = search_cur_bkref_entry(mctx, str_idx);
    if first_idx == -(1 as libc::c_int) as libc::c_long {
        return _REG_NOERROR;
    }
    local_sctx.sifted_states = 0 as *mut *mut re_dfastate_t;
    node_idx = 0 as libc::c_int as Idx;
    's_22: loop {
        if !(node_idx < (*candidates).nelem) {
            current_block = 1118134448028020070;
            break;
        }
        let mut enabled_idx: Idx = 0;
        let mut type_0: re_token_type_t = NON_TYPE;
        let mut entry: *mut re_backref_cache_entry = 0 as *mut re_backref_cache_entry;
        node = *((*candidates).elems).offset(node_idx as isize);
        type_0 = (*((*dfa).nodes).offset(node as isize)).type_0();
        if !(node == (*sctx).last_node && str_idx == (*sctx).last_str_idx) {
            if !(type_0 as libc::c_uint != OP_BACK_REF as libc::c_int as libc::c_uint) {
                entry = ((*mctx).bkref_ents).offset(first_idx as isize);
                enabled_idx = first_idx;
                loop {
                    let mut subexp_len: Idx = 0;
                    let mut to_idx: Idx = 0;
                    let mut dst_node: Idx = 0;
                    let mut ok: bool = false;
                    let mut cur_state: *mut re_dfastate_t = 0 as *mut re_dfastate_t;
                    if !((*entry).node != node) {
                        subexp_len = (*entry).subexp_to - (*entry).subexp_from;
                        to_idx = str_idx + subexp_len;
                        dst_node = if subexp_len != 0 {
                            *((*dfa).nexts).offset(node as isize)
                        } else {
                            *((*((*dfa).edests).offset(node as isize)).elems)
                                .offset(0 as libc::c_int as isize)
                        };
                        if !(to_idx > (*sctx).last_str_idx
                            || (*((*sctx).sifted_states).offset(to_idx as isize))
                                .is_null()
                            || !(!(*((*sctx).sifted_states).offset(to_idx as isize))
                                .is_null()
                                && re_node_set_contains(
                                    &mut (**((*sctx).sifted_states).offset(to_idx as isize))
                                        .nodes,
                                    dst_node,
                                ) != 0)
                            || check_dst_limits(
                                mctx,
                                &mut (*sctx).limits,
                                node,
                                str_idx,
                                dst_node,
                                to_idx,
                            ) as libc::c_int != 0)
                        {
                            if (local_sctx.sifted_states).is_null() {
                                local_sctx = *sctx;
                                err = re_node_set_init_copy(
                                    &mut local_sctx.limits,
                                    &mut (*sctx).limits,
                                );
                                if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    current_block = 18307012651695163191;
                                    break 's_22;
                                }
                            }
                            local_sctx.last_node = node;
                            local_sctx.last_str_idx = str_idx;
                            ok = re_node_set_insert(&mut local_sctx.limits, enabled_idx);
                            if !ok as libc::c_int as libc::c_long != 0 {
                                err = _REG_ESPACE;
                                current_block = 18307012651695163191;
                                break 's_22;
                            } else {
                                cur_state = *(local_sctx.sifted_states)
                                    .offset(str_idx as isize);
                                err = sift_states_backward(mctx, &mut local_sctx);
                                if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    current_block = 18307012651695163191;
                                    break 's_22;
                                }
                                if !((*sctx).limited_states).is_null() {
                                    err = merge_state_array(
                                        dfa,
                                        (*sctx).limited_states,
                                        local_sctx.sifted_states,
                                        str_idx + 1 as libc::c_int as libc::c_long,
                                    );
                                    if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                                        as libc::c_int as libc::c_long != 0
                                    {
                                        current_block = 18307012651695163191;
                                        break 's_22;
                                    }
                                }
                                let ref mut fresh82 = *(local_sctx.sifted_states)
                                    .offset(str_idx as isize);
                                *fresh82 = cur_state;
                                re_node_set_remove_at(
                                    &mut local_sctx.limits,
                                    re_node_set_contains(&mut local_sctx.limits, enabled_idx)
                                        - 1 as libc::c_int as libc::c_long,
                                );
                                entry = ((*mctx).bkref_ents).offset(enabled_idx as isize);
                            }
                        }
                    }
                    enabled_idx += 1;
                    enabled_idx;
                    let fresh83 = entry;
                    entry = entry.offset(1);
                    if !((*fresh83).more != 0) {
                        break;
                    }
                }
            }
        }
        node_idx += 1;
        node_idx;
    }
    match current_block {
        1118134448028020070 => {
            err = _REG_NOERROR;
        }
        _ => {}
    }
    if !(local_sctx.sifted_states).is_null() {
        rpl_free(local_sctx.limits.elems as *mut libc::c_void);
    }
    return err;
}
unsafe extern "C" fn update_cur_sifted_state(
    mut mctx: *const re_match_context_t,
    mut sctx: *mut re_sift_context_t,
    mut str_idx: Idx,
    mut dest_nodes: *mut re_node_set,
) -> reg_errcode_t {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut candidates: *const re_node_set = 0 as *const re_node_set;
    candidates = if (*((*mctx).state_log).offset(str_idx as isize)).is_null() {
        0 as *mut re_node_set
    } else {
        &mut (**((*mctx).state_log).offset(str_idx as isize)).nodes
    };
    if (*dest_nodes).nelem == 0 as libc::c_int as libc::c_long {
        let ref mut fresh84 = *((*sctx).sifted_states).offset(str_idx as isize);
        *fresh84 = 0 as *mut re_dfastate_t;
    } else {
        if !candidates.is_null() {
            err = add_epsilon_src_nodes(dfa, dest_nodes, candidates);
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return err;
            }
            if (*sctx).limits.nelem != 0 {
                err = check_subexp_limits(
                    dfa,
                    dest_nodes,
                    candidates,
                    &mut (*sctx).limits,
                    (*mctx).bkref_ents,
                    str_idx,
                );
                if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    return err;
                }
            }
        }
        let ref mut fresh85 = *((*sctx).sifted_states).offset(str_idx as isize);
        *fresh85 = re_acquire_state(&mut err, dfa, dest_nodes);
        if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return err;
        }
    }
    if !candidates.is_null()
        && (**((*mctx).state_log).offset(str_idx as isize)).has_backref() as libc::c_int
            != 0
    {
        err = sift_states_bkref(mctx, sctx, str_idx, candidates);
        if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return err;
        }
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn sift_states_backward(
    mut mctx: *const re_match_context_t,
    mut sctx: *mut re_sift_context_t,
) -> reg_errcode_t {
    let mut current_block: u64;
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut null_cnt: libc::c_int = 0 as libc::c_int;
    let mut str_idx: Idx = (*sctx).last_str_idx;
    let mut cur_dest: re_node_set = re_node_set {
        alloc: 0,
        nelem: 0,
        elems: 0 as *mut Idx,
    };
    if !((*mctx).state_log).is_null()
        && !(*((*mctx).state_log).offset(str_idx as isize)).is_null()
    {} else {
        unreachable!();
    };
    err = re_node_set_init_1(&mut cur_dest, (*sctx).last_node);
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return err;
    }
    err = update_cur_sifted_state(mctx, sctx, str_idx, &mut cur_dest);
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        current_block = 17306497188988177244;
    } else {
        current_block = 6873731126896040597;
    }
    loop {
        match current_block {
            17306497188988177244 => {
                rpl_free(cur_dest.elems as *mut libc::c_void);
                break;
            }
            _ => {
                if str_idx > 0 as libc::c_int as libc::c_long {
                    null_cnt = if (*((*sctx).sifted_states).offset(str_idx as isize))
                        .is_null()
                    {
                        null_cnt + 1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    if null_cnt > (*mctx).max_mb_elem_len {
                        memset(
                            (*sctx).sifted_states as *mut libc::c_void,
                            '\0' as i32,
                            (::core::mem::size_of::<*mut re_dfastate_t>()
                                as libc::c_ulong)
                                .wrapping_mul(str_idx as libc::c_ulong),
                        );
                        rpl_free(cur_dest.elems as *mut libc::c_void);
                        return _REG_NOERROR;
                    }
                    cur_dest.nelem = 0 as libc::c_int as Idx;
                    str_idx -= 1;
                    str_idx;
                    if !(*((*mctx).state_log).offset(str_idx as isize)).is_null() {
                        err = build_sifted_states(mctx, sctx, str_idx, &mut cur_dest);
                        if (err as libc::c_int != _REG_NOERROR as libc::c_int)
                            as libc::c_int as libc::c_long != 0
                        {
                            current_block = 17306497188988177244;
                            continue;
                        }
                    }
                    err = update_cur_sifted_state(mctx, sctx, str_idx, &mut cur_dest);
                    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        current_block = 17306497188988177244;
                    } else {
                        current_block = 6873731126896040597;
                    }
                } else {
                    err = _REG_NOERROR;
                    current_block = 17306497188988177244;
                }
            }
        }
    }
    return err;
}
unsafe extern "C" fn prune_impossible_nodes(
    mut mctx: *mut re_match_context_t,
) -> reg_errcode_t {
    let mut current_block: u64;
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut halt_node: Idx = 0;
    let mut match_last: Idx = 0;
    let mut ret: reg_errcode_t = _REG_NOERROR;
    let mut sifted_states: *mut *mut re_dfastate_t = 0 as *mut *mut re_dfastate_t;
    let mut lim_states: *mut *mut re_dfastate_t = 0 as *mut *mut re_dfastate_t;
    let mut sctx: re_sift_context_t = re_sift_context_t {
        sifted_states: 0 as *mut *mut re_dfastate_t,
        limited_states: 0 as *mut *mut re_dfastate_t,
        last_node: 0,
        last_str_idx: 0,
        limits: re_node_set {
            alloc: 0,
            nelem: 0,
            elems: 0 as *mut Idx,
        },
    };
    if !((*mctx).state_log).is_null() {} else {
        unreachable!();
    };
    match_last = (*mctx).match_last;
    halt_node = (*mctx).last_node;
    if ((if (9223372036854775807 as libc::c_long as libc::c_ulong)
        < (18446744073709551615 as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong)
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        (18446744073709551615 as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong)
    }) <= match_last as libc::c_ulong) as libc::c_int as libc::c_long != 0
    {
        return _REG_ESPACE;
    }
    sifted_states = malloc(
        ((match_last + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong),
    ) as *mut *mut re_dfastate_t;
    if (sifted_states == 0 as *mut libc::c_void as *mut *mut re_dfastate_t)
        as libc::c_int as libc::c_long != 0
    {
        ret = _REG_ESPACE;
    } else {
        if (*dfa).nbackref != 0 {
            lim_states = malloc(
                ((match_last + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong,
                    ),
            ) as *mut *mut re_dfastate_t;
            if (lim_states == 0 as *mut libc::c_void as *mut *mut re_dfastate_t)
                as libc::c_int as libc::c_long != 0
            {
                ret = _REG_ESPACE;
                current_block = 618630553461347817;
            } else {
                's_62: loop {
                    memset(
                        lim_states as *mut libc::c_void,
                        '\0' as i32,
                        (::core::mem::size_of::<*mut re_dfastate_t>() as libc::c_ulong)
                            .wrapping_mul(
                                (match_last + 1 as libc::c_int as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    sift_ctx_init(
                        &mut sctx,
                        sifted_states,
                        lim_states,
                        halt_node,
                        match_last,
                    );
                    ret = sift_states_backward(mctx, &mut sctx);
                    rpl_free(sctx.limits.elems as *mut libc::c_void);
                    if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        current_block = 618630553461347817;
                        break;
                    }
                    if !(*sifted_states.offset(0 as libc::c_int as isize)).is_null()
                        || !(*lim_states.offset(0 as libc::c_int as isize)).is_null()
                    {
                        current_block = 11042950489265723346;
                        break;
                    }
                    loop {
                        match_last -= 1;
                        match_last;
                        if match_last < 0 as libc::c_int as libc::c_long {
                            ret = _REG_NOMATCH;
                            current_block = 618630553461347817;
                            break 's_62;
                        } else if !((*((*mctx).state_log).offset(match_last as isize))
                            .is_null()
                            || (**((*mctx).state_log).offset(match_last as isize)).halt()
                                == 0)
                        {
                            break;
                        }
                    }
                    halt_node = check_halt_state_context(
                        mctx,
                        *((*mctx).state_log).offset(match_last as isize),
                        match_last,
                    );
                }
                match current_block {
                    618630553461347817 => {}
                    _ => {
                        ret = merge_state_array(
                            dfa,
                            sifted_states,
                            lim_states,
                            match_last + 1 as libc::c_int as libc::c_long,
                        );
                        rpl_free(lim_states as *mut libc::c_void);
                        lim_states = 0 as *mut *mut re_dfastate_t;
                        if (ret as libc::c_int != _REG_NOERROR as libc::c_int)
                            as libc::c_int as libc::c_long != 0
                        {
                            current_block = 618630553461347817;
                        } else {
                            current_block = 2569451025026770673;
                        }
                    }
                }
            }
        } else {
            sift_ctx_init(&mut sctx, sifted_states, lim_states, halt_node, match_last);
            ret = sift_states_backward(mctx, &mut sctx);
            rpl_free(sctx.limits.elems as *mut libc::c_void);
            if (ret as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                current_block = 618630553461347817;
            } else if (*sifted_states.offset(0 as libc::c_int as isize)).is_null() {
                ret = _REG_NOMATCH;
                current_block = 618630553461347817;
            } else {
                current_block = 2569451025026770673;
            }
        }
        match current_block {
            618630553461347817 => {}
            _ => {
                rpl_free((*mctx).state_log as *mut libc::c_void);
                (*mctx).state_log = sifted_states;
                sifted_states = 0 as *mut *mut re_dfastate_t;
                (*mctx).last_node = halt_node;
                (*mctx).match_last = match_last;
                ret = _REG_NOERROR;
            }
        }
    }
    rpl_free(sifted_states as *mut libc::c_void);
    rpl_free(lim_states as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn regmatch_list_mark_failed(mut list: *mut regmatch_list) {
    regmatch_list_free__elements__(
        (*list).u.dynarray_header.array,
        (*list).u.dynarray_header.used,
    );
    regmatch_list_free__array__(list);
    (*list).u.dynarray_header.array = ((*list).scratch).as_mut_ptr();
    (*list).u.dynarray_header.used = 0 as libc::c_int as size_t;
    (*list).u.dynarray_header.allocated = __dynarray_error_marker();
}
unsafe extern "C" fn match_ctx_free(mut mctx: *mut re_match_context_t) {
    match_ctx_clean(mctx);
    rpl_free((*mctx).sub_tops as *mut libc::c_void);
    rpl_free((*mctx).bkref_ents as *mut libc::c_void);
}
unsafe extern "C" fn update_regs(
    mut dfa: *const re_dfa_t,
    mut pmatch: *mut regmatch_t,
    mut prev_idx_match: *mut regmatch_t,
    mut cur_node: Idx,
    mut cur_idx: Idx,
    mut nmatch: Idx,
) {
    let mut type_0: libc::c_int = (*((*dfa).nodes).offset(cur_node as isize)).type_0()
        as libc::c_int;
    if type_0 == OP_OPEN_SUBEXP as libc::c_int {
        let mut reg_num: Idx = (*((*dfa).nodes).offset(cur_node as isize)).opr.idx
            + 1 as libc::c_int as libc::c_long;
        if reg_num < nmatch {
            (*pmatch.offset(reg_num as isize)).rm_so = cur_idx;
            (*pmatch.offset(reg_num as isize)).rm_eo = -(1 as libc::c_int) as regoff_t;
        }
    } else if type_0 == OP_CLOSE_SUBEXP as libc::c_int {
        let mut reg_num_0: Idx = (*((*dfa).nodes).offset(cur_node as isize)).opr.idx
            + 1 as libc::c_int as libc::c_long;
        if reg_num_0 < nmatch {
            if (*pmatch.offset(reg_num_0 as isize)).rm_so < cur_idx {
                (*pmatch.offset(reg_num_0 as isize)).rm_eo = cur_idx;
                memcpy(
                    prev_idx_match as *mut libc::c_void,
                    pmatch as *const libc::c_void,
                    (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
                        .wrapping_mul(nmatch as libc::c_ulong),
                );
            } else if (*((*dfa).nodes).offset(cur_node as isize)).opt_subexp()
                as libc::c_int != 0
                && (*prev_idx_match.offset(reg_num_0 as isize)).rm_so
                    != -(1 as libc::c_int) as libc::c_long
            {
                memcpy(
                    pmatch as *mut libc::c_void,
                    prev_idx_match as *const libc::c_void,
                    (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
                        .wrapping_mul(nmatch as libc::c_ulong),
                );
            } else {
                (*pmatch.offset(reg_num_0 as isize)).rm_eo = cur_idx;
            }
        }
    }
}
unsafe extern "C" fn push_fail_stack(
    mut fs: *mut re_fail_stack_t,
    mut str_idx: Idx,
    mut dest_node: Idx,
    mut nregs: Idx,
    mut regs: *mut regmatch_t,
    mut prevregs: *mut regmatch_t,
    mut eps_via_nodes: *mut re_node_set,
) -> reg_errcode_t {
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut num: Idx = (*fs).num;
    if num == (*fs).alloc {
        let mut new_array: *mut re_fail_stack_ent_t = 0 as *mut re_fail_stack_ent_t;
        new_array = realloc(
            (*fs).stack as *mut libc::c_void,
            (((*fs).alloc * 2 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<re_fail_stack_ent_t>() as libc::c_ulong,
                ),
        ) as *mut re_fail_stack_ent_t;
        if new_array.is_null() {
            return _REG_ESPACE;
        }
        (*fs).alloc *= 2 as libc::c_int as libc::c_long;
        (*fs).stack = new_array;
    }
    (*((*fs).stack).offset(num as isize)).idx = str_idx;
    (*((*fs).stack).offset(num as isize)).node = dest_node;
    let ref mut fresh86 = (*((*fs).stack).offset(num as isize)).regs;
    *fresh86 = malloc(
        ((2 as libc::c_int as libc::c_long * nregs) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<regmatch_t>() as libc::c_ulong),
    ) as *mut regmatch_t;
    if ((*((*fs).stack).offset(num as isize)).regs).is_null() {
        return _REG_ESPACE;
    }
    (*fs).num = num + 1 as libc::c_int as libc::c_long;
    memcpy(
        (*((*fs).stack).offset(num as isize)).regs as *mut libc::c_void,
        regs as *const libc::c_void,
        (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
            .wrapping_mul(nregs as libc::c_ulong),
    );
    memcpy(
        ((*((*fs).stack).offset(num as isize)).regs).offset(nregs as isize)
            as *mut libc::c_void,
        prevregs as *const libc::c_void,
        (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
            .wrapping_mul(nregs as libc::c_ulong),
    );
    err = re_node_set_init_copy(
        &mut (*((*fs).stack).offset(num as isize)).eps_via_nodes,
        eps_via_nodes,
    );
    return err;
}
unsafe extern "C" fn check_node_accept_bytes(
    mut dfa: *const re_dfa_t,
    mut node_idx: Idx,
    mut input: *const re_string_t,
    mut str_idx: Idx,
) -> libc::c_int {
    let mut current_block: u64;
    let mut node: *const re_token_t = ((*dfa).nodes).offset(node_idx as isize);
    let mut char_len: libc::c_int = 0;
    let mut elem_len: libc::c_int = 0;
    let mut i: Idx = 0;
    if ((*node).type_0() as libc::c_int == OP_UTF8_PERIOD as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        let mut c: libc::c_uchar = *((*input).mbs).offset(str_idx as isize);
        let mut d: libc::c_uchar = 0;
        if ((c as libc::c_int) < 0xc2 as libc::c_int) as libc::c_int as libc::c_long != 0
        {
            return 0 as libc::c_int;
        }
        if str_idx + 2 as libc::c_int as libc::c_long > (*input).len {
            return 0 as libc::c_int;
        }
        d = *((*input).mbs)
            .offset((str_idx + 1 as libc::c_int as libc::c_long) as isize);
        if (c as libc::c_int) < 0xe0 as libc::c_int {
            return if (d as libc::c_int) < 0x80 as libc::c_int
                || d as libc::c_int > 0xbf as libc::c_int
            {
                0 as libc::c_int
            } else {
                2 as libc::c_int
            }
        } else if (c as libc::c_int) < 0xf0 as libc::c_int {
            char_len = 3 as libc::c_int;
            if c as libc::c_int == 0xe0 as libc::c_int
                && (d as libc::c_int) < 0xa0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        } else if (c as libc::c_int) < 0xf8 as libc::c_int {
            char_len = 4 as libc::c_int;
            if c as libc::c_int == 0xf0 as libc::c_int
                && (d as libc::c_int) < 0x90 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        } else if (c as libc::c_int) < 0xfc as libc::c_int {
            char_len = 5 as libc::c_int;
            if c as libc::c_int == 0xf8 as libc::c_int
                && (d as libc::c_int) < 0x88 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        } else if (c as libc::c_int) < 0xfe as libc::c_int {
            char_len = 6 as libc::c_int;
            if c as libc::c_int == 0xfc as libc::c_int
                && (d as libc::c_int) < 0x84 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        } else {
            return 0 as libc::c_int
        }
        if str_idx + char_len as libc::c_long > (*input).len {
            return 0 as libc::c_int;
        }
        i = 1 as libc::c_int as Idx;
        while i < char_len as libc::c_long {
            d = *((*input).mbs).offset((str_idx + i) as isize);
            if (d as libc::c_int) < 0x80 as libc::c_int
                || d as libc::c_int > 0xbf as libc::c_int
            {
                return 0 as libc::c_int;
            }
            i += 1;
            i;
        }
        return char_len;
    }
    char_len = re_string_char_size_at(input, str_idx);
    if (*node).type_0() as libc::c_int == OP_PERIOD as libc::c_int {
        if char_len <= 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        if (*dfa).syntax
            & ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int == 0
            && *((*input).mbs).offset(str_idx as isize) as libc::c_int == '\n' as i32
            || (*dfa).syntax
                & (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int != 0
                && *((*input).mbs).offset(str_idx as isize) as libc::c_int == '\0' as i32
        {
            return 0 as libc::c_int;
        }
        return char_len;
    }
    elem_len = re_string_elem_size_at(input, str_idx);
    if elem_len <= 1 as libc::c_int && char_len <= 1 as libc::c_int
        || char_len == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*node).type_0() as libc::c_int == COMPLEX_BRACKET as libc::c_int {
        let mut cset: *const re_charset_t = (*node).opr.mbcset;
        let mut match_len: libc::c_int = 0 as libc::c_int;
        let mut wc: wchar_t = (if (*cset).nranges != 0 || (*cset).nchar_classes != 0
            || (*cset).nmbchars != 0
        {
            re_string_wchar_at(input, str_idx)
        } else {
            0 as libc::c_int as libc::c_uint
        }) as wchar_t;
        i = 0 as libc::c_int as Idx;
        loop {
            if !(i < (*cset).nmbchars) {
                current_block = 1622411330066726685;
                break;
            }
            if wc == *((*cset).mbchars).offset(i as isize) {
                match_len = char_len;
                current_block = 9591855682380032028;
                break;
            } else {
                i += 1;
                i;
            }
        }
        match current_block {
            1622411330066726685 => {
                i = 0 as libc::c_int as Idx;
                loop {
                    if !(i < (*cset).nchar_classes) {
                        current_block = 3689906465960840878;
                        break;
                    }
                    let mut wt: wctype_t = *((*cset).char_classes).offset(i as isize);
                    if iswctype(wc as wint_t, wt) != 0 {
                        match_len = char_len;
                        current_block = 9591855682380032028;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
                match current_block {
                    9591855682380032028 => {}
                    _ => {
                        i = 0 as libc::c_int as Idx;
                        while i < (*cset).nranges {
                            if *((*cset).range_starts).offset(i as isize) <= wc
                                && wc <= *((*cset).range_ends).offset(i as isize)
                            {
                                match_len = char_len;
                                break;
                            } else {
                                i += 1;
                                i;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        if (*cset).non_match() == 0 {
            return match_len
        } else if match_len > 0 as libc::c_int {
            return 0 as libc::c_int
        } else {
            return if elem_len > char_len { elem_len } else { char_len }
        }
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn check_node_accept(
    mut mctx: *const re_match_context_t,
    mut node: *const re_token_t,
    mut idx: Idx,
) -> bool {
    let mut ch: libc::c_uchar = 0;
    ch = *((*mctx).input.mbs).offset(idx as isize);
    let mut current_block_10: u64;
    match (*node).type_0() as libc::c_int {
        1 => {
            if (*node).opr.c as libc::c_int != ch as libc::c_int {
                return 0 as libc::c_int != 0;
            }
            current_block_10 = 7746791466490516765;
        }
        3 => {
            if !bitset_contain((*node).opr.sbcset as *const bitset_word_t, ch as Idx) {
                return 0 as libc::c_int != 0;
            }
            current_block_10 = 7746791466490516765;
        }
        7 => {
            if ch as libc::c_int >= 0x80 as libc::c_int {
                return 0 as libc::c_int != 0;
            }
            current_block_10 = 6872472373556633120;
        }
        5 => {
            current_block_10 = 6872472373556633120;
        }
        _ => return 0 as libc::c_int != 0,
    }
    match current_block_10 {
        6872472373556633120 => {
            if ch as libc::c_int == '\n' as i32
                && (*(*mctx).dfa).syntax
                    & ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int == 0
                || ch as libc::c_int == '\0' as i32
                    && (*(*mctx).dfa).syntax
                        & (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int != 0
            {
                return 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    if (*node).constraint() != 0 {
        let mut context: libc::c_uint = re_string_context_at(
            &(*mctx).input,
            idx,
            (*mctx).eflags,
        );
        if (*node).constraint() as libc::c_int & 0x4 as libc::c_int != 0
            && context & 1 as libc::c_int as libc::c_uint == 0
            || (*node).constraint() as libc::c_int & 0x8 as libc::c_int != 0
                && context & 1 as libc::c_int as libc::c_uint != 0
            || (*node).constraint() as libc::c_int & 0x20 as libc::c_int != 0
                && context & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                    == 0
            || (*node).constraint() as libc::c_int & 0x80 as libc::c_int != 0
                && context
                    & ((((1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) as libc::c_uint == 0
        {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn proceed_next_node(
    mut mctx: *const re_match_context_t,
    mut nregs: Idx,
    mut regs: *mut regmatch_t,
    mut prevregs: *mut regmatch_t,
    mut pidx: *mut Idx,
    mut node: Idx,
    mut eps_via_nodes: *mut re_node_set,
    mut fs: *mut re_fail_stack_t,
) -> Idx {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    if (*((*dfa).nodes).offset(node as isize)).type_0() as libc::c_int & 8 as libc::c_int
        != 0
    {
        let mut cur_nodes: *mut re_node_set = &mut (**((*mctx).state_log)
            .offset(*pidx as isize))
            .nodes;
        let mut edests: *mut re_node_set = &mut *((*dfa).edests).offset(node as isize)
            as *mut re_node_set;
        if re_node_set_contains(eps_via_nodes, node) == 0 {
            let mut ok: bool = re_node_set_insert(eps_via_nodes, node);
            if !ok as libc::c_int as libc::c_long != 0 {
                return -(2 as libc::c_int) as Idx;
            }
        }
        let mut dest_node: Idx = -(1 as libc::c_int) as Idx;
        let mut i: Idx = 0 as libc::c_int as Idx;
        while i < (*edests).nelem {
            let mut candidate: Idx = *((*edests).elems).offset(i as isize);
            if !(re_node_set_contains(cur_nodes, candidate) == 0) {
                if dest_node == -(1 as libc::c_int) as libc::c_long {
                    dest_node = candidate;
                } else {
                    if re_node_set_contains(eps_via_nodes, dest_node) != 0 {
                        return candidate
                    } else if !fs.is_null()
                        && push_fail_stack(
                            fs,
                            *pidx,
                            candidate,
                            nregs,
                            regs,
                            prevregs,
                            eps_via_nodes,
                        ) as libc::c_int != 0
                    {
                        return -(2 as libc::c_int) as Idx
                    }
                    break;
                }
            }
            i += 1;
            i;
        }
        return dest_node;
    } else {
        let mut naccepted: Idx = 0 as libc::c_int as Idx;
        let mut type_0: re_token_type_t = (*((*dfa).nodes).offset(node as isize))
            .type_0();
        if (*((*dfa).nodes).offset(node as isize)).accept_mb() != 0 {
            naccepted = check_node_accept_bytes(dfa, node, &(*mctx).input, *pidx) as Idx;
        } else if type_0 as libc::c_uint == OP_BACK_REF as libc::c_int as libc::c_uint {
            let mut subexp_idx: Idx = (*((*dfa).nodes).offset(node as isize)).opr.idx
                + 1 as libc::c_int as libc::c_long;
            if subexp_idx < nregs {
                naccepted = (*regs.offset(subexp_idx as isize)).rm_eo
                    - (*regs.offset(subexp_idx as isize)).rm_so;
            }
            if !fs.is_null() {
                if subexp_idx >= nregs
                    || (*regs.offset(subexp_idx as isize)).rm_so
                        == -(1 as libc::c_int) as libc::c_long
                    || (*regs.offset(subexp_idx as isize)).rm_eo
                        == -(1 as libc::c_int) as libc::c_long
                {
                    return -(1 as libc::c_int) as Idx
                } else if naccepted != 0 {
                    let mut buf: *mut libc::c_char = (*mctx).input.mbs
                        as *mut libc::c_char;
                    if (*mctx).input.valid_len - *pidx < naccepted
                        || memcmp(
                            buf
                                .offset((*regs.offset(subexp_idx as isize)).rm_so as isize)
                                as *const libc::c_void,
                            buf.offset(*pidx as isize) as *const libc::c_void,
                            naccepted as libc::c_ulong,
                        ) != 0 as libc::c_int
                    {
                        return -(1 as libc::c_int) as Idx;
                    }
                }
            }
            if naccepted == 0 as libc::c_int as libc::c_long {
                let mut dest_node_0: Idx = 0;
                let mut ok_0: bool = re_node_set_insert(eps_via_nodes, node);
                if !ok_0 as libc::c_int as libc::c_long != 0 {
                    return -(2 as libc::c_int) as Idx;
                }
                dest_node_0 = *((*((*dfa).edests).offset(node as isize)).elems)
                    .offset(0 as libc::c_int as isize);
                if re_node_set_contains(
                    &mut (**((*mctx).state_log).offset(*pidx as isize)).nodes,
                    dest_node_0,
                ) != 0
                {
                    return dest_node_0;
                }
            }
        }
        if naccepted != 0 as libc::c_int as libc::c_long
            || check_node_accept(mctx, ((*dfa).nodes).offset(node as isize), *pidx)
                as libc::c_int != 0
        {
            let mut dest_node_1: Idx = *((*dfa).nexts).offset(node as isize);
            *pidx = if naccepted == 0 as libc::c_int as libc::c_long {
                *pidx + 1 as libc::c_int as libc::c_long
            } else {
                *pidx + naccepted
            };
            if !fs.is_null()
                && (*pidx > (*mctx).match_last
                    || (*((*mctx).state_log).offset(*pidx as isize)).is_null()
                    || re_node_set_contains(
                        &mut (**((*mctx).state_log).offset(*pidx as isize)).nodes,
                        dest_node_1,
                    ) == 0)
            {
                return -(1 as libc::c_int) as Idx;
            }
            (*eps_via_nodes).nelem = 0 as libc::c_int as Idx;
            return dest_node_1;
        }
    }
    return -(1 as libc::c_int) as Idx;
}
unsafe extern "C" fn pop_fail_stack(
    mut fs: *mut re_fail_stack_t,
    mut pidx: *mut Idx,
    mut nregs: Idx,
    mut regs: *mut regmatch_t,
    mut prevregs: *mut regmatch_t,
    mut eps_via_nodes: *mut re_node_set,
) -> Idx {
    if fs.is_null() || (*fs).num == 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as Idx;
    }
    (*fs).num -= 1;
    let mut num: Idx = (*fs).num;
    *pidx = (*((*fs).stack).offset(num as isize)).idx;
    memcpy(
        regs as *mut libc::c_void,
        (*((*fs).stack).offset(num as isize)).regs as *const libc::c_void,
        (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
            .wrapping_mul(nregs as libc::c_ulong),
    );
    memcpy(
        prevregs as *mut libc::c_void,
        ((*((*fs).stack).offset(num as isize)).regs).offset(nregs as isize)
            as *const libc::c_void,
        (::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
            .wrapping_mul(nregs as libc::c_ulong),
    );
    rpl_free((*eps_via_nodes).elems as *mut libc::c_void);
    rpl_free((*((*fs).stack).offset(num as isize)).regs as *mut libc::c_void);
    *eps_via_nodes = (*((*fs).stack).offset(num as isize)).eps_via_nodes;
    if 0 as libc::c_int as libc::c_long <= (*((*fs).stack).offset(num as isize)).node
    {} else {
        unreachable!();
    };
    return (*((*fs).stack).offset(num as isize)).node;
}
unsafe extern "C" fn check_arrival_expand_ecl(
    mut dfa: *const re_dfa_t,
    mut cur_nodes: *mut re_node_set,
    mut ex_subexp: Idx,
    mut type_0: libc::c_int,
) -> reg_errcode_t {
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut idx: Idx = 0;
    let mut outside_node: Idx = 0;
    let mut new_nodes: re_node_set = re_node_set {
        alloc: 0,
        nelem: 0,
        elems: 0 as *mut Idx,
    };
    if (*cur_nodes).nelem != 0 {} else {
        unreachable!();
    };
    err = re_node_set_alloc(&mut new_nodes, (*cur_nodes).nelem);
    if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        return err;
    }
    idx = 0 as libc::c_int as Idx;
    while idx < (*cur_nodes).nelem {
        let mut cur_node: Idx = *((*cur_nodes).elems).offset(idx as isize);
        let mut eclosure: *const re_node_set = ((*dfa).eclosures)
            .offset(cur_node as isize);
        outside_node = find_subexp_node(dfa, eclosure, ex_subexp, type_0);
        if outside_node == -(1 as libc::c_int) as libc::c_long {
            err = re_node_set_merge(&mut new_nodes, eclosure);
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                rpl_free(new_nodes.elems as *mut libc::c_void);
                return err;
            }
        } else {
            err = check_arrival_expand_ecl_sub(
                dfa,
                &mut new_nodes,
                cur_node,
                ex_subexp,
                type_0,
            );
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                rpl_free(new_nodes.elems as *mut libc::c_void);
                return err;
            }
        }
        idx += 1;
        idx;
    }
    rpl_free((*cur_nodes).elems as *mut libc::c_void);
    *cur_nodes = new_nodes;
    return _REG_NOERROR;
}
unsafe extern "C" fn check_subexp_matching_top(
    mut mctx: *mut re_match_context_t,
    mut cur_nodes: *mut re_node_set,
    mut str_idx: Idx,
) -> reg_errcode_t {
    let dfa: *const re_dfa_t = (*mctx).dfa;
    let mut node_idx: Idx = 0;
    let mut err: reg_errcode_t = _REG_NOERROR;
    node_idx = 0 as libc::c_int as Idx;
    while node_idx < (*cur_nodes).nelem {
        let mut node: Idx = *((*cur_nodes).elems).offset(node_idx as isize);
        if (*((*dfa).nodes).offset(node as isize)).type_0() as libc::c_int
            == OP_OPEN_SUBEXP as libc::c_int
            && (*((*dfa).nodes).offset(node as isize)).opr.idx
                < 64 as libc::c_int as libc::c_long
            && (*dfa).used_bkref_map
                & (1 as libc::c_int as bitset_word_t)
                    << (*((*dfa).nodes).offset(node as isize)).opr.idx != 0
        {
            err = match_ctx_add_subtop(mctx, node, str_idx);
            if (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return err;
            }
        }
        node_idx += 1;
        node_idx;
    }
    return _REG_NOERROR;
}
#[inline]
unsafe extern "C" fn regmatch_list_free__elements__(
    mut __dynarray_array: *mut regmatch_t,
    mut __dynarray_used: size_t,
) {}
unsafe extern "C" fn match_ctx_clean(mut mctx: *mut re_match_context_t) {
    let mut st_idx: Idx = 0;
    st_idx = 0 as libc::c_int as Idx;
    while st_idx < (*mctx).nsub_tops {
        let mut sl_idx: Idx = 0;
        let mut top: *mut re_sub_match_top_t = *((*mctx).sub_tops)
            .offset(st_idx as isize);
        sl_idx = 0 as libc::c_int as Idx;
        while sl_idx < (*top).nlasts {
            let mut last: *mut re_sub_match_last_t = *((*top).lasts)
                .offset(sl_idx as isize);
            rpl_free((*last).path.array as *mut libc::c_void);
            rpl_free(last as *mut libc::c_void);
            sl_idx += 1;
            sl_idx;
        }
        rpl_free((*top).lasts as *mut libc::c_void);
        if !((*top).path).is_null() {
            rpl_free((*(*top).path).array as *mut libc::c_void);
            rpl_free((*top).path as *mut libc::c_void);
        }
        rpl_free(top as *mut libc::c_void);
        st_idx += 1;
        st_idx;
    }
    (*mctx).nsub_tops = 0 as libc::c_int as Idx;
    (*mctx).nbkref_ents = 0 as libc::c_int as Idx;
}
unsafe extern "C" fn set_regs(
    mut preg: *const regex_t,
    mut mctx: *const re_match_context_t,
    mut nmatch: size_t,
    mut pmatch: *mut regmatch_t,
    mut fl_backtrack: bool,
) -> reg_errcode_t {
    let mut dfa: *const re_dfa_t = (*preg).buffer;
    let mut idx: Idx = 0;
    let mut cur_node: Idx = 0;
    let mut eps_via_nodes: re_node_set = re_node_set {
        alloc: 0,
        nelem: 0,
        elems: 0 as *mut Idx,
    };
    let mut fs: *mut re_fail_stack_t = 0 as *mut re_fail_stack_t;
    let mut fs_body: re_fail_stack_t = {
        let mut init = re_fail_stack_t {
            num: 0 as libc::c_int as Idx,
            alloc: 2 as libc::c_int as Idx,
            stack: 0 as *mut re_fail_stack_ent_t,
        };
        init
    };
    let mut prev_match: regmatch_list = regmatch_list {
        u: C2RustUnnamed_2 {
            dynarray_abstract: dynarray_header {
                used: 0,
                allocated: 0,
                array: 0 as *mut libc::c_void,
            },
        },
        scratch: [regmatch_t { rm_so: 0, rm_eo: 0 }; 8],
    };
    regmatch_list_init(&mut prev_match);
    if nmatch > 1 as libc::c_int as libc::c_ulong {} else {
        unreachable!();
    };
    if !((*mctx).state_log).is_null() {} else {
        unreachable!();
    };
    if fl_backtrack {
        fs = &mut fs_body;
        (*fs)
            .stack = malloc(
            ((*fs).alloc as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<re_fail_stack_ent_t>() as libc::c_ulong,
                ),
        ) as *mut re_fail_stack_ent_t;
        if ((*fs).stack).is_null() {
            return _REG_ESPACE;
        }
    } else {
        fs = 0 as *mut re_fail_stack_t;
    }
    cur_node = (*dfa).init_node;
    memset(
        &mut eps_via_nodes as *mut re_node_set as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<re_node_set>() as libc::c_ulong,
    );
    if !regmatch_list_resize(&mut prev_match, nmatch) {
        regmatch_list_free(&mut prev_match);
        free_fail_stack_return(fs);
        return _REG_ESPACE;
    }
    let mut prev_idx_match: *mut regmatch_t = regmatch_list_begin(&mut prev_match);
    memcpy(
        prev_idx_match as *mut libc::c_void,
        pmatch as *const libc::c_void,
        (::core::mem::size_of::<regmatch_t>() as libc::c_ulong).wrapping_mul(nmatch),
    );
    idx = (*pmatch.offset(0 as libc::c_int as isize)).rm_so;
    while idx <= (*pmatch.offset(0 as libc::c_int as isize)).rm_eo {
        update_regs(dfa, pmatch, prev_idx_match, cur_node, idx, nmatch as Idx);
        if idx == (*pmatch.offset(0 as libc::c_int as isize)).rm_eo
            && cur_node == (*mctx).last_node
            || !fs.is_null() && re_node_set_contains(&mut eps_via_nodes, cur_node) != 0
        {
            let mut reg_idx: Idx = 0;
            cur_node = -(1 as libc::c_int) as Idx;
            if !fs.is_null() {
                reg_idx = 0 as libc::c_int as Idx;
                while (reg_idx as libc::c_ulong) < nmatch {
                    if (*pmatch.offset(reg_idx as isize)).rm_so
                        > -(1 as libc::c_int) as libc::c_long
                        && (*pmatch.offset(reg_idx as isize)).rm_eo
                            == -(1 as libc::c_int) as libc::c_long
                    {
                        cur_node = pop_fail_stack(
                            fs,
                            &mut idx,
                            nmatch as Idx,
                            pmatch,
                            prev_idx_match,
                            &mut eps_via_nodes,
                        );
                        break;
                    } else {
                        reg_idx += 1;
                        reg_idx;
                    }
                }
            }
            if cur_node < 0 as libc::c_int as libc::c_long {
                rpl_free(eps_via_nodes.elems as *mut libc::c_void);
                regmatch_list_free(&mut prev_match);
                return free_fail_stack_return(fs);
            }
        }
        cur_node = proceed_next_node(
            mctx,
            nmatch as Idx,
            pmatch,
            prev_idx_match,
            &mut idx,
            cur_node,
            &mut eps_via_nodes,
            fs,
        );
        if (cur_node < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long
            != 0
        {
            if (cur_node == -(2 as libc::c_int) as libc::c_long) as libc::c_int
                as libc::c_long != 0
            {
                rpl_free(eps_via_nodes.elems as *mut libc::c_void);
                regmatch_list_free(&mut prev_match);
                free_fail_stack_return(fs);
                return _REG_ESPACE;
            }
            cur_node = pop_fail_stack(
                fs,
                &mut idx,
                nmatch as Idx,
                pmatch,
                prev_idx_match,
                &mut eps_via_nodes,
            );
            if cur_node < 0 as libc::c_int as libc::c_long {
                rpl_free(eps_via_nodes.elems as *mut libc::c_void);
                regmatch_list_free(&mut prev_match);
                free_fail_stack_return(fs);
                return _REG_NOMATCH;
            }
        }
    }
    rpl_free(eps_via_nodes.elems as *mut libc::c_void);
    regmatch_list_free(&mut prev_match);
    return free_fail_stack_return(fs);
}
unsafe extern "C" fn free_fail_stack_return(
    mut fs: *mut re_fail_stack_t,
) -> reg_errcode_t {
    if !fs.is_null() {
        let mut fs_idx: Idx = 0;
        fs_idx = 0 as libc::c_int as Idx;
        while fs_idx < (*fs).num {
            rpl_free(
                (*((*fs).stack).offset(fs_idx as isize)).eps_via_nodes.elems
                    as *mut libc::c_void,
            );
            rpl_free((*((*fs).stack).offset(fs_idx as isize)).regs as *mut libc::c_void);
            fs_idx += 1;
            fs_idx;
        }
        rpl_free((*fs).stack as *mut libc::c_void);
    }
    return _REG_NOERROR;
}
unsafe extern "C" fn regmatch_list_free(mut list: *mut regmatch_list) {
    regmatch_list_free__elements__(
        (*list).u.dynarray_header.array,
        (*list).u.dynarray_header.used,
    );
    regmatch_list_free__array__(list);
    regmatch_list_init(list);
}
unsafe extern "C" fn regmatch_list_init(mut list: *mut regmatch_list) {
    (*list).u.dynarray_header.used = 0 as libc::c_int as size_t;
    (*list)
        .u
        .dynarray_header
        .allocated = if ::core::mem::size_of::<regmatch_t>() as libc::c_ulong
        > 64 as libc::c_int as libc::c_ulong
    {
        2 as libc::c_int as libc::c_ulong
    } else {
        (128 as libc::c_int as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<regmatch_t>() as libc::c_ulong)
    };
    (*list).u.dynarray_header.array = ((*list).scratch).as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn rpl_regexec(
    mut preg: *const regex_t,
    mut string: *const libc::c_char,
    mut nmatch: size_t,
    mut pmatch: *mut regmatch_t,
    mut eflags: libc::c_int,
) -> libc::c_int {
    let mut err: reg_errcode_t = _REG_NOERROR;
    let mut start: Idx = 0;
    let mut length: Idx = 0;
    let mut dfa: *mut re_dfa_t = (*preg).buffer;
    if eflags
        & !(1 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 2 as libc::c_int) != 0
    {
        return _REG_BADPAT as libc::c_int;
    }
    if eflags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        start = (*pmatch.offset(0 as libc::c_int as isize)).rm_so;
        length = (*pmatch.offset(0 as libc::c_int as isize)).rm_eo;
    } else {
        start = 0 as libc::c_int as Idx;
        length = strlen(string) as Idx;
    }
    if (*preg).no_sub() != 0 {
        err = re_search_internal(
            preg,
            string,
            length,
            start,
            length,
            length,
            0 as libc::c_int as size_t,
            0 as *mut regmatch_t,
            eflags,
        );
    } else {
        err = re_search_internal(
            preg,
            string,
            length,
            start,
            length,
            length,
            nmatch,
            pmatch,
            eflags,
        );
    }
    return (err as libc::c_int != _REG_NOERROR as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn regmatch_list_free__array__(mut list: *mut regmatch_list) {
    if (*list).u.dynarray_header.array != ((*list).scratch).as_mut_ptr() {
        rpl_free((*list).u.dynarray_header.array as *mut libc::c_void);
    }
}
unsafe extern "C" fn regmatch_list_resize(
    mut list: *mut regmatch_list,
    mut size: size_t,
) -> bool {
    if size > (*list).u.dynarray_header.used {
        let mut ok: bool = false;
        ok = gl_dynarray_resize(
            &mut (*list).u.dynarray_abstract,
            size,
            ((*list).scratch).as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<regmatch_t>() as libc::c_ulong,
        );
        if !ok as libc::c_int as libc::c_long != 0 {
            regmatch_list_mark_failed(list);
        }
        return ok;
    } else {
        regmatch_list_free__elements__(
            ((*list).u.dynarray_header.array).offset(size as isize),
            ((*list).u.dynarray_header.used).wrapping_sub(size),
        );
        (*list).u.dynarray_header.used = size;
        return 1 as libc::c_int != 0;
    };
}
unsafe extern "C" fn run_static_initializers() {
    __re_error_msgid_idx = [
        0 as libc::c_int as size_t,
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong),
    ];
    utf8_sb_map = [
        (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong),
        (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong),
        0,
        0,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
