/*
 * ext.rs - Builtin function that links external gawk functions and related
 *         utilities.
 *
 * Christos Zoulas, Thu Jun 29 17:40:41 EDT 1995
 * Arnold Robbins, update for 3.1, Mon Nov 23 12:53:39 EST 1998
 */

/*
 * Copyright (C) 1995 - 2001, 2003-2014, 2016-2020, 2022,
 * the Free Software Foundation, Inc.
 *
 * This file is part of GAWK, the GNU implementation of the
 * AWK Programming Language.
 *
 * GAWK is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * GAWK is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

use std::ffi::{CString, c_void};
use std::ptr;
use libloading::{Library, Symbol};
use std::os::raw::c_int;

#[cfg(not(feature = "dynamic"))]
pub fn load_ext(lib_name: &str) -> Result<(), String> {
    Err("dynamic loading of libraries is not supported".to_string())
}

#[cfg(feature = "dynamic")]
pub fn load_ext(lib_name: &str) -> Result<(), String> {
    if do_sandbox {
        return Err("extensions are not allowed in sandbox mode".to_string());
    }

    if do_traditional || do_posix {
        return Err("-l / @load are gawk extensions".to_string());
    }

    if lib_name.is_empty() {
        return Err("load_ext: received NULL lib_name".to_string());
    }

    let lib = unsafe { Library::new(lib_name) }
        .map_err(|e| format!("load_ext: cannot open library `{}`: {}", lib_name, e))?;

    let gpl_compat: Symbol<*mut c_int> = unsafe { lib.get(b"plugin_is_GPL_compatible") }
        .map_err(|e| format!("load_ext: library `{}`: does not define `plugin_is_GPL_compatible`: {}", 
            lib_name, e))?;

    if gpl_compat.is_null() {
        return Err(format!("load_ext: library `{}`: does not define `plugin_is_GPL_compatible`", 
            lib_name));
    }

    let install_func: Symbol<unsafe extern fn(*const gawk_api_t, *mut c_void) -> c_int> = 
        unsafe { lib.get(b"dl_load") }
            .map_err(|e| format!("load_ext: library `{}`: cannot call function `dl_load`: {}", 
                lib_name, e))?;

    if unsafe { install_func(&api_impl, ptr::null_mut()) } == 0 {
        return Err(format!("load_ext: library `{}' initialization routine `dl_load' failed", 
            lib_name));
    }

    Ok(())
}

pub fn make_builtin(name_space: &str, funcinfo: &awk_ext_func_t) -> Result<bool, String> {
    let name = funcinfo.name;
    if name.is_empty() {
        return Err("make_builtin: missing function name".to_string());
    }

    if !is_valid_identifier(name) {
        return Ok(false);
    }

    assert!(!name_space.is_empty());
    let install_name = if name_space.is_empty() || name_space == awk_namespace {
        if check_special(name) >= 0 {
            return Err(format!("make_builtin: cannot use gawk built-in `{}` as function name", name));
        }
        let f = lookup(name);
        name.to_string()
    } else {
        if !is_valid_identifier(name_space) {
            return Ok(false);
        }
        if check_special(name_space) >= 0 {
            return Err(format!("make_builtin: cannot use gawk built-in `{}` as namespace name", name_space));
        }
        if check_special(name) >= 0 {
            return Err(format!("make_builtin: cannot use gawk built-in `{}` as function name", name));
        }
        format!("{}::{}", name_space, name)
    };

    let f = lookup(&install_name);
    if let Some(node) = f {
        match node.node_type {
            NodeType::Func => {
                return Err(format!("make_builtin: cannot redefine function `{}`", name));
            }
            NodeType::ExtFunc => {
                if do_lint {
                    lintwarn(format!("make_builtin: function `{}` already defined", name));
                }
                return Ok(false);
            }
            _ => {
                return Err(format!("make_builtin: function name `{}` previously defined", name));
            }
        }
    }

    if funcinfo.max_expected_args < 0 {
        return Err(format!("make_builtin: negative argument count for function `{}`", name));
    }

    let b = bcalloc(Op::Symbol, 1, 0);
    b.extfunc = funcinfo.function;
    b.c_function = funcinfo;

    let symbol = install_symbol(&install_name, NodeType::ExtFunc);
    symbol.code_ptr = b;
    track_ext_func(name);
    Ok(true)
}

pub fn get_argument(i: i32) -> Option<Node> {
    let pc = TOP().code_ptr; // Op_ext_builtin instruction
    let arg_count = pc.expr_count; // # of arguments supplied

    if i < 0 || i >= arg_count {
        return None;
    }

    let t = PEEK(arg_count - i);
    if t.node_type == NodeType::ParamList {
        return Some(GET_PARAM(t.param_cnt));
    }

    match t.node_type {
        NodeType::ArrayRef => {
            if t.orig_array.node_type == NodeType::Var {
                t.node_type = NodeType::Var;
                t.var_value = Nnull_string.clone();
                Some(t)
            } else {
                Some(t.orig_array.clone())
            }
        }
        NodeType::Var => Some(Nnull_string.clone()),
        _ => Some(t.clone()),
    }
}

pub fn get_actual_argument(t: &Node, i: i32, want_array: bool) -> Result<Node, String> {
    let pc = TOP().code_ptr; // Op_ext_builtin instruction
    let fname = (pc + 1).func_name;

    match t.node_type {
        NodeType::VarNew | NodeType::ElemNew => {
            if want_array {
                Ok(force_array(t, false))
            } else {
                let mut new_t = t.clone();
                new_t.node_type = NodeType::Var;
                new_t.var_value = dupnode(&Nnull_string);
                Ok(new_t.var_value)
            }
        }
        _ => {
            if want_array && t.node_type != NodeType::VarArray {
                Err(format!("function `{}`: argument #{}: attempt to use scalar as an array", 
                    fname, i + 1))
            } else if !want_array && t.node_type != NodeType::Val {
                Err(format!("function `{}`: argument #{}: attempt to use array as a scalar", 
                    fname, i + 1))
            } else {
                Ok(t.clone())
            }
        }
    }
}

pub fn close_extensions() {
    if let Some(srcfiles) = srcfiles {
        let mut s = srcfiles.next;
        while s != srcfiles {
            if s.stype == SrcType::ExtLib && s.fini_func.is_some() {
                (s.fini_func.unwrap())();
            }
            s = s.next;
        }
    }
}

pub fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    if let Some(c) = chars.next() {
        if !is_letter(c) {
            return false;
        }
    } else {
        return false;
    }

    chars.all(|c| is_identchar(c))
}