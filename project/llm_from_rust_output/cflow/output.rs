use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, Write};
use std::mem;
use std::os::raw::{c_char, c_int, c_uchar, c_ulong, c_void};
use std::ptr;

type SizeT = c_ulong;

#[derive(Debug, Clone)]
struct Symbol {
    name: String,
    type_: SymType,
    storage: Storage,
    arity: c_int,
    recursive: c_int,
    active: c_int,
    def_line: c_int,
    source: Option<String>,
    decl: Option<String>,
    callee: Vec<Symbol>,
    caller: Vec<Symbol>,
    ref_line: Vec<Ref>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymType {
    Undefined,
    Token,
    Identifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Storage {
    Extern,
    ExplicitExtern,
    Static,
    Auto,
    Any,
}

#[derive(Debug, Clone)]
struct Ref {
    line: c_int,
    source: String,
}

struct OutputDriver {
    name: String,
    handler: Box<dyn Fn(OutputCommand, &mut File, c_int, *mut c_void) -> io::Result<c_int>>,
    handler_data: *mut c_void,
}

enum OutputCommand {
    Init,
    Begin,
    End,
    Newline,
    Separator,
    Symbol,
    Text,
}

static mut LEVEL_MARK: Vec<c_uchar> = Vec::new();
static mut LEVEL_MARK_SIZE: c_int = 0;
static mut LEVEL_MARK_INCR: c_int = 128;
static mut OUT_LINE: c_int = 1;
static mut OUTFILE: Option<File> = None;

static mut DRIVER_INDEX: c_int = 0;
static mut DRIVER_MAX: c_int = 0;
static mut OUTPUT_DRIVERS: Vec<OutputDriver> = Vec::new();

fn set_level_mark(lev: c_int, mark: c_int) {
    unsafe {
        if lev >= LEVEL_MARK_SIZE {
            LEVEL_MARK_SIZE += LEVEL_MARK_INCR;
            LEVEL_MARK.resize(LEVEL_MARK_SIZE as usize, 0);
        }
        LEVEL_MARK[lev as usize] = mark as c_uchar;
    }
}

fn print_level(lev: c_int, last: c_int) {
    unsafe {
        if let Some(file) = &mut OUTFILE {
            if PRINT_LINE_NUMBERS != 0 {
                write!(file, "{:5} ", OUT_LINE).unwrap();
            }
            if PRINT_LEVELS != 0 {
                write!(file, "{{{:4}}} ", lev).unwrap();
            }
            write!(file, "{}", LEVEL_BEGIN).unwrap();
            for i in 0..lev {
                write!(
                    file,
                    "{}",
                    LEVEL_INDENT[LEVEL_MARK[i as usize] as usize]
                ).unwrap();
            }
            write!(file, "{}", LEVEL_END[last as usize]).unwrap();
        }
    }
}

fn register_output(
    name: &str,
    handler: Box<dyn Fn(OutputCommand, &mut File, c_int, *mut c_void) -> io::Result<c_int>>,
    handler_data: *mut c_void,
) -> c_int {
    unsafe {
        if DRIVER_MAX == OUTPUT_DRIVERS.capacity() as c_int - 1 {
            panic!("Too many output drivers");
        }
        OUTPUT_DRIVERS.push(OutputDriver {
            name: name.to_string(),
            handler,
            handler_data,
        });
        let idx = DRIVER_MAX;
        DRIVER_MAX += 1;
        idx
    }
}

fn select_output_driver(name: &str) -> c_int {
    unsafe {
        for (i, driver) in OUTPUT_DRIVERS.iter().enumerate() {
            if driver.name == name {
                DRIVER_INDEX = i as c_int;
                return 0;
            }
        }
        -1
    }
}

fn output_init() {
    unsafe {
        if let Some(driver) = OUTPUT_DRIVERS.get(DRIVER_INDEX as usize) {
            (driver.handler)(OutputCommand::Init, OUTFILE.as_mut().unwrap(), 0, driver.handler_data).unwrap();
        }
    }
}

fn newline() {
    unsafe {
        if let Some(driver) = OUTPUT_DRIVERS.get(DRIVER_INDEX as usize) {
            (driver.handler)(OutputCommand::Newline, OUTFILE.as_mut().unwrap(), OUT_LINE, driver.handler_data).unwrap();
            OUT_LINE += 1;
        }
    }
}

fn begin() {
    unsafe {
        if let Some(driver) = OUTPUT_DRIVERS.get(DRIVER_INDEX as usize) {
            (driver.handler)(OutputCommand::Begin, OUTFILE.as_mut().unwrap(), OUT_LINE, driver.handler_data).unwrap();
        }
    }
}

fn end() {
    unsafe {
        if let Some(driver) = OUTPUT_DRIVERS.get(DRIVER_INDEX as usize) {
            (driver.handler)(OutputCommand::End, OUTFILE.as_mut().unwrap(), OUT_LINE, driver.handler_data).unwrap();
        }
    }
}

fn separator() {
    unsafe {
        if let Some(driver) = OUTPUT_DRIVERS.get(DRIVER_INDEX as usize) {
            (driver.handler)(OutputCommand::Separator, OUTFILE.as_mut().unwrap(), OUT_LINE, driver.handler_data).unwrap();
        }
    }
}

fn print_symbol(direct: c_int, level: c_int, last: c_int, sym: &Symbol) -> c_int {
    unsafe {
        if let Some(driver) = OUTPUT_DRIVERS.get(DRIVER_INDEX as usize) {
            let mut output = OutputSymbol {
                direct,
                level,
                last,
                sym,
            };
            (driver.handler)(
                OutputCommand::Symbol,
                OUTFILE.as_mut().unwrap(),
                OUT_LINE,
                &mut output as *mut _ as *mut c_void,
            ).unwrap()
        } else {
            0
        }
    }
}

fn compare_symbols(a: &Symbol, b: &Symbol) -> Ordering {
    a.name.cmp(&b.name)
}

fn is_var(sym: &Symbol) -> bool {
    if include_symbol(sym) {
        match sym.type_ {
            SymType::Identifier => {
                sym.storage == Storage::Extern || sym.storage == Storage::Static
            }
            _ => true,
        }
    } else {
        false
    }
}

fn symbol_is_function(sym: &Symbol) -> bool {
    sym.type_ == SymType::Identifier && sym.arity >= 0
}

fn clear_active(sym: &mut Symbol) {
    sym.active = 0;
}

fn print_refs(name: &str, refs: &[Ref]) {
    if let Some(file) = unsafe { &mut OUTFILE } {
        for r in refs {
            writeln!(file, "{}   {}:{}", name, r.source, r.line).unwrap();
        }
    }
}

fn print_function(sym: &Symbol) {
    if let Some(file) = unsafe { &mut OUTFILE } {
        if let Some(source) = &sym.source {
            writeln!(
                file,
                "{} * {}:{} {}",
                sym.name,
                source,
                sym.def_line,
                sym.decl.as_deref().unwrap_or("")
            ).unwrap();
        }
        print_refs(&sym.name, &sym.ref_line);
    }
}

fn print_type(sym: &Symbol) {
    if let Some(file) = unsafe { &mut OUTFILE } {
        if let Some(source) = &sym.source {
            writeln!(file, "{} t {}:{}", sym.name, source, sym.def_line).unwrap();
        }
    }
}

fn xref_output(symbols: &mut [Symbol]) {
    symbols.sort_by(compare_symbols);
    for sym in symbols {
        match sym.type_ {
            SymType::Identifier => print_function(sym),
            SymType::Token => print_type(sym),
            _ => {}
        }
    }
}

fn set_active(sym: &mut Symbol) {
    sym.active = unsafe { OUT_LINE };
}

fn is_printable(sym: &Symbol) -> bool {
    include_symbol(sym)
}

fn is_last(sym: &Symbol, list: &[Symbol]) -> bool {
    list.iter().skip_while(|s| s != &sym).all(|s| !is_printable(s))
}

fn direct_tree(lev: c_int, last: c_int, sym: &mut Symbol) {
    if sym.type_ == SymType::Undefined
        || (MAX_DEPTH != 0 && lev >= MAX_DEPTH)
        || !include_symbol(sym)
    {
        return;
    }

    if print_symbol(1, lev, last, sym) != 0 || sym.active != 0 {
        return;
    }

    set_active(sym);
    for (i, callee) in sym.callee.iter_mut().enumerate() {
        let is_last = is_last(callee, &sym.callee);
        set_level_mark(lev + 1, if is_last { 0 } else { 1 });
        direct_tree(lev + 1, if is_last { 1 } else { 0 }, callee);
    }
    clear_active(sym);
}

fn inverted_tree(lev: c_int, last: c_int, sym: &mut Symbol) {
    if sym.type_ == SymType::Undefined
        || (MAX_DEPTH != 0 && lev >= MAX_DEPTH)
        || !include_symbol(sym)
    {
        return;
    }

    if print_symbol(0, lev, last, sym) != 0 || sym.active != 0 {
        return;
    }

    set_active(sym);
    for (i, caller) in sym.caller.iter_mut().enumerate() {
        let is_last = is_last(caller, &sym.caller);
        set_level_mark(lev + 1, if is_last { 0 } else { 1 });
        inverted_tree(lev + 1, if is_last { 1 } else { 0 }, caller);
    }
    clear_active(sym);
}

fn tree_output(symbols: &mut [Symbol]) {
    let mut depmap = HashMap::new();
    
    for (i, sym) in symbols.iter_mut().enumerate() {
        sym.ord = i;
        for callee in &sym.callee {
            if symbol_is_function(callee) {
                depmap.entry(i).or_insert(vec![]).push(callee.ord);
            }
        }
    }

    // Transitive closure
    for k in 0..symbols.len() {
        for i in 0..symbols.len() {
            if depmap.get(&i).map_or(false, |v| v.contains(&k)) {
                if let Some(k_list) = depmap.get(&k).cloned() {
                    depmap.entry(i).or_default().extend(k_list);
                }
            }
        }
    }

    for (i, sym) in symbols.iter_mut().enumerate() {
        if depmap.get(&i).map_or(false, |v| v.contains(&i)) {
            sym.recursive = 1;
        }
    }

    symbols.sort_by(compare_symbols);
    
    begin();
    if REVERSE_TREE != 0 {
        for sym in symbols {
            inverted_tree(0, 0, sym);
            separator();
        }
    } else {
        if let Some(main_sym) = symbols.iter_mut().find(|s| s.name == START_NAME) {
            direct_tree(0, 0, main_sym);
            separator();
        } else {
            for sym in symbols {
                if !sym.callee.is_empty() {
                    direct_tree(0, 0, sym);
                    separator();
                }
            }
        }
    }
    end();
}

fn output() -> io::Result<()> {
    unsafe {
        if OUTNAME == "-" {
            OUTFILE = Some(io::stdout());
        } else {
            OUTFILE = Some(File::create(OUTNAME)?);
        }

        set_level_mark(0, 0);
        if PRINT_OPTION & 0x1 != 0 {
            xref_output(&mut collect_symbols(is_var));
        }
        if PRINT_OPTION & 0x2 != 0 {
            tree_output(&mut collect_functions());
        }
        OUTFILE.take().map(|f| f.flush()).transpose()?;
        Ok(())
    }
}