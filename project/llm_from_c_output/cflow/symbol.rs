use std::collections::{HashMap, LinkedList};
use std::hash::{Hash, Hasher};
use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymbolType {
    Undefined,
    Identifier,
    Token,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymbolFlag {
    None,
    Temp,
    Alias,
    Parm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Storage {
    Auto,
    Static,
    Extern,
}

#[derive(Debug)]
struct Symbol {
    name: String,
    type_: SymbolType,
    flag: SymbolFlag,
    storage: Storage,
    arity: i32,
    decl: Option<Box<Symbol>>,
    source: Option<String>,
    def_line: i32,
    ref_line: Option<LinkedList<Symbol>>,
    caller: Option<LinkedList<Symbol>>,
    callee: Option<LinkedList<Symbol>>,
    level: i32,
    next: Option<Box<Symbol>>,
    owner: Option<Box<TableEntry>>,
}

impl Symbol {
    fn new(name: String) -> Self {
        Symbol {
            name,
            type_: SymbolType::Undefined,
            flag: SymbolFlag::None,
            storage: Storage::Extern,
            arity: -1,
            decl: None,
            source: None,
            def_line: -1,
            ref_line: None,
            caller: None,
            callee: None,
            level: -1,
            next: None,
            owner: None,
        }
    }
}

#[derive(Debug)]
struct TableEntry {
    sym: Option<Symbol>,
}

impl TableEntry {
    fn new(sym: Symbol) -> Self {
        TableEntry { sym: Some(sym) }
    }
}

impl PartialEq for TableEntry {
    fn eq(&self, other: &Self) -> bool {
        match (&self.sym, &other.sym) {
            (Some(s1), Some(s2)) => s1.name == s2.name,
            _ => false,
        }
    }
}

impl Eq for TableEntry {}

impl Hash for TableEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(sym) = &self.sym {
            sym.name.hash(state);
        } else {
            ptr::hash(self, state);
        }
    }
}

struct SymbolTable {
    table: HashMap<TableEntry, Symbol>,
    static_symbols: LinkedList<Symbol>,
    auto_symbols: LinkedList<Symbol>,
    static_funcs: LinkedList<Symbol>,
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable {
            table: HashMap::new(),
            static_symbols: LinkedList::new(),
            auto_symbols: LinkedList::new(),
            static_funcs: LinkedList::new(),
        }
    }

    fn lookup(&self, name: &str) -> Option<&Symbol> {
        let key = TableEntry {
            sym: Some(Symbol::new(name.to_string())),
        };
        self.table.get(&key).and_then(|sym| {
            let mut current = sym;
            while current.type_ == SymbolType::Token && current.flag == SymbolFlag::Alias {
                current = current.alias.as_ref()?;
            }
            Some(current)
        })
    }

    fn install(&mut self, name: String, flags: u32) -> Result<&Symbol, &'static str> {
        let mut sym = Symbol::new(name.clone());
        let key = TableEntry::new(sym.clone());

        if (flags & INSTALL_CHECK_LOCAL != 0 && canonical_filename != filename)
            || (flags & INSTALL_UNIT_LOCAL != 0)
        {
            sym.flag = SymbolFlag::Temp;
            self.append_symbol(&mut self.static_symbols, sym.clone());
        } else {
            sym.flag = SymbolFlag::None;
        }

        let entry = self.table.entry(key).or_insert(sym);
        if entry.name != name {
            if flags & INSTALL_OVERWRITE != 0 {
                return Ok(entry);
            }
            if entry.type_ != SymbolType::Undefined {
                let mut new_sym = Symbol::new(name);
                new_sym.next = Some(Box::new(entry.clone()));
                *entry = new_sym;
            }
        }
        entry.owner = Some(Box::new(TableEntry::new(entry.clone())));
        Ok(entry)
    }

    fn append_symbol(&mut self, list: &mut LinkedList<Symbol>, sym: Symbol) {
        if let Some(entry) = sym.entry {
            list.remove(entry);
        }
        if !list.contains(&sym) {
            list.push_back(sym);
        }
    }

    fn ident_change_storage(&mut self, sym: &mut Symbol, storage: Storage) {
        if sym.storage == storage {
            return;
        }
        if sym.storage == Storage::Static {
            // FIXME
        }

        match storage {
            Storage::Static => self.append_symbol(&mut self.static_symbols, sym.clone()),
            Storage::Auto => self.append_symbol(&mut self.auto_symbols, sym.clone()),
            _ => (),
        }
        sym.storage = storage;
    }

    fn install_ident(&mut self, name: String, storage: Storage) -> Result<&Symbol, &'static str> {
        let flags = if storage != Storage::Auto {
            INSTALL_CHECK_LOCAL
        } else {
            INSTALL_DEFAULT
        };
        let sym = self.install(name, flags)?;
        sym.type_ = SymbolType::Identifier;
        sym.arity = -1;
        sym.storage = Storage::Extern;
        sym.decl = None;
        sym.source = None;
        sym.def_line = -1;
        sym.ref_line = None;
        sym.caller = None;
        sym.callee = None;
        sym.level = -1;
        self.ident_change_storage(sym, storage);
        Ok(sym)
    }

    fn unlink_symbol(&mut self, sym: &mut Symbol) {
        if let Some(owner) = &mut sym.owner {
            let mut prev = None;
            let mut current = &mut owner.sym;
            while let Some(s) = current {
                if *s == *sym {
                    if let Some(prev) = prev {
                        prev.next = s.next.take();
                    } else {
                        owner.sym = s.next.take();
                    }
                    break;
                }
                prev = Some(s);
                current = &mut s.next;
            }
        }
        sym.owner = None;
    }

    fn delete_symbol(&mut self, sym: &mut Symbol) {
        self.unlink_symbol(sym);
        if sym.ref_line.is_none() && !(reverse_tree && sym.callee.is_some()) {
            sym.ref_line = None;
            sym.caller = None;
            sym.callee = None;
        }
    }

    fn static_free(&mut self, sym: &mut Symbol) {
        if let Some(owner) = &sym.owner {
            if sym.flag == SymbolFlag::Temp {
                self.delete_symbol(sym);
            } else {
                self.unlink_symbol(sym);
                if self.symbol_is_function(sym) {
                    self.static_funcs.push_back(sym.clone());
                }
            }
        }
    }

    fn delete_statics(&mut self) {
        for sym in &mut self.static_symbols {
            self.static_free(sym);
        }
        self.static_symbols.clear();
    }

    fn delete_level_autos(&mut self, level: i32) {
        self.auto_symbols.retain(|sym| sym.level != level);
    }

    fn delete_level_statics(&mut self, level: i32) {
        for sym in &mut self.static_symbols {
            if sym.level == level {
                self.unlink_symbol(sym);
            }
        }
    }

    fn delete_autos(&mut self, level: i32) {
        self.delete_level_autos(level);
        self.delete_level_statics(level);
    }

    fn collect_symbols(
        &self,
        sel: fn(&Symbol) -> bool,
        reserved_slots: usize,
    ) -> (Vec<Symbol>, usize) {
        let mut symbols = Vec::new();
        let mut count = 0;

        for (_, sym) in &self.table {
            if sel(sym) {
                count += 1;
            }
        }

        symbols.reserve(count + reserved_slots);
        for (_, sym) in &self.table {
            if sel(sym) {
                symbols.push(sym.clone());
            }
        }

        (symbols, count)
    }

    fn collect_functions(&self) -> (Vec<Symbol>, usize) {
        let static_func_count = self.static_funcs.len();
        let (mut symbols, global_count) = self.collect_symbols(Self::symbol_is_function, static_func_count);

        for sym in &self.static_funcs {
            symbols.push(sym.clone());
        }

        (symbols, global_count + static_func_count)
    }

    fn symbol_is_function(sym: &Symbol) -> bool {
        // Implementation depends on actual criteria for function symbols
        false
    }

    fn delete_parms(&mut self, level: i32) {
        self.auto_symbols.retain(|sym| {
            !(sym.type_ == SymbolType::Identifier
                && sym.storage == Storage::Auto
                && sym.flag == SymbolFlag::Parm
                && sym.level > level)
        });
    }

    fn move_parms(&mut self, level: i32) {
        for sym in &mut self.auto_symbols {
            if sym.type_ == SymbolType::Identifier
                && sym.storage == Storage::Auto
                && sym.flag == SymbolFlag::Parm
            {
                sym.level = level;
                sym.flag = SymbolFlag::None;
            }
        }
    }
}

const INSTALL_CHECK_LOCAL: u32 = 1;
const INSTALL_UNIT_LOCAL: u32 = 2;
const INSTALL_OVERWRITE: u32 = 4;
const INSTALL_DEFAULT: u32 = 0;

static mut symbol_table: Option<SymbolTable> = None;
static mut canonical_filename: Option<String> = None;
static mut filename: Option<String> = None;
static mut reverse_tree: bool = false;

fn main() {
    let mut table = SymbolTable::new();
    unsafe {
        symbol_table = Some(table);
    }
}