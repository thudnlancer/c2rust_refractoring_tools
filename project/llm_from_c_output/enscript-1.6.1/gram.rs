use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug, Clone)]
enum YYSTYPE {
    List(Vec<YYSTYPE>),
    Node(String),
    Cons(Box<YYSTYPE>, Box<YYSTYPE>),
    Stmt(Stmt),
    Expr(Expr),
}

#[derive(Debug, Clone)]
struct Stmt {
    kind: StmtKind,
    expr: Option<Box<Expr>>,
    stmt1: Option<Box<Stmt>>,
    stmt2: Option<Box<Stmt>>,
    stmt3: Option<Box<Stmt>>,
}

#[derive(Debug, Clone)]
enum StmtKind {
    Return,
    Defsub,
    Block,
    If,
    While,
    For,
    Expr,
}

#[derive(Debug, Clone)]
struct Expr {
    kind: ExprKind,
    left: Option<Box<Expr>>,
    right: Option<Box<Expr>>,
    extra: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
enum ExprKind {
    String(String),
    Regexp(String),
    Integer(i64),
    Real(f64),
    Symbol(String),
    Not,
    And,
    Or,
    Fcall,
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
}

static mut yylval: YYSTYPE = YYSTYPE::Node(String::new());
static mut defs_file: *const c_char = std::ptr::null();
static mut linenum: i32 = 0;

const tSYMBOL: i32 = 258;
const tREGEXP: i32 = 259;
const tSTRING: i32 = 260;
const tINTEGER: i32 = 261;
const tREAL: i32 = 262;
const tSUB: i32 = 263;
const tSTATE: i32 = 264;
const tSTART: i32 = 265;
const tSTARTRULES: i32 = 266;
const tNAMERULES: i32 = 267;
const tBEGIN: i32 = 268;
const tEND: i32 = 269;
const tRETURN: i32 = 270;
const tIF: i32 = 271;
const tELSE: i32 = 272;
const tLOCAL: i32 = 273;
const tWHILE: i32 = 274;
const tFOR: i32 = 275;
const tADDASSIGN: i32 = 276;
const tSUBASSIGN: i32 = 277;
const tMULASSIGN: i32 = 278;
const tDIVASSIGN: i32 = 279;
const tOR: i32 = 280;
const tAND: i32 = 281;
const tEQ: i32 = 282;
const tNE: i32 = 283;
const tGE: i32 = 284;
const tLE: i32 = 285;
const tDIV: i32 = 286;
const tPLUSPLUS: i32 = 287;
const tMINUSMINUS: i32 = 288;

fn yyerror(msg: &str) {
    unsafe {
        let file = if defs_file.is_null() {
            "<unknown>"
        } else {
            std::ffi::CStr::from_ptr(defs_file).to_str().unwrap_or("<unknown>")
        };
        eprintln!("{}:{}: {}", file, linenum, msg);
    }
}

fn mk_stmt(
    kind: StmtKind,
    expr: Option<Box<Expr>>,
    stmt1: Option<Box<Stmt>>,
    stmt2: Option<Box<Stmt>>,
    stmt3: Option<Box<Stmt>>,
) -> Stmt {
    Stmt {
        kind,
        expr,
        stmt1,
        stmt2,
        stmt3,
    }
}

fn mk_expr(
    kind: ExprKind,
    left: Option<Box<Expr>>,
    right: Option<Box<Expr>>,
    extra: Option<Box<Expr>>,
) -> Expr {
    Expr {
        kind,
        left,
        right,
        extra,
    }
}

fn list() -> Vec<YYSTYPE> {
    Vec::new()
}

fn list_append(lst: &mut Vec<YYSTYPE>, item: YYSTYPE) {
    lst.push(item);
}

fn define_state(node: YYSTYPE, lst: Vec<YYSTYPE>) {
    // Implementation depends on your specific needs
}

fn start_stmts(lst: Vec<YYSTYPE>) {
    // Implementation depends on your specific needs
}

fn startrules(lst: Vec<YYSTYPE>) {
    // Implementation depends on your specific needs
}

fn namerules(lst: Vec<YYSTYPE>) {
    // Implementation depends on your specific needs
}

fn cons(left: YYSTYPE, right: YYSTYPE) -> YYSTYPE {
    YYSTYPE::Cons(Box::new(left), Box::new(right))
}

// The rest of the parser implementation would follow here,
// including the actual parsing logic and grammar rules.
// This would be quite extensive and would need to be adapted
// from the original Bison-generated parser code.