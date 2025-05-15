use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type hash_table;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn symbol_is_function(sym: *mut Symbol) -> i32;
    fn linked_list_destroy(plist: *mut *mut linked_list);
    fn linked_list_iterate(
        plist: *mut *mut linked_list,
        itr: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> i32>,
        data: *mut libc::c_void,
    );
    fn linked_list_unlink(list: *mut linked_list, ent: *mut linked_list_entry);
    fn data_in_list(data: *mut libc::c_void, list: *mut linked_list) -> i32;
    fn linked_list_append(plist: *mut *mut linked_list, data: *mut libc::c_void);
    fn xalloc_die() -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    static mut reverse_tree: i32;
    static mut filename: *mut i8;
    static mut canonical_filename: *mut i8;
    fn hash_lookup(_: *const Hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_do_for_each(
        _: *const Hash_table,
        _: Hash_processor,
        _: *mut libc::c_void,
    ) -> size_t;
    fn hash_string(_: *const i8, _: size_t) -> size_t;
    fn hash_initialize(
        _: size_t,
        _: *const Hash_tuning,
        _: Hash_hasher,
        _: Hash_comparator,
        _: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(_: *mut Hash_table, _: *const libc::c_void) -> *mut libc::c_void;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_list_entry {
    pub next: *mut linked_list_entry,
    pub prev: *mut linked_list_entry,
    pub list: *mut linked_list,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_list {
    pub free_data: linked_list_free_data_fp,
    pub head: *mut linked_list_entry,
    pub tail: *mut linked_list_entry,
}
pub type linked_list_free_data_fp = Option<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum symtype {
    SymUndefined,
    SymToken,
    SymIdentifier,
}
impl symtype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            symtype::SymUndefined => 0,
            symtype::SymToken => 1,
            symtype::SymIdentifier => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> symtype {
        match value {
            0 => symtype::SymUndefined,
            1 => symtype::SymToken,
            2 => symtype::SymIdentifier,
            _ => panic!("Invalid value for symtype: {}", value),
        }
    }
}
impl AddAssign<u32> for symtype {
    fn add_assign(&mut self, rhs: u32) {
        *self = symtype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for symtype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = symtype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for symtype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = symtype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for symtype {
    fn div_assign(&mut self, rhs: u32) {
        *self = symtype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for symtype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = symtype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for symtype {
    type Output = symtype;
    fn add(self, rhs: u32) -> symtype {
        symtype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for symtype {
    type Output = symtype;
    fn sub(self, rhs: u32) -> symtype {
        symtype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for symtype {
    type Output = symtype;
    fn mul(self, rhs: u32) -> symtype {
        symtype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for symtype {
    type Output = symtype;
    fn div(self, rhs: u32) -> symtype {
        symtype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for symtype {
    type Output = symtype;
    fn rem(self, rhs: u32) -> symtype {
        symtype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum storage {
    ExternStorage,
    ExplicitExternStorage,
    StaticStorage,
    AutoStorage,
    AnyStorage,
}
impl storage {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            storage::ExternStorage => 0,
            storage::ExplicitExternStorage => 1,
            storage::StaticStorage => 2,
            storage::AutoStorage => 3,
            storage::AnyStorage => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> storage {
        match value {
            0 => storage::ExternStorage,
            1 => storage::ExplicitExternStorage,
            2 => storage::StaticStorage,
            3 => storage::AutoStorage,
            4 => storage::AnyStorage,
            _ => panic!("Invalid value for storage: {}", value),
        }
    }
}
impl AddAssign<u32> for storage {
    fn add_assign(&mut self, rhs: u32) {
        *self = storage::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for storage {
    fn sub_assign(&mut self, rhs: u32) {
        *self = storage::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for storage {
    fn mul_assign(&mut self, rhs: u32) {
        *self = storage::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for storage {
    fn div_assign(&mut self, rhs: u32) {
        *self = storage::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for storage {
    fn rem_assign(&mut self, rhs: u32) {
        *self = storage::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for storage {
    type Output = storage;
    fn add(self, rhs: u32) -> storage {
        storage::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for storage {
    type Output = storage;
    fn sub(self, rhs: u32) -> storage {
        storage::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for storage {
    type Output = storage;
    fn mul(self, rhs: u32) -> storage {
        storage::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for storage {
    type Output = storage;
    fn div(self, rhs: u32) -> storage {
        storage::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for storage {
    type Output = storage;
    fn rem(self, rhs: u32) -> storage {
        storage::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum symbol_flag {
    symbol_none,
    symbol_temp,
    symbol_parm,
    symbol_alias,
}
impl symbol_flag {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            symbol_flag::symbol_none => 0,
            symbol_flag::symbol_temp => 1,
            symbol_flag::symbol_parm => 2,
            symbol_flag::symbol_alias => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> symbol_flag {
        match value {
            0 => symbol_flag::symbol_none,
            1 => symbol_flag::symbol_temp,
            2 => symbol_flag::symbol_parm,
            3 => symbol_flag::symbol_alias,
            _ => panic!("Invalid value for symbol_flag: {}", value),
        }
    }
}
impl AddAssign<u32> for symbol_flag {
    fn add_assign(&mut self, rhs: u32) {
        *self = symbol_flag::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for symbol_flag {
    fn sub_assign(&mut self, rhs: u32) {
        *self = symbol_flag::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for symbol_flag {
    fn mul_assign(&mut self, rhs: u32) {
        *self = symbol_flag::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for symbol_flag {
    fn div_assign(&mut self, rhs: u32) {
        *self = symbol_flag::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for symbol_flag {
    fn rem_assign(&mut self, rhs: u32) {
        *self = symbol_flag::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for symbol_flag {
    type Output = symbol_flag;
    fn add(self, rhs: u32) -> symbol_flag {
        symbol_flag::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for symbol_flag {
    type Output = symbol_flag;
    fn sub(self, rhs: u32) -> symbol_flag {
        symbol_flag::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for symbol_flag {
    type Output = symbol_flag;
    fn mul(self, rhs: u32) -> symbol_flag {
        symbol_flag::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for symbol_flag {
    type Output = symbol_flag;
    fn div(self, rhs: u32) -> symbol_flag {
        symbol_flag::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for symbol_flag {
    type Output = symbol_flag;
    fn rem(self, rhs: u32) -> symbol_flag {
        symbol_flag::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol {
    pub owner: *mut table_entry,
    pub next: *mut Symbol,
    pub entry: *mut linked_list_entry,
    pub type_0: symtype,
    pub name: *mut i8,
    pub flag: symbol_flag,
    pub alias: *mut symbol,
    pub active: i32,
    pub expand_line: i32,
    pub token_type: i32,
    pub source: *mut i8,
    pub def_line: i32,
    pub ref_line: *mut linked_list,
    pub level: i32,
    pub decl: *mut i8,
    pub storage: storage,
    pub arity: i32,
    pub recursive: i32,
    pub ord: size_t,
    pub caller: *mut linked_list,
    pub callee: *mut linked_list,
}
pub type Symbol = symbol;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_entry {
    pub sym: *mut Symbol,
}
pub type Hash_table = hash_table;
pub type Hash_data_freer = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_hasher = Option<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_tuning = hash_tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_comparator = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct collect_data {
    pub sym: *mut *mut Symbol,
    pub sel: Option<unsafe extern "C" fn(*mut Symbol) -> i32>,
    pub index: size_t,
}
pub type Hash_processor = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool,
>;
static mut symbol_table: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
static mut static_symbol_list: *mut linked_list = 0 as *const linked_list
    as *mut linked_list;
static mut auto_symbol_list: *mut linked_list = 0 as *const linked_list
    as *mut linked_list;
static mut static_func_list: *mut linked_list = 0 as *const linked_list
    as *mut linked_list;
unsafe extern "C" fn append_symbol(
    mut plist: *mut *mut linked_list,
    mut sp: *mut Symbol,
) {
    if !((*sp).entry).is_null() {
        linked_list_unlink((*(*sp).entry).list, (*sp).entry);
        (*sp).entry = 0 as *mut linked_list_entry;
    }
    if data_in_list(sp as *mut libc::c_void, *plist) == 0 {
        linked_list_append(plist, sp as *mut libc::c_void);
        (*sp).entry = (**plist).tail;
    }
}
unsafe extern "C" fn hash_symbol_hasher(
    mut data: *const libc::c_void,
    mut n_buckets: u32,
) -> size_t {
    let mut t: *const table_entry = data as *const table_entry;
    if ((*t).sym).is_null() {
        return (data as size_t).wrapping_rem(n_buckets as u64);
    }
    return hash_string((*(*t).sym).name, n_buckets as size_t);
}
unsafe extern "C" fn hash_symbol_compare(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> bool {
    let mut t1: *const table_entry = data1 as *const table_entry;
    let mut t2: *const table_entry = data2 as *const table_entry;
    return !((*t1).sym).is_null() && !((*t2).sym).is_null()
        && strcmp((*(*t1).sym).name, (*(*t2).sym).name) == 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn lookup(mut name: *const i8) -> *mut Symbol {
    let mut s: Symbol = Symbol {
        owner: 0 as *mut table_entry,
        next: 0 as *mut Symbol,
        entry: 0 as *mut linked_list_entry,
        type_0: symtype::SymUndefined,
        name: 0 as *mut i8,
        flag: symbol_flag::symbol_none,
        alias: 0 as *mut symbol,
        active: 0,
        expand_line: 0,
        token_type: 0,
        source: 0 as *mut i8,
        def_line: 0,
        ref_line: 0 as *mut linked_list,
        level: 0,
        decl: 0 as *mut i8,
        storage: storage::ExternStorage,
        arity: 0,
        recursive: 0,
        ord: 0,
        caller: 0 as *mut linked_list,
        callee: 0 as *mut linked_list,
    };
    let mut sym: *mut Symbol = 0 as *mut Symbol;
    let mut t: table_entry = table_entry {
        sym: 0 as *mut Symbol,
    };
    let mut tp: *mut table_entry = 0 as *mut table_entry;
    if symbol_table.is_null() {
        return 0 as *mut Symbol;
    }
    s.name = name as *mut i8;
    t.sym = &mut s;
    tp = hash_lookup(symbol_table, &mut t as *mut table_entry as *const libc::c_void)
        as *mut table_entry;
    if !tp.is_null() {
        sym = (*tp).sym;
        while (*sym).type_0 as u32 == symtype::SymToken as i32 as u32
            && (*sym).flag as u32 == symbol_flag::symbol_alias as i32 as u32
        {
            sym = (*sym).alias;
        }
    } else {
        sym = 0 as *mut Symbol;
    }
    return sym;
}
#[no_mangle]
pub unsafe extern "C" fn install(mut name: *mut i8, mut flags: i32) -> *mut Symbol {
    let mut sym: *mut Symbol = 0 as *mut Symbol;
    let mut tp: *mut table_entry = 0 as *mut table_entry;
    let mut ret: *mut table_entry = 0 as *mut table_entry;
    sym = xmalloc(::core::mem::size_of::<Symbol>() as u64) as *mut Symbol;
    memset(sym as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<Symbol>() as u64);
    (*sym).type_0 = symtype::SymUndefined;
    (*sym).name = name;
    tp = xmalloc(::core::mem::size_of::<table_entry>() as u64) as *mut table_entry;
    (*tp).sym = sym;
    if flags & 0x2 as i32 != 0 && !canonical_filename.is_null()
        && strcmp(filename, canonical_filename) != 0 || flags & 0x4 as i32 != 0
    {
        (*sym).flag = symbol_flag::symbol_temp;
        append_symbol(&mut static_symbol_list, sym);
    } else {
        (*sym).flag = symbol_flag::symbol_none;
    }
    if !((!symbol_table.is_null()
        || {
            symbol_table = hash_initialize(
                0 as i32 as size_t,
                0 as *const Hash_tuning,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*const libc::c_void, u32) -> size_t>,
                    Hash_hasher,
                >(
                    Some(
                        hash_symbol_hasher
                            as unsafe extern "C" fn(*const libc::c_void, u32) -> size_t,
                    ),
                ),
                Some(
                    hash_symbol_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> bool,
                ),
                None,
            );
            !symbol_table.is_null()
        })
        && {
            ret = hash_insert(symbol_table, tp as *const libc::c_void)
                as *mut table_entry;
            !ret.is_null()
        })
    {
        xalloc_die();
    }
    if ret != tp {
        if flags & 0x1 as i32 != 0 {
            free(sym as *mut libc::c_void);
            free(tp as *mut libc::c_void);
            return (*ret).sym;
        }
        if (*(*ret).sym).type_0 as u32 != symtype::SymUndefined as i32 as u32 {
            (*sym).next = (*ret).sym;
        }
        (*ret).sym = sym;
        free(tp as *mut libc::c_void);
    }
    (*sym).owner = ret;
    return sym;
}
#[no_mangle]
pub unsafe extern "C" fn ident_change_storage(
    mut sp: *mut Symbol,
    mut storage: storage,
) {
    if (*sp).storage as u32 == storage as u32 {
        return;
    }
    (*sp).storage as u32 == storage::StaticStorage as i32 as u32;
    match storage as u32 {
        2 => {
            append_symbol(&mut static_symbol_list, sp);
        }
        3 => {
            append_symbol(&mut auto_symbol_list, sp);
        }
        _ => {}
    }
    (*sp).storage = storage;
}
#[no_mangle]
pub unsafe extern "C" fn install_ident(
    mut name: *mut i8,
    mut storage: storage,
) -> *mut Symbol {
    let mut sp: *mut Symbol = 0 as *mut Symbol;
    sp = install(
        name,
        if storage as u32 != storage::AutoStorage as i32 as u32 {
            0x2 as i32
        } else {
            0 as i32
        },
    );
    (*sp).type_0 = symtype::SymIdentifier;
    (*sp).arity = -(1 as i32);
    (*sp).storage = storage::ExternStorage;
    (*sp).decl = 0 as *mut i8;
    (*sp).source = 0 as *mut i8;
    (*sp).def_line = -(1 as i32);
    (*sp).ref_line = 0 as *mut linked_list;
    (*sp).callee = 0 as *mut linked_list;
    (*sp).caller = (*sp).callee;
    (*sp).level = -(1 as i32);
    ident_change_storage(sp, storage);
    return sp;
}
unsafe extern "C" fn unlink_symbol(mut sym: *mut Symbol) {
    let mut s: *mut Symbol = 0 as *mut Symbol;
    let mut prev: *mut Symbol = 0 as *mut Symbol;
    let mut tp: *mut table_entry = (*sym).owner;
    s = (*tp).sym;
    while !s.is_null() {
        let mut next: *mut Symbol = (*s).next;
        if s == sym {
            if !prev.is_null() {
                (*prev).next = next;
            } else {
                (*tp).sym = next;
            }
            break;
        } else {
            prev = s;
            s = next;
        }
    }
    (*sym).owner = 0 as *mut table_entry;
}
unsafe extern "C" fn delete_symbol(mut sym: *mut Symbol) {
    unlink_symbol(sym);
    if ((*sym).ref_line).is_null() && !(reverse_tree != 0 && !((*sym).callee).is_null())
    {
        linked_list_destroy(&mut (*sym).ref_line);
        linked_list_destroy(&mut (*sym).caller);
        linked_list_destroy(&mut (*sym).callee);
        free(sym as *mut libc::c_void);
    }
}
unsafe extern "C" fn static_free(mut data: *mut libc::c_void) {
    let mut sym: *mut Symbol = data as *mut Symbol;
    let mut t: *mut table_entry = (*sym).owner;
    if t.is_null() {
        return;
    }
    if (*sym).flag as u32 == symbol_flag::symbol_temp as i32 as u32 {
        delete_symbol(sym);
    } else {
        unlink_symbol(sym);
        if symbol_is_function(sym) != 0 {
            linked_list_append(&mut static_func_list, sym as *mut libc::c_void);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn delete_statics() {
    if !static_symbol_list.is_null() {
        (*static_symbol_list).free_data = Some(
            static_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        linked_list_destroy(&mut static_symbol_list);
    }
}
#[no_mangle]
pub unsafe extern "C" fn delete_level_autos(
    mut data: *mut libc::c_void,
    mut call_data: *mut libc::c_void,
) -> i32 {
    let mut level: i32 = *(call_data as *mut i32);
    let mut s: *mut Symbol = data as *mut Symbol;
    if (*s).level == level {
        delete_symbol(s);
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn delete_level_statics(
    mut data: *mut libc::c_void,
    mut call_data: *mut libc::c_void,
) -> i32 {
    let mut level: i32 = *(call_data as *mut i32);
    let mut s: *mut Symbol = data as *mut Symbol;
    if (*s).level == level {
        unlink_symbol(s);
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn delete_autos(mut level: i32) {
    linked_list_iterate(
        &mut auto_symbol_list,
        Some(
            delete_level_autos
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> i32,
        ),
        &mut level as *mut i32 as *mut libc::c_void,
    );
    linked_list_iterate(
        &mut static_symbol_list,
        Some(
            delete_level_statics
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> i32,
        ),
        &mut level as *mut i32 as *mut libc::c_void,
    );
}
unsafe extern "C" fn collect_processor(
    mut data: *mut libc::c_void,
    mut proc_data: *mut libc::c_void,
) -> bool {
    let mut t: *mut table_entry = data as *mut table_entry;
    let mut cd: *mut collect_data = proc_data as *mut collect_data;
    let mut s: *mut Symbol = 0 as *mut Symbol;
    s = (*t).sym;
    while !s.is_null() {
        if ((*cd).sel).expect("non-null function pointer")(s) != 0 {
            if !((*cd).sym).is_null() {
                let ref mut fresh0 = *((*cd).sym).offset((*cd).index as isize);
                *fresh0 = s;
            }
            (*cd).index = ((*cd).index).wrapping_add(1);
            (*cd).index;
        }
        s = (*s).next;
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn collect_symbols(
    mut return_sym: *mut *mut *mut Symbol,
    mut sel: Option<unsafe extern "C" fn(*mut Symbol) -> i32>,
    mut reserved_slots: size_t,
) -> size_t {
    let mut cdata: collect_data = collect_data {
        sym: 0 as *mut *mut Symbol,
        sel: None,
        index: 0,
    };
    cdata.sym = 0 as *mut *mut Symbol;
    cdata.index = 0 as i32 as size_t;
    cdata.sel = sel;
    hash_do_for_each(
        symbol_table,
        Some(
            collect_processor
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool,
        ),
        &mut cdata as *mut collect_data as *mut libc::c_void,
    );
    cdata.sym = calloc(
        (cdata.index).wrapping_add(reserved_slots),
        ::core::mem::size_of::<*mut Symbol>() as u64,
    ) as *mut *mut Symbol;
    if (cdata.sym).is_null() {
        xalloc_die();
    }
    cdata.index = 0 as i32 as size_t;
    hash_do_for_each(
        symbol_table,
        Some(
            collect_processor
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool,
        ),
        &mut cdata as *mut collect_data as *mut libc::c_void,
    );
    *return_sym = cdata.sym;
    return cdata.index;
}
#[no_mangle]
pub unsafe extern "C" fn collect_functions(
    mut return_sym: *mut *mut *mut Symbol,
) -> size_t {
    let mut symbols: *mut *mut Symbol = 0 as *mut *mut Symbol;
    let mut num: size_t = 0;
    let mut snum: size_t = 0;
    let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
    snum = 0 as i32 as size_t;
    if !static_func_list.is_null() {
        p = if !static_func_list.is_null() {
            (*static_func_list).head
        } else {
            0 as *mut linked_list_entry
        };
        while !p.is_null() {
            snum = snum.wrapping_add(1);
            snum;
            p = (*p).next;
        }
    }
    num = collect_symbols(
        &mut symbols,
        Some(symbol_is_function as unsafe extern "C" fn(*mut Symbol) -> i32),
        snum,
    );
    if snum != 0 {
        p = if !static_func_list.is_null() {
            (*static_func_list).head
        } else {
            0 as *mut linked_list_entry
        };
        while !p.is_null() {
            let fresh1 = num;
            num = num.wrapping_add(1);
            let ref mut fresh2 = *symbols.offset(fresh1 as isize);
            *fresh2 = (*p).data as *mut Symbol;
            p = (*p).next;
        }
    }
    *return_sym = symbols;
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn delete_parms_itr(
    mut data: *mut libc::c_void,
    mut call_data: *mut libc::c_void,
) -> i32 {
    let mut level: i32 = *(call_data as *mut i32);
    let mut s: *mut Symbol = data as *mut Symbol;
    let mut t: *mut table_entry = (*s).owner;
    if t.is_null() {
        return 1 as i32;
    }
    if (*s).type_0 as u32 == symtype::SymIdentifier as i32 as u32
        && (*s).storage as u32 == storage::AutoStorage as i32 as u32
        && (*s).flag as u32 == symbol_flag::symbol_parm as i32 as u32
        && (*s).level > level
    {
        delete_symbol(s);
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn delete_parms(mut level: i32) {
    linked_list_iterate(
        &mut auto_symbol_list,
        Some(
            delete_parms_itr
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> i32,
        ),
        &mut level as *mut i32 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn move_parms(mut level: i32) {
    let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
    p = if !auto_symbol_list.is_null() {
        (*auto_symbol_list).head
    } else {
        0 as *mut linked_list_entry
    };
    while !p.is_null() {
        let mut s: *mut Symbol = (*p).data as *mut Symbol;
        if (*s).type_0 as u32 == symtype::SymIdentifier as i32 as u32
            && (*s).storage as u32 == storage::AutoStorage as i32 as u32
            && (*s).flag as u32 == symbol_flag::symbol_parm as i32 as u32
        {
            (*s).level = level;
            (*s).flag = symbol_flag::symbol_none;
        }
        p = (*p).next;
    }
}