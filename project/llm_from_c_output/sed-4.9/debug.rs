/*  GNU SED, a batch stream editor.
    Copyright (C) 2018-2022 Free Software Foundation, Inc.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 3, or (at your option)
    any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; If not, see <https://www.gnu.org/licenses/>. */

/* Written by Assaf Gordon.  */

/* debug.c: debugging functions */

use std::io::{self, Write};
use std::fmt;

static mut BLOCK_LEVEL: i32 = 0;

fn is_print(c: char) -> bool {
    c.is_ascii_graphic() || c == ' '
}

pub fn debug_print_char(c: char) {
    if is_print(c) && c != '\\' {
        print!("{}", c);
        return;
    }

    print!("\\");
    match c {
        '\x07' => print!("a"),
        '\x0c' => print!("f"),
        '\r' => print!("r"),
        '\t' => print!("t"),
        '\x0b' => print!("v"),
        '\n' => print!("n"),
        '\\' => print!("\\"),
        _ => print!("o{:03o}", c as u32),
    }
}

fn debug_print_regex_pattern(pat: &str) {
    for c in pat.chars() {
        if c == '/' {
            print!("\\/");
        } else {
            debug_print_char(c);
        }
    }
}

fn debug_print_regex_flags(r: &Regex, addr: bool) {
    if r.flags.contains(RegexFlags::ICASE) {
        print!("{}", if addr { 'I' } else { 'i' });
    }
    if r.flags.contains(RegexFlags::NEWLINE) {
        print!("{}", if addr { 'M' } else { 'm' });
    }
}

fn debug_print_regex(r: &Regex) {
    if r.re.is_empty() {
        print!("//");
        return;
    }

    print!("/");
    debug_print_regex_pattern(&r.re);
    print!("/");
}

fn debug_print_addr(a: &Addr) {
    match a.addr_type {
        AddrType::Null => print!("[ADDR-NULL]"),
        AddrType::Regex => {
            debug_print_regex(&a.addr_regex);
            debug_print_regex_flags(&a.addr_regex, true);
        }
        AddrType::Number => print!("{}", a.addr_number),
        AddrType::NumberMod => print!("{}~{}", a.addr_number, a.addr_step),
        AddrType::Step => print!("+{}", a.addr_step),
        AddrType::StepMod => print!("~{}", a.addr_step),
        AddrType::Last => print!("$"),
    }
}

fn debug_print_subst_replacement(r: &Replacement) {
    let mut last_repl_type = ReplacementType::AsIs;
    let mut p = r;

    while let Some(repl) = p {
        if repl.repl_type != last_repl_type {
            print!("\\");
            match repl.repl_type {
                ReplacementType::AsIs => print!("E"),
                ReplacementType::Uppercase => print!("U"),
                ReplacementType::Lowercase => print!("L"),
                _ => {
                    if repl.repl_type.contains(ReplacementType::UppercaseFirst) {
                        print!("u");
                    } else if repl.repl_type.contains(ReplacementType::LowercaseFirst) {
                        print!("l");
                    }
                }
            }
            last_repl_type = repl.repl_type;
        }

        if !repl.prefix.is_empty() {
            print!("{}", repl.prefix);
        }

        if repl.subst_id != -1 {
            if repl.subst_id == 0 {
                print!("&");
            } else {
                print!("\\{}", repl.subst_id);
            }
        }

        p = &repl.next;
    }
}

fn debug_print_output_file(o: &Output) {
    print!("{}", o.name);
}

fn debug_print_subst(s: &Subst) {
    debug_print_regex(&s.regx);
    debug_print_subst_replacement(&s.replacement);
    print!("/");

    debug_print_regex_flags(&s.regx, false);

    if s.global {
        print!("g");
    }
    if s.eval {
        print!("e");
    }
    if s.print {
        print!("p");
    }
    if s.numb != 0 {
        print!("{}", s.numb);
    }
    if let Some(ref outf) = s.outf {
        print!("w");
        debug_print_output_file(outf);
    }
}

fn debug_print_translation(sc: &SedCmd) {
    if MB_CUR_MAX > 1 {
        print!("/");
        for i in 0.. {
            if sc.x.translatemb[2 * i].is_empty() {
                break;
            }
            print!("{}", sc.x.translatemb[2 * i]);
        }
        print!("/");
        for i in 0.. {
            if sc.x.translatemb[2 * i].is_empty() {
                break;
            }
            print!("{}", sc.x.translatemb[2 * i + 1]);
        }
        print!("/");
    } else {
        print!("/");
        for i in 0..256 {
            if sc.x.translate[i] != i as u8 {
                print!("{}", i as u8 as char);
            }
        }
        print!("/");
        for i in 0..256 {
            if sc.x.translate[i] != i as u8 {
                print!("{}", sc.x.translate[i] as char);
            }
        }
        print!("/");
    }
}

fn debug_print_function(program: &Vector, sc: &SedCmd) {
    print!("{}", sc.cmd);

    match sc.cmd {
        '=' => (),
        ':' => print!("{}", sc.x.label_name),
        '{' | '}' => (),
        '#' => panic!("Unexpected command '#'"),
        'a' | 'c' | 'i' => {
            print!("\\");
            if !sc.x.cmd_txt.text.is_empty() {
                print!("{}", sc.x.cmd_txt.text);
            }
        }
        'b' | 't' | 'T' => {
            if (sc.x.jump_index as usize) < program.v.len() {
                if let Some(label_name) = program.v[sc.x.jump_index].x.label_name {
                    print!(" {}", label_name);
                }
            }
        }
        'D' | 'd' | 'F' | 'g' | 'G' | 'h' | 'H' | 'n' | 'N' | 'P' | 'p' | 'x' | 'z' => (),
        'e' => {
            print!(" ");
            print!("{}", sc.x.cmd_txt.text);
        }
        'L' | 'l' | 'q' | 'Q' => {
            if sc.x.int_arg != -1 {
                print!(" {}", sc.x.int_arg);
            }
        }
        'r' => {
            print!(" ");
            print!("{}", sc.x.readcmd.fname);
        }
        'R' => {
            print!(" ");
            print!("{}", sc.x.inf.name);
        }
        's' => debug_print_subst(&sc.x.cmd_subst),
        'v' => panic!("Unexpected command 'v'"),
        'W' | 'w' => debug_print_output_file(&sc.x.outf),
        'y' => debug_print_translation(sc),
        _ => panic!("Unknown sed command"),
    }
}

pub fn debug_print_command(program: &Vector, sc: &SedCmd) {
    unsafe {
        if sc.cmd == '}' {
            BLOCK_LEVEL -= 1;
        }

        for _ in 0..BLOCK_LEVEL {
            print!("  ");
        }

        debug_print_addr(&sc.a1);
        if sc.a2.is_some() {
            print!(",");
        }
        debug_print_addr(&sc.a2);

        let mut addr_bang = sc.addr_bang;
        if sc.cmd == '{' {
            addr_bang = !addr_bang;
        }
        if addr_bang {
            print!("!");
        }

        if sc.a1.is_some() || sc.a2.is_some() {
            print!(" ");
        }

        debug_print_function(program, sc);
        println!();

        if sc.cmd == '{' {
            BLOCK_LEVEL += 1;
        }
    }
}

pub fn debug_print_program(program: &Vector) {
    unsafe {
        BLOCK_LEVEL = 1;
        println!("SED PROGRAM:");
        for cmd in &program.v {
            debug_print_command(program, cmd);
        }
        BLOCK_LEVEL = 0;
    }
}

// Note: The following types and constants need to be defined elsewhere in your Rust code:
// - Regex, RegexFlags, Addr, AddrType, Replacement, ReplacementType, Output, Subst, SedCmd, Vector
// - MB_CUR_MAX constant