use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_void, c_long, c_ulong, c_double};
use std::ptr;
use std::mem;
use std::cmp::Ordering;
use std::collections::HashMap;

type size_t = c_ulong;
type awk_bool = c_uint;
type awk_valtype_t = c_uint;
type NODETYPE = c_uint;
type OPCODE = c_uint;
type SYMBOL_TYPE = c_uint;

const awk_true: awk_bool = 1;
const awk_false: awk_bool = 0;
const VARIABLE: SYMBOL_TYPE = 2;
const FUNCTION: SYMBOL_TYPE = 1;

#[derive(Debug, Clone)]
struct NODE {
    type_: NODETYPE,
    name: String,
    valref: c_long,
    // Other fields omitted for brevity
}

struct SymbolTable {
    global_table: HashMap<String, NODE>,
    func_table: HashMap<String, NODE>,
    symbol_table: HashMap<String, NODE>,
    param_table: HashMap<String, NODE>,
}

impl SymbolTable {
    fn new() -> Self {
        Self {
            global_table: HashMap::new(),
            func_table: HashMap::new(),
            symbol_table: HashMap::new(),
            param_table: HashMap::new(),
        }
    }

    fn install_symbol(&mut self, name: &str, type_: NODETYPE) -> Option<&NODE> {
        let node = NODE {
            type_,
            name: name.to_string(),
            valref: 1,
        };
        
        match type_ {
            Node_var_array => self.symbol_table.insert(name.to_string(), node),
            Node_func | Node_ext_func | Node_builtin_func => self.func_table.insert(name.to_string(), node),
            Node_param_list => self.param_table.insert(name.to_string(), node),
            _ => self.global_table.insert(name.to_string(), node),
        };
        
        self.get_symbol(name)
    }

    fn lookup(&self, name: &str) -> Option<&NODE> {
        let tables = [
            &self.param_table,
            &self.global_table,
            &self.func_table,
            &self.symbol_table,
        ];

        let clean_name = if name.starts_with("awk::") {
            &name[5..]
        } else {
            name
        };

        for table in tables.iter() {
            if let Some(node) = table.get(clean_name) {
                if node.type_ != Node_val {
                    return Some(node);
                }
            }
        }
        None
    }

    fn remove_symbol(&mut self, name: &str) -> Option<NODE> {
        let clean_name = if name.starts_with("awk::") {
            &name[5..]
        } else {
            name
        };
        
        self.symbol_table.remove(clean_name)
    }

    fn get_symbols(&self, what: SYMBOL_TYPE, sort: bool) -> Vec<&NODE> {
        let mut symbols = match what {
            FUNCTION => self.func_table.values().collect(),
            VARIABLE => {
                let mut vars: Vec<&NODE> = self.symbol_table.values()
                    .filter(|n| n.type_ != Node_val)
                    .collect();
                vars.extend(self.global_table.values());
                vars
            }
            _ => vec![],
        };

        if sort {
            symbols.sort_by(|a, b| a.name.cmp(&b.name));
        }

        symbols
    }
}

fn make_symbol(name: &str, type_: NODETYPE) -> NODE {
    NODE {
        type_,
        name: name.to_string(),
        valref: 1,
    }
}

fn is_all_upper(name: &str) -> bool {
    name.chars().all(|c| c.is_ascii_uppercase())
}

// Constants for node types
const Node_var_array: NODETYPE = 5;
const Node_func: NODETYPE = 9;
const Node_ext_func: NODETYPE = 10;
const Node_builtin_func: NODETYPE = 11;
const Node_param_list: NODETYPE = 8;
const Node_val: NODETYPE = 1;
const Node_var: NODETYPE = 4;
const Node_var_new: NODETYPE = 6;