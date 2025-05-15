/*
 * General helper utilities.
 * Copyright (c) 1997 Markku Rossi.
 *
 * Author: Markku Rossi <mtr@iki.fi>
 */

/*
 * This file is part of GNU enscript.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; see the file COPYING.  If not, write to
 * the Free Software Foundation, 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::cell::RefCell;
use std::rc::Rc;

/*
 * Static variables.
 */

static mut CASE_INSENSITIVE_TRANSLATE: Option<Vec<u8>> = None;

/*
 * Types
 */

type ReTranslateType = Option<Vec<u8>>;

#[derive(Debug, Clone, PartialEq)]
enum NodeType {
    Void,
    String,
    Regexp,
    Integer,
    Real,
    Symbol,
    Array,
}

#[derive(Debug, Clone)]
struct Node {
    type_: NodeType,
    refcount: usize,
    linenum: u32,
    data: NodeData,
}

#[derive(Debug, Clone)]
enum NodeData {
    Void,
    StringData { len: usize, data: Vec<u8> },
    RegexpData { data: String, len: usize, flags: u32, compiled: RegexpCompiled },
    Integer(i64),
    Real(f64),
    Symbol(String),
    ArrayData { len: usize, allocated: usize, array: Vec<Rc<RefCell<Node>>> },
}

#[derive(Debug, Clone)]
struct RegexpCompiled {
    fastmap: Vec<u8>,
    translate: ReTranslateType,
}

#[derive(Debug, Clone, PartialEq)]
enum StmtType {
    Expr,
    Return,
    DefSub,
    Block,
    If,
    While,
    For,
}

#[derive(Debug, Clone)]
struct Stmt {
    type_: StmtType,
    linenum: u32,
    data: StmtData,
}

#[derive(Debug, Clone)]
enum StmtData {
    Expr(Rc<RefCell<Expr>>),
    DefSub { name: String, closure: Rc<RefCell<Cons>> },
    Block(Vec<Rc<RefCell<Stmt>>>),
    If { expr: Rc<RefCell<Expr>>, then_stmt: Rc<RefCell<Stmt>>, else_stmt: Option<Rc<RefCell<Stmt>>> },
    While { expr: Rc<RefCell<Expr>>, body: Rc<RefCell<Stmt>> },
    For { init: Option<Rc<RefCell<Expr>>>, cond: Rc<RefCell<Expr>>, incr: Option<Rc<RefCell<Expr>>>, body: Rc<RefCell<Stmt>> },
}

#[derive(Debug, Clone, PartialEq)]
enum ExprType {
    String,
    Regexp,
    Integer,
    Real,
    Symbol,
    Not,
    FCall,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    PostfixAdd,
    PostfixSub,
    PrefixAdd,
    PrefixSub,
    ArrayAssign,
    ArrayRef,
    QuestColon,
    Mult,
    Div,
    Plus,
    Minus,
    Lt,
    Gt,
    Eq,
    Ne,
    Ge,
    Le,
    And,
    Or,
}

#[derive(Debug, Clone)]
struct Expr {
    type_: ExprType,
    linenum: u32,
    data: ExprData,
}

#[derive(Debug, Clone)]
enum ExprData {
    Node(Rc<RefCell<Node>>),
    Not(Rc<RefCell<Expr>>),
    FCall { name: String, args: Vec<Rc<RefCell<Expr>>> },
    Assign { sym: String, expr: Rc<RefCell<Expr>> },
    Op { left: Rc<RefCell<Expr>>, right: Rc<RefCell<Expr>> },
    ArrayAssign { expr1: Rc<RefCell<Expr>>, expr2: Rc<RefCell<Expr>>, expr3: Rc<RefCell<Expr>> },
    ArrayRef { expr1: Rc<RefCell<Expr>>, expr2: Rc<RefCell<Expr>> },
    QuestColon { cond: Rc<RefCell<Expr>>, expr1: Rc<RefCell<Expr>>, expr2: Rc<RefCell<Expr>> },
}

#[derive(Debug, Clone)]
struct Cons {
    car: Rc<RefCell<Node>>,
    cdr: Rc<RefCell<Node>>,
}

#[derive(Debug, Clone)]
struct List {
    head: Option<Rc<RefCell<ListItem>>>,
    tail: Option<Rc<RefCell<ListItem>>>,
}

#[derive(Debug, Clone)]
struct ListItem {
    data: Rc<RefCell<Node>>,
    next: Option<Rc<RefCell<ListItem>>>,
}

#[derive(Debug, Clone)]
struct Environment {
    name: String,
    val: Rc<RefCell<Node>>,
    next: Option<Rc<RefCell<Environment>>>,
}

/*
 * Global variables
 */

lazy_static! {
    static ref NS_VARS: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
    static ref NS_STATES: HashMap<String, Rc<RefCell<List>>> = HashMap::new();
    static ref NS_SUBS: HashMap<String, Rc<RefCell<Cons>>> = HashMap::new();
    static ref NS_PRIMS: HashMap<String, fn(String, Vec<Rc<RefCell<Expr>>, Option<Rc<RefCell<Environment>>, u32) -> Rc<RefCell<Node>>> = HashMap::new();
}

/*
 * Implementation
 */

impl List {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn prepend(&mut self, data: Rc<RefCell<Node>>) {
        let item = Rc::new(RefCell::new(ListItem {
            data,
            next: self.head.take(),
        }));

        self.head = Some(item.clone());
        if self.tail.is_none() {
            self.tail = Some(item);
        }
    }

    fn append(&mut self, data: Rc<RefCell<Node>>) {
        let item = Rc::new(RefCell::new(ListItem {
            data,
            next: None,
        }));

        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(item.clone());
            self.tail = Some(item);
        } else {
            self.head = Some(item.clone());
            self.tail = Some(item);
        }
    }
}

impl Node {
    fn alloc(type_: NodeType) -> Rc<RefCell<Self>> {
        let node = match type_ {
            NodeType::Regexp => {
                Node {
                    type_,
                    refcount: 1,
                    linenum: unsafe { LINENUM },
                    data: NodeData::RegexpData {
                        data: String::new(),
                        len: 0,
                        flags: 0,
                        compiled: RegexpCompiled {
                            fastmap: vec![0; 256],
                            translate: None,
                        },
                    },
                }
            }
            _ => {
                Node {
                    type_,
                    refcount: 1,
                    linenum: unsafe { LINENUM },
                    data: NodeData::Void,
                }
            }
        };
        Rc::new(RefCell::new(node))
    }

    fn copy(node: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let node = node.borrow();
        let new_node = Node::alloc(node.type_.clone());
        let mut new_node_mut = new_node.borrow_mut();
        new_node_mut.linenum = node.linenum;

        match &node.data {
            NodeData::Void => {}
            NodeData::StringData { len, data } => {
                new_node_mut.data = NodeData::StringData {
                    len: *len,
                    data: data.clone(),
                };
            }
            NodeData::RegexpData { data, len, .. } => {
                if let NodeData::RegexpData { ref mut compiled, .. } = new_node_mut.data {
                    compiled.fastmap = vec![0; 256];
                }
                new_node_mut.data = NodeData::RegexpData {
                    data: data.clone(),
                    len: *len,
                    flags: 0,
                    compiled: RegexpCompiled {
                        fastmap: vec![0; 256],
                        translate: None,
                    },
                };
            }
            NodeData::Integer(val) => {
                new_node_mut.data = NodeData::Integer(*val);
            }
            NodeData::Real(val) => {
                new_node_mut.data = NodeData::Real(*val);
            }
            NodeData::Symbol(sym) => {
                new_node_mut.data = NodeData::Symbol(sym.clone());
            }
            NodeData::ArrayData { len, allocated, array } => {
                let mut new_array = Vec::with_capacity(*allocated);
                for item in array {
                    new_array.push(Node::copy(item));
                }
                new_node_mut.data = NodeData::ArrayData {
                    len: *len,
                    allocated: *allocated,
                    array: new_array,
                };
            }
        }

        new_node
    }

    fn reference(node: &Rc<RefCell<Node>>) {
        node.borrow_mut().refcount += 1;
    }

    fn free(node: Rc<RefCell<Node>>) {
        let refcount = {
            let mut node_mut = node.borrow_mut();
            node_mut.refcount -= 1;
            node_mut.refcount
        };

        if refcount > 0 {
            return;
        }

        // Last reference, free the node
        let node = node.borrow();
        match &node.data {
            NodeData::Void => {}
            NodeData::StringData { data, .. } => {
                // Data is automatically dropped
            }
            NodeData::RegexpData { compiled, .. } => {
                // Fastmap is automatically dropped
            }
            NodeData::ArrayData { array, .. } => {
                for item in array {
                    Node::free(item.clone());
                }
            }
            _ => {}
        }
    }
}

fn enter_system_variable(name: &str, value: &str) {
    let n = Node::alloc(NodeType::String);
    {
        let mut n_mut = n.borrow_mut();
        n_mut.data = NodeData::StringData {
            len: value.len(),
            data: value.as_bytes().to_vec(),
        };
    }

    if let Some(old_val) = NS_VARS.insert(name.to_string(), n) {
        Node::free(old_val);
    }
}

fn compile_regexp(re: &Rc<RefCell<Node>>) {
    unsafe {
        if CASE_INSENSITIVE_TRANSLATE.is_none() {
            let mut translate = vec![0; 256];
            for i in 0..256 {
                if i.is_ascii_uppercase() {
                    translate[i] = i.to_ascii_lowercase();
                } else {
                    translate[i] = i as u8;
                }
            }
            CASE_INSENSITIVE_TRANSLATE = Some(translate);
        }
    }

    let mut re_mut = re.borrow_mut();
    if let NodeData::RegexpData { ref mut compiled, flags, .. } = re_mut.data {
        if flags & fRE_CASE_INSENSITIVE != 0 {
            compiled.translate = unsafe { CASE_INSENSITIVE_TRANSLATE.clone() };
        }

        // TODO: Implement regex compilation
        // This would require integrating a regex crate like regex or pcre
    }
}

fn mk_stmt(
    type_: StmtType,
    arg1: Option<Rc<RefCell<Expr>>>,
    arg2: Option<Rc<RefCell<Stmt>>>,
    arg3: Option<Rc<RefCell<Stmt>>>,
    arg4: Option<Rc<RefCell<Stmt>>>,
) -> Rc<RefCell<Stmt>> {
    let stmt = match type_ {
        StmtType::Expr | StmtType::Return => {
            Stmt {
                type_,
                linenum: unsafe { LINENUM },
                data: StmtData::Expr(arg1.unwrap()),
            }
        }
        StmtType::DefSub => {
            Stmt {
                type_,
                linenum: unsafe { LINENUM },
                data: StmtData::DefSub {
                    name: "".to_string(), // TODO: Get name from arg1
                    closure: Rc::new(RefCell::new(Cons {
                        car: Rc::new(RefCell::new(Node::alloc(NodeType::Void))),
                        cdr: Rc::new(RefCell::new(Node::alloc(NodeType::Void))),
                    })),
                },
            }
        }
        StmtType::Block => {
            Stmt {
                type_,
                linenum: unsafe { LINENUM },
                data: StmtData::Block(vec![]), // TODO: Get block from arg1
            }
        }
        StmtType::If => {
            Stmt {
                type_,
                linenum: unsafe { LINENUM },
                data: StmtData::If {
                    expr: arg1.unwrap(),
                    then_stmt: arg2.unwrap(),
                    else_stmt: arg3,
                },
            }
        }
        StmtType::While => {
            Stmt {
                type_,
                linenum: unsafe { LINENUM },
                data: StmtData::While {
                    expr: arg1.unwrap(),
                    body: arg2.unwrap(),
                },
            }
        }
        StmtType::For => {
            Stmt {
                type_,
                linenum: unsafe { LINENUM },
                data: StmtData::For {
                    init: arg1,
                    cond: arg2.unwrap(),
                    incr: arg3,
                    body: arg4.unwrap(),
                },
            }
        }
    };

    Rc::new(RefCell::new(stmt))
}

fn mk_expr(
    type_: ExprType,
    arg1: Option<Rc<RefCell<Expr>>>,
    arg2: Option<Rc<RefCell<Expr>>>,
    arg3: Option<Rc<RefCell<Expr>>>,
) -> Rc<RefCell<Expr>> {
    let expr = match type_ {
        ExprType::String
        | ExprType::Regexp
        | ExprType::Integer
        | ExprType::Real
        | ExprType::Symbol => Expr {
            type_,
            linenum: unsafe { LINENUM },
            data: ExprData::Node(arg1.unwrap()),
        },
        ExprType::Not => Expr {
            type_,
            linenum: unsafe { LINENUM },
            data: ExprData::Not(arg1.unwrap()),
        },
        ExprType::FCall => Expr {
            type_,
            linenum: unsafe { LINENUM },
            data: ExprData::FCall {
                name: "".to_string(), // TODO: Get name from arg1
                args: vec![],        // TODO: Get args from arg2
            },
        },
        ExprType::Assign
        | ExprType::AddAssign
        | ExprType::SubAssign
        | ExprType::MulAssign
        | ExprType::DivAssign => Expr {
            type_,
            linenum: unsafe { LINENUM },
            data: ExprData::Assign {
                sym: "".to_string(), // TODO: Get sym from arg1
                expr: arg2.unwrap(),
            },
        },
        ExprType::ArrayAssign => Expr {
            type_,
            linenum: unsafe { LINENUM },
            data: ExprData::ArrayAssign {
                expr1: arg1.unwrap(),
                expr2: arg2.unwrap(),
                expr3: arg3.unwrap(),
            },
        },
        ExprType::ArrayRef => Expr {
            type_,
            linenum: unsafe { LINENUM },
            data: ExprData::ArrayRef {
                expr1: arg1.unwrap(),
                expr2: arg2.unwrap(),
            },
        },
        ExprType::QuestColon => Expr {
            type_,
            linenum: unsafe { LINENUM },
            data: ExprData::QuestColon {
                cond: arg1.unwrap(),
                expr1: arg2.unwrap(),
                expr2: arg3.unwrap(),
            },
        },
        _ => Expr {
            type_,
            linenum: unsafe { LINENUM },
            data: ExprData::Op {
                left: arg1.unwrap(),
                right: arg2.unwrap(),
            },
        },
    };

    Rc::new(RefCell::new(expr))
}

fn cons(car: Rc<RefCell<Node>>, cdr: Rc<RefCell<Node>>) -> Rc<RefCell<Cons>> {
    Rc::new(RefCell::new(Cons { car, cdr }))
}

fn define_state(sym: &Rc<RefCell<Node>>, rules: &Rc<RefCell<List>>) {
    let sym_borrow = sym.borrow();
    if let NodeData::Symbol(sym_str) = &sym_borrow.data {
        if let Some(old_rules) = NS_STATES.insert(sym_str.clone(), rules.clone()) {
            // TODO: Handle warning about redefining state
        }
    }
}

// TODO: Implement remaining functions (eval_expr, eval_statement, etc.)
// These would require more context about the execution environment and additional helper functions

// Constants
const fRE_CASE_INSENSITIVE: u32 = 1;
static mut LINENUM: u32 = 0;
static mut CURRENT_MATCH: Option<Match