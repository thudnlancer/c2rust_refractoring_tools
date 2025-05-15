use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

#[derive(Debug, Clone, PartialEq)]
enum NodeType {
    VarArray,
    Val,
    Var,
    ElemNew,
    VarNew,
    Func,
    ExtFunc,
    BuiltinFunc,
    ArrayRef,
    ParamList,
    DumpArray,
    Illegal,
}

#[derive(Debug, Clone)]
struct Node {
    type_: NodeType,
    flags: u32,
    stptr: Option<Vec<u8>>,
    stlen: usize,
    stfmt: i32,
    numbr: f64,
    valref: i32,
    vname: Option<String>,
    parent_array: Option<Box<Node>>,
    array_funcs: Option<Box<ArrayFuncs>>,
    table_size: usize,
    array_size: usize,
    array_capacity: usize,
    buckets: Option<Vec<Node>>,
    xarray: Option<Box<Node>>,
    orig_array: Option<Box<Node>>,
    prev_array: Option<Box<Node>>,
    param_cnt: i32,
    var_value: Option<Box<Node>>,
    func_body: Option<Box<Node>>,
    func_name: Option<String>,
    nexti: Option<Box<Node>>,
    expr_count: i32,
    adepth: i32,
    alevel: i32,
}

#[derive(Debug, Clone)]
struct ArrayFuncs {
    name: String,
    init: Option<fn(&Node, &Node) -> Option<Node>>,
    lookup: fn(&Node, &Node) -> Option<Node>,
    type_of: fn(&Node, &Node) -> Option<Node>,
    alist: fn(&Node, &Node) -> Vec<Node>,
    acopy: fn(&Node, &Node) -> Node,
    adump: fn(&Node, &Node),
    clear: fn(&Node),
    remove: fn(&Node, &Node) -> Option<Node>,
    length: fn(&Node) -> usize,
    empty: fn(&Node) -> bool,
}

static mut OUTPUT_FP: Option<*mut libc::FILE> = None;
static mut SUBSEP: Option<*mut c_char> = None;
static mut SUBSEPLEN: usize = 0;
static INDENT_CHAR: &str = "    ";

static mut SUCCESS_NODE: Option<Node> = None;
static mut FMT_LIST: Option<Vec<Node>> = None;
static mut ARRAY_TYPES: [Option<ArrayFuncs>; 10] = [None; 10];
static mut NUM_ARRAY_TYPES: usize = 0;

fn register_array_func(afunc: ArrayFuncs) -> bool {
    unsafe {
        if NUM_ARRAY_TYPES < ARRAY_TYPES.len() {
            if afunc.name != "str" && afunc.type_of == None {
                return false;
            }
            ARRAY_TYPES[NUM_ARRAY_TYPES] = Some(afunc);
            NUM_ARRAY_TYPES += 1;
            if let Some(init) = afunc.init {
                init(&Node::default(), &Node::default());
            }
            true
        } else {
            false
        }
    }
}

fn array_init() {
    let str_func = ArrayFuncs {
        name: "str".to_string(),
        init: None,
        lookup: |_, _| None,
        type_of: |_, _| None,
        alist: |_, _| vec![],
        acopy: |_, _| Node::default(),
        adump: |_, _| {},
        clear: |_| {},
        remove: |_, _| None,
        length: |_| 0,
        empty: |_| true,
    };
    register_array_func(str_func);
    
    // Register other array types if needed
}

fn make_array() -> Node {
    Node {
        type_: NodeType::VarArray,
        array_funcs: None,
        ..Node::default()
    }
}

fn null_array(symbol: &mut Node) {
    symbol.type_ = NodeType::VarArray;
    symbol.array_funcs = None;
    symbol.buckets = None;
    symbol.table_size = 0;
    symbol.array_size = 0;
    symbol.array_capacity = 0;
    symbol.flags = 0;
    assert!(symbol.xarray.is_none());
}

fn null_lookup(symbol: &mut Node, subs: &Node) -> Option<Node> {
    assert!(symbol.table_size == 0);
    
    let mut afunc = None;
    unsafe {
        for i in (1..NUM_ARRAY_TYPES).rev() {
            if let Some(func) = &ARRAY_TYPES[i] {
                if (func.type_of)(symbol, subs).is_some() {
                    afunc = Some(func.clone());
                    break;
                }
            }
        }
    }
    
    if afunc.is_none() {
        unsafe {
            afunc = ARRAY_TYPES[0].clone();
        }
    }
    
    symbol.array_funcs = afunc.map(Box::new);
    (symbol.array_funcs.as_ref().unwrap().lookup)(symbol, subs)
}

fn null_afunc(_symbol: &Node, _subs: &Node) -> Option<Node> {
    None
}

fn null_dump(symbol: &Node, _subs: &Node) -> Option<Node> {
    unsafe {
        if let Some(fp) = OUTPUT_FP {
            libc::fprintf(fp, "array `%s' is empty\n", symbol.vname.as_ref().unwrap().as_ptr());
        }
    }
    None
}

fn assoc_copy(symbol: &Node, newsymb: &mut Node) -> Node {
    assert!(newsymb.vname.is_some());
    
    assoc_clear(newsymb);
    (symbol.array_funcs.as_ref().unwrap().acopy)(symbol, newsymb);
    newsymb.array_funcs = symbol.array_funcs.clone();
    newsymb.flags = symbol.flags;
    newsymb.clone()
}

fn assoc_dump(symbol: &Node, ndump: &Node) {
    if let Some(adump) = symbol.array_funcs.as_ref().map(|f| f.adump) {
        adump(symbol, ndump);
    }
}

fn make_aname(symbol: &Node) -> String {
    static mut ANAME: Option<String> = None;
    static mut ALEN: usize = 0;
    static mut MAX_ALEN: usize = 0;
    
    unsafe {
        if let Some(parent) = &symbol.parent_array {
            let parent_name = make_aname(parent);
            let slen = symbol.vname.as_ref().unwrap().len();
            
            if ALEN + slen + 4 > MAX_ALEN {
                MAX_ALEN = ALEN + slen + 4 + 256;
                ANAME = Some(String::with_capacity(MAX_ALEN + 1));
            }
            
            if let Some(aname) = &mut ANAME {
                aname.push_str(&format!("[\"{}\"]", symbol.vname.as_ref().unwrap()));
                ALEN = aname.len();
                aname.clone()
            } else {
                String::new()
            }
        } else {
            ALEN = symbol.vname.as_ref().unwrap().len();
            if ANAME.is_none() {
                MAX_ALEN = ALEN + 256;
                ANAME = Some(symbol.vname.as_ref().unwrap().clone());
            } else if ALEN > MAX_ALEN {
                MAX_ALEN = ALEN + 256;
                ANAME = Some(symbol.vname.as_ref().unwrap().clone());
            }
            ANAME.as_ref().unwrap().clone()
        }
    }
}

// ... Additional functions would follow the same pattern ...

impl Default for Node {
    fn default() -> Self {
        Node {
            type_: NodeType::Illegal,
            flags: 0,
            stptr: None,
            stlen: 0,
            stfmt: 0,
            numbr: 0.0,
            valref: 0,
            vname: None,
            parent_array: None,
            array_funcs: None,
            table_size: 0,
            array_size: 0,
            array_capacity: 0,
            buckets: None,
            xarray: None,
            orig_array: None,
            prev_array: None,
            param_cnt: 0,
            var_value: None,
            func_body: None,
            func_name: None,
            nexti: None,
            expr_count: 0,
            adepth: 0,
            alevel: 0,
        }
    }
}