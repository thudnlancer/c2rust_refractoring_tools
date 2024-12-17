#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type hash_table;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn symbol_is_function(sym: *mut Symbol) -> libc::c_int;
    fn linked_list_destroy(plist: *mut *mut linked_list);
    fn linked_list_iterate(
        plist: *mut *mut linked_list,
        itr: Option::<
            unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
        >,
        data: *mut libc::c_void,
    );
    fn xalloc_die() -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    static mut reverse_tree: libc::c_int;
    fn linked_list_unlink(list: *mut linked_list, ent: *mut linked_list_entry);
    fn data_in_list(data: *mut libc::c_void, list: *mut linked_list) -> libc::c_int;
    fn linked_list_append(plist: *mut *mut linked_list, data: *mut libc::c_void);
    static mut filename: *mut libc::c_char;
    static mut canonical_filename: *mut libc::c_char;
    fn hash_lookup(_: *const Hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_do_for_each(
        _: *const Hash_table,
        _: Hash_processor,
        _: *mut libc::c_void,
    ) -> size_t;
    fn hash_string(_: *const libc::c_char, _: size_t) -> size_t;
    fn hash_initialize(
        _: size_t,
        _: *const Hash_tuning,
        _: Hash_hasher,
        _: Hash_comparator,
        _: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(_: *mut Hash_table, _: *const libc::c_void) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
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
pub type linked_list_free_data_fp = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum symtype {
    SymIdentifier = 2,
    SymToken = 1,
    SymUndefined = 0,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum storage {
    AnyStorage = 4,
    AutoStorage = 3,
    StaticStorage = 2,
    ExplicitExternStorage = 1,
    ExternStorage = 0,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum symbol_flag {
    symbol_alias = 3,
    symbol_parm = 2,
    symbol_temp = 1,
    symbol_none = 0,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol {
    pub owner: *mut table_entry,
    pub next: *mut Symbol,
    pub entry: *mut linked_list_entry,
    pub type_0: symtype,
    pub name: *mut libc::c_char,
    pub flag: symbol_flag,
    pub alias: *mut symbol,
    pub active: libc::c_int,
    pub expand_line: libc::c_int,
    pub token_type: libc::c_int,
    pub source: *mut libc::c_char,
    pub def_line: libc::c_int,
    pub ref_line: *mut linked_list,
    pub level: libc::c_int,
    pub decl: *mut libc::c_char,
    pub storage: storage,
    pub arity: libc::c_int,
    pub recursive: libc::c_int,
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
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_hasher = Option::<
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
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct collect_data {
    pub sym: *mut *mut Symbol,
    pub sel: Option::<unsafe extern "C" fn(*mut Symbol) -> libc::c_int>,
    pub index: size_t,
}
pub type Hash_processor = Option::<
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
    mut n_buckets: libc::c_uint,
) -> size_t {
    let mut t: *const table_entry = data as *const table_entry;
    if ((*t).sym).is_null() {
        return (data as size_t).wrapping_rem(n_buckets as libc::c_ulong);
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
        && strcmp((*(*t1).sym).name, (*(*t2).sym).name) == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lookup(mut name: *const libc::c_char) -> *mut Symbol {
    let mut s: Symbol = Symbol {
        owner: 0 as *mut table_entry,
        next: 0 as *mut Symbol,
        entry: 0 as *mut linked_list_entry,
        type_0: SymUndefined,
        name: 0 as *mut libc::c_char,
        flag: symbol_none,
        alias: 0 as *mut symbol,
        active: 0,
        expand_line: 0,
        token_type: 0,
        source: 0 as *mut libc::c_char,
        def_line: 0,
        ref_line: 0 as *mut linked_list,
        level: 0,
        decl: 0 as *mut libc::c_char,
        storage: ExternStorage,
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
    s.name = name as *mut libc::c_char;
    t.sym = &mut s;
    tp = hash_lookup(symbol_table, &mut t as *mut table_entry as *const libc::c_void)
        as *mut table_entry;
    if !tp.is_null() {
        sym = (*tp).sym;
        while (*sym).type_0 as libc::c_uint == SymToken as libc::c_int as libc::c_uint
            && (*sym).flag as libc::c_uint == symbol_alias as libc::c_int as libc::c_uint
        {
            sym = (*sym).alias;
        }
    } else {
        sym = 0 as *mut Symbol;
    }
    return sym;
}
#[no_mangle]
pub unsafe extern "C" fn install(
    mut name: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut Symbol {
    let mut sym: *mut Symbol = 0 as *mut Symbol;
    let mut tp: *mut table_entry = 0 as *mut table_entry;
    let mut ret: *mut table_entry = 0 as *mut table_entry;
    sym = xmalloc(::core::mem::size_of::<Symbol>() as libc::c_ulong) as *mut Symbol;
    memset(
        sym as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Symbol>() as libc::c_ulong,
    );
    (*sym).type_0 = SymUndefined;
    (*sym).name = name;
    tp = xmalloc(::core::mem::size_of::<table_entry>() as libc::c_ulong)
        as *mut table_entry;
    (*tp).sym = sym;
    if flags & 0x2 as libc::c_int != 0 && !canonical_filename.is_null()
        && strcmp(filename, canonical_filename) != 0 || flags & 0x4 as libc::c_int != 0
    {
        (*sym).flag = symbol_temp;
        append_symbol(&mut static_symbol_list, sym);
    } else {
        (*sym).flag = symbol_none;
    }
    if !((!symbol_table.is_null()
        || {
            symbol_table = hash_initialize(
                0 as libc::c_int as size_t,
                0 as *const Hash_tuning,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(*const libc::c_void, libc::c_uint) -> size_t,
                    >,
                    Hash_hasher,
                >(
                    Some(
                        hash_symbol_hasher
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                libc::c_uint,
                            ) -> size_t,
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
        if flags & 0x1 as libc::c_int != 0 {
            free(sym as *mut libc::c_void);
            free(tp as *mut libc::c_void);
            return (*ret).sym;
        }
        if (*(*ret).sym).type_0 as libc::c_uint
            != SymUndefined as libc::c_int as libc::c_uint
        {
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
    if (*sp).storage as libc::c_uint == storage as libc::c_uint {
        return;
    }
    (*sp).storage as libc::c_uint == StaticStorage as libc::c_int as libc::c_uint;
    match storage as libc::c_uint {
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
    mut name: *mut libc::c_char,
    mut storage: storage,
) -> *mut Symbol {
    let mut sp: *mut Symbol = 0 as *mut Symbol;
    sp = install(
        name,
        if storage as libc::c_uint != AutoStorage as libc::c_int as libc::c_uint {
            0x2 as libc::c_int
        } else {
            0 as libc::c_int
        },
    );
    (*sp).type_0 = SymIdentifier;
    (*sp).arity = -(1 as libc::c_int);
    (*sp).storage = ExternStorage;
    (*sp).decl = 0 as *mut libc::c_char;
    (*sp).source = 0 as *mut libc::c_char;
    (*sp).def_line = -(1 as libc::c_int);
    (*sp).ref_line = 0 as *mut linked_list;
    (*sp).callee = 0 as *mut linked_list;
    (*sp).caller = (*sp).callee;
    (*sp).level = -(1 as libc::c_int);
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
    if (*sym).flag as libc::c_uint == symbol_temp as libc::c_int as libc::c_uint {
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
        (*static_symbol_list)
            .free_data = Some(
            static_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        linked_list_destroy(&mut static_symbol_list);
    }
}
#[no_mangle]
pub unsafe extern "C" fn delete_level_autos(
    mut data: *mut libc::c_void,
    mut call_data: *mut libc::c_void,
) -> libc::c_int {
    let mut level: libc::c_int = *(call_data as *mut libc::c_int);
    let mut s: *mut Symbol = data as *mut Symbol;
    if (*s).level == level {
        delete_symbol(s);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn delete_level_statics(
    mut data: *mut libc::c_void,
    mut call_data: *mut libc::c_void,
) -> libc::c_int {
    let mut level: libc::c_int = *(call_data as *mut libc::c_int);
    let mut s: *mut Symbol = data as *mut Symbol;
    if (*s).level == level {
        unlink_symbol(s);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn delete_autos(mut level: libc::c_int) {
    linked_list_iterate(
        &mut auto_symbol_list,
        Some(
            delete_level_autos
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut level as *mut libc::c_int as *mut libc::c_void,
    );
    linked_list_iterate(
        &mut static_symbol_list,
        Some(
            delete_level_statics
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut level as *mut libc::c_int as *mut libc::c_void,
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
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn collect_symbols(
    mut return_sym: *mut *mut *mut Symbol,
    mut sel: Option::<unsafe extern "C" fn(*mut Symbol) -> libc::c_int>,
    mut reserved_slots: size_t,
) -> size_t {
    let mut cdata: collect_data = collect_data {
        sym: 0 as *mut *mut Symbol,
        sel: None,
        index: 0,
    };
    cdata.sym = 0 as *mut *mut Symbol;
    cdata.index = 0 as libc::c_int as size_t;
    cdata.sel = sel;
    hash_do_for_each(
        symbol_table,
        Some(
            collect_processor
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool,
        ),
        &mut cdata as *mut collect_data as *mut libc::c_void,
    );
    cdata
        .sym = calloc(
        (cdata.index).wrapping_add(reserved_slots),
        ::core::mem::size_of::<*mut Symbol>() as libc::c_ulong,
    ) as *mut *mut Symbol;
    if (cdata.sym).is_null() {
        xalloc_die();
    }
    cdata.index = 0 as libc::c_int as size_t;
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
    snum = 0 as libc::c_int as size_t;
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
        Some(symbol_is_function as unsafe extern "C" fn(*mut Symbol) -> libc::c_int),
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
) -> libc::c_int {
    let mut level: libc::c_int = *(call_data as *mut libc::c_int);
    let mut s: *mut Symbol = data as *mut Symbol;
    let mut t: *mut table_entry = (*s).owner;
    if t.is_null() {
        return 1 as libc::c_int;
    }
    if (*s).type_0 as libc::c_uint == SymIdentifier as libc::c_int as libc::c_uint
        && (*s).storage as libc::c_uint == AutoStorage as libc::c_int as libc::c_uint
        && (*s).flag as libc::c_uint == symbol_parm as libc::c_int as libc::c_uint
        && (*s).level > level
    {
        delete_symbol(s);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn delete_parms(mut level: libc::c_int) {
    linked_list_iterate(
        &mut auto_symbol_list,
        Some(
            delete_parms_itr
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut level as *mut libc::c_int as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn move_parms(mut level: libc::c_int) {
    let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
    p = if !auto_symbol_list.is_null() {
        (*auto_symbol_list).head
    } else {
        0 as *mut linked_list_entry
    };
    while !p.is_null() {
        let mut s: *mut Symbol = (*p).data as *mut Symbol;
        if (*s).type_0 as libc::c_uint == SymIdentifier as libc::c_int as libc::c_uint
            && (*s).storage as libc::c_uint == AutoStorage as libc::c_int as libc::c_uint
            && (*s).flag as libc::c_uint == symbol_parm as libc::c_int as libc::c_uint
        {
            (*s).level = level;
            (*s).flag = symbol_none;
        }
        p = (*p).next;
    }
}
