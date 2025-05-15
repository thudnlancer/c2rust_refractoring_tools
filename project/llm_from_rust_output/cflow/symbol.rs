use std::cmp::Ordering;
use std::collections::{HashMap, LinkedList};
use std::ffi::{CStr, CString};
use std::hash::{Hash, Hasher};
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymType {
    Undefined,
    Token,
    Identifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Storage {
    Extern,
    ExplicitExtern,
    Static,
    Auto,
    Any,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolFlag {
    None,
    Temp,
    Parm,
    Alias,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub type_: SymType,
    pub name: CString,
    pub flag: SymbolFlag,
    pub alias: Option<Box<Symbol>>,
    pub active: bool,
    pub expand_line: i32,
    pub token_type: i32,
    pub source: Option<CString>,
    pub def_line: i32,
    pub ref_line: LinkedList<i32>,
    pub level: i32,
    pub decl: Option<CString>,
    pub storage: Storage,
    pub arity: i32,
    pub recursive: bool,
    pub ord: usize,
    pub caller: LinkedList<Box<Symbol>>,
    pub callee: LinkedList<Box<Symbol>>,
}

impl Symbol {
    pub fn is_function(&self) -> bool {
        self.type_ == SymType::Identifier && self.arity >= 0
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<CString, Vec<Symbol>>,
    static_symbols: LinkedList<Symbol>,
    auto_symbols: LinkedList<Symbol>,
    static_funcs: LinkedList<Symbol>,
    reverse_tree: bool,
    filename: Option<CString>,
    canonical_filename: Option<CString>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            static_symbols: LinkedList::new(),
            auto_symbols: LinkedList::new(),
            static_funcs: LinkedList::new(),
            reverse_tree: false,
            filename: None,
            canonical_filename: None,
        }
    }

    pub fn lookup(&self, name: &CStr) -> Option<&Symbol> {
        self.symbols.get(name).and_then(|syms| {
            syms.iter().find(|sym| {
                sym.type_ != SymType::Token || sym.flag != SymbolFlag::Alias
            })
        })
    }

    pub fn install(&mut self, name: CString, flags: i32) -> &mut Symbol {
        let is_temp = (flags & 0x2 != 0 && self.filename != self.canonical_filename) || (flags & 0x4 != 0);
        let sym = Symbol {
            type_: SymType::Undefined,
            name: name.clone(),
            flag: if is_temp { SymbolFlag::Temp } else { SymbolFlag::None },
            alias: None,
            active: false,
            expand_line: 0,
            token_type: 0,
            source: None,
            def_line: -1,
            ref_line: LinkedList::new(),
            level: -1,
            decl: None,
            storage: Storage::Extern,
            arity: -1,
            recursive: false,
            ord: 0,
            caller: LinkedList::new(),
            callee: LinkedList::new(),
        };

        let syms = self.symbols.entry(name).or_default();
        if flags & 0x1 != 0 && !syms.is_empty() {
            return &mut syms[0];
        }

        syms.push(sym);
        let installed = syms.last_mut().unwrap();

        if is_temp {
            self.static_symbols.push_back(installed.clone());
        }

        installed
    }

    pub fn install_ident(&mut self, name: CString, storage: Storage) -> &mut Symbol {
        let flags = if storage != Storage::Auto { 0x2 } else { 0 };
        let sym = self.install(name, flags);
        sym.type_ = SymType::Identifier;
        sym.storage = storage;
        self.change_storage(sym, storage);
        sym
    }

    pub fn change_storage(&mut self, sym: &mut Symbol, storage: Storage) {
        if sym.storage == storage {
            return;
        }

        match storage {
            Storage::Static => self.static_symbols.push_back(sym.clone()),
            Storage::Auto => self.auto_symbols.push_back(sym.clone()),
            _ => {}
        }

        sym.storage = storage;
    }

    pub fn delete_statics(&mut self) {
        for sym in self.static_symbols.iter() {
            if sym.flag == SymbolFlag::Temp {
                self.delete_symbol(sym);
            } else if sym.is_function() {
                self.static_funcs.push_back(sym.clone());
            }
        }
        self.static_symbols.clear();
    }

    fn delete_symbol(&mut self, sym: &Symbol) {
        if let Some(syms) = self.symbols.get_mut(&sym.name) {
            syms.retain(|s| s.name != sym.name);
        }
    }

    pub fn delete_autos(&mut self, level: i32) {
        self.auto_symbols.retain(|sym| sym.level != level);
        self.static_symbols.retain(|sym| sym.level != level);
    }

    pub fn delete_parms(&mut self, level: i32) {
        self.auto_symbols.retain(|sym| {
            !(sym.type_ == SymType::Identifier &&
              sym.storage == Storage::Auto &&
              sym.flag == SymbolFlag::Parm &&
              sym.level > level)
        });
    }

    pub fn move_parms(&mut self, level: i32) {
        for sym in self.auto_symbols.iter_mut() {
            if sym.type_ == SymType::Identifier &&
               sym.storage == Storage::Auto &&
               sym.flag == SymbolFlag::Parm {
                sym.level = level;
                sym.flag = SymbolFlag::None;
            }
        }
    }

    pub fn collect_symbols<F>(&self, selector: F, reserved: usize) -> Vec<&Symbol>
    where
        F: Fn(&Symbol) -> bool,
    {
        let mut result = Vec::new();
        for syms in self.symbols.values() {
            for sym in syms {
                if selector(sym) {
                    result.push(sym);
                }
            }
        }
        result.reserve(reserved);
        result
    }

    pub fn collect_functions(&self) -> Vec<&Symbol> {
        let mut funcs = self.collect_symbols(|s| s.is_function(), self.static_funcs.len());
        for sym in &self.static_funcs {
            funcs.push(sym);
        }
        funcs
    }
}