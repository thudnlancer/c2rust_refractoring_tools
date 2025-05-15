use std::cmp::Ordering;
use std::collections::LinkedList;
use std::ffi::CStr;
use std::fs::File;
use std::io::{self, Write};
use std::mem;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;

const MAX_OUTPUT_DRIVERS: usize = 32;
const LEVEL_MARK_INCREMENT: usize = 128;

struct Symbol {
    name: String,
    type_: SymbolType,
    storage: StorageClass,
    arity: i32,
    active: u32,
    ord: usize,
    recursive: bool,
    source: Option<String>,
    def_line: u32,
    decl: Option<String>,
    ref_line: LinkedList<Ref>,
    callee: LinkedList<*mut Symbol>,
    caller: LinkedList<*mut Symbol>,
}

enum SymbolType {
    Identifier,
    Token,
    Undefined,
}

enum StorageClass {
    Extern,
    Static,
    Auto,
    Register,
    Typedef,
    None,
}

struct Ref {
    source: String,
    line: u32,
}

struct OutputDriver {
    name: String,
    handler: fn(OutputCommand, &mut dyn Write, u32, Option<&[u8]>, Option<&mut dyn Any>) -> io::Result<()>,
    handler_data: Option<Box<dyn Any>>,
}

enum OutputCommand {
    Init,
    Newline,
    Begin,
    End,
    Separator,
    Text,
    Symbol,
}

struct OutputSymbol {
    direct: bool,
    level: u32,
    last: bool,
    sym: *mut Symbol,
}

static mut LEVEL_MARK: Vec<u8> = Vec::new();
static mut LEVEL_MARK_SIZE: usize = 0;
static mut OUT_LINE: u32 = 1;
static mut OUTFILE: Option<Box<dyn Write>> = None;
static mut OUTPUT_DRIVERS: [Option<OutputDriver>; MAX_OUTPUT_DRIVERS] = [None; MAX_OUTPUT_DRIVERS];
static mut DRIVER_INDEX: usize = 0;
static mut DRIVER_MAX: usize = 0;

fn set_level_mark(lev: usize, mark: u8) {
    unsafe {
        if lev >= LEVEL_MARK_SIZE {
            LEVEL_MARK_SIZE += LEVEL_MARK_INCREMENT;
            LEVEL_MARK.resize(LEVEL_MARK_SIZE, 0);
        }
        LEVEL_MARK[lev] = mark;
    }
}

fn print_level(lev: usize, last: bool) -> io::Result<()> {
    unsafe {
        let outfile = OUTFILE.as_mut().unwrap();
        if PRINT_LINE_NUMBERS {
            write!(outfile, "{:5} ", OUT_LINE)?;
        }
        if PRINT_LEVELS {
            write!(outfile, "{{{:4}}} ", lev)?;
        }
        write!(outfile, "{}", LEVEL_BEGIN)?;
        for i in 0..lev {
            write!(outfile, "{}", LEVEL_INDENT[LEVEL_MARK[i] as usize])?;
        }
        writeln!(outfile, "{}", LEVEL_END[last as usize])
    }
}

fn register_output(
    name: &str,
    handler: fn(OutputCommand, &mut dyn Write, u32, Option<&[u8]>, Option<&mut dyn Any>) -> io::Result<()>,
    handler_data: Option<Box<dyn Any>>,
) -> usize {
    unsafe {
        if DRIVER_MAX == MAX_OUTPUT_DRIVERS - 1 {
            panic!("Too many output drivers");
        }
        OUTPUT_DRIVERS[DRIVER_MAX] = Some(OutputDriver {
            name: name.to_string(),
            handler,
            handler_data,
        });
        DRIVER_MAX += 1;
        DRIVER_MAX - 1
    }
}

fn select_output_driver(name: &str) -> io::Result<()> {
    unsafe {
        for i in 0..DRIVER_MAX {
            if let Some(driver) = &OUTPUT_DRIVERS[i] {
                if driver.name == name {
                    DRIVER_INDEX = i;
                    return Ok(());
                }
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "Output driver not found"))
    }
}

fn output_init() -> io::Result<()> {
    unsafe {
        if let Some(driver) = &OUTPUT_DRIVERS[DRIVER_INDEX] {
            (driver.handler)(OutputCommand::Init, OUTFILE.as_mut().unwrap(), OUT_LINE, None, None)
        } else {
            Ok(())
        }
    }
}

fn newline() -> io::Result<()> {
    unsafe {
        if let Some(driver) = &OUTPUT_DRIVERS[DRIVER_INDEX] {
            (driver.handler)(OutputCommand::Newline, OUTFILE.as_mut().unwrap(), OUT_LINE, None, None)?;
            OUT_LINE += 1;
            Ok(())
        } else {
            Ok(())
        }
    }
}

fn begin() -> io::Result<()> {
    unsafe {
        if let Some(driver) = &OUTPUT_DRIVERS[DRIVER_INDEX] {
            (driver.handler)(OutputCommand::Begin, OUTFILE.as_mut().unwrap(), OUT_LINE, None, None)
        } else {
            Ok(())
        }
    }
}

fn end() -> io::Result<()> {
    unsafe {
        if let Some(driver) = &OUTPUT_DRIVERS[DRIVER_INDEX] {
            (driver.handler)(OutputCommand::End, OUTFILE.as_mut().unwrap(), OUT_LINE, None, None)
        } else {
            Ok(())
        }
    }
}

fn separator() -> io::Result<()> {
    unsafe {
        if let Some(driver) = &OUTPUT_DRIVERS[DRIVER_INDEX] {
            (driver.handler)(OutputCommand::Separator, OUTFILE.as_mut().unwrap(), OUT_LINE, None, None)
        } else {
            Ok(())
        }
    }
}

fn print_symbol(direct: bool, level: u32, last: bool, sym: *mut Symbol) -> io::Result<()> {
    unsafe {
        if let Some(driver) = &OUTPUT_DRIVERS[DRIVER_INDEX] {
            let output_symbol = OutputSymbol {
                direct,
                level,
                last,
                sym,
            };
            (driver.handler)(
                OutputCommand::Symbol,
                OUTFILE.as_mut().unwrap(),
                OUT_LINE,
                Some(&output_symbol as *const _ as *const u8 as &[u8]),
                None,
            )
        } else {
            Ok(())
        }
    }
}

fn compare_symbols(a: &Symbol, b: &Symbol) -> Ordering {
    a.name.cmp(&b.name)
}

fn is_var(sym: &Symbol) -> bool {
    if include_symbol(sym) {
        match sym.type_ {
            SymbolType::Identifier => matches!(sym.storage, StorageClass::Extern | StorageClass::Static),
            _ => true,
        }
    } else {
        false
    }
}

fn symbol_is_function(sym: &Symbol) -> bool {
    matches!(sym.type_, SymbolType::Identifier) && sym.arity >= 0
}

fn clear_active(sym: &mut Symbol) {
    sym.active = 0;
}

fn print_refs(name: &str, reflist: &LinkedList<Ref>) -> io::Result<()> {
    unsafe {
        let outfile = OUTFILE.as_mut().unwrap();
        for refptr in reflist {
            writeln!(outfile, "{}   {}:{}", name, refptr.source, refptr.line)?;
        }
        Ok(())
    }
}

fn print_function(symp: &Symbol) -> io::Result<()> {
    unsafe {
        let outfile = OUTFILE.as_mut().unwrap();
        if let Some(source) = &symp.source {
            writeln!(
                outfile,
                "{} * {}:{} {}",
                symp.name,
                source,
                symp.def_line,
                symp.decl.as_deref().unwrap_or("")
            )?;
        }
        print_refs(&symp.name, &symp.ref_line)
    }
}

fn print_type(symp: &Symbol) -> io::Result<()> {
    unsafe {
        let outfile = OUTFILE.as_mut().unwrap();
        if let Some(source) = &symp.source {
            writeln!(outfile, "{} t {}:{}", symp.name, source, symp.def_line)?;
        }
        Ok(())
    }
}

fn xref_output() -> io::Result<()> {
    let symbols = collect_symbols(is_var, false);
    let mut symbols: Vec<&Symbol> = symbols.iter().collect();
    symbols.sort_by(|a, b| compare_symbols(a, b));

    for symp in symbols {
        match symp.type_ {
            SymbolType::Identifier => print_function(symp)?,
            SymbolType::Token => print_type(symp)?,
            SymbolType::Undefined => continue,
        }
    }
    Ok(())
}

fn set_active(sym: &mut Symbol) {
    unsafe {
        sym.active = OUT_LINE;
    }
}

fn is_printable(p: &LinkedList<*mut Symbol>) -> bool {
    !p.is_empty() && include_symbol(unsafe { &*p.front().unwrap() })
}

fn is_last(p: &LinkedList<*mut Symbol>) -> bool {
    let mut iter = p.iter().skip(1);
    !iter.any(|sym| include_symbol(unsafe { &**sym }))
}

fn direct_tree(lev: usize, last: bool, sym: &mut Symbol) -> io::Result<()> {
    if matches!(sym.type_, SymbolType::Undefined)
        || (MAX_DEPTH != 0 && lev >= MAX_DEPTH)
        || !include_symbol(sym)
    {
        return Ok(());
    }

    print_symbol(true, lev as u32, last, sym)?;
    newline()?;

    if sym.active != 0 {
        return Ok(());
    }

    set_active(sym);
    for (i, callee) in sym.callee.iter().enumerate() {
        set_level_mark(lev + 1, !is_last(&sym.callee));
        direct_tree(lev + 1, i == sym.callee.len() - 1, unsafe { &mut **callee })?;
    }
    clear_active(sym);
    Ok(())
}

fn inverted_tree(lev: usize, last: bool, sym: &mut Symbol) -> io::Result<()> {
    if matches!(sym.type_, SymbolType::Undefined)
        || (MAX_DEPTH != 0 && lev >= MAX_DEPTH)
        || !include_symbol(sym)
    {
        return Ok(());
    }

    print_symbol(false, lev as u32, last, sym)?;
    newline()?;

    if sym.active != 0 {
        return Ok(());
    }

    set_active(sym);
    for (i, caller) in sym.caller.iter().enumerate() {
        set_level_mark(lev + 1, !is_last(&sym.caller));
        inverted_tree(lev + 1, i == sym.caller.len() - 1, unsafe { &mut **caller })?;
    }
    clear_active(sym);
    Ok(())
}

fn tree_output() -> io::Result<()> {
    let mut symbols = collect_functions();
    for (i, sym) in symbols.iter_mut().enumerate() {
        sym.ord = i;
    }

    let num = symbols.len();
    let mut depmap = vec![vec![false; num]; num];

    for (i, sym) in symbols.iter().enumerate() {
        for callee in &sym.callee {
            if symbol_is_function(unsafe { &**callee }) {
                depmap[i][unsafe { (**callee).ord }] = true;
            }
        }
    }

    // TODO: Implement transitive closure for depmap

    for i in 0..num {
        if depmap[i][i] {
            symbols[i].recursive = true;
        }
    }

    let mut all_symbols = collect_symbols(is_var, false);
    all_symbols.sort_by(compare_symbols);

    begin()?;

    if REVERSE_TREE {
        for sym in &mut all_symbols {
            inverted_tree(0, false, sym)?;
            separator()?;
        }
    } else {
        if let Some(main_sym) = lookup(START_NAME) {
            direct_tree(0, false, main_sym)?;
            separator()?;
        } else {
            for sym in &mut all_symbols {
                if sym.callee.is_empty() {
                    continue;
                }
                direct_tree(0, false, sym)?;
                separator()?;
            }
        }
    }

    end()?;
    Ok(())
}

fn output() -> io::Result<()> {
    unsafe {
        if OUTNAME == "-" {
            OUTFILE = Some(Box::new(io::stdout()));
        } else {
            OUTFILE = Some(Box::new(File::create(OUTNAME)?));
        }

        set_level_mark(0, 0);
        if PRINT_OPTION & PRINT_XREF != 0 {
            xref_output()?;
        }
        if PRINT_OPTION & PRINT_TREE != 0 {
            tree_output()?;
        }

        if let Some(mut outfile) = OUTFILE.take() {
            outfile.flush()?;
        }
        Ok(())
    }
}