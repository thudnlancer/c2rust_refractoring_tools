use std::collections::HashMap;
use std::env;
use std::io::{self, Read, Write};
use std::mem;
use std::ptr;
use std::slice;
use std::str;

#[derive(Debug, Clone, PartialEq)]
enum Obj {
    Int(i32),
    Cell(Box<Obj>, Box<Obj>),
    Symbol(String),
    Primitive(fn(&mut Env, &[Obj]) -> Obj),
    Function {
        params: Box<Obj>,
        body: Box<Obj>,
        env: Box<Env>,
    },
    Macro {
        params: Box<Obj>,
        body: Box<Obj>,
        env: Box<Env>,
    },
    Env {
        vars: HashMap<String, Obj>,
        up: Box<Env>,
    },
    True,
    Nil,
    Dot,
    Cparen,
}

#[derive(Debug, Clone)]
struct Env {
    vars: HashMap<String, Obj>,
    up: Option<Box<Env>>,
}

impl Env {
    fn new(up: Option<Box<Env>>) -> Self {
        Env {
            vars: HashMap::new(),
            up,
        }
    }

    fn find(&self, sym: &str) -> Option<Obj> {
        self.vars.get(sym).cloned().or_else(|| {
            self.up
                .as_ref()
                .and_then(|up| up.find(sym))
        })
    }
}

fn error(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}

fn cons(car: Obj, cdr: Obj) -> Obj {
    Obj::Cell(Box::new(car), Box::new(cdr))
}

fn make_symbol(name: &str) -> Obj {
    Obj::Symbol(name.to_string())
}

fn make_int(value: i32) -> Obj {
    Obj::Int(value)
}

fn make_primitive(f: fn(&mut Env, &[Obj]) -> Obj) -> Obj {
    Obj::Primitive(f)
}

fn make_function(env: Env, params: Obj, body: Obj) -> Obj {
    Obj::Function {
        params: Box::new(params),
        body: Box::new(body),
        env: Box::new(env),
    }
}

fn make_macro(env: Env, params: Obj, body: Obj) -> Obj {
    Obj::Macro {
        params: Box::new(params),
        body: Box::new(body),
        env: Box::new(env),
    }
}

fn make_env(vars: HashMap<String, Obj>, up: Option<Box<Env>>) -> Obj {
    Obj::Env {
        vars,
        up: Box::new(Env::new(up)),
    }
}

fn acons(x: Obj, y: Obj, a: Obj) -> Obj {
    cons(cons(x, y), a)
}

fn reverse(mut list: Obj) -> Obj {
    let mut result = Obj::Nil;
    while let Obj::Cell(head, tail) = list {
        result = cons(*head, result);
        list = *tail;
    }
    result
}

fn is_list(obj: &Obj) -> bool {
    matches!(obj, Obj::Nil | Obj::Cell(_, _))
}

fn length(list: &Obj) -> Option<usize> {
    let mut len = 0;
    let mut current = list;
    while let Obj::Cell(_, tail) = current {
        len += 1;
        current = tail;
    }
    if current == &Obj::Nil {
        Some(len)
    } else {
        None
    }
}

fn read_expr() -> Option<Obj> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    let input = input.trim();
    if input.is_empty() {
        None
    } else {
        Some(parse_expr(input))
    }
}

fn parse_expr(input: &str) -> Obj {
    let mut chars = input.chars().peekable();
    parse(&mut chars)
}

fn parse<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) -> Obj {
    skip_whitespace(chars);
    match chars.peek() {
        Some('(') => {
            chars.next();
            parse_list(chars)
        }
        Some(')') => {
            chars.next();
            Obj::Cparen
        }
        Some('.') => {
            chars.next();
            Obj::Dot
        }
        Some('\'') => {
            chars.next();
            let expr = parse(chars);
            cons(
                make_symbol("quote"),
                cons(expr, Obj::Nil),
            )
        }
        Some(c) if c.is_digit(10) || (*c == '-' && chars.clone().nth(1).map_or(false, |c| c.is_digit(10))) => {
            parse_number(chars)
        }
        Some(c) if c.is_alphabetic() || is_symbol_char(*c) => {
            parse_symbol(chars)
        }
        _ => error("Invalid syntax"),
    }
}

fn parse_list<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) -> Obj {
    let mut list = Obj::Nil;
    loop {
        skip_whitespace(chars);
        match chars.peek() {
            Some(')') => {
                chars.next();
                return reverse(list);
            }
            Some('.') => {
                chars.next();
                let cdr = parse(chars);
                skip_whitespace(chars);
                if chars.next() != Some(')') {
                    error("Expected closing parenthesis after dot");
                }
                return if let Obj::Cell(head, _) = reverse(list) {
                    cons(*head, cdr)
                } else {
                    error("Invalid dotted pair syntax");
                };
            }
            None => error("Unclosed list"),
            _ => {
                let expr = parse(chars);
                list = cons(expr, list);
            }
        }
    }
}

fn parse_number<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) -> Obj {
    let mut s = String::new();
    let mut neg = false;
    if chars.peek() == Some(&'-') {
        neg = true;
        chars.next();
    }
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) {
            s.push(c);
            chars.next();
        } else {
            break;
        }
    }
    let num: i32 = s.parse().unwrap();
    make_int(if neg { -num } else { num })
}

fn parse_symbol<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) -> Obj {
    let mut s = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || is_symbol_char(c) {
            s.push(c);
            chars.next();
        } else {
            break;
        }
    }
    make_symbol(&s)
}

fn is_symbol_char(c: char) -> bool {
    "~!@#$%^&*-_=+:/?<>".contains(c)
}

fn skip_whitespace<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) {
    while let Some(c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

fn print(obj: &Obj) {
    match obj {
        Obj::Int(n) => print!("{}", n),
        Obj::Symbol(s) => print!("{}", s),
        Obj::Cell(car, cdr) => {
            print!("(");
            print(car);
            let mut current = cdr;
            loop {
                match &**current {
                    Obj::Nil => break,
                    Obj::Cell(head, tail) => {
                        print!(" ");
                        print(head);
                        current = tail;
                    }
                    _ => {
                        print!(" . ");
                        print(current);
                        break;
                    }
                }
            }
            print!(")");
        }
        Obj::Primitive(_) => print!("<primitive>"),
        Obj::Function { .. } => print!("<function>"),
        Obj::Macro { .. } => print!("<macro>"),
        Obj::Env { .. } => print!("<environment>"),
        Obj::True => print!("t"),
        Obj::Nil => print!("()"),
        Obj::Dot => print!("."),
        Obj::Cparen => print!(")"),
    }
}

fn eval(env: &mut Env, obj: &Obj) -> Obj {
    match obj {
        Obj::Int(_) | Obj::True | Obj::Nil | Obj::Primitive(_) => obj.clone(),
        Obj::Symbol(sym) => env.find(sym).unwrap_or_else(|| error(&format!("Undefined symbol: {}", sym))),
        Obj::Cell(car, cdr) => {
            let expanded = macroexpand(env, obj);
            if &expanded != obj {
                return eval(env, &expanded);
            }
            let func = eval(env, car);
            let args = eval_list(env, cdr);
            apply(env, &func, &args)
        }
        _ => error("Cannot evaluate object"),
    }
}

fn eval_list(env: &mut Env, list: &Obj) -> Vec<Obj> {
    let mut args = Vec::new();
    let mut current = list;
    while let Obj::Cell(head, tail) = current {
        args.push(eval(env, head));
        current = tail;
    }
    args
}

fn apply(env: &mut Env, func: &Obj, args: &[Obj]) -> Obj {
    match func {
        Obj::Primitive(f) => f(env, args),
        Obj::Function { params, body, env: closure_env } => {
            let mut new_env = Env::new(Some(Box::new(closure_env.clone())));
            bind_params(&mut new_env, params, args);
            progn(&mut new_env, body)
        }
        _ => error("Not a function"),
    }
}

fn bind_params(env: &mut Env, params: &Obj, args: &[Obj]) {
    let mut param_iter = params;
    let mut arg_iter = args.iter();
    loop {
        match param_iter {
            Obj::Symbol(sym) => {
                let rest_args: Vec<_> = arg_iter.cloned().collect();
                env.vars.insert(sym.clone(), list(&rest_args));
                break;
            }
            Obj::Cell(param_head, param_tail) => {
                if let Some(arg) = arg_iter.next() {
                    env.vars.insert(param_head.as_symbol().unwrap(), arg.clone());
                    param_iter = param_tail;
                } else {
                    error("Too few arguments");
                }
            }
            Obj::Nil => {
                if arg_iter.next().is_some() {
                    error("Too many arguments");
                }
                break;
            }
            _ => error("Invalid parameter list"),
        }
    }
}

fn progn(env: &mut Env, body: &Obj) -> Obj {
    let mut result = Obj::Nil;
    let mut current = body;
    while let Obj::Cell(head, tail) = current {
        result = eval(env, head);
        current = tail;
    }
    result
}

fn macroexpand(env: &mut Env, obj: &Obj) -> Obj {
    if let Obj::Cell(car, cdr) = obj {
        if let Obj::Symbol(sym) = &**car {
            if let Some(Obj::Macro { params, body, env: macro_env }) = env.find(sym) {
                let mut new_env = Env::new(Some(Box::new(*macro_env)));
                bind_params(&mut new_env, &params, &eval_list(env, cdr));
                return progn(&mut new_env, &body);
            }
        }
    }
    obj.clone()
}

fn list(elements: &[Obj]) -> Obj {
    let mut result = Obj::Nil;
    for elem in elements.iter().rev() {
        result = cons(elem.clone(), result);
    }
    result
}

// Primitive functions
fn prim_quote(_env: &mut Env, args: &[Obj]) -> Obj {
    if args.len() != 1 {
        error("quote requires exactly one argument");
    }
    args[0].clone()
}

fn prim_cons(_env: &mut Env, args: &[Obj]) -> Obj {
    if args.len() != 2 {
        error("cons requires exactly two arguments");
    }
    cons(args[0].clone(), args[1].clone())
}

fn prim_car(_env: &mut Env, args: &[Obj]) -> Obj {
    if args.len() != 1 {
        error("car requires exactly one argument");
    }
    if let Obj::Cell(car, _) = &args[0] {
        (**car).clone()
    } else {
        error("car requires a cons cell");
    }
}

fn prim_cdr(_env: &mut Env, args: &[Obj]) -> Obj {
    if args.len() != 1 {
        error("cdr requires exactly one argument");
    }
    if let Obj::Cell(_, cdr) = &args[0] {
        (**cdr).clone()
    } else {
        error("cdr requires a cons cell");
    }
}

fn prim_setq(env: &mut Env, args: &[Obj]) -> Obj {
    if args.len() != 2 {
        error("setq requires exactly two arguments");
    }
    if let Obj::Symbol(sym) = &args[0] {
        let value = eval(env, &args[1]);
        env.vars.insert(sym.clone(), value.clone());
        value
    } else {
        error("setq requires a symbol as first argument");
    }
}

fn prim_setcar(_env: &mut Env, args: &[Obj]) -> Obj {
    if args.len() != 2 {
        error("setcar requires exactly two arguments");
    }
    if let Obj::Cell(car, _) = &args[0] {
        **car = args[1].clone();
        args[0].clone()
    } else {
        error("setcar requires a cons cell as first argument");
    }
}

fn prim_while(env: &mut Env, args: &[Obj]) -> Obj {
    if args.len() < 2 {
        error("while requires at least two arguments");
    }
    while eval(env, &args[0]) != Obj::Nil {
        for arg in &args[1..] {
            eval(env, arg);
        }
    }
    Obj::Nil
}

fn prim_plus(_env: &mut Env, args: &[Obj]) -> Obj {
    let mut sum = 0;
    for arg in args {
        if let Obj::Int(n) = arg {
            sum += n;
        } else {
            error("+ requires integers");
        }
    }
    Obj::Int(sum)
}

fn prim_minus(_env: &mut Env, args: &[Obj]) -> Obj {
    if args.is_empty() {
        error("- requires at least one argument");
    }
    let mut result = if let Obj::Int(n) = args[0] {
        n
    } else {
        error("- requires integers");
    };
    if args.len() == 1 {
        Obj::Int(-result)
    } else {
        for arg in &args[1..] {
            if let Obj::Int(n) = arg {
                result -= n;
            } else {
                error("- requires integers");
            }
        }
        Obj::Int(result)
    }
}

fn prim_lt(_env: &mut Env, args: &[Obj]) -> Obj {
    if args.len() != 2 {
        error("< requires exactly two arguments");
    }
    if let (Obj::Int(a), Obj::Int(b)) = (&args[0], &args[1]) {
        if a < b {
            Obj::True
        } else {
            Obj::Nil
        }
    } else {
        error("< requires integers");
    }
}

fn prim_lambda(env: &mut Env, args: &[Obj]) -> Obj {
    if args.len() != 2 {
        error("lambda requires exactly two arguments");
    }
    make_function(env.clone(), args[0].clone(), args[1].clone())
}

fn prim_defun(env: &mut Env, args: &[Obj]) -> Obj {
    if args.len() < 3 || !args[0].is_symbol() {
        error("defun requires a symbol and at least two arguments");
    }
    let name = args[0].as_symbol().unwrap();
    let params = args[1].clone();
    let body = if args.len() > 3 {
        list(&args[2..])
    } else {
        args[2].clone()
    };
    let func = make_function(env.clone(), params, body);
    env.vars.insert(name.to_string(), func.clone());
    func
}

fn prim_define(env: &mut Env, args: &[Obj]) -> Obj {
    if args.len() != 2 || !args[0].is_symbol() {
        error("define requires a symbol and exactly one argument");
    }
    let name = args[0].as_symbol().unwrap();
    let value = eval(env, &args[1]);
    env.vars.insert(name.to_string(), value.clone());
    value
}

fn prim_defmacro(env: &mut Env, args: &[Obj]) -> Obj {
    if args.len() < 3 || !args[0].is_symbol() {
        error("defmacro requires a symbol and at least two arguments");
    }
    let name = args[0].as_symbol().unwrap();
    let params = args[1].clone();
    let body = if args.len() > 3 {
        list(&args[2..])
    } else {
        args[2].clone()
    };
    let macro_ = make_macro(env.clone(), params, body);
    env